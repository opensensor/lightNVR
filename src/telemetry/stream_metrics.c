/**
 * @file stream_metrics.c
 * @brief In-process stream health metric collection and ring buffers
 *
 * Implements per-stream QoS counters with a background sampler thread that
 * snapshots FPS/bitrate into ring buffers every 5 seconds for sparkline
 * rendering.  All metric updates use atomic operations or brief rwlock
 * acquisitions to minimise contention with video pipeline threads.
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <signal.h>
#include <unistd.h>
#include <errno.h>

#include "telemetry/stream_metrics.h"
#include "core/shutdown_coordinator.h"
#define LOG_COMPONENT "Metrics"
#include "core/logger.h"
#include "utils/strings.h"

/* ------------------------------------------------------------------ */
/*  Global state                                                       */
/* ------------------------------------------------------------------ */

static stream_metrics_t *g_metrics = NULL;   /* dynamically allocated array */
static int               g_max_streams = 0;
static bool              g_initialized = false;

/* Sampler thread */
static pthread_t         g_sampler_thread;
static volatile bool     g_sampler_running = false;
#define SAMPLER_INTERVAL_SEC 5

/* ------------------------------------------------------------------ */
/*  Internal helpers                                                   */
/* ------------------------------------------------------------------ */

/**
 * Find an existing slot by name, or the first free slot.
 * Caller must NOT hold any slot lock.  Returns index or -1.
 */
static int find_slot(const char *stream_name) {
    int first_free = -1;
    for (int i = 0; i < g_max_streams; i++) {
        if (g_metrics[i].active &&
            strncmp(g_metrics[i].stream_name, stream_name, MAX_STREAM_NAME) == 0) {
            return i;
        }
        if (!g_metrics[i].active && first_free < 0) {
            first_free = i;
        }
    }
    return first_free;  /* -1 if all slots taken */
}

/**
 * Find an existing slot by name only (no allocation).
 */
static int find_active_slot(const char *stream_name) {
    for (int i = 0; i < g_max_streams; i++) {
        if (g_metrics[i].active &&
            strncmp(g_metrics[i].stream_name, stream_name, MAX_STREAM_NAME) == 0) {
            return i;
        }
    }
    return -1;
}

/* ------------------------------------------------------------------ */
/*  Sampler thread                                                     */
/* ------------------------------------------------------------------ */

static void *sampler_thread_func(void *arg) {
    (void)arg;
    log_set_thread_context("MetricsSampler", NULL);
    log_info("Metrics sampler thread started (interval: %ds)", SAMPLER_INTERVAL_SEC);

    while (g_sampler_running) {
        /* Interruptible sleep */
        for (int s = 0; s < SAMPLER_INTERVAL_SEC && g_sampler_running; s++) {
            sleep(1);
            if (is_shutdown_initiated()) {
                g_sampler_running = false;
                break;
            }
        }
        if (!g_sampler_running) break;

        time_t now = time(NULL);

        for (int i = 0; i < g_max_streams; i++) {
            stream_metrics_t *m = &g_metrics[i];
            if (!m->active) continue;

            pthread_rwlock_wrlock(&m->lock);

            /* --- Compute FPS and bitrate from counter deltas --- */
            uint64_t cur_frames = atomic_load(&m->frames_total);
            uint64_t cur_bytes  = atomic_load(&m->bytes_received);

            if (m->prev_sample_time > 0) {
                double dt = difftime(now, m->prev_sample_time);
                if (dt > 0.0) {
                    uint64_t df = cur_frames - m->prev_frames_total;
                    uint64_t db = cur_bytes  - m->prev_bytes_received;
                    m->current_fps         = 0.5 * m->current_fps + 0.5 * (double)df / dt;
                    m->current_bitrate_bps = 0.5 * m->current_bitrate_bps + 0.5 * ((double)db * 8.0) / dt;
                }
            }
            m->prev_frames_total  = cur_frames;
            m->prev_bytes_received = cur_bytes;
            m->prev_sample_time   = now;

            /* --- Update uptime --- */
            if (m->stream_start_time > 0) {
                atomic_store(&m->uptime_seconds, (uint64_t)difftime(now, m->stream_start_time));
            }

            /* --- Health status --- */
            time_t last_frame = (time_t)atomic_load(&m->last_frame_ts);
            double frame_age  = (last_frame > 0) ? difftime(now, last_frame) : 999.0;
            double cfg_fps    = m->configured_fps;

            stream_health_status_t status;
            if (frame_age > STREAM_HEALTH_FRAME_TIMEOUT_DOWN) {
                status = STREAM_HEALTH_DOWN;
            } else if (frame_age > STREAM_HEALTH_FRAME_TIMEOUT_UP ||
                       m->current_fps < cfg_fps * STREAM_HEALTH_FPS_RATIO) {
                status = STREAM_HEALTH_DEGRADED;
            } else {
                status = STREAM_HEALTH_UP;
            }
            atomic_store(&m->health_status, (int)status);
            atomic_store(&m->stream_up, status == STREAM_HEALTH_UP ? 1 : 0);

            /* --- Append ring buffer sample --- */
            int idx = m->ring_head;
            m->ring[idx].fps          = (float)m->current_fps;
            m->ring[idx].bitrate_kbps = (float)(m->current_bitrate_bps / 1000.0);
            m->ring[idx].timestamp    = now;
            m->ring_head = (idx + 1) % METRICS_RING_SIZE;
            if (m->ring_count < METRICS_RING_SIZE) {
                m->ring_count++;
            }

            pthread_rwlock_unlock(&m->lock);
        }
    }

    log_info("Metrics sampler thread exiting");
    return NULL;
}

