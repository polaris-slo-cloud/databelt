use crate::model::{
    Graph, NodeGraph, NodePath, SkylarkSLOs,
};
use rand::{Rng};
use std::collections::{BinaryHeap, HashMap};
use std::env;

pub fn build_graph(node_graph: &NodeGraph) -> Graph {
    let mut graph: Graph = HashMap::new();
    for edge in node_graph.edges() {
        graph
            .entry(edge.source().node_name().to_string())
            .or_insert_with(Vec::new)
            .push((edge.target().node_name().to_string(), edge.latency()));
        graph
            .entry(edge.target().node_name().to_string())
            .or_insert_with(Vec::new)
            .push((edge.source().node_name().to_string(), edge.latency()));
    }
    graph
}
fn dijkstra(graph: &Graph, start: &String, destination: &String) -> NodePath {
    let mut distances: HashMap<String, i64> = HashMap::new();
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

        if node.eq(destination) {
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
                if next_cost < *distances.get(neighbor).unwrap_or(&i64::MAX) {
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
    while let Some(predecessor) = predecessors.get(current_node) {
        let cost = *distances.get(current_node).unwrap();
        debug!(
            "dijkstra: Path step: Node: {}, Cost: {}",
            current_node, cost
        );
        path.push((cost, current_node.clone()));
        current_node = predecessor;
    }
    path.push((0, start.clone()));
    debug!("dijkstra: Final reverse path: {:?}", path);
    path
}
pub fn apply_skylark_policy(
    start: &String,
    destination: &String,
    size: i64,
    time: i64,
    graph: &Graph,
    slo: &SkylarkSLOs,
) -> Option<String> {
    debug!("apply_skylark_heuristic: start");
    if time > slo.max_latency() {
        warn!("Already violating SLO!, ABORTING");
        return None;
    }
    let reverse_path = dijkstra(&graph, start, destination);
    let avg_bandwidth = env::var("AVG_SAT_BANDWIDTH").unwrap().parse::<i64>().unwrap();
    info!("Computed path: {:?}", reverse_path.len());
    info!("T(ex): {:?}", time);
    info!("D(f): {:?}", size);
    info!("Tmax(f): {:?}", slo.max_latency());
    info!("B(sat): {:?}", avg_bandwidth);
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
        let mig_time = calc_migration_time(size, avg_bandwidth, step.0);
        info!("Tmig({}): {:?}", step.1, mig_time);
        debug!(
            "apply_skylark_heuristic: time: {}, mig_time: {}, latency: {}",
            time, mig_time, step.0
        );
        if (time + mig_time) > slo.max_latency() {
            continue;
        }

        debug!(
            "apply_skylark_heuristic: elected node: {:?} with latency {}",
            step.1.clone(),
            step.0
        );
        info!("Propagated: True");
        return Some(step.1.clone());
    }
    debug!("apply_skylark_heuristic: No node was elected even though path is not empty!");
    info!("Propagated: False");
    None
}

pub fn apply_random_policy(
    start: &String,
    destination: &String,
    graph: &Graph,
) -> Option<String> {
    info!("apply_random_policy: start");
    let reverse_path = dijkstra(&graph, start, destination);
    if reverse_path.is_empty() {
        warn!("apply_random_policy: emtpy node path given, returning None");
        return None;
    }

    let mut rng = rand::thread_rng();
    let random_number = rng.gen_range(0..=reverse_path.len()-1);
    debug!("apply_random_policy: Random number: {}", random_number);
    let random_step = reverse_path.get(random_number).unwrap();
    debug!("apply_random_policy: elected node: {:?}", random_step.1);
    Some(random_step.1.clone())
}

fn calc_migration_time(s: i64, b: i64, l: i64) -> i64 {
    l + (s / (100 * b)) + l
}
