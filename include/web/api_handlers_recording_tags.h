#ifndef API_HANDLERS_RECORDING_TAGS_H
#define API_HANDLERS_RECORDING_TAGS_H

#include "web/request_response.h"

/**
 * @brief GET /api/recordings/tags
 * Returns all unique tags across all recordings.
 */
void handle_get_recording_tags(const http_request_t *req, http_response_t *res);

/**
 * @brief GET /api/recordings/detection-labels
 * Returns all unique detection labels across all recordings.
 */
void handle_get_recording_detection_labels(const http_request_t *req, http_response_t *res);

/**
 * @brief GET /api/recordings/:id/tags
 * Returns tags for a specific recording.
 */
void handle_get_recording_tags_by_id(const http_request_t *req, http_response_t *res);

/**
 * @brief PUT /api/recordings/:id/tags
 * Sets tags for a specific recording (replaces all existing).
 * Body: { "tags": ["tag1", "tag2"] }
 */
void handle_put_recording_tags(const http_request_t *req, http_response_t *res);

/**
 * @brief POST /api/recordings/batch-tags
 * Batch add/remove tags for multiple recordings.
 * Body: { "ids": [1,2,3], "add": ["tag1"], "remove": ["tag2"] }
 */
void handle_batch_recording_tags(const http_request_t *req, http_response_t *res);

#endif // API_HANDLERS_RECORDING_TAGS_H

