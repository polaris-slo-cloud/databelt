#!/bin/bash

# Measuring overhead
DEST="pi5u1"
SAT="pi4u5"
H_SINGLE="Host: $SAT-single.default.svc.cluster.local"
H_BUNDLE="Host: $SAT-bundled.default.svc.cluster.local"

HOST="http://10.152.183.86/get-and-set?destination=$DEST&key="

# Keys of states with size 1MB
K_SINGLE_CLOUD_1=29553585-ae34-4849-9350-1a85c704efba:10.0.0.243:f7a980a6-60d0-4e48-93ec-81c51d3223ee
K_BUNDLE_CLOUD_3=ff8024e1-d033-4722-809f-15e1fd143e86:10.0.0.243:3bf1e9c7-370d-40da-bef7-041705378036

ssf_sumdrs=()
ssf_sumdms=()
bsf_sumdrs=()
bsf_sumdms=()

echo "Storage Mechanism 3 HOP"
echo "State Size: 1MB"
echo "Bundle Size: 3"
echo "Execution Node: $SAT"
echo "Destination Node: $CLOUD"
echo "Latency: 2 5ms"
echo "Bandwidth: 60mbit"
echo "$HOST$K_SINGLE_CLOUD_1 -H $H_SINGLE"
echo "$HOST$K_BUNDLE_CLOUD_3 -H $H_BUNDLE"

echo -e "\n--------------------------------------------------------\n"
for i in {1..100}; do
    # Single
    read dr1 dm1 <<< $(curl -s "$HOST$K_SINGLE_CLOUD_1" -H "$H_SINGLE")
    read dr2 dm2 <<< $(curl -s "$HOST$K_SINGLE_CLOUD_1" -H "$H_SINGLE")
    read dr3 dm3 <<< $(curl -s "$HOST$K_SINGLE_CLOUD_1" -H "$H_SINGLE")
    ssf_sumdr=$((dr1 + dr2 + dr3))
    ssf_sumdm=$((dm1 + dm2 + dm3))
     # Bundle
     read bsf_dr bsf_dm <<< $(curl -s "$HOST$K_BUNDLE_CLOUD_3" -H "$H_BUNDLE")
    echo -e "$i\t$ssf_sumdr\t$ssf_sumdm\t$bsf_dr\t$bsf_dm"
done

echo -e "\n--------------------------------------------------------\n"

sudo tc qdisc del dev wlan0 root
sudo tc qdisc add dev wlan0 root handle 1: htb default 1
sudo tc class add dev wlan0 parent 1: classid 1:1 htb rate 1000mbit ceil 10000mbit
sudo tc class add dev wlan0 parent 1: classid 1:2 htb rate 70mbit ceil 80mbit
sudo tc qdisc add dev wlan0 parent 1:2 handle 20: netem delay 30ms 27ms
sudo tc filter del dev wlan0 parent 1:0
sudo tc filter add dev wlan0 protocol ip parent 1:0 prio 2 u32 match ip dst 10.0.0.243 flowid 1:2

echo "Storage Mechanism 4 HOP"
echo "Latency: 34 ms"
echo "Bandwidth: 60mbit"
echo -e "\n--------------------------------------------------------\n"
for i in {1..100}; do
    # Single
    read dr1 dm1 <<< $(curl -s "$HOST$K_SINGLE_CLOUD_1" -H "$H_SINGLE")
    read dr2 dm2 <<< $(curl -s "$HOST$K_SINGLE_CLOUD_1" -H "$H_SINGLE")
    read dr3 dm3 <<< $(curl -s "$HOST$K_SINGLE_CLOUD_1" -H "$H_SINGLE")
    ssf_sumdr=$((dr1 + dr2 + dr3))
    ssf_sumdm=$((dm1 + dm2 + dm3))
     # Bundle
     read bsf_dr bsf_dm <<< $(curl -s "$HOST$K_BUNDLE_CLOUD_3" -H "$H_BUNDLE")
    echo -e "$i\t$ssf_sumdr\t$ssf_sumdm\t$bsf_dr\t$bsf_dm"
done

echo -e "\n--------------------------------------------------------\n"

sudo tc qdisc del dev wlan0 root
sudo tc qdisc add dev wlan0 root handle 1: htb default 1
sudo tc class add dev wlan0 parent 1: classid 1:1 htb rate 1000mbit ceil 10000mbit
sudo tc class add dev wlan0 parent 1: classid 1:2 htb rate 70mbit ceil 80mbit
sudo tc qdisc add dev wlan0 parent 1:2 handle 20: netem delay 40ms 36ms
sudo tc filter del dev wlan0 parent 1:0
sudo tc filter add dev wlan0 protocol ip parent 1:0 prio 2 u32 match ip dst 10.0.0.243 flowid 1:2

