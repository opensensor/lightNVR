/**
 * @file api_handlers.h
 * @brief API handlers for the web server
 */

#ifndef API_HANDLERS_H
#define API_HANDLERS_H

#include "mongoose.h"
#include "cJSON.h"
#include "core/config.h"
#include "web/api_handlers_auth.h"

/**
 * @brief Register API handlers
 * 
 * This function registers API handlers that work directly with Mongoose's
 * HTTP message format, eliminating the need for conversion between formats.
 * 
 * @param mgr Mongoose event manager
 */
void register_api_handlers(struct mg_mgr *mgr);

/**
 * @brief Direct handler for GET /api/streams
 * 
 * @param c Mongoose connection
 * @param hm Mongoose HTTP message
 */
void mg_handle_get_streams(struct mg_connection *c, struct mg_http_message *hm);

/**
 * @brief Direct handler for GET /api/streams/:id
 * 
 * @param c Mongoose connection
 * @param hm Mongoose HTTP message
 */
void mg_handle_get_stream(struct mg_connection *c, struct mg_http_message *hm);

/**
 * @brief Direct handler for POST /api/streams
 * 
 * @param c Mongoose connection
 * @param hm Mongoose HTTP message
 */
void mg_handle_post_stream(struct mg_connection *c, struct mg_http_message *hm);

/**
 * @brief Direct handler for PUT /api/streams/:id
 * 
 * @param c Mongoose connection
 * @param hm Mongoose HTTP message
 */
void mg_handle_put_stream(struct mg_connection *c, struct mg_http_message *hm);

/**
 * @brief Direct handler for DELETE /api/streams/:id
 * 
 * @param c Mongoose connection
 * @param hm Mongoose HTTP message
 */
void mg_handle_delete_stream(struct mg_connection *c, struct mg_http_message *hm);

/**
 * @brief Direct handler for POST /api/streams/:id/toggle_streaming
 * 
 * @param c Mongoose connection
 * @param hm Mongoose HTTP message
 */
void mg_handle_toggle_streaming(struct mg_connection *c, struct mg_http_message *hm);

/**
 * @brief Direct handler for GET /api/settings
 * 
 * @param c Mongoose connection
 * @param hm Mongoose HTTP message
 */
void mg_handle_get_settings(struct mg_connection *c, struct mg_http_message *hm);

/**
 * @brief Direct handler for POST /api/settings
 * 
 * @param c Mongoose connection
 * @param hm Mongoose HTTP message
 */
void mg_handle_post_settings(struct mg_connection *c, struct mg_http_message *hm);

/**
 * @brief Direct handler for GET /api/system/info
 * 
 * @param c Mongoose connection
 * @param hm Mongoose HTTP message
 */
void mg_handle_get_system_info(struct mg_connection *c, struct mg_http_message *hm);

/**
 * @brief Direct handler for GET /api/system/logs
 * 
 * @param c Mongoose connection
 * @param hm Mongoose HTTP message
 */
void mg_handle_get_system_logs(struct mg_connection *c, struct mg_http_message *hm);

/**
 * @brief Direct handler for POST /api/system/restart
 * 
 * @param c Mongoose connection
 * @param hm Mongoose HTTP message
 */
void mg_handle_post_system_restart(struct mg_connection *c, struct mg_http_message *hm);

/**
 * @brief Direct handler for POST /api/system/shutdown
 * 
 * @param c Mongoose connection
 * @param hm Mongoose HTTP message
 */
void mg_handle_post_system_shutdown(struct mg_connection *c, struct mg_http_message *hm);

/**
 * @brief Direct handler for GET /api/recordings
 * 
 * @param c Mongoose connection
 * @param hm Mongoose HTTP message
 */
void mg_handle_get_recordings(struct mg_connection *c, struct mg_http_message *hm);

/**
 * @brief Direct handler for GET /api/recordings/:id
 * 
 * @param c Mongoose connection
 * @param hm Mongoose HTTP message
 */
void mg_handle_get_recording(struct mg_connection *c, struct mg_http_message *hm);

/**
 * @brief Direct handler for DELETE /api/recordings/:id
 * 
 * @param c Mongoose connection
 * @param hm Mongoose HTTP message
 */
void mg_handle_delete_recording(struct mg_connection *c, struct mg_http_message *hm);

/**
 * @brief Direct handler for GET /api/recordings/play/:id
 * 
 * @param c Mongoose connection
 * @param hm Mongoose HTTP message
 */
void mg_handle_play_recording(struct mg_connection *c, struct mg_http_message *hm);

/**
 * @brief Direct handler for GET /api/recordings/download/:id
 * 
 * @param c Mongoose connection
 * @param hm Mongoose HTTP message
 */
void mg_handle_download_recording(struct mg_connection *c, struct mg_http_message *hm);

/**
 * @brief Direct handler for GET /api/streaming/:stream/hls/index.m3u8
 * 
 * @param c Mongoose connection
 * @param hm Mongoose HTTP message
 */
void mg_handle_hls_master_playlist(struct mg_connection *c, struct mg_http_message *hm);

/**
 * @brief Direct handler for GET /api/streaming/:stream/hls/stream.m3u8
 * 
 * @param c Mongoose connection
 * @param hm Mongoose HTTP message
 */
void mg_handle_hls_media_playlist(struct mg_connection *c, struct mg_http_message *hm);

/**
 * @brief Direct handler for GET /api/streaming/:stream/hls/segment_:id.ts
 * 
 * @param c Mongoose connection
 * @param hm Mongoose HTTP message
 */
