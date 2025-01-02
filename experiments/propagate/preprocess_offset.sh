#!/bin/bash

# ./preprocess_offset.sh Skylark 2K; ./preprocess_offset.sh Skylark 1M; ./preprocess_offset.sh Skylark 2M;
#
# Default values for optional arguments
TEX="20"
POLICY=$1
DESTINATION="pi5u1"
debug=0
IMG=$2
FUNC1_URL="http://10.152.183.86/?tex=$TEX&policy=$POLICY&destination=$DESTINATION&img=eo-$IMG.jpeg"
FUNC1_HEADER="Host: pi5u2-preprocess.default.svc.cluster.local"

# curl "http://10.152.183.86/?tex=20&policy=Skylark&destination=pi5u1&img=eo-1M.jpeg"  -H Host: "pi5u2-preprocess.default.svc.cluster.local"
echo "Preprocess image loading offset"
echo "Init Run"
START_TIME=$(date +%s%3N)
read key1 dm1 eip2 tf <<< $(curl -s "$FUNC1_URL" -H "$FUNC1_HEADER")

END_TIME=$(date +%s%3N)
WORKFLOW_LATENCY=$((END_TIME - START_TIME))
echo -e "0,$WORKFLOW_LATENCY,$tf,$dm1"
echo -e "\n--------------------------------------------------------\n"
echo -e "Run,Tw,Tf,T(dm)"
for i in {1..30}; do
    START_TIME=$(date +%s%3N)
    read key1 dm1 tf eip2 <<< $(curl -s "$FUNC1_URL" -H "$FUNC1_HEADER")

    END_TIME=$(date +%s%3N)
    WORKFLOW_LATENCY=$((END_TIME - START_TIME))
    echo -e "$i,$WORKFLOW_LATENCY,$tf,$dm1"
done

echo -e "\n--------------------------------------------------------\n"

