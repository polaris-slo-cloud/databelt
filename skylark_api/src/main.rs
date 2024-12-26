mod error;
mod http_service;
#[allow(dead_code)]
mod model;
mod policy;

use crate::error::{QueryParseError, SkylarkTopologyError};
use crate::http_service::get_from_url;
use crate::model::{NodeGraph, SkylarkNode, SkylarkNodeMap, SkylarkPolicy, SkylarkSLOs};
use crate::policy::{apply_random_policy, apply_skylark_policy, build_graph};
use hyper::server::conn::Http;
use hyper::service::service_fn;
use hyper::{Body, Error, Method, Request, Uri};
use hyper::{Response, StatusCode};
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::{env, fs};
use std::net::SocketAddr;
use std::string::ToString;
use std::sync::{Mutex};
use std::time::{Duration, Instant};
use tokio::net::TcpListener;
use url::Url;

extern crate pretty_env_logger;
#[macro_use]
extern crate log;

lazy_static! {
    static ref NODE_GRAPH: Mutex<NodeGraph> = Mutex::new(NodeGraph::new(vec![]));
    static ref NODE_MAP: Mutex<SkylarkNodeMap> = Mutex::new(HashMap::new());
    static ref NEIGHBORS: Mutex<Vec<String>> = Mutex::new(vec![]);
    static ref LOCAL_NODE: Mutex<SkylarkNode> = Mutex::new(SkylarkNode::default());
    static ref OBJECTIVES: Mutex<SkylarkSLOs> = Mutex::new(SkylarkSLOs::default());
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    pretty_env_logger::init_timed();
    let node_info_url = format!(
        "http://{}:{}",
        env::var("LOCAL_NODE_HOST").expect("LOCAL_NODE_HOST not provided"),
        env::var("NODE_INFO_PORT").expect("NODE_INFO_PORT not provided")
    );
    match init(&node_info_url).await {
        Ok(_) => {
            info!("main:: Skylark is done initializing")
        }
        Err(e) => {
            error!("main: Error while initializing skylark: {:?}", e);
        }
    }
    info!("Starting Skylark API {}", env!("CARGO_PKG_VERSION"));

    let addr = SocketAddr::from(([0, 0, 0, 0], 8081));
    let listener = TcpListener::bind(addr).await?;
    info!("Listening on http://{}", addr);

    let refresh_rate = env::var("NODE_REFRESH_INTERVAL_SECS")
        .unwrap()
        .parse::<u64>()
        .unwrap();
    info!(
        "Starting node graph handler with refresh rate of {}sec",
        refresh_rate
    );
    tokio::spawn(async move {
        loop {
            if let Err(err) = node_graph_handler(&node_info_url).await {
                error!("Error monitoring node graph: {:?}", err);
            }
            tokio::time::sleep(Duration::from_secs(refresh_rate)).await;
        }
    });
    debug!("spawned node graph thread");
    loop {
        let (stream, _) = listener.accept().await?;

        tokio::spawn(async {
            if let Err(err) = Http::new()
                .serve_connection(stream, service_fn(http_handler))
                .await
            {
                error!("Error serving connection: {:?}", err);
            }
        });
    }
}

async fn node_graph_handler(node_info_url: &String) -> Result<(), SkylarkTopologyError> {
    match get_from_url::<NodeGraph>(&format!("{}/{}", node_info_url, "current-topology").as_str()).await
    {
        Err(e) => {
            error!("node_graph_handler: failed to get node graph!\n {:?}", e);
            return Err(SkylarkTopologyError.into());
        }
        Ok(graph) => {
            let mut node_graph = NODE_GRAPH.lock().unwrap();
            node_graph.set_edges(graph.edges().clone());
            debug!("node_graph_handler: NodeGraph updated");
            let mut neighbor_nodes = NEIGHBORS.lock().unwrap();
            neighbor_nodes.clear();
            let local_node = LOCAL_NODE.lock().unwrap();
            let mut node_map = NODE_MAP.lock().unwrap();
            for edge in graph.edges().clone() {
                if !node_map.contains_key(edge.source().node_name()) {
                    node_map.insert(edge.source().node_name().to_string(), edge.source().clone());
                }
                if !node_map.contains_key(edge.target().node_name()) {
                    node_map.insert(edge.target().node_name().to_string(), edge.target().clone());
                }
                if local_node.node_name().eq(edge.target().node_name())
                    && !neighbor_nodes.contains(&edge.source().node_name().to_string())
                {
                    neighbor_nodes.insert(0, edge.source().node_name().to_string());
                } else if local_node.node_name().eq(edge.source().node_name())
                    && !neighbor_nodes.contains(&edge.target().node_name().to_string())
                {
                    neighbor_nodes.insert(0, edge.target().node_name().to_string());
                }
            }
            debug!("node_graph_handler: NodeMap updated");
        }
    }
    Ok(())
}

