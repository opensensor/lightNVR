#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <pthread.h>
#include <stdatomic.h>
#include <time.h>
#include <signal.h>
#include <errno.h>
#include <sys/socket.h>
#include <netinet/in.h>
#include <arpa/inet.h>
#include <netdb.h>
#include <sys/time.h>
#include <libavformat/avformat.h>
#include <libavcodec/avcodec.h>
#include <libavutil/avutil.h>
#include <libavutil/time.h>

#include "core/logger.h"
#include "core/config.h"
#include "core/shutdown_coordinator.h"
#include "video/stream_manager.h"
#include "video/stream_state.h"
#include "video/streams.h"
#include "video/hls_writer.h"
#include "video/stream_protocol.h"
#include "video/thread_utils.h"
#include "video/timestamp_manager.h"
#include "video/detection_frame_processing.h"
#include "video/hls/hls_context.h"
#include "video/hls/hls_directory.h"
#include "video/hls/hls_unified_thread.h"

// Maximum time (in seconds) without receiving a packet before considering the connection dead
#define MAX_PACKET_TIMEOUT 5

// Base reconnection delay in milliseconds (500ms)
#define BASE_RECONNECT_DELAY_MS 500

// Maximum reconnection delay in milliseconds (30 seconds)
#define MAX_RECONNECT_DELAY_MS 30000

// Forward declaration for go2rtc integration
extern bool go2rtc_integration_is_using_go2rtc_for_hls(const char *stream_name);
extern bool go2rtc_get_rtsp_url(const char *stream_name, char *url, size_t url_size);

// Hash map for tracking running HLS streaming contexts
// These are defined at the bottom of the file as non-static

/**
 * Safe resource cleanup function
 * This function safely cleans up all FFmpeg and HLS resources, handling NULL pointers and other edge cases
 * Enhanced to prevent memory leaks
 */

// Flag to prevent recursive calls to safe_cleanup_resources
static __thread bool in_safe_cleanup = false;

static void safe_cleanup_resources(AVFormatContext **input_ctx, AVPacket **pkt, hls_writer_t **writer) {
    // CRITICAL FIX: Add safety checks to prevent segmentation faults and bus errors
    // This function is designed to be robust against NULL pointers and partially initialized structures

    // CRITICAL FIX: Prevent recursive calls that can cause double free
    if (in_safe_cleanup) {
        log_warn("Recursive call to safe_cleanup_resources detected, aborting to prevent double free");
        return;
    }
    in_safe_cleanup = true;

    // Clean up packet with safety checks
    if (pkt) {
        // CRITICAL FIX: Check if the pointer to pointer is valid before dereferencing
        AVPacket *pkt_to_free = NULL;

        if (*pkt) {
            // CRITICAL FIX: Create a properly aligned local copy to prevent bus errors on embedded devices
            // Some embedded processors require strict memory alignment
            pkt_to_free = *pkt;
            *pkt = NULL; // Clear the pointer first to prevent double-free

            // CRITICAL FIX: Add memory barrier to ensure memory operations are completed
            // This helps prevent bus errors on some embedded architectures
            __sync_synchronize();

            // Safely unref and free the packet
            log_debug("Safely unreferencing packet during cleanup");

            // CRITICAL FIX: Add additional NULL check before unreferencing
            if (pkt_to_free) {
                av_packet_unref(pkt_to_free);
                log_debug("Safely freeing packet during cleanup");

                // CRITICAL FIX: Add memory barrier before freeing to ensure all accesses are complete
                __sync_synchronize();

                av_packet_free(&pkt_to_free);
            }
        }
    }

    // Clean up input context with safety checks
    if (input_ctx) {
        // CRITICAL FIX: Check if the pointer to pointer is valid before dereferencing
        AVFormatContext *ctx_to_close = NULL;

        if (*input_ctx) {
            ctx_to_close = *input_ctx;
            *input_ctx = NULL; // Clear the pointer first to prevent double-free

            // CRITICAL FIX: Add memory barrier to ensure memory operations are completed
            __sync_synchronize();

            // Safely close the input context
            log_debug("Safely closing input context during cleanup");

            // CRITICAL FIX: Add additional NULL check before closing
            if (ctx_to_close) {
                // Check if the context is properly initialized
                if (ctx_to_close->pb) {
                    avformat_close_input(&ctx_to_close);
                    log_debug("Successfully closed input context");
                } else {
                    // If the context is not properly initialized, just free it
                    // CRITICAL FIX: Add memory barrier before freeing to ensure all accesses are complete
                    __sync_synchronize();
                    avformat_free_context(ctx_to_close);
                }
            }
        }
    }

    // Clean up HLS writer with safety checks
    if (writer) {
        // CRITICAL FIX: Check if the pointer to pointer is valid before dereferencing
        hls_writer_t *writer_to_free = NULL;

        if (*writer) {
            writer_to_free = *writer;
            *writer = NULL; // Clear the pointer first to prevent double-free

            // CRITICAL FIX: Add memory barrier to ensure memory operations are completed
            __sync_synchronize();

            // Safely free the HLS writer
            log_debug("Safely closing HLS writer during cleanup");

            // CRITICAL FIX: Add additional NULL check before closing
            if (writer_to_free) {
                // CRITICAL FIX: Add memory barrier before closing to ensure all accesses are complete
                __sync_synchronize();
                hls_writer_close(writer_to_free);
            }
        }
    }

    log_debug("Completed safe cleanup of resources");

    // Reset the recursive call prevention flag
    in_safe_cleanup = false;
}

/**
 * Calculate reconnection delay with exponential backoff
 *
 * @param attempt The current reconnection attempt (1-based)
 * @return Delay in milliseconds
 */
static int calculate_reconnect_delay(int attempt) {
    if (attempt <= 0) return BASE_RECONNECT_DELAY_MS;

    // Exponential backoff: delay = base_delay * 2^(attempt-1)
    // Cap at maximum delay
    int delay = BASE_RECONNECT_DELAY_MS * (1 << (attempt - 1));
    return (delay < MAX_RECONNECT_DELAY_MS) ? delay : MAX_RECONNECT_DELAY_MS;
}

