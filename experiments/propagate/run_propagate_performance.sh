#!/bin/bash


#
#
#
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
echo "CLEAR REDIS"
redis-cli -h pi5u1 -p 6379 FLUSHALL
redis-cli -h pi5u3 -p 6379 FLUSHALL
redis-cli -h pi5u4 -p 6379 FLUSHALL
redis-cli -h pi4u5 -p 6379 FLUSHALL
redis-cli -h pi4u6 -p 6379 FLUSHALL
redis-cli -h pi4u8 -p 6379 FLUSHALL
redis-cli -h pi4p1 -p 6379 FLUSHALL

#Generate Input State
# echo "Generating state.."
# KEY=$(curl -s "http://10.0.0.243:8084/single?destination=$N1&size=$SIZE")
declare -A keys
keys["1MB"]="81152a2c-192f-4a94-ab2b-279a0aa24488:10.0.0.34:49b63bb0-70bd-4473-ad2e-fd7feb78ab91"
keys["5MB"]="4e1f0ded-37d3-4723-a7a8-99671bcbe193:10.0.0.34:20325523-5a1d-458e-960b-b3ef156f07e6"
keys["10MB"]="774d8ce6-8c97-4569-b37f-be3607133335:10.0.0.34:a7f6851a-d63d-4654-88fc-5faecc5032e0"
keys["15MB"]="c00eee85-215f-4e76-beb9-d45ba44dc615:10.0.0.34:0b45a546-5a59-4f80-b0e2-aadcbc60d1d8"
keys["20MB"]="0b96ca6e-2fdb-48d1-853d-b98aef842f4e:10.0.0.34:003272f6-85f5-481f-9076-687f2a1b6bec"
keys["25MB"]="4420d65c-21b9-41e8-ad64-653cc9c5e39a:10.0.0.34:3dd51e19-9271-417e-84a7-12ba3e26f498"
keys["30MB"]="ff526774-58a3-4cb9-a92b-d95ab9f5068f:10.0.0.34:ca03b24d-fb69-42a8-9785-9a1db378b6f3"
keys["35MB"]="a40c2c97-d141-4e91-a832-410e5d729305:10.0.0.34:a2fe0ab1-01ee-4f35-9309-e52cc1038071"
keys["40MB"]="d8e46e34-e1cd-4da0-820e-e509e046913d:10.0.0.34:1027c80e-c2b7-4d44-b2c6-5da57fce446f"
keys["45MB"]="9458be4c-cca0-4abc-9262-4e45b4373c59:10.0.0.34:665ef8be-e05a-44f8-91d6-21e60bb352b4"
keys["50MB"]="89a73942-81d8-4b19-8a52-87459efdc2f0:10.0.0.34:2ec75e6a-541f-4c8c-a081-20b6d36b2a27"

FUNC1_URL="http://10.152.183.86/?policy=$POLICY&destination=$DESTINATION&key=${keys["$SIZE"]}"
FUNC2_URL="http://10.152.183.86/?policy=$POLICY&destination=$DESTINATION&key="
FUNC3_URL="http://10.152.183.86/?policy=Stateless&destination=$DESTINATION&key="
FUNC4_URL="http://10.152.183.86/?policy=$POLICY&destination=$DESTINATION&key="

FUNC1_HEADER="Host: $N1-preprocess.default.svc.cluster.local"
FUNC2_HEADER="Host: $N2-detect.default.svc.cluster.local"
FUNC3_HEADER="Host: $N3-detect.default.svc.cluster.local"
FUNC4_HEADER="Host: $N4-alarm.default.svc.cluster.local"
echo "Workflow: {preprocess->detect->detect->alarm, $DESTINATION, $POLICY, $OBJECTIVE ms};"

echo "Scheduler: {$N1->$N2->$N3->$N4}"
echo -e "\n--------------------------------------------------------\n"
echo -e "Run,Tmax,Tw,T(dm1),T(dr2),T(dm2),LA2,SN2,EN2,T(dr3),T(dm3),LA3,SN3,EN3,T(dr4),LA4,SN4,EN4"
echo "$SIZE, $POLICY, $OBJECTIVE" >> propagate_performance.log
for i in $(seq 0 "$REP"); do
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
    echo -e "$i,$OBJECTIVE,$WORKFLOW_LATENCY,$dm1,$dr2,$dm2,l,$SN2,$EN2,$dr3,$dm3,l,$SN3,$EN3,$dr4,l,$SN4,$EN4" >> propagate_performance.log
done

echo -e "\n--------------------------------------------------------\n"

