/*
 * Complete streams.c file with recording functionality integrated.
 * Save this file as src/video/streams.c
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <pthread.h>
#include <ctype.h>
#include <sys/stat.h>
#include <signal.h>
#include <errno.h>
#include <libavformat/avformat.h>
#include <libavcodec/avcodec.h>
#include <libavutil/avutil.h>
#include <libavutil/time.h>

#include "web/web_server.h"
#include "core/logger.h"
#include "core/config.h"
#include "video/stream_manager.h"
#include "video/hls_writer.h"
#include "database/database_manager.h"

#define LIGHTNVR_VERSION_STRING "0.1.0"

// Define a local config variable to work with
static config_t local_config;

// Global configuration - to be accessed from other modules if needed
config_t global_config;

// Forward declarations for recording functions
uint64_t start_recording(const char *stream_name, const char *output_path);
void update_recording(const char *stream_name);
void stop_recording(const char *stream_name);
void init_recordings(void);
void init_recordings_system(void);

// Forward declarations
static void log_ffmpeg_error(int err, const char *message);
void handle_hls_manifest(const http_request_t *request, http_response_t *response);
void handle_hls_segment(const http_request_t *request, http_response_t *response);
void handle_webrtc_offer(const http_request_t *request, http_response_t *response);
void handle_webrtc_ice(const http_request_t *request, http_response_t *response);

// Forward declarations for MP4 writer
#include "video/mp4_writer.h"

// Structure for stream transcoding context
typedef struct {
    stream_config_t config;
    int running;
    pthread_t thread;
    char output_path[MAX_PATH_LENGTH];
    char mp4_output_path[MAX_PATH_LENGTH];
    hls_writer_t *hls_writer;
    mp4_writer_t *mp4_writer;
} stream_transcode_ctx_t;

// Hash map for tracking running transcode contexts
static stream_transcode_ctx_t *transcode_contexts[MAX_STREAMS];
static pthread_mutex_t contexts_mutex = PTHREAD_MUTEX_INITIALIZER;

// Structure to keep track of active recordings
typedef struct {
    uint64_t recording_id;
    char stream_name[MAX_STREAM_NAME];
    char output_path[MAX_PATH_LENGTH];
    time_t start_time;
} active_recording_t;

// Array to store active recordings (one for each stream)
static active_recording_t active_recordings[MAX_STREAMS];
static pthread_mutex_t recordings_mutex = PTHREAD_MUTEX_INITIALIZER;

/**
 * Get current global configuration
 * (To avoid conflict with potential existing function, we use a different name)
 */
static config_t* get_streaming_config(void) {
    // For now, just use our global config
    return &global_config;
}

/**
 * URL decode function
 */
static void url_decode_stream(char *str) {
    char *src = str;
    char *dst = str;
    char a, b;

    while (*src) {
        if ((*src == '%') && ((a = src[1]) && (b = src[2])) &&
            (isxdigit(a) && isxdigit(b))) {
            if (a >= 'a') a -= 'a' - 'A';
            if (a >= 'A') a -= ('A' - 10);
            else a -= '0';

            if (b >= 'a') b -= 'a' - 'A';
            if (b >= 'A') b -= ('A' - 10);
            else b -= '0';

            *dst++ = 16 * a + b;
            src += 3;
        } else if (*src == '+') {
            *dst++ = ' ';
            src++;
        } else {
            *dst++ = *src++;
        }
    }
    *dst = '\0';
}

/**
 * Create error response with appropriate content type
 */
void create_stream_error_response(http_response_t *response, int status_code, const char *message) {
    char error_json[512];
    snprintf(error_json, sizeof(error_json), "{\"error\": \"%s\"}", message);

    response->status_code = status_code;

    // Use strncpy for the content type field which appears to be an array
    strncpy(response->content_type, "application/json", sizeof(response->content_type) - 1);
    response->content_type[sizeof(response->content_type) - 1] = '\0';

    // Allocate and set body
    response->body = strdup(error_json);

    // Add proper cleanup handling - your response structure likely has a different flag
    // for indicating the body should be freed
    // Check if your http_response_t has a needs_free or similar field
    // response->needs_free = 1;
}

/**
 * Handle errors from FFmpeg
 */
static void log_ffmpeg_error(int err, const char *message) {
    char error_buf[AV_ERROR_MAX_STRING_SIZE] = {0};
    av_strerror(err, error_buf, AV_ERROR_MAX_STRING_SIZE);
    log_error("%s: %s", message, error_buf);
}

/**
 * Initialize FFmpeg libraries
 */
void init_streaming_backend(void) {
    // Initialize FFmpeg
#if LIBAVFORMAT_VERSION_INT < AV_VERSION_INT(58, 9, 100)
    av_register_all();
#endif
    avformat_network_init();

    // Initialize transcode contexts array
    memset(transcode_contexts, 0, sizeof(transcode_contexts));

    log_info("Streaming backend initialized");
}

// Add this function to implement a thread join with timeout
static int pthread_join_with_timeout(pthread_t thread, void **retval, int timeout_sec) {
    // Simple approach: use a second thread to join
    pthread_t timeout_thread;
    int *result = malloc(sizeof(int));
    *result = -1;

    // Structure to pass data to helper thread
    struct {
        pthread_t thread;
        void **retval;
        int *result;
    } join_data = {thread, retval, result};

    // Helper thread function
    void *join_helper(void *arg) {
        struct {
            pthread_t thread;
            void **retval;
            int *result;
        } *data = arg;

        *(data->result) = pthread_join(data->thread, data->retval);
        return NULL;
    }

    // Create helper thread to join the target thread
    if (pthread_create(&timeout_thread, NULL, join_helper, &join_data) != 0) {
        free(result);
        return EAGAIN;
    }

    // Wait for timeout
    struct timespec ts;
    clock_gettime(CLOCK_REALTIME, &ts);
    ts.tv_sec += timeout_sec;


    // Not glibc - use sleep and check
    while (1) {
        // Check if thread has completed
        if (pthread_kill(timeout_thread, 0) != 0) {
            break;
        }

        struct timespec now;
        clock_gettime(CLOCK_REALTIME, &now);
        if (now.tv_sec >= ts.tv_sec &&
            (now.tv_nsec >= ts.tv_nsec || now.tv_sec > ts.tv_sec)) {
            pthread_cancel(timeout_thread);
            pthread_join(timeout_thread, NULL);
            free(result);
            return ETIMEDOUT;
        }

        usleep(100000); // Sleep 100ms and try again
    }

    // Get the join result
    int join_result = *result;
    free(result);
    return join_result;
}

