/*
 * SPDX-License-Identifier: GPL-3.0-or-later
 * Copyright (C) 2026 OpenSensor Engineering and LightNVR contributors
 *
 * This file is part of LightNVR <https://github.com/opensensor/lightNVR>.
 */

/**
 * INT_01 observation and integration API.
 *
 * Contract: docs/integrations/OBSERVATIONS_CONTRACT.md (sections 7-9, 11 and
 * 12). Every coded error body is {"error", "code", ...} (D2).
 *
 * Authorization always resolves the observation's capture-time camera_uuid
 * and evaluates it with httpd_evaluate_camera_identity_action(), never the
 * httpd_authorize_* helpers: those write 403 themselves, which would let an
 * opaque observation ID reveal that it exists. Unknown and unauthorized IDs
 * are therefore byte-identical 404s. The only 403 for a single observation is
 * the D4 snapshot case, where the caller can already see the metadata.
 *
 * The PUT classification route stays a 501 stub until L14.
 */

#define _GNU_SOURCE

#include "web/api_handlers_observations.h"

#include <ctype.h>
#include <errno.h>
#include <limits.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <strings.h>
#include <sys/stat.h>
#include <time.h>

#include "core/authorization.h"
#include "core/config.h"
#include "core/event_identity.h"
#include "core/version.h"
#include "database/db_auth.h"
#include "database/db_fleet_query.h"
#include "database/db_streams.h"
#include "database/db_system_settings.h"
#include "utils/strings.h"
#include "utils/uuid.h"
#include "web/audit_log.h"
#include "web/httpd_utils.h"

#define LOG_COMPONENT "ObservationsAPI"
#include "core/logger.h"

#define OBS_SOURCE_TEXT_SIZE 128
#define OBS_QUERY_VALUE_SIZE 1024        /* query_string is 1,024 bytes (D1) */
#define OBS_FILTER_CAMERAS_MAX 10
#define OBS_CAPABILITY_CAMERAS_MAX 1024
#define OBS_SNAPSHOT_RETRY_MAX 30
#define OBS_SNAPSHOT_CACHE_CONTROL "private, max-age=300"
#define OBS_HISTORY_RETENTION_DEFAULT 30
#define OBS_EVENT_TYPE "io.lightnvr.detection.observation_changed.v1"
#define OBS_ANNOTATIONS_MAX 8
#define URN_PREFIX "urn:lightnvr:"

/* ---- time, namespace and serialization helpers ----------------------- */

int observation_format_time(int64_t ms, char *out, size_t out_size) {
    if (!out || out_size < 25) return -1;
    int64_t seconds = ms / 1000;
    int millis = (int)(ms % 1000);
    if (millis < 0) {
        millis += 1000;
        seconds--;
    }
    time_t t = (time_t)seconds;
    struct tm tm;
    if (!gmtime_r(&t, &tm)) return -1;
    int n = snprintf(out, out_size, "%04d-%02d-%02dT%02d:%02d:%02d.%03dZ",
                     tm.tm_year + 1900, tm.tm_mon + 1, tm.tm_mday, tm.tm_hour, tm.tm_min,
                     tm.tm_sec, millis);
    return n > 0 && (size_t)n < out_size ? 0 : -1;
}

static bool digits(const char *text, int count, int *value) {
    int result = 0;
    for (int i = 0; i < count; i++) {
        if (!isdigit((unsigned char)text[i])) return false;
        result = result * 10 + (text[i] - '0');
    }
    *value = result;
    return true;
}

int observation_parse_time(const char *text, int64_t *ms) {
    if (!text || !ms) return -1;
    int year, month, day, hour, minute, second;
    if (strnlen(text, 20) < 20 || !digits(text, 4, &year) || text[4] != '-' ||
        !digits(text + 5, 2, &month) || text[7] != '-' || !digits(text + 8, 2, &day) ||
        (text[10] != 'T' && text[10] != 't') || !digits(text + 11, 2, &hour) ||
        text[13] != ':' || !digits(text + 14, 2, &minute) || text[16] != ':' ||
        !digits(text + 17, 2, &second))
        return -1;
    const char *p = text + 19;
    int millis = 0;
    if (*p == '.') {
        p++;
        int count = 0;
        while (isdigit((unsigned char)*p)) {
            if (count < 3) millis = millis * 10 + (*p - '0');
            count++;
            p++;
        }
        if (count == 0 || count > 9) return -1;
        for (int i = count; i < 3; i++) millis *= 10;
    }
    int offset_minutes = 0;
    if (*p == 'Z' || *p == 'z') {
        p++;
    } else if (*p == '+' || *p == '-') {
        int oh, om;
        if (!digits(p + 1, 2, &oh) || p[3] != ':' || !digits(p + 4, 2, &om) || oh > 23 ||
            om > 59)
            return -1;
        offset_minutes = (oh * 60 + om) * (*p == '-' ? -1 : 1);
        p += 6;
    } else {
        return -1;
    }
    if (*p != '\0') return -1;
    if (year < 1970 || month < 1 || month > 12 || day < 1 || day > 31 || hour > 23 ||
        minute > 59 || second > 59)
        return -1;
    struct tm tm = {0};
    tm.tm_year = year - 1900;
    tm.tm_mon = month - 1;
    tm.tm_mday = day;
    tm.tm_hour = hour;
    tm.tm_min = minute;
    tm.tm_sec = second;
    time_t t = timegm(&tm);
    struct tm check;
    if (t == (time_t)-1 || !gmtime_r(&t, &check) || check.tm_mday != day ||
        check.tm_mon != month - 1)
        return -1;  /* e.g. 2026-02-30 */
    *ms = ((int64_t)t - (int64_t)offset_minutes * 60) * 1000 + millis;
    return 0;
}

