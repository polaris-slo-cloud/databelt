# Storage Experiment
# Settings:
# Sat-Cloud
#   Bandwith -  80mbit
#   Latency -   60ms
#   Jitter -    15ms
sudo tc qdisc del dev wlan0 root
sudo tc filter del dev wlan0 parent 1:0
tc -s qdisc show dev wlan0
tc -s class show dev wlan0
tc -s filter show dev wlan0
iperf3 -c 10.0.0.
ping 10.0.0.

# pi5u1 3-HOP
sudo tc qdisc del dev wlan0 root
sudo tc qdisc add dev wlan0 root handle 1: htb default 1
sudo tc class add dev wlan0 parent 1: classid 1:1 htb rate 1000mbit ceil 10000mbit
sudo tc class add dev wlan0 parent 1: classid 1:2 htb rate 70mbit ceil 80mbit
sudo tc qdisc add dev wlan0 parent 1:2 handle 20: netem delay 10ms 8ms
sudo tc filter del dev wlan0 parent 1:0
sudo tc filter add dev wlan0 protocol ip parent 1:0 prio 2 u32 match ip dst 10.0.0.58 flowid 1:2
# pi54u5 3-HOP
sudo tc qdisc del dev wlan0 root
sudo tc qdisc add dev wlan0 root handle 1: htb default 1
sudo tc class add dev wlan0 parent 1: classid 1:1 htb rate 1000mbit ceil 10000mbit
sudo tc class add dev wlan0 parent 1: classid 1:2 htb rate 70mbit ceil 80mbit
sudo tc qdisc add dev wlan0 parent 1:2 handle 20: netem delay 10ms 8ms
sudo tc filter del dev wlan0 parent 1:0
sudo tc filter add dev wlan0 protocol ip parent 1:0 prio 2 u32 match ip dst 10.0.0.243 flowid 1:2

# pi5u1 4-HOP
sudo tc qdisc del dev wlan0 root
sudo tc qdisc add dev wlan0 root handle 1: htb default 1
sudo tc class add dev wlan0 parent 1: classid 1:1 htb rate 1000mbit ceil 10000mbit
sudo tc class add dev wlan0 parent 1: classid 1:2 htb rate 70mbit ceil 80mbit
sudo tc qdisc add dev wlan0 parent 1:2 handle 20: netem delay 15ms 12ms
sudo tc filter del dev wlan0 parent 1:0
sudo tc filter add dev wlan0 protocol ip parent 1:0 prio 2 u32 match ip dst 10.0.0.58 flowid 1:2
# pi54u5 4-HOP
sudo tc qdisc del dev wlan0 root
sudo tc qdisc add dev wlan0 root handle 1: htb default 1
sudo tc class add dev wlan0 parent 1: classid 1:1 htb rate 1000mbit ceil 10000mbit
sudo tc class add dev wlan0 parent 1: classid 1:2 htb rate 70mbit ceil 80mbit
sudo tc qdisc add dev wlan0 parent 1:2 handle 20: netem delay 15ms 12ms
sudo tc filter del dev wlan0 parent 1:0
sudo tc filter add dev wlan0 protocol ip parent 1:0 prio 2 u32 match ip dst 10.0.0.243 flowid 1:2


# pi5u1 5-HOP
sudo tc qdisc del dev wlan0 root
sudo tc qdisc add dev wlan0 root handle 1: htb default 1
sudo tc class add dev wlan0 parent 1: classid 1:1 htb rate 1000mbit ceil 10000mbit
sudo tc class add dev wlan0 parent 1: classid 1:2 htb rate 70mbit ceil 80mbit
sudo tc qdisc add dev wlan0 parent 1:2 handle 20: netem delay 20ms 16ms
sudo tc filter del dev wlan0 parent 1:0
sudo tc filter add dev wlan0 protocol ip parent 1:0 prio 2 u32 match ip dst 10.0.0.58 flowid 1:2
# pi54u5 5-HOP
sudo tc qdisc del dev wlan0 root
sudo tc qdisc add dev wlan0 root handle 1: htb default 1
sudo tc class add dev wlan0 parent 1: classid 1:1 htb rate 1000mbit ceil 10000mbit
sudo tc class add dev wlan0 parent 1: classid 1:2 htb rate 70mbit ceil 80mbit
sudo tc qdisc add dev wlan0 parent 1:2 handle 20: netem delay 20ms 16ms
sudo tc filter del dev wlan0 parent 1:0
sudo tc filter add dev wlan0 protocol ip parent 1:0 prio 2 u32 match ip dst 10.0.0.243 flowid 1:2


# pi5u1 6-HOP
sudo tc qdisc del dev wlan0 root
sudo tc qdisc add dev wlan0 root handle 1: htb default 1
sudo tc class add dev wlan0 parent 1: classid 1:1 htb rate 1000mbit ceil 10000mbit
sudo tc class add dev wlan0 parent 1: classid 1:2 htb rate 70mbit ceil 80mbit
sudo tc qdisc add dev wlan0 parent 1:2 handle 20: netem delay 25ms 20ms
sudo tc filter del dev wlan0 parent 1:0
sudo tc filter add dev wlan0 protocol ip parent 1:0 prio 2 u32 match ip dst 10.0.0.58 flowid 1:2
# pi54u5 6-HOP
sudo tc qdisc del dev wlan0 root
sudo tc qdisc add dev wlan0 root handle 1: htb default 1
sudo tc class add dev wlan0 parent 1: classid 1:1 htb rate 1000mbit ceil 10000mbit
sudo tc class add dev wlan0 parent 1: classid 1:2 htb rate 70mbit ceil 80mbit
sudo tc qdisc add dev wlan0 parent 1:2 handle 20: netem delay 25ms 20ms
sudo tc filter del dev wlan0 parent 1:0
sudo tc filter add dev wlan0 protocol ip parent 1:0 prio 2 u32 match ip dst 10.0.0.243 flowid 1:2


