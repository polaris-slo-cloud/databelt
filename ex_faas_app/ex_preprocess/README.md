# Example Image Preprocessor Service `f1`
Takes a raw image as input and simulates image preprocessing by performing hash workloads and outputing a dataset.
This output triggers the object detector function `f2`.

### Build locally
```bash
cargo build --target wasm32-wasip1 --release

# Optional: optimize using `wasmedge compile`
wasmedge compile target/wasm32-wasip1/release/ex_preprocess.wasm ex_preprocess.wasm
```

### Docker Build and Push
```bash
docker buildx build --platform wasi/wasm  --provenance=false -t guelmino/skylark-ex-preprocess:latest .
docker push guelmino/skylark-ex-preprocess:latest
```
### Deploy
```bash
kubectl apply -f ~/deployment/service/ex-preprocess-service.yaml
```
### Troubleshoot
```bash
kubectl get pods -o wide
kubectl describe pod skylark-ex-preprocess
kubectl logs skylark-ex-preprocess-00001-deployment-
curl -X POST -v http://10.152.183.152/ -H "Host: skylark-ex-preprocess.default.svc.cluster.local" -d "skldfjerg"

```
### Remove
```bash
kubectl delete ksvc skylark-ex-preprocess
```
### Access Redis
```bash
kubectl exec -it redis-HASH -- redis-cli
```


