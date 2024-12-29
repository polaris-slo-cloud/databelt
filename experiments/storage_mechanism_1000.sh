#!/bin/bash

SAT="pi4u5"
CLOUD="pi5u1"
H_SINGLE="Host: $SAT-single.default.svc.cluster.local"
H_BUNDLE="Host: $SAT-bundled.default.svc.cluster.local"

HOST_STATEFUL="http://10.152.183.86/get-and-set?destination=$SAT&key="
HOST_STATELESS="http://10.152.183.86/get-and-set?destination=$CLOUD&key="

# Keys of states with size 1MB both single and bundled of depth 1-5
K_SINGLE_SAT_1=f9205b07-1c84-40bf-8f95-1f560566ba47:10.0.0.58:2338379e-b6e5-4762-98ae-27540c6442bc
K_SINGLE_CLOUD_1=29553585-ae34-4849-9350-1a85c704efba:10.0.0.243:f7a980a6-60d0-4e48-93ec-81c51d3223ee
K_BUNDLE_SAT_1=fb3c2e6e-a357-4e27-802b-a763f9b5aa31:10.0.0.58:d6889aab-c7ea-4ed9-83d6-07dec5b4ba1a
K_BUNDLE_CLOUD_1=48246f3b-be9e-442c-b5dd-5317648a135f:10.0.0.243:e1536629-fabc-45ed-a2c3-f6278a09e2ef
K_BUNDLE_SAT_2=6e22a2d8-3ded-4081-a725-a60760168d50:10.0.0.58:9e6e366a-40ad-44cd-8098-2a0184f0c4e5
K_BUNDLE_CLOUD_2=2fe687af-f3f8-41f8-9888-ca610ab311ea:10.0.0.243:0910e55a-9400-4459-8165-408b99989103
K_BUNDLE_SAT_3=4a7203dd-0e1c-4b6a-b1af-6c1037f5bdef:10.0.0.58:c9b3f35c-042d-4dda-94ce-03ca0b93b6a6
K_BUNDLE_CLOUD_3=ff8024e1-d033-4722-809f-15e1fd143e86:10.0.0.243:3bf1e9c7-370d-40da-bef7-041705378036
K_BUNDLE_SAT_4=b1e13e43-18bd-47dc-a235-b70926a81a0b:10.0.0.58:8d19d751-d9f5-4d26-8bb5-11a8b76d2674
K_BUNDLE_CLOUD_4=ae8ef3d6-cd6a-4ca0-9540-6bc865eeb1c4:10.0.0.243:f10fb716-f918-4d27-a654-00a971dbcc9f
K_BUNDLE_SAT_5=5f9da2ce-a39f-4ef3-813f-5e65553f7e4e:10.0.0.58:4d6c0227-e5d4-4aa5-8092-fb96e2003766
K_BUNDLE_CLOUD_5=b7a02a59-a931-4219-8310-4f4a3ef96378:10.0.0.243:4b4f3d9c-a17c-4a8e-bcda-08bd3d030470

ssl_sumdrs=()
ssl_sumdms=()
ssf_sumdrs=()
ssf_sumdms=()
bsl_sumdrs=()
bsl_sumdms=()
bsf_sumdrs=()
bsf_sumdms=()

echo "Starting Storage Mechanism Experiment"
echo "State Size: 1MB"
echo "Sat Node: $SAT"
echo "Cloud Node: $CLOUD"
echo "Latency: 60ms 7ms jitter"
echo "Bandwidth: 35Mbit"

