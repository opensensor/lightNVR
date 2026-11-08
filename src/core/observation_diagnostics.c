/*
 * SPDX-License-Identifier: GPL-3.0-or-later
 * Copyright (C) 2026 OpenSensor Engineering and LightNVR contributors
 *
 * This file is part of LightNVR <https://github.com/opensensor/lightNVR>.
 */

/*
 * INT_01 L10 observation pipeline diagnostics. See
 * include/core/observation_diagnostics.h.
 */

#include "core/observation_diagnostics.h"

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>

static int64_t wall_ms(void) {
    struct timespec ts;
    clock_gettime(CLOCK_REALTIME, &ts);
    return (int64_t)ts.tv_sec * 1000 + ts.tv_nsec / 1000000;
}

/* L4 keeps one row per camera holding ready media; grow until it fits. */
static void collect_cameras(observation_diagnostics_t *out) {
    int capacity = 64;
    for (int attempt = 0; attempt < 8; ++attempt) {
        observation_retention_camera_status_t *rows = calloc((size_t)capacity, sizeof(*rows));
        if (!rows) return;
        int count = observation_retention_camera_status(rows, capacity);
        if (count < capacity || attempt == 7) {
            if (count <= 0) {
                free(rows);
                return;
            }
            out->cameras = rows;
            out->camera_count = count;
            return;
        }
        free(rows);
        capacity *= 2;
    }
}

int observation_diagnostics_collect(observation_diagnostics_t *out, int64_t now_ms) {
    if (!out) return -1;
    memset(out, 0, sizeof(*out));
    out->collected_at_ms = now_ms > 0 ? now_ms : wall_ms();
    /* Stats reads only: never starts the evidence worker or allocates its queue. */
    observation_evidence_get_stats(&out->evidence);
    observation_relay_get_stats(&out->relay);
    out->db_available = db_observation_diagnostics_get(&out->db) == DB_OBS_OK;
    collect_cameras(out);
    out->budget_exhausted_cameras = observation_retention_budget_exhausted_count();
    return 0;
}

void observation_diagnostics_free(observation_diagnostics_t *diagnostics) {
    if (!diagnostics) return;
    free(diagnostics->cameras);
    diagnostics->cameras = NULL;
    diagnostics->camera_count = 0;
}

double observation_diagnostics_relay_lag_seconds(const observation_diagnostics_t *diagnostics) {
    if (!diagnostics || !diagnostics->db_available) return 0.0;
    int64_t oldest = diagnostics->db.oldest_pending_created_at_ms;
    if (oldest <= 0 || diagnostics->collected_at_ms <= oldest) return 0.0;
    return (double)(diagnostics->collected_at_ms - oldest) / 1000.0;
}

int observation_snapshot_retry_after_seconds(int64_t asset_age_ms, uint64_t items_ahead) {
    if (asset_age_ms < 5000) return 1;
    if (items_ahead >= OBS_SNAPSHOT_RETRY_AFTER_MAX / 2) return OBS_SNAPSHOT_RETRY_AFTER_MAX;
    int seconds = (int)(2 * items_ahead);
    return seconds < 1 ? 1 : seconds;
}

/* ---- JSON --------------------------------------------------------------- */

static void add_u64(cJSON *object, const char *name, uint64_t value) {
    cJSON_AddNumberToObject(object, name, (double)value);
}

static cJSON *time_or_null(bool present, int64_t ms) {
    if (!present || ms <= 0) return cJSON_CreateNull();
    time_t seconds = (time_t)(ms / 1000);
    struct tm tm;
    if (!gmtime_r(&seconds, &tm)) return cJSON_CreateNull();
    char text[64];
    snprintf(text, sizeof(text), "%04d-%02d-%02dT%02d:%02d:%02d.%03dZ", tm.tm_year + 1900,
             tm.tm_mon + 1, tm.tm_mday, tm.tm_hour, tm.tm_min, tm.tm_sec, (int)(ms % 1000));
    return cJSON_CreateString(text);
}

