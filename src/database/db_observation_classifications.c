/*
 * SPDX-License-Identifier: GPL-3.0-or-later
 * Copyright (C) 2026 OpenSensor Engineering and LightNVR contributors
 *
 * This file is part of LightNVR <https://github.com/opensensor/lightNVR>.
 */

/*
 * INT_01 P2 classification annotations. See
 * include/database/db_observation_classifications.h and contract section 10.
 * Writes follow the db_auth.c pattern used by db_observations.c: global DB
 * mutex, SQLite connection mutex, BEGIN IMMEDIATE, raw statements plus
 * db_observation_bump_revision_locked(), COMMIT or ROLLBACK. Nothing here
 * calls a db_* helper that takes the global mutex while it is held.
 */

#define _POSIX_C_SOURCE 200809L

#include "database/db_observation_classifications.h"

#include <math.h>
#include <pthread.h>
#include <stdio.h>
#include <string.h>
#include <strings.h>
#include <time.h>

#include "core/event_identity.h"
#include "core/logger.h"
#include "core/observation_relay.h"
#include "database/db_core.h"
#include "utils/strings.h"
#include "utils/uuid.h"

#define CLS_SOURCE_SIZE 160

static const char *const KIND_NAMES[] = {"automated", "owner_confirmed"};

const char *observation_classification_kind_name(observation_classification_kind_t kind) {
    return (kind == OBS_CLS_KIND_AUTOMATED || kind == OBS_CLS_KIND_OWNER_CONFIRMED)
        ? KIND_NAMES[kind] : "automated";
}

int observation_classification_kind_parse(const char *name,
                                          observation_classification_kind_t *kind) {
    if (!name || !kind) return -1;
    for (int i = 0; i < 2; ++i) {
        if (strcmp(name, KIND_NAMES[i]) == 0) {
            *kind = (observation_classification_kind_t)i;
            return 0;
        }
    }
    return -1;
}

bool observation_classification_segment_is_valid(const char *segment) {
    if (!segment) return false;
    size_t length = strnlen(segment, OBS_CLS_SEGMENT_MAX + 1);
    if (length == 0 || length > OBS_CLS_SEGMENT_MAX) return false;
    for (size_t i = 0; i < length; ++i) {
        unsigned char c = (unsigned char)segment[i];
        bool alnum = (c >= 'a' && c <= 'z') || (c >= '0' && c <= '9');
        if (i == 0 ? !alnum : !(alnum || c == '_' || c == '.' || c == '-')) return false;
    }
    return true;
}

/* ---- validation -------------------------------------------------------- */

static bool empty_text(const char *value) { return !value || !value[0]; }

/* Optional text: NULL/"" or 1..max bytes. */
static bool optional_len_ok(const char *value, size_t max) {
    return empty_text(value) || strnlen(value, max + 1) <= max;
}

static bool required_len_ok(const char *value, size_t max) {
    if (empty_text(value)) return false;
    return strnlen(value, max + 1) <= max;
}

static bool valid_input(const observation_classification_input_t *in) {
    return in &&
           observation_classification_segment_is_valid(in->writer_namespace) &&
           observation_classification_segment_is_valid(in->name) &&
           in->writer_user_id > 0 &&
           required_len_ok(in->classifier, OBS_CLS_CLASSIFIER_SIZE - 1) &&
           optional_len_ok(in->model_version, OBS_CLS_MODEL_VERSION_SIZE - 1) &&
           required_len_ok(in->label, OBS_CLS_LABEL_SIZE - 1) &&
           optional_len_ok(in->taxonomy_id, OBS_CLS_TAXONOMY_SIZE - 1) &&
           isfinite(in->confidence) && in->confidence >= 0.0 && in->confidence <= 1.0 &&
           in->classified_at_ms >= 0 &&
           lightnvr_uuid_is_valid(in->input_asset_uuid) &&
           in->input_revision >= 1 &&
           (in->kind == OBS_CLS_KIND_AUTOMATED || in->kind == OBS_CLS_KIND_OWNER_CONFIRMED) &&
           in->expected_revision >= 0;
}

/* ---- small helpers ----------------------------------------------------- */

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
    if (!empty_text(value)) sqlite3_bind_text(stmt, index, value, -1, SQLITE_TRANSIENT);
    else sqlite3_bind_null(stmt, index);
}

static bool same_optional(const char *stored, const char *requested) {
    return strcmp(stored, requested ? requested : "") == 0;
}

