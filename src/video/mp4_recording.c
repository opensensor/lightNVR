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

#include <libavformat/avformat.h>
#include <libavcodec/avcodec.h>
#include <libavutil/avutil.h>
#include <libavutil/time.h>

#include "core/logger.h"
#include "core/config.h"
#include "video/stream_manager.h"
#include "video/streams.h"
#include "video/mp4_writer.h"
#include "video/mp4_recording.h"
#include "video/stream_transcoding.h"
#include "video/stream_reader.h"
#include "database/database_manager.h"
#include "database/db_events.h"

// Hash map for tracking running MP4 recording contexts
static mp4_recording_ctx_t *recording_contexts[MAX_STREAMS];
static pthread_mutex_t contexts_mutex = PTHREAD_MUTEX_INITIALIZER;

// Global array to store MP4 writers
static mp4_writer_t *mp4_writers[MAX_STREAMS] = {0};
static char mp4_writer_stream_names[MAX_STREAMS][64] = {{0}};
static pthread_mutex_t mp4_writers_mutex = PTHREAD_MUTEX_INITIALIZER;

// Forward declarations
static void *mp4_recording_thread(void *arg);

/**
 * MP4 recording thread function for a single stream
 */
static void *mp4_recording_thread(void *arg) {
    mp4_recording_ctx_t *ctx = (mp4_recording_ctx_t *)arg;
    AVPacket *pkt = NULL;
    int ret;
    time_t start_time = time(NULL);  // Record when we started
    config_t *global_config = get_streaming_config();
    stream_reader_ctx_t *reader_ctx = NULL;

    log_info("Starting MP4 recording thread for stream %s", ctx->config.name);

    // Verify output directory exists and is writable
    char mp4_dir[MAX_PATH_LENGTH];
    strncpy(mp4_dir, ctx->output_path, MAX_PATH_LENGTH - 1);
    mp4_dir[MAX_PATH_LENGTH - 1] = '\0';
    
    // Remove filename from path to get directory
    char *last_slash = strrchr(mp4_dir, '/');
    if (last_slash) {
        *last_slash = '\0';
    }
    
    struct stat st;
    if (stat(mp4_dir, &st) != 0 || !S_ISDIR(st.st_mode)) {
        log_error("Output directory does not exist or is not a directory: %s", mp4_dir);

        // Recreate it as a last resort
        char mkdir_cmd[MAX_PATH_LENGTH * 2];
        snprintf(mkdir_cmd, sizeof(mkdir_cmd), "mkdir -p %s", mp4_dir);

        int ret_mkdir = system(mkdir_cmd);
        if (ret_mkdir != 0 || stat(mp4_dir, &st) != 0 || !S_ISDIR(st.st_mode)) {
            log_error("Failed to create output directory: %s (return code: %d)", mp4_dir, ret_mkdir);
            ctx->running = 0;
            return NULL;
        }

        // Set permissions
        snprintf(mkdir_cmd, sizeof(mkdir_cmd), "chmod -R 777 %s", mp4_dir);
        int ret_chmod = system(mkdir_cmd);
        if (ret_chmod != 0) {
            log_warn("Failed to set permissions on directory: %s (return code: %d)", mp4_dir, ret_chmod);
        }
        
        log_info("Successfully created output directory: %s", mp4_dir);
    }

    // Check directory permissions
    if (access(mp4_dir, W_OK) != 0) {
        log_error("Output directory is not writable: %s", mp4_dir);

        // Try to fix permissions
        char chmod_cmd[MAX_PATH_LENGTH * 2];
        snprintf(chmod_cmd, sizeof(chmod_cmd), "chmod -R 777 %s", mp4_dir);
        int ret_chmod = system(chmod_cmd);
        if (ret_chmod != 0) {
            log_warn("Failed to set permissions on directory: %s (return code: %d)", mp4_dir, ret_chmod);
        }

        if (access(mp4_dir, W_OK) != 0) {
            log_error("Still unable to write to output directory: %s", mp4_dir);
            ctx->running = 0;
            return NULL;
        }
        
        log_info("Successfully fixed permissions for output directory: %s", mp4_dir);
    }

    // Create MP4 writer
    ctx->mp4_writer = mp4_writer_create(ctx->output_path, ctx->config.name);
    if (!ctx->mp4_writer) {
        log_error("Failed to create MP4 writer for %s", ctx->config.name);
        ctx->running = 0;
        return NULL;
    }
    
    log_info("Created MP4 writer for %s at %s", ctx->config.name, ctx->output_path);

    // Get or start a dedicated stream reader for MP4 recording
    reader_ctx = get_stream_reader(ctx->config.name);
    if (!reader_ctx) {
        // Start a new dedicated stream reader for MP4 recording
        reader_ctx = start_stream_reader(ctx->config.name, 1); // 1 for dedicated stream reader
        if (!reader_ctx) {
            log_error("Failed to start dedicated stream reader for %s", ctx->config.name);
            
            if (ctx->mp4_writer) {
                mp4_writer_close(ctx->mp4_writer);
                ctx->mp4_writer = NULL;
            }
            
            // Unregister the MP4 writer if it was registered
            unregister_mp4_writer_for_stream(ctx->config.name);
            
            ctx->running = 0;
            return NULL;
        }
        log_info("Started new dedicated stream reader for MP4 recording of stream %s", ctx->config.name);
    } else {
        log_info("Using existing stream reader for MP4 recording of stream %s", ctx->config.name);
    }
    
    // Register as a consumer of the stream
    ctx->consumer_id = register_stream_consumer(reader_ctx);
    if (ctx->consumer_id <= 0) {
        log_error("Failed to register as consumer for stream %s", ctx->config.name);
        
        if (ctx->mp4_writer) {
            mp4_writer_close(ctx->mp4_writer);
            ctx->mp4_writer = NULL;
        }
        
        // Unregister the MP4 writer if it was registered
        unregister_mp4_writer_for_stream(ctx->config.name);
        
        ctx->running = 0;
        return NULL;
    }
    
    // Initialize packet
    pkt = av_packet_alloc();
    if (!pkt) {
        log_error("Failed to allocate packet");
        
        // Unregister as a consumer
        unregister_stream_consumer(reader_ctx, ctx->consumer_id);
        
        if (ctx->mp4_writer) {
            mp4_writer_close(ctx->mp4_writer);
            ctx->mp4_writer = NULL;
        }
        
        // Unregister the MP4 writer if it was registered
        unregister_mp4_writer_for_stream(ctx->config.name);
        
        ctx->running = 0;
        return NULL;
    }

    // Register the MP4 writer so it can be accessed by other parts of the system
    register_mp4_writer_for_stream(ctx->config.name, ctx->mp4_writer);

    // Variables for periodic updates
    time_t last_update = 0;

    // Main packet reading loop
    while (ctx->running) {
        // Check if we need to rotate the MP4 file based on segment duration
        time_t current_time = time(NULL);
        int segment_duration = ctx->config.segment_duration > 0 ? ctx->config.segment_duration : 900; // Default to 15 minutes
        
        // If the MP4 file has been open for longer than the segment duration, rotate it
        if (ctx->mp4_writer && (current_time - ctx->mp4_writer->creation_time) >= segment_duration) {
            log_info("Rotating MP4 file for stream %s after %d seconds", ctx->config.name, segment_duration);
            
            // Generate new timestamp for the new MP4 file
            char timestamp_str[32];
            struct tm *tm_info = localtime(&current_time);
            strftime(timestamp_str, sizeof(timestamp_str), "%Y%m%d_%H%M%S", tm_info);
            
            // Create new MP4 output path with new timestamp
            char mp4_path[MAX_PATH_LENGTH];
            snprintf(mp4_path, MAX_PATH_LENGTH, "%s/mp4/%s/recording_%s.mp4",
                    global_config->storage_path, ctx->config.name, timestamp_str);
            
            // First unregister the current MP4 writer
            unregister_mp4_writer_for_stream(ctx->config.name);
            
            // Store the old writer temporarily
            mp4_writer_t *old_writer = ctx->mp4_writer;
            
            // Update the context's output path
            strncpy(ctx->output_path, mp4_path, MAX_PATH_LENGTH - 1);
            ctx->output_path[MAX_PATH_LENGTH - 1] = '\0';
            
            // Create new MP4 writer before closing the old one
            mp4_writer_t *new_writer = mp4_writer_create(ctx->output_path, ctx->config.name);
            if (!new_writer) {
                log_error("Failed to create new MP4 writer for stream %s during rotation", ctx->config.name);
                // Wait a bit before trying again
                av_usleep(1000000);  // 1 second delay
                continue; // Try again on next iteration
            }
            
            // First unregister the current MP4 writer
            unregister_mp4_writer_for_stream(ctx->config.name);
            
            // Register the new MP4 writer
            if (register_mp4_writer_for_stream(ctx->config.name, new_writer) != 0) {
                log_error("Failed to register new MP4 writer for stream %s during rotation", ctx->config.name);
                
                // Close the new writer since we couldn't register it
                mp4_writer_close(new_writer);
                
                // Re-register the old writer
                register_mp4_writer_for_stream(ctx->config.name, old_writer);
                
                // Wait a bit before trying again
                av_usleep(1000000);  // 1 second delay
                continue; // Try again on next iteration
            }
            
            // Now that the new writer is registered, update the context
            ctx->mp4_writer = new_writer;
            
            // Close the old writer now that everything else is set up
            mp4_writer_close(old_writer);
            old_writer = NULL; // Prevent any accidental use after free
            
            log_info("Successfully rotated MP4 writer for stream %s at %s", ctx->config.name, ctx->output_path);
            
            // Update recording metadata in the database
            update_recording(ctx->config.name);
        }
        
        // Get a packet from the stream reader
        av_packet_unref(pkt);  // Make sure the packet is clean before reusing
        ret = get_packet(reader_ctx, pkt, ctx->consumer_id);

        if (ret < 0) {
            // Error or abort request
            log_warn("Error getting packet from stream reader for %s", ctx->config.name);
            av_usleep(50000);  // Reduced delay to 50ms for faster recovery
            continue;
        }

        // Check if this is a timeout (no packet available)
        if (ret == 1) {
            // No packet available, just continue the loop
            continue;
        }
        
        // Check if the system is under pressure (ret == 2)
        if (ret == 2 && ctx->mp4_writer) {
            // System is under high pressure - set the flag in the writer
            ctx->mp4_writer->is_under_pressure = 1;
            
            // Log this condition occasionally to avoid log spam
            static time_t last_pressure_log = 0;
            time_t now = time(NULL);
            if (now - last_pressure_log >= 10) {  // Log every 10 seconds
                log_warn("MP4 recording under high pressure for %s", ctx->config.name);
                last_pressure_log = now;
            }
        } else if (ctx->mp4_writer) {
            // Normal operation - clear the pressure flag
            ctx->mp4_writer->is_under_pressure = 0;
        }

        // Process video packet
        if (ctx->mp4_writer) {
            // Verify the packet is valid before processing
            if (pkt->data && pkt->size > 0) {
                // Check if this is a key frame
                bool is_key_frame = (pkt->flags & AV_PKT_FLAG_KEY) != 0;
                
                // Only log key frames at debug level to reduce logging overhead
                if (is_key_frame) {
                    log_debug("Processing keyframe for MP4: pts=%lld, dts=%lld, size=%d",
                             (long long)pkt->pts, (long long)pkt->dts, pkt->size);
                }
                
                // Use a more efficient batch processing approach to reduce contention
                // Process packets in batches to reduce I/O operations
                static int packet_counter = 0;
                static int batch_size = 8; // Increased batch size for MP4 recording
                
                // Always process key frames immediately, but batch other frames
                if (is_key_frame || (++packet_counter >= batch_size)) {
                    packet_counter = 0; // Reset counter after processing a batch
                    
                    // Process the packet directly - we already have a clean copy from stream_reader
                    ret = process_video_packet(pkt, reader_ctx->input_ctx->streams[reader_ctx->video_stream_idx], 
                                              ctx->mp4_writer, 1, ctx->config.name);
                    
                    // Only log errors for key frames to reduce log spam
                    if (ret < 0 && is_key_frame) {
                        log_error("Failed to write keyframe to MP4 for stream %s: %d", ctx->config.name, ret);
                        // Continue anyway to keep the stream going
                    }
                }
            } else {
                log_warn("Received invalid packet (null data or zero size) for stream %s", ctx->config.name);
            }
        }

        // Periodically update recording metadata (every 30 seconds)
        time_t now = time(NULL);
        if (now - last_update >= 30) {
            update_recording(ctx->config.name);
            last_update = now;
        }
    }

    // Cleanup resources
    if (pkt) {
        av_packet_free(&pkt);
    }

    // Unregister as a consumer
    if (reader_ctx && ctx->consumer_id > 0) {
        unregister_stream_consumer(reader_ctx, ctx->consumer_id);
    }

    // When done, close writer
    if (ctx->mp4_writer) {
        mp4_writer_close(ctx->mp4_writer);
        ctx->mp4_writer = NULL;
        
        // Unregister the MP4 writer
        unregister_mp4_writer_for_stream(ctx->config.name);
    }

    log_info("MP4 recording thread for stream %s exited", ctx->config.name);
    return NULL;
}

