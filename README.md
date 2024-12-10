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
* sudo apt install redis-tools -y

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
kubectl apply -f ~/deployment/daemonset/redis-daemonset.yaml
kubectl apply -f ~/deployment/daemonset/redis-headless.yaml

kubectl apply -f ~/deployment/daemonset/node-info-daemonset.yaml

kubectl apply -f ~/deployment/daemonset/skylark-api-daemonset.yaml
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



### useful kubectl commands
microk8s kubectl get pods

```bash
# pods  nodes   svc   ksvc  logs  describe  delete                                    apply                           redis
# kgp   kgn     kgs   kgk   kl    kd        kdapi kdni kdcl kdde kdpre kdal kdalles   kaapi kani kacl kade kapre kaal rcliu1 rcliu2 rcliu2 rcliu4 
alias kgp='kubectl get pods -o wide'; alias kgn='kubectl get nodes -o wide'; alias kgs='kubectl get svc -o wide'; alias kgk='kubectl get ksvc -o wide'; alias kl='kubectl logs'; alias klf='kubectl logs -f'; alias kd='kubectl describe'; alias kdwh='kubectl delete ValidatingWebhookConfiguration validation.webhook.serving.knative.dev'
alias kdapi='kubectl delete daemonset skylark-api-daemonset'; alias kdni='kubectl delete daemonset skylark-node-info-daemonset'; alias kdcl='kubectl delete ksvc skylark-ex-client; kubectl delete svc skylark-ex-client; kubectl delete route skylark-ex-client'; alias kdde='kubectl delete ksvc skylark-ex-detect; kubectl delete svc skylark-ex-detect; kubectl delete route skylark-ex-detect'; alias kdpre='kubectl delete ksvc skylark-ex-preprocess; kubectl delete svc skylark-ex-preprocess; kubectl delete route skylark-ex-preprocess'; alias kdal='kubectl delete ksvc skylark-ex-alarm; kubectl delete svc skylark-ex-alarm; kubectl delete route skylark-ex-alarm'; alias kdalles='kubectl delete ksvc skylark-ex-client skylark-ex-detect skylark-ex-alarm skylark-ex-preprocess; kubectl delete svc skylark-ex-client skylark-ex-detect skylark-ex-alarm skylark-ex-preprocess; kubectl delete daemonset skylark-api-daemonset skylark-node-info-daemonset; kubectl delete route skylark-ex-client skylark-ex-detect skylark-ex-alarm skylark-ex-preprocess'
alias kaapi='kubectl apply -f ~/deployment/daemonset/skylark-api-daemonset.yaml'; alias kani='kubectl apply -f ~/deployment/daemonset/node-info-daemonset.yaml'; alias kacl='kubectl apply -f ~/deployment/service/ex-client-service.yaml'; alias kade='kubectl apply -f ~/deployment/service/ex-detect-service.yaml'; alias kapre='kubectl apply -f ~/deployment/service/ex-preprocess-service.yaml'; alias kaal='kubectl apply -f ~/deployment/service/ex-alarm-service.yaml'
alias rcli1='redis-cli -h pi5u1 -p 6379'; alias rcli2='redis-cli -h pi5u2 -p 6379'; alias rcli3='redis-cli -h pi5u3 -p 6379'; alias rcli4='redis-cli -h pi5u4 -p 6379'
alias ssu1='ssh skylark@pi5u1'; alias ssu2='ssh skylark@pi5u2'; alias ssu3='ssh skylark@pi5u3'; alias ssu4='ssh skylark@pi5u4'
```

```bash
# new version
kubectl delete ksvc skylark-ex-client skylark-ex-detect skylark-ex-alarm skylark-ex-preprocess
kubectl delete svc skylark-ex-client skylark-ex-detect skylark-ex-alarm skylark-ex-preprocess
kubectl delete daemonset skylark-api-daemonset skylark-node-info-daemonset
kubectl delete route skylark-ex-client skylark-ex-detect skylark-ex-alarm skylark-ex-preprocess
kubectl get pods && kubectl get ksvc && kubectl get svc
# kubectl delete configuration skylark-ex-client skylark-ex-detect skylark-ex-alarm skylark-ex-preprocess
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
NodeInfo x.x.1xx, ExClient x.x.2xx, ExPreprocess x.x.3xx, ExDetect x.x.4xx, 
ExAlarm x.x.5xx, SkylarkLib x.x.6xx, SkylarkApi x.x.7xx

1. Clear target dir and lock file
2. Bump version
3. run build script
4. deploy updated yaml

Lib Change?

### Access Redis
```bash
kubectl exec -it redis-njxg4 -- redis-cli #pi5u1
kubectl exec -it redis-p55qs -- redis-cli #pi5u2
kubectl exec -it redis-tvxqx -- redis-cli #pi5u3
kubectl exec -it redis-vg4xz -- redis-cli #pi5u4
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

### Node ports
| Service     | Port  | 
|-------------|-------|
| node-info   | 8080 | 
| skylark-api | 8081 |
| redis       | 36379 |
