use axum::{http::StatusCode, response::IntoResponse, routing::get, routing::post, Router};
use hyper::{body::HttpBody as _, Client};
use hyper::{Body, Method, Request};
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

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    pretty_env_logger::init_timed();
    info!(
        "Starting Example Image Detector {}",
        env!("CARGO_PKG_VERSION")
    );
    let app = Router::new()
        .route("/", post(detect_handler))
        .route("/health", get(health_probe));

    debug!("Skylark library loaded: {}", get_version());
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

async fn detect_handler(body: String) -> impl IntoResponse {
    info!(
        "detect_handler: Received POST body with length: {}",
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
    let _: () = con.set(data_hash.clone(), body.clone()).await.unwrap();
    let post_body_str = stringify!(body.clone());
    // send data to next function
    let url_str = "http://skylark-ex-alarm.default.svc.cluster.local/";

    info!("\nPOST and get result as string: {}", url_str);
    info!("with a POST body: {}", post_body_str);
    let url = url_str.parse::<hyper::Uri>().unwrap();
    post_url_return_str(url, post_body_str.as_bytes())
        .await
        .expect("POST to detect failed");

    (StatusCode::OK, format!("Data stored with key: {}", data_hash.clone()))
}

async fn post_url_return_str(url: hyper::Uri, post_body: &'static [u8]) -> Result<()> {
    let client = Client::new();
    let req = Request::builder()
        .method(Method::POST)
        .uri(url)
        .body(Body::from(post_body))?;
    let mut res = client.request(req).await?;

    let mut resp_data = Vec::new();
    while let Some(next) = res.data().await {
        let chunk = next?;
        resp_data.extend_from_slice(&chunk);
    }
    info!("{}", String::from_utf8_lossy(&resp_data));

    Ok(())
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
