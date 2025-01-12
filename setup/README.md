## Setup Node
```bash
sudo apt update 
sudo apt upgrade -y
sudo apt-get install -y curl apt-transport-https ca-certificates software-properties-common htop ufw net-tools snapd redis-tools
sudo apt install iperf3 -y
sudo usermod -aG sudo skylark
sudo groupadd kubeconfig
sudo usermod -aG kubeconfig skylark && sudo usermod -aG kubeconfig root
sudo ufw allow 6443/tcp && sudo reboot
nano /home/skylark/.ssh/authorized_keys # insert pub key
chmod  0700 ~/.ssh
## Microk8s Setup
```bash
# add "cgroup_enable=memory cgroup_memory=1" to:
sudo nano /boot/firmware/cmdline.txt
sudo snap install microk8s --classic
sudo usermod -a -G microk8s $USER
mkdir -p ~/.kube
chmod 0700 ~/.kube
su - $USER

# Docker secret to pull images from registry
kubectl create secret docker-registry regcred \
--docker-server=https://index.docker.io/v1/ \
--docker-username=guelmino \
--docker-password=your-password \
--docker-email=your-email@example.com

kubectl patch serviceaccount default -p '{"imagePullSecrets": [{"name": "regcred"}]}'

# Start here after resetting
sudo microk8s add-node #on main node
kubectl label node pi5u1 node-type=Cloud; kubectl label node pi5u2 node-type=Sat; kubectl label node pi5u3 node-type=Sat; kubectl label node pi5u4 node-type=Sat; kubectl label node pi4u5 node-type=Sat; kubectl label node pi4u6 node-type=Sat; kubectl label node pi4u8 node-type=Sat; kubectl label node pi4p1 node-type=Sat
kubectl label node pi5u1 node-name=pi5u1; kubectl label node pi5u2 node-name=pi5u2; kubectl label node pi5u3 node-name=pi5u3; kubectl label node pi5u4 node-name=pi5u4; kubectl label node pi4u5 node-name=pi4u5; kubectl label node pi4u6 node-name=pi4u6; kubectl label node pi4u8 node-name=pi4u8; kubectl label node pi4p1 node-name=pi4p1
# kubectl label node pi4u7 node-name=pi4u7;; kubectl label node pi4u7 node-type=Edge; kubectl label node pi4u7 local-store=false

sudo microk8s enable community
sudo microk8s enable knative
sudo microk8s enable kwasm
kubectl apply -f ~/deployment/wasmedge-runtimeclass.yaml
kubectl apply -f ~/deployment/skylark-configmap.yaml

kubectl delete ValidatingWebhookConfiguration validation.webhook.serving.knative.dev

# Deploy order from scratch
kubectl apply -f ~/deployment/daemonset/redis-daemonset.yaml
kubectl apply -f ~/deployment/daemonset/node-info-daemonset.yaml
kubectl apply -f ~/deployment/daemonset/skylark-elect-daemonset.yaml
kubectl apply -f ~/deployment/service/ex-preprocess-service.yaml
kubectl apply -f ~/deployment/service/ex-detect-service.yaml
kubectl apply -f ~/deployment/service/ex-alarm-service.yaml
kubectl apply -f ~/deployment/service/ex-client-service.yaml
```