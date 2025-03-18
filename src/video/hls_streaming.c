#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <pthread.h>
#include <sys/stat.h>
#include <sys/types.h>
#include <fcntl.h>
#include <errno.h>
#include <time.h>
#include <dirent.h>

// Define CLOCK_REALTIME if not available
#ifndef CLOCK_REALTIME
#define CLOCK_REALTIME 0
#endif

#include <libavformat/avformat.h>
#include <libavcodec/avcodec.h>
#include <libavutil/avutil.h>
#include <libavutil/time.h>

#include "core/logger.h"
#include "core/config.h"
#include "video/stream_manager.h"
#include "video/streams.h"
#include "video/hls_writer.h"
#include "video/hls_streaming.h"
#include "video/stream_transcoding.h"
#include "video/stream_reader.h"
#include "video/detection_stream.h"
#include "database/database_manager.h"

// Hash map for tracking running HLS streaming contexts
static hls_stream_ctx_t *streaming_contexts[MAX_STREAMS];
static pthread_mutex_t contexts_mutex = PTHREAD_MUTEX_INITIALIZER;

// Forward declarations
static void *hls_stream_thread(void *arg);

/**
 * HLS packet processing callback function
 * Removed adaptive degrading to improve quality
 */
static int hls_packet_callback(const AVPacket *pkt, const AVStream *stream, void *user_data) {
    // CRITICAL FIX: Add extra validation for user_data to prevent segfaults
    if (!user_data) {
        log_error("HLS packet callback received NULL user_data");
        return -1;
    }
    
    hls_stream_ctx_t *streaming_ctx = (hls_stream_ctx_t *)user_data;
    
    // CRITICAL FIX: Add extra validation for streaming context and writer
    if (!streaming_ctx) {
        log_error("HLS packet callback received invalid streaming context");
        return -1;
    }
    
    if (!streaming_ctx->hls_writer) {
        log_error("HLS packet callback: streaming context has NULL hls_writer for stream %s", 
                 streaming_ctx->config.name);
        return -1;
    }
    
    // CRITICAL FIX: Validate that the stream is still running
    if (!streaming_ctx->running) {
        log_debug("HLS packet callback: stream %s is no longer running, skipping packet", 
                 streaming_ctx->config.name);
        return 0; // Return success but don't process the packet
    }
    
    // Check if this is a key frame
    bool is_key_frame = (pkt->flags & AV_PKT_FLAG_KEY) != 0;
    
    // Always set pressure flag to 0 to ensure we never drop frames
    streaming_ctx->hls_writer->is_under_pressure = 0;
    
    // Process all frames for better quality
    int ret = process_video_packet(pkt, stream, streaming_ctx->hls_writer, 0, streaming_ctx->config.name);
    
    // Only log errors for key frames to reduce log spam
    if (ret < 0 && is_key_frame) {
        log_error("Failed to write keyframe to HLS for stream %s: %d", streaming_ctx->config.name, ret);
        return ret;
    }
    
    return 0;
}

/**
 * HLS streaming thread function for a single stream
 */
static void *hls_stream_thread(void *arg) {
    hls_stream_ctx_t *ctx = (hls_stream_ctx_t *)arg;
    int ret;
    time_t start_time = time(NULL);  // Record when we started
    stream_reader_ctx_t *reader_ctx = NULL;

    log_info("Starting HLS streaming thread for stream %s", ctx->config.name);

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
            ctx->running = 0;
            return NULL;
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
            ctx->running = 0;
            return NULL;
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

    // Create HLS writer - adding the segment_duration parameter
    // Using a default of 4 seconds if not specified in config
    // Increased from 2 to 4 seconds for better compatibility with low-powered devices
    int segment_duration = ctx->config.segment_duration > 0 ?
                          ctx->config.segment_duration : 4;

    ctx->hls_writer = hls_writer_create(ctx->output_path, ctx->config.name, segment_duration);
    if (!ctx->hls_writer) {
        log_error("Failed to create HLS writer for %s", ctx->config.name);
        ctx->running = 0;
        return NULL;
    }

    // Always use a dedicated stream reader for HLS streaming
    // This ensures that HLS streaming doesn't interfere with detection or recording
    log_info("Starting new dedicated stream reader for HLS stream %s", ctx->config.name);
    reader_ctx = start_stream_reader(ctx->config.name, 1, NULL, NULL); // 1 for dedicated stream reader
    if (!reader_ctx) {
        log_error("Failed to start dedicated stream reader for %s", ctx->config.name);
        
        if (ctx->hls_writer) {
            hls_writer_close(ctx->hls_writer);
            ctx->hls_writer = NULL;
        }
        
        ctx->running = 0;
        return NULL;
    }
    log_info("Successfully started new dedicated stream reader for HLS stream %s", ctx->config.name);
    
    // Store the reader context first
    ctx->reader_ctx = reader_ctx;
    
    // Now set our callback - doing this separately ensures we don't have a race condition
    // where the callback might be called before ctx->reader_ctx is set
    if (set_packet_callback(reader_ctx, hls_packet_callback, ctx) != 0) {
        log_error("Failed to set packet callback for stream %s", ctx->config.name);
        
        if (ctx->hls_writer) {
            hls_writer_close(ctx->hls_writer);
            ctx->hls_writer = NULL;
        }
        
        // Don't stop the reader if we didn't create it
        if (reader_ctx->dedicated) {
            stop_stream_reader(reader_ctx);
        }
        
        ctx->running = 0;
        return NULL;
    }
    
    log_info("Set packet callback for HLS stream %s", ctx->config.name);

    // Main loop to monitor stream status
    while (ctx->running) {
        // Sleep to avoid busy waiting - reduced from 100ms to 50ms for more responsive handling
        av_usleep(50000);  // 50ms
    }
    
    // Remove our callback from the reader but don't stop it here
    // Let the cleanup function handle stopping the stream reader to avoid double-free
    if (ctx->reader_ctx) {
        set_packet_callback(ctx->reader_ctx, NULL, NULL);
        log_info("Cleared packet callback for HLS stream %s on thread exit", ctx->config.name);
    }

    // When done, close writer
    if (ctx->hls_writer) {
        hls_writer_close(ctx->hls_writer);
        ctx->hls_writer = NULL;
    }

    log_info("HLS streaming thread for stream %s exited", ctx->config.name);
    return NULL;
}

