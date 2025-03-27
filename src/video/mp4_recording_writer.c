/**
 * MP4 Recording Writer Module
 * 
 * This module is responsible for managing MP4 writers.
 * It provides functions to register, unregister, and get MP4 writers for streams.
 */

#define _POSIX_C_SOURCE 200809L
#define _XOPEN_SOURCE 700
#define _GNU_SOURCE

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <pthread.h>
#include <sys/stat.h>
#include <time.h>
#include <errno.h>

#include "core/logger.h"
#include "video/mp4_recording.h"
#include "video/mp4_recording_internal.h"
#include "video/mp4_writer.h"
#include "video/stream_manager.h"
#include "video/streams.h"
#include "database/database_manager.h"
#include "database/db_events.h"

// Global array to store MP4 writers
static mp4_writer_t *mp4_writers[MAX_STREAMS] = {0};
static char mp4_writer_stream_names[MAX_STREAMS][64] = {{0}};

/**
 * Register an MP4 writer for a stream
 * 
 * This function registers an MP4 writer for a stream, replacing any existing writer.
 * 
 * @param stream_name Name of the stream
 * @param writer MP4 writer instance
 * @return 0 on success, non-zero on failure
 */
int register_mp4_writer_for_stream(const char *stream_name, mp4_writer_t *writer) {
    // Validate parameters
    if (!stream_name || !writer) {
        log_error("Invalid parameters for register_mp4_writer_for_stream");
        return -1;
    }
    
    // Make a local copy of the stream name for thread safety
    char local_stream_name[MAX_STREAM_NAME];
    strncpy(local_stream_name, stream_name, MAX_STREAM_NAME - 1);
    local_stream_name[MAX_STREAM_NAME - 1] = '\0';

    // Find empty slot or existing entry for this stream
    int slot = -1;
    for (int i = 0; i < MAX_STREAMS; i++) {
        if (!mp4_writers[i]) {
            slot = i;
            break;
        } else if (mp4_writer_stream_names[i][0] != '\0' && 
                  strcmp(mp4_writer_stream_names[i], local_stream_name) == 0) {
            // Stream already has a writer, replace it
            log_info("Replacing existing MP4 writer for stream %s", local_stream_name);
            
            // Store the old writer to close after releasing the lock
            mp4_writer_t *old_writer = mp4_writers[i];
            
            // Replace with the new writer
            mp4_writers[i] = writer;
            
            // Close the old writer
            if (old_writer) {
                mp4_writer_close(old_writer);
            }
            
            return 0;
        }
    }

    if (slot == -1) {
        log_error("No available slots for MP4 writer registration");
        return -1;
    }

    // Register the new writer
    mp4_writers[slot] = writer;
    strncpy(mp4_writer_stream_names[slot], local_stream_name, sizeof(mp4_writer_stream_names[0]) - 1);
    mp4_writer_stream_names[slot][sizeof(mp4_writer_stream_names[0]) - 1] = '\0';
    
    log_info("Registered MP4 writer for stream %s in slot %d", local_stream_name, slot);

    return 0;
}

/**
 * Get the MP4 writer for a stream
 * 
 * This function returns the MP4 writer for a stream.
 * 
 * @param stream_name Name of the stream
 * @return MP4 writer instance or NULL if not found
 */
mp4_writer_t *get_mp4_writer_for_stream(const char *stream_name) {
    // Validate parameters
    if (!stream_name || stream_name[0] == '\0') {
        return NULL;
    }
    
    // Make a local copy of the stream name for thread safety
    char local_stream_name[MAX_STREAM_NAME];
    strncpy(local_stream_name, stream_name, MAX_STREAM_NAME - 1);
    local_stream_name[MAX_STREAM_NAME - 1] = '\0';

    // Use a local variable to store the writer pointer
    mp4_writer_t *writer_copy = NULL;

    for (int i = 0; i < MAX_STREAMS; i++) {
        if (mp4_writers[i] && 
            mp4_writer_stream_names[i][0] != '\0' && 
            strcmp(mp4_writer_stream_names[i], local_stream_name) == 0) {
            writer_copy = mp4_writers[i];
            break;
        }
    }

    return writer_copy;
}

/**
 * Unregister an MP4 writer for a stream
 * 
 * This function unregisters an MP4 writer for a stream.
 * The caller is responsible for closing the writer if needed.
 * 
 * @param stream_name Name of the stream
 */
