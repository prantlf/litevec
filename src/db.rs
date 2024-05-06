use axum::Extension;
use lazy_static::lazy_static;
use rayon::prelude::*;
use schemars::JsonSchema;
use std::{
	borrow::ToOwned,
	collections::{BinaryHeap, HashMap},
	fs::{self, File},
	path::PathBuf,
	sync::Arc,
};
use tokio::{
	sync::RwLock,
	time::{self, Duration},
};
use url_escape::{decode, encode_component};

use crate::similarity::{get_cache_attr, get_distance_fn, normalize, Distance, ScoreIndex};

lazy_static! {
	pub static ref STORE_PATH: PathBuf = PathBuf::from("./storage");
}

#[allow(clippy::module_name_repetitions)]
pub type DbExtension = Extension<Arc<RwLock<Db>>>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
	#[error("Collection already exists")]
	UniqueViolation,

	#[error("Collection doesn't exist")]
	NotFound,

	#[error("The dimension of the vector doesn't match the dimension of the collection")]
	DimensionMismatch,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Db {
	/// Collections in the database
	pub collections: HashMap<String, Collection>,
	/// List of deleted collection names since the last storing
	#[serde(skip)]
	deleted: Vec<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, JsonSchema)]
pub struct SimilarityResult {
	/// Similarity score
	score: f32,
	/// Matching embedding
	embedding: Embedding,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, JsonSchema)]
pub struct Collection {
	/// Dimension of the vectors in the collection
	pub dimension: usize,
	/// Distance metric used for querying
	pub distance: Distance,
	/// Embeddings in the collection
	#[serde(default)]
	pub embeddings: Vec<Embedding>,
	/// If the collection was modified and hasn't been saved yet
	#[serde(skip)]
	dirty: bool,
}

impl Collection {
	pub const fn is_dirty(&self) -> bool {
		self.dirty
	}

	pub fn set_dirty(&mut self) {
		self.dirty = true;
	}

	pub fn unset_dirty(&mut self) {
		self.dirty = false;
	}

	pub fn list(&self) -> Vec<String> {
		tracing::debug!("Listing {} embeddings", self.embeddings.len());
		self.embeddings.iter().map(|e| e.id.clone()).collect()
	}

	pub fn get(&self, id: &str) -> Option<&Embedding> {
		tracing::debug!("Getting embedding {}", id);
		self.embeddings.iter().find(|e| e.id == id)
	}

	pub fn get_by_metadata(
		&self,
		filter: &[HashMap<String, String>],
		k: usize,
		novector: bool,
	) -> Vec<Embedding> {
		let embeddings: Vec<Embedding> = self
			.embeddings
			.iter()
			.filter_map(|embedding| {
				if match_embedding(embedding, filter) {
					if novector {
						let mut clone = embedding.clone();
						clone.vector.clear();
						Some(clone)
					} else {
						Some(embedding.to_owned())
					}
				} else {
					None
				}
			})
			.take(k)
			.collect();
		tracing::debug!("Found {} embeddings", embeddings.len());
		embeddings
	}

	pub fn get_by_metadata_and_similarity(
		&self,
		filter: &[HashMap<String, String>],
		query: &[f32],
		k: usize,
	) -> Vec<SimilarityResult> {
		let memo_attr = get_cache_attr(self.distance, query);
		let distance_fn = get_distance_fn(self.distance);

		let scores = self
			.embeddings
			.par_iter()
			.enumerate()
			.filter_map(|(index, embedding)| {
				if match_embedding(embedding, filter) {
					let score = distance_fn(&embedding.vector, query, memo_attr);
					Some(ScoreIndex { score, index })
				} else {
					None
				}
			})
			.collect::<Vec<_>>();

		let mut heap = BinaryHeap::new();
		for score_index in scores {
			if heap.len() < k || score_index < *heap.peek().unwrap() {
				heap.push(score_index);

				if heap.len() > k {
					heap.pop();
				}
			}
		}

		tracing::debug!("Found {} embeddings", heap.len());
		heap.into_sorted_vec()
			.into_iter()
			.map(|ScoreIndex { score, index }| SimilarityResult {
				score,
				embedding: self.embeddings[index].clone(),
			})
			.collect()
	}

	pub fn update_metadata(&mut self, id: &str, metadata: Option<HashMap<String, String>>) -> bool {
		tracing::debug!("Updating embedding {}", id);
		let embedding = self.embeddings.iter_mut().find(|e| e.id == id);

		match embedding {
			None => false,
			Some(embedding) => {
				embedding.metadata = metadata;
				true
			},
		}
	}

