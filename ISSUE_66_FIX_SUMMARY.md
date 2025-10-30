# Issue #66 Fix Summary: Database Lost on Container Restart

## Overview

This document summarizes the comprehensive fix for Issue #66, which addresses the problem of the database being lost on Docker container restart, along with a related configuration parsing bug that was discovered during the investigation.

## Problems Identified

### 1. Database Not Persisted (Primary Issue)
The original `docker-compose.yml` and Dockerfiles only mounted `/var/lib/lightnvr/recordings` as a volume, leaving the database at `/var/lib/lightnvr/lightnvr.db` inside the container's ephemeral filesystem. This caused:
- Complete loss of database on container restart
- Loss of all stream configurations
- Loss of user settings and authentication data
- Loss of recording metadata
- Need to reconfigure everything after each restart

### 2. MP4 Configuration Not Parsed (Secondary Issue)
The configuration parser in `src/core/config.c` was missing code to read MP4-related settings from the INI file:
- `record_mp4_directly` - Enable/disable direct MP4 recording
- `mp4_path` - Path for MP4 recordings storage
- `mp4_segment_duration` - Duration of each MP4 segment
- `mp4_retention_days` - Retention period for MP4 files

These settings existed in the config structure and INI file but were never actually read.

## Solution Implemented

### Files Modified

1. **docker-compose.yml**
   - Changed volume mount from `./recordings:/var/lib/lightnvr/recordings` to `./data:/var/lib/lightnvr/data`
   - Added port 1984 for go2rtc API
   - Added timezone environment variable

2. **Dockerfile**
   - Updated VOLUME declarations from `/var/lib/lightnvr/recordings` to `/var/lib/lightnvr/data`
   - Updated directory creation to use `/var/lib/lightnvr/data` structure
   - Added comments explaining volume purposes

3. **Dockerfile.alpine**
   - Same updates as main Dockerfile for Alpine-based builds

4. **config/lightnvr.ini**
   - Updated all paths to use `/var/lib/lightnvr/data` subdirectories:
     - Database: `/var/lib/lightnvr/data/database/lightnvr.db`
     - Recordings: `/var/lib/lightnvr/data/recordings`
     - MP4 recordings: `/var/lib/lightnvr/data/recordings/mp4`
     - Models: `/var/lib/lightnvr/data/models`
     - Swap: `/var/lib/lightnvr/data/swap`

5. **src/core/config.c**
   - Added parsing for `record_mp4_directly` setting
   - Added parsing for `mp4_path` setting
   - Added parsing for `mp4_segment_duration` setting
   - Added parsing for `mp4_retention_days` setting
   - Added default initialization for MP4 settings
   - Added MP4 settings to config save function

6. **README.md**
   - Added Docker Compose installation instructions
   - Updated Docker run command with new volume structure
   - Added important note about data persistence

7. **docs/INSTALLATION.md**
   - Comprehensive Docker installation guide with both Compose and Run options
   - Updated upgrade instructions for both methods
   - Updated uninstall instructions
   - Added notes about proper volume configuration

8. **DOCKER_MIGRATION_GUIDE.md** (New File)
   - Step-by-step migration guide for existing users
   - Separate instructions for Docker Compose and Docker Run users
   - Backup and verification procedures
   - Troubleshooting section

9. **ISSUE_66_FIX_SUMMARY.md** (This File)
   - Complete documentation of the fix

## New Volume Structure

### Before (Broken)
```
./config -> /etc/lightnvr
./recordings -> /var/lib/lightnvr/recordings
Database: /var/lib/lightnvr/lightnvr.db (NOT PERSISTED!)
```

### After (Fixed)
```
./config -> /etc/lightnvr
./data -> /var/lib/lightnvr/data
  ├── database/
  │   └── lightnvr.db
  ├── recordings/
  │   ├── hls/
  │   └── mp4/
  ├── models/
  └── swap
```

## Benefits

