mod model;
mod mechanisms;
mod http_client;
mod redis_client;

use hyper::{Response, StatusCode};
use hyper::{Body, Method, Request};
use tokio::net::TcpListener;
use crate::model::{NodeGraph, SkylarkKey, SkylarkNode, SkylarkSLOs, SkylarkState};
use lazy_static::lazy_static;
use std::env;
use std::net::SocketAddr;
use std::ops::Deref;
use std::string::ToString;
use std::sync::{Mutex};
use hyper::server::conn::Http;
use hyper::service::service_fn;
use crate::http_client::get_from_url;
use crate::mechanisms::compute_viable_nodes;
use crate::redis_client::{del_global_state, del_local_state, get_global_state, get_local_state, store_global_state, store_local_state, store_state_by_url};

extern crate pretty_env_logger;
#[macro_use]
extern crate log;
const NODE_SERVICE_URL: &'static str = "http://node-service.default.svc.cluster.local";
lazy_static! {
    static ref VIABLE_NODE: Mutex<SkylarkNode> = Mutex::new(SkylarkNode::default());
    static ref LOCAL_NODE: Mutex<SkylarkNode> = Mutex::new(SkylarkNode::default());
    static ref CLOUD_NODE: Mutex<SkylarkNode> = Mutex::new(SkylarkNode::default_cloud());
    static ref OBJECTIVES: Mutex<SkylarkSLOs> = Mutex::new(SkylarkSLOs::default());
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    pretty_env_logger::init_timed();
    match init().await {
        Ok(_) => {
            info!("main:: Skylark is done initializing")
        }
        Err(e) => {
            error!("main: Error while initializing skylark: {:?}", e);
        }
    }
    info!("Starting Example Preprocessor {}", env!("CARGO_PKG_VERSION"));

    info!("Skylark library loaded: {}", get_version());
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));

    let listener = TcpListener::bind(addr).await?;
    info!("Listening on http://{}", addr);
    loop {
        let (stream, _) = listener.accept().await?;

        tokio::task::spawn(async move {
            if let Err(err) = Http::new().serve_connection(stream, service_fn(http_handler)).await {
                error!("Error serving connection: {:?}", err);
            }
        });
    }
}

