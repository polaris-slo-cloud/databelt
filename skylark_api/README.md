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
docker buildx build --platform wasi/wasm --provenance=false -t guelmino/skylark-api:0.2.3 . --no-cache
docker push guelmino/skylark-api:0.2.3
```

### Deploy
```bash
# DaemonSet
kubectl apply -f ~/deployment/daemonset/skylark-api-daemonset.yaml
kubectl apply -f ~/deployment/daemonset/skylark-api-nodeport.yaml
kubectl delete daemonset skylark-api-daemonset
kubectl delete service skylark-api-nodeport

```
### Troubleshoot
```bash
kubectl get pods -o wide
kubectl describe pod skylark-api
kubectl logs skylark-api-00001-deployment-
curl -X POST -v http://10.152.183.159/health -H "Host: skylark-api.default.svc.cluster.local" -d "skldfjerg"
curl -x "DELETE" -v http://10.152.183.159/state?key=KEY -H "Host: skylark-api.default.svc.cluster.local"
curl -v http://10.152.183.159/state?key=KEY -H "Host: skylark-api.default.svc.cluster.local"
curl -v http://10.152.183.159/health -H "Host: skylark-api.default.svc.cluster.local"
kubectl exec -it skylark-api-2xg2q -- nslookup skylark-node.default.svc.cluster.local
curl -v http://10.152.183.159/health -H "Host: skylark-api.default.svc.cluster.local"

```

### Access Redis
```bash
kubectl exec -it redis-HASH -- redis-cli
```
### Endpoints

| Path        | Method | Description                                                                                                                                        | Example                                                                                                          |
|-------------|--------|----------------------------------------------------------------------------------------------------------------------------------------------------|------------------------------------------------------------------------------------------------------------------|
| /state      | GET    | Get the state from a given `key` param (`SkylarkKey`)                                                                                              | curl -v http://10.152.183.159/state?key=SKYLARK-KEY -H "Host: skylark-api.default.svc.cluster.local"             | 
| /state      | DELETE | Delete the state from a given `key` param (`SkylarkKey`)                                                                                           | curl -x "DELETE" -v http://10.152.183.159/state?key=SKYLARK-KEY -H "Host: skylark-api.default.svc.cluster.local" | 
| /save/sat   | POST   | Propagate state to viable nodes and replicate to global KV store and save it to the KV Store. State is given in form of JSON body (`SkylarkState`) |                                                                                                                  |
| /save/edge  | POST   | Save state to local KV Store and replicate to global KV store. State is given in form of JSON body (`SkylarkState`)                                |                                                                                                                  |
| /save/cloud | POST   | Save state to global KV store. State is given in form of JSON body (`SkylarkState`)                                                                |                                                                                                                  |
| /health     | GET    | Health endpoint for orchestrator                                                                                                                   | curl -v http://10.152.183.159/health -H "Host: skylark-api.default.svc.cluster.local"                            |

### Env Vars
The following environment variables may be set to control Skylark's behavior

| Name           | Value         | Description                                            |
|----------------|---------------|--------------------------------------------------------|
| NODE_INFO_PORT | <port number> | example: 31016 |   
