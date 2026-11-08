/*
 * SPDX-License-Identifier: GPL-3.0-or-later
 * Copyright (C) 2026 OpenSensor Engineering and LightNVR contributors
 *
 * This file is part of LightNVR <https://github.com/opensensor/lightNVR>.
 */

/*
 * INT_01 L3: evidence frame queue and encoder worker (contract:
 * docs/integrations/OBSERVATIONS_CONTRACT.md sections 4 and 13).
 *
 * Producers (detection threads, the HTTP API detector) hand over an owned
 * RGB24 frame or the exact submitted JPEG bytes plus the observation batch.
 * Admission is lock-free with respect to the worker: it never waits on I/O,
 * on the DB mutex or on any lock the worker holds, and returns immediately.
 * The single worker thread inserts the batch (db_observation_insert_batch,
 * asset pending), encodes the JPEG with the shared jpeg_encoder_cache_* code,
 * writes <final>.tmp, fsyncs, renames, fsyncs the directory and only then
 * commits pending -> ready. Any failure unlinks the temp file and commits
 * pending -> unavailable(encode_failed|write_failed).
 *
 * Producer-side drops (queue_full, budget_exhausted) push a fixed-size drop
 * record (batch metadata, no image) onto a bounded ring; the worker inserts
 * the batch directly as unavailable(<reason>). When that ring is full too the
 * observation is lost and dropped_untracked is incremented.
 *
 * Storage layout (contract section 4.5). The DB stores the path relative to
 * the storage root passed to observation_evidence_init():
 *
 *     observations/<camera_uuid>/<yyyy-mm-dd>/<asset_uuid>.jpg
 *
 * <yyyy-mm-dd> is the UTC date of the batch's observed_at_ms. While being
 * written the file is "<asset_uuid>.jpg.tmp" in the same directory. Files are
 * created 0640, directories 0750. L4 reconciliation resolves the absolute
 * path as <storage_path>/<path>.
 *
 * Lifecycle: observation_evidence_init() only records configuration. The
 * worker thread, the drop ring and any per-item memory are created lazily by
 * the first submission, so with evidence disabled on every camera nothing is
 * allocated and no thread runs (PRD R6). Start order in main.c: L4
 * reconciliation -> observation_evidence_init() -> L5 relay -> detection.
 * Mutations need event_identity_init() to have run (L2 message IDs).
 */

#ifndef LIGHTNVR_CORE_OBSERVATION_EVIDENCE_H
#define LIGHTNVR_CORE_OBSERVATION_EVIDENCE_H

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>

#include "database/db_observations.h"

/* Contract section 13 constants. */
#define OBS_EVIDENCE_PER_CAMERA_DEPTH 8
#define OBS_EVIDENCE_GLOBAL_DEPTH 64
#define OBS_EVIDENCE_DROP_RING_SIZE 256              /* power of two */
#define OBS_EVIDENCE_DEFAULT_GLOBAL_BYTES (32LL * 1024 * 1024)
#define OBS_EVIDENCE_JPEG_QUALITY 85
#define OBS_EVIDENCE_DIR "observations"
#define OBS_EVIDENCE_DEFAULT_DRAIN_MS 2000
#define OBS_EVIDENCE_SUBMIT_LATENCY_BUCKETS 10       /* 9 bounds + +Inf */

typedef enum {
    OBS_EVIDENCE_ACCEPTED = 0,
    OBS_EVIDENCE_DROPPED_QUEUE_FULL = 1,  /* batch recorded as unavailable(queue_full) */
    OBS_EVIDENCE_DROPPED_BUDGET = 2,      /* batch recorded as unavailable(budget_exhausted) */
    OBS_EVIDENCE_DISABLED = 3,            /* not initialized or shutting down: nothing recorded */
    OBS_EVIDENCE_INVALID = -1             /* bad arguments: nothing recorded */
} observation_evidence_result_t;

typedef enum {
    OBS_TRANSFORM_NONE = 0,
    OBS_TRANSFORM_RESIZE,
    OBS_TRANSFORM_LETTERBOX
} observation_transform_mode_t;

/* Contract section 4.4. scale = inference / stored along each axis. */
typedef struct {
    observation_transform_mode_t mode;
    int source_width, source_height;
    int inference_width, inference_height;
    double scale_x, scale_y;
    double pad_x, pad_y;
} observation_evidence_transform_t;

