# Example Object Detector Service
Takes an image as input and simulates image detection by performing hash workloads and outputing a dataset.

### Build locally
```bash
cargo build --target wasm32-wasi --release

# Optional: optimize using `wasmedge compile`
wasmedge compile target/wasm32-wasi/release/ex_obj_detector.wasm ex_obj_detector.wasm
```
### Docker Build and Push
```bash
docker buildx build --platform wasi/wasm  --provenance=false -t guelmino/skylark-rsclient:latest .
docker push guelmino/skylark-rsclient:latest
```
### Deploy
```bash
kubectl apply -f rsclient-service.yaml
```
### Troubleshoot
```bash
kubectl describe pod skylark-rsclient
kubectl logs skylark-rsclient-00001-deployment-
```
### Remove
```bash
kubectl delete ksvc skylark-rsclient
```
### Access Redis
```bash
kubectl exec -it redis-HASH -- redis-cli
```


