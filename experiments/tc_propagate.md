sudo tc qdisc del dev wlan0 root
tc -s qdisc show dev wlan0
tc -s class show dev wlan0
tc -s filter show dev wlan0
sudo tc filter del dev wlan0 parent 1:0
iperf3 -c 10.0.0.
ping 10.0.0.
# Settings:
# Sat-Sat 1 hop
#   Bandwith - min: 80mbit, max: 100mbit
#   Latency - 5ms
#
# Sat-Cloud 1 hop:
#   Bandwith - min: 70mbit, max: 80mbit
#   Latency - 20ms
#
#  u2 -10ms- u3 -10ms- u4 -10ms- u5 -10ms- u6 -10ms- u8 -10ms- p1
#                                                               |
#                                                               45ms
#                                                                |
#                                                               u1
# Filters
# Satellite Latency profiles
sudo tc qdisc del dev wlan0 root
sudo tc qdisc add dev wlan0 root handle 1: htb default 1
sudo tc class add dev wlan0 parent 1: classid 1:1 htb rate 80mbit ceil 100mbit
sudo tc qdisc add dev wlan0 parent 1:1 handle 10: netem delay 1ms
sudo tc class add dev wlan0 parent 1: classid 1:2 htb rate 80mbit ceil 100mbit
sudo tc qdisc add dev wlan0 parent 1:2 handle 20: netem delay 10ms
sudo tc class add dev wlan0 parent 1: classid 1:3 htb rate 80mbit ceil 100mbit
sudo tc qdisc add dev wlan0 parent 1:3 handle 30: netem delay 15ms
sudo tc class add dev wlan0 parent 1: classid 1:4 htb rate 80mbit ceil 100mbit
sudo tc qdisc add dev wlan0 parent 1:4 handle 40: netem delay 20ms
sudo tc class add dev wlan0 parent 1: classid 1:5 htb rate 80mbit ceil 100mbit
sudo tc qdisc add dev wlan0 parent 1:5 handle 50: netem delay 25ms
sudo tc class add dev wlan0 parent 1: classid 1:6 htb rate 80mbit ceil 100mbit
sudo tc qdisc add dev wlan0 parent 1:6 handle 60: netem delay 30ms
sudo tc class add dev wlan0 parent 1: classid 1:7 htb rate 80mbit ceil 100mbit
sudo tc qdisc add dev wlan0 parent 1:7 handle 70: netem delay 35ms



# pi5u1 243
sudo tc qdisc del dev wlan0 root
sudo tc qdisc add dev wlan0 root handle 1: htb default 8
sudo tc class add dev wlan0 parent 1: classid 1:1 htb rate 80mbit ceil 100mbit
sudo tc qdisc add dev wlan0 parent 1:1 handle 10: netem delay 20ms
sudo tc class add dev wlan0 parent 1: classid 1:2 htb rate 70mbit ceil 100mbit
sudo tc qdisc add dev wlan0 parent 1:2 handle 20: netem delay 25ms
sudo tc class add dev wlan0 parent 1: classid 1:3 htb rate 70mbit ceil 100mbit
sudo tc qdisc add dev wlan0 parent 1:3 handle 30: netem delay 30ms
sudo tc class add dev wlan0 parent 1: classid 1:4 htb rate 70mbit ceil 100mbit
sudo tc qdisc add dev wlan0 parent 1:4 handle 40: netem delay 35ms
sudo tc class add dev wlan0 parent 1: classid 1:5 htb rate 70mbit ceil 100mbit
sudo tc qdisc add dev wlan0 parent 1:5 handle 50: netem delay 40ms
sudo tc class add dev wlan0 parent 1: classid 1:6 htb rate 70mbit ceil 100mbit
sudo tc qdisc add dev wlan0 parent 1:6 handle 60: netem delay 45ms
sudo tc class add dev wlan0 parent 1: classid 1:7 htb rate 70mbit ceil 100mbit
sudo tc qdisc add dev wlan0 parent 1:7 handle 70: netem delay 50ms
sudo tc class add dev wlan0 parent 1: classid 1:8 htb rate 1000mbit ceil 1000mbit
sudo tc filter del dev wlan0 parent 1:0
sudo tc filter add dev wlan0 protocol ip parent 1:0 prio 7 u32 match ip dst 10.0.0.34 flowid 1:7
sudo tc filter add dev wlan0 protocol ip parent 1:0 prio 6 u32 match ip dst 10.0.0.45 flowid 1:6
sudo tc filter add dev wlan0 protocol ip parent 1:0 prio 5 u32 match ip dst 10.0.0.167 flowid 1:5
sudo tc filter add dev wlan0 protocol ip parent 1:0 prio 4 u32 match ip dst 10.0.0.58 flowid 1:4
sudo tc filter add dev wlan0 protocol ip parent 1:0 prio 3 u32 match ip dst 10.0.0.122 flowid 1:3
sudo tc filter add dev wlan0 protocol ip parent 1:0 prio 2 u32 match ip dst 10.0.0.210 flowid 1:2
sudo tc filter add dev wlan0 protocol ip parent 1:0 prio 1 u32 match ip dst 10.0.0.245 flowid 1:1

