import json
import time
import redis
from flask import Flask, jsonify, request
from kubernetes import client, config
import random
import os

from logging.config import dictConfig

dictConfig({
    'version': 1,
    'formatters': {'default': {
        'format': '[%(asctime)s] %(levelname)s in %(module)s: %(message)s',
    }},
    'handlers': {'wsgi': {
        'class': 'logging.StreamHandler',
        'stream': 'ext://flask.logging.wsgi_errors_stream',
        'formatter': 'default'
    }},
    'root': {
        'level': 'DEBUG',
        'handlers': ['wsgi']
    }
})
app = Flask(__name__)

try:
    config.load_incluster_config()
except config.ConfigException:
    config.load_kube_config()

v1 = client.CoreV1Api()

STATE = {
    "redis_pods": {},
    "nodes": {},
    "objectives": {},
    "sim_settings": {},
    "node_graph": [],
    "graph_updated": 0.0,
    "local_node_name": "Unknown",
    "cloud_node_name": "Unknown"
}

SAT_NODES = []


def next_topology():
    STATE["node_graph"].clear()
    if len(SAT_NODES) > 1:
        sat = SAT_NODES.pop(0)
        SAT_NODES.append(sat)
    else:
        SAT_NODES.clear()

    cloud_node = None
    edge_nodes = []
    for node in STATE["nodes"].values():
        if node["node_type"] == "Sat":
            SAT_NODES.append(node)
        elif node["node_type"] == "Cloud":
            cloud_node = node
        else:
            edge_nodes.append(node)

    if len(SAT_NODES) > 1:
        min_latency = STATE["sim_settings"]["Sat-Sat"]["min_latency"]
        max_latency = STATE["sim_settings"]["Sat-Sat"]["max_latency"]
        min_bandwidth = STATE["sim_settings"]["Sat-Sat"]["min_bandwidth"]
        max_bandwidth = STATE["sim_settings"]["Sat-Sat"]["max_bandwidth"]
        for i in range(len(SAT_NODES) - 1):
            latency = random.randint(min_latency, max_latency)
            bandwidth = random.randint(min_bandwidth, max_bandwidth)
            STATE["node_graph"].append({
                "source": SAT_NODES[i],
                "target": SAT_NODES[i + 1],
                "bandwidth": bandwidth,
                "latency": latency
            })

        if cloud_node:
            min_latency = STATE["sim_settings"]["Cloud-Sat"]["min_latency"]
            max_latency = STATE["sim_settings"]["Cloud-Sat"]["max_latency"]
            min_bandwidth = STATE["sim_settings"]["Cloud-Sat"]["min_bandwidth"]
            max_bandwidth = STATE["sim_settings"]["Cloud-Sat"]["max_bandwidth"]
            STATE["node_graph"].append({
                "source": cloud_node,
                "target": SAT_NODES[1],
                "bandwidth": random.randint(min_bandwidth, max_bandwidth),
                "latency": random.randint(min_latency, max_latency)
            })
    if edge_nodes:
        app.logger.error("TODO: implement adding edges to topology")

    STATE["graph_updated"] = int(time.time())


def set_nodes():
    """Fetches nodes from the Kubernetes cluster."""
    app.logger.info("initializing cluster node info")
    STATE["nodes"].clear()
    try:
        STATE["local_node_name"] = os.getenv('NODE_NAME', 'Unknown')
        nodes = v1.list_node().items
        for node in nodes:
            node_name = node.metadata.name
            node_type = node.metadata.labels.get('node-type', 'Unknown')
            if node_type == "Cloud":
                STATE["cloud_node_name"] = node_name
            node_conditions = node.status.conditions
            ready_status = next((cond.status for cond in node_conditions if cond.type == "Ready"), "Unknown")
            if ready_status.__eq__("Unknown"): continue
            redis_host = f'redis://{STATE["redis_pods"].get(node_name)["pod_ip"]}:6379'
            addresses = node.status.addresses
            internal_ip = next((addr.address for addr in addresses if addr.type == "InternalIP"), None)
            if internal_ip:
                STATE["nodes"][node_name] = {
                    "node_name": node_name,
                    "node_ip": internal_ip,
                    "redis_host": redis_host,
                    "node_type": node_type
                }
    except Exception as e:
        print(f"Error fetching node information: {e}")


def set_redis_pods():
    """Fetches information about Redis pods in the cluster."""
    app.logger.info("initializing Redis pod info")
    STATE["redis_pods"].clear()
    try:
        pods = v1.list_pod_for_all_namespaces(label_selector="app=redis").items
        for pod in pods:
            STATE["redis_pods"][pod.spec.node_name] = {
                "pod_name": pod.metadata.name,
                "pod_ip": pod.status.pod_ip
            }
    except Exception as e:
        print(f"Error fetching Redis pod information: {e}")


@app.route("/objectives", methods=["GET"])
def objectives():
    """Returns Cluster SLOs"""
    app.logger.info("objectives: incoming")
    return jsonify(STATE["objectives"])


@app.route("/local-node-info", methods=["GET"])
def local_node_info():
    app.logger.info("local_node_info: incoming")
    try:
        node_name = os.getenv('NODE_NAME', 'Unknown')
        return jsonify(STATE["nodes"].get(node_name)), 200
    except Exception as e:
        return jsonify({'error': str(e)}), 500


