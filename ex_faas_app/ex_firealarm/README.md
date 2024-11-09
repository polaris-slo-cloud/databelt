# Example Object Detector Service
Takes an image as input and simulates image detection by performing hash workloads and outputing a dataset.

### Build locally
```bash
cargo build --target wasm32-wasi --release

# Optional: optimize using `wasmedge compile`
wasmedge compile target/wasm32-wasi/release/ex_firealarm.wasm ex_firealarm.wasm
```
### Docker Build and Push
```bash
docker buildx build --platform wasi/wasm  --provenance=false -t guelmino/skylark-ex-firealarm:latest .
docker push guelmino/skylark-ex-firealarm:latest
```
### Deploy
```bash
microk8s kubectl apply -f ex-firealarm-service.yaml
```
### Troubleshoot
```bash
kubectl describe pod skylark-ex-firealarm
kubectl logs skylark-ex-firealarm-
```
### Remove
```bash
kubectl delete ksvc skylark-ex-firealarm
```
### Access Redis
```bash
kubectl exec -it redis-HASH -- redis-cli
```


