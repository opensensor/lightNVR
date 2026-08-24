/**
 * @file api_handlers_recording_tags.c
 * @brief API handlers for recording tag management
 */

#define _XOPEN_SOURCE
#define _GNU_SOURCE

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <cjson/cJSON.h>

#include "web/api_handlers_recording_tags.h"
#include "web/api_handlers.h"
#include "web/request_response.h"
#include "web/httpd_utils.h"
#include "web/audit_log.h"
#define LOG_COMPONENT "RecordingsAPI"
#include "core/logger.h"
#include "core/config.h"
#include "database/db_detections.h"
#include "database/db_recording_tags.h"
#include "database/db_auth.h"
#include "database/db_recordings.h"
#include "database/db_fleet_query.h"

static void audit_recording_tag_operation(
    const http_request_t *req, const user_t *user, const char *target_uuid,
    const char *operation, const char *outcome, int requested_count,
    int changed_count, const char *reason) {
    cJSON *details = cJSON_CreateObject();
    if (details) {
        cJSON_AddNumberToObject(details, "requested_count", requested_count);
        cJSON_AddNumberToObject(details, "changed_count", changed_count);
        if (reason) cJSON_AddStringToObject(details, "reason", reason);
    }
    audit_log_operation(req, user, "evidence.protect",
                        target_uuid ? "recording" : "recording_batch",
                        target_uuid, operation, outcome, details);
    cJSON_Delete(details);
}

static int load_replay_authorized_streams(
    const http_request_t *req, http_response_t *res,
    fleet_camera_t **cameras, const char **stream_names, int *stream_count) {
    user_t user;
    memset(&user, 0, sizeof(user));
    *cameras = NULL;
    *stream_count = 0;
    if (!httpd_check_action_access(req, &user)) {
        http_response_set_json_error(res, 401, "Unauthorized");
        return 0;
    }
    int camera_count = 0;
    if (db_fleet_camera_load(cameras, &camera_count) != 0 ||
        authorization_filter_cameras(&user, AUTHZ_RECORDINGS_REPLAY,
                                     *cameras, &camera_count) != 0) {
        free(*cameras);
        *cameras = NULL;
        http_response_set_json_error(
            res, 500, "Authorization policy evaluation failed");
        return 0;
    }
    for (int i = 0; i < camera_count; i++) {
        stream_names[i] = (*cameras)[i].name;
    }
    *stream_count = camera_count;
    return 1;
}

/* ------------------------------------------------------------------ */
/* GET /api/recordings/tags — list all unique tags                     */
/* ------------------------------------------------------------------ */
void handle_get_recording_tags(const http_request_t *req, http_response_t *res) {
    log_debug("Handling GET /api/recordings/tags");

    fleet_camera_t *cameras = NULL;
    const char *stream_names[MAX_STREAMS];
    int stream_count = 0;
    if (!load_replay_authorized_streams(req, res, &cameras, stream_names,
                                        &stream_count)) return;

    char tags[MAX_RECORDING_TAGS][MAX_TAG_LENGTH];
    int count = db_recording_tag_get_unique_for_streams(
        stream_names, stream_count, tags, MAX_RECORDING_TAGS);
    free(cameras);
    if (count < 0) {
        http_response_set_json_error(res, 500, "Failed to get tags");
        return;
    }

    cJSON *response = cJSON_CreateObject();
    cJSON *arr = cJSON_CreateArray();
    for (int i = 0; i < count; i++) {
        cJSON_AddItemToArray(arr, cJSON_CreateString(tags[i]));
    }
    cJSON_AddItemToObject(response, "tags", arr);

    char *json_str = cJSON_PrintUnformatted(response);
    http_response_set_json(res, 200, json_str);
    free(json_str);
    cJSON_Delete(response);
}