void mg_handle_hls_segment(struct mg_connection *c, struct mg_http_message *hm);

/**
 * @brief Direct handler for GET /api/detection/results/:stream
 * 
 * @param c Mongoose connection
 * @param hm Mongoose HTTP message
 */
void mg_handle_get_detection_results(struct mg_connection *c, struct mg_http_message *hm);

/**
 * @brief Direct handler for GET /api/detection/models
 * 
 * @param c Mongoose connection
 * @param hm Mongoose HTTP message
 */
void mg_handle_get_detection_models(struct mg_connection *c, struct mg_http_message *hm);

/**
 * @brief Direct handler for POST /api/system/logs/clear
 * 
 * @param c Mongoose connection
 * @param hm Mongoose HTTP message
 */
void mg_handle_post_system_logs_clear(struct mg_connection *c, struct mg_http_message *hm);

/**
 * @brief Direct handler for POST /api/system/backup
 * 
 * @param c Mongoose connection
 * @param hm Mongoose HTTP message
 */
void mg_handle_post_system_backup(struct mg_connection *c, struct mg_http_message *hm);

/**
 * @brief Direct handler for GET /api/system/status
 * 
 * @param c Mongoose connection
 * @param hm Mongoose HTTP message
 */
void mg_handle_get_system_status(struct mg_connection *c, struct mg_http_message *hm);

/**
 * @brief Direct handler for POST /api/streaming/:stream/webrtc/offer
 * 
 * @param c Mongoose connection
 * @param hm Mongoose HTTP message
 */
void mg_handle_webrtc_offer(struct mg_connection *c, struct mg_http_message *hm);

/**
 * @brief Direct handler for POST /api/streaming/:stream/webrtc/ice
 * 
 * @param c Mongoose connection
 * @param hm Mongoose HTTP message
 */
void mg_handle_webrtc_ice(struct mg_connection *c, struct mg_http_message *hm);

/**
 * @brief Direct handler for POST /api/streams/test
 * 
 * @param c Mongoose connection
 * @param hm Mongoose HTTP message
 */
void mg_handle_test_stream(struct mg_connection *c, struct mg_http_message *hm);

/**
 * @brief Helper function to extract path parameter from URL
 * 
 * @param hm Mongoose HTTP message
 * @param prefix URL prefix to strip
 * @param param_buf Buffer to store the extracted parameter
 * @param buf_size Size of the buffer
 * @return int 0 on success, non-zero on error
 */
int mg_extract_path_param(struct mg_http_message *hm, const char *prefix, char *param_buf, size_t buf_size);

/**
 * @brief Helper function to send a JSON response
 * 
 * @param c Mongoose connection
 * @param status_code HTTP status code
 * @param json_str JSON string to send
 */
void mg_send_json_response(struct mg_connection *c, int status_code, const char *json_str);

/**
 * @brief Helper function to send a JSON error response
 * 
 * @param c Mongoose connection
 * @param status_code HTTP status code
 * @param error_message Error message
 */
void mg_send_json_error(struct mg_connection *c, int status_code, const char *error_message);

/**
 * @brief Helper function to parse JSON from request body
 * 
 * @param hm Mongoose HTTP message
 * @return cJSON* Parsed JSON object or NULL on error
 */
cJSON* mg_parse_json_body(struct mg_http_message *hm);

/**
 * @brief Create a JSON string from a config structure
 * 
 * @param config Configuration structure
 * @return char* JSON string (must be freed by caller)
 */
char* mg_create_config_json(const config_t *config);

/**
 * @brief Create a JSON error response
 * 
 * @param c Mongoose connection
 * @param status_code HTTP status code
 * @param message Error message
 */
void mg_create_error_response(struct mg_connection *c, int status_code, const char *message);

/**
 * @brief URL decode a string
 * 
 * @param src Source string
 * @param dst Destination buffer
 * @param dst_size Destination buffer size
 */
void mg_url_decode_string(const char *src, char *dst, size_t dst_size);

/**
 * @brief Extract a parameter from a URL path
 * 
 * @param path URL path
 * @param prefix Path prefix to skip
 * @param param_buf Buffer to store the parameter
 * @param buf_size Buffer size
 * @return int 0 on success, non-zero on error
 */
int mg_extract_path_parameter(const char *path, const char *prefix, char *param_buf, size_t buf_size);

/**
 * @brief Parse a JSON boolean value
 * 
 * @param json JSON object
 * @param key Key to look for
 * @param default_value Default value if key is not found
 * @return int Boolean value (0 or 1)
 */
int mg_parse_json_boolean(const cJSON *json, const char *key, int default_value);

/**
 * @brief Parse a JSON integer value
 * 
 * @param json JSON object
 * @param key Key to look for
 * @param default_value Default value if key is not found
 * @return long long Integer value
 */
long long mg_parse_json_integer(const cJSON *json, const char *key, long long default_value);

/**
 * @brief Parse a JSON string value
 * 
 * @param json JSON object
 * @param key Key to look for
 * @return char* String value (must be freed by caller) or NULL if not found
 */
char* mg_parse_json_string(const cJSON *json, const char *key);

/**
 * @brief Check if a JSON object has a key
 * 
 * @param json JSON object
 * @param key Key to look for
 * @return int 1 if key exists, 0 otherwise
 */
int mg_json_has_key(const cJSON *json, const char *key);

#endif /* API_HANDLERS_H */
