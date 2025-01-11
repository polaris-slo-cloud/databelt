#!/bin/bash

SAT="pi4u5"
CLOUD="pi5u1"
H_SINGLE="Host: $SAT-single.default.svc.cluster.local"
H_BUNDLE="Host: $SAT-bundled.default.svc.cluster.local"

HOST_STATEFUL="http://10.152.183.221/get-and-set?destination=$SAT&key="
HOST_STATELESS="http://10.152.183.221/get-and-set?destination=$CLOUD&key="

# Keys of states with size 5MB both single and bundled of depth 1-5
K_SINGLE_SAT_1=81714b59-0f88-4bf9-98fd-0f1f14bd7ea3:10.0.0.58:245f4b66-151b-4607-8174-ec6a4962018e
K_SINGLE_CLOUD_1=9135f1a8-66eb-42bd-9155-799c08290630:10.0.0.243:9d458f00-9bdd-4c30-b436-a189f9f56961
K_BUNDLE_SAT_1=5f36fa93-26c1-47b6-b9bd-cb0eb5b5ad07:10.0.0.58:a52dedbe-914e-4036-9fe8-e01c072073de
K_BUNDLE_CLOUD_1=bea5ed36-7484-4455-bb68-cc73c55eed83:10.0.0.243:21fdc627-85bd-44ed-b632-251dba956abe
K_BUNDLE_SAT_2=29be3027-5664-424f-9bde-4463bd8672d2:10.0.0.58:a6219c51-97f6-46e4-aa3f-581c96f95286
K_BUNDLE_CLOUD_2=74cfe6e4-b0bf-46f0-8e7b-84199426f3f7:10.0.0.243:34f511e2-feea-4eac-84f0-78128fff65b7
K_BUNDLE_SAT_3=026ee760-dd8c-4f5d-be44-08f4b5f1f8df:10.0.0.58:c9ed11b6-fb74-4157-839c-7cf9e56422b6
K_BUNDLE_CLOUD_3=ca2a79b5-bd74-4824-8781-15f65271f0fb:10.0.0.243:34854bc8-20cc-4ab9-9895-9be245a1f257
K_BUNDLE_SAT_4=fa3a65ff-c816-4917-9dc3-082217ef7d42:10.0.0.58:5fcd3d0e-1a95-456c-b016-e5ee93afcc13
K_BUNDLE_CLOUD_4=70b1e97b-d429-4cc2-9fe6-3d2a404e887e:10.0.0.243:e650044d-b540-4447-8d27-4b742f13c3dc
K_BUNDLE_SAT_5=c1256317-e12a-47d4-a1e6-e0ed27339f30:10.0.0.58:4dc6ea40-f99d-4824-8499-84f5b94e3c6c
K_BUNDLE_CLOUD_5=bf98d6f7-2717-4883-ac6d-6d5e90ad312a:10.0.0.243:6a5561be-3bf7-497b-bba0-931b5893328f

echo "Starting Storage Mechanism Experiment"
echo "State Size: 5MB"
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