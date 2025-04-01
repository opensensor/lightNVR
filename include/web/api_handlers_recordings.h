#ifndef API_HANDLERS_RECORDINGS_H
#define API_HANDLERS_RECORDINGS_H

#include <sys/types.h>  /* for off_t */
#include <time.h>       /* for time_t */
#include "database/database_manager.h"  /* for recording_metadata_t */
#include "mongoose.h"  /* for mongoose-specific handlers */

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
 * Serve an MP4 file with proper headers for download
 */
void serve_mp4_file(struct mg_connection *c, const char *file_path, const char *filename);

/**
 * Serve a file for download with proper headers to force browser download
 */
void serve_file_for_download(struct mg_connection *c, const char *file_path, const char *filename, off_t file_size);

/**
 * Serve the direct file download
 */
void serve_direct_download(struct mg_connection *c, uint64_t id, recording_metadata_t *metadata);

/**
 * Serve a file for download with proper headers
 */
void serve_download_file(struct mg_connection *c, const char *file_path, const char *content_type,
                       const char *stream_name, time_t timestamp);

/**
 * Schedule a file for deletion after it has been served
 */
void schedule_file_deletion(const char *file_path);

/**
 * Callback to remove temporary files after they've been sent
 */
void remove_temp_file_callback(void *data);

/* Mongoose-specific handlers */

/**
 * Worker function for GET request for recordings list
 * 
 * This function is called by the multithreading system to handle recordings requests.
 */
void mg_handle_get_recordings_worker(struct mg_connection *c, struct mg_http_message *hm);

/**
 * Handle GET request for recordings list
 * 
 * This handler uses the multithreading pattern to handle the request in a worker thread.
 */
void mg_handle_get_recordings(struct mg_connection *c, struct mg_http_message *hm);

/**
 * Worker function for GET request for a specific recording
 * 
 * This function is called by the multithreading system to handle recording detail requests.
 */
void mg_handle_get_recording_worker(struct mg_connection *c, struct mg_http_message *hm);

/**
 * Handle GET request for a specific recording
 * 
 * This handler uses the multithreading pattern to handle the request in a worker thread.
 */
void mg_handle_get_recording(struct mg_connection *c, struct mg_http_message *hm);

/**
 * Handle DELETE request to remove a recording
 */
void mg_handle_delete_recording(struct mg_connection *c, struct mg_http_message *hm);

/**
 * Handle POST request to batch delete recordings
 */
void mg_handle_batch_delete_recordings(struct mg_connection *c, struct mg_http_message *hm);

/**
 * Handle GET request to play a recording
 */
void mg_handle_play_recording(struct mg_connection *c, struct mg_http_message *hm);

/**
 * Handle GET request to download a recording
 */
void mg_handle_download_recording(struct mg_connection *c, struct mg_http_message *hm);

/**
 * Handle GET request to check if a recording file exists
 */
void mg_handle_check_recording_file(struct mg_connection *c, struct mg_http_message *hm);

/**
 * Handle DELETE request to delete a recording file
 */
void mg_handle_delete_recording_file(struct mg_connection *c, struct mg_http_message *hm);

#endif /* API_HANDLERS_RECORDINGS_H */
