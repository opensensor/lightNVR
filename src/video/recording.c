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

#include "core/logger.h"
#include "core/config.h"
#include "video/stream_manager.h"
#include "video/streams.h"
#include "video/mp4_writer.h"
#include "database/database_manager.h"

// Global array to store MP4 writers
static mp4_writer_t *mp4_writers[MAX_STREAMS] = {0};
static char mp4_writer_stream_names[MAX_STREAMS][64] = {{0}};
static pthread_mutex_t mp4_writers_mutex = PTHREAD_MUTEX_INITIALIZER;

// Array to store active recordings (one for each stream)
// These are made non-static so they can be accessed from mp4_writer.c
active_recording_t active_recordings[MAX_STREAMS];
pthread_mutex_t recordings_mutex = PTHREAD_MUTEX_INITIALIZER;

/**
 * Initialize the active recordings array
 */
void init_recordings(void) {
    pthread_mutex_lock(&recordings_mutex);
    memset(active_recordings, 0, sizeof(active_recordings));
    pthread_mutex_unlock(&recordings_mutex);
}

/**
 * Function to initialize the recording system
 */
void init_recordings_system(void) {
    init_recordings();
    log_info("Recordings system initialized");
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
            mp4_writer_close(mp4_writers[i]);
            mp4_writers[i] = NULL;
            mp4_writer_stream_names[i][0] = '\0';
            break;
        }
    }

    pthread_mutex_unlock(&mp4_writers_mutex);
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
 * New function to start MP4 recording for a stream
 * This is completely separate from HLS streaming
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
            return -1;
        }
        
        // Try again to create the stream-specific directory
        snprintf(dir_cmd, sizeof(dir_cmd), "mkdir -p %s", mp4_dir);
        ret = system(dir_cmd);
        if (ret != 0) {
            log_error("Still failed to create MP4 directory: %s (return code: %d)", mp4_dir, ret);
            return -1;
        }
    }

    // Set full permissions for MP4 directory
    snprintf(dir_cmd, sizeof(dir_cmd), "chmod -R 777 %s", mp4_dir);
    int ret_chmod = system(dir_cmd);
    if (ret_chmod != 0) {
        log_warn("Failed to set permissions on MP4 directory: %s (return code: %d)", mp4_dir, ret_chmod);
    }
    
    // Verify the directory is writable
    if (access(mp4_dir, W_OK) != 0) {
        log_error("MP4 directory is not writable: %s (error: %s)", mp4_dir, strerror(errno));
        return -1;
    }
    
    log_info("Verified MP4 directory is writable: %s", mp4_dir);

    // Full path for the MP4 file
    char mp4_path[MAX_PATH_LENGTH];
    snprintf(mp4_path, MAX_PATH_LENGTH, "%s/recording_%s.mp4",
             mp4_dir, timestamp_str);

    // Create MP4 writer directly
    mp4_writer_t *writer = mp4_writer_create(mp4_path, stream_name);
    if (!writer) {
        log_error("Failed to create MP4 writer for stream %s at %s", stream_name, mp4_path);
        return -1;
    }

    // Store the writer reference somewhere it can be accessed by the stream processing code
    // This would depend on your application's architecture
    // For now, we'll assume there's a function to register the MP4 writer with the stream
    if (register_mp4_writer_for_stream(stream_name, writer) != 0) {
        log_error("Failed to register MP4 writer for stream %s", stream_name);
        mp4_writer_close(writer);
        return -1;
    }

    log_info("Started MP4 recording for stream %s at %s", stream_name, mp4_path);
    return 0;
}

/**
 * Find MP4 recording for a stream based on timestamp
 * Returns 1 if found, 0 if not found, -1 on error
 */
