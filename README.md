# Master Thesis
## Skylark - Stateful Serverless Functions for the Edge-Cloud-Space 3D-Continuum
This repository includes the following subprojects: 
- [Skylark Elect: ](skylark_elect/README.md) Node Election service State Propagation
- [Skylark Manage: ](skylark_manage/README.md) Client library for state bundling using Skylark Elect for successor node determination
- [Node Info](examples/node_info/README.md) Helper service which provides node topology information via HTTP API.
- [Alarm](examples/ex_alarm/README.md) Example function using Skylark Manage simulating a fire alarm. 
- [Image Preprocessor](examples/ex_preprocess/README.md) Example function using Skylark Manage simulating image preprocessor.
- [Object Detector](examples/ex_detect/README.md) Example function using Skylark Manage simulating image object detector.
- [Single](examples/ex_single/README.md) Example function simulating a baseline scenario for storing and migrating embedded function state. 
- [Bundled](examples/ex_bundled/README.md) Example function using state bundling for storing and migrating embedded function state.




### useful kubectl commands
microk8s kubectl get pods

```bash
nano ~/.bashrc
# pods  nodes   svc   ksvc  logs  describe        delete                                    apply                           redis
# kgp   kgn     kgs   kgk   kl    kd kdp kds kdd  dapi kdni kdcl kdde kdpre kdal kdalles   kaapi kani kacl kade kapre kaal rcliu1 rcliu2 rcliu2 rcliu4 
alias kubectl='microk8s kubectl'; alias kgp='kubectl get pods -o wide'; alias kgn='kubectl get nodes -o wide'; alias kgs='kubectl get svc -o wide'; alias kgk='kubectl get ksvc -o wide'; alias kl='kubectl logs'; alias klf='kubectl logs -f'; alias kd='kubectl describe'; alias kdwh='kubectl delete ValidatingWebhookConfiguration validation.webhook.serving.knative.dev'; alias kare='kubectl apply -f ~/deployment/daemonset/redis-daemonset.yaml'
alias kdre='kubectl delete daemonset redis'; alias kdapi='kubectl delete daemonset skylark-elect-daemonset'; alias kdni='kubectl delete daemonset skylark-node-info-daemonset'; alias kdcl='kubectl delete ksvc skylark-ex-client; kubectl delete svc skylark-ex-client; kubectl delete route skylark-ex-client'; alias kdde='kubectl delete ksvc skylark-ex-detect; kubectl delete svc skylark-ex-detect; kubectl delete route skylark-ex-detect'; alias kdpre='kubectl delete ksvc skylark-ex-preprocess; kubectl delete svc skylark-ex-preprocess; kubectl delete route skylark-ex-preprocess'; alias kdal='kubectl delete ksvc skylark-ex-alarm; kubectl delete svc skylark-ex-alarm; kubectl delete route skylark-ex-alarm'; alias kdalles='kubectl delete ksvc skylark-ex-client skylark-ex-detect skylark-ex-alarm skylark-ex-preprocess; kubectl delete svc skylark-ex-client skylark-ex-detect skylark-ex-alarm skylark-ex-preprocess; kubectl delete daemonset skylark-elect-daemonset skylark-node-info-daemonset; kubectl delete route skylark-ex-client skylark-ex-detect skylark-ex-alarm skylark-ex-preprocess'
alias kaapi='kubectl apply -f ~/deployment/daemonset/skylark-elect-daemonset.yaml'; alias kani='kubectl apply -f ~/deployment/daemonset/node-info-daemonset.yaml'; alias kacl='kubectl apply -f ~/deployment/service/ex-client-service.yaml'; alias kade='kubectl apply -f ~/deployment/service/ex-detect-service.yaml'; alias kapre='kubectl apply -f ~/deployment/service/ex-preprocess-service.yaml'; alias kaal='kubectl apply -f ~/deployment/service/ex-alarm-service.yaml'
alias rcli1='redis-cli -h pi5u1 -p 6379'; alias rcli2='redis-cli -h pi5u2 -p 6379'; alias rcli3='redis-cli -h pi5u3 -p 6379'; alias rcli4='redis-cli -h pi5u4 -p 6379'; alias rcli5='redis-cli -h pi4u5 -p 6379'; alias rcli6='redis-cli -h pi4u6 -p 6379'; alias rcli7='redis-cli -h pi4u7 -p 6379'; alias rcli8='redis-cli -h pi4u8 -p 6379';  alias rclip1='redis-cli -h pi4p1 -p 6379'
alias ssu1='ssh skylark@pi5u1'; alias ssu2='ssh skylark@pi5u2'; alias ssu3='ssh skylark@pi5u3'; alias ssu4='ssh skylark@pi5u4'
alias ssu5='ssh skylark@pi4u5'; alias ssu6='ssh skylark@pi4u6'; alias ssu7='ssh skylark@pi4u7'; alias ssu8='ssh skylark@pi4u8'; alias ssp1='ssh skylark@pi4p1'
alias kdp='kubectl describe pod'; alias kds='kubectl describe service'; alias kdd='kubectl describe daemonset'
alias kddes='kubectl delete ksvc pi5u2-detect; kubectl delete svc pi5u2-detect; kubectl delete route pi5u2-detect; kubectl delete ksvc pi5u3-detect; kubectl delete svc pi5u3-detect; kubectl delete route pi5u3-detect; kubectl delete ksvc pi5u4-detect; kubectl delete svc pi5u4-detect; kubectl delete route pi5u4-detect; kubectl delete ksvc pi4u5-detect; kubectl delete svc pi4u5-detect; kubectl delete route pi4u5-detect; kubectl delete ksvc pi4u6-detect; kubectl delete svc pi4u6-detect; kubectl delete route pi4u6-detect; kubectl delete ksvc pi4u8-detect; kubectl delete svc pi4u8-detect; kubectl delete route pi4u8-detect; kubectl delete ksvc pi4p1-detect; kubectl delete svc pi4p1-detect; kubectl delete route pi4p1-detect' 
alias kdpres='kubectl delete ksvc pi5u2-preprocess; kubectl delete svc pi5u2-preprocess; kubectl delete route pi5u2-preprocess; kubectl delete ksvc pi5u3-preprocess; kubectl delete svc pi5u3-preprocess; kubectl delete route pi5u3-preprocess; kubectl delete ksvc pi5u4-preprocess; kubectl delete svc pi5u4-preprocess; kubectl delete route pi5u4-preprocess; kubectl delete ksvc pi4u5-preprocess; kubectl delete svc pi4u5-preprocess; kubectl delete route pi4u5-preprocess; kubectl delete ksvc pi4u6-preprocess; kubectl delete svc pi4u6-preprocess; kubectl delete route pi4u6-preprocess; kubectl delete ksvc pi4u8-preprocess; kubectl delete svc pi4u8-preprocess; kubectl delete route pi4u8-preprocess; kubectl delete ksvc pi4p1-preprocess; kubectl delete svc pi4p1-preprocess; kubectl delete route pi4p1-preprocess'
alias kdals='kubectl delete ksvc pi5u1-alarm; kubectl delete svc pi5u1-alarm; kubectl delete route pi5u1-alarm'; alias kades='kubectl apply -f ~/deployment/service/simulation-detect-services.yaml'; alias kaals='kubectl apply -f ~/deployment/service/simulation-alarm-service.yaml'; alias kapres='kubectl apply -f ~/deployment/service/simulation-preprocess-services.yaml'
alias kdsis='kubectl delete ksvc pi5u2-single; kubectl delete svc pi5u2-single; kubectl delete route pi5u2-single; kubectl delete ksvc pi5u3-single; kubectl delete svc pi5u3-single; kubectl delete route pi5u3-single; kubectl delete ksvc pi5u4-single; kubectl delete svc pi5u4-single; kubectl delete route pi5u4-single; kubectl delete ksvc pi4u5-single; kubectl delete svc pi4u5-single; kubectl delete route pi4u5-single; kubectl delete ksvc pi4u6-single; kubectl delete svc pi4u6-single; kubectl delete route pi4u6-single; kubectl delete ksvc pi4u8-single; kubectl delete svc pi4u8-single; kubectl delete route pi4u8-single; kubectl delete ksvc pi4p1-single; kubectl delete svc pi4p1-single; kubectl delete route pi4p1-single'
alias kdbus='kubectl delete ksvc pi5u2-bundled; kubectl delete svc pi5u2-bundled; kubectl delete route pi5u2-bundled; kubectl delete ksvc pi5u3-bundled; kubectl delete svc pi5u3-bundled; kubectl delete route pi5u3-bundled; kubectl delete ksvc pi5u4-bundled; kubectl delete svc pi5u4-bundled; kubectl delete route pi5u4-bundled; kubectl delete ksvc pi4u5-bundled; kubectl delete svc pi4u5-bundled; kubectl delete route pi4u5-bundled; kubectl delete ksvc pi4u6-bundled; kubectl delete svc pi4u6-bundled; kubectl delete route pi4u6-bundled; kubectl delete ksvc pi4u8-bundled; kubectl delete svc pi4u8-bundled; kubectl delete route pi4u8-bundled; kubectl delete ksvc pi4p1-bundled; kubectl delete svc pi4p1-bundled; kubectl delete route pi4p1-bundled'
alias kasis='kubectl apply -f ~/deployment/service/simulation-single-services.yaml'; alias kabus='kubectl apply -f ~/deployment/service/simulation-bundled-services.yaml'; alias kawd='kubectl apply -f ~/deployment/daemonset/write-data-daemonset.yaml'; alias kawds='kubectl apply -f ~/deployment/service/simulation-write-data-service.yaml'
alias kdwds='kubectl delete ksvc pi5u1-write-data; kubectl delete svc pi5u1-write-data; kubectl delete route pi5u1-write-data'; alias kdwd='kubectl delete daemonset skylark-write-data-daemonset'
```

