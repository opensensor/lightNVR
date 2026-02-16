/**
 * @file api_handlers.h
 * @brief API handlers for the web server
 */

#ifndef API_HANDLERS_H
#define API_HANDLERS_H

#include "web/request_response.h"
#include <cjson/cJSON.h>
#include "core/config.h"
#include "web/api_handlers_auth.h"
#include "web/request_response.h"

/**
 * @brief Backend-agnostic handler for GET /api/streams
 */
void handle_get_streams(const http_request_t *req, http_response_t *res);

/**
 * @brief Backend-agnostic handler for GET /api/streams/:id
 */
void handle_get_stream(const http_request_t *req, http_response_t *res);

/**
 * @brief Backend-agnostic handler for GET /api/streams/:id/full
 * Returns both stream and motion recording config
 */
void handle_get_stream_full(const http_request_t *req, http_response_t *res);


/**
 * @brief Handler for POST /api/streams
 */
void handle_post_stream(const http_request_t *req, http_response_t *res);

/**
 * @brief Handler for PUT /api/streams/:id
 */
void handle_put_stream(const http_request_t *req, http_response_t *res);

/**
 * @brief Handler for DELETE /api/streams/:id
 */
void handle_delete_stream(const http_request_t *req, http_response_t *res);

/**
 * @brief Handler for POST /api/streams/:id/refresh
 *
 * Triggers a re-registration of the stream with go2rtc.
 * Useful when WebRTC connections fail and the stream needs to be refreshed.
 */
void handle_post_stream_refresh(const http_request_t *req, http_response_t *res);

/**
 * @brief Handler for GET /api/settings
 */
void handle_get_settings(const http_request_t *req, http_response_t *res);

/**
 * @brief Handler for POST /api/settings
 */
void handle_post_settings(const http_request_t *req, http_response_t *res);

/**
 * @brief Handler for GET /api/system/info
 */
void handle_get_system_info(const http_request_t *req, http_response_t *res);

/**
 * @brief Handler for GET /api/system/logs
 */
void handle_get_system_logs(const http_request_t *req, http_response_t *res);

/**
 * @brief Handler for POST /api/system/restart
 */
void handle_post_system_restart(const http_request_t *req, http_response_t *res);

/**
 * @brief Handler for POST /api/system/shutdown
 */
void handle_post_system_shutdown(const http_request_t *req, http_response_t *res);

/**
 * @brief Handler for POST /api/recordings/sync
 */
void handle_post_recordings_sync(const http_request_t *req, http_response_t *res);

/**
 * @brief Handler for GET /api/detection/results/:stream
 */
void handle_get_detection_results(const http_request_t *req, http_response_t *res);

/**
 * @brief Handler for GET /api/detection/models
 */
void handle_get_detection_models(const http_request_t *req, http_response_t *res);

/**
 * @brief Handler for direct HLS requests
 * Endpoint: /hls/{stream_name}/{file}
 */
void handle_direct_hls_request(const http_request_t *req, http_response_t *res);

/**
 * @brief Handler for POST /api/system/logs/clear
 */
void handle_post_system_logs_clear(const http_request_t *req, http_response_t *res);

/**
 * @brief Handler for POST /api/system/backup
 */
void handle_post_system_backup(const http_request_t *req, http_response_t *res);

/**
 * @brief Handler for GET /api/system/status
 */
void handle_get_system_status(const http_request_t *req, http_response_t *res);

/**
 * @brief Handler for POST /api/streams/test
 */
void handle_test_stream(const http_request_t *req, http_response_t *res);

// Retention policy API handlers
/**
 * @brief Handler for GET /api/streams/:name/retention
 */
void handle_get_stream_retention(const http_request_t *req, http_response_t *res);

/**
 * @brief Handler for PUT /api/streams/:name/retention
 */
void handle_put_stream_retention(const http_request_t *req, http_response_t *res);

/**
 * @brief Handler for PUT /api/recordings/:id/protect
 */
void handle_put_recording_protect(const http_request_t *req, http_response_t *res);

/**
 * @brief Handler for PUT /api/recordings/:id/retention
 */
void handle_put_recording_retention(const http_request_t *req, http_response_t *res);

/**
 * @brief Handler for GET /api/recordings
 * List all recordings with pagination and filtering
 */
void handle_get_recordings(const http_request_t *req, http_response_t *res);

/**
 * @brief Handler for POST /api/auth/login
 * User login with username and password
 */
void handle_auth_login(const http_request_t *req, http_response_t *res);

/**
 * @brief Handler for POST /api/auth/logout and GET /logout
 * User logout and session invalidation
 */
void handle_auth_logout(const http_request_t *req, http_response_t *res);

/**
 * @brief Handler for GET /api/auth/verify
 * Verify authentication and return user info
 */
void handle_auth_verify(const http_request_t *req, http_response_t *res);

/**
 * @brief Handler for GET /api/auth/login/config
 * Returns public login config (force_mfa_on_login). No auth required.
 */
void handle_auth_login_config(const http_request_t *req, http_response_t *res);

/**
 * @brief Handler for GET /api/recordings/protected
 */
void handle_get_protected_recordings(const http_request_t *req, http_response_t *res);

/**
 * @brief Handler for POST /api/recordings/batch-protect
 */
void handle_batch_protect_recordings(const http_request_t *req, http_response_t *res);

// Storage health and cleanup API handlers
/**
 * @brief Handler for GET /api/storage/health
 * Returns disk health status, pressure level, free space from cached data
 */
void handle_get_storage_health(const http_request_t *req, http_response_t *res);

/**
 * @brief Handler for POST /api/storage/cleanup
 * Triggers immediate cleanup cycle with optional aggressive flag
 */
void handle_post_storage_cleanup(const http_request_t *req, http_response_t *res);

// Health check functions
void init_health_check_system(void);
void update_health_metrics(bool request_succeeded);
bool is_web_server_healthy(void);
int get_failed_health_checks(void);
void reset_health_metrics(void);

// Server restart functions
bool check_server_restart_needed(void);
void mark_server_for_restart(void);
void reset_server_restart_flag(void);

// Health check thread functions
void start_health_check_thread(void);
void stop_health_check_thread(void);
void cleanup_health_check_system(void);

// Log level utility functions
int log_level_meets_minimum(const char *log_level, const char *min_level);
int get_json_logs_tail(const char *min_level, const char *last_timestamp, char ***logs, int *count);

#endif /* API_HANDLERS_H */
