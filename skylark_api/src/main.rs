mod error;
mod http_service;
mod mechanisms;
#[allow(dead_code)]
mod model;
mod redis_client;

use crate::error::QueryParseError;
use crate::http_service::get_from_url;
use crate::mechanisms::{compute_viable_nodes, get_closest_viable_node};
use crate::model::{NodeGraph, SkylarkKey, SkylarkMode, SkylarkNode, SkylarkSLOs, SkylarkState};
use crate::redis_client::{
    del_global_state, del_local_state, get_global_state, get_state_by_url, store_global_state,
    store_local_state, store_state_by_url,
};
use hyper::server::conn::Http;
use hyper::service::service_fn;
use hyper::{Body, Error, Method, Request, Uri};
use hyper::{Response, StatusCode};
use lazy_static::lazy_static;
use std::env;
use std::net::SocketAddr;
use std::string::ToString;
use std::sync::Mutex;
use tokio::net::TcpListener;
use url::Url;

extern crate pretty_env_logger;
#[macro_use]
extern crate log;
// type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
//Result<T, Box<dyn std::error::Error + Send + Sync>>

const NODE_INFO_PORT: &'static str = "8080";
lazy_static! {
    static ref VIABLE_NODE: Mutex<SkylarkNode> = Mutex::new(SkylarkNode::default());
    static ref NODE_GRAPH: Mutex<NodeGraph> = Mutex::new(NodeGraph::new(vec![]));
    static ref LOCAL_NODE: Mutex<SkylarkNode> = Mutex::new(SkylarkNode::default());
    static ref CLOUD_NODE: Mutex<SkylarkNode> = Mutex::new(SkylarkNode::default_cloud());
    static ref OBJECTIVES: Mutex<SkylarkSLOs> = Mutex::new(SkylarkSLOs::default());
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    pretty_env_logger::init_timed();
    match init().await {
        Ok(_) => {
            info!("main:: Skylark is done initializing")
        }
        Err(e) => {
            error!("main: Error while initializing skylark: {:?}", e);
        }
    }
    info!("Starting Skylark API {}", env!("CARGO_PKG_VERSION"));

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

fn parse_from_query(uri: &Uri) -> Result<(SkylarkKey, SkylarkMode), Box<dyn std::error::Error>> {
    debug!("Parsing URI: {}", uri);
    let mut parsed_key = SkylarkKey::default();
    let mut parsed_mode = SkylarkMode::Sat;
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
        if param.0.eq_ignore_ascii_case("key") {
            debug!("Parsing key: {}", param.0);
            parsed_key = match SkylarkKey::try_from(param.1.to_string()) {
                Ok(key) => key,
                Err(_) => return Err(QueryParseError.into()),
            }
        } else if param.0.eq_ignore_ascii_case("mode") {
            debug!("Parsing mode: {}", param.0);
            parsed_mode = match SkylarkMode::try_from(param.1.to_string()) {
                Ok(mode) => mode,
                Err(_) => return Err(QueryParseError.into()),
            };
        }
    }

    match parsed_key.fn_name() {
        "unknown" => Err(QueryParseError.into()),
        _ => Ok((parsed_key, parsed_mode)),
    }
}