fn parse_from_query(
    uri: &Uri,
) -> Result<(i16, i16, SkylarkPolicy, String), Box<dyn std::error::Error>> {
    debug!("Parsing URI: {}", uri);
    let mut parsed_size = 0i16;
    let mut parsed_time = 0i16;
    let mut parsed_policy = SkylarkPolicy::Skylark;
    let mut parsed_destination_node: String = "".to_string();
    let request_url = match Url::parse(&format!("http://skylark.at{}", uri.to_string())) {
        Ok(url) => url,
        Err(e) => {
            error!("Error parsing URI: {}", e.to_string());
            return Err(QueryParseError.into());
        }
    };
    let params = request_url.query_pairs();
    for param in params {
        debug!("Parsing parameter: {}={}", param.0, param.1);
        if param.0.eq_ignore_ascii_case("size") {
            debug!("Parsing size: {}", param.1);
            parsed_size = match param.1.parse::<i16>() {
                Ok(size) => size,
                Err(_) => return Err(QueryParseError.into()),
            }
        } else if param.0.eq_ignore_ascii_case("time") {
            debug!("Parsing time: {}", param.1);
            parsed_time = match param.1.parse::<i16>() {
                Ok(time) => time,
                Err(_) => return Err(QueryParseError.into()),
            }
        } else if param.0.eq_ignore_ascii_case("policy") {
            debug!("Parsing policy: {}", param.1);
            parsed_policy = match SkylarkPolicy::try_from(param.1.to_string()) {
                Ok(policy) => policy,
                Err(_) => return Err(QueryParseError.into()),
            };
        } else if param.0.eq_ignore_ascii_case("destination") {
            debug!("Parsing destination: {}", param.1);
            parsed_destination_node = param.1.to_string();
        }
    }
    Ok((
        parsed_size,
        parsed_time,
        parsed_policy,
        parsed_destination_node,
    ))
}

async fn http_handler(req: Request<Body>) -> Result<Response<Body>, Error> {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/storage-node") => {
            info!("GET_STATE: incoming");
            let params = match parse_from_query(&req.uri()) {
                Ok(p) => p,
                Err(e) => {
                    info!("GET_STATE: BAD_REQUEST expected params 'size' and 'time', 'policy' and 'destination'");
                    return Ok(Response::builder()
                        .status(StatusCode::BAD_REQUEST)
                        .body(Body::from(e.to_string()))
                        .unwrap())
                    .into();
                }
            };
            elect_storage_node(params.0, params.1, params.2, params.3).await
        }
        (&Method::GET, "/benchmark") => {
            info!("BENCHMARK: incoming");
            let params = match parse_from_query(&req.uri()) {
                Ok(p) => p,
                Err(e) => {
                    info!("BENCHMARK: BAD_REQUEST expected params 'size' and 'time', 'policy' and 'destination'");
                    return Ok(Response::builder()
                        .status(StatusCode::BAD_REQUEST)
                        .body(Body::from(e.to_string()))
                        .unwrap())
                        .into();
                }
            };
            benchmark(params.0, params.1).await
        }
        (&Method::GET, "/neighbors") => {
            info!("GET_NEIGHBORS: incoming");
            let neighbors = NEIGHBORS.lock().unwrap().clone();
            Ok(Response::builder()
                .status(StatusCode::OK)
                .body(Body::from(serde_json::to_string(&neighbors).unwrap()))
                .unwrap())
        }
        (&Method::GET, "/health") => Ok(Response::new(Body::from("OK\n"))),
        // Return the 404 Not Found for other routes.
        _ => Ok(Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::from("Route not found"))
            .unwrap()),
    }
}

