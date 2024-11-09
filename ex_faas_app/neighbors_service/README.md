# Neighbor Simulation Service
Returns available neighbor nodes via http GET.

Build and Push
```bash
docker build -t guelmino/skylark-neighbors --platform linux/arm64 .
docker push guelmino/skylark-neighbors
```

Deploy
```bash
microk8s kubectl apply -f neighbors.service.yaml
microk8s kubectl get ksvc
kubectl get pods
kubectl logs NAME
kubectl describe pod skylark-neighbors
kubectl delete ksvc --all
```

Get nodes from cli
```bash
curl -X GET -v http://10.152.183.152/neighbors -H "Host: skylark-neighbors.default.svc.cluster.local"
```

Access Redis
```bash
kubectl exec -it redis-HASH -- redis-cli
```