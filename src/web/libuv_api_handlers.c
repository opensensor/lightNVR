/**
 * @file libuv_api_handlers.c
 * @brief API handler registration for libuv HTTP server
 *
 * This file registers all API handlers with the libuv HTTP server backend.
 * All handlers use the backend-agnostic http_request_t/http_response_t interface.
 */

#include <stdio.h>
#include <string.h>
#include <sys/stat.h>
#include "web/libuv_server.h"
#include "web/libuv_connection.h"
#include "web/http_server.h"
#include "web/api_handlers.h"
#include "web/api_handlers_health.h"
#include "web/api_handlers_settings.h"
#include "web/api_handlers_system.h"
#include "web/api_handlers_zones.h"
#include "web/api_handlers_motion.h"
#include "web/api_handlers_ptz.h"
#include "web/api_handlers_detection.h"
#include "web/api_handlers_recordings_playback.h"
#include "web/api_handlers_recordings.h"
#include "web/api_handlers_timeline.h"
#include "web/api_handlers_onvif.h"
#include "web/api_handlers_users.h"
#include "core/logger.h"
#include "core/config.h"

// Forward declarations
extern int libuv_serve_file(libuv_connection_t *conn, const char *path,
                            const char *content_type, const char *extra_headers);
extern const char *libuv_get_mime_type(const char *path);

/**
 * @brief Register all API handlers with the libuv server
 * 
 * This function should be called after libuv_server_init() but before http_server_start().
 * It registers all API routes with their corresponding handler functions.
 * 
 * @param server The HTTP server handle
 * @return 0 on success, -1 on error
 */