int observation_writer_namespace(const char *username, char out[OBS_WRITER_NAMESPACE_SIZE]) {
    static const char *const reserved[] = {"lightnvr", "system", "owner", "local"};
    if (!out) return -1;
    out[0] = '\0';
    if (!username) return -1;
    size_t length = strlen(username);
    if (length < 1 || length > OBS_WRITER_NAMESPACE_SIZE - 1) return -1;
    char normalized[OBS_WRITER_NAMESPACE_SIZE];
    for (size_t i = 0; i < length; i++) {
        unsigned char c = (unsigned char)username[i];
        if (c >= 'A' && c <= 'Z') c = (unsigned char)(c - 'A' + 'a');
        bool allowed = (c >= 'a' && c <= 'z') || (c >= '0' && c <= '9') || c == '_' ||
                       c == '.' || c == '-';
        normalized[i] = allowed ? (char)c : '-';
    }
    normalized[length] = '\0';
    char first = normalized[0];
    if (!((first >= 'a' && first <= 'z') || (first >= '0' && first <= '9'))) return -1;
    for (size_t i = 0; i < sizeof(reserved) / sizeof(reserved[0]); i++)
        if (strcmp(normalized, reserved[i]) == 0) return -2;
    memcpy(out, normalized, length + 1);
    return 0;
}

static cJSON *time_or_null(int64_t ms) {
    char text[OBS_TIME_TEXT_SIZE];
    if (ms <= 0 || observation_format_time(ms, text, sizeof(text)) != 0) return cJSON_CreateNull();
    return cJSON_CreateString(text);
}

static cJSON *text_or_null(const char *value) {
    return value && value[0] ? cJSON_CreateString(value) : cJSON_CreateNull();
}

static cJSON *evidence_to_json(const observation_record_t *record) {
    const observation_asset_record_t *asset = &record->asset;
    cJSON *evidence = cJSON_CreateObject();
    if (!evidence) return NULL;
    /* C3 conventions: evidence that never had media (no origin) has every
     * media field null; pending keeps origin/media_type/captured_at/transform
     * and nulls the size and expiry, which are only known once ready. */
    bool has_media = asset->origin[0] != '\0';
    bool sized = has_media && asset->state != OBS_ASSET_PENDING;
    bool reasoned = asset->state == OBS_ASSET_UNAVAILABLE || asset->state == OBS_ASSET_EXPIRED;
    cJSON_AddStringToObject(evidence, "asset_id", asset->uuid);
    cJSON_AddStringToObject(evidence, "state", observation_asset_state_name(asset->state));
    cJSON_AddItemToObject(evidence, "reason",
                          text_or_null(reasoned ? asset->reason : NULL));
    cJSON_AddItemToObject(evidence, "origin", text_or_null(has_media ? asset->origin : NULL));
    cJSON_AddItemToObject(evidence, "media_type",
                          text_or_null(has_media ? asset->media_type : NULL));
    if (sized && asset->width > 0) cJSON_AddNumberToObject(evidence, "width", asset->width);
    else cJSON_AddNullToObject(evidence, "width");
    if (sized && asset->height > 0) cJSON_AddNumberToObject(evidence, "height", asset->height);
    else cJSON_AddNullToObject(evidence, "height");
    if (sized && asset->has_bytes) cJSON_AddNumberToObject(evidence, "bytes", (double)asset->bytes);
    else cJSON_AddNullToObject(evidence, "bytes");
    cJSON_AddItemToObject(evidence, "captured_at",
                          has_media ? time_or_null(asset->captured_at_ms) : cJSON_CreateNull());
    cJSON_AddItemToObject(evidence, "expires_at",
                          sized ? time_or_null(asset->expires_at_ms) : cJSON_CreateNull());
    cJSON *transform = has_media && asset->transform_json[0]
        ? cJSON_Parse(asset->transform_json) : NULL;
    if (transform && !cJSON_IsObject(transform)) {
        cJSON_Delete(transform);
        transform = NULL;
    }
    cJSON_AddItemToObject(evidence, "transform", transform ? transform : cJSON_CreateNull());
    if (asset->state == OBS_ASSET_READY) {
        char url[96];
        snprintf(url, sizeof(url), "/api/observations/%s/snapshot", record->uuid);
        cJSON_AddStringToObject(evidence, "snapshot_url", url);
    }
    /* asset->path is internal and never serialized (contract section 2). */
    return evidence;
}

cJSON *observation_record_to_json(const observation_record_t *record, const char *source) {
    if (!record || !source) return NULL;
    cJSON *root = cJSON_CreateObject();
    if (!root) return NULL;
    cJSON_AddStringToObject(root, "id", record->uuid);
    cJSON_AddStringToObject(root, "source", source);
    cJSON_AddStringToObject(root, "camera_uuid", record->camera_uuid);
    cJSON_AddStringToObject(root, "stream_name_at_capture", record->stream_name_at_capture);
    cJSON_AddStringToObject(root, "batch_id", record->batch_id);
    cJSON_AddNumberToObject(root, "object_index", record->object_index);
    cJSON_AddItemToObject(root, "observed_at", time_or_null(record->observed_at_ms));
    cJSON_AddItemToObject(root, "created_at", time_or_null(record->created_at_ms));

    cJSON *detector = cJSON_AddObjectToObject(root, "detector");
    if (detector) {
        cJSON_AddStringToObject(detector, "engine_key", record->engine_key);
        cJSON_AddStringToObject(detector, "engine_type", record->engine_type);
        cJSON_AddItemToObject(detector, "model", text_or_null(record->model));
        cJSON_AddItemToObject(detector, "model_version", text_or_null(record->model_version));
    }
    cJSON_AddStringToObject(root, "label", record->label);
    cJSON_AddNumberToObject(root, "confidence", record->confidence);
    if (record->has_box) {
        cJSON *box = cJSON_AddObjectToObject(root, "box");
        if (box) {
            cJSON_AddStringToObject(box, "format", "xywh_normalized");
            cJSON_AddNumberToObject(box, "x", record->box_x);
            cJSON_AddNumberToObject(box, "y", record->box_y);
            cJSON_AddNumberToObject(box, "width", record->box_width);
            cJSON_AddNumberToObject(box, "height", record->box_height);
        }
    } else {
        cJSON_AddNullToObject(root, "box");
    }
    cJSON *evidence = evidence_to_json(record);
    if (evidence) cJSON_AddItemToObject(root, "evidence", evidence);
    if (record->has_track && record->track_id[0]) {
        cJSON *track = cJSON_AddObjectToObject(root, "track");
        if (track) {
            cJSON_AddStringToObject(track, "id", record->track_id);
            cJSON_AddStringToObject(track, "namespace", record->track_namespace);
            cJSON_AddStringToObject(track, "session", record->track_session);
        }
    } else {
        cJSON_AddNullToObject(root, "track");
    }
    if (record->recording_id > 0) {
        cJSON *recording = cJSON_AddObjectToObject(root, "recording_reference");
        if (recording)
            cJSON_AddNumberToObject(recording, "recording_id", (double)record->recording_id);
    } else {
        cJSON_AddNullToObject(root, "recording_reference");
    }
    cJSON_AddNumberToObject(root, "revision", record->revision);
    /* P1 has no annotations; L14 fills these lists (contract section 10.5). */
    cJSON *classifications = cJSON_AddObjectToObject(root, "classifications");
    if (classifications) {
        cJSON_AddArrayToObject(classifications, "automated");
        cJSON_AddArrayToObject(classifications, "owner_confirmed");
    }
    if (!detector || !evidence || !classifications) {
        cJSON_Delete(root);
        return NULL;
    }
    return root;
}

