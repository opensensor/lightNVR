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
#include "web/request_response.h"
#include "web/httpd_utils.h"
#include "web/audit_log.h"
#define LOG_COMPONENT "RecordingsAPI"
#include "core/logger.h"
#include "database/db_streams.h"
#include "database/db_recordings.h"
#include "database/db_fleet_query.h"

static int authorize_recording_action(const http_request_t *req,
                                      http_response_t *res, uint64_t id,
                                      authorization_action_t action,
                                      recording_metadata_t *recording,
                                      user_t *user, fleet_camera_t *camera,
                                      authorization_evaluation_t *evaluation) {
    memset(recording, 0, sizeof(*recording));
    if (get_recording_metadata_by_id(id, recording) != 0) {
        http_response_set_json_error(res, 404, "Recording not found");
        return 0;
    }
    return httpd_authorize_stream_action_with_context(
        req, res, action, recording->stream_name, user, camera, evaluation);
}

static void audit_recording_policy_operation(
    const http_request_t *req, const user_t *user,
    const fleet_camera_t *camera, uint64_t recording_id,
    const char *operation, const char *outcome, const char *reason,
    int value, const char *value_name) {
    char recording_uuid[32];
    snprintf(recording_uuid, sizeof(recording_uuid), "%llu",
             (unsigned long long)recording_id);
    cJSON *details = cJSON_CreateObject();
    if (details) {
        cJSON_AddStringToObject(details, "camera_uuid", camera->camera_uuid);
        cJSON_AddStringToObject(details, "reason", reason);
        if (value_name) cJSON_AddNumberToObject(details, value_name, value);
    }
    audit_log_operation(req, user, "evidence.protect", "recording",
                        recording_uuid, operation, outcome, details);
    cJSON_Delete(details);
}

static void audit_batch_policy_operation(const http_request_t *req,
                                         const user_t *user,
                                         bool protected, int requested_count,
                                         int success_count, int fail_count,
                                         const char *outcome,
                                         const char *reason) {
    cJSON *details = cJSON_CreateObject();
    if (details) {
        cJSON_AddStringToObject(details, "reason", reason);
        cJSON_AddBoolToObject(details, "protected", protected);
        cJSON_AddNumberToObject(details, "requested_count", requested_count);
        cJSON_AddNumberToObject(details, "success_count", success_count);
        cJSON_AddNumberToObject(details, "fail_count", fail_count);
    }
    audit_log_operation(req, user, "evidence.protect", "recording_batch",
                        NULL, protected ? "batch_protect" : "batch_unprotect",
                        outcome, details);
    cJSON_Delete(details);
}

/**
 * @brief Handler for GET /api/streams/:name/retention
 * Get retention configuration for a stream
 */
void handle_get_stream_retention(const http_request_t *req, http_response_t *res) {
    log_info("Handling GET /api/streams/:name/retention request");

    // Extract stream name from URL
    char stream_name[MAX_STREAM_NAME] = {0};
    if (http_request_extract_path_param(req, "/api/streams/", stream_name, sizeof(stream_name)) != 0) {
        http_response_set_json_error(res, 400, "Invalid stream name in URL");
        return;
    }

    // Remove /retention suffix if present
    char *suffix = strstr(stream_name, "/retention");
    if (suffix) {
        *suffix = '\0';
    }

    if (!httpd_authorize_stream_action(req, res, AUTHZ_CAMERA_CONFIGURE,
                                       stream_name)) return;

    // Get retention config
    stream_retention_config_t config;
    if (get_stream_retention_config(stream_name, &config) != 0) {
        http_response_set_json_error(res, 404, "Stream not found or failed to get retention config");
        return;
    }

    // Build JSON response
    cJSON *json = cJSON_CreateObject();
    cJSON_AddStringToObject(json, "stream_name", stream_name);
    cJSON_AddNumberToObject(json, "retention_days", config.retention_days);
    cJSON_AddNumberToObject(json, "detection_retention_days", config.detection_retention_days);
    cJSON_AddNumberToObject(json, "max_storage_mb", (double)config.max_storage_mb);

    char *json_str = cJSON_PrintUnformatted(json);
    http_response_set_json(res, 200, json_str);

    free(json_str);
    cJSON_Delete(json);
}

/**
 * @brief Handler for PUT /api/streams/:name/retention
 * Update retention configuration for a stream
 */
