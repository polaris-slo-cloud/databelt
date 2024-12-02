# Neighbor Simulation Service
Exposes HTTP API to serve the following information:

  | Endpoint        | Method | Description                                                                             |
  |-----------------|--------|-----------------------------------------------------------------------------------------|
  | node-graph      | GET    | Array of edges between cluster nodes wih bandwidth, latency and Node info               | 
| cloud-node-info | GET    | Node info of next cloud node holding global replicated state                            |
| local-node-info | GET    | Node info of the local node                                                             |
| objectives      | GET    | SLOs in SkylarkSLOs format. Can be configured in [slo-settings.json](settings/slo-settings.json) |
| refresh         | GET    | reloads SLOs, redis pods and cluste nodes                                               |

- Node info ≙ SkylarkNode
- objectives ≙ SkylarkSLOs
Bandwidth and latencies can be configured in [simulation-settings.json](settings/simulation-settings.json)
### Docker Build and Push
```bash
docker buildx build -t guelmino/skylark-node-service:latest --platform linux/arm64 .
docker push guelmino/skylark-node-service:latest
```
### Deploy
```bash
kubectl apply -f ~/deployment/daemonset/node-service-daemonset.yaml
```
### Troubleshoot
```bash
kubectl get pods
kubectl describe pod skylark-node-service
kubectl logs skylark-node-service-
```
### Remove
```bash
kubectl delete -f ~/deployment/daemonset/node-service-daemonset.yaml
```

Get nodes from cli
```bash
curl -v http://10.1.52.185:8080/node-graph
curl -v http://10.1.46.219:8080/objectives
curl -v http://10.1.46.219:8080/local-node-info
curl -v http://10.1.46.219:8080/cloud-node-info
curl -v http://10.1.46.194:8080/refresh
curl -v http://skylark-node-service-:8080/refresh 
```

Access Redis
```bash
kubectl exec -it redis-HASH -- redis-cli
```