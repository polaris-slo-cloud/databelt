# Write Data Function
Helper function writing data of variable size to KV stores. 

### Build locally
```bash
cargo build --target wasm32-wasip1 --release

# Optional: optimize using `wasmedge compile`
wasmedge compile target/wasm32-wasip1/release/write_data.wasm write_data.wasm
```

### Docker Build and Push
```bash
docker buildx build --platform wasi/wasm  --provenance=false -t guelmino/skylark-write-data:0.1.1004 .
docker push guelmino/skylark-write-data:0.1.1004
```
### Deploy
```bash
kubectl apply -f ~/deployment/service/write-data-service.yaml
kubectl delete ksvc skylark-write-data && kubectl delete route skylark-write-data && kubectl delete configuration skylark-write-data && kubectl delete svc skylark-write-data

```
### Troubleshoot
```bash
# params: policy, destination
curl -v http://10.0.0.243:8084/single?policy=Stateless\&destination=pi5u1\&stype=Single\&size=500\&scount=1
curl -v http://10.0.0.243:8084/single?policy=Stateless\&destination=pi5u1\&stype=Single\&size=500\&scount=1
curl -v http://10.0.0.243:8084/single?policy=Stateless\&destination=pi5u1\&stype=Single\&size=500\&scount=1
curl -v http://10.0.0.243:8084/single?policy=Stateless\&destination=pi5u1\&stype=Single\&size=500\&scount=1
curl -v http://10.0.0.243:8084/single?policy=Stateless\&destination=pi5u1\&stype=Single\&size=500\&scount=1

curl -v http://10.0.0.243:8084/single?policy=Stateless\&destination=pi4u5\&stype=Single\&size=500\&scount=1
curl -v http://10.0.0.243:8084/single?policy=Stateless\&destination=pi4u5\&stype=Single\&size=500\&scount=1
curl -v http://10.0.0.243:8084/single?policy=Stateless\&destination=pi4u5\&stype=Single\&size=500\&scount=1
curl -v http://10.0.0.243:8084/single?policy=Stateless\&destination=pi4u5\&stype=Single\&size=500\&scount=1
curl -v http://10.0.0.243:8084/single?policy=Stateless\&destination=pi4u5\&stype=Single\&size=500\&scount=1

curl -v http://10.0.0.243:8084/bundled?policy=Stateless\&stype=Bundled\&destination=pi5u2\&size=100\&scount=3
curl -v http://10.152.183.86/?policy=Skylark\&destination=pi5u1\&img=eo-2K.jpeg -H "Host: pi5u4-write-data.default.svc.cluster.local"
curl -v http://10.152.183.86/health -H "Host: pi5u4-write-data.default.svc.cluster.local"
curl -v http://pi5u4-write-data.default.svc.cluster.local/health
```