async fn elect_storage_node(
    size: i16,
    time: i16,
    policy: SkylarkPolicy,
    destination_node: String,
) -> Result<Response<Body>, Error> {
    info!("elect_storage_node: Incoming");
    let timer = Instant::now();
    let start_node = LOCAL_NODE.lock().unwrap().node_name().to_string();
    let node_graph = NODE_GRAPH.lock().unwrap().clone();
    let slo = OBJECTIVES.lock().unwrap().clone();
    let graph = build_graph(&node_graph);
    let node_map = NODE_MAP.lock().unwrap();
    match policy {
        SkylarkPolicy::Skylark => {
            match apply_skylark_policy(&start_node, &destination_node, size, time, &graph, &slo) {
                Some(node_name) => {
                    info!("elect_storage_node::SkylarkPolicy: OK elected node for state propagation {}", &node_name);
                    let node = node_map.get(&node_name).unwrap();
                    Ok(Response::builder()
                        .status(StatusCode::OK)
                        .header("policy-execution-time", timer.elapsed().as_millis().to_string())
                        .body(Body::from(node.node_ip().to_string()))
                        .unwrap())
                }
                None => {
                    info!(
                        "elect_storage_node::SkylarkPolicy: OK No Node elected for given input, returning local node"
                    );
                    Ok(Response::builder()
                        .status(StatusCode::OK)
                        .body(Body::from(LOCAL_NODE.lock().unwrap().node_ip().to_string()))
                        .unwrap())
                }
            }
        }
        SkylarkPolicy::Random => {
            match apply_random_policy(&start_node, &destination_node, &graph) {
                Some(node_name) => {
                    info!("elect_storage_node::RandomPolicy: OK elected node for state propagation {}", &node_name);
                    let node = node_map.get(&node_name).unwrap();
                    Ok(Response::builder()
                        .status(StatusCode::OK)
                        .header("policy-execution-time", timer.elapsed().as_millis().to_string())
                        .body(Body::from(node.node_ip().to_string()))
                        .unwrap())
                }
                None => {
                    error!(
                        "elect_storage_node::RandomPolicy: ERROR No Node elected for given input"
                    );
                    Ok(Response::builder()
                        .status(StatusCode::INTERNAL_SERVER_ERROR)
                        .body(Body::from("RandomPolicy: No Node elected for given input"))
                        .unwrap())
                }
            }
        }
        SkylarkPolicy::Serverless => {
            info!(
                "elect_storage_node::ServerlessPolicy: OK returning destination node host {}",
                destination_node
            );
            let node_map = NODE_MAP.lock().unwrap();
            match node_map.get(&destination_node) {
                Some(node) => {
                    let ip = node.node_ip().to_string();
                    Ok(Response::builder()
                        .status(StatusCode::OK)
                        .header("policy-execution-time", timer.elapsed().as_millis().to_string())
                        .body(Body::from(ip))
                        .unwrap())
                },
                None => {
                    Ok(Response::builder()
                        .status(StatusCode::NOT_FOUND)
                        .body(Body::from("ServerlessPolicy: IP of destination node not found"))
                        .unwrap())
                }
            }
        }
    }
}