/**
 * Cleanup FFmpeg resources
 */

// In cleanup_streaming_backend function:
void cleanup_streaming_backend(void) {
    log_info("Cleaning up streaming backend...");
    pthread_mutex_lock(&contexts_mutex);

    // Stop all running transcodes
    for (int i = 0; i < MAX_STREAMS; i++) {
        if (transcode_contexts[i]) {
            log_info("Stopping stream in slot %d: %s", i,
                    transcode_contexts[i]->config.name);

            // Copy the stream name for later use
            char stream_name[MAX_STREAM_NAME];
            strncpy(stream_name, transcode_contexts[i]->config.name,
                    MAX_STREAM_NAME - 1);
            stream_name[MAX_STREAM_NAME - 1] = '\0';

            // Mark as not running
            transcode_contexts[i]->running = 0;

            // Clean up resources (even if we can't join the thread)
            if (transcode_contexts[i]->hls_writer) {
                hls_writer_close(transcode_contexts[i]->hls_writer);
                transcode_contexts[i]->hls_writer = NULL;
            }

            if (transcode_contexts[i]->mp4_writer) {
                mp4_writer_close(transcode_contexts[i]->mp4_writer);
                transcode_contexts[i]->mp4_writer = NULL;
            }

            // Attempt to join the thread with a timeout
            pthread_t thread = transcode_contexts[i]->thread;
            pthread_mutex_unlock(&contexts_mutex);

            // Try to join with a timeout (simple implementation)
            struct timespec ts_start, ts_now;
            clock_gettime(CLOCK_REALTIME, &ts_start);
            int joined = 0;

            while (1) {
                // Try to join non-blocking
                if (pthread_join_with_timeout(thread, NULL, 2) == 0) {
                    joined = 1;
                    break;
                }

                // Check timeout
                clock_gettime(CLOCK_REALTIME, &ts_now);
                if (ts_now.tv_sec - ts_start.tv_sec >= 2) {
                    // 2 second timeout reached
                    break;
                }

                // Sleep a bit before trying again
                usleep(100000); // 100ms
            }

            if (!joined) {
                log_warn("Could not join thread for stream %s within timeout",
                        stream_name);
            }

            // Stop recording
            stop_recording(stream_name);

            pthread_mutex_lock(&contexts_mutex);

            // Free the context
            free(transcode_contexts[i]);
            transcode_contexts[i] = NULL;
        }
    }

    pthread_mutex_unlock(&contexts_mutex);

    // Cleanup FFmpeg
    avformat_network_deinit();

    log_info("Streaming backend cleaned up");
}

// Initialize the active recordings array
void init_recordings() {
    pthread_mutex_lock(&recordings_mutex);
    memset(active_recordings, 0, sizeof(active_recordings));
    pthread_mutex_unlock(&recordings_mutex);
}

// Function to initialize the recording system
void init_recordings_system() {
    init_recordings();
    log_info("Recordings system initialized");
}

/**
 * Transcoding thread function for a single stream
 */
