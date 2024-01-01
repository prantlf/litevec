use anyhow::Context;
use axum::Extension;
use lazy_static::lazy_static;
use rayon::prelude::*;
use schemars::JsonSchema;
use std::{
	borrow::ToOwned,
	collections::{BinaryHeap, HashMap},
	fs,
	path::PathBuf,
	sync::Arc,
};
use tokio::{
	sync::RwLock,
	time::{self, Duration},
};

use crate::similarity::{get_cache_attr, get_distance_fn, normalize, Distance, ScoreIndex};

lazy_static! {
	pub static ref STORE_PATH: PathBuf = PathBuf::from("./storage/db");
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
	/// If a collection was modified and hasn't been saved yet
	#[serde(skip)]
	dirty: bool,
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
}

impl Collection {
	pub fn list(&self) -> Vec<String> {
		tracing::debug!("Listing {} embeddings", self.embeddings.len());
		self.embeddings.iter().map(|e| e.id.clone()).collect()
	}

	pub fn get(&self, id: &str) -> Option<&Embedding> {
		tracing::debug!("Getting embedding {}", id);
		self.embeddings.iter().find(|e| e.id == id)
	}

	pub fn get_by_metadata(&self, filter: &[HashMap<String, String>], k: usize) -> Vec<Embedding> {
		let embeddings: Vec<Embedding> = self
			.embeddings
			.iter()
			.filter_map(|embedding| {
				if match_embedding(embedding, filter) {
					Some(embedding.clone())
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
			dirty: false,
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
		};

		self.collections.insert(name, collection.clone());
		self.set_dirty();

		Ok(collection)
	}

	pub fn delete_collection(&mut self, name: &str) -> Result<(), Error> {
		tracing::debug!("Deleting collection {name}");

		if !self.collections.contains_key(name) {
			return Err(Error::NotFound);
		}

		self.collections.remove(name);
		self.set_dirty();

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
		self.set_dirty();

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
		if !STORE_PATH.exists() {
			tracing::debug!("Creating database store");
			fs::create_dir_all(STORE_PATH.parent().context("Invalid store path")?)?;

			return Ok(Self::new());
		}

		tracing::debug!("Loading database from store");
		let db = fs::read(STORE_PATH.as_path())?;
		Ok(bincode::deserialize(&db[..])?)
	}

	pub const fn is_dirty(&self) -> bool {
		self.dirty
	}

	pub fn set_dirty(&mut self) {
		self.dirty = true;
	}

	pub fn save_to_store(&mut self) -> anyhow::Result<()> {
		tracing::debug!("Saving database to store");
		let db = bincode::serialize(self)?;

		fs::write(STORE_PATH.as_path(), db)?;
		self.dirty = false;

		Ok(())
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
