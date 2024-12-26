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
docker buildx build --platform wasi/wasm  --provenance=false -t guelmino/skylark-ex-detect:0.2.417 .
docker push guelmino/skylark-ex-detect:0.2.417
```
### Deploy
```bash
kubectl apply -f ~/deployment/service/ex-detect-service.yaml
kubectl delete ksvc skylark-ex-detect && kubectl delete route skylark-ex-detect && kubectl delete configuration skylark-ex-detect && kubectl delete svc skylark-ex-detect
```
### API
```bash
# params: key, policy, destination
curl http://pi5u4-detect.default.svc.cluster.local/?key=debugchain\:pi5u4-preprocess\&policy=Skylark\&destination=pi5u1
curl -v http://10.152.183.86/?policy=Skylark\&destination=pi5u1\&key= -H "Host: pi5u4-detect.default.svc.cluster.local"
curl http://pi5u4-detect.default.svc.cluster.local/?key=24b32795-69a5-4ac1-9762-e0f644abd0bf\:pi5u4-preprocess\&policy=Skylark\&destination=pi5u1
curl http://pi4u8-detect.default.svc.cluster.local/health

```

### Access Redis
```bash
kubectl exec -it redis-2v4gc -- redis-cli
```


