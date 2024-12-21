use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};
use crate::model::{Edge, NodeGraph, NodeType, SkylarkNode, SkylarkSLOs};

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
type AdjacencyList = HashMap<String, Vec<(String, i16)>>;

fn define_build_graph(graph: &NodeGraph) -> AdjacencyList {
    let mut adj_list = HashMap::new();

    for edge in graph.edges() {
    adj_list
    .entry(edge.source().node_name().to_string())
    .or_insert_with(Vec::new)
    .push((edge.target().node_name().to_string(), edge.latency()));

    adj_list
    .entry(edge.target().node_name().to_string())
    .or_insert_with(Vec::new)
    .push((edge.source().node_name().to_string(), edge.latency()));
    }

    adj_list
}

fn dijkstra(adj_list: &AdjacencyList, start: &str, destination: &str) -> Option<(i16, Vec<String>)> {
    let mut distances = HashMap::new();
    let mut heap = BinaryHeap::new();
    let mut predecessors = HashMap::new();

    // Initialize distances
    for node in adj_list.keys() {
        distances.insert(node.clone(), i16::MAX);
    }

    distances.insert(start.to_string(), 0);
    heap.push(Reverse((0, start.to_string())));

    while let Some(Reverse((cost, node))) = heap.pop() {
        if node == destination {
            let mut path = Vec::new();
            let mut current = destination;
            while let Some(pred) = predecessors.get(current) {
                path.push(current.to_string());
                current = pred;
            }
            path.push(start.to_string());
            path.reverse();
            return Some((cost, path));
        }

        if cost > *distances.get(&node).unwrap_or(&i16::MAX) {
            continue;
        }

        if let Some(neighbors) = adj_list.get(&node) {
            for (neighbor, weight) in neighbors {
                let next_cost = cost + weight;
                if next_cost < *distances.get(neighbor).unwrap_or(&i16::MAX) {
                    distances.insert(neighbor.clone(), next_cost);
                    predecessors.insert(neighbor.clone(), node.);
                    heap.push(Reverse((next_cost, neighbor.clone())));
                }
            }
        }
    }

    None // Return None if no path is found
}
pub fn apply_skylark_heuristic(shortest_path: Vec<SkylarkNode>, slo: SkylarkSLOs, node_type: NodeType) -> Result<SkylarkNode> {
    let mut
    for node in shortest_path {

    }
}