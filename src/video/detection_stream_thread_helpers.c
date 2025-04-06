/**
 * Helper functions for detection_stream_thread.c
 * These functions break down the complex check_for_new_segments function
 * into smaller, more manageable pieces.
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <pthread.h>
#include <stdbool.h>
#include <sys/stat.h>
#include <dirent.h>
#include <time.h>
#include <stdatomic.h>
#include <curl/curl.h>
#include <errno.h>

#include "core/logger.h"
#include "core/config.h"
#include "video/detection_stream_thread.h"
#include "video/detection_stream_thread_helpers.h"
#include "video/streams.h"
#include "video/hls_writer.h"
#include "utils/strings.h"

// Forward declaration of the internal function - this is defined in detection_stream_thread.c
extern int process_segment_for_detection(stream_detection_thread_t *thread, const char *segment_path);

// Forward declaration of global variable from detection_stream_thread.c
extern time_t global_startup_delay_end;

/**
 * Check if detection should run based on startup delay, detection in progress, and time interval
 * Returns true if detection should run, false otherwise
 */
bool should_run_detection_check(stream_detection_thread_t *thread, time_t current_time) {
    // CRITICAL FIX: Add safety checks to prevent memory corruption
    if (!thread) {
        log_error("should_run_detection_check: thread is NULL");
        return false;
    }

    // Check if we're still in the startup delay period with safety checks
    if (global_startup_delay_end > 0 && current_time < global_startup_delay_end) {
        log_info("[Stream %s] In startup delay period, waiting %ld more seconds before processing segments",
                thread->stream_name ? thread->stream_name : "unknown", global_startup_delay_end - current_time);
        return false;
    }

    // Check if a detection is already in progress with safety checks
    int detection_running = 0;

    // Safely load the atomic value
    detection_running = atomic_load(&thread->detection_in_progress);

    // Check if a detection has been running for too long (more than 60 seconds)
    // This prevents a stuck detection_in_progress flag from blocking all future detections
    if (detection_running && thread->last_detection_time > 0) {
        time_t detection_time = current_time - thread->last_detection_time;
        if (detection_time > 60) {  // 60 seconds timeout
            log_warn("[Stream %s] Detection has been running for %ld seconds, which is too long. Resetting flag.",
                    thread->stream_name ? thread->stream_name : "unknown", detection_time);
            atomic_store(&thread->detection_in_progress, 0);
            detection_running = 0;  // Update local variable to reflect the change
        }
    }

    if (detection_running) {
        log_info("[Stream %s] Detection already in progress, skipping segment check", thread->stream_name);
        return false;
    }

    // Check if enough time has passed since the last detection
    if (thread->last_detection_time > 0) {
        time_t time_since_last = current_time - thread->last_detection_time;
        if (time_since_last < thread->detection_interval) {
            // Not enough time has passed for detection
            log_info("[Stream %s] Checking for segments (last detection was %ld seconds ago, interval: %d seconds)",
                     thread->stream_name, time_since_last, thread->detection_interval);
            return false;
        }

        // Enough time has passed and no detection is running
        log_info("[Stream %s] Time for a new detection (%ld seconds since last, interval: %d seconds)",
                thread->stream_name, time_since_last, thread->detection_interval);
        return true;
    }

    // No previous detection, we should run one
    log_info("[Stream %s] No previous detection, running first detection", thread->stream_name);
    return true;
}

/**
 * Check and manage HLS writer status
 * Returns true if HLS writer is recording, false otherwise
 */
bool check_hls_writer_status(stream_detection_thread_t *thread, time_t current_time,
                            time_t *last_warning_time, bool first_check, int *consecutive_failures) {
    bool hls_writer_recording = false;
    stream_handle_t stream = get_stream_by_name(thread->stream_name);

    if (!stream) {
        // Only log a warning every 60 seconds to avoid log spam
        if (current_time - *last_warning_time > 60 || first_check) {
            log_warn("[Stream %s] Failed to get stream handle, but will still check for segments", thread->stream_name);
            *last_warning_time = current_time;
        }
        return false;
    }

    // Get the HLS writer
    hls_writer_t *writer = get_stream_hls_writer(stream);
    if (!writer) {
        // Only log a warning every 60 seconds to avoid log spam
        if (current_time - *last_warning_time > 60 || first_check) {
            log_warn("[Stream %s] No HLS writer available, but will still check for segments", thread->stream_name);
            *last_warning_time = current_time;
        }
        return false;
    }

    // Check if the HLS writer is recording
    hls_writer_recording = is_hls_stream_active(thread->stream_name);
    if (!hls_writer_recording) {
        // Only log a warning every 60 seconds to avoid log spam
        if (current_time - *last_warning_time > 60 || first_check) {
            log_warn("[Stream %s] HLS writer is not recording, attempting to restart...", thread->stream_name);
            *last_warning_time = current_time;

            // Try to restart the HLS writer if it's not recording
            stream_config_t config;
            if (get_stream_config(stream, &config) == 0) {
                // Stop and restart the HLS stream
                if (config.url[0] != '\0') {
                    log_info("[Stream %s] Attempting to restart HLS stream with URL: %s",
                            thread->stream_name, config.url);
                    stop_hls_stream(thread->stream_name);

                    // Wait a short time before restarting
                    usleep(500000); // 500ms

                    // Restart the HLS stream
                    start_hls_stream(thread->stream_name);
                }
            }
        }
    } else {
        log_info("[Stream %s] HLS writer is recording, checking for new segments", thread->stream_name);
        *consecutive_failures = 0; // Reset failure counter when HLS writer is recording
    }

    return hls_writer_recording;
}