1. **Database Persistence**: Database now survives container restarts
2. **Unified Data Management**: All persistent data in one volume for easier backup
3. **MP4 Configuration**: MP4 recording settings now properly read from config
4. **Better Organization**: Clear separation between config and data
5. **Easier Backups**: Single data directory to backup
6. **Clearer Documentation**: Comprehensive guides for installation and migration

## Impact on Published Docker Images

### GitHub Container Registry (ghcr.io)

The GitHub Actions workflow (`.github/workflows/docker-publish.yml`) will automatically build and publish updated images when:
- A new version tag is pushed (e.g., `1.0.0`)
- Manual workflow dispatch is triggered

**Important**: The next published image will have the updated volume structure. Users pulling the new image will need to:
1. Follow the migration guide in `DOCKER_MIGRATION_GUIDE.md`
2. Update their volume mounts to use `/var/lib/lightnvr/data`
3. Update their configuration files with new paths

### Breaking Change Notice

This is a **breaking change** for existing Docker users. The volume mount point has changed from:
- Old: `/var/lib/lightnvr/recordings`
- New: `/var/lib/lightnvr/data`

**Recommendation**: 
- Bump the version to indicate a breaking change (e.g., 0.12.0 or 1.0.0)
- Add a release note highlighting this change
- Link to the migration guide in the release notes

## Testing Checklist

Before releasing, verify:

- [ ] Docker image builds successfully
- [ ] Database persists across container restarts
- [ ] Recordings are saved to correct location
- [ ] MP4 settings are read from config file
- [ ] Migration from old structure works
- [ ] Fresh installation works
- [ ] All documentation is accurate
- [ ] GitHub Actions workflow builds multi-arch images

## Migration Path for Users

### New Users
Simply use the updated `docker-compose.yml` or follow the new installation instructions.

### Existing Users
Must follow the migration guide in `DOCKER_MIGRATION_GUIDE.md` to:
1. Backup existing data
2. Copy database from container
3. Move recordings to new structure
4. Update configuration paths
5. Start with new volume mounts

## Backward Compatibility

**None** - This is a breaking change. The old volume structure will not work with the new configuration paths.

However, the migration is straightforward and documented in detail.

## Related Configuration

The fix also enables proper parsing of MP4 recording options:

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

These settings were previously ignored but are now functional.

## Recommendations for Release

1. **Version Bump**: Increment to 0.12.0 or 1.0.0 to indicate breaking change
2. **Release Notes**: Include prominent notice about volume structure change
3. **Migration Guide**: Link to `DOCKER_MIGRATION_GUIDE.md` in release notes
4. **Announcement**: Post in discussions/issues about the breaking change
5. **Tag**: Create a git tag before the breaking change for users who need the old version

## Files Changed Summary

- `docker-compose.yml` - Updated volume mounts
- `Dockerfile` - Updated volume declarations and directory structure
- `Dockerfile.alpine` - Updated volume declarations and directory structure
- `config/lightnvr.ini` - Updated all paths to use data directory
- `src/core/config.c` - Added MP4 configuration parsing
- `README.md` - Updated Docker installation instructions
- `docs/INSTALLATION.md` - Comprehensive Docker documentation
- `DOCKER_MIGRATION_GUIDE.md` - New migration guide
- `ISSUE_66_FIX_SUMMARY.md` - This summary document

## Verification Commands

After deployment, users can verify the fix with:

```bash
# Check database exists and persists
ls -la ./data/database/lightnvr.db

# Restart container
docker-compose restart

# Verify database still exists
ls -la ./data/database/lightnvr.db

# Check that streams are still configured
# (Access web interface and verify settings)
```

## Support

For issues during migration or with the new structure:
1. Check `DOCKER_MIGRATION_GUIDE.md`
2. Review container logs: `docker-compose logs -f`
3. Verify file permissions on data directory
4. Open a GitHub issue with details

## Credits

- Issue reported by: @BrianLakstins
- Fix implemented by: AI Assistant (Augment)
- Repository: opensensor/lightNVR

