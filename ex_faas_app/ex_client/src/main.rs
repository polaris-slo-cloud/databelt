use hyper::server::conn::Http;
use hyper::service::service_fn;
use hyper::{Method, Request, Response, StatusCode, Uri};
use rand::distributions::Alphanumeric;
use rand::Rng;
use reqwest::Url;
use std::collections::VecDeque;
use std::net::SocketAddr;
use std::time::{Duration, Instant};
use std::{env, usize};
use tokio::net::TcpListener;

extern crate pretty_env_logger;
#[macro_use]
extern crate log;

fn parse_workflow_metadata(
    uri: &Uri,
) -> Result<(usize, String, String, VecDeque<String>), Box<dyn std::error::Error>> {
    debug!("Parsing URI: {}", uri);
    let mut parsed_policy = "Skylark".to_string();
    let mut parsed_destination_node: String = "pi5u1".to_string();
    let mut parsed_node_path: VecDeque<String> = VecDeque::new();
    let mut parsed_size_mb: usize = 1;
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
        if param.0.eq_ignore_ascii_case("size_mb") {
            debug!("Parsing size_mb: {}", param.1);
            parsed_size_mb = match param.1.parse::<usize>() {
                Ok(num) => num,
                Err(e) => {
                    return Err(e.into());
                }
            };
        } else if param.0.eq_ignore_ascii_case("destination") {
            debug!("Parsing destination: {}", param.1);
            parsed_destination_node = param.1.to_string();
        } else if param.0.eq_ignore_ascii_case("node_path") {
            debug!("Parsing node_path: {}", param.1);
            for step in param.1.split(",") {
                parsed_node_path.push_back(step.to_string());
            }
        } else if param.0.eq_ignore_ascii_case("policy") {
            debug!("Parsing policy: {}", param.1);
            parsed_policy = param.1.to_string();
        }
    }
    Ok((
        parsed_size_mb,
        parsed_policy,
        parsed_destination_node,
        parsed_node_path,
    ))
}
async fn http_handler(req: Request<hyper::Body>) -> Result<Response<hyper::Body>, hyper::Error> {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => {
            info!("incoming");
            let policy: String;
            let dest_node: String;
            let size_mb: usize;
            let mut node_path: VecDeque<String>;
            (size_mb, policy, dest_node, node_path) = match parse_workflow_metadata(req.uri()) {
                Ok(res) => res,
                Err(e) => {
                    error!("Error parsing URI: {}", e.to_string());
                    return Ok(Response::builder()
                        .status(StatusCode::BAD_REQUEST)
                        .body(hyper::Body::from("Not able to parse params"))
                        .unwrap());
                }
            };

            let data = generate_random_data(size_mb);
            debug!("generated random data");
            let client = reqwest::Client::new();
            let preprocess_url = format!(
                "http://{}-preprocess.default.svc.cluster.local/process?policy={}&destination={}",
                node_path.pop_front().unwrap(),
                policy,
                dest_node
            );
            info!("BENCHMARK: policy: {} - Finished initializing", &policy);
            let start = Instant::now();
            info!("BENCHMARK: started clock at: {:?}", start);
            // PREPROCESS
            let preprocess_result = client
                .post(preprocess_url)
                .body(reqwest::Body::from(data))
                .send()
                .await;
            debug!("POSTed data as body to preprocess");

            let pre_response = match preprocess_result {
                Ok(resp) => {
                    debug!("Preprocess Response Status: {}", resp.status());
                    resp
                }
                Err(err) => {
                    error!("{:?}", err);
                    return Ok(Response::builder()
                        .status(StatusCode::INTERNAL_SERVER_ERROR)
                        .body(hyper::Body::from("Failed at Preprocessor step\n"))
                        .unwrap());
                }
            };
            let preprocess_node = &pre_response
                .headers()
                .get("Node-Name")
                .unwrap()
                .to_str()
                .unwrap()
                .to_string();
            let preprocess_key = pre_response.text().await.unwrap();
            info!("BENCHMARK: checkpoint PREPROCESS: {:?}", start.elapsed());
            // DETECT
            let detect_url = format!(
                "http://{}-detect.default.svc.cluster.local/?key={}&policy={}&destination={}",
                node_path.pop_front().unwrap(),
                preprocess_key,
                policy,
                dest_node
            );
            let detect_result = client.get(detect_url).send().await;
            let detect_response = match detect_result {
                Ok(resp) => {
                    debug!("Detector Response Status: {}", resp.status());
                    resp
                }
                Err(err) => {
                    error!("{:?}", err);
                    return Ok(Response::builder()
                        .status(StatusCode::INTERNAL_SERVER_ERROR)
                        .body(hyper::Body::from("Failed at Object Detector step\n"))
                        .unwrap());
                }
            };
            let detect_node = &detect_response
                .headers()
                .get("Node-Name")
                .unwrap()
                .to_str()
                .unwrap()
                .to_string();
            let detect_key = detect_response.text().await.unwrap();
            info!("BENCHMARK: checkpoint DETECT: {:?}", start.elapsed());
            //ALARM
            let alarm_url = format!(
                "http://{}-alarm.default.svc.cluster.local/?key={}&policy={}&destination={}",
                node_path.pop_front().unwrap(),
                detect_key,
                policy,
                dest_node
            );
            let alarm_res = client.get(alarm_url).send().await;
            let alarm_response = match alarm_res {
                Ok(resp) => {
                    debug!("Alarm Response Status: {}", resp.status());
                    resp
                }
                Err(err) => {
                    error!("{:?}", err);
                    return Ok(Response::builder()
                        .status(StatusCode::INTERNAL_SERVER_ERROR)
                        .body(hyper::Body::from("Failed at Alarm step\n"))
                        .unwrap());
                }
            };
            let alarm_node = &alarm_response
                .headers()
                .get("Node-Name")
                .unwrap()
                .to_str()
                .unwrap()
                .to_string();
            let alarm_text = alarm_response.text().await.unwrap();
            let duration: Duration = start.elapsed();
            debug!("Alarm Response Text: {}", alarm_text);
            info!("BENCHMARK: checkpoint ALARM: {:?}", duration.clone());
            info!("DONE");
            Ok(Response::new(hyper::Body::from(format!(
                "Workflow execution time: {}ms\nNode path: {} -> {} -> {}\n",
                duration.as_millis(),
                preprocess_node,
                detect_node,
                alarm_node
            ))))
        }
        (&Method::GET, "/health") => Ok(Response::new(hyper::Body::from("OK\n"))),
        // Return the 404 Not Found for other routes.
        _ => Ok(Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(hyper::Body::from("Route not found\n"))
            .unwrap()),
    }
}

fn generate_random_data(size_mb: usize) -> String {
    debug!("generate_random_data");
    let size_bytes = size_mb * 1048576;
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(size_bytes)
        .map(char::from)
        .collect()
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    pretty_env_logger::init_timed();

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
