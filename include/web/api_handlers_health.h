/**
 * @file api_handlers_health.h
 * @brief Health check API handlers for the web server
 */

#ifndef API_HANDLERS_HEALTH_H
#define API_HANDLERS_HEALTH_H

#include <stdbool.h>
#include "mongoose.h"

/**
 * @brief Initialize health check system
 */
void init_health_check_system(void);

/**
 * @brief Update health check metrics for a request
 * 
 * @param request_succeeded Whether the request succeeded
 */
void update_health_metrics(bool request_succeeded);

/**
 * @brief Direct handler for GET /api/health
 * 
 * @param c Mongoose connection
 * @param hm Mongoose HTTP message
 */
void mg_handle_get_health(struct mg_connection *c, struct mg_http_message *hm);

/**
 * @brief Check if the web server is healthy
 * 
 * @return true if healthy, false otherwise
 */
bool is_web_server_healthy(void);

/**
 * @brief Get the number of consecutive failed health checks
 * 
 * @return int Number of consecutive failed health checks
 */
int get_failed_health_checks(void);

/**
 * @brief Reset health check metrics
 */
void reset_health_metrics(void);

#endif /* API_HANDLERS_HEALTH_H */
