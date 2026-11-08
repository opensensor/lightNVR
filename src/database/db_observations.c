/*
 * SPDX-License-Identifier: GPL-3.0-or-later
 * Copyright (C) 2026 OpenSensor Engineering and LightNVR contributors
 *
 * This file is part of LightNVR <https://github.com/opensensor/lightNVR>.
 */

/*
 * INT_01 observation DB layer. See include/database/db_observations.h and
 * docs/integrations/OBSERVATIONS_CONTRACT.md. Writes follow the db_auth.c
 * pattern: global DB mutex, SQLite connection mutex, BEGIN IMMEDIATE, raw
 * statements only, COMMIT or ROLLBACK. Nothing here calls a db_* helper that
 * takes the global mutex while it is held.
 */

#define _POSIX_C_SOURCE 200809L

#include "database/db_observations.h"

#include <cjson/cJSON.h>
#include <math.h>
#include <pthread.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <strings.h>
#include <time.h>

#include "core/event_identity.h"
#include "core/logger.h"
#include "database/db_core.h"
#include "utils/base64.h"
#include "utils/strings.h"
#include "utils/uuid.h"

#define OBS_SOURCE_SIZE 160
#define OBS_PRUNE_MAX_BATCH 10000
#define OBS_MAX_SAFE_SEQ 9007199254740991LL  /* 2^53 - 1: exact in JSON numbers */

/* ---- enums and validation --------------------------------------------- */

static const char *const STATE_NAMES[] = {"pending", "ready", "unavailable", "expired"};
static const char *const CAUSE_NAMES[] = {"created", "evidence", "classification"};
static const char *const PUBLICATION_NAMES[] = {
    "pending", "relayed", "relayed_no_route", "expired", "failed"
};

const char *observation_asset_state_name(observation_asset_state_t state) {
    return (unsigned)state < 4 ? STATE_NAMES[state] : NULL;
}

int observation_asset_state_parse(const char *name, observation_asset_state_t *state) {
    if (!name || !state) return -1;
    for (int i = 0; i < 4; ++i) {
        if (strcmp(name, STATE_NAMES[i]) == 0) {
            *state = (observation_asset_state_t)i;
            return 0;
        }
    }
    return -1;
}

const char *observation_change_cause_name(observation_change_cause_t cause) {
    return (unsigned)cause < 3 ? CAUSE_NAMES[cause] : NULL;
}

static int cause_parse(const char *name, observation_change_cause_t *cause) {
    for (int i = 0; name && i < 3; ++i) {
        if (strcmp(name, CAUSE_NAMES[i]) == 0) {
            *cause = (observation_change_cause_t)i;
            return 0;
        }
    }
    return -1;
}

const char *observation_publication_state_name(observation_publication_state_t state) {
    return (unsigned)state < 5 ? PUBLICATION_NAMES[state] : NULL;
}

/* Contract section 4.3. */
static const struct {
    const char *reason;
    observation_asset_state_t state;
} REASONS[] = {
    {"queue_full", OBS_ASSET_UNAVAILABLE},
    {"budget_exhausted", OBS_ASSET_UNAVAILABLE},
    {"encode_failed", OBS_ASSET_UNAVAILABLE},
    {"write_failed", OBS_ASSET_UNAVAILABLE},
    {"interrupted", OBS_ASSET_UNAVAILABLE},
    {"no_matching_media", OBS_ASSET_UNAVAILABLE},
    {"disabled", OBS_ASSET_UNAVAILABLE},
    {"retention_expired", OBS_ASSET_EXPIRED},
    {"deleted", OBS_ASSET_EXPIRED},
};

static bool empty_text(const char *value) { return !value || !value[0]; }

bool observation_reason_is_valid(observation_asset_state_t state, const char *reason) {
    if (state == OBS_ASSET_PENDING || state == OBS_ASSET_READY) return empty_text(reason);
    if (state != OBS_ASSET_UNAVAILABLE && state != OBS_ASSET_EXPIRED) return false;
    if (empty_text(reason)) return false;
    for (size_t i = 0; i < sizeof(REASONS) / sizeof(REASONS[0]); ++i) {
        if (strcmp(reason, REASONS[i].reason) == 0) return REASONS[i].state == state;
    }
    return false;
}

static bool reason_in(const char *reason, const char *const *allowed, size_t count) {
    for (size_t i = 0; reason && i < count; ++i)
        if (strcmp(reason, allowed[i]) == 0) return true;
    return false;
}

/* Section 4.2 edges. Direct-creation reasons (queue_full, budget_exhausted,
 * no_matching_media) are never transition targets. */
static bool transition_allowed(observation_asset_state_t from, observation_asset_state_t to,
                               const char *reason) {
    static const char *const pending_unavailable[] = {
        "encode_failed", "write_failed", "interrupted", "disabled"
    };
    static const char *const ready_unavailable[] = {"write_failed"};
    static const char *const ready_expired[] = {"retention_expired", "deleted"};
    if (from == OBS_ASSET_PENDING && to == OBS_ASSET_READY) return empty_text(reason);
    if (from == OBS_ASSET_PENDING && to == OBS_ASSET_UNAVAILABLE)
        return reason_in(reason, pending_unavailable, 4);
    if (from == OBS_ASSET_READY && to == OBS_ASSET_UNAVAILABLE)
        return reason_in(reason, ready_unavailable, 1);
    if (from == OBS_ASSET_READY && to == OBS_ASSET_EXPIRED)
        return reason_in(reason, ready_expired, 2);
    return false;
}

static bool terminated(const char *value, size_t size) {
    return value && memchr(value, '\0', size) != NULL;
}

static bool text_len_between(const char *value, size_t size, size_t min, size_t max) {
    if (!terminated(value, size)) return false;
    size_t length = strlen(value);
    return length >= min && length <= max;
}

static int64_t current_time_ms(void) {
    struct timespec now;
    if (clock_gettime(CLOCK_REALTIME, &now) != 0) return (int64_t)time(NULL) * 1000;
    return (int64_t)now.tv_sec * 1000 + now.tv_nsec / 1000000;
}

static void copy_column(sqlite3_stmt *stmt, int column, char *out, size_t size) {
    const unsigned char *value = sqlite3_column_text(stmt, column);
    safe_strcpy(out, value ? (const char *)value : "", size, 0);
}

static void bind_text_or_null(sqlite3_stmt *stmt, int index, const char *value) {
    if (value && value[0]) sqlite3_bind_text(stmt, index, value, -1, SQLITE_TRANSIENT);
    else sqlite3_bind_null(stmt, index);
}

static void bind_positive_or_null(sqlite3_stmt *stmt, int index, int64_t value) {
    if (value > 0) sqlite3_bind_int64(stmt, index, value);
    else sqlite3_bind_null(stmt, index);
}

/* ---- identifiers and cursor ------------------------------------------- */

int observation_message_id(const char *source, const char *observation_uuid,
                           int revision, char out[OBS_UUID_SIZE]) {
    if (!out || empty_text(source) || !lightnvr_uuid_is_valid(observation_uuid) ||
        revision < 1)
        return -1;
    char name[OBS_SOURCE_SIZE + OBS_UUID_SIZE + 24];
    int written = snprintf(name, sizeof(name), "%s|%s|%d", source, observation_uuid, revision);
    if (written < 0 || (size_t)written >= sizeof(name)) return -1;
    return lightnvr_uuid_generate_v5(OBS_MESSAGE_ID_NAMESPACE, name, (size_t)written, out);
}

int observation_cursor_encode(int64_t seq, const char *epoch_uuid, char *out, size_t out_size) {
    if (!out || out_size == 0) return -1;
    out[0] = '\0';
    if (seq < 0 || seq > OBS_MAX_SAFE_SEQ || !lightnvr_uuid_is_valid(epoch_uuid)) return -1;
    char json[128];
    int written = snprintf(json, sizeof(json), "{\"v\":1,\"seq\":%lld,\"epoch\":\"%s\"}",
                           (long long)seq, epoch_uuid);
    if (written < 0 || (size_t)written >= sizeof(json)) return -1;
    return base64url_encode((const unsigned char *)json, (size_t)written, out, out_size, NULL);
}

static bool json_integer(const cJSON *item, double min, double max, int64_t *value) {
    if (!cJSON_IsNumber(item)) return false;
    double number = item->valuedouble;
    if (!isfinite(number) || number < min || number > max || floor(number) != number)
        return false;
    *value = (int64_t)number;
    return true;
}