#define CLS_COLUMNS \
    "key, writer_namespace, writer_user_id, classifier, model_version, label, taxonomy_id, " \
    "confidence, classified_at_ms, input_asset_uuid, input_revision, kind, " \
    "annotation_revision, created_at_ms, updated_at_ms"

static void read_annotation(sqlite3_stmt *stmt, observation_classification_t *out) {
    memset(out, 0, sizeof(*out));
    copy_column(stmt, 0, out->key, sizeof(out->key));
    copy_column(stmt, 1, out->writer_namespace, sizeof(out->writer_namespace));
    out->writer_user_id = sqlite3_column_int64(stmt, 2);
    copy_column(stmt, 3, out->classifier, sizeof(out->classifier));
    copy_column(stmt, 4, out->model_version, sizeof(out->model_version));
    copy_column(stmt, 5, out->label, sizeof(out->label));
    copy_column(stmt, 6, out->taxonomy_id, sizeof(out->taxonomy_id));
    out->confidence = sqlite3_column_double(stmt, 7);
    out->classified_at_ms = sqlite3_column_int64(stmt, 8);
    copy_column(stmt, 9, out->input_asset_uuid, sizeof(out->input_asset_uuid));
    out->input_revision = sqlite3_column_int(stmt, 10);
    char kind[24];
    copy_column(stmt, 11, kind, sizeof(kind));
    if (observation_classification_kind_parse(kind, &out->kind) != 0)
        out->kind = OBS_CLS_KIND_AUTOMATED;
    out->annotation_revision = sqlite3_column_int(stmt, 12);
    out->created_at_ms = sqlite3_column_int64(stmt, 13);
    out->updated_at_ms = sqlite3_column_int64(stmt, 14);
}

/* 1 when found, 0 when absent, -1 on error. */
static int load_annotation(sqlite3 *db, const char *observation_uuid, const char *key,
                           observation_classification_t *out) {
    sqlite3_stmt *stmt = NULL;
    int found = -1;
    if (sqlite3_prepare_v2(db, "SELECT " CLS_COLUMNS " FROM observation_classifications "
                           "WHERE observation_uuid = ? AND key = ?;", -1, &stmt, NULL) == SQLITE_OK) {
        sqlite3_bind_text(stmt, 1, observation_uuid, -1, SQLITE_TRANSIENT);
        sqlite3_bind_text(stmt, 2, key, -1, SQLITE_TRANSIENT);
        int step = sqlite3_step(stmt);
        if (step == SQLITE_ROW) {
            read_annotation(stmt, out);
            found = 1;
        } else if (step == SQLITE_DONE) {
            found = 0;
        }
    }
    if (found < 0) log_error("Failed to read classification: %s", sqlite3_errmsg(db));
    sqlite3_finalize(stmt);
    return found;
}

static bool same_payload(const observation_classification_t *stored,
                         const observation_classification_input_t *in) {
    return strcmp(stored->classifier, in->classifier) == 0 &&
           same_optional(stored->model_version, in->model_version) &&
           strcmp(stored->label, in->label) == 0 &&
           same_optional(stored->taxonomy_id, in->taxonomy_id) &&
           fabs(stored->confidence - in->confidence) <= OBS_CLS_CONFIDENCE_EPSILON &&
           stored->classified_at_ms == in->classified_at_ms &&
           strcasecmp(stored->input_asset_uuid, in->input_asset_uuid) == 0 &&
           stored->input_revision == in->input_revision &&
           stored->kind == in->kind;
}

typedef struct {
    int revision;
    char asset_uuid[OBS_UUID_SIZE];
    observation_asset_state_t state;
} observation_state_t;

static int load_observation(sqlite3 *db, const char *observation_uuid, observation_state_t *out) {
    sqlite3_stmt *stmt = NULL;
    int rc = DB_OBS_CLS_ERROR;
    if (sqlite3_prepare_v2(db,
            "SELECT o.revision, a.uuid, a.state FROM object_observations o "
            "JOIN observation_assets a ON a.uuid = o.asset_uuid WHERE o.uuid = ?;",
            -1, &stmt, NULL) == SQLITE_OK) {
        sqlite3_bind_text(stmt, 1, observation_uuid, -1, SQLITE_TRANSIENT);
        int step = sqlite3_step(stmt);
        if (step == SQLITE_ROW) {
            out->revision = sqlite3_column_int(stmt, 0);
            copy_column(stmt, 1, out->asset_uuid, sizeof(out->asset_uuid));
            char state[16];
            copy_column(stmt, 2, state, sizeof(state));
            if (observation_asset_state_parse(state, &out->state) != 0)
                out->state = OBS_ASSET_UNAVAILABLE;
            rc = DB_OBS_CLS_CREATED;  /* found */
        } else if (step == SQLITE_DONE) {
            rc = DB_OBS_CLS_NOT_FOUND;
        }
    }
    if (rc == DB_OBS_CLS_ERROR) log_error("Failed to read observation: %s", sqlite3_errmsg(db));
    sqlite3_finalize(stmt);
    return rc;
}

