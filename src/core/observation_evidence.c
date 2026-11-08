/*
 * SPDX-License-Identifier: GPL-3.0-or-later
 * Copyright (C) 2026 OpenSensor Engineering and LightNVR contributors
 *
 * This file is part of LightNVR <https://github.com/opensensor/lightNVR>.
 */

/*
 * INT_01 L3: evidence frame queue and encoder worker. See
 * include/core/observation_evidence.h for the producer contract.
 *
 * Admission is lock-free with respect to the worker:
 *   - reservations (items/bytes, global and per camera) are atomic counters;
 *   - accepted items go onto a Treiber stack (CAS push, the single consumer
 *     takes the whole list with one exchange, so there is no ABA);
 *   - drop records go onto a bounded Vyukov MPSC ring preallocated at start;
 *   - the worker is woken with sem_post(), which never blocks.
 * The only mutexes a producer can touch are the slot-registration lock (first
 * submission of a camera) and the lazy-start lock (trylock); the worker never
 * holds either.
 */

#define _GNU_SOURCE

#include "core/observation_evidence.h"

#include <errno.h>
#include <fcntl.h>
#include <math.h>
#include <pthread.h>
#include <semaphore.h>
#include <stdatomic.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/stat.h>
#include <time.h>
#include <unistd.h>

#include "core/config.h"
#include "core/logger.h"
#include "core/shutdown_coordinator.h"
#include "database/db_streams.h"
#include "utils/strings.h"
#include "utils/uuid.h"
#include "video/ffmpeg_utils.h"

#define EVIDENCE_MAX_CAMERAS MAX_STREAMS
#define EVIDENCE_WAIT_MS 200

enum { LC_OFF = 0, LC_READY, LC_RUNNING, LC_STOPPING };

typedef struct evidence_item {
    struct evidence_item *next;
    bool is_jpeg;
    int slot;
    size_t charge;
    uint8_t *data;
    size_t data_size;
    int width, height;
    observation_batch_input_t batch;
    observation_asset_input_t asset;
} evidence_item_t;

typedef struct {
    observation_batch_input_t batch;
    char asset_uuid[OBS_UUID_SIZE];
    char reason[OBS_REASON_SIZE];
    uint8_t *discard;        /* rejected frame the worker frees (may be NULL) */
    size_t discard_charge;   /* bytes reserved against g.discard_bytes */
} drop_record_t;

typedef struct {
    atomic_size_t seq;
    drop_record_t record;
} drop_cell_t;

typedef struct {
    drop_cell_t cells[OBS_EVIDENCE_DROP_RING_SIZE];
    atomic_size_t enqueue_pos;
    size_t dequeue_pos;  /* consumer only */
} drop_ring_t;

_Static_assert((OBS_EVIDENCE_DROP_RING_SIZE & (OBS_EVIDENCE_DROP_RING_SIZE - 1)) == 0,
               "drop ring size must be a power of two");

typedef struct {
    char camera_uuid[OBS_UUID_SIZE];  /* immutable once published */
    atomic_int items;
    atomic_llong bytes;
    atomic_int budget_exhausted;  /* 0/1: atomic_bool breaks the C->Rust conversion */
} camera_slot_t;

/* Per-camera slots are static (no allocation while disabled) and are never
 * recycled: a slot index is valid forever once published via g_slot_count. */
static camera_slot_t g_slots[EVIDENCE_MAX_CAMERAS];
static atomic_int g_slot_count;
static pthread_mutex_t g_slot_lock = PTHREAD_MUTEX_INITIALIZER;

static pthread_mutex_t g_hook_lock = PTHREAD_MUTEX_INITIALIZER;
static observation_evidence_stage_fn g_stage_hook;
static void *g_stage_ctx;
static observation_evidence_enabled_fn g_enabled_probe;
static void *g_enabled_ctx;

static struct {
    atomic_int lifecycle;
    atomic_int inflight;
    char storage_path[MAX_PATH_LENGTH];
    int64_t global_max_bytes;
    int global_max_items;
    int per_camera_max_items;
    int64_t per_camera_max_bytes;

    pthread_mutex_t start_lock;
    pthread_t thread;
    atomic_int thread_started;
    atomic_int worker_running;
    int component_id;

    _Atomic(evidence_item_t *) head;
    _Atomic(drop_ring_t *) ring;
    atomic_int drop_pending;
    sem_t wake;
    atomic_int stop;
    _Atomic int64_t drain_deadline_ns;

    atomic_int items;
    atomic_llong bytes;
    atomic_uint high_water_items;
    atomic_ullong high_water_bytes;

    atomic_uint camera_high_water_items;
    int64_t discard_max_bytes;
    atomic_llong discard_bytes;
    atomic_ullong discard_deferred, discard_inline;
    atomic_ullong submit_latency_count, submit_latency_sum_ns, submit_latency_max_ns;
    atomic_ullong submit_latency_bucket[OBS_EVIDENCE_SUBMIT_LATENCY_BUCKETS];

    atomic_ullong accepted, dropped_queue_full, dropped_budget, dropped_disabled,
        dropped_untracked, unavailable_submitted, ready, failed_encode, failed_write,
        failed_db, disabled_at_encode, interrupted, bytes_written;

    /* Worker-private encoder: keeps the worker independent of the global
     * jpeg_encoder_get_cached() table, whose eviction could destroy an
     * encoder another thread is using. */
    jpeg_encoder_cache_t *encoder;
    int encoder_w, encoder_h;
} g = {.start_lock = PTHREAD_MUTEX_INITIALIZER, .component_id = -1};

/* ---- small helpers ---------------------------------------------------- */

static int64_t mono_ns(void) {
    struct timespec ts;
    clock_gettime(CLOCK_MONOTONIC, &ts);
    return (int64_t)ts.tv_sec * 1000000000LL + ts.tv_nsec;
}

static void bump_max_uint(atomic_uint *target, unsigned value) {
    unsigned current = atomic_load(target);
    while (value > current && !atomic_compare_exchange_weak(target, &current, value)) {}
}

static void bump_max_ull(atomic_ullong *target, unsigned long long value) {
    unsigned long long current = atomic_load(target);
    while (value > current && !atomic_compare_exchange_weak(target, &current, value)) {}
}

