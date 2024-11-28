# Example Object Detector Service
Takes an image as input and simulates image detection by performing hash workloads and outputing a dataset.

### Build locally
```bash
cargo build --target wasm32-wasip1 --release

# Optional: optimize using `wasmedge compile`
wasmedge compile target/wasm32-wasip1/release/ex_detect.wasm ex_detect.wasm
```
### Docker Build and Push
```bash
docker buildx build --platform wasi/wasm  --provenance=false -t guelmino/skylark-rsclient:latest .
docker push guelmino/skylark-rsclient:latest
```
### Deploy
```bash
kubectl apply -f ~/deployment/service/rsclient-service.yaml
```
### Troubleshoot
```bash
kubectl describe pod skylark-rsclient
kubectl logs skylark-rsclient-00001-deployment-
```
### Remove
```bash
kubectl delete ksvc skylark-rsclient
```
### Access Redis
```bash
kubectl exec -it redis-2v4gc -- redis-cli
```


