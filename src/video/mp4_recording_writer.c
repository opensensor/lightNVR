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
#include <stdbool.h>
#include <unistd.h>
#include <pthread.h>
#include <sys/stat.h>
#include <time.h>
#include <errno.h>

#include "core/logger.h"
#include "utils/strings.h"
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

// Guards both arrays above. Recording threads register and unregister writers
// concurrently, so an unsynchronized scan can observe a slot mid-update and
// hand back a writer that another thread is closing. Never held across
// mp4_writer_close(), which does blocking file I/O.
static pthread_mutex_t mp4_writers_mutex = PTHREAD_MUTEX_INITIALIZER;

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
    safe_strcpy(local_stream_name, stream_name, MAX_STREAM_NAME, 0);

    // Scan for this stream's existing entry before looking at free slots. A
    // single pass that breaks on the first free slot would miss an entry sitting
    // above it and register the stream twice, after which lookups and
    // unregistration would only ever see the first copy and the second would
    // hold its writer open forever.
    mp4_writer_t *old_writer = NULL;
    int slot = -1;
    int existing = -1;
    pthread_mutex_lock(&mp4_writers_mutex);
    for (int i = 0; i < g_config.max_streams; i++) {
        if (!mp4_writers[i]) {
            if (slot == -1) slot = i;
        } else if (mp4_writer_stream_names[i][0] != '\0' &&
                  strcmp(mp4_writer_stream_names[i], local_stream_name) == 0) {
            existing = i;
            break;
        }
    }

    if (existing >= 0) {
        // Stream already has a writer, replace it
        log_info("Replacing existing MP4 writer for stream %s", local_stream_name);

        // Store the old writer to close after releasing the lock
        old_writer = mp4_writers[existing];

        // Replace with the new writer
        mp4_writers[existing] = writer;
        pthread_mutex_unlock(&mp4_writers_mutex);

        // Close the old writer
        if (old_writer) {
            mp4_writer_close(old_writer);
        }

        return 0;
    }

    if (slot == -1) {
        pthread_mutex_unlock(&mp4_writers_mutex);
        log_error("No available slots for MP4 writer registration");
        return -1;
    }

    // Register the new writer
    mp4_writers[slot] = writer;
    safe_strcpy(mp4_writer_stream_names[slot], local_stream_name, sizeof(mp4_writer_stream_names[0]), 0);
    pthread_mutex_unlock(&mp4_writers_mutex);

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
    safe_strcpy(local_stream_name, stream_name, MAX_STREAM_NAME, 0);

    // Use a local variable to store the writer pointer
    mp4_writer_t *writer_copy = NULL;

    pthread_mutex_lock(&mp4_writers_mutex);
    for (int i = 0; i < g_config.max_streams; i++) {
        if (mp4_writers[i] &&
            mp4_writer_stream_names[i][0] != '\0' &&
            strcmp(mp4_writer_stream_names[i], local_stream_name) == 0) {
            writer_copy = mp4_writers[i];
            break;
        }
    }
    pthread_mutex_unlock(&mp4_writers_mutex);

    return writer_copy;
}

/**
 * Get the current recording ID for a stream's continuous recording
 *
 * This function is used in annotation mode to link detections to the
 * ongoing continuous recording for the stream.
 *
 * @param stream_name Name of the stream
 * @return Recording ID (>0) if a continuous recording is active, 0 if not recording
 */