/**
 * Initialize MP4 recording backend
 */
void init_mp4_recording_backend(void) {
    // Initialize contexts array
    memset(recording_contexts, 0, sizeof(recording_contexts));
    
    // Initialize MP4 writers array
    pthread_mutex_lock(&mp4_writers_mutex);
    memset(mp4_writers, 0, sizeof(mp4_writers));
    memset(mp4_writer_stream_names, 0, sizeof(mp4_writer_stream_names));
    pthread_mutex_unlock(&mp4_writers_mutex);

    log_info("MP4 recording backend initialized");
}

/**
 * Cleanup MP4 recording backend
 */
void cleanup_mp4_recording_backend(void) {
    log_info("Starting MP4 recording backend cleanup");

    // First mark all contexts as not running
    pthread_mutex_lock(&contexts_mutex);
    for (int i = 0; i < MAX_STREAMS; i++) {
        if (recording_contexts[i]) {
            recording_contexts[i]->running = 0;
        }
    }
    pthread_mutex_unlock(&contexts_mutex);

    // Give threads time to exit cleanly
    usleep(50000);  // 50ms

    // Now try to join threads
    pthread_mutex_lock(&contexts_mutex);
    for (int i = 0; i < MAX_STREAMS; i++) {
        // Join code
    }
    pthread_mutex_unlock(&contexts_mutex);

    // Finally close all writers
    close_all_mp4_writers();

    log_info("MP4 recording backend cleanup complete");
}

