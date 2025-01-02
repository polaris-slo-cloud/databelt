use hyper::server::conn::Http;
use hyper::service::service_fn;
use hyper::{Body, Method, Request, Response, StatusCode, Uri};
use rand::distributions::Alphanumeric;
use rand::Rng;
use skylark_lib::{init_new_chain, skylark_lib_version, start_timing, store_single_state, SkylarkKey, SkylarkPolicy};
use std::env;
use std::fs::File;
use std::io::Read;
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

fn parse_workflow_metadata(
    uri: &Uri,
) -> Result<(SkylarkPolicy, String, String), Box<dyn std::error::Error>> {
    debug!("Parsing URI: {}", uri);
    let mut parsed_policy = SkylarkPolicy::Skylark;
    let mut parsed_destination_node: String = "pi5u1".to_string();
    let mut parsed_image_name: String = "eo-2K.jpeg".to_string();
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
        } else if param.0.eq_ignore_ascii_case("img") {
            debug!("Parsing img: {}", param.1);
            parsed_image_name = param.1.to_string();
        }
    }
    Ok((parsed_policy, parsed_destination_node, parsed_image_name))
}

async fn http_handler(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => {
            info!("Incoming");
            start_timing().await;
            init_new_chain().await;
            let timer_tf = Instant::now();
            let policy: SkylarkPolicy;
            let dest_node: String;
            let img_name: String;
            (policy, dest_node, img_name) = match parse_workflow_metadata(req.uri()) {
                Ok(res) => res,
                Err(e) => {
                    error!("Error parsing URI: {}", e.to_string());
                    return Ok(Response::builder()
                        .status(StatusCode::BAD_REQUEST)
                        .body(hyper::Body::from("Not able to parse URL"))
                        .unwrap());
                }
            };
            debug!("preprocess_handler: Opening file");
            let mut file = File::open(img_name).expect("Failed to open image file.");
            let mut img_buffer = Vec::new();
            debug!("preprocess_handler: Reading image");
            file.read_to_end(&mut img_buffer)
                .expect("Failed to read image file.");
            debug!(
                "preprocess_handler: Image was read and has length: {}",
                img_buffer.len()
            );

            // let mut hasher = Sha256::new();
            // hasher.update(&img_buffer);
            let rnd_str = generate_random_data(img_buffer.len());
            // debug!("preprocess_handler: Computed hash: {:x}", hasher.finalize());
            let tf = timer_tf.elapsed().as_millis();
            let timer_tdm = Instant::now();
            match store_single_state(rnd_str, &dest_node, &policy)
                .await
            {
                Ok(key) => {
                    debug!(
                        "preprocess_handler::store_state: skylark lib result: {:?}",
                        key
                    );
                    let tdm = timer_tdm.elapsed().as_millis();
                    let s_key = SkylarkKey::try_from(key).unwrap();
                    Ok(Response::builder()
                        .status(StatusCode::OK)
                        .body(Body::from(format!("{}\t{:?}\t{:?}\t{}", s_key.to_string(), tdm, tf, s_key.node_id())))
                        .unwrap())
                }
                Err(e) => {
                    error!(
                        "preprocess_handler::store_state: NOT_FOUND Error calling skylark lib store state: {:?}",
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

fn generate_random_data(size: usize) -> String {
    debug!("generate_random_data");
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(size)
        .map(char::from)
        .collect()
}