int observation_cursor_decode(const char *cursor, int64_t *seq, char epoch_uuid[OBS_UUID_SIZE]) {
    if (!cursor || !seq || !epoch_uuid) return DB_OBS_INVALID_CURSOR;
    size_t length = strnlen(cursor, OBS_CURSOR_SIZE);
    if (length == 0 || length > OBS_CURSOR_SIZE - 1) return DB_OBS_INVALID_CURSOR;
    unsigned char json[OBS_CURSOR_SIZE];
    size_t json_length = 0;
    if (base64url_decode(cursor, length, json, sizeof(json) - 1, &json_length) != 0 ||
        memchr(json, '\0', json_length) != NULL)
        return DB_OBS_INVALID_CURSOR;
    json[json_length] = '\0';

    cJSON *root = cJSON_ParseWithLength((const char *)json, json_length);
    int result = DB_OBS_INVALID_CURSOR;
    int64_t version = 0, value = 0;
    const cJSON *epoch = root ? cJSON_GetObjectItemCaseSensitive(root, "epoch") : NULL;
    if (cJSON_IsObject(root) &&
        json_integer(cJSON_GetObjectItemCaseSensitive(root, "v"), 1, 1, &version) &&
        json_integer(cJSON_GetObjectItemCaseSensitive(root, "seq"), 0,
                     (double)OBS_MAX_SAFE_SEQ, &value) &&
        cJSON_IsString(epoch) && lightnvr_uuid_is_valid(epoch->valuestring)) {
        *seq = value;
        safe_strcpy(epoch_uuid, epoch->valuestring, OBS_UUID_SIZE, 0);
        result = DB_OBS_OK;
    }
    cJSON_Delete(root);
    return result;
}

/* ---- post-commit publication hook (L5) ------------------------------- */

static db_observation_publication_hook_t publication_hook = NULL;

void db_observations_set_publication_hook(db_observation_publication_hook_t hook) {
    __atomic_store_n(&publication_hook, hook, __ATOMIC_RELEASE);
}

static void notify_publications_committed(void) {
    db_observation_publication_hook_t hook =
        __atomic_load_n(&publication_hook, __ATOMIC_ACQUIRE);
    if (hook) hook();
}

/* ---- write transactions (db_auth.c pattern) ---------------------------- */

typedef struct {
    pthread_mutex_t *mutex;
    sqlite3 *db;
    sqlite3_mutex *connection_mutex;
} obs_tx_t;

static int tx_begin(obs_tx_t *tx) {
    tx->mutex = get_db_mutex();
    if (!tx->mutex) return -1;
    pthread_mutex_lock(tx->mutex);
    tx->db = get_db_handle();
    if (!tx->db) {
        pthread_mutex_unlock(tx->mutex);
        return -1;
    }
    /* Legacy helpers use the shared connection without get_db_mutex(); the
     * connection mutex keeps their statements out of this transaction. */
    tx->connection_mutex = sqlite3_db_mutex(tx->db);
    sqlite3_mutex_enter(tx->connection_mutex);
    if (sqlite3_exec(tx->db, "BEGIN IMMEDIATE;", NULL, NULL, NULL) != SQLITE_OK) {
        log_error("Failed to begin observation transaction: %s", sqlite3_errmsg(tx->db));
        sqlite3_mutex_leave(tx->connection_mutex);
        pthread_mutex_unlock(tx->mutex);
        return -1;
    }
    return 0;
}

/* Commits when rc is DB_OBS_OK; otherwise rolls back. Returns the final rc. */
static int tx_end(obs_tx_t *tx, int rc) {
    if (rc == DB_OBS_OK && sqlite3_exec(tx->db, "COMMIT;", NULL, NULL, NULL) != SQLITE_OK) {
        log_error("Failed to commit observation transaction: %s", sqlite3_errmsg(tx->db));
        rc = DB_OBS_ERROR;
    }
    if (rc != DB_OBS_OK) sqlite3_exec(tx->db, "ROLLBACK;", NULL, NULL, NULL);
    sqlite3_mutex_leave(tx->connection_mutex);
    pthread_mutex_unlock(tx->mutex);
    return rc;
}

static int get_source(char source[OBS_SOURCE_SIZE]) {
    if (event_identity_get_source(source, OBS_SOURCE_SIZE) != 0) {
        log_error("Observation write needs the event installation identity");
        return -1;
    }
    return 0;
}

/* Insert one change row (state snapshot at this revision) and its pending
 * publication with the deterministic message ID. */
static int write_change_locked(sqlite3 *db, const char *source, const char *observation_uuid,
                               int revision, observation_change_cause_t cause,
                               const char *evidence_state, const char *evidence_reason,
                               int64_t now_ms, int64_t *seq_out) {
    char message_id[OBS_UUID_SIZE];
    if (observation_message_id(source, observation_uuid, revision, message_id) != 0)
        return DB_OBS_ERROR;

    sqlite3_stmt *stmt = NULL;
    int rc = DB_OBS_ERROR;
    if (sqlite3_prepare_v2(db,
            "INSERT INTO observation_changes (observation_uuid, revision, cause, "
            "evidence_state, evidence_reason, created_at_ms) VALUES (?, ?, ?, ?, ?, ?);",
            -1, &stmt, NULL) == SQLITE_OK) {
        sqlite3_bind_text(stmt, 1, observation_uuid, -1, SQLITE_TRANSIENT);
        sqlite3_bind_int(stmt, 2, revision);
        sqlite3_bind_text(stmt, 3, observation_change_cause_name(cause), -1, SQLITE_STATIC);
        sqlite3_bind_text(stmt, 4, evidence_state, -1, SQLITE_TRANSIENT);
        bind_text_or_null(stmt, 5, evidence_reason);
        sqlite3_bind_int64(stmt, 6, now_ms);
        if (sqlite3_step(stmt) == SQLITE_DONE) rc = DB_OBS_OK;
    }
    sqlite3_finalize(stmt);
    if (rc != DB_OBS_OK) {
        log_error("Failed to insert observation change: %s", sqlite3_errmsg(db));
        return rc;
    }
    int64_t seq = sqlite3_last_insert_rowid(db);

    rc = DB_OBS_ERROR;
    stmt = NULL;
    if (sqlite3_prepare_v2(db,
            "INSERT INTO observation_publications (change_seq, message_id, state, attempts, "
            "updated_at_ms) VALUES (?, ?, 'pending', 0, ?);",
            -1, &stmt, NULL) == SQLITE_OK) {
        sqlite3_bind_int64(stmt, 1, seq);
        sqlite3_bind_text(stmt, 2, message_id, -1, SQLITE_TRANSIENT);
        sqlite3_bind_int64(stmt, 3, now_ms);
        if (sqlite3_step(stmt) == SQLITE_DONE) rc = DB_OBS_OK;
    }
    sqlite3_finalize(stmt);
    if (rc != DB_OBS_OK) {
        log_error("Failed to insert observation publication: %s", sqlite3_errmsg(db));
        return rc;
    }
    if (seq_out) *seq_out = seq;
    return DB_OBS_OK;
}

int db_observation_bump_revision_locked(sqlite3 *db, const char *source,
                                        const char *observation_uuid,
                                        observation_change_cause_t cause,
                                        int64_t now_ms,
                                        int64_t *seq_out, int *revision_out) {
    if (!db || empty_text(source) || !lightnvr_uuid_is_valid(observation_uuid) ||
        (cause != OBS_CAUSE_EVIDENCE && cause != OBS_CAUSE_CLASSIFICATION))
        return DB_OBS_INVALID;

    sqlite3_stmt *stmt = NULL;
    int revision = 0;
    char state[OBS_ENGINE_TYPE_SIZE] = "";
    char reason[OBS_REASON_SIZE] = "";
    int rc = DB_OBS_ERROR;
    if (sqlite3_prepare_v2(db,
            "SELECT o.revision, a.state, a.reason FROM object_observations o "
            "JOIN observation_assets a ON a.uuid = o.asset_uuid WHERE o.uuid = ?;",
            -1, &stmt, NULL) == SQLITE_OK) {
        sqlite3_bind_text(stmt, 1, observation_uuid, -1, SQLITE_TRANSIENT);
        int step = sqlite3_step(stmt);
        if (step == SQLITE_ROW) {
            revision = sqlite3_column_int(stmt, 0);
            copy_column(stmt, 1, state, sizeof(state));
            copy_column(stmt, 2, reason, sizeof(reason));
            rc = DB_OBS_OK;
        } else if (step == SQLITE_DONE) {
            rc = DB_OBS_NOT_FOUND;
        }
    }
    sqlite3_finalize(stmt);
    if (rc != DB_OBS_OK) return rc;

    rc = DB_OBS_ERROR;
    stmt = NULL;
    if (sqlite3_prepare_v2(db,
            "UPDATE object_observations SET revision = ? WHERE uuid = ? AND revision = ?;",
            -1, &stmt, NULL) == SQLITE_OK) {
        sqlite3_bind_int(stmt, 1, revision + 1);
        sqlite3_bind_text(stmt, 2, observation_uuid, -1, SQLITE_TRANSIENT);
        sqlite3_bind_int(stmt, 3, revision);
        if (sqlite3_step(stmt) == SQLITE_DONE && sqlite3_changes(db) == 1) rc = DB_OBS_OK;
    }
    sqlite3_finalize(stmt);
    if (rc != DB_OBS_OK) return rc;

    rc = write_change_locked(db, source, observation_uuid, revision + 1, cause, state,
                             reason, now_ms, seq_out);
    if (rc == DB_OBS_OK && revision_out) *revision_out = revision + 1;
    return rc;
}

/* ---- insert ------------------------------------------------------------ */

static double round6(double value) { return round(value * 1e6) / 1e6; }