/**
 * Start MP4 recording for a stream
 */
int start_mp4_recording(const char *stream_name) {
    stream_handle_t stream = get_stream_by_name(stream_name);
    if (!stream) {
        log_error("Stream %s not found for MP4 recording", stream_name);
        return -1;
    }

    stream_config_t config;
    if (get_stream_config(stream, &config) != 0) {
        log_error("Failed to get config for stream %s for MP4 recording", stream_name);
        return -1;
    }

    // Check if already running
    pthread_mutex_lock(&contexts_mutex);
    for (int i = 0; i < MAX_STREAMS; i++) {
        if (recording_contexts[i] && strcmp(recording_contexts[i]->config.name, stream_name) == 0) {
            pthread_mutex_unlock(&contexts_mutex);
            log_info("MP4 recording for stream %s already running", stream_name);
            return 0;  // Already running
        }
    }

    // Find empty slot
    int slot = -1;
    for (int i = 0; i < MAX_STREAMS; i++) {
        if (!recording_contexts[i]) {
            slot = i;
            break;
        }
    }

    if (slot == -1) {
        pthread_mutex_unlock(&contexts_mutex);
        log_error("No slot available for new MP4 recording");
        return -1;
    }

    // Create context
    mp4_recording_ctx_t *ctx = malloc(sizeof(mp4_recording_ctx_t));
    if (!ctx) {
        pthread_mutex_unlock(&contexts_mutex);
        log_error("Memory allocation failed for MP4 recording context");
        return -1;
    }

    memset(ctx, 0, sizeof(mp4_recording_ctx_t));
    memcpy(&ctx->config, &config, sizeof(stream_config_t));
    ctx->running = 1;
    ctx->consumer_id = 0;  // Will be set in the thread

    // Create output paths
    config_t *global_config = get_streaming_config();

    // Create timestamp for MP4 filename
    char timestamp_str[32];
    time_t now = time(NULL);
    struct tm *tm_info = localtime(&now);
    strftime(timestamp_str, sizeof(timestamp_str), "%Y%m%d_%H%M%S", tm_info);

    // Create MP4 directory path
    char mp4_dir[MAX_PATH_LENGTH];
    if (global_config->record_mp4_directly && global_config->mp4_storage_path[0] != '\0') {
        // Use configured MP4 storage path if available
        snprintf(mp4_dir, MAX_PATH_LENGTH, "%s/%s",
                global_config->mp4_storage_path, stream_name);
    } else {
        // Use mp4 directory parallel to hls, NOT inside it
        snprintf(mp4_dir, MAX_PATH_LENGTH, "%s/mp4/%s",
                global_config->storage_path, stream_name);
    }

    // Create MP4 directory if it doesn't exist
    char dir_cmd[MAX_PATH_LENGTH * 2];
    snprintf(dir_cmd, sizeof(dir_cmd), "mkdir -p %s", mp4_dir);
    int ret = system(dir_cmd);
    if (ret != 0) {
        log_error("Failed to create MP4 directory: %s (return code: %d)", mp4_dir, ret);
        
            // Try to create the parent directory first
            char parent_dir[MAX_PATH_LENGTH];
            if (global_config->record_mp4_directly && global_config->mp4_storage_path[0] != '\0') {
                strncpy(parent_dir, global_config->mp4_storage_path, MAX_PATH_LENGTH - 1);
            } else {
                snprintf(parent_dir, MAX_PATH_LENGTH, "%s/mp4", global_config->storage_path);
            }
        
        snprintf(dir_cmd, sizeof(dir_cmd), "mkdir -p %s", parent_dir);
        ret = system(dir_cmd);
        if (ret != 0) {
            log_error("Failed to create parent MP4 directory: %s (return code: %d)", parent_dir, ret);
            free(ctx);
            pthread_mutex_unlock(&contexts_mutex);
            return -1;
        }
        
        // Try again to create the stream-specific directory
        snprintf(dir_cmd, sizeof(dir_cmd), "mkdir -p %s", mp4_dir);
        ret = system(dir_cmd);
        if (ret != 0) {
            log_error("Still failed to create MP4 directory: %s (return code: %d)", mp4_dir, ret);
            free(ctx);
            pthread_mutex_unlock(&contexts_mutex);
            return -1;
        }
    }

    // Set full permissions for MP4 directory
    snprintf(dir_cmd, sizeof(dir_cmd), "chmod -R 777 %s", mp4_dir);
    int ret_chmod = system(dir_cmd);
    if (ret_chmod != 0) {
        log_warn("Failed to set permissions on MP4 directory: %s (return code: %d)", mp4_dir, ret_chmod);
    }
    
    // Full path for the MP4 file
    snprintf(ctx->output_path, MAX_PATH_LENGTH, "%s/recording_%s.mp4",
             mp4_dir, timestamp_str);

    // Start recording thread
    if (pthread_create(&ctx->thread, NULL, mp4_recording_thread, ctx) != 0) {
        free(ctx);
        pthread_mutex_unlock(&contexts_mutex);
        log_error("Failed to create MP4 recording thread for %s", stream_name);
        return -1;
    }

    // Store context
    recording_contexts[slot] = ctx;
    pthread_mutex_unlock(&contexts_mutex);

    log_info("Started MP4 recording for %s in slot %d", stream_name, slot);

    return 0;
}