static int count_keys(sqlite3 *db, const char *observation_uuid) {
    sqlite3_stmt *stmt = NULL;
    int count = -1;
    if (sqlite3_prepare_v2(db, "SELECT count(*) FROM observation_classifications "
                           "WHERE observation_uuid = ?;", -1, &stmt, NULL) == SQLITE_OK) {
        sqlite3_bind_text(stmt, 1, observation_uuid, -1, SQLITE_TRANSIENT);
        if (sqlite3_step(stmt) == SQLITE_ROW) count = sqlite3_column_int(stmt, 0);
    }
    sqlite3_finalize(stmt);
    return count;
}

/* Insert (existing == NULL) or replace the stored values of one key. */
static int store_annotation(sqlite3 *db, const char *observation_uuid, const char *key,
                            const observation_classification_input_t *in,
                            const char *asset_uuid,
                            const observation_classification_t *existing, int64_t now_ms) {
    static const char *const INSERT_SQL =
        "INSERT INTO observation_classifications (classifier, model_version, label, "
        "taxonomy_id, confidence, classified_at_ms, input_asset_uuid, input_revision, kind, "
        "updated_at_ms, observation_uuid, key, writer_namespace, writer_user_id, "
        "annotation_revision, created_at_ms) "
        "VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, 1, ?10);";
    static const char *const UPDATE_SQL =
        "UPDATE observation_classifications SET classifier = ?1, model_version = ?2, "
        "label = ?3, taxonomy_id = ?4, confidence = ?5, classified_at_ms = ?6, "
        "input_asset_uuid = ?7, input_revision = ?8, kind = ?9, updated_at_ms = ?10, "
        "annotation_revision = annotation_revision + 1 "
        "WHERE observation_uuid = ?11 AND key = ?12 AND writer_user_id = ?14 "
        "AND annotation_revision = ?15;";
    sqlite3_stmt *stmt = NULL;
    int rc = DB_OBS_CLS_ERROR;
    if (sqlite3_prepare_v2(db, existing ? UPDATE_SQL : INSERT_SQL, -1, &stmt, NULL) == SQLITE_OK) {
        sqlite3_bind_text(stmt, 1, in->classifier, -1, SQLITE_TRANSIENT);
        bind_text_or_null(stmt, 2, in->model_version);
        sqlite3_bind_text(stmt, 3, in->label, -1, SQLITE_TRANSIENT);
        bind_text_or_null(stmt, 4, in->taxonomy_id);
        sqlite3_bind_double(stmt, 5, in->confidence);
        sqlite3_bind_int64(stmt, 6, in->classified_at_ms);
        sqlite3_bind_text(stmt, 7, asset_uuid, -1, SQLITE_TRANSIENT);
        sqlite3_bind_int(stmt, 8, in->input_revision);
        sqlite3_bind_text(stmt, 9, observation_classification_kind_name(in->kind), -1,
                          SQLITE_STATIC);
        sqlite3_bind_int64(stmt, 10, now_ms);
        sqlite3_bind_text(stmt, 11, observation_uuid, -1, SQLITE_TRANSIENT);
        sqlite3_bind_text(stmt, 12, key, -1, SQLITE_TRANSIENT);
        if (existing) {
            sqlite3_bind_int64(stmt, 14, existing->writer_user_id);
            sqlite3_bind_int(stmt, 15, existing->annotation_revision);
        } else {
            sqlite3_bind_text(stmt, 13, in->writer_namespace, -1, SQLITE_TRANSIENT);
            sqlite3_bind_int64(stmt, 14, in->writer_user_id);
        }
        if (sqlite3_step(stmt) == SQLITE_DONE && sqlite3_changes(db) == 1)
            rc = existing ? DB_OBS_CLS_UPDATED : DB_OBS_CLS_CREATED;
    }
    if (rc == DB_OBS_CLS_ERROR)
        log_error("Failed to store classification: %s", sqlite3_errmsg(db));
    sqlite3_finalize(stmt);
    return rc;
}

/* ---- write transaction (db_auth.c pattern) ------------------------------ */

