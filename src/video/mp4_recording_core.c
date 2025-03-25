#define _POSIX_C_SOURCE 200809L
#define _XOPEN_SOURCE 700
#define _GNU_SOURCE

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
#include <sys/time.h>
#include <signal.h>

#include <libavformat/avformat.h>
#include <libavcodec/avcodec.h>
#include <libavutil/avutil.h>
#include <libavutil/time.h>

#include "core/logger.h"
#include "core/config.h"
#include "core/shutdown_coordinator.h"
#include "video/stream_manager.h"
#include "video/streams.h"
#include "video/mp4_writer.h"
#include "video/mp4_recording.h"
#include "video/mp4_recording_internal.h"
#include "video/stream_transcoding.h"
#include "video/stream_reader.h"
#include "video/stream_state.h"
#include "video/stream_packet_processor.h"
#include "database/database_manager.h"
#include "database/db_events.h"

// Hash map for tracking running MP4 recording contexts
mp4_recording_ctx_t *recording_contexts[MAX_STREAMS];

// Flag to indicate if shutdown is in progress
volatile sig_atomic_t shutdown_in_progress = 0;

// Forward declarations
static void *mp4_recording_thread(void *arg);

/**
 * MP4 recording thread function for a single stream
 * 
 * CRITICAL FIX: Simplified to work with the new architecture where the HLS streaming thread
 * also writes to the MP4 file. This thread now just handles file rotation and metadata updates.
 */
