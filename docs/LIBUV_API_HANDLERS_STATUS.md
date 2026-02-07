# libuv API Handlers Migration Status

This document tracks the status of API handler migration from Mongoose-specific to backend-agnostic implementations for the libuv HTTP server.

## Overview

The libuv migration requires all API handlers to use the backend-agnostic `http_request_t` and `http_response_t` interfaces instead of Mongoose-specific `mg_connection` and `mg_http_message` types.

## ✅ Completed Handlers (Registered in libuv)

### Health API
- ✅ `GET /api/health` - `handle_get_health`
- ✅ `GET /api/health/hls` - `handle_get_hls_health`

### Streams API
- ✅ `GET /api/streams` - `handle_get_streams`
- ✅ `POST /api/streams` - `handle_post_stream`
- ✅ `POST /api/streams/test` - `handle_test_stream`
- ✅ `GET /api/streams/#/zones` - `handle_get_zones`
- ✅ `POST /api/streams/#/zones` - `handle_post_zones`
- ✅ `DELETE /api/streams/#/zones` - `handle_delete_zones`
- ✅ `GET /api/streams/#/retention` - `handle_get_stream_retention`
- ✅ `PUT /api/streams/#/retention` - `handle_put_stream_retention`
- ✅ `POST /api/streams/#/refresh` - `handle_post_stream_refresh`
- ✅ `GET /api/streams/#/ptz/capabilities` - `handle_ptz_capabilities`
- ✅ `GET /api/streams/#/ptz/presets` - `handle_ptz_get_presets`
- ✅ `POST /api/streams/#/ptz/move` - `handle_ptz_move`
- ✅ `POST /api/streams/#/ptz/stop` - `handle_ptz_stop`
- ✅ `POST /api/streams/#/ptz/absolute` - `handle_ptz_absolute`
- ✅ `POST /api/streams/#/ptz/relative` - `handle_ptz_relative`
- ✅ `POST /api/streams/#/ptz/home` - `handle_ptz_home`
- ✅ `POST /api/streams/#/ptz/set-home` - `handle_ptz_set_home`
- ✅ `POST /api/streams/#/ptz/goto-preset` - `handle_ptz_goto_preset`
- ✅ `PUT /api/streams/#/ptz/preset` - `handle_ptz_set_preset`
- ✅ `GET /api/streams/#/full` - `handle_get_stream_full`
- ✅ `GET /api/streams/#` - `handle_get_stream`
- ✅ `PUT /api/streams/#` - `handle_put_stream`
- ✅ `DELETE /api/streams/#` - `handle_delete_stream`

### Settings API
- ✅ `GET /api/settings` - `handle_get_settings`
- ✅ `POST /api/settings` - `handle_post_settings`

### System API
- ✅ `GET /api/system` - `handle_get_system_info`
- ✅ `GET /api/system/info` - `handle_get_system_info`
- ✅ `GET /api/system/logs` - `handle_get_system_logs`
- ✅ `POST /api/system/restart` - `handle_post_system_restart`
- ✅ `POST /api/system/shutdown` - `handle_post_system_shutdown`
- ✅ `POST /api/system/logs/clear` - `handle_post_system_logs_clear`
- ✅ `POST /api/system/backup` - `handle_post_system_backup`
- ✅ `GET /api/system/status` - `handle_get_system_status`

### Detection API
- ✅ `GET /api/detection/results/` - `handle_get_detection_results`
- ✅ `GET /api/detection/models` - `handle_get_detection_models`

### Motion Recording API
- ✅ `GET /api/motion/config/` - `handle_get_motion_config`
- ✅ `POST /api/motion/config/` - `handle_post_motion_config`
- ✅ `DELETE /api/motion/config/` - `handle_delete_motion_config`
- ✅ `POST /api/motion/test/` - `handle_test_motion_event`
- ✅ `GET /api/motion/stats/` - `handle_get_motion_stats`
- ✅ `GET /api/motion/recordings/` - `handle_get_motion_recordings`
- ✅ `DELETE /api/motion/recordings/` - `handle_delete_motion_recording`
- ✅ `POST /api/motion/cleanup` - `handle_post_motion_cleanup`
- ✅ `GET /api/motion/storage` - `handle_get_motion_storage`

