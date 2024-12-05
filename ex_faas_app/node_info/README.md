# Neighbor Simulation Service
Exposes HTTP API to serve the following information:

| Endpoint        | Method | Description                                                                                      |
|-----------------|--------|--------------------------------------------------------------------------------------------------|
| node-graph      | GET    | Array of edges between cluster nodes wih bandwidth, latency and Node info                        | 
| cloud-node-info | GET    | Node info of next cloud node holding global replicated state                                     |
| local-node-info | GET    | Node info of the local node                                                                      |
| objectives      | GET    | SLOs in SkylarkSLOs format. Can be configured in [slo-settings.json](settings/slo-settings.json) |
| refresh         | GET    | reloads SLOs, redis pods and cluste nodes                                                        |

- Node info ≙ SkylarkNode
- objectives ≙ SkylarkSLOs
Bandwidth and latencies can be configured in [simulation-settings.json](settings/simulation-settings.json)
### Docker Build and Push
```bash
docker buildx build --platform linux/arm64 -t guelmino/skylark-node-info:0.2.3 . --no-cache
docker push guelmino/skylark-node-info:0.2.3
```
### Deploy
```bash
# DaemonSet
kubectl apply -f ~/deployment/daemonset/node-info-daemonset.yaml
kubectl apply -f ~/deployment/daemonset/node-info-nodeport.yaml
kubectl delete daemonset skylark-node-info-daemonset
kubectl delete service skylark-node-info-nodeport
```
### Troubleshoot
```bash
kubectl get pods
kubectl describe pod skylark-node-info
kubectl logs skylark-node-info-
```

Endpoints
```bash
curl -v http://10.152.183.159/node-graph -H "Host: skylark-node-info.default.svc.cluster.local"
curl -v http://10.152.183.159/objectives -H "Host: skylark-node-info.default.svc.cluster.local"

curl -v http://10.152.183.159/local-node-info -H "Host: skylark-node-info.default.svc.cluster.local"
curl -v http://10.152.183.159/cloud-node-info -H "Host: skylark-node-info.default.svc.cluster.local"
curl -v http://10.152.183.159/refresh -H "Host: skylark-node-info.default.svc.cluster.local"
curl -v http://10.152.183.159/health -H "Host: skylark-node-info.default.svc.cluster.local"

curl -v http://10.152.183.159/objectives -H "Host: skylark-node-info-cluster-ip.default.svc.cluster.local"
curl -v http://skylark-node.default.svc.cluster.local/node-graph
curl -v http://10.152.183.159/health -H "Host: skylark-node.default.svc.cluster.local"
```

Access Redis
```bash
kubectl exec -it redis-HASH -- redis-cli
```