int find_mp4_recording(const char *stream_name, time_t timestamp, char *mp4_path, size_t path_size) {
    if (!stream_name || !mp4_path || path_size == 0) {
        log_error("Invalid parameters for find_mp4_recording");
        return -1;
    }

    // Get global config for storage paths
    config_t *global_config = get_streaming_config();
    char base_path[256];

    // Try different possible locations for the MP4 file

    // 1. Try main recordings directory with stream subdirectory
    snprintf(base_path, sizeof(base_path), "%s/recordings/%s",
            global_config->storage_path, stream_name);

    // Format timestamp for pattern matching
    char timestamp_str[32];
    struct tm *tm_info = localtime(&timestamp);
    strftime(timestamp_str, sizeof(timestamp_str), "%Y%m%d_%H%M", tm_info);

    // Log what we're looking for
    log_info("Looking for MP4 recording for stream '%s' with timestamp around %s in %s",
            stream_name, timestamp_str, base_path);

    // Use system command to find matching files
    char find_cmd[512];
    snprintf(find_cmd, sizeof(find_cmd),
            "find %s -type f -name \"recording_%s*.mp4\" | sort",
            base_path, timestamp_str);

    FILE *cmd_pipe = popen(find_cmd, "r");
    if (!cmd_pipe) {
        log_error("Failed to execute find command: %s", find_cmd);

        // Try fallback with ls and grep
        snprintf(find_cmd, sizeof(find_cmd),
                "ls -1 %s/recording_%s*.mp4 2>/dev/null | head -1",
                base_path, timestamp_str);

        cmd_pipe = popen(find_cmd, "r");
        if (!cmd_pipe) {
            log_error("Failed to execute fallback find command");
            return -1;
        }
    }

    char found_path[256] = {0};
    if (fgets(found_path, sizeof(found_path), cmd_pipe)) {
        // Remove trailing newline
        size_t len = strlen(found_path);
        if (len > 0 && found_path[len-1] == '\n') {
            found_path[len-1] = '\0';
        }

        // Check if file exists and has content
        struct stat st;
        if (stat(found_path, &st) == 0 && st.st_size > 0) {
            log_info("Found MP4 file: %s (%lld bytes)",
                    found_path, (long long)st.st_size);

            strncpy(mp4_path, found_path, path_size - 1);
            mp4_path[path_size - 1] = '\0';
            pclose(cmd_pipe);
            return 1;
        }
    }

    pclose(cmd_pipe);

    // 2. Try alternative location if MP4 direct storage is configured
    if (global_config->record_mp4_directly && global_config->mp4_storage_path[0] != '\0') {
        snprintf(base_path, sizeof(base_path), "%s/%s",
                global_config->mp4_storage_path, stream_name);

        log_info("Looking in alternative MP4 location: %s", base_path);

        // Same approach with alternative path
        snprintf(find_cmd, sizeof(find_cmd),
                "find %s -type f -name \"recording_%s*.mp4\" | sort",
                base_path, timestamp_str);

        cmd_pipe = popen(find_cmd, "r");
        if (cmd_pipe) {
            if (fgets(found_path, sizeof(found_path), cmd_pipe)) {
                // Remove trailing newline
                size_t len = strlen(found_path);
                if (len > 0 && found_path[len-1] == '\n') {
                    found_path[len-1] = '\0';
                }

                // Check if file exists and has content
                struct stat st;
                if (stat(found_path, &st) == 0 && st.st_size > 0) {
                    log_info("Found MP4 file in alternative location: %s (%lld bytes)",
                            found_path, (long long)st.st_size);

                    strncpy(mp4_path, found_path, path_size - 1);
                    mp4_path[path_size - 1] = '\0';
                    pclose(cmd_pipe);
                    return 1;
                }
            }
            pclose(cmd_pipe);
        }
    }

    // 3. Try less restrictive search in case the timestamp format is different
    // This will look for any MP4 with the stream name in various directories

    // Try in the HLS directory itself (sometimes MP4s are stored alongside HLS files)
    snprintf(base_path, sizeof(base_path), "%s/hls/%s",
            global_config->storage_path, stream_name);

    log_info("Looking in HLS directory: %s", base_path);

    snprintf(find_cmd, sizeof(find_cmd),
            "find %s -type f -name \"*.mp4\" | sort", base_path);

    cmd_pipe = popen(find_cmd, "r");
    if (cmd_pipe) {
        if (fgets(found_path, sizeof(found_path), cmd_pipe)) {
            // Remove trailing newline
            size_t len = strlen(found_path);
            if (len > 0 && found_path[len-1] == '\n') {
                found_path[len-1] = '\0';
            }

            // Check if file exists and has content
            struct stat st;
            if (stat(found_path, &st) == 0 && st.st_size > 0) {
                log_info("Found MP4 file in HLS directory: %s (%lld bytes)",
                        found_path, (long long)st.st_size);

                strncpy(mp4_path, found_path, path_size - 1);
                mp4_path[path_size - 1] = '\0';
                pclose(cmd_pipe);
                return 1;
            }
        }
        pclose(cmd_pipe);
    }

    // No MP4 file found
    log_warn("No matching MP4 recording found for stream '%s' with timestamp around %s",
            stream_name, timestamp_str);
    return 0;
}