static bool valid_box(const observation_object_input_t *object) {
    if (!object->has_box) return true;
    double x = object->box_x, y = object->box_y, w = object->box_width, h = object->box_height;
    return isfinite(x) && isfinite(y) && isfinite(w) && isfinite(h) &&
           x >= 0.0 && y >= 0.0 && w > 0.0 && h > 0.0 &&
           x + w <= 1.000001 && y + h <= 1.000001;
}

static bool valid_object(const observation_object_input_t *object) {
    if (object->object_index < 0 || object->object_index >= OBS_MAX_BATCH) return false;
    if (!text_len_between(object->label, sizeof(object->label), 1, OBS_LABEL_SIZE - 1))
        return false;
    if (!isfinite(object->confidence) || object->confidence < 0.0 || object->confidence > 1.0)
        return false;
    if (!valid_box(object)) return false;
    if (object->has_track &&
        (!text_len_between(object->track_id, sizeof(object->track_id), 1, 64) ||
         !text_len_between(object->track_namespace, sizeof(object->track_namespace), 1, 64) ||
         !lightnvr_uuid_is_valid(object->track_session)))
        return false;
    return true;
}

static bool valid_engine_type(const char *type) {
    static const char *const types[] = {"object", "api", "onvif", "external"};
    return reason_in(type, types, 4);
}

static bool valid_batch(const observation_batch_input_t *batch) {
    if (!batch || !terminated(batch->batch_id, sizeof(batch->batch_id)) ||
        !lightnvr_uuid_is_valid(batch->batch_id) ||
        !terminated(batch->camera_uuid, sizeof(batch->camera_uuid)) ||
        !lightnvr_uuid_is_valid(batch->camera_uuid) ||
        !text_len_between(batch->stream_name, sizeof(batch->stream_name), 0, 63) ||
        batch->observed_at_ms <= 0 ||
        !text_len_between(batch->engine_key, sizeof(batch->engine_key), 1, 64) ||
        !terminated(batch->engine_type, sizeof(batch->engine_type)) ||
        !valid_engine_type(batch->engine_type) ||
        !text_len_between(batch->model, sizeof(batch->model), 0, 128) ||
        strchr(batch->model, '/') || strchr(batch->model, '\\') ||
        !text_len_between(batch->model_version, sizeof(batch->model_version), 0, 64) ||
        batch->recording_id < 0 ||
        batch->object_count < 1 || batch->object_count > OBS_MAX_BATCH)
        return false;
    bool used[OBS_MAX_BATCH] = {false};
    for (int i = 0; i < batch->object_count; ++i) {
        const observation_object_input_t *object = &batch->objects[i];
        if (!valid_object(object) || used[object->object_index]) return false;
        used[object->object_index] = true;
    }
    return true;
}

static bool valid_asset_input(const observation_asset_input_t *asset) {
    static const char *const origins[] = {"inference_input", "submitted_image"};
    if (!asset || !terminated(asset->asset_uuid, sizeof(asset->asset_uuid)) ||
        (asset->asset_uuid[0] && !lightnvr_uuid_is_valid(asset->asset_uuid)) ||
        !terminated(asset->reason, sizeof(asset->reason)) ||
        !terminated(asset->origin, sizeof(asset->origin)) ||
        !terminated(asset->media_type, sizeof(asset->media_type)) ||
        !terminated(asset->transform_json, sizeof(asset->transform_json)) ||
        asset->captured_at_ms < 0 || asset->expires_at_ms < 0)
        return false;
    if (asset->state != OBS_ASSET_PENDING && asset->state != OBS_ASSET_UNAVAILABLE) return false;
    if (!observation_reason_is_valid(asset->state, asset->reason)) return false;
    if (asset->origin[0] && !reason_in(asset->origin, origins, 2)) return false;
    if (asset->media_type[0] && strcmp(asset->media_type, "image/jpeg") != 0) return false;
    return true;
}

/* Returns DB_OBS_DUPLICATE with result filled, DB_OBS_OK when the batch is
 * new, DB_OBS_CONFLICT when another camera owns the batch ID. */
static int find_existing_batch(sqlite3 *db, const observation_batch_input_t *batch,
                               observation_insert_result_t *result) {
    sqlite3_stmt *stmt = NULL;
    if (sqlite3_prepare_v2(db,
            "SELECT uuid, asset_uuid, camera_uuid FROM object_observations "
            "WHERE batch_id = ? ORDER BY object_index;", -1, &stmt, NULL) != SQLITE_OK)
        return DB_OBS_ERROR;
    sqlite3_bind_text(stmt, 1, batch->batch_id, -1, SQLITE_TRANSIENT);
    int rc = DB_OBS_OK;
    int count = 0;
    int step;
    while ((step = sqlite3_step(stmt)) == SQLITE_ROW) {
        char camera[OBS_UUID_SIZE];
        copy_column(stmt, 2, camera, sizeof(camera));
        if (strcmp(camera, batch->camera_uuid) != 0) {
            rc = DB_OBS_CONFLICT;
            break;
        }
        if (count < OBS_MAX_BATCH) {
            copy_column(stmt, 0, result->observation_uuids[count], OBS_UUID_SIZE);
            if (count == 0) copy_column(stmt, 1, result->asset_uuid, OBS_UUID_SIZE);
            count++;
        }
        rc = DB_OBS_DUPLICATE;
    }
    if (step != SQLITE_ROW && step != SQLITE_DONE) rc = DB_OBS_ERROR;
    sqlite3_finalize(stmt);
    if (rc == DB_OBS_DUPLICATE) result->object_count = count;
    return rc;
}

static int resolve_stream_name(sqlite3 *db, const char *camera_uuid, char *name, size_t size) {
    sqlite3_stmt *stmt = NULL;
    if (sqlite3_prepare_v2(db, "SELECT name FROM streams WHERE camera_uuid = ?;",
                           -1, &stmt, NULL) != SQLITE_OK)
        return DB_OBS_ERROR;
    sqlite3_bind_text(stmt, 1, camera_uuid, -1, SQLITE_TRANSIENT);
    int step = sqlite3_step(stmt);
    int rc = step == SQLITE_ROW ? DB_OBS_OK : step == SQLITE_DONE ? DB_OBS_NOT_FOUND : DB_OBS_ERROR;
    if (rc == DB_OBS_OK) copy_column(stmt, 0, name, size);
    sqlite3_finalize(stmt);
    return rc;
}

static int insert_asset_row(sqlite3 *db, const char *asset_uuid,
                            const observation_batch_input_t *batch,
                            const observation_asset_input_t *asset, int64_t now_ms) {
    sqlite3_stmt *stmt = NULL;
    int rc = DB_OBS_ERROR;
    if (sqlite3_prepare_v2(db,
            "INSERT INTO observation_assets (uuid, camera_uuid, state, reason, origin, "
            "media_type, captured_at_ms, transform_json, protected, expires_at_ms, "
            "created_at_ms, updated_at_ms) VALUES (?, ?, ?, ?, ?, ?, ?, ?, 0, ?, ?, ?);",
            -1, &stmt, NULL) == SQLITE_OK) {
        sqlite3_bind_text(stmt, 1, asset_uuid, -1, SQLITE_TRANSIENT);
        sqlite3_bind_text(stmt, 2, batch->camera_uuid, -1, SQLITE_TRANSIENT);
        sqlite3_bind_text(stmt, 3, observation_asset_state_name(asset->state), -1, SQLITE_STATIC);
        bind_text_or_null(stmt, 4, asset->reason);
        bind_text_or_null(stmt, 5, asset->origin);
        bind_text_or_null(stmt, 6, asset->media_type);
        sqlite3_bind_int64(stmt, 7, asset->captured_at_ms > 0 ? asset->captured_at_ms
                                                              : batch->observed_at_ms);
        bind_text_or_null(stmt, 8, asset->transform_json);
        bind_positive_or_null(stmt, 9, asset->expires_at_ms);
        sqlite3_bind_int64(stmt, 10, now_ms);
        sqlite3_bind_int64(stmt, 11, now_ms);
        if (sqlite3_step(stmt) == SQLITE_DONE) rc = DB_OBS_OK;
    }
    if (rc != DB_OBS_OK) log_error("Failed to insert observation asset: %s", sqlite3_errmsg(db));
    sqlite3_finalize(stmt);
    return rc;
}

