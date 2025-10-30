# Docker Container Testing Checklist

This checklist ensures all Docker improvements are working correctly.

## Pre-Testing Setup

- [ ] Docker installed and running
- [ ] Docker Compose installed
- [ ] Git repository cloned
- [ ] No existing `config/` or `data/` directories

## Test 1: Fresh Installation (Docker Compose)

### Steps
```bash
# Clean start
rm -rf config/ data/
docker-compose down -v

# Start container
docker-compose up -d

# Wait for initialization
sleep 10
```

### Verification
- [ ] Container starts without errors
- [ ] `config/lightnvr.ini` created automatically
- [ ] `config/go2rtc/go2rtc.yaml` created automatically
- [ ] `data/database/` directory exists
- [ ] `data/recordings/` directory exists
- [ ] Web UI accessible at http://localhost:8080
- [ ] Login page loads correctly
- [ ] Can login with admin/admin
- [ ] Dashboard loads without errors

### Expected Logs
```
[INFO] Initializing LightNVR configuration...
[INFO] Copying web assets from template...
[INFO] Creating default configuration file...
[INFO] Creating default go2rtc configuration...
[INFO] Starting LightNVR...
```

## Test 2: Configuration Persistence

### Steps
```bash
# Modify configuration
echo "# Custom comment" >> config/lightnvr.ini
echo "# Custom go2rtc comment" >> config/go2rtc/go2rtc.yaml

# Restart container
docker-compose restart

# Wait for restart
sleep 5
```

### Verification
- [ ] Custom comments still present in `lightnvr.ini`
- [ ] Custom comments still present in `go2rtc.yaml`
- [ ] Web UI still accessible
- [ ] No new config files created
- [ ] Logs show "Configuration file already exists"

## Test 3: Database Persistence

### Steps
```bash
# Add a test stream via web UI or API
curl -X POST http://localhost:8080/api/streams \
  -H "Content-Type: application/json" \
  -d '{"name":"test-stream","url":"rtsp://test"}'

# Restart container
docker-compose restart

# Wait for restart
sleep 5
```

### Verification
- [ ] Test stream still exists after restart
- [ ] Database file exists at `data/database/lightnvr.db`
- [ ] Database file size > 0
- [ ] Can query streams via API

## Test 4: Web Assets Protection

### Steps
```bash
# Stop container
docker-compose down

# Remove web directory (simulating volume mount issue)
rm -rf data/web

# Start container
docker-compose up -d

# Wait for initialization
sleep 10
```

### Verification
- [ ] Web UI still accessible
- [ ] Web assets re-copied from template
- [ ] All CSS/JS files present
- [ ] No 404 errors in browser console

## Test 5: Fresh Installation (Docker Run)

### Steps
```bash
# Clean start
rm -rf config/ data/
docker stop lightnvr 2>/dev/null || true
docker rm lightnvr 2>/dev/null || true

# Run container
docker run -d \
  --name lightnvr \
  --restart unless-stopped \
  -p 8080:8080 \
  -p 8554:8554 \
  -p 8555:8555 \
  -p 8555:8555/udp \
  -p 1984:1984 \
  -v $(pwd)/config:/etc/lightnvr \
  -v $(pwd)/data:/var/lib/lightnvr/data \
  -e TZ=America/New_York \
  ghcr.io/opensensor/lightnvr:latest

# Wait for initialization
sleep 10
```

### Verification
- [ ] Container starts without errors
- [ ] Config files created
- [ ] Web UI accessible
- [ ] Timezone set correctly (check logs)

## Test 6: Port Accessibility

### Steps
```bash
# Test each port
curl -f http://localhost:8080/ || echo "Port 8080 FAILED"
curl -f http://localhost:1984/api/config || echo "Port 1984 FAILED"
nc -zv localhost 8554 || echo "Port 8554 FAILED"
nc -zuv localhost 8555 || echo "Port 8555 UDP FAILED"
```

### Verification
- [ ] Port 8080 (Web UI) accessible
- [ ] Port 1984 (go2rtc API) accessible
- [ ] Port 8554 (RTSP) listening
- [ ] Port 8555 (WebRTC TCP) listening
- [ ] Port 8555 (WebRTC UDP) listening

