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
docker buildx build --platform wasi/wasm  --provenance=false -t guelmino/skylark-ex-obj-detector:latest .
docker push guelmino/skylark-ex-obj-detector:latest
```
### Deploy
```bash
microk8s kubectl apply -f ex-obj-detector-service.yaml
```
### Troubleshoot
```bash
kubectl describe pod skylark-ex-obj-detector
kubectl logs skylark-ex-obj-detector-
```
### Remove
```bash
kubectl delete ksvc skylark-ex-obj-detector
```
### Access Redis
```bash
kubectl exec -it redis-HASH -- redis-cli
```


