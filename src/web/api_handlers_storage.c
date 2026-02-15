/**
 * @file api_handlers_storage.c
 * @brief API handlers for storage health and cleanup endpoints
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>

#include <cjson/cJSON.h>
#include "web/api_handlers.h"
#include "web/request_response.h"
#include "web/httpd_utils.h"
#include "storage/storage_manager.h"
#include "core/config.h"
#include "core/logger.h"
#include "database/db_auth.h"

/**
 * @brief Backend-agnostic handler for GET /api/storage/health
 *
 * Returns disk health status, pressure level, free space, and last
 * cleanup statistics from the unified storage controller's cached data.
 */
void handle_get_storage_health(const http_request_t *req, http_response_t *res) {
    log_info("Handling GET /api/storage/health request");

    // Check authentication
    if (g_config.web_auth_enabled) {
        user_t user;
        if (!httpd_get_authenticated_user(req, &user)) {
            log_error("Authentication failed for GET /api/storage/health request");
            http_response_set_json_error(res, 401, "Unauthorized");
            return;
        }
    }

    // Get cached storage health from unified controller
    storage_health_t health;
    get_storage_health(&health);

    // Build JSON response
    cJSON *root = cJSON_CreateObject();
    if (!root) {
        http_response_set_json_error(res, 500, "Failed to create JSON response");
        return;
    }

    // Pressure info
    cJSON_AddStringToObject(root, "pressure_level", disk_pressure_level_str(health.pressure_level));
    cJSON_AddNumberToObject(root, "pressure_level_num", (double)health.pressure_level);
    cJSON_AddNumberToObject(root, "free_space_pct", health.free_space_pct);
    cJSON_AddNumberToObject(root, "free_space_bytes", (double)health.free_space_bytes);
    cJSON_AddNumberToObject(root, "total_space_bytes", (double)health.total_space_bytes);
    cJSON_AddNumberToObject(root, "used_space_bytes", (double)health.used_space_bytes);

    // Timing info
    cJSON_AddNumberToObject(root, "last_check_time", (double)health.last_check_time);
    cJSON_AddNumberToObject(root, "last_cleanup_time", (double)health.last_cleanup_time);
    cJSON_AddNumberToObject(root, "last_deep_time", (double)health.last_deep_time);

    // Last cleanup stats
    cJSON_AddNumberToObject(root, "last_cleanup_deleted", health.last_cleanup_deleted);
    cJSON_AddNumberToObject(root, "last_cleanup_freed", (double)health.last_cleanup_freed);

    char *json_str = cJSON_PrintUnformatted(root);
    cJSON_Delete(root);

    if (!json_str) {
        http_response_set_json_error(res, 500, "Failed to serialize JSON");
        return;
    }

    http_response_set_json(res, 200, json_str);
    free(json_str);
}

/**
 * @brief Backend-agnostic handler for POST /api/storage/cleanup
 *
 * Triggers an immediate cleanup cycle. Accepts optional JSON body:
 *   { "aggressive": true }  - forces aggressive cleanup regardless of pressure level
 */
void handle_post_storage_cleanup(const http_request_t *req, http_response_t *res) {
    log_info("Handling POST /api/storage/cleanup request");

    // Check authentication
    if (g_config.web_auth_enabled) {
        user_t user;
        if (!httpd_get_authenticated_user(req, &user)) {
            log_error("Authentication failed for POST /api/storage/cleanup request");
            http_response_set_json_error(res, 401, "Unauthorized");
            return;
        }
    }

    // Parse optional body for aggressive flag
    bool aggressive = false;
    if (req->body && req->body_len > 0) {
        char body_buf[256];
        size_t copy_len = req->body_len < sizeof(body_buf) - 1 ? req->body_len : sizeof(body_buf) - 1;
        memcpy(body_buf, req->body, copy_len);
        body_buf[copy_len] = '\0';

        cJSON *body_json = cJSON_Parse(body_buf);
        if (body_json) {
            cJSON *agg = cJSON_GetObjectItemCaseSensitive(body_json, "aggressive");
            if (cJSON_IsBool(agg) && cJSON_IsTrue(agg)) {
                aggressive = true;
            }
            cJSON_Delete(body_json);
        }
    }

    // Trigger cleanup via the unified controller (signals the controller thread)
    trigger_storage_cleanup(aggressive);

    // Build response
    cJSON *root = cJSON_CreateObject();
    if (!root) {
        http_response_set_json_error(res, 500, "Failed to create JSON response");
        return;
    }

    cJSON_AddBoolToObject(root, "success", true);
    cJSON_AddBoolToObject(root, "aggressive", aggressive);
    cJSON_AddStringToObject(root, "message", "Cleanup triggered successfully");

    char *json_str = cJSON_PrintUnformatted(root);
    cJSON_Delete(root);

    if (!json_str) {
        http_response_set_json_error(res, 500, "Failed to serialize JSON");
        return;
    }

    http_response_set_json(res, 200, json_str);
    free(json_str);
}