const uint64_t observation_evidence_submit_latency_bounds_ns[OBS_EVIDENCE_SUBMIT_LATENCY_BUCKETS - 1] = {
    1000ULL, 5000ULL, 10000ULL, 50000ULL, 100000ULL, 250000ULL, 500000ULL, 1000000ULL, 10000000ULL,
};

/* Lock-free: three relaxed atomic adds and one CAS-max per submit. */
static void record_submit_latency(int64_t start_ns) {
    int64_t elapsed = mono_ns() - start_ns;
    unsigned long long ns = elapsed > 0 ? (unsigned long long)elapsed : 0ULL;
    int bucket = OBS_EVIDENCE_SUBMIT_LATENCY_BUCKETS - 1;
    for (int i = 0; i < OBS_EVIDENCE_SUBMIT_LATENCY_BUCKETS - 1; ++i) {
        if (ns <= observation_evidence_submit_latency_bounds_ns[i]) {
            bucket = i;
            break;
        }
    }
    atomic_fetch_add_explicit(&g.submit_latency_bucket[bucket], 1, memory_order_relaxed);
    atomic_fetch_add_explicit(&g.submit_latency_sum_ns, ns, memory_order_relaxed);
    atomic_fetch_add_explicit(&g.submit_latency_count, 1, memory_order_relaxed);
    bump_max_ull(&g.submit_latency_max_ns, ns);
}

static int stage_hook(observation_evidence_stage_t stage, const char *path) {
    pthread_mutex_lock(&g_hook_lock);
    observation_evidence_stage_fn fn = g_stage_hook;
    void *ctx = g_stage_ctx;
    pthread_mutex_unlock(&g_hook_lock);
    return fn ? fn(stage, path, ctx) : 0;
}

static bool camera_enabled(const char *camera_uuid) {
    pthread_mutex_lock(&g_hook_lock);
    observation_evidence_enabled_fn fn = g_enabled_probe;
    void *ctx = g_enabled_ctx;
    pthread_mutex_unlock(&g_hook_lock);
    return fn ? fn(camera_uuid, ctx) : true;
}

void observation_evidence_set_stage_hook(observation_evidence_stage_fn fn, void *ctx) {
    pthread_mutex_lock(&g_hook_lock);
    g_stage_hook = fn;
    g_stage_ctx = ctx;
    pthread_mutex_unlock(&g_hook_lock);
}

void observation_evidence_set_enabled_probe(observation_evidence_enabled_fn fn, void *ctx) {
    pthread_mutex_lock(&g_hook_lock);
    g_enabled_probe = fn;
    g_enabled_ctx = ctx;
    pthread_mutex_unlock(&g_hook_lock);
}

bool observation_evidence_stream_enabled_probe(const char *camera_uuid, void *ctx) {
    (void)ctx;
    stream_config_t *stream = calloc(1, sizeof(*stream));
    if (!stream) return true;  /* cannot tell: keep the evidence */
    bool enabled = get_stream_config_by_uuid(camera_uuid, stream) == 0 &&
                   stream->observation_evidence_enabled;
    free(stream);
    return enabled;
}

/* ---- camera slots ------------------------------------------------------ */

static int slot_find(const char *camera_uuid) {
    int count = atomic_load_explicit(&g_slot_count, memory_order_acquire);
    for (int i = 0; i < count; ++i)
        if (strcmp(g_slots[i].camera_uuid, camera_uuid) == 0) return i;
    return -1;
}

/* Returns the slot, creating it when absent; -1 when the table is full (the
 * camera is then bounded by the global limits only). */
static int slot_get(const char *camera_uuid) {
    int slot = slot_find(camera_uuid);
    if (slot >= 0) return slot;
    pthread_mutex_lock(&g_slot_lock);
    slot = slot_find(camera_uuid);
    if (slot < 0) {
        int count = atomic_load_explicit(&g_slot_count, memory_order_relaxed);
        if (count < EVIDENCE_MAX_CAMERAS) {
            camera_slot_t *s = &g_slots[count];
            safe_strcpy(s->camera_uuid, camera_uuid, sizeof(s->camera_uuid), 0);
            atomic_store(&s->items, 0);
            atomic_store(&s->bytes, 0);
            atomic_store(&s->budget_exhausted, 0);
            atomic_store_explicit(&g_slot_count, count + 1, memory_order_release);
            slot = count;
        }
    }
    pthread_mutex_unlock(&g_slot_lock);
    return slot;
}

void observation_evidence_set_budget_exhausted(const char *camera_uuid, bool exhausted) {
    if (!lightnvr_uuid_is_valid(camera_uuid)) return;
    int slot = exhausted ? slot_get(camera_uuid) : slot_find(camera_uuid);
    if (slot < 0) {
        if (exhausted) log_warn("Evidence: camera table full; budget flag for %s not stored",
                                camera_uuid);
        return;
    }
    bool previous = atomic_exchange(&g_slots[slot].budget_exhausted, exhausted ? 1 : 0) != 0;
    if (previous != exhausted)
        log_info("Evidence: budget_exhausted %s for camera %s", exhausted ? "set" : "cleared",
                 camera_uuid);
}

bool observation_evidence_is_budget_exhausted(const char *camera_uuid) {
    if (!camera_uuid) return false;
    int slot = slot_find(camera_uuid);
    return slot >= 0 && atomic_load(&g_slots[slot].budget_exhausted);
}

/* ---- formatting --------------------------------------------------------- */

static bool format_number(double value, char *out, size_t size) {
    if (!isfinite(value)) return false;
    int n = snprintf(out, size, "%.6g", value);
    return n > 0 && (size_t)n < size;
}

