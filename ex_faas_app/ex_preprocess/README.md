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
docker buildx build --platform wasi/wasm  --provenance=false -t guelmino/skylark-ex-preprocess:0.2.322 .
docker push guelmino/skylark-ex-preprocess:0.2.322
```
### Deploy
```bash
kubectl apply -f ~/deployment/service/ex-preprocess-service.yaml
kubectl delete ksvc skylark-ex-preprocess && kubectl delete route skylark-ex-preprocess && kubectl delete configuration skylark-ex-preprocess && kubectl delete svc skylark-ex-preprocess

```
### Troubleshoot
```bash
# params: policy, destination
curl -v http://pi5u4-preprocess.default.svc.cluster.local/?policy=Skylark\&destination=pi5u1\&img=eo-2K.jpeg
curl -v http://10.152.183.86/?policy=Skylark\&destination=pi5u1\&img=eo-2K.jpeg -H "Host: pi5u4-preprocess.default.svc.cluster.local"
curl -v http://10.152.183.86/health -H "Host: pi5u4-preprocess.default.svc.cluster.local"
curl -v http://pi5u4-preprocess.default.svc.cluster.local/health
```