void handle_put_stream_retention(const http_request_t *req, http_response_t *res) {
    log_info("Handling PUT /api/streams/:name/retention request");

    // Extract stream name from URL
    char stream_name[MAX_STREAM_NAME] = {0};
    if (http_request_extract_path_param(req, "/api/streams/", stream_name, sizeof(stream_name)) != 0) {
        http_response_set_json_error(res, 400, "Invalid stream name in URL");
        return;
    }

    // Remove /retention suffix if present
    char *suffix = strstr(stream_name, "/retention");
    if (suffix) {
        *suffix = '\0';
    }

    if (!httpd_authorize_stream_action(req, res, AUTHZ_CAMERA_CONFIGURE,
                                       stream_name)) return;

    // Parse JSON body
    cJSON *json = httpd_parse_json_body(req);
    if (!json) {
        http_response_set_json_error(res, 400, "Invalid JSON in request body");
        return;
    }

    // Get current config as defaults
    stream_retention_config_t config;
    if (get_stream_retention_config(stream_name, &config) != 0) {
        cJSON_Delete(json);
        http_response_set_json_error(res, 404, "Stream not found");
        return;
    }

    // Update with provided values
    cJSON *retention_days = cJSON_GetObjectItem(json, "retention_days");
    if (retention_days && cJSON_IsNumber(retention_days)) {
        if (retention_days->valueint < -1 || retention_days->valueint > 365) {
            cJSON_Delete(json);
            http_response_set_json_error(res, 400,
                "retention_days must be between -1 and 365");
            return;
        }
        config.retention_days = retention_days->valueint;
    }

    cJSON *detection_retention_days = cJSON_GetObjectItem(json, "detection_retention_days");
    if (detection_retention_days && cJSON_IsNumber(detection_retention_days)) {
        if (detection_retention_days->valueint < -1 ||
            detection_retention_days->valueint > 365) {
            cJSON_Delete(json);
            http_response_set_json_error(res, 400,
                "detection_retention_days must be between -1 and 365");
            return;
        }
        config.detection_retention_days = detection_retention_days->valueint;
    }

    cJSON *max_storage_mb = cJSON_GetObjectItem(json, "max_storage_mb");
    if (max_storage_mb && cJSON_IsNumber(max_storage_mb)) {
        config.max_storage_mb = (uint64_t)max_storage_mb->valuedouble;
    }

    cJSON_Delete(json);

    // Save config
    if (set_stream_retention_config(stream_name, &config) != 0) {
        http_response_set_json_error(res, 500, "Failed to save retention config");
        return;
    }

    // Return updated config
    cJSON *response = cJSON_CreateObject();
    cJSON_AddStringToObject(response, "stream_name", stream_name);
    cJSON_AddNumberToObject(response, "retention_days", config.retention_days);
    cJSON_AddNumberToObject(response, "detection_retention_days", config.detection_retention_days);
    cJSON_AddNumberToObject(response, "max_storage_mb", (double)config.max_storage_mb);
    cJSON_AddStringToObject(response, "message", "Retention config updated successfully");

    char *json_str = cJSON_PrintUnformatted(response);
    http_response_set_json(res, 200, json_str);

    free(json_str);
    cJSON_Delete(response);

    log_info("Updated retention config for stream %s: retention=%d, detection_retention=%d, max_storage=%lu MB",
             stream_name, config.retention_days, config.detection_retention_days,
             (unsigned long)config.max_storage_mb);
}

/**
 * @brief Handler for PUT /api/recordings/:id/protect
 * Set protection status for a recording
 */
void handle_put_recording_protect(const http_request_t *req, http_response_t *res) {
    log_info("Handling PUT /api/recordings/:id/protect request");

    // Extract recording ID from URL
    char id_str[32] = {0};
    if (http_request_extract_path_param(req, "/api/recordings/", id_str, sizeof(id_str)) != 0) {
        http_response_set_json_error(res, 400, "Invalid recording ID in URL");
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
        http_response_set_json_error(res, 400, "Invalid recording ID");
        return;
    }

    // Parse JSON body
    cJSON *json = httpd_parse_json_body(req);
    if (!json) {
        http_response_set_json_error(res, 400, "Invalid JSON in request body");
        return;
    }

    // Get protected status
    cJSON *protected_json = cJSON_GetObjectItem(json, "protected");
    if (!protected_json || !cJSON_IsBool(protected_json)) {
        cJSON_Delete(json);
        http_response_set_json_error(res, 400, "Missing or invalid 'protected' field (boolean required)");
        return;
    }

    bool protected = cJSON_IsTrue(protected_json);
    cJSON_Delete(json);

    recording_metadata_t recording;
    user_t user;
    fleet_camera_t camera;
    authorization_evaluation_t evaluation;
    if (!authorize_recording_action(req, res, id, AUTHZ_EVIDENCE_PROTECT,
                                    &recording, &user, &camera, &evaluation)) {
        return;
    }

    // Update protection status
    if (set_recording_protected(id, protected) != 0) {
        audit_recording_policy_operation(
            req, &user, &camera, id, protected ? "protect" : "unprotect",
            "error", "database_update_failed", protected, "protected");
        http_response_set_json_error(res, 500, "Failed to update recording protection status");
        return;
    }

    audit_recording_policy_operation(
        req, &user, &camera, id, protected ? "protect" : "unprotect",
        "success", "completed", protected, "protected");

    // Return success response
    cJSON *response = cJSON_CreateObject();
    cJSON_AddNumberToObject(response, "id", (double)id);
    cJSON_AddBoolToObject(response, "protected", protected);
    cJSON_AddStringToObject(response, "message", protected ? "Recording protected" : "Recording unprotected");

    char *json_str = cJSON_PrintUnformatted(response);
    http_response_set_json(res, 200, json_str);

    free(json_str);
    cJSON_Delete(response);

    log_info("Recording %llu protection set to %s", (unsigned long long)id, protected ? "true" : "false");
}

