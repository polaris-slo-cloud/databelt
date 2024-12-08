use hyper::server::conn::Http;
use hyper::service::service_fn;
use hyper::{Body, Method, Request, Response, StatusCode};
use sha2::{Digest, Sha256};
use skylark_lib::{skylark_init, skylark_lib_version, store_state};
use std::env;
use std::net::SocketAddr;
use tokio::net::TcpListener;

extern crate pretty_env_logger;
#[macro_use]
extern crate log;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    pretty_env_logger::init_timed();

    info!(
        "Starting Example Preprocessor {}",
        env!("CARGO_PKG_VERSION")
    );
    info!("Skylark library loaded: {}", skylark_lib_version());
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));

    let listener = TcpListener::bind(addr).await?;
    info!("Listening on http://{}", addr);
    loop {
        let (stream, _) = listener.accept().await?;

        tokio::task::spawn(async move {
            if let Err(err) = Http::new()
                .serve_connection(stream, service_fn(http_handler))
                .await
            {
                error!("Error serving connection: {:?}", err);
            }
        });
    }
}

async fn http_handler(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    match (req.method(), req.uri().path()) {
        (&Method::POST, "/process") => {
            info!("main::http_handler::preprocess_handler: incoming");
            match skylark_init(env!("CARGO_PKG_NAME").to_string(), None, skylark_lib::SkylarkMode::Sat).await{
                Ok(r) =>{
                    info!("main::http_handler::preprocess_handler: Skylark initialized: {:?}", r);
                }
                Err(err) => {
                    error!("main::http_handler::preprocess_handler: Failed to initialize: {:?}", err);
                    return Ok(Response::builder()
                        .status(StatusCode::INTERNAL_SERVER_ERROR)
                        .body(Body::from("Failed to initialize Skylark State"))
                        .unwrap())
                }
            }
            info!("main::http_handler::preprocess_handler: initialized skylark lib");
            let whole_body = hyper::body::to_bytes(req.into_body()).await?;
            let str_body = String::from_utf8(whole_body.to_vec()).unwrap();

            info!(
                "preprocess_handler: Received POST body with length: {}",
                str_body.len()
            );

            let mut hasher = Sha256::new();
            hasher.update(whole_body);
            let data_hash = format!("{:x}", hasher.finalize());
            match store_state(data_hash).await {
                Ok(key) => {
                    info!(
                        "main::http_handler::store_state: skylark lib result: {:?}",
                        key
                    );
                    Ok(Response::new(Body::from(key)))
                }
                Err(e) => {
                    error!("main::http_handler::store_state: Error calling skylark lib store state: {:?}", e);
                    Ok(Response::builder()
                        .status(StatusCode::NOT_FOUND)
                        .body(Body::from("Error calling skylark lib store state"))
                        .unwrap())
                }
            }
        }
        (&Method::GET, "/health") => Ok(Response::new(Body::from("OK\n"))),
        _ => Ok(Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::from("Route not found"))
            .unwrap()),
    }
}
