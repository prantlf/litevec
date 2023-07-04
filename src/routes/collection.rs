use aide::axum::{
	routing::{delete, get, post, put},
	ApiRouter,
};
use axum::{extract::Path, http::StatusCode, Extension};
use axum_jsonschema::Json;
use schemars::JsonSchema;

use crate::{
	db::{self, Collection, DbExtension, Embedding, Error as DbError, SimilarityResult},
	errors::HTTPError,
};

pub fn handler() -> ApiRouter {
	ApiRouter::new().nest(
		"/collections",
		ApiRouter::new()
			.api_route("/:collection_name", put(create_collection))
			.api_route("/:collection_name", post(query_collection))
			.api_route("/:collection_name", get(get_collection_info))
			.api_route("/:collection_name", delete(delete_collection))
			.api_route("/:collection_name/insert", post(insert_into_collection)),
	)
}

/// Create a new collection
async fn create_collection(
	Path(collection_name): Path<String>,
	Extension(db): DbExtension,
	Json(req): Json<Collection>,
) -> Result<StatusCode, HTTPError> {
	tracing::trace!(
		"Creating collection {collection_name} with dimension {}",
		req.dimension
	);

	let mut db = db.write().await;

	let create_result = db.create_collection(collection_name, req.dimension, req.distance);
	drop(db);

	match create_result {
		Ok(_) => Ok(StatusCode::CREATED),
		Err(db::Error::UniqueViolation) => {
			Err(HTTPError::new("Collection already exists").with_status(StatusCode::CONFLICT))
		},
		Err(_) => Err(HTTPError::new("Couldn't create collection")),
	}
}

#[derive(Debug, serde::Deserialize, JsonSchema)]
struct QueryCollectionQuery {
	query: Vec<f32>,
	k: Option<usize>,
}

#[allow(clippy::significant_drop_tightening)]
async fn query_collection(
	Path(collection_name): Path<String>,
	Extension(db): DbExtension,
	Json(req): Json<QueryCollectionQuery>,
) -> Result<Json<Vec<SimilarityResult>>, HTTPError> {
	tracing::trace!("Querying collection {collection_name}");

	let db = db.read().await;
	let collection = db
		.get_collection(&collection_name)
		.ok_or_else(|| HTTPError::new("Collection not found").with_status(StatusCode::NOT_FOUND))?;

	if req.query.len() != collection.dimension {
		return Err(HTTPError::new("Query dimension mismatch").with_status(StatusCode::BAD_REQUEST));
	}

	Ok(Json(
		collection.get_similarity(&req.query, req.k.unwrap_or(1)),
	))
}

async fn get_collection_info(
	Path(collection_name): Path<String>,
	Extension(db): DbExtension,
) -> Result<Json<Collection>, HTTPError> {
	tracing::trace!("Getting collection info for {collection_name}");

	let db = db.read().await;

	Ok(Json(
		db.get_collection(&collection_name)
			.ok_or_else(|| HTTPError::new("Collection not found"))?
			.clone(),
	))
}

async fn delete_collection(
	Path(collection_name): Path<String>,
	Extension(db): DbExtension,
) -> Result<StatusCode, HTTPError> {
	tracing::trace!("Deleting collection {collection_name}");

	let mut db = db.write().await;

	let delete_result = db.delete_collection(&collection_name);
	drop(db);

	match delete_result {
		Ok(_) => Ok(StatusCode::NO_CONTENT),
		Err(DbError::NotFound) => {
			Err(HTTPError::new("Collection not found").with_status(StatusCode::NOT_FOUND))
		},
		Err(_) => Err(HTTPError::new("Couldn't delete collection")),
	}
}

async fn insert_into_collection(
	Path(collection_name): Path<String>,
	Extension(db): DbExtension,
	Json(embedding): Json<Embedding>,
) -> Result<StatusCode, HTTPError> {
	tracing::trace!("Inserting into collection {collection_name}");

	let mut db = db.write().await;

	let insert_result = db.insert_into_collection(&collection_name, embedding);
	drop(db);

	match insert_result {
		Ok(_) => Ok(StatusCode::CREATED),
		Err(DbError::NotFound) => {
			Err(HTTPError::new("Collection not found").with_status(StatusCode::NOT_FOUND))
		},
		Err(DbError::UniqueViolation) => {
			Err(HTTPError::new("Vector already exists").with_status(StatusCode::CONFLICT))
		},
		Err(DbError::DimensionMismatch) => Err(HTTPError::new(
			"The provided vector has the wrong dimension",
		)
		.with_status(StatusCode::BAD_REQUEST)),
	}
}