int observation_evidence_transform_json(const observation_evidence_transform_t *transform,
                                        int width, int height, char *out, size_t out_size) {
    if (!out || out_size == 0) return -1;
    out[0] = '\0';
    observation_evidence_transform_t t;
    if (transform) {
        t = *transform;
    } else {
        if (width <= 0 || height <= 0) return -1;
        memset(&t, 0, sizeof(t));
        t.mode = OBS_TRANSFORM_NONE;
        t.source_width = t.inference_width = width;
        t.source_height = t.inference_height = height;
        t.scale_x = t.scale_y = 1.0;
    }
    static const char *const modes[] = {"none", "resize", "letterbox"};
    if ((int)t.mode < 0 || (int)t.mode > OBS_TRANSFORM_LETTERBOX) return -1;
    if (t.source_width <= 0 || t.source_height <= 0 || t.inference_width <= 0 ||
        t.inference_height <= 0 || !(t.scale_x > 0) || !(t.scale_y > 0) ||
        !(t.pad_x >= 0) || !(t.pad_y >= 0))
        return -1;
    char sx[32], sy[32], px[32], py[32];
    if (!format_number(t.scale_x, sx, sizeof(sx)) || !format_number(t.scale_y, sy, sizeof(sy)) ||
        !format_number(t.pad_x, px, sizeof(px)) || !format_number(t.pad_y, py, sizeof(py)))
        return -1;
    int n = snprintf(out, out_size,
                     "{\"mode\":\"%s\",\"source_width\":%d,\"source_height\":%d,"
                     "\"inference_width\":%d,\"inference_height\":%d,"
                     "\"scale_x\":%s,\"scale_y\":%s,\"pad_x\":%s,\"pad_y\":%s}",
                     modes[t.mode], t.source_width, t.source_height, t.inference_width,
                     t.inference_height, sx, sy, px, py);
    if (n < 0 || (size_t)n >= out_size) {
        out[0] = '\0';
        return -1;
    }
    return 0;
}

int observation_evidence_relative_path(const char *camera_uuid, const char *asset_uuid,
                                       int64_t observed_at_ms, char *out, size_t out_size) {
    if (!out || out_size == 0) return -1;
    out[0] = '\0';
    /* UUID validation also guarantees no path separators or dot segments. */
    if (!lightnvr_uuid_is_valid(camera_uuid) || !lightnvr_uuid_is_valid(asset_uuid) ||
        observed_at_ms < 0)
        return -1;
    time_t seconds = (time_t)(observed_at_ms / 1000);
    struct tm tm;
    if (!gmtime_r(&seconds, &tm)) return -1;
    int n = snprintf(out, out_size, "%s/%s/%04d-%02d-%02d/%s.jpg", OBS_EVIDENCE_DIR,
                     camera_uuid, tm.tm_year + 1900, tm.tm_mon + 1, tm.tm_mday, asset_uuid);
    if (n < 0 || (size_t)n >= out_size) {
        out[0] = '\0';
        return -1;
    }
    return 0;
}

/* ---- drop ring (Vyukov bounded MPSC) ------------------------------------ */

static drop_ring_t *ring_create(void) {
    drop_ring_t *ring = calloc(1, sizeof(*ring));
    if (!ring) return NULL;
    for (size_t i = 0; i < OBS_EVIDENCE_DROP_RING_SIZE; ++i)
        atomic_store_explicit(&ring->cells[i].seq, i, memory_order_relaxed);
    atomic_store(&ring->enqueue_pos, 0);
    return ring;
}

static bool ring_push(drop_ring_t *ring, const observation_batch_input_t *batch,
                      const char *asset_uuid, const char *reason, uint8_t *discard,
                      size_t discard_charge) {
    if (!ring) return false;
    size_t pos = atomic_load_explicit(&ring->enqueue_pos, memory_order_relaxed);
    drop_cell_t *cell;
    for (;;) {
        cell = &ring->cells[pos & (OBS_EVIDENCE_DROP_RING_SIZE - 1)];
        size_t seq = atomic_load_explicit(&cell->seq, memory_order_acquire);
        intptr_t diff = (intptr_t)seq - (intptr_t)pos;
        if (diff == 0) {
            if (atomic_compare_exchange_weak_explicit(&ring->enqueue_pos, &pos, pos + 1,
                                                      memory_order_relaxed,
                                                      memory_order_relaxed))
                break;
        } else if (diff < 0) {
            return false;  /* full */
        } else {
            pos = atomic_load_explicit(&ring->enqueue_pos, memory_order_relaxed);
        }
    }
    cell->record.batch = *batch;
    safe_strcpy(cell->record.asset_uuid, asset_uuid, sizeof(cell->record.asset_uuid), 0);
    safe_strcpy(cell->record.reason, reason, sizeof(cell->record.reason), 0);
    cell->record.discard = discard;
    cell->record.discard_charge = discard_charge;
    atomic_fetch_add(&g.drop_pending, 1);
    atomic_store_explicit(&cell->seq, pos + 1, memory_order_release);
    return true;
}

static bool ring_pop(drop_ring_t *ring, drop_record_t *out) {
    if (!ring) return false;
    drop_cell_t *cell = &ring->cells[ring->dequeue_pos & (OBS_EVIDENCE_DROP_RING_SIZE - 1)];
    size_t seq = atomic_load_explicit(&cell->seq, memory_order_acquire);
    if ((intptr_t)seq - (intptr_t)(ring->dequeue_pos + 1) < 0) return false;
    *out = cell->record;
    atomic_store_explicit(&cell->seq, ring->dequeue_pos + OBS_EVIDENCE_DROP_RING_SIZE,
                          memory_order_release);
    ring->dequeue_pos++;
    return true;
}

/* ---- worker ------------------------------------------------------------- */

static void release_item(evidence_item_t *item) {
    if (item->slot >= 0) {
        atomic_fetch_sub(&g_slots[item->slot].items, 1);
        atomic_fetch_sub(&g_slots[item->slot].bytes, (long long)item->charge);
    }
    atomic_fetch_sub(&g.items, 1);
    atomic_fetch_sub(&g.bytes, (long long)item->charge);
    free(item->data);
    free(item);
}

static void insert_unavailable(const observation_batch_input_t *batch, const char *asset_uuid,
                               const char *reason) {
    observation_asset_input_t asset;
    memset(&asset, 0, sizeof(asset));
    safe_strcpy(asset.asset_uuid, asset_uuid, sizeof(asset.asset_uuid), 0);
    asset.state = OBS_ASSET_UNAVAILABLE;
    safe_strcpy(asset.reason, reason, sizeof(asset.reason), 0);
    observation_insert_result_t result;
    int rc = db_observation_insert_batch(batch, &asset, &result);
    if (rc != DB_OBS_OK && rc != DB_OBS_DUPLICATE) {
        atomic_fetch_add(&g.failed_db, 1);
        log_warn("Evidence: recording batch %s as unavailable(%s) failed: %d", batch->batch_id,
                 reason, rc);
    }
}

