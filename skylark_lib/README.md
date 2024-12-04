# Skylark library
Provides apis for state retrieval and -propagation mechanisms. 

### Build locally
```bash
cargo build --target wasm32-wasip1 --release
```
### Optional: optimize using `wasmedge compile`
```bash
wasmedge compile target/wasm32-wasip1/release/skylark.wasm skylark.wasm
```
### Docker Build and Push
```bash
docker buildx build --platform wasi/wasm  --provenance=false -t guelmino/skylark:latest .
docker push guelmino/skylark:latest
```

### Access Redis
```bash
kubectl exec -it redis-2v4gc -- redis-cli
```

### Env Vars
The following environment variables may be set to control Skylark's behavior

| Name              | Value         | Description                                                  |
|-------------------|---------------|--------------------------------------------------------------|
| REDIS_LOCAL_URL   | <REDIS_URI>   | example: redis://redis.default.svc.cluster.local:6379        |   
| REDIS_CLOUD_URL   | <REDIS_URI>   |                                                              |
| NODE_PROVIDER_URL | <KNATIVE_URL> | example: http://skylark-node-info.default.svc.cluster.local  |