static int insert_observation_row(sqlite3 *db, const char *uuid, const char *asset_uuid,
                                  const char *stream_name,
                                  const observation_batch_input_t *batch,
                                  const observation_object_input_t *object, int64_t now_ms) {
    sqlite3_stmt *stmt = NULL;
    int rc = DB_OBS_ERROR;
    if (sqlite3_prepare_v2(db,
            "INSERT OR IGNORE INTO object_observations (uuid, camera_uuid, "
            "stream_name_at_capture, batch_id, object_index, observed_at_ms, created_at_ms, "
            "engine_key, engine_type, model, model_version, label, confidence, box_x, box_y, "
            "box_width, box_height, track_id, track_namespace, track_session, recording_id, "
            "asset_uuid, revision) VALUES "
            "(?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, 1);",
            -1, &stmt, NULL) == SQLITE_OK) {
        sqlite3_bind_text(stmt, 1, uuid, -1, SQLITE_TRANSIENT);
        sqlite3_bind_text(stmt, 2, batch->camera_uuid, -1, SQLITE_TRANSIENT);
        sqlite3_bind_text(stmt, 3, stream_name, -1, SQLITE_TRANSIENT);
        sqlite3_bind_text(stmt, 4, batch->batch_id, -1, SQLITE_TRANSIENT);
        sqlite3_bind_int(stmt, 5, object->object_index);
        sqlite3_bind_int64(stmt, 6, batch->observed_at_ms);
        sqlite3_bind_int64(stmt, 7, now_ms);
        sqlite3_bind_text(stmt, 8, batch->engine_key, -1, SQLITE_TRANSIENT);
        sqlite3_bind_text(stmt, 9, batch->engine_type, -1, SQLITE_TRANSIENT);
        bind_text_or_null(stmt, 10, batch->model);
        bind_text_or_null(stmt, 11, batch->model_version);
        sqlite3_bind_text(stmt, 12, object->label, -1, SQLITE_TRANSIENT);
        sqlite3_bind_double(stmt, 13, object->confidence);
        if (object->has_box) {
            sqlite3_bind_double(stmt, 14, round6(object->box_x));
            sqlite3_bind_double(stmt, 15, round6(object->box_y));
            sqlite3_bind_double(stmt, 16, round6(object->box_width));
            sqlite3_bind_double(stmt, 17, round6(object->box_height));
        } else {
            for (int i = 14; i <= 17; ++i) sqlite3_bind_null(stmt, i);
        }
        if (object->has_track) {
            sqlite3_bind_text(stmt, 18, object->track_id, -1, SQLITE_TRANSIENT);
            sqlite3_bind_text(stmt, 19, object->track_namespace, -1, SQLITE_TRANSIENT);
            sqlite3_bind_text(stmt, 20, object->track_session, -1, SQLITE_TRANSIENT);
        } else {
            for (int i = 18; i <= 20; ++i) sqlite3_bind_null(stmt, i);
        }
        bind_positive_or_null(stmt, 21, batch->recording_id);
        sqlite3_bind_text(stmt, 22, asset_uuid, -1, SQLITE_TRANSIENT);
        /* Under BEGIN IMMEDIATE and the mutex the batch was checked absent,
         * so an ignored insert means a broken invariant, not a retry. */
        if (sqlite3_step(stmt) == SQLITE_DONE && sqlite3_changes(db) == 1) rc = DB_OBS_OK;
    }
    if (rc != DB_OBS_OK)
        log_error("Failed to insert observation: %s", sqlite3_errmsg(db));
    sqlite3_finalize(stmt);
    return rc;
}

int db_observation_insert_batch(const observation_batch_input_t *batch,
                                const observation_asset_input_t *asset,
                                observation_insert_result_t *result) {
    if (!result) return DB_OBS_INVALID;
    memset(result, 0, sizeof(*result));
    if (!valid_batch(batch) || !valid_asset_input(asset)) return DB_OBS_INVALID;

    char source[OBS_SOURCE_SIZE];
    if (get_source(source) != 0) return DB_OBS_ERROR;

    /* Generate IDs before taking the mutex. */
    char asset_uuid[OBS_UUID_SIZE];
    char uuids[OBS_MAX_BATCH][OBS_UUID_SIZE];
    if (asset->asset_uuid[0]) {
        safe_strcpy(asset_uuid, asset->asset_uuid, sizeof(asset_uuid), 0);
        for (char *c = asset_uuid; *c; ++c)
            if (*c >= 'A' && *c <= 'F') *c = (char)(*c - 'A' + 'a');
    } else if (lightnvr_uuid_generate_v4(asset_uuid) != 0) {
        return DB_OBS_ERROR;
    }
    for (int i = 0; i < batch->object_count; ++i)
        if (lightnvr_uuid_generate_v4(uuids[i]) != 0) return DB_OBS_ERROR;

    /* Process objects in object_index order so results and seq follow it. */
    int order[OBS_MAX_BATCH];
    int ordered = 0;
    for (int index = 0; index < OBS_MAX_BATCH; ++index)
        for (int i = 0; i < batch->object_count; ++i)
            if (batch->objects[i].object_index == index) order[ordered++] = i;

    int64_t now_ms = current_time_ms();
    obs_tx_t tx;
    if (tx_begin(&tx) != 0) return DB_OBS_ERROR;
    int rc = find_existing_batch(tx.db, batch, result);
    if (rc != DB_OBS_OK) {
        /* Duplicate or conflict: nothing written. */
        int final_rc = rc;
        tx_end(&tx, DB_OBS_ERROR);
        if (final_rc != DB_OBS_DUPLICATE) memset(result, 0, sizeof(*result));
        return final_rc;
    }

    char stream_name[OBS_STREAM_NAME_SIZE];
    rc = resolve_stream_name(tx.db, batch->camera_uuid, stream_name, sizeof(stream_name));
    if (rc == DB_OBS_OK && batch->stream_name[0])
        safe_strcpy(stream_name, batch->stream_name, sizeof(stream_name), 0);
    if (rc == DB_OBS_OK) rc = insert_asset_row(tx.db, asset_uuid, batch, asset, now_ms);

    const char *state = observation_asset_state_name(asset->state);
    for (int n = 0; rc == DB_OBS_OK && n < ordered; ++n) {
        int i = order[n];
        rc = insert_observation_row(tx.db, uuids[i], asset_uuid, stream_name, batch,
                                    &batch->objects[i], now_ms);
        if (rc == DB_OBS_OK)
            rc = write_change_locked(tx.db, source, uuids[i], 1, OBS_CAUSE_CREATED, state,
                                     asset->reason, now_ms, &result->last_seq);
        if (rc == DB_OBS_OK)
            safe_strcpy(result->observation_uuids[n], uuids[i], OBS_UUID_SIZE, 0);
    }
    rc = tx_end(&tx, rc);
    if (rc == DB_OBS_OK) {
        safe_strcpy(result->asset_uuid, asset_uuid, sizeof(result->asset_uuid), 0);
        result->object_count = ordered;
        if (ordered > 0) notify_publications_committed();
    } else {
        memset(result, 0, sizeof(*result));
    }
    return rc;
}

/* ---- transitions ------------------------------------------------------- */

int db_observation_asset_transition(const char *asset_uuid,
                                    observation_asset_state_t from,
                                    observation_asset_state_t to,
                                    const char *reason,
                                    const observation_asset_media_t *media,
                                    observation_transition_result_t *result) {
    if (result) memset(result, 0, sizeof(*result));
    if (!lightnvr_uuid_is_valid(asset_uuid) || !transition_allowed(from, to, reason))
        return DB_OBS_INVALID;
    if (to == OBS_ASSET_READY &&
        (!media || empty_text(media->path) || strlen(media->path) >= MAX_PATH_LENGTH ||
         media->bytes < 0 || media->width <= 0 || media->height <= 0))
        return DB_OBS_INVALID;
    if (media && (media->bytes < 0 || media->width < 0 || media->height < 0 ||
                  media->expires_at_ms < 0))
        return DB_OBS_INVALID;

    char source[OBS_SOURCE_SIZE];
    if (get_source(source) != 0) return DB_OBS_ERROR;
    int64_t now_ms = current_time_ms();

    obs_tx_t tx;
    if (tx_begin(&tx) != 0) return DB_OBS_ERROR;
    sqlite3_stmt *stmt = NULL;
    int rc = DB_OBS_ERROR;
    if (sqlite3_prepare_v2(tx.db,
            "UPDATE observation_assets SET state = ?, reason = ?, "
            "path = COALESCE(?, path), bytes = COALESCE(?, bytes), "
            "width = COALESCE(?, width), height = COALESCE(?, height), "
            "expires_at_ms = COALESCE(?, expires_at_ms), updated_at_ms = ? "
            "WHERE uuid = ? AND state = ?;", -1, &stmt, NULL) == SQLITE_OK) {
        sqlite3_bind_text(stmt, 1, observation_asset_state_name(to), -1, SQLITE_STATIC);
        bind_text_or_null(stmt, 2, reason);
        bind_text_or_null(stmt, 3, media ? media->path : NULL);
        if (media && to == OBS_ASSET_READY) sqlite3_bind_int64(stmt, 4, media->bytes);
        else sqlite3_bind_null(stmt, 4);
        bind_positive_or_null(stmt, 5, media ? media->width : 0);
        bind_positive_or_null(stmt, 6, media ? media->height : 0);
        bind_positive_or_null(stmt, 7, media ? media->expires_at_ms : 0);
        sqlite3_bind_int64(stmt, 8, now_ms);
        sqlite3_bind_text(stmt, 9, asset_uuid, -1, SQLITE_TRANSIENT);
        sqlite3_bind_text(stmt, 10, observation_asset_state_name(from), -1, SQLITE_STATIC);
        if (sqlite3_step(stmt) == SQLITE_DONE) rc = DB_OBS_OK;
    }
    sqlite3_finalize(stmt);

    if (rc == DB_OBS_OK && sqlite3_changes(tx.db) == 0) {
        stmt = NULL;
        rc = DB_OBS_ERROR;
        if (sqlite3_prepare_v2(tx.db, "SELECT 1 FROM observation_assets WHERE uuid = ?;",
                               -1, &stmt, NULL) == SQLITE_OK) {
            sqlite3_bind_text(stmt, 1, asset_uuid, -1, SQLITE_TRANSIENT);
            int step = sqlite3_step(stmt);
            if (step == SQLITE_ROW) rc = DB_OBS_CAS_LOST;
            else if (step == SQLITE_DONE) rc = DB_OBS_NOT_FOUND;
        }
        sqlite3_finalize(stmt);
        tx_end(&tx, DB_OBS_ERROR);
        return rc;
    }

    /* Collect referencing observations before mutating the same table. */
    char (*observations)[OBS_UUID_SIZE] = NULL;
    size_t count = 0, capacity = 0;
    stmt = NULL;
    if (rc == DB_OBS_OK) {
        rc = DB_OBS_ERROR;
        if (sqlite3_prepare_v2(tx.db,
                "SELECT uuid FROM object_observations WHERE asset_uuid = ? ORDER BY "
                "batch_id, object_index;", -1, &stmt, NULL) == SQLITE_OK) {
            sqlite3_bind_text(stmt, 1, asset_uuid, -1, SQLITE_TRANSIENT);
            int step;
            rc = DB_OBS_OK;
            while ((step = sqlite3_step(stmt)) == SQLITE_ROW) {
                if (count == capacity) {
                    size_t next = capacity ? capacity * 2 : OBS_MAX_BATCH;
                    void *grown = realloc(observations, next * sizeof(*observations));
                    if (!grown) { rc = DB_OBS_ERROR; break; }
                    observations = grown;
                    capacity = next;
                }
                copy_column(stmt, 0, observations[count++], OBS_UUID_SIZE);
            }
            if (rc == DB_OBS_OK && step != SQLITE_DONE) rc = DB_OBS_ERROR;
        }
        sqlite3_finalize(stmt);
    }

    int64_t last_seq = 0;
    for (size_t i = 0; rc == DB_OBS_OK && i < count; ++i)
        rc = db_observation_bump_revision_locked(tx.db, source, observations[i],
                                                 OBS_CAUSE_EVIDENCE, now_ms, &last_seq, NULL);
    free(observations);
    rc = tx_end(&tx, rc);
    if (rc == DB_OBS_OK && result) {
        result->observations_bumped = (int)count;
        result->last_seq = last_seq;
    }
    if (rc == DB_OBS_OK && count > 0) notify_publications_committed();
    return rc;
}

