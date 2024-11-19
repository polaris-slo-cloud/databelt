# Neighbor Simulation Service
Returns available neighbor nodes via http GET.

### Docker Build and Push
```bash
docker build -t guelmino/skylark-neighbors:latest --platform linux/arm64 .
docker push guelmino/skylark-neighbors:latest
```
### Deploy
```bash
kubectl apply -f ~/deployment/service/neighbors-service.yaml
```
### Troubleshoot
```bash
kubectl get pods
kubectl describe pod skylark-neighbors
kubectl logs skylark-neighbors-00001-deployment-
```
### Remove
```bash
kubectl delete ksvc skylark-neighbors
```

Get nodes from cli
```bash
curl -v http://10.152.183.152/node-topology -H "Host: skylark-neighbors.default.svc.cluster.local"
curl -v http://10.152.183.152/redis-pods -H "Host: skylark-neighbors.default.svc.cluster.local"
```

Access Redis
```bash
kubectl exec -it redis-HASH -- redis-cli
```