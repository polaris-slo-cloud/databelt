Build and Push
```bash
docker build -t guelmino/skylark:pyclient --platform linux/arm64 .
docker push guelmino/skylark:pyclient

```

Deploy
```bash
microk8s kubectl apply -f pyclient.yaml
microk8s kubectl get ksvc

kubectl get pods
kubectl logs NAME
kubectl describe pod skylark
kubectl apply -f ex2.yaml
kubectl delete ksvc --all
```

Access Redis
```bash
kubectl exec -it redis-HASH -- redis-cli
```