/* ---- reads ------------------------------------------------------------- */

static int ro_begin(sqlite3 **db) {
    if (db_open_readonly_connection(db) != 0) return DB_OBS_ERROR;
    /* One read transaction = one WAL snapshot for the whole call. */
    if (sqlite3_exec(*db, "BEGIN;", NULL, NULL, NULL) != SQLITE_OK) {
        db_close_readonly_connection(*db);
        *db = NULL;
        return DB_OBS_ERROR;
    }
    return DB_OBS_OK;
}

static void ro_end(sqlite3 *db) {
    if (!db) return;
    sqlite3_exec(db, "COMMIT;", NULL, NULL, NULL);
    db_close_readonly_connection(db);
}

#define ASSET_COLUMNS \
    "a.uuid, a.camera_uuid, a.state, a.reason, a.origin, a.media_type, a.path, a.bytes, " \
    "a.width, a.height, a.captured_at_ms, a.transform_json, a.protected, a.expires_at_ms, " \
    "a.created_at_ms, a.updated_at_ms"

#define RECORD_SELECT \
    "SELECT o.uuid, o.camera_uuid, o.stream_name_at_capture, o.batch_id, o.object_index, " \
    "o.observed_at_ms, o.created_at_ms, o.engine_key, o.engine_type, o.model, " \
    "o.model_version, o.label, o.confidence, o.box_x, o.box_y, o.box_width, o.box_height, " \
    "o.track_id, o.track_namespace, o.track_session, o.recording_id, o.revision, " \
    ASSET_COLUMNS " FROM object_observations o " \
    "JOIN observation_assets a ON a.uuid = o.asset_uuid " \
    "WHERE o.uuid = ? AND EXISTS (SELECT 1 FROM streams s WHERE s.camera_uuid = o.camera_uuid);"

static void read_asset(sqlite3_stmt *stmt, int c, observation_asset_record_t *asset) {
    memset(asset, 0, sizeof(*asset));
    copy_column(stmt, c + 0, asset->uuid, sizeof(asset->uuid));
    copy_column(stmt, c + 1, asset->camera_uuid, sizeof(asset->camera_uuid));
    char state[OBS_ENGINE_TYPE_SIZE];
    copy_column(stmt, c + 2, state, sizeof(state));
    if (observation_asset_state_parse(state, &asset->state) != 0)
        asset->state = OBS_ASSET_UNAVAILABLE;
    copy_column(stmt, c + 3, asset->reason, sizeof(asset->reason));
    copy_column(stmt, c + 4, asset->origin, sizeof(asset->origin));
    copy_column(stmt, c + 5, asset->media_type, sizeof(asset->media_type));
    copy_column(stmt, c + 6, asset->path, sizeof(asset->path));
    asset->has_bytes = sqlite3_column_type(stmt, c + 7) != SQLITE_NULL;
    asset->bytes = sqlite3_column_int64(stmt, c + 7);
    asset->width = sqlite3_column_int(stmt, c + 8);
    asset->height = sqlite3_column_int(stmt, c + 9);
    asset->captured_at_ms = sqlite3_column_int64(stmt, c + 10);
    copy_column(stmt, c + 11, asset->transform_json, sizeof(asset->transform_json));
    asset->is_protected = sqlite3_column_int(stmt, c + 12) != 0;
    asset->expires_at_ms = sqlite3_column_int64(stmt, c + 13);
    asset->created_at_ms = sqlite3_column_int64(stmt, c + 14);
    asset->updated_at_ms = sqlite3_column_int64(stmt, c + 15);
}

static void read_record(sqlite3_stmt *stmt, observation_record_t *out) {
    memset(out, 0, sizeof(*out));
    copy_column(stmt, 0, out->uuid, sizeof(out->uuid));
    copy_column(stmt, 1, out->camera_uuid, sizeof(out->camera_uuid));
    copy_column(stmt, 2, out->stream_name_at_capture, sizeof(out->stream_name_at_capture));
    copy_column(stmt, 3, out->batch_id, sizeof(out->batch_id));
    out->object_index = sqlite3_column_int(stmt, 4);
    out->observed_at_ms = sqlite3_column_int64(stmt, 5);
    out->created_at_ms = sqlite3_column_int64(stmt, 6);
    copy_column(stmt, 7, out->engine_key, sizeof(out->engine_key));
    copy_column(stmt, 8, out->engine_type, sizeof(out->engine_type));
    copy_column(stmt, 9, out->model, sizeof(out->model));
    copy_column(stmt, 10, out->model_version, sizeof(out->model_version));
    copy_column(stmt, 11, out->label, sizeof(out->label));
    out->confidence = sqlite3_column_double(stmt, 12);
    out->has_box = sqlite3_column_type(stmt, 13) != SQLITE_NULL;
    if (out->has_box) {
        out->box_x = sqlite3_column_double(stmt, 13);
        out->box_y = sqlite3_column_double(stmt, 14);
        out->box_width = sqlite3_column_double(stmt, 15);
        out->box_height = sqlite3_column_double(stmt, 16);
    }
    out->has_track = sqlite3_column_type(stmt, 17) != SQLITE_NULL;
    copy_column(stmt, 17, out->track_id, sizeof(out->track_id));
    copy_column(stmt, 18, out->track_namespace, sizeof(out->track_namespace));
    copy_column(stmt, 19, out->track_session, sizeof(out->track_session));
    out->recording_id = sqlite3_column_int64(stmt, 20);
    out->revision = sqlite3_column_int(stmt, 21);
    read_asset(stmt, 22, &out->asset);
}

static int get_record_on(sqlite3 *db, sqlite3_stmt **cached, const char *uuid,
                         observation_record_t *out) {
    if (!*cached && sqlite3_prepare_v2(db, RECORD_SELECT, -1, cached, NULL) != SQLITE_OK) {
        log_error("Failed to prepare observation read: %s", sqlite3_errmsg(db));
        return DB_OBS_ERROR;
    }
    sqlite3_reset(*cached);
    sqlite3_clear_bindings(*cached);
    sqlite3_bind_text(*cached, 1, uuid, -1, SQLITE_TRANSIENT);
    int step = sqlite3_step(*cached);
    if (step == SQLITE_ROW) {
        read_record(*cached, out);
        return DB_OBS_OK;
    }
    return step == SQLITE_DONE ? DB_OBS_NOT_FOUND : DB_OBS_ERROR;
}

int db_observation_get(const char *observation_uuid, observation_record_t *out) {
    if (!out || !lightnvr_uuid_is_valid(observation_uuid)) return DB_OBS_INVALID;
    sqlite3 *db = NULL;
    if (ro_begin(&db) != DB_OBS_OK) return DB_OBS_ERROR;
    sqlite3_stmt *stmt = NULL;
    int rc = get_record_on(db, &stmt, observation_uuid, out);
    sqlite3_finalize(stmt);
    ro_end(db);
    return rc;
}

