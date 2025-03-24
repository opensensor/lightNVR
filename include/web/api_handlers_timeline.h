#ifndef API_HANDLERS_TIMELINE_H
#define API_HANDLERS_TIMELINE_H

#include "web/web_server.h"
#include "database/database_manager.h"

/**
 * Structure to hold timeline segment information
 */
typedef struct {
    uint64_t id;
    char stream_name[64];
    char file_path[256];
    time_t start_time;
    time_t end_time;
    uint64_t size_bytes;
    bool has_detection;
} timeline_segment_t;

/**
 * Get timeline segments for a specific stream and time range
 * 
 * @param stream_name   Stream name to filter by
 * @param start_time    Start time of the range
 * @param end_time      End time of the range
 * @param segments      Array to store the segments
 * @param max_segments  Maximum number of segments to return
 * 
 * @return Number of segments found, or -1 on error
 */
int get_timeline_segments(const char *stream_name, time_t start_time, time_t end_time,
                         timeline_segment_t *segments, int max_segments);

/**
 * Handle GET request for timeline segments
 * Endpoint: /api/timeline/segments
 */
void handle_get_timeline_segments(const http_request_t *request, http_response_t *response);

/**
 * Create a playback manifest for a sequence of recordings
 * 
 * @param segments      Array of segments to include in the manifest
 * @param segment_count Number of segments in the array
 * @param start_time    Requested playback start time
 * @param manifest_path Output path where the manifest will be written
 * 
 * @return 0 on success, non-zero on failure
 */
int create_timeline_manifest(const timeline_segment_t *segments, int segment_count,
                            time_t start_time, char *manifest_path);

/**
 * Handle GET request for timeline playback
 * Endpoint: /api/timeline/play
 */
void handle_timeline_playback(const http_request_t *request, http_response_t *response);

/**
 * Handle GET request for timeline manifest
 * Endpoint: /api/timeline/manifest
 */
void handle_timeline_manifest(const http_request_t *request, http_response_t *response);

#endif /* API_HANDLERS_TIMELINE_H */
