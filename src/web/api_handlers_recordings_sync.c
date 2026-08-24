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
#define LOG_COMPONENT "RecordingsAPI"
#include "core/logger.h"
#include "core/config.h"
#include "database/db_recordings_sync.h"
#include "web/audit_log.h"

/**
 * Handler for POST /api/recordings/sync
 * 
 * Triggers a manual synchronization of recording file sizes with the database
 */
void handle_post_recordings_sync(const http_request_t *req, http_response_t *res) {
    log_info("Processing POST /api/recordings/sync request");
    
    user_t user;
    authorization_evaluation_t evaluation;
    if (!httpd_authorize_action(req, res, AUTHZ_STORAGE_CONFIGURE, NULL,
                                &user, &evaluation)) return;
    
    // Trigger sync
    log_info("Triggering recording file size sync");
    int result = force_recording_sync();
    
    if (result < 0) {
        log_error("Recording sync failed");
        http_response_set_json_error(res, 500, "Recording sync failed");
        audit_log_operation(req, &user, "storage.configure", "recordings",
                            NULL, "recordings.sync", "error", NULL);
        return;
    }
    
    // Create response
    char response[256];
    snprintf(response, sizeof(response), 
            "{\"success\":true,\"message\":\"Recording sync complete\",\"updated\":%d}",
            result);
    
    http_response_set_json(res, 200, response);
    cJSON *context = cJSON_CreateObject();
    if (context) cJSON_AddNumberToObject(context, "updated", result);
    audit_log_operation(req, &user, "storage.configure", "recordings", NULL,
                        "recordings.sync", "success", context);
    cJSON_Delete(context);
    
    log_info("Recording sync complete: %d recordings updated", result);
}
