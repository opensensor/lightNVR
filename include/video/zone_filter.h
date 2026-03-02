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

/**
 * Build a zone mask for a motion detection grid.
 *
 * For each grid cell, checks whether the cell center (in normalized 0-1
 * coordinates) falls inside any enabled zone polygon for the given stream.
 *
 * If no zones are configured or no zones are enabled, all mask entries are
 * set to true (i.e. the entire frame is considered active).
 *
 * @param stream_name Stream name to load zones for
 * @param grid_size   Size of the grid (grid_size x grid_size cells)
 * @param zone_mask   Output array of size grid_size*grid_size.  true = cell
 *                    is inside at least one enabled zone.
 * @return Number of enabled zones found (0 means no filtering), -1 on error
 */
int build_motion_zone_mask(const char *stream_name, int grid_size, bool *zone_mask);

#endif /* LIGHTNVR_ZONE_FILTER_H */

