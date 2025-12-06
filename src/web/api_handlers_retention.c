/**
 * @file api_handlers_retention.c
 * @brief API handlers for recording retention policies and protection
 */

#define _XOPEN_SOURCE
#define _GNU_SOURCE

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <ctype.h>

#include "web/api_handlers.h"
#include "web/mongoose_adapter.h"
#include "core/logger.h"
#include "mongoose.h"
#include "database/db_streams.h"
#include "database/db_recordings.h"

/**
 * @brief Handler for GET /api/streams/:name/retention
 * Get retention configuration for a stream
 */
void mg_handle_get_stream_retention(struct mg_connection *c, struct mg_http_message *hm) {
    log_info("Handling GET /api/streams/:name/retention request");

    // Extract stream name from URL
    char stream_name[64] = {0};
    if (mg_extract_path_param(hm, "/api/streams/", stream_name, sizeof(stream_name)) != 0) {
        mg_send_json_error(c, 400, "Invalid stream name in URL");
        return;
    }

    // Remove /retention suffix if present
    char *suffix = strstr(stream_name, "/retention");
    if (suffix) {
        *suffix = '\0';
    }

    // URL decode the stream name
    char decoded_name[64] = {0};
    mg_url_decode_string(stream_name, decoded_name, sizeof(decoded_name));

    // Get retention config
    stream_retention_config_t config;
    if (get_stream_retention_config(decoded_name, &config) != 0) {
        mg_send_json_error(c, 404, "Stream not found or failed to get retention config");
        return;
    }

    // Build JSON response
    cJSON *json = cJSON_CreateObject();
    cJSON_AddStringToObject(json, "stream_name", decoded_name);
    cJSON_AddNumberToObject(json, "retention_days", config.retention_days);
    cJSON_AddNumberToObject(json, "detection_retention_days", config.detection_retention_days);
    cJSON_AddNumberToObject(json, "max_storage_mb", (double)config.max_storage_mb);

    char *json_str = cJSON_PrintUnformatted(json);
    mg_send_json_response(c, 200, json_str);

    free(json_str);
    cJSON_Delete(json);
}

/**
 * @brief Handler for PUT /api/streams/:name/retention
 * Update retention configuration for a stream
 */
void mg_handle_put_stream_retention(struct mg_connection *c, struct mg_http_message *hm) {
    log_info("Handling PUT /api/streams/:name/retention request");

    // Extract stream name from URL
    char stream_name[64] = {0};
    if (mg_extract_path_param(hm, "/api/streams/", stream_name, sizeof(stream_name)) != 0) {
        mg_send_json_error(c, 400, "Invalid stream name in URL");
        return;
    }

    // Remove /retention suffix if present
    char *suffix = strstr(stream_name, "/retention");
    if (suffix) {
        *suffix = '\0';
    }

    // URL decode the stream name
    char decoded_name[64] = {0};
    mg_url_decode_string(stream_name, decoded_name, sizeof(decoded_name));

    // Parse JSON body
    cJSON *json = mg_parse_json_body(hm);
    if (!json) {
        mg_send_json_error(c, 400, "Invalid JSON in request body");
        return;
    }

    // Get current config as defaults
    stream_retention_config_t config;
    if (get_stream_retention_config(decoded_name, &config) != 0) {
        cJSON_Delete(json);
        mg_send_json_error(c, 404, "Stream not found");
        return;
    }

    // Update with provided values
    cJSON *retention_days = cJSON_GetObjectItem(json, "retention_days");
    if (retention_days && cJSON_IsNumber(retention_days)) {
        config.retention_days = retention_days->valueint;
    }

    cJSON *detection_retention_days = cJSON_GetObjectItem(json, "detection_retention_days");
    if (detection_retention_days && cJSON_IsNumber(detection_retention_days)) {
        config.detection_retention_days = detection_retention_days->valueint;
    }

    cJSON *max_storage_mb = cJSON_GetObjectItem(json, "max_storage_mb");
    if (max_storage_mb && cJSON_IsNumber(max_storage_mb)) {
        config.max_storage_mb = (uint64_t)max_storage_mb->valuedouble;
    }

    cJSON_Delete(json);

    // Save config
    if (set_stream_retention_config(decoded_name, &config) != 0) {
        mg_send_json_error(c, 500, "Failed to save retention config");
        return;
    }

    // Return updated config
    cJSON *response = cJSON_CreateObject();
    cJSON_AddStringToObject(response, "stream_name", decoded_name);
    cJSON_AddNumberToObject(response, "retention_days", config.retention_days);
    cJSON_AddNumberToObject(response, "detection_retention_days", config.detection_retention_days);
    cJSON_AddNumberToObject(response, "max_storage_mb", (double)config.max_storage_mb);
    cJSON_AddStringToObject(response, "message", "Retention config updated successfully");

    char *json_str = cJSON_PrintUnformatted(response);
    mg_send_json_response(c, 200, json_str);

    free(json_str);
    cJSON_Delete(response);

    log_info("Updated retention config for stream %s: retention=%d, detection_retention=%d, max_storage=%lu MB",
             decoded_name, config.retention_days, config.detection_retention_days,
             (unsigned long)config.max_storage_mb);
}

