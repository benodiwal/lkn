#!/bin/bash

cargo build --release
sudo setcap cap_net_admin=eip target/release/lkn
./target/release/lkn &
pid=$!
sudo ip addr add 10.0.0.1/24 dev lkn0
sudo ip link set up dev lkn0
wait $pid
