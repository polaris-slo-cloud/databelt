#!/bin/bash

# ./setup_storage_mechanism.sh 500
# Default values for optional arguments

SIZE=$1
SAT="pi4u5"
CLOUD="pi5u1"
WD_HOST="http://10.0.0.243:8084"


SINGLE_SAT_URL="$WD_HOST/single?destination=$SAT&size=$SIZE"
SINGLE_CLOUD_URL="$WD_HOST/single?destination=$CLOUD&size=$SIZE"

BUNDLE_SAT_URL="$WD_HOST/bundled?destination=$SAT&size=$SIZE&scount="
BUNDLE_CLOUD_URL="$WD_HOST/bundled?destination=$CLOUD&size=$SIZE&scount="

# Store single keys
RESPONSE=$(curl -s "$SINGLE_SAT_URL")
echo "K_SINGLE_SAT_1=$RESPONSE"
RESPONSE=$(curl -s "$SINGLE_CLOUD_URL")
echo "K_SINGLE_CLOUD_1=$RESPONSE"

for i in {1..5}
do
  RESPONSE=$(curl -s "$BUNDLE_SAT_URL$i")
  echo "K_BUNDLE_SAT_$i=$RESPONSE"
  RESPONSE=$(curl -s "$BUNDLE_CLOUD_URL$i")
  echo "K_Bundle_CLOUD_$i=$RESPONSE"
done