/**
 * @brief Handler for PUT /api/recordings/:id/protect
 * Set protection status for a recording
 */
void mg_handle_put_recording_protect(struct mg_connection *c, struct mg_http_message *hm) {
    log_info("Handling PUT /api/recordings/:id/protect request");

    // Extract recording ID from URL
    char id_str[32] = {0};
    if (mg_extract_path_param(hm, "/api/recordings/", id_str, sizeof(id_str)) != 0) {
        mg_send_json_error(c, 400, "Invalid recording ID in URL");
        return;
    }

    // Remove /protect suffix if present
    char *suffix = strstr(id_str, "/protect");
    if (suffix) {
        *suffix = '\0';
    }

    // Parse ID
    uint64_t id = strtoull(id_str, NULL, 10);
    if (id == 0) {
        mg_send_json_error(c, 400, "Invalid recording ID");
        return;
    }

    // Parse JSON body
    cJSON *json = mg_parse_json_body(hm);
    if (!json) {
        mg_send_json_error(c, 400, "Invalid JSON in request body");
        return;
    }

    // Get protected status
    cJSON *protected_json = cJSON_GetObjectItem(json, "protected");
    if (!protected_json || !cJSON_IsBool(protected_json)) {
        cJSON_Delete(json);
        mg_send_json_error(c, 400, "Missing or invalid 'protected' field (boolean required)");
        return;
    }

    bool protected = cJSON_IsTrue(protected_json);
    cJSON_Delete(json);

    // Update protection status
    if (set_recording_protected(id, protected) != 0) {
        mg_send_json_error(c, 500, "Failed to update recording protection status");
        return;
    }

    // Return success response
    cJSON *response = cJSON_CreateObject();
    cJSON_AddNumberToObject(response, "id", (double)id);
    cJSON_AddBoolToObject(response, "protected", protected);
    cJSON_AddStringToObject(response, "message", protected ? "Recording protected" : "Recording unprotected");

    char *json_str = cJSON_PrintUnformatted(response);
    mg_send_json_response(c, 200, json_str);

    free(json_str);
    cJSON_Delete(response);

    log_info("Recording %llu protection set to %s", (unsigned long long)id, protected ? "true" : "false");
}

/**
 * @brief Handler for PUT /api/recordings/:id/retention
 * Set custom retention override for a recording
 */
void mg_handle_put_recording_retention(struct mg_connection *c, struct mg_http_message *hm) {
    log_info("Handling PUT /api/recordings/:id/retention request");

    // Extract recording ID from URL
    char id_str[32] = {0};
    if (mg_extract_path_param(hm, "/api/recordings/", id_str, sizeof(id_str)) != 0) {
        mg_send_json_error(c, 400, "Invalid recording ID in URL");
        return;
    }

    // Remove /retention suffix if present
    char *suffix = strstr(id_str, "/retention");
    if (suffix) {
        *suffix = '\0';
    }

    // Parse ID
    uint64_t id = strtoull(id_str, NULL, 10);
    if (id == 0) {
        mg_send_json_error(c, 400, "Invalid recording ID");
        return;
    }

    // Parse JSON body
    cJSON *json = mg_parse_json_body(hm);
    if (!json) {
        mg_send_json_error(c, 400, "Invalid JSON in request body");
        return;
    }

    // Get retention_days (-1 to remove override)
    cJSON *days_json = cJSON_GetObjectItem(json, "retention_days");
    if (!days_json || !cJSON_IsNumber(days_json)) {
        cJSON_Delete(json);
        mg_send_json_error(c, 400, "Missing or invalid 'retention_days' field (number required, -1 to remove override)");
        return;
    }

    int days = days_json->valueint;
    cJSON_Delete(json);

    // Update retention override
    if (set_recording_retention_override(id, days) != 0) {
        mg_send_json_error(c, 500, "Failed to update recording retention override");
        return;
    }

    // Return success response
    cJSON *response = cJSON_CreateObject();
    cJSON_AddNumberToObject(response, "id", (double)id);
    cJSON_AddNumberToObject(response, "retention_days", days);
    if (days < 0) {
        cJSON_AddStringToObject(response, "message", "Retention override removed, using stream default");
    } else {
        cJSON_AddStringToObject(response, "message", "Custom retention set");
    }

    char *json_str = cJSON_PrintUnformatted(response);
    mg_send_json_response(c, 200, json_str);

    free(json_str);
    cJSON_Delete(response);

    log_info("Recording %llu retention override set to %d days", (unsigned long long)id, days);
}