async fn http_handler(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    match (req.method(), req.uri().path()) {
        // Serve some instructions at /
        (&Method::GET, "/state/") => {
            info!("main::http_handler::GET_STATE: incoming");
            let params = match parse_from_query(&req.uri()) {
                Ok(p) => p,
                Err(e) => {
                    info!("main::http_handler::GET_STATE: expected params 'key' and 'value'");
                    return Ok(Response::builder()
                        .status(StatusCode::BAD_REQUEST)
                        .body(Body::from(e.to_string()))
                        .unwrap())
                    .into();
                }
            };
            fetch_state_with_strategy(params.0, params.1).await
        }
        (&Method::DELETE, "/state") => {
            info!("main::http_handler::DELETE_STATE: incoming");
            let params = match parse_from_query(&req.uri()) {
                Ok(p) => p,
                Err(e) => {
                    info!("main::http_handler::DELETE_STATE: expected params 'key' and 'value'");
                    return Ok(Response::builder()
                        .status(StatusCode::BAD_REQUEST)
                        .body(Body::from(e.to_string()))
                        .unwrap())
                    .into();
                }
            };
            delete_state(params.0, params.1).await
        }
        (&Method::POST, "/save/sat") => {
            info!("main::http_handler::SAVE_SAT: incoming");
            let whole_body = hyper::body::to_bytes(req.into_body()).await?;
            debug!(
                "main::http_handler::SAVE_SAT body: {:?}",
                &whole_body.to_vec()
            );
            let state: SkylarkState = serde_json::from_slice(&whole_body.to_vec()).unwrap();
            let mut err = false;
            let mut err_msg = "None";
            let mut propagated_node_name: String = "None".to_string();
            let viable_node = VIABLE_NODE.lock().unwrap().clone();
            if viable_node.node_name() != "unknown" {
                match store_state_by_url(&state, viable_node.redis_host().to_string()).await {
                    Ok(_) => {
                        propagated_node_name = viable_node.node_name().to_string();
                        info!(
                            "main::http_handler::SAVE_SAT: successfully stored state to node {}",
                            propagated_node_name
                        )
                    }
                    Err(e) => {
                        error!(
                            "main::http_handler::SAVE_SAT: Error propagating state: {:?}",
                            e
                        );
                        err = true;
                        err_msg = "Error propagating state";
                    }
                }
                match store_global_state(&state).await {
                    Ok(_) => {
                        info!(
                            "main::http_handler::SAVE_SAT: successfully stored state to cloud node"
                        )
                    }
                    Err(e) => {
                        error!(
                            "main::http_handler::SAVE_SAT: Error saving global state: {:?}",
                            e
                        );
                        err = true;
                        err_msg = "Error saving global state";
                    }
                }
            } else {
                warn!("main::http_handler::SAVE_SAT: No viable node found");
                err = true;
                err_msg = "No viable node found";
            }
            if err {
                Ok(Response::builder()
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .body(Body::from(err_msg))
                    .unwrap())
            } else {
                info!("main::http_handler::SAVE_SAT: successfully propagated state to viable node- and global store");
                Ok(Response::new(Body::from(format!(
                    "Successfully stored state on node {}",
                    propagated_node_name
                ))))
            }
        }
        (&Method::POST, "/save/edge") => {
            info!("main::http_handler::SAVE_EDGE: incoming");
            let whole_body = hyper::body::to_bytes(req.into_body()).await?;
            let state: SkylarkState = serde_json::from_slice(&whole_body.to_vec()).unwrap();
            match store_local_state(&state).await {
                Ok(res) => {
                    info!("main::http_handler::SAVE_EDGE: Saved edge state: {:?}", res);
                    Ok(Response::new(Body::from("OK\n")))
                }
                Err(e) => {
                    error!(
                        "main::http_handler::SAVE_EDGE: Error saving local state: {:?}",
                        e
                    );
                    Ok(Response::builder()
                        .status(StatusCode::INTERNAL_SERVER_ERROR)
                        .body(Body::from(e.to_string()))
                        .unwrap())
                }
            }
        }
        (&Method::POST, "/save/cloud") => {
            info!("main::http_handler::SAVE_CLOUD: incoming");
            let whole_body = hyper::body::to_bytes(req.into_body()).await?;
            let state: SkylarkState = serde_json::from_slice(&whole_body.to_vec()).unwrap();
            match store_global_state(&state).await {
                Ok(res) => {
                    info!("main::http_handler::SAVE_CLOUD: Saved sat state: {:?}", res);
                    Ok(Response::new(Body::from("OK\n")))
                }
                Err(e) => {
                    error!(
                        "main::http_handler::SAVE_CLOUD: Error saving global state: {:?}",
                        e
                    );
                    Ok(Response::builder()
                        .status(StatusCode::INTERNAL_SERVER_ERROR)
                        .body(Body::from(e.to_string()))
                        .unwrap())
                }
            }
        }
        (&Method::GET, "/refresh") => {
            info!("main::http_handler::REFRESH: incoming");
            match init().await {
                Ok(_) => {
                    debug!("main::http_handler::REFRESH: successfully refreshed");
                    Ok(Response::new(Body::from("OK\n")))
                }
                Err(e) => {
                    error!("main::http_handler::REFRESH: refresh failed");
                    Ok(Response::builder()
                        .status(StatusCode::INTERNAL_SERVER_ERROR)
                        .body(Body::from(e.to_string()))
                        .unwrap())
                }
            }
        }
        (&Method::GET, "/health") => Ok(Response::new(Body::from("OK\n"))),
        // Return the 404 Not Found for other routes.
        _ => Ok(Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::from("Route not found"))
            .unwrap()),
    }
}

async fn delete_state(key: SkylarkKey, mode: SkylarkMode) -> Result<Response<Body>, Error> {
    let mut err = false;
    if mode != SkylarkMode::Cloud {
        match del_local_state(&key).await {
            Ok(_) => {
                info!("main::http_handler::DELETE_STATE: successfully deleted local state");
            }
            Err(e) => {
                warn!(
                    "main::http_handler::DELETE_STATE: Error deleting local state: {:?}",
                    e
                );
                err = true;
            }
        }
    }

    match del_global_state(&key).await {
        Ok(_) => {
            info!("main::http_handler::DELETE_STATE: successfully deleted global state");
        }
        Err(e) => {
            warn!(
                "main::http_handler::DELETE_STATE: Error deleting global state: {:?}",
                e
            );
            return Ok(Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::from("Error deleting global state"))
                .unwrap());
        }
    }
    if err {
        Ok(Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(Body::from("Error deleting local state"))
            .unwrap())
    } else {
        Ok(Response::new(Body::from("Successfully deleted state")))
    }
}

