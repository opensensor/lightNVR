#define _POSIX_C_SOURCE 200809L

#include "web/api_handlers_lpr.h"

#include <cjson/cJSON.h>
#include <math.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "core/authorization.h"
#include "database/db_lpr_reads.h"
#include "utils/lpr_crypto.h"
#include "utils/uuid.h"
#include "web/audit_log.h"
#include "web/httpd_utils.h"

#define LPR_SEARCH_BODY_MAX 4096
#define LPR_SEARCH_MAX_RANGE_MS (INT64_C(366) * 24 * 60 * 60 * 1000)
#define LPR_SEARCH_RESULT_MAX 100
#define LPR_EXPORT_RESULT_MAX 1000

static void secure_zero(void *data, size_t size) {
    volatile unsigned char *bytes = data;
    while (bytes && size-- > 0) *bytes++ = 0;
}

static void delete_query_body(cJSON *body) {
    cJSON *plate = body
        ? cJSON_GetObjectItemCaseSensitive(body, "plate") : NULL;
    if (cJSON_IsString(plate) && plate->valuestring) {
        secure_zero(plate->valuestring, strlen(plate->valuestring));
    }
    cJSON_Delete(body);
}

static void wipe_response_plates(cJSON *root) {
    cJSON *items = root
        ? cJSON_GetObjectItemCaseSensitive(root, "reads") : NULL;
    if (!cJSON_IsArray(items)) return;
    for (cJSON *item = items->child; item; item = item->next) {
        cJSON *plate = cJSON_GetObjectItemCaseSensitive(item, "plate");
        if (cJSON_IsString(plate) && plate->valuestring) {
            secure_zero(plate->valuestring, strlen(plate->valuestring));
        }
    }
}

static bool json_int64(const cJSON *body, const char *name, int64_t *value) {
    const cJSON *item = cJSON_GetObjectItemCaseSensitive(body, name);
    if (!cJSON_IsNumber(item) || !isfinite(item->valuedouble) ||
        floor(item->valuedouble) != item->valuedouble ||
        item->valuedouble < 1 || item->valuedouble > (double)INT64_MAX) return false;
    *value = (int64_t)item->valuedouble;
    return true;
}

static void digest_hex(const uint8_t digest[LPR_CRYPTO_HMAC_SIZE], char output[65]) {
    static const char hex[] = "0123456789abcdef";
    for (int i = 0; i < LPR_CRYPTO_HMAC_SIZE; ++i) {
        output[i * 2] = hex[digest[i] >> 4];
        output[i * 2 + 1] = hex[digest[i] & 0x0f];
    }
    output[64] = '\0';
}