/**
 * @brief Handler for PUT /api/recordings/:id/retention
 * Set custom retention override for a recording
 */
void handle_put_recording_retention(const http_request_t *req, http_response_t *res) {
    log_info("Handling PUT /api/recordings/:id/retention request");

    // Extract recording ID from URL
    char id_str[32] = {0};
    if (http_request_extract_path_param(req, "/api/recordings/", id_str, sizeof(id_str)) != 0) {
        http_response_set_json_error(res, 400, "Invalid recording ID in URL");
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
        http_response_set_json_error(res, 400, "Invalid recording ID");
        return;
    }

    // Parse JSON body
    cJSON *json = httpd_parse_json_body(req);
    if (!json) {
        http_response_set_json_error(res, 400, "Invalid JSON in request body");
        return;
    }

    // Get retention_days (-1 to remove override)
    cJSON *days_json = cJSON_GetObjectItem(json, "retention_days");
    if (!days_json || !cJSON_IsNumber(days_json)) {
        cJSON_Delete(json);
        http_response_set_json_error(res, 400, "Missing or invalid 'retention_days' field (number required, -1 to remove override)");
        return;
    }

    int days = days_json->valueint;
    cJSON_Delete(json);

    recording_metadata_t recording;
    user_t user;
    fleet_camera_t camera;
    authorization_evaluation_t evaluation;
    if (!authorize_recording_action(req, res, id, AUTHZ_EVIDENCE_PROTECT,
                                    &recording, &user, &camera, &evaluation)) {
        return;
    }

    // Update retention override
    if (set_recording_retention_override(id, days) != 0) {
        audit_recording_policy_operation(
            req, &user, &camera, id,
            days < 0 ? "clear_retention_override" : "set_retention_override",
            "error", "database_update_failed", days, "retention_days");
        http_response_set_json_error(res, 500, "Failed to update recording retention override");
        return;
    }

    audit_recording_policy_operation(
        req, &user, &camera, id,
        days < 0 ? "clear_retention_override" : "set_retention_override",
        "success", "completed", days, "retention_days");

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
    http_response_set_json(res, 200, json_str);

    free(json_str);
    cJSON_Delete(response);

    log_info("Recording %llu retention override set to %d days", (unsigned long long)id, days);
}

/**
 * @brief Handler for GET /api/recordings/protected
 * Get count of protected recordings
 */
void handle_get_protected_recordings(const http_request_t *req, http_response_t *res) {
    log_info("Handling GET /api/recordings/protected request");

    // Check for stream_name query parameter
    char stream_name[64] = {0};
    http_request_get_query_param(req, "stream", stream_name, sizeof(stream_name));

    int count = -1;
    if (stream_name[0]) {
        if (!httpd_authorize_stream_action(req, res, AUTHZ_RECORDINGS_REPLAY,
                                           stream_name)) return;
        count = get_protected_recordings_count(stream_name);
    } else {
        user_t user;
        if (!httpd_check_action_access(req, &user)) {
            http_response_set_json_error(res, 401, "Unauthorized");
            return;
        }
        fleet_camera_t *cameras = NULL;
        int camera_count = 0;
        if (db_fleet_camera_load(&cameras, &camera_count) != 0 ||
            authorization_filter_cameras(&user, AUTHZ_RECORDINGS_REPLAY,
                                         cameras, &camera_count) != 0) {
            free(cameras);
            http_response_set_json_error(
                res, 500, "Authorization policy evaluation failed");
            return;
        }
        const char **names = camera_count > 0
            ? calloc((size_t)camera_count, sizeof(*names)) : NULL;
        if (camera_count > 0 && !names) {
            free(cameras);
            http_response_set_json_error(res, 500, "Out of memory");
            return;
        }
        for (int i = 0; i < camera_count; i++) names[i] = cameras[i].name;
        count = get_protected_recordings_count_for_streams(names, camera_count);
        free(names);
        free(cameras);
    }
    if (count < 0) {
        http_response_set_json_error(res, 500, "Failed to get protected recordings count");
        return;
    }

    // Return response
    cJSON *response = cJSON_CreateObject();
    cJSON_AddNumberToObject(response, "protected_count", count);
    if (stream_name[0]) {
        cJSON_AddStringToObject(response, "stream_name", stream_name);
    }

    char *json_str = cJSON_PrintUnformatted(response);
    http_response_set_json(res, 200, json_str);

    free(json_str);
    cJSON_Delete(response);
}

