/**
 * Internal header file for HLS writer thread implementation
 * Contains shared definitions and declarations for the HLS writer thread components
 */

#ifndef HLS_WRITER_THREAD_INTERNAL_H
#define HLS_WRITER_THREAD_INTERNAL_H

#include <pthread.h>
#include <stdatomic.h>
#include <libavformat/avformat.h>
#include "video/hls_writer.h"
#include "video/hls_writer_thread.h"

// Stream thread state constants
typedef enum {
    STREAM_THREAD_INITIALIZING = 0,
    STREAM_THREAD_CONNECTING,
    STREAM_THREAD_RUNNING,
    STREAM_THREAD_RECONNECTING,
    STREAM_THREAD_STOPPING,
    STREAM_THREAD_STOPPED
} stream_thread_state_t;

// Maximum time (in seconds) without receiving a packet before considering the connection dead
#define MAX_PACKET_TIMEOUT 5

// Base reconnection delay in milliseconds (500ms)
#define BASE_RECONNECT_DELAY_MS 500

// Maximum reconnection delay in milliseconds (30 seconds)
#define MAX_RECONNECT_DELAY_MS 30000

// Forward declaration for go2rtc integration
extern bool go2rtc_integration_is_using_go2rtc_for_hls(const char *stream_name);
extern bool go2rtc_get_rtsp_url(const char *stream_name, char *url, size_t url_size);

// Thread function declaration
void *hls_writer_thread_func(void *arg);

// Utility functions
void safe_cleanup_resources(AVFormatContext **input_ctx, AVPacket **pkt);
int calculate_reconnect_delay(int attempt);

// Connection management functions
int check_rtsp_connection(const char *rtsp_url, char *host, int *port);
int send_rtsp_options_request(const char *host, int port, const char *path);

// State machine functions
void handle_initializing_state(hls_writer_thread_ctx_t *ctx, stream_thread_state_t *thread_state);
int handle_connecting_state(hls_writer_thread_ctx_t *ctx, stream_thread_state_t *thread_state,
                           AVFormatContext **input_ctx, int *video_stream_idx,
                           int *reconnect_attempt, int *reconnect_delay_ms, time_t *last_packet_time);
int handle_running_state(hls_writer_thread_ctx_t *ctx, stream_thread_state_t *thread_state,
                        AVFormatContext *input_ctx, AVPacket *pkt, int video_stream_idx,
                        time_t *last_packet_time, int *reconnect_attempt);
int handle_reconnecting_state(hls_writer_thread_ctx_t *ctx, stream_thread_state_t *thread_state,
                             AVFormatContext **input_ctx, int *video_stream_idx,
                             int *reconnect_attempt, int *reconnect_delay_ms, time_t *last_packet_time);
void handle_stopping_state(hls_writer_thread_ctx_t *ctx, stream_thread_state_t *thread_state);

#endif /* HLS_WRITER_THREAD_INTERNAL_H */
