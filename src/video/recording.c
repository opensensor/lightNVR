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

// We no longer maintain a separate array of MP4 writers here
// Instead, we use the functions from mp4_recording.c

// Array to store active recordings (one for each stream)
// These are made non-static so they can be accessed from mp4_writer.c
active_recording_t active_recordings[MAX_STREAMS];

/**
 * Initialize the active recordings array
 */
void init_recordings(void) {
    memset(active_recordings, 0, sizeof(active_recordings));
}

/**
 * Function to initialize the recording system
 */
void init_recordings_system(void) {
    init_recordings();
    log_info("Recordings system initialized");
}

// These functions are now defined in mp4_recording.c
// Forward declarations to use them here
extern int register_mp4_writer_for_stream(const char *stream_name, mp4_writer_t *writer);
extern mp4_writer_t *get_mp4_writer_for_stream(const char *stream_name);
extern void unregister_mp4_writer_for_stream(const char *stream_name);

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

            // Mark the existing recording as complete
            time_t end_time = time(NULL);
            update_recording_metadata(existing_recording_id, end_time, 0, true);
            
            log_info("Marked existing recording %llu as complete", 
                    (unsigned long long)existing_recording_id);
            
            // Re-lock the mutex for the next section
            break;
        }
    }

    // Create recording metadata
    recording_metadata_t metadata;
    memset(&metadata, 0, sizeof(recording_metadata_t));

    strncpy(metadata.stream_name, stream_name, sizeof(metadata.stream_name) - 1);

    // Format paths for the recording - MAKE SURE THIS POINTS TO REAL FILES
    char mp4_path[MAX_PATH_LENGTH];
    
    // Get the MP4 writer for this stream to get the actual path
    mp4_writer_t *mp4_writer = get_mp4_writer_for_stream(stream_name);
    if (mp4_writer && mp4_writer->output_path) {
        // Use the actual MP4 file path from the writer
        strncpy(mp4_path, mp4_writer->output_path, sizeof(mp4_path) - 1);
        mp4_path[sizeof(mp4_path) - 1] = '\0';
        
        // Store the actual MP4 path in the metadata
        strncpy(metadata.file_path, mp4_path, sizeof(metadata.file_path) - 1);
        metadata.file_path[sizeof(metadata.file_path) - 1] = '\0';
        
        log_info("Using actual MP4 path for recording: %s", mp4_path);
    } else {
        // Fallback to a default path if no writer is available
        snprintf(mp4_path, sizeof(mp4_path), "%s/recording.mp4", output_path);
        strncpy(metadata.file_path, mp4_path, sizeof(metadata.file_path) - 1);
        metadata.file_path[sizeof(metadata.file_path) - 1] = '\0';
        
        log_warn("No MP4 writer found for stream %s, using default path: %s", stream_name, mp4_path);
    }

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
    for (int i = 0; i < MAX_STREAMS; i++) {
        if (active_recordings[i].recording_id == 0) {
            active_recordings[i].recording_id = recording_id;
            strncpy(active_recordings[i].stream_name, stream_name, MAX_STREAM_NAME - 1);
            strncpy(active_recordings[i].output_path, output_path, MAX_PATH_LENGTH - 1);
            active_recordings[i].start_time = metadata.start_time;
            
            log_info("Started recording for stream %s with ID %llu", 
                    stream_name, (unsigned long long)recording_id);
            
            return recording_id;
        }
    }
    
    // No free slots
    log_error("No free slots for active recordings");
    return 0;
}

/**
 * Update recording metadata with current size and segment count
 */
void update_recording(const char *stream_name) {
    if (!stream_name) return;

    // Find the active recording for this stream
    for (int i = 0; i < MAX_STREAMS; i++) {
        if (active_recordings[i].recording_id > 0 && 
            strcmp(active_recordings[i].stream_name, stream_name) == 0) {
            
            uint64_t recording_id = active_recordings[i].recording_id;
            char output_path[MAX_PATH_LENGTH];
            strncpy(output_path, active_recordings[i].output_path, MAX_PATH_LENGTH - 1);
            output_path[MAX_PATH_LENGTH - 1] = '\0';

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
}

/**
 * Stop an active recording
 */
void stop_recording(const char *stream_name) {
    if (!stream_name) return;

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
            
            // Get the MP4 writer for this stream
            mp4_writer_t *mp4_writer = get_mp4_writer_for_stream(stream_name);
            if (mp4_writer) {
                // Update the file path in the database with the actual MP4 path
                recording_metadata_t metadata;
                if (get_recording_metadata_by_id(recording_id, &metadata) == 0) {
                    if (mp4_writer->output_path && mp4_writer->output_path[0] != '\0') {
                        strncpy(metadata.file_path, mp4_writer->output_path, sizeof(metadata.file_path) - 1);
                        metadata.file_path[sizeof(metadata.file_path) - 1] = '\0';
                        update_recording_metadata(recording_id, end_time, total_size, true);
                        log_info("Updated recording %llu with actual MP4 path: %s", 
                                (unsigned long long)recording_id, metadata.file_path);
                    }
                }
                
                // Note: We don't unregister the MP4 writer here as that's handled by stop_mp4_recording
                // which should be called separately
            }

            log_info("Completed recording %llu for stream %s, duration: %ld seconds, size: %llu bytes", 
                    (unsigned long long)recording_id, stream_name, 
                    (long)(end_time - start_time), 
                    (unsigned long long)total_size);
            
            return;
        }
    }
}

// This function is now defined in mp4_recording.c
extern int start_mp4_recording(const char *stream_name);

/**
 * Get the recording state for a stream
 * Returns 1 if recording is active, 0 if not, -1 on error
 */
int get_recording_state(const char *stream_name) {
    if (!stream_name) {
        log_error("Invalid stream name for get_recording_state");
        return -1;
    }

    // Check if there's an active MP4 writer with a running recording thread
    // This is the more reliable check as it verifies the thread is actually running
    mp4_writer_t *writer = get_mp4_writer_for_stream(stream_name);
    if (writer) {
        // Check if the recording thread is actually running
        // A writer object can exist but have a stopped/dead thread
        if (mp4_writer_is_recording(writer)) {
            return 1; // Recording thread is actively running
        }
        // Writer exists but thread is not running - recording has died
        log_debug("MP4 writer exists for stream %s but recording thread is not running", stream_name);
        return 0;
    }

    // Also check the active_recordings array for backward compatibility
    for (int i = 0; i < MAX_STREAMS; i++) {
        if (active_recordings[i].recording_id > 0 &&
            strcmp(active_recordings[i].stream_name, stream_name) == 0) {
            return 1; // Recording is active in legacy system
        }
    }

    return 0; // No active recording
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

// This function is now defined in mp4_recording.c
extern void close_all_mp4_writers(void);
