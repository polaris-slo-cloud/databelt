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
docker buildx build --platform wasi/wasm  --provenance=false -t guelmino/skylark-ex-client:0.2.203 .
docker push guelmino/skylark-ex-client:0.2.203
```
### Deploy
```bash
kubectl apply -f ~/deployment/service/ex-client-service.yaml
kubectl delete -f ~/deployment/service/ex-client-service.yaml
kubectl delete ksvc skylark-ex-client && kubectl delete route skylark-ex-client  && kubectl delete svc skylark-ex-client && kubectl delete configuration skylark-ex-client

```
### Troubleshoot
```bash
curl -v http://pi5u4-preprocess.default.svc.cluster.local/process?policy=Skylark\&destination=pi5u1\&node_path=pi5u2,pu5u4,pi5u1\&size_mb=1

alias clc='curl -v http://pi5u2-client.default.svc.cluster.local/process?policy=Skylark\&destination=pi5u1\&node_path=pi5u2,pu5u4,pi5u1\&size_mb=1 -H "Host: skylark-ex-client.default.svc.cluster.local"'
alias cls='curl -v http://10.152.183.159/?size=400\&mode=Sat -H "Host: skylark-ex-client.default.svc.cluster.local"'

# 20 concurrent 
clc & clc & clc & clc & clc & clc & clc & clc & clc & clc & clc & clc & clc & clc & clc & clc & clc & clc & clc & clc 
cls & cls & cls & cls & cls & cls & cls & cls & cls & cls & cls & cls & cls & cls & cls & cls & cls & cls & cls & cls
```