/**
 * Stop MP4 recording for a stream
 */
int stop_mp4_recording(const char *stream_name) {
    int found = 0;

    // Log that we're attempting to stop the recording
    log_info("Attempting to stop MP4 recording: %s", stream_name);

    pthread_mutex_lock(&contexts_mutex);

    // Find the recording context
    mp4_recording_ctx_t *ctx = NULL;
    int index = -1;

    for (int i = 0; i < MAX_STREAMS; i++) {
        if (recording_contexts[i] && strcmp(recording_contexts[i]->config.name, stream_name) == 0) {
            ctx = recording_contexts[i];
            index = i;
            found = 1;
            break;
        }
    }

    if (!found) {
        log_warn("MP4 recording for stream %s not found for stopping", stream_name);
        pthread_mutex_unlock(&contexts_mutex);
        return -1;
    }

    // Mark as not running first
    ctx->running = 0;
    log_info("Marked MP4 recording for stream %s as stopping (index: %d)", stream_name, index);

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
    if (index >= 0 && index < MAX_STREAMS && recording_contexts[index] == ctx) {
        // Cleanup resources
        if (ctx->mp4_writer) {
            log_info("Closing MP4 writer for stream %s", stream_name);
            mp4_writer_close(ctx->mp4_writer);
            ctx->mp4_writer = NULL;
            
            // Unregister the MP4 writer
            unregister_mp4_writer_for_stream(stream_name);
        }

        // Free context and clear slot
        free(ctx);
        recording_contexts[index] = NULL;

        log_info("Successfully cleaned up resources for stream %s", stream_name);
    } else {
        log_warn("Context for stream %s was modified during cleanup", stream_name);
    }

    pthread_mutex_unlock(&contexts_mutex);

    log_info("Stopped MP4 recording for stream %s", stream_name);
    return 0;
}