/**
 * Check RTSP connection by sending an OPTIONS request
 *
 * @param rtsp_url The RTSP URL to check
 * @param host Buffer to store the extracted hostname
 * @param port Pointer to store the extracted port
 * @return 0 on success, negative on error
 */
static int check_rtsp_connection(const char *rtsp_url, char *host, int *port) {
    if (!rtsp_url || !host || !port || strncmp(rtsp_url, "rtsp://", 7) != 0) {
        return -1;
    }

    // Skip the rtsp:// prefix
    const char *host_start = rtsp_url + 7;

    // Skip any authentication info (user:pass@)
    const char *at_sign = strchr(host_start, '@');
    if (at_sign) {
        host_start = at_sign + 1;
    }

    // Find the end of the host part
    const char *host_end = strchr(host_start, ':');
    if (!host_end) {
        host_end = strchr(host_start, '/');
        if (!host_end) {
            host_end = host_start + strlen(host_start);
        }
    }

    // Copy the host part with bounds checking
    size_t host_len = host_end - host_start;
    if (host_len >= 256) {
        host_len = 255;
    }
    if (host_len == 0) {
        // Empty hostname
        return -1;
    }
    memcpy(host, host_start, host_len);
    host[host_len] = '\0';

    // Extract the port if specified
    *port = 554; // Default RTSP port
    if (*host_end == ':') {
        const char *port_start = host_end + 1;
        char *port_end;
        long port_val = strtol(port_start, &port_end, 10);
        if (port_start == port_end || port_val <= 0 || port_val > 65535) {
            *port = 554; // Invalid port, use default
        } else {
            *port = (int)port_val;
        }
    }

    // Create a socket with error handling
    int sock = socket(AF_INET, SOCK_STREAM, 0);
    if (sock < 0) {
        log_error("Failed to create socket for RTSP connection check: %s", strerror(errno));
        return -1;
    }

    // Set a short timeout for the connection
    struct timeval tv;
    tv.tv_sec = 1;
    tv.tv_usec = 0;
    if (setsockopt(sock, SOL_SOCKET, SO_RCVTIMEO, &tv, sizeof(tv)) < 0 ||
        setsockopt(sock, SOL_SOCKET, SO_SNDTIMEO, &tv, sizeof(tv)) < 0) {
        log_warn("Failed to set socket timeout: %s", strerror(errno));
        // Continue anyway
    }

    // Connect to the server
    struct sockaddr_in server_addr;
    memset(&server_addr, 0, sizeof(server_addr));
    server_addr.sin_family = AF_INET;
    server_addr.sin_port = htons(*port);

    // Convert hostname to IP address
    struct hostent *he = gethostbyname(host);
    if (!he) {
        close(sock);
        return -1;
    }

    memcpy(&server_addr.sin_addr, he->h_addr_list[0], he->h_length);

    // Connect to the server
    if (connect(sock, (struct sockaddr*)&server_addr, sizeof(server_addr)) < 0) {
        close(sock);
        return -1;
    }

    // Extract the path part of the URL
    const char *path = strchr(host_start, '/');
    if (!path) {
        path = "/";
    }

    // Send a simple RTSP OPTIONS request
    char request[1024];
    snprintf(request, sizeof(request),
             "OPTIONS %s RTSP/1.0\r\n"
             "CSeq: 1\r\n"
             "User-Agent: LightNVR\r\n"
             "\r\n",
             path);

    if (send(sock, request, strlen(request), 0) < 0) {
        close(sock);
        return -1;
    }

    // Receive the response
    char response[1024] = {0};
    int bytes_received = recv(sock, response, sizeof(response) - 1, 0);
    close(sock);

    if (bytes_received <= 0) {
        return -1;
    }

    // Check if the response contains "404 Not Found"
    if (strstr(response, "404 Not Found") != NULL) {
        return -1;
    }

    return 0;
}

/**
 * Unified HLS thread function
 * This function handles all HLS streaming operations for a single stream
 */