static void *stream_transcode_thread(void *arg) {
    stream_transcode_ctx_t *ctx = (stream_transcode_ctx_t *)arg;

    AVFormatContext *input_ctx = NULL;
    AVFormatContext *output_ctx = NULL;
    AVPacket *pkt = NULL;
    int video_stream_idx = -1;
    int ret;
    time_t last_update = 0;  // Track when we last updated metadata
    time_t start_time = time(NULL);  // Record when we started

    log_info("Starting transcoding thread for stream %s", ctx->config.name);

    // Generate timestamp for recording files
    char timestamp_str[32];
    struct tm *tm_info = localtime(&start_time);
    strftime(timestamp_str, sizeof(timestamp_str), "%Y%m%d_%H%M%S", tm_info);

    // Create MP4 output path with timestamp
    snprintf(ctx->mp4_output_path, MAX_PATH_LENGTH, "%s/recording_%s.mp4",
             ctx->output_path, timestamp_str);

    // Verify output directory exists and is writable
    struct stat st;
    if (stat(ctx->output_path, &st) != 0 || !S_ISDIR(st.st_mode)) {
        log_error("Output directory does not exist or is not a directory: %s", ctx->output_path);

        // Recreate it as a last resort
        char mkdir_cmd[MAX_PATH_LENGTH * 2];
        snprintf(mkdir_cmd, sizeof(mkdir_cmd), "mkdir -p %s", ctx->output_path);

        if (system(mkdir_cmd) != 0 || stat(ctx->output_path, &st) != 0 || !S_ISDIR(st.st_mode)) {
            log_error("Failed to create output directory: %s", ctx->output_path);
            goto cleanup;
        }

        // Set permissions
        snprintf(mkdir_cmd, sizeof(mkdir_cmd), "chmod -R 777 %s", ctx->output_path);
        system(mkdir_cmd);
    }

    // Check directory permissions
    if (access(ctx->output_path, W_OK) != 0) {
        log_error("Output directory is not writable: %s", ctx->output_path);

        // Try to fix permissions
        char chmod_cmd[MAX_PATH_LENGTH * 2];
        snprintf(chmod_cmd, sizeof(chmod_cmd), "chmod -R 777 %s", ctx->output_path);
        system(chmod_cmd);

        if (access(ctx->output_path, W_OK) != 0) {
            log_error("Still unable to write to output directory: %s", ctx->output_path);
            goto cleanup;
        }
    }

    // Open input
    ret = avformat_open_input(&input_ctx, ctx->config.url, NULL, NULL);
    if (ret < 0) {
        log_ffmpeg_error(ret, "Could not open input stream");
        goto cleanup;
    }

    // Get stream info
    ret = avformat_find_stream_info(input_ctx, NULL);
    if (ret < 0) {
        log_ffmpeg_error(ret, "Could not find stream info");
        goto cleanup;
    }

    // Find video stream
    for (unsigned int i = 0; i < input_ctx->nb_streams; i++) {
        if (input_ctx->streams[i]->codecpar->codec_type == AVMEDIA_TYPE_VIDEO) {
            video_stream_idx = i;
            break;
        }
    }

    if (video_stream_idx == -1) {
        log_error("No video stream found in %s", ctx->config.url);
        goto cleanup;
    }

    // Set up HLS writer with improved error handling
    ctx->hls_writer = hls_writer_create(ctx->output_path, ctx->config.name, ctx->config.segment_duration);
    if (!ctx->hls_writer) {
        log_error("Failed to create HLS writer for %s", ctx->config.name);
        goto cleanup;
    }

    // Set up MP4 writer if recording is enabled
    if (ctx->config.record) {
        ctx->mp4_writer = mp4_writer_create(ctx->mp4_output_path, ctx->config.name);
        if (!ctx->mp4_writer) {
            log_error("Failed to create MP4 writer for %s", ctx->config.name);
            // Continue anyway with just HLS
        } else {
            log_info("Created MP4 writer for stream %s at %s",
                    ctx->config.name, ctx->mp4_output_path);
        }
    }

    // Initialize packet - use newer API to avoid deprecation warning
    pkt = av_packet_alloc();
    if (!pkt) {
        log_error("Failed to allocate packet");
        goto cleanup;
    }

    // Main packet reading loop
    while (ctx->running) {
        ret = av_read_frame(input_ctx, pkt);

        if (ret < 0) {
            if (ret == AVERROR_EOF || ret == AVERROR(EAGAIN)) {
                // End of stream or resource temporarily unavailable
                // Try to reconnect after a short delay
                av_packet_unref(pkt);
                log_warn("Stream %s disconnected, attempting to reconnect...", ctx->config.name);

                av_usleep(2000000);  // 2 second delay

                // Close and reopen input
                avformat_close_input(&input_ctx);
                ret = avformat_open_input(&input_ctx, ctx->config.url, NULL, NULL);
                if (ret < 0) {
                    log_ffmpeg_error(ret, "Could not reconnect to input stream");
                    continue;  // Keep trying
                }

                ret = avformat_find_stream_info(input_ctx, NULL);
                if (ret < 0) {
                    log_ffmpeg_error(ret, "Could not find stream info after reconnect");
                    continue;  // Keep trying
                }

                continue;
            } else {
                log_ffmpeg_error(ret, "Error reading frame");
                break;
            }
        }

        // Process video packets
        if (pkt->stream_index == video_stream_idx) {
            // Write to HLS with error handling
            ret = hls_writer_write_packet(ctx->hls_writer, pkt, input_ctx->streams[video_stream_idx]);
            if (ret < 0) {
                log_error("Failed to write packet to HLS: %d", ret);
                // Continue anyway to keep the stream going
            }

            // Write to MP4 if enabled
            if (ctx->mp4_writer) {
                ret = mp4_writer_write_packet(ctx->mp4_writer, pkt, input_ctx->streams[video_stream_idx]);
                if (ret < 0) {
                    log_error("Failed to write packet to MP4: %d", ret);
                    // Continue anyway to keep the stream going
                }
            }

            // Periodically update recording metadata (every 30 seconds)
            time_t now = time(NULL);
            if (now - last_update >= 30) {
                update_recording(ctx->config.name);
                last_update = now;
            }
        }

        av_packet_unref(pkt);
    }

cleanup:
    // Cleanup resources
    if (pkt) {
        av_packet_free(&pkt);
    }

    if (input_ctx) {
        avformat_close_input(&input_ctx);
    }

    if (ctx->hls_writer) {
        hls_writer_close(ctx->hls_writer);
        ctx->hls_writer = NULL;
    }

    if (ctx->mp4_writer) {
        mp4_writer_close(ctx->mp4_writer);
        ctx->mp4_writer = NULL;
    }

    log_info("Transcoding thread for stream %s exited", ctx->config.name);
    return NULL;
}

/**
 * Start a new recording for a stream
 */