uint64_t get_current_recording_id_for_stream(const char *stream_name) {
    mp4_writer_t *writer = get_mp4_writer_for_stream(stream_name);
    if (writer) {
        return writer->current_recording_id;
    }
    return 0;
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
    safe_strcpy(local_stream_name, stream_name, MAX_STREAM_NAME, 0);
    
    log_info("Unregistering MP4 writer for stream %s", local_stream_name);

    // Find the writer for this stream
    int writer_idx = -1;
    pthread_mutex_lock(&mp4_writers_mutex);
    for (int i = 0; i < g_config.max_streams; i++) {
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
        pthread_mutex_unlock(&mp4_writers_mutex);

        log_info("Unregistered MP4 writer for stream %s", local_stream_name);
    } else {
        pthread_mutex_unlock(&mp4_writers_mutex);
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

    // Keep the shutdown worklist off the stack. At the 1024-stream ceiling,
    // the path buffers alone are too large for common worker-thread stacks.
    typedef struct {
        mp4_writer_t *writer;
        char stream_name[64];
        char file_path[MAX_PATH_LENGTH];
    } writer_cleanup_item_t;

    int capacity = g_config.max_streams;
    if (capacity < 1 || capacity > MAX_STREAMS) {
        capacity = MAX_STREAMS;
    }
    writer_cleanup_item_t *items_to_close = calloc((size_t)capacity, sizeof(*items_to_close));
    if (!items_to_close) {
        log_error("Failed to allocate MP4 writer shutdown worklist for %d streams", capacity);
        return;
    }
    int num_writers_to_close = 0;

    // Claim every registered writer atomically so a concurrent register or
    // unregister cannot hand the same writer to two closers. The stat() calls
    // below run under the lock, which is acceptable on the shutdown path.
    pthread_mutex_lock(&mp4_writers_mutex);
    for (int i = 0; i < capacity; i++) {
        if (mp4_writers[i] && mp4_writer_stream_names[i][0] != '\0') {
            // Store the writer pointer
            mp4_writer_t *writer = mp4_writers[i];
            items_to_close[num_writers_to_close].writer = writer;

            // Make a safe copy of the stream name FIRST from the static array (known safe memory)
            safe_strcpy(items_to_close[num_writers_to_close].stream_name,
                    mp4_writer_stream_names[i], sizeof(items_to_close[0].stream_name), 0);

            // Clear the entry in the global array IMMEDIATELY to prevent any race conditions
            // This must be done before we access the writer's fields
            mp4_writers[i] = NULL;
            mp4_writer_stream_names[i][0] = '\0';

            // Now safely try to access the writer's output_path
            // Use the stream_name from the writer to validate it's still valid
            // (if the stream_name matches, the writer is likely still valid)
            bool writer_valid = false;
            if (writer->stream_name[0] != '\0' &&
                strcmp(writer->stream_name, items_to_close[num_writers_to_close].stream_name) == 0) {
                writer_valid = true;
            }

            if (writer_valid && writer->output_path[0] != '\0') {
                safe_strcpy(items_to_close[num_writers_to_close].file_path,
                        writer->output_path, MAX_PATH_LENGTH, 0);

                // Log the path we're about to check
                log_info("Checking MP4 file: %s", items_to_close[num_writers_to_close].file_path);

                // Get file size before closing
                struct stat st;
                if (stat(items_to_close[num_writers_to_close].file_path, &st) == 0) {
                    log_info("MP4 file size: %llu bytes", (unsigned long long)st.st_size);
                } else {
                    log_warn("Cannot stat MP4 file: %s (error: %s)",
                            items_to_close[num_writers_to_close].file_path,
                            strerror(errno));
                }
            } else {
                log_warn("MP4 writer for stream %s has invalid or empty output path (writer_valid=%d)",
                        items_to_close[num_writers_to_close].stream_name, writer_valid);
                // Still set an empty path so we know not to use it later
                items_to_close[num_writers_to_close].file_path[0] = '\0';
            }

            // Increment counter
            num_writers_to_close++;
        }
    }
    pthread_mutex_unlock(&mp4_writers_mutex);

    // Now close each writer
    for (int i = 0; i < num_writers_to_close; i++) {
        log_info("Finalizing MP4 recording for stream: %s", items_to_close[i].stream_name);
        
        // Log before closing
        log_info("Closing MP4 writer for stream %s at %s", 
                items_to_close[i].stream_name,
                items_to_close[i].file_path[0] != '\0' ? items_to_close[i].file_path : "(empty path)");
        
        // Update recording contexts to prevent double-free
        for (int j = 0; j < capacity; j++) {
            if (recording_contexts[j] && 
                strcmp(recording_contexts[j]->config.name, items_to_close[i].stream_name) == 0) {
                // If this recording context references the writer we're about to close,
                // NULL out the reference to prevent double-free
                if (recording_contexts[j]->mp4_writer == items_to_close[i].writer) {
                    log_info("Clearing mp4_writer reference in recording context for %s", 
                            items_to_close[i].stream_name);
                    recording_contexts[j]->mp4_writer = NULL;
                }
            }
        }

        // Close the MP4 writer to finalize the file
        if (items_to_close[i].writer != NULL) {
            mp4_writer_close(items_to_close[i].writer);
            items_to_close[i].writer = NULL; // Set to NULL to prevent any accidental use after free
        }
        
        // Update the database to mark the recording as complete
        if (items_to_close[i].file_path[0] != '\0') {
            // Add an event to the database
            add_event(EVENT_RECORDING_STOP, items_to_close[i].stream_name,
                     "Recording stopped during shutdown", items_to_close[i].file_path);
        }
    }

    free(items_to_close);
    log_info("All MP4 recordings finalized (%d writers closed)", num_writers_to_close);
}
