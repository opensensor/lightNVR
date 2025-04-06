#ifndef DETECTION_STREAM_THREAD_HELPERS_H
#define DETECTION_STREAM_THREAD_HELPERS_H

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <sys/stat.h>
#include <dirent.h>
#include <stdbool.h>
#include "video/detection_stream_thread.h"

/**
 * Check if detection should run based on startup delay, detection in progress, and time interval
 * Returns true if detection should run, false otherwise
 */
bool should_run_detection_check(stream_detection_thread_t *thread, time_t current_time);

/**
 * Check and manage HLS writer status
 * Returns true if HLS writer is recording, false otherwise
 */
bool check_hls_writer_status(stream_detection_thread_t *thread, time_t current_time, 
                            time_t *last_warning_time, bool first_check, int *consecutive_failures);

/**
 * Find and set the HLS directory path for a stream
 * Updates thread->hls_dir with the correct path
 * Returns true if a valid directory was found, false otherwise
 */
bool find_hls_directory(stream_detection_thread_t *thread, time_t current_time, 
                        time_t *last_warning_time, int *consecutive_failures, bool first_check);

/**
 * Find the newest segment file in the HLS directory
 * Returns true if a segment was found, false otherwise
 * Updates newest_segment with the path to the newest segment if found
 */
bool find_newest_segment(stream_detection_thread_t *thread, char *newest_segment, 
                         time_t *newest_time, int *segment_count);

/**
 * Check if a segment should be processed
 * Returns true if the segment should be processed, false otherwise
 */
bool should_process_segment(const char *newest_segment, const char *last_processed_segment, 
                          bool should_run_detection);

/**
 * Attempt to restart the HLS stream if no segments were found
 */
void restart_hls_stream_if_needed(stream_detection_thread_t *thread, time_t current_time, 
                                 time_t *last_warning_time, bool first_check);

/**
 * Process a segment if it should be processed
 * Returns 0 on success, non-zero on failure
 */
int process_segment_if_needed(stream_detection_thread_t *thread, 
                             const char *newest_segment, 
                             const char *last_processed_segment,
                             bool should_run_detection,
                             time_t newest_time,
                             time_t current_time);

#endif /* DETECTION_STREAM_THREAD_HELPERS_H */
