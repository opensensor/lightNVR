/*
 * SPDX-License-Identifier: GPL-3.0-or-later
 * Copyright (C) 2026 OpenSensor Engineering and LightNVR contributors
 *
 * This file is part of LightNVR <https://github.com/opensensor/lightNVR>.
 */

/*
 * INT_01 durable object observations (contract:
 * docs/integrations/OBSERVATIONS_CONTRACT.md, sections 3-5, 7 and 14).
 *
 * Every mutating call is one BEGIN IMMEDIATE transaction under the global DB
 * mutex (db_auth.c pattern) and writes the observation/asset rows, their
 * observation_changes rows and their observation_publications rows (with the
 * deterministic UUIDv5 message ID) atomically. No mutex-taking db_* helper is
 * called inside. Reads use a private read-only connection inside one read
 * transaction, so they never hold the global DB mutex.
 *
 * Mutations need the installation source (event_identity_init() must have
 * run) because message IDs are derived from it.
 */

#ifndef LIGHTNVR_DATABASE_DB_OBSERVATIONS_H
#define LIGHTNVR_DATABASE_DB_OBSERVATIONS_H

#include <sqlite3.h>
#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>

#include "core/config.h"

#define OBS_UUID_SIZE 37
#define OBS_MAX_BATCH 20                 /* MAX_DETECTIONS; object_index 0..19 */
#define OBS_LABEL_SIZE 32                /* 1..31 bytes */
#define OBS_ENGINE_KEY_SIZE 65
#define OBS_ENGINE_TYPE_SIZE 16
#define OBS_MODEL_SIZE 129
#define OBS_MODEL_VERSION_SIZE 65
#define OBS_TRACK_ID_SIZE 65
#define OBS_STREAM_NAME_SIZE 64          /* stream_name_at_capture <= 63 */
#define OBS_REASON_SIZE 33
#define OBS_ORIGIN_SIZE 24
#define OBS_MEDIA_TYPE_SIZE 24
#define OBS_TRANSFORM_SIZE 1024
#define OBS_CURSOR_SIZE 257              /* contract: cursor over 256 chars is invalid */
#define OBS_ERROR_TEXT_SIZE 256

#define OBS_LIST_DEFAULT_LIMIT 100
#define OBS_LIST_MAX_LIMIT 500
#define OBS_LIST_SCAN_CAP 5000           /* scan cap per page (sections 7.3, 13) */
#define OBS_QUERY_MAX_LABELS 8

/* Message-ID namespace (contract section 5.4). */
#define OBS_MESSAGE_ID_NAMESPACE "0bc01005-a431-522b-9124-a114f67d187a"

typedef enum {
    DB_OBS_OK = 0,
    DB_OBS_DUPLICATE = 1,          /* insert retry: nothing written, existing IDs returned */
    DB_OBS_CAS_LOST = 2,           /* transition: asset is no longer in the from-state */
    DB_OBS_ERROR = -1,             /* SQLite/IO failure; everything rolled back */
    DB_OBS_INVALID = -2,           /* argument or contract validation failure */
    DB_OBS_NOT_FOUND = -3,
    DB_OBS_INVALID_CURSOR = -4,    /* 400 invalid_cursor */
    DB_OBS_CURSOR_EXPIRED = -5,    /* 410 cursor_expired */
    DB_OBS_CONFLICT = -6           /* batch_id reused for another camera */
} db_obs_result_t;

typedef enum {
    OBS_ASSET_PENDING = 0,
    OBS_ASSET_READY,
    OBS_ASSET_UNAVAILABLE,
    OBS_ASSET_EXPIRED
} observation_asset_state_t;

typedef enum {
    OBS_CAUSE_CREATED = 0,
    OBS_CAUSE_EVIDENCE,
    OBS_CAUSE_CLASSIFICATION
} observation_change_cause_t;

typedef enum {
    OBS_PUBLICATION_PENDING = 0,
    OBS_PUBLICATION_RELAYED,
    OBS_PUBLICATION_RELAYED_NO_ROUTE,
    OBS_PUBLICATION_EXPIRED,
    OBS_PUBLICATION_FAILED
} observation_publication_state_t;

