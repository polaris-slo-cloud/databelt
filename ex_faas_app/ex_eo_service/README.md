# Earth Observation Simulation Service
Exposes API to POST JPEG image which is converted to base64 and stored in a RedisStream. 

### Docker Build and Push
```bash
docker buildx build --platform linux/arm64 -t guelmino/skylark-ex-eo-service:latest .
docker push guelmino/skylark-ex-eo-service:latest
```
### Deploy
```bash
kubectl apply -f ~/deployment/service/ex-eo-service.yaml
```
### Troubleshoot
```bash
kubectl get pods -o wide
kubectl describe pod skylark-ex-eo-service
kubectl logs skylark-ex-eo-service-00001-deployment-
```
### Remove
```bash
kubectl delete ksvc skylark-ex-eo-service
```

Get nodes from cli
```bash
curl -X POST -F "image=@input.jpeg" -v http://10.152.183.152/direct -H "Host: skylark-ex-eo-service.default.svc.cluster.local"
curl -X POST -F "image=@input.jpeg" -v http://10.152.183.152/stream -H "Host: skylark-ex-eo-service.default.svc.cluster.local"
```

Access Redis
```bash
kubectl exec -it redis-b2ztp -- redis-cli
# check for incoming stream data in stream "eo_image_stream"
XRANGE eo_image_stream - +
```