async fn benchmark(
    size: i16,
    time: i16,
) -> Result<Response<Body>, Error> {
    info!("benchmark: initializing");
    let slo = OBJECTIVES.lock().unwrap().clone();
    let graph_10_file = fs::read_to_string("graph_10.json").unwrap();
    let start_10 = "Node1".to_string();
    let destination_10 = "Node6".to_string();
    let graph_10: HashMap<String, Vec<(String, i16)>> = serde_json::from_str(&graph_10_file).unwrap();
    let graph_100_file = fs::read_to_string("graph_100.json").unwrap();
    let start_100 = "Node51".to_string();
    let destination_100 = "Node58".to_string();
    let graph_100: HashMap<String, Vec<(String, i16)>> = serde_json::from_str(&graph_100_file).unwrap();
    let graph_1000_file = fs::read_to_string("graph_1000.json").unwrap();
    let start_1000 = "Node18".to_string();
    let destination_1000 = "Node28".to_string();
    let graph_1000: HashMap<String, Vec<(String, i16)>> = serde_json::from_str(&graph_1000_file).unwrap();
    let graph_10000_file = fs::read_to_string("graph_10000.json").unwrap();
    let start_10000 = "Node3615".to_string();
    let destination_10000 = "Node5807".to_string();
    let graph_10000: HashMap<String, Vec<(String, i16)>> = serde_json::from_str(&graph_10000_file).unwrap();
    info!("benchmark: initializing done, starting...");
    let mut timer = Instant::now();
    apply_skylark_policy(&start_10, &destination_10, size, time, &graph_10, &slo);
    info!("benchmark::SKYLARK: graph_10 done: {}ms", timer.elapsed().as_millis());
    timer = Instant::now();
    apply_skylark_policy(&start_100, &destination_100, size, time, &graph_100, &slo);
    info!("benchmark::SKYLARK: graph_100 done: {}ms", timer.elapsed().as_millis());
    timer = Instant::now();
    apply_skylark_policy(&start_1000, &destination_1000, size, time, &graph_1000, &slo);
    info!("benchmark::SKYLARK: graph_1000 done: {}ms", timer.elapsed().as_millis());
    timer = Instant::now();
    apply_skylark_policy(&start_10000, &destination_10000, size, time, &graph_10000, &slo);
    info!("benchmark::SKYLARK: graph_10000 done: {}ms", timer.elapsed().as_millis());

    timer = Instant::now();
    apply_random_policy(&start_10, &destination_10, &graph_10);
    info!("benchmark::RANDOM: graph_10 done: {}ms", timer.elapsed().as_millis());
    timer = Instant::now();
    apply_random_policy(&start_100, &destination_100, &graph_100);
    info!("benchmark::RANDOM: graph_100 done: {}ms", timer.elapsed().as_millis());
    timer = Instant::now();
    apply_random_policy(&start_1000, &destination_1000, &graph_1000);
    info!("benchmark::RANDOM: graph_1000 done: {}ms", timer.elapsed().as_millis());
    timer = Instant::now();
    apply_random_policy(&start_10000, &destination_10000, &graph_10000);
    info!("benchmark::RANDOM: graph_10000 done: {}ms", timer.elapsed().as_millis());

    Ok(Response::builder()
        .status(StatusCode::OK)
        .body(Body::from("Benchmark OK\n"))
        .unwrap())
}

async fn init(node_info_url: &String) -> Result<(), Box<dyn std::error::Error>> {
    match get_from_url::<SkylarkNode>(&format!("{}/{}", node_info_url, "local-node-info").as_str())
        .await
    {
        Err(e) => {
            error!("skylark::init: failed to get local node info!\n {:?}", e);
            return Err(e);
        }
        Ok(node) => {
            info!("skylark::init: successfully fetched node info");
            debug!("skylark::init: {:?}", serde_json::to_string(&node).unwrap());
            LOCAL_NODE.lock().unwrap().clone_from(&node);
        }
    }
    match get_from_url::<SkylarkSLOs>(&format!("{}/{}", node_info_url, "objectives").as_str())
        .await
    {
        Err(e) => {
            error!("skylark::init: failed to get objectives!\n {:?}", e);
            return Err(e);
        }
        Ok(objectives) => {
            info!("skylark::init: successfully fetched objectives");
            OBJECTIVES.lock().unwrap().clone_from(&objectives);
        }
    }
    info!("skylark::init: Finished initializing");
    Ok(())
}