static void *mp4_recording_thread(void *arg) {
    mp4_recording_ctx_t *ctx = (mp4_recording_ctx_t *)arg;
    int ret;
    time_t start_time = time(NULL);  // Record when we started
    config_t *global_config = get_streaming_config();

    // Make a local copy of the stream name for thread safety
    char stream_name[MAX_STREAM_NAME];
    strncpy(stream_name, ctx->config.name, MAX_STREAM_NAME - 1);
    stream_name[MAX_STREAM_NAME - 1] = '\0';

    log_info("Starting MP4 recording thread for stream %s", stream_name);

    // Check if we're still running (might have been stopped during initialization)
    if (!ctx->running || shutdown_in_progress) {
        log_info("MP4 recording thread for %s exiting early due to shutdown", stream_name);
        return NULL;
    }

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

    // Check again if we're still running
    if (!ctx->running || shutdown_in_progress) {
        log_info("MP4 recording thread for %s exiting after directory checks due to shutdown", stream_name);
        return NULL;
    }

    // Create MP4 writer
    ctx->mp4_writer = mp4_writer_create(ctx->output_path, stream_name);
    if (!ctx->mp4_writer) {
        log_error("Failed to create MP4 writer for %s", stream_name);
        ctx->running = 0;
        return NULL;
    }
    
    log_info("Created MP4 writer for %s at %s", stream_name, ctx->output_path);
    
    // CRITICAL FIX: Register the MP4 writer so the HLS streaming thread can access it
    // This is the key change - instead of using a dedicated stream reader, we rely on
    // the HLS streaming thread to write packets to the MP4 file
    register_mp4_writer_for_stream(stream_name, ctx->mp4_writer);
    log_info("Registered MP4 writer for stream %s - HLS thread will write packets to it", stream_name);
    
    // CRITICAL FIX: Add a timeout check to detect if the HLS thread is not writing packets
    time_t last_activity_check = time(NULL);
    time_t last_packet_time = 0;
    
    // Register with shutdown coordinator
    char component_name[128];
    snprintf(component_name, sizeof(component_name), "mp4_writer_%s", stream_name);
    int component_id = register_component(component_name, COMPONENT_MP4_WRITER, ctx, 80); // Medium priority (80)
    if (component_id >= 0) {
        log_info("Registered MP4 writer %s with shutdown coordinator (ID: %d)", stream_name, component_id);
    }

    // Reset the timestamp tracker to ensure we have a clean state for keyframe tracking
    reset_timestamp_tracker(stream_name);
    log_info("Reset timestamp tracker for stream %s to ensure clean keyframe tracking", stream_name);

    // Variables for periodic updates
    time_t last_update = 0;

    // Variables for tracking key frames and rotation
    int segment_duration = ctx->config.segment_duration > 0 ? ctx->config.segment_duration : 30;
    time_t rotation_ready_time = 0;
    int waiting_for_keyframe = 0;
    time_t last_rotation_time = time(NULL); // Track when we last rotated
    int force_rotation_counter = 0; // Counter to force rotation if we're stuck
    
    // Main loop to handle file rotation and metadata updates
    while (ctx->running && !shutdown_in_progress) {
        // Check if shutdown has been initiated
        if (is_shutdown_initiated()) {
            log_info("MP4 recording thread for %s stopping due to system shutdown", stream_name);
            ctx->running = 0;
            break;
        }
        // Get current time
        time_t current_time = time(NULL);
        
        // Check if we need to rotate the MP4 file based on segment duration
        if (ctx->mp4_writer) {
            // If we haven't set rotation_ready_time yet and we've reached the segment duration
            if (rotation_ready_time == 0 && 
                (current_time - ctx->mp4_writer->creation_time) >= segment_duration) {
                
                // Mark that we're ready to rotate, but waiting for the next key frame
                rotation_ready_time = current_time;
                waiting_for_keyframe = 1;
                log_info("MP4 file for stream %s has reached duration of %d seconds, waiting for next keyframe to rotate",
                        stream_name, segment_duration);
            }
            
            // If we're waiting for a key frame to rotate, check if one has been received
            if (waiting_for_keyframe) {
                // Get the timestamp of the last key frame from the timestamp manager
                time_t last_keyframe_time = 0;
                
                // BUGFIX: Store rotation_ready_time in a temporary variable to pass to last_keyframe_received
                // This ensures we're checking if a keyframe was received after rotation_ready_time
                time_t check_time = rotation_ready_time;
                int keyframe_received_after_rotation = last_keyframe_received(stream_name, &check_time);
                last_keyframe_time = check_time; // Get the actual last keyframe time
                
                // Log detailed information about keyframe status for debugging
                log_debug("Keyframe check for stream %s: received=%d, last_time=%ld, rotation_ready_time=%ld, diff=%ld seconds",
                        stream_name, keyframe_received_after_rotation, (long)last_keyframe_time, 
                        (long)rotation_ready_time, 
                        last_keyframe_time > 0 ? (long)(last_keyframe_time - rotation_ready_time) : 0);
                
                // If we've received a key frame after marking for rotation, perform the rotation
                if (keyframe_received_after_rotation && last_keyframe_time > rotation_ready_time) {
                    log_info("Rotating MP4 file for stream %s after %d seconds (waited for keyframe)",
                            stream_name, (int)(current_time - ctx->mp4_writer->creation_time));
                    
                    // Generate new timestamp for the new MP4 file
                    char timestamp_str[32];
                    struct tm *tm_info = localtime(&current_time);
                    strftime(timestamp_str, sizeof(timestamp_str), "%Y%m%d_%H%M%S", tm_info);
                    
                    // Create new MP4 output path with new timestamp
                    char mp4_path[MAX_PATH_LENGTH];
                    snprintf(mp4_path, MAX_PATH_LENGTH, "%s/mp4/%s/recording_%s.mp4",
                            global_config->storage_path, stream_name, timestamp_str);
                    
                    // First unregister the current MP4 writer
                    unregister_mp4_writer_for_stream(stream_name);
                    
                    // Store the old writer temporarily
                    mp4_writer_t *old_writer = ctx->mp4_writer;
                    
                    // Update the context's output path
                    strncpy(ctx->output_path, mp4_path, MAX_PATH_LENGTH - 1);
                    ctx->output_path[MAX_PATH_LENGTH - 1] = '\0';
                    
                    // Create new MP4 writer before closing the old one
                    mp4_writer_t *new_writer = mp4_writer_create(ctx->output_path, stream_name);
                    if (!new_writer) {
                        log_error("Failed to create new MP4 writer for stream %s during rotation", stream_name);
                        
                        // Re-register the old writer since we couldn't create a new one
                        register_mp4_writer_for_stream(stream_name, old_writer);
                        
                        // Reset rotation flags to try again later
                        rotation_ready_time = 0;
                        waiting_for_keyframe = 0;
                        
                        // Wait a bit before trying again
                        av_usleep(500000);  // 500ms delay
                        continue; // Try again on next iteration
                    }
                    
                    // Register the new MP4 writer
                    if (register_mp4_writer_for_stream(stream_name, new_writer) != 0) {
                        log_error("Failed to register new MP4 writer for stream %s during rotation", stream_name);
                        
                        // Close the new writer since we couldn't register it
                        mp4_writer_close(new_writer);
                        
                        // Re-register the old writer
                        register_mp4_writer_for_stream(stream_name, old_writer);
                        
                        // Reset rotation flags to try again later
                        rotation_ready_time = 0;
                        waiting_for_keyframe = 0;
                        
                        // Wait a bit before trying again
                        av_usleep(500000);  // 500ms delay
                        continue; // Try again on next iteration
                    }
                    
                    // Now that the new writer is registered, update the context
                    ctx->mp4_writer = new_writer;
                    
                    // Close the old writer now that everything else is set up
                    mp4_writer_close(old_writer);
                    old_writer = NULL; // Prevent any accidental use after free
                    
                    log_info("Successfully rotated MP4 writer for stream %s at %s", stream_name, ctx->output_path);
                    
                    // Update recording metadata in the database
                    update_recording(stream_name);
                    
                    // Reset rotation flags
                    rotation_ready_time = 0;
                    waiting_for_keyframe = 0;
                    
                    // Log that we're not resetting the timestamp tracker
                    log_info("Keeping timestamp tracker state for stream %s after forced rotation to maintain keyframe tracking", stream_name);
                    
                    // Don't reset the timestamp tracker after rotation to avoid losing keyframe tracking
                }
                // If we've been waiting too long for a keyframe, force rotation
                // For streams that don't send keyframes frequently, this prevents recordings from becoming too long
                else if ((current_time - rotation_ready_time) > 5) {  // 5 seconds timeout
                    log_warn("No keyframe received for over 5 seconds after reaching duration, forcing MP4 rotation for stream %s (total duration: %d seconds)", 
                            stream_name, (int)(current_time - ctx->mp4_writer->creation_time));
                    
                    // Force rotation similar to above, but with a warning
                    // (Same code as above, but with different log messages)
                    
                    // Generate new timestamp for the new MP4 file
                    char timestamp_str[32];
                    struct tm *tm_info = localtime(&current_time);
                    strftime(timestamp_str, sizeof(timestamp_str), "%Y%m%d_%H%M%S", tm_info);
                    
                    // Create new MP4 output path with new timestamp
                    char mp4_path[MAX_PATH_LENGTH];
                    snprintf(mp4_path, MAX_PATH_LENGTH, "%s/mp4/%s/recording_%s.mp4",
                            global_config->storage_path, stream_name, timestamp_str);
                    
                    // First unregister the current MP4 writer
                    unregister_mp4_writer_for_stream(stream_name);
                    
                    // Store the old writer temporarily
                    mp4_writer_t *old_writer = ctx->mp4_writer;
                    
                    // Update the context's output path
                    strncpy(ctx->output_path, mp4_path, MAX_PATH_LENGTH - 1);
                    ctx->output_path[MAX_PATH_LENGTH - 1] = '\0';
                    
                    // Create new MP4 writer before closing the old one
                    mp4_writer_t *new_writer = mp4_writer_create(ctx->output_path, stream_name);
                    if (!new_writer) {
                        log_error("Failed to create new MP4 writer for stream %s during forced rotation", stream_name);
                        
                        // Re-register the old writer since we couldn't create a new one
                        register_mp4_writer_for_stream(stream_name, old_writer);
                        
                        // Reset rotation flags to try again later
                        rotation_ready_time = 0;
                        waiting_for_keyframe = 0;
                        
                        // Wait a bit before trying again
                        av_usleep(500000);  // 500ms delay
                        continue; // Try again on next iteration
                    }
                    
                    // Register the new MP4 writer
                    if (register_mp4_writer_for_stream(stream_name, new_writer) != 0) {
                        log_error("Failed to register new MP4 writer for stream %s during forced rotation", stream_name);
                        
                        // Close the new writer since we couldn't register it
                        mp4_writer_close(new_writer);
                        
                        // Re-register the old writer
                        register_mp4_writer_for_stream(stream_name, old_writer);
                        
                        // Reset rotation flags to try again later
                        rotation_ready_time = 0;
                        waiting_for_keyframe = 0;
                        
                        // Wait a bit before trying again
                        av_usleep(500000);  // 500ms delay
                        continue; // Try again on next iteration
                    }
                    
                    // Now that the new writer is registered, update the context
                    ctx->mp4_writer = new_writer;
                    
                    // Close the old writer now that everything else is set up
                    mp4_writer_close(old_writer);
                    old_writer = NULL; // Prevent any accidental use after free
                    
                    log_info("Successfully forced rotation of MP4 writer for stream %s at %s", 
                            stream_name, ctx->output_path);
                    
                    // Update recording metadata in the database
                    update_recording(stream_name);
                    
                    // Reset rotation flags
                    rotation_ready_time = 0;
                    waiting_for_keyframe = 0;
                }
            }
            
            // Absolute maximum duration check - force rotation if we've gone over the segment duration
            // This is a failsafe in case the keyframe detection or rotation logic fails
            if (ctx->mp4_writer && 
                (current_time - ctx->mp4_writer->creation_time) >= (segment_duration + 2)) {
                
                // Increment force rotation counter
                force_rotation_counter++;
                
                // Log more frequently to better track the issue
                if (force_rotation_counter == 1 || force_rotation_counter % 3 == 0) {
                    log_warn("MP4 file for stream %s has exceeded target duration (%d seconds), preparing for forced rotation",
                            stream_name, (int)(current_time - ctx->mp4_writer->creation_time));
                }
                
                // BUGFIX: Reduce the wait time before forcing rotation to prevent segments from becoming too large
                // If we've been waiting too long, force an immediate rotation regardless of keyframes
                if (force_rotation_counter >= 2) {
                    log_warn("MP4 file for stream %s has exceeded maximum allowed duration (%d seconds), forcing immediate rotation",
                            stream_name, (int)(current_time - ctx->mp4_writer->creation_time));
                
                    // Force rotation immediately
                    rotation_ready_time = current_time;
                    waiting_for_keyframe = 1;
                    
                    // Generate new timestamp for the new MP4 file
                    char timestamp_str[32];
                    struct tm *tm_info = localtime(&current_time);
                    strftime(timestamp_str, sizeof(timestamp_str), "%Y%m%d_%H%M%S", tm_info);
                    
                    // Create new MP4 output path with new timestamp
                    char mp4_path[MAX_PATH_LENGTH];
                    snprintf(mp4_path, MAX_PATH_LENGTH, "%s/mp4/%s/recording_%s.mp4",
                            global_config->storage_path, stream_name, timestamp_str);
                    
                    // First unregister the current MP4 writer
                    unregister_mp4_writer_for_stream(stream_name);
                    
                    // Store the old writer temporarily
                    mp4_writer_t *old_writer = ctx->mp4_writer;
                    
                    // Update the context's output path
                    strncpy(ctx->output_path, mp4_path, MAX_PATH_LENGTH - 1);
                    ctx->output_path[MAX_PATH_LENGTH - 1] = '\0';
                    
                    // Create new MP4 writer before closing the old one
                    mp4_writer_t *new_writer = mp4_writer_create(ctx->output_path, stream_name);
                    if (!new_writer) {
                        log_error("Failed to create new MP4 writer for stream %s during emergency rotation", stream_name);
                        
                        // Re-register the old writer since we couldn't create a new one
                        register_mp4_writer_for_stream(stream_name, old_writer);
                        
                        // Reset rotation flags to try again later
                        rotation_ready_time = 0;
                        waiting_for_keyframe = 0;
                        
                        // Wait a bit before trying again
                        av_usleep(500000);  // 500ms delay
                        continue; // Try again on next iteration
                    }
                    
                    // Register the new MP4 writer
                    if (register_mp4_writer_for_stream(stream_name, new_writer) != 0) {
                        log_error("Failed to register new MP4 writer for stream %s during emergency rotation", stream_name);
                        
                        // Close the new writer since we couldn't register it
                        mp4_writer_close(new_writer);
                        
                        // Re-register the old writer
                        register_mp4_writer_for_stream(stream_name, old_writer);
                        
                        // Reset rotation flags to try again later
                        rotation_ready_time = 0;
                        waiting_for_keyframe = 0;
                        
                        // Wait a bit before trying again
                        av_usleep(500000);  // 500ms delay
                        continue; // Try again on next iteration
                    }
                    
                    // Now that the new writer is registered, update the context
                    ctx->mp4_writer = new_writer;
                    
                    // Close the old writer now that everything else is set up
                    mp4_writer_close(old_writer);
                    old_writer = NULL; // Prevent any accidental use after free
                    
                    log_info("Successfully performed emergency rotation of MP4 writer for stream %s at %s", 
                            stream_name, ctx->output_path);
                    
                    // Update recording metadata in the database
                    update_recording(stream_name);
                    
                    // Reset rotation flags
                    rotation_ready_time = 0;
                    waiting_for_keyframe = 0;
                    force_rotation_counter = 0;
                    
                    // Update last rotation time
                    last_rotation_time = current_time;
                    
                    // Log the successful rotation with duration information
                    log_info("Successfully rotated MP4 file for stream %s. Duration: %d seconds",
                            stream_name, (int)(current_time - ctx->mp4_writer->creation_time));
                }
            }
            
            // BUGFIX: Add an absolute maximum duration check to prevent any segment from becoming too large
            // This is a hard limit that will force rotation regardless of other conditions
            // Use a lower threshold for streams with higher bitrates (like "face")
            int max_duration_factor = 2;
            if (strcmp(stream_name, "face") == 0) {
                // Use a lower threshold for the face stream which has higher bitrate
                max_duration_factor = 1;
                
                // Check if we're approaching the limit
                if (ctx->mp4_writer && 
                    (current_time - ctx->mp4_writer->creation_time) >= (segment_duration + 5)) {
                    log_warn("High bitrate stream %s approaching segment limit at %d seconds, preparing for rotation",
                            stream_name, (int)(current_time - ctx->mp4_writer->creation_time));
                }
            }
            
            if (ctx->mp4_writer && 
                (current_time - ctx->mp4_writer->creation_time) >= (segment_duration * max_duration_factor)) {
                
                log_error("MP4 file for stream %s has reached maximum allowed duration (%d seconds, limit factor: %d), forcing immediate rotation",
                        stream_name, (int)(current_time - ctx->mp4_writer->creation_time), max_duration_factor);
                
                // Force rotation immediately
                char timestamp_str[32];
                struct tm *tm_info = localtime(&current_time);
                strftime(timestamp_str, sizeof(timestamp_str), "%Y%m%d_%H%M%S", tm_info);
                
                // Create new MP4 output path with new timestamp
                char mp4_path[MAX_PATH_LENGTH];
                snprintf(mp4_path, MAX_PATH_LENGTH, "%s/mp4/%s/recording_%s.mp4",
                        global_config->storage_path, stream_name, timestamp_str);
                
                // First unregister the current MP4 writer
                unregister_mp4_writer_for_stream(stream_name);
                
                // Store the old writer temporarily
                mp4_writer_t *old_writer = ctx->mp4_writer;
                
                // Update the context's output path
                strncpy(ctx->output_path, mp4_path, MAX_PATH_LENGTH - 1);
                ctx->output_path[MAX_PATH_LENGTH - 1] = '\0';
                
                // Create new MP4 writer before closing the old one
                mp4_writer_t *new_writer = mp4_writer_create(ctx->output_path, stream_name);
                if (!new_writer) {
                    log_error("Failed to create new MP4 writer for stream %s during hard limit rotation", stream_name);
                    
                    // Re-register the old writer since we couldn't create a new one
                    register_mp4_writer_for_stream(stream_name, old_writer);
                } else {
                    // Register the new MP4 writer
                    if (register_mp4_writer_for_stream(stream_name, new_writer) != 0) {
                        log_error("Failed to register new MP4 writer for stream %s during hard limit rotation", stream_name);
                        
                        // Close the new writer since we couldn't register it
                        mp4_writer_close(new_writer);
                        
                        // Re-register the old writer
                        register_mp4_writer_for_stream(stream_name, old_writer);
                    } else {
                        // Now that the new writer is registered, update the context
                        ctx->mp4_writer = new_writer;
                        
                        // Close the old writer now that everything else is set up
                        mp4_writer_close(old_writer);
                        old_writer = NULL; // Prevent any accidental use after free
                        
                        log_info("Successfully performed hard limit rotation of MP4 writer for stream %s at %s", 
                                stream_name, ctx->output_path);
                        
                        // Update recording metadata in the database
                        update_recording(stream_name);
                        
                        // Reset rotation flags
                        rotation_ready_time = 0;
                        waiting_for_keyframe = 0;
                        force_rotation_counter = 0;
                        
                        // Update last rotation time
                        last_rotation_time = current_time;
                    }
                }
            }
        }

        // CRITICAL FIX: Check if the HLS thread is writing packets to the MP4 file
        // If no packets have been written for a while, log a warning and potentially restart
        time_t now = time(NULL);
        if (now - last_activity_check >= 10) { // Check every 10 seconds
            last_activity_check = now;
            
            // Get the last packet time from the MP4 writer
            if (ctx->mp4_writer) {
                time_t current_packet_time = ctx->mp4_writer->last_packet_time;
                
                // If this is the first check, initialize last_packet_time
                if (last_packet_time == 0) {
                    last_packet_time = current_packet_time;
                } else if (current_packet_time > 0 && current_packet_time != last_packet_time) {
                    // Packets are being written, update the last packet time
                    last_packet_time = current_packet_time;
                    log_debug("MP4 writer for stream %s is receiving packets", stream_name);
                } else if (now - start_time > 30) { // Give it 30 seconds to start up
                    // No new packets have been written for 10 seconds
                    log_warn("No packets written to MP4 file for stream %s in the last 10 seconds", stream_name);
                    
                    // If no packets have been written for 30 seconds, try to restart the HLS stream
                    if (now - last_packet_time > 30 || last_packet_time == 0) {
                        log_error("No packets written to MP4 file for stream %s in 30 seconds, restarting HLS stream", stream_name);
                        
                        // Restart the HLS stream
                        extern int restart_hls_stream(const char *stream_name);
                        int restart_result = restart_hls_stream(stream_name);
                        if (restart_result != 0) {
                            log_error("Failed to restart HLS stream for %s", stream_name);
                        } else {
                            log_info("Successfully restarted HLS stream for %s", stream_name);
                            // Reset the last packet time to give it time to start writing packets
                            last_packet_time = now;
                        }
                    }
                }
            }
        }
        
        // Periodically update recording metadata (every 30 seconds)
        if (now - last_update >= 30) {
            update_recording(stream_name);
            last_update = now;
        }
        
        // Sleep to avoid busy waiting
        av_usleep(50000);  // 50ms
    }

    // When done, close writer
    if (ctx->mp4_writer) {
        log_info("Closing MP4 writer for stream %s during thread exit", stream_name);
        mp4_writer_close(ctx->mp4_writer);
        ctx->mp4_writer = NULL;
        
        // Unregister the MP4 writer
        unregister_mp4_writer_for_stream(stream_name);
    }

    // Update component state in shutdown coordinator
    if (component_id >= 0) {
        update_component_state(component_id, COMPONENT_STOPPED);
        log_info("Updated MP4 writer %s state to STOPPED in shutdown coordinator", stream_name);
    }

    log_info("MP4 recording thread for stream %s exited", stream_name);
    return NULL;
}

