/**
 * MP4 Recording Module
 * 
 * This module is responsible for managing MP4 recording threads.
 * Each recording thread is responsible for starting and stopping an MP4 recorder
 * for a specific stream. The actual RTSP interaction is contained within the
 * MP4 writer module.
 */

#ifndef MP4_RECORDING_H
#define MP4_RECORDING_H

#include <pthread.h>
#include "core/config.h"
#include "video/mp4_writer.h"

/**
 * Structure for MP4 recording context
 * 
 * This structure represents a single recording thread that manages
 * an MP4 writer for a specific stream. The MP4 writer handles all
 * RTSP interaction internally.
 */
typedef struct {
    stream_config_t config;    // Stream configuration
    int running;               // Flag indicating if the thread is running
    pthread_t thread;          // Recording thread
    char output_path[MAX_PATH_LENGTH]; // Path to the output MP4 file
    mp4_writer_t *mp4_writer;  // MP4 writer instance
    char trigger_type[16];     // Type of trigger ('scheduled', 'detection', 'motion', 'manual')
} mp4_recording_ctx_t;

/**
 * Initialize MP4 recording backend
 * 
 * This function initializes the recording contexts array and resets the shutdown flag.
 */
void init_mp4_recording_backend(void);

/**
 * Cleanup MP4 recording backend
 * 
 * This function stops all recording threads and frees all recording contexts.
 */
void cleanup_mp4_recording_backend(void);

/**
 * Start MP4 recording for a stream
 * 
 * This function creates a new recording thread for the specified stream.
 * The recording thread will create an MP4 writer and start the RTSP recording.
 * 
 * @param stream_name Name of the stream to record
 * @return 0 on success, non-zero on failure
 */
int start_mp4_recording(const char *stream_name);

/**
 * Start MP4 recording for a stream with a specific URL
 *
 * This function creates a new recording thread for the specified stream
 * using the provided URL instead of the stream's configured URL.
 * This is useful for using go2rtc's RTSP URL as the input for MP4 recording.
 *
 * @param stream_name Name of the stream to record
 * @param url URL to use for recording
 * @return 0 on success, non-zero on failure
 */
int start_mp4_recording_with_url(const char *stream_name, const char *url);

/**
 * Start MP4 recording for a stream with a specific trigger type
 *
 * This function creates a new recording thread for the specified stream
 * and marks the recording with the specified trigger type.
 *
 * @param stream_name Name of the stream to record
 * @param trigger_type Type of trigger ('scheduled', 'detection', 'motion', 'manual')
 * @return 0 on success, non-zero on failure
 */
int start_mp4_recording_with_trigger(const char *stream_name, const char *trigger_type);

/**
 * Start MP4 recording for a stream with a specific URL and trigger type
 *
 * This function creates a new recording thread for the specified stream
 * using the provided URL and marks the recording with the specified trigger type.
 *
 * @param stream_name Name of the stream to record
 * @param url URL to use for recording
 * @param trigger_type Type of trigger ('scheduled', 'detection', 'motion', 'manual')
 * @return 0 on success, non-zero on failure
 */
int start_mp4_recording_with_url_and_trigger(const char *stream_name, const char *url, const char *trigger_type);

/**
 * Stop MP4 recording for a stream
 * 
 * This function stops the recording thread for the specified stream.
 * 
 * @param stream_name Name of the stream to stop recording
 * @return 0 on success, non-zero on failure
 */
int stop_mp4_recording(const char *stream_name);

/**
 * Get the MP4 writer for a stream
 * 
 * @param stream_name Name of the stream
 * @return MP4 writer instance or NULL if not found
 */
mp4_writer_t *get_mp4_writer_for_stream(const char *stream_name);

/**
 * Register an MP4 writer for a stream
 * 
 * @param stream_name Name of the stream
 * @param writer MP4 writer instance
 * @return 0 on success, non-zero on failure
 */
int register_mp4_writer_for_stream(const char *stream_name, mp4_writer_t *writer);

/**
 * Unregister an MP4 writer for a stream
 * 
 * @param stream_name Name of the stream
 */
void unregister_mp4_writer_for_stream(const char *stream_name);

/**
 * Close all MP4 writers during shutdown
 * @deprecated Use safe_close_all_mp4_writers from mp4_writer_shutdown.h instead
 */
void close_all_mp4_writers(void);

/**
 * Signal all active MP4 recording threads to force reconnection
 *
 * This is useful when the upstream source (e.g., go2rtc) has restarted
 * and all current RTSP connections are stale. Each recording thread
 * will close its current connection and attempt to reconnect.
 */
void signal_all_mp4_recordings_reconnect(void);

#endif /* MP4_RECORDING_H */
