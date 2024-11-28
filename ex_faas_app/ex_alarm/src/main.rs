use axum::{http::StatusCode, response::IntoResponse, routing::get, routing::post, Router};
use rand::{distributions::Alphanumeric, Rng};
use sha2::{Digest, Sha256};
use skylark_lib::{get_version, store_state, init_and_get_predecessor_state};
use std::env;
use tokio::net::TcpListener;
extern crate pretty_env_logger;
#[macro_use]
extern crate log;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    pretty_env_logger::init_timed();
    info!("Starting Example Fire Alarm {}", env!("CARGO_PKG_VERSION"));
    let app = Router::new()
        .route("/", post(alarm_handler))
        .route("/health", get(health_probe));

    info!("Skylark library loaded: {}", get_version());
    let addr = "0.0.0.0:8080";


    let tcp_listener = TcpListener::bind(addr).await.unwrap();
    info!("listening on {}", addr);
    axum::Server::from_tcp(tcp_listener.into_std().unwrap())
        .unwrap()
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn alarm_handler(body: String) -> impl IntoResponse {
    info!(
        "alarm_handler: Received POST body with length: {}",
        body.len()
    );
    // Generate random data and compute its hash
    let data = generate_random_data(500);
    let mut hasher = Sha256::new();
    hasher.update(data.as_bytes());
    let data_hash = format!("{:x}", hasher.finalize());

    warn!("ALARM ALARM");
    // Return a JSON response with the hash
    (StatusCode::OK, format!("Data stored with key: {}", data_hash.clone()))
}

fn generate_random_data(size_kb: usize) -> String {
    let size_bytes = size_kb * 1024;
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(size_bytes)
        .map(char::from)
        .collect()
}

async fn health_probe() -> impl IntoResponse {
    StatusCode::OK
}