/**
 * Initialize MP4 recording backend
 */
void init_mp4_recording_backend(void) {
    // Initialize contexts array
    memset(recording_contexts, 0, sizeof(recording_contexts));
    
    // Initialize MP4 writers array
    memset(mp4_writers, 0, sizeof(mp4_writers));
    memset(mp4_writer_stream_names, 0, sizeof(mp4_writer_stream_names));

    // Reset shutdown flag
    shutdown_in_progress = 0;

    log_info("MP4 recording backend initialized");
}

/**
 * Cleanup MP4 recording backend
 */
void cleanup_mp4_recording_backend(void) {
    log_info("Starting MP4 recording backend cleanup");

    // Set shutdown flag to signal all threads to exit
    shutdown_in_progress = 1;

    // Create a local array to store contexts we need to clean up
    // This prevents race conditions by ensuring we handle each context safely
    typedef struct {
        mp4_recording_ctx_t *ctx;
        pthread_t thread;
        char stream_name[MAX_STREAM_NAME];
        int index;
    } cleanup_item_t;
    
    cleanup_item_t items_to_cleanup[MAX_STREAMS];
    int cleanup_count = 0;
    
    // First collect all contexts under lock
    for (int i = 0; i < MAX_STREAMS; i++) {
        if (recording_contexts[i]) {
            // Mark as not running
            recording_contexts[i]->running = 0;
            
            // Safely NULL out the mp4_writer pointer to prevent double free
            // This is critical since close_all_mp4_writers() was already called
            recording_contexts[i]->mp4_writer = NULL;
            
            // Store context info for cleanup
            items_to_cleanup[cleanup_count].ctx = recording_contexts[i];
            items_to_cleanup[cleanup_count].thread = recording_contexts[i]->thread;
            strncpy(items_to_cleanup[cleanup_count].stream_name, 
                    recording_contexts[i]->config.name, 
                    MAX_STREAM_NAME - 1);
            items_to_cleanup[cleanup_count].stream_name[MAX_STREAM_NAME - 1] = '\0';
            items_to_cleanup[cleanup_count].index = i;
            
            cleanup_count++;
        }
    }

    // Now join threads and free contexts outside the lock
    for (int i = 0; i < cleanup_count; i++) {
        // Join thread with timeout
        log_info("Waiting for MP4 recording thread for %s to exit", 
                items_to_cleanup[i].stream_name);
        
        int join_result = pthread_join_with_timeout(items_to_cleanup[i].thread, NULL, 3);
        if (join_result != 0) {
            log_warn("Could not join MP4 recording thread for %s within timeout: %s", 
                    items_to_cleanup[i].stream_name, strerror(join_result));
        } else {
            log_info("Successfully joined MP4 recording thread for %s", 
                    items_to_cleanup[i].stream_name);
        }

        // Double-check that the context is still at the expected index
        if (recording_contexts[items_to_cleanup[i].index] == items_to_cleanup[i].ctx) {
            // Free context - no need to close mp4_writer as it was already NULLed out
            free(recording_contexts[items_to_cleanup[i].index]);
            recording_contexts[items_to_cleanup[i].index] = NULL;
            log_info("Freed MP4 recording context for %s", items_to_cleanup[i].stream_name);
        } else {
            log_warn("MP4 recording context for %s was already cleaned up or moved", 
                    items_to_cleanup[i].stream_name);
        }
    }

    // Note: We don't call close_all_mp4_writers() here anymore
    // It's already called earlier in the main cleanup process
    // Calling it twice could lead to double free issues

    log_info("MP4 recording backend cleanup complete");
}

