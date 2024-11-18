# Skylark
The implementation of the skylark model. Provides mechanisms to 
1. Fetch and store bundled state of serverless functions in a request-optimized way.
2. Propagates function state to the potential local execution environment of neighboring nodes.

This repository includes the following subprojects: 
- [Skylark:](skylark/README.md) Implementation of the proposed mechanisms in the thesis as a rust library.  
- [Example alarm](ex_faas_app/ex_alarm/README.md)
- [Example image preprocessor](ex_faas_app/ex_preprocess/README.md)
- [Example object detector](ex_faas_app/ex_detect/README.md)
- [Example alarm](ex_faas_app/neighbors_service/README.md)

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


### Setup Knative Serving
Knative Serving CRDs/Core Components, Kurier (Networking)
* kubectl apply \-f https://github.com/knative/serving/releases/download/knative-v1.12.1/serving-crds.yaml
* kubectl apply \-f https://github.com/knative/serving/releases/download/knative-v1.12.1/serving-core.yaml
* kubectl apply \-f https://github.com/knative/net-kourier/releases/download/knative-v1.12.1/kourier.yaml
* kubectl patch configmap/config-network \\  
  \--namespace knative-serving \\  
  \--patch '{"data":{"ingress.class":"kourier.ingress.networking.knative.dev"}}'
* kubectl get pods \-n knative-serving

### Setup Knative Eventing
Knative Eventing CRDs/Core Components, Broker (MT-Channel-Based Broker)
* kubectl apply -f https://github.com/knative/eventing/releases/download/knative-v1.12.1/eventing-crds.yaml
* kubectl apply -f https://github.com/knative/eventing/releases/download/knative-v1.12.1/eventing-core.yaml
* kubectl apply -f https://github.com/knative/eventing/releases/download/knative-v1.12.1/mt-channel-broker.yaml
* kubectl apply -f https://github.com/knative/eventing/releases/download/knative-v1.12.1/in-memory-channel.yaml

### Setup Redis
KV-Stores will be availiable on each node. In order to trigger services based on changes in KV stores we use a RedisStreamSource
* kubectl apply -f https://github.com/knative-extensions/eventing-redis/releases/download/knative-v1.16.0/redis-source.yaml
* kubectl apply -f daemonset/redis-daemonset.yaml && kubectl apply -f service/redis-headless-service.yaml && kubectl apply -f source/redis-image-source.yaml
### Docker secret to pull images from registry
```bash
kubectl create secret docker-registry regcred \
--docker-server=https://index.docker.io/v1/ \
--docker-username=guelmino \
--docker-password=your-password \
--docker-email=your-email@example.com

kubectl patch serviceaccount default -p '{"imagePullSecrets": [{"name": "regcred"}]}'
```
### Rust development
Build
```bash
cargo build --target wasm32-wasi --release
```
Optional: optimize using `wasmedge compile`
```bash
wasmedge compile target/wasm32-wasi/release/ex_fn_1.wasm appname.wasm
```

### Package and push to registry
In the respective function root folder, run:
``` bash
docker buildx build --platform wasi/wasm  --provenance=false -t guelmino/skylark:latest .
docker push guelmino/skylark:latest
docker run --runtime=io.containerd.wasmedge.v1 --platform=wasi/wasm guelmino/skylark:latest
```
### WASM Deployment
If the wasm module acts as a client, the dns server has to be specified in the deployment yaml. Get the dns cluster ip
```bash
kubectl get svc -n kube-system kube-dns
```
```yaml
apiVersion: serving.knative.dev/v1
kind: Service
metadata:
  name: skylark-reqwestclient
  namespace: default
spec:
  template:
    metadata:
      annotations:
        module.wasm.image/variant: compat-smart
        autoscaling.knative.dev/minScale: "0" # to ensure scaling to zero
    spec:
      dnsPolicy: ClusterFirst
      runtimeClassName: wasmedge
      containers:
        - image: guelmino/skylark-reqwestclient:latest
          command: ["./reqwestclient.wasm"]
          env:
            - name: DNS_SERVER
              value: "10.152.183.10:53"
```

### useful kubectl commands
microk8s kubectl get pods
``` bash
kubectl delete ValidatingWebhookConfiguration validation.webhook.serving.knative.dev
kubectl get pods -o wide
kubectl logs NAME
kubectl describe pod skylark
kubectl apply -f <name>.yaml
kubectl get ksvc
kubectl delete ksvc --all
kubectl apply -f ex2.yaml
kubectl get events NAME -n NAMESPACE
microk8s inspect

microk8s add-node
kubectl get service.serving.knative.dev
curl -X POST -v -H "Host: skylark-pyclient.default.svc.cluster.local" http://10.152.183.152
kubectl exec -it redis -- sh

# get dns info of cluster services
kubectl get svc -n default

# status 
kubectl get pods -o wide
kubectl get ksvc -o wide

# ping source
kubectl apply -f ~/deployment/eventing/eo-ping-source.yaml
kubectl get pingsources.sources.knative.dev --all-namespaces
kubectl get pingsource eo-ping-source -n default -o yaml
kubectl delete -f ~/deployment/eventing/eo-ping-source.yaml
kubectl logs deployment/pingsource-mt-adapter -n knative-eventing
```

