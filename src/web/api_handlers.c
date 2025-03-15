#define _XOPEN_SOURCE
#define _GNU_SOURCE

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "web/api_handlers.h"
#include "web/web_server.h"
#include "core/logger.h"
#include "web/api_handlers_detection_results.h"

/**
 * Register fixed API handlers to ensure proper URL handling
 */
void register_api_handlers(void) {
    // Register settings API handlers
    register_request_handler("/api/settings", "POST", handle_post_settings);
    register_request_handler("/api/settings", "GET", handle_get_settings);

    // Register stream API handlers
    register_request_handler("/api/streams", "GET", handle_get_streams);
    register_request_handler("/api/streams", "POST", handle_post_stream);
    register_request_handler("/api/streams/test", "POST", handle_test_stream);

    // Register improved stream-specific API handlers for individual streams
    // Use a more specific pattern that matches the exact pattern of IDs
    register_request_handler("/api/streams/*", "GET", handle_get_stream);
    register_request_handler("/api/streams/*", "PUT", handle_put_stream);
    register_request_handler("/api/streams/*", "DELETE", handle_delete_stream);

    // Register system API handlers
    register_request_handler("/api/system/info", "GET", handle_get_system_info);
    register_request_handler("/api/system/logs", "GET", handle_get_system_logs);
    register_request_handler("/api/system/restart", "POST", handle_post_system_restart);
    register_request_handler("/api/system/shutdown", "POST", handle_post_system_shutdown);
    register_request_handler("/api/system/logs/clear", "POST", handle_post_system_clear_logs);
    register_request_handler("/api/system/backup", "POST", handle_post_system_backup);
    register_request_handler("/api/system/status", "GET", handle_get_system_status);

    // Register recording API handlers
    // IMPORTANT: Register more specific routes first to avoid conflicts
    register_request_handler("/api/recordings/download/*", "GET", handle_download_recording);
    register_request_handler("/api/recordings", "GET", handle_get_recordings);
    
    // These must come last as they're more general patterns
    register_request_handler("/api/recordings/*", "GET", handle_get_recording);
    register_request_handler("/api/recordings/*", "DELETE", handle_delete_recording);

    // Register streaming API handlers
    register_streaming_api_handlers();
    
    // Register detection results API handlers
    register_detection_results_api_handlers();

    log_info("API handlers registered with improved URL handling and route priority");
}