static void process_drops(void) {
    drop_ring_t *ring = atomic_load(&g.ring);
    drop_record_t *record = NULL;
    while (ring) {
        if (!record) {
            record = malloc(sizeof(*record));
            if (!record) return;
        }
        if (!ring_pop(ring, record)) break;
        if (record->discard) {
            free(record->discard);
            atomic_fetch_sub(&g.discard_bytes, (long long)record->discard_charge);
        }
        insert_unavailable(&record->batch, record->asset_uuid, record->reason);
        atomic_fetch_sub(&g.drop_pending, 1);
    }
    free(record);
}

static void transition_failed(evidence_item_t *item, const char *reason) {
    observation_transition_result_t result;
    int rc = db_observation_asset_transition(item->asset.asset_uuid, OBS_ASSET_PENDING,
                                             OBS_ASSET_UNAVAILABLE, reason, NULL, &result);
    if (rc != DB_OBS_OK && rc != DB_OBS_CAS_LOST) {
        atomic_fetch_add(&g.failed_db, 1);
        log_warn("Evidence: asset %s pending -> unavailable(%s) failed: %d",
                 item->asset.asset_uuid, reason, rc);
    }
}

static int mkdir_parents(const char *path) {
    char buf[MAX_PATH_LENGTH];
    if (safe_strcpy(buf, path, sizeof(buf), 0) != 0) return -1;
    for (char *p = buf + 1; *p; ++p) {
        if (*p != '/') continue;
        *p = '\0';
        if (mkdir(buf, 0750) != 0 && errno != EEXIST) return -1;
        *p = '/';
    }
    if (mkdir(buf, 0750) != 0 && errno != EEXIST) return -1;
    return 0;
}

static int fsync_dir(const char *dir) {
    int fd = open(dir, O_RDONLY | O_DIRECTORY | O_CLOEXEC);
    if (fd < 0) return -1;
    int rc = fsync(fd);
    close(fd);
    return rc;
}

/* Write data to final_path atomically: <final>.tmp, fsync, rename, fsync dir.
 * On failure nothing is left under either name. */
static int write_atomic(const char *dir, const char *final_path, const uint8_t *data,
                        size_t size) {
    char tmp_path[MAX_PATH_LENGTH + 8];
    int n = snprintf(tmp_path, sizeof(tmp_path), "%s.tmp", final_path);
    if (n < 0 || (size_t)n >= sizeof(tmp_path)) return -1;
    if (mkdir_parents(dir) != 0) {
        log_warn("Evidence: cannot create %s: %s", dir, strerror(errno));
        return -1;
    }
    int fd = open(tmp_path, O_WRONLY | O_CREAT | O_TRUNC | O_CLOEXEC | O_NOFOLLOW, 0640);
    if (fd < 0) {
        log_warn("Evidence: cannot create %s: %s", tmp_path, strerror(errno));
        return -1;
    }
    size_t written = 0;
    while (written < size) {
        ssize_t w = write(fd, data + written, size - written);
        if (w < 0 && errno == EINTR) continue;
        if (w <= 0) break;
        written += (size_t)w;
    }
    bool ok = written == size && fsync(fd) == 0;
    if (close(fd) != 0) ok = false;
    if (ok && stage_hook(OBS_EVIDENCE_STAGE_BEFORE_RENAME, final_path) != 0) ok = false;
    if (!ok) {
        unlink(tmp_path);
        return -1;
    }
    if (rename(tmp_path, final_path) != 0) {
        log_warn("Evidence: rename %s failed: %s", tmp_path, strerror(errno));
        unlink(tmp_path);
        return -1;
    }
    if (fsync_dir(dir) != 0) {
        log_warn("Evidence: fsync of %s failed: %s", dir, strerror(errno));
        unlink(final_path);
        return -1;
    }
    return 0;
}

static int encode_rgb(evidence_item_t *item, unsigned char **jpeg, size_t *jpeg_size) {
    if (!g.encoder || g.encoder_w != item->width || g.encoder_h != item->height) {
        if (g.encoder) jpeg_encoder_cache_destroy(g.encoder);
        g.encoder = jpeg_encoder_cache_create(item->width, item->height, 3,
                                              OBS_EVIDENCE_JPEG_QUALITY);
        g.encoder_w = g.encoder ? item->width : 0;
        g.encoder_h = g.encoder ? item->height : 0;
        if (!g.encoder) return -1;
    }
    return jpeg_encoder_cache_encode_to_memory(g.encoder, item->data, jpeg, jpeg_size);
}

static void clear_media_fields(observation_asset_input_t *asset) {
    asset->origin[0] = '\0';
    asset->media_type[0] = '\0';
    asset->transform_json[0] = '\0';
}

static void abandon_item(evidence_item_t *item) {
    clear_media_fields(&item->asset);
    insert_unavailable(&item->batch, item->asset.asset_uuid, "interrupted");
    atomic_fetch_add(&g.interrupted, 1);
}

