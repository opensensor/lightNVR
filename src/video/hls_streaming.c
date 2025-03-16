#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <pthread.h>
#include <ctype.h>
#include <sys/stat.h>
#include <sys/types.h>
#include <fcntl.h>
#include <signal.h>
#include <errno.h>
#include <time.h>
#include <dirent.h>
#include <libavformat/avformat.h>
#include <libavcodec/avcodec.h>
#include <libavutil/avutil.h>
#include <libavutil/time.h>

// Define CLOCK_REALTIME if not available
#ifndef CLOCK_REALTIME
#define CLOCK_REALTIME 0
#endif

#include "web/web_server.h"
#include "core/logger.h"
#include "core/config.h"
#include "video/stream_manager.h"
#include "video/streams.h"
#include "video/hls_writer.h"
#include "video/mp4_writer.h"
#include "video/detection_integration.h"
#include "database/database_manager.h"

// Hash map for tracking running transcode contexts
static stream_transcode_ctx_t *transcode_contexts[MAX_STREAMS];
static pthread_mutex_t contexts_mutex = PTHREAD_MUTEX_INITIALIZER;

// Forward declarations
static void log_ffmpeg_error(int err, const char *message);
static void *stream_transcode_thread(void *arg);
static int pthread_join_with_timeout(pthread_t thread, void **retval, int timeout_sec);

/**
 * Handle errors from FFmpeg
 */
static void log_ffmpeg_error(int err, const char *message) {
    char error_buf[AV_ERROR_MAX_STRING_SIZE] = {0};
    av_strerror(err, error_buf, AV_ERROR_MAX_STRING_SIZE);
    log_error("%s: %s", message, error_buf);
}

/**
 * Helper thread function for pthread_join_with_timeout
 */
static void *join_helper(void *arg) {
    struct {
        pthread_t thread;
        void **retval;
        int *result;
    } *data = arg;

    *(data->result) = pthread_join(data->thread, data->retval);
    return NULL;
}

