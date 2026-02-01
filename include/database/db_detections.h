#ifndef LIGHTNVR_DB_DETECTIONS_H
#define LIGHTNVR_DB_DETECTIONS_H

#include <stdint.h>
#include <time.h>
#include "video/detection_result.h"

/**
 * Store detection results in the database
 * 
 * @param stream_name Stream name
 * @param result Detection results
 * @param timestamp Timestamp of the detection (0 for current time)
 * @return 0 on success, non-zero on failure
 */
int store_detections_in_db(const char *stream_name, const detection_result_t *result, time_t timestamp);

/**
 * Get detection results from the database with time range filtering
 * 
 * @param stream_name Stream name
 * @param result Detection results to fill
 * @param max_age Maximum age in seconds (0 for all)
 * @param start_time Start time filter (0 for no filter)
 * @param end_time End time filter (0 for no filter)
 * @return Number of detections found, or -1 on error
 */
int get_detections_from_db_time_range(const char *stream_name, detection_result_t *result, 
                                     uint64_t max_age, time_t start_time, time_t end_time);

/**
 * Get timestamps for detections
 * 
 * @param stream_name Stream name
 * @param result Detection results to match
 * @param timestamps Array to store timestamps (must be same size as result->count)
 * @param max_age Maximum age in seconds (0 for all)
 * @param start_time Start time filter (0 for no filter)
 * @param end_time End time filter (0 for no filter)
 * @return 0 on success, non-zero on failure
 */
int get_detection_timestamps(const char *stream_name, detection_result_t *result, time_t *timestamps,
                           uint64_t max_age, time_t start_time, time_t end_time);

/**
 * Get detection results from the database
 * 
 * @param stream_name Stream name
 * @param result Detection results to fill
 * @param max_age Maximum age in seconds (0 for all)
 * @return Number of detections found, or -1 on error
 */
int get_detections_from_db(const char *stream_name, detection_result_t *result, uint64_t max_age);

/**
 * Check if there are any detections for a stream within a time range
 *
 * @param stream_name Stream name
 * @param start_time Start time (inclusive)
 * @param end_time End time (inclusive)
 * @return 1 if detections exist, 0 if none, -1 on error
 */
int has_detections_in_time_range(const char *stream_name, time_t start_time, time_t end_time);

/**
 * Delete old detections from the database
 *
 * @param max_age Maximum age in seconds
 * @return Number of detections deleted, or -1 on error
 */
int delete_old_detections(uint64_t max_age);

/**
 * Maximum number of unique labels to return in a summary
 */
#define MAX_DETECTION_LABELS 10

/**
 * Structure to hold a detection label summary entry
 */
typedef struct {
    char label[MAX_LABEL_LENGTH];  // Detection label (e.g., "person", "car")
    int count;                      // Number of times this label was detected
} detection_label_summary_t;

/**
 * Get a summary of detection labels for a stream within a time range
 * Returns unique labels with their counts, sorted by count descending
 *
 * @param stream_name Stream name
 * @param start_time Start time (inclusive)
 * @param end_time End time (inclusive)
 * @param labels Array to store label summaries (must have space for MAX_DETECTION_LABELS entries)
 * @param max_labels Maximum number of labels to return
 * @return Number of unique labels found, or -1 on error
 */
int get_detection_labels_summary(const char *stream_name, time_t start_time, time_t end_time,
                                 detection_label_summary_t *labels, int max_labels);

#endif // LIGHTNVR_DB_DETECTIONS_H