/* ------------------------------------------------------------------ */
/* GET /api/recordings/detection-labels — list all unique labels       */
/* ------------------------------------------------------------------ */
void handle_get_recording_detection_labels(const http_request_t *req, http_response_t *res) {
    log_debug("Handling GET /api/recordings/detection-labels");

    fleet_camera_t *cameras = NULL;
    const char *stream_names[MAX_STREAMS];
    int stream_count = 0;
    if (!load_replay_authorized_streams(req, res, &cameras, stream_names,
                                        &stream_count)) return;

    char labels[MAX_UNIQUE_DETECTION_LABELS][MAX_LABEL_LENGTH];
    int count = get_unique_detection_labels_for_streams(
        stream_names, stream_count, labels, MAX_UNIQUE_DETECTION_LABELS);
    free(cameras);
    if (count < 0) {
        http_response_set_json_error(res, 500, "Failed to get detection labels");
        return;
    }

    cJSON *response = cJSON_CreateObject();
    cJSON *arr = cJSON_CreateArray();
    for (int i = 0; i < count; i++) {
        cJSON_AddItemToArray(arr, cJSON_CreateString(labels[i]));
    }
    cJSON_AddItemToObject(response, "labels", arr);

    char *json_str = cJSON_PrintUnformatted(response);
    http_response_set_json(res, 200, json_str);
    free(json_str);
    cJSON_Delete(response);
}

/* ------------------------------------------------------------------ */
/* GET /api/recordings/:id/tags                                       */
/* ------------------------------------------------------------------ */
void handle_get_recording_tags_by_id(const http_request_t *req, http_response_t *res) {
    log_debug("Handling GET /api/recordings/:id/tags");

    char id_str[32] = {0};
    if (http_request_extract_path_param(req, "/api/recordings/", id_str, sizeof(id_str)) != 0) {
        http_response_set_json_error(res, 400, "Invalid recording ID in URL");
        return;
    }
    char *suffix = strstr(id_str, "/tags");
    if (suffix) *suffix = '\0';

    uint64_t id = strtoull(id_str, NULL, 10);
    if (id == 0) {
        http_response_set_json_error(res, 400, "Invalid recording ID");
        return;
    }
    recording_metadata_t recording = {0};
    if (get_recording_metadata_by_id(id, &recording) != 0) {
        http_response_set_json_error(res, 404, "Recording not found");
        return;
    }
    if (!httpd_authorize_camera_identity_action_with_context(
            req, res, AUTHZ_RECORDINGS_REPLAY, recording.camera_uuid,
            recording.stream_name, &(user_t){0}, &(fleet_camera_t){0},
            &(authorization_evaluation_t){0})) return;

    char tags[MAX_RECORDING_TAGS][MAX_TAG_LENGTH];
    int count = db_recording_tag_get(id, tags, MAX_RECORDING_TAGS);
    if (count < 0) {
        http_response_set_json_error(res, 500, "Failed to get tags for recording");
        return;
    }

    cJSON *response = cJSON_CreateObject();
    cJSON_AddNumberToObject(response, "id", (double)id);
    cJSON *arr = cJSON_CreateArray();
    for (int i = 0; i < count; i++) {
        cJSON_AddItemToArray(arr, cJSON_CreateString(tags[i]));
    }
    cJSON_AddItemToObject(response, "tags", arr);

    char *json_str = cJSON_PrintUnformatted(response);
    http_response_set_json(res, 200, json_str);
    free(json_str);
    cJSON_Delete(response);
}

