#ifndef STREAM_READER_H
#define STREAM_READER_H

#include <pthread.h>
#include <libavformat/avformat.h>
#include <libavcodec/avcodec.h>
#include "core/config.h"

// Maximum number of packets in the queue
#define MAX_PACKET_QUEUE_SIZE 300
// Maximum number of consumers per stream
#define MAX_QUEUE_CONSUMERS 5

// Packet status for each consumer
typedef struct {
    int consumer_id;
    int processed;  // 1 if processed, 0 if not
} packet_consumer_status_t;

// Packet entry in the broadcast queue
typedef struct {
    AVPacket *pkt;          // Pointer to the packet data (in-memory)
    int64_t pts;            // Presentation timestamp for sorting/debugging
    int key_frame;          // 1 if key frame, 0 if not
    int stream_index;       // Stream index from the original packet
    int size;               // Size of the packet data
    packet_consumer_status_t consumer_status[MAX_QUEUE_CONSUMERS];
    int consumer_count;     // Number of consumers that need to process this packet
    int all_processed;      // 1 if all consumers have processed, 0 otherwise
} broadcast_packet_t;

// Broadcast packet queue structure
typedef struct {
    broadcast_packet_t packets[MAX_PACKET_QUEUE_SIZE];
    int head;
    int tail;
    int size;
    pthread_mutex_t mutex;
    pthread_cond_t cond_not_empty;
    pthread_cond_t cond_not_full;
    int abort_request;
    int next_consumer_id;   // For assigning unique consumer IDs
    int active_consumers;   // Count of currently active consumers
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
 * @param consumer_id ID of the consumer to unregister
 * @return 0 on success, non-zero on failure
 */
int unregister_stream_consumer(stream_reader_ctx_t *ctx, int consumer_id);

/**
 * Get a packet from the queue (blocking)
 * 
 * @param ctx Stream reader context
 * @param pkt Packet to fill
 * @param consumer_id ID of the consumer requesting the packet
 * @return 0 on success, non-zero on failure or abort
 */
int get_packet(stream_reader_ctx_t *ctx, AVPacket *pkt, int consumer_id);

/**
 * Get the stream reader for a stream
 * 
 * @param stream_name Name of the stream
 * @return Stream reader context or NULL if not found
 */
stream_reader_ctx_t *get_stream_reader(const char *stream_name);

#endif /* STREAM_READER_H */
