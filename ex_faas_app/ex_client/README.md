# Example Client Service
Takes a raw image as input and sequentially invokes `f1 -> f2 -> f3`.

### Build locally
```bash
cargo build --target wasm32-wasip1 --release

# Optional: optimize using `wasmedge compile`
wasmedge compile target/wasm32-wasip1/release/ex_client.wasm ex_client.wasm
```

### Docker Build and Push
```bash
docker buildx build --platform wasi/wasm  --provenance=false -t guelmino/skylark-ex-client:0.2.202 .
docker push guelmino/skylark-ex-client:0.2.202
```
### Deploy
```bash
kubectl apply -f ~/deployment/service/ex-client-service.yaml
kubectl delete -f ~/deployment/service/ex-client-service.yaml
kubectl delete ksvc skylark-ex-client && kubectl delete route skylark-ex-client  && kubectl delete svc skylark-ex-client && kubectl delete configuration skylark-ex-client

```
### Troubleshoot
```bash
kubectl get pods -o wide
kubectl describe pod skylark-ex-client
kubectl logs skylark-ex-client-00001-deployment-
curl -v http://10.152.183.159/?size=800\&mode=Cloud -H "Host: skylark-ex-client.default.svc.cluster.local"
curl -v http://10.152.183.159/health -H "Host: skylark-ex-client.default.svc.cluster.local"

```

