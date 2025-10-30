# Docker Entrypoint Script Fix

## Issue

The container was stuck in a restart loop with the following error in the logs:

```
[INFO] Copying default models...
cp: cannot stat '/usr/share/lightnvr/models/*': No such file or directory
```

The container would exit with code 1 and continuously restart.

## Root Cause

The entrypoint script (`docker-entrypoint.sh`) was attempting to copy models from `/usr/share/lightnvr/models/*` without properly checking if:
1. The directory exists
2. The directory contains any files

The original code:
```bash
# Set up models if needed
if [ -d /usr/share/lightnvr/models ] && [ -z "$(ls -A /var/lib/lightnvr/data/models 2>/dev/null)" ]; then
    log_info "Copying default models..."
    cp -r /usr/share/lightnvr/models/* /var/lib/lightnvr/data/models/
    log_info "Models copied successfully"
fi
```

**Problem:** The `cp` command with a wildcard (`*`) fails when the source directory is empty, even if the directory exists. This causes the script to exit with a non-zero status, triggering the container restart.

## The Fix

Updated the entrypoint script to check if the source directory has files before attempting to copy:

```bash
# Set up models if needed
if [ -d /usr/share/lightnvr/models ] && [ -n "$(ls -A /usr/share/lightnvr/models 2>/dev/null)" ]; then
    if [ -z "$(ls -A /var/lib/lightnvr/data/models 2>/dev/null)" ]; then
        log_info "Copying default models..."
        cp -r /usr/share/lightnvr/models/* /var/lib/lightnvr/data/models/
        log_info "Models copied successfully"
    fi
fi
```

**Changes:**
1. Added check: `[ -n "$(ls -A /usr/share/lightnvr/models 2>/dev/null)" ]` - ensures source directory has files
2. Nested the destination check inside - only checks destination if source has files
3. No error message if models directory is empty - this is expected behavior

## Logic Flow

### Before (Broken)
```
1. Check if /usr/share/lightnvr/models directory exists ✓
2. Check if destination is empty ✓
3. Try to copy /usr/share/lightnvr/models/* ✗ (fails if source is empty)
4. Container exits with code 1
5. Docker restarts container
6. Loop continues...
```

### After (Fixed)
```
1. Check if /usr/share/lightnvr/models directory exists ✓
2. Check if source directory has files ✓ (empty, so skip)
3. Skip copy operation (no error)
4. Continue with startup
5. Container runs successfully ✓
```

## Why Models Directory is Empty

The models directory is created in the Dockerfile but intentionally left empty:

```dockerfile
RUN mkdir -p \
    /usr/share/lightnvr/web-template \
    /usr/share/lightnvr/models \
    /etc/lightnvr \
    ...
```

**Reasons:**
1. **Optional Feature** - Object detection models are optional and large
2. **User Choice** - Users can add their own models if needed
3. **Image Size** - Keeps the Docker image smaller
4. **Flexibility** - Different users may want different models

## Expected Behavior

### Scenario 1: No Models (Default)
```
Container starts
    ↓
Entrypoint checks for models
    ↓
Source directory empty, skip copy
    ↓
Container continues startup ✓
```

### Scenario 2: Models Added to Image
```
Container starts
    ↓
Entrypoint checks for models
    ↓
Source has files, destination empty
    ↓
Copy models to /var/lib/lightnvr/data/models/
    ↓
Container continues startup ✓
```

### Scenario 3: Models Already Exist
```
Container starts
    ↓
Entrypoint checks for models
    ↓
Source has files, destination has files
    ↓
Skip copy (preserve existing)
    ↓
Container continues startup ✓
```

## Testing

### Test 1: Fresh Start (No Models)
```bash
# Clean start
docker-compose down -v
rm -rf config/ data/

# Start container
docker-compose up -d

# Check logs - should NOT see error
docker-compose logs | grep models
# Expected: No "cp: cannot stat" error

# Verify container is running
docker ps | grep lightnvr
# Expected: Container running, not restarting
```

### Test 2: With Models
```bash
# Build custom image with models
cat > Dockerfile.custom << 'EOF'
FROM ghcr.io/opensensor/lightnvr:latest
COPY my-models/* /usr/share/lightnvr/models/
EOF

docker build -f Dockerfile.custom -t lightnvr:with-models .
docker run -d --name lightnvr-test lightnvr:with-models

# Check logs
docker logs lightnvr-test | grep models
# Expected: "Copying default models..." and "Models copied successfully"

# Verify models copied
docker exec lightnvr-test ls -la /var/lib/lightnvr/data/models/
# Expected: Model files present
```

### Test 3: Container Restart
```bash
# Start container
docker-compose up -d

# Restart multiple times
docker-compose restart
docker-compose restart
docker-compose restart

# Check logs
docker-compose logs | grep -c "cp: cannot stat"
# Expected: 0 (no errors)

# Verify container is healthy
docker ps | grep lightnvr
# Expected: Container running, not restarting
```

## Files Modified

- `docker-entrypoint.sh` (lines 158-165)

## Related Issues

This fix also prevents similar issues with other optional components:

1. **Web Assets** - Already has proper checks
2. **Configuration Files** - Already has proper checks
3. **go2rtc Config** - Already has proper checks
4. **Models** - Now fixed ✓

## Best Practices Applied

1. **Defensive Programming** - Check all conditions before operations
2. **Graceful Degradation** - Missing optional components don't break startup
3. **Silent Success** - Don't log errors for expected conditions (empty models)
4. **Idempotent Operations** - Can run multiple times safely

## Impact

### Before Fix
- ❌ Container stuck in restart loop
- ❌ Logs filled with error messages
- ❌ Container never becomes healthy
- ❌ Users frustrated and confused

### After Fix
- ✅ Container starts successfully
- ✅ No error messages for expected conditions
- ✅ Container becomes healthy
- ✅ Users have working system immediately

## Future Enhancements

### Option 1: Download Models on Demand
```bash
# In entrypoint script
if [ "$LIGHTNVR_DOWNLOAD_MODELS" = "true" ]; then
    log_info "Downloading default models..."
    wget -O /var/lib/lightnvr/data/models/yolov5s.pt \
        https://github.com/ultralytics/yolov5/releases/download/v6.0/yolov5s.pt
fi
```

### Option 2: Model Management Command
```bash
# Add to lightnvr CLI
lightnvr models list
lightnvr models download yolov5s
lightnvr models remove yolov5s
```

### Option 3: Web UI Model Management
- Upload models through web interface
- Download from model zoo
- Enable/disable models

## Conclusion

The fix ensures the container starts successfully even when the models directory is empty (which is the default and expected state). This follows the principle of graceful degradation - optional features don't break core functionality.

The container now:
- ✅ Starts successfully without models
- ✅ Copies models if they exist in the image
- ✅ Preserves existing models on restart
- ✅ Provides clear logs about what's happening
- ✅ Never enters a restart loop

## Status

✅ **Fixed** - Container now starts successfully and runs without errors.

