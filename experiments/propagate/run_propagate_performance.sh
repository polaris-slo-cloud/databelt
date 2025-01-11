#!/bin/bash

# Default values for optional arguments

REP=$1
SIZE=$2
OBJECTIVE=$3
POLICY=$4

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
keys["50MB"]="d7f34034-68b3-49a2-abc4-a349d430d63d:10.0.0.34:60b304ad-f1d5-4ee4-8026-1069f123f728"
keys["1MB"]="0a03715e-5457-432f-9df1-aaa18b34f153:10.0.0.34:74dd5f0f-06f2-4a21-97a2-ce0f3d757cd4"
#
# curl -v http://10.152.183.221/?policy=Skylark\&destination=pi5u1\&key=0a03715e-5457-432f-9df1-aaa18b34f153\:10.0.0.34\:74dd5f0f-06f2-4a21-97a2-ce0f3d757cd4 -H "Host: pi5u2-preprocess.default.svc.cluster.local"
# curl -v http://10.152.183.221/?policy=Skylark\&destination=pi5u1\&key=0a03715e-5457-432f-9df1-aaa18b34f153\:10.0.0.34\:74dd5f0f-06f2-4a21-97a2-ce0f3d757cd4 -H "Host: pi5u4-detect.default.svc.cluster.local"
# curl -v http://10.152.183.221/?policy=Skylark\&destination=pi5u1\&key=0a03715e-5457-432f-9df1-aaa18b34f153\:10.0.0.34\:74dd5f0f-06f2-4a21-97a2-ce0f3d757cd4 -H "Host: pi4u6-detect.default.svc.cluster.local"
#

FUNC1_URL="http://10.1.46.213:8080/?policy=$POLICY&destination=$DESTINATION&key=${keys["$SIZE"]}"
FUNC2_URL="http://10.1.52.145:8080/?policy=$POLICY&destination=$DESTINATION&key="
FUNC3_URL="http://10.1.48.17:8080/?policy=Stateless&destination=$DESTINATION&key="
FUNC4_URL="http://10.152.183.221/?policy=$POLICY&destination=$DESTINATION&key="

FUNC1_HEADER="Host: $N1-preprocess.default.svc.cluster.local"
FUNC2_HEADER="Host: $N2-detect.default.svc.cluster.local"
FUNC3_HEADER="Host: $N3-detect.default.svc.cluster.local"
FUNC4_HEADER="Host: $N4-alarm.default.svc.cluster.local"
echo "Workflow: {preprocess->detect->detect->alarm, $DESTINATION, $POLICY, $OBJECTIVE ms};"

echo "Scheduler: {$N1->$N2->$N3->$N4}"
echo -e "\n--------------------------------------------------------\n"
echo -e "Run,Tmax,Tw,T(dm1),T(dr2),T(dm2),LA2,SN2,EN2,T(dr3),T(dm3),LA3,SN3,EN3,T(dr4),LA4,SN4,EN4"
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

echo -e "\n--------------------------------------------------------\n"

