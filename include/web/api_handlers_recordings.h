#ifndef API_HANDLERS_RECORDINGS_H
#define API_HANDLERS_RECORDINGS_H

#include <sys/types.h>  /* for off_t */
#include <time.h>       /* for time_t */
#include "web/web_server.h"
#include "database/database_manager.h"  /* for recording_metadata_t */

/**
 * Get the total count of recordings matching given filters
 * This function performs a lightweight COUNT query against the database
 *
 * @param start_time    Start time filter (0 for no filter)
 * @param end_time      End time filter (0 for no filter)
 * @param stream_name   Stream name filter (NULL for all streams)
 * @param detection_only Filter to only include recordings with detection events
 *
 * @return Total number of matching recordings, or -1 on error
 */
int get_recording_count(time_t start_time, time_t end_time, const char *stream_name, int detection_only);

/**
 * Get paginated recording metadata from the database with sorting
 * This function fetches only the specified page of results with the given sort order
 * 
 * @param start_time    Start time filter (0 for no filter)
 * @param end_time      End time filter (0 for no filter)
 * @param stream_name   Stream name filter (NULL for all streams)
 * @param has_detection Filter to only include recordings with detection events
 * @param sort_field    Field to sort by
 * @param sort_order    Sort order (asc or desc)
 * @param metadata      Array to store the results
 * @param limit         Limit for pagination
 * @param offset        Offset for pagination
 * 
 * @return Number of recordings fetched, or -1 on error
 */
int get_recording_metadata_paginated(time_t start_time, time_t end_time, 
                                   const char *stream_name, int has_detection,
                                   const char *sort_field, const char *sort_order,
                                   recording_metadata_t *metadata, 
                                   int limit, int offset);

/**
 * Handle GET request for recordings
 */
void handle_get_recordings(const http_request_t *request, http_response_t *response);

/**
 * Handle GET request for a specific recording
 */
void handle_get_recording(const http_request_t *request, http_response_t *response);

/**
 * Handle DELETE request to remove a recording
 */
void handle_delete_recording(const http_request_t *request, http_response_t *response);

/**
 * Handle GET request to download a recording
 */
void handle_download_recording(const http_request_t *request, http_response_t *response);

/**
 * Serve an MP4 file with proper headers for download
 */
void serve_mp4_file(http_response_t *response, const char *file_path, const char *filename);

/**
 * Serve a file for download with proper headers to force browser download
 */
void serve_file_for_download(http_response_t *response, const char *file_path, const char *filename, off_t file_size);

/**
 * Serve the direct file download
 */
void serve_direct_download(http_response_t *response, uint64_t id, recording_metadata_t *metadata);

/**
 * Serve a file for download with proper headers
 */
void serve_download_file(http_response_t *response, const char *file_path, const char *content_type,
                       const char *stream_name, time_t timestamp);

/**
 * Schedule a file for deletion after it has been served
 */
void schedule_file_deletion(const char *file_path);

/**
 * Callback to remove temporary files after they've been sent
 */
void remove_temp_file_callback(void *data);

#endif /* API_HANDLERS_RECORDINGS_H */