# Single-Stateless
echo -e "\nRun\tT(dr)\tT(dm)\tSingle-Stateless-1"
read dr dm <<< $(curl -s "$HOST_STATELESS$K_SINGLE_CLOUD_1" -H "$H_SINGLE")
echo -e "1\t$dr\t$dm"
ssl_sumdrs+=($dr)
ssl_sumdms+=($dm)
read dr dm <<< $(curl -s "$HOST_STATELESS$K_SINGLE_CLOUD_1" -H "$H_SINGLE")
echo -e "2\t$dr\t$dm"
ssl_sumdrs+=($dr)
ssl_sumdms+=($dm)
echo -e "\nRun\tT(dr)\tT(dm)\tSingle-Stateless-2"
read dr1 dm1 <<< $(curl -s "$HOST_STATELESS$K_SINGLE_CLOUD_1" -H "$H_SINGLE")
read dr2 dm2 <<< $(curl -s "$HOST_STATELESS$K_SINGLE_CLOUD_1" -H "$H_SINGLE")
sumdr=$((dr1 + dr2))
sumdm=$((dm1 + dm2))
ssl_sumdrs+=($sumdr)
ssl_sumdms+=($sumdm)
echo -e "3\t$sumdr\t$sumdm"
read dr1 dm1 <<< $(curl -s "$HOST_STATELESS$K_SINGLE_CLOUD_1" -H "$H_SINGLE")
read dr2 dm2 <<< $(curl -s "$HOST_STATELESS$K_SINGLE_CLOUD_1" -H "$H_SINGLE")
sumdr=$((dr1 + dr2))
sumdm=$((dm1 + dm2))
ssl_sumdrs+=($sumdr)
ssl_sumdms+=($sumdm)
echo -e "4\t$sumdr\t$sumdm"

echo -e "\nRun\tT(dr)\tT(dm)\tSingle-Stateless-3"
read dr1 dm1 <<< $(curl -s "$HOST_STATELESS$K_SINGLE_CLOUD_1" -H "$H_SINGLE")
read dr2 dm2 <<< $(curl -s "$HOST_STATELESS$K_SINGLE_CLOUD_1" -H "$H_SINGLE")
read dr3 dm3 <<< $(curl -s "$HOST_STATELESS$K_SINGLE_CLOUD_1" -H "$H_SINGLE")
sumdr=$((dr1 + dr2 + dr3))
sumdm=$((dm1 + dm2 + dr3))
ssl_sumdrs+=($sumdr)
ssl_sumdms+=($sumdm)
echo -e "5\t$sumdr\t$sumdm"
read dr1 dm1 <<< $(curl -s "$HOST_STATELESS$K_SINGLE_CLOUD_1" -H "$H_SINGLE")
read dr2 dm2 <<< $(curl -s "$HOST_STATELESS$K_SINGLE_CLOUD_1" -H "$H_SINGLE")
read dr3 dm3 <<< $(curl -s "$HOST_STATELESS$K_SINGLE_CLOUD_1" -H "$H_SINGLE")
sumdr=$((dr1 + dr2 + dr3))
sumdm=$((dm1 + dm2 + dm3))
ssl_sumdrs+=($sumdr)
ssl_sumdms+=($sumdm)
echo -e "6\t$sumdr\t$sumdm"

echo -e "\nRun\tT(dr)\tT(dm)\tSingle-Stateless-4"
read dr1 dm1 <<< $(curl -s "$HOST_STATELESS$K_SINGLE_CLOUD_1" -H "$H_SINGLE")
read dr2 dm2 <<< $(curl -s "$HOST_STATELESS$K_SINGLE_CLOUD_1" -H "$H_SINGLE")
read dr3 dm3 <<< $(curl -s "$HOST_STATELESS$K_SINGLE_CLOUD_1" -H "$H_SINGLE")
read dr4 dm4 <<< $(curl -s "$HOST_STATELESS$K_SINGLE_CLOUD_1" -H "$H_SINGLE")
sumdr=$((dr1 + dr2 + dr3 + dr4))
sumdm=$((dm1 + dm2 + dm3 + dm4))
ssl_sumdrs+=($sumdr)
ssl_sumdms+=($sumdm)
echo -e "7\t$sumdr\t$sumdm"
read dr1 dm1 <<< $(curl -s "$HOST_STATELESS$K_SINGLE_CLOUD_1" -H "$H_SINGLE")
read dr2 dm2 <<< $(curl -s "$HOST_STATELESS$K_SINGLE_CLOUD_1" -H "$H_SINGLE")
read dr3 dm3 <<< $(curl -s "$HOST_STATELESS$K_SINGLE_CLOUD_1" -H "$H_SINGLE")
read dr4 dm4 <<< $(curl -s "$HOST_STATELESS$K_SINGLE_CLOUD_1" -H "$H_SINGLE")
sumdr=$((dr1 + dr2 + dr3 + dr4))
sumdm=$((dm1 + dm2 + dm3 + dm4))
ssl_sumdrs+=($sumdr)
ssl_sumdms+=($sumdm)
echo -e "8\t$sumdr\t$sumdm"

