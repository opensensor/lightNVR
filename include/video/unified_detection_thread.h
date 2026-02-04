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
#include "video/mp4_writer.h"

// Maximum number of unified detection threads
#define MAX_UNIFIED_DETECTION_THREADS 16

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
    
    // Detection configuration
    char model_path[MAX_PATH_LENGTH];
    detection_model_t model;
    float detection_threshold;
    int detection_interval;  // Process every Nth keyframe
    
    // Buffer configuration
    int pre_buffer_seconds;   // Seconds to keep before detection
    int post_buffer_seconds;  // Seconds to record after last detection
    
    // Circular buffer for pre-detection content
    packet_buffer_t *packet_buffer;
    
    // MP4 recording
    mp4_writer_t *mp4_writer;
    char current_recording_path[MAX_PATH_LENGTH];
    uint64_t current_recording_id;
    
    // Detection state
    time_t last_detection_time;      // When last detection occurred
    time_t post_buffer_end_time;     // When post-buffer recording should end
    int keyframe_counter;            // For detection interval
    
    // Connection state
    atomic_int_fast64_t last_packet_time;
    atomic_int consecutive_failures;
    int reconnect_attempt;
    
    // Audio recording configuration
    bool record_audio;  // Whether to include audio in recordings

    // FFmpeg contexts (managed by thread)
    AVFormatContext *input_ctx;
    AVCodecContext *decoder_ctx;
    int video_stream_idx;
    int audio_stream_idx;
    
    // Statistics
    uint64_t total_packets_processed;
    uint64_t total_detections;
    uint64_t total_recordings;
    
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
 * @return 0 on success, -1 on error
 */
int start_unified_detection_thread(const char *stream_name, const char *model_path,
                                   float threshold, int pre_buffer_seconds,
                                   int post_buffer_seconds);

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

#endif /* UNIFIED_DETECTION_THREAD_H */

