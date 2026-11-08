/*
 * SPDX-License-Identifier: GPL-3.0-or-later
 * Copyright (C) 2026 OpenSensor Engineering and LightNVR contributors
 *
 * This file is part of LightNVR <https://github.com/opensensor/lightNVR>.
 */

/*
 * INT_01 L4: startup reconciliation, retention and eviction of observation
 * evidence (docs/integrations/OBSERVATIONS_CONTRACT.md sections 4.2, 4.5,
 * 7.6, 13, 14 and 15).
 *
 * Files live under <storage_path>/observations/<camera_uuid>/<yyyy-mm-dd>/
 * <asset_uuid>.jpg (".jpg.tmp" while being written). observation_assets.path
 * holds the path relative to <storage_path> ("observations/..."); a path
 * relative to <storage_path>/observations or an absolute path inside it is
 * also accepted. Nothing outside <storage_path>/observations is ever
 * deleted, so recordings are never touched.
 *
 * Startup order (main.c, design decision 2): observation_retention_startup()
 * runs before the L3 evidence worker and the L5 relay start. Periodic runs
 * (observation_retention_maintenance_tick(), driven by the storage manager
 * heartbeat) are no-ops until startup reconciliation has completed, and never
 * touch pending assets, which belong to the worker.
 */

#ifndef LIGHTNVR_CORE_OBSERVATION_RETENTION_H
#define LIGHTNVR_CORE_OBSERVATION_RETENTION_H

#include <stdbool.h>
#include <stdint.h>

#include "database/db_observations.h"

#define OBS_RETENTION_DIR "observations"
#define OBS_CURSOR_HEAD_FILE ".cursor_head.json"

#define OBS_RETENTION_INTERVAL_SEC 300        /* periodic pass cadence */
#define OBS_RETENTION_MAX_EVICTIONS 256       /* asset expiries/deletions per pass */
#define OBS_RETENTION_MAX_PRUNED_CHANGES 5000 /* history change rows per pass */
#define OBS_RETENTION_PRUNE_BATCH 500         /* change rows per prune transaction */
#define OBS_RETENTION_DEFAULT_HISTORY_DAYS 30 /* section 13: 1..365 */
#define OBS_RETENTION_DEFAULT_MAX_AGE_DAYS 7
#define OBS_RETENTION_DEFAULT_MAX_BYTES 2147483648LL

typedef struct {
    int tmp_files_deleted;     /* *.jpg.tmp under the observations tree */
    int stale_files_deleted;   /* <uuid>.jpg with no ready asset row pointing at it */
    int pending_interrupted;   /* pending -> unavailable(interrupted) */
    int ready_missing;         /* ready with a missing file -> unavailable(write_failed) */
    bool epoch_regenerated;    /* D9 sidecar rewind detection fired */
} observation_reconcile_result_t;

/* Startup reconciliation (section 14). Must run after the DB and the event
 * identity are initialized and before the evidence worker starts. Order:
 * cursor-head rewind check (before any transition advances the head), file
 * sweep, pending -> interrupted, ready-missing -> write_failed, sidecar
 * rewrite. Idempotent. Returns 0, or -1 when the DB could not be read (the
 * pass is then not marked done and periodic eviction stays off). */
int observation_retention_startup(const char *storage_path,
                                  observation_reconcile_result_t *result);
bool observation_retention_startup_done(void);

typedef struct {
    int assets_expired;            /* ready -> expired(retention_expired) by age or bytes */
    int unreferenced_deleted;      /* asset rows left with no observation, reclaimed */
    int files_deleted;
    int changes_pruned;
    int observations_pruned;
    int cameras_budget_exhausted;  /* over the byte budget with only protected media left */
    bool more_work;                /* a per-pass bound was hit; run again soon */
    int64_t oldest_seq;            /* observation_meta.oldest_seq after the pass */
} observation_retention_result_t;

/* One bounded periodic pass at now_ms:
 *  1. prune history older than observation_history_retention_days (advances
 *     observation_meta.oldest_seq);
 *  2. retry terminal assets' pending file deletions, then reclaim assets with
 *     no remaining observation (file, then row);
 *  3. per camera: expire unprotected ready media older than the age budget,
 *     where an asset's age is its newest reference (a shared asset survives
 *     until its last reference expires), then evict oldest unprotected ready
 *     media until within the byte budget; set the camera's budget-exhausted
 *     flag when still over budget with only protected media left.
 * Expiry is ready -> expired(retention_expired) first (CAS), then the file is
 * unlinked. The stored path is cleared only after cleanup, so failed unlinks
 * are retried by a later pass even if history has been pruned. A crash in
 * between also leaves a stale file the startup sweep removes.
 * Returns 0, 1 when skipped because startup reconciliation has not
 * completed, or -1 on a DB error. */
int observation_retention_run(const char *storage_path, int64_t now_ms,
                              observation_retention_result_t *result);

/* Storage-manager hook (heartbeat). Rate-limited to OBS_RETENTION_INTERVAL_SEC
 * unless the previous pass hit a bound; uses g_config.storage_path. Cheap
 * no-op before startup reconciliation. */
void observation_retention_maintenance_tick(void);

/* ---- Budget-exhausted flag (read by L3 admission and L10) -------------- */

/* Each pass also pushes flag changes to L3 through
 * observation_evidence_set_budget_exhausted() (weak reference: a no-op when
 * the evidence worker is not linked), so L3 admission returns DROPPED_BUDGET
 * without calling back into this module.
 *
 * Non-blocking: lock-free when no camera is flagged, otherwise a try-lock
 * (returns false on contention). */
bool observation_retention_budget_exhausted(const char *camera_uuid);
int observation_retention_budget_exhausted_count(void);

typedef struct {
    char camera_uuid[OBS_UUID_SIZE];
    int64_t retained_bytes;        /* ready media bytes after the last pass */
    int64_t max_bytes;
    int max_age_days;
    bool budget_exhausted;
} observation_retention_camera_status_t;

/* Copies up to max per-camera rows from the last pass; returns the count. */
int observation_retention_camera_status(observation_retention_camera_status_t *out, int max);

/* ---- D9 cursor head sidecar (<storage>/observations/.cursor_head.json) -- */

/* Atomic tmp + fsync + rename + dir fsync of {"epoch":..., "head_seq":N}.
 * L5 calls this when the head advances (at most once per second). */
int observation_cursor_head_write(const char *storage_path, const char *epoch_uuid,
                                  int64_t head_seq);
/* 0 on success, 1 when missing, -1 when unreadable or malformed. */
int observation_cursor_head_read(const char *storage_path, char epoch_uuid[OBS_UUID_SIZE],
                                 int64_t *head_seq);
/* Section 7.6: same epoch with DB head < sidecar head means the DB was
 * restored, so the epoch is regenerated. A missing or malformed sidecar is
 * recreated without regenerating. The sidecar is rewritten with the current
 * epoch and head either way. */
int observation_cursor_head_reconcile(const char *storage_path, bool *regenerated);

void observation_retention_reset_for_testing(void);

#endif /* LIGHTNVR_CORE_OBSERVATION_RETENTION_H */