/**
 * Find and set the HLS directory path for a stream
 * Updates thread->hls_dir with the correct path
 * Returns true if a valid directory was found, false otherwise
 */
bool find_hls_directory(stream_detection_thread_t *thread, time_t current_time,
                        time_t *last_warning_time, int *consecutive_failures, bool first_check) {
    extern config_t g_config;

    // Get the HLS directory for this stream using the global config
    char hls_dir[MAX_PATH_LENGTH];

    // Use storage_path_hls if specified, otherwise fall back to storage_path
    const char *base_storage_path = g_config.storage_path;
    if (g_config.storage_path_hls[0] != '\0') {
        base_storage_path = g_config.storage_path_hls;
        log_info("[Stream %s] Using dedicated HLS storage path: %s", thread->stream_name, base_storage_path);
    }

    // Try both possible HLS directory paths
    char standard_path[MAX_PATH_LENGTH];
    char alternative_path[MAX_PATH_LENGTH];

    // Standard path: base_storage_path/hls/stream_name
    snprintf(standard_path, MAX_PATH_LENGTH, "%s/hls/%s", base_storage_path, thread->stream_name);

    // Alternative path: base_storage_path/hls/hls/stream_name
    snprintf(alternative_path, MAX_PATH_LENGTH, "%s/hls/hls/%s", base_storage_path, thread->stream_name);

    // Check which path exists and has segments
    struct stat st_standard, st_alternative;
    bool standard_exists = (stat(standard_path, &st_standard) == 0 && S_ISDIR(st_standard.st_mode));
    bool alternative_exists = (stat(alternative_path, &st_alternative) == 0 && S_ISDIR(st_alternative.st_mode));

    // Determine which path to use
    if (standard_exists) {
        DIR *dir = opendir(standard_path);
        int segment_count = 0;
        if (dir) {
            struct dirent *entry;
            while ((entry = readdir(dir)) != NULL) {
                if (strstr(entry->d_name, ".ts") || strstr(entry->d_name, ".m4s")) {
                    segment_count++;
                    break;
                }
            }
            closedir(dir);
        }

        if (segment_count > 0) {
            strncpy(hls_dir, standard_path, MAX_PATH_LENGTH - 1);
            log_info("[Stream %s] Using standard HLS directory path with segments: %s",
                    thread->stream_name, standard_path);
        } else if (alternative_exists) {
            // Standard path exists but has no segments, try alternative
            dir = opendir(alternative_path);
            segment_count = 0;
            if (dir) {
                struct dirent *entry;
                while ((entry = readdir(dir)) != NULL) {
                    if (strstr(entry->d_name, ".ts") || strstr(entry->d_name, ".m4s")) {
                        segment_count++;
                        break;
                    }
                }
                closedir(dir);
            }

            if (segment_count > 0) {
                strncpy(hls_dir, alternative_path, MAX_PATH_LENGTH - 1);
                log_info("[Stream %s] Using alternative HLS directory path with segments: %s",
                        thread->stream_name, alternative_path);
            } else {
                // No segments in either path, default to standard
                strncpy(hls_dir, standard_path, MAX_PATH_LENGTH - 1);
                log_info("[Stream %s] No segments found, defaulting to standard HLS directory path: %s",
                        thread->stream_name, standard_path);
            }
        } else {
            // Alternative doesn't exist, use standard
            strncpy(hls_dir, standard_path, MAX_PATH_LENGTH - 1);
            log_info("[Stream %s] Using standard HLS directory path: %s",
                    thread->stream_name, standard_path);
        }
    } else if (alternative_exists) {
        // Standard doesn't exist but alternative does
        strncpy(hls_dir, alternative_path, MAX_PATH_LENGTH - 1);
        log_info("[Stream %s] Using alternative HLS directory path: %s",
                thread->stream_name, alternative_path);
    } else {
        // Neither exists, create and use standard path
        strncpy(hls_dir, standard_path, MAX_PATH_LENGTH - 1);
        log_info("[Stream %s] Creating standard HLS directory path: %s",
                thread->stream_name, standard_path);

        // Create the directory
        char cmd[MAX_PATH_LENGTH * 2];
        snprintf(cmd, sizeof(cmd), "mkdir -p %s", standard_path);
        system(cmd);
    }

    hls_dir[MAX_PATH_LENGTH - 1] = '\0';

    // Update the thread's HLS directory path
    strncpy(thread->hls_dir, hls_dir, MAX_PATH_LENGTH - 1);
    thread->hls_dir[MAX_PATH_LENGTH - 1] = '\0';

    log_info("[Stream %s] Using HLS directory: %s", thread->stream_name, thread->hls_dir);

    // Check if the directory exists and is accessible
    DIR *dir = opendir(thread->hls_dir);
    if (!dir) {
        // Only log an error every 60 seconds to avoid log spam
        if (current_time - *last_warning_time > 60 || first_check) {
            log_error("[Stream %s] Failed to open HLS directory: %s (error: %s)",
                     thread->stream_name, thread->hls_dir, strerror(errno));
            *last_warning_time = current_time;
        }

        (*consecutive_failures)++;

        // If we've failed too many times, try to create the directory
        if (*consecutive_failures > 10) {
            log_warn("[Stream %s] Too many consecutive failures, trying to create HLS directory", thread->stream_name);
            if (mkdir(thread->hls_dir, 0755) == 0) {
                log_info("[Stream %s] Successfully created HLS directory: %s", thread->stream_name, thread->hls_dir);
                *consecutive_failures = 0;
                return true;
            } else {
                log_error("[Stream %s] Failed to create HLS directory: %s (error: %s)",
                         thread->stream_name, thread->hls_dir, strerror(errno));
                return false;
            }
        }
        return false;
    }

    closedir(dir);
    return true;
}

