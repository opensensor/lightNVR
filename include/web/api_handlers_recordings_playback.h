#ifndef API_HANDLERS_RECORDINGS_PLAYBACK_H
#define API_HANDLERS_RECORDINGS_PLAYBACK_H

#include "web/request_response.h"

/**
 * @brief Backend-agnostic handler for GET /api/recordings/play/:id
 *
 * Serves a recording file for playback with range request support for seeking.
 * With ?prepare=1, returns JSON: 200 when ready or 202 with Retry-After while
 * a bounded background transcode is pending. Media requests made before the
 * cache is ready return 503 with Retry-After instead of blocking an HTTP worker.
 *
 * @param req HTTP request
 * @param res HTTP response
 */
void handle_recordings_playback(const http_request_t *req, http_response_t *res);

/**
 * @brief Backend-agnostic handler for GET /api/recordings/download/:id
 *
 * Serves a recording file for download with proper Content-Disposition header.
 *
 * @param req HTTP request
 * @param res HTTP response
 */
void handle_recordings_download(const http_request_t *req, http_response_t *res);

#endif // API_HANDLERS_RECORDINGS_PLAYBACK_H
