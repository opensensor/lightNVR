#ifndef LIGHTNVR_ZONE_FILTER_H
#define LIGHTNVR_ZONE_FILTER_H

#include "video/detection.h"

/**
 * Filter detections based on configured zones for a stream
 *
 * This function:
 * 1. Loads zones for the stream from the database
 * 2. Filters detections to only include those within enabled zones
 * 3. Applies per-zone class filters and confidence thresholds
 * 4. Sets the zone_id field for each accepted detection
 *
 * If no zones are configured or no zones are enabled, all detections are allowed.
 *
 * @param stream_name The name of the stream
 * @param result Pointer to detection_result_t to filter (modified in place)
 * @return 0 on success, -1 on error
 */
int filter_detections_by_zones(const char *stream_name, detection_result_t *result);

/**
 * Filter detections based on per-stream object include/exclude lists
 *
 * This function checks the stream's detection_object_filter and
 * detection_object_filter_list settings to filter detections:
 * - "none": No filtering, all detections pass through
 * - "include": Only keep detections whose labels are in the list
 * - "exclude": Remove detections whose labels are in the list
 *
 * @param stream_name The name of the stream
 * @param result Pointer to detection_result_t to filter (modified in place)
 * @return 0 on success, -1 on error
 */
int filter_detections_by_stream_objects(const char *stream_name, detection_result_t *result);

#endif /* LIGHTNVR_ZONE_FILTER_H */

