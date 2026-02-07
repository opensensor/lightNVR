# Mongoose Cleanup Checklist

**Date:** 2026-02-07  
**Purpose:** Complete removal of all mongoose references from lightNVR codebase

## 📋 Complete List of Remaining References

### 1. Header Files - Dead Function Declarations

#### `include/web/api_handlers.h`
- [ ] Line 23: Remove `void register_api_handlers(struct mg_mgr *mgr);`
- [ ] Line 83: Remove `void mg_handle_toggle_streaming(struct mg_connection *c, struct mg_http_message *hm);`
- [ ] Line 139: Remove `void mg_handle_get_recordings(struct mg_connection *c, struct mg_http_message *hm);`
- [ ] Line 147: Remove `void mg_handle_get_recording(struct mg_connection *c, struct mg_http_message *hm);`
- [ ] Line 155: Remove `void mg_handle_delete_recording(struct mg_connection *c, struct mg_http_message *hm);`
- [ ] Line 163: Remove `void mg_handle_batch_delete_recordings(struct mg_connection *c, struct mg_http_message *hm);`
- [ ] Line 171: Remove `void mg_handle_batch_delete_progress(struct mg_connection *c, struct mg_http_message *hm);`
- [ ] Line 184: Remove `void mg_handle_play_recording(struct mg_connection *c, struct mg_http_message *hm);`
- [ ] Line 192: Remove `void mg_handle_download_recording(struct mg_connection *c, struct mg_http_message *hm);`
- [ ] Line 200: Remove `void mg_handle_hls_master_playlist(struct mg_connection *c, struct mg_http_message *hm);`
- [ ] Line 208: Remove `void mg_handle_hls_media_playlist(struct mg_connection *c, struct mg_http_message *hm);`
- [ ] Line 216: Remove `void mg_handle_hls_segment(struct mg_connection *c, struct mg_http_message *hm);`
- [ ] Line 267: Remove `void mg_handle_webrtc_offer(struct mg_connection *c, struct mg_http_message *hm);`
- [ ] Line 275: Remove `void mg_handle_webrtc_ice(struct mg_connection *c, struct mg_http_message *hm);`
- [ ] Line 291: Remove `int mg_extract_path_param(struct mg_http_message *hm, const char *prefix, char *param_buf, size_t buf_size);`
- [ ] Line 300: Remove `void mg_send_json_response(struct mg_connection *c, int status_code, const char *json_str);`
- [ ] Line 309: Remove `void mg_send_json_error(struct mg_connection *c, int status_code, const char *error_message);`
- [ ] Line 317: Remove `cJSON* mg_parse_json_body(struct mg_http_message *hm);`
- [ ] Line 334: Remove `void mg_create_error_response(struct mg_connection *c, int status_code, const char *message);`
- [ ] Lines 18-22: Update comment "registers API handlers that work directly with Mongoose's HTTP message format"
- [ ] Lines 45-46, 52-55, etc.: Update comments referencing "Mongoose connection" and "Mongoose HTTP message"

#### `include/web/api_handlers_recordings.h`
- [ ] Line 47: Remove `void serve_mp4_file(struct mg_connection *c, const char *file_path, const char *filename);`
- [ ] Line 52: Remove `void serve_file_for_download(struct mg_connection *c, const char *file_path, const char *filename, off_t file_size);`
- [ ] Line 57: Remove `void serve_direct_download(struct mg_connection *c, uint64_t id, recording_metadata_t *metadata);`
- [ ] Line 62: Remove `void serve_download_file(struct mg_connection *c, const char *file_path, const char *content_type, ...);`
- [ ] Line 82: Remove `void mg_handle_get_recordings_worker(struct mg_connection *c, struct mg_http_message *hm);`
- [ ] Line 89: Remove `void mg_handle_get_recordings(struct mg_connection *c, struct mg_http_message *hm);`
- [ ] Line 96: Remove `void mg_handle_get_recording_worker(struct mg_connection *c, struct mg_http_message *hm);`
- [ ] Line 103: Remove `void mg_handle_get_recording(struct mg_connection *c, struct mg_http_message *hm);`
- [ ] Line 108: Remove `void mg_handle_delete_recording(struct mg_connection *c, struct mg_http_message *hm);`
- [ ] Line 113: Remove `void mg_handle_batch_delete_recordings(struct mg_connection *c, struct mg_http_message *hm);`
- [ ] Line 118: Remove `void mg_handle_play_recording(struct mg_connection *c, struct mg_http_message *hm);`
- [ ] Line 123: Remove `void mg_handle_download_recording(struct mg_connection *c, struct mg_http_message *hm);`
- [ ] Line 128: Remove `void mg_handle_check_recording_file(struct mg_connection *c, struct mg_http_message *hm);`
- [ ] Line 133: Remove `void mg_handle_delete_recording_file(struct mg_connection *c, struct mg_http_message *hm);`
- [ ] Line 75: Remove comment "/* Mongoose-specific handlers */"

