mod mechanism;
mod model;
mod service;

use crate::model::{Node, NodeGraph, NodeType, PodInfo, Skylark, SkylarkKey, SkylarkMetadata, SkylarkMode, SkylarkSLOs, SkylarkState};
use crate::service::{get_node_graph, get_redis_metadata, get_skylark_state, get_slos, store_skylark_state};
use hyper::body::HttpBody;
use hyper::{Body, Client, Method, Request, Uri};
use lazy_static::lazy_static;
use serde::Deserialize;
use serde::Serialize;
use std::cmp::PartialEq;
use std::collections::HashMap;
use std::env;
use std::fmt::format;
use std::sync::Mutex;
use uuid::uuid;
use crate::mechanism::compute_viable_nodes;

extern crate pretty_env_logger;
#[macro_use]
extern crate log;

lazy_static! {
    static ref SKYLARK: Mutex<Skylark> = Mutex::new(Skylark::new(
        SkylarkMetadata::new(
            Node::new("unknown".to_string(), NodeType::Sat),
            "unknown".to_string(),
            "unknown".to_string(),
            SkylarkMode::from("unknown".to_string()),
            Default::default()
        ),
        SkylarkState::new(
            SkylarkKey::new(Option::from("unknown".to_string()), "unknown".to_string()),
            None,
        ),
        NodeGraph::new(vec![], vec![]),
        SkylarkSLOs::new("unknown".to_string(), "unknown".to_string(), 0, 0),));
}

pub fn get_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

pub async fn init(mode: String, fn_name: String, node_name: String, key: String) {
    pretty_env_logger::init_timed();
    info!("skylark::init");
    let mut predecessor_key = SkylarkKey::from(key);
    let node_graph: NodeGraph;
    let mut redis_metadata: HashMap<Node, PodInfo> = Default::default();
    let mut slos: SkylarkSLOs =
        SkylarkSLOs::new("Mbps".parse().unwrap(), "ms".parse().unwrap(), 90, 40);
    let predecessor_state: SkylarkState;
    let node = Node::new(node_name, NodeType::Sat);
    match get_node_graph().await {
        Err(err) => error!("skylark::init: Error fetching NodeGraph: {}", err),
        Some(nodeGraph) => {
            info!("skylark::init: NodeGraph received");
            trace!("skylark::init: NodeGraph: {:?}", nodeGraph);
            node_graph = nodeGraph;
        }
    }
    match get_redis_metadata().await {
        Err(err) => error!("skylark::init: Error fetching Redis Metadata: {}", err),
        Some(redisMeteData) => {
            info!("skylark::init: Redis Metadata received");
            trace!("skylark::init: Redis Metadata: {:?}", redisMeteData);
            redis_metadata = redisMeteData;
        }
    }
    match get_slos().await {
        Err(err) => error!("skylark::init: Error fetching SLOs: {}", err),
        Some(SLOs) => {
            info!("skylark::init: SLOs received");
            trace!("skylark::init: SLOs: {:?}", SLOs);
            slos = SLOs;
        }
    }
    match get_skylark_state(predecessor_key.clone(), None).await {
        Err(err) => error!("skylark::init: Error fetching predecessor state: {}", err),
        Some(state) => {
            info!("skylark::init: predecessor state received");
            trace!("skylark::init: predecessor state: {:?}", state);
            predecessor_state = state;
        }
    }

    let node_graph = get_node_graph();
    let mut skylark = SKYLARK.lock().unwrap();
    skylark.set_metadata(SkylarkMetadata::new(node, predecessor_key.clone().chain_id(), fn_name.clone(), SkylarkMode::from(mode), redis_metadata));
    skylark.set_state(SkylarkState::new(
        SkylarkKey::new(Option::from(predecessor_key.chain_id()), fn_name),
        None,
    ));
    skylark.set_node_graph(node_graph);
    skylark.set_objectives(slos);
    info!(skylark, "skylark::init: Finished initializing");
}
pub fn store_state(final_state: String) {
    info!("skylark::store_state");
    trace!("skylark::store_state: {}", final_state);
    let mut skylark = SKYLARK.lock().unwrap();
    skylark.state().update(final_state);
    match skylark.metadata().mode() {
        SkylarkMode::Cloud => {
            info!("skylark::store_state::Cloud - Save to cloud node");
            //TODO: supply cloud url
            store_skylark_state(skylark.state(), None)
        }
        SkylarkMode::Sat => {
            info!("skylark::store_state::Sat - Save to viable nodes");
            for node in compute_viable_nodes(skylark.metadata().node_info(), skylark.node_graph(), skylark.objectives()){
                match skylark.metadata().redis_info().get(&node){
                    Some(info) => {store_skylark_state(skylark.state(), Option::from(info.pod_ip().to_string()))}
                    None => {info!("skylark::store_state::Sat - No redis info found for node {}", node);}
                }
            }
        }
        SkylarkMode::Edge => {
            info!("skylark::store_state::Edge - Save to local node");
            store_skylark_state(skylark.state(), None)
        }
    }
}
