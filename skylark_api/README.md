# Skylark API
Exposes http endpoints for state retrieval and -propagation mechanisms. 

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
docker buildx build --platform wasi/wasm --provenance=false -t guelmino/skylark-api:0.2.738 .
docker push guelmino/skylark-api:0.2.738
```

### Deploy
```bash
# DaemonSet
kubectl apply -f ~/deployment/daemonset/skylark-api-daemonset.yaml
kubectl delete daemonset skylark-api-daemonset
```

### Troubleshoot
```bash
curl -v  http://10.0.0.34:8081/health; curl -v  http://10.0.0.45:8081/health; curl -v  http://10.0.0.167:8081/health
curl http://10.0.0.34:8081/refresh; curl http://10.0.0.45:8081/refresh; curl http://10.0.0.167:8081/refresh; curl http://10.0.0.58:8081/refresh; curl http://10.0.0.122:8081/refresh; curl http://10.0.0.210:8081/refresh; curl http://10.0.0.245:8081/refresh; curl http://10.0.0.243:8081/refresh

curl - http://10.0.0.167:8081/storage-node?size=3000\&time=57\&policy=Skylark\&destination=pi5u1
curl -v http://10.0.0.34:8081/save/edge -H "Content-Type: application/json" -d '{"key": {"chain_id": "ch1","task_id": "fn1"},"value": "V0.2.738E"}'
curl -v http://10.0.0.34:8081/save/cloud -H "Content-Type: application/json" -d '{"key": {"chain_id": "ch1","task_id": "fn1"},"value": "V0.2.738C"}'
curl -v http://10.0.0.34:8081/save/sat -H "Content-Type: application/json" -d '{"key": {"chain_id": "ch1","task_id": "fn1"},"value": "V0.2.738S"}'
curl -v http://10.0.0.34:8081/save/sat -H "Content-Type: application/json" -d '{"key":{"chain_id":"78599338-10aa-41be-961e-227d91b690be","task_id":"ex_preprocess"},"value":"11b430a1795c0608903b6d6f4ff2565b32c3456c0ddc74ad4ef2fc92205b211a"}'
curl -v http://10.0.0.34:8081/state?key=ch1:fn1
curl -X DELETE -v http://10.0.0.34:8081/state?key=ch1:fn1

kubectl get pods -o wide
kubectl describe pod skylark-api
kubectl logs skylark-api-00001-deployment-
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
| NODE_INFO_PORT | <port number> | example: 8080 |   