/* ---- response helpers -------------------------------------------------- */

static void send_json(http_response_t *res, int status, cJSON *body) {
    char *json = body ? cJSON_PrintUnformatted(body) : NULL;
    cJSON_Delete(body);
    if (!json) {
        http_response_set_json_error(res, 500, "Internal server error");
        return;
    }
    http_response_set_json(res, status, json);
    cJSON_free(json);
}

static cJSON *error_body(const char *message, const char *code) {
    cJSON *body = cJSON_CreateObject();
    if (body) {
        cJSON_AddStringToObject(body, "error", message);
        cJSON_AddStringToObject(body, "code", code);
    }
    return body;
}

static void send_error(http_response_t *res, int status, const char *code, const char *message) {
    send_json(res, status, error_body(message, code));
}

static void send_invalid(http_response_t *res, const char *field, const char *message) {
    cJSON *body = error_body(message, "invalid_request");
    if (body && field) cJSON_AddStringToObject(body, "field", field);
    send_json(res, 400, body);
}

static void send_internal(http_response_t *res) {
    send_error(res, 500, "internal_error", "Internal server error");
}

static void send_not_found(http_response_t *res) {
    send_error(res, 404, "observation_not_found", "Observation not found");
}

static void send_unavailable(http_response_t *res) {
    http_response_add_header(res, "Retry-After", "5");
    send_error(res, 503, "service_unavailable", "Service temporarily unavailable");
}

static bool authenticate(const http_request_t *req, http_response_t *res, user_t *user) {
    memset(user, 0, sizeof(*user));
    if (!httpd_check_action_access(req, user)) {
        http_response_set_json_error(res, 401, "Unauthorized");
        return false;
    }
    return true;
}

static bool installation_source(char source[OBS_SOURCE_TEXT_SIZE]) {
    return event_identity_get_source(source, OBS_SOURCE_TEXT_SIZE) == 0 &&
           strncmp(source, URN_PREFIX, strlen(URN_PREFIX)) == 0;
}

static void lowercase(char *text) {
    for (; *text; text++)
        if (*text >= 'A' && *text <= 'Z') *text = (char)(*text - 'A' + 'a');
}

/* Observation ID from /api/observations/{id}[suffix]; lowercased. */
static bool path_observation_id(const http_request_t *req, const char *suffix,
                                char id[OBS_UUID_SIZE]) {
    char param[256];
    if (http_request_extract_path_param(req, "/api/observations/", param, sizeof(param)) != 0)
        return false;
    if (suffix) {
        size_t length = strlen(param), suffix_length = strlen(suffix);
        if (length <= suffix_length || strcmp(param + length - suffix_length, suffix) != 0)
            return false;
        param[length - suffix_length] = '\0';
    }
    lowercase(param);
    if (!lightnvr_uuid_is_valid(param) || strlen(param) != OBS_UUID_SIZE - 1) return false;
    memcpy(id, param, OBS_UUID_SIZE);
    return true;
}

/* Audit a camera-scoped decision with the existing read-decision modes. Only
 * the camera UUID of the stub is read by the audit path. */
static void audit_camera_decision(const http_request_t *req, const user_t *user,
                                  authorization_action_t action, const char *camera_uuid,
                                  const authorization_evaluation_t *evaluation,
                                  const char *outcome) {
    fleet_camera_t *camera = calloc(1, sizeof(*camera));
    if (!camera) return;
    safe_strcpy(camera->camera_uuid, camera_uuid, sizeof(camera->camera_uuid), 0);
    audit_log_authorization(req, user, action, camera, evaluation, outcome);
    free(camera);
}

/* Contract section 12.2: snapshot fetches are audited as snapshot.create
 * decisions on the observation (target_type "observation"), recording the
 * camera, asset and outcome. Allowed reads honour the per-action decision
 * mode: off records nothing, summarize folds into the camera summary (a
 * summary row cannot carry an observation ID), record writes the full row.
 * Never records bytes, filesystem paths or credentials. */
