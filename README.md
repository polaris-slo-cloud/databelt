# Skylark
The implementation of the skylark model. Provides mechanisms to 
1. Fetch and store bundled state of serverless functions in a request-optimized way.
2. Propagates function state to the potential local execution environment of neighboring nodes.

This repository includes the following subprojects: 
- [Skylark api: ](skylark_api/README.md) Implementation of the proposed mechanisms for state propagation and bundled state retrieval
- [Skylark lib: ](skylark_lib/README.md) Wrapper lib to access the Skylark API  
- [Example alarm](ex_faas_app/ex_alarm/README.md)
- [Example image preprocessor](ex_faas_app/ex_preprocess/README.md)
- [Example object detector](ex_faas_app/ex_detect/README.md)
- [Example alarm](ex_faas_app/node_info/README.md)

### General setup
* sudo apt update
* sudo apt install -y openssh-server
* sudo systemctl start ssh
* sudo systemctl enable ssh

* sudo apt-get install \-y curl apt-transport-https ca-certificates software-properties-common htop ufw net-tools snapd

### User and Groups \+ Firewall Port

* su
* sudo usermod \-aG sudo \[username\]
* sudo groupadd kubeconfig
* sudo usermod \-aG kubeconfig $(whoami)
* sudo ufw allow 6443/tcp
* reboot

## Microk8s Setup
```bash
# add "cgroup_enable=memory cgroup_memory=1" to:
sudo nano /boot/firmware/cmdline.txt
sudo snap install microk8s --classic
sudo usermod -a -G microk8s $USER
mkdir -p ~/.kube
chmod 0700 ~/.kube
su - $USER
# Start here after resetting
microk8s add-node #on main node
kubectl label node pi5u1 node-type=Cloud && kubectl label node pi5u2 node-type=Sat && kubectl label node pi5u3 node-type=Sat && kubectl label node pi5u4 node-type=Sat

microk8s enable community
microk8s enable knative
microk8s enable kwasm
kubectl apply -f ~/deployment/wasmedge-runtimeclass.yaml
kubectl create namespace skylark
kubectl label namespace skylark knative-serving=enabled
kubectl config set-context --current --namespace=skylark

kubectl delete ValidatingWebhookConfiguration validation.webhook.serving.knative.dev

# Deploy order from scratch
# Redis
kubectl apply -f ~/deployment/daemonset/redis-daemonset.yaml
kubectl apply -f ~/deployment/daemonset/redis-headless-service.yaml
# Node Info Service
kubectl apply -f ~/deployment/daemonset/node-info-daemonset.yaml
kubectl apply -f ~/deployment/daemonset/node-info-nodeport.yaml
# SkylarkAPI
kubectl apply -f ~/deployment/daemonset/skylark-api-daemonset.yaml
kubectl apply -f ~/deployment/daemonset/skylark-api-nodeport.yaml
# Example functions
kubectl apply -f ~/deployment/service/ex-preprocess-service.yaml
kubectl apply -f ~/deployment/service/ex-detect-service.yaml
kubectl apply -f ~/deployment/service/ex-alarm-service.yaml
kubectl apply -f ~/deployment/service/ex-client-service.yaml
```
### Reset microk8s
```bash
# On each client node
microk8s leave
# On main node remove client nodes
microk8s remove-node pi5u3

sudo microk8s reset
sudo reboot now
# Continue Setup steps from the indicated point above
```
### Docker secret to pull images from registry
```bash
kubectl create secret docker-registry regcred \
--docker-server=https://index.docker.io/v1/ \
--docker-username=guelmino \
--docker-password=your-password \
--docker-email=your-email@example.com

kubectl patch serviceaccount default -p '{"imagePullSecrets": [{"name": "regcred"}]}'
```


### Node ports
| Service     | Port  | 
|-------------|-------|
| node-info   | 31016 | 
| skylark-api | 30163 |
| redis       | x     |


### useful kubectl commands
microk8s kubectl get pods

```bash
# delete all services
kubectl delete ksvc -n skylark --all && kubectl delete pods -n skylark --all
kubectl delete configuration -n skylark --all
kubectl delete route -n skylark --all
kubectl delete svc -n skylark --all
kubectl delete ValidatingWebhookConfiguration validation.webhook.serving.knative.dev
kubectl get pods -o wide
kubectl logs NAME
kubectl describe pod skylark
kubectl apply -f <name>.yaml
kubectl get ksvc
kubectl delete ksvc -n skylark --all
kubectl delete pods -n skylark --all
kubectl delete configuration -n skylark --all
kubectl delete route -n skylark --all
kubectl delete svc -n skylark --all

microk8s add-node
kubectl get service.serving.knative.dev
curl -X POST -v -H "Host: skylark-pyclient.default.svc.cluster.local" http://10.152.183.159
kubectl exec -it redis -- sh

# get dns info of cluster services
kubectl get svc -n default
kubectl get svc -n kube-system kube-dns # 10.152.183.10

# status 
kubectl get pods -o wide
kubectl get ksvc -o wide

# ping source
kubectl apply -f ~/deployment/eventing/eo-ping-source.yaml
kubectl get pingsources.sources.knative.dev --all-namespaces
kubectl get pingsource eo-ping-source -n default -o yaml
kubectl delete -f ~/deployment/eventing/eo-ping-source.yaml
kubectl logs deployment/pingsource-mt-adapter -n knative-eventing

# testing
kubectl run curl-service --image=curlimages/curl -i --tty -- sh
kubectl attach curl-service -c curl-service -i -t

# IP of ingress gateway
kubectl get service kourier-internal -n knative-serving -o wide
kubectl exec -it skylark-api-tpsjq -- printenv | grep _SERVICE_
```

### New Build
1. Bump version
2. run build script
3. deploy updated yaml

### Access Redis
```bash
kubectl exec -it redis-HASH -- redis-cli
```

## Experiments
### Techstack
- MicroK8s
- Knative
- WasmEdge

### Cluster
4x Raspberry Pi 5 Nodes
1x Cloud node, 3x Sat Node
`kubectl label node pi5u1 node-type=Cloud`
`kubectl label node pi5u2 node-type=Sat`

| Node  | Node Type (node-type label) | 
|-------|-----------------------------|
| pi5u1 | Cloud                       | 
| pi5u2 | Sat                         |
| pi5u3 | Sat                         |
| pi5u4 | Sat                         |