async fn fetch_state_with_strategy(
    key: SkylarkKey,
    mode: SkylarkMode,
) -> Result<Response<Body>, Error> {
    if mode == SkylarkMode::Sat {
        let closest_node = get_closest_viable_node(
            &LOCAL_NODE.lock().unwrap(),
            &NODE_GRAPH.lock().unwrap(),
            &OBJECTIVES.lock().unwrap(),
        );
        if closest_node.is_some() {
            match get_state_by_url(&key, closest_node.unwrap().redis_host()).await {
                Ok(state) => {
                    info!(
                        "fetch_state_with_strategy::Sat: closest node result: {:?}",
                        state.clone()
                    );
                    return Ok(Response::new(Body::from(state)));
                }
                Err(e) => {
                    warn!(
                            "fetch_state_with_strategy::Sat: Error getting state from closest node: {:?}",
                            e
                        );
                }
            }
        }
        info!("fetch_state_with_strategy::Sat: Fallback to cloud");
    }
    match get_global_state(&key).await {
        Ok(state) => {
            info!(
                "fetch_state_with_strategy: global redis result: {:?}",
                state.clone()
            );
            Ok(Response::new(Body::from(state)))
        }
        Err(e) => {
            error!(
                "fetch_state_with_strategy: Error fetching global and local state: {:?}",
                e
            );
            Ok(Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(Body::from("Error fetching global and local state"))
                .unwrap())
        }
    }
}

async fn init() -> Result<(), Box<dyn std::error::Error>> {
    let viable_nodes: Vec<SkylarkNode>;
    let local_node_host = env::var("LOCAL_NODE_HOST").unwrap_or("nonProvided".to_string());
    debug!("skylark::init: Local node host: {}", local_node_host);
    let node_info_port = env::var("NODE_INFO_PORT").unwrap_or(NODE_INFO_PORT.to_string());
    debug!(
        "skylark::init: Local Node Info Service Port: {}",
        node_info_port
    );
    match get_from_url::<SkylarkNode>(
        &format!(
            "http://{}:{}/{}",
            local_node_host, node_info_port, "local-node-info"
        )
        .as_str(),
    )
    .await
    {
        Err(e) => {
            warn!("skylark::init: failed to get local node info!\n {:?}", e);
            info!("skylark::init: moving on with default");
        }
        Ok(node) => {
            info!("skylark::init: successfully fetched node info");
            debug!("skylark::init: {:?}", serde_json::to_string(&node).unwrap());
            let mut local_node = LOCAL_NODE.lock().unwrap();
            local_node.set_node_ip(node.node_ip().to_string());
            local_node.set_node_name(node.node_name().to_string());
            local_node.set_redis_host(node.redis_host().to_string());
            local_node.set_node_type(node.node_type().clone());
        }
    }
    match get_from_url::<SkylarkNode>(
        &format!(
            "http://{}:{}/{}",
            local_node_host, node_info_port, "cloud-node-info"
        )
        .as_str(),
    )
    .await
    {
        Err(e) => {
            warn!("skylark::init: failed to get cloud node info!\n {:?}", e);
            info!("skylark::init: moving on with default");
        }
        Ok(node) => {
            info!("skylark::init: successfully fetched node info");
            let mut cloud_node = CLOUD_NODE.lock().unwrap();
            cloud_node.set_node_ip(node.node_ip().to_string());
            cloud_node.set_node_name(node.node_name().to_string());
            cloud_node.set_redis_host(node.redis_host().to_string());
            cloud_node.set_node_type(node.node_type().clone());
        }
    }
    match get_from_url::<SkylarkSLOs>(
        &format!(
            "http://{}:{}/{}",
            local_node_host, node_info_port, "objectives"
        )
        .as_str(),
    )
    .await
    {
        Err(e) => {
            warn!("skylark::init: failed to get objectives!\n {:?}", e);
            info!("skylark::init: moving on with default");
        }
        Ok(objectives) => {
            info!("skylark::init: successfully fetched objectives");
            let mut o = OBJECTIVES.lock().unwrap();
            o.set_latency_metric(objectives.latency_metric().to_string());
            o.set_bandwidth_metric(objectives.bandwidth_metric().to_string());
            o.set_max_latency(objectives.max_latency().clone());
            o.set_min_bandwidth(objectives.min_bandwidth().clone());
        }
    }
    match get_from_url::<NodeGraph>(
        &format!(
            "http://{}:{}/{}",
            local_node_host, node_info_port, "node-graph"
        )
        .as_str(),
    )
    .await
    {
        Err(e) => {
            error!("skylark::init: failed to get node graph!\n {:?}", e);
        }
        Ok(graph) => {
            info!("skylark::init: successfully fetched node graph");
            let mut node_graph = NODE_GRAPH.lock().unwrap();
            node_graph.set_edges(graph.edges().clone());
            viable_nodes = compute_viable_nodes(
                &LOCAL_NODE.lock().unwrap(),
                &node_graph.clone(),
                &OBJECTIVES.lock().unwrap(),
            );
            for node in viable_nodes {
                let mut viable_node = VIABLE_NODE.lock().unwrap();
                viable_node.set_node_ip(node.node_ip().to_string());
                viable_node.set_node_name(node.node_name().to_string());
                viable_node.set_redis_host(node.redis_host().to_string());
                viable_node.set_node_type(node.node_type().clone());
                break;
            }
        }
    }

    info!("skylark::init: Finished initializing");
    Ok(())
}
