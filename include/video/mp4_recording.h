#ifndef MP4_RECORDING_H
#define MP4_RECORDING_H

#include <pthread.h>
#include "core/config.h"
#include "video/mp4_writer.h"
#include "video/stream_reader.h"

// Structure for MP4 recording context
typedef struct {
    stream_config_t config;
    int running;
    pthread_t thread;
    char output_path[MAX_PATH_LENGTH];
    mp4_writer_t *mp4_writer;
    // CRITICAL FIX: Removed reader_ctx field since we now use the HLS streaming thread
    // to write packets to the MP4 file instead of a dedicated stream reader
} mp4_recording_ctx_t;

/**
 * Initialize MP4 recording backend
 */
void init_mp4_recording_backend(void);

/**
 * Cleanup MP4 recording backend
 */
void cleanup_mp4_recording_backend(void);

/**
 * Start MP4 recording for a stream
 * 
 * @param stream_name Name of the stream to record
 * @return 0 on success, non-zero on failure
 */
int start_mp4_recording(const char *stream_name);

/**
 * Stop MP4 recording for a stream
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
 */
void close_all_mp4_writers(void);

// The get_recording_state function is already defined in src/video/recording.c
// We'll use that implementation instead of defining it here

#endif /* MP4_RECORDING_H */
