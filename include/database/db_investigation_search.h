#ifndef LIGHTNVR_DB_INVESTIGATION_SEARCH_H
#define LIGHTNVR_DB_INVESTIGATION_SEARCH_H

#include <stdbool.h>
#include <stdint.h>
#include <time.h>

#include "core/config.h"
#include "database/db_recording_tags.h"
#include "video/detection_result.h"

#define INVESTIGATION_SEARCH_MAX_CAMERAS 64
#define INVESTIGATION_SEARCH_MAX_FILTER_VALUES 16
#define INVESTIGATION_SEARCH_MAX_RESULTS 500
#define INVESTIGATION_SEARCH_MAX_FACETS 64
#define INVESTIGATION_SEARCH_MAX_HISTOGRAM_BUCKETS 48
#define INVESTIGATION_SEARCH_VALUE_MAX 128

typedef enum {
    INVESTIGATION_REGION_NONE = 0,
    INVESTIGATION_REGION_CENTER,
    INVESTIGATION_REGION_INTERSECTS,
    INVESTIGATION_REGION_MIN_INTERSECTION
} investigation_region_match_t;

typedef struct {
    char camera_uuids[INVESTIGATION_SEARCH_MAX_CAMERAS]
                     [CAMERA_UUID_STRING_SIZE];
    char legacy_stream_names[INVESTIGATION_SEARCH_MAX_CAMERAS]
                            [MAX_STREAM_NAME];
    int camera_count;
    time_t start_time;
    time_t end_time;
    char labels[INVESTIGATION_SEARCH_MAX_FILTER_VALUES][MAX_LABEL_LENGTH];
    int label_count;
    char zones[INVESTIGATION_SEARCH_MAX_FILTER_VALUES]
              [INVESTIGATION_SEARCH_VALUE_MAX];
    int zone_count;
    char sources[INVESTIGATION_SEARCH_MAX_FILTER_VALUES]
                [INVESTIGATION_SEARCH_VALUE_MAX];
    int source_count;
    char event_types[INVESTIGATION_SEARCH_MAX_FILTER_VALUES]
                    [INVESTIGATION_SEARCH_VALUE_MAX];
    int event_type_count;
    char capture_methods[INVESTIGATION_SEARCH_MAX_FILTER_VALUES]
                        [INVESTIGATION_SEARCH_VALUE_MAX];
    int capture_method_count;
    char recording_tags[INVESTIGATION_SEARCH_MAX_FILTER_VALUES]
                       [MAX_TAG_LENGTH];
    int recording_tag_count;
    int protected_filter; /* -1 = all, 0 = unprotected, 1 = protected */
    bool has_region;
    char region_camera_uuid[CAMERA_UUID_STRING_SIZE];
    double region_x;
    double region_y;
    double region_width;
    double region_height;
    investigation_region_match_t region_match;
    double region_min_intersection;
    bool has_min_confidence;
    bool has_max_confidence;
    double min_confidence;
    double max_confidence;
    bool has_cursor;
    time_t cursor_timestamp;
    uint64_t cursor_id;
    int limit;
    bool include_results;
    bool include_summary;
} investigation_search_query_t;

typedef struct {
    uint64_t detection_id;
    char camera_uuid[CAMERA_UUID_STRING_SIZE];
    char legacy_stream_name[MAX_STREAM_NAME];
    time_t start_time;
    time_t end_time;
    char label[MAX_LABEL_LENGTH];
    double confidence;
    bool has_box;
    double x;
    double y;
    double width;
    double height;
    char zone_uuid[INVESTIGATION_SEARCH_VALUE_MAX];
    int track_id;
    char source[INVESTIGATION_SEARCH_VALUE_MAX];
    uint64_t recording_id;
    bool media_available;
    char capture_method[INVESTIGATION_SEARCH_VALUE_MAX];
    bool recording_protected;
} investigation_search_result_t;

typedef struct {
    char value[INVESTIGATION_SEARCH_VALUE_MAX];
    int64_t count;
} investigation_search_facet_t;

typedef struct {
    investigation_search_facet_t cameras[INVESTIGATION_SEARCH_MAX_FACETS];
    int camera_count;
    investigation_search_facet_t labels[INVESTIGATION_SEARCH_MAX_FACETS];
    int label_count;
    investigation_search_facet_t zones[INVESTIGATION_SEARCH_MAX_FACETS];
    int zone_count;
    investigation_search_facet_t sources[INVESTIGATION_SEARCH_MAX_FACETS];
    int source_count;
    investigation_search_facet_t
        event_types[INVESTIGATION_SEARCH_MAX_FACETS];
    int event_type_count;
    investigation_search_facet_t
        capture_methods[INVESTIGATION_SEARCH_MAX_FACETS];
    int capture_method_count;
    investigation_search_facet_t
        recording_tags[INVESTIGATION_SEARCH_MAX_FACETS];
    int recording_tag_count;
    investigation_search_facet_t
        protection[INVESTIGATION_SEARCH_MAX_FACETS];
    int protection_count;
} investigation_search_facets_t;

typedef struct {
    time_t start_time;
    time_t end_time;
    time_t event_time;
    int64_t count;
} investigation_search_histogram_bucket_t;

typedef struct {
    int result_count;
    bool has_more;
    int64_t total_count;
    int64_t unresolved_legacy_count;
    int64_t spatial_metadata_rows;
    int64_t spatial_missing_rows;
    int histogram_bucket_seconds;
    int histogram_count;
    investigation_search_histogram_bucket_t
        histogram[INVESTIGATION_SEARCH_MAX_HISTOGRAM_BUCKETS];
    investigation_search_facets_t facets;
} investigation_search_summary_t;

int db_investigation_search(
    const investigation_search_query_t *query,
    investigation_search_result_t *results,
    investigation_search_summary_t *summary);

#endif /* LIGHTNVR_DB_INVESTIGATION_SEARCH_H */