/* ------------------------------------------------------------------ */
/* PUT /api/recordings/:id/tags                                       */
/* ------------------------------------------------------------------ */
void handle_put_recording_tags(const http_request_t *req, http_response_t *res) {
    log_info("Handling PUT /api/recordings/:id/tags");

    char id_str[32] = {0};
    if (http_request_extract_path_param(req, "/api/recordings/", id_str, sizeof(id_str)) != 0) {
        http_response_set_json_error(res, 400, "Invalid recording ID in URL");
        return;
    }
    char *suffix = strstr(id_str, "/tags");
    if (suffix) *suffix = '\0';

    uint64_t id = strtoull(id_str, NULL, 10);
    if (id == 0) {
        http_response_set_json_error(res, 400, "Invalid recording ID");
        return;
    }
    recording_metadata_t recording = {0};
    if (get_recording_metadata_by_id(id, &recording) != 0) {
        http_response_set_json_error(res, 404, "Recording not found");
        return;
    }
    user_t user;
    fleet_camera_t camera;
    authorization_evaluation_t evaluation;
    if (!httpd_authorize_camera_identity_action_with_context(
            req, res, AUTHZ_EVIDENCE_PROTECT, recording.camera_uuid,
            recording.stream_name, &user, &camera, &evaluation)) return;
    char recording_uuid[32];
    snprintf(recording_uuid, sizeof(recording_uuid), "%llu",
             (unsigned long long)id);

    cJSON *json = httpd_parse_json_body(req);
    if (!json) {
        http_response_set_json_error(res, 400, "Invalid JSON in request body");
        return;
    }

    cJSON *tags_json = cJSON_GetObjectItem(json, "tags");
    if (!tags_json || !cJSON_IsArray(tags_json)) {
        cJSON_Delete(json);
        http_response_set_json_error(res, 400, "Missing or invalid 'tags' field (array required)");
        return;
    }



    int tag_count = cJSON_GetArraySize(tags_json);
    const char **tag_strs = NULL;
    if (tag_count > 0) {
        tag_strs = (const char **)malloc((size_t)tag_count * sizeof(char *));
        if (!tag_strs) {
            cJSON_Delete(json);
            http_response_set_json_error(res, 500, "Memory allocation failed");
            return;
        }
        int valid = 0;
        cJSON *item;
        cJSON_ArrayForEach(item, tags_json) {
            if (cJSON_IsString(item) && item->valuestring[0] != '\0') {
                tag_strs[valid++] = item->valuestring;
            }
        }
        tag_count = valid;
    }

    int rc = db_recording_tag_set(id, tag_strs, tag_count);
    free((void *)tag_strs);
    cJSON_Delete(json);

    if (rc != 0) {
        audit_recording_tag_operation(req, &user, recording_uuid,
                                      "recording_tags.replace", "error",
                                      tag_count, 0, "database_update_failed");
        http_response_set_json_error(res, 500, "Failed to set tags");
        return;
    }

    /* Return the updated tags */
    char result_tags[MAX_RECORDING_TAGS][MAX_TAG_LENGTH];
    int count = db_recording_tag_get(id, result_tags, MAX_RECORDING_TAGS);

    cJSON *response = cJSON_CreateObject();
    cJSON_AddNumberToObject(response, "id", (double)id);
    cJSON *arr = cJSON_CreateArray();
    if (count > 0) {
        for (int i = 0; i < count; i++) {
            cJSON_AddItemToArray(arr, cJSON_CreateString(result_tags[i]));
        }
    }
    cJSON_AddItemToObject(response, "tags", arr);

    char *json_str = cJSON_PrintUnformatted(response);
    http_response_set_json(res, 200, json_str);
    free(json_str);
    cJSON_Delete(response);

    audit_recording_tag_operation(req, &user, recording_uuid,
                                  "recording_tags.replace", "success",
                                  tag_count, count > 0 ? count : 0,
                                  "completed");

    log_info("Set %d tags for recording %llu", count, (unsigned long long)id);
}

