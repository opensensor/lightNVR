/**
 * @file api_handlers_setup.c
 * @brief Setup wizard API handlers: GET/POST /api/setup/status
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdbool.h>

#include "web/api_handlers_setup.h"
#include "web/request_response.h"
#include "web/http_server.h"
#include "database/db_system_settings.h"
#include "core/logger.h"
#include "cjson/cJSON.h"

/**
 * GET /api/setup/status
 * No authentication guard – needed so the wizard can show before auth is configured.
 */
void handle_get_setup_status(const http_request_t *req, http_response_t *res) {
    (void)req;

    bool complete = db_is_setup_complete();

    /* Read optional completed_at timestamp */
    char ts_buf[32] = {0};
    long long completed_at = 0;
    if (db_get_system_setting("setup_completed_at", ts_buf, sizeof(ts_buf)) == 0) {
        completed_at = atoll(ts_buf);
    }

    cJSON *obj = cJSON_CreateObject();
    if (!obj) {
        http_response_set_json_error(res, 500, "Failed to allocate JSON");
        return;
    }

    cJSON_AddBoolToObject(obj, "complete", complete);
    cJSON_AddNumberToObject(obj, "setup_completed_at", (double)completed_at);

    char *json_str = cJSON_PrintUnformatted(obj);
    cJSON_Delete(obj);
    if (!json_str) {
        http_response_set_json_error(res, 500, "Failed to serialise JSON");
        return;
    }

    http_response_set_json(res, 200, json_str);
    free(json_str);
}

/**
 * POST /api/setup/complete
 * Body (optional): { "complete": true }
 */
void handle_post_setup_complete(const http_request_t *req, http_response_t *res) {
    (void)req;

    if (db_mark_setup_complete() != 0) {
        log_error("handle_post_setup_complete: failed to persist setup_complete flag");
        http_response_set_json_error(res, 500, "Failed to save setup state");
        return;
    }

    cJSON *obj = cJSON_CreateObject();
    if (!obj) {
        http_response_set_json_error(res, 500, "Failed to allocate JSON");
        return;
    }
    cJSON_AddBoolToObject(obj, "success", true);

    char *json_str = cJSON_PrintUnformatted(obj);
    cJSON_Delete(obj);
    if (!json_str) {
        http_response_set_json_error(res, 500, "Failed to serialise JSON");
        return;
    }

    http_response_set_json(res, 200, json_str);
    free(json_str);
}