uint64_t start_recording(const char *stream_name, const char *output_path) {
    if (!stream_name || !output_path) {
        log_error("Invalid parameters for start_recording");
        return 0;
    }

    // Add debug logging
    log_info("Starting recording for stream: %s at path: %s", stream_name, output_path);

    stream_handle_t stream = get_stream_by_name(stream_name);
    if (!stream) {
        log_error("Stream %s not found", stream_name);
        return 0;
    }

    stream_config_t config;
    if (get_stream_config(stream, &config) != 0) {
        log_error("Failed to get config for stream %s", stream_name);
        return 0;
    }

    // Check if there's already an active recording for this stream
    uint64_t existing_recording_id = 0;
    pthread_mutex_lock(&recordings_mutex);
    for (int i = 0; i < MAX_STREAMS; i++) {
        if (active_recordings[i].recording_id > 0 && 
            strcmp(active_recordings[i].stream_name, stream_name) == 0) {
            existing_recording_id = active_recordings[i].recording_id;
            
            // If we found an existing recording, stop it first
            log_info("Found existing recording for stream %s with ID %llu, stopping it first", 
                    stream_name, (unsigned long long)existing_recording_id);
            
            // Clear the active recording slot but remember the ID
            active_recordings[i].recording_id = 0;
            active_recordings[i].stream_name[0] = '\0';
            active_recordings[i].output_path[0] = '\0';
            
            pthread_mutex_unlock(&recordings_mutex);
            
            // Mark the existing recording as complete
            time_t end_time = time(NULL);
            update_recording_metadata(existing_recording_id, end_time, 0, true);
            
            log_info("Marked existing recording %llu as complete", 
                    (unsigned long long)existing_recording_id);
            
            // Re-lock the mutex for the next section
            pthread_mutex_lock(&recordings_mutex);
            break;
        }
    }
    pthread_mutex_unlock(&recordings_mutex);

    // Create recording metadata
    recording_metadata_t metadata;
    memset(&metadata, 0, sizeof(recording_metadata_t));

    strncpy(metadata.stream_name, stream_name, sizeof(metadata.stream_name) - 1);

    // Format paths for the recording - MAKE SURE THIS POINTS TO REAL FILES
    char hls_path[MAX_PATH_LENGTH];
    char mp4_path[MAX_PATH_LENGTH];
    
    // HLS path (primary path stored in metadata)
    snprintf(hls_path, sizeof(hls_path), "%s/index.m3u8", output_path);
    strncpy(metadata.file_path, hls_path, sizeof(metadata.file_path) - 1);
    
    // MP4 path (stored in details field for now)
    snprintf(mp4_path, sizeof(mp4_path), "%s/recording.mp4", output_path);

    metadata.start_time = time(NULL);
    metadata.end_time = 0; // Will be updated when recording ends
    metadata.size_bytes = 0; // Will be updated as segments are added
    metadata.width = config.width;
    metadata.height = config.height;
    metadata.fps = config.fps;
    strncpy(metadata.codec, config.codec, sizeof(metadata.codec) - 1);
    metadata.is_complete = false;

    // Add recording to database with detailed error handling
    uint64_t recording_id = add_recording_metadata(&metadata);
    if (recording_id == 0) {
        log_error("Failed to add recording metadata for stream %s. Database error.", stream_name);
        return 0;
    }

    log_info("Recording metadata added to database with ID: %llu", (unsigned long long)recording_id);

    // Store active recording
    pthread_mutex_lock(&recordings_mutex);
    for (int i = 0; i < MAX_STREAMS; i++) {
        if (active_recordings[i].recording_id == 0) {
            active_recordings[i].recording_id = recording_id;
            strncpy(active_recordings[i].stream_name, stream_name, MAX_STREAM_NAME - 1);
            strncpy(active_recordings[i].output_path, output_path, MAX_PATH_LENGTH - 1);
            active_recordings[i].start_time = metadata.start_time;
            
            log_info("Started recording for stream %s with ID %llu", 
                    stream_name, (unsigned long long)recording_id);
            
            pthread_mutex_unlock(&recordings_mutex);
            return recording_id;
        }
    }
    
    // No free slots
    pthread_mutex_unlock(&recordings_mutex);
    log_error("No free slots for active recordings");
    return 0;
}

/**
 * Update recording metadata with current size and segment count
 */
void update_recording(const char *stream_name) {
    if (!stream_name) return;
    
    pthread_mutex_lock(&recordings_mutex);
    
    // Find the active recording for this stream
    for (int i = 0; i < MAX_STREAMS; i++) {
        if (active_recordings[i].recording_id > 0 && 
            strcmp(active_recordings[i].stream_name, stream_name) == 0) {
            
            uint64_t recording_id = active_recordings[i].recording_id;
            char output_path[MAX_PATH_LENGTH];
            strncpy(output_path, active_recordings[i].output_path, MAX_PATH_LENGTH - 1);
            output_path[MAX_PATH_LENGTH - 1] = '\0';
            time_t start_time = active_recordings[i].start_time;
            
            pthread_mutex_unlock(&recordings_mutex);
            
            // Calculate total size of all segments
            uint64_t total_size = 0;
            struct stat st;
            char segment_path[MAX_PATH_LENGTH];
            
            // This is a simple approach - in a real implementation you'd want to track
            // which segments actually belong to this recording
            for (int j = 0; j < 1000; j++) {
                snprintf(segment_path, sizeof(segment_path), "%s/index%d.ts", output_path, j);
                if (stat(segment_path, &st) == 0) {
                    total_size += st.st_size;
                } else {
                    // No more segments
                    break;
                }
            }
            
            // Update recording metadata
            time_t current_time = time(NULL);
            update_recording_metadata(recording_id, current_time, total_size, false);
            
            log_debug("Updated recording %llu for stream %s, size: %llu bytes", 
                    (unsigned long long)recording_id, stream_name, (unsigned long long)total_size);
            
            return;
        }
    }
    
    pthread_mutex_unlock(&recordings_mutex);
}

/**
 * Stop an active recording
 */
