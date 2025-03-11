#ifndef WEB_SERVER_H
#define WEB_SERVER_H

#include <stdint.h>
#include <stdbool.h>
#include "web/request_response.h"

// External server socket for signal handling
extern int server_socket;

// Server functions
int init_web_server(int port, const char *web_root);
void shutdown_web_server(void);
int register_request_handler(const char *path, const char *method, request_handler_t handler);
int set_authentication(bool enabled, const char *username, const char *password);
int set_cors_settings(bool enabled, const char *allowed_origins, const char *allowed_methods, const char *allowed_headers);
int set_ssl_settings(bool enabled, const char *cert_path, const char *key_path);
int set_max_connections(int max_connections);
int set_connection_timeout(int timeout_seconds);
int daemonize_web_server(const char *pid_file);

// Request/response functions
int create_json_response(http_response_t *response, int status_code, const char *json_data);
int create_file_response(http_response_t *response, int status_code, const char *file_path, const char *content_type);
int create_text_response(http_response_t *response, int status_code, const char *text, const char *content_type);
int create_redirect_response(http_response_t *response, int status_code, const char *location);
int get_query_param(const http_request_t *request, const char *param_name, char *value, size_t value_size);
int get_form_param(const http_request_t *request, const char *name, char *value, size_t value_size);
const char *get_request_header(const http_request_t *request, const char *name);
int set_response_header(http_response_t *response, const char *name, const char *value);
int get_web_server_stats(int *active_connections, double *requests_per_second, uint64_t *bytes_sent, uint64_t *bytes_received);

// URL utilities
int url_decode(const char *src, char *dst, size_t dst_size);

// Helper functions
void cleanup_request(http_request_t *request);
void cleanup_response(http_response_t *response);

#endif /* WEB_SERVER_H */