/**
 * Register an MP4 writer for a stream
 */
int register_mp4_writer_for_stream(const char *stream_name, mp4_writer_t *writer) {
    if (!stream_name || !writer) return -1;

    pthread_mutex_lock(&mp4_writers_mutex);

    // Find empty slot or existing entry for this stream
    int slot = -1;
    for (int i = 0; i < MAX_STREAMS; i++) {
        if (!mp4_writers[i]) {
            slot = i;
            break;
        } else if (strcmp(mp4_writer_stream_names[i], stream_name) == 0) {
            // Stream already has a writer, replace it
            log_info("Replacing existing MP4 writer for stream %s", stream_name);
            mp4_writer_close(mp4_writers[i]);
            mp4_writers[i] = writer;
            pthread_mutex_unlock(&mp4_writers_mutex);
            return 0;
        }
    }

    if (slot == -1) {
        log_error("No available slots for MP4 writer registration");
        pthread_mutex_unlock(&mp4_writers_mutex);
        return -1;
    }

    mp4_writers[slot] = writer;
    strncpy(mp4_writer_stream_names[slot], stream_name, 63);
    mp4_writer_stream_names[slot][63] = '\0';
    
    log_info("Registered MP4 writer for stream %s in slot %d", stream_name, slot);

    pthread_mutex_unlock(&mp4_writers_mutex);
    return 0;
}

