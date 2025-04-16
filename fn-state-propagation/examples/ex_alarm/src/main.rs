use hyper::server::conn::Http;
use hyper::service::service_fn;
use hyper::{Body, Method, Request, Response, StatusCode, Uri};
use skylark_manage::{get_single_state, skylark_manage_version, start_timing, SkylarkPolicy};
use std::env;
use std::net::SocketAddr;
use std::time::Instant;
use tokio::net::TcpListener;
use url::Url;

extern crate pretty_env_logger;
#[macro_use]
extern crate log;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    pretty_env_logger::init_timed();

    info!("Starting Example Alarm {}", env!("CARGO_PKG_VERSION"));
    info!("Skylark Manage loaded: {}", skylark_manage_version());
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

fn parse_workflow_metadata(
    uri: &Uri,
) -> Result<(SkylarkPolicy, String), Box<dyn std::error::Error>> {
    debug!("Parsing URI: {}", uri);
    let mut parsed_policy = SkylarkPolicy::Stateless;
    let mut parsed_key: String = "".to_string();
    let request_url = match Url::parse(&format!("http://ex.at{}", uri.to_string())) {
        Ok(url) => url,
        Err(e) => {
            error!("Error parsing URI: {}", e.to_string());
            return Err(e.into());
        }
    };
    let params = request_url.query_pairs();
    for param in params {
        debug!("Parsing parameter: {}={}", param.0, param.1);
        if param.0.eq_ignore_ascii_case("policy") {
            debug!("Parsing policy: {}", param.1);
            parsed_policy = SkylarkPolicy::try_from(param.1.to_string()).unwrap();
        } else if param.0.eq_ignore_ascii_case("key") {
            debug!("Parsing key: {}", param.1);
            parsed_key = param.1.to_string();
        }
    }
    Ok((parsed_policy, parsed_key))
}

async fn http_handler(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => {
            info!("Incoming");
            start_timing().await;
            let policy: SkylarkPolicy;
            let key: String;
            (policy, key) = match parse_workflow_metadata(req.uri()) {
                Ok(res) => res,
                Err(e) => {
                    error!("Error parsing URI: {}", e.to_string());
                    return Ok(Response::builder()
                        .status(StatusCode::BAD_REQUEST)
                        .body(hyper::Body::from("Not able to parse URL"))
                        .unwrap());
                }
            };
            let timer_tdr = Instant::now();
            match get_single_state(&key, &policy).await {
                Ok(s) => {
                    info!("get_state: OK");
                    debug!("get_state: found state of length {}", s.len());
                    let tdr = timer_tdr.elapsed().as_millis();

                    Ok(Response::builder()
                        .status(StatusCode::OK)
                        .body(Body::from(format!("{:?}", tdr)))
                        .unwrap())
                }
                Err(err) => {
                    error!("get_state: Error fetching predecessor state: {:?}", err);
                    Ok(Response::builder()
                        .status(StatusCode::NOT_FOUND)
                        .body(Body::from("Error fetching predecessor state"))
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
        }
    }
}
