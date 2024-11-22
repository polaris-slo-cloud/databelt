use crate::model::{Node, NodeGraph, SkylarkSLOs};

pub fn compute_viable_nodes(current_node: &Node, node_graph: &NodeGraph, objectives: &SkylarkSLOs) -> Vec<Node>{
    let mut viable_nodes: Vec<Node> = Vec::new();
    for edge in node_graph.edges() {
        if !current_node.eq(&edge.source()) && !current_node.eq(&edge.target()) {continue}
        if edge.latency() > objectives.max_latency() {continue}
        if edge.bandwidth() < objectives.min_bandwidth() {continue}
        viable_nodes.insert(viable_nodes.len(),
           if current_node.eq(&edge.source()) {edge.target().clone()}
           else {edge.source().clone()}
        );
    }
    viable_nodes
}

//strategy: find best/find first
//finding the nodes should not impact the latency