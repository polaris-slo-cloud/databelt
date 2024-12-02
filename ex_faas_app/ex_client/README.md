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
docker buildx build --platform wasi/wasm  --provenance=false -t guelmino/skylark-ex-client:latest .
docker push guelmino/skylark-ex-client:latest
```
### Deploy
```bash
kubectl apply -f ~/deployment/service/ex-client-service.yaml
```
### Troubleshoot
```bash
kubectl get pods -o wide
kubectl describe pod skylark-ex-client
kubectl logs skylark-ex-client-00001-deployment-
curl -X POST -v http://10.152.183.152/ -H "Host: skylark-ex-client.default.svc.cluster.local" -d "skldfjerg"

```
### Remove
```bash
kubectl delete ksvc skylark-ex-client
```
### Access Redis
```bash
kubectl exec -it redis-HASH -- redis-cli
```


