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
#include "core/logger.h"
#include "core/config.h"
#include "database/db_detections.h"
#include "database/db_recording_tags.h"
#include "database/db_auth.h"

/* ------------------------------------------------------------------ */
/* GET /api/recordings/tags — list all unique tags                     */
/* ------------------------------------------------------------------ */
void handle_get_recording_tags(const http_request_t *req, http_response_t *res) {
    log_debug("Handling GET /api/recordings/tags");

    if (g_config.web_auth_enabled) {
        user_t user;
        if (!httpd_check_viewer_access(req, &user)) {
            http_response_set_json_error(res, 401, "Unauthorized");
            return;
        }
    }

    char tags[MAX_RECORDING_TAGS][MAX_TAG_LENGTH];
    int count = db_recording_tag_get_all_unique(tags, MAX_RECORDING_TAGS);
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

    if (g_config.web_auth_enabled) {
        user_t user;
        if (!httpd_check_viewer_access(req, &user)) {
            http_response_set_json_error(res, 401, "Unauthorized");
            return;
        }
    }

    char labels[MAX_UNIQUE_DETECTION_LABELS][MAX_LABEL_LENGTH];
    int count = get_all_unique_detection_labels(labels, MAX_UNIQUE_DETECTION_LABELS);
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

    if (g_config.web_auth_enabled) {
        user_t user;
        if (!httpd_check_viewer_access(req, &user)) {
            http_response_set_json_error(res, 401, "Unauthorized");
            return;
        }
    }

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

    if (g_config.web_auth_enabled) {
        user_t user;
        if (!httpd_get_authenticated_user(req, &user)) {
            http_response_set_json_error(res, 401, "Unauthorized");
            return;
        }
    }

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

    log_info("Set %d tags for recording %llu", count, (unsigned long long)id);
}

/* ------------------------------------------------------------------ */
/* POST /api/recordings/batch-tags                                    */
/* ------------------------------------------------------------------ */
void handle_batch_recording_tags(const http_request_t *req, http_response_t *res) {
    log_info("Handling POST /api/recordings/batch-tags");

    if (g_config.web_auth_enabled) {
        user_t user;
        if (!httpd_get_authenticated_user(req, &user)) {
            http_response_set_json_error(res, 401, "Unauthorized");
            return;
        }
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
    cJSON *id_item;
    cJSON_ArrayForEach(id_item, ids_json) {
        if (cJSON_IsNumber(id_item)) {
            ids[valid_ids++] = (uint64_t)id_item->valuedouble;
        }
    }

    int add_success = 0, remove_success = 0;

    /* Process "add" tags */
    cJSON *add_json = cJSON_GetObjectItem(json, "add");
    if (add_json && cJSON_IsArray(add_json)) {
        cJSON *tag_item;
        cJSON_ArrayForEach(tag_item, add_json) {
            if (cJSON_IsString(tag_item) && tag_item->valuestring[0] != '\0') {
                int r = db_recording_tag_batch_add(ids, valid_ids, tag_item->valuestring);
                if (r > 0) add_success += r;
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
            }
        }
    }

    free(ids);
    cJSON_Delete(json);

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