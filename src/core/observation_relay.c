/*
 * SPDX-License-Identifier: GPL-3.0-or-later
 * Copyright (C) 2026 OpenSensor Engineering and LightNVR contributors
 *
 * This file is part of LightNVR <https://github.com/opensensor/lightNVR>.
 */

/*
 * INT_01 L5 publication relay. See include/core/observation_relay.h and
 * docs/integrations/OBSERVATIONS_CONTRACT.md sections 5.5, 6 and 7.6.
 *
 * Ordering and delivery rules:
 *   - Pending rows are read in seq order in bounded batches. A FULL outbox,
 *     a router/outbox error, a mark error or the D8 cap stops the batch at
 *     that row, so later rows never overtake an earlier retryable one.
 *   - A row is marked relayed / relayed_no_route only after the outbox step.
 *     A crash in between replays the same message ID, which the outbox
 *     reports as DUPLICATE (still relayed). Delivery is at least once.
 *   - event_bus_publish runs after the mark, for in-process subscribers.
 */

#define _GNU_SOURCE

#include "core/observation_relay.h"
#include "core/observation_retention.h"

#include <pthread.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>

#include <cjson/cJSON.h>

#include "core/config.h"
#include "core/event_bus.h"
#include "core/event_identity.h"
#include "core/event_producers.h"
#include "core/logger.h"
#include "database/db_event_outbox.h"
#include "utils/strings.h"

#define RELAY_DEFAULT_STARTUP_DRAIN_MS 5000

typedef struct {
    int batch_size;
    int poll_interval_ms;
    int outbox_cap;
    int startup_drain_ms;
    char storage_path[MAX_PATH_LENGTH];
} relay_settings_t;

static struct {
    pthread_mutex_t mutex;          /* state, stats, wake flags */
    pthread_cond_t wake;
    bool cond_ready;
    pthread_mutex_t run_mutex;      /* serializes passes */
    pthread_t thread;
    bool running;
    bool stop_requested;
    bool notified;
    relay_settings_t settings;
    observation_relay_stats_t stats;
    /* D9 sidecar bookkeeping (guarded by run_mutex). */
    bool sidecar_dirty;
    int64_t sidecar_last_write_ms;
} RELAY = {
    .mutex = PTHREAD_MUTEX_INITIALIZER,
    .run_mutex = PTHREAD_MUTEX_INITIALIZER,
    .sidecar_dirty = true,
};

static int64_t realtime_ms(void) {
    struct timespec ts;
    clock_gettime(CLOCK_REALTIME, &ts);
    return (int64_t)ts.tv_sec * 1000 + ts.tv_nsec / 1000000;
}

static int64_t monotonic_ms(void) {
    struct timespec ts;
    clock_gettime(CLOCK_MONOTONIC, &ts);
    return (int64_t)ts.tv_sec * 1000 + ts.tv_nsec / 1000000;
}

static void set_error(char *error, size_t error_size, const char *message) {
    if (!error || error_size == 0) return;
    snprintf(error, error_size, "%s", message ? message : "observation relay error");
}

static void ensure_cond_locked(void) {
    if (RELAY.cond_ready) return;
    pthread_condattr_t attr;
    pthread_condattr_init(&attr);
    pthread_condattr_setclock(&attr, CLOCK_MONOTONIC);
    pthread_cond_init(&RELAY.wake, &attr);
    pthread_condattr_destroy(&attr);
    RELAY.cond_ready = true;
}