echo "Storage Mechanism 5 HOP"
echo "Latency: 43 ms"
echo "Bandwidth: 60mbit"
echo -e "\n--------------------------------------------------------\n"
for i in {1..100}; do
    # Single
    read dr1 dm1 <<< $(curl -s "$HOST$K_SINGLE_CLOUD_1" -H "$H_SINGLE")
    read dr2 dm2 <<< $(curl -s "$HOST$K_SINGLE_CLOUD_1" -H "$H_SINGLE")
    read dr3 dm3 <<< $(curl -s "$HOST$K_SINGLE_CLOUD_1" -H "$H_SINGLE")
    ssf_sumdr=$((dr1 + dr2 + dr3))
    ssf_sumdm=$((dm1 + dm2 + dm3))
     # Bundle
     read bsf_dr bsf_dm <<< $(curl -s "$HOST$K_BUNDLE_CLOUD_3" -H "$H_BUNDLE")
    echo -e "$i\t$ssf_sumdr\t$ssf_sumdm\t$bsf_dr\t$bsf_dm"
done

echo -e "\n--------------------------------------------------------\n"

sudo tc qdisc del dev wlan0 root
sudo tc qdisc add dev wlan0 root handle 1: htb default 1
sudo tc class add dev wlan0 parent 1: classid 1:1 htb rate 1000mbit ceil 10000mbit
sudo tc class add dev wlan0 parent 1: classid 1:2 htb rate 70mbit ceil 80mbit
sudo tc qdisc add dev wlan0 parent 1:2 handle 20: netem delay 50ms 45ms
sudo tc filter del dev wlan0 parent 1:0
sudo tc filter add dev wlan0 protocol ip parent 1:0 prio 2 u32 match ip dst 10.0.0.243 flowid 1:2

echo "Storage Mechanism 6 HOP"
echo "Latency: 52 ms"
echo "Bandwidth: 60mbit"
echo -e "\n--------------------------------------------------------\n"
for i in {1..100}; do
    # Single
    read dr1 dm1 <<< $(curl -s "$HOST$K_SINGLE_CLOUD_1" -H "$H_SINGLE")
    read dr2 dm2 <<< $(curl -s "$HOST$K_SINGLE_CLOUD_1" -H "$H_SINGLE")
    read dr3 dm3 <<< $(curl -s "$HOST$K_SINGLE_CLOUD_1" -H "$H_SINGLE")
    ssf_sumdr=$((dr1 + dr2 + dr3))
    ssf_sumdm=$((dm1 + dm2 + dm3))
     # Bundle
     read bsf_dr bsf_dm <<< $(curl -s "$HOST$K_BUNDLE_CLOUD_3" -H "$H_BUNDLE")
    echo -e "$i\t$ssf_sumdr\t$ssf_sumdm\t$bsf_dr\t$bsf_dm"
done

echo -e "\n--------------------------------------------------------\n"

sudo tc qdisc del dev wlan0 root
sudo tc qdisc add dev wlan0 root handle 1: htb default 1
sudo tc class add dev wlan0 parent 1: classid 1:1 htb rate 1000mbit ceil 10000mbit
sudo tc class add dev wlan0 parent 1: classid 1:2 htb rate 70mbit ceil 80mbit
sudo tc qdisc add dev wlan0 parent 1:2 handle 20: netem delay 60ms 54ms
sudo tc filter del dev wlan0 parent 1:0
sudo tc filter add dev wlan0 protocol ip parent 1:0 prio 2 u32 match ip dst 10.0.0.243 flowid 1:2

echo "Storage Mechanism 7 HOP"
echo "Latency: 61 ms"
echo "Bandwidth: 60mbit"
echo -e "\n--------------------------------------------------------\n"
for i in {1..100}; do
    # Single
    read dr1 dm1 <<< $(curl -s "$HOST$K_SINGLE_CLOUD_1" -H "$H_SINGLE")
    read dr2 dm2 <<< $(curl -s "$HOST$K_SINGLE_CLOUD_1" -H "$H_SINGLE")
    read dr3 dm3 <<< $(curl -s "$HOST$K_SINGLE_CLOUD_1" -H "$H_SINGLE")
    ssf_sumdr=$((dr1 + dr2 + dr3))
    ssf_sumdm=$((dm1 + dm2 + dm3))
     # Bundle
     read bsf_dr bsf_dm <<< $(curl -s "$HOST$K_BUNDLE_CLOUD_3" -H "$H_BUNDLE")
    echo -e "$i\t$ssf_sumdr\t$ssf_sumdm\t$bsf_dr\t$bsf_dm"
done

echo -e "\n--------------------------------------------------------\n"

sudo tc qdisc del dev wlan0 root
sudo tc qdisc add dev wlan0 root handle 1: htb default 1
sudo tc class add dev wlan0 parent 1: classid 1:1 htb rate 1000mbit ceil 10000mbit
sudo tc class add dev wlan0 parent 1: classid 1:2 htb rate 70mbit ceil 80mbit
sudo tc qdisc add dev wlan0 parent 1:2 handle 20: netem delay 70ms 63ms
sudo tc filter del dev wlan0 parent 1:0
sudo tc filter add dev wlan0 protocol ip parent 1:0 prio 2 u32 match ip dst 10.0.0.243 flowid 1:2