static void audit_snapshot(const http_request_t *req, const user_t *user,
                           const observation_record_t *record,
                           const authorization_evaluation_t *evaluation,
                           const char *outcome) {
    if (strcmp(outcome, "allowed") == 0) {
        audit_decision_mode_t mode = audit_log_get_decision_mode(AUTHZ_SNAPSHOT_CREATE);
        if (mode == AUDIT_DECISION_MODE_OFF) return;
        if (mode == AUDIT_DECISION_MODE_SUMMARIZE) {
            audit_camera_decision(req, user, AUTHZ_SNAPSHOT_CREATE, record->camera_uuid,
                                  evaluation, outcome);
            return;
        }
    }
    const authorization_action_metadata_t *metadata =
        authorization_action_metadata(AUTHZ_SNAPSHOT_CREATE);
    cJSON *details = cJSON_CreateObject();
    if (details) {
        cJSON_AddStringToObject(details, "event_type", "authorization.decision");
        cJSON_AddStringToObject(details, "method", req->method_str);
        cJSON_AddStringToObject(details, "path", req->path);
        cJSON_AddStringToObject(details, "camera_uuid", record->camera_uuid);
        cJSON_AddStringToObject(details, "asset_id", record->asset.uuid);
        cJSON_AddStringToObject(details, "evidence_state",
                                observation_asset_state_name(record->asset.state));
        if (evaluation) {
            cJSON_AddStringToObject(details, "decision_source",
                                    authorization_decision_source_name(evaluation->source));
            cJSON_AddStringToObject(details, "explanation", evaluation->explanation);
        }
    }
    audit_log_append(req, user, metadata ? metadata->key : "snapshot.create", "observation",
                     record->uuid, outcome, details);
    cJSON_Delete(details);
}

/* Load the fleet and reduce it to the cameras where the user holds action.
 * The caller frees *cameras. */
static int authorized_cameras(const user_t *user, authorization_action_t action,
                              fleet_camera_t **cameras, int *count) {
    *cameras = NULL;
    *count = 0;
    if (db_fleet_camera_load_readonly(cameras, count) != 0) return -1;
    if (*count > 0 && authorization_filter_cameras(user, action, *cameras, count) != 0) {
        free(*cameras);
        *cameras = NULL;
        *count = 0;
        return -1;
    }
    return 0;
}

static bool camera_in(const fleet_camera_t *cameras, int count, const char *uuid) {
    for (int i = 0; i < count; i++)
        if (strcmp(cameras[i].camera_uuid, uuid) == 0) return true;
    return false;
}

/* ---- GET /api/integrations/capabilities ------------------------------ */

static void add_operation(cJSON *operations, const char *name, bool supported,
                          bool authorized, const char *reason) {
    cJSON *op = cJSON_AddObjectToObject(operations, name);
    if (!op) return;
    cJSON_AddBoolToObject(op, "supported", supported);
    cJSON_AddBoolToObject(op, "authorized", authorized);
    if (reason && (!supported || !authorized)) cJSON_AddStringToObject(op, "reason", reason);
}

static int history_retention_days(void) {
    char value[32] = {0};
    if (db_get_system_setting("observation_history_retention_days", value, sizeof(value)) != 0)
        return OBS_HISTORY_RETENTION_DEFAULT;
    char *end = NULL;
    errno = 0;
    long days = strtol(value, &end, 10);
    if (errno != 0 || end == value || *end != '\0' || days < 1 || days > 365)
        return OBS_HISTORY_RETENTION_DEFAULT;
    return (int)days;
}

/* oldest_observed_at is reported by the cursor_expired path; ask for it with
 * a cursor from a deliberately different DB generation and no cameras, which
 * expires before any change row is scanned. */
static int history_bounds(cJSON *history) {
    observation_meta_t meta;
    if (db_observation_meta_get(&meta) != DB_OBS_OK) return -1;
    char oldest[OBS_CURSOR_SIZE];
    if (observation_cursor_encode(meta.oldest_seq - 1, meta.epoch_uuid, oldest,
                                  sizeof(oldest)) != 0)
        return -1;
    char other_epoch[OBS_UUID_SIZE];
    do {
        if (lightnvr_uuid_generate_v4(other_epoch) != 0) return -1;
    } while (strcmp(other_epoch, meta.epoch_uuid) == 0);
    char probe[OBS_CURSOR_SIZE];
    if (observation_cursor_encode(0, other_epoch, probe, sizeof(probe)) != 0) return -1;
    observation_list_query_t query;
    memset(&query, 0, sizeof(query));
    observation_page_t page;
    if (db_observations_list(&query, probe, &page) != DB_OBS_CURSOR_EXPIRED) {
        db_observation_page_free(&page);
        return -1;
    }
    cJSON_AddStringToObject(history, "oldest_cursor", oldest);
    cJSON_AddItemToObject(history, "oldest_observed_at",
                          page.has_oldest_observed_at ? time_or_null(page.oldest_observed_at_ms)
                                                      : cJSON_CreateNull());
    db_observation_page_free(&page);
    return 0;
}

static bool camera_evidence_enabled(const stream_config_t *streams, int count, const char *uuid) {
    for (int i = 0; streams && i < count; i++)
        if (strcmp(streams[i].camera_uuid, uuid) == 0) return streams[i].observation_evidence_enabled;
    return false;
}

