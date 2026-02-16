/**
 * @file api_handlers_recordings_thumbnail.h
 * @brief Backend-agnostic handler for recording thumbnail generation and serving
 */

#ifndef API_HANDLERS_RECORDINGS_THUMBNAIL_H
#define API_HANDLERS_RECORDINGS_THUMBNAIL_H

#include "web/request_response.h"

/**
 * @brief Backend-agnostic handler for GET /api/recordings/thumbnail/:id/:index
 * 
 * Serves a thumbnail image for a recording. If the thumbnail doesn't exist yet,
 * generates it on-the-fly using ffmpeg and caches it to disk.
 * 
 * Index 0 = start frame, 1 = middle frame, 2 = end frame.
 * Thumbnails are stored in {storage_path}/thumbnails/{id}_{index}.jpg
 * 
 * @param req HTTP request
 * @param res HTTP response
 */
void handle_recordings_thumbnail(const http_request_t *req, http_response_t *res);

/**
 * @brief Delete thumbnail files associated with a recording
 * 
 * Called when a recording is deleted to clean up associated thumbnails.
 * 
 * @param recording_id The recording ID
 */
void delete_recording_thumbnails(uint64_t recording_id);

#endif /* API_HANDLERS_RECORDINGS_THUMBNAIL_H */

