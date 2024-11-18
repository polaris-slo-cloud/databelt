use axum::{http::StatusCode, response::IntoResponse, routing::get, routing::post, Router};
use hyper::{body::HttpBody as _, Client};
use hyper::{Body, Method, Request};
use redis::AsyncCommands;
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
    info!("Starting Example Preprocessor {}", env!("CARGO_PKG_VERSION"));
    let app = Router::new()
        .route("/", post(preprocess_handler))
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

async fn preprocess_handler(body: String) -> impl IntoResponse {
    info!(
        "preprocess_handler: Received POST body with length: {}",
        body.len()
    );
    let redis_host = env::var("REDIS_HOST").unwrap_or_else(|_| "redis".to_string());
    let redis_url = format!("redis://{}:6379", redis_host);
    info!("Connecting to Redis at URL: {}", redis_url);

    let client = redis::Client::open(redis_url).unwrap();
    let mut con = client.get_multiplexed_async_connection().await.unwrap();
    info!("Redis connection established");
    // Return a JSON response with the hash
    let _: () = con.set("PreProcessKey", body.clone()).await.unwrap();

    // send data to next function
    let url_str = "http://skylark-ex-detect.default.svc.cluster.local/";
    let post_body_str = stringify!(body.clone());
    info!("\nPOST and get result as string: {}", url_str);
    info!("with a POST body of length: {}", post_body_str.len());
    let url = url_str.parse::<hyper::Uri>().unwrap();
    post_url_return_str(url, post_body_str.as_bytes())
        .await
        .expect("POST to detect failed");

    (StatusCode::OK, "Data stored with key: PreProcessKey")
}

async fn health_probe() -> impl IntoResponse {
    StatusCode::OK
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
