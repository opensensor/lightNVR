/**
 * @file api_handlers_health.h
 * @brief Health check API handlers for the web server
 */

#ifndef API_HANDLERS_HEALTH_H
#define API_HANDLERS_HEALTH_H

#include <stdbool.h>
#include <pthread.h>
#include "mongoose.h"

/**
 * @brief Initialize health check system
 */
void init_health_check_system(void);

/**
 * @brief Cleanup health check system during shutdown
 */
void cleanup_health_check_system(void);

/**
 * @brief Update health check metrics for a request
 *
 * @param request_succeeded true if the request was successful, false otherwise
 */
void update_health_metrics(bool request_succeeded);

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

/**
 * @brief Check if the server needs to be restarted
 *
 * @return true if server needs restart, false otherwise
 */
bool check_server_restart_needed(void);

/**
 * @brief Mark the server as needing restart
 */
void mark_server_for_restart(void);

/**
 * @brief Reset the restart flag after successful restart
 */
void reset_server_restart_flag(void);

/**
 * @brief Set the web server thread ID
 *
 * @param thread_id The thread ID of the web server thread
 */
void set_web_server_thread_id(pthread_t thread_id);

/**
 * @brief Direct handler for GET /api/health
 *
 * @param c Mongoose connection
 * @param hm Mongoose HTTP message
 */
void mg_handle_get_health(struct mg_connection *c, struct mg_http_message *hm);

/**
 * @brief Direct handler for GET /api/health/hls
 *
 * @param c Mongoose connection
 * @param hm Mongoose HTTP message
 */
void mg_handle_get_hls_health(struct mg_connection *c, struct mg_http_message *hm);

#endif /* API_HANDLERS_HEALTH_H */