echo -e "\nRun\tT(dr)\tT(dm)\tSingle-Stateless-5"
read dr1 dm1 <<< $(curl -s "$HOST_STATELESS$K_SINGLE_CLOUD_1" -H "$H_SINGLE")
read dr2 dm2 <<< $(curl -s "$HOST_STATELESS$K_SINGLE_CLOUD_1" -H "$H_SINGLE")
read dr3 dm3 <<< $(curl -s "$HOST_STATELESS$K_SINGLE_CLOUD_1" -H "$H_SINGLE")
read dr4 dm4 <<< $(curl -s "$HOST_STATELESS$K_SINGLE_CLOUD_1" -H "$H_SINGLE")
read dr5 dm5 <<< $(curl -s "$HOST_STATELESS$K_SINGLE_CLOUD_1" -H "$H_SINGLE")
sumdr=$((dr1 + dr2 + dr3 + dr4 + dr5))
sumdm=$((dm1 + dm2 + dm3 + dm4 + dm5))
ssl_sumdrs+=($sumdr)
ssl_sumdms+=($sumdm)
echo -e "9\t$sumdr\t$sumdm"
read dr1 dm1 <<< $(curl -s "$HOST_STATELESS$K_SINGLE_CLOUD_1" -H "$H_SINGLE")
read dr2 dm2 <<< $(curl -s "$HOST_STATELESS$K_SINGLE_CLOUD_1" -H "$H_SINGLE")
read dr3 dm3 <<< $(curl -s "$HOST_STATELESS$K_SINGLE_CLOUD_1" -H "$H_SINGLE")
read dr4 dm4 <<< $(curl -s "$HOST_STATELESS$K_SINGLE_CLOUD_1" -H "$H_SINGLE")
read dr5 dm5 <<< $(curl -s "$HOST_STATELESS$K_SINGLE_CLOUD_1" -H "$H_SINGLE")
sumdr=$((dr1 + dr2 + dr3 + dr4 + dr5))
sumdm=$((dm1 + dm2 + dm3 + dm4 + dm5))
ssl_sumdrs+=($sumdr)
ssl_sumdms+=($sumdm)
echo -e "10\t$sumdr\t$sumdm"
echo -e "\n--------------------------------------------------------"

# Single-Stateful
echo -e "\nRun\tT(dr)\tT(dm)\tSingle-Stateful-1"
read dr dm <<< $(curl -s "$HOST_STATEFUL$K_SINGLE_SAT_1" -H "$H_SINGLE")
ssf_sumdrs+=($dr)
ssf_sumdms+=($dm)
echo -e "1\t$dr\t$dm"

read dr dm <<< $(curl -s "$HOST_STATEFUL$K_SINGLE_SAT_1" -H "$H_SINGLE")
ssf_sumdrs+=($dr)
ssf_sumdms+=($dm)
echo -e "2\t$dr\t$dm"

echo -e "\nRun\tT(dr)\tT(dm)\tSingle-Stateful-2"
read dr1 dm1 <<< $(curl -s "$HOST_STATEFUL$K_SINGLE_SAT_1" -H "$H_SINGLE")
read dr2 dm2 <<< $(curl -s "$HOST_STATEFUL$K_SINGLE_SAT_1" -H "$H_SINGLE")
sumdr=$((dr1 + dr2))
sumdm=$((dm1 + dm2))
ssf_sumdrs+=($sumdr)
ssf_sumdms+=($sumdm)
echo -e "3\t$sumdr\t$sumdm"

read dr1 dm1 <<< $(curl -s "$HOST_STATEFUL$K_SINGLE_SAT_1" -H "$H_SINGLE")
read dr2 dm2 <<< $(curl -s "$HOST_STATEFUL$K_SINGLE_SAT_1" -H "$H_SINGLE")
sumdr=$((dr1 + dr2))
sumdm=$((dm1 + dm2))
ssf_sumdrs+=($sumdr)
ssf_sumdms+=($sumdm)
echo -e "4\t$sumdr\t$sumdm"

