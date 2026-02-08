/**
 * @file api_handlers_motion.h
 * @brief API handlers for motion recording management
 */

#ifndef API_HANDLERS_MOTION_H
#define API_HANDLERS_MOTION_H

#include "web/request_response.h"

/**
 * @brief Handler for GET /api/motion/config/:stream
 * Get motion recording configuration for a stream
 */
void handle_get_motion_config(const http_request_t *req, http_response_t *res);

/**
 * @brief Handler for POST /api/motion/config/:stream
 * Set motion recording configuration for a stream
 */
void handle_post_motion_config(const http_request_t *req, http_response_t *res);

/**
 * @brief Handler for DELETE /api/motion/config/:stream
 * Delete motion recording configuration for a stream
 */
void handle_delete_motion_config(const http_request_t *req, http_response_t *res);

/**
 * @brief Handler for GET /api/motion/stats/:stream
 * Get motion recording statistics for a stream
 */
void handle_get_motion_stats(const http_request_t *req, http_response_t *res);

/**
 * @brief Handler for GET /api/motion/recordings/:stream
 * Get list of motion recordings for a stream
 */
void handle_get_motion_recordings(const http_request_t *req, http_response_t *res);

/**
 * @brief Handler for DELETE /api/motion/recordings/:id
 * Delete a specific motion recording
 */
void handle_delete_motion_recording(const http_request_t *req, http_response_t *res);

/**
 * @brief Handler for POST /api/motion/cleanup
 * Trigger manual cleanup of old recordings
 */
void handle_post_motion_cleanup(const http_request_t *req, http_response_t *res);

/**
 * @brief Handler for GET /api/motion/storage
 * Get storage statistics for all motion recordings
 */
void handle_get_motion_storage(const http_request_t *req, http_response_t *res);

/**
 * @brief Handler for POST /api/motion/test/:stream
 * Simulate an ONVIF motion event for testing
 */
void handle_test_motion_event(const http_request_t *req, http_response_t *res);


#endif /* API_HANDLERS_MOTION_H */