typedef struct {
    pthread_mutex_t *mutex;
    sqlite3 *db;
    sqlite3_mutex *connection_mutex;
} cls_tx_t;

static int tx_begin(cls_tx_t *tx) {
    tx->mutex = get_db_mutex();
    if (!tx->mutex) return -1;
    pthread_mutex_lock(tx->mutex);
    tx->db = get_db_handle();
    if (!tx->db) {
        pthread_mutex_unlock(tx->mutex);
        return -1;
    }
    tx->connection_mutex = sqlite3_db_mutex(tx->db);
    sqlite3_mutex_enter(tx->connection_mutex);
    if (sqlite3_exec(tx->db, "BEGIN IMMEDIATE;", NULL, NULL, NULL) != SQLITE_OK) {
        log_error("Failed to begin classification transaction: %s", sqlite3_errmsg(tx->db));
        sqlite3_mutex_leave(tx->connection_mutex);
        pthread_mutex_unlock(tx->mutex);
        return -1;
    }
    return 0;
}

/* Commits when commit is true; otherwise rolls back. Returns false when the
 * commit failed (and was rolled back). */
static bool tx_end(cls_tx_t *tx, bool commit) {
    bool ok = true;
    if (commit && sqlite3_exec(tx->db, "COMMIT;", NULL, NULL, NULL) != SQLITE_OK) {
        log_error("Failed to commit classification transaction: %s", sqlite3_errmsg(tx->db));
        ok = false;
    }
    if (!commit || !ok) sqlite3_exec(tx->db, "ROLLBACK;", NULL, NULL, NULL);
    sqlite3_mutex_leave(tx->connection_mutex);
    pthread_mutex_unlock(tx->mutex);
    return ok;
}

/* Rules 2 and 4-8 of contract section 10.4, in order, on the open
 * transaction. Returns the outcome; only CREATED/UPDATED leave writes. */
static int put_locked(sqlite3 *db, const char *source, const char *observation_uuid,
                      const char *key, const observation_classification_input_t *in,
                      observation_classification_result_t *result) {
    observation_state_t obs;
    int rc = load_observation(db, observation_uuid, &obs);
    if (rc != DB_OBS_CLS_CREATED) return rc;
    result->observation_revision = obs.revision;
    safe_strcpy(result->observation_asset_uuid, obs.asset_uuid,
                sizeof(result->observation_asset_uuid), 0);
    result->evidence_state = obs.state;

    observation_classification_t existing;
    int found = load_annotation(db, observation_uuid, key, &existing);
    if (found < 0) return DB_OBS_CLS_ERROR;

    /* 2: authority is pinned to the user that created the key (D3). */
    if (found && (existing.writer_user_id != in->writer_user_id ||
                  strcmp(existing.writer_namespace, in->writer_namespace) != 0))
        return DB_OBS_CLS_WRITER_MISMATCH;

    /* 4: the annotation must describe this observation's evidence. */
    if (strcasecmp(in->input_asset_uuid, obs.asset_uuid) != 0 ||
        in->input_revision > obs.revision ||
        (obs.state != OBS_ASSET_READY && obs.state != OBS_ASSET_EXPIRED))
        return DB_OBS_CLS_EVIDENCE_MISMATCH;

    if (found) {
        result->has_annotation = true;
        result->annotation = existing;
        /* 5: identical retry, regardless of expected_revision. */
        if (same_payload(&existing, in)) return DB_OBS_CLS_NOOP;
        /* 6: owner decisions are not replaced by automated results. */
        if (existing.kind == OBS_CLS_KIND_OWNER_CONFIRMED && in->kind == OBS_CLS_KIND_AUTOMATED)
            return DB_OBS_CLS_OWNER_LOCKED;
        /* 7: compare-and-set on the annotation revision. */
        if (in->expected_revision != existing.annotation_revision)
            return DB_OBS_CLS_REVISION_CONFLICT;
    } else {
        if (in->expected_revision != 0) return DB_OBS_CLS_REVISION_CONFLICT;
        /* Section 10.5 limit, checked at step 8 before the insert. */
        int keys = count_keys(db, observation_uuid);
        if (keys < 0) return DB_OBS_CLS_ERROR;
        if (keys >= OBS_CLS_MAX_PER_OBSERVATION) return DB_OBS_CLS_LIMIT;
    }

    /* 8: store, then bump the observation revision (change + publication). */
    int64_t now_ms = current_time_ms();
    int stored = store_annotation(db, observation_uuid, key, in, obs.asset_uuid,
                                  found ? &existing : NULL, now_ms);
    if (stored != DB_OBS_CLS_CREATED && stored != DB_OBS_CLS_UPDATED) return DB_OBS_CLS_ERROR;

    int64_t seq = 0;
    int revision = 0;
    if (db_observation_bump_revision_locked(db, source, observation_uuid,
                                            OBS_CAUSE_CLASSIFICATION, now_ms,
                                            &seq, &revision) != DB_OBS_OK)
        return DB_OBS_CLS_ERROR;

    observation_classification_t written;
    if (load_annotation(db, observation_uuid, key, &written) != 1) return DB_OBS_CLS_ERROR;
    result->has_annotation = true;
    result->annotation = written;
    result->observation_revision = revision;
    result->change_seq = seq;
    return stored;
}