void handle_get_integration_capabilities(const http_request_t *req, http_response_t *res) {
    user_t user;
    if (!authenticate(req, res, &user)) return;
    char source[OBS_SOURCE_TEXT_SIZE];
    if (!installation_source(source)) {
        send_unavailable(res);
        return;
    }

    fleet_camera_t *viewable = NULL, *snapshot = NULL;
    int viewable_count = 0, snapshot_count = 0;
    if (authorized_cameras(&user, AUTHZ_LIVE_VIEW, &viewable, &viewable_count) != 0 ||
        authorized_cameras(&user, AUTHZ_SNAPSHOT_CREATE, &snapshot, &snapshot_count) != 0) {
        free(viewable);
        free(snapshot);
        send_error(res, 500, "internal_error", "Authorization policy evaluation failed");
        return;
    }
    int slots = configured_stream_slots();
    stream_config_t *streams = calloc((size_t)slots, sizeof(*streams));
    int stream_count = streams ? get_all_stream_configs_readonly(streams, slots) : -1;

    cJSON *root = cJSON_CreateObject();
    cJSON *history = NULL;
    bool ok = root && stream_count >= 0;
    if (ok) {
        cJSON *versions = cJSON_AddObjectToObject(root, "contract_versions");
        cJSON_AddNumberToObject(versions, "observations", 1);
        cJSON_AddNumberToObject(versions, "classifications", 1);
        cJSON_AddStringToObject(versions, "observation_event", OBS_EVENT_TYPE);

        cJSON *src = cJSON_AddObjectToObject(root, "source");
        cJSON_AddStringToObject(src, "id", source);
        cJSON_AddStringToObject(src, "kind", "lightnvr");
        cJSON_AddStringToObject(src, "installation_uuid", source + strlen(URN_PREFIX));
        cJSON_AddStringToObject(src, "product_version", LIGHTNVR_VERSION_STRING);

        char writer[OBS_WRITER_NAMESPACE_SIZE];
        cJSON *principal = cJSON_AddObjectToObject(root, "principal");
        cJSON_AddItemToObject(principal, "writer_namespace",
                              observation_writer_namespace(user.username, writer) == 0
                                  ? cJSON_CreateString(writer) : cJSON_CreateNull());

        cJSON *operations = cJSON_AddObjectToObject(root, "operations");
        add_operation(operations, "observations.history", true, viewable_count > 0,
                      "not_authorized");
        add_operation(operations, "observations.snapshot", true, snapshot_count > 0,
                      "not_authorized");
        /* Supported once L14 lands. */
        add_operation(operations, "observations.annotate", false, false,
                      "not_available_in_this_version");
        add_operation(operations, "track_lifecycle", false, false, "not_implemented");
        add_operation(operations, "recorded_frame", false, false, "not_implemented");
        add_operation(operations, "event_clip", false, false, "not_implemented");
        add_operation(operations, "range_clip", false, false, "not_implemented");

        cJSON *limits = cJSON_AddObjectToObject(root, "limits");
        cJSON_AddNumberToObject(limits, "history_page_default", OBS_LIST_DEFAULT_LIMIT);
        cJSON_AddNumberToObject(limits, "history_page_max", OBS_LIST_MAX_LIMIT);
        cJSON_AddNumberToObject(limits, "history_filter_cameras_max", OBS_FILTER_CAMERAS_MAX);
        cJSON_AddNumberToObject(limits, "history_filter_labels_max", OBS_QUERY_MAX_LABELS);
        cJSON_AddNumberToObject(limits, "max_objects_per_batch", OBS_MAX_BATCH);
        cJSON_AddNumberToObject(limits, "label_max_bytes", OBS_LABEL_SIZE - 1);
        cJSON_AddNumberToObject(limits, "snapshot_retry_after_max_seconds",
                                OBS_SNAPSHOT_RETRY_MAX);
        cJSON_AddNumberToObject(limits, "annotations_per_observation_max", OBS_ANNOTATIONS_MAX);

        history = cJSON_AddObjectToObject(root, "history");
        cJSON_AddNumberToObject(history, "retention_days", history_retention_days());
        ok = history && history_bounds(history) == 0;

        cJSON *delivery = cJSON_AddObjectToObject(root, "delivery");
        cJSON_AddBoolToObject(delivery, "mqtt_enabled", g_config.mqtt_enabled);
        if (g_config.mqtt_topic_prefix[0]) {
            char topic[256];
            snprintf(topic, sizeof(topic), "%s/v1/events/%s/{camera_uuid}",
                     g_config.mqtt_topic_prefix, OBS_EVENT_TYPE);
            cJSON_AddStringToObject(delivery, "topic", topic);
        } else {
            cJSON_AddNullToObject(delivery, "topic");
        }

        /* Cameras with live.view or snapshot.create, in fleet order. */
        cJSON *cameras = cJSON_AddArrayToObject(root, "cameras");
        fleet_camera_t *all = NULL;
        int all_count = 0;
        if (db_fleet_camera_load_readonly(&all, &all_count) != 0) ok = false;
        int listed = 0;
        for (int i = 0; ok && i < all_count && listed < OBS_CAPABILITY_CAMERAS_MAX; i++) {
            const char *uuid = all[i].camera_uuid;
            bool live = camera_in(viewable, viewable_count, uuid);
            bool snap = camera_in(snapshot, snapshot_count, uuid);
            if (!live && !snap) continue;
            cJSON *camera = cJSON_CreateObject();
            cJSON_AddStringToObject(camera, "camera_uuid", uuid);
            cJSON_AddBoolToObject(camera, "evidence_enabled",
                                  camera_evidence_enabled(streams, stream_count, uuid));
            cJSON *actions = cJSON_AddArrayToObject(camera, "actions");
            if (live) cJSON_AddItemToArray(actions, cJSON_CreateString("live.view"));
            if (snap) cJSON_AddItemToArray(actions, cJSON_CreateString("snapshot.create"));
            cJSON_AddItemToArray(cameras, camera);
            listed++;
        }
        free(all);
    }
    free(streams);
    free(viewable);
    free(snapshot);
    if (!ok) {
        cJSON_Delete(root);
        send_internal(res);
        return;
    }
    send_json(res, 200, root);
}

/* ---- GET /api/observations ------------------------------------------- */

static bool query_has(const char *query, const char *name) {
    size_t length = strlen(name);
    for (const char *p = query; p && *p; ) {
        if (strncmp(p, name, length) == 0 && p[length] == '=') return true;
        p = strchr(p, '&');
        if (p) p++;
    }
    return false;
}

/* Returns the decoded length, -1 when absent and -2 when present but not
 * decodable. The query string is bounded at 1,024 bytes, so value (1,024
 * bytes) never truncates. Only the first occurrence is read (D1). */
static int query_value(const http_request_t *req, const char *name, char *value, size_t size) {
    value[0] = '\0';
    if (!query_has(req->query_string, name)) return -1;
    int n = http_request_get_query_param(req, name, value, size);
    return n < 0 ? -2 : n;
}

