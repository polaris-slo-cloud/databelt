use hyper::server::conn::Http;
use hyper::service::service_fn;
use hyper::{Body, Method, Request, Response, StatusCode, Uri};
use skylark_lib::{delete_single_state, get_single_state, init_new_chain, skylark_lib_version, start_timing, store_single_state, SkylarkPolicy};
use std::env;
use std::net::SocketAddr;
use std::time::Instant;
use tokio::net::TcpListener;
use url::Url;

extern crate pretty_env_logger;
#[macro_use]
extern crate log;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    pretty_env_logger::init_timed();

    info!(
        "Starting Example Single State Function {}",
        env!("CARGO_PKG_VERSION")
    );
    info!("Skylark library loaded: {}", skylark_lib_version());
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));

    let listener = TcpListener::bind(addr).await?;
    info!("Listening on http://{}", addr);
    loop {
        let (stream, _) = listener.accept().await?;

        tokio::task::spawn(async {
            if let Err(err) = Http::new()
                .serve_connection(stream, service_fn(http_handler))
                .await
            {
                error!("Error serving connection: {:?}", err);
            }
        });
    }
}

fn parse_query_data(
    uri: &Uri,
) -> Result<(SkylarkPolicy, String, String), Box<dyn std::error::Error>> {
    debug!("Parsing URI: {}", uri);
    let mut parsed_policy = SkylarkPolicy::Stateless;
    let mut parsed_destination_node: String = "pi5u1".to_string();
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
        } else if param.0.eq_ignore_ascii_case("destination") {
            debug!("Parsing destination: {}", param.1);
            parsed_destination_node = param.1.to_string();
        } else if param.0.eq_ignore_ascii_case("key") {
            debug!("Parsing key: {}", param.1);
            parsed_key = param.1.to_string();
        }
    }
    Ok((parsed_policy, parsed_destination_node, parsed_key))
}

async fn http_handler(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/get-and-set") => {
            info!("Incoming");
            start_timing().await;
            init_new_chain().await;
            let policy: SkylarkPolicy;
            let dest_node: String;
            let key: String;
            (policy, dest_node, key) = match parse_query_data(req.uri()) {
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
            let state: String = match get_single_state(&key, &policy).await {
                Ok(s) => {
                    info!("get_state: OK");
                    debug!("get_state: found state of length {}", s.len());
                    s
                }
                Err(err) => {
                    error!("get_state: Error fetching predecessor state: {:?}", err);
                    return Ok(Response::builder()
                        .status(StatusCode::NOT_FOUND)
                        .body(Body::from("Error fetching predecessor state"))
                        .unwrap());
                }
            };
            let tdr = timer_tdr.elapsed().as_millis();
            let timer_tdm = Instant::now();
            let new_key = match store_single_state(state, &dest_node, &policy).await {
                Ok(key) => {
                    info!("get-and-set::new state set");
                    debug!("store_state: skylark lib result: {:?}", key);
                    key
                }
                Err(e) => {
                    error!(
                        "store_state: Error calling skylark lib store state: {:?}",
                        e
                    );
                    return Ok(Response::builder()
                        .status(StatusCode::INTERNAL_SERVER_ERROR)
                        .body(Body::from("Error calling skylark lib store state"))
                        .unwrap());
                }
            };
            let tdm = timer_tdm.elapsed().as_millis();
            let result = format!("{:?}\t{:?}", tdr, tdm);
            match delete_single_state(new_key).await {
                Ok(_) => {
                    info!("get-and-set::new state deleted");
                    let tdm = timer_tdm.elapsed().as_millis();
                    info!("\n\tRESULT\n\tT(dr)\t\t{:?}\n\tT(dm)\t\t{:?}", tdr, tdm);
                    Ok(Response::builder()
                        .status(StatusCode::OK)
                        .body(Body::from(result))
                        .unwrap())
                }
                Err(e) => {
                    error!(
                        "get-and-set::delete_single_state: Error calling skylark lib delete state: {:?}",
                        e
                    );
                    Ok(Response::builder()
                        .status(StatusCode::INTERNAL_SERVER_ERROR)
                        .body(Body::from("Error calling skylark lib store state"))
                        .unwrap())
                }
            }
        }
        (&Method::GET, "/health") => Ok(Response::new(Body::from("OK\n"))),
        // Return the 404 Not Found for other routes.
        _ => {
            warn!("bad request {:?}", req.uri());
            Ok(Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(Body::from("Route not found"))
                .unwrap())
        }
    }
}
