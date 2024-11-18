use axum::{
    extract::Query,
    routing::get,
    Router,
};
use redis::{Commands, Connection};
use serde::Deserialize;
use std::env;
use std::io::Cursor;
use base64::{Engine as _, engine::{general_purpose}};
use image::ImageFormat;

#[derive(Deserialize)]
struct Params {
    key: String, // The Redis key of the image
}

// Connect to Redis
fn get_redis_connection() -> Connection {
    let redis_host = env::var("REDIS_HOST").unwrap_or_else(|_| "redis.default.svc.cluster.local".to_string());
    let redis_port = env::var("REDIS_PORT").unwrap_or_else(|_| "6379".to_string());
    let redis_url = format!("redis://{}:{}", redis_host, redis_port);

    let client = redis::Client::open(redis_url).expect("Failed to create Redis client");
    client.get_connection().expect("Failed to connect to Redis")
}

// Image preprocessing function
fn preprocess_image(base64_data: &str) -> String {
    // Decode base64 data
    let decoded_data = general_purpose::STANDARD
        .decode(base64_data).unwrap();
    println!("{:?}", decoded_data);

    // Load image
    let img = image::load_from_memory(&decoded_data).expect("Failed to load image");

    // Example preprocessing: Resize to 100x100 pixels
    let resized = img.resize(100, 100, image::imageops::FilterType::Nearest);

    // Convert processed image back to base64
    let mut buffer = Cursor::new(Vec::new());
    resized
        .write_to(&mut buffer, ImageFormat::Jpeg)
        .expect("Failed to write image");

    // Retrieve the inner `Vec<u8>` containing the image data
    let image_data = buffer.into_inner();

    general_purpose::STANDARD.encode(image_data)
}

async fn preprocess_handler(Query(params): Query<Params>) -> String {
    let mut redis_conn = get_redis_connection();

    // Get the raw image data from Redis
    let base64_data: String = redis_conn
        .get(&params.key)
        .expect("Failed to retrieve image data from Redis");

    // Preprocess the image
    let processed_data = preprocess_image(&base64_data);

    // Store the processed image back in Redis
    let processed_key = format!("processed_{}", params.key);
    let _: () = redis_conn
        .set(&processed_key, processed_data.clone())
        .expect("Failed to store processed image in Redis");

    // Return the key of the processed image
    processed_key
}

#[tokio::main(flavor = "current_thread")]
async fn main() {

    let app = Router::new().route("/", get(preprocess_handler));
    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
