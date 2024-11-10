# Example Object Detector Service
Takes an image as input and simulates image detection by performing hash workloads and outputing a dataset.

### Build locally
```bash
cargo build --target wasm32-wasi --release

# Optional: optimize using `wasmedge compile`
wasmedge compile target/wasm32-wasi/release/reqwestclient.wasm reqwestclient.wasm
```
### Docker Build and Push
```bash
docker buildx build --platform wasi/wasm  --provenance=false -t guelmino/skylark-reqwestclient:latest .
docker push guelmino/skylark-reqwestclient:latest
```
### Deploy
```bash
kubectl apply -f reqwestclient-service.yaml
```
### Run
```bash
curl -X GET -v -H "Host: skylark-reqwestclient.default.svc.cluster.local" http://10.152.183.152
```
### Troubleshoot
```bash
kubectl get pods
kubectl describe pod skylark-reqwestclient
kubectl logs skylark-reqwestclient-00001-deployment-
```
### Remove
```bash
kubectl delete ksvc skylark-reqwestclient
```
### Access Redis
```bash
kubectl exec -it redis-HASH -- redis-cli
```


