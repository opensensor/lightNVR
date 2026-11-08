/*
 * SPDX-License-Identifier: GPL-3.0-or-later
 * Copyright (C) 2026 OpenSensor Engineering and LightNVR contributors
 *
 * This file is part of LightNVR <https://github.com/opensensor/lightNVR>.
 */

/*
 * INT_01 L5: observation publication relay (contract
 * docs/integrations/OBSERVATIONS_CONTRACT.md sections 5.5, 6 and 7.6).
 *
 * The DB layer stages one observation_publications row per observation
 * revision in the same transaction as the change. The relay turns pending rows
 * into io.lightnvr.detection.observation_changed.v1 envelopes (deterministic
 * message ID), hands them to the router -> durable outbox through
 * event_producer_enqueue_durable(), marks them relayed / relayed_no_route only
 * after that, and then notifies in-process subscribers via event_bus_publish.
 * Delivery is at least once.
 */

#ifndef LIGHTNVR_CORE_OBSERVATION_RELAY_H
#define LIGHTNVR_CORE_OBSERVATION_RELAY_H

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>

#include "core/event_envelope.h"
#include "database/db_observations.h"

#define OBSERVATION_CHANGED_EVENT_TYPE "io.lightnvr.detection.observation_changed.v1"

/* D8: stop enqueuing while this many outbox rows of the type are undelivered. */
#define OBSERVATION_RELAY_OUTBOX_CAP 2000
#define OBSERVATION_RELAY_DEFAULT_BATCH 64
#define OBSERVATION_RELAY_MAX_BATCH 256
#define OBSERVATION_RELAY_DEFAULT_POLL_MS 1000
#define OBSERVATION_RELAY_MAX_BACKOFF_MS 30000
/* D9 sidecar, relative to the storage path. */
#define OBSERVATION_CURSOR_HEAD_FILE "observations/.cursor_head.json"

typedef struct {
    int batch_size;             /* 0 = default; clamped to 1..MAX_BATCH */
    int poll_interval_ms;       /* 0 = 1000 */
    int outbox_cap;             /* 0 = OBSERVATION_RELAY_OUTBOX_CAP */
    int startup_drain_ms;       /* 0 = 5000; bound on the synchronous start drain */
    const char *storage_path;   /* NULL = g_config.storage_path ("" disables D9 sidecar) */
} observation_relay_options_t;

typedef struct {
    bool running;
    uint64_t passes;
    uint64_t relayed;               /* outbox accepted (enqueued or duplicate) */
    uint64_t duplicates;            /* subset of relayed */
    uint64_t relayed_no_route;      /* route suppressed / no match / MQTT disabled */
    uint64_t expired;               /* older than the type's delivery expiry */
    uint64_t failed;                /* envelope build/validation failures */
    uint64_t enqueue_full;          /* outbox FULL; row left pending */
    uint64_t enqueue_errors;        /* router/outbox error; row left pending */
    uint64_t mark_errors;           /* publication_mark failures */
    uint64_t read_errors;           /* pending-publication read failures */
    uint64_t cap_pauses;            /* D8 cap reached */
    uint64_t backoffs;
    uint64_t sidecar_writes;
    uint64_t sidecar_errors;
    uint64_t epoch_regenerations;   /* D9 rewind detections */
    int64_t last_relayed_seq;
    int64_t oldest_pending_created_at_ms;  /* from the last pass; 0 = none seen */
    int last_pass_pending;                  /* rows read by the last pass */
    int64_t outbox_type_pending;            /* last observed D8 count */
    int consecutive_failures;
    int64_t backoff_until_ms;               /* 0 = not backing off */
} observation_relay_stats_t;

typedef struct {
    int read;              /* pending rows read */
    int processed;         /* rows moved out of pending */
    bool backoff;          /* FULL, error or cap: caller should back off */
    bool cap_reached;
} observation_relay_pass_t;

/* Build the section 6 envelope for one staged publication. The envelope id is
 * the stored message ID and time is the change created_at truncated to
 * seconds, so a retry produces an identical envelope. */
int observation_relay_build_envelope(const observation_publication_t *publication,
                                     const char *source, event_envelope_t *event,
                                     char *error, size_t error_size);

/* One bounded seq-ordered pass. Usable without the thread (tests, drain).
 * Returns the number of rows moved out of pending, or -1 on a read error. */
int observation_relay_run_once(const observation_relay_options_t *options,
                               observation_relay_pass_t *pass);

/* D9: compare the sidecar with the DB meta. Regenerates the epoch when the
 * sidecar epoch equals the DB epoch but its head_seq is ahead of the DB head
 * (the DB was restored). Rewrites the sidecar. Returns 1 when the epoch was
 * regenerated, 0 otherwise, -1 on error. storage_path NULL = g_config. */
int observation_relay_check_cursor_head(const char *storage_path);

/* Write the sidecar now (tmp + fsync + rename). */
int observation_relay_write_cursor_head(const char *storage_path);

/* Run the D9 check, drain pending publications (bounded), then start the
 * steady-state thread (condvar wake + poll). Idempotent. NULL = defaults. */
int observation_relay_start(const observation_relay_options_t *options);

/* Stop the thread. Pending rows stay pending for the next start. */
void observation_relay_stop(void);

/* Wake the relay after committing publication rows. Cheap and safe to call
 * from any thread, including while the relay is stopped. The DB layer calls
 * it after its own commits; L13-style callers using
 * db_observation_bump_revision_locked() call it after their COMMIT. */
void observation_relay_notify(void);

void observation_relay_get_stats(observation_relay_stats_t *stats);
void observation_relay_reset_stats(void);  /* tests */

#endif /* LIGHTNVR_CORE_OBSERVATION_RELAY_H */
