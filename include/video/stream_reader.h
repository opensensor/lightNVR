#ifndef STREAM_READER_H
#define STREAM_READER_H

#include <pthread.h>
#include <libavformat/avformat.h>
#include <libavcodec/avcodec.h>
#include "core/config.h"

// Maximum number of packets in the queue
#define MAX_PACKET_QUEUE_SIZE 300

// Packet queue structure
typedef struct {
    AVPacket *packets[MAX_PACKET_QUEUE_SIZE];
    int head;
    int tail;
    int size;
    pthread_mutex_t mutex;
    pthread_cond_t cond_not_empty;
    pthread_cond_t cond_not_full;
    int abort_request;
} packet_queue_t;

// Stream reader context
typedef struct {
    stream_config_t config;
    int running;
    pthread_t thread;
    packet_queue_t queue;
    int consumers;
    pthread_mutex_t consumers_mutex;
    AVFormatContext *input_ctx;
    int video_stream_idx;
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
 * Start a stream reader for a stream
 * 
 * @param stream_name Name of the stream to read
 * @return Stream reader context or NULL on failure
 */
stream_reader_ctx_t *start_stream_reader(const char *stream_name);

/**
 * Stop a stream reader
 * 
 * @param ctx Stream reader context
 * @return 0 on success, non-zero on failure
 */
int stop_stream_reader(stream_reader_ctx_t *ctx);

/**
 * Register as a consumer of the stream reader
 * 
 * @param ctx Stream reader context
 * @return 0 on success, non-zero on failure
 */
int register_stream_consumer(stream_reader_ctx_t *ctx);

/**
 * Unregister as a consumer of the stream reader
 * 
 * @param ctx Stream reader context
 * @return 0 on success, non-zero on failure
 */
int unregister_stream_consumer(stream_reader_ctx_t *ctx);

/**
 * Get a packet from the queue (blocking)
 * 
 * @param ctx Stream reader context
 * @param pkt Packet to fill
 * @return 0 on success, non-zero on failure or abort
 */
int get_packet(stream_reader_ctx_t *ctx, AVPacket *pkt);

/**
 * Get the stream reader for a stream
 * 
 * @param stream_name Name of the stream
 * @return Stream reader context or NULL if not found
 */
stream_reader_ctx_t *get_stream_reader(const char *stream_name);

#endif /* STREAM_READER_H */
