# Neighbor Simulation Service
Returns available neighbor nodes via http GET.

### Deploy
```bash
kubectl apply -f neighbors-service.yaml
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
curl -X GET -v http://10.152.183.152/neighbors -H "Host: skylark-neighbors.default.svc.cluster.local"
```

Access Redis
```bash
kubectl exec -it redis-HASH -- redis-cli
```