use aide::openapi::{self, OpenApi};
use anyhow::Result;
use axum::Extension;
use std::{env, net::SocketAddr};
use tower_http::trace::TraceLayer;

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

	let db = db::from_store()?;
	let router = routes::handler()
		.finish_api(&mut openapi)
		.layer(Extension(openapi))
		.layer(db.extension())
		.layer(TraceLayer::new_for_http());

	let addr = SocketAddr::from((
		[0, 0, 0, 0],
		env::var("PORT").map_or(Ok(8000), |p| p.parse())?,
	));
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
