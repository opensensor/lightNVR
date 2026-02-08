/**
 * API Handler for Recording Sync
 * 
 * This module provides an API endpoint to manually trigger recording file size synchronization
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "web/api_handlers.h"
#include "web/request_response.h"
#include "web/httpd_utils.h"
#include "core/logger.h"
#include "core/config.h"
#include "database/db_recordings_sync.h"

/**
 * Handler for POST /api/recordings/sync
 * 
 * Triggers a manual synchronization of recording file sizes with the database
 */
void handle_post_recordings_sync(const http_request_t *req, http_response_t *res) {
    log_info("Processing POST /api/recordings/sync request");
    
    // Check authentication
    if (g_config.web_auth_enabled) {
        user_t user;
        if (!httpd_get_authenticated_user(req, &user)) {
            log_error("Authentication failed for recordings sync request");
            http_response_set_json_error(res, 401, "Unauthorized");
            return;
        }
    }
    
    // Trigger sync
    log_info("Triggering recording file size sync");
    int result = force_recording_sync();
    
    if (result < 0) {
        log_error("Recording sync failed");
        http_response_set_json_error(res, 500, "Recording sync failed");
        return;
    }
    
    // Create response
    char response[256];
    snprintf(response, sizeof(response), 
            "{\"success\":true,\"message\":\"Recording sync complete\",\"updated\":%d}",
            result);
    
    http_response_set_json(res, 200, response);
    
    log_info("Recording sync complete: %d recordings updated", result);
}