cJSON *observation_diagnostics_to_json(const observation_diagnostics_t *d) {
    if (!d) return NULL;
    const observation_evidence_stats_t *e = &d->evidence;
    cJSON *root = cJSON_CreateObject();
    if (!root) return NULL;

    /* "active": anything has happened or is allocated in this process run, or
     * observations are retained. False = the feature is effectively off. */
    bool active = e->allocated || e->accepted || e->dropped_queue_full || e->dropped_budget ||
                  e->dropped_untracked || e->unavailable_submitted ||
                  (d->db_available && d->db.observations > 0);
    cJSON_AddBoolToObject(root, "active", active);
    add_u64(root, "accepted", e->accepted);

    cJSON *dropped = cJSON_AddObjectToObject(root, "dropped");
    if (dropped) {
        add_u64(dropped, "queue_full", e->dropped_queue_full);
        add_u64(dropped, "budget_exhausted", e->dropped_budget);
        add_u64(dropped, "disabled", e->dropped_disabled);
        add_u64(dropped, "untracked", e->dropped_untracked);
    }

    cJSON *evidence = cJSON_AddObjectToObject(root, "evidence");
    if (evidence) {
        cJSON_AddBoolToObject(evidence, "queue_allocated", e->allocated);
        cJSON_AddBoolToObject(evidence, "worker_running", e->worker_running);
        add_u64(evidence, "backlog_items", e->backlog_items);
        add_u64(evidence, "backlog_bytes", e->backlog_bytes);
        add_u64(evidence, "high_water_items", e->high_water_items);
        add_u64(evidence, "high_water_bytes", e->high_water_bytes);
        add_u64(evidence, "drop_ring_depth", e->drop_ring_depth);
        add_u64(evidence, "ready", e->ready);
        add_u64(evidence, "unavailable_submitted", e->unavailable_submitted);
        add_u64(evidence, "bytes_written", e->bytes_written);
        cJSON *failures = cJSON_AddObjectToObject(evidence, "failures");
        if (failures) {
            add_u64(failures, "encode_failed", e->failed_encode);
            add_u64(failures, "write_failed", e->failed_write);
            add_u64(failures, "db_error", e->failed_db);
            add_u64(failures, "disabled", e->disabled_at_encode);
            add_u64(failures, "interrupted", e->interrupted);
        }
    }

    cJSON *history = cJSON_AddObjectToObject(root, "history");
    if (history) {
        cJSON_AddBoolToObject(history, "available", d->db_available);
        cJSON_AddNumberToObject(history, "observations", (double)d->db.observations);
        cJSON_AddNumberToObject(history, "oldest_seq", (double)d->db.meta.oldest_seq);
        cJSON_AddNumberToObject(history, "head_seq", (double)d->db.meta.head_seq);
        cJSON_AddItemToObject(history, "oldest_observed_at",
                              time_or_null(d->db_available && d->db.has_oldest_observed_at,
                                           d->db.oldest_observed_at_ms));
        add_u64(history, "epoch_regenerations", d->relay.epoch_regenerations);
    }

    cJSON *publications = cJSON_AddObjectToObject(root, "publications");
    if (publications) {
        cJSON_AddNumberToObject(publications, "pending", (double)d->db.publications_pending);
        cJSON_AddNumberToObject(publications, "failed", (double)d->db.publications_failed);
        cJSON_AddNumberToObject(publications, "expired", (double)d->db.publications_expired);
        cJSON_AddNumberToObject(publications, "relay_lag_seconds",
                                observation_diagnostics_relay_lag_seconds(d));
        cJSON_AddBoolToObject(publications, "relay_running", d->relay.running);
        add_u64(publications, "relayed", d->relay.relayed);
        add_u64(publications, "relayed_no_route", d->relay.relayed_no_route);
    }

    cJSON_AddNumberToObject(root, "budget_exhausted_cameras", d->budget_exhausted_cameras);
    cJSON *cameras = cJSON_AddArrayToObject(root, "cameras");
    for (int i = 0; cameras && i < d->camera_count; ++i) {
        const observation_retention_camera_status_t *c = &d->cameras[i];
        cJSON *item = cJSON_CreateObject();
        if (!item) break;
        cJSON_AddStringToObject(item, "camera_uuid", c->camera_uuid);
        cJSON_AddNumberToObject(item, "retained_bytes", (double)c->retained_bytes);
        cJSON_AddNumberToObject(item, "max_bytes", (double)c->max_bytes);
        cJSON_AddNumberToObject(item, "max_age_days", c->max_age_days);
        cJSON_AddBoolToObject(item, "budget_exhausted", c->budget_exhausted);
        cJSON_AddItemToArray(cameras, item);
    }
    return root;
}
