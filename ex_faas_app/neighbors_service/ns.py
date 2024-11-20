from flask import Flask, jsonify
from kubernetes import client, config
import random
import json
import os

app = Flask(__name__)

try:
    config.load_incluster_config()
except config.ConfigException:
    # Fallback to local kubeconfig if running outside the cluster (for testing)
    config.load_kube_config()

v1 = client.CoreV1Api()

def load_config():
    """Load configuration from a file, if available."""
    config_path = os.getenv("TOPOLOGY_CONFIG_PATH", "config.json")
    if os.path.exists(config_path):
        with open(config_path, "r") as f:
            return json.load(f)
    return None

def get_default_node_info():
    """Fetches nodes from the Kubernetes cluster."""
    nodes_info = []
    try:
        nodes = v1.list_node().items
        for node in nodes:
            node_name = node.metadata.name
            node_conditions = node.status.conditions
            ready_status = next((cond.status for cond in node_conditions if cond.type == "Ready"), "Unknown")
            if ready_status.__eq__("Unknown"): continue
            addresses = node.status.addresses
            internal_ip = next((addr.address for addr in addresses if addr.type == "InternalIP"), None)
            if internal_ip:
                nodes_info.append({"name": node_name, "node_type": "Satellite"})
    except Exception as e:
        print(f"Error fetching node information: {e}")
    return nodes_info

def generate_edges(nodes):
    """Generate random edges between nodes with default parameters."""
    edges = []
    if len(nodes) > 1:
        for i in range(len(nodes) - 1):
            edges.append({
                "source": nodes[i]["name"],
                "target": nodes[i + 1]["name"],
                "bandwidth": {random.randint(50, 200)},
                "latency": {random.randint(5, 50)},
                "bandwidth_metric": "Mbps",
                "latency_metric": "ms"
            })
    return edges

@app.route("/node-topology", methods=["GET"])
def topology():
    """Returns a JSON response with node and edge information."""
    config_data = load_config()

    if config_data:
        nodes = config_data.get("nodes", [])
        edges = config_data.get("edges", [])
    else:
        nodes = get_default_node_info()
        edges = generate_edges(nodes)

    if not nodes:
        return jsonify({"error": "No nodes found or unable to fetch nodes info"}), 500

    return jsonify({"nodes": nodes, "edges": edges})


def get_redis_pods_info():
    """Fetches information about Redis pods in the cluster."""
    redis_pods_info = []
    try:
        # List all pods in the cluster with the 'app=redis' label
        pods = v1.list_pod_for_all_namespaces(label_selector="app=redis").items
        for pod in pods:
            redis_pods_info.append({
                "node": {"name": pod.spec.node_name, "node_type": "Satellite"},
                "pod_name": pod.metadata.name,
                "pod_ip": pod.status.pod_ip
            })
    except Exception as e:
        print(f"Error fetching Redis pod information: {e}")
    return redis_pods_info

@app.route("/redis-pods", methods=["GET"])
def redis_pods():
    """Returns information about Redis pods in the cluster."""
    pods_info = get_redis_pods_info()
    if not pods_info:
        return jsonify({"error": "No Redis pods found or unable to fetch pods info"}), 500
    return jsonify(pods_info)

@app.route("/slo", methods=["GET"])
def redis_pods():
    """Returns Cluster SLOs"""
    return jsonify({"bandwidth_metric": "Mbps", "latency_metric": "ms", "min_bandwidth": 100, "max_latency": 40})

if __name__ == "__main__":
    app.run(host="0.0.0.0", port=8080)
