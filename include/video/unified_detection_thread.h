/**
 * Unified Detection Recording Thread
 * 
 * This module implements a unified thread that handles:
 * - Continuous RTSP packet reading from go2rtc
 * - Circular buffer for pre-detection content
 * - Object detection on keyframes
 * - MP4 recording with proper pre/post buffer support
 * 
 * The key innovation is that a single thread manages the entire pipeline,
 * ensuring that pre-buffer content is always available when detection triggers.
 */

#ifndef UNIFIED_DETECTION_THREAD_H
#define UNIFIED_DETECTION_THREAD_H

#include <pthread.h>
#include <stdatomic.h>
#include <stdbool.h>
#include <time.h>
#include <libavformat/avformat.h>
#include <libavcodec/avcodec.h>

#include "core/config.h"
#include "video/packet_buffer.h"
#include "video/detection_model.h"
#include "video/detection_result.h"
#include "video/mp4_writer.h"
#include "video/stream_manager.h"

// Maximum number of unified detection threads
#define MAX_UNIFIED_DETECTION_THREADS MAX_STREAMS

/**
 * Thread state machine states
 */
typedef enum {
    UDT_STATE_INITIALIZING = 0,  // Thread starting up
    UDT_STATE_CONNECTING,        // Connecting to RTSP stream
    UDT_STATE_BUFFERING,         // Connected, buffering packets, running detection
    UDT_STATE_RECORDING,         // Detection triggered, recording to MP4
    UDT_STATE_POST_BUFFER,       // Detection ended, recording post-buffer
    UDT_STATE_RECONNECTING,      // Lost connection, attempting reconnect
    UDT_STATE_STOPPING,          // Thread shutting down
    UDT_STATE_STOPPED            // Thread has stopped
} unified_detection_state_t;

/**
 * Unified Detection Thread Context
 * 
 * Contains all state needed for a single stream's detection and recording.
 */