int db_observation_asset_get(const char *asset_uuid, observation_asset_record_t *out) {
    if (!out || !lightnvr_uuid_is_valid(asset_uuid)) return DB_OBS_INVALID;
    sqlite3 *db = NULL;
    if (ro_begin(&db) != DB_OBS_OK) return DB_OBS_ERROR;
    sqlite3_stmt *stmt = NULL;
    int rc = DB_OBS_ERROR;
    if (sqlite3_prepare_v2(db, "SELECT " ASSET_COLUMNS " FROM observation_assets a "
                           "WHERE a.uuid = ?;", -1, &stmt, NULL) == SQLITE_OK) {
        sqlite3_bind_text(stmt, 1, asset_uuid, -1, SQLITE_TRANSIENT);
        int step = sqlite3_step(stmt);
        if (step == SQLITE_ROW) {
            read_asset(stmt, 0, out);
            rc = DB_OBS_OK;
        } else if (step == SQLITE_DONE) {
            rc = DB_OBS_NOT_FOUND;
        }
    }
    sqlite3_finalize(stmt);
    ro_end(db);
    return rc;
}

static int read_meta_on(sqlite3 *db, observation_meta_t *meta) {
    memset(meta, 0, sizeof(*meta));
    sqlite3_stmt *stmt = NULL;
    int rc = DB_OBS_ERROR;
    if (sqlite3_prepare_v2(db,
            "SELECT m.epoch_uuid, m.oldest_seq, (SELECT MAX(seq) FROM observation_changes) "
            "FROM observation_meta m WHERE m.id = 1;", -1, &stmt, NULL) == SQLITE_OK &&
        sqlite3_step(stmt) == SQLITE_ROW) {
        copy_column(stmt, 0, meta->epoch_uuid, sizeof(meta->epoch_uuid));
        meta->oldest_seq = sqlite3_column_int64(stmt, 1);
        int64_t max_seq = sqlite3_column_int64(stmt, 2);  /* NULL -> 0 */
        meta->head_seq = max_seq > meta->oldest_seq - 1 ? max_seq : meta->oldest_seq - 1;
        rc = lightnvr_uuid_is_valid(meta->epoch_uuid) ? DB_OBS_OK : DB_OBS_ERROR;
    }
    sqlite3_finalize(stmt);
    return rc;
}

int db_observation_meta_get(observation_meta_t *out) {
    if (!out) return DB_OBS_INVALID;
    sqlite3 *db = NULL;
    if (ro_begin(&db) != DB_OBS_OK) return DB_OBS_ERROR;
    int rc = read_meta_on(db, out);
    ro_end(db);
    return rc;
}

/* L10: one bounded statement per figure. The pending figures use the partial
 * pending index; failed/expired scan the publication table, which is bounded
 * by history retention. */
static int diag_scalar(sqlite3 *db, const char *sql, int64_t *value, bool *is_null) {
    sqlite3_stmt *stmt = NULL;
    int rc = DB_OBS_ERROR;
    if (sqlite3_prepare_v2(db, sql, -1, &stmt, NULL) == SQLITE_OK) {
        int step = sqlite3_step(stmt);
        if (step == SQLITE_ROW) {
            if (is_null) *is_null = sqlite3_column_type(stmt, 0) == SQLITE_NULL;
            *value = sqlite3_column_int64(stmt, 0);
            rc = DB_OBS_OK;
        } else if (step == SQLITE_DONE) {
            if (is_null) *is_null = true;
            *value = 0;
            rc = DB_OBS_OK;
        }
    }
    sqlite3_finalize(stmt);
    return rc;
}

int db_observation_diagnostics_get(observation_db_diagnostics_t *out) {
    if (!out) return DB_OBS_INVALID;
    memset(out, 0, sizeof(*out));
    sqlite3 *db = NULL;
    if (ro_begin(&db) != DB_OBS_OK) return DB_OBS_ERROR;
    bool oldest_null = true;
    int rc = read_meta_on(db, &out->meta);
    if (rc == DB_OBS_OK)
        rc = diag_scalar(db, "SELECT MIN(observed_at_ms) FROM object_observations;",
                         &out->oldest_observed_at_ms, &oldest_null);
    if (rc == DB_OBS_OK)
        rc = diag_scalar(db, "SELECT COUNT(*) FROM object_observations;",
                         &out->observations, NULL);
    if (rc == DB_OBS_OK)
        rc = diag_scalar(db, "SELECT COUNT(*) FROM observation_publications "
                             "WHERE state = 'pending';",
                         &out->publications_pending, NULL);
    if (rc == DB_OBS_OK)
        rc = diag_scalar(db, "SELECT COUNT(*) FROM observation_publications "
                             "WHERE state = 'failed';",
                         &out->publications_failed, NULL);
    if (rc == DB_OBS_OK)
        rc = diag_scalar(db, "SELECT COUNT(*) FROM observation_publications "
                             "WHERE state = 'expired';",
                         &out->publications_expired, NULL);
    if (rc == DB_OBS_OK)
        rc = diag_scalar(db, "SELECT c.created_at_ms FROM observation_publications p "
                             "JOIN observation_changes c ON c.seq = p.change_seq "
                             "WHERE p.state = 'pending' ORDER BY p.change_seq LIMIT 1;",
                         &out->oldest_pending_created_at_ms, NULL);
    ro_end(db);
    if (rc != DB_OBS_OK) {
        memset(out, 0, sizeof(*out));
        return DB_OBS_ERROR;
    }
    out->has_oldest_observed_at = !oldest_null;
    if (oldest_null) out->oldest_observed_at_ms = 0;
    return DB_OBS_OK;
}

int db_observation_meta_regenerate_epoch(void) {
    char epoch[OBS_UUID_SIZE];
    if (lightnvr_uuid_generate_v4(epoch) != 0) return DB_OBS_ERROR;
    obs_tx_t tx;
    if (tx_begin(&tx) != 0) return DB_OBS_ERROR;
    sqlite3_stmt *stmt = NULL;
    int rc = DB_OBS_ERROR;
    if (sqlite3_prepare_v2(tx.db, "UPDATE observation_meta SET epoch_uuid = ? WHERE id = 1;",
                           -1, &stmt, NULL) == SQLITE_OK) {
        sqlite3_bind_text(stmt, 1, epoch, -1, SQLITE_TRANSIENT);
        if (sqlite3_step(stmt) == SQLITE_DONE && sqlite3_changes(tx.db) == 1) rc = DB_OBS_OK;
    }
    sqlite3_finalize(stmt);
    rc = tx_end(&tx, rc);
    if (rc == DB_OBS_OK) log_info("Regenerated observation cursor epoch");
    return rc;
}

static bool valid_query(const observation_list_query_t *query) {
    if (!query) return false;
    int limit = query->limit == 0 ? OBS_LIST_DEFAULT_LIMIT : query->limit;
    if (limit < 1 || limit > OBS_LIST_MAX_LIMIT) return false;
    if (query->label_count > OBS_QUERY_MAX_LABELS) return false;
    for (size_t i = 0; i < query->label_count; ++i)
        if (!text_len_between(query->labels[i], OBS_LABEL_SIZE, 1, OBS_LABEL_SIZE - 1))
            return false;
    if (query->has_since && query->has_until && query->since_ms >= query->until_ms)
        return false;
    if (!query->all_cameras && query->camera_count > 0 && !query->camera_uuids) return false;
    return true;
}

static bool camera_allowed(const observation_list_query_t *query, const char *camera) {
    if (query->all_cameras) return true;
    for (size_t i = 0; i < query->camera_count; ++i)
        if (strcmp(query->camera_uuids[i], camera) == 0) return true;
    return false;
}

static bool change_matches(const observation_list_query_t *query, sqlite3_stmt *stmt) {
    if (sqlite3_column_int(stmt, 5) == 0) return false;  /* capture camera deleted */
    const char *camera = (const char *)sqlite3_column_text(stmt, 2);
    if (!camera || !camera_allowed(query, camera)) return false;
    int64_t observed = sqlite3_column_int64(stmt, 4);
    if (query->has_since && observed < query->since_ms) return false;
    if (query->has_until && observed >= query->until_ms) return false;
    if (query->label_count > 0) {
        const char *label = (const char *)sqlite3_column_text(stmt, 3);
        bool found = false;
        for (size_t i = 0; label && i < query->label_count && !found; ++i)
            found = strcasecmp(label, query->labels[i]) == 0;
        if (!found) return false;
    }
    return true;
}

static int fill_expired(sqlite3 *db, const observation_meta_t *meta, observation_page_t *page) {
    if (observation_cursor_encode(meta->oldest_seq - 1, meta->epoch_uuid, page->oldest_cursor,
                                  sizeof(page->oldest_cursor)) != 0)
        return DB_OBS_ERROR;
    sqlite3_stmt *stmt = NULL;
    int rc = DB_OBS_ERROR;
    if (sqlite3_prepare_v2(db, "SELECT MIN(observed_at_ms) FROM object_observations;",
                           -1, &stmt, NULL) == SQLITE_OK &&
        sqlite3_step(stmt) == SQLITE_ROW) {
        page->has_oldest_observed_at = sqlite3_column_type(stmt, 0) != SQLITE_NULL;
        page->oldest_observed_at_ms = sqlite3_column_int64(stmt, 0);
        rc = DB_OBS_CURSOR_EXPIRED;
    }
    sqlite3_finalize(stmt);
    return rc;
}

