#!/bin/bash

# ./setup_input_state.sh
# Default values for optional arguments
#

DEST=$1
CLOUD="pi5u1"
HOST="http://10.0.0.243:8084/single?destination=$DEST&size=$SIZE"
echo -e "Generating single keys"

RESPONSE=$(curl -s "http://10.0.0.243:8084/single?destination=$DEST&size=1000")
echo "keys[\"1MB\"]=\"$RESPONSE\""

RESPONSE=$(curl -s "http://10.0.0.243:8084/single?destination=$DEST&size=5000")
  echo "keys[\"5MB\"]=\"$RESPONSE\""

RESPONSE=$(curl -s "http://10.0.0.243:8084/single?destination=$DEST&size=10000")
  echo "keys[\"10MB\"]=\"$RESPONSE\""

RESPONSE=$(curl -s "http://10.0.0.243:8084/single?destination=$DEST&size=15000")
  echo "keys[\"15MB\"]=\"$RESPONSE\""

RESPONSE=$(curl -s "http://10.0.0.243:8084/single?destination=$DEST&size=20000")
  echo "keys[\"20MB\"]=\"$RESPONSE\""

RESPONSE=$(curl -s "http://10.0.0.243:8084/single?destination=$DEST&size=25000")
  echo "keys[\"25MB\"]=\"$RESPONSE\""

RESPONSE=$(curl -s "http://10.0.0.243:8084/single?destination=$DEST&size=30000")
  echo "keys[\"30MB\"]=\"$RESPONSE\""

RESPONSE=$(curl -s "http://10.0.0.243:8084/single?destination=$DEST&size=35000")
  echo "keys[\"35MB\"]=\"$RESPONSE\""

RESPONSE=$(curl -s "http://10.0.0.243:8084/single?destination=$DEST&size=40000")
  echo "keys[\"40MB\"]=\"$RESPONSE\""

RESPONSE=$(curl -s "http://10.0.0.243:8084/single?destination=$DEST&size=45000")
  echo "keys[\"45MB\"]=\"$RESPONSE\""

RESPONSE=$(curl -s "http://10.0.0.243:8084/single?destination=$DEST&size=50000")
  echo "keys[\"50MB\"]=\"$RESPONSE\""
