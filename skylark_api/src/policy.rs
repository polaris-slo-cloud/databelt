use crate::model::{
    Edge, Graph, NodeGraph, NodePath, NodeType, SkylarkNode, SkylarkNodeMap, SkylarkSLOs,
};
use rand::{random, Rng};
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

pub fn compute_viable_node(
    current_node: &SkylarkNode,
    node_graph: &NodeGraph,
    objectives: &SkylarkSLOs,
) -> Option<SkylarkNode> {
    let mut viable_node: Option<SkylarkNode> = None;
    let mut best_edge: Option<Edge> = None;

    for edge in node_graph.edges() {
        if (!current_node.eq(&edge.source()) && !current_node.eq(&edge.target()))
            || edge.latency() > objectives.max_latency()
            || edge.bandwidth() < objectives.min_bandwidth()
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

fn build_graph_and_node_map(node_graph: &NodeGraph) -> (Graph, SkylarkNodeMap) {
    let mut graph: Graph = HashMap::new();
    let mut node_map: SkylarkNodeMap = HashMap::new();
    for edge in node_graph {
        if !node_map.contains_key(edge.source.node_name.clone()) {
            node_map.insert(edge.source.node_name.clone(), edge.source.clone())
        }
        if !node_map.contains_key(edge.target.node_name.clone()) {
            node_map.insert(edge.target.node_name.clone(), edge.target.clone())
        }
        graph
            .entry(edge.source.node_name.clone())
            .or_insert_with(Vec::new)
            .push((edge.target.node_name.clone(), edge.latency));
        graph
            .entry(edge.target.node_name.clone())
            .or_insert_with(Vec::new)
            .push((edge.source.node_name.clone(), edge.latency));
    }
    (graph, node_map)
}
fn dijkstra(graph: &Graph, start: String, destination: String) -> NodePath {
    let mut distances: HashMap<String, i16> = HashMap::new();
    let mut heap = BinaryHeap::new();

    distances.insert(start.clone(), 0);
    heap.push((0, start.clone()));

    let mut predecessors: HashMap<String, String> = HashMap::new();

    while let Some((cost, node)) = heap.pop() {
        let cost = -cost;
        debug!(
            "dijkstra: Processing node: {}, Current cost: {}",
            node, cost
        );

        if node == destination {
            debug!("dijkstra: Reached destination: {}", destination);
            break;
        }

        if let Some(neighbors) = graph.get(&node) {
            for (neighbor, weight) in neighbors {
                let next_cost = cost + weight;
                debug!(
                    "dijkstra: Checking neighbor: {}, Edge weight: {}, Next cost: {}",
                    neighbor, weight, next_cost
                );
                if next_cost < *distances.get(neighbor).unwrap_or(&i16::MAX) {
                    debug!(
                        "dijkstra: Updating distance for neighbor: {}, New cost: {}",
                        neighbor, next_cost
                    );
                    distances.insert(neighbor.clone(), next_cost);
                    predecessors.insert(neighbor.clone(), node.clone());
                    heap.push((-next_cost, neighbor.clone()));
                }
            }
        }
    }

    debug!("dijkstra: Reconstructing shortest path");
    let mut path = Vec::new();
    let mut current_node = destination;
    while let Some(predecessor) = predecessors.get(&current_node) {
        let cost = *distances.get(&current_node).unwrap();
        debug!(
            "dijkstra: Path step: Node: {}, Cost: {}",
            current_node, cost
        );
        path.push((cost, current_node.clone()));
        current_node = predecessor.clone();
    }
    path.push((0, start));
    debug!("dijkstra: Final reverse path: {:?}", path);
    path
}
pub fn apply_skylark_policy(
    start: String,
    destination: String,
    topology: &NodeGraph,
    slo: SkylarkSLOs,
    next_task_type: NodeType,
) -> Option<SkylarkNode> {
    info!("apply_skylark_heuristic: start");
    let graph: Graph;
    let node_map: SkylarkNodeMap;
    (graph, node_map) = build_graph_and_node_map(topology);
    let reverse_path = dijkstra(&graph, start, destination);

    if reverse_path.is_empty() {
        warn!("apply_skylark_heuristic: emtpy node path given, returning None");
        return None;
    }
    for step in reverse_path {
        debug!(
            "apply_skylark_heuristic: node: {:?}, latency: {}",
            step.1.clone(),
            step.0
        );
        let step_node = node_map.get(&step.1).unwrap();
        if !next_task_type.eq(step_node.node_type()) {
            continue;
        }
        if step.0 <= slo.max_latency() {
            debug!(
                "apply_skylark_heuristic: elected node: {:?} with latency {}",
                step.1.clone(),
                step.0
            );
            return Some(step_node.clone());
        }
    }
    error!("apply_skylark_heuristic: No node was elected even though path is not empty!");
    None
}

pub fn apply_random_policy(
    start: String,
    destination: String,
    topology: &NodeGraph,
) -> Option<SkylarkNode> {
    info!("apply_random_policy: start");
    let graph: Graph;
    let node_map: SkylarkNodeMap;
    (graph, node_map) = build_graph_and_node_map(topology);
    let reverse_path = dijkstra(&graph, start, destination);
    if reverse_path.is_empty() {
        warn!("apply_random_policy: emtpy node path given, returning None");
        return None;
    }

    let mut rng = rand::thread_rng();
    let random_number = rng.gen_range(0..=reverse_path.len());
    debug!("apply_random_policy: Random number: {}", random_number);
    let random_step = reverse_path.get(random_number).unwrap();
    let random_node = node_map.get(&random_step.1).unwrap();
    debug!("apply_random_policy: elected node: {:?}", random_node);
    Some(random_node.cline());
}

pub fn apply_serverless_policy(
    start: String,
    destination: String,
    topology: &NodeGraph,
) -> Option<SkylarkNode> {
    info!("apply_serverless_policy: start");
    let graph: Graph;
    let node_map: SkylarkNodeMap;
    (graph, node_map) = build_graph_and_node_map(topology);
    let reverse_path = dijkstra(&graph, start, destination);

    if reverse_path.is_empty() {
        warn!("apply_serverless_policy: emtpy node path given, returning None");
        return None;
    }
    for step in reverse_path {
        debug!(
            "apply_serverless_policy: node: {:?}, latency: {}",
            step.1.clone(),
            step.0
        );
        let step_node = node_map.get(&step.1).unwrap();
        if !NodeType::Cloud.eq(step_node.node_type()) {
            continue;
        }
        debug!(
            "apply_serverless_policy: elected node: {:?} with latency {}",
            step.1.clone(),
            step.0
        );
        return Some(step_node.clone());
    }
    error!("apply_serverless_policy: No node was elected because no Cloud node was found!");
    None
}