#### `include/web/http_server.h`
- [ ] Line 3: Update comment from "HTTP server implementation using Mongoose" to "HTTP server interface"
- [ ] Lines 14-17: Remove forward declarations:
  ```c
  // Forward declaration of Mongoose structures
  struct mg_mgr;
  struct mg_connection;
  struct mg_http_message;
  ```
- [ ] Line 47: Remove from `http_server_t` struct: `struct mg_mgr *mgr;  // Mongoose event manager`
- [ ] Lines 188-189: Remove `int http_server_mg_to_request(struct mg_connection *conn, struct mg_http_message *msg, http_request_t *request);`
- [ ] Lines 197-198: Remove `int http_server_send_response(struct mg_connection *conn, const http_response_t *response);`

### 2. Source Files - Comments Only

#### `src/core/shutdown_coordinator.c`
- [ ] Line ~200: Update comment "The context pointer might be a mg_connection, but we can't safely use it"

### 3. Documentation Files

#### `README.md`
- [ ] Line 519: Replace "**[Mongoose](https://github.com/cesanta/mongoose)** - Embedded web server" with "**[libuv](https://libuv.org/)** + **[llhttp](https://github.com/nodejs/llhttp)** - High-performance HTTP server"

#### `memory-bank/techContext.md`
- [ ] Line 40: Replace "**Mongoose:** (`src/web/mongoose.*`, `include/web/mongoose.*`) - Handles HTTP/WebSocket server logic." with "**libuv + llhttp:** - Handles HTTP server logic with async I/O."

#### `docs/ARCHITECTURE.md`
- [ ] Update references to mongoose_server.c and Mongoose Server Thread

#### `docs/API.md`
- [ ] Update "served by the Mongoose web server" to "served by the libuv HTTP server"

#### `docs/LIBUV_API_HANDLERS_STATUS.md`
- [ ] Mark as outdated or move to archive (migration is complete)

### 4. Build Artifacts (Clean Only)
- [ ] Run `rm -rf build cmake-build-debug` to remove old build artifacts
- [ ] Rebuild to verify no mongoose references in new builds

### 5. Optional - Archive Migration Documentation
- [ ] Move `docs/MONGOOSE_REMOVAL_STATUS.md` to `docs/archive/`
- [ ] Move `docs/LIBUV_LLHTTP_MIGRATION.md` to `docs/archive/`
- [ ] Move `docs/LIBUV_API_HANDLERS_STATUS.md` to `docs/archive/`
- [ ] Move `src/web/README_MULTITHREADING.md` to `docs/archive/`

## ✅ Verification Steps

After completing all checklist items:

1. **Search for mongoose references:**
   ```bash
   grep -r "mongoose\|Mongoose" --include="*.c" --include="*.h" src/ include/
   grep -r "struct mg_" --include="*.c" --include="*.h" src/ include/
   grep -r "mg_handle_\|mg_send_\|mg_parse_" --include="*.c" --include="*.h" src/ include/
   ```
   Expected result: No matches

2. **Build verification:**
   ```bash
   rm -rf build && mkdir build && cd build
   cmake ..
   make
   ```
   Expected result: Clean build with no mongoose references

3. **Runtime verification:**
   ```bash
   ./build/bin/lightnvr --help
   ```
   Expected result: Application runs without errors

4. **Documentation check:**
   ```bash
   grep -r "mongoose\|Mongoose" README.md docs/*.md memory-bank/*.md
   ```
   Expected result: Only in archived migration docs

## 📊 Progress Tracking

- **Total items:** 60+
- **Completed:** 0
- **Remaining:** 60+
- **Estimated time:** 1-2 hours