/* ------------------------------------------------------------------ */
/*  Public API                                                         */
/* ------------------------------------------------------------------ */

int metrics_init(int max_streams) {
    if (g_initialized) {
        log_warn("metrics_init called but already initialized");
        return 0;
    }
    if (max_streams <= 0 || max_streams > MAX_STREAMS) {
        max_streams = MAX_STREAMS;
    }

    g_max_streams = max_streams;
    g_metrics = calloc((size_t)max_streams, sizeof(stream_metrics_t));
    if (!g_metrics) {
        log_error("Failed to allocate metrics array for %d streams", max_streams);
        return -1;
    }

    for (int i = 0; i < max_streams; i++) {
        pthread_rwlock_init(&g_metrics[i].lock, NULL);
    }

    g_initialized = true;

    /* Start sampler thread */
    g_sampler_running = true;
    if (pthread_create(&g_sampler_thread, NULL, sampler_thread_func, NULL) != 0) {
        log_error("Failed to create metrics sampler thread: %s", strerror(errno));
        g_sampler_running = false;
        /* Non-fatal: metrics still work, just no sparkline/health updates */
    }

    log_info("Metrics subsystem initialized for %d streams", max_streams);
    return 0;
}

void metrics_shutdown(void) {
    if (!g_initialized) return;

    /* Stop sampler thread */
    if (g_sampler_running) {
        g_sampler_running = false;
        pthread_kill(g_sampler_thread, SIGALRM);
        sched_yield();

        pthread_join(g_sampler_thread, NULL);
        log_info("Metrics sampler thread stopped");
    }

    /* Destroy locks and free */
    for (int i = 0; i < g_max_streams; i++) {
        pthread_rwlock_destroy(&g_metrics[i].lock);
    }
    free(g_metrics);
    g_metrics = NULL;
    g_max_streams = 0;
    g_initialized = false;

    log_info("Metrics subsystem shut down");
}

