# Example Fire Alarm Service `f3`
Takes an image from `f2` as input and simulates setting of an alarm by performing hash workloads and outputing a dataset.

### Build locally
```bash
cargo build --target wasm32-wasi --release

# Optional: optimize using `wasmedge compile`
wasmedge compile target/wasm32-wasi/release/evnt_preprocessor.wasm evnt_preprocessor.wasm
```
### Docker Build and Push
```bash
docker buildx build --platform wasi/wasm  --provenance=false -t guelmino/skylark-evnt-preprocessor:latest .
docker push guelmino/skylark-evnt-preprocessor:latest
```
### Deploy
```bash
kubectl apply --force -f ~/deployment/service/evnt-preprocessor-service.yaml
```
### Troubleshoot
```bash
kubectl get pods
kubectl describe pod skylark-evnt-preprocessor
kubectl logs skylark-evnt-preprocessor-00001-deployment-
```
### Remove
```bash
kubectl delete ksvc skylark-evnt-preprocessor
```
### API
```bash
# generate hash and store it to local redis
curl GET -v http://10.152.183.152/hash -H "Host: skylark-evnt-preprocessor.default.svc.cluster.local"
```
### Access Redis
```bash
kubectl exec -it redis-HASH -- redis-cli
```