/**
 * Find the newest segment file in the HLS directory
 * Returns true if a segment was found, false otherwise
 * Updates newest_segment with the path to the newest segment if found
 */
bool find_newest_segment(stream_detection_thread_t *thread, char *newest_segment,
                         time_t *newest_time, int *segment_count) {
    DIR *dir;
    struct dirent *entry;
    struct stat st;

    *segment_count = 0;
    *newest_time = 0;
    newest_segment[0] = '\0';

    log_info("[Stream %s] Scanning directory for segments: %s", thread->stream_name, thread->hls_dir);

    dir = opendir(thread->hls_dir);
    if (!dir) {
        log_error("[Stream %s] Failed to open HLS directory: %s (error: %s)",
                 thread->stream_name, thread->hls_dir, strerror(errno));
        return false;
    }

    while ((entry = readdir(dir)) != NULL) {
        // Check for both .ts and .m4s files (different HLS segment formats)
        if (!strstr(entry->d_name, ".ts") && !strstr(entry->d_name, ".m4s")) {
            continue;
        }

        (*segment_count)++;

        // Construct full path
        char segment_path[MAX_PATH_LENGTH];
        snprintf(segment_path, MAX_PATH_LENGTH, "%s/%s", thread->hls_dir, entry->d_name);

        log_info("[Stream %s] Found segment file: %s", thread->stream_name, entry->d_name);

        // Get file stats
        if (stat(segment_path, &st) == 0) {
            // Check if this is the newest file
            if (st.st_mtime > *newest_time) {
                *newest_time = st.st_mtime;
                strncpy(newest_segment, segment_path, MAX_PATH_LENGTH - 1);
                newest_segment[MAX_PATH_LENGTH - 1] = '\0';
                log_info("[Stream %s] New newest segment: %s (mtime: %ld)",
                        thread->stream_name, segment_path, (long)st.st_mtime);
            }
        }
    }

    closedir(dir);
    return (*segment_count > 0 && newest_segment[0] != '\0');
}

/**
 * Check if a segment should be processed
 * Returns true if the segment should be processed, false otherwise
 */
bool should_process_segment(const char *newest_segment, const char *last_processed_segment,
                          bool should_run_detection) {
    bool is_new_segment = (strcmp(newest_segment, last_processed_segment) != 0);
    bool should_process = is_new_segment && should_run_detection;

    if (should_process) {
        return true;
    }

    if (!is_new_segment) {
        return false;
    }

    if (!should_run_detection) {
        return false;
    }

    return false;
}

/**
 * Attempt to restart the HLS stream if no segments were found
 */
