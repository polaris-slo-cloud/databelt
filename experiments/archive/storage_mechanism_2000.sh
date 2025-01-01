#!/bin/bash

SAT="pi4u5"
CLOUD="pi5u1"
H_SINGLE="Host: $SAT-single.default.svc.cluster.local"
H_BUNDLE="Host: $SAT-bundled.default.svc.cluster.local"

HOST_STATEFUL="http://10.152.183.86/get-and-set?destination=$SAT&key="
HOST_STATELESS="http://10.152.183.86/get-and-set?destination=$CLOUD&key="

# Keys of states with size 2MB both single and bundled of depth 1-5
K_SINGLE_SAT_1=71d7121c-23e2-444c-9f66-08b9415db50a:10.0.0.58:60a0d930-0ea1-4d45-a72f-b8027335691f
K_SINGLE_CLOUD_1=8d541447-d012-4dfc-b2c5-a52a256d32f8:10.0.0.243:fce904a5-13d4-4532-b073-6266767df772
K_BUNDLE_SAT_1=814dd87f-6a5b-470c-b67c-70b56066db56:10.0.0.58:c5954fa6-ba3a-4d13-98e8-35b01f6980f1
K_BUNDLE_CLOUD_1=c19a8271-4365-4537-83a8-8748453feb94:10.0.0.243:b6ee6a9e-5db3-499e-abd4-43cf4ecab789
K_BUNDLE_SAT_2=692acf16-ee63-4476-9791-ed4d66e35fe1:10.0.0.58:ee8e03e9-b41f-4adc-aeaa-fb573040ea9c
K_BUNDLE_CLOUD_2=ac7ded8c-4eb3-4a63-bab9-5ab57c7a002e:10.0.0.243:acbbfed3-19bb-4bd7-a18a-611362948037
K_BUNDLE_SAT_3=e2024670-e590-4cd5-81c9-01cf8212da3d:10.0.0.58:a348f080-b23d-44ce-8af5-cc8de4ec50a5
K_BUNDLE_CLOUD_3=5101add8-1be5-4e1d-8c4e-bb629091a030:10.0.0.243:bff7e4ca-0d21-42af-b564-57b466b240c4
K_BUNDLE_SAT_4=49ee16e6-19d0-4787-a7eb-12eaad3c9905:10.0.0.58:24e150c8-8917-444d-9848-ff666dfe331e
K_BUNDLE_CLOUD_4=6093fb96-ca90-4e52-8a5c-4a8c854bfd9b:10.0.0.243:086e266e-8654-4697-8029-69c66bc71168
K_BUNDLE_SAT_5=49d09f2b-07cf-42f0-8043-7deef5cbe95a:10.0.0.58:dea0ce3e-1f13-4e4a-a50b-c24f1dbd12a1
K_BUNDLE_CLOUD_5=3710061d-0783-47d2-a86b-ae41156448b1:10.0.0.243:21bf360f-6f3d-4bff-8e5b-dcae0f0ed7d3

echo "Starting Storage Mechanism Experiment"
echo "State Size: 2MB"
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