static bool parse_limit(const char *text, int *limit) {
    char *end = NULL;
    errno = 0;
    long value = strtol(text, &end, 10);
    if (errno != 0 || end == text || *end != '\0' || !isdigit((unsigned char)text[0]))
        return false;
    if (value < 1 || value > OBS_LIST_MAX_LIMIT) return false;
    *limit = (int)value;
    return true;
}

static bool parse_labels(char *text, observation_list_query_t *query) {
    size_t count = 0;
    char *start = text;
    for (;;) {
        char *comma = strchr(start, ',');
        size_t length = comma ? (size_t)(comma - start) : strlen(start);
        if (length < 1 || length > OBS_LABEL_SIZE - 1 || count >= OBS_QUERY_MAX_LABELS)
            return false;
        for (size_t i = 0; i < length; i++)
            if ((unsigned char)start[i] < 0x20 || start[i] == 0x7f) return false;
        memcpy(query->labels[count], start, length);
        query->labels[count][length] = '\0';
        count++;
        if (!comma) break;
        start = comma + 1;
    }
    query->label_count = count;
    return true;
}

static bool parse_cameras(char *text, char (*cameras)[OBS_UUID_SIZE], size_t *count) {
    *count = 0;
    char *start = text;
    for (;;) {
        char *comma = strchr(start, ',');
        size_t length = comma ? (size_t)(comma - start) : strlen(start);
        if (length != OBS_UUID_SIZE - 1 || *count >= OBS_FILTER_CAMERAS_MAX) return false;
        memcpy(cameras[*count], start, length);
        cameras[*count][length] = '\0';
        lowercase(cameras[*count]);
        if (!lightnvr_uuid_is_valid(cameras[*count])) return false;
        (*count)++;
        if (!comma) break;
        start = comma + 1;
    }
    return true;
}

static void send_cursor_expired(http_response_t *res, const observation_page_t *page) {
    cJSON *body = error_body("Cursor is older than retained observation history",
                             "cursor_expired");
    if (body) {
        cJSON_AddStringToObject(body, "oldest_cursor", page->oldest_cursor);
        cJSON_AddItemToObject(body, "oldest_observed_at",
                              page->has_oldest_observed_at
                                  ? time_or_null(page->oldest_observed_at_ms)
                                  : cJSON_CreateNull());
    }
    send_json(res, 410, body);
}

void handle_get_observations(const http_request_t *req, http_response_t *res) {
    user_t user;
    if (!authenticate(req, res, &user)) return;
    char source[OBS_SOURCE_TEXT_SIZE];
    if (!installation_source(source)) {
        send_unavailable(res);
        return;
    }

    observation_list_query_t query;
    memset(&query, 0, sizeof(query));
    char value[OBS_QUERY_VALUE_SIZE];
    int n;

    if ((n = query_value(req, "limit", value, sizeof(value))) != -1 &&
        (n <= 0 || !parse_limit(value, &query.limit))) {
        send_invalid(res, "limit", "limit must be an integer from 1 to 500");
        return;
    }
    if ((n = query_value(req, "label", value, sizeof(value))) != -1 &&
        (n <= 0 || !parse_labels(value, &query))) {
        send_invalid(res, "label", "label must list 1 to 8 labels of 1 to 31 bytes");
        return;
    }
    if ((n = query_value(req, "since", value, sizeof(value))) != -1) {
        if (n <= 0 || observation_parse_time(value, &query.since_ms) != 0) {
            send_invalid(res, "since", "since must be an RFC 3339 time");
            return;
        }
        query.has_since = true;
    }
    if ((n = query_value(req, "until", value, sizeof(value))) != -1) {
        if (n <= 0 || observation_parse_time(value, &query.until_ms) != 0) {
            send_invalid(res, "until", "until must be an RFC 3339 time");
            return;
        }
        query.has_until = true;
    }
    if (query.has_since && query.has_until && query.since_ms >= query.until_ms) {
        send_invalid(res, "until", "since must be before until");
        return;
    }
    char cursor[OBS_QUERY_VALUE_SIZE];
    bool has_cursor = false;
    if ((n = query_value(req, "cursor", cursor, sizeof(cursor))) != -1) {
        if (n <= 0) {
            send_invalid(res, "cursor", "cursor must not be empty");
            return;
        }
        has_cursor = true;
    }
    char requested[OBS_FILTER_CAMERAS_MAX][OBS_UUID_SIZE];
    size_t requested_count = 0;
    if ((n = query_value(req, "camera_uuid", value, sizeof(value))) != -1 &&
        (n <= 0 || !parse_cameras(value, requested, &requested_count))) {
        send_invalid(res, "camera_uuid", "camera_uuid must list 1 to 10 camera UUIDs");
        return;
    }

    /* Authorization: intersect with the cameras where the principal holds
     * live.view. An explicit camera outside that set (unknown included) is a
     * 403, matching the investigation-scope precedent. */
    fleet_camera_t *cameras = NULL;
    int camera_count = 0;
    if (authorized_cameras(&user, AUTHZ_LIVE_VIEW, &cameras, &camera_count) != 0) {
        send_error(res, 500, "internal_error", "Authorization policy evaluation failed");
        return;
    }
    char (*scope)[OBS_UUID_SIZE] = NULL;
    if (requested_count > 0) {
        for (size_t i = 0; i < requested_count; i++) {
            if (!camera_in(cameras, camera_count, requested[i])) {
                audit_camera_decision(req, &user, AUTHZ_LIVE_VIEW, requested[i], NULL, "denied");
                free(cameras);
                send_error(res, 403, "forbidden", "Forbidden");
                return;
            }
        }
        query.camera_uuids = (const char (*)[OBS_UUID_SIZE])requested;
        query.camera_count = requested_count;
    } else if (camera_count > 0) {
        scope = calloc((size_t)camera_count, sizeof(*scope));
        if (!scope) {
            free(cameras);
            send_internal(res);
            return;
        }
        for (int i = 0; i < camera_count; i++)
            safe_strcpy(scope[i], cameras[i].camera_uuid, OBS_UUID_SIZE, 0);
        query.camera_uuids = (const char (*)[OBS_UUID_SIZE])scope;
        query.camera_count = (size_t)camera_count;
    }
    free(cameras);

    observation_page_t page;
    int rc = db_observations_list(&query, has_cursor ? cursor : NULL, &page);
    free(scope);
    switch (rc) {
    case DB_OBS_OK:
        break;
    case DB_OBS_INVALID_CURSOR:
        send_error(res, 400, "invalid_cursor", "Malformed cursor");
        return;
    case DB_OBS_CURSOR_EXPIRED:
        send_cursor_expired(res, &page);
        db_observation_page_free(&page);
        return;
    case DB_OBS_INVALID:
        send_invalid(res, NULL, "Invalid observation query");
        return;
    default:
        send_internal(res);
        return;
    }

    cJSON *body = cJSON_CreateObject();
    cJSON *items = body ? cJSON_AddArrayToObject(body, "items") : NULL;
    bool ok = items != NULL;
    for (size_t i = 0; ok && i < page.count; i++) {
        cJSON *item = observation_record_to_json(&page.items[i], source);
        if (item) cJSON_AddItemToArray(items, item);
        else ok = false;
    }
    if (ok) {
        cJSON_AddStringToObject(body, "next_cursor", page.next_cursor);
        cJSON_AddBoolToObject(body, "has_more", page.has_more);
    }
    db_observation_page_free(&page);
    if (!ok) {
        cJSON_Delete(body);
        send_internal(res);
        return;
    }
    send_json(res, 200, body);
}