### Recordings API (Partial)
- ✅ `GET /api/recordings/play/#` - `handle_recordings_playback`
- ✅ `GET /api/recordings/download/#` - `handle_recordings_download`
- ✅ `GET /api/recordings/protected` - `handle_get_protected_recordings`
- ✅ `POST /api/recordings/batch-protect` - `handle_batch_protect_recordings`
- ✅ `POST /api/recordings/sync` - `handle_post_recordings_sync`
- ✅ `PUT /api/recordings/#/protect` - `handle_put_recording_protect`
- ✅ `PUT /api/recordings/#/retention` - `handle_put_recording_retention`

**Total Registered: 47 handlers**

## ❌ Missing Handlers (Need to be Ported)

### Recordings API (CRITICAL)
These are **required** for the recordings page to work:

- ❌ `GET /api/recordings` - List all recordings (Mongoose-specific: `mg_handle_get_recordings`)
  - **File**: `src/web/api_handlers_recordings_get.c`
  - **Impact**: Recordings page shows 404
- ❌ `GET /api/recordings/#` - Get single recording (Mongoose-specific: `mg_handle_get_recording`)
- ❌ `DELETE /api/recordings/#` - Delete recording (Mongoose-specific: `mg_handle_delete_recording`)
- ❌ `POST /api/recordings/batch-delete` - Batch delete (Mongoose-specific: `mg_handle_batch_delete_recordings`)
- ❌ `GET /api/recordings/batch-delete/progress/#` - Progress check (Mongoose-specific: `mg_handle_batch_delete_progress`)
- ❌ `GET /api/recordings/files/check` - Check file (Mongoose-specific: `mg_handle_check_recording_file`)
- ❌ `DELETE /api/recordings/files` - Delete file (Mongoose-specific: `mg_handle_delete_recording_file`)

### Auth API (CRITICAL)
These are **required** for login/logout to work:

- ❌ `POST /api/auth/login` - User login (Mongoose-specific: `mg_handle_auth_login`)
  - **File**: `src/web/api_handlers_auth.c`
  - **Impact**: Cannot log in
- ❌ `POST /api/auth/logout` - User logout (Mongoose-specific: `mg_handle_auth_logout`)
  - **Impact**: Cannot log out
- ❌ `GET /logout` - Simple logout (Mongoose-specific: `mg_handle_auth_logout`)
- ❌ `GET /api/auth/verify` - Verify auth (Mongoose-specific: `mg_handle_auth_verify`)
  - **Impact**: Auth verification fails

### User Management API
- ❌ `GET /api/auth/users` - List users (Mongoose-specific: `mg_handle_users_list`)
  - **File**: `src/web/api_handlers_users.c`
- ❌ `GET /api/auth/users/#` - Get user (Mongoose-specific: `mg_handle_users_get`)
- ❌ `POST /api/auth/users` - Create user (Mongoose-specific: `mg_handle_users_create`)
- ❌ `PUT /api/auth/users/#` - Update user (Mongoose-specific: `mg_handle_users_update`)
- ❌ `DELETE /api/auth/users/#` - Delete user (Mongoose-specific: `mg_handle_users_delete`)
- ❌ `POST /api/auth/users/#/api-key` - Generate API key (Mongoose-specific: `mg_handle_users_generate_api_key`)

## Next Steps

1. **Port Recordings List Handler** (`GET /api/recordings`)
   - This is the highest priority as it blocks the recordings page
   - Refactor `mg_handle_get_recordings` to use `http_request_t`/`http_response_t`

2. **Port Auth Handlers** (`login`, `logout`, `verify`)
   - Required for basic authentication to work
   - Refactor handlers in `src/web/api_handlers_auth.c`

3. **Port User Management Handlers**
   - Lower priority but needed for full functionality

4. **Port Remaining Recordings Handlers**
   - Delete, batch operations, file checks

## Notes

- All ported handlers must use `http_request_t` and `http_response_t` types
- Handlers should be declared in appropriate header files (e.g., `include/web/api_handlers.h`)
- Pattern matching with `#` wildcards is now supported in libuv (e.g., `/api/recordings/#`)