echo -e "\nRun\tT(dr)\tT(dm)\tSingle-Stateful-3"
read dr1 dm1 <<< $(curl -s "$HOST_STATEFUL$K_SINGLE_SAT_1" -H "$H_SINGLE")
read dr2 dm2 <<< $(curl -s "$HOST_STATEFUL$K_SINGLE_SAT_1" -H "$H_SINGLE")
read dr3 dm3 <<< $(curl -s "$HOST_STATEFUL$K_SINGLE_SAT_1" -H "$H_SINGLE")
sumdr=$((dr1 + dr2 + dr3))
sumdm=$((dm1 + dm2 + dm3))
ssf_sumdrs+=($sumdr)
ssf_sumdms+=($sumdm)
echo -e "5\t$sumdr\t$sumdm"

read dr1 dm1 <<< $(curl -s "$HOST_STATEFUL$K_SINGLE_SAT_1" -H "$H_SINGLE")
read dr2 dm2 <<< $(curl -s "$HOST_STATEFUL$K_SINGLE_SAT_1" -H "$H_SINGLE")
read dr3 dm3 <<< $(curl -s "$HOST_STATEFUL$K_SINGLE_SAT_1" -H "$H_SINGLE")
sumdr=$((dr1 + dr2 + dr3))
sumdm=$((dm1 + dm2 + dm3))
ssf_sumdrs+=($sumdr)
ssf_sumdms+=($sumdm)
echo -e "6\t$sumdr\t$sumdm"

echo -e "\nRun\tT(dr)\tT(dm)\tSingle-Stateful-4"
read dr1 dm1 <<< $(curl -s "$HOST_STATEFUL$K_SINGLE_SAT_1" -H "$H_SINGLE")
read dr2 dm2 <<< $(curl -s "$HOST_STATEFUL$K_SINGLE_SAT_1" -H "$H_SINGLE")
read dr3 dm3 <<< $(curl -s "$HOST_STATEFUL$K_SINGLE_SAT_1" -H "$H_SINGLE")
read dr4 dm4 <<< $(curl -s "$HOST_STATEFUL$K_SINGLE_SAT_1" -H "$H_SINGLE")
sumdr=$((dr1 + dr2 + dr3 + dr4))
sumdm=$((dm1 + dm2 + dm3 + dm4))
ssf_sumdrs+=($sumdr)
ssf_sumdms+=($sumdm)
echo -e "7\t$sumdr\t$sumdm"

read dr1 dm1 <<< $(curl -s "$HOST_STATEFUL$K_SINGLE_SAT_1" -H "$H_SINGLE")
read dr2 dm2 <<< $(curl -s "$HOST_STATEFUL$K_SINGLE_SAT_1" -H "$H_SINGLE")
read dr3 dm3 <<< $(curl -s "$HOST_STATEFUL$K_SINGLE_SAT_1" -H "$H_SINGLE")
read dr4 dm4 <<< $(curl -s "$HOST_STATEFUL$K_SINGLE_SAT_1" -H "$H_SINGLE")
sumdr=$((dr1 + dr2 + dr3 + dr4))
sumdm=$((dm1 + dm2 + dm3 + dm4))
ssf_sumdrs+=($sumdr)
ssf_sumdms+=($sumdm)
echo -e "8\t$sumdr\t$sumdm"