# pi5u1 7-HOP
sudo tc qdisc del dev wlan0 root
sudo tc qdisc add dev wlan0 root handle 1: htb default 1
sudo tc class add dev wlan0 parent 1: classid 1:1 htb rate 1000mbit ceil 10000mbit
sudo tc class add dev wlan0 parent 1: classid 1:2 htb rate 70mbit ceil 80mbit
sudo tc qdisc add dev wlan0 parent 1:2 handle 20: netem delay 30ms 24ms
sudo tc filter del dev wlan0 parent 1:0
sudo tc filter add dev wlan0 protocol ip parent 1:0 prio 2 u32 match ip dst 10.0.0.58 flowid 1:2
# pi54u5 7-HOP
sudo tc qdisc del dev wlan0 root
sudo tc qdisc add dev wlan0 root handle 1: htb default 1
sudo tc class add dev wlan0 parent 1: classid 1:1 htb rate 1000mbit ceil 10000mbit
sudo tc class add dev wlan0 parent 1: classid 1:2 htb rate 70mbit ceil 80mbit
sudo tc qdisc add dev wlan0 parent 1:2 handle 20: netem delay 30ms 24ms
sudo tc filter del dev wlan0 parent 1:0
sudo tc filter add dev wlan0 protocol ip parent 1:0 prio 2 u32 match ip dst 10.0.0.243 flowid 1:2


# pi5u1 8-HOP
sudo tc qdisc del dev wlan0 root
sudo tc qdisc add dev wlan0 root handle 1: htb default 1
sudo tc class add dev wlan0 parent 1: classid 1:1 htb rate 1000mbit ceil 10000mbit
sudo tc class add dev wlan0 parent 1: classid 1:2 htb rate 70mbit ceil 80mbit
sudo tc qdisc add dev wlan0 parent 1:2 handle 20: netem delay 35ms 28ms
sudo tc filter del dev wlan0 parent 1:0
sudo tc filter add dev wlan0 protocol ip parent 1:0 prio 2 u32 match ip dst 10.0.0.58 flowid 1:2
# pi54u5 8-HOP
sudo tc qdisc del dev wlan0 root
sudo tc qdisc add dev wlan0 root handle 1: htb default 1
sudo tc class add dev wlan0 parent 1: classid 1:1 htb rate 1000mbit ceil 10000mbit
sudo tc class add dev wlan0 parent 1: classid 1:2 htb rate 70mbit ceil 80mbit
sudo tc qdisc add dev wlan0 parent 1:2 handle 20: netem delay 35ms 28ms
sudo tc filter del dev wlan0 parent 1:0
sudo tc filter add dev wlan0 protocol ip parent 1:0 prio 2 u32 match ip dst 10.0.0.243 flowid 1:2


# pi5u1 9-HOP
sudo tc qdisc del dev wlan0 root
sudo tc qdisc add dev wlan0 root handle 1: htb default 1
sudo tc class add dev wlan0 parent 1: classid 1:1 htb rate 1000mbit ceil 10000mbit
sudo tc class add dev wlan0 parent 1: classid 1:2 htb rate 70mbit ceil 80mbit
sudo tc qdisc add dev wlan0 parent 1:2 handle 20: netem delay 40ms 32ms
sudo tc filter del dev wlan0 parent 1:0
sudo tc filter add dev wlan0 protocol ip parent 1:0 prio 2 u32 match ip dst 10.0.0.58 flowid 1:2
# pi54u5 9-HOP
sudo tc qdisc del dev wlan0 root
sudo tc qdisc add dev wlan0 root handle 1: htb default 1
sudo tc class add dev wlan0 parent 1: classid 1:1 htb rate 1000mbit ceil 10000mbit
sudo tc class add dev wlan0 parent 1: classid 1:2 htb rate 70mbit ceil 80mbit
sudo tc qdisc add dev wlan0 parent 1:2 handle 20: netem delay 40ms 32ms
sudo tc filter del dev wlan0 parent 1:0
sudo tc filter add dev wlan0 protocol ip parent 1:0 prio 2 u32 match ip dst 10.0.0.243 flowid 1:2


# pi5u1 10-HOP
sudo tc qdisc del dev wlan0 root
sudo tc qdisc add dev wlan0 root handle 1: htb default 1
sudo tc class add dev wlan0 parent 1: classid 1:1 htb rate 1000mbit ceil 10000mbit
sudo tc class add dev wlan0 parent 1: classid 1:2 htb rate 70mbit ceil 80mbit
sudo tc qdisc add dev wlan0 parent 1:2 handle 20: netem delay 45ms 36ms
sudo tc filter del dev wlan0 parent 1:0
sudo tc filter add dev wlan0 protocol ip parent 1:0 prio 2 u32 match ip dst 10.0.0.58 flowid 1:2
# pi54u5 10-HOP
sudo tc qdisc del dev wlan0 root
sudo tc qdisc add dev wlan0 root handle 1: htb default 1
sudo tc class add dev wlan0 parent 1: classid 1:1 htb rate 1000mbit ceil 10000mbit
sudo tc class add dev wlan0 parent 1: classid 1:2 htb rate 70mbit ceil 80mbit
sudo tc qdisc add dev wlan0 parent 1:2 handle 20: netem delay 45ms 36ms
sudo tc filter del dev wlan0 parent 1:0
sudo tc filter add dev wlan0 protocol ip parent 1:0 prio 2 u32 match ip dst 10.0.0.243 flowid 1:2