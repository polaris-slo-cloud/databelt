#!/bin/bash

# Usage: ./run_propagate_performance <repetition-count> <state-size> <t-max-SLO> <policy>
# Simulates a Serverless workflow with 4 chained functions.

REP=$1
SIZE=$2
OBJECTIVE=$3
POLICY=$4

# Scheduler config
DESTINATION="pi5u1"
N1="pi5u2"
N2="pi5u4"
N3="pi4u6"
N4="pi5u1"
SN2="167"
SN3="122"
SN4="243"


# SETUP
# OBJECTIVES
echo "NODE INFO OBJECTIVES"
curl http://10.0.0.34:8080/objectives -d "{\"min_bandwidth\": 60,\"max_latency\": $OBJECTIVE}" -H "Content-Type: application/json"
curl http://10.0.0.45:8080/objectives -d "{\"min_bandwidth\": 60,\"max_latency\": $OBJECTIVE}" -H "Content-Type: application/json"
curl http://10.0.0.167:8080/objectives -d "{\"min_bandwidth\": 60,\"max_latency\": $OBJECTIVE}" -H "Content-Type: application/json"
curl http://10.0.0.58:8080/objectives -d "{\"min_bandwidth\": 60,\"max_latency\": $OBJECTIVE}" -H "Content-Type: application/json"
curl http://10.0.0.243:8080/objectives -d "{\"min_bandwidth\": 60,\"max_latency\": $OBJECTIVE}" -H "Content-Type: application/json"
curl http://10.0.0.245:8080/objectives -d "{\"min_bandwidth\": 60,\"max_latency\": $OBJECTIVE}" -H "Content-Type: application/json"
curl http://10.0.0.210:8080/objectives -d "{\"min_bandwidth\": 60,\"max_latency\": $OBJECTIVE}" -H "Content-Type: application/json"
curl http://10.0.0.122:8080/objectives -d "{\"min_bandwidth\": 60,\"max_latency\": $OBJECTIVE}" -H "Content-Type: application/json"

# REFRESH SKYLARK ELECT
echo "REFRESH SKYLARK"
curl http://10.0.0.34:8081/refresh
curl http://10.0.0.45:8081/refresh
curl http://10.0.0.167:8081/refresh
curl http://10.0.0.122:8081/refresh
curl http://10.0.0.210:8081/refresh
curl http://10.0.0.245:8081/refresh
curl http://10.0.0.243:8081/refresh
curl http://10.0.0.58:8081/refresh

# CLEAR REDIS
clear_redis() {
    echo "CLEAR REDIS"
    redis-cli -h pi5u1 -p 6379 FLUSHALL
    redis-cli -h pi5u3 -p 6379 FLUSHALL
    redis-cli -h pi5u4 -p 6379 FLUSHALL
    redis-cli -h pi4u5 -p 6379 FLUSHALL
    redis-cli -h pi4u6 -p 6379 FLUSHALL
    redis-cli -h pi4u8 -p 6379 FLUSHALL
    redis-cli -h pi4p1 -p 6379 FLUSHALL
}
#Generate Input State
# echo "Generating state.."
# KEY=$(curl -s "http://10.0.0.243:8084/single?destination=$N1&size=$SIZE")
declare -A keys
keys["1MB"]="1c43555e-dd95-404a-bb61-b23cea9375fe:10.0.0.34:687a1305-5fa3-49d3-bc22-a70fba690e61"
keys["5MB"]="94133329-8bdc-4bb5-82c2-b77795ed5d5c:10.0.0.34:30381f01-54b8-45b8-bed1-82ee314aa4a6"
keys["10MB"]="bdc5a692-cd93-49e0-a3d2-30be88768896:10.0.0.34:40f39e12-ca9c-4cfb-8977-321a53750a81"
keys["15MB"]="c1224c3f-eff4-406b-9a4b-52e4fd08fe09:10.0.0.34:0b6ed264-02c6-4b7b-9971-21eee1073d05"
keys["20MB"]="68901e46-f0ab-4b73-ae08-812db581ef09:10.0.0.34:ff6c0067-cea1-4e02-8716-49c0f76409fd"
keys["25MB"]="7d7fe66f-fe84-4f71-accd-2df98578620e:10.0.0.34:344f489c-4d9a-45c4-b361-73be2423aaf5"
keys["30MB"]="73890c19-a658-487f-928c-00816e659c60:10.0.0.34:c4249c6a-e4b7-47d4-95f9-61c90630e89b"
keys["35MB"]="6c903e99-219e-4f72-9d9c-b3bf7bb849b1:10.0.0.34:cd05d34b-47ce-42fe-8f74-2379bacba34f"
keys["40MB"]="9ebc191c-fd18-4372-bd6d-6827330a52b2:10.0.0.34:e4c31fae-02eb-4ce0-a5e2-9f78b289139b"
keys["45MB"]="cec3edb5-535c-46c0-b0cc-105c5dcf9e40:10.0.0.34:6a79ab0d-59cc-45a9-a753-51a8b6f3d729"
keys["50MB"]="b929da93-8b49-4a8c-b874-4d16f01e0d2f:10.0.0.34:762de052-e01f-499d-9e31-e165989371fb"