/**
 * @brief Handler for GET /api/recordings/protected
 * Get count of protected recordings
 */
void mg_handle_get_protected_recordings(struct mg_connection *c, struct mg_http_message *hm) {
    log_info("Handling GET /api/recordings/protected request");

    // Check for stream_name query parameter
    char stream_name[64] = {0};
    struct mg_str query = hm->query;

    if (query.len > 0) {
        struct mg_str stream_param = mg_http_var(query, mg_str("stream"));
        if (stream_param.len > 0 && stream_param.len < sizeof(stream_name)) {
            memcpy(stream_name, stream_param.buf, stream_param.len);
            stream_name[stream_param.len] = '\0';
        }
    }

    // Get protected count
    int count = get_protected_recordings_count(stream_name[0] ? stream_name : NULL);
    if (count < 0) {
        mg_send_json_error(c, 500, "Failed to get protected recordings count");
        return;
    }

    // Return response
    cJSON *response = cJSON_CreateObject();
    cJSON_AddNumberToObject(response, "protected_count", count);
    if (stream_name[0]) {
        cJSON_AddStringToObject(response, "stream_name", stream_name);
    }

    char *json_str = cJSON_PrintUnformatted(response);
    mg_send_json_response(c, 200, json_str);

    free(json_str);
    cJSON_Delete(response);
}

/**
 * @brief Handler for POST /api/recordings/batch-protect
 * Batch protect/unprotect multiple recordings
 */
void mg_handle_batch_protect_recordings(struct mg_connection *c, struct mg_http_message *hm) {
    log_info("Handling POST /api/recordings/batch-protect request");

    // Parse JSON body
    cJSON *json = mg_parse_json_body(hm);
    if (!json) {
        mg_send_json_error(c, 400, "Invalid JSON in request body");
        return;
    }

    // Get recording IDs array
    cJSON *ids_json = cJSON_GetObjectItem(json, "ids");
    if (!ids_json || !cJSON_IsArray(ids_json)) {
        cJSON_Delete(json);
        mg_send_json_error(c, 400, "Missing or invalid 'ids' field (array required)");
        return;
    }

    // Get protected status
    cJSON *protected_json = cJSON_GetObjectItem(json, "protected");
    if (!protected_json || !cJSON_IsBool(protected_json)) {
        cJSON_Delete(json);
        mg_send_json_error(c, 400, "Missing or invalid 'protected' field (boolean required)");
        return;
    }

    bool protected = cJSON_IsTrue(protected_json);
    int success_count = 0;
    int fail_count = 0;

    // Process each ID
    cJSON *id_item;
    cJSON_ArrayForEach(id_item, ids_json) {
        if (cJSON_IsNumber(id_item)) {
            uint64_t id = (uint64_t)id_item->valuedouble;
            if (set_recording_protected(id, protected) == 0) {
                success_count++;
            } else {
                fail_count++;
            }
        } else {
            fail_count++;
        }
    }

    cJSON_Delete(json);

    // Return response
    cJSON *response = cJSON_CreateObject();
    cJSON_AddNumberToObject(response, "success_count", success_count);
    cJSON_AddNumberToObject(response, "fail_count", fail_count);
    cJSON_AddBoolToObject(response, "protected", protected);
    cJSON_AddStringToObject(response, "message", protected ? "Recordings protected" : "Recordings unprotected");

    char *json_str = cJSON_PrintUnformatted(response);
    mg_send_json_response(c, 200, json_str);

    free(json_str);
    cJSON_Delete(response);

    log_info("Batch protect: %d succeeded, %d failed, protected=%s",
             success_count, fail_count, protected ? "true" : "false");
}
