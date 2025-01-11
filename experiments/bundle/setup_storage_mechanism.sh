#!/bin/bash

# ./setup_storage_mechanism.sh 10000
# Default values for optional arguments
#

SIZE=$1
SAT="pi5u2"
CLOUD="pi5u1"
WD_HOST="http://10.0.0.34:8084"


SINGLE_SAT_URL="$WD_HOST/single?destination=$SAT&size=$SIZE"
SINGLE_CLOUD_URL="$WD_HOST/single?destination=$CLOUD&size=$SIZE"

BUNDLE_SAT_URL="$WD_HOST/bundled?destination=$SAT&size=$SIZE&scount="
BUNDLE_CLOUD_URL="$WD_HOST/bundled?destination=$CLOUD&size=$SIZE&scount="

RESPONSE=$(curl -s "$SINGLE_SAT_URL")
echo "keys[\"SS1\"]=\"$RESPONSE\""
RESPONSE=$(curl -s "$SINGLE_CLOUD_URL")
echo "keys[\"SC1\"]=\"$RESPONSE\""

for i in {1..5}
do
  RESPONSE=$(curl -s "$BUNDLE_SAT_URL$i")
  echo "keys[\"BS$i\"]=\"$RESPONSE\""
  RESPONSE=$(curl -s "$BUNDLE_CLOUD_URL$i")
  echo "keys[\"BC$i\"]=\"$RESPONSE\""
done
