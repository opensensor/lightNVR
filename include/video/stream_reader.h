#ifndef STREAM_READER_H
#define STREAM_READER_H

#include <pthread.h>
#include <libavformat/avformat.h>
#include <libavcodec/avcodec.h>
#include <time.h>
#include "core/config.h"

// Callback function type for packet processing
typedef int (*packet_callback_t)(const AVPacket *pkt, const AVStream *stream, void *user_data);

// Stream reader context
typedef struct {
    stream_config_t config;
    int running;
    pthread_t thread;
    AVFormatContext *input_ctx;
    int video_stream_idx;
    int audio_stream_idx;   // Index of the audio stream (-1 if none)
    int dedicated;          // Flag to indicate if this is a dedicated stream reader
    
    // Callback function for packet processing
    packet_callback_t packet_callback;
    void *callback_data;    // User data for the callback
    
    // Timestamp tracking for UDP streams
    int last_pts_initialized;  // Flag to indicate if last_pts has been initialized
    int64_t last_pts;          // Last PTS value for timestamp recovery
    int64_t frame_duration;    // Duration of a frame in timebase units
} stream_reader_ctx_t;

/**
 * Initialize the stream reader backend
 */
void init_stream_reader_backend(void);

/**
 * Cleanup the stream reader backend
 */
void cleanup_stream_reader_backend(void);

/**
 * Start a stream reader for a stream with a callback for packet processing
 * 
 * @param stream_name Name of the stream to read
 * @param dedicated Whether this is a dedicated stream reader (not shared)
 * @param callback Function to call for each packet (can be NULL)
 * @param user_data User data to pass to the callback (can be NULL)
 * @return Stream reader context or NULL on failure
 */
stream_reader_ctx_t *start_stream_reader(const char *stream_name, int dedicated, 
                                        packet_callback_t callback, void *user_data);

/**
 * Stop a stream reader
 * 
 * @param ctx Stream reader context
 * @return 0 on success, non-zero on failure
 */
int stop_stream_reader(stream_reader_ctx_t *ctx);

/**
 * Set or update the packet callback for a stream reader
 * 
 * @param ctx Stream reader context
 * @param callback Function to call for each packet
 * @param user_data User data to pass to the callback
 * @return 0 on success, non-zero on failure
 */
int set_packet_callback(stream_reader_ctx_t *ctx, packet_callback_t callback, void *user_data);

/**
 * Get the stream reader for a stream
 * 
 * @param stream_name Name of the stream
 * @return Stream reader context or NULL if not found
 */
stream_reader_ctx_t *get_stream_reader(const char *stream_name);

/**
 * Get stream reader by index
 * 
 * @param index Index of the stream reader in the array
 * @return Stream reader context or NULL if not found
 */
stream_reader_ctx_t *get_stream_reader_by_index(int index);

#endif /* STREAM_READER_H */
