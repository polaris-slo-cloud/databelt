#!/bin/bash
# t = 2
sudo tc filter del dev wlan0 parent 1:0
sudo tc filter add dev wlan0 protocol ip parent 1:0 prio 1 u32 match ip dst 10.0.0.167 flowid 1:1
sudo tc filter add dev wlan0 protocol ip parent 1:0 prio 2 u32 match ip dst 10.0.0.45 flowid 1:2
sudo tc filter add dev wlan0 protocol ip parent 1:0 prio 3 u32 match ip dst 10.0.0.34 flowid 1:3
sudo tc filter add dev wlan0 protocol ip parent 1:0 prio 4 u32 match ip dst 10.0.0.245 flowid 1:4
sudo tc filter add dev wlan0 protocol ip parent 1:0 prio 4 u32 match ip dst 10.0.0.210 flowid 1:4
sudo tc filter add dev wlan0 protocol ip parent 1:0 prio 3 u32 match ip dst 10.0.0.123 flowid 1:3
sudo tc filter add dev wlan0 protocol ip parent 1:0 prio 2 u32 match ip dst 10.0.0.58 flowid 1:2