typedef struct {
    // Stream identification
    char stream_name[MAX_STREAM_NAME];
    char rtsp_url[MAX_PATH_LENGTH];
    char output_dir[MAX_PATH_LENGTH];
    
    // Thread management
    pthread_t thread;
    atomic_int running;
    atomic_int state;  // Uses unified_detection_state_t values
    int shutdown_component_id;
    int slot_idx;  // Index in the global detection_contexts array
    
    // Detection configuration
    char model_path[MAX_PATH_LENGTH];
    detection_model_t model;
    float detection_threshold;
    int detection_interval;  // Seconds between detection checks
    
    // Buffer configuration
    int pre_buffer_seconds;   // Seconds to keep before detection
    int post_buffer_seconds;  // Seconds to record after last detection
    int segment_duration;     // Maximum segment duration in seconds (for chunking detection recordings)
    
    // Circular buffer for pre-detection content
    packet_buffer_t *packet_buffer;
    
    // MP4 recording
    mp4_writer_t *mp4_writer;
    char current_recording_path[MAX_PATH_LENGTH];
    // Written by the main UDT thread (recording start/stop), read by the
    // detection-stream producer thread for API recording_id linkage.
    // _Atomic so those cross-thread reads/writes are race-free on all targets
    // (notably 32-bit ARM, where a plain 64-bit access is not atomic).
    _Atomic uint64_t current_recording_id;
    
    // Detection state
    atomic_llong last_detection_time;      // When last detection occurred (stored as atomic epoch seconds)
    atomic_llong last_detection_check_time; // When last detection check was attempted (for time-based interval)
    atomic_llong post_buffer_end_time;     // When post-buffer recording should end
    atomic_int log_counter;          // Counter for periodic logging; intentionally accessed without ctx->mutex,
                                     // but all accesses must use atomic operations, and exact accuracy is not critical.
    bool model_load_failed;          // Set on first load_detection_model() failure to suppress
                                     // per-frame retry/log spam. Once true, the stream has been
                                     // driven into STREAM_STATE_ERROR via handle_stream_error().
    
    // Connection state
    atomic_int_fast64_t last_packet_time;
    atomic_int consecutive_failures;
    int reconnect_attempt;
    
    // Audio recording configuration
    bool record_audio;  // Whether to include audio in recordings

    // External motion trigger: set to 1 by unified_detection_notify_motion() when
    // an ONVIF-managed master stream propagates its motion event to this UDT-managed
    // slave stream (e.g. the PTZ lens on a TP-Link C545D that has no ONVIF profile).
    // The UDT processing path checks this flag when handling eligible video
    // keyframes and treats a rising edge as equivalent to a local detection
    // event (starts/extends recording).
    // Use atomic_store/atomic_load to avoid races between the event-processor thread
    // (writer) and the UDT thread (reader/resetter).
    atomic_int external_motion_trigger;  // 0 = idle, 1 = motion active, 2 = motion ended

    // Annotation-only mode: when true, detection runs but does NOT create separate MP4 files
    // Detections are stored in the database and linked to the continuous recording
    bool annotation_only;

    // -------------------------------------------------------------------------
    // ONVIF async detection thread
    // -------------------------------------------------------------------------
    // When model_path == "onvif" a dedicated background thread calls
    // detect_motion_onvif() in a loop so that the UDT main loop (and therefore
    // av_read_frame()) is never blocked by a CURL/SOAP round-trip.
    //
    // Lifecycle:
    //   • Created by start_unified_detection_thread() alongside the UDT.
    //   • Joined inside unified_detection_thread_func() before ctx is freed.
    //   • shutdown_unified_detection_system() joins it during forced shutdown.
    //
    // Thread-safety:
    //   • onvif_thread_running  – writer: UDT teardown; reader: ONVIF thread.
    //   • onvif_motion_detected – writer: ONVIF thread; reader: UDT main loop.
    //   • onvif_motion_timestamp– same as above.
    //   All three use atomic operations only; no mutex required.
    pthread_t    onvif_thread;           // handle (valid only when thread was started)
    atomic_int   onvif_thread_running;   // 1 = running, 0 = stop requested
    atomic_int   onvif_motion_detected;  // 1 = motion active, 0 = idle
    atomic_llong onvif_motion_timestamp; // epoch-seconds of last detected motion

    // ONVIF connection parameters cached at thread-start time so that the
    // polling thread never needs to call get_stream_config_by_name().
    char onvif_url_cached[MAX_PATH_LENGTH]; // http://host[:port]
    char onvif_username_cached[64];
    char onvif_password_cached[64];

    // -------------------------------------------------------------------------
    // Secondary detection stream thread
    // -------------------------------------------------------------------------
    // When detection_url is non-empty a dedicated background thread (pure
    // producer) opens that URL (any FFmpeg-readable source), decodes frames,
    // and runs the configured detection model.  It is responsible ONLY for
    // detection inference; all actions (DB storage, recording state) are
    // handled by the UDT main loop (consumer).
    //
    // Producer → consumer protocol (single-slot, overwrite semantics):
    //   1. Thread detects → writes result to detection_stream_pending (mutex)
    //                     → sets detection_stream_result = 1 (atomic)
    //   2. process_packet reads result via atomic_exchange(0) + mutex
    //                     → calls report_detections + handle_recording_state
    //
    // detection_stream_connected is set to 1 only once the stream is
    // successfully opened and decoding.  It is 0 while connecting or
    // reconnecting.  process_packet uses this — not thread_running — as the
    // guard to suppress main-stream detection, so fallback is automatic
    // during any outage.
    //
    // Lifecycle mirrors the ONVIF thread:
    //   • Created alongside the UDT in start_unified_detection_thread().
    //   • Joined inside unified_detection_thread_func() before ctx is freed.
    //   • Stopped during forced shutdown.
    pthread_t         detection_stream_thread;
    atomic_int        detection_stream_thread_running; // 1 = running, 0 = stop requested
    atomic_int        detection_stream_connected;      // 1 = open & decoding, 0 = connecting/disconnected
    atomic_int        detection_stream_result;         // 1 = pending result ready, 0 = none
    detection_result_t detection_stream_pending;       // latest result; guarded by result_mutex
    time_t            detection_stream_pending_ts;     // frame time of the pending result (producer's detect time); guarded by result_mutex
    pthread_mutex_t   detection_stream_result_mutex;   // protects detection_stream_pending
    char              detection_stream_url[MAX_URL_LENGTH]; // cached at thread-start

    // FFmpeg contexts (managed exclusively by the unified detection thread).
    // Concurrency / lifetime:
    // - These pointers and stream indices are created, updated, and freed only
    //   by the unified detection thread during connect/reconnect/shutdown.
    // - Other threads MUST NOT read or modify these fields directly unless they
    //   first acquire the appropriate synchronization primitive (for example,
    //   the context mutex defined alongside this struct in the implementation).
    // - Accessing these fields without synchronization while the detection
    //   thread may be reconnecting or shutting down can lead to data races or
    //   use-after-free of the FFmpeg contexts.
    AVFormatContext *input_ctx;
    AVCodecContext *decoder_ctx;
    int video_stream_idx;
    int audio_stream_idx;
    
    // Statistics
    uint64_t total_packets_processed;
    uint64_t total_detections;
    uint64_t total_recordings;

    // Runtime FPS measurement (used when SDP omits frame rate). After enough frames
    // have been observed in the measurement window, the measured FPS is finalized,
    // fps_is_provisional is set to false, and the resulting FPS value is persisted
    // to the database for future use.
    atomic_bool fps_is_provisional;           // true when stored FPS is a fallback guess
    atomic_int fps_measurement_frame_count;   // video frames counted during measurement window
    atomic_llong fps_measurement_start_ns;    // start of measurement window, in nanoseconds since an epoch

    // go2rtc replay detection: when the UDT connects, go2rtc replays its ring
    // buffer in real-time before switching to live packets.  We track the wall
    // clock time of the successful connect and the PTS of the very first video
    // packet.  For every subsequent packet we compute:
    //   pts_elapsed  = (pts - first_video_pts) * timebase   [stream time passed]
    //   wall_elapsed = now - stream_connect_time             [real time passed]
    //   replay_lag   = wall_elapsed - pts_elapsed            [how far behind live]
    // While replay_lag > pre_buffer_seconds the stream is still replaying old
    // data and packets must NOT enter the pre-buffer.
    time_t stream_connect_time;   // wall time of last successful connect
    int64_t first_video_pts;      // PTS of first video packet after connect
    bool first_video_pts_set;     // whether first_video_pts has been recorded
    bool stream_is_live;          // true once replay_lag <= pre_buffer_seconds

    // Thread safety
    pthread_mutex_t mutex;
} unified_detection_ctx_t;