static cJSON *read_json(const lpr_read_t *read) {
    cJSON *object = cJSON_CreateObject();
    if (!object) return NULL;
    cJSON_AddStringToObject(object, "id", read->uuid);
    cJSON_AddStringToObject(object, "type", "license_plate");
    cJSON_AddStringToObject(object, "camera_uuid", read->read.camera_uuid);
    cJSON_AddStringToObject(object, "stream_name", read->read.stream_name);
    cJSON_AddNumberToObject(object, "observed_at", (double)read->read.observed_at_ms);
    cJSON_AddNumberToObject(object, "received_at", (double)read->received_at_ms);
    cJSON_AddStringToObject(object, "source", read->read.source);
    cJSON_AddStringToObject(object, "plate", read->read.plate);
    if (read->read.has_confidence)
        cJSON_AddNumberToObject(object, "confidence", read->read.confidence);
    else cJSON_AddNullToObject(object, "confidence");

    cJSON *attributes = cJSON_AddObjectToObject(object, "attributes");
#define ADD_OPTIONAL(name) do { \
    if (read->read.name[0]) cJSON_AddStringToObject(attributes, #name, read->read.name); \
    else cJSON_AddNullToObject(attributes, #name); \
} while (0)
    ADD_OPTIONAL(country);
    ADD_OPTIONAL(region);
    ADD_OPTIONAL(plate_type);
    ADD_OPTIONAL(direction);
    ADD_OPTIONAL(lane);
    ADD_OPTIONAL(vehicle_type);
    ADD_OPTIONAL(vehicle_color);
    ADD_OPTIONAL(object_id);
    ADD_OPTIONAL(correlation_id);
#undef ADD_OPTIONAL
    cJSON_AddStringToObject(attributes, "vendor_topic", read->read.vendor_topic);
    if (read->read.recording_id)
        cJSON_AddNumberToObject(object, "recording_id", (double)read->read.recording_id);
    else cJSON_AddNullToObject(object, "recording_id");
    if (read->read.has_bounding_box) {
        cJSON *bbox = cJSON_AddObjectToObject(attributes, "bounding_box");
        cJSON_AddNumberToObject(bbox, "left", read->read.bbox_left);
        cJSON_AddNumberToObject(bbox, "top", read->read.bbox_top);
        cJSON_AddNumberToObject(bbox, "right", read->read.bbox_right);
        cJSON_AddNumberToObject(bbox, "bottom", read->read.bbox_bottom);
    } else {
        cJSON_AddNullToObject(attributes, "bounding_box");
    }
    return object;
}

static void audit_query(const http_request_t *req, const user_t *user,
                        const fleet_camera_t *camera,
                        const lpr_read_query_t *query, const char *fingerprint,
                        int result_count, const char *outcome,
                        bool exporting) {
    cJSON *context = cJSON_CreateObject();
    if (context) {
        const char *mode = query->match_mode == LPR_MATCH_EXACT ? "exact" :
                           query->match_mode == LPR_MATCH_PARTIAL ? "partial" : "none";
        cJSON_AddStringToObject(context, "match_mode", mode);
        cJSON_AddNumberToObject(context, "start_at", (double)query->start_at_ms);
        cJSON_AddNumberToObject(context, "end_at", (double)query->end_at_ms);
        cJSON_AddNumberToObject(context, "result_count", result_count);
        if (fingerprint && fingerprint[0])
            cJSON_AddStringToObject(context, "query_fingerprint", fingerprint);
    }
    audit_log_operation(req, user, exporting ? "lpr.export" : "lpr.search",
                        "camera",
                        camera ? camera->camera_uuid : NULL,
                        exporting ? "export" : "search", outcome, context);
    cJSON_Delete(context);
}

static void handle_lpr_query(const http_request_t *req, http_response_t *res,
                             bool exporting) {
    char body_text[LPR_SEARCH_BODY_MAX];
    if (!req || !res || http_request_get_body_str(req, body_text,
                                                   sizeof(body_text)) != 0) {
        http_response_set_json_error(res, 400, "Invalid JSON body");
        return;
    }
    cJSON *body = cJSON_Parse(body_text);
    secure_zero(body_text, sizeof(body_text));
    const cJSON *camera_item = body
        ? cJSON_GetObjectItemCaseSensitive(body, "camera_uuid") : NULL;
    if (!cJSON_IsObject(body) || !cJSON_IsString(camera_item) ||
        !lightnvr_uuid_is_valid(camera_item->valuestring)) {
        delete_query_body(body);
        http_response_set_json_error(res, 400, "camera_uuid is required");
        return;
    }

    user_t user;
    fleet_camera_t camera;
    authorization_evaluation_t evaluation;
    if (exporting &&
        !httpd_authorize_camera_identity_action_with_context(
            req, res, AUTHZ_LPR_EXPORT, camera_item->valuestring, NULL,
            &user, &camera, &evaluation)) {
        delete_query_body(body);
        return;
    }
    if (!httpd_authorize_camera_identity_action_with_context(
            req, res, AUTHZ_LPR_SEARCH, camera_item->valuestring, NULL,
            &user, &camera, &evaluation)) {
        delete_query_body(body);
        return;
    }
    user_t read_user;
    fleet_camera_t read_camera;
    authorization_evaluation_t read_evaluation;
    if (!httpd_authorize_camera_identity_action_with_context(
            req, res, AUTHZ_LPR_READ, camera_item->valuestring, NULL,
            &read_user, &read_camera, &read_evaluation)) {
        delete_query_body(body);
        return;
    }

    lpr_read_query_t query;
    memset(&query, 0, sizeof(query));
    snprintf(query.camera_uuid, sizeof(query.camera_uuid), "%s",
             camera_item->valuestring);
    const cJSON *mode = cJSON_GetObjectItemCaseSensitive(body, "match");
    const cJSON *plate = cJSON_GetObjectItemCaseSensitive(body, "plate");
    const cJSON *limit = cJSON_GetObjectItemCaseSensitive(body, "limit");
    int maximum_results = exporting ? LPR_EXPORT_RESULT_MAX : LPR_SEARCH_RESULT_MAX;
    query.limit = limit && cJSON_IsNumber(limit) ? limit->valueint
                                                 : (exporting ? 1000 : 50);
    bool valid = json_int64(body, "start_at", &query.start_at_ms) &&
                 json_int64(body, "end_at", &query.end_at_ms) &&
                 query.end_at_ms >= query.start_at_ms &&
                 query.end_at_ms - query.start_at_ms <= LPR_SEARCH_MAX_RANGE_MS &&
                 query.limit >= 1 && query.limit <= maximum_results;
    if (!mode) {
        query.match_mode = LPR_MATCH_NONE;
    } else if (cJSON_IsString(mode) && strcmp(mode->valuestring, "exact") == 0) {
        query.match_mode = LPR_MATCH_EXACT;
    } else if (cJSON_IsString(mode) && strcmp(mode->valuestring, "partial") == 0) {
        query.match_mode = LPR_MATCH_PARTIAL;
    } else {
        valid = false;
    }
    if (query.match_mode != LPR_MATCH_NONE) {
        if (!cJSON_IsString(plate) || !plate->valuestring ||
            strlen(plate->valuestring) >= sizeof(query.plate_query)) valid = false;
        else snprintf(query.plate_query, sizeof(query.plate_query), "%s",
                      plate->valuestring);
    }

    char query_fingerprint[65] = {0};
    if (valid && query.match_mode != LPR_MATCH_NONE) {
        char canonical[LPR_QUERY_MAX];
        uint8_t digest[LPR_CRYPTO_HMAC_SIZE];
        if (lpr_canonicalize_plate(query.plate_query, canonical,
                                   sizeof(canonical)) < 0 ||
            lpr_crypto_fingerprint(canonical, strlen(canonical), digest) != 0) {
            valid = false;
        } else {
            digest_hex(digest, query_fingerprint);
        }
        memset(canonical, 0, sizeof(canonical));
    }
    delete_query_body(body);

    if (!valid) {
        audit_query(req, &user, &camera, &query, query_fingerprint, 0,
                    "failure", exporting);
        http_response_set_json_error(res, 400, "Invalid LPR search");
        return;
    }

    lpr_read_t *reads = calloc((size_t)query.limit, sizeof(*reads));
    if (!reads) {
        audit_query(req, &user, &camera, &query, query_fingerprint, 0,
                    "failure", exporting);
        http_response_set_json_error(res, 500, "Out of memory");
        return;
    }
    int count = db_lpr_reads_search(&query, reads, (size_t)query.limit);
    if (count < 0) {
        secure_zero(reads, (size_t)query.limit * sizeof(*reads));
        free(reads);
        audit_query(req, &user, &camera, &query, query_fingerprint, 0,
                    "failure", exporting);
        http_response_set_json_error(res, 503, "Protected LPR storage unavailable");
        return;
    }

    cJSON *root = cJSON_CreateObject();
    cJSON *items = cJSON_CreateArray();
    if (!root || !items) {
        cJSON_Delete(root);
        cJSON_Delete(items);
        secure_zero(reads, (size_t)query.limit * sizeof(*reads));
        free(reads);
        audit_query(req, &user, &camera, &query, query_fingerprint, 0,
                    "failure", exporting);
        http_response_set_json_error(res, 500, "Failed to create response");
        return;
    }
    cJSON_AddItemToObject(root, "reads", items);
    for (int i = 0; i < count; ++i) {
        cJSON *item = read_json(&reads[i]);
        if (item) cJSON_AddItemToArray(items, item);
    }
    cJSON_AddNumberToObject(root, "count", count);
    char *encoded = cJSON_PrintUnformatted(root);
    wipe_response_plates(root);
    cJSON_Delete(root);
    secure_zero(reads, (size_t)query.limit * sizeof(*reads));
    free(reads);
    if (!encoded) {
        audit_query(req, &user, &camera, &query, query_fingerprint, 0,
                    "failure", exporting);
        http_response_set_json_error(res, 500, "Failed to serialize response");
        return;
    }
    http_response_set_json(res, 200, encoded);
    if (exporting) {
        http_response_add_header(
            res, "Content-Disposition",
            "attachment; filename=\"lightnvr-lpr-export.json\"");
    }
    secure_zero(encoded, strlen(encoded));
    free(encoded);
    audit_query(req, &user, &camera, &query, query_fingerprint, count,
                "success", exporting);
}

void handle_post_lpr_search(const http_request_t *req, http_response_t *res) {
    handle_lpr_query(req, res, false);
}

void handle_post_lpr_export(const http_request_t *req, http_response_t *res) {
    handle_lpr_query(req, res, true);
}

void handle_delete_lpr_read(const http_request_t *req, http_response_t *res) {
    char read_uuid[LPR_READ_UUID_SIZE];
    if (!req || !res ||
        http_request_extract_path_param(req, "/api/lpr/reads/", read_uuid,
                                        sizeof(read_uuid)) != 0 ||
        !lightnvr_uuid_is_valid(read_uuid)) {
        http_response_set_json_error(res, 400, "Invalid LPR read ID");
        return;
    }

    char camera_uuid[CAMERA_UUID_STRING_SIZE];
    if (db_lpr_read_get_camera_uuid(read_uuid, camera_uuid) != 0) {
        http_response_set_json_error(res, 404, "LPR read not found");
        return;
    }
    user_t user;
    fleet_camera_t camera;
    authorization_evaluation_t evaluation;
    if (!httpd_authorize_camera_identity_action_with_context(
            req, res, AUTHZ_LPR_DELETE, camera_uuid, NULL, &user, &camera,
            &evaluation)) {
        return;
    }

    int result = db_lpr_read_delete(read_uuid);
    cJSON *context = cJSON_CreateObject();
    if (context) cJSON_AddStringToObject(context, "read_id", read_uuid);
    audit_log_operation(req, &user, "lpr.delete", "camera", camera_uuid,
                        "delete", result == 0 ? "success" : "failure",
                        context);
    cJSON_Delete(context);
    if (result != 0) {
        http_response_set_json_error(res, 404, "LPR read not found");
        return;
    }
    http_response_set_json(res, 200, "{\"deleted\":true}");
}
