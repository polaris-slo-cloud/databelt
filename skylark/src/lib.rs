mod mechanism;
mod model;
mod service;

use crate::model::{Node, NodeGraph, NodeType, PodInfo, Skylark, SkylarkKey, SkylarkMetadata, SkylarkMode, SkylarkSLOs, SkylarkState};
use crate::service::{get_skylark_state, store_skylark_state, get_from_node_provider};
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::env;
use std::string::ToString;
use std::sync::Mutex;
use crate::mechanism::compute_viable_nodes;
extern crate pretty_env_logger;
use k8s_openapi::api::core::v1 as api;
use k8s_openapi::Metadata;
use uuid::{Uuid};

#[macro_use]
extern crate log;




lazy_static! {
    static ref SKYLARK: Mutex<Skylark> = Mutex::new(Skylark::new(None, None, None, None));
}
pub fn get_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

pub async fn init(mode: String, fn_name: String, key: Option<String>) -> Result<Option<SkylarkState>, Box<dyn std::error::Error>> {
    pretty_env_logger::init_timed();
    info!("skylark::init");
    let self_node_info: Node = Node::new(api::Node::metadata(&Default::default()).name.clone().unwrap(), NodeType::Sat);
    let cloud_node_info: Node = Node::new(api::Node::metadata(&Default::default()).name.clone().unwrap(), NodeType::Cloud);
    let local_redis_pod_info: PodInfo = PodInfo::new("redis".to_string(), None, Option::from("redis://redis.default.svc.cluster.local:6379".to_string()));
    let local_redis_info: HashMap<Node, PodInfo> = HashMap::from([(cloud_node_info.clone(), local_redis_pod_info.clone())]);
    let default_slo_config: SkylarkSLOs = SkylarkSLOs::new("Mbps".to_string(), "ms".to_string(), 100, 100);
    let local_key : SkylarkKey;
    let predecessor_key: SkylarkKey;
    let node_graph: NodeGraph;
    let redis_metadata: HashMap<Node, PodInfo>;
    let objectives: SkylarkSLOs;
    let mut initial_state: Option<SkylarkState> = None;

    match get_from_node_provider("node-topology").await {
        Err(e) => {
            warn!("skylark::init: failed to get node graph, moving on with empty graph: {:?}", e);
            node_graph = NodeGraph::new(vec![], vec![]);
        }
        Ok(graph) => node_graph = graph,
    }
    match get_from_node_provider("redis-pods").await {
        Err(e) => {
            warn!("skylark::init: failed to get redis metadata: {:?}", e);
            info!("skylark::init: assuming default redis info");
            redis_metadata = local_redis_info;
        }
        Ok(metadata) => redis_metadata = metadata,
    }
    match get_from_node_provider("slo").await {
        Err(e) => {
            warn!("skylark::init: failed to get SLOs: {:?}", e);
            info!("skylark::init: assuming default SLOs");
            objectives = default_slo_config
        }
        Ok(o) => objectives = o,
    }

    match key{
        Some(k) => {
            predecessor_key = SkylarkKey::from(k);
            local_key = SkylarkKey::new(predecessor_key.chain_id().to_string(), fn_name.clone());
            match get_skylark_state(&predecessor_key, local_redis_pod_info.pod_url().clone()).await {
                Err(e) => {
                    warn!("skylark::init: failed to get predecessor state from local store: {:?}", e);
                    info!("skylark::init: Fallback: fetch from cloud");
                    let cloud_pod_url = redis_metadata.get(&cloud_node_info).unwrap().pod_url().clone();
                    match get_skylark_state(&predecessor_key, cloud_pod_url).await {
                        Err(e) => {
                            error!("skylark::init: failed to get predecessor state from cloud store: {:?}", e);
                            info!("skylark::init: Fallback: move on with empty predecessor state");
                        },
                        Ok(state) => initial_state = Option::from(state),
                    }
                },
                Ok(state) => initial_state = Option::from(state),
            }
        }
        None => {
            local_key = SkylarkKey::new(Uuid::new_v4().to_string(), fn_name.clone());
        }
    }

    let mut skylark = SKYLARK.lock().unwrap();
    skylark.set_metadata(Option::from(SkylarkMetadata::new(self_node_info, local_key.chain_id().to_string(), fn_name.clone(), SkylarkMode::from(mode), redis_metadata)));
    skylark.set_node_graph(Option::from(node_graph));
    skylark.set_objectives(Option::from(objectives));
    info!("skylark::init: Finished initializing");
    Ok(initial_state)
}
pub async fn store_state(final_state: String) -> Result<String, Box<dyn std::error::Error>> {
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
            for node in compute_viable_nodes(skylark.metadata().node_info(), skylark.node_graph(), skylark.objectives()) {
                match skylark.metadata().redis_info().get(&node) {
                    Some(info) => { store_skylark_state(skylark.state(), Option::from(info.pod_ip().to_string())) }
                    None => { info!("skylark::store_state::Sat - No redis info found for node {}", node); }
                }
            }
        }
        SkylarkMode::Edge => {
            info!("skylark::store_state::Edge - Save to local node");
            store_skylark_state(skylark.state(), None).await
        }
    }
}