void *hls_unified_thread_func(void *arg) {
    hls_unified_thread_ctx_t *ctx = (hls_unified_thread_ctx_t *)arg;
    AVFormatContext *input_ctx = NULL;
    AVPacket *pkt = NULL;
    int video_stream_idx = -1;
    int ret;
    hls_thread_state_t thread_state = HLS_THREAD_INITIALIZING;
    int reconnect_attempt = 0;
    int reconnect_delay_ms = BASE_RECONNECT_DELAY_MS;
    time_t last_packet_time = 0;

    // Validate context
    if (!ctx) {
        log_error("NULL context passed to unified HLS thread");
        return NULL;
    }

    // Create a local copy of the stream name for thread safety
    char stream_name[MAX_STREAM_NAME];
    strncpy(stream_name, ctx->stream_name, MAX_STREAM_NAME - 1);
    stream_name[MAX_STREAM_NAME - 1] = '\0';

    log_info("Starting unified HLS thread for stream %s", stream_name);

    // Check if we're still running before proceeding
    if (!atomic_load(&ctx->running)) {
        log_warn("Unified HLS thread for %s started but already marked as not running", stream_name);
        return NULL;
    }

    // Get the stream state manager
    stream_state_manager_t *state = get_stream_state_by_name(stream_name);
    if (!state) {
        log_error("Could not find stream state for %s", stream_name);
        atomic_store(&ctx->running, 0);
        return NULL;
    }

    // Register with shutdown coordinator
    char component_name[128];
    snprintf(component_name, sizeof(component_name), "hls_unified_%s", stream_name);
    ctx->shutdown_component_id = register_component(component_name, COMPONENT_HLS_WRITER, ctx, 60);
    if (ctx->shutdown_component_id >= 0) {
        log_info("Registered unified HLS thread %s with shutdown coordinator (ID: %d)",
                stream_name, ctx->shutdown_component_id);
    }

    // Initialize packet
    pkt = av_packet_alloc();
    if (!pkt) {
        log_error("Failed to allocate packet for stream %s", stream_name);
        atomic_store(&ctx->running, 0);

        // Update component state in shutdown coordinator
        if (ctx->shutdown_component_id >= 0) {
            update_component_state(ctx->shutdown_component_id, COMPONENT_STOPPED);
        }

        return NULL;
    }

    // Create HLS writer with appropriate segment duration
    ctx->writer = hls_writer_create(ctx->output_path, stream_name, ctx->segment_duration);
    if (!ctx->writer) {
        log_error("Failed to create HLS writer for %s", stream_name);

        // Clean up resources
        safe_cleanup_resources(NULL, &pkt, NULL);

        atomic_store(&ctx->running, 0);

        // Update component state in shutdown coordinator
        if (ctx->shutdown_component_id >= 0) {
            update_component_state(ctx->shutdown_component_id, COMPONENT_STOPPED);
        }

        return NULL;
    }

    // Store the HLS writer in the stream state for other components to access
    if (state) {
        state->hls_ctx = ctx->writer;
    }

    // Main state machine loop
    while (atomic_load(&ctx->running)) {
        // Update thread state in context
        atomic_store(&ctx->thread_state, thread_state);

        // Check for shutdown conditions
        if (is_shutdown_initiated() || is_stream_state_stopping(state) || !are_stream_callbacks_enabled(state)) {
            log_info("Unified HLS thread for %s stopping due to %s",
                    stream_name,
                    is_shutdown_initiated() ? "system shutdown" :
                    is_stream_state_stopping(state) ? "stream state STOPPING" :
                    "callbacks disabled");
            thread_state = HLS_THREAD_STOPPING;
        }

        // State machine
        switch (thread_state) {
            case HLS_THREAD_INITIALIZING:
                log_info("Initializing unified HLS thread for stream %s", stream_name);
                thread_state = HLS_THREAD_CONNECTING;
                reconnect_attempt = 0;
                break;

            case HLS_THREAD_CONNECTING:
                log_info("Connecting to stream %s (attempt %d)", stream_name, reconnect_attempt + 1);

                // Close any existing connection first
                safe_cleanup_resources(&input_ctx, NULL, NULL);

                // Check if the RTSP URL exists before trying to connect
                if (strncmp(ctx->rtsp_url, "rtsp://", 7) == 0) {
                    char host[256] = {0};
                    int port = 554; // Default RTSP port

                    if (check_rtsp_connection(ctx->rtsp_url, host, &port) < 0) {
                        log_error("Failed to connect to RTSP server: %s:%d", host, port);

                        // Mark connection as invalid
                        atomic_store(&ctx->connection_valid, 0);

                        // Increment reconnection attempt counter
                        reconnect_attempt++;

                        // Calculate reconnection delay with exponential backoff
                        reconnect_delay_ms = calculate_reconnect_delay(reconnect_attempt);

                        log_info("Will retry connection to stream %s in %d ms (attempt %d)",
                                stream_name, reconnect_delay_ms, reconnect_attempt + 1);

                        // Sleep before retrying
                        av_usleep(reconnect_delay_ms * 1000);

                        // Stay in CONNECTING state and try again
                        break;
                    }
                }

                // Open input stream
                input_ctx = NULL;
                ret = open_input_stream(&input_ctx, ctx->rtsp_url, ctx->protocol);
                if (ret < 0) {
                    char error_buf[AV_ERROR_MAX_STRING_SIZE] = {0};
                    av_strerror(ret, error_buf, AV_ERROR_MAX_STRING_SIZE);

                    // Log the error but don't treat any error type specially
                    log_error("Failed to connect to stream %s: %s (error code: %d)",
                             stream_name, error_buf, ret);

                    // Ensure input_ctx is NULL after a failed open
                    if (input_ctx) {
                        avformat_close_input(&input_ctx);
                        input_ctx = NULL;
                    }

                    // Mark connection as invalid
                    atomic_store(&ctx->connection_valid, 0);

                    // Increment reconnection attempt counter
                    reconnect_attempt++;

                    // Calculate reconnection delay with exponential backoff
                    reconnect_delay_ms = calculate_reconnect_delay(reconnect_attempt);

                    // Cap reconnection attempts to avoid integer overflow
                    if (reconnect_attempt > 1000) {
                        reconnect_attempt = 1000;
                    }

                    log_info("Will retry connection to stream %s in %d ms (attempt %d)",
                            stream_name, reconnect_delay_ms, reconnect_attempt + 1);

                    // Sleep before retrying
                    av_usleep(reconnect_delay_ms * 1000);

                    // Stay in CONNECTING state and try again
                    break;
                }

                // Find video stream
                video_stream_idx = find_video_stream_index(input_ctx);
                if (video_stream_idx == -1) {
                    log_error("No video stream found in %s", ctx->rtsp_url);

                    // Close input context
                    avformat_close_input(&input_ctx);

                    // Mark connection as invalid
                    atomic_store(&ctx->connection_valid, 0);

                    // Increment reconnection attempt counter
                    reconnect_attempt++;

                    // Calculate reconnection delay with exponential backoff
                    reconnect_delay_ms = calculate_reconnect_delay(reconnect_attempt);

                    // Cap reconnection attempts to avoid integer overflow
                    if (reconnect_attempt > 1000) {
                        reconnect_attempt = 1000;
                    }

                    log_info("Will retry connection to stream %s in %d ms (attempt %d)",
                            stream_name, reconnect_delay_ms, reconnect_attempt + 1);

                    // Sleep before retrying
                    av_usleep(reconnect_delay_ms * 1000);

                    // Stay in CONNECTING state and try again
                    break;
                }

                // Log information about all streams in the input
                log_info("Stream %s has %d streams:", stream_name, input_ctx->nb_streams);
                for (unsigned int i = 0; i < input_ctx->nb_streams; i++) {
                    AVStream *stream = input_ctx->streams[i];
                    AVCodecParameters *codecpar = stream->codecpar;

                    if (codecpar->codec_type == AVMEDIA_TYPE_VIDEO) {
                        log_info("Stream %d: Video stream detected (codec: %d, width: %d, height: %d)",
                                i, codecpar->codec_id, codecpar->width, codecpar->height);
                    } else if (codecpar->codec_type == AVMEDIA_TYPE_AUDIO) {
                        log_info("Stream %d: Audio stream detected (codec: %d, channels: %d, sample_rate: %d)",
                                i, codecpar->codec_id, codecpar->ch_layout.nb_channels, codecpar->sample_rate);
                    } else {
                        log_info("Stream %d: Other stream type detected (codec_type: %d)",
                                i, codecpar->codec_type);
                    }
                }

                // Initialize HLS writer with stream information
                if (input_ctx->streams[video_stream_idx]) {
                    ret = hls_writer_initialize(ctx->writer, input_ctx->streams[video_stream_idx]);
                    if (ret < 0) {
                        log_error("Failed to initialize HLS writer for stream %s", stream_name);

                        // Close input context
                        avformat_close_input(&input_ctx);

                        // Mark connection as invalid
                        atomic_store(&ctx->connection_valid, 0);

                        // Increment reconnection attempt counter
                        reconnect_attempt++;

                        // Calculate reconnection delay with exponential backoff
                        reconnect_delay_ms = calculate_reconnect_delay(reconnect_attempt);

                        log_info("Will retry connection to stream %s in %d ms (attempt %d)",
                                stream_name, reconnect_delay_ms, reconnect_attempt + 1);

                        // Sleep before retrying
                        av_usleep(reconnect_delay_ms * 1000);

                        // Stay in CONNECTING state and try again
                        break;
                    }
                }

                // Connection successful
                log_info("Successfully connected to stream %s", stream_name);
                thread_state = HLS_THREAD_RUNNING;
                reconnect_attempt = 0;
                atomic_store(&ctx->connection_valid, 1);
                atomic_store(&ctx->consecutive_failures, 0);
                last_packet_time = time(NULL);
                atomic_store(&ctx->last_packet_time, (int_fast64_t)last_packet_time);
                break;

            case HLS_THREAD_RUNNING:
                // Check if we should exit before potentially blocking on av_read_frame
                if (!atomic_load(&ctx->running)) {
                    log_info("Unified HLS thread for %s detected shutdown before read", stream_name);
                    thread_state = HLS_THREAD_STOPPING;
                    break;
                }

                // Read packet
                ret = av_read_frame(input_ctx, pkt);

                if (ret < 0) {
                    // Handle read errors
                    char error_buf[AV_ERROR_MAX_STRING_SIZE] = {0};
                    av_strerror(ret, error_buf, AV_ERROR_MAX_STRING_SIZE);

                    log_error("Error reading from stream %s: %s (code: %d)",
                             stream_name, error_buf, ret);

                    // Unref packet before reconnecting
                    av_packet_unref(pkt);

                    // Transition to reconnecting state
                    thread_state = HLS_THREAD_RECONNECTING;
                    reconnect_attempt = 1;
                    atomic_store(&ctx->connection_valid, 0);
                    atomic_fetch_add(&ctx->consecutive_failures, 1);
                    break;
                }

                // Get the stream for this packet
                AVStream *input_stream = NULL;
                if (pkt->stream_index >= 0 && pkt->stream_index < input_ctx->nb_streams) {
                    input_stream = input_ctx->streams[pkt->stream_index];
                } else {
                    log_warn("Invalid stream index %d for stream %s", pkt->stream_index, stream_name);
                    av_packet_unref(pkt);
                    continue;
                }

                // Validate packet data
                if (!pkt->data || pkt->size <= 0) {
                    log_warn("Invalid packet (null data or zero size) for stream %s", stream_name);
                    av_packet_unref(pkt);
                    continue;
                }

                // Process packets based on stream type
                if (pkt->stream_index == video_stream_idx) {
                    // This is a video packet - process it
                    // Lock the writer mutex
                    pthread_mutex_lock(&ctx->writer->mutex);

                    ret = hls_writer_write_packet(ctx->writer, pkt, input_stream);

                    // Log key frames for debugging
                    bool is_key_frame = (pkt->flags & AV_PKT_FLAG_KEY) != 0;
                    if (is_key_frame && ret >= 0) {
                        log_debug("Processed video key frame for stream %s", stream_name);
                    }

                    // Handle write errors
                    if (ret < 0) {
                        char error_buf[AV_ERROR_MAX_STRING_SIZE] = {0};
                        av_strerror(ret, error_buf, AV_ERROR_MAX_STRING_SIZE);
                        log_warn("Error writing video packet to HLS for stream %s: %s", stream_name, error_buf);
                    } else {
                        // Successfully processed a packet
                        last_packet_time = time(NULL);
                        atomic_store(&ctx->last_packet_time, (int_fast64_t)last_packet_time);
                        atomic_store(&ctx->consecutive_failures, 0);
                        atomic_store(&ctx->connection_valid, 1);
                    }

                    // Unlock the writer mutex
                    pthread_mutex_unlock(&ctx->writer->mutex);
                } else {
                    // This is a non-video packet (likely audio)
                    // For now, we'll just log it and skip processing
                    // This prevents the "Invalid packet stream index" errors
                    if (pkt->stream_index >= 0 && pkt->stream_index < input_ctx->nb_streams) {
                        AVStream *stream = input_ctx->streams[pkt->stream_index];
                        AVCodecParameters *codecpar = stream->codecpar;

                        if (codecpar->codec_type == AVMEDIA_TYPE_AUDIO) {
                            log_debug("Skipping audio packet for stream %s (stream index: %d)",
                                    stream_name, pkt->stream_index);
                        } else {
                            log_debug("Skipping non-video packet for stream %s (stream index: %d, type: %d)",
                                    stream_name, pkt->stream_index, codecpar->codec_type);
                        }
                    } else {
                        log_warn("Skipping packet with invalid stream index %d for stream %s",
                                pkt->stream_index, stream_name);
                    }
                }

                // Unref packet
                av_packet_unref(pkt);

                // Check if we haven't received a packet in a while
                time_t now = time(NULL);
                if (now - last_packet_time > MAX_PACKET_TIMEOUT) {
                    log_error("No packets received from stream %s for %ld seconds, reconnecting",
                             stream_name, now - last_packet_time);
                    thread_state = HLS_THREAD_RECONNECTING;
                    reconnect_attempt = 1;
                    atomic_store(&ctx->connection_valid, 0);
                    atomic_fetch_add(&ctx->consecutive_failures, 1);
                }
                break;

            case HLS_THREAD_RECONNECTING:
                log_info("Reconnecting to stream %s (attempt %d)", stream_name, reconnect_attempt);

                // Close existing connection
                safe_cleanup_resources(&input_ctx, NULL, NULL);

                // Calculate reconnection delay with exponential backoff
                reconnect_delay_ms = calculate_reconnect_delay(reconnect_attempt);

                // Sleep before reconnecting
                log_info("Waiting %d ms before reconnecting to stream %s", reconnect_delay_ms, stream_name);
                av_usleep(reconnect_delay_ms * 1000);

                // Check if we should stop during the sleep
                if (!atomic_load(&ctx->running)) {
                    log_info("Unified HLS thread for %s stopping during reconnection", stream_name);
                    thread_state = HLS_THREAD_STOPPING;
                    break;
                }

                // Open input stream
                input_ctx = NULL;
                ret = open_input_stream(&input_ctx, ctx->rtsp_url, ctx->protocol);
                if (ret < 0) {
                    char error_buf[AV_ERROR_MAX_STRING_SIZE] = {0};
                    av_strerror(ret, error_buf, AV_ERROR_MAX_STRING_SIZE);

                    // Log the error but don't treat any error type specially
                    log_error("Failed to reconnect to stream %s: %s (error code: %d)",
                             stream_name, error_buf, ret);

                    // Ensure input_ctx is NULL after a failed open
                    if (input_ctx) {
                        avformat_close_input(&input_ctx);
                        input_ctx = NULL;
                    }

                    // Increment reconnection attempt counter
                    reconnect_attempt++;

                    // Cap reconnection attempts to avoid integer overflow
                    if (reconnect_attempt > 1000) {
                        reconnect_attempt = 1000;
                    }

                    // Stay in reconnecting state
                    break;
                }

                // Find video stream
                video_stream_idx = find_video_stream_index(input_ctx);
                if (video_stream_idx == -1) {
                    log_error("No video stream found in %s during reconnection", ctx->rtsp_url);

                    // Close input context
                    avformat_close_input(&input_ctx);

                    // Increment reconnection attempt counter
                    reconnect_attempt++;

                    // Cap reconnection attempts to avoid integer overflow
                    if (reconnect_attempt > 1000) {
                        reconnect_attempt = 1000;
                    }

                    // Stay in reconnecting state
                    break;
                }

                // Log information about all streams in the input during reconnection
                log_info("Stream %s has %d streams during reconnection:", stream_name, input_ctx->nb_streams);
                for (unsigned int i = 0; i < input_ctx->nb_streams; i++) {
                    AVStream *stream = input_ctx->streams[i];
                    AVCodecParameters *codecpar = stream->codecpar;

                    if (codecpar->codec_type == AVMEDIA_TYPE_VIDEO) {
                        log_info("Stream %d: Video stream detected (codec: %d, width: %d, height: %d)",
                                i, codecpar->codec_id, codecpar->width, codecpar->height);
                    } else if (codecpar->codec_type == AVMEDIA_TYPE_AUDIO) {
                        log_info("Stream %d: Audio stream detected (codec: %d, channels: %d, sample_rate: %d)",
                                i, codecpar->codec_id, codecpar->ch_layout.nb_channels, codecpar->sample_rate);
                    } else {
                        log_info("Stream %d: Other stream type detected (codec_type: %d)",
                                i, codecpar->codec_type);
                    }
                }

                // CRITICAL FIX: Add additional validation before considering reconnection successful
                // Verify that the input context is valid and has streams
                if (!input_ctx || input_ctx->nb_streams == 0) {
                    log_error("Reconnection to stream %s failed: Invalid input context or no streams",
                            stream_name);

                    // Clean up any partially initialized resources
                    if (input_ctx) {
                        log_warn("Cleaning up invalid input context for stream %s", stream_name);
                        safe_cleanup_resources(&input_ctx, NULL, NULL);
                    }

                    // Increment reconnection attempt counter
                    reconnect_attempt++;

                    // Cap reconnection attempts to avoid integer overflow
                    if (reconnect_attempt > 1000) {
                        reconnect_attempt = 1000;
                    }

                    // Stay in reconnecting state
                    break;
                }

                // Reconnection successful
                log_info("Successfully reconnected to stream %s after %d attempts",
                        stream_name, reconnect_attempt);
                thread_state = HLS_THREAD_RUNNING;
                reconnect_attempt = 0;
                atomic_store(&ctx->connection_valid, 1);
                atomic_store(&ctx->consecutive_failures, 0);
                last_packet_time = time(NULL);
                atomic_store(&ctx->last_packet_time, (int_fast64_t)last_packet_time);
                break;

            case HLS_THREAD_STOPPING:
                log_info("Stopping unified HLS thread for stream %s", stream_name);
                atomic_store(&ctx->running, 0);
                atomic_store(&ctx->connection_valid, 0);
                thread_state = HLS_THREAD_STOPPED;
                break;

            case HLS_THREAD_STOPPED:
                // We should never reach this state in the loop
                log_warn("Unified HLS thread for %s in STOPPED state but still in main loop", stream_name);
                atomic_store(&ctx->running, 0);
                break;

            default:
                // Unknown state
                log_error("Unified HLS thread for %s in unknown state: %d", stream_name, thread_state);
                atomic_store(&ctx->running, 0);
                break;
        }
    }

    // CRITICAL FIX: Add safety checks before accessing ctx members
    if (ctx) {
        // Mark connection as invalid
        atomic_store(&ctx->connection_valid, 0);
    }

    // Clear the reference in the stream state
    // CRITICAL FIX: Add additional safety checks to prevent segfault
    if (state && ctx && ctx->writer && state->hls_ctx == ctx->writer) {
        state->hls_ctx = NULL;
    }

    // CRITICAL FIX: Store stream name in local buffer before cleanup
    char stream_name_buf[MAX_STREAM_NAME];
    strncpy(stream_name_buf, stream_name, MAX_STREAM_NAME - 1);
    stream_name_buf[MAX_STREAM_NAME - 1] = '\0';

    // CRITICAL FIX: Add safety checks before cleaning up resources
    log_info("Cleaning up all resources for stream %s", stream_name_buf);

    // Verify that the resources are valid before cleaning them up
    if (input_ctx) {
        log_debug("Cleaning up input context for stream %s", stream_name_buf);
    }

    if (pkt) {
        log_debug("Cleaning up packet for stream %s", stream_name_buf);
    }

    // CRITICAL FIX: Check ctx before accessing its members
    hls_writer_t *writer_to_cleanup = NULL;
    int shutdown_id = -1;

    if (ctx) {
        if (ctx->writer) {
            log_debug("Cleaning up HLS writer for stream %s", stream_name_buf);
            writer_to_cleanup = ctx->writer;

            // CRITICAL FIX: Store a local copy of the stream name for logging
            char writer_stream_name[MAX_STREAM_NAME] = {0};
            if (ctx->writer->stream_name) {
                strncpy(writer_stream_name, ctx->writer->stream_name, MAX_STREAM_NAME - 1);
                writer_stream_name[MAX_STREAM_NAME - 1] = '\0';
                log_info("Preparing to clean up HLS writer for stream %s", writer_stream_name);
            }
        }
        shutdown_id = ctx->shutdown_component_id;
    }

    // CRITICAL FIX: Add memory barrier before cleanup to ensure all threads see consistent state
    __sync_synchronize();

    // Make local copies of pointers to prevent race conditions during cleanup
    AVFormatContext *input_ctx_local = input_ctx;
    AVPacket *pkt_local = pkt;

    // Clear original pointers immediately to prevent double-free
    input_ctx = NULL;
    pkt = NULL;

    // CRITICAL FIX: Clear the writer reference in the context before cleaning up
    if (ctx) {
        log_info("Clearing writer reference in context for stream %s", stream_name_buf);
        ctx->writer = NULL;
    }

    // Clean up all resources using local copies
    safe_cleanup_resources(&input_ctx_local, &pkt_local, &writer_to_cleanup);

    // Update component state in shutdown coordinator only if we have a valid ID
    if (shutdown_id >= 0) {
        update_component_state(shutdown_id, COMPONENT_STOPPED);
        log_info("Updated unified HLS thread %s state to STOPPED in shutdown coordinator", stream_name_buf);
    }

    // Unmark the stream as stopping to indicate we've completed our shutdown
    unmark_stream_stopping(stream_name_buf);
    log_info("Unmarked stream %s as stopping before thread exit", stream_name_buf);

    // Free the context if it exists
    free(ctx);
    ctx = NULL;

    log_info("Unified HLS thread for stream %s exited", stream_name_buf);
    return NULL;
}