const char *observation_asset_state_name(observation_asset_state_t state);
int observation_asset_state_parse(const char *name, observation_asset_state_t *state);
const char *observation_change_cause_name(observation_change_cause_t cause);
const char *observation_publication_state_name(observation_publication_state_t state);

/* True when reason is in the bounded section 4.3 enum and valid with state.
 * pending/ready require reason == NULL or "". */
bool observation_reason_is_valid(observation_asset_state_t state, const char *reason);

/* Deterministic message ID: UUIDv5(NS, source|observation_id|revision). */
int observation_message_id(const char *source, const char *observation_uuid,
                           int revision, char out[OBS_UUID_SIZE]);

/* Opaque cursor: base64url (no padding) of {"v":1,"seq":N,"epoch":"<uuid>"}. */
int observation_cursor_encode(int64_t seq, const char *epoch_uuid,
                              char *out, size_t out_size);
/* DB_OBS_OK or DB_OBS_INVALID_CURSOR (malformed, v != 1, seq < 0, > 256 chars). */
int observation_cursor_decode(const char *cursor, int64_t *seq,
                              char epoch_uuid[OBS_UUID_SIZE]);

/* ---- Insert ---------------------------------------------------------- */

typedef struct {
    int object_index;                        /* 0..19, unique in the batch */
    char label[OBS_LABEL_SIZE];
    double confidence;                       /* 0..1 */
    bool has_box;                            /* normalized xywh to the stored image */
    double box_x, box_y, box_width, box_height;
    bool has_track;
    char track_id[OBS_TRACK_ID_SIZE];
    char track_namespace[OBS_ENGINE_KEY_SIZE];
    char track_session[OBS_UUID_SIZE];
} observation_object_input_t;

typedef struct {
    char batch_id[OBS_UUID_SIZE];
    char camera_uuid[OBS_UUID_SIZE];         /* capture-time identity from the stream row */
    char stream_name[OBS_STREAM_NAME_SIZE];  /* display only; "" = taken from the stream row */
    int64_t observed_at_ms;
    char engine_key[OBS_ENGINE_KEY_SIZE];
    char engine_type[OBS_ENGINE_TYPE_SIZE];  /* object|api|onvif|external */
    char model[OBS_MODEL_SIZE];              /* basename or "" */
    char model_version[OBS_MODEL_VERSION_SIZE];
    int64_t recording_id;                    /* 0 = none */
    int object_count;                        /* 1..OBS_MAX_BATCH */
    observation_object_input_t objects[OBS_MAX_BATCH];
} observation_batch_input_t;

typedef struct {
    char asset_uuid[OBS_UUID_SIZE];          /* "" = generate a UUIDv4 */
    observation_asset_state_t state;         /* initial: pending or unavailable */
    char reason[OBS_REASON_SIZE];            /* required iff unavailable */
    char origin[OBS_ORIGIN_SIZE];            /* "", inference_input, submitted_image */
    char media_type[OBS_MEDIA_TYPE_SIZE];    /* "" or image/jpeg */
    int64_t captured_at_ms;                  /* 0 = observed_at_ms */
    char transform_json[OBS_TRANSFORM_SIZE]; /* "" = NULL */
    int64_t expires_at_ms;                   /* 0 = NULL */
} observation_asset_input_t;

typedef struct {
    char asset_uuid[OBS_UUID_SIZE];
    int object_count;
    char observation_uuids[OBS_MAX_BATCH][OBS_UUID_SIZE];  /* ordered by object_index */
    int64_t last_seq;                                      /* 0 on duplicate */
} observation_insert_result_t;

/* Insert the asset row, then one observation per object (revision 1, cause
 * created) with its change and publication rows. A retry of the same batch_id
 * is a no-op returning DB_OBS_DUPLICATE and the existing IDs. */
int db_observation_insert_batch(const observation_batch_input_t *batch,
                                const observation_asset_input_t *asset,
                                observation_insert_result_t *result);

/* ---- Evidence transitions -------------------------------------------- */

typedef struct {
    const char *path;          /* ready: required (internal; never in API bodies) */
    int64_t bytes;             /* ready: >= 0 */
    int width, height;         /* ready: > 0 */
    int64_t expires_at_ms;     /* optional, 0 = unchanged */
} observation_asset_media_t;