/**
 * Initialize the unified detection thread system
 * Must be called before starting any threads
 * 
 * @return 0 on success, -1 on error
 */
int init_unified_detection_system(void);

/**
 * Shutdown the unified detection thread system
 * Stops all running threads and cleans up resources
 */
void shutdown_unified_detection_system(void);

/**
 * Start unified detection recording for a stream
 *
 * @param stream_name Name of the stream
 * @param model_path Path to detection model
 * @param threshold Detection confidence threshold (0.0-1.0)
 * @param pre_buffer_seconds Seconds of pre-detection buffer
 * @param post_buffer_seconds Seconds of post-detection recording
 * @param annotation_only If true, detection runs but does NOT create separate MP4 files.
 *                        Detections are stored in DB and linked to the continuous recording.
 * @return 0 on success, -1 on error
 */
int start_unified_detection_thread(const char *stream_name, const char *model_path,
                                   float threshold, int pre_buffer_seconds,
                                   int post_buffer_seconds, bool annotation_only);

/**
 * Stop unified detection recording for a stream
 * 
 * @param stream_name Name of the stream
 * @return 0 on success, -1 on error
 */
int stop_unified_detection_thread(const char *stream_name);

/**
 * Check if unified detection is running for a stream
 * 
 * @param stream_name Name of the stream
 * @return true if running, false otherwise
 */
bool is_unified_detection_running(const char *stream_name);

/**
 * Get the current state of a unified detection thread
 *
 * @param stream_name Name of the stream
 * @return Current state, or UDT_STATE_STOPPED if not found
 */
unified_detection_state_t get_unified_detection_state(const char *stream_name);

/**
 * Get the effective stream status based on the UDT state for API reporting.
 *
 * Maps the internal UDT state (and reconnect attempt count) to a
 * stream_status_t value so callers can report an accurate status without
 * needing access to the raw UDT context.
 *
 * Mapping:
 *  UDT_STATE_INITIALIZING               -> STREAM_STATUS_STARTING
 *  UDT_STATE_CONNECTING (attempt == 0)  -> STREAM_STATUS_STARTING
 *  UDT_STATE_CONNECTING (attempt  > 0)  -> STREAM_STATUS_RECONNECTING
 *  UDT_STATE_BUFFERING / RECORDING / POST_BUFFER -> STREAM_STATUS_RUNNING
 *  UDT_STATE_RECONNECTING               -> STREAM_STATUS_RECONNECTING
 *  UDT_STATE_STOPPING                   -> STREAM_STATUS_STOPPING
 *  UDT_STATE_STOPPED (or not found)     -> STREAM_STATUS_STOPPED
 *
 * @param stream_name Name of the stream
 * @return Appropriate stream_status_t value
 */
stream_status_t get_unified_detection_effective_status(const char *stream_name);

/**
 * Get the number of reconnect attempts made by a unified detection thread.
 *
 * This counter increments each time the UDT fails to open the RTSP stream
 * and transitions through UDT_STATE_RECONNECTING back to UDT_STATE_CONNECTING.
 * A value > 0 means the initial connection was never established (or was lost),
 * making it a reliable proxy for "how many times has this stream failed" for
 * go2rtc-managed streams where the state manager stays at STREAM_STATE_INACTIVE.
 *
 * @param stream_name Name of the stream
 * @return Reconnect attempt count, or 0 if not found
 */
int get_unified_detection_reconnect_attempts(const char *stream_name);

/**
 * Get statistics for a unified detection thread
 *
 * @param stream_name Name of the stream
 * @param packets_processed Output: total packets processed
 * @param detections Output: total detections
 * @param recordings Output: total recordings created
 * @return 0 on success, -1 if not found
 */
int get_unified_detection_stats(const char *stream_name,
                                uint64_t *packets_processed,
                                uint64_t *detections,
                                uint64_t *recordings);

/**
 * Notify a UDT-managed stream of an externally-detected motion event.
 *
 * Called by the ONVIF motion recording system when a master stream's ONVIF
 * event must be propagated to a slave stream that is managed by a UDT
 * (e.g. the PTZ lens on a dual-lens camera).  The UDT thread polls
 * ctx->external_motion_trigger and reacts on the next packet boundary.
 *
 * @param stream_name   Name of the slave stream (must be running as a UDT)
 * @param motion_active true  = motion started / ongoing
 *                      false = motion ended
 */
void unified_detection_notify_motion(const char *stream_name, bool motion_active);

#endif /* UNIFIED_DETECTION_THREAD_H */

