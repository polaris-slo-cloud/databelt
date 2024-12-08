# Example Fire Alarm Service `f3`
Takes an image from `f2` as input and simulates setting of an alarm by performing hash workloads and outputing a dataset.

### Build locally
```bash
cargo build --target wasm32-wasip1 --release

# Optional: optimize using `wasmedge compile`
wasmedge compile target/wasm32-wasip1/release/ex_alarm.wasm ex_alarm.wasm
```
### Docker Build and Push
```bash
docker buildx build --platform wasi/wasm  --provenance=false -t guelmino/skylark-ex-alarm:0.2.501 .
docker push guelmino/skylark-ex-alarm:0.2.501
```
### Deploy
```bash
kubectl apply -f ~/deployment/service/ex-alarm-service.yaml
kubectl delete ksvc skylark-ex-alarm && kubectl delete route skylark-ex-alarm && kubectl delete configuration skylark-ex-alarm && kubectl delete svc skylark-ex-alarm
```
### Troubleshoot
```bash
kubectl get pods -o wide
kubectl describe pod skylark-ex-alarm
kubectl logs skylark-ex-alarm-00001-deployment-
```

### API
```bash
# generate hash and store it to local redis
curl -v http://10.152.183.159/?key=sdlj -H "Host: skylark-ex-alarm.default.svc.cluster.local"
curl -v http://10.152.183.159/health -H "Host: skylark-ex-alarm.default.svc.cluster.local"
```
### Access Redis
```bash
kubectl exec -it redis-HASH -- redis-cli
```