## Test 7: Health Check

### Steps
```bash
# Wait for health check to pass
sleep 30

# Check health status
docker inspect lightnvr | grep -A 5 Health
```

### Verification
- [ ] Health status is "healthy"
- [ ] Health check passes consistently
- [ ] No health check failures in logs

## Test 8: Environment Variables

### Steps
```bash
# Stop container
docker-compose down

# Modify docker-compose.yml
# Set GO2RTC_CONFIG_PERSIST=false

# Start container
docker-compose up -d
sleep 10

# Check go2rtc config
cat config/go2rtc/go2rtc.yaml
```

### Verification
- [ ] Environment variables respected
- [ ] Timezone applied correctly
- [ ] GO2RTC_CONFIG_PERSIST behavior correct

## Test 9: go2rtc Integration

### Steps
```bash
# Check go2rtc is running
docker exec lightnvr ps aux | grep go2rtc

# Test go2rtc API
curl http://localhost:1984/api/config

# Check go2rtc logs
docker exec lightnvr cat /var/log/lightnvr/go2rtc.log 2>/dev/null || \
  docker logs lightnvr 2>&1 | grep go2rtc
```

### Verification
- [ ] go2rtc process running
- [ ] go2rtc API responding
- [ ] go2rtc config loaded correctly
- [ ] WebRTC configuration present
- [ ] STUN servers configured

## Test 10: Volume Mount Scenarios

### Test 10a: Empty Volumes
```bash
rm -rf config/ data/
mkdir -p config data
docker-compose up -d
sleep 10
```

**Verification:**
- [ ] Directories populated automatically
- [ ] Default configs created

### Test 10b: Existing Config, No Data
```bash
rm -rf data/
mkdir -p data
docker-compose restart
sleep 10
```

**Verification:**
- [ ] Existing config preserved
- [ ] New database created
- [ ] Web UI works

### Test 10c: Existing Data, No Config
```bash
rm -rf config/
mkdir -p config
docker-compose restart
sleep 10
```

**Verification:**
- [ ] New config created
- [ ] Existing database preserved
- [ ] Streams still present

## Test 11: Upgrade Scenario

### Steps
```bash
# Simulate old version with data
mkdir -p config data/database data/recordings
echo "existing data" > data/database/lightnvr.db

# Pull latest image
docker-compose pull

# Start with new image
docker-compose up -d
sleep 10
```

### Verification
- [ ] Existing database preserved
- [ ] New config created if missing
- [ ] Web assets updated
- [ ] No data loss

## Test 12: Multi-Container Scenario

### Steps
```bash
# Create second instance
mkdir -p lightnvr-2/{config,data}
cd lightnvr-2

# Create docker-compose.yml with different ports
cat > docker-compose.yml << 'EOF'
version: '3.8'
services:
  lightnvr:
    image: ghcr.io/opensensor/lightnvr:latest
    ports:
      - "8081:8080"
      - "8555:8554"
      - "8556:8555"
      - "8556:8555/udp"
      - "1985:1984"
    volumes:
      - ./config:/etc/lightnvr
      - ./data:/var/lib/lightnvr/data
EOF

docker-compose up -d
sleep 10
```

### Verification
- [ ] Second instance starts
- [ ] No port conflicts
- [ ] Both instances accessible
- [ ] Separate databases
- [ ] Separate configurations

## Test 13: Resource Usage

### Steps
```bash
# Monitor resource usage
docker stats lightnvr --no-stream

# Check disk usage
du -sh config/ data/
```

### Verification
- [ ] Memory usage reasonable (< 500MB idle)
- [ ] CPU usage low when idle
- [ ] Disk usage reasonable
- [ ] No memory leaks over time

## Test 14: Log Output

### Steps
```bash
# Check logs
docker-compose logs

# Check for errors
docker-compose logs | grep -i error
docker-compose logs | grep -i warn
```

### Verification
- [ ] Startup messages clear and informative
- [ ] No unexpected errors
- [ ] Warnings are informative
- [ ] Log levels appropriate

