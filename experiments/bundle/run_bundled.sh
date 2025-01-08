#!/bin/bash

# ./bundle__1M.sh 1
# Bundle size 1-5
# State size 10MB
# Default values for optional arguments
# Measuring overhead

# Sate keys
declare -A keys
keys["SS1"]="e1348879-14b0-466d-b394-401e554cdb75:10.0.0.34:c2d4e7da-10c3-4c07-8b7f-0572253a950b"
keys["SC1"]="a0e4d0bd-14d0-4811-abe6-40c73f489f23:10.0.0.243:1d4a3969-a659-4d39-a8da-56b88bfacc9b"
keys["BS1"]="a47fe193-a75f-4346-9ada-5bb7c667701b:10.0.0.34:a4952c8b-4ba6-47d3-93d3-5887eb1808c5"
keys["BC1"]="ffb797e8-cbde-48eb-8728-5529b450c0be:10.0.0.243:c7757955-b8e2-4ef4-b388-e2406512ba03"
keys["SS2"]="4ebe63ad-e38e-4026-91cc-dcc6095697c8:10.0.0.34:bbc8d811-85e3-49dd-b94f-75394335d95e"
keys["SC2"]="b5f65a1b-5133-428d-927a-df9fb51c6ade:10.0.0.243:af79d694-de11-4e55-87f1-f11705824363"
keys["BS2"]="e6c086e3-3f13-4946-885a-91c79cd1f23c:10.0.0.34:171e7dff-f1a7-4bda-8485-9cb476e877ec"
keys["BC2"]="50940e55-48ae-417b-8637-87c436e7dbe1:10.0.0.243:0ea2cde3-b7c1-40f0-86ea-3cfd803c52ac"
keys["SS3"]="8e1d694b-5712-4752-9474-1be1385b98db:10.0.0.34:05ea059c-3742-401e-a7a3-79aaf1ec207e"
keys["SC3"]="c2ae32c4-a8df-48dc-bc07-b34ef01a16e6:10.0.0.243:bc1518f1-3cbb-4c5b-a157-080fa88d6de6"
keys["BS3"]="4243ae2a-4ba3-46ee-b397-81b2c63b7b07:10.0.0.34:9181bfaa-bf37-4296-8225-88a7614a075d"
keys["BC3"]="8ab7c55a-e1de-42e5-8b3c-f8ac31f68785:10.0.0.243:d42b6b33-0afe-4627-b9b5-209b57249e97"
keys["SS4"]="46a6e1ce-7905-49c9-8de1-e917dcf943d5:10.0.0.34:0810f6bf-4c44-4115-ba73-2cf17c2d1478"
keys["SC4"]="38e02191-2c5c-4283-a7f5-fe86128ea84d:10.0.0.243:45366101-e425-40ec-97ae-890d5d09a36c"
keys["BS4"]="f88efb44-60d0-400e-881e-10de5f133011:10.0.0.34:76861cfa-1509-405c-a0f8-1abdb2a848dc"
keys["BC4"]="6c02e8e9-6499-47cb-8c32-0a0435ee485b:10.0.0.243:8c2f3c5b-ff5f-4f71-b53f-0a38ef35a03f"
keys["SS5"]="e4edd5d7-6a22-4244-a0ed-5296d6d47b1f:10.0.0.34:cc529996-b2ad-4153-af67-67d35370ab0a"
keys["SC5"]="c380fd6b-79a8-4681-a66d-5eeea928b794:10.0.0.243:da74815e-4a48-435f-9b16-45b494d1c9de"
keys["BS5"]="1ee80dc0-8f70-4e69-82c8-32feaae459d1:10.0.0.34:fb42e716-1d8b-4feb-9ba7-024979bb5dd3"
keys["BC5"]="076e9671-b7ab-4b4c-8934-2bcf332bf9bb:10.0.0.243:1afcd33d-747f-4d83-a466-8a34f53780ad"

B=$1
CLOUD="pi5u1"
SAT="pi5u2"
H_SINGLE="Host: $SAT-single.default.svc.cluster.local"
H_BUNDLE="Host: $SAT-bundled.default.svc.cluster.local"

HOST_DCLOUD="http://10.152.183.86/get-and-set?destination=$CLOUD&key="
HOST_DSAT="http://10.152.183.86/get-and-set?destination=$SAT&key="

echo "Storage Mechanism"
echo "State Size: 1MB"
echo "Bundle Size: $B"
echo "Sat Node: $SAT"
echo "Cloud Node: $CLOUD"
echo "Latency: 60ms"
echo -e "\n--------------------------------------------------------\n"
echo -e "Single-Stateful  Single-Stateless	Bundled-Stateful  Bundled-Stateless"
echo -e "Run,Tt,Tr,Tm,Tt,Tr,Tm,Tt,Tr,Tm,Tt,Tr,Tm"
for i in {0..100}; do
  # Single-Stateless
  ssl_tr=0
  ssl_tm=0
  START_SSL=$(date +%s%3N)
  for b in $(seq 1 "$B"); do
    read tr tm <<< $(curl -s "$HOST_DCLOUD${keys["SC$b"]}" -H "$H_SINGLE")
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
    read tr tm <<< $(curl -s "$HOST_DSAT${keys["SS$b"]}" -H "$H_SINGLE")
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

  echo -e "$i,$ssf_tt,$ssf_tr,$ssf_tm,$ssl_tt,$ssl_tr,$ssl_tm,$bsf_tt,$bsf_tr,$bsf_tm,$bsl_tt,$bsl_tr,$bsl_tm"
done

echo -e "\n--------------------------------------------------------\n"

