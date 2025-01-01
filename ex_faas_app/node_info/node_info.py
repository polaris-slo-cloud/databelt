import json
import random
from datetime import datetime

from flask import Flask, jsonify, request
from kubernetes import client, config
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
    "nodes": {},
    "objectives": {},
    "topologies": [],
    "local_node_name": "Unknown",
    "cloud_node_name": "Unknown"
}

SAT_NODES = []

def update_topology_weights():
    topologies = STATE["topologies"]
    for topology in topologies:
        for edge in topology:
            if edge["source"]["node_type"] == "Cloud" or edge["target"]["node_type"] == "Cloud":
                edge["latency"] = random.randint(int(os.environ['CLOUD_LATENCY_LOWER']), int(os.environ['CLOUD_LATENCY_UPPER']))
            else:
                edge["latency"] = random.randint(int(os.environ['SAT_LATENCY_LOWER']), int(os.environ['SAT_LATENCY_UPPER']))




def set_nodes():
    """Fetches nodes from the Kubernetes cluster."""
    app.logger.info("initializing cluster node info")
    STATE["nodes"].clear()
    try:
        STATE["local_node_name"] = os.getenv('NODE_NAME', 'Unknown')
        nodes = v1.list_node().items
        app.logger.info("got node list")
        for node in nodes:
            node_name = node.metadata.name
            app.logger.info(f"got node name: {node_name}")
            node_type = node.metadata.labels.get('node-type', 'Unknown')
            app.logger.info(f"got node type: {node_type}")
            if node_type == "Cloud":
                STATE["cloud_node_name"] = node_name
                app.logger.info(f"got cloud node name: {node_name}")
            node_conditions = node.status.conditions
            ready_status = next((cond.status for cond in node_conditions if cond.type == "Ready"), "Unknown")
            if ready_status.__eq__("Unknown"): continue
            addresses = node.status.addresses
            internal_ip = next((addr.address for addr in addresses if addr.type == "InternalIP"), None)
            if internal_ip:
                app.logger.info(f"got internal ip: {internal_ip}")
                STATE["nodes"][node_name] = {
                    "node_name": node_name,
                    "node_ip": internal_ip,
                    "node_type": node_type
                }
    except Exception as e:
        print(f"Error fetching node information: {e}")


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


@app.route("/current-topology", methods=["GET"])
def current_topology():
    """Returns a JSON response with node and edge information."""
    app.logger.info("current_topology: incoming")
    try:
        update_topology_weights()
        t = datetime.now().minute % 7
        return jsonify({'edges': STATE.get('topologies')[t]}), 200
    except Exception as e:
        return jsonify({'error': str(e)}), 500


@app.route("/static-topology", methods=["GET"])
def static_topology():
    """Returns a JSON response with node and edge information."""
    app.logger.info("static_topology: incoming")
    try:
        update_topology_weights()
        t = request.args.get('t')
        if t is None:
            t = 6
        return jsonify({'edges': STATE.get('topologies')[int(t)]}), 200
    except Exception as e:
        return jsonify({'error': str(e)}), 500


@app.route("/next-topology", methods=["GET"])
def next_topology():
    """Returns a JSON response with node and edge information."""
    app.logger.info("next_topology: incoming")
    try:
        update_topology_weights()
        t = ((datetime.now().minute + 1) % 60) % 7
        return jsonify({'edges': STATE.get('topologies')[t]}), 200
    except Exception as e:
        return jsonify({'error': str(e)}), 500


@app.route("/health", methods=["GET"])
def health():
    """Returns a JSON response with node and edge information."""
    app.logger.info("health: incoming")
    return "Up and running!\n", 200


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


def init():
    app.logger.info("Initializing...")
    set_nodes()
    app.logger.info("initializing objectives")
    with open("settings/topologies.json") as f:
        STATE["topologies"].clear()
        STATE["topologies"] = json.load(f)
    with open("settings/slo-settings.json") as f:
        STATE["objectives"].clear()
        STATE["objectives"] = json.load(f)
    app.logger.info("initializing simulation settings")


if __name__ == "__main__":
    app.logger.info("Skylark Node Service is starting")
    init()
    app.run(host="0.0.0.0", port=8080)
