#ifndef API_HANDLERS_DETECTION_RESULTS_H
#define API_HANDLERS_DETECTION_RESULTS_H

#include "web/request_response.h"
#include "video/detection_result.h"

/**
 * Initialize detection results storage
 */
void init_detection_results(void);

/**
 * Store detection result for a stream
 */
void store_detection_result(const char *stream_name, const detection_result_t *result);

/**
 * Handle GET request for detection results
 */
void handle_get_detection_results(const http_request_t *request, http_response_t *response);

/**
 * Register detection results API handlers
 */
void register_detection_results_api_handlers(void);

/**
 * Debug function to dump current detection results
 */
void debug_dump_detection_results(void);

#endif /* API_HANDLERS_DETECTION_RESULTS_H */
