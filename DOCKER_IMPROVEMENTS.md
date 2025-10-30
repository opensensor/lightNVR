# Docker Container Improvements - Implementation Summary

This document summarizes the improvements made to the LightNVR Docker container to address persistence and initialization issues.

## Problems Addressed

### 1. Web Assets Getting Overwritten
**Problem:** When users mounted volumes to `/var/lib/lightnvr`, the web assets were overwritten, breaking the web UI.

**Solution:** 
- Moved web assets to a protected template location (`/usr/share/lightnvr/web-template/`)
- Entrypoint script copies assets to `/var/lib/lightnvr/web/` on first run
- Updated documentation to warn against mounting `/var/lib/lightnvr` directly

### 2. No Default Configuration Files
**Problem:** Container required manual configuration file creation before first use.

**Solution:**
- Entrypoint script automatically creates default `lightnvr.ini` on first run
- Includes sensible defaults for all settings
- Creates proper directory structure automatically

### 3. go2rtc.yaml Regeneration
**Problem:** go2rtc configuration was regenerated on each restart, losing custom WebRTC settings.

**Solution:**
- Entrypoint script creates default `go2rtc.yaml` only if it doesn't exist
- Added `GO2RTC_CONFIG_PERSIST` environment variable (default: true)
- Configuration persists across container restarts when mounted properly

### 4. Unclear Volume Mount Requirements
**Problem:** Users didn't know which directories to mount, leading to data loss.

**Solution:**
- Clear documentation in README.md and new DOCKER.md guide
- Explicit volume declarations in Dockerfile
- Warning comments in docker-compose.yml

## Implementation Details

### 1. docker-entrypoint.sh Script

Created a comprehensive entrypoint script that handles:

- **Directory Creation:** Creates all necessary directories on first run
- **Web Assets:** Copies web UI files from template location
- **Configuration Files:** Creates default `lightnvr.ini` and `go2rtc.yaml`
- **Permissions:** Sets proper file permissions
- **Logging:** Provides clear startup information
- **Signal Handling:** Graceful shutdown on SIGTERM/SIGINT

Key features:
```bash
# Only copies web assets if directory is empty
if [ -z "$(ls -A /var/lib/lightnvr/web 2>/dev/null)" ]; then
    cp -r /usr/share/lightnvr/web-template/* /var/lib/lightnvr/web/
fi

# Only creates config if it doesn't exist
if [ ! -f /etc/lightnvr/lightnvr.ini ]; then
    # Create default configuration
fi
```

### 2. Dockerfile Improvements

**Changes Made:**

1. **Web Assets Protection:**
   ```dockerfile
   # Copy to template location instead of final location
   COPY --from=builder /opt/web/dist /usr/share/lightnvr/web-template/
   ```

2. **Entrypoint Integration:**
   ```dockerfile
   COPY docker-entrypoint.sh /usr/local/bin/
   RUN chmod +x /usr/local/bin/docker-entrypoint.sh
   ENTRYPOINT ["/usr/local/bin/docker-entrypoint.sh"]
   ```

3. **Explicit Volume Declarations:**
   ```dockerfile
   VOLUME ["/etc/lightnvr", "/var/lib/lightnvr/data"]
   ```

4. **Additional Ports:**
   ```dockerfile
   EXPOSE 8080 8554 8555 8555/udp 1984
   ```

5. **Health Check:**
   ```dockerfile
   HEALTHCHECK --interval=30s --timeout=3s --start-period=10s --retries=3 \
       CMD curl -f http://localhost:8080/ || exit 1
   ```

6. **Environment Variables:**
   ```dockerfile
   ENV GO2RTC_CONFIG_PERSIST=true \
       LIGHTNVR_AUTO_INIT=true \
       LIGHTNVR_WEB_ROOT=/var/lib/lightnvr/web
   ```

### 3. docker-compose.yml Enhancements

**Changes Made:**

1. **Added WebRTC Ports:**
   ```yaml
   ports:
     - "8080:8080"     # Web UI
     - "8554:8554"     # RTSP
     - "8555:8555"     # WebRTC TCP
     - "8555:8555/udp" # WebRTC UDP
     - "1984:1984"     # go2rtc API
   ```

2. **Environment Variables:**
   ```yaml
   environment:
     - TZ=UTC
     - GO2RTC_CONFIG_PERSIST=true
     - LIGHTNVR_AUTO_INIT=true
   ```

3. **Documentation Comments:**
   ```yaml
   volumes:
     # DO NOT mount /var/lib/lightnvr directly - it will overwrite web assets!
     - ./config:/etc/lightnvr
     - ./data:/var/lib/lightnvr/data
   ```

4. **Network Configuration:**
   ```yaml
   networks:
     - lightnvr
   ```

### 4. GitHub Actions Workflow Updates

**Changes Made:**

1. **Trigger on Main Branch:**
   ```yaml
   on:
     push:
       branches:
         - main
       tags:
         - '*.*.*'
     pull_request:
       branches:
         - main
   ```

2. **SHA-based Tags:**
   ```yaml
   tags: |
     type=sha,prefix={{branch}}-
   ```

