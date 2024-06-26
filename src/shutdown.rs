use axum_server::Handle;
use lazy_static::lazy_static;
use std::net::SocketAddr;
use tokio::{signal, time::Duration};

lazy_static! {
	static ref HANDLE: Handle = Handle::new();
}

pub fn handle() -> Handle {
	HANDLE.clone()
}

pub fn trigger() {
	HANDLE.graceful_shutdown(Some(Duration::from_secs(1)));
}

pub async fn watch_for_signal(addr: SocketAddr) {
	let ctrl_c = async {
		signal::ctrl_c()
			.await
			.expect("failed to install SIGINT handler");
	};

	#[cfg(unix)]
	let terminate = async {
		signal::unix::signal(signal::unix::SignalKind::terminate())
			.expect("failed to install SIGTERM handler")
			.recv()
			.await;
	};

	#[cfg(not(unix))]
	let terminate = std::future::pending::<()>();

	tokio::select! {
		() = ctrl_c => {},
		() = terminate => {},
	}

	let client = reqwest::Client::new();
	let res = client.post(format!("http://{addr}/shutdown")).send().await;
	match res {
		Ok(_) | Err(_) => {},
	}
}