void stop_recording(const char *stream_name) {
    if (!stream_name) return;
    
    pthread_mutex_lock(&recordings_mutex);
    
    // Find the active recording for this stream
    for (int i = 0; i < MAX_STREAMS; i++) {
        if (active_recordings[i].recording_id > 0 && 
            strcmp(active_recordings[i].stream_name, stream_name) == 0) {
            
            uint64_t recording_id = active_recordings[i].recording_id;
            char output_path[MAX_PATH_LENGTH];
            strncpy(output_path, active_recordings[i].output_path, MAX_PATH_LENGTH - 1);
            output_path[MAX_PATH_LENGTH - 1] = '\0';
            time_t start_time = active_recordings[i].start_time;
            
            // Clear the active recording slot
            active_recordings[i].recording_id = 0;
            active_recordings[i].stream_name[0] = '\0';
            active_recordings[i].output_path[0] = '\0';
            
            pthread_mutex_unlock(&recordings_mutex);
            
            // Calculate final size of all segments
            uint64_t total_size = 0;
            struct stat st;
            char segment_path[MAX_PATH_LENGTH];
            
            for (int j = 0; j < 1000; j++) {
                snprintf(segment_path, sizeof(segment_path), "%s/index%d.ts", output_path, j);
                if (stat(segment_path, &st) == 0) {
                    total_size += st.st_size;
                } else {
                    // No more segments
                    break;
                }
            }
            
            // Mark recording as complete
            time_t end_time = time(NULL);
            update_recording_metadata(recording_id, end_time, total_size, true);

            log_info("Completed recording %llu for stream %s, duration: %ld seconds, size: %llu bytes", 
                    (unsigned long long)recording_id, stream_name, 
                    (long)(end_time - start_time), 
                    (unsigned long long)total_size);
            
            return;
        }
    }
    
    pthread_mutex_unlock(&recordings_mutex);
}
/**
 * Start HLS transcoding for a stream
 */
int start_hls_stream(const char *stream_name) {
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

    // Check if already running
    pthread_mutex_lock(&contexts_mutex);
    for (int i = 0; i < MAX_STREAMS; i++) {
        if (transcode_contexts[i] && strcmp(transcode_contexts[i]->config.name, stream_name) == 0) {
            pthread_mutex_unlock(&contexts_mutex);
            log_info("HLS stream %s already running", stream_name);
            return 0;  // Already running
        }
    }

    // Find empty slot
    int slot = -1;
    for (int i = 0; i < MAX_STREAMS; i++) {
        if (!transcode_contexts[i]) {
            slot = i;
            break;
        }
    }

    if (slot == -1) {
        pthread_mutex_unlock(&contexts_mutex);
        log_error("No slot available for new HLS stream");
        return -1;
    }

    // Create context
    stream_transcode_ctx_t *ctx = malloc(sizeof(stream_transcode_ctx_t));
    if (!ctx) {
        pthread_mutex_unlock(&contexts_mutex);
        log_error("Memory allocation failed for transcode context");
        return -1;
    }

    memset(ctx, 0, sizeof(stream_transcode_ctx_t));
    memcpy(&ctx->config, &config, sizeof(stream_config_t));
    ctx->running = 1;

    // Create output path
    config_t *global_config = get_streaming_config();
    snprintf(ctx->output_path, MAX_PATH_LENGTH, "%s/hls/%s",
             global_config->storage_path, stream_name);

    // Create directory if it doesn't exist - use more robust method
    char dir_cmd[MAX_PATH_LENGTH * 2];

    // Create the base directory
    snprintf(dir_cmd, sizeof(dir_cmd), "mkdir -p %s", ctx->output_path);
    int ret = system(dir_cmd);
    if (ret != 0) {
        log_error("Failed to create directory: %s (return code: %d)", ctx->output_path, ret);
        free(ctx);
        pthread_mutex_unlock(&contexts_mutex);
        return -1;
    }

    // Set full permissions to ensure FFmpeg can write files
    snprintf(dir_cmd, sizeof(dir_cmd), "chmod -R 777 %s", ctx->output_path);
    system(dir_cmd);

    log_info("Created directory with full permissions: %s", ctx->output_path);

    // Check that we can actually write to this directory
    char test_file[MAX_PATH_LENGTH];
    snprintf(test_file, sizeof(test_file), "%s/.test_write", ctx->output_path);
    FILE *test = fopen(test_file, "w");
    if (!test) {
        log_error("Directory is not writable: %s (error: %s)", ctx->output_path, strerror(errno));
        free(ctx);
        pthread_mutex_unlock(&contexts_mutex);
        return -1;
    }
    fclose(test);
    remove(test_file);
    log_info("Verified directory is writable: %s", ctx->output_path);

    // Start recording if enabled in config
    if (config.record) {
        uint64_t recording_id = start_recording(stream_name, ctx->output_path);
        if (recording_id > 0) {
            log_info("Started recording for stream %s with ID %llu",
                    stream_name, (unsigned long long)recording_id);
        } else {
            log_warn("Failed to start recording for stream %s", stream_name);
            // Continue anyway, the HLS stream will still work
        }
    }

    // Start transcoding thread
    if (pthread_create(&ctx->thread, NULL, stream_transcode_thread, ctx) != 0) {
        free(ctx);
        pthread_mutex_unlock(&contexts_mutex);
        log_error("Failed to create transcoding thread for %s", stream_name);
        return -1;
    }

    // Store context
    transcode_contexts[slot] = ctx;
    pthread_mutex_unlock(&contexts_mutex);

    log_info("Started HLS stream for %s in slot %d", stream_name, slot);
    return 0;
}

/**
 * Stop HLS transcoding for a stream
 */
