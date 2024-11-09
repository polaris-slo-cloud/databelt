# Example Object Detector Service
Takes an image as input and simulates image detection by performing hash workloads and outputing a dataset.

### Build locally
```bash
cargo build --target wasm32-wasi --release

# Optional: optimize using `wasmedge compile`
wasmedge compile target/wasm32-wasi/release/ex_img_preprocessor.wasm ex_img_preprocessor.wasm
```

### Docker Build and Push
```bash
docker buildx build --platform wasi/wasm  --provenance=false -t guelmino/skylark-ex-img-preprocessor:latest .
docker push guelmino/skylark-ex-img-preprocessor:latest
```
### Deploy
```bash
microk8s kubectl apply -f ex-img-preprocessor-service.yaml
```
### Troubleshoot
```bash
kubectl describe pod skylark-ex-img-preprocessor
kubectl logs skylark-ex-img-preprocessor-
```
### Remove
```bash
kubectl delete ksvc skylark-ex-img-preprocessor
```
### Access Redis
```bash
kubectl exec -it redis-HASH -- redis-cli
```


