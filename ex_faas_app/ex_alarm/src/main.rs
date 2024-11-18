use axum::{http::StatusCode, response::IntoResponse, routing::get, routing::post, Router};
use rand::{distributions::Alphanumeric, Rng};
use redis::AsyncCommands;
use sha2::{Digest, Sha256};
use skylark::get_nodes;
use skylark::get_version;
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

    get_nodes().await.expect("TODO: oof");

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

    let redis_host = env::var("REDIS_HOST").unwrap_or_else(|_| "redis".to_string());
    let redis_url = format!("redis://{}:6379", redis_host);
    info!("Connecting to Redis at URL: {}", redis_url);

    let client = redis::Client::open(redis_url).unwrap();
    let mut con = client.get_multiplexed_async_connection().await.unwrap();
    info!("Redis connection established");
    // Store hash and data in Redis
    let _: () = con.set(data_hash.clone(), body).await.unwrap();
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
