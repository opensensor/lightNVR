#ifndef API_HANDLERS_DETECTION_H
#define API_HANDLERS_DETECTION_H

#include <dirent.h>
#include "web/request_response.h"
#include "web/api_handlers_common.h"

/**
 * Initialize detection settings
 */
void init_detection_settings(void);

/**
 * Handle GET request for detection settings
 */
void handle_get_detection_settings(const http_request_t *request, http_response_t *response);

/**
 * Handle POST request to update detection settings
 */
void handle_post_detection_settings(const http_request_t *request, http_response_t *response);

/**
 * Handle GET request to list available detection models
 */
void handle_get_detection_models(const http_request_t *request, http_response_t *response);

/**
 * Handle POST request to test a detection model
 */
void handle_post_test_detection_model(const http_request_t *request, http_response_t *response);

/**
 * Register detection API handlers
 */
void register_detection_api_handlers(void);

#endif /* API_HANDLERS_DETECTION_H */
