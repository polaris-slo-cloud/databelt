#!/bin/bash

# pi5u2 -> pi4u6 -> pi4p1 -> pi5u1
# Default values for optional arguments
TEX="20"
POLICY="Skylark"
DESTINATION="pi5u1"
IMG="1M" # 2K/1M/2M

N1="pi5u2"
N2="pi4u6"
N3="pi4p1"
N4="pi5u1"

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

# curl "http://10.152.183.86/?tex=20&policy=Skylark&destination=pi5u1&img=eo-1M.jpeg"  -H Host: "pi5u2-preprocess.default.svc.cluster.local"
START_TIME=$(date +%s%3N)
echo "$FUNC1_URL  -H $FUNC1_HEADER"
RESPONSE1=$(curl -s "$FUNC1_URL" -H "$FUNC1_HEADER")
echo "$RESPONSE1"
read key2 dr2 dm2 <<< $(curl -s "$FUNC2_URL$key1" -H "$FUNC2_HEADER")
read key3 dr3 dm3 <<< $(curl -s "$FUNC3_URL$key2" -H "$FUNC3_HEADER")
read dr4 <<< $(curl -s "$FUNC4_URL$key3" -H "$FUNC4_HEADER")

# End time
END_TIME=$(date +%s%3N)

# Calculate timings
WORKFLOW_LATENCY=$((END_TIME - START_TIME))
echo -e "\n--------------------------------------------------------\n"
for i in {1..100}; do

    START_TIME=$(date +%s%3N)
    read key1 dm1 <<< $(curl -s "$FUNC1_URL" -H "$FUNC1_HEADER")
    echo "$key1"
    echo "$dm1"
    read key2 dr2 dm2 <<< $(curl -s "$FUNC2_URL$key1" -H "$FUNC2_HEADER")
    read key3 dr3 dm3 <<< $(curl -s "$FUNC3_URL$key2" -H "$FUNC3_HEADER")
    read dr4 <<< $(curl -s "$FUNC4_URL$key3" -H "$FUNC4_HEADER")

    # End time
    END_TIME=$(date +%s%3N)

    # Calculate timings
    WORKFLOW_LATENCY=$((END_TIME - START_TIME))
    echo "$i\t$WORKFLOW_LATENCY\t$dm1\t$dr2\t$dm2\t$dr3\t$dm3\t$dr4"
done

echo -e "\n--------------------------------------------------------\n"