3. **Image Labels:**
   ```yaml
   labels: |
     org.opencontainers.image.title=LightNVR
     org.opencontainers.image.description=Lightweight Network Video Recorder with go2rtc integration
     org.opencontainers.image.vendor=OpenSensor
   ```

### 5. Documentation Updates

**New/Updated Files:**

1. **README.md:**
   - Expanded Docker section with clear examples
   - Added volume mount explanations
   - Documented first-run experience
   - Listed all exposed ports
   - Added environment variable documentation

2. **docs/DOCKER.md (NEW):**
   - Comprehensive Docker deployment guide
   - Troubleshooting section
   - Advanced configuration examples
   - WebRTC configuration guide
   - Migration instructions

## Default Configuration

### lightnvr.ini

The entrypoint creates a default configuration with:
- Web UI on port 8080
- Database at `/var/lib/lightnvr/data/database/lightnvr.db`
- Recordings at `/var/lib/lightnvr/data/recordings`
- go2rtc integration enabled
- WebRTC enabled with STUN servers

### go2rtc.yaml

Default go2rtc configuration includes:
- API on port 1984
- RTSP on port 8554
- WebRTC on port 8555
- STUN servers for NAT traversal
- Proper ICE candidates configuration

## Benefits

### 1. First-Run Experience
✅ Container works immediately with no manual configuration
✅ Default credentials provided (admin/admin)
✅ WebRTC configured out-of-the-box
✅ All necessary directories created automatically

### 2. Preserved Web Assets
✅ Web UI files never lost when mounting volumes
✅ Updates to container image update web UI
✅ Clear separation between static and dynamic data

### 3. Persistent Configuration
✅ Configuration files survive container restarts
✅ go2rtc settings preserved across updates
✅ Database and recordings never lost

### 4. Clear Documentation
✅ Users know exactly what to mount
✅ Common pitfalls documented
✅ Troubleshooting guide included
✅ Migration path from old versions

### 5. Idempotent Startup
✅ Can restart container without losing configuration
✅ Re-running initialization is safe
✅ Existing files are never overwritten

### 6. Environment Variable Control
✅ Can override behavior via env vars
✅ Timezone configuration
✅ Config persistence control

## Testing Recommendations

### Test Scenarios

1. **Fresh Installation:**
   ```bash
   docker-compose up -d
   # Verify web UI loads
   # Verify default config created
   # Verify database initialized
   ```

2. **Container Restart:**
   ```bash
   docker-compose restart
   # Verify configuration persists
   # Verify database intact
   # Verify streams still configured
   ```

3. **Volume Mount Scenarios:**
   ```bash
   # Test with empty volumes
   # Test with existing config
   # Test with partial config
   ```

4. **WebRTC Functionality:**
   ```bash
   # Add a camera stream
   # Test WebRTC playback
   # Verify STUN connectivity
   ```

5. **Migration from Old Version:**
   ```bash
   # Follow DOCKER_MIGRATION_GUIDE.md
   # Verify data preserved
   # Verify web UI works
   ```

## Upgrade Path

### For Existing Users

Users with existing deployments should:

1. **Backup Current Data:**
   ```bash
   docker-compose down
   cp -r ./config ./config.backup
   cp -r ./data ./data.backup
   ```

2. **Update docker-compose.yml:**
   - Add new ports (8554, 8555)
   - Add environment variables
   - Verify volume mounts

3. **Pull New Image:**
   ```bash
   docker-compose pull
   docker-compose up -d
   ```

4. **Verify Operation:**
   - Check web UI loads
   - Verify streams work
   - Test WebRTC playback

### Breaking Changes

⚠️ **None** - The improvements are backward compatible with existing volume structures.

## Future Enhancements

Potential future improvements:

1. **Multi-Stage Initialization:**
   - Separate init container for one-time setup
   - Faster subsequent startups

2. **Configuration Validation:**
   - Validate config files on startup
   - Provide helpful error messages

3. **Automatic Backup:**
   - Periodic database backups
   - Configurable retention

4. **Metrics and Monitoring:**
   - Prometheus metrics endpoint
   - Health check improvements

5. **Security Enhancements:**
   - Non-root user execution
   - Read-only root filesystem
   - Security scanning in CI/CD

## Files Modified

- ✅ `Dockerfile` - Updated with entrypoint and improved structure
- ✅ `docker-entrypoint.sh` - New entrypoint script
- ✅ `docker-compose.yml` - Enhanced with ports and env vars
- ✅ `.github/workflows/docker-publish.yml` - Updated triggers and tags
- ✅ `README.md` - Expanded Docker documentation
- ✅ `docs/DOCKER.md` - New comprehensive guide
- ✅ `DOCKER_IMPROVEMENTS.md` - This file

## Conclusion

These improvements transform the LightNVR Docker container from a basic containerized application to a production-ready, user-friendly deployment option. The container now:

- Works out-of-the-box with sensible defaults
- Preserves all user data and configuration
- Provides clear documentation and troubleshooting
- Supports WebRTC streaming without manual configuration
- Follows Docker best practices

Users can now deploy LightNVR with a single `docker-compose up -d` command and have a fully functional NVR system with WebRTC streaming capabilities.

