#!/bin/bash

# ./storage_mechanism.sh pi5u2 pi4u6 pi4p1 pi5u1 --tex 30 --img 1M --policy Stateless
# ./workflow_latency.sh pi5u2 pi4u6 pi4p1 pi5u1 --tex 30 --img 1M --policy Skylark
# ./workflow_latency.sh pi5u2 pi4u6 pi4p1 pi5u1 --tex 30 --img 1M --policy Random
# ./workflow_latency.sh pi5u2 pi4u6 pi4p1 pi5u1 --tex 30 --img 2M --policy Stateless
# ./workflow_latency.sh pi5u2 pi4u6 pi4p1 pi5u1 --tex 30 --img 2M --policy Skylark
# ./workflow_latency.sh pi5u2 pi4u6 pi4p1 pi5u1 --tex 30 --img 2M --policy Random
# ./workflow_latency.sh pi5u2 pi4u6 pi4p1 pi5u1 --tex 30 --img 2K --policy Stateless
# ./workflow_latency.sh pi5u2 pi4u6 pi4p1 pi5u1 --tex 30 --img 2K --policy Skylark
# ./workflow_latency.sh pi5u2 pi4u6 pi4p1 pi5u1 --tex 30 --img 2K --policy Random
# pi5u2 -> pi4u6 -> pi4p1 -> pi5u1
# Default values for optional arguments
TEX="20"
POLICY="Skylark"
DESTINATION="pi5u1"
IMG="2K" # 2K/1M/2M

N1=$1
N2=$2
N3=$3
N4=$4

# Parse named arguments
while [[ $# -gt 0 ]]; do
    case "$1" in
        --tex)
            TEX="$2"
            shift 2
            ;;
        --policy)
            POLICY="$2"
            shift 2
            ;;
        --dest)
            DESTINATION="$2"
            shift 2
            ;;
        --img)
            IMG="$2"
            shift 2
            ;;
        --help)
            echo "Usage: $0 --tex TIME_IN_MS --img 2K|1M|2M --policy Skylark|Stateless|Random --dest <node-name>"
            exit 0
            ;;
        *)
            UNNAMED_ARGS+=("$1")
            shift
            ;;
    esac
done

FUNC1_URL="http://10.152.183.86/?tex=$TEX&policy=$POLICY&destination=$DESTINATION&img=eo-$IMG.jpeg"
FUNC1_HEADER="Host: $N1-preprocess.default.svc.cluster.local"
FUNC2_URL="http://10.152.183.86/?tex=$TEX&policy=$POLICY&destination=$DESTINATION&key="
FUNC2_HEADER="Host: $N2-detect.default.svc.cluster.local"
FUNC3_URL="http://10.152.183.86/?tex=$TEX&policy=$POLICY&destination=$DESTINATION&key="
FUNC3_HEADER="Host: $N3-detect.default.svc.cluster.local"
FUNC4_URL="http://10.152.183.86/?tex=$TEX&policy=$POLICY&destination=$DESTINATION&key="
FUNC4_HEADER="Host: $N4-alarm.default.svc.cluster.local"

echo "Workflow: {preprocess->detect->detect->alarm, $DESTINATION, $POLICY, $TEX ms}"
echo "Scheduler: {$N1->$N2->$N3->$N4}"
START_TIME=$(date +%s%3N)

echo "Calling Function 1 on $N1..."
RESPONSE1=$(curl -s "$FUNC1_URL" -H "$FUNC1_HEADER")
CHECKPOINT1=$(date +%s%3N)
echo "Response from Function 1: $RESPONSE1"

echo "Calling Function 2 on $N2..."
RESPONSE2=$(curl -s "$FUNC2_URL$RESPONSE1" -H "$FUNC2_HEADER")
CHECKPOINT2=$(date +%s%3N)
echo "Response from Function 2: $RESPONSE2"

echo "Calling Function 3 on $N3..."
RESPONSE3=$(curl -s "$FUNC3_URL$RESPONSE2" -H "$FUNC3_HEADER")
CHECKPOINT3=$(date +%s%3N)
echo "Response from Function 3: $RESPONSE3"

echo "Calling Function 4 on $N4..."
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
echo "Function 1 execution time: ${TIME1}"
echo "Function 2 execution time: ${TIME2}"
echo "Function 3 execution time: ${TIME3}"
echo "Function 4 execution time: ${TIME4}"
echo "Total execution time: ${TOTAL_TIME}"