```bash
microk8s kubectl get pods -n knative-serving
# new version
kubectl delete ksvc skylark-ex-client skylark-ex-detect skylark-ex-alarm skylark-ex-preprocess
kubectl delete svc skylark-ex-client skylark-ex-detect skylark-ex-alarm skylark-ex-preprocess
kubectl delete daemonset skylark-elect-daemonset skylark-node-info-daemonset
kubectl delete route skylark-ex-client skylark-ex-detect skylark-ex-alarm skylark-ex-preprocess
kubectl get pods; kubectl get ksvc; kubectl get svc
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
kubectl exec -it skylark-elect-tpsjq -- printenv | grep _SERVICE_
```

### New Build
NodeInfo x.x.1xx, ExClient x.x.2xx, ExPreprocess x.x.3xx, ExDetect x.x.4xx, 
ExAlarm x.x.5xx, SkylarkManage x.x.6xx, SkylarkApi x.x.7xx

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
### Service ports
| Service     | Port  | 
|-------------|-------|
| node-info   | 8080 | 
| skylark-elect | 8081 |
| redis       | 36379 |

## Evaluation
### Techstack
- MicroK8s
- Knative
- WasmEdge

### Cluster
4x Raspberry Pi 5 Nodes
1x Cloud node, 3x Sat Node
`kubectl label node pi5u1 node-type=Cloud`
`kubectl label node pi5u2 node-type=Sat`

