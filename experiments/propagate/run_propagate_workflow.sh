#!/bin/bash

# ./run_propagate_workflow.sh Skylark KEY; ./workflow.sh Stateless KEY; ./workflow.sh Random KEY
#
# Default values for optional arguments
POLICY=$1
DESTINATION="pi5u1"
OBJECTIVE="100"
KEY=$2

debug=0
#echo "KEY: $KEY"
#echo "POLICY: $POLICY"
FUNC1_URL="http://10.152.183.251/?policy=$POLICY&destination=$DESTINATION&key="
FUNC2_URL="http://10.152.183.251/?policy=$POLICY&destination=$DESTINATION&key="
FUNC3_URL="http://10.152.183.251/?policy=Stateless&destination=$DESTINATION&key="
FUNC4_URL="http://10.152.183.251/?policy=$POLICY&destination=$DESTINATION&key="

N1="pi5u2"
N2="pi5u4"
N3="pi4u6"
N4="pi5u1"
SN2="167"
SN3="122"
SN4="243"
FUNC1_HEADER="Host: $N1-preprocess.default.svc.cluster.local"
FUNC2_HEADER="Host: $N2-detect.default.svc.cluster.local"
FUNC3_HEADER="Host: $N3-detect.default.svc.cluster.local"
FUNC4_HEADER="Host: $N4-alarm.default.svc.cluster.local"
# echo "Workflow: {preprocess->detect->detect->alarm, $DESTINATION, $POLICY, $OBJECTIVE ms};"
# echo "Scheduler: {$N1->$N2->$N3->$N4}"
# echo -e "Run,Tmax,Tw,T(dm1),T(dr2),T(dm2),LA2,SN2,EN2,T(dr3),T(dm3),LA3,SN3,EN3,T(dr4),LA4,SN4,EN4"
START_TIME=$(date +%s%3N)
read key1 dm1 dr1 eip2 <<< $(curl -s "$FUNC1_URL$KEY" -H "$FUNC1_HEADER")
read key2 dr2 dm2 eip3 <<< $(curl -s "$FUNC2_URL$key1" -H "$FUNC2_HEADER")
read key3 dr3 dm3 eip4 <<< $(curl -s "$FUNC3_URL$key2" -H "$FUNC3_HEADER")
read dr4 <<< $(curl -s "$FUNC4_URL$key3" -H "$FUNC4_HEADER")

END_TIME=$(date +%s%3N)
WORKFLOW_LATENCY=$((END_TIME - START_TIME))
EN2="${eip2##*.}"
EN3="${eip3##*.}"
EN4="${eip4##*.}"
# echo -e "$i,$OBJECTIVE,$WORKFLOW_LATENCY,$dm1,$dr2,$dm2,l,$SN2,$EN2,$dr3,$dm3,l,$SN3,$EN3,$dr4,l,$SN4,$EN4"