## Test 15: Cleanup and Removal

### Steps
```bash
# Stop and remove
docker-compose down

# Remove volumes
rm -rf config/ data/

# Verify cleanup
docker ps -a | grep lightnvr
```

### Verification
- [ ] Container stopped cleanly
- [ ] No orphaned processes
- [ ] Volumes can be removed
- [ ] Clean removal possible

## Performance Tests

### Test P1: Startup Time
```bash
time docker-compose up -d
```
**Expected:** < 30 seconds

### Test P2: Restart Time
```bash
time docker-compose restart
```
**Expected:** < 10 seconds

### Test P3: Web UI Load Time
```bash
time curl -f http://localhost:8080/
```
**Expected:** < 2 seconds

## Security Tests

### Test S1: Non-Root User
```bash
docker exec lightnvr whoami
```
**Expected:** Should not be root (future enhancement)

### Test S2: File Permissions
```bash
docker exec lightnvr ls -la /etc/lightnvr
docker exec lightnvr ls -la /var/lib/lightnvr/data
```
**Expected:** Appropriate permissions (755 for directories)

### Test S3: Exposed Secrets
```bash
docker inspect lightnvr | grep -i password
```
**Expected:** No hardcoded passwords in environment

## Documentation Tests

### Test D1: README Accuracy
- [ ] All commands in README work
- [ ] Port numbers match
- [ ] Volume paths correct
- [ ] Examples run successfully

### Test D2: Quick Start Guide
- [ ] Can complete quick start in < 5 minutes
- [ ] All steps clear and accurate
- [ ] Default credentials work

### Test D3: Troubleshooting Guide
- [ ] Common issues documented
- [ ] Solutions work
- [ ] Examples accurate

## Final Checklist

- [ ] All tests passed
- [ ] No critical errors in logs
- [ ] Documentation accurate
- [ ] Performance acceptable
- [ ] Security considerations addressed
- [ ] Ready for production use

## Test Results Template

```
Test Date: _______________
Tester: _______________
Docker Version: _______________
Docker Compose Version: _______________
Host OS: _______________

Test Results:
- Fresh Installation: PASS/FAIL
- Configuration Persistence: PASS/FAIL
- Database Persistence: PASS/FAIL
- Web Assets Protection: PASS/FAIL
- Port Accessibility: PASS/FAIL
- Health Check: PASS/FAIL
- Environment Variables: PASS/FAIL
- go2rtc Integration: PASS/FAIL
- Volume Mount Scenarios: PASS/FAIL
- Upgrade Scenario: PASS/FAIL

Overall Result: PASS/FAIL

Notes:
_______________________________________________
_______________________________________________
_______________________________________________
```

## Automated Testing Script

```bash
#!/bin/bash
# docker-test.sh - Automated testing script

set -e

echo "Starting LightNVR Docker Tests..."

# Test 1: Fresh Installation
echo "Test 1: Fresh Installation"
rm -rf config/ data/
docker-compose down -v 2>/dev/null || true
docker-compose up -d
sleep 10
curl -f http://localhost:8080/ || exit 1
echo "✓ Test 1 Passed"

# Test 2: Configuration Persistence
echo "Test 2: Configuration Persistence"
echo "# Test comment" >> config/lightnvr.ini
docker-compose restart
sleep 5
grep "Test comment" config/lightnvr.ini || exit 1
echo "✓ Test 2 Passed"

# Test 3: Database Persistence
echo "Test 3: Database Persistence"
test -f data/database/lightnvr.db || exit 1
docker-compose restart
sleep 5
test -f data/database/lightnvr.db || exit 1
echo "✓ Test 3 Passed"

# Test 4: Port Accessibility
echo "Test 4: Port Accessibility"
curl -f http://localhost:8080/ || exit 1
curl -f http://localhost:1984/api/config || exit 1
echo "✓ Test 4 Passed"

echo "All tests passed! ✓"
```

Save this as `docker-test.sh`, make it executable with `chmod +x docker-test.sh`, and run it to verify all improvements.

