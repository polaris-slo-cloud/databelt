# Storage Experiment
# Settings:
# Sat-Cloud
#   Bandwith -  80mbit
#   Latency -   60ms
#   Jitter -    15ms
sudo tc qdisc del dev wlan0 root
tc -s qdisc show dev wlan0
tc -s class show dev wlan0
tc -s filter show dev wlan0
sudo tc filter del dev wlan0 parent 1:0
iperf3 -c 10.0.0.
ping 10.0.0.

# pi5u1
sudo tc qdisc del dev wlan0 root
sudo tc qdisc add dev wlan0 root handle 1: htb default 1
sudo tc class add dev wlan0 parent 1: classid 1:1 htb rate 1000mbit ceil 10000mbit
sudo tc class add dev wlan0 parent 1: classid 1:2 htb rate 80mbit
sudo tc qdisc add dev wlan0 parent 1:2 handle 20: netem delay 25ms 7ms
sudo tc filter del dev wlan0 parent 1:0
sudo tc filter add dev wlan0 protocol ip parent 1:0 prio 2 u32 match ip dst 10.0.0.58 flowid 1:2
# pi54u5
sudo tc qdisc del dev wlan0 root
sudo tc qdisc add dev wlan0 root handle 1: htb default 1
sudo tc class add dev wlan0 parent 1: classid 1:1 htb rate 1000mbit ceil 10000mbit
sudo tc class add dev wlan0 parent 1: classid 1:2 htb rate 80mbit
sudo tc qdisc add dev wlan0 parent 1:2 handle 20: netem delay 25ms 7ms
sudo tc filter del dev wlan0 parent 1:0
sudo tc filter add dev wlan0 protocol ip parent 1:0 prio 2 u32 match ip dst 10.0.0.243 flowid 1:2