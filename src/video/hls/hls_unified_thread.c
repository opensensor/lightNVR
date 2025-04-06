/* Feature test macros for POSIX.1-2008 features */
#define _POSIX_C_SOURCE 200809L
#define _XOPEN_SOURCE 700
#define _GNU_SOURCE

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
#include <sys/stat.h>  /* For stat() and S_ISDIR */
#include <netinet/in.h>
#include <arpa/inet.h>
#include <netdb.h>
#include <sys/time.h>
#include <execinfo.h>  /* For backtrace() */
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
// CRITICAL FIX: Add a signal handler to catch segmentation faults
// This will help us identify exactly where the crash is happening
void segfault_handler(int sig) {
    void *array[20];
    size_t size;

    // Get the backtrace
    size = backtrace(array, 20);

    // Print the backtrace
    fprintf(stderr, "\n\nCaught segmentation fault! Backtrace:\n");
    backtrace_symbols_fd(array, size, STDERR_FILENO);

    // Also log it
    char **strings = backtrace_symbols(array, size);
    if (strings) {
        log_error("Segmentation fault detected! Backtrace:");
        for (size_t i = 0; i < size; i++) {
            log_error("  %s", strings[i]);
        }
        free(strings);
    }

    // Exit with error code
    exit(1);
}
#include "video/ffmpeg_utils.h"
#include "video/timeout_utils.h"

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
 *
 * MEMORY LEAK FIX: Updated to use the new FFmpeg utility functions for more thorough cleanup
 */
