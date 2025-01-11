#!/bin/bash

# ./run_bundled.sh 1; ./run_bundled.sh 2; ./run_bundled.sh 3; ./run_bundled.sh 4; ./run_bundled.sh 5
# Bundle size 1-5
# State size 10MB
# Default values for optional arguments
# Measuring overhead

# Sate keys
declare -A keys
keys["SS1"]="d7c0eee7-535d-4993-b54d-2d20ab5ff430:10.0.0.34:a2be9783-6670-4aed-a1e4-bff07ac3d322"
keys["SC1"]="efc6b1dd-1350-43fe-9523-642baf3a24d0:10.0.0.243:0540af22-9994-4061-a0de-5de896c16ef3"
keys["BS1"]="2824bc33-4edf-4139-b826-491eca1fd9c4:10.0.0.34:07f3968e-4242-47da-84f0-0716c40b047c"
keys["BC1"]="3edcf8b5-cfb0-4cdf-bffb-9a47f701e871:10.0.0.243:6fe2dd35-c5b4-4de9-ad31-177a9d131d0c"
keys["BS2"]="a57d934a-7564-4991-b955-d4dc2210a15a:10.0.0.34:346360b2-02c1-41ec-a34f-7f2e0cf64989"
keys["BC2"]="01fe0eaf-5860-439e-90fa-222a0357bfaa:10.0.0.243:43cd8d6d-81d2-4af6-8262-b7337f08c6aa"
keys["BS3"]="aca2e4bc-a146-47fd-ab06-fa775c3963bb:10.0.0.34:38bad0b5-16a2-4927-a45c-de332e59f880"
keys["BC3"]="f8d05885-8db4-42af-9542-47ef3be918af:10.0.0.243:dfe0e33b-a485-4c84-b112-41858f77ebbd"
keys["BS4"]="fc69f286-9555-4358-ba89-eff273c16c2a:10.0.0.34:44d04326-a125-41b4-8a2f-b705897e9f8a"
keys["BC4"]="7928c888-acc8-47d5-a7eb-99597f6c801d:10.0.0.243:24540523-cf8e-4ad2-be8f-c0a1c89fc880"
keys["BS5"]="44c54241-0d13-4815-a639-5c750aabdfce:10.0.0.34:aa6a3e98-2787-42c3-963d-abaa999a6e4d"
keys["BC5"]="49661694-e0b7-4f1a-a52b-d708c6ba978a:10.0.0.243:8349c631-5573-4ac8-9d7c-f9b78d1b2b01"


B=$1
CLOUD="pi5u1"
SAT="pi5u2"
H_SINGLE="Host: $SAT-single.default.svc.cluster.local"
H_BUNDLE="Host: $SAT-bundled.default.svc.cluster.local"

HOST_DCLOUD="http://10.152.183.221/get-and-set?destination=$CLOUD&key="
HOST_DSAT="http://10.152.183.221/get-and-set?destination=$SAT&key="

echo "Storage Mechanism"
echo "State Size: 10MB"
echo "Bundle Size: $B" >> bundle_performance.log
echo "Sat Node: $SAT"
echo "Cloud Node: $CLOUD"
echo "Latency: 60ms"
echo -e "\n--------------------------------------------------------\n"
echo -e "Single-Stateful  Single-Stateless	Bundled-Stateful  Bundled-Stateless"
echo -e "Run,Tt,Tr,Tm,Tt,Tr,Tm,Tt,Tr,Tm,Tt,Tr,Tm"
for i in {0..10}; do
  # Single-Stateless
  ssl_tr=0
  ssl_tm=0
  START_SSL=$(date +%s%3N)
  for b in $(seq 1 "$B"); do
    read tr tm <<< $(curl -s "$HOST_DCLOUD${keys["SC1"]}" -H "$H_SINGLE")
    ssl_tr=$((ssl_tr + tr))
    ssl_tm=$((ssl_tm + tm))
  done
  END_SSL=$(date +%s%3N)
  ssl_tt=$((END_SSL - START_SSL))
  
  # Single-Stateful
  ssf_tr=0
  ssf_tm=0
  START_SSF=$(date +%s%3N)
  for b in $(seq 1 "$B"); do
    read tr tm <<< $(curl -s "$HOST_DSAT${keys["SS1"]}" -H "$H_SINGLE")
    ssf_tr=$((ssf_tr + tr))
    ssf_tm=$((ssf_tm + tm))
  done
  END_SSF=$(date +%s%3N)
  ssf_tt=$((END_SSF - START_SSF))
  
  # Bundle-Stateless
  START_BSL=$(date +%s%3N)
  read bsl_tr bsl_tm <<< $(curl -s "$HOST_DCLOUD${keys["BC$b"]}" -H "$H_BUNDLE")
  END_BSL=$(date +%s%3N)
  bsl_tt=$((END_BSL - START_BSL))
    
  # Bundle-Stateful
  START_BSF=$(date +%s%3N)
  read bsf_tr bsf_tm <<< $(curl -s "$HOST_DSAT${keys["BS$b"]}" -H "$H_BUNDLE")
  END_BSF=$(date +%s%3N)
  bsf_tt=$((END_BSF - START_BSF))

  echo -e "$i,$ssf_tt,$ssf_tr,$ssf_tm,$ssl_tt,$ssl_tr,$ssl_tm,$bsf_tt,$bsf_tr,$bsf_tm,$bsl_tt,$bsl_tr,$bsl_tm"  >> bundle_performance.log
done

echo -e "\n--------------------------------------------------------\n"

