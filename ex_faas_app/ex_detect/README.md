# Example Object Detector Service `f2`
Takes an image from `f1` as input and simulates image detection by performing hash workloads and outputing a dataset.
The output data is sent to `f3` if a wildfire is detected.

### Build locally
```bash
cargo build --target wasm32-wasip1 --release

# Optional: optimize using `wasmedge compile`
wasmedge compile target/wasm32-wasip1/release/ex_detect.wasm ex_detect.wasm
```
### Docker Build and Push
```bash
docker buildx build --platform wasi/wasm  --provenance=false -t guelmino/skylark-ex-detect:0.2.401 .
docker push guelmino/skylark-ex-detect:0.2.401
```
### Deploy
```bash
kubectl apply -f ~/deployment/service/ex-detect-service.yaml
kubectl delete ksvc skylark-ex-detect && kubectl delete route skylark-ex-detect && kubectl delete configuration skylark-ex-detect && kubectl delete svc skylark-ex-detect
```
### Troubleshoot
```bash
kubectl get pods
kubectl describe pod skylark-ex-detect
kubectl logs skylark-ex-detect-00001-deployment-
```
### API
```bash
# generate hash and store it to local redis
curl -v http://10.152.183.159/?key=ch1:fn1 -H "Host: skylark-ex-detect.default.svc.cluster.local"
curl -v http://10.152.183.159/health -H "Host: skylark-ex-detect.default.svc.cluster.local"
```

### Access Redis
```bash
kubectl exec -it redis-2v4gc -- redis-cli
```


