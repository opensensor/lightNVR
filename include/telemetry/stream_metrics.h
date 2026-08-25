/**
 * @file stream_metrics.h
 * @brief In-process stream health metric counters and ring buffers
 *
 * Provides per-stream QoS/QoE metric collection for Prometheus exposition,
 * JSON health summaries, and UI sparklines. Zero external dependencies.
 *
 * Thread safety: metric slots use pthread_rwlock_t. Writer threads (recording,
 * HLS, stream_state) acquire write locks briefly to update counters. The HTTP
 * thread acquires read locks to snapshot metrics for API responses.
 */

#ifndef LIGHTNVR_STREAM_METRICS_H
#define LIGHTNVR_STREAM_METRICS_H

#include <stdint.h>
#include <stdbool.h>
#include <stdatomic.h>
#include <time.h>
#include <pthread.h>
#include "core/config.h"

/* Ring buffer: 5-second sample interval, 720 samples = 1 hour of sparkline data */
#define METRICS_RING_SIZE 720

/* Thresholds for health status classification */
#define STREAM_HEALTH_FRAME_TIMEOUT_UP    5   /* seconds: no frame → not UP */
#define STREAM_HEALTH_FRAME_TIMEOUT_DOWN  10  /* seconds: no frame → DOWN */
#define STREAM_HEALTH_FPS_RATIO           0.5 /* current/configured < this → degraded */

/* Gap detection threshold */
#define RECORDING_GAP_THRESHOLD_SEC 5

/**
 * Stream health status
 */
typedef enum {
    STREAM_HEALTH_UP = 0,       /* Receiving frames within thresholds */
    STREAM_HEALTH_DEGRADED,     /* Receiving frames but below FPS threshold */
    STREAM_HEALTH_DOWN          /* No frames received recently */
} stream_health_status_t;

/**
 * Pipeline reporting a frame observation.
 *
 * A single camera packet can pass through several LightNVR consumers at once.
 * Keeping their counters separate lets the sampler select one observation per
 * interval instead of reporting the sum as the camera's frame rate.
 */
typedef enum {
    METRICS_SOURCE_GENERIC = 0,
    METRICS_SOURCE_RECORDING,
    METRICS_SOURCE_HLS,
    METRICS_SOURCE_DETECTION,
    METRICS_SOURCE_COUNT
} metrics_frame_source_t;

typedef struct {
    uint64_t frames;
    uint64_t bytes;
} metrics_counter_delta_t;

/**
 * Single ring buffer sample for sparkline rendering
 */
typedef struct {
    float fps;
    float bitrate_kbps;
    time_t timestamp;
} metrics_ring_sample_t;

/**
 * Per-stream metric slot
 *
 * Gauges use _Atomic for lock-free reads from the HTTP thread.
 * The rwlock protects the ring buffer and multi-field updates.
 */
typedef struct {
    /* Identity */
    char stream_name[MAX_STREAM_NAME];
    bool active;                          /* slot in use */

    /* Health status (recomputed by sampler thread) */
    atomic_int health_status;             /* stream_health_status_t */
    atomic_int stream_up;                 /* 1 if UP, 0 otherwise */
    bool health_observed;                 /* initial health sample completed */
    time_t health_changed_at;             /* start of current health state */

    /* Gauges */
    atomic_int_fast64_t last_frame_ts;    /* unix epoch seconds of last frame */
    double current_fps;                   /* computed FPS (updated by sampler) */
    double current_bitrate_bps;           /* computed bitrate in bits/s (updated by sampler) */
    double connection_latency_ms;         /* last (re)connect latency */
    double configured_fps;               /* configured FPS for health threshold */

    /* Monotonic counters */
    atomic_uint_fast64_t frames_total;
    atomic_uint_fast64_t frames_dropped;
    atomic_uint_fast64_t bytes_received;
    atomic_uint_fast64_t reconnects_total;
    atomic_uint_fast64_t uptime_seconds;

    /* Error counters by type */
    atomic_uint_fast64_t error_decode;
    atomic_uint_fast64_t error_timeout;
    atomic_uint_fast64_t error_protocol;
    atomic_uint_fast64_t error_io;

    /* Recording metrics */
    atomic_int recording_active;          /* 1 if recording, 0 otherwise */
    atomic_uint_fast64_t recording_bytes_written;
    atomic_uint_fast64_t recording_segments_total;
    atomic_uint_fast64_t recording_gaps_total;

    /* Ring buffer for sparkline data (protected by rwlock) */
    metrics_ring_sample_t ring[METRICS_RING_SIZE];
    int ring_head;                        /* next write position */
    int ring_count;                       /* number of valid samples */

    /* Gap detection state */
    time_t last_segment_end_time;
    int expected_segment_duration;

    /* Stream start time for uptime calculation */
    time_t stream_start_time;

    /* FPS/bitrate computation state (used internally by sampler) */
    atomic_uint_fast64_t source_frames_total[METRICS_SOURCE_COUNT];
    atomic_uint_fast64_t source_bytes_received[METRICS_SOURCE_COUNT];
    uint64_t prev_source_frames[METRICS_SOURCE_COUNT];
    uint64_t prev_source_bytes[METRICS_SOURCE_COUNT];
    time_t prev_sample_time;

    /* Read-write lock for this slot */
    pthread_rwlock_t lock;
} stream_metrics_t;

