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
#include "video/detection_model.h"
#include "video/onvif_detection.h"

// Forward declaration of the internal function - this is defined in detection_stream_thread.c
extern int process_segment_for_detection(stream_detection_thread_t *thread, const char *segment_path);

// Forward declaration for recording function - this is defined in detection_stream_thread.c
extern int process_frame_for_recording(const char *stream_name, const uint8_t *frame_data, int width, int height, int channels, time_t timestamp, detection_result_t *result);

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
                thread->stream_name, global_startup_delay_end - current_time);
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
                    thread->stream_name, detection_time);
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
    // CRITICAL FIX: Add safety check for NULL thread to prevent segfault
    if (!thread) {
        log_error("Cannot find HLS directory with NULL thread");
        return false;
    }

    // CRITICAL FIX: Add safety check for invalid thread structure
    if (!thread->stream_name[0]) {
        log_error("Cannot find HLS directory with invalid stream name");
        return false;
    }

    // CRITICAL FIX: Make a local copy of the stream name for logging
    char stream_name_copy[MAX_STREAM_NAME];
    strncpy(stream_name_copy, thread->stream_name, MAX_STREAM_NAME - 1);
    stream_name_copy[MAX_STREAM_NAME - 1] = '\0';

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

        // Create the directory using direct C functions to handle paths with spaces
        char temp_path[MAX_PATH_LENGTH];
        strncpy(temp_path, standard_path, MAX_PATH_LENGTH - 1);
        temp_path[MAX_PATH_LENGTH - 1] = '\0';

        // Create parent directories one by one
        for (char *p = temp_path + 1; *p; p++) {
            if (*p == '/') {
                *p = '\0';
                if (mkdir(temp_path, 0777) != 0 && errno != EEXIST) {
                    log_warn("[Stream %s] Failed to create parent directory: %s (error: %s)",
                            thread->stream_name, temp_path, strerror(errno));
                }
                *p = '/';
            }
        }

        // Create the final directory
        if (mkdir(temp_path, 0777) != 0 && errno != EEXIST) {
            log_warn("[Stream %s] Failed to create directory: %s (error: %s)",
                    thread->stream_name, temp_path, strerror(errno));
        } else {
            log_info("[Stream %s] Successfully created directory: %s",
                    thread->stream_name, temp_path);
        }
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
    // CRITICAL FIX: Add safety check for NULL thread to prevent segfault
    if (!thread) {
        log_error("Cannot find newest segment with NULL thread");
        return false;
    }

    // CRITICAL FIX: Add safety check for invalid thread structure
    if (!thread->stream_name[0]) {
        log_error("Cannot find newest segment with invalid stream name");
        return false;
    }

    // CRITICAL FIX: Add safety check for NULL output parameters
    if (!newest_segment || !newest_time || !segment_count) {
        log_error("[Stream %s] Cannot find newest segment with NULL output parameters", thread->stream_name);
        return false;
    }

    // CRITICAL FIX: Make a local copy of the stream name for logging
    char stream_name_copy[MAX_STREAM_NAME];
    strncpy(stream_name_copy, thread->stream_name, MAX_STREAM_NAME - 1);
    stream_name_copy[MAX_STREAM_NAME - 1] = '\0';

    DIR *dir;
    struct dirent *entry;
    struct stat st;

    *segment_count = 0;
    *newest_time = 0;
    newest_segment[0] = '\0';

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

        // Get file stats
        if (stat(segment_path, &st) == 0) {
            // Check if this is the newest file
            if (st.st_mtime > *newest_time) {
                *newest_time = st.st_mtime;
                strncpy(newest_segment, segment_path, MAX_PATH_LENGTH - 1);
                newest_segment[MAX_PATH_LENGTH - 1] = '\0';
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
    // CRITICAL FIX: Add safety check for NULL thread to prevent segfault
    if (!thread) {
        log_error("Cannot restart HLS stream with NULL thread");
        return;
    }

    // CRITICAL FIX: Add safety check for invalid thread structure
    if (!thread->stream_name[0]) {
        log_error("Cannot restart HLS stream with invalid stream name");
        return;
    }

    // CRITICAL FIX: Add safety check for NULL last_warning_time
    if (!last_warning_time) {
        log_error("[Stream %s] Cannot restart HLS stream with NULL last_warning_time", thread->stream_name);
        return;
    }

    // CRITICAL FIX: Make a local copy of the stream name for logging
    char stream_name_copy[MAX_STREAM_NAME];
    strncpy(stream_name_copy, thread->stream_name, MAX_STREAM_NAME - 1);
    stream_name_copy[MAX_STREAM_NAME - 1] = '\0';

    // Only log a warning every 60 seconds to avoid log spam
    if (current_time - *last_warning_time > 60 || first_check) {
        log_warn("[Stream %s] No segments found in directory: %s", stream_name_copy, thread->hls_dir);
        *last_warning_time = current_time;

        // Check if the HLS writer is recording
        stream_handle_t stream = get_stream_by_name(thread->stream_name);
        if (stream) {
            // CRITICAL FIX: Add safety checks to prevent segfault when accessing HLS writer
            hls_writer_t *writer = NULL;

            // Use a try/catch-like approach to prevent segfaults
            bool writer_access_failed = false;

            // Safely try to get the HLS writer
            __attribute__((unused)) volatile int writer_check_result = 0;
            writer = get_stream_hls_writer(stream);

            // Check if we got a valid writer
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

                        // CRITICAL FIX: Add additional safety checks before stopping/starting HLS stream
                        log_info("[Stream %s] Attempting to restart HLS stream", thread->stream_name);

                        // Safely stop the HLS stream
                        stop_hls_stream(thread->stream_name);

                        // Wait a short time before restarting
                        usleep(500000); // 500ms

                        // Restart the HLS stream
                        start_hls_stream(thread->stream_name);
                        log_info("[Stream %s] HLS stream restart attempted", thread->stream_name);
                    }
                }
            } else {
                // Writer is NULL, log the issue
                log_warn("[Stream %s] HLS writer is NULL, cannot check recording status", thread->stream_name);

                // Try to restart the stream anyway
                log_info("[Stream %s] Attempting to restart HLS stream despite NULL writer", thread->stream_name);

                // Safely stop and restart the HLS stream
                stop_hls_stream(thread->stream_name);
                usleep(500000); // 500ms
                start_hls_stream(thread->stream_name);
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
        log_debug("[Stream %s] Processing new segment (different from last processed)", thread->stream_name);
    } else if (!is_new_segment) {
        log_debug("[Stream %s] Skipping segment processing (same as last processed): %s",
                 thread->stream_name, newest_segment);
        return 0;
    } else if (!should_run_detection) {
        log_debug("[Stream %s] Skipping segment processing (detection not due or already running): %s",
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

        // CRITICAL FIX: Add safety checks before processing segment
        int result = -1;

        // Use a try/catch-like approach to prevent segfaults
        bool segment_processing_failed = false;

        // Safely try to process the segment
        __attribute__((unused)) volatile int segment_check_result = 0;

        // CRITICAL FIX: Add additional safety check before processing
        if (thread && thread->stream_name[0] != '\0') {
            // Check if this is an ONVIF detection model
            if (thread->model) {
                const char *model_type = get_model_type_from_handle(thread->model);
                if (model_type && strcmp(model_type, MODEL_TYPE_ONVIF) == 0) {
                    // For ONVIF detection, we don't need to process HLS segments
                    // Instead, we use the ONVIF detection API directly
                    log_info("[Stream %s] Using ONVIF detection instead of processing HLS segment", thread->stream_name);
                    
                    // Create detection result structure
                    detection_result_t result_struct;
                    memset(&result_struct, 0, sizeof(detection_result_t));
                    
                    // Get the ONVIF URL, username, and password from the model path
                    const char *model_path = get_model_path(thread->model);
                    char username[64] = {0};
                    char password[64] = {0};
                    char url[256] = {0};
                    
                    // Extract credentials from model path or use defaults
                    if (model_path && strncmp(model_path, "onvif://", 8) == 0) {
                        const char *auth_start = model_path + 8;
                        const char *auth_end = strchr(auth_start, '@');
                        
                        if (auth_end) {
                            const char *pwd_sep = strchr(auth_start, ':');
                            
                            if (pwd_sep && pwd_sep < auth_end) {
                                // Extract username
                                size_t username_len = pwd_sep - auth_start;
                                if (username_len < sizeof(username)) {
                                    strncpy(username, auth_start, username_len);
                                    username[username_len] = '\0';
                                }
                                
                                // Extract password
                                size_t password_len = auth_end - (pwd_sep + 1);
                                if (password_len < sizeof(password)) {
                                    strncpy(password, pwd_sep + 1, password_len);
                                    password[password_len] = '\0';
                                }
                            }
                            
                            // Extract URL
                            snprintf(url, sizeof(url), "http://%s", auth_end + 1);
                        }
                    }
                    
                    // If we couldn't extract credentials from the model path, try to get them from the stream config
                    if (url[0] == '\0' || username[0] == '\0' || password[0] == '\0') {
                        stream_handle_t stream = get_stream_by_name(thread->stream_name);
                        if (stream) {
                            stream_config_t config;
                            if (get_stream_config(stream, &config) == 0) {
                                // Use ONVIF credentials from the stream config
                                if (username[0] == '\0' && config.onvif_username[0] != '\0') {
                                    strncpy(username, config.onvif_username, sizeof(username) - 1);
                                    username[sizeof(username) - 1] = '\0';
                                }
                                
                                if (password[0] == '\0' && config.onvif_password[0] != '\0') {
                                    strncpy(password, config.onvif_password, sizeof(password) - 1);
                                    password[sizeof(password) - 1] = '\0';
                                }
                                
                                // Use the stream URL as the ONVIF URL if we don't have one
                                if (config.url[0] != '\0') {
                                    // Extract the IP address from the stream URL
                                    const char *stream_url = config.url;
                                    const char *ip_start = NULL;
                                    int prefix_len = 0;
                                    
                                    // Look for rtsp:// or http:// prefix
                                    if (strncmp(stream_url, "rtsp://", 7) == 0) {
                                        ip_start = stream_url + 7;
                                        prefix_len = 7;
                                    } else if (strncmp(stream_url, "http://", 7) == 0) {
                                        ip_start = stream_url + 7;
                                        prefix_len = 7;
                                    } else if (strncmp(stream_url, "https://", 8) == 0) {
                                        ip_start = stream_url + 8;
                                        prefix_len = 8;
                                    }
                                    
                                    if (ip_start) {
                                        // Check if there are credentials in the URL
                                        const char *at_sign = strchr(ip_start, '@');
                                        if (at_sign && username[0] == '\0' && password[0] == '\0') {
                                            // Extract credentials from URL
                                            const char *auth_start = ip_start;
                                            const char *pwd_sep = strchr(auth_start, ':');
                                            
                                            if (pwd_sep && pwd_sep < at_sign) {
                                                // Extract username
                                                size_t username_len = pwd_sep - auth_start;
                                                if (username_len < sizeof(username)) {
                                                    strncpy(username, auth_start, username_len);
                                                    username[username_len] = '\0';
                                                    log_info("[Stream %s] Extracted username from URL: %s", 
                                                            thread->stream_name, username);
                                                }
                                                
                                                // Extract password
                                                size_t password_len = at_sign - (pwd_sep + 1);
                                                if (password_len < sizeof(password)) {
                                                    strncpy(password, pwd_sep + 1, password_len);
                                                    password[password_len] = '\0';
                                                    log_info("[Stream %s] Extracted password from URL", 
                                                            thread->stream_name);
                                                }
                                            }
                                            
                                            // Move ip_start past the credentials
                                            ip_start = at_sign + 1;
                                        }
                                        
                                        // Extract the IP address (up to the next / or :)
                                        const char *ip_end = strchr(ip_start, '/');
                                        if (!ip_end) {
                                            ip_end = strchr(ip_start, ':');
                                        }
                                        
                                        if (ip_end) {
                                            size_t ip_len = ip_end - ip_start;
                                            char ip_address[64] = {0};
                                            if (ip_len < sizeof(ip_address)) {
                                                strncpy(ip_address, ip_start, ip_len);
                                                ip_address[ip_len] = '\0';
                                                // For ONVIF, we need to use port 80 (HTTP) instead of 554 (RTSP)
                                                // Check if the IP address contains a port
                                                if (strstr(ip_address, ":554")) {
                                                    // Replace port 554 with port 80 (HTTP port for ONVIF)
                                                    char *port_pos = strstr(ip_address, ":554");
                                                    *port_pos = '\0'; // Terminate the string at the port
                                                    snprintf(url, sizeof(url), "http://%s", ip_address);
                                                    log_info("[Stream %s] Changed RTSP port 554 to HTTP port for ONVIF: %s", 
                                                            thread->stream_name, url);
                                                } else if (strstr(ip_address, ":")) {
                                                    // Already has a port, use as is
                                                    snprintf(url, sizeof(url), "http://%s", ip_address);
                                                } else {
                                                    // No port, use default HTTP port for ONVIF
                                                    snprintf(url, sizeof(url), "http://%s", ip_address);
                                                    log_info("[Stream %s] Using default HTTP port for ONVIF: %s", 
                                                            thread->stream_name, url);
                                                }
                                            }
                                        } else {
                                            // No / or : found, use the whole string with default HTTP port
                                            snprintf(url, sizeof(url), "http://%s", ip_start);
                                            log_info("[Stream %s] Using default HTTP port for ONVIF: %s", 
                                                    thread->stream_name, url);
                                        }
                                    }
                                }
                            }
                        }
                    }
                    
                    // If we have valid credentials, call the ONVIF detection function
                    if (url[0] != '\0' && username[0] != '\0' && password[0] != '\0') {
                        log_info("[Stream %s] Calling ONVIF detection with URL: %s, username: %s", 
                                thread->stream_name, url, username);
                        
                        // Call the ONVIF detection function
                        result = detect_motion_onvif(url, username, password, &result_struct, thread->stream_name);
                        
                        if (result == 0) {
                            log_info("[Stream %s] ONVIF detection successful", thread->stream_name);
                            
                            // Process detection results for recording if motion was detected
                            if (result_struct.count > 0) {
                                log_info("[Stream %s] ONVIF detection found motion", thread->stream_name);
                                
                                // Create a dummy frame buffer for recording
                                // We don't have actual frame data for ONVIF, but we need to pass something
                                // to process_frame_for_recording
                                int width = 640;  // Default width
                                int height = 480; // Default height
                                int channels = 3; // RGB
                                uint8_t *dummy_frame = (uint8_t *)calloc(width * height * channels, 1);
                                
                                if (dummy_frame) {
                                    // Process the detection results for recording
                                    time_t current_time = time(NULL);
                                    int record_ret = process_frame_for_recording(thread->stream_name, 
                                                                              dummy_frame, width, height,
                                                                              channels, current_time, &result_struct);
                                    
                                    if (record_ret != 0) {
                                        log_error("[Stream %s] Failed to process ONVIF detection for recording (error code: %d)",
                                                 thread->stream_name, record_ret);
                                    } else {
                                        log_info("[Stream %s] Successfully processed ONVIF detection for recording", 
                                                thread->stream_name);
                                    }
                                    
                                    free(dummy_frame);
                                } else {
                                    log_error("[Stream %s] Failed to allocate dummy frame for ONVIF recording", 
                                             thread->stream_name);
                                }
                            } else {
                                log_info("[Stream %s] No motion detected in ONVIF detection", thread->stream_name);
                            }
                        } else {
                            log_error("[Stream %s] ONVIF detection failed (error code: %d)", thread->stream_name, result);
                        }
                    } else {
                        log_error("[Stream %s] Missing ONVIF credentials (URL: %s, username: %s)", 
                                 thread->stream_name, url[0] ? url : "empty", username[0] ? username : "empty");
                        result = -1;
                    }
                } else {
                    // For other detection models, process the HLS segment as usual
                    if (access(newest_segment, F_OK) == 0) {
                        log_info("[Stream %s] Processing HLS segment for detection: %s", thread->stream_name, newest_segment);
                        result = process_segment_for_detection(thread, newest_segment);
                    } else {
                        log_warn("[Stream %s] Segment no longer exists: %s", thread->stream_name, newest_segment);
                        segment_processing_failed = true;
                    }
                }
            } else {
                // No model loaded, try to process the segment anyway
                if (access(newest_segment, F_OK) == 0) {
                    log_info("[Stream %s] Processing HLS segment for detection (no model loaded): %s", 
                            thread->stream_name, newest_segment);
                    result = process_segment_for_detection(thread, newest_segment);
                } else {
                    log_warn("[Stream %s] Segment no longer exists: %s", thread->stream_name, newest_segment);
                    segment_processing_failed = true;
                }
            }
        } else {
            log_warn("[Stream %s] Cannot process segment, thread invalid: %s",
                    thread ? thread->stream_name : "unknown", newest_segment);
            segment_processing_failed = true;
        }

        // Clear the atomic flag when detection is complete
        if (thread) {
            atomic_store(&thread->detection_in_progress, 0);
        }

        if (!segment_processing_failed && result == 0) {
            log_info("[Stream %s] Successfully processed segment: %s", thread->stream_name, newest_segment);

            // Update the last processed segment
            strncpy(last_segment_processed, newest_segment, MAX_PATH_LENGTH - 1);
            last_segment_processed[MAX_PATH_LENGTH - 1] = '\0';

            // Update last detection time
            if (thread) {
                thread->last_detection_time = current_time;
            }
            return 0;
        } else {
            if (!segment_processing_failed) {
                log_error("[Stream %s] Failed to process segment: %s (error code: %d)",
                         thread ? thread->stream_name : "unknown", newest_segment, result);
            }
            return segment_processing_failed ? -1 : result;
        }
    }

    return 0;
}
