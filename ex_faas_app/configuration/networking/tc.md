sudo tc qdisc del dev wlan0 root
tc -s qdisc show dev wlan0
tc -s class show dev wlan0
tc -s filter show dev wlan0
sudo tc filter del dev wlan0 parent 1:0
iperf3 -c 10.0.0.
ping 10.0.0.
# Satellite
# 1:1 - 1 Hop, 1:2 - 2 Hops, 1:3 - 3 Hops
sudo tc qdisc add dev wlan0 root handle 1: htb default 5
sudo tc class add dev wlan0 parent 1: classid 1:1 htb rate 80mbit ceil 100mbit
sudo tc qdisc add dev wlan0 parent 1:1 handle 10: netem delay 10ms 9ms
sudo tc class add dev wlan0 parent 1: classid 1:2 htb rate 80mbit ceil 100mbit
sudo tc qdisc add dev wlan0 parent 1:2 handle 20: netem delay 20ms 18ms
sudo tc class add dev wlan0 parent 1: classid 1:3 htb rate 80mbit ceil 100mbit
sudo tc qdisc add dev wlan0 parent 1:3 handle 30: netem delay 30ms 27ms
sudo tc class add dev wlan0 parent 1: classid 1:4 htb rate 20mbit ceil 40mbit
sudo tc qdisc add dev wlan0 parent 1:4 handle 40: netem delay 40ms 20ms
sudo tc class add dev wlan0 parent 1: classid 1:5 htb rate 1000mbit ceil 1000mbit


# Filters
# pi5u2 34
sudo tc filter add dev wlan0 protocol ip parent 1:0 prio 1 u32 match ip dst 10.0.0.45 flowid 1:1
sudo tc filter add dev wlan0 protocol ip parent 1:0 prio 2 u32 match ip dst 10.0.0.167 flowid 1:2
sudo tc filter add dev wlan0 protocol ip parent 1:0 prio 3 u32 match ip dst 10.0.0.58 flowid 1:3
sudo tc filter add dev wlan0 protocol ip parent 1:0 prio 3 u32 match ip dst 10.0.0.123 flowid 1:3
sudo tc filter add dev wlan0 protocol ip parent 1:0 prio 2 u32 match ip dst 10.0.0.210 flowid 1:2
sudo tc filter add dev wlan0 protocol ip parent 1:0 prio 1 u32 match ip dst 10.0.0.245 flowid 1:1
sudo tc filter add dev wlan0 protocol ip parent 1:0 prio 4 u32 match ip dst 10.0.0.243 flowid 1:4

# pi5u3 45
sudo tc filter add dev wlan0 protocol ip parent 1:0 prio 1 u32 match ip dst 10.0.0.34 flowid 1:1
sudo tc filter add dev wlan0 protocol ip parent 1:0 prio 1 u32 match ip dst 10.0.0.167 flowid 1:1
sudo tc filter add dev wlan0 protocol ip parent 1:0 prio 2 u32 match ip dst 10.0.0.58 flowid 1:2
sudo tc filter add dev wlan0 protocol ip parent 1:0 prio 3 u32 match ip dst 10.0.0.123 flowid 1:3
sudo tc filter add dev wlan0 protocol ip parent 1:0 prio 3 u32 match ip dst 10.0.0.210 flowid 1:3
sudo tc filter add dev wlan0 protocol ip parent 1:0 prio 1 u32 match ip dst 10.0.0.245 flowid 1:1
sudo tc filter add dev wlan0 protocol ip parent 1:0 prio 4 u32 match ip dst 10.0.0.243 flowid 1:4

# pi5u4 167
sudo tc filter add dev wlan0 protocol ip parent 1:0 prio 2 u32 match ip dst 10.0.0.34 flowid 1:2
sudo tc filter add dev wlan0 protocol ip parent 1:0 prio 1 u32 match ip dst 10.0.0.45 flowid 1:1
sudo tc filter add dev wlan0 protocol ip parent 1:0 prio 1 u32 match ip dst 10.0.0.58 flowid 1:1
sudo tc filter add dev wlan0 protocol ip parent 1:0 prio 2 u32 match ip dst 10.0.0.123 flowid 1:2
sudo tc filter add dev wlan0 protocol ip parent 1:0 prio 3 u32 match ip dst 10.0.0.210 flowid 1:3
sudo tc filter add dev wlan0 protocol ip parent 1:0 prio 3 u32 match ip dst 10.0.0.245 flowid 1:3
sudo tc filter add dev wlan0 protocol ip parent 1:0 prio 4 u32 match ip dst 10.0.0.243 flowid 1:4

