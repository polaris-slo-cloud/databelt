# Example Image Preprocessor Service `f1`
Takes a raw image as input and simulates image preprocessing by performing hash workloads and outputing a dataset.
This output triggers the object detector function `f2`.

### Build locally
```bash
cargo build --target wasm32-wasi --release

# Optional: optimize using `wasmedge compile`
wasmedge compile target/wasm32-wasi/release/ex_preprocess.wasm ex_preprocess.wasm
```

### Docker Build and Push
```bash
docker buildx build --platform wasi/wasm  --provenance=false -t guelmino/skylark-ex-preprocess:latest .
docker push guelmino/skylark-ex-preprocess:latest
```
### Deploy
```bash
microk8s kubectl apply -f ex-preprocess-service.yaml
```
### Troubleshoot
```bash
kubectl describe pod skylark-ex-preprocess
kubectl logs skylark-ex-preprocess-00001-deployment-
```
### Remove
```bash
kubectl delete ksvc skylark-ex-preprocess
```
### Access Redis
```bash
kubectl exec -it redis-HASH -- redis-cli
```