# pi5u2 34
sudo tc class add dev wlan0 parent 1: classid 1:8 htb rate 80mbit ceil 100mbit
sudo tc qdisc add dev wlan0 parent 1:8 handle 80: netem delay 50ms
sudo tc filter del dev wlan0 parent 1:0
sudo tc filter add dev wlan0 protocol ip parent 1:0 prio 1 u32 match ip dst 10.0.0.45 flowid 1:1
sudo tc filter add dev wlan0 protocol ip parent 1:0 prio 2 u32 match ip dst 10.0.0.167 flowid 1:2
sudo tc filter add dev wlan0 protocol ip parent 1:0 prio 3 u32 match ip dst 10.0.0.58 flowid 1:3
sudo tc filter add dev wlan0 protocol ip parent 1:0 prio 4 u32 match ip dst 10.0.0.122 flowid 1:4
sudo tc filter add dev wlan0 protocol ip parent 1:0 prio 5 u32 match ip dst 10.0.0.210 flowid 1:5
sudo tc filter add dev wlan0 protocol ip parent 1:0 prio 6 u32 match ip dst 10.0.0.245 flowid 1:6
sudo tc filter add dev wlan0 protocol ip parent 1:0 prio 8 u32 match ip dst 10.0.0.243 flowid 1:8

# pi5u3 45
sudo tc class add dev wlan0 parent 1: classid 1:8 htb rate 80mbit ceil 100mbit
sudo tc qdisc add dev wlan0 parent 1:8 handle 80: netem delay 45ms
sudo tc filter del dev wlan0 parent 1:0
sudo tc filter add dev wlan0 protocol ip parent 1:0 prio 1 u32 match ip dst 10.0.0.34 flowid 1:1
sudo tc filter add dev wlan0 protocol ip parent 1:0 prio 1 u32 match ip dst 10.0.0.167 flowid 1:1
sudo tc filter add dev wlan0 protocol ip parent 1:0 prio 2 u32 match ip dst 10.0.0.58 flowid 1:2
sudo tc filter add dev wlan0 protocol ip parent 1:0 prio 3 u32 match ip dst 10.0.0.122 flowid 1:3
sudo tc filter add dev wlan0 protocol ip parent 1:0 prio 4 u32 match ip dst 10.0.0.210 flowid 1:4
sudo tc filter add dev wlan0 protocol ip parent 1:0 prio 5 u32 match ip dst 10.0.0.245 flowid 1:5
sudo tc filter add dev wlan0 protocol ip parent 1:0 prio 8 u32 match ip dst 10.0.0.243 flowid 1:8

# pi5u4 167
sudo tc class add dev wlan0 parent 1: classid 1:8 htb rate 80mbit ceil 100mbit
sudo tc qdisc add dev wlan0 parent 1:8 handle 80: netem delay 40ms
sudo tc filter del dev wlan0 parent 1:0
sudo tc filter add dev wlan0 protocol ip parent 1:0 prio 2 u32 match ip dst 10.0.0.34 flowid 1:2
sudo tc filter add dev wlan0 protocol ip parent 1:0 prio 1 u32 match ip dst 10.0.0.45 flowid 1:1
sudo tc filter add dev wlan0 protocol ip parent 1:0 prio 1 u32 match ip dst 10.0.0.58 flowid 1:1
sudo tc filter add dev wlan0 protocol ip parent 1:0 prio 2 u32 match ip dst 10.0.0.122 flowid 1:2
sudo tc filter add dev wlan0 protocol ip parent 1:0 prio 3 u32 match ip dst 10.0.0.210 flowid 1:3
sudo tc filter add dev wlan0 protocol ip parent 1:0 prio 4 u32 match ip dst 10.0.0.245 flowid 1:4
sudo tc filter add dev wlan0 protocol ip parent 1:0 prio 8 u32 match ip dst 10.0.0.243 flowid 1:8

