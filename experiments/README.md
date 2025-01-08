# Skylark Evaluation

## Telemetry
``` bash
alias tcpu${HOSTNAME: -2}='redis-cli -h $HOSTNAME -p 6379 SUBSCRIBE "telem/$HOSTNAME/cpu" >> cpu-$HOSTNAME.log'
alias tram${HOSTNAME: -2}='redis-cli -h $HOSTNAME -p 6379 SUBSCRIBE "telem/$HOSTNAME/ram" >> ram-$HOSTNAME.log'
alias flushr='redis-cli -h $HOSTNAME -p 6379 FLUSHALL'

tcpu${HOSTNAME: -2} & tram${HOSTNAME: -2}

alias flushu1='redis-cli -h pi5u1 -p 6379 FLUSHALL'
alias flushu2='redis-cli -h pi5u2 -p 6379 FLUSHALL'
alias flushu3='redis-cli -h pi5u3 -p 6379 FLUSHALL'
alias flushu4='redis-cli -h pi5u4 -p 6379 FLUSHALL'
alias flushu5='redis-cli -h pi4u5 -p 6379 FLUSHALL'
alias flushu6='redis-cli -h pi4u6 -p 6379 FLUSHALL'
alias flushu8='redis-cli -h pi4u8 -p 6379 FLUSHALL'
alias flushp1='redis-cli -h pi4p1 -p 6379 FLUSHALL'
alias flushallall='flushu2; flushu1; flushu2; flushu3; flushu4; flushu5; flushu6; flushu8; flushp1'
alias flushallexceptu2='flushu2; flushu1; flushu2; flushu3; flushu4; flushu5; flushu6; flushu8; flushp1'
flushu2; flushu1; flushu2; flushu3; flushu4; flushu5; flushu6; flushu8; flushp1

alias tcpuu1='redis-cli -h pi5u1 -p 6379 SUBSCRIBE "telem/pi5u1/cpu" >> cpu-pi5u1.log'
alias tramu1='redis-cli -h pi5u1 -p 6379 SUBSCRIBE "telem/pi5u1/ram" >> ram-pi5u1.log'
alias tcpuu2='redis-cli -h pi5u2 -p 6379 SUBSCRIBE "telem/pi5u2/cpu" >> cpu-pi5u2.log'
alias tramu2='redis-cli -h pi5u2 -p 6379 SUBSCRIBE "telem/pi5u2/ram" >> ram-pi5u2.log'
alias tcpuu3='redis-cli -h pi5u3 -p 6379 SUBSCRIBE "telem/pi5u3/cpu" >> cpu-pi5u3.log'
alias tramu3='redis-cli -h pi5u3 -p 6379 SUBSCRIBE "telem/pi5u3/ram" >> ram-pi5u3.log'
alias tcpuu4='redis-cli -h pi5u4 -p 6379 SUBSCRIBE "telem/pi5u4/cpu" >> cpu-pi5u4.log'
alias tramu4='redis-cli -h pi5u4 -p 6379 SUBSCRIBE "telem/pi5u4/ram" >> ram-pi5u4.log'
alias tcpuu5='redis-cli -h pi4u5 -p 6379 SUBSCRIBE "telem/pi4u5/cpu" >> cpu-pi4u5.log'
alias tramu5='redis-cli -h pi4u5 -p 6379 SUBSCRIBE "telem/pi4u5/ram" >> ram-pi4u5.log'
alias tcpuu6='redis-cli -h pi4u6 -p 6379 SUBSCRIBE "telem/pi4u6/cpu" >> cpu-pi4u6.log'
alias tramu6='redis-cli -h pi4u6 -p 6379 SUBSCRIBE "telem/pi4u6/ram" >> ram-pi4u6.log'
alias tcpuu8='redis-cli -h pi4u8 -p 6379 SUBSCRIBE "telem/pi4u8/cpu" >> cpu-pi4u8.log'
alias tramu8='redis-cli -h pi4u8 -p 6379 SUBSCRIBE "telem/pi4u8/ram" >> ram-pi4u8.log'
alias tcpup1='redis-cli -h pi4p1 -p 6379 SUBSCRIBE "telem/pi4p1/cpu" >> cpu-pi4p1.log'
alias tramp1='redis-cli -h pi4p1 -p 6379 SUBSCRIBE "telem/pi4p1/ram" >> ram-pi4p1.log'
alias monitor_all='tcpuu1 & tcpuu2 & tcpuu2 & tcpuu3 & tcpuu4 & tcpuu5 & tcpuu6 & tcpuu8 & tcpup1 & tramu1 & tramu2 & tramu2 & tramu3 & tramu4 & tramu5 & tramu6 & tramu8 & tramp1'
tcpuu1 & tcpuu2 & tcpuu2 & tcpuu3 & tcpuu4 & tcpuu5 & tcpuu6 & tcpuu8 & tcpup1 & tramu1 & tramu2 & tramu2 & tramu3 & tramu4 & tramu5 & tramu6 & tramu8 & tramp1
alias propagate_random='./run_propagate_performance.sh 20 1MB 100 Random; ./run_propagate_performance.sh 20 5MB 150 Random; ./run_propagate_performance.sh 20 10MB 200 Random; ./run_propagate_performance.sh 20 15MB 200 Random; ./run_propagate_performance.sh 20 20MB 200 Random; ./run_propagate_performance.sh 20 25MB 200 Random; ./run_propagate_performance.sh 20 30MB 200 Random; ./run_propagate_performance.sh 20 35MB 200 Random; ./run_propagate_performance.sh 20 40MB 200 Random; ./run_propagate_performance.sh 20 45MB 200 Random; ./run_propagate_performance.sh 20 50MB 200 Random'
alias propagate_skylark='./run_propagate_performance.sh 20 1MB 100 Skylark; ./run_propagate_performance.sh 20 5MB 150 Skylark; ./run_propagate_performance.sh 20 10MB 200 Skylark; ./run_propagate_performance.sh 20 15MB 260 Skylark; ./run_propagate_performance.sh 20 20MB 330 Skylark; ./run_propagate_performance.sh 20 25MB 390 Skylark; ./run_propagate_performance.sh 20 30MB 450 Skylark; ./run_propagate_performance.sh 20 35MB 510 Skylark; ./run_propagate_performance.sh 20 40MB 570 Skylark; ./run_propagate_performance.sh 20 45MB 630 Skylark; ./run_propagate_performance.sh 20 50MB 700 Skylark'
alias propagate_stateless='./run_propagate_performance.sh 20 1MB 100 Stateless; ./run_propagate_performance.sh 20 5MB 150 Stateless; ./run_propagate_performance.sh 20 10MB 200 Stateless; ./run_propagate_performance.sh 20 15MB 200 Stateless; ./run_propagate_performance.sh 20 20MB 200 Stateless; ./run_propagate_performance.sh 20 25MB 200 Stateless; ./run_propagate_performance.sh 20 30MB 200 Stateless; ./run_propagate_performance.sh 20 35MB 200 Stateless; ./run_propagate_performance.sh 20 40MB 200 Stateless; ./run_propagate_performance.sh 20 45MB 200 Stateless; ./run_propagate_performance.sh 20 50MB 200 Stateless'
propagate_skylark; propagate_stateless; propagate_random
```

