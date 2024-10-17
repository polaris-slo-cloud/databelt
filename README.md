# skylark

## Setup Test Environment

Debian 12 on VirtualBox

ssh-keygen -t rsa -b 4096 -C "leonard.guelmino@gmx.at"

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

### Setup k3s

* install and run with `curl -sfL https://get.k3s.io | sh -`
* check status sudo systemctl status k3s
* export KUBECONFIG=/etc/rancher/k3s/k3s.yaml
* echo "export KUBECONFIG=/etc/rancher/k3s/k3s.yaml" \>\> \~/.bashrc
* source \~/.bashrc
* sudo chown root:kubeconfig /etc/rancher/k3s/k3s.yaml
* sudo chmod 640 /etc/rancher/k3s/k3s.yaml
* newgrp kubeconfig
* kubectl get nodes


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

### Setup WasmEdge