static void safe_cleanup_resources(AVFormatContext **input_ctx, AVPacket **pkt, hls_writer_t **writer) {
    // Clean up packet with enhanced safety using our utility function
    if (pkt && *pkt) {
        // Use our new utility function for safer packet cleanup
        safe_packet_cleanup(pkt);
    }

    // Clean up input context with extra safety checks using our utility function
    if (input_ctx && *input_ctx) {
        // Use our new utility function for safer context cleanup
        safe_avformat_cleanup(input_ctx);

        // MEMORY LEAK FIX: Explicitly call garbage collection
        // FFmpeg doesn't have a built-in garbage collection function, but we've done our best
        // to clean up all resources manually
    }

    // Clean up HLS writer
    if (writer && *writer) {
        hls_writer_t *writer_to_close = *writer;
        *writer = NULL; // Clear the pointer first to prevent double-free

        // Safely close the writer
        hls_writer_close(writer_to_close);
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
 * Check RTSP connection by sending an OPTIONS request
 *
 * @param rtsp_url The RTSP URL to check
 * @param host Buffer to store the extracted hostname
 * @param port Pointer to store the extracted port
 * @return 0 on success, negative on error
 */
static int check_rtsp_connection(const char *rtsp_url, char *host, int *port) {
    if (!rtsp_url || strncmp(rtsp_url, "rtsp://", 7) != 0) {
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

    // Copy the host part
    size_t host_len = host_end - host_start;
    if (host_len >= 256) {
        host_len = 255;
    }
    memcpy(host, host_start, host_len);
    host[host_len] = '\0';

    // Extract the port if specified
    *port = 554; // Default RTSP port
    if (*host_end == ':') {
        *port = atoi(host_end + 1);
        if (*port <= 0) {
            *port = 554; // Default RTSP port
        }
    }

    // Create a socket
    int sock = socket(AF_INET, SOCK_STREAM, 0);
    if (sock < 0) {
        return -1;
    }

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
    // CRITICAL FIX: Register the segmentation fault handler
    // This will help us identify exactly where the crash is happening
    signal(SIGSEGV, segfault_handler);

    hls_unified_thread_ctx_t *ctx = (hls_unified_thread_ctx_t *)arg;
    AVFormatContext *input_ctx = NULL;
    // Initialize packet with extra safety measures
    AVPacket *pkt = NULL;
    int video_stream_idx = -1;
    int ret;
    hls_thread_state_t thread_state = HLS_THREAD_INITIALIZING;
    int reconnect_attempt = 0;
    int reconnect_delay_ms = BASE_RECONNECT_DELAY_MS;
    time_t last_packet_time = 0;

    // Allocate packet with proper initialization
    pkt = av_packet_alloc();
    if (!pkt) {
        log_error("Failed to allocate packet for stream %s", ctx ? ctx->stream_name : "unknown");
        return NULL;
    }

    // Initialize packet fields to safe values
    pkt->data = NULL;
    pkt->size = 0;
    pkt->buf = NULL;

    // Create local copies of important context data to prevent accessing freed memory
    char stream_name[MAX_STREAM_NAME];
    int shutdown_component_id = -1;

    // Set thread cancellation state
    pthread_setcancelstate(PTHREAD_CANCEL_ENABLE, NULL);
    pthread_setcanceltype(PTHREAD_CANCEL_DEFERRED, NULL);

    // Validate context
    if (!ctx) {
        log_error("NULL context passed to unified HLS thread");
        return NULL;
    }

    // Create a local copy of the stream name and component ID for thread safety
    strncpy(stream_name, ctx->stream_name, MAX_STREAM_NAME - 1);
    stream_name[MAX_STREAM_NAME - 1] = '\0';
    shutdown_component_id = ctx->shutdown_component_id;

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

    // Initialize packet with extra safety checks
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

    // Initialize packet fields to prevent potential issues
    // Note: av_packet_alloc already initializes the packet in newer FFmpeg versions
    pkt->data = NULL;
    pkt->size = 0;

    // Create HLS writer with appropriate segment duration
    // Add extra logging for debugging
    log_info("Creating HLS writer for stream %s with path %s and segment duration %d",
             stream_name, ctx->output_path, ctx->segment_duration);

    // Ensure the output directory exists before creating the writer
    char dir_cmd[MAX_PATH_LENGTH * 2];
    snprintf(dir_cmd, sizeof(dir_cmd), "mkdir -p %s && chmod -R 777 %s",
             ctx->output_path, ctx->output_path);
    int dir_result = system(dir_cmd);
    if (dir_result != 0) {
        log_warn("Directory creation command returned %d for %s", dir_result, ctx->output_path);
    }

    // Verify directory exists and is writable before creating writer
    struct stat st;
    if (stat(ctx->output_path, &st) != 0 || !S_ISDIR(st.st_mode)) {
        log_error("HLS output directory does not exist or is not a directory: %s", ctx->output_path);
        safe_cleanup_resources(NULL, &pkt, NULL);
        atomic_store(&ctx->running, 0);
        if (ctx->shutdown_component_id >= 0) {
            update_component_state(ctx->shutdown_component_id, COMPONENT_STOPPED);
        }
        return NULL;
    }

    if (access(ctx->output_path, W_OK) != 0) {
        log_error("HLS output directory is not writable: %s", ctx->output_path);
        safe_cleanup_resources(NULL, &pkt, NULL);
        atomic_store(&ctx->running, 0);
        if (ctx->shutdown_component_id >= 0) {
            update_component_state(ctx->shutdown_component_id, COMPONENT_STOPPED);
        }
        return NULL;
    }

    // Now create the HLS writer with verified directory
    log_info("Creating HLS writer for stream %s with verified directory", stream_name);
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

    // CRITICAL FIX: Add a delay after creating the writer to ensure it's fully initialized
    // This helps prevent race conditions in multi-threaded environments
    log_info("Adding delay after creating HLS writer for stream %s to ensure initialization", stream_name);
    usleep(100000); // 100ms delay

    // Store the HLS writer in the stream state for other components to access
    // Add extra validation
    if (state && ctx->writer) {
        log_info("Storing HLS writer for stream %s in stream state", stream_name);
        state->hls_ctx = ctx->writer;
    } else if (!state) {
        log_warn("Stream state is NULL for %s, cannot store HLS writer", stream_name);
    } else if (!ctx->writer) {
        log_error("HLS writer is NULL for %s, cannot store in stream state", stream_name);
        atomic_store(&ctx->running, 0);
        if (ctx->shutdown_component_id >= 0) {
            update_component_state(ctx->shutdown_component_id, COMPONENT_STOPPED);
        }
        return NULL;
    }

    // Make a local copy of the writer pointer for safety
    hls_writer_t *local_writer = ctx->writer;

    // Double-check that the writer is valid
    if (!local_writer) {
        log_error("Local writer pointer is NULL for %s", stream_name);
        atomic_store(&ctx->running, 0);
        if (ctx->shutdown_component_id >= 0) {
            update_component_state(ctx->shutdown_component_id, COMPONENT_STOPPED);
        }
        return NULL;
    }

    // Main state machine loop
    while (ctx && atomic_load(&ctx->running)) {
        // Update thread state in context if it's still valid
        if (ctx) {
            atomic_store(&ctx->thread_state, thread_state);
        } else {
            // Context has been freed, exit the thread
            log_warn("Context for stream %s has been freed, exiting thread", stream_name);
            thread_state = HLS_THREAD_STOPPING;
            break;
        }

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

                // Close any existing connection first with comprehensive cleanup
                comprehensive_ffmpeg_cleanup(&input_ctx, NULL, &pkt, NULL);

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

                // Open input stream with improved timeout handling
                input_ctx = NULL;

                // Set up a timeout context for the connection attempt
                timeout_context_t connect_timeout;
                init_timeout(&connect_timeout, 10); // 10 second timeout for connection

                // Attempt to open the input stream
                ret = open_input_stream(&input_ctx, ctx->rtsp_url, ctx->protocol);

                // Check if a timeout occurred
                if (check_timeout(&connect_timeout)) {
                    log_warn("Timeout occurred while connecting to stream %s", stream_name);

                    // Handle the timeout using our cleanup function
                    ret = handle_timeout_cleanup(ctx->rtsp_url, &input_ctx);
                } else if (ret < 0) {
                    // Handle normal errors
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
                }

                if (ret < 0) {
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

                // CRITICAL FIX: Add extra validation before transitioning to RUNNING state
                // This helps prevent segmentation faults by ensuring all resources are valid
                if (!ctx) {
                    log_error("Context became NULL after successful connection for stream %s", stream_name);
                    comprehensive_ffmpeg_cleanup(&input_ctx, NULL, &pkt, NULL);
                    return NULL; // Exit thread completely
                }

                if (!input_ctx) {
                    log_error("Input context is NULL after successful connection for stream %s", stream_name);
                    thread_state = HLS_THREAD_RECONNECTING;
                    reconnect_attempt = 1;
                    break;
                }

                if (!local_writer) {
                    log_error("HLS writer is NULL after successful connection for stream %s", stream_name);
                    comprehensive_ffmpeg_cleanup(&input_ctx, NULL, &pkt, NULL);
                    thread_state = HLS_THREAD_STOPPING;
                    break;
                }

                // Validate that the writer is still valid
                if (!local_writer->output_ctx) {
                    log_error("HLS writer output context is NULL after successful connection for stream %s", stream_name);
                    comprehensive_ffmpeg_cleanup(&input_ctx, NULL, &pkt, NULL);
                    thread_state = HLS_THREAD_STOPPING;
                    break;
                }

                // Now it's safe to transition to RUNNING state
                thread_state = HLS_THREAD_RUNNING;
                reconnect_attempt = 0;
                atomic_store(&ctx->connection_valid, 1);
                atomic_store(&ctx->consecutive_failures, 0);
                last_packet_time = time(NULL);
                atomic_store(&ctx->last_packet_time, (int_fast64_t)last_packet_time);
                break;

            case HLS_THREAD_RUNNING:
                // Check if we should exit before potentially blocking on av_read_frame
                if (!ctx || !atomic_load(&ctx->running)) {
                    log_info("Unified HLS thread for %s detected shutdown before read", stream_name);
                    thread_state = HLS_THREAD_STOPPING;
                    break;
                }

                // Set cancellation point before potentially blocking operation
                pthread_testcancel();

                // MEMORY LEAK FIX: Reset packet before reading to ensure clean state
                // This is now handled in the extra safety checks below

                // MEMORY LEAK FIX: Periodically reset all FFmpeg resources to prevent memory growth
                // This is critical to address the memory leaks shown in Valgrind
                static int packet_count = 0;
                if (++packet_count >= 1000) { // Reset every 1000 packets
                    // Save the current video stream index
                    int saved_video_stream_idx = video_stream_idx;

                    // 1. Free the packet
                    safe_packet_cleanup(&pkt);

                    // 2. Use our new utility function to perform the reset
                    ret = periodic_ffmpeg_reset(&input_ctx, ctx->rtsp_url, ctx->protocol);
                    if (ret < 0) {
                        log_error("Failed to reset FFmpeg resources for stream %s", stream_name);
                        thread_state = HLS_THREAD_RECONNECTING;
                        reconnect_attempt = 1;
                        atomic_store(&ctx->connection_valid, 0);
                        atomic_fetch_add(&ctx->consecutive_failures, 1);

                        // Allocate a new packet before breaking
                        pkt = av_packet_alloc();
                        if (!pkt) {
                            log_error("Failed to allocate packet during reset");
                            thread_state = HLS_THREAD_STOPPING;
                        }
                        break;
                    }

                    // 3. Find video stream index
                    video_stream_idx = find_video_stream_index(input_ctx);
                    if (video_stream_idx == -1) {
                        log_error("No video stream found after reset for stream %s", stream_name);
                        thread_state = HLS_THREAD_RECONNECTING;
                        reconnect_attempt = 1;
                        break;
                    }

                    // 4. Allocate a new packet
                    pkt = av_packet_alloc();
                    if (!pkt) {
                        log_error("Failed to allocate packet during reset for stream %s", stream_name);
                        thread_state = HLS_THREAD_STOPPING;
                        break;
                    }

                    // Reset packet counter
                    packet_count = 0;

                    log_info("Successfully reset FFmpeg resources for stream %s (old video stream idx: %d, new: %d)",
                             stream_name, saved_video_stream_idx, video_stream_idx);
                }

                // Read packet with timeout handling
                timeout_context_t read_timeout;
                init_timeout(&read_timeout, 5); // 5 second timeout for reading frames

                // CRITICAL FIX: Add extra validation before calling av_read_frame
                if (!input_ctx || !input_ctx->pb) {
                    log_error("Invalid input context before av_read_frame for stream %s", stream_name);
                    thread_state = HLS_THREAD_RECONNECTING;
                    reconnect_attempt = 1;
                    break;
                }

                // CRITICAL FIX: Check for invalid packet
                if (!pkt) {
                    log_error("NULL packet before av_read_frame for stream %s", stream_name);
                    // Allocate a new packet
                    pkt = av_packet_alloc();
                    if (!pkt) {
                        log_error("Failed to allocate packet before av_read_frame");
                        thread_state = HLS_THREAD_STOPPING;
                        break;
                    }
                }

                // CRITICAL FIX: Use a try-catch approach to handle potential segmentation faults
                // We'll set a flag before the potentially dangerous operation and check it after
                log_debug("Attempting to read frame for stream %s", stream_name);

                // CRITICAL FIX: Add extra safety checks around av_read_frame
                // Ensure packet is in a clean state
                safe_packet_unref(pkt, "before_read_frame");

                // CRITICAL FIX: Check for shutdown before potentially blocking on av_read_frame
                if (!ctx || !atomic_load(&ctx->running)) {
                    log_info("Unified HLS thread for %s detected shutdown before read", stream_name);
                    thread_state = HLS_THREAD_STOPPING;
                    break;
                }

                // CRITICAL FIX: Add a timeout check before and after read
                // This helps prevent the thread from blocking indefinitely during shutdown
                struct timeval tv;
                gettimeofday(&tv, NULL);
                time_t start_time = tv.tv_sec;

                // Call av_read_frame with error handling
                ret = av_read_frame(input_ctx, pkt);

                // CRITICAL FIX: Check for shutdown again after av_read_frame
                if (!ctx || !atomic_load(&ctx->running)) {
                    log_info("Unified HLS thread for %s detected shutdown after read", stream_name);
                    thread_state = HLS_THREAD_STOPPING;
                    break;
                }

                // CRITICAL FIX: Check if read took too long (more than 1 second)
                gettimeofday(&tv, NULL);
                time_t end_time = tv.tv_sec;
                if (end_time - start_time > 1) {
                    log_warn("Read operation took too long (%ld seconds) for stream %s",
                             end_time - start_time, stream_name);

                    // Check for shutdown again
                    if (!ctx || !atomic_load(&ctx->running)) {
                        log_info("Unified HLS thread for %s detected shutdown after long read", stream_name);
                        thread_state = HLS_THREAD_STOPPING;
                        break;
                    }
                }

                // Check for errors
                if (ret < 0 && ret != AVERROR_EOF && ret != AVERROR(EAGAIN)) {
                    char error_buf[AV_ERROR_MAX_STRING_SIZE] = {0};
                    av_strerror(ret, error_buf, AV_ERROR_MAX_STRING_SIZE);
                    log_debug("Error reading frame for stream %s: %s (code: %d)",
                             stream_name, error_buf, ret);
                }

                // Check if a timeout occurred
                if (check_timeout(&read_timeout)) {
                    log_warn("Timeout occurred while reading from stream %s", stream_name);

                    // Handle the timeout using our cleanup function
                    ret = handle_timeout_cleanup(ctx->rtsp_url, &input_ctx);

                    // MEMORY LEAK FIX: Ensure packet is properly unreferenced with our safe function
                    safe_packet_unref(pkt, "timeout_handler");
                } else if (ret < 0) {
                    // Handle read errors
                    char error_buf[AV_ERROR_MAX_STRING_SIZE] = {0};
                    av_strerror(ret, error_buf, AV_ERROR_MAX_STRING_SIZE);

                    log_error("Error reading from stream %s: %s (code: %d)",
                             stream_name, error_buf, ret);

                    // MEMORY LEAK FIX: Ensure packet is properly unreferenced with our safe function
                    safe_packet_unref(pkt, "error_handler");
                }

                if (ret < 0) {

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
                    safe_packet_unref(pkt, "invalid_stream_index");
                    continue;
                }

                // Validate packet data
                if (!pkt->data || pkt->size <= 0) {
                    log_warn("Invalid packet (null data or zero size) for stream %s", stream_name);
                    safe_packet_unref(pkt, "invalid_packet_data");
                    continue;
                }

                // Only process video packets
                if (pkt->stream_index == video_stream_idx) {
                    // CRITICAL FIX: Add additional validation for packet and stream
                    if (!pkt->data || pkt->size <= 0) {
                        log_warn("Skipping invalid packet for stream %s (null data or zero size)", stream_name);
                        safe_packet_unref(pkt, "skip_invalid_packet");
                        continue;
                    }

                    if (!input_stream || !input_stream->codecpar) {
                        log_warn("Skipping packet with invalid stream info for stream %s", stream_name);
                        safe_packet_unref(pkt, "skip_invalid_stream_info");
                        continue;
                    }

                    // Check if writer is still valid
                    if (!local_writer) {
                        log_warn("HLS writer for stream %s is no longer valid", stream_name);
                        thread_state = HLS_THREAD_STOPPING;
                        safe_packet_unref(pkt, "invalid_writer");
                        continue;
                    }

                    // CRITICAL FIX: Skip the first few packets to avoid potential issues with initialization
                    static int packet_counter = 0;
                    if (packet_counter < 5) {
                        packet_counter++;
                        log_info("Skipping packet %d of 5 for stream %s during initialization",
                                packet_counter, stream_name);
                        safe_packet_unref(pkt, "skip_init_packet");

                        // If this is the 5th packet, update the connection status
                        if (packet_counter == 5) {
                            log_info("Stream %s initialization complete, processing packets normally now", stream_name);
                            if (ctx) {
                                last_packet_time = time(NULL);
                                atomic_store(&ctx->last_packet_time, (int_fast64_t)last_packet_time);
                                atomic_store(&ctx->consecutive_failures, 0);
                                atomic_store(&ctx->connection_valid, 1);
                            }
                        }
                        continue;
                    }

                    // CRITICAL FIX: Create a safe copy of the packet before processing
                    AVPacket *safe_pkt = av_packet_alloc();
                    if (!safe_pkt) {
                        log_error("Failed to allocate safe packet for stream %s", stream_name);
                        safe_packet_unref(pkt, "alloc_safe_packet_failed");
                        continue;
                    }

                    // Copy the packet data
                    if (av_packet_ref(safe_pkt, pkt) < 0) {
                        log_error("Failed to reference packet for stream %s", stream_name);
                        av_packet_free(&safe_pkt);
                        safe_packet_unref(pkt, "packet_ref_failed");
                        continue;
                    }

                    // Extra validation for local_writer
                    if (!local_writer) {
                        log_error("HLS writer became NULL for stream %s", stream_name);
                        safe_packet_unref(safe_pkt, "writer_became_null_safe_pkt");
                        av_packet_free(&safe_pkt);
                        safe_packet_unref(pkt, "writer_became_null");
                        thread_state = HLS_THREAD_STOPPING;
                        continue;
                    }

                    // Try to lock the writer mutex with timeout to prevent deadlocks
                    // First try a simple trylock which is more widely supported
                    int lock_result = pthread_mutex_trylock(&local_writer->mutex);

                    // If trylock fails, try with a timeout if available
                    if (lock_result != 0) {
                        // Sleep for a short time and try again a few times
                        for (int retry = 0; retry < 5 && lock_result != 0; retry++) {
                            usleep(100000); // 100ms
                            lock_result = pthread_mutex_trylock(&local_writer->mutex);
                        }
                    }
                    if (lock_result != 0) {
                        log_error("Failed to lock HLS writer mutex for stream %s (error: %d)",
                                 stream_name, lock_result);
                        safe_packet_unref(safe_pkt, "lock_failed_safe_pkt");
                        av_packet_free(&safe_pkt);
                        safe_packet_unref(pkt, "lock_failed");
                        continue;
                    }

                    // Double-check that writer is still valid after locking
                    if (!local_writer->output_ctx) {
                        log_warn("HLS writer output context is NULL for stream %s", stream_name);
                        pthread_mutex_unlock(&local_writer->mutex);
                        safe_packet_unref(safe_pkt, "output_ctx_null_safe_pkt");
                        av_packet_free(&safe_pkt);
                        safe_packet_unref(pkt, "output_ctx_null");
                        continue;
                    }

                    // CRITICAL FIX: Add a delay after initialization to ensure the writer is fully initialized
                    // This is especially important for the first few packets
                    static int first_packet = 1;
                    if (first_packet) {
                        log_info("First packet for stream %s, adding extra delay to ensure writer is fully initialized", stream_name);
                        usleep(50000); // 50ms delay for the first packet
                        first_packet = 0;
                    }

                    // Process the packet with additional error handling and retry logic
                    ret = hls_writer_write_packet(local_writer, safe_pkt, input_stream);

                    // If the writer is not initialized, wait a bit and try again
                    if (ret == AVERROR(EINVAL)) {
                        log_warn("Writer not initialized for stream %s, waiting and retrying", stream_name);
                        usleep(100000); // 100ms delay
                        ret = hls_writer_write_packet(local_writer, safe_pkt, input_stream);
                    }

                    // Log key frames for debugging
                    bool is_key_frame = (safe_pkt->flags & AV_PKT_FLAG_KEY) != 0;
                    if (is_key_frame && ret >= 0) {
                        log_debug("Processed key frame for stream %s", stream_name);
                    }

                    // Handle write errors
                    if (ret < 0) {
                        char error_buf[AV_ERROR_MAX_STRING_SIZE] = {0};
                        av_strerror(ret, error_buf, AV_ERROR_MAX_STRING_SIZE);
                        log_warn("Error writing packet to HLS for stream %s: %s", stream_name, error_buf);
                    } else if (ctx) { // Only update context if it's still valid
                        // Successfully processed a packet
                        last_packet_time = time(NULL);
                        atomic_store(&ctx->last_packet_time, (int_fast64_t)last_packet_time);
                        atomic_store(&ctx->consecutive_failures, 0);
                        atomic_store(&ctx->connection_valid, 1);
                    }

                    // Unlock the writer mutex
                    pthread_mutex_unlock(&local_writer->mutex);

                    // Clean up the safe packet
                    safe_packet_unref(safe_pkt, "cleanup_safe_pkt");
                    av_packet_free(&safe_pkt);

                    // Skip detection processing to avoid potential issues
                    // This is a temporary fix to isolate the segmentation fault
                    // process_packet_for_detection(stream_name, pkt, input_stream->codecpar);
                }

                // MEMORY LEAK FIX: Enhanced packet unreferencing
                if (pkt) {
                    // Ensure packet is properly unreferenced
                    safe_packet_unref(pkt, "cleanup_pkt");

                    // Log packet memory usage periodically
                    static int unref_count = 0;
                    if (++unref_count % 1000 == 0) {
                        log_debug("Unreferenced 1000 packets for stream %s", stream_name);
                    }
                } else {
                    log_warn("Attempted to unref NULL packet for stream %s", stream_name);
                }

                // Check if we haven't received a packet in a while
                time_t now = time(NULL);
                if (now - last_packet_time > MAX_PACKET_TIMEOUT) {
                    log_error("No packets received from stream %s for %ld seconds, reconnecting",
                             stream_name, now - last_packet_time);
                    thread_state = HLS_THREAD_RECONNECTING;
                    reconnect_attempt = 1;

                    // Only update context if it's still valid
                    if (ctx) {
                        atomic_store(&ctx->connection_valid, 0);
                        atomic_fetch_add(&ctx->consecutive_failures, 1);
                    }
                }
                break;

            case HLS_THREAD_RECONNECTING:
                log_info("Reconnecting to stream %s (attempt %d)", stream_name, reconnect_attempt);

                // Close existing connection with comprehensive cleanup
                comprehensive_ffmpeg_cleanup(&input_ctx, NULL, &pkt, NULL);

                // Calculate reconnection delay with exponential backoff
                reconnect_delay_ms = calculate_reconnect_delay(reconnect_attempt);

                // Sleep before reconnecting
                log_info("Waiting %d ms before reconnecting to stream %s", reconnect_delay_ms, stream_name);
                av_usleep(reconnect_delay_ms * 1000);

                // Check if we should stop during the sleep
                if (!ctx || !atomic_load(&ctx->running)) {
                    log_info("Unified HLS thread for %s stopping during reconnection", stream_name);
                    thread_state = HLS_THREAD_STOPPING;
                    break;
                }

                // Open input stream with improved timeout handling
                input_ctx = NULL;

                // Set up a timeout context for the reconnection attempt
                timeout_context_t reconnect_timeout;
                init_timeout(&reconnect_timeout, 10); // 10 second timeout for reconnection

                // Attempt to open the input stream
                ret = open_input_stream(&input_ctx, ctx->rtsp_url, ctx->protocol);

                // Check if a timeout occurred
                if (check_timeout(&reconnect_timeout)) {
                    log_warn("Timeout occurred while reconnecting to stream %s", stream_name);

                    // Handle the timeout using our cleanup function
                    ret = handle_timeout_cleanup(ctx->rtsp_url, &input_ctx);

                    // Increment reconnection attempt counter
                    reconnect_attempt++;

                    // Cap reconnection attempts to avoid integer overflow
                    if (reconnect_attempt > 1000) {
                        reconnect_attempt = 1000;
                    }

                    // Stay in reconnecting state
                    break;
                } else if (ret < 0) {
                    // Handle normal errors
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

                // CRITICAL FIX: Clean up resources immediately to avoid stalling during shutdown
                log_info("Cleaning up resources for stream %s during shutdown", stream_name);

                // Clean up FFmpeg resources
                if (input_ctx) {
                    log_info("Closing input context for stream %s during shutdown", stream_name);
                    avformat_close_input(&input_ctx);
                }

                // Clean up packet
                if (pkt) {
                    log_info("Freeing packet for stream %s during shutdown", stream_name);
                    safe_packet_unref(pkt, "stopping_state");
                    av_packet_free(&pkt);
                    pkt = NULL;
                }

                // Safely close the HLS writer before exiting
                if (local_writer) {
                    log_info("Safely closing HLS writer for stream %s during thread shutdown", stream_name);

                    // Try to lock the mutex with a short timeout
                    int lock_result = pthread_mutex_trylock(&local_writer->mutex);

                    // Only try once with a very short retry
                    if (lock_result != 0) {
                        usleep(10000); // 10ms
                        lock_result = pthread_mutex_trylock(&local_writer->mutex);
                    }

                    // Whether we got the lock or not, close the writer
                    hls_writer_close(local_writer);
                    local_writer = NULL;

                    // Also clear the writer in the context to prevent double-free
                    if (ctx) {
                        ctx->writer = NULL;
                    }

                    log_info("Closed HLS writer for stream %s during shutdown", stream_name);
                }

                // Update state
                atomic_store(&ctx->running, 0);
                atomic_store(&ctx->connection_valid, 0);

                // Unmark the stream as stopping to indicate we've completed our shutdown
                unmark_stream_stopping(stream_name);
                log_info("Unmarked stream %s as stopping during thread shutdown", stream_name);

                // Exit the thread immediately
                log_info("Unified HLS thread for stream %s has been stopped", stream_name);
                return NULL;

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

    // Mark connection as invalid if context is still valid
    if (ctx) {
        atomic_store(&ctx->connection_valid, 0);
    }

    // Clear the reference in the stream state
    if (state && ctx && state->hls_ctx == ctx->writer) {
        state->hls_ctx = NULL;
    }

    // Clean up all resources in one call with extra logging
    log_info("Cleaning up all resources for stream %s", stream_name);

    // First, explicitly close any open codecs and parsers
    if (input_ctx) {
        log_info("Closing input context for stream %s with %d streams",
                stream_name, input_ctx->nb_streams);

        // Log information about each stream for debugging
        for (unsigned int i = 0; i < input_ctx->nb_streams; i++) {
            if (input_ctx->streams[i] && input_ctx->streams[i]->codecpar) {
                log_info("Stream %d: codec type %d, codec ID %d",
                        i,
                        input_ctx->streams[i]->codecpar->codec_type,
                        input_ctx->streams[i]->codecpar->codec_id);
            } else if (input_ctx->streams[i]) {
                log_info("Stream %d: no codec parameters available", i);
            }
        }
    }

    // MEMORY LEAK FIX: Enhanced final cleanup with multiple safety checks
    log_info("Performing aggressive final cleanup for stream %s", stream_name);

    // First attempt: Use our comprehensive cleanup function with enhanced safety
    if (ctx) {
        // Clean up FFmpeg resources first
        comprehensive_ffmpeg_cleanup(&input_ctx, NULL, &pkt, NULL);

        // Only close the writer if it hasn't been closed in the STOPPING state
        if (ctx->writer) {
            log_info("Closing HLS writer for stream %s during final cleanup", stream_name);

            // Try to lock the mutex with timeout to prevent deadlocks
            struct timespec timeout;
            clock_gettime(CLOCK_REALTIME, &timeout);
            timeout.tv_sec += 2; // 2 second timeout

            hls_writer_t *writer_to_close = ctx->writer;
            ctx->writer = NULL; // Clear the pointer first to prevent double-free

            int lock_result = pthread_mutex_trylock(&writer_to_close->mutex);
            if (lock_result == 0) {
                // Successfully locked the mutex, now close the writer
                hls_writer_close(writer_to_close);
                pthread_mutex_unlock(&writer_to_close->mutex);
                log_info("Successfully closed HLS writer for stream %s during final cleanup", stream_name);
            } else {
                // If we can't lock the mutex, try to close it anyway
                log_warn("Could not lock HLS writer mutex for stream %s during final cleanup (error: %d), closing anyway",
                        stream_name, lock_result);
                hls_writer_close(writer_to_close);
            }
        }
    } else {
        // If context is no longer valid, clean up what we can
        comprehensive_ffmpeg_cleanup(&input_ctx, NULL, &pkt, NULL);

        // Only close the local writer if it hasn't been closed in the STOPPING state
        if (local_writer) {
            log_info("Closing local HLS writer for stream %s during final cleanup", stream_name);

            // Try to lock the mutex with timeout to prevent deadlocks
            struct timespec timeout;
            clock_gettime(CLOCK_REALTIME, &timeout);
            timeout.tv_sec += 2; // 2 second timeout

            hls_writer_t *writer_to_close = local_writer;
            local_writer = NULL; // Clear the pointer first to prevent double-free

            int lock_result = pthread_mutex_trylock(&writer_to_close->mutex);
            if (lock_result == 0) {
                // Successfully locked the mutex, now close the writer
                hls_writer_close(writer_to_close);
                pthread_mutex_unlock(&writer_to_close->mutex);
                log_info("Successfully closed local HLS writer for stream %s during final cleanup", stream_name);
            } else {
                // If we can't lock the mutex, try to close it anyway
                log_warn("Could not lock local HLS writer mutex for stream %s during final cleanup (error: %d), closing anyway",
                        stream_name, lock_result);
                hls_writer_close(writer_to_close);
            }
        }
    }

    // Second attempt: Double check that packet is freed
    if (pkt) {
        log_warn("Packet still exists after cleanup, forcing cleanup");
        // Use our safe packet unref function
        safe_packet_unref(pkt, "final_cleanup");
        av_packet_free(&pkt);
    }

    // Second attempt: Double check that input context is freed
    if (input_ctx) {
        log_warn("Input context still exists after cleanup, forcing cleanup");
        // Try one more time with our comprehensive cleanup
        comprehensive_ffmpeg_cleanup(&input_ctx, NULL, NULL, NULL);
    }

    log_info("Completed aggressive final cleanup for stream %s", stream_name);

    // Update component state in shutdown coordinator using the local copy
    if (shutdown_component_id >= 0) {
        update_component_state(shutdown_component_id, COMPONENT_STOPPED);
        log_info("Updated unified HLS thread %s state to STOPPED in shutdown coordinator", stream_name);
    }

    // Unmark the stream as stopping to indicate we've completed our shutdown
    unmark_stream_stopping(stream_name);
    log_info("Unmarked stream %s as stopping before thread exit", stream_name);

    log_info("Unified HLS thread for stream %s exited", stream_name);
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

    // CRITICAL FIX: Use a shorter timeout to avoid stalling during shutdown
    int wait_attempts = 10; // Try up to 10 times (5 seconds total)
    bool thread_exited = false;

    while (wait_attempts > 0) {
        // Check if the thread has marked itself as exiting
        if (is_stream_stopping(stream_name)) {
            // Thread is still in the process of stopping
            log_info("Thread for stream %s is still stopping, waiting... (%d attempts left)",
                    stream_name, wait_attempts);
            usleep(500000); // Wait 500ms between checks
            wait_attempts--;

            // CRITICAL FIX: After a few attempts, try to cancel the thread
            if (wait_attempts == 5 && ctx && ctx->thread) {
                log_warn("Thread for stream %s is taking too long to exit, sending cancellation signal", stream_name);
                pthread_cancel(ctx->thread);
            }
        } else {
            // Thread has completed its shutdown
            log_info("Thread for stream %s has completed its shutdown", stream_name);
            thread_exited = true;
            break;
        }
    }

    // If the thread didn't exit properly, log a warning but continue with cleanup
    if (!thread_exited) {
        log_warn("Thread for stream %s did not exit properly after waiting, proceeding with cleanup anyway", stream_name);
        // Force unmark the stream as stopping to avoid deadlocks
        unmark_stream_stopping(stream_name);
    }

    // CRITICAL FIX: Use a shorter delay to avoid stalling during shutdown
    usleep(100000); // 100ms

    // Re-acquire the mutex for cleanup
    pthread_mutex_lock(&unified_contexts_mutex);

    // Verify context is still valid
    if (index >= 0 && index < MAX_STREAMS && unified_contexts[index] == ctx) {
        // First clear the slot in the array to prevent new accesses
        unified_contexts[index] = NULL;
        pthread_mutex_unlock(&unified_contexts_mutex);

        // Make sure the writer is properly closed before freeing the context
        if (ctx->writer) {
            log_info("Closing HLS writer for stream %s during context cleanup", stream_name);
            hls_writer_t *writer_to_close = ctx->writer;
            ctx->writer = NULL; // Clear the pointer first to prevent double-free
            hls_writer_close(writer_to_close);
        }

        // Ensure any FFmpeg resources are properly freed
        // This is a safety measure in case the thread didn't clean up properly
        if (ctx->thread) {
            log_info("Thread for stream %s may still be running, sending cancellation signal", stream_name);
            pthread_cancel(ctx->thread);
            // Wait a bit for the thread to exit
            usleep(100000); // 100ms
        }

        // Free the context structure
        log_info("Freeing context structure for stream %s", stream_name);
        memset(ctx, 0, sizeof(hls_unified_thread_ctx_t)); // Clear memory before freeing
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
