use hyper::server::conn::Http;
use hyper::service::service_fn;
use hyper::{Body, Method, Request, Response, StatusCode, Uri};
use rand::distributions::Alphanumeric;
use rand::Rng;
use skylark_lib::{
    skylark_lib_version, start_timing, store_bundled_state, store_single_state, SkylarkPolicy,
    SkylarkStorageType,
};
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

    info!("Starting Data Writer {}", env!("CARGO_PKG_VERSION"));
    info!("Skylark library loaded: {}", skylark_lib_version());
    let addr = SocketAddr::from(([0, 0, 0, 0], 8084));

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
) -> Result<(SkylarkPolicy, SkylarkStorageType, String, usize, usize), Box<dyn std::error::Error>> {
    debug!("Parsing URI: {}", uri);
    let mut parsed_policy = SkylarkPolicy::Serverless;
    let mut parsed_storage_type = SkylarkStorageType::Single;
    let mut parsed_destination_node: String = "pi5u1".to_string();
    let mut parsed_size_kb: usize = 100;
    let mut parsed_state_count: usize = 1;
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
        } else if param.0.eq_ignore_ascii_case("size") {
            debug!("Parsing size: {}", param.1);
            parsed_size_kb = match param.1.parse::<usize>() {
                Ok(num) => num,
                Err(e) => {
                    return Err(e.into());
                }
            };
        } else if param.0.eq_ignore_ascii_case("scount") {
            debug!("Parsing scount: {}", param.1);
            parsed_state_count = match param.1.parse::<usize>() {
                Ok(num) => num,
                Err(e) => {
                    return Err(e.into());
                }
            };
        } else if param.0.eq_ignore_ascii_case("stype") {
            debug!("Parsing storage type: {}", param.1);
            debug!("Parsing policy: {}", param.1);
            parsed_storage_type = SkylarkStorageType::try_from(param.1.to_string()).unwrap();
        }
    }
    Ok((
        parsed_policy,
        parsed_storage_type,
        parsed_destination_node,
        parsed_size_kb,
        parsed_state_count,
    ))
}

async fn http_handler(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/single") => {
            info!("Incoming");
            start_timing().await;
            let policy: SkylarkPolicy;
            let dest_node: String;
            let size_kb: usize;
            let state_count: usize;
            let storage_type: SkylarkStorageType;
            (policy, storage_type, dest_node, size_kb, state_count) =
                match parse_workflow_metadata(req.uri()) {
                    Ok(res) => res,
                    Err(e) => {
                        error!("Error parsing URI: {}", e.to_string());
                        return Ok(Response::builder()
                            .status(StatusCode::BAD_REQUEST)
                            .body(hyper::Body::from("Not able to parse URL"))
                            .unwrap());
                    }
                };
            debug!("Parsed state_count: {:?}", state_count);
            let rnd_str = generate_random_data(size_kb);
            match store_single_state(rnd_str, &dest_node, &policy, &storage_type).await {
                Ok(key) => {
                    info!("single_handler::store_state: OK");
                    debug!(
                        "single_handler::store_state: skylark lib result: {:?}",
                        key
                    );
                    Ok(Response::builder()
                        .status(StatusCode::OK)
                        .header("Node-Name", env::var("NODE_NAME").unwrap())
                        .body(Body::from(key))
                        .unwrap())
                }
                Err(e) => {
                    error!(
                        "single_handler::store_state: NOT_FOUND Error calling skylark lib store state: {:?}",
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
        (&Method::GET, "/bundled") => {
            info!("Incoming");
            start_timing().await;
            let policy: SkylarkPolicy;
            let dest_node: String;
            let size_kb: usize;
            let state_count: usize;
            let storage_type: SkylarkStorageType;
            (policy, storage_type, dest_node, size_kb, state_count) =
                match parse_workflow_metadata(req.uri()) {
                    Ok(res) => res,
                    Err(e) => {
                        error!("Error parsing URI: {}", e.to_string());
                        return Ok(Response::builder()
                            .status(StatusCode::BAD_REQUEST)
                            .body(hyper::Body::from("Not able to parse URL"))
                            .unwrap());
                    }
                };
            debug!("Parsed storage_type: {:?}", storage_type);
            let rnd_str = generate_random_data(size_kb);
            let mut bundled_state: Vec<(String, String)> = vec![];
            for n in 1..(state_count + 1) {
                debug!("bundled_handler: Preparing bundle-child-{}", n);
                let fn_name = format!("bundle-child-{}", n);
                bundled_state.push((fn_name.clone(), rnd_str.clone()));
            }

            match store_bundled_state(bundled_state, &dest_node, &policy).await {
                Ok(key) => {
                    info!("bundled_handler::store_state: OK");
                    debug!(
                        "bundled_handler::store_state: skylark lib result: {:?}",
                        key
                    );
                    Ok(Response::builder()
                        .status(StatusCode::OK)
                        .header("Node-Name", env::var("NODE_NAME").unwrap())
                        .body(Body::from(key))
                        .unwrap())
                }
                Err(e) => {
                    error!(
                        "bundled_handler::store_state: NOT_FOUND Error calling skylark lib store state: {:?}",
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

fn generate_random_data(size_kb: usize) -> String {
    debug!("generate_random_data: generating {:?}KB", size_kb);
    let size = size_kb * 1024;
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(size)
        .map(char::from)
        .collect()
}
