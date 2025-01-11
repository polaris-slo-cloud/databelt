#!/bin/bash

SAT="pi4u5"
CLOUD="pi5u1"
H_SINGLE="Host: $SAT-single.default.svc.cluster.local"
H_BUNDLE="Host: $SAT-bundled.default.svc.cluster.local"

HOST_STATEFUL="http://10.152.183.221/get-and-set?destination=$SAT&key="
HOST_STATELESS="http://10.152.183.221/get-and-set?destination=$CLOUD&key="

# Keys of states with size 500KB both single and bundled of depth 1-5
K_SINGLE_SAT_1=f3fcdf78-46fe-487a-8b11-d465ca5ad50c:10.0.0.58:9dfdbe0e-0255-4126-acd3-8e00e25a4fbe
K_SINGLE_CLOUD_1=ba1b45ad-641f-4385-9223-a2c37ba533af:10.0.0.243:0b95dbd4-e372-48b5-b8c4-1bab345bfb15
K_BUNDLE_SAT_1=a5318eeb-cfc2-4d08-a1d3-53e39d04644a:10.0.0.58:68e35eb2-1a99-4ec2-92e1-5fc6fa66dbbe
K_BUNDLE_CLOUD_1=5da25c4b-369b-4009-be14-999e710a2777:10.0.0.243:57c105d0-4027-4260-ac52-5bfaf7ae149f
K_BUNDLE_SAT_2=5e8846d5-f124-4c91-9efd-b90140054cbd:10.0.0.58:ef21787a-f78d-4578-ab0a-95ee832bc161
K_BUNDLE_CLOUD_2=4d45378e-a349-4458-83c4-a4640b382b8e:10.0.0.243:50bb967c-cdd5-45b1-a9d5-9eab7eaf8b10
K_BUNDLE_SAT_3=4212c35e-65c3-4dbc-bfc5-18b8770663ca:10.0.0.58:e5f5401e-b80c-454d-8331-6a719fb01084
K_BUNDLE_CLOUD_3=46452fad-3fb5-4222-9220-3f4de747b37c:10.0.0.243:7997efa4-6ea8-4936-a8f6-2da8157432f9
K_BUNDLE_SAT_4=442c1c2b-b9ba-4bbb-b97a-b0c939702e0f:10.0.0.58:5ac27162-d2df-46c8-b6d3-2be2eb350d26
K_BUNDLE_CLOUD_4=93fd6442-e32a-45a8-814d-00ef3060739d:10.0.0.243:001b35dc-91a4-4af4-92c6-041f4efa907a
K_BUNDLE_SAT_5=993ffb85-4412-47ca-afc5-2ffee9e251c9:10.0.0.58:eeca55e2-608e-4ac8-85c6-7657cc0e3687
K_BUNDLE_CLOUD_5=dd713db8-07d7-4682-99ae-ba6641a5d037:10.0.0.243:a7055c5c-056f-4b32-baef-af38292056e8

echo "Starting Storage Mechanism Experiment"
echo "State Size: 0.5MB"
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