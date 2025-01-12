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
docker buildx build --platform wasi/wasm  --provenance=false -t guelmino/skylark-ex-preprocess:0.2.341 .
docker push guelmino/skylark-ex-preprocess:0.2.341
```
### Deploy
```bash
kubectl apply -f ~/deployment/service/ex-preprocess-service.yaml
kubectl delete ksvc skylark-ex-preprocess && kubectl delete route skylark-ex-preprocess && kubectl delete configuration skylark-ex-preprocess && kubectl delete svc skylark-ex-preprocess

```
### Troubleshoot
```bash
# params: policy, destination
curl -v http://pi5u2-preprocess.default.svc.cluster.local/?policy=Skylark\&destination=pi5u1\&key=8d8b97eb-8c65-4bd3-bebb-0a799895f8dd\:10.0.0.34\:f4559e7f-c6f3-4e4a-9607-16f835a76e59
curl -v http://10.152.183.251/?policy=Skylark\&destination=pi5u1\&key=1c43555e-dd95-404a-bb61-b23cea9375fe\:10.0.0.34\:687a1305-5fa3-49d3-bc22-a70fba690e61 -H "Host: pi5u2-preprocess.default.svc.cluster.local"
curl -v http://10.152.183.251/?policy=Skylark\&destination=pi5u1\&img=eo-1M.jpeg -H "Host: pi5u3-preprocess.default.svc.cluster.local"
curl -v http://10.152.183.251/health -H "Host: pi5u4-preprocess.default.svc.cluster.local"
curl -v http://pi5u4-preprocess.default.svc.cluster.local/health
```


