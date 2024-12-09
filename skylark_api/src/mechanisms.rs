use crate::model::{NodeGraph, SkylarkNode, SkylarkSLOs};

pub fn compute_viable_nodes(
    current_node: &SkylarkNode,
    node_graph: &NodeGraph,
    objectives: &SkylarkSLOs,
) -> Vec<SkylarkNode> {
    let mut viable_nodes: Vec<SkylarkNode> = Vec::new();
    for edge in node_graph.edges() {
        if !current_node.eq(&edge.source()) && !current_node.eq(&edge.target()) {
            continue;
        }
        if edge.latency() > objectives.max_latency() {
            continue;
        }
        if edge.bandwidth() < objectives.min_bandwidth() {
            continue;
        }
        viable_nodes.insert(
            viable_nodes.len(),
            if current_node.eq(&edge.source()) {
                edge.target().clone()
            } else {
                edge.source().clone()
            },
        );
    }
    viable_nodes
}

pub fn get_closest_viable_node(
    current_node: &SkylarkNode,
    node_graph: &NodeGraph,
    objectives: &SkylarkSLOs,
) -> Option<SkylarkNode> {
    let mut min_latency: i16 = i16::MAX;
    let mut closest_node: Option<SkylarkNode> = None;
    for edge in node_graph.edges() {
        if !current_node.eq(&edge.source()) && !current_node.eq(&edge.target()) {
            continue;
        }
        if edge.latency() > objectives.max_latency() {
            continue;
        }
        if edge.bandwidth() < objectives.min_bandwidth() {
            continue;
        }
        if edge.latency() < min_latency {
            debug!("Found a closer node");
            min_latency = edge.latency();
            if current_node.eq(&edge.source()){
                closest_node = Some(edge.target().clone());
                debug!("New closest node: {}", edge.target().node_name());
            } else {
                closest_node = Some(edge.source().clone());
                debug!("New closest node: {}", edge.target().node_name());
            }
        }
    }
    closest_node
}
