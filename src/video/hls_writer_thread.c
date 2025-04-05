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
#include "core/shutdown_coordinator.h"
#include "video/hls_writer.h"
#include "video/hls_writer_thread.h"
#include "video/stream_protocol.h"
#include "video/stream_state.h"
#include "video/streams.h"
#include "video/thread_utils.h"
#include "video/detection_frame_processing.h"

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

/**
 * Safe resource cleanup function
 * This function safely cleans up resources, handling NULL pointers and other edge cases
 */
static void safe_cleanup_resources(AVFormatContext **input_ctx, AVPacket **pkt) {
    // Clean up packet
    if (pkt && *pkt) {
        AVPacket *pkt_to_free = *pkt;
        *pkt = NULL; // Clear the pointer first to prevent double-free

        // Safely unref and free the packet
        av_packet_unref(pkt_to_free);
        av_packet_free(&pkt_to_free);
    }

    // Clean up input context
    if (input_ctx && *input_ctx) {
        AVFormatContext *ctx_to_close = *input_ctx;
        *input_ctx = NULL; // Clear the pointer first to prevent double-free

        // Safely close the input context
        avformat_close_input(&ctx_to_close);
    }
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
 * HLS writer thread function - simplified and more robust implementation
 * with indefinite graceful retries for all error types
 */
static void *hls_writer_thread_func(void *arg) {
    hls_writer_thread_ctx_t *ctx = (hls_writer_thread_ctx_t *)arg;
    AVFormatContext *input_ctx = NULL;
    AVPacket *pkt = NULL;
    int video_stream_idx = -1;
    int ret;
    stream_thread_state_t thread_state = STREAM_THREAD_INITIALIZING;
    int reconnect_attempt = 0;
    int reconnect_delay_ms = BASE_RECONNECT_DELAY_MS;
    time_t last_packet_time = 0;

    // Validate context
    if (!ctx) {
        log_error("NULL context passed to HLS writer thread");
        return NULL;
    }

    // Create a local copy of the stream name for thread safety
    char stream_name[MAX_STREAM_NAME];
    strncpy(stream_name, ctx->stream_name, MAX_STREAM_NAME - 1);
    stream_name[MAX_STREAM_NAME - 1] = '\0';

    log_info("Starting HLS writer thread for stream %s", stream_name);

    // Check if we're still running before proceeding
    if (!atomic_load(&ctx->running)) {
        log_warn("HLS writer thread for %s started but already marked as not running", stream_name);
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
    snprintf(component_name, sizeof(component_name), "hls_writer_thread_%s", stream_name);
    ctx->shutdown_component_id = register_component(component_name, COMPONENT_HLS_WRITER, ctx, 60);
    if (ctx->shutdown_component_id >= 0) {
        log_info("Registered HLS writer thread %s with shutdown coordinator (ID: %d)",
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

    // Main state machine loop
    while (atomic_load(&ctx->running)) {
        // Check for shutdown conditions
        if (is_shutdown_initiated() || is_stream_state_stopping(state)) {
            log_info("HLS writer thread for %s stopping due to %s",
                    stream_name, is_shutdown_initiated() ? "system shutdown" : "stream state STOPPING");
            thread_state = STREAM_THREAD_STOPPING;
            break;
        }

        // State machine
        switch (thread_state) {
            case STREAM_THREAD_INITIALIZING:
                log_info("Initializing HLS writer thread for stream %s", stream_name);
                thread_state = STREAM_THREAD_CONNECTING;
                reconnect_attempt = 0;
                break;

            case STREAM_THREAD_CONNECTING:
                log_info("Connecting to stream %s (attempt %d)", stream_name, reconnect_attempt + 1);

                // Close any existing connection first
                safe_cleanup_resources(&input_ctx, NULL);

                // Check if the RTSP URL exists before trying to connect
                // This is a lightweight check to avoid FFmpeg crashes
                if (strncmp(ctx->rtsp_url, "rtsp://", 7) == 0) {
                    // Extract the host and port from the URL
                    char host[256] = {0};
                    int port = 554; // Default RTSP port

                    // Skip the rtsp:// prefix
                    const char *host_start = ctx->rtsp_url + 7;

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

                    // Copy the host part
                    size_t host_len = host_end - host_start;
                    if (host_len >= sizeof(host)) {
                        host_len = sizeof(host) - 1;
                    }
                    memcpy(host, host_start, host_len);
                    host[host_len] = '\0';

                    // Extract the port if specified
                    if (*host_end == ':') {
                        port = atoi(host_end + 1);
                        if (port <= 0) {
                            port = 554; // Default RTSP port
                        }
                    }

                    // Create a socket
                    int sock = socket(AF_INET, SOCK_STREAM, 0);
                    if (sock >= 0) {
                        // Set a short timeout for the connection
                        struct timeval tv;
                        tv.tv_sec = 1;
                        tv.tv_usec = 0;
                        setsockopt(sock, SOL_SOCKET, SO_RCVTIMEO, (const char*)&tv, sizeof tv);
                        setsockopt(sock, SOL_SOCKET, SO_SNDTIMEO, (const char*)&tv, sizeof tv);

                        // Connect to the server
                        struct sockaddr_in server_addr;
                        memset(&server_addr, 0, sizeof(server_addr));
                        server_addr.sin_family = AF_INET;
                        server_addr.sin_port = htons(port);

                        // Convert hostname to IP address
                        struct hostent *he = gethostbyname(host);
                        if (he) {
                            memcpy(&server_addr.sin_addr, he->h_addr_list[0], he->h_length);

                            // Connect to the server
                            if (connect(sock, (struct sockaddr*)&server_addr, sizeof(server_addr)) < 0) {
                                log_error("Failed to connect to RTSP server: %s:%d", host, port);
                                close(sock);

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
                                log_error("Failed to send RTSP OPTIONS request");
                                close(sock);

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

                            // Receive the response
                            char response[1024] = {0};
                            int bytes_received = recv(sock, response, sizeof(response) - 1, 0);
                            close(sock);

                            if (bytes_received <= 0) {
                                log_error("Failed to receive RTSP OPTIONS response");

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

                            // Check if the response contains "404 Not Found"
                            if (strstr(response, "404 Not Found") != NULL) {
                                log_error("RTSP stream not found (404): %s", ctx->rtsp_url);

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
                        } else {
                            log_error("Failed to resolve hostname: %s", host);
                            close(sock);
                        }
                    }
                }

                // Open input stream
                // CRITICAL FIX: Ensure input_ctx is NULL before calling open_input_stream
                input_ctx = NULL;

                ret = open_input_stream(&input_ctx, ctx->rtsp_url, ctx->protocol);
                if (ret < 0) {
                    char error_buf[AV_ERROR_MAX_STRING_SIZE] = {0};
                    av_strerror(ret, error_buf, AV_ERROR_MAX_STRING_SIZE);

                    // Log the error but don't treat any error type specially
                    log_error("Failed to connect to stream %s: %s (error code: %d)",
                             stream_name, error_buf, ret);

                    // CRITICAL FIX: Ensure input_ctx is NULL after a failed open
                    // This prevents potential use-after-free issues
                    if (input_ctx) {
                        avformat_close_input(&input_ctx);
                        input_ctx = NULL;
                    }

                    // Never quit, always keep trying
                    // Just continue the loop and try again after the delay

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

                // Connection successful
                log_info("Successfully connected to stream %s", stream_name);
                thread_state = STREAM_THREAD_RUNNING;
                reconnect_attempt = 0;
                atomic_store(&ctx->connection_valid, 1);
                atomic_store(&ctx->consecutive_failures, 0);
                last_packet_time = time(NULL);
                atomic_store(&ctx->last_packet_time, (int_fast64_t)last_packet_time);
                break;

            case STREAM_THREAD_RUNNING:
                // Check if we should exit before potentially blocking on av_read_frame
                if (!atomic_load(&ctx->running)) {
                    log_info("HLS writer thread for %s detected shutdown before read", stream_name);
                    thread_state = STREAM_THREAD_STOPPING;
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
                    thread_state = STREAM_THREAD_RECONNECTING;
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

                // Only process video packets
                if (pkt->stream_index == video_stream_idx) {
                    // Lock the writer mutex
                    pthread_mutex_lock(&ctx->writer->mutex);

                    ret = hls_writer_write_packet(ctx->writer, pkt, input_stream);

                    // Log key frames for debugging
                    bool is_key_frame = (pkt->flags & AV_PKT_FLAG_KEY) != 0;
                    if (is_key_frame && ret >= 0) {
                        log_debug("Processed key frame for stream %s", stream_name);
                    }

                    // Handle write errors
                    if (ret < 0) {
                        char error_buf[AV_ERROR_MAX_STRING_SIZE] = {0};
                        av_strerror(ret, error_buf, AV_ERROR_MAX_STRING_SIZE);
                        log_warn("Error writing packet to HLS for stream %s: %s", stream_name, error_buf);
                    } else {
                        // Successfully processed a packet
                        last_packet_time = time(NULL);
                        atomic_store(&ctx->last_packet_time, (int_fast64_t)last_packet_time);
                        atomic_store(&ctx->consecutive_failures, 0);
                        atomic_store(&ctx->connection_valid, 1);
                    }

                    // Unlock the writer mutex
                    pthread_mutex_unlock(&ctx->writer->mutex);
                }

                // Unref packet
                av_packet_unref(pkt);

                // Check if we haven't received a packet in a while
                time_t now = time(NULL);
                if (now - last_packet_time > MAX_PACKET_TIMEOUT) {
                    log_error("No packets received from stream %s for %ld seconds, reconnecting",
                             stream_name, now - last_packet_time);
                    thread_state = STREAM_THREAD_RECONNECTING;
                    reconnect_attempt = 1;
                    atomic_store(&ctx->connection_valid, 0);
                    atomic_fetch_add(&ctx->consecutive_failures, 1);
                }
                break;

            case STREAM_THREAD_RECONNECTING:
                log_info("Reconnecting to stream %s (attempt %d)", stream_name, reconnect_attempt);

                // Close existing connection
                safe_cleanup_resources(&input_ctx, NULL);

                // Calculate reconnection delay with exponential backoff
                reconnect_delay_ms = calculate_reconnect_delay(reconnect_attempt);

                // Sleep before reconnecting
                log_info("Waiting %d ms before reconnecting to stream %s", reconnect_delay_ms, stream_name);
                av_usleep(reconnect_delay_ms * 1000);

                // Check if we should stop during the sleep
                if (!atomic_load(&ctx->running)) {
                    log_info("HLS writer thread for %s stopping during reconnection", stream_name);
                    thread_state = STREAM_THREAD_STOPPING;
                    break;
                }

                // Open input stream
                // CRITICAL FIX: Ensure input_ctx is NULL before calling open_input_stream
                input_ctx = NULL;

                ret = open_input_stream(&input_ctx, ctx->rtsp_url, ctx->protocol);
                if (ret < 0) {
                    char error_buf[AV_ERROR_MAX_STRING_SIZE] = {0};
                    av_strerror(ret, error_buf, AV_ERROR_MAX_STRING_SIZE);

                    // Log the error but don't treat any error type specially
                    log_error("Failed to reconnect to stream %s: %s (error code: %d)",
                             stream_name, error_buf, ret);

                    // CRITICAL FIX: Ensure input_ctx is NULL after a failed open
                    // This prevents potential use-after-free issues
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

                // Reconnection successful
                log_info("Successfully reconnected to stream %s after %d attempts",
                        stream_name, reconnect_attempt);
                thread_state = STREAM_THREAD_RUNNING;
                reconnect_attempt = 0;
                atomic_store(&ctx->connection_valid, 1);
                atomic_store(&ctx->consecutive_failures, 0);
                last_packet_time = time(NULL);
                atomic_store(&ctx->last_packet_time, (int_fast64_t)last_packet_time);
                break;

            case STREAM_THREAD_STOPPING:
                log_info("Stopping HLS writer thread for stream %s", stream_name);
                atomic_store(&ctx->running, 0);
                atomic_store(&ctx->connection_valid, 0);
                thread_state = STREAM_THREAD_STOPPED;
                break;

            case STREAM_THREAD_STOPPED:
                // We should never reach this state in the loop
                log_warn("HLS writer thread for %s in STOPPED state but still in main loop", stream_name);
                atomic_store(&ctx->running, 0);
                break;

            default:
                // Unknown state
                log_error("HLS writer thread for %s in unknown state: %d", stream_name, thread_state);
                atomic_store(&ctx->running, 0);
                break;
        }
    }

    // Clean up resources
    safe_cleanup_resources(&input_ctx, &pkt);

    // Mark connection as invalid
    atomic_store(&ctx->connection_valid, 0);

    // Update component state in shutdown coordinator
    if (ctx->shutdown_component_id >= 0) {
        update_component_state(ctx->shutdown_component_id, COMPONENT_STOPPED);
        log_info("Updated HLS writer thread %s state to STOPPED in shutdown coordinator", stream_name);
    }

    log_info("HLS writer thread for stream %s exited", stream_name);
    return NULL;
}

/**
 * Start a recording thread that reads from the RTSP stream and writes to the HLS files
 * Enhanced with robust error handling for go2rtc integration
 */
int hls_writer_start_recording_thread(hls_writer_t *writer, const char *rtsp_url, const char *stream_name, int protocol) {
    // Validate input parameters with detailed error messages
    if (!writer) {
        log_error("NULL writer passed to hls_writer_start_recording_thread");
        return -1;
    }

    if (!rtsp_url) {
        log_error("NULL RTSP URL passed to hls_writer_start_recording_thread");
        return -1;
    }

    if (!stream_name) {
        log_error("NULL stream name passed to hls_writer_start_recording_thread");
        return -1;
    }

    // Check if thread is already running
    if (writer->thread_ctx) {
        log_warn("HLS writer thread for %s is already running", stream_name);
        return 0;
    }

    // Check if this stream is using go2rtc for HLS
    char actual_url[MAX_PATH_LENGTH];

    // Use the original URL by default
    strncpy(actual_url, rtsp_url, sizeof(actual_url) - 1);
    actual_url[sizeof(actual_url) - 1] = '\0';

    // If the stream is using go2rtc for HLS, get the go2rtc RTSP URL with enhanced error handling
    // The go2rtc_integration_is_using_go2rtc_for_hls function will only return true
    // if go2rtc is fully ready and the stream is registered
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

    // Create thread context with additional error handling
    hls_writer_thread_ctx_t *ctx = NULL;

    // Use a try/catch style approach with goto for cleanup
    int result = -1;

    // Allocate thread context
    ctx = (hls_writer_thread_ctx_t *)calloc(1, sizeof(hls_writer_thread_ctx_t));
    if (!ctx) {
        log_error("Failed to allocate HLS writer thread context");
        return -1;
    }

    // Initialize context with safe string operations
    if (strlen(actual_url) >= MAX_PATH_LENGTH) {
        log_error("RTSP URL too long for HLS writer thread context");
        goto cleanup;
    }

    if (strlen(stream_name) >= MAX_STREAM_NAME) {
        log_error("Stream name too long for HLS writer thread context");
        goto cleanup;
    }

    // Use the go2rtc URL if available, otherwise use the original URL
    strncpy(ctx->rtsp_url, actual_url, MAX_PATH_LENGTH - 1);
    ctx->rtsp_url[MAX_PATH_LENGTH - 1] = '\0';

    // Log the URL being used
    log_info("Using URL for HLS streaming of stream %s: %s", stream_name, actual_url);

    strncpy(ctx->stream_name, stream_name, MAX_STREAM_NAME - 1);
    ctx->stream_name[MAX_STREAM_NAME - 1] = '\0';

    ctx->writer = writer;
    ctx->protocol = protocol;
    atomic_store(&ctx->running, 1);
    ctx->shutdown_component_id = -1;

    // Initialize new fields
    atomic_store(&ctx->last_packet_time, (int_fast64_t)time(NULL));
    atomic_store(&ctx->connection_valid, 0); // Start with connection invalid
    atomic_store(&ctx->consecutive_failures, 0);

    // Store thread context in writer BEFORE creating the thread
    // This ensures the thread context is available to other threads
    writer->thread_ctx = ctx;

    // Set up thread attributes to create a detached thread
    // This ensures thread resources are automatically cleaned up when the thread exits
    pthread_attr_t attr;
    pthread_attr_init(&attr);
    pthread_attr_setdetachstate(&attr, PTHREAD_CREATE_DETACHED);

    // Start thread with error handling and detached state
    int thread_result = pthread_create(&ctx->thread, &attr, hls_writer_thread_func, ctx);

    // Clean up thread attributes
    pthread_attr_destroy(&attr);

    if (thread_result != 0) {
        log_error("Failed to create HLS writer thread for %s (error: %s)",
                 stream_name, strerror(thread_result));
        writer->thread_ctx = NULL; // Reset the thread context pointer
        goto cleanup;
    }

    // Log that we're using a detached thread
    log_info("Created detached HLS writer thread for %s", stream_name);

    log_info("Started HLS writer thread for %s", stream_name);
    return 0;

cleanup:
    // Clean up resources if thread creation failed
    if (ctx) {
        free(ctx);
        ctx = NULL;
    }

    return result;
}

/**
 * Stop the recording thread
 * This function is now safer with go2rtc integration
 */
void hls_writer_stop_recording_thread(hls_writer_t *writer) {
    if (!writer) {
        log_warn("NULL writer passed to hls_writer_stop_recording_thread");
        return;
    }

    // Safely check if thread is running
    void *thread_ctx_ptr = writer->thread_ctx;
    if (!thread_ctx_ptr) {
        log_warn("HLS writer thread is not running");
        return;
    }

    // Make a safe local copy of the thread context pointer
    hls_writer_thread_ctx_t *ctx = (hls_writer_thread_ctx_t *)thread_ctx_ptr;

    // Create a local copy of the stream name for logging
    char stream_name[MAX_STREAM_NAME] = {0};
    if (ctx->stream_name[0] != '\0') {
        strncpy(stream_name, ctx->stream_name, MAX_STREAM_NAME - 1);
        stream_name[MAX_STREAM_NAME - 1] = '\0';
    } else {
        strcpy(stream_name, "unknown");
    }

    log_info("Stopping HLS writer thread for %s", stream_name);

    // Signal thread to stop
    atomic_store(&ctx->running, 0);
    atomic_store(&ctx->connection_valid, 0);

    // Mark the thread context as NULL in the writer
    // This prevents other threads from trying to access it while we're shutting down
    writer->thread_ctx = NULL;

    // Update component state in shutdown coordinator
    if (ctx->shutdown_component_id >= 0) {
        update_component_state(ctx->shutdown_component_id, COMPONENT_STOPPING);
    }

    // Since we're using detached threads, we don't need to join or detach the thread
    // The thread will exit on its own when it checks ctx->running
    // We just need to wait a bit to ensure the thread has time to notice it should exit
    log_info("Waiting for detached HLS writer thread for %s to exit on its own", stream_name);

    // Update component state in shutdown coordinator
    if (ctx->shutdown_component_id >= 0) {
        update_component_state(ctx->shutdown_component_id, COMPONENT_STOPPED);
    }

    // Add a small delay to ensure any in-progress operations complete
    usleep(100000); // 100ms

    // Free thread context
    free(ctx);
    ctx = NULL; // Set to NULL after freeing to prevent double-free

    log_info("Stopped HLS writer thread for %s", stream_name);
}

/**
 * Check if the recording thread is running and has a valid connection
 * This function is now safer with go2rtc integration and handles NULL pointers properly
 */
int hls_writer_is_recording(hls_writer_t *writer) {
    // Basic validation
    if (!writer) {
        log_debug("hls_writer_is_recording: NULL writer pointer");
        return 0;
    }

    // Safely check thread context with memory barrier to ensure thread safety
    void *thread_ctx_ptr = writer->thread_ctx;
    if (!thread_ctx_ptr) {
        log_debug("hls_writer_is_recording: NULL thread context pointer");
        return 0;
    }

    // Make a safe local copy of the thread context pointer
    hls_writer_thread_ctx_t *ctx = (hls_writer_thread_ctx_t *)thread_ctx_ptr;

    // Create a local copy of the stream name for logging
    char stream_name[MAX_STREAM_NAME] = {0};
    if (ctx->stream_name && ctx->stream_name[0] != '\0') {
        strncpy(stream_name, ctx->stream_name, MAX_STREAM_NAME - 1);
        stream_name[MAX_STREAM_NAME - 1] = '\0';
    } else {
        strcpy(stream_name, "unknown");
    }

    // Check if the thread is running
    if (!atomic_load(&ctx->running)) {
        log_debug("HLS writer thread for %s is not running", stream_name);
        return 0;
    }

    // Check if the connection is valid
    if (!atomic_load(&ctx->connection_valid)) {
        log_debug("HLS writer thread for %s has invalid connection", stream_name);
        return 0;
    }

    // Check if we've received a packet recently
    time_t now = time(NULL);
    time_t last_packet_time = (time_t)atomic_load(&ctx->last_packet_time);
    if (now - last_packet_time > MAX_PACKET_TIMEOUT) {
        log_error("HLS writer thread for %s has not received a packet in %ld seconds, considering it dead",
                 stream_name, now - last_packet_time);

        // Mark connection as invalid to trigger reconnection
        atomic_store(&ctx->connection_valid, 0);

        // Increment consecutive failures to track timeout issues
        atomic_fetch_add(&ctx->consecutive_failures, 1);

        return 0;
    }

    return 1;
}
