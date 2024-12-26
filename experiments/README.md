# Skylark Evaluation
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

| Tf(max)[ms] | State size [MB] | Policy  | Tw[ms] | SLO Viol. [%] | Migr. Data [MB] | Loc. State Av. [%] | 
|-------------|-----------------|---------|--------|---------------|-----------------|--------------------|
| 100         | 1               | Skylark |        |               |                 |                    |