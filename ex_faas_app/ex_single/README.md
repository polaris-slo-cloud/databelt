# Example Storage retrieval (Single) Service
Retrieves single state of variable size, does dummy computation on it and finally stores the state.

### Build locally
```bash
cargo build --target wasm32-wasip1 --release

# Optional: optimize using `wasmedge compile`
wasmedge compile target/wasm32-wasip1/release/ex_single.wasm ex_single.wasm
```

### Docker Build and Push
```bash
docker buildx build --platform wasi/wasm  --provenance=false -t guelmino/skylark-ex-single:0.2.803 .
docker push guelmino/skylark-ex-single:0.2.803
```
### Deploy
```bash
kubectl apply -f ~/deployment/service/ex-single-service.yaml
kubectl delete ksvc skylark-ex-single && kubectl delete route skylark-ex-single && kubectl delete configuration skylark-ex-single && kubectl delete svc skylark-ex-single

```
### Troubleshoot
```bash
# params: policy, destination
curl -v http://pi5u4-single.default.svc.cluster.local/?policy=Skylark\&destination=pi5u1\&key=debugchain\:pi5u4-single
curl -v http://10.152.183.86/?policy=Skylark\&destination=pi5u1\&key=debugchain\:pi5u4-single -H "Host: pi5u4-single.default.svc.cluster.local"
curl -v http://10.152.183.86/health -H "Host: pi5u4-single.default.svc.cluster.local"
curl -v http://pi5u4-single.default.svc.cluster.local/health
```


