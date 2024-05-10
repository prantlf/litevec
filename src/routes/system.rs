use aide::axum::{
	routing::{get, post},
	ApiRouter,
};
use axum::{http::StatusCode, Extension};
use axum_jsonschema::Json;
use schemars::JsonSchema;

use crate::{db::DbExtension, shutdown};

pub fn handler() -> ApiRouter {
	ApiRouter::new()
		.api_route("/", get(root))
		.api_route("/ping", get(trigger_ping).head(trigger_ping))
		.api_route("/shutdown", post(trigger_shutdown))
}

#[derive(Debug, serde::Serialize, JsonSchema)]
pub struct AppVersion {
	semver: String,
	rev: Option<String>,
	compile_time: String,
}

#[derive(Debug, serde::Serialize, JsonSchema)]
pub struct RootResponse {
	/// Relative URL to Swagger UI
	pub docs_url: String,
	/// Relative URL to `OpenAPI` specification
	pub openapi_url: String,
	/// Application version
	pub version: AppVersion,
}

pub async fn root() -> Json<RootResponse> {
	Json(RootResponse {
		docs_url: "/docs".to_string(),
		openapi_url: "/openapi.json".to_string(),
		version: AppVersion {
			semver: env!("CARGO_PKG_VERSION").to_string(),
			compile_time: env!("STATIC_BUILD_DATE").to_string(),
			rev: option_env!("GIT_REV").map(ToString::to_string),
		},
	})
}

pub async fn trigger_ping() -> StatusCode {
	StatusCode::NO_CONTENT
}

pub async fn trigger_shutdown(Extension(db): DbExtension) -> StatusCode {
	let db = db.read().await;
	drop(db);

	shutdown::trigger();

	StatusCode::NO_CONTENT
}