int register_all_libuv_handlers(http_server_handle_t server) {
    if (!server) {
        log_error("register_all_libuv_handlers: Invalid server handle");
        return -1;
    }

    log_info("Registering API handlers with libuv server");

    // Health API
    http_server_register_handler(server, "/api/health", "GET", handle_get_health);
    http_server_register_handler(server, "/api/health/hls", "GET", handle_get_hls_health);

    // Streams API
    http_server_register_handler(server, "/api/streams", "GET", handle_get_streams);
    http_server_register_handler(server, "/api/streams", "POST", handle_post_stream);
    http_server_register_handler(server, "/api/streams/test", "POST", handle_test_stream);

    // Stream-specific routes (must come before /api/streams/# wildcard)
    // Detection Zones API
    http_server_register_handler(server, "/api/streams/#/zones", "GET", handle_get_zones);
    http_server_register_handler(server, "/api/streams/#/zones", "POST", handle_post_zones);
    http_server_register_handler(server, "/api/streams/#/zones", "DELETE", handle_delete_zones);

    // Stream Retention API
    http_server_register_handler(server, "/api/streams/#/retention", "GET", handle_get_stream_retention);
    http_server_register_handler(server, "/api/streams/#/retention", "PUT", handle_put_stream_retention);

    // Stream Refresh API
    http_server_register_handler(server, "/api/streams/#/refresh", "POST", handle_post_stream_refresh);

    // PTZ API
    http_server_register_handler(server, "/api/streams/#/ptz/capabilities", "GET", handle_ptz_capabilities);
    http_server_register_handler(server, "/api/streams/#/ptz/presets", "GET", handle_ptz_get_presets);
    http_server_register_handler(server, "/api/streams/#/ptz/move", "POST", handle_ptz_move);
    http_server_register_handler(server, "/api/streams/#/ptz/stop", "POST", handle_ptz_stop);
    http_server_register_handler(server, "/api/streams/#/ptz/absolute", "POST", handle_ptz_absolute);
    http_server_register_handler(server, "/api/streams/#/ptz/relative", "POST", handle_ptz_relative);
    http_server_register_handler(server, "/api/streams/#/ptz/home", "POST", handle_ptz_home);
    http_server_register_handler(server, "/api/streams/#/ptz/set-home", "POST", handle_ptz_set_home);
    http_server_register_handler(server, "/api/streams/#/ptz/goto-preset", "POST", handle_ptz_goto_preset);
    http_server_register_handler(server, "/api/streams/#/ptz/preset", "PUT", handle_ptz_set_preset);

    // Stream CRUD (wildcards - must come after specific routes)
    http_server_register_handler(server, "/api/streams/#/full", "GET", handle_get_stream_full);
    http_server_register_handler(server, "/api/streams/#", "GET", handle_get_stream);
    http_server_register_handler(server, "/api/streams/#", "PUT", handle_put_stream);
    http_server_register_handler(server, "/api/streams/#", "DELETE", handle_delete_stream);

    // Settings API
    http_server_register_handler(server, "/api/settings", "GET", handle_get_settings);
    http_server_register_handler(server, "/api/settings", "POST", handle_post_settings);

    // System API
    http_server_register_handler(server, "/api/system", "GET", handle_get_system_info);
    http_server_register_handler(server, "/api/system/info", "GET", handle_get_system_info);
    http_server_register_handler(server, "/api/system/logs", "GET", handle_get_system_logs);
    http_server_register_handler(server, "/api/system/restart", "POST", handle_post_system_restart);
    http_server_register_handler(server, "/api/system/shutdown", "POST", handle_post_system_shutdown);
    http_server_register_handler(server, "/api/system/logs/clear", "POST", handle_post_system_logs_clear);
    http_server_register_handler(server, "/api/system/backup", "POST", handle_post_system_backup);
    http_server_register_handler(server, "/api/system/status", "GET", handle_get_system_status);

    // Detection API
    http_server_register_handler(server, "/api/detection/results/#", "GET", handle_get_detection_results);
    http_server_register_handler(server, "/api/detection/models", "GET", handle_get_detection_models);

    // Motion Recording API
    http_server_register_handler(server, "/api/motion/config/", "GET", handle_get_motion_config);
    http_server_register_handler(server, "/api/motion/config/", "POST", handle_post_motion_config);
    http_server_register_handler(server, "/api/motion/config/", "DELETE", handle_delete_motion_config);
    http_server_register_handler(server, "/api/motion/test/", "POST", handle_test_motion_event);
    http_server_register_handler(server, "/api/motion/stats/", "GET", handle_get_motion_stats);
    http_server_register_handler(server, "/api/motion/recordings/", "GET", handle_get_motion_recordings);
    http_server_register_handler(server, "/api/motion/recordings/", "DELETE", handle_delete_motion_recording);
    http_server_register_handler(server, "/api/motion/cleanup", "POST", handle_post_motion_cleanup);
    http_server_register_handler(server, "/api/motion/storage", "GET", handle_get_motion_storage);

    // Auth API (backend-agnostic handlers)
    http_server_register_handler(server, "/api/auth/login", "POST", handle_auth_login);
    http_server_register_handler(server, "/api/auth/logout", "POST", handle_auth_logout);
    http_server_register_handler(server, "/api/auth/verify", "GET", handle_auth_verify);
    http_server_register_handler(server, "/logout", "GET", handle_auth_logout);  // Simple GET logout route

    // User Management API (backend-agnostic handlers)
    http_server_register_handler(server, "/api/auth/users", "GET", handle_users_list);
    http_server_register_handler(server, "/api/auth/users", "POST", handle_users_create);
    http_server_register_handler(server, "/api/auth/users/#", "GET", handle_users_get);
    http_server_register_handler(server, "/api/auth/users/#", "PUT", handle_users_update);
    http_server_register_handler(server, "/api/auth/users/#", "DELETE", handle_users_delete);
    http_server_register_handler(server, "/api/auth/users/#/api-key", "POST", handle_users_generate_api_key);

    // ONVIF API (backend-agnostic handlers)
    http_server_register_handler(server, "/api/onvif/discovery/status", "GET", handle_get_onvif_discovery_status);
    http_server_register_handler(server, "/api/onvif/devices", "GET", handle_get_discovered_onvif_devices);
    http_server_register_handler(server, "/api/onvif/discovery/discover", "POST", handle_post_discover_onvif_devices);
    http_server_register_handler(server, "/api/onvif/device/profiles", "GET", handle_get_onvif_device_profiles);
    http_server_register_handler(server, "/api/onvif/device/add", "POST", handle_post_add_onvif_device_as_stream);
    http_server_register_handler(server, "/api/onvif/device/test", "POST", handle_post_test_onvif_connection);

    // Recordings API (backend-agnostic handlers)
    // Note: More specific routes must come before wildcard routes
    http_server_register_handler(server, "/api/recordings/play/#", "GET", handle_recordings_playback);
    http_server_register_handler(server, "/api/recordings/download/#", "GET", handle_recordings_download);
    http_server_register_handler(server, "/api/recordings/files/check", "GET", handle_check_recording_file);
    http_server_register_handler(server, "/api/recordings/files", "DELETE", handle_delete_recording_file);
    http_server_register_handler(server, "/api/recordings/batch-delete/progress/#", "GET", handle_batch_delete_progress);
    http_server_register_handler(server, "/api/recordings/batch-delete", "POST", handle_batch_delete_recordings);
    http_server_register_handler(server, "/api/recordings/protected", "GET", handle_get_protected_recordings);
    http_server_register_handler(server, "/api/recordings/batch-protect", "POST", handle_batch_protect_recordings);
    http_server_register_handler(server, "/api/recordings/sync", "POST", handle_post_recordings_sync);
    http_server_register_handler(server, "/api/recordings/#/protect", "PUT", handle_put_recording_protect);
    http_server_register_handler(server, "/api/recordings/#/retention", "PUT", handle_put_recording_retention);
    http_server_register_handler(server, "/api/recordings/#", "GET", handle_get_recording);
    http_server_register_handler(server, "/api/recordings/#", "DELETE", handle_delete_recording);
    http_server_register_handler(server, "/api/recordings", "GET", handle_get_recordings);

    // Timeline API (backend-agnostic handlers)
    http_server_register_handler(server, "/api/timeline/segments", "GET", handle_get_timeline_segments);
    http_server_register_handler(server, "/api/timeline/manifest", "GET", handle_timeline_manifest);
    http_server_register_handler(server, "/api/timeline/play", "GET", handle_timeline_playback);

    // HLS Streaming (backend-agnostic handler)
    http_server_register_handler(server, "/hls/", "GET", handle_direct_hls_request);

    log_info("Successfully registered %d API handlers", 74);  // Updated: added HLS handler

    return 0;
}