typedef struct {
    const char *storage_path;        /* required; evidence root is <storage_path>/observations */
    int64_t global_max_bytes;        /* 0 = OBS_EVIDENCE_DEFAULT_GLOBAL_BYTES
                                        (system_settings.observation_global_queue_bytes) */
    int global_max_items;            /* 0 = OBS_EVIDENCE_GLOBAL_DEPTH */
    int per_camera_max_items;        /* 0 = OBS_EVIDENCE_PER_CAMERA_DEPTH */
    int64_t per_camera_max_bytes;    /* 0 = global_max_bytes (one frame may use the whole
                                        global budget; the depth bound keeps fairness) */
} observation_evidence_config_t;

/* Records configuration and enables admission. Allocates nothing and starts
 * no thread. Returns 0, or -1 on bad config / already initialized. */
int observation_evidence_init(const observation_evidence_config_t *config);

/* Stops admission (new submits return DISABLED), lets the worker encode for
 * up to drain_timeout_ms (< 0 = OBS_EVIDENCE_DEFAULT_DRAIN_MS), then records
 * every item still queued as unavailable(interrupted) without encoding,
 * inserts pending drop records, joins the worker and frees everything. Safe to
 * call when never initialized or never started. init() may be called again. */
void observation_evidence_shutdown(int drain_timeout_ms);

/*
 * Submit a decoded inference frame. Takes ownership of rgb24 on EVERY return.
 * When not accepted it is released with free(); for a tracked drop the worker
 * does that, so the producer never pays for unmapping a large frame (bounded
 * by limit_discard_bytes pending). rgb24 is packed RGB24,
 * width * 3 bytes per row, and must be allocated with at least
 * RGB24_SWS_TAIL_PADDING bytes of slack after width * height * 3, exactly as
 * ffmpeg_frame_to_rgb24_padded() returns it. batch is copied; its
 * camera_uuid is the capture-time identity. transform NULL = mode none with
 * source = inference = stored size. asset_uuid_out (optional) receives the
 * asset ID for ACCEPTED and for tracked drops, "" otherwise.
 *
 * Never blocks: no I/O, no DB access, no lock the worker holds (< 1 ms).
 */
observation_evidence_result_t observation_evidence_submit(
    const observation_batch_input_t *batch,
    uint8_t *rgb24, int width, int height,
    const observation_evidence_transform_t *transform,
    char asset_uuid_out[OBS_UUID_SIZE]);

/* Same as observation_evidence_submit() for the HTTP API detector (L7): the
 * exact submitted JPEG bytes (ownership transferred on every return) are
 * stored without re-encoding, origin submitted_image. width/height are the
 * JPEG's pixel size. */
observation_evidence_result_t observation_evidence_submit_jpeg(
    const observation_batch_input_t *batch,
    uint8_t *jpeg, size_t jpeg_size, int width, int height,
    const observation_evidence_transform_t *transform,
    char asset_uuid_out[OBS_UUID_SIZE]);

/* Record a batch whose source cannot supply media (ONVIF / external events:
 * reason "no_matching_media"), via the drop ring, so the calling thread never
 * touches the DB. Returns ACCEPTED when queued, DROPPED_QUEUE_FULL when the
 * ring is full (dropped_untracked), DISABLED or INVALID. */
observation_evidence_result_t observation_evidence_submit_unavailable(
    const observation_batch_input_t *batch, const char *reason,
    char asset_uuid_out[OBS_UUID_SIZE]);

/* L4: per-camera "budget exhausted, only protected assets remain" flag.
 * While set, admission for that camera returns DROPPED_BUDGET. Memory only;
 * works before init and while the worker is not started. */
void observation_evidence_set_budget_exhausted(const char *camera_uuid, bool exhausted);
bool observation_evidence_is_budget_exhausted(const char *camera_uuid);

/* Optional per-camera enablement probe consulted by the worker right before
 * encoding (contract reason "disabled": evidence turned off between admission
 * and encode). NULL (default) = always enabled. Called on the worker thread
 * only, outside any DB transaction, so it may read the DB. */
typedef bool (*observation_evidence_enabled_fn)(const char *camera_uuid, void *ctx);
void observation_evidence_set_enabled_probe(observation_evidence_enabled_fn fn, void *ctx);

/* Stock probe (main.c installs it): streams.observation_evidence_enabled for
 * the camera via get_stream_config_by_uuid(); an unknown camera is disabled. */
bool observation_evidence_stream_enabled_probe(const char *camera_uuid, void *ctx);

/* Serialize a transform (contract section 4.4) as compact JSON. NULL =
 * identity for width x height. Returns 0, or -1 when invalid / too small. */
int observation_evidence_transform_json(const observation_evidence_transform_t *transform,
                                        int width, int height, char *out, size_t out_size);

