#!/bin/bash

cargo build
./target/debug/trust & # "&" is running comand and skipping the other command. (runs in background)
pid=$! # $! returns the last applications pid
trap "echo i got u $pid & kill -9 $pid" SIGINT
sudo ip link set up dev tun0
ip addr add 192.168.0.1/24 dev tun0
wait $pid