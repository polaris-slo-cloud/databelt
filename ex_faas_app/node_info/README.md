# Neighbor Simulation Service
Exposes HTTP API to serve the following information:

| Endpoint         | Method | Description                                                                                      |
|------------------|--------|--------------------------------------------------------------------------------------------------|
| current-topology | GET    | Array of edges between cluster nodes wih bandwidth, latency and Node info                        | 
| next-topology    | GET    | Array of edges between cluster nodes wih bandwidth, latency and Node info                        | 
| cloud-node-info  | GET    | Node info of next cloud node holding global replicated state                                     |
| local-node-info  | GET    | Node info of the local node                                                                      |
| objectives       | GET    | SLOs in SkylarkSLOs format. Can be configured in [slo-settings.json](settings/slo-settings.json) |
| refresh          | GET    | reloads SLOs, redis pods and cluste nodes                                                        |
| test-redis       | GET    | executes set/get/delete commands on local and cloud redis                                        |
| health           | GET    | health probe, returns 200                                                                        |

- Node info ≙ SkylarkNode
- objectives ≙ SkylarkSLOs
Bandwidth and latencies can be configured in [simulation-settings.json](settings/simulation-settings.json)
### Docker Build and Push
```bash
docker buildx build --platform linux/arm64 -t guelmino/skylark-node-info:0.2.108 .
docker push guelmino/skylark-node-info:0.2.108
```
### Deploy
```bash
# DaemonSet
kubectl apply -f ~/deployment/daemonset/node-info-daemonset.yaml
kubectl delete daemonset skylark-node-info-daemonset
```
### Troubleshoot
```bash
kubectl get pods
kubectl describe pod skylark-node-info
kubectl logs skylark-node-info-
```

Endpoints
```bash
curl http://10.0.0.34:8080/objectives -d '{"min_bandwidth": 60,"max_latency": 10000}' -H "Content-Type: application/json"
curl http://10.0.0.45:8080/objectives -d '{"min_bandwidth": 60,"max_latency": 10000}' -H "Content-Type: application/json"
curl http://10.0.0.167:8080/objectives -d '{"min_bandwidth": 60,"max_latency": 10000}' -H "Content-Type: application/json"
curl http://10.0.0.58:8080/objectives -d '{"min_bandwidth": 60,"max_latency": 10000}' -H "Content-Type: application/json"
curl http://10.0.0.243:8080/objectives -d '{"min_bandwidth": 60,"max_latency": 10000}' -H "Content-Type: application/json"
curl http://10.0.0.245:8080/objectives -d '{"min_bandwidth": 60,"max_latency": 10000}' -H "Content-Type: application/json"
curl http://10.0.0.210:8080/objectives -d '{"min_bandwidth": 60,"max_latency": 10000}' -H "Content-Type: application/json"
curl http://10.0.0.122:8080/objectives -d '{"min_bandwidth": 60,"max_latency": 10000}' -H "Content-Type: application/json"

curl http://10.0.0.34:8080/health
curl http://10.0.0.34:8080/local-node-info
curl http://10.0.0.34:8080/cloud-node-info
curl http://10.0.0.34:8080/refresh
curl http://10.0.0.34:8080/objectives
curl http://10.0.0.34:8080/current-topology
curl http://10.0.0.34:8080/static-topology
curl http://10.0.0.34:8080/next-topology
curl http://10.0.0.34:8080/test-redis

```

Access Redis
```bash
kubectl exec -it redis-HASH -- redis-cli
```