typedef struct {
    int observations_bumped;
    int64_t last_seq;
} observation_transition_result_t;

/* Compare-and-set the asset from -> to (section 4.2 machine), then bump every
 * referencing observation's revision with cause evidence plus change and
 * publication rows. DB_OBS_CAS_LOST when the asset is no longer in from. */
int db_observation_asset_transition(const char *asset_uuid,
                                    observation_asset_state_t from,
                                    observation_asset_state_t to,
                                    const char *reason,
                                    const observation_asset_media_t *media,
                                    observation_transition_result_t *result);

/* For callers already inside a db_observations-style transaction (caller holds
 * get_db_mutex() and has run BEGIN IMMEDIATE), e.g. L13 classifications. Bumps
 * the revision, snapshots the current asset state into a new change row and
 * writes its pending publication. source is the installation URN. */
int db_observation_bump_revision_locked(sqlite3 *db, const char *source,
                                        const char *observation_uuid,
                                        observation_change_cause_t cause,
                                        int64_t now_ms,
                                        int64_t *seq_out, int *revision_out);

/* ---- Reads ------------------------------------------------------------ */

typedef struct {
    char uuid[OBS_UUID_SIZE];
    char camera_uuid[OBS_UUID_SIZE];
    observation_asset_state_t state;
    char reason[OBS_REASON_SIZE];
    char origin[OBS_ORIGIN_SIZE];
    char media_type[OBS_MEDIA_TYPE_SIZE];
    char path[MAX_PATH_LENGTH];            /* internal only */
    bool has_bytes;
    int64_t bytes;
    int width, height;                     /* 0 = NULL */
    int64_t captured_at_ms;                /* 0 = NULL */
    char transform_json[OBS_TRANSFORM_SIZE];
    bool is_protected;
    int64_t expires_at_ms;                 /* 0 = NULL */
    int64_t created_at_ms;
    int64_t updated_at_ms;
} observation_asset_record_t;

typedef struct {
    char uuid[OBS_UUID_SIZE];
    char camera_uuid[OBS_UUID_SIZE];
    char stream_name_at_capture[OBS_STREAM_NAME_SIZE];
    char batch_id[OBS_UUID_SIZE];
    int object_index;
    int64_t observed_at_ms;
    int64_t created_at_ms;
    char engine_key[OBS_ENGINE_KEY_SIZE];
    char engine_type[OBS_ENGINE_TYPE_SIZE];
    char model[OBS_MODEL_SIZE];            /* "" = NULL */
    char model_version[OBS_MODEL_VERSION_SIZE];
    char label[OBS_LABEL_SIZE];
    double confidence;
    bool has_box;
    double box_x, box_y, box_width, box_height;
    bool has_track;
    char track_id[OBS_TRACK_ID_SIZE];
    char track_namespace[OBS_ENGINE_KEY_SIZE];
    char track_session[OBS_UUID_SIZE];
    int64_t recording_id;                  /* 0 = NULL */
    int revision;
    observation_asset_record_t asset;
} observation_record_t;

/* Current state of one observation. DB_OBS_NOT_FOUND when unknown or when the
 * capture camera no longer has a streams row. Authorization is the caller's. */
int db_observation_get(const char *observation_uuid, observation_record_t *out);
int db_observation_asset_get(const char *asset_uuid, observation_asset_record_t *out);

typedef struct {
    /* Authorized camera set (already intersected with any request filter).
     * Ignored when all_cameras is true; an empty set yields no items. */
    const char (*camera_uuids)[OBS_UUID_SIZE];
    size_t camera_count;
    bool all_cameras;
    char labels[OBS_QUERY_MAX_LABELS][OBS_LABEL_SIZE];  /* ASCII case-insensitive */
    size_t label_count;
    bool has_since;
    int64_t since_ms;          /* inclusive, observed_at */
    bool has_until;
    int64_t until_ms;          /* exclusive, observed_at */
    int limit;                 /* 1..500; 0 = default 100 */
} observation_list_query_t;

typedef struct {
    observation_record_t *items;   /* heap; free with db_observation_page_free */
    size_t count;
    char next_cursor[OBS_CURSOR_SIZE];
    bool has_more;
    /* Set when the result is DB_OBS_CURSOR_EXPIRED. */
    char oldest_cursor[OBS_CURSOR_SIZE];
    bool has_oldest_observed_at;
    int64_t oldest_observed_at_ms;
} observation_page_t;

