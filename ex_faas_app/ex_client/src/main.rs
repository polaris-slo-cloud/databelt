use hyper::server::conn::Http;
use hyper::service::service_fn;
use hyper::{Method, Request, Response, StatusCode};
use rand::distributions::Alphanumeric;
use rand::Rng;
use reqwest::Url;
use std::net::SocketAddr;
use std::sync::OnceLock;
use std::time::{Duration, Instant};
use std::{env, usize};
use tokio::net::TcpListener;

extern crate pretty_env_logger;
#[macro_use]
extern crate log;

static PREPROCESS_URL: OnceLock<String> = OnceLock::new();
static DETECT_URL: OnceLock<String> = OnceLock::new();
static ALARM_URL: OnceLock<String> = OnceLock::new();

async fn http_handler(req: Request<hyper::Body>) -> Result<Response<hyper::Body>, hyper::Error> {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => {
            info!("incoming");
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
            let mut parsed_size: usize = 0;
            let mut parsed_mode: String = "Sat".to_string();
            for param in params {
                debug!("Parsing parameter: {}={}", param.0, param.1);
                if param.0.eq_ignore_ascii_case("size") {
                    debug!("Parsing size: {}", param.1);
                    parsed_size = match param.1.parse::<usize>() {
                        Ok(size) => size,
                        Err(e) => {
                            error!("Error parsing Integer: {}", e.to_string());
                            return Ok(Response::builder()
                                .status(StatusCode::BAD_REQUEST)
                                .body(hyper::Body::from("Not able to parse size param"))
                                .unwrap());
                        }
                    }
                } else if param.0.eq_ignore_ascii_case("mode") {
                    debug!("Parsing size: {}", param.1);
                    parsed_mode = param.1.to_string();
                }
            }
            let data = generate_random_data(parsed_size);
            debug!("generated random data");
            let client = reqwest::Client::new();
            let preprocess_url = format!(
                "{}?mode={}",
                PREPROCESS_URL.get().unwrap(),
                parsed_mode.clone()
            );
            info!(
                "BENCHMARK: mode: {} - Finished initializing",
                parsed_mode.clone()
            );
            let start = Instant::now();
            info!("BENCHMARK: started clock at: {:?}", start);
            // PREPROCESS
            let preprocess_res = client
                .post(preprocess_url)
                .body(reqwest::Body::from(data))
                .send()
                .await;
            debug!("POSTed data as body to preprocess");
            let preprocess_key;
            match preprocess_res {
                Ok(res) => {
                    debug!("Preprocess Response: {}", res.status());
                    preprocess_key = res.text().await.unwrap();
                    debug!("Preprocess Key: {}", preprocess_key.clone());
                }
                Err(err) => {
                    error!("{:?}", err);
                    return Ok(Response::builder()
                        .status(StatusCode::INTERNAL_SERVER_ERROR)
                        .body(hyper::Body::from("Failed at Preprocessor step"))
                        .unwrap());
                }
            }
            info!("BENCHMARK: checkpoint PREPROCESS: {:?}", start.elapsed());
            // DETECT
            let detect_key;
            let detect_url = format!(
                "{}/?key={}&mode={}",
                DETECT_URL.get().unwrap(),
                preprocess_key.clone(),
                parsed_mode.clone()
            );
            let detect_res = client.get(detect_url).send().await;

            match detect_res {
                Ok(res) => {
                    debug!("Detector Response: {}", res.status());
                    detect_key = res.text().await.unwrap();
                    debug!("Detector Key: {}", detect_key.clone());
                }
                Err(err) => {
                    error!("{:?}", err);
                    return Ok(Response::builder()
                        .status(StatusCode::INTERNAL_SERVER_ERROR)
                        .body(hyper::Body::from("Failed at Object Detector step"))
                        .unwrap());
                }
            }
            info!("BENCHMARK: checkpoint DETECT: {:?}", start.elapsed());
            //ALARM
            let alarm_url = format!(
                "{}/?key={}&mode={}",
                ALARM_URL.get().unwrap(),
                detect_key.clone(),
                parsed_mode
            );
            let alarm_res = client.get(alarm_url).send().await;

            match alarm_res {
                Ok(res) => {
                    debug!(
                        "Alarm Response: {}",
                        Some(res.text().await.unwrap()).unwrap()
                    );
                }
                Err(err) => {
                    error!("{:?}", err);
                    return Ok(Response::builder()
                        .status(StatusCode::INTERNAL_SERVER_ERROR)
                        .body(hyper::Body::from("Failed at Alarm step"))
                        .unwrap());
                }
            }
            let duration: Duration = start.elapsed();
            info!("BENCHMARK: checkpoint DETECT: {:?}", duration.clone());
            info!("DONE");
            Ok(Response::new(hyper::Body::from(format!(
                "Workflow execution time: {}ms\n",
                duration.as_millis()
            ))))
        }
        (&Method::GET, "/health") => Ok(Response::new(hyper::Body::from("OK\n"))),
        // Return the 404 Not Found for other routes.
        _ => Ok(Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(hyper::Body::from("Route not found"))
            .unwrap()),
    }
}

fn generate_random_data(size_kb: usize) -> String {
    debug!("generate_random_data");
    let size_bytes = size_kb * 1024;
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(size_bytes)
        .map(char::from)
        .collect()
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    pretty_env_logger::init_timed();
    PREPROCESS_URL
        .set(
            env::var("PREPROCESS_URL").unwrap_or(
                "http://skylark-ex-preprocess.default.svc.cluster.local:8080".to_string(),
            ),
        )
        .expect("Error while initializing PREPROCESS_URL");
    DETECT_URL
        .set(
            env::var("DETECT_URL")
                .unwrap_or("http://skylark-ex-detect.default.svc.cluster.local:8080".to_string()),
        )
        .expect("Error while initializing Skylark API url");
    ALARM_URL
        .set(
            env::var("ALARM_URL")
                .unwrap_or("http://skylark-ex-alarm.default.svc.cluster.local:8080".to_string()),
        )
        .expect("Error while initializing Skylark API url");

    info!("Starting Example Client {}", env!("CARGO_PKG_VERSION"));
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
