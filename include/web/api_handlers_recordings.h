#ifndef API_HANDLERS_RECORDINGS_H
#define API_HANDLERS_RECORDINGS_H

#include <sys/types.h>  /* for off_t */
#include <time.h>       /* for time_t */
#include "web/web_server.h"
#include "database/database_manager.h"  /* for recording_metadata_t */

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
