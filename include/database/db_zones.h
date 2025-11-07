#ifndef DB_ZONES_H
#define DB_ZONES_H

#include <stdbool.h>
#include "core/config.h"

#define MAX_ZONE_NAME 64
#define MAX_ZONE_ID 64
#define MAX_ZONE_POINTS 32
#define MAX_ZONES_PER_STREAM 16

/**
 * Point structure for zone polygon
 */
typedef struct {
    float x;  // Normalized coordinate (0.0 - 1.0)
    float y;  // Normalized coordinate (0.0 - 1.0)
} zone_point_t;

/**
 * Detection zone configuration
 */
typedef struct {
    char id[MAX_ZONE_ID];
    char stream_name[MAX_STREAM_NAME];
    char name[MAX_ZONE_NAME];
    bool enabled;
    char color[8];  // Hex color code (e.g., "#3b82f6")
    zone_point_t polygon[MAX_ZONE_POINTS];
    int polygon_count;
    char filter_classes[256];  // Comma-separated list of classes to detect in this zone
    float min_confidence;      // Minimum confidence for this zone (0.0 = use default)
} detection_zone_t;

/**
 * Initialize zones table
 * @return 0 on success, -1 on error
 */
int init_zones_table(void);

/**
 * Save detection zones for a stream
 * @param stream_name Stream name
 * @param zones Array of zones
 * @param count Number of zones
 * @return 0 on success, -1 on error
 */
int save_detection_zones(const char *stream_name, const detection_zone_t *zones, int count);

/**
 * Get detection zones for a stream
 * @param stream_name Stream name
 * @param zones Output array for zones
 * @param max_zones Maximum number of zones to retrieve
 * @return Number of zones retrieved, or -1 on error
 */
int get_detection_zones(const char *stream_name, detection_zone_t *zones, int max_zones);

/**
 * Delete all zones for a stream
 * @param stream_name Stream name
 * @return 0 on success, -1 on error
 */
int delete_detection_zones(const char *stream_name);

/**
 * Delete a specific zone
 * @param zone_id Zone ID
 * @return 0 on success, -1 on error
 */
int delete_detection_zone(const char *zone_id);

/**
 * Update a zone's enabled status
 * @param zone_id Zone ID
 * @param enabled New enabled status
 * @return 0 on success, -1 on error
 */
int update_zone_enabled(const char *zone_id, bool enabled);

#endif /* DB_ZONES_H */