static void process_item(evidence_item_t *item) {
    const char *camera = item->batch.camera_uuid;
    if (!camera_enabled(camera)) {
        clear_media_fields(&item->asset);
        insert_unavailable(&item->batch, item->asset.asset_uuid, "disabled");
        atomic_fetch_add(&g.disabled_at_encode, 1);
        return;
    }

    observation_insert_result_t inserted;
    int rc = db_observation_insert_batch(&item->batch, &item->asset, &inserted);
    if (rc == DB_OBS_DUPLICATE) {
        log_warn("Evidence: batch %s already recorded; dropping its frame", item->batch.batch_id);
        return;
    }
    if (rc != DB_OBS_OK) {
        atomic_fetch_add(&g.failed_db, 1);
        log_warn("Evidence: inserting batch %s failed: %d", item->batch.batch_id, rc);
        return;
    }
    (void)stage_hook(OBS_EVIDENCE_STAGE_AFTER_INSERT, NULL);

    char relative[MAX_PATH_LENGTH];
    char final_path[MAX_PATH_LENGTH];
    char dir[MAX_PATH_LENGTH];
    if (observation_evidence_relative_path(camera, item->asset.asset_uuid,
                                           item->batch.observed_at_ms, relative,
                                           sizeof(relative)) != 0 ||
        snprintf(final_path, sizeof(final_path), "%s/%s", g.storage_path, relative) >=
            (int)sizeof(final_path)) {
        atomic_fetch_add(&g.failed_write, 1);
        transition_failed(item, "write_failed");
        return;
    }
    safe_strcpy(dir, final_path, sizeof(dir), 0);
    char *slash = strrchr(dir, '/');
    if (slash) *slash = '\0';

    unsigned char *jpeg = NULL;
    size_t jpeg_size = 0;
    if (item->is_jpeg) {
        jpeg = item->data;
        jpeg_size = item->data_size;
    } else if (encode_rgb(item, &jpeg, &jpeg_size) != 0 || !jpeg || jpeg_size == 0) {
        free(jpeg);
        atomic_fetch_add(&g.failed_encode, 1);
        transition_failed(item, "encode_failed");
        return;
    } else {
        /* The RGB frame is no longer needed; release its memory early. */
        free(item->data);
        item->data = NULL;
    }

    int write_rc = write_atomic(dir, final_path, jpeg, jpeg_size);
    if (!item->is_jpeg) free(jpeg);
    if (write_rc != 0) {
        atomic_fetch_add(&g.failed_write, 1);
        transition_failed(item, "write_failed");
        return;
    }

    observation_asset_media_t media = {
        .path = relative, .bytes = (int64_t)jpeg_size,
        .width = item->width, .height = item->height, .expires_at_ms = 0,
    };
    observation_transition_result_t result;
    (void)stage_hook(OBS_EVIDENCE_STAGE_BEFORE_COMMIT, final_path);
    rc = db_observation_asset_transition(item->asset.asset_uuid, OBS_ASSET_PENDING,
                                         OBS_ASSET_READY, NULL, &media, &result);
    if (rc == DB_OBS_ERROR)
        rc = db_observation_asset_transition(item->asset.asset_uuid, OBS_ASSET_PENDING,
                                             OBS_ASSET_READY, NULL, &media, &result);
    if (rc != DB_OBS_OK) {
        /* CAS lost (someone else resolved the asset) or a DB failure: never
         * leave a final file behind a non-ready asset. A still-pending asset
         * without a file becomes unavailable(interrupted) at L4 startup. */
        unlink(final_path);
        fsync_dir(dir);
        if (rc != DB_OBS_CAS_LOST) {
            atomic_fetch_add(&g.failed_db, 1);
            transition_failed(item, "write_failed");
        }
        log_warn("Evidence: asset %s pending -> ready failed: %d", item->asset.asset_uuid, rc);
        return;
    }
    atomic_fetch_add(&g.ready, 1);
    atomic_fetch_add(&g.bytes_written, (unsigned long long)jpeg_size);
}

static evidence_item_t *take_all_fifo(void) {
    evidence_item_t *list = atomic_exchange(&g.head, NULL);
    evidence_item_t *fifo = NULL;
    while (list) {
        evidence_item_t *next = list->next;
        list->next = fifo;
        fifo = list;
        list = next;
    }
    return fifo;
}

static bool drain_expired(void) {
    return atomic_load(&g.stop) && mono_ns() >= atomic_load(&g.drain_deadline_ns);
}

static void run_items(evidence_item_t *list) {
    while (list) {
        evidence_item_t *item = list;
        list = list->next;
        stage_hook(OBS_EVIDENCE_STAGE_ITEM_BEGIN, NULL);
        if (drain_expired()) abandon_item(item);
        else process_item(item);
        stage_hook(OBS_EVIDENCE_STAGE_ITEM_DONE, NULL);
        release_item(item);
        process_drops();
    }
}

static void *evidence_worker(void *arg) {
    (void)arg;
    g.component_id = register_component("observation_evidence", COMPONENT_OTHER, NULL, 20);
    atomic_store(&g.worker_running, 1);
    log_info("Observation evidence worker started");
    for (;;) {
        process_drops();
        evidence_item_t *list = take_all_fifo();
        if (list) {
            run_items(list);
            continue;
        }
        if (atomic_load(&g.stop)) break;
        struct timespec deadline;
        clock_gettime(CLOCK_REALTIME, &deadline);
        deadline.tv_nsec += (long)EVIDENCE_WAIT_MS * 1000000L;
        if (deadline.tv_nsec >= 1000000000L) {
            deadline.tv_sec += 1;
            deadline.tv_nsec -= 1000000000L;
        }
        while (sem_timedwait(&g.wake, &deadline) != 0 && errno == EINTR) {}
    }
    if (g.encoder) {
        jpeg_encoder_cache_destroy(g.encoder);
        g.encoder = NULL;
        g.encoder_w = g.encoder_h = 0;
    }
    atomic_store(&g.worker_running, 0);
    if (g.component_id >= 0) update_component_state(g.component_id, COMPONENT_STOPPED);
    log_info("Observation evidence worker stopped");
    return NULL;
}

/* Lazy start on the first submission. trylock: a producer that loses the
 * race simply queues; the item is picked up once the winner starts. */
static void ensure_started(void) {
    if (atomic_load(&g.lifecycle) == LC_RUNNING) return;
    if (pthread_mutex_trylock(&g.start_lock) != 0) return;
    if (atomic_load(&g.lifecycle) == LC_READY && !atomic_load(&g.thread_started)) {
        drop_ring_t *ring = ring_create();
        if (ring) {
            atomic_store(&g.ring, ring);
            atomic_store(&g.stop, 0);
            if (pthread_create(&g.thread, NULL, evidence_worker, NULL) == 0) {
                atomic_store(&g.thread_started, 1);
                int expected = LC_READY;
                atomic_compare_exchange_strong(&g.lifecycle, &expected, LC_RUNNING);
            } else {
                log_error("Evidence: cannot start worker thread");
                atomic_store(&g.ring, NULL);
                free(ring);
            }
        }
    }
    pthread_mutex_unlock(&g.start_lock);
}

/* ---- admission ---------------------------------------------------------- */

static bool enter(void) {
    atomic_fetch_add(&g.inflight, 1);
    int lc = atomic_load(&g.lifecycle);
    if (lc == LC_READY || lc == LC_RUNNING) return true;
    atomic_fetch_sub(&g.inflight, 1);
    return false;
}

static void leave(void) { atomic_fetch_sub(&g.inflight, 1); }

