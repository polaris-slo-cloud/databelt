# Example Bundled Function
Retrieves bundled state of variable size for a variable amount of simulated embedded functions, does dummy computation on it and finally stores bundled state.

### Build locally
```bash
cargo build --target wasm32-wasip1 --release

# Optional: optimize using `wasmedge compile`
wasmedge compile target/wasm32-wasip1/release/ex_bundled.wasm ex_bundled.wasm
```

### Docker Build and Push
```bash
docker buildx build --platform wasi/wasm  --provenance=false -t guelmino/skylark-ex-bundled:0.2.911 .
docker push guelmino/skylark-ex-bundled:0.2.911
```
### Deploy
```bash
kubectl apply -f ~/deployment/service/ex-bundled-service.yaml
kubectl delete ksvc skylark-ex-bundled && kubectl delete route skylark-ex-bundled && kubectl delete configuration skylark-ex-bundled && kubectl delete svc skylark-ex-bundled

```
### Troubleshoot
```bash
# params: policy, destination
# 0c10e7a2-4b6f-4167-a5cd-2540d6d4b5f2:9adc8480-60f5-4efc-a85c-cd32d4c99b61
curl -v http://pi5u2-bundled.default.svc.cluster.local/get-and-set?policy=Stateless\&destination=pi5u2\&key=0c10e7a2-4b6f-4167-a5cd-2540d6d4b5f2\:9adc8480-60f5-4efc-a85c-cd32d4c99b61
curl -v http://10.152.183.251/get-and-set?destination=pi5u2\&key=7f97c9a1-34de-455a-8e09-62fc18912f33\:10.0.0.34\:dc00973a-fb5f-47f7-8033-aa14ac2f0b7a -H "Host: pi5u2-bundled.default.svc.cluster.local"
curl -v http://10.152.183.251/health -H "Host: pi5u4-bundled.default.svc.cluster.local"
curl -v http://pi5u4-bundled.default.svc.cluster.local/health
```


