#!/bin/bash

# Measuring overhead
DEST="pi4u5"
SAT="pi4u5"
H_SINGLE="Host: $SAT-single.default.svc.cluster.local"
H_BUNDLE="Host: $SAT-bundled.default.svc.cluster.local"

HOST="http://10.152.183.86/get-and-set?destination=$SAT&key="

# Keys of states with size 1MB both single and bundled of depth 1-5
K_SINGLE_SAT_1=f9205b07-1c84-40bf-8f95-1f560566ba47:10.0.0.58:2338379e-b6e5-4762-98ae-27540c6442bc
K_BUNDLE_SAT_3=4a7203dd-0e1c-4b6a-b1af-6c1037f5bdef:10.0.0.58:c9b3f35c-042d-4dda-94ce-03ca0b93b6a6
K_SINGLE_CLOUD_1=29553585-ae34-4849-9350-1a85c704efba:10.0.0.243:f7a980a6-60d0-4e48-93ec-81c51d3223ee
K_BUNDLE_CLOUD_3=ff8024e1-d033-4722-809f-15e1fd143e86:10.0.0.243:3bf1e9c7-370d-40da-bef7-041705378036

ssf_sumdrs=()
ssf_sumdms=()
bsf_sumdrs=()
bsf_sumdms=()

echo "Starting Storage Mechanism Experiment - BASELINE"
echo "State Size: 1MB"
echo "Bundle Size: 3"
echo "Execution Node: $SAT"
echo "Destination Node: $CLOUD"
echo "Latency: None"
echo "Bandwidth: Unlimited"
echo "$HOST$K_SINGLE_SAT_1 -H $H_SINGLE"
echo "$HOST$K_BUNDLE_SAT_3 -H $H_BUNDLE"
echo -e "\n--------------------------------------------------------\n"
for i in {1..100}; do
    # Single
    read dr1 dm1 <<< $(curl -s "$HOST$K_SINGLE_SAT_1" -H "$H_SINGLE")
    read dr2 dm2 <<< $(curl -s "$HOST$K_SINGLE_SAT_1" -H "$H_SINGLE")
    read dr3 dm3 <<< $(curl -s "$HOST$K_SINGLE_SAT_1" -H "$H_SINGLE")
    ssf_sumdr=$((dr1 + dr2 + dr3))
    ssf_sumdm=$((dm1 + dm2 + dm3))
     # Bundle
     read bsf_dr bsf_dm <<< $(curl -s "$HOST$K_BUNDLE_SAT_3" -H "$H_BUNDLE")
    echo -e "$i\t$ssf_sumdr\t$ssf_sumdm\t$bsf_dr\t$bsf_dm"
done

echo -e "\n--------------------------------------------------------\n"