// Define the mutex and contexts array as non-static so they can be accessed from other files
pthread_mutex_t unified_contexts_mutex = PTHREAD_MUTEX_INITIALIZER;
hls_unified_thread_ctx_t *unified_contexts[MAX_STREAMS];

/**
 * Start HLS streaming for a stream using the unified thread approach
 * This is the implementation that will be called by the API functions
 */
int start_hls_unified_stream(const char *stream_name) {
    stream_handle_t stream = get_stream_by_name(stream_name);
    if (!stream) {
        log_error("Stream %s not found", stream_name);
        return -1;
    }

    stream_config_t config;
    if (get_stream_config(stream, &config) != 0) {
        log_error("Failed to get config for stream %s", stream_name);
        return -1;
    }

    // Get the stream state manager
    stream_state_manager_t *state = get_stream_state_by_name(stream_name);
    if (!state) {
        log_error("Stream state not found for %s", stream_name);
        return -1;
    }

    // Check if the stream is in the process of being stopped
    if (is_stream_state_stopping(state)) {
        log_warn("Cannot start HLS stream %s while it is in the process of being stopped", stream_name);
        return -1;
    }

    // Add a reference for the HLS component
    stream_state_add_ref(state, STREAM_COMPONENT_HLS);
    log_info("Added HLS reference to stream %s", stream_name);

    // Check if already running
    pthread_mutex_lock(&unified_contexts_mutex);
    for (int i = 0; i < MAX_STREAMS; i++) {
        if (unified_contexts[i] && strcmp(unified_contexts[i]->stream_name, stream_name) == 0) {
            pthread_mutex_unlock(&unified_contexts_mutex);
            log_info("HLS stream %s already running", stream_name);
            return 0;  // Already running
        }
    }

    // Find empty slot
    int slot = -1;
    for (int i = 0; i < MAX_STREAMS; i++) {
        if (!unified_contexts[i]) {
            slot = i;
            break;
        }
    }

    if (slot == -1) {
        pthread_mutex_unlock(&unified_contexts_mutex);
        log_error("No slot available for new HLS stream");
        return -1;
    }
    pthread_mutex_unlock(&unified_contexts_mutex);

    // Clear any existing HLS segments for this stream
    log_info("Clearing any existing HLS segments for stream %s before starting", stream_name);
    clear_stream_hls_segments(stream_name);

    // Create context
    hls_unified_thread_ctx_t *ctx = malloc(sizeof(hls_unified_thread_ctx_t));
    if (!ctx) {
        log_error("Memory allocation failed for unified HLS context");
        return -1;
    }

    memset(ctx, 0, sizeof(hls_unified_thread_ctx_t));
    strncpy(ctx->stream_name, stream_name, MAX_STREAM_NAME - 1);
    ctx->stream_name[MAX_STREAM_NAME - 1] = '\0';

    // Get RTSP URL
    char actual_url[MAX_PATH_LENGTH];
    strncpy(actual_url, config.url, sizeof(actual_url) - 1);
    actual_url[sizeof(actual_url) - 1] = '\0';

    // If the stream is using go2rtc for HLS, get the go2rtc RTSP URL
    if (go2rtc_integration_is_using_go2rtc_for_hls(stream_name)) {
        // Get the go2rtc RTSP URL with retry logic
        int rtsp_retries = 3;
        bool rtsp_url_success = false;

        while (rtsp_retries > 0 && !rtsp_url_success) {
            if (go2rtc_get_rtsp_url(stream_name, actual_url, sizeof(actual_url))) {
                log_info("Using go2rtc RTSP URL for HLS streaming: %s", actual_url);
                rtsp_url_success = true;
            } else {
                log_warn("Failed to get go2rtc RTSP URL for stream %s (retries left: %d)",
                        stream_name, rtsp_retries - 1);
                rtsp_retries--;

                if (rtsp_retries > 0) {
                    log_info("Waiting before retrying to get RTSP URL for stream %s...", stream_name);
                    sleep(3); // Wait 3 seconds before retrying
                }
            }
        }

        if (!rtsp_url_success) {
            log_warn("Failed to get go2rtc RTSP URL for stream %s after multiple attempts, falling back to original URL", stream_name);
        }
    }

    // Copy the URL to the context
    strncpy(ctx->rtsp_url, actual_url, MAX_PATH_LENGTH - 1);
    ctx->rtsp_url[MAX_PATH_LENGTH - 1] = '\0';

    // Set protocol information in the context
    ctx->protocol = config.protocol;

    // Set segment duration
    ctx->segment_duration = config.segment_duration > 0 ? config.segment_duration : 2;

    // Create output paths
    config_t *global_config = get_streaming_config();

    // Use storage_path_hls if specified, otherwise fall back to storage_path
    const char *base_storage_path = global_config->storage_path;
    if (global_config->storage_path_hls[0] != '\0') {
        base_storage_path = global_config->storage_path_hls;
        log_info("Using dedicated HLS storage path: %s", base_storage_path);
    } else {
        log_info("Using default storage path for HLS: %s", base_storage_path);
    }

    // Create HLS output path
    snprintf(ctx->output_path, MAX_PATH_LENGTH, "%s/hls/%s",
             base_storage_path, stream_name);

    // Create HLS directory if it doesn't exist
    char dir_cmd[MAX_PATH_LENGTH * 2];
    snprintf(dir_cmd, sizeof(dir_cmd), "mkdir -p %s", ctx->output_path);
    int ret = system(dir_cmd);
    if (ret != 0) {
        log_error("Failed to create HLS directory: %s (return code: %d)", ctx->output_path, ret);
        free(ctx);
        return -1;
    }

    // Set full permissions to ensure FFmpeg can write files
    snprintf(dir_cmd, sizeof(dir_cmd), "chmod -R 777 %s", ctx->output_path);
    if (system(dir_cmd) != 0) {
        log_warn("Failed to set permissions on HLS directory: %s", ctx->output_path);
    }

    // Also ensure the parent directory of the HLS directory exists and is writable
    char parent_dir[MAX_PATH_LENGTH];
    snprintf(parent_dir, sizeof(parent_dir), "%s/hls", global_config->storage_path);
    snprintf(dir_cmd, sizeof(dir_cmd), "mkdir -p %s && chmod -R 777 %s",
             parent_dir, parent_dir);
    if (system(dir_cmd) != 0) {
        log_warn("Failed to create or set permissions on parent HLS directory: %s", parent_dir);
    }

    log_info("Created HLS directory with full permissions: %s", ctx->output_path);

    // Check that we can actually write to this directory
    char test_file[MAX_PATH_LENGTH];
    snprintf(test_file, sizeof(test_file), "%s/.test_write", ctx->output_path);
    FILE *test = fopen(test_file, "w");
    if (!test) {
        log_error("Directory is not writable: %s (error: %s)", ctx->output_path, strerror(errno));
        free(ctx);
        return -1;
    }
    fclose(test);
    remove(test_file);
    log_info("Verified HLS directory is writable: %s", ctx->output_path);

    // Initialize thread state
    atomic_store(&ctx->running, 1);
    atomic_store(&ctx->connection_valid, 0);
    atomic_store(&ctx->consecutive_failures, 0);
    atomic_store(&ctx->last_packet_time, (int_fast64_t)time(NULL));
    atomic_store(&ctx->thread_state, HLS_THREAD_INITIALIZING);

    // Set up thread attributes to create a detached thread
    pthread_attr_t attr;
    pthread_attr_init(&attr);
    pthread_attr_setdetachstate(&attr, PTHREAD_CREATE_DETACHED);

    // Start thread with detached state
    int thread_result = pthread_create(&ctx->thread, &attr, hls_unified_thread_func, ctx);

    // Clean up thread attributes
    pthread_attr_destroy(&attr);

    if (thread_result != 0) {
        log_error("Failed to create unified HLS thread for %s", stream_name);
        free(ctx);
        return -1;
    }

    // Store context in the global array
    pthread_mutex_lock(&unified_contexts_mutex);
    unified_contexts[slot] = ctx;
    pthread_mutex_unlock(&unified_contexts_mutex);

    log_info("Started unified HLS thread for %s in slot %d", stream_name, slot);
    return 0;
}