	pub fn delete(&mut self, id: &str) -> bool {
		let index_opt = self.embeddings.iter().position(|e| e.id == id);

		match index_opt {
			None => false,
			Some(index) => {
				tracing::debug!("Deleting embedding {}", id);
				self.embeddings.remove(index);
				true
			},
		}
	}

	pub fn delete_by_metadata(&mut self, filter: &[HashMap<String, String>]) -> bool {
		if filter.is_empty() {
			let len = self.embeddings.len();
			tracing::debug!("Deleting {} embeddings", len);
			self.embeddings.clear();
			return len > 0;
		}

		let indexes = self
			.embeddings
			.par_iter()
			.enumerate()
			.filter_map(|(index, embedding)| {
				if match_embedding(embedding, filter) {
					tracing::debug!("Deleting embedding {}", embedding.id);
					Some(index)
				} else {
					None
				}
			})
			.collect::<Vec<_>>();
		let len = indexes.len();

		for index in indexes {
			self.embeddings.remove(index);
		}

		tracing::debug!("Deleted {} embeddings", len);
		len > 0
	}
}

fn match_embedding(embedding: &Embedding, filter: &[HashMap<String, String>]) -> bool {
	// an empty filter matches any embedding
	if filter.is_empty() {
		return true;
	}

	match &embedding.metadata {
		// no metadata in an embedding cannot be matched by a not empty filter
		None => false,
		Some(metadata) => {
			// enumerate criteria with OR semantics; look for the first one matching
			for criteria in filter {
				let mut matches = true;
				// enumerate entries with AND semantics; look for the first one failing
				for (key, expected) in criteria {
					let found = metadata.get(key).map_or(false, |actual| actual == expected);
					// a not matching entry means the whole embedding not matching
					if !found {
						matches = false;
						break;
					}
				}
				// all entries matching mean the whole embedding matching
				if matches {
					return true;
				}
			}
			// no match found
			false
		},
	}
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, JsonSchema)]
pub struct Embedding {
	/// Unique identifier
	pub id: String,
	/// Vector computed from a text chunk
	pub vector: Vec<f32>,
	/// Metadata about the source text
	pub metadata: Option<HashMap<String, String>>,
}

impl Db {
	pub fn new() -> Self {
		Self {
			collections: HashMap::new(),
			deleted: Vec::<String>::new(),
		}
	}

	pub fn create_collection(
		&mut self,
		name: String,
		dimension: usize,
		distance: Distance,
	) -> Result<Collection, Error> {
		tracing::debug!("Creating collection {name}");

		if self.collections.contains_key(&name) {
			return Err(Error::UniqueViolation);
		}

		let collection = Collection {
			dimension,
			distance,
			embeddings: Vec::new(),
			dirty: true,
		};

		let name_for_remove = name.clone();
		self.collections.insert(name, collection.clone());
		self.remove_deleted(&name_for_remove);

		Ok(collection)
	}

	pub fn rename_collection(&mut self, name: &str, new_name: String) -> Result<(), Error> {
		tracing::debug!("Renaming collection {name} to {new_name}");

		if self.collections.contains_key(&new_name) {
			return Err(Error::UniqueViolation);
		}

		let mut collection = self.collections.remove(name).ok_or(Error::NotFound)?;
		self.add_deleted(name);

		collection.set_dirty();
		let name_for_remove = new_name.clone();
		self.collections.insert(new_name, collection);
		self.remove_deleted(&name_for_remove);

		Ok(())
	}

	pub fn delete_collection(&mut self, name: &str) -> Result<(), Error> {
		tracing::debug!("Deleting collection {name}");

		if !self.collections.contains_key(name) {
			return Err(Error::NotFound);
		}

		self.collections.remove(name);
		self.add_deleted(name);

		Ok(())
	}

	pub fn insert_into_collection(
		&mut self,
		collection_name: &str,
		mut embedding: Embedding,
	) -> Result<(), Error> {
		let collection = self
			.collections
			.get_mut(collection_name)
			.ok_or(Error::NotFound)?;

		if collection.embeddings.iter().any(|e| e.id == embedding.id) {
			return Err(Error::UniqueViolation);
		}

		if embedding.vector.len() != collection.dimension {
			return Err(Error::DimensionMismatch);
		}

		// Normalize the vector if the distance metric is cosine, so we can use dot product later
		if collection.distance == Distance::Cosine {
			embedding.vector = normalize(&embedding.vector);
		}

		tracing::debug!(
			"Inserting embedding {} to collection {}",
			embedding.id,
			collection_name
		);
		collection.embeddings.push(embedding);
		collection.set_dirty();

		Ok(())
	}