static int scan_page(sqlite3 *db, const observation_list_query_t *query, int64_t start,
                     int64_t watermark, int limit, char (*selected)[OBS_UUID_SIZE],
                     size_t *selected_count, int64_t *last_scanned) {
    sqlite3_stmt *stmt = NULL;
    if (sqlite3_prepare_v2(db,
            "SELECT c.seq, o.uuid, o.camera_uuid, o.label, o.observed_at_ms, "
            "EXISTS (SELECT 1 FROM streams s WHERE s.camera_uuid = o.camera_uuid) "
            "FROM observation_changes c JOIN object_observations o "
            "ON o.uuid = c.observation_uuid WHERE c.seq > ? AND c.seq <= ? "
            "ORDER BY c.seq LIMIT ?;", -1, &stmt, NULL) != SQLITE_OK) {
        log_error("Failed to prepare observation scan: %s", sqlite3_errmsg(db));
        return DB_OBS_ERROR;
    }
    sqlite3_bind_int64(stmt, 1, start);
    sqlite3_bind_int64(stmt, 2, watermark);
    sqlite3_bind_int(stmt, 3, OBS_LIST_SCAN_CAP);
    *last_scanned = start;
    *selected_count = 0;
    int step;
    while ((step = sqlite3_step(stmt)) == SQLITE_ROW) {
        if (change_matches(query, stmt)) {
            const char *uuid = (const char *)sqlite3_column_text(stmt, 1);
            bool present = false;
            for (size_t i = 0; uuid && i < *selected_count && !present; ++i)
                present = strcmp(selected[i], uuid) == 0;
            if (!present) {
                /* Stop before the change that would add observation limit + 1. */
                if (*selected_count == (size_t)limit) break;
                safe_strcpy(selected[(*selected_count)++], uuid ? uuid : "", OBS_UUID_SIZE, 0);
            }
        }
        *last_scanned = sqlite3_column_int64(stmt, 0);
    }
    int rc = (step == SQLITE_ROW || step == SQLITE_DONE) ? DB_OBS_OK : DB_OBS_ERROR;
    sqlite3_finalize(stmt);
    return rc;
}

static int changes_exist(sqlite3 *db, int64_t after, int64_t watermark, bool *exists) {
    sqlite3_stmt *stmt = NULL;
    int rc = DB_OBS_ERROR;
    if (sqlite3_prepare_v2(db,
            "SELECT EXISTS (SELECT 1 FROM observation_changes WHERE seq > ? AND seq <= ?);",
            -1, &stmt, NULL) == SQLITE_OK) {
        sqlite3_bind_int64(stmt, 1, after);
        sqlite3_bind_int64(stmt, 2, watermark);
        if (sqlite3_step(stmt) == SQLITE_ROW) {
            *exists = sqlite3_column_int(stmt, 0) != 0;
            rc = DB_OBS_OK;
        }
    }
    sqlite3_finalize(stmt);
    return rc;
}

int db_observations_list(const observation_list_query_t *query, const char *cursor,
                         observation_page_t *page) {
    if (!page) return DB_OBS_INVALID;
    memset(page, 0, sizeof(*page));
    if (!valid_query(query)) return DB_OBS_INVALID;
    int limit = query->limit == 0 ? OBS_LIST_DEFAULT_LIMIT : query->limit;

    bool has_cursor = cursor && cursor[0];
    bool head = has_cursor && strcmp(cursor, "head") == 0;
    int64_t start = 0;
    char epoch[OBS_UUID_SIZE] = "";
    if (has_cursor && !head && observation_cursor_decode(cursor, &start, epoch) != DB_OBS_OK)
        return DB_OBS_INVALID_CURSOR;

    sqlite3 *db = NULL;
    if (ro_begin(&db) != DB_OBS_OK) return DB_OBS_ERROR;
    observation_meta_t meta;
    int rc = read_meta_on(db, &meta);
    int64_t watermark = meta.head_seq;
    if (rc == DB_OBS_OK && has_cursor && !head &&
        (strcmp(epoch, meta.epoch_uuid) != 0 || start < meta.oldest_seq - 1 ||
         start > watermark)) {
        rc = fill_expired(db, &meta, page);
        ro_end(db);
        return rc;
    }
    if (rc == DB_OBS_OK && head) {
        rc = observation_cursor_encode(watermark, meta.epoch_uuid, page->next_cursor,
                                       sizeof(page->next_cursor)) == 0 ? DB_OBS_OK : DB_OBS_ERROR;
        ro_end(db);
        return rc;
    }
    if (!has_cursor) start = meta.oldest_seq - 1;

    char (*selected)[OBS_UUID_SIZE] = NULL;
    size_t selected_count = 0;
    int64_t last_scanned = start;
    if (rc == DB_OBS_OK) {
        selected = calloc((size_t)limit, sizeof(*selected));
        if (!selected) rc = DB_OBS_ERROR;
    }
    if (rc == DB_OBS_OK)
        rc = scan_page(db, query, start, watermark, limit, selected, &selected_count,
                       &last_scanned);
    if (rc == DB_OBS_OK)
        rc = changes_exist(db, last_scanned, watermark, &page->has_more);
    if (rc == DB_OBS_OK &&
        observation_cursor_encode(last_scanned, meta.epoch_uuid, page->next_cursor,
                                  sizeof(page->next_cursor)) != 0)
        rc = DB_OBS_ERROR;
    if (rc == DB_OBS_OK && selected_count > 0) {
        page->items = calloc(selected_count, sizeof(*page->items));
        if (!page->items) rc = DB_OBS_ERROR;
    }
    sqlite3_stmt *record_stmt = NULL;
    for (size_t i = 0; rc == DB_OBS_OK && i < selected_count; ++i) {
        int got = get_record_on(db, &record_stmt, selected[i], &page->items[page->count]);
        if (got == DB_OBS_OK) page->count++;
        else if (got != DB_OBS_NOT_FOUND) rc = got;
    }
    sqlite3_finalize(record_stmt);
    free(selected);
    ro_end(db);
    if (rc != DB_OBS_OK) db_observation_page_free(page);
    return rc;
}

void db_observation_page_free(observation_page_t *page) {
    if (!page) return;
    free(page->items);
    page->items = NULL;
    page->count = 0;
}

/* ---- publications ------------------------------------------------------ */

int db_observation_pending_publications(observation_publication_t *out, int max) {
    if (!out || max <= 0) return DB_OBS_INVALID;
    sqlite3 *db = NULL;
    if (ro_begin(&db) != DB_OBS_OK) return DB_OBS_ERROR;
    sqlite3_stmt *stmt = NULL;
    int count = 0;
    int rc = DB_OBS_ERROR;
    if (sqlite3_prepare_v2(db,
            "SELECT p.change_seq, p.message_id, p.attempts, c.observation_uuid, c.revision, "
            "c.cause, c.evidence_state, c.evidence_reason, c.created_at_ms, o.camera_uuid, "
            "o.label, o.observed_at_ms, o.asset_uuid FROM observation_publications p "
            "JOIN observation_changes c ON c.seq = p.change_seq "
            "JOIN object_observations o ON o.uuid = c.observation_uuid "
            "WHERE p.state = 'pending' ORDER BY p.change_seq LIMIT ?;",
            -1, &stmt, NULL) == SQLITE_OK) {
        sqlite3_bind_int(stmt, 1, max);
        int step;
        while ((step = sqlite3_step(stmt)) == SQLITE_ROW) {
            observation_publication_t *pub = &out[count];
            memset(pub, 0, sizeof(*pub));
            pub->change_seq = sqlite3_column_int64(stmt, 0);
            copy_column(stmt, 1, pub->message_id, sizeof(pub->message_id));
            pub->attempts = sqlite3_column_int(stmt, 2);
            copy_column(stmt, 3, pub->observation_uuid, sizeof(pub->observation_uuid));
            pub->revision = sqlite3_column_int(stmt, 4);
            char text[OBS_ENGINE_TYPE_SIZE];
            copy_column(stmt, 5, text, sizeof(text));
            if (cause_parse(text, &pub->cause) != 0) continue;
            copy_column(stmt, 6, text, sizeof(text));
            if (observation_asset_state_parse(text, &pub->evidence_state) != 0) continue;
            copy_column(stmt, 7, pub->evidence_reason, sizeof(pub->evidence_reason));
            pub->change_created_at_ms = sqlite3_column_int64(stmt, 8);
            copy_column(stmt, 9, pub->camera_uuid, sizeof(pub->camera_uuid));
            copy_column(stmt, 10, pub->label, sizeof(pub->label));
            pub->observed_at_ms = sqlite3_column_int64(stmt, 11);
            copy_column(stmt, 12, pub->asset_uuid, sizeof(pub->asset_uuid));
            count++;
        }
        if (step == SQLITE_DONE) rc = DB_OBS_OK;
    }
    sqlite3_finalize(stmt);
    ro_end(db);
    return rc == DB_OBS_OK ? count : rc;
}

