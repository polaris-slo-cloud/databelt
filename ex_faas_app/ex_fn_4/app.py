from flask import Flask
import redis
import os
import hashlib
import random
import string

app = Flask(__name__)

def generate_random_data(size_kb=500):
    data = ''.join(random.choices(string.ascii_letters + string.digits, k=size_kb * 1024))
    return data

@app.route("/", methods=["POST"])
def store_random_data_hash():
    redis_host = os.getenv("REDIS_HOST", "redis")
    redis_port = int(os.getenv("REDIS_PORT", 6379))
    redis_client = redis.StrictRedis(host=redis_host, port=redis_port, db=0)

    data = generate_random_data()
    data_hash = hashlib.sha256(data.encode()).hexdigest()

    redis_client.set(data_hash, data)
    return "Data stored", 200

if __name__ == "__main__":
    app.run(host="0.0.0.0", port=8080)