/**
 * @brief Handler for POST /api/recordings/batch-protect
 * Batch protect/unprotect multiple recordings
 */
void handle_batch_protect_recordings(const http_request_t *req, http_response_t *res) {
    log_info("Handling POST /api/recordings/batch-protect request");

    // Parse JSON body
    cJSON *json = httpd_parse_json_body(req);
    if (!json) {
        http_response_set_json_error(res, 400, "Invalid JSON in request body");
        return;
    }

    // Get recording IDs array
    cJSON *ids_json = cJSON_GetObjectItem(json, "ids");
    if (!ids_json || !cJSON_IsArray(ids_json)) {
        cJSON_Delete(json);
        http_response_set_json_error(res, 400, "Missing or invalid 'ids' field (array required)");
        return;
    }

    // Get protected status
    cJSON *protected_json = cJSON_GetObjectItem(json, "protected");
    if (!protected_json || !cJSON_IsBool(protected_json)) {
        cJSON_Delete(json);
        http_response_set_json_error(res, 400, "Missing or invalid 'protected' field (boolean required)");
        return;
    }

    bool protected = cJSON_IsTrue(protected_json);
    user_t user;
    if (!httpd_check_action_access(req, &user)) {
        cJSON_Delete(json);
        http_response_set_json_error(res, 401, "Unauthorized");
        return;
    }

    int item_count = cJSON_GetArraySize(ids_json);
    uint64_t *authorized_ids = item_count > 0
        ? calloc((size_t)item_count, sizeof(*authorized_ids)) : NULL;
    if (item_count > 0 && !authorized_ids) {
        audit_batch_policy_operation(req, &user, protected, item_count, 0,
                                     item_count, "error",
                                     "allocation_failed");
        cJSON_Delete(json);
        http_response_set_json_error(res, 500, "Failed to authorize batch");
        return;
    }
    int authorized_count = 0;
    int fail_count = 0;

    // Resolve and authorize every fixed member before mutating any recording.
    cJSON *id_item;
    cJSON_ArrayForEach(id_item, ids_json) {
        if (!cJSON_IsNumber(id_item) || id_item->valuedouble <= 0) {
            fail_count++;
            continue;
        }
        uint64_t id = (uint64_t)id_item->valuedouble;
        recording_metadata_t recording;
        if (get_recording_metadata_by_id(id, &recording) != 0) {
            fail_count++;
            continue;
        }
        authorization_evaluation_t evaluation;
        int result = httpd_evaluate_stream_action(
            &user, AUTHZ_EVIDENCE_PROTECT, recording.stream_name,
            &evaluation);
        if (result < 0) {
            audit_batch_policy_operation(
                req, &user, protected, item_count, 0, fail_count, "error",
                "authorization_evaluation_failed");
            free(authorized_ids);
            cJSON_Delete(json);
            http_response_set_json_error(
                res, 500, "Authorization policy evaluation failed");
            return;
        }
        if (result > 0 || evaluation.decision != AUTHZ_DECISION_ALLOW) {
            fail_count++;
            continue;
        }
        authorized_ids[authorized_count++] = id;
    }

    int success_count = 0;
    for (int i = 0; i < authorized_count; i++) {
        if (set_recording_protected(authorized_ids[i], protected) == 0) {
            success_count++;
        } else {
            fail_count++;
        }
    }

    free(authorized_ids);
    cJSON_Delete(json);

    audit_batch_policy_operation(
        req, &user, protected, item_count, success_count, fail_count,
        fail_count == 0 ? "success" : "failure",
        fail_count == 0 ? "completed" : "partial_failure");

    // Return response
    cJSON *response = cJSON_CreateObject();
    cJSON_AddNumberToObject(response, "success_count", success_count);
    cJSON_AddNumberToObject(response, "fail_count", fail_count);
    cJSON_AddBoolToObject(response, "protected", protected);
    cJSON_AddStringToObject(response, "message", protected ? "Recordings protected" : "Recordings unprotected");

    char *json_str = cJSON_PrintUnformatted(response);
    http_response_set_json(res, 200, json_str);

    free(json_str);
    cJSON_Delete(response);

    log_info("Batch protect: %d succeeded, %d failed, protected=%s",
             success_count, fail_count, protected ? "true" : "false");
}
