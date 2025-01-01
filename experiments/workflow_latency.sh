#!/bin/bash

# ./workflow_latency.sh Skylark 2M
# Default values for optional arguments
TEX="20"
POLICY=$1
DESTINATION="pi5u1"
OBJECTIVE="100"
IMG=$2 # 2K/1M/2M

debug=0

FUNC1_URL="http://10.152.183.86/?tex=$TEX&policy=$POLICY&destination=$DESTINATION&img=eo-$IMG.jpeg"
FUNC2_URL="http://10.152.183.86/?tex=$TEX&policy=$POLICY&destination=$DESTINATION&key="
FUNC3_URL="http://10.152.183.86/?tex=$TEX&policy=$POLICY&destination=$DESTINATION&key="
FUNC4_URL="http://10.152.183.86/?tex=$TEX&policy=$POLICY&destination=$DESTINATION&key="

N1="pi5u2"
N2="pi4u5"
N3="pi4p1"
N4="pi5u1"
SN2="58"
SN3="245"
SN4="243"
FUNC1_HEADER="Host: $N1-preprocess.default.svc.cluster.local"
FUNC2_HEADER="Host: $N2-detect.default.svc.cluster.local"
FUNC3_HEADER="Host: $N3-detect.default.svc.cluster.local"
FUNC4_HEADER="Host: $N4-alarm.default.svc.cluster.local"
echo "Workflow: {preprocess->detect->detect->alarm, $DESTINATION, $POLICY, $OBJECTIVE ms}"

# curl "http://10.152.183.86/?tex=20&policy=Skylark&destination=pi5u1&img=eo-1M.jpeg"  -H Host: "pi5u2-preprocess.default.svc.cluster.local"
echo "Scheduler $i: {$N1->$N2->$N3->$N4}"
echo -e "\n--------------------------------------------------------\n"
echo -e "Run,Tmax,Tw,T(dm1),T(dr2),T(dm2),SN2,EN2,T(dr3),T(dm3),SN3,EN3,T(dr4),SN4,EN4"
for i in {1..100}; do
    START_TIME=$(date +%s%3N)
    read key1 dm1 eip2 <<< $(curl -s "$FUNC1_URL" -H "$FUNC1_HEADER")
    read key2 dr2 dm2 eip3 <<< $(curl -s "$FUNC2_URL$key1" -H "$FUNC2_HEADER")
    read key3 dr3 dm3 eip4 <<< $(curl -s "$FUNC3_URL$key2" -H "$FUNC3_HEADER")
    read dr4 <<< $(curl -s "$FUNC4_URL$key3" -H "$FUNC4_HEADER")

    END_TIME=$(date +%s%3N)
    WORKFLOW_LATENCY=$((END_TIME - START_TIME))
    eip2="${eip2%\"}"
    eip3="${eip3%\"}"
    eip4="${eip4%\"}"
    EN2="${eip2##*.}"
    EN3="${eip3##*.}"
    EN4=${eip4##*.}
    echo -e "$i,$OBJECTIVE,$WORKFLOW_LATENCY,$dm1,$dr2,$dm2,$SN2,$EN2,$dr3,$dm3,$SN3,$EN3,$dr4,$SN4,$EN4"
done

echo -e "\n--------------------------------------------------------\n"

