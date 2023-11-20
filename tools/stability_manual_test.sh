#!/bin/bash

set -e -x

echo "Starting the test"

export RUST_BACKTRACE=1
export RUST_LOG=trace

function cleanup {
  echo "Terminating ank-server and ank-agents(s)"
  pkill -f ank-
}

trap cleanup EXIT

for i in {1..100}
do
  echo "Starting $i-th iteration"
  ./ank-server -c startConfig.yaml &

  sleep 4

  ./ank-agent --name agent_A &
  ./ank-agent --name agent_B &

  sleep 4

  ./ank get workloads
  ./ank delete workload hello1
  ./ank delete workload hello2
  ./ank delete workload nginx
  ./ank delete workload hello-pod

  pkill -f ank-

  echo "Done"
done