FUNC1_URL="http://10.152.183.251/?policy=$POLICY&destination=$DESTINATION&key=${keys["$SIZE"]}"
FUNC2_URL="http://10.152.183.251/?policy=$POLICY&destination=$DESTINATION&key="
FUNC3_URL="http://10.152.183.251/?policy=Stateless&destination=$DESTINATION&key="
FUNC4_URL="http://10.152.183.251/?policy=$POLICY&destination=$DESTINATION&key="

FUNC1_HEADER="Host: $N1-preprocess.default.svc.cluster.local"
FUNC2_HEADER="Host: $N2-detect.default.svc.cluster.local"
FUNC3_HEADER="Host: $N3-detect.default.svc.cluster.local"
FUNC4_HEADER="Host: $N4-alarm.default.svc.cluster.local"
echo "Workflow: {preprocess->detect->detect->alarm, $DESTINATION, $POLICY, $OBJECTIVE ms};"

echo "Scheduler: {$N1->$N2->$N3->$N4}"
echo -e "\n--------------------------------------------------------\n"
EX_START_TIME=$(date +%s%3N)
echo "EX START: $((EX_START_TIME / 1000))"
echo "$SIZE, $POLICY, $OBJECTIVE" >> propagate_performance_$POLICY.log
for i in $(seq 0 "$REP"); do
    clear_redis
    START_TIME=$(date +%s%3N)
    read key1 dm1 dr1 eip2 <<< $(curl -s "$FUNC1_URL" -H "$FUNC1_HEADER")
    read key2 dr2 dm2 eip3 <<< $(curl -s "$FUNC2_URL$key1" -H "$FUNC2_HEADER")
    read key3 dr3 dm3 eip4 <<< $(curl -s "$FUNC3_URL$key2" -H "$FUNC3_HEADER")
    read dr4 <<< $(curl -s "$FUNC4_URL$key3" -H "$FUNC4_HEADER")

    END_TIME=$(date +%s%3N)
    WORKFLOW_LATENCY=$((END_TIME - START_TIME))
    EN2="${eip2##*.}"
    EN3="${eip3##*.}"
    EN4="${eip4##*.}"
    echo -e "$i,$OBJECTIVE,$WORKFLOW_LATENCY,$dm1,$dr2,$dm2,l,$SN2,$EN2,$dr3,$dm3,l,$SN3,$EN3,$dr4,l,$SN4,$EN4" >> propagate_performance_$POLICY.log
done
EX_END_TIME=$(date +%s%3N)
echo "EX END: $((EX_END_TIME / 1000))"
echo "EX RUNTIME: $(((EX_END_TIME - EX_START_TIME) / 1000))"
echo "$((EX_START_TIME / 1000)),$((EX_END_TIME / 1000)),$(((EX_END_TIME - EX_START_TIME) / 1000))" >> propagate_performance_$POLICY.log
echo -e "\n--------------------------------------------------------\n"