/* ---- GET /api/observations/{id} --------------------------------------- */

/* Load the observation named by the path. Returns 1 when found, 0 when
 * unknown (or its capture camera was deleted), and -1 after writing an error
 * response. Authorization is the caller's. */
static int load_observation(const http_request_t *req, http_response_t *res,
                            const char *suffix, observation_record_t *record) {
    char id[OBS_UUID_SIZE];
    if (!path_observation_id(req, suffix, id)) {
        send_invalid(res, "id", "Observation ID must be a UUID");
        return -1;
    }
    int rc = db_observation_get(id, record);
    if (rc == DB_OBS_NOT_FOUND) return 0;
    if (rc != DB_OBS_OK) {
        send_internal(res);
        return -1;
    }
    return 1;
}

void handle_get_observation(const http_request_t *req, http_response_t *res) {
    user_t user;
    if (!authenticate(req, res, &user)) return;
    char source[OBS_SOURCE_TEXT_SIZE];
    if (!installation_source(source)) {
        send_unavailable(res);
        return;
    }
    observation_record_t *record = calloc(1, sizeof(*record));
    if (!record) {
        send_internal(res);
        return;
    }
    int found = load_observation(req, res, NULL, record);
    if (found <= 0) {
        if (found == 0) send_not_found(res);
        free(record);
        return;
    }
    authorization_evaluation_t evaluation;
    int rc = httpd_evaluate_camera_identity_action(&user, AUTHZ_LIVE_VIEW, record->camera_uuid,
                                                   NULL, &evaluation);
    if (rc < 0) {
        audit_camera_decision(req, &user, AUTHZ_LIVE_VIEW, record->camera_uuid, NULL, "error");
        send_internal(res);
        free(record);
        return;
    }
    if (rc > 0 || evaluation.decision != AUTHZ_DECISION_ALLOW) {
        if (rc == 0)
            audit_camera_decision(req, &user, AUTHZ_LIVE_VIEW, record->camera_uuid, &evaluation,
                                  "denied");
        send_not_found(res);
        free(record);
        return;
    }
    audit_camera_decision(req, &user, AUTHZ_LIVE_VIEW, record->camera_uuid, &evaluation,
                          "allowed");
    cJSON *body = observation_record_to_json(record, source);
    free(record);
    if (!body) {
        send_internal(res);
        return;
    }
    send_json(res, 200, body);
}

/* ---- GET /api/observations/{id}/snapshot ------------------------------ */

/* Resolve the stored asset path (relative to storage_path, or absolute under
 * it) to a file under <storage_path>/observations/. Rejects anything else so a
 * corrupt row can never serve an arbitrary file. */
static int resolve_asset_file(const char *stored, char *out, size_t size) {
    char root[MAX_PATH_LENGTH];
    safe_strcpy(root, g_config.storage_path, sizeof(root), 0);
    size_t root_length = strlen(root);
    while (root_length > 1 && root[root_length - 1] == '/') root[--root_length] = '\0';
    if (!stored || !stored[0] || root_length == 0) return -1;
    const char *relative = stored;
    if (stored[0] == '/') {
        if (strncmp(stored, root, root_length) != 0 || stored[root_length] != '/') return -1;
        relative = stored + root_length + 1;
    }
    if (strncmp(relative, "observations/", strlen("observations/")) != 0) return -1;
    if (strchr(relative, '\\') || strstr(relative, "//")) return -1;
    for (const char *segment = relative; segment; ) {
        if (strncmp(segment, "..", 2) == 0 && (segment[2] == '/' || segment[2] == '\0'))
            return -1;
        if (segment[0] == '.' && (segment[1] == '/' || segment[1] == '\0')) return -1;
        segment = strchr(segment, '/');
        if (segment) segment++;
    }
    size_t length = strlen(relative);
    if (length < 5 || strcmp(relative + length - 4, ".jpg") != 0) return -1;
    int n = snprintf(out, size, "%s/%s", root, relative);
    return n > 0 && (size_t)n < size ? 0 : -1;
}

static int64_t now_ms(void) {
    struct timespec ts;
    clock_gettime(CLOCK_REALTIME, &ts);
    return (int64_t)ts.tv_sec * 1000 + ts.tv_nsec / 1000000;
}

/* Contract section 8.2 (provisional): 1 s while the asset is under 5 s old,
 * otherwise 2 s until the L3 queue exposes its depth (min(30, 2 x ahead)). */
