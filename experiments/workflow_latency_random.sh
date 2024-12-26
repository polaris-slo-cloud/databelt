#!/bin/bash

FUNC1_URL="http://10.152.183.86/?policy=Random\&destination=pi5u1\&img=eo-$1.jpeg"
FUNC1_HEADER="Host: pi5u2-preprocess.default.svc.cluster.local"
FUNC2_URL="http://10.152.183.86/?policy=Random\&destination=pi5u1\&key="
FUNC2_HEADER="Host: pi5u4-detect.default.svc.cluster.local"
FUNC3_URL="http://10.152.183.86/?policy=Random\&destination=pi5u1\&key="
FUNC3_HEADER="Host: pi4u5-detect.default.svc.cluster.local"
FUNC4_URL="http://10.152.183.86/?policy=Random\&destination=pi5u1\&key="
FUNC4_HEADER="Host: pi5u1-alarm.default.svc.cluster.local"

START_TIME=$(date +%s%3N)

echo "Calling Function 1..."
RESPONSE1=$(curl -s "$FUNC1_URL" -H "$FUNC1_HEADER")
CHECKPOINT1=$(date +%s%3N)
echo "Response from Function 1: $RESPONSE1"

echo "Calling Function 2..."
RESPONSE2=$(curl -s "$FUNC2_URL$RESPONSE1" -H "$FUNC2_HEADER")
CHECKPOINT2=$(date +%s%3N)
echo "Response from Function 2: $RESPONSE2"

echo "Calling Function 3..."
RESPONSE3=$(curl -s "$FUNC3_URL$RESPONSE2" -H "$FUNC3_HEADER")
CHECKPOINT3=$(date +%s%3N)
echo "Response from Function 3: $RESPONSE3"

echo "Calling Function 4..."
RESPONSE4=$(curl -s "$FUNC4_URL$RESPONSE3" -H "$FUNC4_HEADER")
CHECKPOINT4=$(date +%s%3N)
echo "Response from Function 4: $RESPONSE4"



# End time
END_TIME=$(date +%s%3N)

# Calculate timings
TOTAL_TIME=$((END_TIME - START_TIME))
TIME1=$((CHECKPOINT1 - START_TIME))
TIME2=$((CHECKPOINT2 - CHECKPOINT1))
TIME3=$((CHECKPOINT3 - CHECKPOINT2))
TIME4=$((CHECKPOINT4 - CHECKPOINT3))

# Output timings
echo
echo "Timing Summary:"
echo "Function 1 execution time: ${TIME1}ms"
echo "Function 2 execution time: ${TIME2}ms"
echo "Function 3 execution time: ${TIME3}ms"
echo "Function 4 execution time: ${TIME4}ms"
echo "Total execution time: ${TOTAL_TIME}ms"
