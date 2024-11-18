import logging

from flask import Flask, request, jsonify
import redis
import base64
import os

from redis import ResponseError

app = Flask(__name__)
logger = logging.getLogger("Example EO Service")
logger.setLevel('DEBUG')
redis_host = os.getenv("REDIS_HOST", "redis.default.svc.cluster.local")
redis_port = int(os.getenv("REDIS_PORT", 6379))
redis_stream_key = os.getenv("REDIS_STREAM_KEY", "eo_image_stream")
r = redis.Redis(host=redis_host, port=redis_port, db=0)

@app.route('/direct', methods=['POST'])
def eo_image_direct():
    logger.debug("parsing image direct")
    if 'image' not in request.files:
        return jsonify({"error": "No image file provided"}), 400

    image_file = request.files['image']
    image_base64 = base64.b64encode(image_file.read()).decode('utf-8')
    r.set(image_base64[:5], image_base64)
    return jsonify({"status": f"Image uploaded and stored in local Redis with key {image_base64[:5]}"}), 200

@app.route('/stream', methods=['POST'])
def eo_image_stream():
    app.logger.debug("parsing image stream")
    if 'image' not in request.files:
        return jsonify({"error": "No image file provided"}), 400

    image_file = request.files['image']
    image_base64 = base64.b64encode(image_file.read()).decode('utf-8')
    stream_data = {"image": image_base64}
    app.logger.debug(f"image key: {redis_stream_key}")
    app.logger.debug(f"type of stream data: {type(stream_data)}")
    try:
        r.xadd(redis_stream_key, stream_data)
    except ResponseError as e:
        app.logger.debug(f"Error adding image to stream: {e}")
    return jsonify({"status": f"Image uploaded and stored in Redis stream {redis_stream_key}"}), 200

if __name__ == '__main__':
    app.logger.setLevel(os.getenv("LOG_LEVEL", "DEBUG"))
    app.logger.debug("Starting HTTP Service")
    app.run(host="0.0.0.0", port=8080)