/* Section 7.3 page algorithm. cursor is NULL/"" (start of retained history),
 * "head" (D10) or an opaque cursor. Returns DB_OBS_OK, DB_OBS_INVALID,
 * DB_OBS_INVALID_CURSOR or DB_OBS_CURSOR_EXPIRED (page->oldest_* set). */
int db_observations_list(const observation_list_query_t *query, const char *cursor,
                         observation_page_t *page);
void db_observation_page_free(observation_page_t *page);

typedef struct {
    char epoch_uuid[OBS_UUID_SIZE];
    int64_t oldest_seq;
    int64_t head_seq;          /* max(MAX(seq), oldest_seq - 1) */
} observation_meta_t;

int db_observation_meta_get(observation_meta_t *out);
/* Regenerate the DB-generation epoch (D9: restore/import/rewind detection). */
int db_observation_meta_regenerate_epoch(void);

/* ---- Diagnostics (L10) ------------------------------------------------- */

typedef struct {
    observation_meta_t meta;               /* oldest recoverable cursor = oldest_seq - 1 */
    bool has_oldest_observed_at;
    int64_t oldest_observed_at_ms;         /* MIN(observed_at_ms) of retained observations */
    int64_t observations;                  /* retained observation rows */
    int64_t publications_pending;
    int64_t publications_failed;
    int64_t publications_expired;
    int64_t oldest_pending_created_at_ms;  /* change created_at of the oldest pending row; 0 = none */
} observation_db_diagnostics_t;

/* One read-only snapshot (single WAL read transaction). DB_OBS_OK or
 * DB_OBS_ERROR. */
int db_observation_diagnostics_get(observation_db_diagnostics_t *out);

/* ---- Publications (L5 relay) ------------------------------------------ */

typedef struct {
    int64_t change_seq;
    char message_id[OBS_UUID_SIZE];
    int attempts;
    char observation_uuid[OBS_UUID_SIZE];
    int revision;
    observation_change_cause_t cause;
    observation_asset_state_t evidence_state;   /* at this revision (D6) */
    char evidence_reason[OBS_REASON_SIZE];
    int64_t change_created_at_ms;
    char camera_uuid[OBS_UUID_SIZE];
    char label[OBS_LABEL_SIZE];
    int64_t observed_at_ms;
    char asset_uuid[OBS_UUID_SIZE];
} observation_publication_t;

/* Pending publications in seq order. Returns the count (0..max) or < 0. */
int db_observation_pending_publications(observation_publication_t *out, int max);

/* Pending -> state. state may be PENDING to record a retryable failure.
 * attempts is incremented and last_error stored (NULL clears). Returns
 * DB_OBS_NOT_FOUND, DB_OBS_CAS_LOST when no longer pending, or DB_OBS_OK. */
int db_observation_publication_mark(int64_t change_seq,
                                    observation_publication_state_t state,
                                    const char *last_error);

/* Post-commit notification (L5 relay wakeup). Called after a committed
 * db_observation_insert_batch() or db_observation_asset_transition() that
 * staged publication rows, outside the DB mutex. NULL clears it. Callers of
 * db_observation_bump_revision_locked() own their COMMIT and must notify
 * themselves (observation_relay_notify()). */
typedef void (*db_observation_publication_hook_t)(void);
void db_observations_set_publication_hook(db_observation_publication_hook_t hook);

/* ---- Retention -------------------------------------------------------- */

typedef struct {
    int changes_deleted;
    int observations_deleted;
    int assets_deleted;
    int64_t oldest_seq;
} observation_prune_result_t;

/* Delete up to batch change rows in ascending seq while created_at_ms <
 * before_ms, then their observations with no remaining change rows, then
 * unreferenced assets among them whose state is unavailable or expired (a
 * ready/pending asset keeps its row so L4 can delete its file first). Advances
 * observation_meta.oldest_seq to last pruned seq + 1 in the same transaction. */
int db_observations_prune(int64_t before_ms, int batch,
                          observation_prune_result_t *result);

#endif /* LIGHTNVR_DATABASE_DB_OBSERVATIONS_H */
