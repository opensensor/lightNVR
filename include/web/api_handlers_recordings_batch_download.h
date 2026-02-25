/**
 * @file api_handlers_recordings_batch_download.h
 * @brief Batch download (ZIP) handler declarations
 */

#ifndef API_HANDLERS_RECORDINGS_BATCH_DOWNLOAD_H
#define API_HANDLERS_RECORDINGS_BATCH_DOWNLOAD_H

#include "web/request_response.h"

/**
 * @brief POST /api/recordings/batch-download
 *
 * Accepts { "ids": [...], "filename": "archive.zip" }.
 * Spawns a background thread that creates a ZIP archive, returns a polling token.
 */
void handle_batch_download_recordings(const http_request_t *req, http_response_t *res);

/**
 * @brief GET /api/recordings/batch-download/status/{token}
 *
 * Returns JSON with status, current progress, and total count.
 */
void handle_batch_download_status(const http_request_t *req, http_response_t *res);

/**
 * @brief GET /api/recordings/batch-download/result/{token}
 *
 * Serves the completed ZIP file for download. Returns 409 if not yet ready.
 */
void handle_batch_download_result(const http_request_t *req, http_response_t *res);

#endif /* API_HANDLERS_RECORDINGS_BATCH_DOWNLOAD_H */