/* Relative path of an asset file (layout above). Returns 0 or -1. */
int observation_evidence_relative_path(const char *camera_uuid, const char *asset_uuid,
                                       int64_t observed_at_ms, char *out, size_t out_size);

/* ---- Diagnostics (L10) ------------------------------------------------- */

typedef struct {
    bool initialized;
    bool worker_running;
    bool allocated;                  /* drop ring or worker exists */
    uint64_t accepted;
    uint64_t dropped_queue_full;
    uint64_t dropped_budget;
    uint64_t dropped_disabled;       /* submits while not initialized / shutting down */
    uint64_t dropped_untracked;      /* drop ring full: observation lost */
    uint64_t unavailable_submitted;  /* submit_unavailable() records queued */
    uint64_t ready;                  /* pending -> ready committed */
    uint64_t failed_encode;
    uint64_t failed_write;
    uint64_t failed_db;              /* insert / transition errors */
    uint64_t disabled_at_encode;
    uint64_t interrupted;            /* abandoned at shutdown */
    uint64_t bytes_written;
    uint32_t backlog_items;
    uint64_t backlog_bytes;
    uint32_t drop_ring_depth;
    uint32_t high_water_items;
    uint64_t high_water_bytes;
    uint32_t budget_exhausted_cameras;
    /* X2 (A11): configured admission bounds and the highest item count any
     * single camera held this process run. */
    uint32_t limit_items;
    uint32_t limit_camera_items;
    uint64_t limit_bytes;
    uint64_t limit_camera_bytes;
    uint32_t camera_high_water_items;
    /* Producer time spent inside every submit that reached admission (every
     * result except DISABLED), this process run. Buckets are NOT cumulative:
     * bucket i counts calls with latency <= bounds_ns[i] and > bounds_ns[i-1];
     * the last bucket counts calls above the largest bound. */
    uint64_t submit_latency_count;
    uint64_t submit_latency_sum_ns;
    uint64_t submit_latency_max_ns;
    uint64_t submit_latency_bucket[OBS_EVIDENCE_SUBMIT_LATENCY_BUCKETS];
    /* Frames of rejected submissions: released by the worker (deferred, so the
     * producer never pays for unmapping a large buffer) or, past
     * limit_discard_bytes pending or with the drop ring full, inline. */
    uint64_t discard_deferred;
    uint64_t discard_inline;
    uint64_t discard_pending_bytes;
    uint64_t limit_discard_bytes;    /* max(limit_bytes, OBS_EVIDENCE_DEFAULT_GLOBAL_BYTES) */
} observation_evidence_stats_t;

void observation_evidence_get_stats(observation_evidence_stats_t *out);

/* Upper bounds (ns) of submit_latency_bucket[0 .. BUCKETS-2]; strictly
 * increasing (1 us .. 10 ms). The last bucket is +Inf. */
extern const uint64_t
    observation_evidence_submit_latency_bounds_ns[OBS_EVIDENCE_SUBMIT_LATENCY_BUCKETS - 1];

/* ---- Test hooks ---------------------------------------------------------- */

typedef enum {
    OBS_EVIDENCE_STAGE_ITEM_BEGIN = 0,  /* worker picked an item (may block to stall the queue) */
    OBS_EVIDENCE_STAGE_BEFORE_RENAME,   /* tmp written + fsynced; nonzero = inject failure */
    OBS_EVIDENCE_STAGE_ITEM_DONE,
    OBS_EVIDENCE_STAGE_AFTER_INSERT,    /* batch + pending asset committed, nothing written yet */
    OBS_EVIDENCE_STAGE_BEFORE_COMMIT    /* final file renamed + dir fsynced, pending->ready not
                                         * yet committed */
} observation_evidence_stage_t;

/* Invoked on the worker thread. A nonzero return at BEFORE_RENAME is treated
 * as a write failure; other stages ignore the return value. NULL clears.
 * Intended for tests and fault injection only: the hook can be installed only
 * by code linked into the process (there is no configuration, environment or
 * API path to it), so a production build can never trigger it. The L11
 * crash-boundary test (tests/unit/test_observation_crash_boundaries.c) raises
 * SIGKILL from AFTER_INSERT, BEFORE_RENAME and BEFORE_COMMIT. */
typedef int (*observation_evidence_stage_fn)(observation_evidence_stage_t stage,
                                             const char *path, void *ctx);
void observation_evidence_set_stage_hook(observation_evidence_stage_fn fn, void *ctx);

#endif /* LIGHTNVR_CORE_OBSERVATION_EVIDENCE_H */
