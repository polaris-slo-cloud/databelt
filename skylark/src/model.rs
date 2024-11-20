// This is a part of Skylark.
// See README.md and LICENSE for details.
use chrono::{Local};
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize)]
pub struct Skylark {
    metadata: SkylarkMetadata,
    state: SkylarkState,
    node_graph: NodeGraph,
    objectives: SkylarkSLOs,
}

impl Skylark {
    pub fn new(metadata: SkylarkMetadata, state: SkylarkState, node_graph: NodeGraph, objectives: SkylarkSLOs) -> Self {
        Self { metadata, state, node_graph, objectives }
    }

    pub fn metadata(&self) -> &SkylarkMetadata {
        &self.metadata
    }

    pub fn state(&self) -> &SkylarkState {
        &self.state
    }

    pub fn node_graph(&self) -> &NodeGraph {
        &self.node_graph
    }

    pub fn objectives(&self) -> &SkylarkSLOs {
        &self.objectives
    }

    pub fn set_metadata(&mut self, metadata: SkylarkMetadata) {
        self.metadata = metadata;
    }

    pub fn set_state(&mut self, state: SkylarkState) {
        self.state = state;
    }

    pub fn set_node_graph(&mut self, node_graph: NodeGraph) {
        self.node_graph = node_graph;
    }

