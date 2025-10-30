# Docker Volume Migration Guide

## Issue #66: Database Lost on Container Restart

This guide helps existing Docker users migrate to the new volume structure that properly persists the database and all data.

## Problem

Previous versions of the `docker-compose.yml` only mounted the recordings directory, causing the database to be lost on container restart. This resulted in:
- Loss of all stream configurations
- Loss of all user settings
- Loss of recording metadata
- Need to reconfigure everything after each restart

## Solution

The updated configuration uses a unified data volume that includes:
- Database files
- Recordings (HLS and MP4)
- Models
- Swap files
- All other persistent data

## Migration Steps

### For Docker Compose Users

1. **Stop the current container:**
   ```bash
   docker-compose down
   ```

2. **Backup your existing data (recommended):**
   ```bash
   # Backup config
   cp -r ./config ./config.backup
   
   # Backup recordings if they exist
   if [ -d "./recordings" ]; then
     cp -r ./recordings ./recordings.backup
   fi
   ```

3. **Create the new data directory structure:**
   ```bash
   mkdir -p ./data/recordings
   mkdir -p ./data/database
   mkdir -p ./data/models
   ```

4. **Copy the database from the container (if it still exists):**
   ```bash
   # Start the old container temporarily
   docker-compose up -d
   
   # Copy the database
   docker cp lightnvr:/var/lib/lightnvr/lightnvr.db ./data/database/
   
   # Stop the container
   docker-compose down
   ```

5. **Move existing recordings to the new structure:**
   ```bash
   if [ -d "./recordings" ]; then
     # Move HLS recordings
     if [ -d "./recordings/hls" ]; then
       mv ./recordings/hls/* ./data/recordings/ 2>/dev/null || true
     fi
     
     # Move MP4 recordings
     if [ -d "./recordings/mp4" ]; then
       mv ./recordings/mp4 ./data/recordings/ 2>/dev/null || true
     fi
     
     # Move any other recordings
     mv ./recordings/* ./data/recordings/ 2>/dev/null || true
   fi
   ```

6. **Update your configuration file:**
   ```bash
   # Edit config/lightnvr.ini and update the paths:
   nano config/lightnvr.ini
   ```
   
   Update these sections:
   ```ini
   [storage]
   path = /var/lib/lightnvr/data/recordings
   mp4_path = /var/lib/lightnvr/data/recordings/mp4
   
   [database]
   path = /var/lib/lightnvr/data/database/lightnvr.db
   
   [models]
   path = /var/lib/lightnvr/data/models
   
   [memory]
   swap_file = /var/lib/lightnvr/data/swap
   ```

7. **Pull the latest changes:**
   ```bash
   git pull origin main
   ```

8. **Start the container with the new configuration:**
   ```bash
   docker-compose up -d
   ```

9. **Verify the migration:**
   ```bash
   # Check logs
   docker-compose logs -f
   
   # Verify database exists
   ls -la ./data/database/
   
   # Access the web interface
   # http://localhost:8080
   ```

10. **Clean up old directories (after verifying everything works):**
    ```bash
    # Remove old recordings directory
    rm -rf ./recordings
    
    # Remove backups (optional)
    rm -rf ./config.backup
    rm -rf ./recordings.backup
    ```

### For Docker Run Users

1. **Stop the current container:**
   ```bash
   docker stop lightnvr
   docker rm lightnvr
   ```

2. **Backup your existing data:**
   ```bash
   # Backup config
   cp -r /path/to/config /path/to/config.backup
   
   # Backup recordings
   cp -r /path/to/recordings /path/to/recordings.backup
   ```

3. **Create the new data directory:**
   ```bash
   mkdir -p /path/to/data/recordings
   mkdir -p /path/to/data/database
   mkdir -p /path/to/data/models
   ```

4. **Move existing recordings:**
   ```bash
   mv /path/to/recordings/* /path/to/data/recordings/
   ```

5. **Update configuration file:**
   Edit `/path/to/config/lightnvr.ini` with the paths shown above.

6. **Start the new container:**
   ```bash
   docker run -d \
     --name lightnvr \
     --restart unless-stopped \
     -p 8080:8080 \
     -p 1984:1984 \
     -v /path/to/config:/etc/lightnvr \
     -v /path/to/data:/var/lib/lightnvr/data \
     ghcr.io/opensensor/lightnvr:latest
   ```

## Verification

After migration, verify that:

1. **Database is persisted:**
   ```bash
   # Check database file exists
   ls -la ./data/database/lightnvr.db
   
   # Restart container and verify data persists
   docker-compose restart
   # or
   docker restart lightnvr
   
   # Check that streams and settings are still there
   ```

2. **Recordings are accessible:**
   - Access the web interface
   - Navigate to the recordings page
   - Verify old recordings are visible

3. **New recordings work:**
   - Add a stream
   - Verify recordings are being saved to `./data/recordings/`

## Troubleshooting

### Database file not found

If the database file doesn't exist after migration:
```bash
# The application will create a new database automatically
# You'll need to reconfigure your streams
```

### Permission issues

If you encounter permission errors:
```bash
# Fix permissions on the data directory
chmod -R 755 ./data
```

### Recordings not visible

If old recordings aren't showing up:
```bash
# Check the recordings directory structure
ls -la ./data/recordings/

# Verify paths in config
cat config/lightnvr.ini | grep path
```

## New Configuration Options

The updated configuration also adds support for MP4 recording settings that were previously not parsed:

```ini
[storage]
; Enable direct MP4 recording alongside HLS
record_mp4_directly = false

; Path for MP4 recordings
mp4_path = /var/lib/lightnvr/data/recordings/mp4

; Duration of each MP4 segment in seconds (default: 900 = 15 minutes)
mp4_segment_duration = 900

; Number of days to keep MP4 recordings
mp4_retention_days = 30
```

These settings are now properly read from the configuration file.

## Support

If you encounter issues during migration, please:
1. Check the container logs: `docker-compose logs -f` or `docker logs lightnvr`
2. Verify file permissions on the data directory
3. Open an issue on GitHub with details about the error

## Related Issues

- Issue #66: Database lost on container restart