/**
 * Force restart of HLS streaming for a stream using the unified thread approach
 */
int restart_hls_unified_stream(const char *stream_name) {
    log_info("Force restarting HLS stream for %s", stream_name);

    // Clear the HLS segments before stopping the stream
    log_info("Clearing HLS segments for stream %s before restart", stream_name);
    clear_stream_hls_segments(stream_name);

    // First stop the stream if it's running
    int stop_result = stop_hls_stream(stream_name);
    if (stop_result != 0) {
        log_warn("Failed to stop HLS stream %s for restart, continuing anyway", stream_name);
    }

    // Wait a bit to ensure resources are released
    usleep(500000); // 500ms

    // Verify that the HLS directory exists and is writable
    config_t *global_config = get_streaming_config();
    if (global_config) {
        // Use storage_path_hls if specified, otherwise fall back to storage_path
        const char *base_storage_path = global_config->storage_path;
        if (global_config->storage_path_hls[0] != '\0') {
            base_storage_path = global_config->storage_path_hls;
            log_info("Using dedicated HLS storage path for restart: %s", base_storage_path);
        }

        char hls_dir[MAX_PATH_LENGTH];
        snprintf(hls_dir, MAX_PATH_LENGTH, "%s/hls/%s",
                base_storage_path, stream_name);

        // Ensure the directory exists and has proper permissions
        log_info("Ensuring HLS directory exists and is writable: %s", hls_dir);
        ensure_hls_directory(hls_dir, stream_name);
    }

    // Start the stream again
    int start_result = start_hls_stream(stream_name);
    if (start_result != 0) {
        log_error("Failed to restart HLS stream %s", stream_name);
        return -1;
    }

    log_info("Successfully restarted HLS stream %s", stream_name);
    return 0;
}

