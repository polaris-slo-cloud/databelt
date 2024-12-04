use std::fmt::{Display, Formatter};
// This is a part of Skylark.
// See README.md and LICENSE for details.
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct SkylarkKey {
    chain_id: String,
    fn_name: String,
}

impl SkylarkKey {

    pub fn to_string(&self) -> String {format!("{}:{}", self.chain_id, self.fn_name)}

    pub fn new(chain_id: String, fn_name: String) -> Self {
        Self { chain_id, fn_name }
    }

    pub fn chain_id(&self) -> &str {
        &self.chain_id
    }

    pub fn fn_name(&self) -> &str {
        &self.fn_name
    }

    pub fn set_chain_id(&mut self, chain_id: String) {
        self.chain_id = chain_id;
    }

    pub fn set_fn_name(&mut self, fn_name: String) {
        self.fn_name = fn_name;
    }
}
impl From<String> for SkylarkKey {
    fn from(item: String) -> Self {
        let mut split = item.split(':');
        let chain_id = split.next().unwrap().to_string();
        let fn_name = split.next().unwrap().to_string();
        SkylarkKey {
            chain_id,
            fn_name,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct SkylarkState {
    key: SkylarkKey,
    value: String,
}

impl SkylarkState {

    pub fn to_string(&self) -> String {
        format!("SkylarkState\n\tSkylarkKey: {}\n\tValue: {}\n", self.key.to_string(), self.value.to_string())
    }

    pub fn new(key: SkylarkKey, value: String) -> Self {
        Self { key, value }
    }

    pub fn key(&self) -> &SkylarkKey {
        &self.key
    }

    pub fn value(&self) -> &str {
        &self.value
    }

    pub fn set_key(&mut self, key: SkylarkKey) {
        self.key = key;
    }

    pub fn set_value(&mut self, value: String) {
        self.value = value;
    }
}

#[derive(Serialize, Deserialize)]
pub struct NodeGraph {
    edges: Vec<Edge>,
}

impl NodeGraph {
    pub fn new(edges: Vec<Edge>) -> Self {
        Self { edges }
    }

    pub fn edges(&self) -> &Vec<Edge> {
        &self.edges
    }

    pub fn set_edges(&mut self, edges: Vec<Edge>) {
        self.edges = edges;
    }
}
#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Hash)]
pub enum NodeType {
    Cloud,
    Edge,
    Sat,
}

#[derive(Serialize, Deserialize, Clone, Hash, Eq)]
pub struct SkylarkNode {
    node_name: String,
    node_ip: String,
    redis_host: String,
    node_type: NodeType,
}
impl Default for SkylarkNode {
    fn default() -> Self {
        Self {
            node_name: "unknown".to_string(),
            node_ip: "127.0.0.1".to_string(),
            redis_host: "redis://redis.default.svc.cluster.local:6379".to_string(),
            node_type: NodeType::Sat,
        }
    }
}
impl PartialEq<Self> for SkylarkNode {
    fn eq(&self, other: &Self) -> bool {
        self.node_name == other.node_name && self.node_ip == other.node_ip && self.redis_host == other.redis_host
    }
}
impl SkylarkNode {
    pub fn new(node_name: String, node_ip: String, redis_host: String, node_type: NodeType) -> Self {
        Self { node_name, node_ip, redis_host, node_type }
    }
    pub fn default_cloud() -> Self {
        Self {
            node_name: "unknown".to_string(),
            node_ip: "127.0.0.1".to_string(),
            redis_host: "redis://redis.default.svc.cluster.local:6379".to_string(),
            node_type: NodeType::Cloud,
        }
    }
    pub fn node_name(&self) -> &str {
        &self.node_name
    }

    pub fn node_ip(&self) -> &str {
        &self.node_ip
    }

    pub fn redis_host(&self) -> &str {
        &self.redis_host
    }

    pub fn node_type(&self) -> &NodeType {
        &self.node_type
    }

    pub fn set_node_name(&mut self, node_name: String) {
        self.node_name = node_name;
    }

    pub fn set_node_ip(&mut self, node_ip: String) {
        self.node_ip = node_ip;
    }

    pub fn set_redis_host(&mut self, redis_host: String) {
        self.redis_host = redis_host;
    }

    pub fn set_node_type(&mut self, node_type: NodeType) {
        self.node_type = node_type;
    }
}
#[derive(Serialize, Deserialize)]
pub struct Edge {
    source: SkylarkNode,
    target: SkylarkNode,
    bandwidth: i16,
    latency: i16,
}

impl Edge {
    pub fn new(source: SkylarkNode, target: SkylarkNode, bandwidth: i16, latency: i16) -> Self {
        Self { source, target, bandwidth, latency }
    }

    pub fn source(&self) -> &SkylarkNode {
        &self.source
    }

    pub fn target(&self) -> &SkylarkNode {
        &self.target
    }

    pub fn bandwidth(&self) -> i16 {
        self.bandwidth
    }

    pub fn latency(&self) -> i16 {
        self.latency
    }

    pub fn set_source(&mut self, source: SkylarkNode) {
        self.source = source;
    }

    pub fn set_target(&mut self, target: SkylarkNode) {
        self.target = target;
    }

    pub fn set_bandwidth(&mut self, bandwidth: i16) {
        self.bandwidth = bandwidth;
    }

    pub fn set_latency(&mut self, latency: i16) {
        self.latency = latency;
    }
}

#[derive(Serialize, Deserialize)]
pub struct SkylarkSLOs {
    bandwidth_metric: String,
    latency_metric: String,
    min_bandwidth: i16,
    max_latency: i16,
}

impl SkylarkSLOs {
    pub fn new(bandwidth_metric: String, latency_metric: String, min_bandwidth: i16, max_latency: i16) -> Self {
        Self { bandwidth_metric, latency_metric, min_bandwidth, max_latency }
    }

    pub fn bandwidth_metric(&self) -> &str {
        &self.bandwidth_metric
    }

    pub fn latency_metric(&self) -> &str {
        &self.latency_metric
    }

    pub fn min_bandwidth(&self) -> i16 {
        self.min_bandwidth
    }

    pub fn max_latency(&self) -> i16 {
        self.max_latency
    }

    pub fn set_bandwidth_metric(&mut self, bandwidth_metric: String) {
        self.bandwidth_metric = bandwidth_metric;
    }

    pub fn set_latency_metric(&mut self, latency_metric: String) {
        self.latency_metric = latency_metric;
    }

    pub fn set_min_bandwidth(&mut self, min_bandwidth: i16) {
        self.min_bandwidth = min_bandwidth;
    }

    pub fn set_max_latency(&mut self, max_latency: i16) {
        self.max_latency = max_latency;
    }
}
impl Default for SkylarkSLOs {
    fn default() -> Self {
        Self {
            bandwidth_metric: "Mbps".to_string(),
            latency_metric: "ms".to_string(),
            min_bandwidth: 100,
            max_latency: 80,
        }
    }
}
#[derive(Serialize, Deserialize, Debug)]
pub enum SkylarkMode {
    Cloud,
    Edge,
    Sat,
}
impl Display for SkylarkMode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl From<String> for SkylarkMode {
    fn from(mode: String) -> Self {
        match mode.to_lowercase().as_str() {
            "satellite" => SkylarkMode::Sat,
            "sat" => SkylarkMode::Sat,
            "edge" => SkylarkMode::Edge,
            _ => SkylarkMode::Cloud,
        }
    }
}