	pub fn get_collection(&self, name: &str) -> Option<&Collection> {
		tracing::debug!("Getting collection {}", name);
		self.collections.get(name)
	}

	pub fn get_collection_mut(&mut self, name: &str) -> Option<&mut Collection> {
		tracing::debug!("Getting collection {}", name);
		self.collections.get_mut(name)
	}

	pub fn list(&self) -> Vec<String> {
		tracing::debug!("Listing {} collections", self.collections.len());
		self.collections.keys().map(ToOwned::to_owned).collect()
	}

	fn load_from_store() -> anyhow::Result<Self> {
		let marker = STORE_PATH.join("._collections");
		if STORE_PATH.exists() {
			if marker.exists() {
				let mut db = Self::new();
				db.load_collections()?;
				return Ok(db);
			}
			let db = Self::convert_old_store()?;
			File::create(marker)?;
			return Ok(db);
		}
		tracing::debug!("Creating database store");
		fs::create_dir_all(STORE_PATH.as_path())?;
		File::create(marker)?;
		Ok(Self::new())
	}

	pub fn is_dirty(&self) -> bool {
		return !self.deleted.is_empty() || self.collections.values().any(Collection::is_dirty);
	}

	pub fn add_deleted(&mut self, name: &str) {
		self.deleted.push(name.to_string());
	}

	pub fn remove_deleted(&mut self, name: &String) -> bool {
		let deleted_len = self.deleted.len();
		let deleted_index = self
			.deleted
			.iter()
			.position(|deleted_name| deleted_name == name)
			.unwrap_or(deleted_len);
		if deleted_index < deleted_len {
			self.deleted.remove(deleted_index);
			true
		} else {
			false
		}
	}

	pub fn save_to_store(&mut self) -> anyhow::Result<()> {
		self.delete_collections()?;
		self.store_collections()
	}

	fn delete_collections(&mut self) -> anyhow::Result<()> {
		for name in &self.deleted {
			tracing::debug!("Deleting collection {} from store", name);
			let file_name = encode_component(name).to_string();
			fs::remove_file(STORE_PATH.join(file_name).as_path())?;
		}
		self.deleted.clear();
		Ok(())
	}

	fn store_collections(&mut self) -> anyhow::Result<()> {
		for (name, collection) in &mut self.collections {
			if collection.is_dirty() {
				tracing::debug!("Saving collection {} to store", name);
				let binary = bincode::serialize(&collection)?;
				let file_name = encode_component(name).to_string();
				fs::write(STORE_PATH.join(file_name).as_path(), binary)?;
				collection.unset_dirty();
			}
		}
		Ok(())
	}

	fn load_collections(&mut self) -> anyhow::Result<()> {
		for entry in fs::read_dir(STORE_PATH.as_path())? {
			let entry = entry?;
			let entry_name = entry.file_name();
			if entry_name == "._collections" {
				continue;
			}
			let file_name = entry_name.to_str().ok_or(Error::NotFound)?;
			let collection_name = decode(file_name).to_string();
			tracing::debug!("Loading collection {} from store", collection_name);
			let binary = fs::read(entry.path())?;
			let collection: Collection = bincode::deserialize(&binary[..])?;
			self.collections.insert(collection_name, collection);
		}
		Ok(())
	}

	fn convert_old_store() -> anyhow::Result<Self> {
		tracing::debug!("Converting old database store");
		let db_path = STORE_PATH.join("db");
		let binary = fs::read(db_path.clone())?;
		let mut db: Self = bincode::deserialize(&binary[..])?;
		for collection in &mut db.collections.values_mut() {
			collection.set_dirty();
		}
		db.store_collections()?;
		fs::remove_file(db_path)?;
		Ok(db)
	}
}

impl Drop for Db {
	fn drop(&mut self) {
		if self.is_dirty() {
			self.save_to_store().ok();
		}
	}
}

pub fn from_store() -> anyhow::Result<Arc<RwLock<Db>>> {
	Ok(Arc::new(RwLock::new(Db::load_from_store()?)))
}

#[allow(clippy::similar_names)]
pub fn autosave(db: Arc<RwLock<Db>>, duration: u32) {
	let mut interval = time::interval(Duration::from_secs(duration.into()));
	tokio::spawn(async move {
		loop {
			interval.tick().await;
			let dbr = db.read().await;
			let dirty = dbr.is_dirty();
			drop(dbr);
			if dirty {
				let mut dbw = db.write().await;
				if dbw.is_dirty() {
					dbw.save_to_store().ok();
				}
				drop(dbw);
			}
		}
	});
}