static bool reserve_counter(atomic_int *counter, int limit) {
    if (atomic_fetch_add(counter, 1) + 1 > limit) {
        atomic_fetch_sub(counter, 1);
        return false;
    }
    return true;
}

static bool reserve_bytes(atomic_llong *counter, long long charge, long long limit) {
    if (atomic_fetch_add(counter, charge) + charge > limit) {
        atomic_fetch_sub(counter, charge);
        return false;
    }
    return true;
}

static bool reserve(int slot, size_t charge) {
    long long c = (long long)charge;
    if (!reserve_counter(&g.items, g.global_max_items)) return false;
    if (!reserve_bytes(&g.bytes, c, g.global_max_bytes)) goto undo_items;
    if (slot >= 0) {
        if (!reserve_counter(&g_slots[slot].items, g.per_camera_max_items)) goto undo_bytes;
        if (!reserve_bytes(&g_slots[slot].bytes, c, g.per_camera_max_bytes)) {
            atomic_fetch_sub(&g_slots[slot].items, 1);
            goto undo_bytes;
        }
        bump_max_uint(&g.camera_high_water_items, (unsigned)atomic_load(&g_slots[slot].items));
    }
    return true;
undo_bytes:
    atomic_fetch_sub(&g.bytes, c);
undo_items:
    atomic_fetch_sub(&g.items, 1);
    return false;
}

static void wake_worker(void) {
    if (atomic_load(&g.thread_started)) sem_post(&g.wake);
}

/* Push a drop record; returns true when tracked. Takes ownership of discard
 * (the rejected frame, may be NULL): releasing a multi-megabyte buffer is an
 * munmap that measured 1.5-2.6 ms on a busy process (X2), so the worker frees
 * it after popping the record. Pending deferred frames are bounded by
 * g.discard_max_bytes; past that, or with the ring full, it is freed here. */
static bool record_drop(const observation_batch_input_t *batch, const char *asset_uuid,
                        const char *reason, uint8_t *discard, size_t discard_size) {
    size_t charge = 0;
    if (discard) {
        if (reserve_bytes(&g.discard_bytes, (long long)discard_size, g.discard_max_bytes)) {
            charge = discard_size;
        } else {
            free(discard);
            discard = NULL;
            atomic_fetch_add(&g.discard_inline, 1);
        }
    }
    if (ring_push(atomic_load(&g.ring), batch, asset_uuid, reason, discard, charge)) {
        if (discard) atomic_fetch_add(&g.discard_deferred, 1);
        wake_worker();
        return true;
    }
    if (discard) {
        atomic_fetch_sub(&g.discard_bytes, (long long)charge);
        free(discard);
        atomic_fetch_add(&g.discard_inline, 1);
    }
    atomic_fetch_add(&g.dropped_untracked, 1);
    return false;
}

static bool batch_valid(const observation_batch_input_t *batch) {
    return batch && lightnvr_uuid_is_valid(batch->camera_uuid) &&
           lightnvr_uuid_is_valid(batch->batch_id) && batch->object_count >= 1 &&
           batch->object_count <= OBS_MAX_BATCH && batch->observed_at_ms >= 0;
}

static observation_evidence_result_t admit(const observation_batch_input_t *batch,
                                           uint8_t *data, size_t data_size, int width,
                                           int height, bool is_jpeg,
                                           const observation_evidence_transform_t *transform,
                                           char asset_uuid_out[OBS_UUID_SIZE]) {
    if (asset_uuid_out) asset_uuid_out[0] = '\0';
    if (!enter()) {
        free(data);
        atomic_fetch_add(&g.dropped_disabled, 1);
        return OBS_EVIDENCE_DISABLED;
    }
    observation_evidence_result_t result = OBS_EVIDENCE_INVALID;
    char transform_json[OBS_TRANSFORM_SIZE];
    char asset_uuid[OBS_UUID_SIZE];
    if (!batch_valid(batch) || !data || data_size == 0 || width <= 0 || height <= 0 ||
        observation_evidence_transform_json(transform, width, height, transform_json,
                                            sizeof(transform_json)) != 0 ||
        lightnvr_uuid_generate_v4(asset_uuid) != 0) {
        free(data);
        leave();
        return OBS_EVIDENCE_INVALID;
    }

    ensure_started();
    int slot = slot_get(batch->camera_uuid);
    size_t charge = data_size + (is_jpeg ? 0 : RGB24_SWS_TAIL_PADDING) + sizeof(evidence_item_t);
    evidence_item_t *item = NULL;
    if (slot >= 0 && atomic_load(&g_slots[slot].budget_exhausted)) {
        result = OBS_EVIDENCE_DROPPED_BUDGET;
    } else if (!reserve(slot, charge)) {
        result = OBS_EVIDENCE_DROPPED_QUEUE_FULL;
    } else if (!(item = malloc(sizeof(*item)))) {
        if (slot >= 0) {
            atomic_fetch_sub(&g_slots[slot].items, 1);
            atomic_fetch_sub(&g_slots[slot].bytes, (long long)charge);
        }
        atomic_fetch_sub(&g.items, 1);
        atomic_fetch_sub(&g.bytes, (long long)charge);
        result = OBS_EVIDENCE_DROPPED_QUEUE_FULL;
    } else {
        result = OBS_EVIDENCE_ACCEPTED;
    }

    if (result != OBS_EVIDENCE_ACCEPTED) {
        bool budget = result == OBS_EVIDENCE_DROPPED_BUDGET;
        atomic_fetch_add(budget ? &g.dropped_budget : &g.dropped_queue_full, 1);
        if (record_drop(batch, asset_uuid, budget ? "budget_exhausted" : "queue_full", data,
                        data_size) &&
            asset_uuid_out)
            safe_strcpy(asset_uuid_out, asset_uuid, OBS_UUID_SIZE, 0);
        leave();
        return result;
    }

    memset(item, 0, offsetof(evidence_item_t, batch));
    item->is_jpeg = is_jpeg;
    item->slot = slot;
    item->charge = charge;
    item->data = data;
    item->data_size = data_size;
    item->width = width;
    item->height = height;
    item->batch = *batch;
    memset(&item->asset, 0, sizeof(item->asset));
    safe_strcpy(item->asset.asset_uuid, asset_uuid, sizeof(item->asset.asset_uuid), 0);
    item->asset.state = OBS_ASSET_PENDING;
    safe_strcpy(item->asset.origin, is_jpeg ? "submitted_image" : "inference_input",
                sizeof(item->asset.origin), 0);
    safe_strcpy(item->asset.media_type, "image/jpeg", sizeof(item->asset.media_type), 0);
    item->asset.captured_at_ms = batch->observed_at_ms;
    safe_strcpy(item->asset.transform_json, transform_json, sizeof(item->asset.transform_json), 0);

    evidence_item_t *head = atomic_load(&g.head);
    do {
        item->next = head;
    } while (!atomic_compare_exchange_weak(&g.head, &head, item));

    bump_max_uint(&g.high_water_items, (unsigned)atomic_load(&g.items));
    bump_max_ull(&g.high_water_bytes, (unsigned long long)atomic_load(&g.bytes));
    atomic_fetch_add(&g.accepted, 1);
    if (asset_uuid_out) safe_strcpy(asset_uuid_out, asset_uuid, OBS_UUID_SIZE, 0);
    wake_worker();
    leave();
    return OBS_EVIDENCE_ACCEPTED;
}

