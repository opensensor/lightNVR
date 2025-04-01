#!/bin/bash

# API Stress Test Script for Recordings API
# ---------------------------------------

# Configuration variables
BASE_URL="http://127.0.0.1:8080/api/recordings"
AUTH_HEADER="Basic YWRtaW46YWRtaW4="
START_DATE="2025-03-25T04%3A00%3A00.000Z"
END_DATE="2025-04-02T03%3A59%3A59.000Z"
CONCURRENT_REQUESTS=10  # Number of concurrent requests
TOTAL_REQUESTS=100      # Total number of requests to make
DELAY_BETWEEN_BATCHES=1 # Delay in seconds between batches
REQUEST_TIMEOUT=5       # Timeout for each request in seconds

# Create a results directory
TIMESTAMP=$(date +%Y%m%d_%H%M%S)
RESULTS_DIR="stress_test_results_$TIMESTAMP"
mkdir -p $RESULTS_DIR

# Function to make a single API request and log the result
make_request() {
  local request_id=$1
  local start_time=$(date +%s.%N)
  
  # Generate random pagination to simulate different user patterns
  local page=$((RANDOM % 5 + 1))
  local limit=$((RANDOM % 30 + 10))
  local sort_options=("start_time" "duration" "size" "name")
  local sort=${sort_options[$RANDOM % ${#sort_options[@]}]}
  local order_options=("asc" "desc")
  local order=${order_options[$RANDOM % ${#order_options[@]}]}
  
  # Build the URL with randomized parameters
  local url="${BASE_URL}?page=${page}&limit=${limit}&sort=${sort}&order=${order}&start=${START_DATE}&end=${END_DATE}"
  
  # Make the request
  local response=$(curl -s -w "\n%{http_code}\n%{time_total}" -o "$RESULTS_DIR/response_${request_id}.json" \
    --connect-timeout $REQUEST_TIMEOUT \
    -H 'Accept: application/json' \
    -H 'Accept-Language: en-US,en;q=0.9' \
    -H "Authorization: $AUTH_HEADER" \
    -H 'Connection: keep-alive' \
    -H 'Content-Type: application/json' \
    -H 'Referer: http://127.0.0.1:8080/recordings.html' \
    -H 'Sec-Fetch-Dest: empty' \
    -H 'Sec-Fetch-Mode: cors' \
    -H 'Sec-Fetch-Site: same-origin' \
    -H 'User-Agent: StressTestScript/1.0' \
    -H 'sec-ch-ua: "Chromium";v="134"' \
    -H 'sec-ch-ua-mobile: ?0' \
    -H 'sec-ch-ua-platform: "Linux"' \
    "$url")
  
  # Parse response
  local status_code=$(echo "$response" | tail -n2 | head -n1)
  local time_taken=$(echo "$response" | tail -n1)
  local end_time=$(date +%s.%N)
  
  # Log result to file
  echo "$request_id,$url,$status_code,$time_taken,$(date +%Y-%m-%d\ %H:%M:%S.%N)" >> "$RESULTS_DIR/results.csv"
  
  # Output to console
  echo "Request $request_id: Status $status_code, Time: ${time_taken}s"
  
  # Simulate user behavior with a small random delay
  sleep 0.$(( RANDOM % 5 + 1 ))
}

# Initialize results CSV with header
echo "request_id,url,status_code,time_taken,timestamp" > "$RESULTS_DIR/results.csv"

# Record start time
TOTAL_START_TIME=$(date +%s)

echo "Starting stress test with $TOTAL_REQUESTS total requests, $CONCURRENT_REQUESTS concurrent..."
echo "Results will be stored in $RESULTS_DIR/"

# Run the test in batches
for ((i=1; i<=$TOTAL_REQUESTS; i+=$CONCURRENT_REQUESTS)); do
  echo "Starting batch $((i / $CONCURRENT_REQUESTS + 1))..."
  
  # Launch concurrent requests
  for ((j=0; j<$CONCURRENT_REQUESTS && i+j<=$TOTAL_REQUESTS; j++)); do
    make_request $((i+j)) &
  done
  
  # Wait for all background processes to complete
  wait
  
  echo "Batch completed. Waiting $DELAY_BETWEEN_BATCHES seconds before next batch..."
  sleep $DELAY_BETWEEN_BATCHES
done

# Record end time
TOTAL_END_TIME=$(date +%s)
TOTAL_DURATION=$((TOTAL_END_TIME - TOTAL_START_TIME))

# Generate summary report
echo "Generating summary report..."

# Calculate statistics
SUCCESS_COUNT=$(grep -c ",200," "$RESULTS_DIR/results.csv")
ERROR_COUNT=$((TOTAL_REQUESTS - SUCCESS_COUNT))
SUCCESS_RATE=$(echo "scale=2; $SUCCESS_COUNT * 100 / $TOTAL_REQUESTS" | bc)

# Calculate average, min, max response times
if [ -x "$(command -v awk)" ]; then
  AVG_TIME=$(awk -F, '{sum+=$4} END {print sum/NR}' "$RESULTS_DIR/results.csv")
  MIN_TIME=$(awk -F, '{if(NR==1 || $4<min) min=$4} END {print min}' "$RESULTS_DIR/results.csv")
  MAX_TIME=$(awk -F, '{if($4>max) max=$4} END {print max}' "$RESULTS_DIR/results.csv")
else
  AVG_TIME="N/A (awk not available)"
  MIN_TIME="N/A (awk not available)"
  MAX_TIME="N/A (awk not available)"
fi

# Write summary to file
cat > "$RESULTS_DIR/summary.txt" << EOL
==============================================
API Stress Test Summary
==============================================
Test Date: $(date)
Total Requests: $TOTAL_REQUESTS
Concurrent Requests: $CONCURRENT_REQUESTS
Total Test Duration: $TOTAL_DURATION seconds

Results:
- Successful Requests: $SUCCESS_COUNT ($SUCCESS_RATE%)
- Failed Requests: $ERROR_COUNT

Response Times:
- Average: $AVG_TIME seconds
- Minimum: $MIN_TIME seconds
- Maximum: $MAX_TIME seconds

Server: http://127.0.0.1:8080
Endpoint: /api/recordings
Time Period Tested: $START_DATE to $END_DATE
==============================================
EOL

# Create a visualization if gnuplot is available
if [ -x "$(command -v gnuplot)" ]; then
  echo "Generating response time graph..."
  
  # Extract data for plotting
  awk -F, '{print $1, $4}' "$RESULTS_DIR/results.csv" > "$RESULTS_DIR/plot_data.txt"
  
  # Create gnuplot script
  cat > "$RESULTS_DIR/plot.gp" << EOL
set terminal png size 1200,600
set output "$RESULTS_DIR/response_times.png"
set title "API Response Times"
set xlabel "Request Number"
set ylabel "Response Time (seconds)"
set grid
plot "$RESULTS_DIR/plot_data.txt" using 1:2 with linespoints pt 7 ps 0.5 title "Response Time"
EOL

  # Generate the plot
  gnuplot "$RESULTS_DIR/plot.gp"
  echo "Graph generated at $RESULTS_DIR/response_times.png"
fi

echo "Stress test completed!"
echo "Summary report available at $RESULTS_DIR/summary.txt"
