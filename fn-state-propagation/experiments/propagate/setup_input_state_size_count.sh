#!/bin/bash

# ./setup_input_state_size_count.sh pi5u2 2000 50
# Default values for optional arguments
#

DEST=$1
SIZE=$2
COUNT=$3
echo -e "Generating $COUNT single keys http://10.0.0.34:8084/single?destination=$DEST&size=$SIZE"
for i in $(seq 0 "$COUNT"); do
  RESPONSE=$(curl -s "http://10.0.0.34:8084/single?destination=$DEST&size=$SIZE")
  echo "$RESPONSE"
done