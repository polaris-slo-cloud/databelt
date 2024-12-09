use hyper::server::conn::Http;
use hyper::service::service_fn;
use hyper::{Body, Method, Request, Response, StatusCode, Uri};
use sha2::{Digest, Sha256};
use skylark_lib::{get_state, skylark_lib_version, store_state};
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

    info!("Starting Example Alarm {}", env!("CARGO_PKG_VERSION"));
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

fn parse_key_from_query(uri: &Uri) -> Result<String, Box<dyn std::error::Error>> {
    debug!("Parsing URI: {}", uri);
    let request_url = match Url::parse(&format!("http://skylark.at{}", uri.to_string())) {
        Ok(url) => url,
        Err(e) => {
            error!("Error parsing URI: {}", e.to_string());
            return Err(e.into());
        }
    };
    let params = request_url.query_pairs();
    for param in params {
        debug!("Parsing parameter: {}={}", param.0, param.1);
        if param.0.eq_ignore_ascii_case("key") {
            debug!("Parsing key: {}", param.0);
            return Ok(param.1.to_string());
        }
    }
    error!("Error parsing URI: No key param found");
    Ok(String::new())
}

async fn http_handler(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => {
            info!("Incoming request with params: {:?}", req.uri().query());
            let key = match parse_key_from_query(req.uri()){
                Ok(key) => key,
                Err(e) => {
                    error!("Error parsing key: {}", e);
                    return Ok(Response::builder()
                        .status(StatusCode::BAD_REQUEST)
                        .body(Body::from("Error parsing key param"))
                        .unwrap());
                }
            };
            let state: String = match get_state(
                env!("CARGO_PKG_NAME").to_string(),
                key,
                skylark_lib::SkylarkMode::Sat,
            )
                .await
            {
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
                        .unwrap());
                }
            };
            let mut hasher = Sha256::new();
            hasher.update(state.as_bytes());
            let data_hash = format!("{:x}", hasher.finalize());
            info!("http_handler::/: generated data hash, attempting to store");
            match store_state(
                data_hash,
                env!("CARGO_PKG_NAME").to_string(),
                skylark_lib::SkylarkMode::Sat,
            )
                .await
            {
                Ok(key) => {
                    info!(
                        "main::http_handler::store_state: skylark lib result: {:?}",
                        key
                    );
                    Ok(Response::new(Body::from("ALARM")))
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
