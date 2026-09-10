#ifndef API_HANDLERS_RECORDINGS_PLAYBACK_H
#define API_HANDLERS_RECORDINGS_PLAYBACK_H

#include "web/request_response.h"

/**
 * @brief Backend-agnostic handler for GET /api/recordings/play/:id
 *
 * Serves the original recording with range support, without probing or encoding.
 * For browsers that reject it, ?prepare=1 requests a compatibility copy and
 * returns JSON: 200 when ready or 202 with Retry-After while pending. Load the
 * prepared media using ?transcode=1 (503 with Retry-After if not yet ready).
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