void unregister_mp4_writer_for_stream(const char *stream_name) {
    // Validate parameters
    if (!stream_name || stream_name[0] == '\0') {
        log_warn("Invalid stream name passed to unregister_mp4_writer_for_stream");
        return;
    }
    
    // Make a local copy of the stream name for thread safety
    char local_stream_name[MAX_STREAM_NAME];
    strncpy(local_stream_name, stream_name, MAX_STREAM_NAME - 1);
    local_stream_name[MAX_STREAM_NAME - 1] = '\0';
    
    log_info("Unregistering MP4 writer for stream %s", local_stream_name);

    // Find the writer for this stream
    int writer_idx = -1;
    for (int i = 0; i < MAX_STREAMS; i++) {
        if (mp4_writers[i] && 
            mp4_writer_stream_names[i][0] != '\0' && 
            strcmp(mp4_writer_stream_names[i], local_stream_name) == 0) {
            writer_idx = i;
            break;
        }
    }
    
    // If we found a writer, unregister it
    if (writer_idx >= 0) {
        // Don't close the writer here, just unregister it
        // The caller is responsible for closing the writer if needed
        mp4_writers[writer_idx] = NULL;
        mp4_writer_stream_names[writer_idx][0] = '\0';
        
        log_info("Unregistered MP4 writer for stream %s", local_stream_name);
    } else {
        log_warn("No MP4 writer found for stream %s", local_stream_name);
    }
}

/**
 * Close all MP4 writers during shutdown
 * 
 * This function closes all MP4 writers and updates the database.
 */
void close_all_mp4_writers(void) {
    log_info("Finalizing all MP4 recordings...");
    
    // Create a local array to store writers we need to close
    // This prevents double-free issues by ensuring we only close each writer once
    mp4_writer_t *writers_to_close[MAX_STREAMS] = {0};
    char stream_names_to_close[MAX_STREAMS][64] = {{0}};
    char file_paths_to_close[MAX_STREAMS][MAX_PATH_LENGTH] = {{0}};
    int num_writers_to_close = 0;
    
    for (int i = 0; i < MAX_STREAMS; i++) {
        if (mp4_writers[i] && mp4_writer_stream_names[i][0] != '\0') {
            // Store the writer pointer
            writers_to_close[num_writers_to_close] = mp4_writers[i];
            
            // Make a safe copy of the stream name
            strncpy(stream_names_to_close[num_writers_to_close], 
                    mp4_writer_stream_names[i], 
                    sizeof(stream_names_to_close[0]) - 1);
            stream_names_to_close[num_writers_to_close][sizeof(stream_names_to_close[0]) - 1] = '\0';
            
            // Safely get the file path from the MP4 writer
            if (mp4_writers[i]->output_path && mp4_writers[i]->output_path[0] != '\0') {
                strncpy(file_paths_to_close[num_writers_to_close], 
                        mp4_writers[i]->output_path, 
                        MAX_PATH_LENGTH - 1);
                file_paths_to_close[num_writers_to_close][MAX_PATH_LENGTH - 1] = '\0';
                
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

    // Now close each writer
    for (int i = 0; i < num_writers_to_close; i++) {
        log_info("Finalizing MP4 recording for stream: %s", stream_names_to_close[i]);
        
        // Log before closing
        log_info("Closing MP4 writer for stream %s at %s", 
                stream_names_to_close[i], 
                file_paths_to_close[i][0] != '\0' ? file_paths_to_close[i] : "(empty path)");
        
        // Update recording contexts to prevent double-free
        for (int j = 0; j < MAX_STREAMS; j++) {
            if (recording_contexts[j] && 
                strcmp(recording_contexts[j]->config.name, stream_names_to_close[i]) == 0) {
                // If this recording context references the writer we're about to close,
                // NULL out the reference to prevent double-free
                if (recording_contexts[j]->mp4_writer == writers_to_close[i]) {
                    log_info("Clearing mp4_writer reference in recording context for %s", 
                            stream_names_to_close[i]);
                    recording_contexts[j]->mp4_writer = NULL;
                }
            }
        }

        // Close the MP4 writer to finalize the file
        if (writers_to_close[i] != NULL) {
            mp4_writer_close(writers_to_close[i]);
            writers_to_close[i] = NULL; // Set to NULL to prevent any accidental use after free
        }
        
        // Update the database to mark the recording as complete
        if (file_paths_to_close[i][0] != '\0') {
            // Add an event to the database
            add_event(EVENT_RECORDING_STOP, stream_names_to_close[i], 
                     "Recording stopped during shutdown", file_paths_to_close[i]);
        }
    }
    
    log_info("All MP4 recordings finalized (%d writers closed)", num_writers_to_close);
}