/* ------------------------------------------------------------------ */
/* POST /api/recordings/batch-tags                                    */
/* ------------------------------------------------------------------ */
void handle_batch_recording_tags(const http_request_t *req, http_response_t *res) {
    log_info("Handling POST /api/recordings/batch-tags");

    user_t user;
    if (!httpd_check_action_access(req, &user)) {
        http_response_set_json_error(res, 401, "Unauthorized");
        return;
    }

    cJSON *json = httpd_parse_json_body(req);
    if (!json) {
        http_response_set_json_error(res, 400, "Invalid JSON in request body");
        return;
    }

    cJSON *ids_json = cJSON_GetObjectItem(json, "ids");
    if (!ids_json || !cJSON_IsArray(ids_json)) {
        cJSON_Delete(json);
        http_response_set_json_error(res, 400, "Missing or invalid 'ids' field (array required)");
        return;
    }

    int id_count = cJSON_GetArraySize(ids_json);
    if (id_count <= 0 || id_count > 10000) {
        cJSON_Delete(json);
        http_response_set_json_error(res, 400, "Invalid number of IDs (1-10000)");
        return;
    }

    uint64_t *ids = malloc(id_count * sizeof(uint64_t));
    if (!ids) {
        cJSON_Delete(json);
        http_response_set_json_error(res, 500, "Memory allocation failed");
        return;
    }

    int valid_ids = 0;
    bool invalid_member = false;
    bool denied_member = false;
    bool evaluation_error = false;
    cJSON *id_item;
    cJSON_ArrayForEach(id_item, ids_json) {
        if (!cJSON_IsNumber(id_item) || id_item->valuedouble <= 0) {
            invalid_member = true;
            break;
        }
        uint64_t id = (uint64_t)id_item->valuedouble;
        recording_metadata_t recording = {0};
        authorization_evaluation_t evaluation;
        if (get_recording_metadata_by_id(id, &recording) != 0) {
            denied_member = true;
            break;
        }
        int eval_result = httpd_evaluate_stream_action(
            &user, AUTHZ_EVIDENCE_PROTECT, recording.stream_name,
            &evaluation);
        if (eval_result < 0) {
            evaluation_error = true;
            break;
        }
        if (eval_result > 0 ||
            evaluation.decision != AUTHZ_DECISION_ALLOW) {
            denied_member = true;
            break;
        }
        ids[valid_ids++] = id;
    }
    if (invalid_member || denied_member || evaluation_error ||
        valid_ids != id_count) {
        free(ids);
        cJSON_Delete(json);
        audit_recording_tag_operation(
            req, &user, NULL, "recording_tags.batch", evaluation_error
                ? "error" : "failure", id_count, 0,
            invalid_member ? "invalid_member" :
            (evaluation_error ? "authorization_evaluation_failed" :
                                "unauthorized_member"));
        if (invalid_member) {
            http_response_set_json_error(res, 400, "Invalid recording IDs");
        } else if (evaluation_error) {
            http_response_set_json_error(
                res, 500, "Authorization policy evaluation failed");
        } else {
            http_response_set_json_error(res, 403, "Forbidden");
        }
        return;
    }

    int add_success = 0, remove_success = 0;
    bool database_error = false;

    /* Process "add" tags */
    cJSON *add_json = cJSON_GetObjectItem(json, "add");
    if (add_json && cJSON_IsArray(add_json)) {
        cJSON *tag_item;
        cJSON_ArrayForEach(tag_item, add_json) {
            if (cJSON_IsString(tag_item) && tag_item->valuestring[0] != '\0') {
                int r = db_recording_tag_batch_add(ids, valid_ids, tag_item->valuestring);
                if (r > 0) add_success += r;
                if (r < 0) database_error = true;
            }
        }
    }

    /* Process "remove" tags */
    cJSON *remove_json = cJSON_GetObjectItem(json, "remove");
    if (remove_json && cJSON_IsArray(remove_json)) {
        cJSON *tag_item;
        cJSON_ArrayForEach(tag_item, remove_json) {
            if (cJSON_IsString(tag_item) && tag_item->valuestring[0] != '\0') {
                int r = db_recording_tag_batch_remove(ids, valid_ids, tag_item->valuestring);
                if (r > 0) remove_success += r;
                if (r < 0) database_error = true;
            }
        }
    }

    free(ids);
    cJSON_Delete(json);

    audit_recording_tag_operation(
        req, &user, NULL, "recording_tags.batch",
        database_error ? "error" : "success", valid_ids,
        add_success + remove_success,
        database_error ? "database_update_failed" : "completed");
    if (database_error) {
        http_response_set_json_error(res, 500, "Failed to update recording tags");
        return;
    }

    cJSON *response = cJSON_CreateObject();
    cJSON_AddNumberToObject(response, "recordings_count", valid_ids);
    cJSON_AddNumberToObject(response, "tags_added", add_success);
    cJSON_AddNumberToObject(response, "tags_removed", remove_success);

    char *json_str = cJSON_PrintUnformatted(response);
    http_response_set_json(res, 200, json_str);
    free(json_str);
    cJSON_Delete(response);

    log_info("Batch tags: %d recordings, %d added, %d removed",
             valid_ids, add_success, remove_success);
}
