/**
 * @file api_handlers_recordings_download.h
 * @brief Backend-agnostic handler for recording downloads
 */

#ifndef API_HANDLERS_RECORDINGS_DOWNLOAD_H
#define API_HANDLERS_RECORDINGS_DOWNLOAD_H

#include "web/request_response.h"

/**
 * @brief Backend-agnostic handler for GET /api/recordings/download/:id
 * 
 * Serves a recording file for download with proper Content-Disposition header.
 * 
 * @param req HTTP request
 * @param res HTTP response
 */
void handle_recordings_download(const http_request_t *req, http_response_t *res);

#endif /* API_HANDLERS_RECORDINGS_DOWNLOAD_H */

