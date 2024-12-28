use hyper::server::conn::Http;
use hyper::service::service_fn;
use hyper::{Body, Method, Request, Response, StatusCode, Uri};
use sha2::{Digest, Sha256};
use skylark_lib::{get_bundled_state, init_new_chain, skylark_lib_version, start_timing, store_bundled_state, SkylarkPolicy};
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

fn parse_workflow_metadata(
    uri: &Uri,
) -> Result<(SkylarkPolicy, String, String, Vec<String>), Box<dyn std::error::Error>> {
    debug!("Parsing URI: {}", uri);
    let mut parsed_policy = SkylarkPolicy::Skylark;
    let mut parsed_destination_node: String = "pi5u1".to_string();
    let mut parsed_key: String = "".to_string();
    let mut parsed_task_ids: Vec<String> = vec![];
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
        } else if param.0.eq_ignore_ascii_case("tasks") {
            debug!("Parsing tasks: {}", param.1);
            for step in param.1.split(",") {
                parsed_task_ids.push(step.to_string());
            }
        }
    }
    Ok((parsed_policy, parsed_destination_node, parsed_key, parsed_task_ids))
}

async fn http_handler(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/get-and-set") => {
            info!("get-and-set: Incoming");
            start_timing().await;
            init_new_chain().await;
            let timer_tf = Instant::now();
            let policy: SkylarkPolicy;
            let dest_node: String;
            let key: String;
            let mut task_ids: Vec<String> = vec![];
            (policy, dest_node, key, task_ids) = match parse_workflow_metadata(req.uri()) {
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
            let states: Vec<(String, String)> = match get_bundled_state(&key, &policy).await {
                Ok(s) => {
                    info!("get-and-set::get_bundled_state: OK");
                    debug!("get-and-set::get_bundled_state: found state of {} functions", s.len());
                    s
                }
                Err(err) => {
                    error!("get-and-set::get_bundled_state: NOT_FOUND Error fetching predecessor state: {:?}", err);
                    return Ok(Response::builder()
                        .status(StatusCode::NOT_FOUND)
                        .body(Body::from("Error fetching predecessor state"))
                        .unwrap());
                }
            };
            let tdr = timer_tdr.elapsed().as_millis();
            let timer_ex = Instant::now();
            let mut df = 0;
            for state in &states {
                let mut hasher = Sha256::new();
                debug!("get-and-set: hashing state for function {} with length {:?}", &state.0, state.1.len());
                df += state.1.len();
                hasher.update(state.1.as_bytes());
                let data_hash = format!("{:x}", hasher.finalize());
                debug!("get-and-set: generated data hash {}", data_hash);
            }
            let tex = timer_ex.elapsed().as_millis();
            let timer_tdm = Instant::now();
            match store_bundled_state(states, &dest_node, &policy).await {
                Ok(key) => {
                    debug!("get-and-set::store_bundled_state: skylark lib result: {:?}", key);
                    let tf = timer_tf.elapsed().as_millis();
                    let tdm = timer_tdm.elapsed().as_millis();
                    info!("\n\tRESULT\n\tT(f)\t\t{:?}\n\tT(ex)\t\t{:?}\n\tT(dm)\t\t{:?}\n\tT(dr)\t\t{:?}\n\tD(f)\t\t{:?}", tf, tex, tdm, tdr, df);
                    Ok(Response::builder()
                        .status(StatusCode::OK)
                        .body(Body::from(key))
                        .unwrap())
                }
                Err(e) => {
                    error!(
                        "get-and-set::store_bundled_state: Error calling skylark lib store state: {:?}",
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
