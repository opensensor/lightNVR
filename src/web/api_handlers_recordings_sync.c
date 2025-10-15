/**
 * API Handler for Recording Sync
 * 
 * This module provides an API endpoint to manually trigger recording file size synchronization
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "web/api_handlers.h"
#include "web/mongoose_adapter.h"
#include "web/mongoose_server_auth.h"
#include "web/http_server.h"
#include "core/logger.h"
#include "mongoose.h"
#include "database/db_recordings_sync.h"

/**
 * Handler for POST /api/recordings/sync
 * 
 * Triggers a manual synchronization of recording file sizes with the database
 */
void mg_handle_post_recordings_sync(struct mg_connection *c, struct mg_http_message *hm) {
    log_info("Processing POST /api/recordings/sync request");
    
    // Check authentication
    http_server_t *server = (http_server_t *)c->fn_data;
    if (server && server->config.auth_enabled) {
        if (mongoose_server_basic_auth_check(hm, server) != 0) {
            log_error("Authentication failed for recordings sync request");
            mg_send_json_error(c, 401, "Unauthorized");
            return;
        }
    }
    
    // Trigger sync
    log_info("Triggering recording file size sync");
    int result = force_recording_sync();
    
    if (result < 0) {
        log_error("Recording sync failed");
        mg_send_json_error(c, 500, "Recording sync failed");
        return;
    }
    
    // Create response
    char response[256];
    snprintf(response, sizeof(response), 
            "{\"success\":true,\"message\":\"Recording sync complete\",\"updated\":%d}",
            result);
    
    mg_send_json_response(c, 200, response);
    
    log_info("Recording sync complete: %d recordings updated", result);
}