echo "Storage Mechanism 8 HOP"
echo "Latency: 70 ms"
echo "Bandwidth: 60mbit"
echo -e "\n--------------------------------------------------------\n"
for i in {1..100}; do
    # Single
    read dr1 dm1 <<< $(curl -s "$HOST$K_SINGLE_CLOUD_1" -H "$H_SINGLE")
    read dr2 dm2 <<< $(curl -s "$HOST$K_SINGLE_CLOUD_1" -H "$H_SINGLE")
    read dr3 dm3 <<< $(curl -s "$HOST$K_SINGLE_CLOUD_1" -H "$H_SINGLE")
    ssf_sumdr=$((dr1 + dr2 + dr3))
    ssf_sumdm=$((dm1 + dm2 + dm3))
     # Bundle
     read bsf_dr bsf_dm <<< $(curl -s "$HOST$K_BUNDLE_CLOUD_3" -H "$H_BUNDLE")
    echo -e "$i\t$ssf_sumdr\t$ssf_sumdm\t$bsf_dr\t$bsf_dm"
done

echo -e "\n--------------------------------------------------------\n"

sudo tc qdisc del dev wlan0 root
sudo tc qdisc add dev wlan0 root handle 1: htb default 1
sudo tc class add dev wlan0 parent 1: classid 1:1 htb rate 1000mbit ceil 10000mbit
sudo tc class add dev wlan0 parent 1: classid 1:2 htb rate 70mbit ceil 80mbit
sudo tc qdisc add dev wlan0 parent 1:2 handle 20: netem delay 80ms 72ms
sudo tc filter del dev wlan0 parent 1:0
sudo tc filter add dev wlan0 protocol ip parent 1:0 prio 2 u32 match ip dst 10.0.0.243 flowid 1:2

echo "Storage Mechanism 9 HOP"
echo "Latency: 79 ms"
echo "Bandwidth: 60mbit"
echo -e "\n--------------------------------------------------------\n"
for i in {1..100}; do
    # Single
    read dr1 dm1 <<< $(curl -s "$HOST$K_SINGLE_CLOUD_1" -H "$H_SINGLE")
    read dr2 dm2 <<< $(curl -s "$HOST$K_SINGLE_CLOUD_1" -H "$H_SINGLE")
    read dr3 dm3 <<< $(curl -s "$HOST$K_SINGLE_CLOUD_1" -H "$H_SINGLE")
    ssf_sumdr=$((dr1 + dr2 + dr3))
    ssf_sumdm=$((dm1 + dm2 + dm3))
     # Bundle
     read bsf_dr bsf_dm <<< $(curl -s "$HOST$K_BUNDLE_CLOUD_3" -H "$H_BUNDLE")
    echo -e "$i\t$ssf_sumdr\t$ssf_sumdm\t$bsf_dr\t$bsf_dm"
done

echo -e "\n--------------------------------------------------------\n"

sudo tc qdisc del dev wlan0 root
sudo tc qdisc add dev wlan0 root handle 1: htb default 1
sudo tc class add dev wlan0 parent 1: classid 1:1 htb rate 1000mbit ceil 10000mbit
sudo tc class add dev wlan0 parent 1: classid 1:2 htb rate 70mbit ceil 80mbit
sudo tc qdisc add dev wlan0 parent 1:2 handle 20: netem delay 90ms 61ms
sudo tc filter del dev wlan0 parent 1:0
sudo tc filter add dev wlan0 protocol ip parent 1:0 prio 2 u32 match ip dst 10.0.0.243 flowid 1:2

echo "Storage Mechanism 10 HOP"
echo "Latency: 88 ms"
echo "Bandwidth: 60mbit"
echo -e "\n--------------------------------------------------------\n"
for i in {1..100}; do
    # Single
    read dr1 dm1 <<< $(curl -s "$HOST$K_SINGLE_CLOUD_1" -H "$H_SINGLE")
    read dr2 dm2 <<< $(curl -s "$HOST$K_SINGLE_CLOUD_1" -H "$H_SINGLE")
    read dr3 dm3 <<< $(curl -s "$HOST$K_SINGLE_CLOUD_1" -H "$H_SINGLE")
    ssf_sumdr=$((dr1 + dr2 + dr3))
    ssf_sumdm=$((dm1 + dm2 + dm3))
     # Bundle
     read bsf_dr bsf_dm <<< $(curl -s "$HOST$K_BUNDLE_CLOUD_3" -H "$H_BUNDLE")
    echo -e "$i\t$ssf_sumdr\t$ssf_sumdm\t$bsf_dr\t$bsf_dm"
done

echo -e "\n--------------------------------------------------------\n"