echo -e "\nRun\tT(dr)\tT(dm)\tSingle-Stateful-5"
read dr1 dm1 <<< $(curl -s "$HOST_STATEFUL$K_SINGLE_SAT_1" -H "$H_SINGLE")
read dr2 dm2 <<< $(curl -s "$HOST_STATEFUL$K_SINGLE_SAT_1" -H "$H_SINGLE")
read dr3 dm3 <<< $(curl -s "$HOST_STATEFUL$K_SINGLE_SAT_1" -H "$H_SINGLE")
read dr4 dm4 <<< $(curl -s "$HOST_STATEFUL$K_SINGLE_SAT_1" -H "$H_SINGLE")
read dr5 dm5 <<< $(curl -s "$HOST_STATEFUL$K_SINGLE_SAT_1" -H "$H_SINGLE")
sumdr=$((dr1 + dr2 + dr3 + dr4 + dr5))
sumdm=$((dm1 + dm2 + dm3 + dm4 + dm5))
ssf_sumdrs+=($sumdr)
ssf_sumdms+=($sumdm)
echo -e "9\t$sumdr\t$sumdm"
read dr1 dm1 <<< $(curl -s "$HOST_STATEFUL$K_SINGLE_SAT_1" -H "$H_SINGLE")
read dr2 dm2 <<< $(curl -s "$HOST_STATEFUL$K_SINGLE_SAT_1" -H "$H_SINGLE")
read dr3 dm3 <<< $(curl -s "$HOST_STATEFUL$K_SINGLE_SAT_1" -H "$H_SINGLE")
read dr4 dm4 <<< $(curl -s "$HOST_STATEFUL$K_SINGLE_SAT_1" -H "$H_SINGLE")
read dr5 dm5 <<< $(curl -s "$HOST_STATEFUL$K_SINGLE_SAT_1" -H "$H_SINGLE")
sumdr=$((dr1 + dr2 + dr3 + dr4 + dr5))
sumdm=$((dm1 + dm2 + dm3 + dm4 + dm5))
ssf_sumdrs+=($sumdr)
ssf_sumdms+=($sumdm)
echo -e "10\t$sumdr\t$sumdm"
echo -e "\n--------------------------------------------------------"