/**
 * Get the MP4 writer for a stream
 */
mp4_writer_t *get_mp4_writer_for_stream(const char *stream_name) {
    if (!stream_name) return NULL;

    pthread_mutex_lock(&mp4_writers_mutex);

    mp4_writer_t *writer = NULL;
    for (int i = 0; i < MAX_STREAMS; i++) {
        if (mp4_writers[i] && strcmp(mp4_writer_stream_names[i], stream_name) == 0) {
            writer = mp4_writers[i];
            break;
        }
    }

    pthread_mutex_unlock(&mp4_writers_mutex);
    return writer;
}

/**
 * Unregister an MP4 writer for a stream
 */
void unregister_mp4_writer_for_stream(const char *stream_name) {
    if (!stream_name) return;

    pthread_mutex_lock(&mp4_writers_mutex);

    for (int i = 0; i < MAX_STREAMS; i++) {
        if (mp4_writers[i] && strcmp(mp4_writer_stream_names[i], stream_name) == 0) {
            // Don't close the writer here, just unregister it
            // The caller is responsible for closing the writer if needed
            mp4_writers[i] = NULL;
            mp4_writer_stream_names[i][0] = '\0';
            log_info("Unregistered MP4 writer for stream %s", stream_name);
            break;
        }
    }

    pthread_mutex_unlock(&mp4_writers_mutex);
}

