use redis::AsyncCommands;
use rand::{distributions::Alphanumeric, Rng};
use sha2::{Sha256, Digest};
use std::env;
use anyhow::Result;
use bytes::Bytes;
use futures_util::{StreamExt};
use axum::{extract::BodyStream, routing::get, routing::post, Json, Router};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use tokio::net::TcpListener;
use serde_json::json;
use skylark::get_version;
use skylark::get_nodes;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    println!("ex_detect starting");
    let app = Router::new()
        .route("/", get(info))
        .route("/hash", get(hash))
        .route("/health", get(health_probe))
        .route("/echo", post(echo));

    println!("Skylark library loaded: {}", get_version());
    let addr = "0.0.0.0:8080";

    get_nodes().await.expect("TODO: oof");


    let tcp_listener = TcpListener::bind(addr).await.unwrap();
    println!("listening on {}", addr);
    axum::Server::from_tcp(tcp_listener.into_std().unwrap())
        .unwrap()
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn info() -> &'static str {
    "Example Image Object Detector"
}

async fn echo(mut stream: BodyStream) -> Bytes {
    if let Some(Ok(s)) = stream.next().await {
        s
    } else {
        Bytes::new()
    }
}

async fn hash() -> Result<impl IntoResponse, StatusCode> {
    println!("Generating hash data");
    // Generate random data and compute its hash
    let data = generate_random_data(500);
    let mut hasher = Sha256::new();
    hasher.update(data.as_bytes());
    let data_hash = format!("{:x}", hasher.finalize());
    println!("Hash: {}", data_hash);

    let redis_host = env::var("REDIS_HOST").unwrap_or_else(|_| "redis".to_string());
    let redis_url = format!("redis://{}:6379", redis_host);
    println!("Connecting to Redis at URL: {}", redis_url);

    let client = redis::Client::open(redis_url).unwrap();
    let mut con = client.get_multiplexed_async_connection().await.unwrap();
    println!("Redis connection established");
    // Store hash and data in Redis
    let _: () = con.set(data_hash.clone(), data).await.unwrap();
    println!("Data stored");
    // Return a JSON response with the hash
    Ok((
        StatusCode::OK,
        Json(json!({"status": "success", "hash": data_hash}))
    ))
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