/**
 * Start MP4 recording for a stream
 */
int start_mp4_recording(const char *stream_name) {
    // Check if shutdown is in progress
    if (shutdown_in_progress) {
        log_warn("Cannot start MP4 recording for %s during shutdown", stream_name);
        return -1;
    }

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
    for (int i = 0; i < MAX_STREAMS; i++) {
        if (recording_contexts[i] && strcmp(recording_contexts[i]->config.name, stream_name) == 0) {
            log_info("MP4 recording for stream %s already running", stream_name);
            return 0;  // Already running
        }
    }
    
    // CRITICAL FIX: Ensure HLS streaming is started for this stream
    // This is necessary because the MP4 recording relies on the HLS streaming thread
    // to write packets to the MP4 file
    extern int start_hls_stream(const char *stream_name);
    int hls_result = start_hls_stream(stream_name);
    if (hls_result != 0) {
        log_warn("Failed to start HLS streaming for MP4 recording of stream %s", stream_name);
        // Continue anyway, as the HLS streaming might already be running
    } else {
        log_info("Started HLS streaming for MP4 recording of stream %s", stream_name);
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
        log_error("No slot available for new MP4 recording");
        return -1;
    }

    // Create context
    mp4_recording_ctx_t *ctx = malloc(sizeof(mp4_recording_ctx_t));
    if (!ctx) {
        log_error("Memory allocation failed for MP4 recording context");
        return -1;
    }

    memset(ctx, 0, sizeof(mp4_recording_ctx_t));
    memcpy(&ctx->config, &config, sizeof(stream_config_t));
    ctx->running = 1;

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
            return -1;
        }
        
        // Try again to create the stream-specific directory
        snprintf(dir_cmd, sizeof(dir_cmd), "mkdir -p %s", mp4_dir);
        ret = system(dir_cmd);
        if (ret != 0) {
            log_error("Still failed to create MP4 directory: %s (return code: %d)", mp4_dir, ret);
            free(ctx);
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
        log_error("Failed to create MP4 recording thread for %s", stream_name);
        return -1;
    }

    // Store context
    recording_contexts[slot] = ctx;

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
        return -1;
    }

    // Mark as not running first
    ctx->running = 0;
    log_info("Marked MP4 recording for stream %s as stopping (index: %d)", stream_name, index);

    // Join thread with timeout
    int join_result = pthread_join_with_timeout(ctx->thread, NULL, 5);
    if (join_result != 0) {
        log_error("Failed to join thread for stream %s (error: %d), will continue cleanup",
                 stream_name, join_result);
    } else {
        log_info("Successfully joined thread for stream %s", stream_name);
    }

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

    log_info("Stopped MP4 recording for stream %s", stream_name);
    return 0;
}
