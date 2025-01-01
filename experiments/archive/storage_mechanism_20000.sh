#!/bin/bash

SAT="pi4u5"
CLOUD="pi5u1"
H_SINGLE="Host: $SAT-single.default.svc.cluster.local"
H_BUNDLE="Host: $SAT-bundled.default.svc.cluster.local"

HOST_STATEFUL="http://10.152.183.86/get-and-set?destination=$SAT&key="
HOST_STATELESS="http://10.152.183.86/get-and-set?destination=$CLOUD&key="

# Keys of states with size 20MB both single and bundled of depth 1-5
K_SINGLE_SAT_1=b57cea06-e9a3-4b7a-a8b4-e929c1d16d81:10.0.0.58:ed6e0b49-bbf3-45ae-8527-f55d4cb644dd
K_SINGLE_CLOUD_1=8322e6da-6f50-4172-816d-2f6594d9dd49:10.0.0.243:d02160f2-d65f-45b9-930b-4514f95cf5e4
K_BUNDLE_SAT_1=a7200bb9-e275-492f-8c74-a3340184d9e1:10.0.0.58:737d1aef-837b-4301-a01e-2f68583c0547
K_BUNDLE_CLOUD_1=3f2c075a-7ed4-404d-a21f-7e13c39ca1fd:10.0.0.243:4fdeca7d-370d-4e1c-80ff-2b885afaf9a2
K_BUNDLE_SAT_2=ed14e76c-364c-4860-9c59-b3c220c02fe3:10.0.0.58:328ad688-1e08-4e9f-bba0-27a8830b7144
K_BUNDLE_CLOUD_2=178c2c27-5276-45c6-91a8-59f42fd48452:10.0.0.243:5d4128be-5c58-429e-94b4-5814a89c594e
K_BUNDLE_SAT_3=6bd56274-3eeb-48cd-be0b-d9239614b63e:10.0.0.58:e5ea80cb-88e2-44bc-bccc-8c65ca9a8fa6
K_BUNDLE_CLOUD_3=385dac39-55e8-4b32-96ad-2a2b80a97dec:10.0.0.243:cb2a7744-731e-4cb9-b548-eb9a6c696033
K_BUNDLE_SAT_4=dd861364-412a-44b8-bd5b-aca32d64f699:10.0.0.58:984e97d0-b1bc-467d-87f4-3889b706e837
K_BUNDLE_CLOUD_4=f61552ed-ec4f-46fb-a2ff-fe7a3e12f036:10.0.0.243:96cf6999-7f68-4e42-9f94-ff68cd241d3c
K_BUNDLE_SAT_5=2562b2fb-570c-400f-a0e8-52c218c499b5:10.0.0.58:160ffdf1-931b-4538-88f7-344c6b80e9ef
K_BUNDLE_CLOUD_5=7116e064-14ca-4350-b71a-a938dab78a56:10.0.0.243:a734f281-cc01-4ed5-bd3e-b65d38582a5e

echo "Starting Storage Mechanism Experiment"
echo "State Size: 20MB"
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