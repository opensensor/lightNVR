/**
 * @file api_handlers_motion.h
 * @brief API handlers for motion recording management
 */

#ifndef API_HANDLERS_MOTION_H
#define API_HANDLERS_MOTION_H

#include "mongoose.h"

/**
 * @brief Handler for GET /api/motion/config/:stream
 * Get motion recording configuration for a stream
 */
void mg_handle_get_motion_config(struct mg_connection *c, struct mg_http_message *hm);

/**
 * @brief Handler for POST /api/motion/config/:stream
 * Set motion recording configuration for a stream
 */
void mg_handle_post_motion_config(struct mg_connection *c, struct mg_http_message *hm);

/**
 * @brief Handler for DELETE /api/motion/config/:stream
 * Delete motion recording configuration for a stream
 */
void mg_handle_delete_motion_config(struct mg_connection *c, struct mg_http_message *hm);

/**
 * @brief Handler for GET /api/motion/stats/:stream
 * Get motion recording statistics for a stream
 */
void mg_handle_get_motion_stats(struct mg_connection *c, struct mg_http_message *hm);

/**
 * @brief Handler for GET /api/motion/recordings/:stream
 * Get list of motion recordings for a stream
 */
void mg_handle_get_motion_recordings(struct mg_connection *c, struct mg_http_message *hm);

/**
 * @brief Handler for DELETE /api/motion/recordings/:id
 * Delete a specific motion recording
 */
void mg_handle_delete_motion_recording(struct mg_connection *c, struct mg_http_message *hm);

/**
 * @brief Handler for POST /api/motion/cleanup
 * Trigger manual cleanup of old recordings
 */
void mg_handle_post_motion_cleanup(struct mg_connection *c, struct mg_http_message *hm);

/**
 * @brief Handler for GET /api/motion/storage
 * Get storage statistics for all motion recordings
 */
void mg_handle_get_motion_storage(struct mg_connection *c, struct mg_http_message *hm);

/**
 * @brief Handler for POST /api/motion/test/:stream
 * Simulate an ONVIF motion event for testing
 */
void mg_handle_test_motion_event(struct mg_connection *c, struct mg_http_message *hm);


#endif /* API_HANDLERS_MOTION_H */

