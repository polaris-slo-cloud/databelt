#!/bin/bash

# ./run_propagate_scalability.sh 5 run_propagate_workflow.sh keys_1M Skylark 10
# Check for input arguments
if [ "$#" -lt 4 ]; then
    echo "Usage: $0  <test_script> <policy> <repetition_count> <input_file> <fanout_degree>"
    exit 1
fi

# Arguments

TEST_SCRIPT=$1
POLICY=$2
REP=$3
KEY_FILE=$4
FANOUT_DEGREE=$5
echo -e "FANOUT_DEGREE: $FANOUT_DEGREE\nTEST_SCRIPT: $TEST_SCRIPT\nKEY_FILE: $KEY_FILE\nPOLICY: $POLICY\nREP: $REP"
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
for i in $(seq 1 "$REP"); do
  # Loop through inputs with fanout logic
  count=0
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

  echo "$FANOUT_DEGREE,$WORKFLOW_LATENCY"
done
