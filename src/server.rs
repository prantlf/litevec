use aide::openapi::{self, OpenApi};
use anyhow::Result;
use axum::{
	Extension,
	http::{header::CONTENT_TYPE, Method},
};
use std::{env, net::SocketAddr};
use tokio::time::Duration;
use tower_http::{
	compression::CompressionLayer,
	cors::{Any, CorsLayer},
	decompression::RequestDecompressionLayer,
	limit::RequestBodyLimitLayer,
	timeout::TimeoutLayer,
	trace::TraceLayer,
	validate_request::ValidateRequestHeaderLayer,
};

use crate::{db, routes, shutdown};

pub async fn start() -> Result<()> {
	let mut openapi = OpenApi {
		info: openapi::Info {
			title: "litevec".to_string(),
			version: env!("CARGO_PKG_VERSION").to_string(),
			..openapi::Info::default()
		},
		..OpenApi::default()
	};

	let maxage = env::var("LITEVEC_CORS_MAXAGE").map_or(Ok(86_400), |v| v.parse())?;
	let cors = CorsLayer::new()
    .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
		.allow_headers([CONTENT_TYPE])
		.max_age(Duration::from_secs(maxage))
    .allow_origin(Any);

	let db = db::from_store()?;
	let timeout = env::var("LITEVEC_TIMEOUT").map_or(Ok(30), |v| v.parse())?;
	let limit = env::var("LITEVEC_PAYLOAD_LIMIT").map_or(Ok(1_073_741_824), |v| v.parse())?;
	let router = routes::handler()
		.finish_api(&mut openapi)
		.layer(Extension(openapi))
		.layer(db.extension())
		.layer(TimeoutLayer::new(Duration::from_secs(timeout)))
		.layer(RequestBodyLimitLayer::new(limit))
		.layer(ValidateRequestHeaderLayer::accept("application/json"))
		.layer(RequestDecompressionLayer::new())
		.layer(CompressionLayer::new())
		.layer(cors)
		.layer(TraceLayer::new_for_http());

	let host = env::var("LITEVEC_HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
	let port = env::var("LITEVEC_PORT").map_or(Ok(8000), |v| v.parse())?;
	let addr: SocketAddr = format!("{host}:{port}").parse()?;
	tracing::info!("Starting server on {addr}...");
	let server_fut = axum_server::bind(addr)
		.handle(shutdown::handle())
		.serve(router.into_make_service());

	let signal_fut = shutdown::watch_for_signal(addr);
	tokio::select! {
		() = signal_fut => {},
		res = server_fut => res?,
	}

	tracing::info!("Stopping server...");
	Ok(())
}
