#!/bin/bash

# ./run_propagate_scalability.sh run_propagate_workflow.sh keys_2M 5 Skylark 5
# Check for input arguments
if [ "$#" -lt 4 ]; then
    echo "Usage: $0  <test_script> <input_file> <repetition_count> <policy> <fanout_degree>"
    exit 1
fi

# Arguments
TEST_SCRIPT=$1
KEY_FILE=$2
REP=$3
POLICY=$4
FANOUT_DEGREE=$5

# CLEAR REDIS
clear_redis() {
    echo "CLEAR REDIS"
    redis-cli -h pi5u1 -p 6379 FLUSHALL
    redis-cli -h pi5u3 -p 6379 FLUSHALL
    redis-cli -h pi5u4 -p 6379 FLUSHALL
    redis-cli -h pi4u5 -p 6379 FLUSHALL
    redis-cli -h pi4u6 -p 6379 FLUSHALL
    redis-cli -h pi4u8 -p 6379 FLUSHALL
    redis-cli -h pi4p1 -p 6379 FLUSHALL
}

echo -e "$FANOUT_DEGREE,$KEY_FILE,$POLICY,$REP"
# Verify test script exists
if [ ! -f "$TEST_SCRIPT" ]; then
    echo "Error: Test script '$TEST_SCRIPT' not found!"
    exit 1
fi

# Verify input file exists
if [ ! -f "$KEY_FILE" ]; then
    echo "Error: Input file '$KEY_FILE' not found!"
    exit 1
fi

# Read inputs from the input file
readarray -t KEYS < "$KEY_FILE"

# Function to run the test script
run_test() {
    local key="$1"
    # echo "Running $TEST_SCRIPT with input: $POLICY $key"
    bash "$TEST_SCRIPT" "$POLICY" "$key" &
}

EX_START_TIME=$(date +%s%3N)
echo "EX START: $((EX_START_TIME / 1000))"

for i in $(seq 1 "$REP"); do
  # Loop through inputs with fanout logic
  count=0
  clear_redis
  START_TIME=$(date +%s%3N)
  for key in "${KEYS[@]}"; do
      run_test "$key"
      count=$((count + 1))

      # Wait for all processes to complete when fanout degree is reached
      if (( count % FANOUT_DEGREE == 0 )); then
          wait
          break
      fi
  done
  wait
  END_TIME=$(date +%s%3N)
  WORKFLOW_LATENCY=$((END_TIME - START_TIME))
  echo "$FANOUT_DEGREE,$WORKFLOW_LATENCY" >> propagate_scalability.log
done

EX_END_TIME=$(date +%s%3N)
echo "EX END: $((EX_END_TIME / 1000))"
echo "EX RUNTIME: $(((EX_END_TIME - EX_START_TIME) / 1000))"
echo "$((EX_START_TIME / 1000)),$((EX_END_TIME / 1000)),$(((EX_END_TIME - EX_START_TIME) / 1000))" >> propagate_scalability.log