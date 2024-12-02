use hyper::server::conn::Http;
use hyper::service::service_fn;
use hyper::{Method, Request, Response, StatusCode};
use std::env;
use std::net::SocketAddr;
use std::path::Path;
use std::sync::OnceLock;
use tokio::fs::File;
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
            info!("main::http_handler::preprocess_handler: incoming");
            // PREPROCESS
            let path = Path::new("image.jpg");
            let eo_file = File::open(path).await.expect("Failed to open image.jpg");

            let client = reqwest::Client::new();
            let preprocess_res = client
                .post(PREPROCESS_URL.get().unwrap())
                .body(reqwest::Body::from(eo_file))
                .send()
                .await;

            let preprocess_key: Option<String>;
            match preprocess_res {
                Ok(res) => {
                    info!("Preprocess Response: {}", res.status());
                    preprocess_key = Some(res.text().await.unwrap()).or_else(|| panic!("Failed to parse preprocess result"));
                }
                Err(err) => {
                    error!("{:?}", err);
                    panic!("Failed to preprocess image.jpg");
                }
            }

            // DETECT
            let detect_key: Option<String>;
            let detect_res = client
                .get(format!("{}/?key={}", DETECT_URL.get().unwrap(), preprocess_key.unwrap()))
                .send()
                .await;

            match detect_res {
                Ok(res) => {
                    info!("Detector Response: {}", res.status());
                    detect_key = Some(res.text().await.unwrap()).or_else(|| panic!("Failed to parse detector result"));
                }
                Err(err) => {
                    error!("{:?}", err);
                    panic!("Failed to do object detection");
                }
            }

            //ALARM
            let alarm_res = client
                .get(format!("{}/?key={}", ALARM_URL.get().unwrap(), detect_key.unwrap()))
                .send()
                .await;

            match alarm_res {
                Ok(res) => {
                    info!("Alarm Response: {}", Some(res.text().await.unwrap()).unwrap());
                }
                Err(err) => {
                    error!("{:?}", err);
                    panic!("Failed to do alarm");
                }
            }

            Ok(Response::new(hyper::Body::from("OK")))
        }
        (&Method::GET, "/health") => Ok(Response::new(hyper::Body::from("OK"))),
        // Return the 404 Not Found for other routes.
        _ => Ok(Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(hyper::Body::empty())
            .unwrap()),
    }
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
    info!(
        "Starting Example Preprocessor {}",
        env!("CARGO_PKG_VERSION")
    );
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
