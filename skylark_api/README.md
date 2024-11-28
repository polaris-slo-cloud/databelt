# Skylark library
Provides apis for state retrieval and -propagation mechanisms. 

### Build locally
```bash
cargo build --target wasm32-wasip1 --release
```
### Optional: optimize using `wasmedge compile`
```bash
wasmedge compile target/wasm32-wasip1/release/skylark_api.wasm skylark_api.wasm
```
### Docker Build and Push
```bash
docker buildx build --platform wasi/wasm --provenance=false -t guelmino/skylark-api:latest .
docker push guelmino/skylark-api:latest
```

### Deploy
```bash
kubectl apply -f ~/deployment/daemonset/skylark-api-daemonset.yaml
kubectl apply -f ~/deployment/daemonset/skylark-api-headless.yaml
```
### Troubleshoot
```bash
kubectl get pods -o wide
kubectl describe pod skylark-api
kubectl logs skylark-api-00001-deployment-
curl -X POST -v http://10.152.183.152/health -H "Host: skylark-api.default.svc.cluster.local" -d "skldfjerg"

```
### Remove
```bash
kubectl delete ksvc skylark-skylark-api
```
### Access Redis
```bash
kubectl exec -it redis-HASH -- redis-cli
```

### Env Vars
The following environment variables may be set to control Skylark's behavior

| Name              | Value         | Description                                                  |
|-------------------|---------------|--------------------------------------------------------------|
| REDIS_LOCAL_URL   | <REDIS_URI>   | example: redis://redis.default.svc.cluster.local:6379        |   
| REDIS_CLOUD_URL   | <REDIS_URI>   |                                                              |
| NODE_PROVIDER_URL | <KNATIVE_URL> | example: http://skylark-api-neighbors.default.svc.cluster.local  |