/**
 * Initialize HLS streaming backend
 */
void init_hls_streaming_backend(void) {
    // Initialize contexts array
    memset(streaming_contexts, 0, sizeof(streaming_contexts));

    log_info("HLS streaming backend initialized");
}

/**
 * Cleanup HLS streaming backend
 */
void cleanup_hls_streaming_backend(void) {
    log_info("Cleaning up HLS streaming backend...");
    pthread_mutex_lock(&contexts_mutex);

    // Stop all running streams
    for (int i = 0; i < MAX_STREAMS; i++) {
        if (streaming_contexts[i]) {
            log_info("Stopping HLS stream in slot %d: %s", i,
                    streaming_contexts[i]->config.name);

            // Copy the stream name and thread for later use
            char stream_name[MAX_STREAM_NAME];
            strncpy(stream_name, streaming_contexts[i]->config.name,
                    MAX_STREAM_NAME - 1);
            stream_name[MAX_STREAM_NAME - 1] = '\0';
            
            pthread_t thread_to_join = streaming_contexts[i]->thread;

            // Mark as not running
            streaming_contexts[i]->running = 0;
            
            // Remove callback from reader if it exists
            if (streaming_contexts[i]->reader_ctx) {
                set_packet_callback(streaming_contexts[i]->reader_ctx, NULL, NULL);
            }

            // Unlock before joining thread to prevent deadlocks
            pthread_mutex_unlock(&contexts_mutex);

            // Try to join with a timeout
            log_info("Waiting for HLS streaming thread for %s to exit", stream_name);
            int join_result = pthread_join_with_timeout(thread_to_join, NULL, 3);
            if (join_result != 0) {
                log_warn("Could not join thread for stream %s within timeout: %s",
                        stream_name, strerror(join_result));
            } else {
                log_info("Successfully joined HLS streaming thread for %s", stream_name);
            }

            // Re-lock for cleanup
            pthread_mutex_lock(&contexts_mutex);

            // Check if the context is still valid
            int found = 0;
            for (int j = 0; j < MAX_STREAMS; j++) {
                if (streaming_contexts[j] && strcmp(streaming_contexts[j]->config.name, stream_name) == 0) {
                    // Clean up resources
                    if (streaming_contexts[j]->hls_writer) {
                        hls_writer_close(streaming_contexts[j]->hls_writer);
                        streaming_contexts[j]->hls_writer = NULL;
                    }
                    
                    // Stop the dedicated stream reader if it exists
                    if (streaming_contexts[j]->reader_ctx) {
                        log_info("Stopping dedicated stream reader for HLS stream %s during cleanup", stream_name);
                        stop_stream_reader(streaming_contexts[j]->reader_ctx);
                        streaming_contexts[j]->reader_ctx = NULL;
                        log_info("Successfully stopped dedicated stream reader for HLS stream %s during cleanup", stream_name);
                    }
                    
                    // Free the context
                    free(streaming_contexts[j]);
                    streaming_contexts[j] = NULL;
                    found = 1;
                    break;
                }
            }
            
            if (!found) {
                log_warn("HLS streaming context for %s was already cleaned up", stream_name);
            }
        }
    }

    pthread_mutex_unlock(&contexts_mutex);

    log_info("HLS streaming backend cleaned up");
}

