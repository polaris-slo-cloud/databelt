use hyper::server::conn::Http;
use hyper::service::service_fn;
use hyper::{Body, Method, Request, Response, StatusCode};
use sha2::{Digest, Sha256};
use skylark_lib::{skylark_init, skylark_lib_version, store_state};
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
        "Starting Example Image Detector {}",
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
        // Serve some instructions at /
        (&Method::GET, "/") => {
            info!("Incoming request with params: {:?}", req.uri().query());
            let params = Url::parse(&req.uri().to_string()).unwrap();
            let pairs = params.query_pairs();
            let mut key: Option<String> = None;
            for (k, v) in pairs {
                if k == "key" {
                    key = Option::from(v.to_string());
                }
            }
            match key.clone() {
                Some(key) => {
                    debug!("http_handler::/: key param: {}", key);
                }
                None => {
                    warn!("http_handler::/: no key provided");
                }
            }
            let state: String = match skylark_init(env!("CARGO_PKG_NAME").to_string(), key, skylark_lib::SkylarkMode::Sat).await{
                Ok(s) => {
                    info!("http_handler::/: state ok");
                    s
                }
                Err(err) => {
                    error!(
                        "main::http_handler::skylark_init_lib: Error fetching predecessor state: {:?}",
                        err
                    );
                    return Ok(Response::builder()
                        .status(StatusCode::INTERNAL_SERVER_ERROR)
                        .body(Body::from("Error fetching predecessor state"))
                        .unwrap())
                }
            };
            let mut hasher = Sha256::new();
            hasher.update(state.as_bytes());
            let data_hash = format!("{:x}", hasher.finalize());
            info!("http_handler::/: generated data hash, attempting to store");
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
        // Return the 404 Not Found for other routes.
        _ => {
            warn!("http_handler: bad request {:?}", req.uri());
            Ok(Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(Body::from("Route not found"))
                .unwrap())
        },
    }
}