    pub fn set_objectives(&mut self, objectives: SkylarkSLOs) {
        self.objectives = objectives;
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct SkylarkKey {
    chain_id: String,
    fn_name: String,
}

impl SkylarkKey {
    pub fn new(chain_id: Option<String>, fn_name: String) -> Self {
        SkylarkKey {
            chain_id: chain_id.unwrap_or(uuid::Uuid::new_v4().to_string()),
            fn_name,
        }
    }
    pub fn to_string(&self) -> String {format!("{}:{}", self.chain_id, self.fn_name)}

    pub fn chain_id(self) -> String {
        self.chain_id
    }

    pub fn fn_name(self) -> String {
        self.fn_name
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
    data: SkylarkData,
}
#[derive(Serialize, Deserialize)]
pub struct SkylarkData {
    value: String,
    created: String,
    changed: String,
}

impl SkylarkState {
    pub fn new(key: SkylarkKey, data: Option<SkylarkData>) -> Self {
        SkylarkState {
            key,
            data: data.unwrap_or(SkylarkData::new(None)),
        }
    }
    pub fn update(&mut self, value: String) {
        self.data.set_value(value.clone());
    }
    pub fn get_key(&self) -> String {
        self.key.to_string()
    }
    pub fn get_data(&self) -> &SkylarkData {
        &self.data
    }
    pub fn to_string(&self) -> String {
        format!("SkylarkState\n\tSkylarkKey: {}\n\tSkylarkData: {}\n", self.key.to_string(), self.data.to_string())
    }
}
impl SkylarkData {
    pub fn new(existing_data: Option<SkylarkData>) -> Self {
        existing_data.unwrap_or({
            SkylarkData {
                value: String::new(),
                created: Local::now().to_rfc2822(),
                changed: String::new(),
            }
        })
    }
    pub fn to_string(&self) -> String {
        format!("\t\tvalue: {}\n\t\tcreated: {}\n\t\tchanged: {}", self.value, self.created, self.changed)
    }

    pub fn value(self) -> String {
        self.value
    }

    pub fn created(self) -> String {
        self.created
    }

    pub fn changed(self) -> String {
        self.changed
    }

    pub fn set_value(&mut self, value: String) {
        self.value = value;
        self.changed = Local::now().to_rfc2822();
    }

}
#[derive(Serialize, Deserialize)]
pub struct NodeGraph {
    nodes: Vec<Node>,
    edges: Vec<Edge>,
}

impl NodeGraph {
    pub fn new(nodes: Vec<Node>, edges: Vec<Edge>) -> Self {
        Self { nodes, edges }
    }

    pub fn nodes(&self) -> &Vec<Node> {
        &self.nodes
    }

    pub fn edges(&self) -> &Vec<Edge> {
        &self.edges
    }

    pub fn set_nodes(&mut self, nodes: Vec<Node>) {
        self.nodes = nodes;
    }

    pub fn set_edges(&mut self, edges: Vec<Edge>) {
        self.edges = edges;
    }
}
#[derive(Serialize, Deserialize, Clone)]
pub enum NodeType {
    Cloud,
    Edge,
    Sat,
}
#[derive(Serialize, Deserialize, Clone)]
pub struct Node {
    name: String,
    node_type: NodeType,
}

impl Node {
    pub fn new(name: String, node_type: NodeType) -> Self {
        Self { name, node_type }
    }

    pub fn name(self) -> String {
        self.name
    }

    pub fn node_type(&self) -> &NodeType {
        &self.node_type
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn set_node_type(&mut self, node_type: NodeType) {
        self.node_type = node_type;
    }

}

impl PartialEq<Self> for Node {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}
#[derive(Serialize, Deserialize)]
pub struct Edge {
    source: Node,
    target: Node,
    bandwidth: i16,
    latency: i16,
}

impl Edge {
    pub fn new(source: Node, target: Node, bandwidth: i16, latency: i16) -> Self {
        Self { source, target, bandwidth, latency }
    }

    pub fn source(&self) -> &Node {
        &self.source
    }

    pub fn target(&self) -> &Node {
        &self.target
    }

    pub fn bandwidth(&self) -> i16 {
        self.bandwidth
    }

    pub fn latency(&self) -> i16 {
        self.latency
    }

    pub fn set_source(&mut self, source: Node) {
        self.source = source;
    }

    pub fn set_target(&mut self, target: Node) {
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

    pub fn bandwidth_metric(self) -> String {
        self.bandwidth_metric
    }

    pub fn latency_metric(self) -> String {
        self.latency_metric
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
#[derive(Serialize, Deserialize)]
pub enum SkylarkMode {
    Single,
    Bundled,
}
impl From<String> for SkylarkMode {
    fn from(mode: String) -> Self {
        match mode.as_ref() {
            "bundled" => SkylarkMode::Bundled,
            _ => SkylarkMode::Single,
        }
    }
}
#[derive(Serialize, Deserialize)]
pub struct SkylarkMetadata {
    node_info: Node,
    chain_id: String,
    fn_name: String,
    mode: SkylarkMode,
}

impl SkylarkMetadata {
    pub fn new(node_info: Node, chain_id: String, fn_name: String, mode: SkylarkMode) -> Self {
        Self { node_info, chain_id, fn_name, mode }
    }

    pub fn node_info(&self) -> &Node {
        &self.node_info
    }

    pub fn chain_id(self) -> String {
        self.chain_id
    }

    pub fn fn_name(self) -> String {
        self.fn_name
    }

    pub fn mode(&self) -> &SkylarkMode {
        &self.mode
    }

    pub fn set_node_info(&mut self, node_info: Node) {
        self.node_info = node_info;
    }

    pub fn set_chain_id(&mut self, chain_id: String) {
        self.chain_id = chain_id;
    }

    pub fn set_fn_name(&mut self, fn_name: String) {
        self.fn_name = fn_name;
    }

    pub fn set_mode(&mut self, mode: SkylarkMode) {
        self.mode = mode;
    }
}

#[derive(Serialize, Deserialize)]
pub struct SkylarkRedisMetadata {
    node: Node,
    pod_name: String,
    pod_ip: String,
}

impl SkylarkRedisMetadata {
    pub fn new(node: Node, pod_name: String, pod_ip: String) -> Self {
        Self { node, pod_name, pod_ip }
    }

    pub fn node(&self) -> &Node {
        &self.node
    }

    pub fn pod_name(self) -> String {
        self.pod_name
    }

    pub fn pod_ip(self) -> String {
        self.pod_ip
    }

    pub fn set_node(&mut self, node: Node) {
        self.node = node;
    }

    pub fn set_pod_name(&mut self, pod_name: String) {
        self.pod_name = pod_name;
    }

    pub fn set_pod_ip(&mut self, pod_ip: String) {
        self.pod_ip = pod_ip;
    }
}