/**
 * Close all MP4 writers during shutdown
 * This ensures all MP4 files are properly finalized and marked as complete in the database
 */
void close_all_mp4_writers(void) {
    log_info("Finalizing all MP4 recordings...");
    
    pthread_mutex_lock(&mp4_writers_mutex);
    
    for (int i = 0; i < MAX_STREAMS; i++) {
        if (mp4_writers[i]) {
            log_info("Finalizing MP4 recording for stream: %s", mp4_writer_stream_names[i]);
            
            // Get the file path from the MP4 writer
            char file_path[MAX_PATH_LENGTH];
            if (mp4_writers[i]->output_path) {
                strncpy(file_path, mp4_writers[i]->output_path, MAX_PATH_LENGTH - 1);
                file_path[MAX_PATH_LENGTH - 1] = '\0';
            } else {
                file_path[0] = '\0';
            }
            
            // Get file size before closing
            struct stat st;
            uint64_t file_size = 0;
            if (file_path[0] != '\0' && stat(file_path, &st) == 0) {
                file_size = st.st_size;
            }
            
            // Close the MP4 writer to finalize the file
            mp4_writer_close(mp4_writers[i]);
            
            // Update the database to mark the recording as complete
            if (file_path[0] != '\0') {
                // Find any incomplete recordings for this stream in the database
                recording_metadata_t metadata;
                memset(&metadata, 0, sizeof(recording_metadata_t));
                
                // Get the current time for the end timestamp
                time_t end_time = time(NULL);
                
                // Look for recordings with this file path
                // This is a simplified approach - in a real implementation, you'd query the database
                // to find the recording ID for this specific file
                
                // For each active recording, check if it matches this stream
                pthread_mutex_lock(&recordings_mutex);
                for (int j = 0; j < MAX_STREAMS; j++) {
                    if (active_recordings[j].recording_id > 0 && 
                        strcmp(active_recordings[j].stream_name, mp4_writer_stream_names[i]) == 0) {
                        
                        uint64_t recording_id = active_recordings[j].recording_id;
                        
                        // Clear the active recording slot
                        active_recordings[j].recording_id = 0;
                        active_recordings[j].stream_name[0] = '\0';
                        active_recordings[j].output_path[0] = '\0';
                        
                        pthread_mutex_unlock(&recordings_mutex);
                        
                        // Update the recording metadata in the database
                        log_info("Marking recording %llu as complete in database", 
                                (unsigned long long)recording_id);
                        update_recording_metadata(recording_id, end_time, file_size, true);
                        
                        // Re-lock for the next iteration
                        pthread_mutex_lock(&recordings_mutex);
                        break;
                    }
                }
                pthread_mutex_unlock(&recordings_mutex);
                
                // If we didn't find an active recording, try to find it in the database
                // This is a more complex case that would require querying the database
                // by file path, which isn't directly supported in our current API
                
                // Add an event to the database
                add_event(EVENT_RECORDING_STOP, mp4_writer_stream_names[i], 
                         "Recording stopped during shutdown", file_path);
            }
            
            // Clear the writer
            mp4_writers[i] = NULL;
            mp4_writer_stream_names[i][0] = '\0';
        }
    }
    
    pthread_mutex_unlock(&mp4_writers_mutex);
    log_info("All MP4 recordings finalized and marked as complete in database");
}
