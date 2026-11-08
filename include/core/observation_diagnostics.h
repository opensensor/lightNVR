/*
 * SPDX-License-Identifier: GPL-3.0-or-later
 * Copyright (C) 2026 OpenSensor Engineering and LightNVR contributors
 *
 * This file is part of LightNVR <https://github.com/opensensor/lightNVR>.
 */

/*
 * INT_01 L10: observation pipeline diagnostics
 * (docs/integrations/OBSERVATIONS_CONTRACT.md section 15).
 *
 * One read-only snapshot of the L3 evidence queue, the L4 retention pass, the
 * L5 relay and the observation tables, serialized two ways:
 *  - Prometheus text for GET /api/metrics (api_metrics_render_observations()
 *    in src/web/api_handlers_metrics.c);
 *  - JSON for GET /api/system/info ("observation_pipeline"), shown by the
 *    System view.
 *
 * Collecting never starts the evidence worker or allocates its queue: with
 * evidence disabled on every camera the counters are zero and
 * queue_allocated is false.
 */

#ifndef LIGHTNVR_CORE_OBSERVATION_DIAGNOSTICS_H
#define LIGHTNVR_CORE_OBSERVATION_DIAGNOSTICS_H

#include <stdbool.h>
#include <stdint.h>

#include <cjson/cJSON.h>

#include "core/observation_evidence.h"
#include "core/observation_relay.h"
#include "core/observation_retention.h"
#include "database/db_observations.h"

typedef struct observation_diagnostics_snapshot {
    int64_t collected_at_ms;

    observation_evidence_stats_t evidence;
    observation_relay_stats_t relay;

    /* Observation tables (db_observation_diagnostics_get); false when the DB
     * could not be read, in which case the fields below are zero. */
    bool db_available;
    observation_db_diagnostics_t db;

    /* L4 per-camera status as of its last pass (cameras holding ready
     * media). Heap-allocated; release with observation_diagnostics_free(). */
    observation_retention_camera_status_t *cameras;
    int camera_count;
    int budget_exhausted_cameras;  /* L4 flag count (authoritative) */
} observation_diagnostics_t;

/* Fill a snapshot at now_ms (0 = current wall clock). Returns 0, or -1 when
 * out is NULL. A DB read failure is reported through db_available. */
int observation_diagnostics_collect(observation_diagnostics_t *out, int64_t now_ms);
void observation_diagnostics_free(observation_diagnostics_t *diagnostics);

/* Relay lag: age of the oldest pending publication in seconds, 0 when none. */
double observation_diagnostics_relay_lag_seconds(const observation_diagnostics_t *diagnostics);

/* JSON object for system info ("observation_pipeline"); NULL on OOM. */
cJSON *observation_diagnostics_to_json(const observation_diagnostics_t *diagnostics);

/* Contract section 8.2 Retry-After for a pending snapshot: 1 while the asset
 * is under 5 s old, otherwise min(30, 2 x items_ahead), always 1..30.
 * Callers without a per-item queue position pass the evidence backlog
 * (observation_evidence_stats_t.backlog_items), an upper bound. */
#define OBS_SNAPSHOT_RETRY_AFTER_MAX 30
int observation_snapshot_retry_after_seconds(int64_t asset_age_ms, uint64_t items_ahead);

#endif /* LIGHTNVR_CORE_OBSERVATION_DIAGNOSTICS_H */