/**
 * Stop HLS streaming for a stream using the unified thread approach
 */
int stop_hls_unified_stream(const char *stream_name) {
    int found = 0;

    // Log that we're attempting to stop the stream
    log_info("Attempting to stop HLS stream: %s", stream_name);

    // Get the stream state manager
    stream_state_manager_t *state = get_stream_state_by_name(stream_name);
    if (state) {
        // Only disable callbacks if we're actually stopping the stream
        bool found = false;
        pthread_mutex_lock(&unified_contexts_mutex);
        for (int i = 0; i < MAX_STREAMS; i++) {
            if (unified_contexts[i] && strcmp(unified_contexts[i]->stream_name, stream_name) == 0) {
                found = true;
                break;
            }
        }
        pthread_mutex_unlock(&unified_contexts_mutex);

        if (found) {
            // Disable callbacks to prevent new packets from being processed
            set_stream_callbacks_enabled(state, false);
            log_info("Disabled callbacks for stream %s during HLS shutdown", stream_name);
        } else {
            log_info("Stream %s not found in HLS contexts, not disabling callbacks", stream_name);
        }
    }

    // Find the stream context with mutex protection
    pthread_mutex_lock(&unified_contexts_mutex);

    hls_unified_thread_ctx_t *ctx = NULL;
    int index = -1;

    for (int i = 0; i < MAX_STREAMS; i++) {
        if (unified_contexts[i] && strcmp(unified_contexts[i]->stream_name, stream_name) == 0) {
            ctx = unified_contexts[i];
            index = i;
            found = 1;
            break;
        }
    }

    // If not found, unlock and return
    if (!found) {
        pthread_mutex_unlock(&unified_contexts_mutex);
        log_warn("HLS stream %s not found for stopping", stream_name);
        return -1;
    }

    // Check if the stream is already stopped
    if (!atomic_load(&ctx->running)) {
        pthread_mutex_unlock(&unified_contexts_mutex);
        log_warn("HLS stream %s is already stopped", stream_name);
        return 0;
    }

    // Mark as stopping in the global stopping list to prevent race conditions
    mark_stream_stopping(stream_name);

    // Now mark as not running using atomic store for thread safety
    atomic_store(&ctx->running, 0);
    log_info("Marked HLS stream %s as stopping (index: %d)", stream_name, index);

    // Reset the timestamp tracker for this stream to ensure clean state when restarted
    reset_timestamp_tracker(stream_name);
    log_info("Reset timestamp tracker for stream %s", stream_name);

    // Unlock the mutex to allow the thread to access shared resources during shutdown
    pthread_mutex_unlock(&unified_contexts_mutex);

    // Wait for the thread to begin its shutdown process
    usleep(100000); // 100ms

    log_info("Waiting for detached thread for stream %s to exit on its own", stream_name);

    // Wait longer to ensure the thread has fully exited
    int wait_attempts = 10; // Try up to 10 times
    while (wait_attempts > 0) {
        // Check if the thread has marked itself as exiting
        if (is_stream_stopping(stream_name)) {
            // Thread is still in the process of stopping
            log_info("Thread for stream %s is still stopping, waiting... (%d attempts left)",
                    stream_name, wait_attempts);
            usleep(500000); // Wait 500ms between checks
            wait_attempts--;
        } else {
            // Thread has completed its shutdown
            log_info("Thread for stream %s has completed its shutdown", stream_name);
            break;
        }
    }

    // Re-acquire the mutex for cleanup
    pthread_mutex_lock(&unified_contexts_mutex);

    // Verify context is still valid
    if (index >= 0 && index < MAX_STREAMS && unified_contexts[index] == ctx) {
        // CRITICAL FIX: Clear the writer reference in the context before freeing to prevent double free
        if (ctx && ctx->writer) {
            // Store a local copy of the stream name for logging
            char writer_stream_name[MAX_STREAM_NAME] = {0};
            if (ctx->writer->stream_name) {
                strncpy(writer_stream_name, ctx->writer->stream_name, MAX_STREAM_NAME - 1);
                writer_stream_name[MAX_STREAM_NAME - 1] = '\0';
            } else {
                strcpy(writer_stream_name, "unknown");
            }

            log_info("Clearing writer reference in context for stream %s", writer_stream_name);
            ctx->writer = NULL;
        }

        // Free context and clear slot
        unified_contexts[index] = NULL;

        // Unlock the mutex before freeing the context
        pthread_mutex_unlock(&unified_contexts_mutex);

        // Free the context structure
        free(ctx);
        ctx = NULL;

        log_info("Successfully cleaned up resources for stream %s", stream_name);
    } else {
        pthread_mutex_unlock(&unified_contexts_mutex);
        log_warn("Context for stream %s was modified during cleanup", stream_name);
    }

    // Remove from the stopping list
    unmark_stream_stopping(stream_name);

    log_info("Stopped HLS stream %s", stream_name);

    // Release the HLS reference
    if (state) {
        // Re-enable callbacks before releasing the reference
        set_stream_callbacks_enabled(state, true);
        log_info("Re-enabled callbacks for stream %s after HLS shutdown", stream_name);

        stream_state_release_ref(state, STREAM_COMPONENT_HLS);
        log_info("Released HLS reference to stream %s", stream_name);
    }

    return 0;
}

/**
 * Check if HLS streaming is active for a stream
 */
int is_hls_stream_active(const char *stream_name) {
    pthread_mutex_lock(&unified_contexts_mutex);

    for (int i = 0; i < MAX_STREAMS; i++) {
        if (unified_contexts[i] &&
            strcmp(unified_contexts[i]->stream_name, stream_name) == 0 &&
            atomic_load(&unified_contexts[i]->running) &&
            atomic_load(&unified_contexts[i]->connection_valid)) {

            pthread_mutex_unlock(&unified_contexts_mutex);
            return 1;
        }
    }

    pthread_mutex_unlock(&unified_contexts_mutex);
    return 0;
}
