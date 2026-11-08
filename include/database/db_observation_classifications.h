/*
 * SPDX-License-Identifier: GPL-3.0-or-later
 * Copyright (C) 2026 OpenSensor Engineering and LightNVR contributors
 *
 * This file is part of LightNVR <https://github.com/opensensor/lightNVR>.
 */

/*
 * INT_01 P2 classification annotations (contract:
 * docs/integrations/OBSERVATIONS_CONTRACT.md section 10; migration 0096).
 *
 * An annotation is keyed by (observation_uuid, "<writer_namespace>/<name>").
 * Write authority is pinned to the writer_user_id stored when the key was
 * created (D3), so a later user reusing the username cannot overwrite it.
 *
 * db_observation_classification_put() runs the section 10.4 rules that need
 * stored state, in contract order, inside one BEGIN IMMEDIATE transaction
 * under the global DB mutex (db_auth.c pattern):
 *
 *   writer check -> evidence check -> idempotent retry -> owner precedence
 *   -> expected_revision CAS -> per-observation key limit -> store.
 *
 * A stored change bumps the observation revision with cause classification
 * and writes its change and publication rows in the same transaction, then
 * wakes the publication relay after COMMIT. The observation's detector fields
 * (label, confidence, box, ...) are never modified. Authentication, camera
 * authorization, namespace derivation and body parsing are the caller's (L14).
 */

#ifndef LIGHTNVR_DATABASE_DB_OBSERVATION_CLASSIFICATIONS_H
#define LIGHTNVR_DATABASE_DB_OBSERVATION_CLASSIFICATIONS_H

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>

#include "database/db_observations.h"

#define OBS_CLS_SEGMENT_MAX 32                  /* ^[a-z0-9][a-z0-9_.-]{0,31}$ */
#define OBS_CLS_NAMESPACE_SIZE (OBS_CLS_SEGMENT_MAX + 1)
#define OBS_CLS_KEY_SIZE (2 * OBS_CLS_SEGMENT_MAX + 2)   /* "<ns>/<name>" + NUL */
#define OBS_CLS_CLASSIFIER_SIZE 65              /* 1..64 bytes */
#define OBS_CLS_MODEL_VERSION_SIZE 65           /* 0..64 bytes; "" = null */
#define OBS_CLS_LABEL_SIZE 129                  /* 1..128 bytes UTF-8 */
#define OBS_CLS_TAXONOMY_SIZE 65                /* 0..64 bytes; "" = null */
#define OBS_CLS_MAX_PER_OBSERVATION 8           /* annotations_per_observation_max */
#define OBS_CLS_CONFIDENCE_EPSILON 1e-6         /* idempotent-retry comparison */

typedef enum {
    OBS_CLS_KIND_AUTOMATED = 0,
    OBS_CLS_KIND_OWNER_CONFIRMED = 1
} observation_classification_kind_t;

/* Outcomes of db_observation_classification_put(). Non-negative values are
 * contract outcomes (the transaction committed only for CREATED/UPDATED);
 * negative values are failures. */
typedef enum {
    DB_OBS_CLS_CREATED = 0,             /* 201, changed=true */
    DB_OBS_CLS_UPDATED = 1,             /* 200, changed=true */
    DB_OBS_CLS_NOOP = 2,                /* 200, changed=false (identical retry) */
    DB_OBS_CLS_REVISION_CONFLICT = 3,   /* 409 revision_conflict (+current if any) */
    DB_OBS_CLS_OWNER_LOCKED = 4,        /* 409 owner_decision_locked (+current) */
    DB_OBS_CLS_EVIDENCE_MISMATCH = 5,   /* 422 evidence_mismatch (+asset, state) */
    DB_OBS_CLS_WRITER_MISMATCH = 6,     /* 403 namespace_forbidden; current withheld */
    DB_OBS_CLS_LIMIT = 7,               /* 409 annotation_limit (+limit) */
    DB_OBS_CLS_ERROR = -1,              /* SQLite failure; everything rolled back */
    DB_OBS_CLS_INVALID = -2,            /* argument / bounds validation (400) */
    DB_OBS_CLS_NOT_FOUND = -3           /* observation unknown (404) */
} db_obs_cls_result_t;

const char *observation_classification_kind_name(observation_classification_kind_t kind);
/* 0 on success, -1 for anything other than "automated"/"owner_confirmed". */
int observation_classification_kind_parse(const char *name,
                                          observation_classification_kind_t *kind);