static relay_settings_t resolve_settings(const observation_relay_options_t *options) {
    relay_settings_t settings;
    memset(&settings, 0, sizeof(settings));
    settings.batch_size = OBSERVATION_RELAY_DEFAULT_BATCH;
    settings.poll_interval_ms = OBSERVATION_RELAY_DEFAULT_POLL_MS;
    settings.outbox_cap = OBSERVATION_RELAY_OUTBOX_CAP;
    settings.startup_drain_ms = RELAY_DEFAULT_STARTUP_DRAIN_MS;
    const char *storage = g_config.storage_path;
    if (options) {
        if (options->batch_size > 0) settings.batch_size = options->batch_size;
        if (options->poll_interval_ms > 0) settings.poll_interval_ms = options->poll_interval_ms;
        if (options->outbox_cap > 0) settings.outbox_cap = options->outbox_cap;
        if (options->startup_drain_ms > 0) settings.startup_drain_ms = options->startup_drain_ms;
        if (options->storage_path) storage = options->storage_path;
    }
    if (settings.batch_size > OBSERVATION_RELAY_MAX_BATCH)
        settings.batch_size = OBSERVATION_RELAY_MAX_BATCH;
    safe_strcpy(settings.storage_path, storage ? storage : "", sizeof(settings.storage_path), 0);
    return settings;
}

#define STAT_ADD(field, value)          \
    do {                                \
        pthread_mutex_lock(&RELAY.mutex); \
        RELAY.stats.field += (value);   \
        pthread_mutex_unlock(&RELAY.mutex); \
    } while (0)

/* ---- envelope ------------------------------------------------------------ */

static int format_ms_utc(int64_t ms, char out[32]) {
    if (ms <= 0) return -1;
    time_t seconds = (time_t)(ms / 1000);
    struct tm utc;
    if (!gmtime_r(&seconds, &utc)) return -1;
    char base[24];
    if (strftime(base, sizeof(base), "%Y-%m-%dT%H:%M:%S", &utc) == 0) return -1;
    int written = snprintf(out, 32, "%s.%03dZ", base, (int)(ms % 1000));
    return written > 0 && written < 32 ? 0 : -1;
}

int observation_relay_build_envelope(const observation_publication_t *publication,
                                     const char *source, event_envelope_t *event,
                                     char *error, size_t error_size) {
    if (error && error_size > 0) error[0] = '\0';
    if (!publication || !source || !event) {
        set_error(error, error_size, "publication, source and event are required");
        return -1;
    }
    memset(event, 0, sizeof(*event));
    const char *state = observation_asset_state_name(publication->evidence_state);
    const char *cause = observation_change_cause_name(publication->cause);
    char observed_at[32];
    if (!state || !cause || format_ms_utc(publication->observed_at_ms, observed_at) != 0 ||
        publication->change_created_at_ms < 1000) {
        set_error(error, error_size, "publication state, cause or timestamps are invalid");
        return -1;
    }
    char subject[EVENT_SUBJECT_MAX];
    int written = snprintf(subject, sizeof(subject), "camera/%s", publication->camera_uuid);
    if (written < 0 || (size_t)written >= sizeof(subject)) {
        set_error(error, error_size, "publication camera identity is invalid");
        return -1;
    }

    /* Field order is the section 6.3 / C3 fixture order. */
    cJSON *data = cJSON_CreateObject();
    bool ok = data &&
        cJSON_AddStringToObject(data, "observation_id", publication->observation_uuid) &&
        cJSON_AddNumberToObject(data, "revision", publication->revision) &&
        cJSON_AddStringToObject(data, "camera_uuid", publication->camera_uuid) &&
        cJSON_AddStringToObject(data, "label", publication->label) &&
        cJSON_AddStringToObject(data, "observed_at", observed_at) &&
        cJSON_AddStringToObject(data, "change_cause", cause) &&
        cJSON_AddStringToObject(data, "evidence_state", state) &&
        (publication->evidence_reason[0]
             ? cJSON_AddStringToObject(data, "evidence_reason", publication->evidence_reason) != NULL
             : cJSON_AddNullToObject(data, "evidence_reason") != NULL) &&
        cJSON_AddStringToObject(data, "asset_id", publication->asset_uuid);
    if (ok && publication->evidence_state == OBS_ASSET_READY) {
        char url[96];
        snprintf(url, sizeof(url), "/api/observations/%s/snapshot",
                 publication->observation_uuid);
        ok = cJSON_AddStringToObject(data, "snapshot_url", url) != NULL;
    }
    if (!ok) {
        cJSON_Delete(data);
        set_error(error, error_size, "observation event allocation failed");
        return -1;
    }
    /* time = change created_at truncated to seconds: retries are identical. */
    int result = event_envelope_create_with_severity_and_id(
        event, OBSERVATION_CHANGED_EVENT_TYPE, source, subject,
        (time_t)(publication->change_created_at_ms / 1000), data, EVENT_SEVERITY_INFO,
        publication->message_id, error, error_size);
    cJSON_Delete(data);
    return result;
}