async fn http_handler(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    match (req.method(), req.uri().path()) {
        // Serve some instructions at /
        (&Method::GET, "/state") => {
            info!("main::http_handler::GET_STATE: incoming");
            // split at '=' of 'key=CHAIN_ID:FN_NAME'
            let key = req.uri().query().unwrap().split_at(3).1;
            match get_local_state(&SkylarkKey::from(key.to_string())).await {
                Ok(state) => {
                    info!("main::http_handler::GET_STATE: local redis result: {:?}", state.clone());
                    Ok(Response::new(Body::from(state)))
                }
                Err(e) => {
                    error!("main::http_handler::GET_STATE: Error getting local state: {:?}", e);
                    match get_global_state(&SkylarkKey::from(key.to_string())).await {
                        Ok(state) => {
                            info!("main::http_handler::GET_STATE: global redis result: {:?}", state.clone());
                            Ok(Response::new(Body::from(state)))
                        }
                        Err(e) => {
                            error!("main::http_handler::GET_STATE: Error fetching global and local state: {:?}", e);
                            Ok(Response::builder().status(StatusCode::NOT_FOUND).body(Body::empty()).unwrap())
                        }
                    }
                }
            }
        }
        (&Method::DELETE, "/state") => {
            info!("main::http_handler::DELETE_STATE: incoming");
            // split at '=' of 'key=CHAIN_ID:FN_NAME'
            let key = req.uri().query().unwrap().split_at(3).1;
            let mut err = false;
            match del_local_state(&SkylarkKey::from(key.to_string())).await {
                Ok(_) => {
                    info!("main::http_handler::DELETE_STATE: successfully deleted local state");
                }
                Err(e) => {
                    warn!("main::http_handler::DELETE_STATE: Error deleting local state: {:?}", e);
                    err = true;
                }
            }
            match del_global_state(&SkylarkKey::from(key.to_string())).await {
                Ok(_) => {
                    info!("main::http_handler::DELETE_STATE: successfully deleted global state");
                }
                Err(e) => {
                    warn!("main::http_handler::DELETE_STATE: Error deleting global state: {:?}", e);
                    err = true;
                }
            }
            if err {
                Ok(Response::builder().status(StatusCode::INTERNAL_SERVER_ERROR).body(Body::empty()).unwrap())
            } else {
                Ok(Response::new(Body::from("Successfully deleted state")))
            }
        }
        (&Method::POST, "/save/sat") => {
            info!("main::http_handler::SAVE_SAT: incoming");
            let whole_body = hyper::body::to_bytes(req.into_body()).await?;
            let state: SkylarkState = serde_json::from_slice(&whole_body.to_vec()).unwrap();
            let mut err = false;
            let mut propagated_node_name: String = "None".to_string();
            let viable_node = VIABLE_NODE.lock().unwrap().clone();
            if viable_node.node_name() != "unknown" {
                match store_state_by_url(&state, viable_node.redis_host().to_string()).await {
                    Ok(_) => {
                        propagated_node_name = viable_node.node_name().to_string();
                        info!("main::http_handler::SAVE_SAT: successfully stored state to node {}", propagated_node_name)
                    }
                    Err(e) => {
                        error!("main::http_handler::SAVE_SAT: Error propagating state: {:?}", e);
                        err = true;
                    }
                }
                match store_global_state(&state).await {
                    Ok(_) => {
                        info!("main::http_handler::SAVE_SAT: successfully stored state to cloud node")
                    }
                    Err(e) => {
                        error!("main::http_handler::SAVE_SAT: Error saving global state: {:?}", e);
                        err = true;
                    }
                }

            } else {
                warn!("main::http_handler::SAVE_SAT: No viable node found");
                err = true;
            }
            if err {
                Ok(Response::builder().status(StatusCode::INTERNAL_SERVER_ERROR).body(Body::empty()).unwrap())
            } else {
                info!("main::http_handler::SAVE_SAT: successfully propagated state to viable node- and global store");
                Ok(Response::new(Body::from(format!("Successfully stored state on node {}", propagated_node_name))))
            }

        }
        (&Method::POST, "/save/edge") => {
            info!("main::http_handler::SAVE_EDGE: incoming");
            let whole_body = hyper::body::to_bytes(req.into_body()).await?;
            let state: SkylarkState = serde_json::from_slice(&whole_body.to_vec()).unwrap();
            match store_local_state(&state).await {
                Ok(res) => {
                    info!("main::http_handler::SAVE_EDGE: Saved edge state: {:?}", res);
                    Ok(Response::new(Body::from("OK")))
                }
                Err(e) => {
                    error!("main::http_handler::SAVE_EDGE: Error saving local state: {:?}", e);
                    Ok(Response::builder().status(StatusCode::INTERNAL_SERVER_ERROR).body(Body::empty()).unwrap())
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
                    Ok(Response::new(Body::from("OK")))
                }
                Err(e) => {
                    error!("main::http_handler::SAVE_CLOUD: Error saving global state: {:?}", e);
                    Ok(Response::builder().status(StatusCode::INTERNAL_SERVER_ERROR).body(Body::empty()).unwrap())
                }
            }
        }
        (&Method::GET, "/health") => {
            Ok(Response::new(Body::from("OK")))
        }
        // Return the 404 Not Found for other routes.
        _ => {
            Ok(Response::builder().status(StatusCode::NOT_FOUND).body(Body::empty()).unwrap())
        }
    }
}


pub fn get_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}
async fn init() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let node_graph: NodeGraph;
    let viable_nodes: Vec<SkylarkNode>;

    match get_from_url::<SkylarkNode>(format!("{}/{}", NODE_SERVICE_URL, "local-node-info")).await {
        Err(e) => {
            warn!("skylark::init: failed to get local node info!\n {:?}", e);
            info!("skylark::init: moving on with default");
        }
        Ok(node) => {
            info!("skylark::init: successfully fetched node info");
            let mut local_node = LOCAL_NODE.lock().unwrap();
            local_node.set_node_ip(node.node_ip().to_string());
            local_node.set_node_name(node.node_name().to_string());
            local_node.set_redis_host(node.redis_host().to_string());
            local_node.set_node_type(node.node_type().clone());
        }
    }
    match get_from_url::<SkylarkNode>(format!("{}/{}", NODE_SERVICE_URL, "cloud-node-info")).await {
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
    match get_from_url::<SkylarkSLOs>(format!("{}/{}", NODE_SERVICE_URL, "objectives")).await {
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
    match get_from_url::<NodeGraph>(format!("{}/{}", NODE_SERVICE_URL, "node-graph")).await {
        Err(e) => {
            error!("skylark::init: failed to get node graph!\n {:?}", e);
        }
        Ok(graph) => {
            info!("skylark::init: successfully fetched node graph");
            node_graph = graph;
            viable_nodes = compute_viable_nodes(&LOCAL_NODE.lock().unwrap(), &node_graph, &OBJECTIVES.lock().unwrap());
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
