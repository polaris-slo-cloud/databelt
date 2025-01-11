#!/bin/bash

# ./setup_input_state_size.sh pi5u2 4500 45MB; ./setup_input_state_size.sh pi5u2 40000 40MB; ./setup_input_state_size.sh pi5u2 35000 35MB; ./setup_input_state_size.sh pi5u2 50000 50MB; ./setup_input_state_size.sh pi5u2 30000 30MB
#
# Default values for optional arguments
#

DEST=$1
SIZE=$2
TEXT=$3
echo -e "Generating single keys http://10.0.0.34:8084/single?destination=$DEST&size=$SIZE"

RESPONSE=$(curl -s "http://10.0.0.34:8084/single?destination=$DEST&size=$SIZE")
echo "keys[\"$TEXT\"]=\"$RESPONSE\""