/**
 * Close all MP4 writers during shutdown
 */
void close_all_mp4_writers(void) {
    log_info("Finalizing all MP4 recordings...");
    
    // Create a local array to store writers we need to close
    // This prevents double-free issues by ensuring we only close each writer once
    mp4_writer_t *writers_to_close[MAX_STREAMS] = {0};
    char stream_names_to_close[MAX_STREAMS][64] = {{0}};
    char file_paths_to_close[MAX_STREAMS][MAX_PATH_LENGTH] = {{0}};
    int num_writers_to_close = 0;
    
    // First, collect all writers under lock
    pthread_mutex_lock(&mp4_writers_mutex);
    
    for (int i = 0; i < MAX_STREAMS; i++) {
        if (mp4_writers[i] && mp4_writer_stream_names[i][0] != '\0') {
            // Store the writer pointer
            writers_to_close[num_writers_to_close] = mp4_writers[i];
            
            // Make a safe copy of the stream name
            strncpy(stream_names_to_close[num_writers_to_close], 
                    mp4_writer_stream_names[i], 
                    sizeof(stream_names_to_close[0]) - 1);
            
            // Safely get the file path from the MP4 writer
            if (mp4_writers[i]->output_path && mp4_writers[i]->output_path[0] != '\0') {
                strncpy(file_paths_to_close[num_writers_to_close], 
                        mp4_writers[i]->output_path, 
                        MAX_PATH_LENGTH - 1);
                
                // Log the path we're about to check
                log_info("Checking MP4 file: %s", file_paths_to_close[num_writers_to_close]);
                
                // Get file size before closing
                struct stat st;
                if (stat(file_paths_to_close[num_writers_to_close], &st) == 0) {
                    log_info("MP4 file size: %llu bytes", (unsigned long long)st.st_size);
                } else {
                    log_warn("Cannot stat MP4 file: %s (error: %s)", 
                            file_paths_to_close[num_writers_to_close], 
                            strerror(errno));
                }
            } else {
                log_warn("MP4 writer for stream %s has invalid or empty output path", 
                        stream_names_to_close[num_writers_to_close]);
            }
            
            // Clear the entry in the global array
            mp4_writers[i] = NULL;
            mp4_writer_stream_names[i][0] = '\0';
            
            // Increment counter
            num_writers_to_close++;
        }
    }
    
    // Release the lock before closing writers
    pthread_mutex_unlock(&mp4_writers_mutex);
    
    // Now close each writer (outside the lock to prevent deadlocks)
    for (int i = 0; i < num_writers_to_close; i++) {
        log_info("Finalizing MP4 recording for stream: %s", stream_names_to_close[i]);
        
        // Log before closing
        log_info("Closing MP4 writer for stream %s at %s", 
                stream_names_to_close[i], 
                file_paths_to_close[i][0] != '\0' ? file_paths_to_close[i] : "(empty path)");
        
        // Close the MP4 writer to finalize the file
        if (writers_to_close[i] != NULL) {
            mp4_writer_close(writers_to_close[i]);
            // No need to set to NULL as this is a local copy
        }
        
        // Update the database to mark the recording as complete
        if (file_paths_to_close[i][0] != '\0') {
            // Get the current time for the end timestamp
            time_t end_time = time(NULL);
            
            // Add an event to the database
            add_event(EVENT_RECORDING_STOP, stream_names_to_close[i], 
                     "Recording stopped during shutdown", file_paths_to_close[i]);
        }
    }
    
    log_info("All MP4 recordings finalized (%d writers closed)", num_writers_to_close);
}
