use hyper::server::conn::Http;
use hyper::service::service_fn;
use hyper::{Body, Method, Request, Response, StatusCode};
use sha2::{Digest, Sha256};
use skylark_lib::{init_skylark_and_fetch_state, skylark_lib_version, store_state};
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
        // Serve some instructions at /
        (&Method::GET, "/") => {
            let params = Url::parse(&req.uri().to_string()).unwrap();
            let pairs = params.query_pairs();
            let mut key: Option<String> = None;
            for (k, v) in pairs {
                if k == "key" {
                    key = Option::from(v.to_string());
                }
            }
            let state: String;
            match init_skylark_and_fetch_state(
                env!("CARGO_PKG_NAME").to_string(),
                key.unwrap(),
                "Sat",
            )
            .await
            {
                Ok(s) => {
                    state = s;
                    let mut hasher = Sha256::new();
                    hasher.update(state.as_bytes());
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
                                .body(Body::empty())
                                .unwrap())
                        }
                    }
                }
                Err(err) => {
                    error!(
                        "main::http_handler::init_skylark_lib: Error fetching predecessor state: {:?}",
                        err
                    );
                    Ok(Response::builder()
                        .status(StatusCode::INTERNAL_SERVER_ERROR)
                        .body(Body::empty())
                        .unwrap())
                }
            }
        }
        (&Method::GET, "/health") => Ok(Response::new(Body::from("OK"))),
        // Return the 404 Not Found for other routes.
        _ => Ok(Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::empty())
            .unwrap()),
    }
}
