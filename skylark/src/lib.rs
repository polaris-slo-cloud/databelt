mod model;
mod service;
mod mechanism;

use std::cmp::PartialEq;
use hyper::body::HttpBody;
use hyper::{Body, Client, Method, Request, Uri};
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use std::env;
use std::fmt::format;
use std::sync::Mutex;
use uuid::uuid;
use crate::service::{get_node_graph, get_redis_metadata, get_skylark_state, get_slos};
use crate::model::{Node, NodeGraph, NodeType, Skylark, SkylarkKey, SkylarkMetadata, SkylarkMode, SkylarkRedisMetadata, SkylarkSLOs, SkylarkState};

extern crate pretty_env_logger;
#[macro_use]
extern crate log;

pub fn get_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

pub async fn init(mode: String, fn_name: String, node_name: String, key: String) {
    pretty_env_logger::init_timed();
    info!("skylark::init");
    let mut predecessor_key = SkylarkKey::from(key);
    let node_graph: NodeGraph;
    let redis_metadata: SkylarkRedisMetadata;
    let mut slos: SkylarkSLOs = SkylarkSLOs::new("Mbps".parse().unwrap(), "ms".parse().unwrap(), 90, 40);
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
    let skylark = Skylark::new(
        SkylarkMetadata::new(node, predecessor_key.clone().chain_id(), fn_name.clone(), SkylarkMode::from(mode)),
        SkylarkState::new(SkylarkKey::new(Option::from(predecessor_key.chain_id()), fn_name), None),
        node_graph,
        slos);
    info!(skylark, "skylark::init: Finished initializing");
}