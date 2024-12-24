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
docker buildx build --platform wasi/wasm  --provenance=false -t guelmino/skylark-ex-alarm:0.2.512 .
docker push guelmino/skylark-ex-alarm:0.2.512
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
# params: key, policy, destination
curl http://pi5u1-alarm.default.svc.cluster.local/?key=debugchain\:debugchain:pi5u2-detect\&policy=Skylark\&destination=pi5u1
curl http://pi5u1-alarm.default.svc.cluster.local/?key=24b32795-69a5-4ac1-9762-e0f644abd0bf\:pi5u4-detect\&policy=Skylark\&destination=pi5u1
curl http://pi5u1-detect.default.svc.cluster.local/health
```
### Access Redis
```bash
kubectl exec -it redis-HASH -- redis-cli
```


