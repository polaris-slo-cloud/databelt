use hyper::server::conn::Http;
use hyper::service::service_fn;
use hyper::{Body, Method, Request, Response, StatusCode};
use sha2::{Digest, Sha256};
use skylark_lib::{skylark_lib_version, store_state, SkylarkMode};
use std::env;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use url::Url;

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
            info!("Incoming");
            let request_url =
                match Url::parse(&format!("http://skylark.at{}", req.uri().to_string())) {
                    Ok(url) => url,
                    Err(e) => {
                        error!("Error parsing URI: {}", e.to_string());
                        return Ok(Response::builder()
                            .status(StatusCode::BAD_REQUEST)
                            .body(hyper::Body::from("Not able to parse URL"))
                            .unwrap());
                    }
                };
            let params = request_url.query_pairs();
            let mut parsed_mode: SkylarkMode = SkylarkMode::Sat;
            for param in params {
                debug!("Parsing parameter: {}={}", param.0, param.1);
                if param.0.eq_ignore_ascii_case("mode") {
                    debug!("Parsing mode: {}", param.1);
                    parsed_mode = match SkylarkMode::try_from(param.1.to_string()) {
                        Ok(mode) => mode,
                        Err(e) => {
                            error!("Error parsing SkylarkMode: {}", e.to_string());
                            return Ok(Response::builder()
                                .status(StatusCode::BAD_REQUEST)
                                .body(hyper::Body::from("Not able to parse mode param"))
                                .unwrap());
                        }
                    }
                }
            }
            let whole_body = hyper::body::to_bytes(req.into_body()).await?;
            let str_body = String::from_utf8(whole_body.to_vec()).unwrap();
            debug!(
                "preprocess_handler: Received POST body with length: {}",
                str_body.len()
            );

            let mut hasher = Sha256::new();
            hasher.update(whole_body);
            debug!("Computed hash: {:x}", hasher.finalize());
            match store_state(str_body, env!("CARGO_PKG_NAME").to_string(), parsed_mode).await {
                Ok(key) => {
                    info!("store_state: OK");
                    debug!("store_state: skylark lib result: {:?}", key);
                    Ok(Response::builder()
                        .status(StatusCode::OK)
                        .header("Node-Name", env::var("NODE_NAME").unwrap())
                        .body(Body::from(key))
                        .unwrap())
                }
                Err(e) => {
                    error!(
                        "store_state: Error calling skylark lib store state: {:?}",
                        e
                    );
                    Ok(Response::builder()
                        .status(StatusCode::NOT_FOUND)
                        .header("Node-Name", env::var("NODE_NAME").unwrap())
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