@app.route("/cloud-node-info", methods=["GET"])
def cloud_node_info():
    app.logger.info("cloud_node_info: incoming")
    try:
        nodes = v1.list_node(label_selector='node-type=Cloud').items
        if nodes is None:
            return jsonify({'error': 'no node with label "Cloud" found'}), 404

        node = nodes[0]
        node_name = node.metadata.name

        if len(nodes) > 1:
            app.logger.warning(f'Cluster has more than 1 "Cloud" node. Picking node "{node_name}"')

        return jsonify(STATE["nodes"].get(node_name)), 200

    except Exception as e:
        return jsonify({'error': str(e)}), 500


@app.route("/refresh", methods=["GET"])
def refresh():
    app.logger.info("refresh: incoming")
    try:
        init()
        return jsonify({'result': 'successfully re-initialized node service'}), 200
    except Exception as e:
        return jsonify({'error': str(e)}), 500


@app.route("/node-graph", methods=["GET"])
def node_topology():
    """Returns a JSON response with node and edge information."""
    app.logger.info("node_topology: incoming")
    try:
        if (time.time() - STATE["graph_updated"]) > 15:
            next_topology()
        return jsonify({'edges': STATE.get('node_graph')}), 200
    except Exception as e:
        return jsonify({'error': str(e)}), 500


@app.route("/health", methods=["GET"])
def health():
    """Returns a JSON response with node and edge information."""
    app.logger.info("health: incoming")
    return "Up and running!\n", 200


@app.route("/test-redis", methods=["GET"])
def test_redis():
    """tests local and global redis store"""
    app.logger.info("test_redis: incoming")
    local_node = STATE["nodes"].get(STATE["local_node_name"])
    cloud_node = STATE["nodes"].get(STATE["cloud_node_name"])
    try:
        local_redis_client = redis.from_url(local_node["redis_host"])
        app.logger.info("Connected to Local Redis successfully.")
        test_key = "local_test_key"
        test_value = "local_test_value"
        local_redis_client.set(test_key, test_value)
        app.logger.info(f"Set Local key '{test_key}' with value '{test_value}'.")
        retrieved_value = local_redis_client.get(test_key)
        app.logger.info(f"Retrieved Local key '{test_key}' with value '{retrieved_value}'.")
        local_redis_client.delete(test_key)
        app.logger.info(f"Deleted Local key '{test_key}'.")
        local_redis_client.close()

        cloud_redis_client = redis.from_url(cloud_node["redis_host"])
        app.logger.info("Connected to Cloud Redis successfully.")
        test_key = "cloud_test_key"
        test_value = "cloud_test_value"
        cloud_redis_client.set(test_key, test_value)
        app.logger.info(f"Set Cloud key '{test_key}' with value '{test_value}'.")
        retrieved_value = cloud_redis_client.get(test_key)
        app.logger.info(f"Retrieved Cloud key '{test_key}' with value '{retrieved_value}'.")
        cloud_redis_client.delete(test_key)
        app.logger.info(f"Deleted Cloud key '{test_key}'.")
        cloud_redis_client.close()

        return jsonify({"message": "Redis Test Success", "Nodes": [local_node, cloud_node]}), 200

    except redis.ConnectionError as e:
        app.logger.error(f"Redis connection error: {e}")
        return jsonify({"error": str(e)}), 500
    except Exception as e:
        app.logger.error(f"An error occurred: {e}")
        return jsonify({"error": str(e)}), 500


@app.route('/objectives', methods=['POST'])
def set_objectives():
    if not request.is_json:
        return jsonify({"error": "Request body must be JSON"}), 400
    try:
        request_data = request.get_json()
        if not isinstance(request_data, dict):
            return jsonify({"error": "JSON body must be a dictionary"}), 400

        STATE["objectives"].clear()
        STATE["objectives"].update(request_data)
        return jsonify({"message": "Data updated successfully", "objectives": STATE["objectives"]}), 200
    except Exception as e:
        return jsonify({"error": str(e)}), 500


@app.route('/sim-settings', methods=['POST'])
def set_sim_settings():
    if not request.is_json:
        return jsonify({"error": "Request body must be JSON"}), 400
    try:
        request_data = request.get_json()
        if not isinstance(request_data, dict):
            return jsonify({"error": "JSON body must be a dictionary"}), 400

        STATE["sim_settings"].clear()
        STATE["sim_settings"].update(request_data)
        return jsonify({"message": "Data updated successfully", "sim_settings": STATE["sim_settings"]}), 200
    except Exception as e:
        return jsonify({"error": str(e)}), 500


def init():
    app.logger.info("Initializing...")
    set_redis_pods()
    set_nodes()
    app.logger.info("initializing objectives")
    with open("settings/slo-settings.json") as f:
        STATE["objectives"].clear()
        STATE["objectives"] = json.load(f)
    app.logger.info("initializing simulation settings")
    with open("settings/simulation-settings.json") as f:
        STATE["sim_settings"].clear()
        STATE["sim_settings"] = json.load(f)
    next_topology()


if __name__ == "__main__":
    app.logger.info("Skylark Node Service is starting")
    init()
    app.run(host="0.0.0.0", port=8080)