/* ---- D9 cursor-head sidecar ------------------------------------------------ */

/* The sidecar file, its format and the rewind check belong to L4
 * (observation_cursor_head_write/read/reconcile in observation_retention.c);
 * the relay only decides when to write it. */

static int write_sidecar_now(const char *storage_path) {
    observation_meta_t meta;
    if (db_observation_meta_get(&meta) != DB_OBS_OK ||
        observation_cursor_head_write(storage_path, meta.epoch_uuid, meta.head_seq) != 0) {
        STAT_ADD(sidecar_errors, 1);
        return -1;
    }
    STAT_ADD(sidecar_writes, 1);
    return 0;
}

int observation_relay_write_cursor_head(const char *storage_path) {
    const char *path = storage_path ? storage_path : g_config.storage_path;
    if (!path || !path[0]) return -1;
    return write_sidecar_now(path);
}

int observation_relay_check_cursor_head(const char *storage_path) {
    const char *path = storage_path ? storage_path : g_config.storage_path;
    if (!path || !path[0]) return 0;
    /* L4 already ran this at startup, before reconciliation could advance the
     * head; here it is a no-op unless the sidecar changed in between. */
    bool regenerated = false;
    if (observation_cursor_head_reconcile(path, &regenerated) != 0) {
        STAT_ADD(sidecar_errors, 1);
        return -1;
    }
    STAT_ADD(sidecar_writes, 1);
    if (!regenerated) return 0;
    STAT_ADD(epoch_regenerations, 1);
    return 1;
}

/* Caller holds run_mutex. Writes at most once per second after head moves. */
static void maybe_write_sidecar_locked(const relay_settings_t *settings) {
    if (!RELAY.sidecar_dirty || !settings->storage_path[0]) return;
    int64_t now = monotonic_ms();
    if (RELAY.sidecar_last_write_ms != 0 && now - RELAY.sidecar_last_write_ms < 1000) return;
    if (write_sidecar_now(settings->storage_path) == 0) {
        RELAY.sidecar_dirty = false;
        RELAY.sidecar_last_write_ms = now;
    }
}

/* ---- relay pass ------------------------------------------------------------ */

static bool mark(int64_t seq, observation_publication_state_t state, const char *error,
                 bool *cas_lost) {
    int rc = db_observation_publication_mark(seq, state, error);
    *cas_lost = rc == DB_OBS_CAS_LOST || rc == DB_OBS_NOT_FOUND;
    if (rc == DB_OBS_OK || *cas_lost) return true;
    STAT_ADD(mark_errors, 1);
    log_error("Observation relay could not mark publication %lld (%d)", (long long)seq, rc);
    return false;
}