# Bundled-Stateless
echo -e "\nRun\tT(dr)\tT(dm)\tBundled-Stateless-1"
read dr dm <<< $(curl -s "$HOST_STATELESS$K_BUNDLE_CLOUD_1" -H "$H_BUNDLE")
bsl_sumdms+=($dm)
bsl_sumdrs+=($dr)
echo -e "1\t$dr\t$dm"
read dr dm <<< $(curl -s "$HOST_STATELESS$K_BUNDLE_CLOUD_1" -H "$H_BUNDLE")
bsl_sumdms+=($dm)
bsl_sumdrs+=($dr)
echo -e "2\t$dr\t$dm"
echo -e "\nRun\tT(dr)\tT(dm)\tBundled-Stateless-2"
read dr dm <<< $(curl -s "$HOST_STATELESS$K_BUNDLE_CLOUD_2" -H "$H_BUNDLE")
bsl_sumdms+=($dm)
bsl_sumdrs+=($dr)
echo -e "3\t$dr\t$dm"
read dr dm <<< $(curl -s "$HOST_STATELESS$K_BUNDLE_CLOUD_2" -H "$H_BUNDLE")
bsl_sumdms+=($dm)
bsl_sumdrs+=($dr)
echo -e "4\t$dr\t$dm"
echo -e "\nRun\tT(dr)\tT(dm)\tBundled-Stateless-3"
read dr dm <<< $(curl -s "$HOST_STATELESS$K_BUNDLE_CLOUD_3" -H "$H_BUNDLE")
bsl_sumdms+=($dm)
bsl_sumdrs+=($dr)
echo -e "5\t$dr\t$dm"
read dr dm <<< $(curl -s "$HOST_STATELESS$K_BUNDLE_CLOUD_3" -H "$H_BUNDLE")
bsl_sumdms+=($dm)
bsl_sumdrs+=($dr)
echo -e "6\t$dr\t$dm"
echo -e "\nRun\tT(dr)\tT(dm)\tBundled-Stateless-4"
read dr dm <<< $(curl -s "$HOST_STATELESS$K_BUNDLE_CLOUD_4" -H "$H_BUNDLE")
bsl_sumdms+=($dm)
bsl_sumdrs+=($dr)
echo -e "7\t$dr\t$dm"
read dr dm <<< $(curl -s "$HOST_STATELESS$K_BUNDLE_CLOUD_4" -H "$H_BUNDLE")
bsl_sumdms+=($dm)
bsl_sumdrs+=($dr)
echo -e "8\t$dr\t$dm"
echo -e "\nRun\tT(dr)\tT(dm)\tBundled-Stateless-5"
read dr dm <<< $(curl -s "$HOST_STATELESS$K_BUNDLE_CLOUD_5" -H "$H_BUNDLE")
bsl_sumdms+=($dm)
bsl_sumdrs+=($dr)
echo -e "9\t$dr\t$dm"
read dr dm <<< $(curl -s "$HOST_STATELESS$K_BUNDLE_CLOUD_5" -H "$H_BUNDLE")
bsl_sumdms+=($dm)
bsl_sumdrs+=($dr)
echo -e "10\t$dr\t$dm"
echo -e "\n--------------------------------------------------------"
# Bundled-Stateful
echo -e "\nRun\tT(dr)\tT(dm)\tBundled-Stateful-1"
read dr dm <<< $(curl -s "$HOST_STATEFUL$K_BUNDLE_SAT_1" -H "$H_BUNDLE")
bsf_sumdms+=($dm)
bsf_sumdrs+=($dr)
echo -e "1\t$dr\t$dm"
read dr dm <<< $(curl -s "$HOST_STATEFUL$K_BUNDLE_SAT_1" -H "$H_BUNDLE")
bsf_sumdms+=($dm)
bsf_sumdrs+=($dr)
echo -e "2\t$dr\t$dm"
echo -e "\nRun\tT(dr)\tT(dm)\tBundled-Stateful-2"
read dr dm <<< $(curl -s "$HOST_STATEFUL$K_BUNDLE_SAT_2" -H "$H_BUNDLE")
bsf_sumdms+=($dm)
bsf_sumdrs+=($dr)
echo -e "3\t$dr\t$dm"
read dr dm <<< $(curl -s "$HOST_STATEFUL$K_BUNDLE_SAT_2" -H "$H_BUNDLE")
bsf_sumdms+=($dm)
bsf_sumdrs+=($dr)
echo -e "4\t$dr\t$dm"
echo -e "\nRun\tT(dr)\tT(dm)\tBundled-Stateful-3"
read dr dm <<< $(curl -s "$HOST_STATEFUL$K_BUNDLE_SAT_3" -H "$H_BUNDLE")
bsf_sumdms+=($dm)
bsf_sumdrs+=($dr)
echo -e "5\t$dr\t$dm"
read dr dm <<< $(curl -s "$HOST_STATEFUL$K_BUNDLE_SAT_3" -H "$H_BUNDLE")
bsf_sumdms+=($dm)
bsf_sumdrs+=($dr)
echo -e "6\t$dr\t$dm"
echo -e "\nRun\tT(dr)\tT(dm)\tBundled-Stateful-4"
read dr dm <<< $(curl -s "$HOST_STATEFUL$K_BUNDLE_SAT_4" -H "$H_BUNDLE")
bsf_sumdms+=($dm)
bsf_sumdrs+=($dr)
echo -e "7\t$dr\t$dm"
read dr dm <<< $(curl -s "$HOST_STATEFUL$K_BUNDLE_SAT_4" -H "$H_BUNDLE")
bsf_sumdms+=($dm)
bsf_sumdrs+=($dr)
echo -e "8\t$dr\t$dm"
echo -e "\nRun\tT(dr)\tT(dm)\tBundled-Stateful-5"
read dr dm <<< $(curl -s "$HOST_STATEFUL$K_BUNDLE_SAT_5" -H "$H_BUNDLE")
bsf_sumdms+=($dm)
bsf_sumdrs+=($dr)
echo -e "9\t$dr\t$dm"
read dr dm <<< $(curl -s "$HOST_STATEFUL$K_BUNDLE_SAT_5" -H "$H_BUNDLE")
bsf_sumdms+=($dm)
bsf_sumdrs+=($dr)
echo -e "10\t$dr\t$dm"
echo -e "\n--------------------------------------------------------"


echo -e "run,ssl_tdr,ssl_tdm,ssl_tdr,ssf_tdm,bsf_tdr,bsl_tdm,bsf_tdr,bsf_tdm"
for i in {0..9}; do
  echo "$i,${ssl_sumdrs[i]},${ssl_sumdms[i]},${ssf_sumdrs[i]},${ssf_sumdms[i]},${bsl_sumdrs[i]},${bsl_sumdms[i]},${bsf_sumdrs[i]},${bsf_sumdms[i]}"
done