/* True for ^[a-z0-9][a-z0-9_.-]{0,31}$ (a key namespace or name segment). */
bool observation_classification_segment_is_valid(const char *segment);

typedef struct {
    char key[OBS_CLS_KEY_SIZE];                       /* "<ns>/<name>" */
    char writer_namespace[OBS_CLS_NAMESPACE_SIZE];
    int64_t writer_user_id;                           /* internal; never in API bodies */
    char classifier[OBS_CLS_CLASSIFIER_SIZE];
    char model_version[OBS_CLS_MODEL_VERSION_SIZE];   /* "" = null */
    char label[OBS_CLS_LABEL_SIZE];
    char taxonomy_id[OBS_CLS_TAXONOMY_SIZE];          /* "" = null */
    double confidence;
    int64_t classified_at_ms;
    char input_asset_uuid[OBS_UUID_SIZE];
    int input_revision;
    observation_classification_kind_t kind;
    int annotation_revision;
    int64_t created_at_ms;
    int64_t updated_at_ms;
} observation_classification_t;

typedef struct {
    /* Writer identity, derived by the caller from the authenticated user. */
    const char *writer_namespace;   /* must equal the key's namespace segment */
    int64_t writer_user_id;         /* > 0 */
    const char *name;               /* key name segment */
    /* Request fields (section 10.3). NULL or "" = null for the optional ones. */
    const char *classifier;
    const char *model_version;
    const char *label;
    const char *taxonomy_id;
    double confidence;              /* 0..1 */
    int64_t classified_at_ms;       /* >= 0 */
    const char *input_asset_uuid;
    int input_revision;             /* >= 1 */
    observation_classification_kind_t kind;
    int expected_revision;          /* >= 0; 0 = absent ("must not exist") */
} observation_classification_input_t;

typedef struct {
    /* Stored annotation after CREATED/UPDATED/NOOP; the current one for
     * REVISION_CONFLICT (when the key exists) and OWNER_LOCKED. Withheld for
     * WRITER_MISMATCH so another writer's values never leak. */
    bool has_annotation;
    observation_classification_t annotation;
    int observation_revision;       /* after the write; current otherwise; 0 = unknown */
    int64_t change_seq;             /* the classification change row; 0 unless written */
    /* Set for every non-negative outcome (observation found); 422 body. */
    char observation_asset_uuid[OBS_UUID_SIZE];
    observation_asset_state_t evidence_state;
    int limit;                      /* OBS_CLS_MAX_PER_OBSERVATION, for 409 annotation_limit */
} observation_classification_result_t;

/* Create or update one annotation. Requires the installation source
 * (event_identity_init()). Returns a db_obs_cls_result_t. */
int db_observation_classification_put(const char *observation_uuid,
                                      const observation_classification_input_t *input,
                                      observation_classification_result_t *result);

typedef struct {
    int automated_count;
    observation_classification_t automated[OBS_CLS_MAX_PER_OBSERVATION];
    int owner_confirmed_count;
    observation_classification_t owner_confirmed[OBS_CLS_MAX_PER_OBSERVATION];
} observation_classification_set_t;

/* Section 10.5 read: annotations split by kind, each list ordered by
 * updated_at descending (key ascending on ties). Uses a private read-only
 * connection, never the global DB mutex. An unknown observation yields two
 * empty lists and DB_OBS_OK; authorization is the caller's. Returns DB_OBS_OK,
 * DB_OBS_INVALID or DB_OBS_ERROR. */
int db_observation_classifications_list(const char *observation_uuid,
                                        observation_classification_set_t *out);

/* The same read for several observations (a history page) on one read-only
 * connection and one prepared statement. visit is called once per UUID, in
 * order, with that observation's set (valid only during the call); a non-zero
 * return stops the walk and is returned. Returns DB_OBS_OK, DB_OBS_INVALID,
 * DB_OBS_ERROR or the visitor's value. */
typedef int (*observation_classification_visit_fn)(size_t index,
                                                   const observation_classification_set_t *set,
                                                   void *context);
int db_observation_classifications_list_many(const char *const *observation_uuids, size_t count,
                                             observation_classification_visit_fn visit,
                                             void *context);

#endif /* LIGHTNVR_DATABASE_DB_OBSERVATION_CLASSIFICATIONS_H */