// Allocate a new slot
static int metrics_get_slot(const char *stream_name) {
    if (!g_initialized || !stream_name || !stream_name[0]) return -1;

    while (1) {
        int idx = find_slot(stream_name);
        if (idx < 0) return -1;

        stream_metrics_t *m = &g_metrics[idx];
        if (!m->active) {
            pthread_rwlock_wrlock(&m->lock);
            /* Double-check after acquiring lock */
            if (!m->active) {
                memset(m->stream_name, 0, sizeof(m->stream_name));
                safe_strcpy(m->stream_name, stream_name, MAX_STREAM_NAME, 0);
                m->active = true;
                m->stream_start_time = time(NULL);
                m->prev_sample_time = 0;
                m->prev_frames_total = 0;
                m->prev_bytes_received = 0;
                m->ring_head = 0;
                m->ring_count = 0;
                m->last_segment_end_time = 0;
                m->configured_fps = 30.0; /* default, overridden by set_configured_fps */
                atomic_store(&m->health_status, (int)STREAM_HEALTH_DOWN);
                atomic_store(&m->stream_up, 0);
                atomic_store(&m->last_frame_ts, 0);
                atomic_store(&m->frames_total, 0);
                atomic_store(&m->frames_dropped, 0);
                atomic_store(&m->bytes_received, 0);
                atomic_store(&m->reconnects_total, 0);
                atomic_store(&m->uptime_seconds, 0);
                atomic_store(&m->error_decode, 0);
                atomic_store(&m->error_timeout, 0);
                atomic_store(&m->error_protocol, 0);
                atomic_store(&m->error_io, 0);
                atomic_store(&m->recording_active, 0);
                atomic_store(&m->recording_bytes_written, 0);
                atomic_store(&m->recording_segments_total, 0);
                atomic_store(&m->recording_gaps_total, 0);
                log_info("Metrics slot %d allocated for stream '%s'", idx, stream_name);
                pthread_rwlock_unlock(&m->lock);
                return idx;
            }
            pthread_rwlock_unlock(&m->lock);

            // If m->active is true but the stream_name doesn't match,
            // someone else took this empty slot. We must loop and try again!
            if (strncmp(m->stream_name, stream_name, MAX_STREAM_NAME) != 0) {
                continue;
            }
        }
        return idx;
    }
}

void metrics_release_slot(const char *stream_name) {
    if (!g_initialized || !stream_name) return;

    int idx = find_active_slot(stream_name);
    if (idx < 0) return;

    stream_metrics_t *m = &g_metrics[idx];
    pthread_rwlock_wrlock(&m->lock);
    m->active = false;
    log_info("Metrics slot %d released for stream '%s'", idx, stream_name);
    pthread_rwlock_unlock(&m->lock);
}

void metrics_record_frame(const char *stream_name, int bytes, bool is_video) {
    if (!g_initialized || !stream_name) return;

    int idx = metrics_get_slot(stream_name);
    if (idx < 0) return;

    stream_metrics_t *m = &g_metrics[idx];
    if (is_video) {
        atomic_fetch_add(&m->frames_total, 1);
    }
    atomic_fetch_add(&m->bytes_received, (uint64_t)bytes);
    atomic_store(&m->last_frame_ts, (int_fast64_t)time(NULL));
}

void metrics_record_drop(const char *stream_name) {
    if (!g_initialized || !stream_name) return;
    int idx = metrics_get_slot(stream_name);
    if (idx < 0) return;
    atomic_fetch_add(&g_metrics[idx].frames_dropped, 1);
}

void metrics_record_error(const char *stream_name, const char *error_type) {
    if (!g_initialized || !stream_name || !error_type) return;
    int idx = metrics_get_slot(stream_name);
    if (idx < 0) return;

    stream_metrics_t *m = &g_metrics[idx];
    if (strcmp(error_type, "decode") == 0) {
        atomic_fetch_add(&m->error_decode, 1);
    } else if (strcmp(error_type, "timeout") == 0) {
        atomic_fetch_add(&m->error_timeout, 1);
    } else if (strcmp(error_type, "protocol") == 0) {
        atomic_fetch_add(&m->error_protocol, 1);
    } else if (strcmp(error_type, "io") == 0) {
        atomic_fetch_add(&m->error_io, 1);
    }
}

void metrics_record_reconnect(const char *stream_name) {
    if (!g_initialized || !stream_name) return;
    int idx = metrics_get_slot(stream_name);
    if (idx < 0) return;
    atomic_fetch_add(&g_metrics[idx].reconnects_total, 1);
}

void metrics_record_segment_complete(const char *stream_name, time_t start_time,
                                     time_t end_time, uint64_t bytes) {
    if (!g_initialized || !stream_name) return;
    int idx = metrics_get_slot(stream_name);
    if (idx < 0) return;

    stream_metrics_t *m = &g_metrics[idx];
    pthread_rwlock_wrlock(&m->lock);

    /* Gap detection: compare with previous segment end */
    if (m->last_segment_end_time > 0 && start_time > 0) {
        double gap = difftime(start_time, m->last_segment_end_time);
        if (gap > RECORDING_GAP_THRESHOLD_SEC) {
            atomic_fetch_add(&m->recording_gaps_total, 1);
            log_warn("Recording gap detected for stream '%s': %.0fs gap", stream_name, gap);
        }
    }
    if (end_time > 0) {
        m->last_segment_end_time = end_time;
    }

    atomic_fetch_add(&m->recording_segments_total, 1);
    atomic_fetch_add(&m->recording_bytes_written, bytes);

    pthread_rwlock_unlock(&m->lock);
}

