# Skylark library
Provides apis for state retrieval and -propagation mechanisms. 

### Build locally
```bash
cargo build --target wasm32-wasi --release
```
### Optional: optimize using `wasmedge compile`
```bash
wasmedge compile target/wasm32-wasi/release/skylark.wasm skylark.wasm
```
### Docker Build and Push
```bash
docker buildx build --platform wasi/wasm  --provenance=false -t guelmino/skylark:latest .
docker push guelmino/skylark:latest
```

### Access Redis
```bash
kubectl exec -it redis-HASH -- redis-cli
```