# pi4u5 58
sudo tc filter add dev wlan0 protocol ip parent 1:0 prio 3 u32 match ip dst 10.0.0.34 flowid 1:3
sudo tc filter add dev wlan0 protocol ip parent 1:0 prio 2 u32 match ip dst 10.0.0.45 flowid 1:2
sudo tc filter add dev wlan0 protocol ip parent 1:0 prio 1 u32 match ip dst 10.0.0.167 flowid 1:1
sudo tc filter add dev wlan0 protocol ip parent 1:0 prio 1 u32 match ip dst 10.0.0.123 flowid 1:1
sudo tc filter add dev wlan0 protocol ip parent 1:0 prio 2 u32 match ip dst 10.0.0.210 flowid 1:2
sudo tc filter add dev wlan0 protocol ip parent 1:0 prio 3 u32 match ip dst 10.0.0.245 flowid 1:3
sudo tc filter add dev wlan0 protocol ip parent 1:0 prio 4 u32 match ip dst 10.0.0.243 flowid 1:4

# pi4u6 123
sudo tc filter add dev wlan0 protocol ip parent 1:0 prio 3 u32 match ip dst 10.0.0.34 flowid 1:3
sudo tc filter add dev wlan0 protocol ip parent 1:0 prio 3 u32 match ip dst 10.0.0.45 flowid 1:3
sudo tc filter add dev wlan0 protocol ip parent 1:0 prio 2 u32 match ip dst 10.0.0.167 flowid 1:2
sudo tc filter add dev wlan0 protocol ip parent 1:0 prio 1 u32 match ip dst 10.0.0.58 flowid 1:1
sudo tc filter add dev wlan0 protocol ip parent 1:0 prio 1 u32 match ip dst 10.0.0.210 flowid 1:1
sudo tc filter add dev wlan0 protocol ip parent 1:0 prio 2 u32 match ip dst 10.0.0.245 flowid 1:2
sudo tc filter add dev wlan0 protocol ip parent 1:0 prio 4 u32 match ip dst 10.0.0.243 flowid 1:4

# pi4u8 210
sudo tc filter add dev wlan0 protocol ip parent 1:0 prio 2 u32 match ip dst 10.0.0.34 flowid 1:2
sudo tc filter add dev wlan0 protocol ip parent 1:0 prio 3 u32 match ip dst 10.0.0.45 flowid 1:3
sudo tc filter add dev wlan0 protocol ip parent 1:0 prio 3 u32 match ip dst 10.0.0.167 flowid 1:3
sudo tc filter add dev wlan0 protocol ip parent 1:0 prio 2 u32 match ip dst 10.0.0.58 flowid 1:2
sudo tc filter add dev wlan0 protocol ip parent 1:0 prio 1 u32 match ip dst 10.0.0.123 flowid 1:1
sudo tc filter add dev wlan0 protocol ip parent 1:0 prio 1 u32 match ip dst 10.0.0.245 flowid 1:1
sudo tc filter add dev wlan0 protocol ip parent 1:0 prio 4 u32 match ip dst 10.0.0.243 flowid 1:4

# pi4p1 245
sudo tc filter add dev wlan0 protocol ip parent 1:0 prio 1 u32 match ip dst 10.0.0.34 flowid 1:1
sudo tc filter add dev wlan0 protocol ip parent 1:0 prio 2 u32 match ip dst 10.0.0.45 flowid 1:2
sudo tc filter add dev wlan0 protocol ip parent 1:0 prio 3 u32 match ip dst 10.0.0.167 flowid 1:3
sudo tc filter add dev wlan0 protocol ip parent 1:0 prio 3 u32 match ip dst 10.0.0.58 flowid 1:3
sudo tc filter add dev wlan0 protocol ip parent 1:0 prio 2 u32 match ip dst 10.0.0.123 flowid 1:2
sudo tc filter add dev wlan0 protocol ip parent 1:0 prio 1 u32 match ip dst 10.0.0.210 flowid 1:1
sudo tc filter add dev wlan0 protocol ip parent 1:0 prio 4 u32 match ip dst 10.0.0.243 flowid 1:4

# pi5u1 243
sudo tc filter add dev wlan0 protocol ip parent 1:0 prio 4 u32 match ip dst 10.0.0.34 flowid 1:4
sudo tc filter add dev wlan0 protocol ip parent 1:0 prio 4 u32 match ip dst 10.0.0.45 flowid 1:4
sudo tc filter add dev wlan0 protocol ip parent 1:0 prio 4 u32 match ip dst 10.0.0.167 flowid 1:4
sudo tc filter add dev wlan0 protocol ip parent 1:0 prio 4 u32 match ip dst 10.0.0.58 flowid 1:4
sudo tc filter add dev wlan0 protocol ip parent 1:0 prio 4 u32 match ip dst 10.0.0.123 flowid 1:4
sudo tc filter add dev wlan0 protocol ip parent 1:0 prio 4 u32 match ip dst 10.0.0.210 flowid 1:4
sudo tc filter add dev wlan0 protocol ip parent 1:0 prio 4 u32 match ip dst 10.0.0.245 flowid 1:4
