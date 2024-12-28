use std::collections::HashMap;
use std::fmt::{Display, Formatter};
// This is a part of Skylark.
// See README.md and LICENSE for details.
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SkylarkKey {
    chain_id: String,
    node_id: String,
    task_id: String,
}

impl SkylarkKey {
    pub fn to_string(&self) -> String {
        format!("{}:{}:{}", self.chain_id, self.node_id, self.task_id)
    }

    pub fn chain_id(&self) -> &str {
        &self.chain_id
    }

    pub fn task_id(&self) -> &str {
        &self.task_id
    }

    pub fn set_chain_id(&mut self, chain_id: String) {
        self.chain_id = chain_id;
    }

    pub fn set_task_id(&mut self, task_id: String) {
        self.task_id = task_id;
    }
    pub fn valid(&self) -> bool {
        !self.chain_id.is_empty()
            && !self.task_id.is_empty()
            && !self.node_id.is_empty()
            && !self.chain_id.eq("unknown")
            && !self.chain_id.eq("unknown")
            && !self.node_id.eq("unknown")
    }

    pub fn new(chain_id: String, node_id: String, task_id: String) -> Self {
        Self {
            chain_id,
            node_id,
            task_id,
        }
    }

    pub fn node_id(&self) -> &str {
        &self.node_id
    }

    pub fn set_node_id(&mut self, node_id: String) {
        self.node_id = node_id;
    }
}
impl TryFrom<String> for SkylarkKey {
    type Error = ();

    fn try_from(item: String) -> Result<Self, Self::Error> {
        let mut split = item.split(':');
        let chain_id = split.next().ok_or(())?.to_string();
        let node_id = split.next().ok_or(())?.to_string();
        let task_id = split.next().ok_or(())?.to_string();
        Ok(SkylarkKey { chain_id, node_id, task_id })
    }
}

impl Default for SkylarkKey {
    fn default() -> Self {
        Self {
            task_id: "unknown".to_string(),
            chain_id: "unknown".to_string(),
            node_id: "unknown".to_string(),
        }
    }
}

impl PartialEq for SkylarkKey {
    fn eq(&self, other: &Self) -> bool {
        self.chain_id == other.chain_id && self.task_id == other.task_id && self.node_id == other.node_id
    }
}
#[derive(Serialize, Deserialize)]
pub struct SkylarkState {
    key: SkylarkKey,
    value: String,
}

impl SkylarkState {
    pub fn to_string(&self) -> String {
        format!(
            "SkylarkState\n\tSkylarkKey: {}\n\tValue: {}\n",
            self.key.to_string(),
            self.value.to_string()
        )
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

pub type Graph = HashMap<String, Vec<(String, i64)>>;
pub type NodePath = Vec<(i64, String)>;
pub type SkylarkNodeMap = HashMap<String, SkylarkNode>;

#[derive(Serialize, Deserialize, Clone)]
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
#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Hash, Debug)]
pub enum NodeType {
    Cloud,
    Edge,
    Sat,
}

#[derive(Serialize, Deserialize, Clone, Hash, Eq, Debug)]
pub struct SkylarkNode {
    node_name: String,
    node_ip: String,
    node_type: NodeType,
}
impl Default for SkylarkNode {
    fn default() -> Self {
        Self {
            node_name: "unknown".to_string(),
            node_ip: "127.0.0.1".to_string(),
            node_type: NodeType::Sat,
        }
    }
}
impl PartialEq<Self> for SkylarkNode {
    fn eq(&self, other: &Self) -> bool {
        self.node_name == other.node_name
            && self.node_ip == other.node_ip
    }
}
impl SkylarkNode {
    pub fn new(
        node_name: String,
        node_ip: String,
        node_type: NodeType,
    ) -> Self {
        Self {
            node_name,
            node_ip,
            node_type,
        }
    }
    pub fn default_cloud() -> Self {
        Self {
            node_name: "unknown".to_string(),
            node_ip: "127.0.0.1".to_string(),
            node_type: NodeType::Cloud,
        }
    }
    pub fn node_name(&self) -> &str {
        &self.node_name
    }

    pub fn node_ip(&self) -> &str {
        &self.node_ip
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

    pub fn set_node_type(&mut self, node_type: NodeType) {
        self.node_type = node_type;
    }
}
#[derive(Serialize, Deserialize, Clone)]
pub struct Edge {
    source: SkylarkNode,
    target: SkylarkNode,
    bandwidth: i64,
    latency: i64,
}

impl Edge {
    pub fn new(source: SkylarkNode, target: SkylarkNode, bandwidth: i64, latency: i64) -> Self {
        Self {
            source,
            target,
            bandwidth,
            latency,
        }
    }

    pub fn source(&self) -> &SkylarkNode {
        &self.source
    }

    pub fn target(&self) -> &SkylarkNode {
        &self.target
    }

    pub fn bandwidth(&self) -> i64 {
        self.bandwidth
    }

    pub fn latency(&self) -> i64 {
        self.latency
    }

    pub fn set_source(&mut self, source: SkylarkNode) {
        self.source = source;
    }

    pub fn set_target(&mut self, target: SkylarkNode) {
        self.target = target;
    }

    pub fn set_bandwidth(&mut self, bandwidth: i64) {
        self.bandwidth = bandwidth;
    }

    pub fn set_latency(&mut self, latency: i64) {
        self.latency = latency;
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct SkylarkSLOs {
    min_bandwidth: i64,
    max_latency: i64,
}

impl SkylarkSLOs {
    pub fn new(
        min_bandwidth: i64,
        max_latency: i64,
    ) -> Self {
        Self {
            min_bandwidth,
            max_latency,
        }
    }

    pub fn min_bandwidth(&self) -> i64 {
        self.min_bandwidth
    }

    pub fn max_latency(&self) -> i64 {
        self.max_latency
    }

    pub fn set_min_bandwidth(&mut self, min_bandwidth: i64) {
        self.min_bandwidth = min_bandwidth;
    }

    pub fn set_max_latency(&mut self, max_latency: i64) {
        self.max_latency = max_latency;
    }
}
impl Default for SkylarkSLOs {
    fn default() -> Self {
        Self {
            min_bandwidth: 100,
            max_latency: 80,
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum SkylarkPolicy {
    Skylark,
    Random,
    Stateless
}
impl From<String> for SkylarkPolicy {
    fn from(mode: String) -> Self {
        match mode.to_lowercase().as_str() {
            "skylark" => SkylarkPolicy::Skylark,
            "random" => SkylarkPolicy::Random,
            "stateless" => SkylarkPolicy::Stateless,
            _ => SkylarkPolicy::Skylark,
        }
    }
}

impl Display for SkylarkPolicy {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