/**
 * Initialize the metrics subsystem
 *
 * @param max_streams Maximum number of stream slots to allocate
 * @return 0 on success, -1 on failure
 */
int metrics_init(int max_streams);

/**
 * Shutdown the metrics subsystem and free resources
 */
void metrics_shutdown(void);

/**
 * Release (deactivate) a metric slot for a stream
 *
 * @param stream_name Stream name
 */
void metrics_release_slot(const char *stream_name);

/**
 * Record a received frame (video or audio)
 *
 * @param stream_name Stream name
 * @param bytes       Packet size in bytes
 * @param is_video    true for video frames, false for audio
 */
void metrics_record_frame(const char *stream_name, int bytes, bool is_video);

/**
 * Record a frame observed by a specific pipeline consumer.
 *
 * Parallel consumers of the same stream are de-duplicated by the sampler, so
 * recording plus HLS does not make a 30 FPS camera appear to run at 60 FPS.
 *
 * @param stream_name Stream name
 * @param bytes       Packet size in bytes
 * @param is_video    true for video frames, false for audio
 * @param source      Pipeline reporting the observation
 */
void metrics_record_frame_from_source(const char *stream_name, int bytes,
                                      bool is_video, metrics_frame_source_t source);

/**
 * Merge per-source counter deltas into one logical stream observation.
 * Exposed for deterministic regression tests; the sampler uses this directly.
 */
metrics_counter_delta_t metrics_merge_source_deltas(
    const uint64_t current_frames[METRICS_SOURCE_COUNT],
    const uint64_t current_bytes[METRICS_SOURCE_COUNT],
    uint64_t previous_frames[METRICS_SOURCE_COUNT],
    uint64_t previous_bytes[METRICS_SOURCE_COUNT]);

/**
 * Record a dropped frame
 *
 * @param stream_name Stream name
 */
void metrics_record_drop(const char *stream_name);

/**
 * Record a stream error
 *
 * @param stream_name Stream name
 * @param error_type  One of: "decode", "timeout", "protocol", "io"
 */
void metrics_record_error(const char *stream_name, const char *error_type);

/**
 * Record a reconnection event
 *
 * @param stream_name Stream name
 */
void metrics_record_reconnect(const char *stream_name);

/**
 * Record a completed recording segment (for gap detection)
 *
 * @param stream_name Stream name
 * @param start_time  Segment start timestamp
 * @param end_time    Segment end timestamp
 * @param bytes       Bytes written in this segment
 */
void metrics_record_segment_complete(const char *stream_name, time_t start_time,
                                     time_t end_time, uint64_t bytes);

/**
 * Set recording active state for a stream
 *
 * @param stream_name Stream name
 * @param active      true if recording is active
 */
void metrics_set_recording_active(const char *stream_name, bool active);

/**
 * Set the connection latency for the last (re)connect
 *
 * @param stream_name Stream name
 * @param latency_ms  Latency in milliseconds
 */
void metrics_set_connection_latency(const char *stream_name, double latency_ms);

/**
 * Set the configured FPS for a stream (used for health threshold computation)
 *
 * @param stream_name Stream name
 * @param fps         Configured FPS value
 */
void metrics_set_configured_fps(const char *stream_name, double fps);

/**
 * Thread-safe snapshot of all active stream metrics
 *
 * @param out_array   Pre-allocated array to receive snapshots
 * @param max_count   Size of out_array
 * @return Number of active streams copied
 */
int metrics_snapshot_all(stream_metrics_t *out_array, int max_count);

/**
 * Get ring buffer data for a stream's sparkline
 *
 * @param stream_name Stream name
 * @param out         Pre-allocated array to receive samples
 * @param max_count   Size of out array
 * @return Number of samples copied (oldest first)
 */
int metrics_get_ring_data(const char *stream_name, metrics_ring_sample_t *out, int max_count);

/**
 * Get the total number of allocated metric slots
 */
int metrics_get_max_streams(void);

#endif /* LIGHTNVR_STREAM_METRICS_H */