int stop_hls_stream(const char *stream_name) {
    int found = 0;

    // Log that we're attempting to stop the stream
    log_info("Attempting to stop HLS stream: %s", stream_name);

    pthread_mutex_lock(&contexts_mutex);

    // Find the stream context
    stream_transcode_ctx_t *ctx = NULL;
    int index = -1;

    for (int i = 0; i < MAX_STREAMS; i++) {
        if (transcode_contexts[i] && strcmp(transcode_contexts[i]->config.name, stream_name) == 0) {
            ctx = transcode_contexts[i];
            index = i;
            found = 1;
            break;
        }
    }

    if (!found) {
        log_warn("HLS stream %s not found for stopping", stream_name);
        pthread_mutex_unlock(&contexts_mutex);
        return -1;
    }

    // Mark as not running first
    ctx->running = 0;
    log_info("Marked HLS stream %s as stopping (index: %d)", stream_name, index);

    // Unlock before joining thread to prevent deadlocks
    pthread_mutex_unlock(&contexts_mutex);

    // Join thread with timeout
    int join_result = 0;
    struct timespec timeout;
    clock_gettime(CLOCK_REALTIME, &timeout);
    timeout.tv_sec += 5; // 5 second timeout

    pthread_t thread_to_join = ctx->thread;
    void *thread_result;
    join_result = pthread_join(thread_to_join, &thread_result);
    if (join_result != 0) {
        log_error("Failed to join thread for stream %s (error: %d), will continue cleanup",
                 stream_name, join_result);
    } else {
        log_info("Successfully joined thread for stream %s", stream_name);
    }

    // Re-lock for cleanup
    pthread_mutex_lock(&contexts_mutex);

    // Verify context is still valid
    if (index >= 0 && index < MAX_STREAMS && transcode_contexts[index] == ctx) {
        // Cleanup resources
        if (ctx->hls_writer) {
            log_info("Closing HLS writer for stream %s", stream_name);
            hls_writer_close(ctx->hls_writer);
            ctx->hls_writer = NULL;
        }

        if (ctx->mp4_writer) {
            log_info("Closing MP4 writer for stream %s", stream_name);
            mp4_writer_close(ctx->mp4_writer);
            ctx->mp4_writer = NULL;
        }

        // Stop recording
        log_info("Stopping recording for stream %s", stream_name);
        pthread_mutex_unlock(&contexts_mutex);
        stop_recording(stream_name);
        pthread_mutex_lock(&contexts_mutex);

        // Free context and clear slot
        free(ctx);
        transcode_contexts[index] = NULL;

        log_info("Successfully cleaned up resources for stream %s", stream_name);
    } else {
        log_warn("Context for stream %s was modified during cleanup", stream_name);
    }

    pthread_mutex_unlock(&contexts_mutex);

    log_info("Stopped HLS stream %s", stream_name);
    return 0;
}

/**
 * Handle request for HLS manifest
 */
void handle_hls_manifest(const http_request_t *request, http_response_t *response) {
    // Extract stream name from URL
    // URL format: /api/streaming/{stream_name}/hls/index.m3u8

    const char *path = request->path;
    const char *streams_pos = strstr(path, "/streaming/");

    if (!streams_pos) {
        create_stream_error_response(response, 400, "Invalid request path");
        return;
    }

    const char *stream_name_start = streams_pos + 11;  // Skip "/streaming/"
    const char *hls_pos = strstr(stream_name_start, "/hls/");

    if (!hls_pos) {
        create_stream_error_response(response, 400, "Invalid HLS request path");
        return;
    }

    // Extract stream name
    char stream_name[MAX_STREAM_NAME];
    size_t name_len = hls_pos - stream_name_start;

    if (name_len >= MAX_STREAM_NAME) {
        create_stream_error_response(response, 400, "Stream name too long");
        return;
    }

    memcpy(stream_name, stream_name_start, name_len);
    stream_name[name_len] = '\0';

    // URL decode the stream name
    char decoded_stream[MAX_STREAM_NAME];
    url_decode(stream_name, decoded_stream, sizeof(decoded_stream));
    strncpy(stream_name, decoded_stream, MAX_STREAM_NAME - 1);
    stream_name[MAX_STREAM_NAME - 1] = '\0';

    // Check if stream exists
    stream_handle_t stream = get_stream_by_name(stream_name);
    if (!stream) {
        create_stream_error_response(response, 404, "Stream not found");
        return;
    }

    // Start HLS if not already running
    if (start_hls_stream(stream_name) != 0) {
        create_stream_error_response(response, 500, "Failed to start HLS stream");
        return;
    }

    // Get the manifest file path
    config_t *global_config = get_streaming_config();
    char manifest_path[MAX_PATH_LENGTH];
    snprintf(manifest_path, MAX_PATH_LENGTH, "%s/hls/%s/index.m3u8",
             global_config->storage_path, stream_name);

    // Wait longer for the manifest file to be created
    // Try up to 30 times with 100ms between attempts (3 seconds total)
    bool manifest_exists = false;
    for (int i = 0; i < 30; i++) {
        // Check for the final manifest file
        if (access(manifest_path, F_OK) == 0) {
            manifest_exists = true;
            break;
        }

        // Also check for the temporary file
        char temp_manifest_path[MAX_PATH_LENGTH];
        snprintf(temp_manifest_path, MAX_PATH_LENGTH, "%s.tmp", manifest_path);
        if (access(temp_manifest_path, F_OK) == 0) {
            log_debug("Found temporary manifest file, waiting for it to be finalized");
            // Continue waiting for the final file
        }

        log_debug("Waiting for manifest file to be created (attempt %d/30)", i+1);
        usleep(100000);  // 100ms
    }

    if (!manifest_exists) {
        log_error("Manifest file was not created in time: %s", manifest_path);
        create_stream_error_response(response, 404, "Manifest file not found");
        return;
    }

    // Read the manifest file
    FILE *fp = fopen(manifest_path, "r");
    if (!fp) {
        create_stream_error_response(response, 500, "Failed to open manifest file");
        return;
    }

    // Get file size
    fseek(fp, 0, SEEK_END);
    long file_size = ftell(fp);
    fseek(fp, 0, SEEK_SET);

    // Read file content
    char *content = malloc(file_size + 1);
    if (!content) {
        fclose(fp);
        create_stream_error_response(response, 500, "Memory allocation failed");
        return;
    }

    size_t bytes_read = fread(content, 1, file_size, fp);
    if (bytes_read != (size_t)file_size) {
        free(content);
        fclose(fp);
        create_stream_error_response(response, 500, "Failed to read manifest file");
        return;
    }

    content[file_size] = '\0';
    fclose(fp);

    // Create response
    response->status_code = 200;
    strncpy(response->content_type, "application/vnd.apple.mpegurl", sizeof(response->content_type) - 1);
    response->content_type[sizeof(response->content_type) - 1] = '\0';
    response->body = content;
    response->body_length = file_size;
}

