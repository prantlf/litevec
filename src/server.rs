use aide::openapi::{self, OpenApi};
use anyhow::Result;
use axum::{
	http::{header::CONTENT_TYPE, Method},
	Extension,
};
use std::{env, net::SocketAddr, sync::Arc};
use tokio::time::Duration;
use tower_http::{
	compression::{predicate::SizeAbove, CompressionLayer, DefaultPredicate, Predicate},
	cors::{Any, CorsLayer},
	decompression::RequestDecompressionLayer,
	limit::RequestBodyLimitLayer,
	timeout::TimeoutLayer,
	trace::TraceLayer,
	validate_request::ValidateRequestHeaderLayer,
	CompressionLevel,
};

use crate::{db, routes, shutdown};

pub async fn start() -> Result<()> {
	let db = db::from_store()?;
	let duration = env::var("LITEVEC_AUTOSAVE_INTERVAL").map_or(Ok(10), |v| v.parse())?;
	db::autosave(Arc::clone(&db), duration);

	let mut openapi = OpenApi {
		info: openapi::Info {
			title: "litevec".to_string(),
			version: env!("CARGO_PKG_VERSION").to_string(),
			..openapi::Info::default()
		},
		..OpenApi::default()
	};

	let compression_limit =
		env::var("LITEVEC_COMPRESSION_LIMIT").map_or(Ok(1024), |v| v.parse())?;
	let compression_predicate = DefaultPredicate::new().and(SizeAbove::new(compression_limit));
	let compression = CompressionLayer::new()
		.quality(CompressionLevel::Best)
		.compress_when(compression_predicate);

	let maxage = env::var("LITEVEC_CORS_MAXAGE").map_or(Ok(86_400), |v| v.parse())?;
	let cors = CorsLayer::new()
		.allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
		.allow_headers([CONTENT_TYPE])
		.max_age(Duration::from_secs(maxage))
		.allow_origin(Any);

	let timeout = env::var("LITEVEC_TIMEOUT").map_or(Ok(30), |v| v.parse())?;
	let payload_limit =
		env::var("LITEVEC_PAYLOAD_LIMIT").map_or(Ok(1_073_741_824), |v| v.parse())?;
	let router = routes::handler()
		.finish_api(&mut openapi)
		.layer(Extension(openapi))
		.layer(Extension(db))
		.layer(TimeoutLayer::new(Duration::from_secs(timeout)))
		.layer(RequestBodyLimitLayer::new(payload_limit))
		.layer(ValidateRequestHeaderLayer::accept("application/json"))
		.layer(RequestDecompressionLayer::new())
		.layer(compression)
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