int db_observation_publication_mark(int64_t change_seq,
                                    observation_publication_state_t state,
                                    const char *last_error) {
    if (change_seq <= 0 || !observation_publication_state_name(state)) return DB_OBS_INVALID;
    char error_text[OBS_ERROR_TEXT_SIZE] = "";
    if (last_error) safe_strcpy(error_text, last_error, sizeof(error_text), 0);
    int64_t now_ms = current_time_ms();

    obs_tx_t tx;
    if (tx_begin(&tx) != 0) return DB_OBS_ERROR;
    sqlite3_stmt *stmt = NULL;
    int rc = DB_OBS_ERROR;
    if (sqlite3_prepare_v2(tx.db,
            "UPDATE observation_publications SET state = ?, attempts = attempts + 1, "
            "last_error = ?, updated_at_ms = ? WHERE change_seq = ? AND state = 'pending';",
            -1, &stmt, NULL) == SQLITE_OK) {
        sqlite3_bind_text(stmt, 1, observation_publication_state_name(state), -1, SQLITE_STATIC);
        bind_text_or_null(stmt, 2, error_text);
        sqlite3_bind_int64(stmt, 3, now_ms);
        sqlite3_bind_int64(stmt, 4, change_seq);
        if (sqlite3_step(stmt) == SQLITE_DONE) rc = DB_OBS_OK;
    }
    sqlite3_finalize(stmt);
    if (rc == DB_OBS_OK && sqlite3_changes(tx.db) == 0) {
        rc = DB_OBS_ERROR;
        stmt = NULL;
        if (sqlite3_prepare_v2(tx.db,
                "SELECT 1 FROM observation_publications WHERE change_seq = ?;",
                -1, &stmt, NULL) == SQLITE_OK) {
            sqlite3_bind_int64(stmt, 1, change_seq);
            int step = sqlite3_step(stmt);
            if (step == SQLITE_ROW) rc = DB_OBS_CAS_LOST;
            else if (step == SQLITE_DONE) rc = DB_OBS_NOT_FOUND;
        }
        sqlite3_finalize(stmt);
        tx_end(&tx, DB_OBS_ERROR);
        return rc;
    }
    return tx_end(&tx, rc);
}

/* ---- prune ------------------------------------------------------------- */

static int collect_texts(sqlite3 *db, const char *sql, int64_t bound,
                         char (**out)[OBS_UUID_SIZE], size_t *count) {
    *out = NULL;
    *count = 0;
    size_t capacity = 0;
    sqlite3_stmt *stmt = NULL;
    if (sqlite3_prepare_v2(db, sql, -1, &stmt, NULL) != SQLITE_OK) return DB_OBS_ERROR;
    sqlite3_bind_int64(stmt, 1, bound);
    int rc = DB_OBS_OK;
    int step;
    while ((step = sqlite3_step(stmt)) == SQLITE_ROW) {
        if (sqlite3_column_type(stmt, 0) == SQLITE_NULL) continue;
        if (*count == capacity) {
            size_t next = capacity ? capacity * 2 : 64;
            void *grown = realloc(*out, next * sizeof(**out));
            if (!grown) { rc = DB_OBS_ERROR; break; }
            *out = grown;
            capacity = next;
        }
        copy_column(stmt, 0, (*out)[(*count)++], OBS_UUID_SIZE);
    }
    if (rc == DB_OBS_OK && step != SQLITE_DONE) rc = DB_OBS_ERROR;
    sqlite3_finalize(stmt);
    return rc;
}

static int delete_each(sqlite3 *db, const char *sql, char (*uuids)[OBS_UUID_SIZE],
                       size_t count, int *deleted) {
    sqlite3_stmt *stmt = NULL;
    if (sqlite3_prepare_v2(db, sql, -1, &stmt, NULL) != SQLITE_OK) return DB_OBS_ERROR;
    int rc = DB_OBS_OK;
    for (size_t i = 0; i < count && rc == DB_OBS_OK; ++i) {
        sqlite3_reset(stmt);
        sqlite3_bind_text(stmt, 1, uuids[i], -1, SQLITE_TRANSIENT);
        if (sqlite3_step(stmt) != SQLITE_DONE) rc = DB_OBS_ERROR;
        else *deleted += sqlite3_changes(db);
    }
    sqlite3_finalize(stmt);
    return rc;
}

int db_observations_prune(int64_t before_ms, int batch, observation_prune_result_t *result) {
    if (!result) return DB_OBS_INVALID;
    memset(result, 0, sizeof(*result));
    if (batch <= 0 || batch > OBS_PRUNE_MAX_BATCH) return DB_OBS_INVALID;

    obs_tx_t tx;
    if (tx_begin(&tx) != 0) return DB_OBS_ERROR;
    int rc = DB_OBS_ERROR;
    int64_t oldest_seq = 1;
    sqlite3_stmt *stmt = NULL;
    if (sqlite3_prepare_v2(tx.db, "SELECT oldest_seq FROM observation_meta WHERE id = 1;",
                           -1, &stmt, NULL) == SQLITE_OK &&
        sqlite3_step(stmt) == SQLITE_ROW) {
        oldest_seq = sqlite3_column_int64(stmt, 0);
        rc = DB_OBS_OK;
    }
    sqlite3_finalize(stmt);

    /* Ascending seq while created_at is older than the boundary. */
    int64_t last_pruned = 0;
    stmt = NULL;
    if (rc == DB_OBS_OK) {
        rc = DB_OBS_ERROR;
        if (sqlite3_prepare_v2(tx.db,
                "SELECT seq, created_at_ms FROM observation_changes ORDER BY seq LIMIT ?;",
                -1, &stmt, NULL) == SQLITE_OK) {
            sqlite3_bind_int(stmt, 1, batch);
            int step;
            while ((step = sqlite3_step(stmt)) == SQLITE_ROW &&
                   sqlite3_column_int64(stmt, 1) < before_ms)
                last_pruned = sqlite3_column_int64(stmt, 0);
            if (step == SQLITE_ROW || step == SQLITE_DONE) rc = DB_OBS_OK;
        }
        sqlite3_finalize(stmt);
    }

    char (*observations)[OBS_UUID_SIZE] = NULL;
    char (*assets)[OBS_UUID_SIZE] = NULL;
    size_t observation_count = 0, asset_count = 0;
    if (rc == DB_OBS_OK && last_pruned > 0) {
        rc = collect_texts(tx.db,
            "SELECT DISTINCT observation_uuid FROM observation_changes WHERE seq <= ?;",
            last_pruned, &observations, &observation_count);
        if (rc == DB_OBS_OK)
            rc = collect_texts(tx.db,
                "SELECT DISTINCT o.asset_uuid FROM object_observations o JOIN "
                "observation_changes c ON c.observation_uuid = o.uuid WHERE c.seq <= ?;",
                last_pruned, &assets, &asset_count);
        if (rc == DB_OBS_OK) {
            rc = DB_OBS_ERROR;
            stmt = NULL;
            if (sqlite3_prepare_v2(tx.db, "DELETE FROM observation_changes WHERE seq <= ?;",
                                   -1, &stmt, NULL) == SQLITE_OK) {
                sqlite3_bind_int64(stmt, 1, last_pruned);
                if (sqlite3_step(stmt) == SQLITE_DONE) {
                    result->changes_deleted = sqlite3_changes(tx.db);
                    rc = DB_OBS_OK;
                }
            }
            sqlite3_finalize(stmt);
        }
        /* Observations before assets: asset_uuid is a foreign key. */
        if (rc == DB_OBS_OK)
            rc = delete_each(tx.db,
                "DELETE FROM object_observations WHERE uuid = ?1 AND NOT EXISTS "
                "(SELECT 1 FROM observation_changes WHERE observation_uuid = ?1);",
                observations, observation_count, &result->observations_deleted);
        if (rc == DB_OBS_OK)
            rc = delete_each(tx.db,
                "DELETE FROM observation_assets WHERE uuid = ?1 AND state IN "
                "('unavailable', 'expired') AND NOT EXISTS "
                "(SELECT 1 FROM object_observations WHERE asset_uuid = ?1);",
                assets, asset_count, &result->assets_deleted);
        if (rc == DB_OBS_OK && last_pruned + 1 > oldest_seq) {
            rc = DB_OBS_ERROR;
            stmt = NULL;
            if (sqlite3_prepare_v2(tx.db,
                    "UPDATE observation_meta SET oldest_seq = ? WHERE id = 1;",
                    -1, &stmt, NULL) == SQLITE_OK) {
                sqlite3_bind_int64(stmt, 1, last_pruned + 1);
                if (sqlite3_step(stmt) == SQLITE_DONE) rc = DB_OBS_OK;
            }
            sqlite3_finalize(stmt);
            if (rc == DB_OBS_OK) oldest_seq = last_pruned + 1;
        }
    }
    free(observations);
    free(assets);
    rc = tx_end(&tx, rc);
    if (rc == DB_OBS_OK) result->oldest_seq = oldest_seq;
    else memset(result, 0, sizeof(*result));
    return rc;
}