void restart_hls_stream_if_needed(stream_detection_thread_t *thread, time_t current_time,
                                 time_t *last_warning_time, bool first_check) {
    // Only log a warning every 60 seconds to avoid log spam
    if (current_time - *last_warning_time > 60 || first_check) {
        log_warn("[Stream %s] No segments found in directory: %s", thread->stream_name, thread->hls_dir);
        *last_warning_time = current_time;

        // Check if the HLS writer is recording
        stream_handle_t stream = get_stream_by_name(thread->stream_name);
        if (stream) {
            hls_writer_t *writer = get_stream_hls_writer(stream);
            if (writer) {
                bool is_recording = is_hls_stream_active(thread->stream_name);
                log_info("[Stream %s] HLS writer recording status: %s",
                        thread->stream_name, is_recording ? "RECORDING" : "NOT RECORDING");

                if (!is_recording) {
                    // Try to restart the HLS writer
                    stream_config_t config;
                    if (get_stream_config(stream, &config) == 0 && config.url[0] != '\0') {
                        log_info("[Stream %s] Attempting to restart HLS writer with URL: %s",
                                thread->stream_name, config.url);

                        // Stop and restart the HLS stream
                        log_info("[Stream %s] Attempting to restart HLS stream", thread->stream_name);
                        stop_hls_stream(thread->stream_name);

                        // Wait a short time before restarting
                        usleep(500000); // 500ms

                        // Restart the HLS stream
                        start_hls_stream(thread->stream_name);
                        log_info("[Stream %s] HLS stream restart attempted", thread->stream_name);
                    }
                }
            }
        }
    }
}

/**
 * Process a segment if it should be processed
 * Returns 0 on success, non-zero on failure
 */
int process_segment_if_needed(stream_detection_thread_t *thread,
                             const char *newest_segment,
                             const char *last_processed_segment,
                             bool should_run_detection,
                             time_t newest_time,
                             time_t current_time) {
    // Static variable to track the last processed segment
    static char last_segment_processed[MAX_PATH_LENGTH] = {0};
    char local_last_processed[MAX_PATH_LENGTH] = {0};

    // If last_processed_segment is provided, use it, otherwise use our static variable
    if (last_processed_segment && last_processed_segment[0] != '\0') {
        strncpy(local_last_processed, last_processed_segment, MAX_PATH_LENGTH - 1);
        local_last_processed[MAX_PATH_LENGTH - 1] = '\0';
    } else {
        strncpy(local_last_processed, last_segment_processed, MAX_PATH_LENGTH - 1);
        local_last_processed[MAX_PATH_LENGTH - 1] = '\0';
    }

    // Determine if we should process this segment
    // Process if:
    // 1. It's a new segment (different from the last one we processed) AND
    // 2. We should run a detection (based on time and no detection in progress)
    bool is_new_segment = (strcmp(newest_segment, local_last_processed) != 0);
    bool should_process = is_new_segment && should_run_detection;

    // Log the decision
    if (should_process) {
        log_info("[Stream %s] Processing new segment (different from last processed)", thread->stream_name);
    } else if (!is_new_segment) {
        log_info("[Stream %s] Skipping segment processing (same as last processed): %s",
                 thread->stream_name, newest_segment);
        return 0;
    } else if (!should_run_detection) {
        log_info("[Stream %s] Skipping segment processing (detection not due or already running): %s",
                 thread->stream_name, newest_segment);
        return 0;
    }

    if (should_process) {
        // Verify the segment still exists before processing
        if (access(newest_segment, F_OK) != 0) {
            log_warn("[Stream %s] Segment no longer exists: %s", thread->stream_name, newest_segment);
            return -1;
        }

        log_info("[Stream %s] Processing segment: %s (age: %ld seconds)",
                thread->stream_name, newest_segment, current_time - newest_time);

        // Set the atomic flag to indicate a detection is in progress
        atomic_store(&thread->detection_in_progress, 1);

        int result = process_segment_for_detection(thread, newest_segment);

        // Clear the atomic flag when detection is complete
        atomic_store(&thread->detection_in_progress, 0);

        if (result == 0) {
            log_info("[Stream %s] Successfully processed segment: %s", thread->stream_name, newest_segment);

            // Update the last processed segment
            strncpy(last_segment_processed, newest_segment, MAX_PATH_LENGTH - 1);
            last_segment_processed[MAX_PATH_LENGTH - 1] = '\0';

            // Update last detection time
            thread->last_detection_time = current_time;
            return 0;
        } else {
            log_error("[Stream %s] Failed to process segment: %s (error code: %d)",
                     thread->stream_name, newest_segment, result);
            return result;
        }
    }

    return 0;
}