/**
 * Start HLS streaming for a stream
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
        if (streaming_contexts[i] && strcmp(streaming_contexts[i]->config.name, stream_name) == 0) {
            pthread_mutex_unlock(&contexts_mutex);
            log_info("HLS stream %s already running", stream_name);
            return 0;  // Already running
        }
    }

    // Find empty slot
    int slot = -1;
    for (int i = 0; i < MAX_STREAMS; i++) {
        if (!streaming_contexts[i]) {
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
    hls_stream_ctx_t *ctx = malloc(sizeof(hls_stream_ctx_t));
    if (!ctx) {
        pthread_mutex_unlock(&contexts_mutex);
        log_error("Memory allocation failed for HLS streaming context");
        return -1;
    }

    memset(ctx, 0, sizeof(hls_stream_ctx_t));
    memcpy(&ctx->config, &config, sizeof(stream_config_t));
    ctx->running = 1;
    ctx->frame_counter = 0; // Initialize frame counter

    // Create output paths
    config_t *global_config = get_streaming_config();

    // Log the storage path for debugging
    log_info("Using storage path for HLS: %s", global_config->storage_path);

    // Create HLS output path
    snprintf(ctx->output_path, MAX_PATH_LENGTH, "%s/hls/%s",
             global_config->storage_path, stream_name);

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

    // Also ensure the parent directory of the HLS directory exists and is writable
    char parent_dir[MAX_PATH_LENGTH];
    snprintf(parent_dir, sizeof(parent_dir), "%s/hls", global_config->storage_path);
    snprintf(dir_cmd, sizeof(dir_cmd), "mkdir -p %s && chmod -R 777 %s", 
             parent_dir, parent_dir);
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

    // Start streaming thread
    if (pthread_create(&ctx->thread, NULL, hls_stream_thread, ctx) != 0) {
        free(ctx);
        pthread_mutex_unlock(&contexts_mutex);
        log_error("Failed to create HLS streaming thread for %s", stream_name);
        return -1;
    }

    // Store context
    streaming_contexts[slot] = ctx;
    pthread_mutex_unlock(&contexts_mutex);

    log_info("Started HLS stream for %s in slot %d", stream_name, slot);

    return 0;
}

/**
 * Stop HLS streaming for a stream
 */
int stop_hls_stream(const char *stream_name) {
    int found = 0;

    // Log that we're attempting to stop the stream
    log_info("Attempting to stop HLS stream: %s", stream_name);

    pthread_mutex_lock(&contexts_mutex);

    // Find the stream context
    hls_stream_ctx_t *ctx = NULL;
    int index = -1;

    for (int i = 0; i < MAX_STREAMS; i++) {
        if (streaming_contexts[i] && strcmp(streaming_contexts[i]->config.name, stream_name) == 0) {
            ctx = streaming_contexts[i];
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
    int join_result = pthread_join_with_timeout(ctx->thread, NULL, 5);
    if (join_result != 0) {
        log_error("Failed to join thread for stream %s (error: %d), will continue cleanup",
                 stream_name, join_result);
    } else {
        log_info("Successfully joined thread for stream %s", stream_name);
    }

    // Re-lock for cleanup
    pthread_mutex_lock(&contexts_mutex);

    // Verify context is still valid
    if (index >= 0 && index < MAX_STREAMS && streaming_contexts[index] == ctx) {
        // Cleanup resources
        if (ctx->hls_writer) {
            log_info("Closing HLS writer for stream %s", stream_name);
            hls_writer_close(ctx->hls_writer);
            ctx->hls_writer = NULL;
        }
        
        // Stop the dedicated stream reader if it exists
        if (ctx->reader_ctx) {
            log_info("Stopping dedicated stream reader for HLS stream %s", stream_name);
            stop_stream_reader(ctx->reader_ctx);
            ctx->reader_ctx = NULL;
            log_info("Successfully stopped dedicated stream reader for HLS stream %s", stream_name);
        }

        // Free context and clear slot
        free(ctx);
        streaming_contexts[index] = NULL;

        log_info("Successfully cleaned up resources for stream %s", stream_name);
    } else {
        log_warn("Context for stream %s was modified during cleanup", stream_name);
    }

    pthread_mutex_unlock(&contexts_mutex);

    log_info("Stopped HLS stream %s", stream_name);
    return 0;
}

/**
 * Clean up HLS directories during shutdown
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
                if (streaming_contexts[i] && 
                    strcmp(streaming_contexts[i]->config.name, entry->d_name) == 0 &&
                    streaming_contexts[i]->running) {
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