# pi4u5 58
sudo tc class add dev wlan0 parent 1: classid 1:8 htb rate 80mbit ceil 100mbit
sudo tc qdisc add dev wlan0 parent 1:8 handle 80: netem delay 35ms
sudo tc filter del dev wlan0 parent 1:0
sudo tc filter add dev wlan0 protocol ip parent 1:0 prio 3 u32 match ip dst 10.0.0.34 flowid 1:3
sudo tc filter add dev wlan0 protocol ip parent 1:0 prio 2 u32 match ip dst 10.0.0.45 flowid 1:2
sudo tc filter add dev wlan0 protocol ip parent 1:0 prio 1 u32 match ip dst 10.0.0.167 flowid 1:1
sudo tc filter add dev wlan0 protocol ip parent 1:0 prio 1 u32 match ip dst 10.0.0.122 flowid 1:1
sudo tc filter add dev wlan0 protocol ip parent 1:0 prio 2 u32 match ip dst 10.0.0.210 flowid 1:2
sudo tc filter add dev wlan0 protocol ip parent 1:0 prio 3 u32 match ip dst 10.0.0.245 flowid 1:3
sudo tc filter add dev wlan0 protocol ip parent 1:0 prio 8 u32 match ip dst 10.0.0.243 flowid 1:8

# pi4u6 122
sudo tc filter del dev wlan0 parent 1:0
sudo tc filter add dev wlan0 protocol ip parent 1:0 prio 4 u32 match ip dst 10.0.0.34 flowid 1:4
sudo tc filter add dev wlan0 protocol ip parent 1:0 prio 3 u32 match ip dst 10.0.0.45 flowid 1:3
sudo tc filter add dev wlan0 protocol ip parent 1:0 prio 2 u32 match ip dst 10.0.0.167 flowid 1:2
sudo tc filter add dev wlan0 protocol ip parent 1:0 prio 1 u32 match ip dst 10.0.0.58 flowid 1:1
sudo tc filter add dev wlan0 protocol ip parent 1:0 prio 1 u32 match ip dst 10.0.0.210 flowid 1:1
sudo tc filter add dev wlan0 protocol ip parent 1:0 prio 2 u32 match ip dst 10.0.0.245 flowid 1:2
sudo tc filter add dev wlan0 protocol ip parent 1:0 prio 7 u32 match ip dst 10.0.0.243 flowid 1:7

# pi4u8 210
sudo tc filter del dev wlan0 parent 1:0
sudo tc filter add dev wlan0 protocol ip parent 1:0 prio 5 u32 match ip dst 10.0.0.34 flowid 1:5
sudo tc filter add dev wlan0 protocol ip parent 1:0 prio 4 u32 match ip dst 10.0.0.45 flowid 1:4
sudo tc filter add dev wlan0 protocol ip parent 1:0 prio 3 u32 match ip dst 10.0.0.167 flowid 1:3
sudo tc filter add dev wlan0 protocol ip parent 1:0 prio 2 u32 match ip dst 10.0.0.58 flowid 1:2
sudo tc filter add dev wlan0 protocol ip parent 1:0 prio 1 u32 match ip dst 10.0.0.122 flowid 1:1
sudo tc filter add dev wlan0 protocol ip parent 1:0 prio 1 u32 match ip dst 10.0.0.245 flowid 1:1
sudo tc filter add dev wlan0 protocol ip parent 1:0 prio 6 u32 match ip dst 10.0.0.243 flowid 1:6

# pi4p1 245
sudo tc filter del dev wlan0 parent 1:0
sudo tc filter add dev wlan0 protocol ip parent 1:0 prio 6 u32 match ip dst 10.0.0.34 flowid 1:6
sudo tc filter add dev wlan0 protocol ip parent 1:0 prio 5 u32 match ip dst 10.0.0.45 flowid 1:5
sudo tc filter add dev wlan0 protocol ip parent 1:0 prio 4 u32 match ip dst 10.0.0.167 flowid 1:4
sudo tc filter add dev wlan0 protocol ip parent 1:0 prio 3 u32 match ip dst 10.0.0.58 flowid 1:3
sudo tc filter add dev wlan0 protocol ip parent 1:0 prio 2 u32 match ip dst 10.0.0.122 flowid 1:2
sudo tc filter add dev wlan0 protocol ip parent 1:0 prio 1 u32 match ip dst 10.0.0.210 flowid 1:1
sudo tc filter add dev wlan0 protocol ip parent 1:0 prio 5 u32 match ip dst 10.0.0.243 flowid 1:5


