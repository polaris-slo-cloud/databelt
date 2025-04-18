use hyper::server::conn::Http;
use hyper::service::service_fn;
use hyper::{Body, Method, Request, Response, StatusCode, Uri};
use skylark_manage::{
    init_new_chain, skylark_manage_version, start_timing, store_bundled_state, store_single_state,
    SkylarkPolicy,
};
use std::env;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use url::Url;
use std::fmt::Write;

extern crate pretty_env_logger;
#[macro_use]
extern crate log;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    pretty_env_logger::init_timed();

    info!("Starting Data Writer {}", env!("CARGO_PKG_VERSION"));
    info!("Skylark Manage loaded: {}", skylark_manage_version());
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

fn parse_query_data(uri: &Uri) -> Result<(String, usize, usize), Box<dyn std::error::Error>> {
    debug!("Parsing URI: {}", uri);
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
        if param.0.eq_ignore_ascii_case("destination") {
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
        }
    }
    Ok((parsed_destination_node, parsed_size_kb, parsed_state_count))
}

async fn http_handler(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/single") => {
            info!("Incoming");
            init_new_chain().await;
            start_timing().await;
            let dest_node: String;
            let size_kb: usize;
            let state_count: usize;
            (dest_node, size_kb, state_count) = match parse_query_data(req.uri()) {
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
            match store_single_state(rnd_str, &dest_node, &SkylarkPolicy::Stateless).await {
                Ok(key) => {
                    info!("single_handler::store_state: OK");
                    debug!("single_handler::store_state: skylark manage result: {:?}", key);
                    Ok(Response::builder()
                        .status(StatusCode::OK)
                        .header("Node-Name", env::var("NODE_NAME").unwrap())
                        .body(Body::from(key))
                        .unwrap())
                }
                Err(e) => {
                    error!(
                        "single_handler::store_state: NOT_FOUND Error calling skylark manage store state: {:?}",
                        e
                    );
                    Ok(Response::builder()
                        .status(StatusCode::NOT_FOUND)
                        .header("Node-Name", env::var("NODE_NAME").unwrap())
                        .body(Body::from("Error calling skylark manage store state"))
                        .unwrap())
                }
            }
        }
        (&Method::GET, "/bundled") => {
            info!("Incoming");
            start_timing().await;
            init_new_chain().await;
            let dest_node: String;
            let size_kb: usize;
            let state_count: usize;
            (dest_node, size_kb, state_count) = match parse_query_data(req.uri()) {
                Ok(res) => res,
                Err(e) => {
                    error!("Error parsing URI: {}", e.to_string());
                    return Ok(Response::builder()
                        .status(StatusCode::BAD_REQUEST)
                        .body(hyper::Body::from("Not able to parse URL"))
                        .unwrap());
                }
            };
            let rnd_str = generate_random_data(size_kb);
            let mut bundled_state: Vec<(String, String)> = vec![];
            for n in 1..(state_count + 1) {
                debug!("bundled_handler: Preparing bundle-child-{}", n);
                let fn_name = format!("bundle-child-{}", n);
                bundled_state.push((fn_name.clone(), rnd_str.clone()));
            }

            match store_bundled_state(bundled_state, &dest_node, &SkylarkPolicy::Stateless).await {
                Ok(key) => {
                    info!("bundled_handler::store_state: OK");
                    debug!(
                        "bundled_handler::store_state: skylark manage result: {:?}",
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
                        "bundled_handler::store_state: NOT_FOUND Error calling skylark manage store state: {:?}",
                        e
                    );
                    Ok(Response::builder()
                        .status(StatusCode::NOT_FOUND)
                        .header("Node-Name", env::var("NODE_NAME").unwrap())
                        .body(Body::from("Error calling skylark manage store state"))
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
    let mut s = String::with_capacity(size);
    for i in 0..size {
        write!(s, "{}", i % 10).unwrap();
    }
    s
}
