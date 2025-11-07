#include "video/zone_filter.h"
#include "database/db_zones.h"
#include "core/logger.h"
#include <string.h>
#include <math.h>

/**
 * Check if a point is inside a polygon using ray casting algorithm
 */
static bool point_in_polygon(float x, float y, const zone_point_t *polygon, int polygon_count) {
    if (!polygon || polygon_count < 3) {
        return false;
    }

    bool inside = false;
    for (int i = 0, j = polygon_count - 1; i < polygon_count; j = i++) {
        float xi = polygon[i].x;
        float yi = polygon[i].y;
        float xj = polygon[j].x;
        float yj = polygon[j].y;

        bool intersect = ((yi > y) != (yj > y)) &&
                        (x < (xj - xi) * (y - yi) / (yj - yi) + xi);
        if (intersect) {
            inside = !inside;
        }
    }
    return inside;
}

/**
 * Check if a detection's center point is within a zone
 */
static bool detection_in_zone(const detection_t *detection, const detection_zone_t *zone) {
    if (!detection || !zone || !zone->enabled) {
        return false;
    }

    // Calculate center point of detection bounding box
    float center_x = detection->x + (detection->width / 2.0f);
    float center_y = detection->y + (detection->height / 2.0f);

    // Check if center is in polygon
    return point_in_polygon(center_x, center_y, zone->polygon, zone->polygon_count);
}

/**
 * Check if a detection's class matches the zone's filter
 */
static bool detection_class_matches(const detection_t *detection, const detection_zone_t *zone) {
    // If no filter is set, all classes match
    if (!zone->filter_classes || zone->filter_classes[0] == '\0') {
        return true;
    }

    // Check if detection label is in the comma-separated filter list
    char filter_copy[256];
    strncpy(filter_copy, zone->filter_classes, sizeof(filter_copy) - 1);
    filter_copy[sizeof(filter_copy) - 1] = '\0';

    char *token = strtok(filter_copy, ",");
    while (token) {
        // Trim whitespace
        while (*token == ' ') token++;
        char *end = token + strlen(token) - 1;
        while (end > token && *end == ' ') {
            *end = '\0';
            end--;
        }

        // Check if it matches
        if (strcmp(token, detection->label) == 0) {
            return true;
        }

        token = strtok(NULL, ",");
    }

    return false;
}

/**
 * Check if a detection meets the zone's confidence threshold
 */
static bool detection_meets_confidence(const detection_t *detection, const detection_zone_t *zone) {
    // If no minimum confidence is set (0.0), accept all
    if (zone->min_confidence <= 0.0f) {
        return true;
    }

    return detection->confidence >= zone->min_confidence;
}

/**
 * Filter detections based on zones for a stream
 */
int filter_detections_by_zones(const char *stream_name, detection_result_t *result) {
    if (!stream_name || !result) {
        log_error("Invalid parameters for filter_detections_by_zones");
        return -1;
    }

    // If no detections, nothing to filter
    if (result->count == 0) {
        return 0;
    }

    // Get zones for this stream
    detection_zone_t zones[MAX_ZONES_PER_STREAM];
    int zone_count = get_detection_zones(stream_name, zones, MAX_ZONES_PER_STREAM);

    if (zone_count < 0) {
        log_error("Failed to get detection zones for stream %s", stream_name);
        return -1;
    }

    // If no zones are configured, don't filter (allow all detections)
    if (zone_count == 0) {
        log_debug("No zones configured for stream %s, allowing all detections", stream_name);
        return 0;
    }

    // Count enabled zones
    int enabled_zone_count = 0;
    for (int i = 0; i < zone_count; i++) {
        if (zones[i].enabled) {
            enabled_zone_count++;
        }
    }

    // If no enabled zones, don't filter (allow all detections)
    if (enabled_zone_count == 0) {
        log_debug("No enabled zones for stream %s, allowing all detections", stream_name);
        return 0;
    }

    log_info("Filtering %d detections using %d enabled zones for stream %s",
             result->count, enabled_zone_count, stream_name);

    // Create a filtered result
    detection_result_t filtered;
    memset(&filtered, 0, sizeof(detection_result_t));

    // Check each detection against all zones
    for (int i = 0; i < result->count; i++) {
        detection_t *det = &result->detections[i];
        bool detection_accepted = false;
        const char *matched_zone_id = NULL;

        // Check if detection is in any enabled zone
        for (int j = 0; j < zone_count; j++) {
            detection_zone_t *zone = &zones[j];

            if (!zone->enabled) {
                continue;
            }

            // Check if detection is in this zone
            if (!detection_in_zone(det, zone)) {
                continue;
            }

            // Check if detection class matches zone filter
            if (!detection_class_matches(det, zone)) {
                log_debug("Detection %s rejected by zone %s (class filter)",
                         det->label, zone->name);
                continue;
            }

            // Check if detection meets zone confidence threshold
            if (!detection_meets_confidence(det, zone)) {
                log_debug("Detection %s rejected by zone %s (confidence %.2f < %.2f)",
                         det->label, zone->name, det->confidence, zone->min_confidence);
                continue;
            }

            // Detection passed all checks for this zone
            detection_accepted = true;
            matched_zone_id = zone->id;
            log_info("Detection %s (%.2f%%) accepted by zone %s",
                    det->label, det->confidence * 100.0f, zone->name);
            break;
        }

        // If detection was accepted by at least one zone, add it to filtered result
        if (detection_accepted) {
            memcpy(&filtered.detections[filtered.count], det, sizeof(detection_t));
            
            // Set the zone_id for this detection
            if (matched_zone_id) {
                strncpy(filtered.detections[filtered.count].zone_id, matched_zone_id,
                       sizeof(filtered.detections[filtered.count].zone_id) - 1);
                filtered.detections[filtered.count].zone_id[sizeof(filtered.detections[filtered.count].zone_id) - 1] = '\0';
            }
            
            filtered.count++;
        } else {
            log_debug("Detection %s (%.2f%%) at [%.2f, %.2f] rejected (not in any enabled zone)",
                     det->label, det->confidence * 100.0f,
                     det->x + det->width / 2.0f, det->y + det->height / 2.0f);
        }
    }

    log_info("Zone filtering: %d detections -> %d detections (filtered out %d)",
             result->count, filtered.count, result->count - filtered.count);

    // Replace original result with filtered result
    memcpy(result, &filtered, sizeof(detection_result_t));

    return 0;
}