void metrics_set_recording_active(const char *stream_name, bool active) {
    if (!g_initialized || !stream_name) return;
    int idx = metrics_get_slot(stream_name);
    if (idx < 0) return;
    atomic_store(&g_metrics[idx].recording_active, active ? 1 : 0);
}

void metrics_set_connection_latency(const char *stream_name, double latency_ms) {
    if (!g_initialized || !stream_name) return;
    int idx = metrics_get_slot(stream_name);
    if (idx < 0) return;
    pthread_rwlock_wrlock(&g_metrics[idx].lock);
    g_metrics[idx].connection_latency_ms = latency_ms;
    pthread_rwlock_unlock(&g_metrics[idx].lock);
}

void metrics_set_configured_fps(const char *stream_name, double fps) {
    if (!g_initialized || !stream_name) return;
    int idx = metrics_get_slot(stream_name);
    if (idx < 0) return;
    pthread_rwlock_wrlock(&g_metrics[idx].lock);
    g_metrics[idx].configured_fps = fps;
    pthread_rwlock_unlock(&g_metrics[idx].lock);
}

int metrics_snapshot_all(stream_metrics_t *out_array, int max_count) {
    if (!g_initialized || !out_array || max_count <= 0) return 0;

    int count = 0;
    for (int i = 0; i < g_max_streams && count < max_count; i++) {
        stream_metrics_t *m = &g_metrics[i];
        if (!m->active) continue;

        pthread_rwlock_rdlock(&m->lock);
        memcpy(&out_array[count], m, sizeof(stream_metrics_t));
        pthread_rwlock_unlock(&m->lock);

        /* Re-read atomics into the snapshot for consistency */
        out_array[count].health_status    = atomic_load(&m->health_status);
        out_array[count].stream_up        = atomic_load(&m->stream_up);
        out_array[count].last_frame_ts    = atomic_load(&m->last_frame_ts);
        out_array[count].frames_total     = atomic_load(&m->frames_total);
        out_array[count].frames_dropped   = atomic_load(&m->frames_dropped);
        out_array[count].bytes_received   = atomic_load(&m->bytes_received);
        out_array[count].reconnects_total = atomic_load(&m->reconnects_total);
        out_array[count].uptime_seconds   = atomic_load(&m->uptime_seconds);
        out_array[count].error_decode     = atomic_load(&m->error_decode);
        out_array[count].error_timeout    = atomic_load(&m->error_timeout);
        out_array[count].error_protocol   = atomic_load(&m->error_protocol);
        out_array[count].error_io         = atomic_load(&m->error_io);
        out_array[count].recording_active         = atomic_load(&m->recording_active);
        out_array[count].recording_bytes_written  = atomic_load(&m->recording_bytes_written);
        out_array[count].recording_segments_total = atomic_load(&m->recording_segments_total);
        out_array[count].recording_gaps_total     = atomic_load(&m->recording_gaps_total);

        count++;
    }
    return count;
}

int metrics_get_ring_data(const char *stream_name, metrics_ring_sample_t *out, int max_count) {
    if (!g_initialized || !stream_name || !out || max_count <= 0) return 0;

    int idx = find_active_slot(stream_name);
    if (idx < 0) return 0;

    stream_metrics_t *m = &g_metrics[idx];
    pthread_rwlock_rdlock(&m->lock);

    int avail = m->ring_count;
    int to_copy = avail < max_count ? avail : max_count;

    /* Copy oldest-first from the ring buffer */
    int start;
    if (avail < METRICS_RING_SIZE) {
        /* Buffer not yet full: data starts at index 0 */
        start = 0;
    } else {
        /* Buffer full: oldest sample is at ring_head */
        start = m->ring_head;
    }

    /* Only copy the most recent `to_copy` samples */
    int skip = avail - to_copy;
    int read_pos = (start + skip) % METRICS_RING_SIZE;

    for (int i = 0; i < to_copy; i++) {
        out[i] = m->ring[read_pos];
        read_pos = (read_pos + 1) % METRICS_RING_SIZE;
    }

    pthread_rwlock_unlock(&m->lock);
    return to_copy;
}

int metrics_get_max_streams(void) {
    return g_max_streams;
}