static int run_pass_locked(const relay_settings_t *settings, observation_relay_pass_t *pass) {
    observation_publication_t *rows = calloc((size_t)settings->batch_size, sizeof(*rows));
    if (!rows) {
        pass->backoff = true;
        STAT_ADD(read_errors, 1);
        return -1;
    }
    int count = db_observation_pending_publications(rows, settings->batch_size);
    if (count < 0) {
        free(rows);
        pass->backoff = true;
        STAT_ADD(read_errors, 1);
        return -1;
    }
    pass->read = count;
    pthread_mutex_lock(&RELAY.mutex);
    RELAY.stats.passes++;
    RELAY.stats.last_pass_pending = count;
    RELAY.stats.oldest_pending_created_at_ms = count > 0 ? rows[0].change_created_at_ms : 0;
    pthread_mutex_unlock(&RELAY.mutex);
    if (count == 0) {
        free(rows);
        maybe_write_sidecar_locked(settings);
        return 0;
    }
    RELAY.sidecar_dirty = true;

    char source[EVENT_SOURCE_MAX];
    int64_t outbox_pending = 0;
    if (event_identity_get_source(source, sizeof(source)) != 0 ||
        db_event_outbox_count_undelivered_type(OBSERVATION_CHANGED_EVENT_TYPE,
                                               &outbox_pending) != 0) {
        free(rows);
        pass->backoff = true;
        STAT_ADD(read_errors, 1);
        return -1;
    }
    pthread_mutex_lock(&RELAY.mutex);
    RELAY.stats.outbox_type_pending = outbox_pending;
    pthread_mutex_unlock(&RELAY.mutex);
    int64_t room = settings->outbox_cap - outbox_pending;
    const event_type_definition_t *definition =
        event_registry_find(OBSERVATION_CHANGED_EVENT_TYPE);
    int64_t expiry_ms = (int64_t)(definition ? definition->default_expiry_seconds : 3600) * 1000;

    for (int index = 0; index < count; index++) {
        const observation_publication_t *row = &rows[index];
        bool cas_lost = false;

        /* Section 5.5: past the type's delivery expiry -> expired. */
        if (realtime_ms() >= row->change_created_at_ms + expiry_ms) {
            if (!mark(row->change_seq, OBS_PUBLICATION_EXPIRED, "delivery expiry elapsed",
                      &cas_lost)) {
                pass->backoff = true;
                break;
            }
            if (!cas_lost) {
                pass->processed++;
                STAT_ADD(expired, 1);
            }
            continue;
        }

        event_envelope_t event;
        char error[256] = "";
        if (observation_relay_build_envelope(row, source, &event, error, sizeof(error)) != 0) {
            log_error("Observation publication %lld is not a valid event: %s",
                      (long long)row->change_seq, error);
            if (!mark(row->change_seq, OBS_PUBLICATION_FAILED, error, &cas_lost)) {
                pass->backoff = true;
                break;
            }
            if (!cas_lost) {
                pass->processed++;
                STAT_ADD(failed, 1);
            }
            continue;
        }

        /* D8: leave the rest pending while the type holds its outbox share. */
        if (room <= 0) {
            event_envelope_clear(&event);
            pass->cap_reached = true;
            pass->backoff = true;
            STAT_ADD(cap_pauses, 1);
            break;
        }

        event_producer_enqueue_result_t result =
            event_producer_enqueue_durable(&event, error, sizeof(error));
        observation_publication_state_t state;
        bool retry = false;
        switch (result) {
            case EVENT_PRODUCER_ENQUEUE_ENQUEUED:
                room--;
                state = OBS_PUBLICATION_RELAYED;
                break;
            case EVENT_PRODUCER_ENQUEUE_DUPLICATE:
                state = OBS_PUBLICATION_RELAYED;
                break;
            case EVENT_PRODUCER_ENQUEUE_NO_ROUTE:
            case EVENT_PRODUCER_ENQUEUE_MQTT_DISABLED:
                state = OBS_PUBLICATION_RELAYED_NO_ROUTE;
                break;
            case EVENT_PRODUCER_ENQUEUE_FULL:
                STAT_ADD(enqueue_full, 1);
                state = OBS_PUBLICATION_PENDING;
                retry = true;
                set_error(error, sizeof(error), "outbox full");
                break;
            case EVENT_PRODUCER_ENQUEUE_FAILED:
            default:
                STAT_ADD(enqueue_errors, 1);
                state = OBS_PUBLICATION_PENDING;
                retry = true;
                if (!error[0]) set_error(error, sizeof(error), "durable enqueue failed");
                break;
        }
        /* PENDING records the attempt and error; the row stays retryable. */
        bool marked = mark(row->change_seq, state, retry ? error : NULL, &cas_lost);
        if (retry || !marked) {
            event_envelope_clear(&event);
            pass->backoff = true;
            break;
        }
        if (!cas_lost) {
            pass->processed++;
            pthread_mutex_lock(&RELAY.mutex);
            if (state == OBS_PUBLICATION_RELAYED) {
                RELAY.stats.relayed++;
                if (result == EVENT_PRODUCER_ENQUEUE_DUPLICATE) RELAY.stats.duplicates++;
            } else {
                RELAY.stats.relayed_no_route++;
            }
            RELAY.stats.last_relayed_seq = row->change_seq;
            pthread_mutex_unlock(&RELAY.mutex);
            /* In-process subscribers after the durable step. The MQTT
             * compatibility subscriber re-evaluates routes and hits outbox
             * duplicate detection, as for system-health transitions. */
            (void)event_bus_publish(&event, NULL, 0);
        }
        event_envelope_clear(&event);
    }
    free(rows);
    maybe_write_sidecar_locked(settings);
    return pass->processed;
}

