#!/bin/bash

# Configuration
URL="http://10.0.0.167:8081/benchmark"
REQUESTS_PER_SECOND=10
DURATION=10

# Calculate delay between requests
DELAY=$(echo "scale=2; 1 / $REQUESTS_PER_SECOND" | bc)

# Run the loop
END_TIME=$((SECONDS + DURATION))
COUNT=0

echo "Sending $REQUESTS_PER_SECOND requests per second to $URL for $DURATION seconds..."

while [ $SECONDS -lt $END_TIME ]; do
  # Send the request
  curl -s -o /dev/null -w "%{http_code}\n" "$URL" &

  # Count requests
  COUNT=$((COUNT + 1))

  # Sleep to maintain rate
  sleep "$DELAY"
done

echo "Completed $COUNT requests."