observation_evidence_result_t observation_evidence_submit(
    const observation_batch_input_t *batch, uint8_t *rgb24, int width, int height,
    const observation_evidence_transform_t *transform, char asset_uuid_out[OBS_UUID_SIZE]) {
    int64_t start = mono_ns();
    size_t size = (width > 0 && height > 0) ? (size_t)width * (size_t)height * 3 : 0;
    observation_evidence_result_t rc =
        admit(batch, rgb24, size, width, height, false, transform, asset_uuid_out);
    if (rc != OBS_EVIDENCE_DISABLED) record_submit_latency(start);
    return rc;
}

observation_evidence_result_t observation_evidence_submit_jpeg(
    const observation_batch_input_t *batch, uint8_t *jpeg, size_t jpeg_size, int width,
    int height, const observation_evidence_transform_t *transform,
    char asset_uuid_out[OBS_UUID_SIZE]) {
    int64_t start = mono_ns();
    observation_evidence_result_t rc =
        admit(batch, jpeg, jpeg_size, width, height, true, transform, asset_uuid_out);
    if (rc != OBS_EVIDENCE_DISABLED) record_submit_latency(start);
    return rc;
}

observation_evidence_result_t observation_evidence_submit_unavailable(
    const observation_batch_input_t *batch, const char *reason,
    char asset_uuid_out[OBS_UUID_SIZE]) {
    int64_t start = mono_ns();
    if (asset_uuid_out) asset_uuid_out[0] = '\0';
    if (!enter()) {
        atomic_fetch_add(&g.dropped_disabled, 1);
        return OBS_EVIDENCE_DISABLED;
    }
    char asset_uuid[OBS_UUID_SIZE];
    /* Only the insert-only "no media" reason is a producer's to choose;
     * queue_full / budget_exhausted are decided by admission. */
    if (!batch_valid(batch) || !reason || strcmp(reason, "no_matching_media") != 0 ||
        lightnvr_uuid_generate_v4(asset_uuid) != 0) {
        leave();
        record_submit_latency(start);
        return OBS_EVIDENCE_INVALID;
    }
    ensure_started();
    observation_evidence_result_t result = OBS_EVIDENCE_DROPPED_QUEUE_FULL;
    if (record_drop(batch, asset_uuid, reason, NULL, 0)) {
        atomic_fetch_add(&g.unavailable_submitted, 1);
        if (asset_uuid_out) safe_strcpy(asset_uuid_out, asset_uuid, OBS_UUID_SIZE, 0);
        result = OBS_EVIDENCE_ACCEPTED;
    }
    leave();
    record_submit_latency(start);
    return result;
}

/* ---- lifecycle ---------------------------------------------------------- */

static void reset_counters(void) {
    atomic_ullong *counters[] = {
        &g.accepted, &g.dropped_queue_full, &g.dropped_budget, &g.dropped_disabled,
        &g.dropped_untracked, &g.unavailable_submitted, &g.ready, &g.failed_encode,
        &g.failed_write, &g.failed_db, &g.disabled_at_encode, &g.interrupted,
        &g.bytes_written, &g.high_water_bytes, &g.submit_latency_count,
        &g.submit_latency_sum_ns, &g.submit_latency_max_ns, &g.discard_deferred,
        &g.discard_inline,
    };
    for (size_t i = 0; i < sizeof(counters) / sizeof(counters[0]); ++i)
        atomic_store(counters[i], 0);
    for (int i = 0; i < OBS_EVIDENCE_SUBMIT_LATENCY_BUCKETS; ++i)
        atomic_store(&g.submit_latency_bucket[i], 0);
    atomic_store(&g.high_water_items, 0);
    atomic_store(&g.camera_high_water_items, 0);
}

int observation_evidence_init(const observation_evidence_config_t *config) {
    if (!config || !config->storage_path || !config->storage_path[0] ||
        config->global_max_bytes < 0 || config->global_max_items < 0 ||
        config->per_camera_max_items < 0 || config->per_camera_max_bytes < 0)
        return -1;
    if (strlen(config->storage_path) + 128 >= sizeof(g.storage_path)) return -1;
    pthread_mutex_lock(&g.start_lock);
    if (atomic_load(&g.lifecycle) != LC_OFF) {
        pthread_mutex_unlock(&g.start_lock);
        return -1;
    }
    safe_strcpy(g.storage_path, config->storage_path, sizeof(g.storage_path), 0);
    size_t len = strlen(g.storage_path);
    while (len > 1 && g.storage_path[len - 1] == '/') g.storage_path[--len] = '\0';
    g.global_max_bytes = config->global_max_bytes > 0 ? config->global_max_bytes
                                                      : OBS_EVIDENCE_DEFAULT_GLOBAL_BYTES;
    g.global_max_items = config->global_max_items > 0 ? config->global_max_items
                                                      : OBS_EVIDENCE_GLOBAL_DEPTH;
    g.per_camera_max_items = config->per_camera_max_items > 0
                                 ? config->per_camera_max_items
                                 : OBS_EVIDENCE_PER_CAMERA_DEPTH;
    g.per_camera_max_bytes = config->per_camera_max_bytes > 0 ? config->per_camera_max_bytes
                                                              : g.global_max_bytes;
    g.discard_max_bytes = g.global_max_bytes > OBS_EVIDENCE_DEFAULT_GLOBAL_BYTES
                              ? g.global_max_bytes
                              : OBS_EVIDENCE_DEFAULT_GLOBAL_BYTES;
    atomic_store(&g.discard_bytes, 0);
    if (sem_init(&g.wake, 0, 0) != 0) {
        pthread_mutex_unlock(&g.start_lock);
        return -1;
    }
    reset_counters();
    atomic_store(&g.items, 0);
    atomic_store(&g.bytes, 0);
    atomic_store(&g.drop_pending, 0);
    atomic_store(&g.stop, 0);
    atomic_store(&g.lifecycle, LC_READY);
    pthread_mutex_unlock(&g.start_lock);
    log_info("Observation evidence initialized (worker starts on first submission; "
             "queue %d items / %lld bytes, %d per camera)",
             g.global_max_items, (long long)g.global_max_bytes, g.per_camera_max_items);
    return 0;
}