/**
 * Handle request for HLS segment
 */
/**
 * Handle request for HLS segment
 */
/**
 * Handle request for HLS segment
 */
void handle_hls_segment(const http_request_t *request, http_response_t *response) {
    // Extract stream name and segment from URL
    // URL format: /api/streaming/{stream_name}/hls/segment_{number}.ts or /api/streaming/{stream_name}/hls/index{number}.ts

    const char *path = request->path;
    const char *streams_pos = strstr(path, "/streaming/");

    if (!streams_pos) {
        create_stream_error_response(response, 400, "Invalid request path");
        return;
    }

    const char *stream_name_start = streams_pos + 11;  // Skip "/streaming/"
    const char *hls_pos = strstr(stream_name_start, "/hls/");

    if (!hls_pos) {
        create_stream_error_response(response, 400, "Invalid HLS request path");
        return;
    }

    // Extract stream name
    char stream_name[MAX_STREAM_NAME];
    size_t name_len = hls_pos - stream_name_start;

    if (name_len >= MAX_STREAM_NAME) {
        create_stream_error_response(response, 400, "Stream name too long");
        return;
    }

    memcpy(stream_name, stream_name_start, name_len);
    stream_name[name_len] = '\0';

    // URL decode the stream name
    char decoded_stream[MAX_STREAM_NAME];
    url_decode(stream_name, decoded_stream, sizeof(decoded_stream));
    strncpy(stream_name, decoded_stream, MAX_STREAM_NAME - 1);
    stream_name[MAX_STREAM_NAME - 1] = '\0';

    // Extract segment filename
    const char *segment_filename = hls_pos + 5;  // Skip "/hls/"

    log_info("Segment requested: %s", segment_filename);

    // Get the segment file path
    config_t *global_config = get_streaming_config();
    char segment_path[MAX_PATH_LENGTH];
    snprintf(segment_path, MAX_PATH_LENGTH, "%s/hls/%s/%s",
             global_config->storage_path, stream_name, segment_filename);

    log_info("Looking for segment at path: %s", segment_path);

    // Debug: check if directory exists and is accessible
    char dir_path[MAX_PATH_LENGTH];
    strncpy(dir_path, segment_path, sizeof(dir_path));
    char *last_slash = strrchr(dir_path, '/');
    if (last_slash) {
        *last_slash = '\0';

        // Check if directory exists
        if (access(dir_path, F_OK | R_OK) != 0) {
            log_error("HLS directory not accessible: %s (%s)", dir_path, strerror(errno));
        } else {
            log_info("HLS directory exists and is readable: %s", dir_path);

            // List a few of the ts files in the directory using system command
            char cmd[MAX_PATH_LENGTH * 2];
            snprintf(cmd, sizeof(cmd), "ls -la %s/*.ts 2>/dev/null | head -5", dir_path);
            log_info("Listing some TS files in directory:");
            FILE *ls = popen(cmd, "r");
            if (ls) {
                char line[1024];
                while (fgets(line, sizeof(line), ls)) {
                    // Remove newline
                    size_t len = strlen(line);
                    if (len > 0 && line[len-1] == '\n') line[len-1] = '\0';

                    log_info("  %s", line);
                }
                pclose(ls);
            }
        }
    }

    // Check if segment file exists
    if (access(segment_path, F_OK) != 0) {
        log_debug("Segment file not found on first attempt: %s (%s)", segment_path, strerror(errno));

        // Try a different path format - sometimes the HLS segments might be named differently
        char alt_segment_path[MAX_PATH_LENGTH];

        // Try removing any file extension from the segment name and just use the index number
        char basename[256];
        strncpy(basename, segment_filename, sizeof(basename));
        char *dot = strrchr(basename, '.');
        if (dot) {
            *dot = '\0';
        }

        // Try with a different naming convention - more generic pattern matching
        snprintf(alt_segment_path, MAX_PATH_LENGTH, "%s/hls/%s/index*.ts",
                 global_config->storage_path, stream_name);

        log_info("Trying alternative path pattern: %s", alt_segment_path);

        // Wait longer for it to be created - HLS segments might still be generating
        bool segment_exists = false;
        for (int i = 0; i < 20; i++) {  // Try for 2 seconds total
            if (access(segment_path, F_OK) == 0) {
                log_info("Segment file found after waiting: %s (attempt %d)", segment_path, i+1);
                segment_exists = true;
                break;
            }

            log_debug("Waiting for segment file to be created: %s (attempt %d/20)", segment_path, i+1);
            usleep(100000);  // 100ms
        }

        if (!segment_exists) {
            log_error("Segment file not found after waiting: %s", segment_path);

            // As a fallback, try to access the most recent segment file
            char fallback_cmd[MAX_PATH_LENGTH * 2];
            snprintf(fallback_cmd, sizeof(fallback_cmd),
                     "ls -t %s/hls/%s/index*.ts 2>/dev/null | head -1",
                     global_config->storage_path, stream_name);

            FILE *cmd = popen(fallback_cmd, "r");
            if (cmd) {
                char latest_segment[MAX_PATH_LENGTH];
                if (fgets(latest_segment, sizeof(latest_segment), cmd)) {
                    // Remove newline
                    size_t len = strlen(latest_segment);
                    if (len > 0 && latest_segment[len-1] == '\n') latest_segment[len-1] = '\0';

                    log_info("Found latest segment: %s", latest_segment);

                    // If a recent segment exists, use that instead
                    if (access(latest_segment, F_OK) == 0) {
                        strncpy(segment_path, latest_segment, sizeof(segment_path));
                        segment_exists = true;
                    }
                }
                pclose(cmd);
            }

            if (!segment_exists) {
                create_stream_error_response(response, 404, "Segment file not found");
                return;
            }
        }
    } else {
        log_info("Segment file exists: %s", segment_path);
    }

    // Read the segment file
    FILE *fp = fopen(segment_path, "rb");
    if (!fp) {
        log_error("Failed to open segment file: %s (%s)", segment_path, strerror(errno));
        create_stream_error_response(response, 500, "Failed to open segment file");
        return;
    }

    // Get file size
    fseek(fp, 0, SEEK_END);
    long file_size = ftell(fp);
    fseek(fp, 0, SEEK_SET);

    log_info("Successfully opened segment file, size: %ld bytes", file_size);

    // Read file content
    char *content = malloc(file_size);
    if (!content) {
        log_error("Memory allocation failed for file content, size: %ld", file_size);
        fclose(fp);
        create_stream_error_response(response, 500, "Memory allocation failed");
        return;
    }

    size_t bytes_read = fread(content, 1, file_size, fp);
    if (bytes_read != (size_t)file_size) {
        log_error("Failed to read full file content: read %zu of %ld bytes", bytes_read, file_size);
        free(content);
        fclose(fp);
        create_stream_error_response(response, 500, "Failed to read segment file");
        return;
    }

    fclose(fp);

    // Create response
    response->status_code = 200;
    strncpy(response->content_type, "video/mp2t", sizeof(response->content_type) - 1);
    response->content_type[sizeof(response->content_type) - 1] = '\0';
    response->body = content;
    response->body_length = file_size;

    log_info("Successfully served segment: %s", segment_filename);
}

