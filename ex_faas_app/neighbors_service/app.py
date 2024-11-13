from flask import Flask, jsonify
from kubernetes import client, config
import random

app = Flask(__name__)

# Configure Kubernetes client
try:
    config.load_incluster_config()  # Load config from within the cluster
except config.ConfigException:
    # Fallback to local kubeconfig if running outside the cluster (for testing)
    config.load_kube_config()

v1 = client.CoreV1Api()

def get_node_info():
    """Fetches a list of nodes in the Kubernetes cluster with their internal IPs and names."""
    nodes_info = []
    try:
        # Get the list of nodes in the cluster
        nodes = v1.list_node().items
        for node in nodes:
            # Extract node name and internal IP address
            name = node.metadata.name
            addresses = node.status.addresses
            internal_ip = next((addr.address for addr in addresses if addr.type == "InternalIP"), None)
            if internal_ip:
                nodes_info.append({"name": name, "host": internal_ip, "latency": random.randint(1, 100)})
    except Exception as e:
        print(f"Error fetching node information: {e}")
    return nodes_info

@app.route("/neighbors", methods=["GET"])
def neighbors():
    """Returns a JSON response with information about 1-3 neighboring nodes."""
    nodes_info = get_node_info()

    if not nodes_info:
        return jsonify({"error": "No nodes found or unable to fetch nodes info"}), 500

    # Randomly select 1-3 nodes to simulate neighboring nodes
    neighbor_nodes = random.sample(nodes_info, min(len(nodes_info), random.randint(1, 3)))
    return jsonify(neighbor_nodes)

if __name__ == "__main__":
    app.run(host="0.0.0.0", port=8080)