void observation_evidence_shutdown(int drain_timeout_ms) {
    int lc = atomic_load(&g.lifecycle);
    if ((lc != LC_READY && lc != LC_RUNNING) ||
        !atomic_compare_exchange_strong(&g.lifecycle, &lc, LC_STOPPING))
        return;

    /* Producers never block, so in-flight submissions finish promptly. */
    for (int i = 0; atomic_load(&g.inflight) > 0 && i < 50000; ++i) usleep(100);

    if (drain_timeout_ms < 0) drain_timeout_ms = OBS_EVIDENCE_DEFAULT_DRAIN_MS;
    atomic_store(&g.drain_deadline_ns, mono_ns() + (int64_t)drain_timeout_ms * 1000000LL);
    atomic_store(&g.stop, 1);

    pthread_mutex_lock(&g.start_lock);
    if (atomic_load(&g.thread_started)) {
        sem_post(&g.wake);
        pthread_join(g.thread, NULL);
        atomic_store(&g.thread_started, 0);
    }
    pthread_mutex_unlock(&g.start_lock);

    /* Leftovers when the worker never started (or failed to start). */
    evidence_item_t *list = take_all_fifo();
    while (list) {
        evidence_item_t *item = list;
        list = list->next;
        abandon_item(item);
        release_item(item);
    }
    process_drops();
    if (g.encoder) {
        jpeg_encoder_cache_destroy(g.encoder);
        g.encoder = NULL;
    }
    free(atomic_exchange(&g.ring, NULL));
    atomic_store(&g.drop_pending, 0);
    sem_destroy(&g.wake);
    g.component_id = -1;
    atomic_store(&g.lifecycle, LC_OFF);
    log_info("Observation evidence shut down (ready=%llu interrupted=%llu dropped_untracked=%llu)",
             (unsigned long long)atomic_load(&g.ready),
             (unsigned long long)atomic_load(&g.interrupted),
             (unsigned long long)atomic_load(&g.dropped_untracked));
}

void observation_evidence_get_stats(observation_evidence_stats_t *out) {
    if (!out) return;
    memset(out, 0, sizeof(*out));
    int lc = atomic_load(&g.lifecycle);
    out->initialized = lc == LC_READY || lc == LC_RUNNING;
    out->worker_running = atomic_load(&g.worker_running);
    out->allocated = atomic_load(&g.ring) != NULL || atomic_load(&g.thread_started);
    out->accepted = atomic_load(&g.accepted);
    out->dropped_queue_full = atomic_load(&g.dropped_queue_full);
    out->dropped_budget = atomic_load(&g.dropped_budget);
    out->dropped_disabled = atomic_load(&g.dropped_disabled);
    out->dropped_untracked = atomic_load(&g.dropped_untracked);
    out->unavailable_submitted = atomic_load(&g.unavailable_submitted);
    out->ready = atomic_load(&g.ready);
    out->failed_encode = atomic_load(&g.failed_encode);
    out->failed_write = atomic_load(&g.failed_write);
    out->failed_db = atomic_load(&g.failed_db);
    out->disabled_at_encode = atomic_load(&g.disabled_at_encode);
    out->interrupted = atomic_load(&g.interrupted);
    out->bytes_written = atomic_load(&g.bytes_written);
    int items = atomic_load(&g.items);
    long long bytes = atomic_load(&g.bytes);
    int drops = atomic_load(&g.drop_pending);
    out->backlog_items = items > 0 ? (uint32_t)items : 0;
    out->backlog_bytes = bytes > 0 ? (uint64_t)bytes : 0;
    out->drop_ring_depth = drops > 0 ? (uint32_t)drops : 0;
    out->high_water_items = atomic_load(&g.high_water_items);
    out->high_water_bytes = atomic_load(&g.high_water_bytes);
    out->camera_high_water_items = atomic_load(&g.camera_high_water_items);
    if (out->initialized) {
        out->limit_items = (uint32_t)g.global_max_items;
        out->limit_camera_items = (uint32_t)g.per_camera_max_items;
        out->limit_bytes = (uint64_t)g.global_max_bytes;
        out->limit_camera_bytes = (uint64_t)g.per_camera_max_bytes;
        out->limit_discard_bytes = (uint64_t)g.discard_max_bytes;
    }
    out->discard_deferred = atomic_load(&g.discard_deferred);
    out->discard_inline = atomic_load(&g.discard_inline);
    long long pending_discard = atomic_load(&g.discard_bytes);
    out->discard_pending_bytes = pending_discard > 0 ? (uint64_t)pending_discard : 0;
    out->submit_latency_count = atomic_load(&g.submit_latency_count);
    out->submit_latency_sum_ns = atomic_load(&g.submit_latency_sum_ns);
    out->submit_latency_max_ns = atomic_load(&g.submit_latency_max_ns);
    for (int i = 0; i < OBS_EVIDENCE_SUBMIT_LATENCY_BUCKETS; ++i)
        out->submit_latency_bucket[i] = atomic_load(&g.submit_latency_bucket[i]);
    int count = atomic_load_explicit(&g_slot_count, memory_order_acquire);
    for (int i = 0; i < count; ++i)
        if (atomic_load(&g_slots[i].budget_exhausted)) out->budget_exhausted_cameras++;
}