| Node  | Node Type | IP         |  
|-------|-----------|------------|
| pi5u1 | Cloud     | 10.0.0.243 | 
| pi5u2 | Sat       | 10.0.0.34  |
| pi5u3 | Sat       | 10.0.0.45  |
| pi5u4 | Sat       | 10.0.0.167 |
| pi4u5 | Sat       | 10.0.0.58  |
| pi4u6 | Sat       | 10.0.0.122 |
| pi4u7 | Edge      | 10.0.0.96  |
| pi4u8 | Sat       | 10.0.0.210 |
| pi4p1 | Sat       | 10.0.0.245 |

*Network characteristics*

| Connection type | Network delay | Jitter | Bandwidth |  
|-----------------|---------------|--------|-----------|
| Cloud-to-Sat    | 60ms          | 30ms   | 90Mbps    | 
| Sat-to-Sat      | 10ms          | 9ms    | 40Mbps    |

### Constellation
```
minute % 7 = t

                 t=0                    
 -(34)-(45)-(167)-(58)-(122)-(210)-(245)-
    |
  (243)---------(96)
  
                 t=1                    
 -(34)-(45)-(167)-(58)-(122)-(210)-(245)-
         |
       (243)---------(96)
            
                  t=2                    
 -(34)-(45)-(167)-(58)-(122)-(210)-(245)-
              |
            (243)---------(96)

                  t=3                    
 -(34)-(45)-(167)-(58)-(122)-(210)-(245)-
                    |
                  (243)---------(96)

                 t=4                    
-(34)-(45)-(167)-(58)-(122)-(210)-(245)-
                         |
                       (243)---------(96)

                 t=5                    
-(34)-(45)-(167)-(58)-(122)-(210)-(245)-
                               |
                             (243)----(96)

                 t=6                    
'-(34)-(45)-(167)-(58)-(122)-(210)-(245)-'
                                     |
                          (96)-----(243)

                
                
-(pi5u2)-(pi5u3)-(pi5u4)-(pi4u5)-(pi4u6)-(pi4u8)-(pi4p1)-
                                                      |
                                                    (pi5u1)---------(pi4u7)
```