/**
 * Add this function to implement a thread join with timeout
 */
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
    config_t *global_config = get_streaming_config();

    log_info("Starting transcoding thread for stream %s", ctx->config.name);

    // Generate timestamp for recording files
    char timestamp_str[32];
    struct tm *tm_info = localtime(&start_time);
    strftime(timestamp_str, sizeof(timestamp_str), "%Y%m%d_%H%M%S", tm_info);

    // Create MP4 output path with timestamp - ensure it's within our configured storage
    snprintf(ctx->mp4_output_path, MAX_PATH_LENGTH, "%s/mp4/%s/recording_%s.mp4",
            global_config->storage_path, ctx->config.name, timestamp_str);
    
    // Log the MP4 output path for debugging
    log_info("MP4 output path: %s", ctx->mp4_output_path);

    // Verify output directory exists and is writable
    struct stat st;
    if (stat(ctx->output_path, &st) != 0 || !S_ISDIR(st.st_mode)) {
        log_error("Output directory does not exist or is not a directory: %s", ctx->output_path);

        // Recreate it as a last resort
        char mkdir_cmd[MAX_PATH_LENGTH * 2];
        snprintf(mkdir_cmd, sizeof(mkdir_cmd), "mkdir -p %s", ctx->output_path);

        int ret_mkdir = system(mkdir_cmd);
        if (ret_mkdir != 0 || stat(ctx->output_path, &st) != 0 || !S_ISDIR(st.st_mode)) {
            log_error("Failed to create output directory: %s (return code: %d)", ctx->output_path, ret_mkdir);
            goto cleanup;
        }

        // Set permissions
        snprintf(mkdir_cmd, sizeof(mkdir_cmd), "chmod -R 777 %s", ctx->output_path);
        int ret_chmod = system(mkdir_cmd);
        if (ret_chmod != 0) {
            log_warn("Failed to set permissions on directory: %s (return code: %d)", ctx->output_path, ret_chmod);
        }
        
        log_info("Successfully created output directory: %s", ctx->output_path);
    }

    // Check directory permissions
    if (access(ctx->output_path, W_OK) != 0) {
        log_error("Output directory is not writable: %s", ctx->output_path);

        // Try to fix permissions
        char chmod_cmd[MAX_PATH_LENGTH * 2];
        snprintf(chmod_cmd, sizeof(chmod_cmd), "chmod -R 777 %s", ctx->output_path);
        int ret_chmod = system(chmod_cmd);
        if (ret_chmod != 0) {
            log_warn("Failed to set permissions on directory: %s (return code: %d)", ctx->output_path, ret_chmod);
        }

        if (access(ctx->output_path, W_OK) != 0) {
            log_error("Still unable to write to output directory: %s", ctx->output_path);
            goto cleanup;
        }
        
        log_info("Successfully fixed permissions for output directory: %s", ctx->output_path);
    }
    
    // Create a parent directory check file to ensure the parent directory exists
    char parent_dir[MAX_PATH_LENGTH];
    const char *last_slash = strrchr(ctx->output_path, '/');
    if (last_slash) {
        size_t parent_len = last_slash - ctx->output_path;
        strncpy(parent_dir, ctx->output_path, parent_len);
        parent_dir[parent_len] = '\0';
        
        // Create a test file in the parent directory
        char test_file[MAX_PATH_LENGTH];
        snprintf(test_file, sizeof(test_file), "%s/.hls_parent_check", parent_dir);
        FILE *fp = fopen(test_file, "w");
        if (fp) {
            fclose(fp);
            // Leave the file there as a marker
            log_info("Verified parent directory is writable: %s", parent_dir);
        } else {
            log_warn("Parent directory may not be writable: %s (error: %s)", 
                    parent_dir, strerror(errno));
            
            // Try to create parent directory with full permissions
            char parent_cmd[MAX_PATH_LENGTH * 2];
            snprintf(parent_cmd, sizeof(parent_cmd), "mkdir -p %s && chmod -R 777 %s", 
                    parent_dir, parent_dir);
            int ret_parent = system(parent_cmd);
            if (ret_parent != 0) {
                log_warn("Failed to create parent directory: %s (return code: %d)", parent_dir, ret_parent);
            }
            
            log_info("Attempted to recreate parent directory with full permissions: %s", parent_dir);
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

    // Create HLS writer - adding the segment_duration parameter
    // Using a default of 2 seconds if not specified in config
    int segment_duration = ctx->config.segment_duration > 0 ?
                          ctx->config.segment_duration : 2;

    ctx->hls_writer = hls_writer_create(ctx->output_path, ctx->config.name, segment_duration);
    if (!ctx->hls_writer) {
        log_error("Failed to create HLS writer for %s", ctx->config.name);
        ctx->running = 0;
        return NULL;
    }

    // Only create MP4 writer if path is specified (not empty)
    if (ctx->mp4_output_path[0] != '\0') {
        ctx->mp4_writer = mp4_writer_create(ctx->mp4_output_path, ctx->config.name);
        if (!ctx->mp4_writer) {
            log_error("Failed to create MP4 writer for %s", ctx->config.name);
            // Continue anyway, HLS streaming will work
        } else {
            log_info("Created MP4 writer for %s at %s", ctx->config.name, ctx->mp4_output_path);
        }
    } else {
        // Get MP4 writer that might have been registered separately
        ctx->mp4_writer = get_mp4_writer_for_stream(ctx->config.name);
        if (ctx->mp4_writer) {
            log_info("Using separately registered MP4 writer for %s", ctx->config.name);
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
        // Check if we need to rotate the MP4 file based on segment duration
        time_t current_time = time(NULL);
        int segment_duration = ctx->config.segment_duration > 0 ? ctx->config.segment_duration : 900; // Default to 15 minutes
        
        // If the MP4 file has been open for longer than the segment duration, rotate it
        if (ctx->mp4_writer && (current_time - ctx->mp4_writer->creation_time) >= segment_duration) {
            log_info("Rotating MP4 file for stream %s after %d seconds", ctx->config.name, segment_duration);
            
            // Close the current MP4 writer
            mp4_writer_close(ctx->mp4_writer);
            ctx->mp4_writer = NULL;
            
            // Generate new timestamp for the new MP4 file
            char timestamp_str[32];
            struct tm *tm_info = localtime(&current_time);
            strftime(timestamp_str, sizeof(timestamp_str), "%Y%m%d_%H%M%S", tm_info);
            
            // Create new MP4 output path with new timestamp
            snprintf(ctx->mp4_output_path, MAX_PATH_LENGTH, "%s/mp4/%s/recording_%s.mp4",
                    global_config->storage_path, ctx->config.name, timestamp_str);
            
            // Create new MP4 writer
            ctx->mp4_writer = mp4_writer_create(ctx->mp4_output_path, ctx->config.name);
            if (!ctx->mp4_writer) {
                log_error("Failed to create new MP4 writer for stream %s during rotation", ctx->config.name);
            } else {
                log_info("Created new MP4 writer for stream %s at %s", ctx->config.name, ctx->mp4_output_path);
            }
        }
        
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
            // Check if this is a key frame
            bool is_key_frame = (pkt->flags & AV_PKT_FLAG_KEY) != 0;
            
            // Write to HLS with error handling
            ret = hls_writer_write_packet(ctx->hls_writer, pkt, input_ctx->streams[video_stream_idx]);
            if (ret < 0) {
                log_error("Failed to write packet to HLS for stream %s: %d", ctx->config.name, ret);
                // Continue anyway to keep the stream going
            }

            // Write to MP4 if enabled - only write key frames if we're having issues
            if (ctx->mp4_writer) {
                ret = mp4_writer_write_packet(ctx->mp4_writer, pkt, input_ctx->streams[video_stream_idx]);
                if (ret < 0) {
                    log_error("Failed to write packet to MP4 for stream %s: %d", ctx->config.name, ret);
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

    // When done, close writers
    if (ctx->hls_writer) {
        hls_writer_close(ctx->hls_writer);
        ctx->hls_writer = NULL;
    }

    // Only close the MP4 writer if we created it here
    // If it came from get_mp4_writer_for_stream, it will be closed elsewhere
    if (ctx->mp4_writer && ctx->mp4_output_path[0] != '\0') {
        mp4_writer_close(ctx->mp4_writer);
        ctx->mp4_writer = NULL;
    }

    log_info("Transcoding thread for stream %s exited", ctx->config.name);
    return NULL;
}

/**
 * Start HLS transcoding for a stream without MP4 recording
 * This function handles only the HLS streaming
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

    // Create output paths
    config_t *global_config = get_streaming_config();

    // Create HLS output path
    snprintf(ctx->output_path, MAX_PATH_LENGTH, "%s/hls/%s",
             global_config->storage_path, stream_name);

    // IMPORTANT: Set mp4_output_path to empty string to indicate no MP4 recording
    ctx->mp4_output_path[0] = '\0';

    // Create HLS directory if it doesn't exist
    char dir_cmd[MAX_PATH_LENGTH * 2];
    snprintf(dir_cmd, sizeof(dir_cmd), "mkdir -p %s", ctx->output_path);
    int ret = system(dir_cmd);
    if (ret != 0) {
        log_error("Failed to create HLS directory: %s (return code: %d)", ctx->output_path, ret);
        free(ctx);
        pthread_mutex_unlock(&contexts_mutex);
        return -1;
    }

    // Set full permissions to ensure FFmpeg can write files
    snprintf(dir_cmd, sizeof(dir_cmd), "chmod -R 777 %s", ctx->output_path);
    system(dir_cmd);

    log_info("Created HLS directory with full permissions: %s", ctx->output_path);

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
    log_info("Verified HLS directory is writable: %s", ctx->output_path);

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

    log_info("Started HLS stream for %s in slot %d (no MP4 recording)", stream_name, slot);

    // Start MP4 recording separately if enabled in config
    if (config.record) {
        start_mp4_recording(stream_name);
    }

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

/**
 * Cleanup FFmpeg resources
 */
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

            pthread_mutex_lock(&contexts_mutex);

            // Clean up resources AFTER attempting to join the thread
            if (transcode_contexts[i] && transcode_contexts[i]->hls_writer) {
                hls_writer_close(transcode_contexts[i]->hls_writer);
                transcode_contexts[i]->hls_writer = NULL;
            }

            if (transcode_contexts[i] && transcode_contexts[i]->mp4_writer) {
                mp4_writer_close(transcode_contexts[i]->mp4_writer);
                transcode_contexts[i]->mp4_writer = NULL;
            }

            // Stop recording
            pthread_mutex_unlock(&contexts_mutex);
            stop_recording(stream_name);
            pthread_mutex_lock(&contexts_mutex);

            // Free the context if it still exists
            if (transcode_contexts[i]) {
                free(transcode_contexts[i]);
                transcode_contexts[i] = NULL;
            }
        }
    }

    pthread_mutex_unlock(&contexts_mutex);

    // Cleanup FFmpeg
    avformat_network_deinit();

    log_info("Streaming backend cleaned up");
}

/**
 * Clean up HLS directories during shutdown
 * This function removes old HLS segment files (.ts) and temporary playlists (.m3u8.tmp)
 * but preserves active playlists to prevent issues with ongoing streams
 */
void cleanup_hls_directories(void) {
    config_t *global_config = get_streaming_config();
    
    if (!global_config || !global_config->storage_path) {
        log_error("Cannot clean up HLS directories: global config or storage path is NULL");
        return;
    }
    
    char hls_base_dir[MAX_PATH_LENGTH];
    snprintf(hls_base_dir, MAX_PATH_LENGTH, "%s/hls", global_config->storage_path);
    
    // Check if HLS base directory exists
    struct stat st;
    if (stat(hls_base_dir, &st) != 0 || !S_ISDIR(st.st_mode)) {
        log_info("HLS base directory does not exist, nothing to clean up: %s", hls_base_dir);
        return;
    }
    
    log_info("Cleaning up HLS directories in: %s", hls_base_dir);
    
    // Open the HLS base directory
    DIR *dir = opendir(hls_base_dir);
    if (!dir) {
        log_error("Failed to open HLS base directory for cleanup: %s (error: %s)", 
                 hls_base_dir, strerror(errno));
        return;
    }
    
    // Iterate through each stream directory
    struct dirent *entry;
    while ((entry = readdir(dir)) != NULL) {
        // Skip . and .. directories
        if (strcmp(entry->d_name, ".") == 0 || strcmp(entry->d_name, "..") == 0) {
            continue;
        }
        
        // Form the full path to the stream's HLS directory
        char stream_hls_dir[MAX_PATH_LENGTH];
        snprintf(stream_hls_dir, MAX_PATH_LENGTH, "%s/%s", hls_base_dir, entry->d_name);
        
        // Check if it's a directory
        if (stat(stream_hls_dir, &st) == 0 && S_ISDIR(st.st_mode)) {
            log_info("Cleaning up HLS files for stream: %s", entry->d_name);
            
            // Check if this stream is currently active
            bool is_active = false;
            pthread_mutex_lock(&contexts_mutex);
            for (int i = 0; i < MAX_STREAMS; i++) {
                if (transcode_contexts[i] && 
                    strcmp(transcode_contexts[i]->config.name, entry->d_name) == 0 &&
                    transcode_contexts[i]->running) {
                    is_active = true;
                    break;
                }
            }
            pthread_mutex_unlock(&contexts_mutex);
            
            if (is_active) {
                log_info("Stream %s is active, skipping cleanup of main playlist file", entry->d_name);
                
                // For active streams, only remove temporary files and old segments
                // but preserve the main index.m3u8 file
                
                // Remove temporary .m3u8.tmp files
                char rm_cmd[MAX_PATH_LENGTH * 2];
                snprintf(rm_cmd, sizeof(rm_cmd), "rm -f %s/*.m3u8.tmp", stream_hls_dir);
                system(rm_cmd);
                
                // Only remove segments that are older than 5 minutes
                // This ensures we don't delete segments that might still be in use
                snprintf(rm_cmd, sizeof(rm_cmd), 
                        "find %s -name \"*.ts\" -type f -mmin +5 -delete", 
                        stream_hls_dir);
                system(rm_cmd);
                
                log_info("Cleaned up temporary files for active stream: %s", entry->d_name);
            } else {
                // For inactive streams, we can safely remove all files
                log_info("Stream %s is inactive, removing all HLS files", entry->d_name);
                
                // Remove all .ts segment files
                char rm_cmd[MAX_PATH_LENGTH * 2];
                snprintf(rm_cmd, sizeof(rm_cmd), "rm -f %s/*.ts", stream_hls_dir);
                int ret = system(rm_cmd);
                if (ret != 0) {
                    log_warn("Failed to remove HLS segment files in %s (return code: %d)", 
                            stream_hls_dir, ret);
                } else {
                    log_info("Removed HLS segment files in %s", stream_hls_dir);
                }
                
                // Remove all .m3u8 playlist files
                snprintf(rm_cmd, sizeof(rm_cmd), "rm -f %s/*.m3u8*", stream_hls_dir);
                ret = system(rm_cmd);
                if (ret != 0) {
                    log_warn("Failed to remove HLS playlist files in %s (return code: %d)", 
                            stream_hls_dir, ret);
                } else {
                    log_info("Removed HLS playlist files in %s", stream_hls_dir);
                }
            }
            
            // Ensure the directory has proper permissions
            char chmod_cmd[MAX_PATH_LENGTH * 2];
            snprintf(chmod_cmd, sizeof(chmod_cmd), "chmod -R 777 %s", stream_hls_dir);
            system(chmod_cmd);
        }
    }
    
    closedir(dir);
    log_info("HLS directory cleanup completed");
}