int db_observation_classification_put(const char *observation_uuid,
                                      const observation_classification_input_t *input,
                                      observation_classification_result_t *result) {
    if (!result) return DB_OBS_CLS_INVALID;
    memset(result, 0, sizeof(*result));
    result->limit = OBS_CLS_MAX_PER_OBSERVATION;
    if (!lightnvr_uuid_is_valid(observation_uuid) || !valid_input(input))
        return DB_OBS_CLS_INVALID;

    char key[OBS_CLS_KEY_SIZE];
    int written = snprintf(key, sizeof(key), "%s/%s", input->writer_namespace, input->name);
    if (written < 0 || (size_t)written >= sizeof(key)) return DB_OBS_CLS_INVALID;

    char source[CLS_SOURCE_SIZE];
    if (event_identity_get_source(source, sizeof(source)) != 0) {
        log_error("Classification write needs the event installation identity");
        return DB_OBS_CLS_ERROR;
    }

    cls_tx_t tx;
    if (tx_begin(&tx) != 0) return DB_OBS_CLS_ERROR;
    int rc = put_locked(tx.db, source, observation_uuid, key, input, result);
    bool changed = rc == DB_OBS_CLS_CREATED || rc == DB_OBS_CLS_UPDATED;
    if (!tx_end(&tx, changed)) rc = DB_OBS_CLS_ERROR;

    if (rc == DB_OBS_CLS_CREATED || rc == DB_OBS_CLS_UPDATED) {
        observation_relay_notify();
    } else if (rc < 0 && rc != DB_OBS_CLS_NOT_FOUND) {
        /* Nothing was written; never report values from a rolled-back write. */
        memset(result, 0, sizeof(*result));
        result->limit = OBS_CLS_MAX_PER_OBSERVATION;
    } else if (rc == DB_OBS_CLS_WRITER_MISMATCH) {
        result->has_annotation = false;
        memset(&result->annotation, 0, sizeof(result->annotation));
    }
    return rc;
}

/* ---- reads ------------------------------------------------------------- */

int db_observation_classifications_list(const char *observation_uuid,
                                        observation_classification_set_t *out) {
    if (!out || !lightnvr_uuid_is_valid(observation_uuid)) return DB_OBS_INVALID;
    memset(out, 0, sizeof(*out));
    sqlite3 *db = NULL;
    if (db_open_readonly_connection(&db) != 0) return DB_OBS_ERROR;
    sqlite3_stmt *stmt = NULL;
    int rc = DB_OBS_ERROR;
    if (sqlite3_prepare_v2(db, "SELECT " CLS_COLUMNS " FROM observation_classifications "
                           "WHERE observation_uuid = ? ORDER BY updated_at_ms DESC, key ASC;",
                           -1, &stmt, NULL) == SQLITE_OK) {
        sqlite3_bind_text(stmt, 1, observation_uuid, -1, SQLITE_TRANSIENT);
        int step;
        while ((step = sqlite3_step(stmt)) == SQLITE_ROW) {
            observation_classification_t row;
            read_annotation(stmt, &row);
            if (row.kind == OBS_CLS_KIND_OWNER_CONFIRMED) {
                if (out->owner_confirmed_count < OBS_CLS_MAX_PER_OBSERVATION)
                    out->owner_confirmed[out->owner_confirmed_count++] = row;
            } else if (out->automated_count < OBS_CLS_MAX_PER_OBSERVATION) {
                out->automated[out->automated_count++] = row;
            }
        }
        if (step == SQLITE_DONE) rc = DB_OBS_OK;
    }
    if (rc != DB_OBS_OK) {
        log_error("Failed to list classifications: %s", sqlite3_errmsg(db));
        memset(out, 0, sizeof(*out));
    }
    sqlite3_finalize(stmt);
    db_close_readonly_connection(db);
    return rc;
}