int observation_relay_run_once(const observation_relay_options_t *options,
                               observation_relay_pass_t *pass) {
    observation_relay_pass_t local;
    if (!pass) pass = &local;
    memset(pass, 0, sizeof(*pass));
    relay_settings_t settings;
    if (options) {
        settings = resolve_settings(options);
    } else {
        pthread_mutex_lock(&RELAY.mutex);
        bool running = RELAY.running;
        settings = RELAY.settings;
        pthread_mutex_unlock(&RELAY.mutex);
        if (!running) settings = resolve_settings(NULL);
    }
    pthread_mutex_lock(&RELAY.run_mutex);
    int result = run_pass_locked(&settings, pass);
    pthread_mutex_unlock(&RELAY.run_mutex);
    return result;
}

/* ---- thread ---------------------------------------------------------------- */

static void *relay_main(void *argument) {
    (void)argument;
    int backoff_ms = 0;
    for (;;) {
        pthread_mutex_lock(&RELAY.mutex);
        bool stop = RELAY.stop_requested;
        relay_settings_t settings = RELAY.settings;
        RELAY.notified = false;
        pthread_mutex_unlock(&RELAY.mutex);
        if (stop) break;

        observation_relay_pass_t pass;
        memset(&pass, 0, sizeof(pass));
        pthread_mutex_lock(&RELAY.run_mutex);
        (void)run_pass_locked(&settings, &pass);
        pthread_mutex_unlock(&RELAY.run_mutex);

        int wait_ms;
        bool in_backoff = pass.backoff;
        if (pass.backoff) {
            backoff_ms = backoff_ms <= 0 ? 1000 : backoff_ms * 2;
            if (backoff_ms > OBSERVATION_RELAY_MAX_BACKOFF_MS)
                backoff_ms = OBSERVATION_RELAY_MAX_BACKOFF_MS;
            wait_ms = backoff_ms;
        } else {
            backoff_ms = 0;
            /* A full batch probably has more behind it. */
            wait_ms = pass.read >= settings.batch_size ? 0 : settings.poll_interval_ms;
        }

        pthread_mutex_lock(&RELAY.mutex);
        if (in_backoff) {
            RELAY.stats.backoffs++;
            RELAY.stats.consecutive_failures++;
            RELAY.stats.backoff_until_ms = realtime_ms() + wait_ms;
        } else {
            RELAY.stats.consecutive_failures = 0;
            RELAY.stats.backoff_until_ms = 0;
        }
        int64_t deadline = monotonic_ms() + wait_ms;
        while (!RELAY.stop_requested && (in_backoff || !RELAY.notified)) {
            int64_t remaining = deadline - monotonic_ms();
            if (remaining <= 0) break;
            struct timespec until;
            clock_gettime(CLOCK_MONOTONIC, &until);
            until.tv_sec += remaining / 1000;
            until.tv_nsec += (remaining % 1000) * 1000000L;
            if (until.tv_nsec >= 1000000000L) {
                until.tv_sec++;
                until.tv_nsec -= 1000000000L;
            }
            pthread_cond_timedwait(&RELAY.wake, &RELAY.mutex, &until);
        }
        pthread_mutex_unlock(&RELAY.mutex);
    }
    return NULL;
}

