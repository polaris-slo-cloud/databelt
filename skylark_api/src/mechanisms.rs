use crate::model::{Edge, NodeGraph, SkylarkNode, SkylarkSLOs};

pub fn compute_viable_node(
    current_node: &SkylarkNode,
    node_graph: &NodeGraph,
    objectives: &SkylarkSLOs,
) -> Option<SkylarkNode> {
    let mut viable_node: Option<SkylarkNode> = None;
    let mut best_edge: Option<Edge> = None;

    for edge in node_graph.edges() {
        if (!current_node.eq(&edge.source()) && !current_node.eq(&edge.target())) ||
            edge.latency() > objectives.max_latency() ||
            edge.bandwidth() < objectives.min_bandwidth()
        {
            continue;
        }
        if best_edge.is_some() {
            if best_edge.clone().unwrap().latency() < edge.latency() {
                continue;
            }
        }

        best_edge = Option::from(edge.to_owned());
        viable_node = Option::from({
            if current_node.eq(&edge.source()) {
                edge.target().clone()
            } else {
                edge.source().clone()
            }
        });
    }
    if viable_node.is_none() {
        get_lowest_latency_node(current_node, node_graph);
    }
    viable_node
}

pub fn get_lowest_latency_node(
    current_node: &SkylarkNode,
    node_graph: &NodeGraph,
) -> Option<SkylarkNode> {
    let mut min_latency: i16 = i16::MAX;
    let mut closest_node: Option<SkylarkNode> = None;
    for edge in node_graph.edges() {
        if !current_node.eq(&edge.source()) && !current_node.eq(&edge.target()) {
            continue;
        }
        if edge.latency() < min_latency {
            debug!("Found a closer node");
            min_latency = edge.latency();
            if current_node.eq(&edge.source()) {
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