static int retry_after_seconds(const observation_asset_record_t *asset) {
    int64_t age = now_ms() - asset->created_at_ms;
    int seconds = age < 5000 ? 1 : 2;
    if (seconds < 1) seconds = 1;
    if (seconds > OBS_SNAPSHOT_RETRY_MAX) seconds = OBS_SNAPSHOT_RETRY_MAX;
    return seconds;
}

static void send_evidence_unavailable(http_response_t *res, const char *reason) {
    cJSON *body = error_body("Evidence is unavailable", "evidence_unavailable");
    if (body) cJSON_AddStringToObject(body, "reason", reason && reason[0] ? reason : "write_failed");
    send_json(res, 404, body);
}

static void send_evidence_expired(http_response_t *res, const observation_asset_record_t *asset) {
    cJSON *body = error_body("Evidence media has expired", "evidence_expired");
    if (body) {
        cJSON_AddStringToObject(body, "reason",
                                asset->reason[0] ? asset->reason : "retention_expired");
        cJSON_AddItemToObject(body, "expired_at", time_or_null(asset->updated_at_ms));
    }
    send_json(res, 410, body);
}

static bool etag_matches(const http_request_t *req, const char *etag) {
    const char *header = http_request_get_header(req, "If-None-Match");
    if (!header) return false;
    while (*header == ' ') header++;
    if (strcmp(header, "*") == 0) return true;
    return strstr(header, etag) != NULL;
}

static void serve_ready_snapshot(const http_request_t *req, http_response_t *res,
                                 const observation_record_t *record) {
    char file[MAX_PATH_LENGTH];
    if (resolve_asset_file(record->asset.path, file, sizeof(file)) != 0) {
        log_error("Observation asset %s has a path outside observation storage",
                  record->asset.uuid);
        send_internal(res);
        return;
    }
    struct stat st;
    if (stat(file, &st) != 0 || !S_ISREG(st.st_mode)) {
        /* Retention may have expired it since the row was read. */
        observation_asset_record_t current;
        if (db_observation_asset_get(record->asset.uuid, &current) == DB_OBS_OK &&
            current.state == OBS_ASSET_EXPIRED) {
            send_evidence_expired(res, &current);
            return;
        }
        log_warn("Ready observation asset %s has no media file", record->asset.uuid);
        send_evidence_unavailable(res, "write_failed");
        return;
    }
    char etag[OBS_UUID_SIZE + 2];
    snprintf(etag, sizeof(etag), "\"%s\"", record->asset.uuid);
    if (etag_matches(req, etag)) {
        res->status_code = 304;
        http_response_add_header(res, "ETag", etag);
        http_response_add_header(res, "Cache-Control", OBS_SNAPSHOT_CACHE_CONTROL);
        return;
    }
    char headers[160];
    snprintf(headers, sizeof(headers), "Cache-Control: %s\r\nETag: %s\r\n",
             OBS_SNAPSHOT_CACHE_CONTROL, etag);
    if (http_serve_file(req, res, file, "image/jpeg", headers) != 0)
        send_internal(res);
}

void handle_get_observation_snapshot(const http_request_t *req, http_response_t *res) {
    user_t user;
    if (!authenticate(req, res, &user)) return;
    observation_record_t *record = calloc(1, sizeof(*record));
    if (!record) {
        send_internal(res);
        return;
    }
    int found = load_observation(req, res, "/snapshot", record);
    if (found <= 0) {
        if (found == 0) send_not_found(res);
        free(record);
        return;
    }

    authorization_evaluation_t evaluation;
    int rc = httpd_evaluate_camera_identity_action(&user, AUTHZ_SNAPSHOT_CREATE,
                                                   record->camera_uuid, NULL, &evaluation);
    if (rc < 0) {
        audit_snapshot(req, &user, record, NULL, "error");
        send_internal(res);
        free(record);
        return;
    }
    if (rc > 0) {  /* capture camera vanished between the read and the check */
        send_not_found(res);
        free(record);
        return;
    }
    if (evaluation.decision != AUTHZ_DECISION_ALLOW) {
        audit_snapshot(req, &user, record, &evaluation, "denied");
        /* D4: a caller who can read the metadata learns nothing from a 403. */
        authorization_evaluation_t view;
        int view_rc = httpd_evaluate_camera_identity_action(&user, AUTHZ_LIVE_VIEW,
                                                            record->camera_uuid, NULL, &view);
        if (view_rc < 0) send_internal(res);
        else if (view_rc == 0 && view.decision == AUTHZ_DECISION_ALLOW)
            send_error(res, 403, "forbidden", "Forbidden");
        else send_not_found(res);
        free(record);
        return;
    }
    audit_snapshot(req, &user, record, &evaluation, "allowed");

    /* Only this observation's own asset is ever served; there is no fallback
     * to a latest or go2rtc frame. */
    switch (record->asset.state) {
    case OBS_ASSET_READY:
        serve_ready_snapshot(req, res, record);
        break;
    case OBS_ASSET_PENDING: {
        int seconds = retry_after_seconds(&record->asset);
        char header[16];
        snprintf(header, sizeof(header), "%d", seconds);
        http_response_add_header(res, "Retry-After", header);
        cJSON *body = error_body("Evidence is not ready yet", "evidence_pending");
        if (body) cJSON_AddNumberToObject(body, "retry_after_seconds", seconds);
        send_json(res, 202, body);
        break;
    }
    case OBS_ASSET_EXPIRED:
        send_evidence_expired(res, &record->asset);
        break;
    case OBS_ASSET_UNAVAILABLE:
    default:
        send_evidence_unavailable(res, record->asset.reason);
        break;
    }
    free(record);
}

/* ---- PUT classification (L14) ----------------------------------------- */

void handle_put_observation_classification(const http_request_t *req, http_response_t *res) {
    user_t user;
    if (!authenticate(req, res, &user)) return;
    send_error(res, 501, "not_implemented", "Not implemented in this version");
}