int observation_relay_start(const observation_relay_options_t *options) {
    relay_settings_t settings = resolve_settings(options);
    pthread_mutex_lock(&RELAY.mutex);
    ensure_cond_locked();
    if (RELAY.running) {
        pthread_mutex_unlock(&RELAY.mutex);
        return 0;
    }
    RELAY.settings = settings;
    RELAY.stop_requested = false;
    RELAY.notified = false;
    pthread_mutex_unlock(&RELAY.mutex);
    db_observations_set_publication_hook(observation_relay_notify);

    /* D9 before any cursor is served for this generation. */
    if (settings.storage_path[0] && observation_relay_check_cursor_head(settings.storage_path) < 0)
        log_warn("Observation relay could not verify the cursor-head sidecar");

    /* Drain what a previous process staged, bounded so startup cannot stall. */
    int64_t drain_deadline = monotonic_ms() + settings.startup_drain_ms;
    int drained = 0;
    for (;;) {
        observation_relay_pass_t pass;
        memset(&pass, 0, sizeof(pass));
        pthread_mutex_lock(&RELAY.run_mutex);
        (void)run_pass_locked(&settings, &pass);
        pthread_mutex_unlock(&RELAY.run_mutex);
        drained += pass.processed;
        if (pass.backoff || pass.read < settings.batch_size || monotonic_ms() >= drain_deadline)
            break;
    }
    if (drained > 0) log_info("Observation relay drained %d staged publication(s)", drained);

    pthread_mutex_lock(&RELAY.mutex);
    int rc = pthread_create(&RELAY.thread, NULL, relay_main, NULL);
    if (rc == 0) {
        RELAY.running = true;
        RELAY.stats.running = true;
    }
    pthread_mutex_unlock(&RELAY.mutex);
    if (rc != 0) {
        log_error("Failed to start observation relay thread: %s", strerror(rc));
        return -1;
    }
    log_info("Observation publication relay started");
    return 0;
}

void observation_relay_stop(void) {
    pthread_mutex_lock(&RELAY.mutex);
    if (!RELAY.running) {
        pthread_mutex_unlock(&RELAY.mutex);
        return;
    }
    RELAY.stop_requested = true;
    pthread_cond_broadcast(&RELAY.wake);
    pthread_t thread = RELAY.thread;
    pthread_mutex_unlock(&RELAY.mutex);
    pthread_join(thread, NULL);
    pthread_mutex_lock(&RELAY.mutex);
    RELAY.running = false;
    RELAY.stats.running = false;
    RELAY.stop_requested = false;
    pthread_mutex_unlock(&RELAY.mutex);
    log_info("Observation publication relay stopped");
}

void observation_relay_notify(void) {
    pthread_mutex_lock(&RELAY.mutex);
    RELAY.notified = true;
    if (RELAY.cond_ready) pthread_cond_signal(&RELAY.wake);
    pthread_mutex_unlock(&RELAY.mutex);
}

void observation_relay_get_stats(observation_relay_stats_t *stats) {
    if (!stats) return;
    pthread_mutex_lock(&RELAY.mutex);
    *stats = RELAY.stats;
    pthread_mutex_unlock(&RELAY.mutex);
}

void observation_relay_reset_stats(void) {
    pthread_mutex_lock(&RELAY.run_mutex);
    RELAY.sidecar_dirty = true;
    RELAY.sidecar_last_write_ms = 0;
    pthread_mutex_unlock(&RELAY.run_mutex);
    pthread_mutex_lock(&RELAY.mutex);
    bool running = RELAY.stats.running;
    memset(&RELAY.stats, 0, sizeof(RELAY.stats));
    RELAY.stats.running = running;
    pthread_mutex_unlock(&RELAY.mutex);
}