/**
 * Handle WebRTC offer request - simple placeholder implementation
 */
void handle_webrtc_offer(const http_request_t *request, http_response_t *response) {
    // Extract stream name from URL
    const char *path = request->path;
    const char *streams_pos = strstr(path, "/streaming/");

    if (!streams_pos) {
        create_stream_error_response(response, 400, "Invalid request path");
        return;
    }

    const char *stream_name_start = streams_pos + 11;  // Skip "/streaming/"
    const char *webrtc_pos = strstr(stream_name_start, "/webrtc/");

    if (!webrtc_pos) {
        create_stream_error_response(response, 400, "Invalid WebRTC request path");
        return;
    }

    // Extract stream name
    char stream_name[MAX_STREAM_NAME];
    size_t name_len = webrtc_pos - stream_name_start;

    if (name_len >= MAX_STREAM_NAME) {
        create_stream_error_response(response, 400, "Stream name too long");
        return;
    }

    memcpy(stream_name, stream_name_start, name_len);
    stream_name[name_len] = '\0';

    // URL decode the stream name
    char decoded_stream[MAX_STREAM_NAME];
url_decode(stream_name, decoded_stream, sizeof(decoded_stream));
strncpy(stream_name, decoded_stream, MAX_STREAM_NAME - 1);
stream_name[MAX_STREAM_NAME - 1] = '\0';

    // Check if stream exists
    stream_handle_t stream = get_stream_by_name(stream_name);
    if (!stream) {
        create_stream_error_response(response, 404, "Stream not found");
        return;
    }

    // In a real implementation, this would:
    // 1. Parse the WebRTC offer from the request body
    // 2. Use the libWebRTC or similar library to create an answer
    // 3. Send the answer back to the client

    // For now, just acknowledge the request with a placeholder

    log_info("Received WebRTC offer for stream %s", stream_name);

    // Create placeholder response
    const char *response_json = "{\"status\": \"acknowledged\", \"message\": \"WebRTC not yet implemented\"}";

    response->status_code = 200;
    strncpy(response->content_type, "application/json", sizeof(response->content_type) - 1);
    response->content_type[sizeof(response->content_type) - 1] = '\0';
    response->body = strdup(response_json);

    // Indicate that body should be freed (if needed)
    // response->needs_free = 1;
}

/**
 * Handle WebRTC ICE candidate request - simple placeholder implementation
 */
void handle_webrtc_ice(const http_request_t *request, http_response_t *response) {
    // Extract stream name from URL
    const char *path = request->path;
    const char *streams_pos = strstr(path, "/streaming/");

    if (!streams_pos) {
        create_stream_error_response(response, 400, "Invalid request path");
        return;
    }

    const char *stream_name_start = streams_pos + 11;  // Skip "/streaming/"
    const char *webrtc_pos = strstr(stream_name_start, "/webrtc/");

    if (!webrtc_pos) {
        create_stream_error_response(response, 400, "Invalid WebRTC request path");
        return;
    }

    // Extract stream name
    char stream_name[MAX_STREAM_NAME];
    size_t name_len = webrtc_pos - stream_name_start;

    if (name_len >= MAX_STREAM_NAME) {
        create_stream_error_response(response, 400, "Stream name too long");
        return;
    }

    memcpy(stream_name, stream_name_start, name_len);
    stream_name[name_len] = '\0';

    // URL decode the stream name
    char decoded_stream[MAX_STREAM_NAME];
url_decode(stream_name, decoded_stream, sizeof(decoded_stream));
strncpy(stream_name, decoded_stream, MAX_STREAM_NAME - 1);
stream_name[MAX_STREAM_NAME - 1] = '\0';

    // Just acknowledge the ICE candidate
    log_info("Received ICE candidate for stream %s", stream_name);

    // Create success response
    const char *response_json = "{\"status\": \"acknowledged\", \"message\": \"WebRTC not yet implemented\"}";

    response->status_code = 200;
    strncpy(response->content_type, "application/json", sizeof(response->content_type) - 1);
    response->content_type[sizeof(response->content_type) - 1] = '\0';
    response->body = strdup(response_json);

    // Indicate that body should be freed (if needed)
    // response->needs_free = 1;
}