## Metrics
W:=Workflow, T:=Time, D:=Data
- T(W): E2E Workflow latency (ms)
- T(f): E2E Function latency (ms)
- T(ex): Function execution latency (ms)
- T(dm): Data migration time (ms)
- T(dr): Data retrieval time (ms)
- D(f): 
- D(W): Migrated data (MB)
- Dp: Data proximity (amount of hops)
- Av(l): Local state availability (%)
- SLA Viol.: SLA Violations (%)
- Bundle depth (amount of functions)
- Amount of nodes

## Experiments
### Policy Scalability
#### Growing Topology
Measures the policy execution time with increasing topology size
- n=10: 10 nodes in a sequence like in the use case: start: node1, destination: node6
- n=100: 100 nodes with each 4 neighbors. start: Node51 destination: Node58
- n=1000: 1000 nodes with each 4 neighbors. start: Node18 destination: Node28
- n=10000: 10000 nodes with each 4 neighbors. start: Node3615 destination: Node5807

Run experiment:
```bash
curl http://10.0.0.167:8081/benchmark
curl http://10.0.0.34:8081/benchmark
```

*Growing Graph Results*

| Policy  | time(ms) | nodes   | 
|---------|----------|---------|
| Skylark | 1        | 10      |
| Skylark | 13       | 100     |
| Skylark | 162      | 1.000   |
| Skylark | 832      | 10.000  |

#### Growing parallel requests
Measures the policy execution time with increasing req/s (5,10,20,40)

*Parallel requests Results*

| req/s   | nodes   | time(ms) |
|---------|---------|----------|
| 5       | 10      | 1        |
| 10      | 10      | 1        |
| 20      | 10      | 1        |
| 40      | 10      | 1        |
| Skylark | 100     | 13       |
| Skylark | 1.000   | 162      |
| Skylark | 10.000  | 832      |

### Workflow Latency
Measures the end-2-end workflow runtime with varying Policies, SLOs and state sizes.
Uses the proposed EO use case.

| Tf(max)[ms] | State size [MB] | Policy  | Tw[ms] | SLO Viol. [%] | Migr. Data [MB] | Loc. State Av. [%] | Avg. hop distance | 
|-------------|-----------------|---------|--------|---------------|-----------------|--------------------|-------------------|
| 100         | 1               | Skylark |        |               |                 |                    |                   |