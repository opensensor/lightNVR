#ifndef STREAM_READER_H
#define STREAM_READER_H

#include <pthread.h>
#include <libavformat/avformat.h>
#include <libavcodec/avcodec.h>
#include <time.h>
#include "core/config.h"

// Maximum number of packets in the queue - increased for smoother streaming
#define MAX_PACKET_QUEUE_SIZE 200
// Maximum number of consumers per stream
#define MAX_QUEUE_CONSUMERS 3
// Maximum retention time for packets in seconds
#define MAX_PACKET_RETENTION_TIME 5

// Consumer cursor structure
typedef struct {
    int consumer_id;        // Unique ID for this consumer
    int cursor;             // Current position in the circular buffer
    time_t last_read_time;  // Last time this consumer read a packet
    int active;             // 1 if active, 0 if not
} consumer_cursor_t;

// Simplified packet entry in the broadcast queue
typedef struct {
    AVPacket *pkt;          // Pointer to the packet data (in-memory)
    int64_t pts;            // Presentation timestamp for sorting/debugging
    int key_frame;          // 1 if key frame, 0 if not
    int stream_index;       // Stream index from the original packet
    int size;               // Size of the packet data
    time_t arrival_time;    // Time when this packet was added to the queue
} broadcast_packet_t;

// Simplified broadcast packet queue structure
typedef struct {
    broadcast_packet_t packets[MAX_PACKET_QUEUE_SIZE];
    int head;               // Oldest packet in the queue
    int tail;               // Next position to write a packet
    int size;               // Current number of packets in the queue
    pthread_mutex_t mutex;
    pthread_cond_t cond_not_empty;
    pthread_cond_t cond_not_full;
    int abort_request;
    int next_consumer_id;   // For assigning unique consumer IDs
    consumer_cursor_t consumers[MAX_QUEUE_CONSUMERS]; // Array of consumer cursors
} packet_queue_t;

// Stream reader context
typedef struct {
    stream_config_t config;
    int running;
    pthread_t thread;
    packet_queue_t queue;
    int consumers;          // Number of active consumers
    pthread_mutex_t consumers_mutex;
    AVFormatContext *input_ctx;
    int video_stream_idx;
    int dedicated;          // Flag to indicate if this is a dedicated stream reader
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
 * @param dedicated Whether this is a dedicated stream reader (not shared)
 * @return Stream reader context or NULL on failure
 */
stream_reader_ctx_t *start_stream_reader(const char *stream_name, int dedicated);

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
 * @return Consumer ID on success, negative value on failure
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
 * @return 0 on success, 1 on timeout, 2 under pressure, negative on failure or abort
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
