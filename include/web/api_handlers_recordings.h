#ifndef API_HANDLERS_RECORDINGS_H
#define API_HANDLERS_RECORDINGS_H

#include <sys/types.h>  /* for off_t */
#include <time.h>       /* for time_t */
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
 * Schedule a file for deletion after it has been served
 */
void schedule_file_deletion(const char *file_path);

/**
 * Callback to remove temporary files after they've been sent
 */
void remove_temp_file_callback(void *data);

/* Recording Handlers */
#include "web/request_response.h"

/**
 * @brief Backend-agnostic handler for GET /api/recordings/:id
 *
 * Gets detailed information about a single recording.
 *
 * @param req HTTP request
 * @param res HTTP response
 */
void handle_get_recording(const http_request_t *req, http_response_t *res);

/**
 * @brief Backend-agnostic handler for DELETE /api/recordings/:id
 *
 * Deletes a single recording from the database and filesystem.
 *
 * @param req HTTP request
 * @param res HTTP response
 */
void handle_delete_recording(const http_request_t *req, http_response_t *res);

/**
 * @brief Backend-agnostic handler for POST /api/recordings/batch-delete
 *
 * Initiates a batch delete operation and returns a job ID for progress tracking.
 *
 * @param req HTTP request
 * @param res HTTP response
 */
void handle_batch_delete_recordings(const http_request_t *req, http_response_t *res);

/**
 * @brief Backend-agnostic handler for GET /api/recordings/batch-delete/progress/:job_id
 *
 * Returns the progress status of a batch delete operation.
 *
 * @param req HTTP request
 * @param res HTTP response
 */
void handle_batch_delete_progress(const http_request_t *req, http_response_t *res);

/**
 * @brief Backend-agnostic handler for GET /api/recordings/files/check
 *
 * Checks if a recording file exists and returns its metadata.
 *
 * @param req HTTP request
 * @param res HTTP response
 */
void handle_check_recording_file(const http_request_t *req, http_response_t *res);

/**
 * @brief Backend-agnostic handler for DELETE /api/recordings/files
 *
 * Deletes a recording file from the filesystem.
 *
 * @param req HTTP request
 * @param res HTTP response
 */
void handle_delete_recording_file(const http_request_t *req, http_response_t *res);

#endif /* API_HANDLERS_RECORDINGS_H */