// Root path handler removed - static file serving will handle "/" by serving index.html

/**
 * @brief Handler for serving static files from web_root
 */
static void handle_static_file(const http_request_t *req, http_response_t *res) {
    // Get the server from user_data
    libuv_server_t *server = (libuv_server_t *)req->user_data;
    if (!server) {
        log_error("handle_static_file: No server in request user_data");
        http_response_set_json_error(res, 500, "Internal server error");
        return;
    }

    // Build file path
    char file_path[1024];
    snprintf(file_path, sizeof(file_path), "%s%s",
             server->config.web_root, req->path);

    // Check if file exists
    struct stat st;
    if (stat(file_path, &st) != 0) {
        log_debug("Static file not found: %s", file_path);
        http_response_set_json_error(res, 404, "File not found");
        return;
    }

    // If it's a directory, try to serve index.html
    if (S_ISDIR(st.st_mode)) {
        snprintf(file_path, sizeof(file_path), "%s%s/index.html",
                 server->config.web_root, req->path);
        if (stat(file_path, &st) != 0) {
            log_debug("Directory index not found: %s", file_path);
            http_response_set_json_error(res, 404, "Directory index not found");
            return;
        }
    }

    // Get MIME type
    const char *mime_type = libuv_get_mime_type(file_path);

    // Get connection from response user_data (this is a bit of a hack)
    // We need to refactor this to pass the connection properly
    // For now, we'll just set an error
    log_error("handle_static_file: Static file serving not yet fully implemented");
    http_response_set_json_error(res, 501, "Static file serving not yet implemented");
}

/**
 * @brief Register static file handler for serving web assets
 *
 * This should be called after registering API handlers to ensure API routes
 * take precedence over static file serving.
 *
 * Note: Static file serving is handled automatically in libuv_connection.c
 * when no handler matches. This function is kept for future extensions.
 *
 * @param server The HTTP server handle
 * @return 0 on success, -1 on error
 */
int register_static_file_handler(http_server_handle_t server) {
    if (!server) {
        log_error("register_static_file_handler: Invalid server handle");
        return -1;
    }

    log_info("Static file serving enabled (handled by default fallback)");

    // Static files are served automatically when no handler matches
    // See libuv_connection.c on_message_complete() for implementation

    return 0;
}

