#ifndef LIGHTNVR_WEB_SERVER_H
#define LIGHTNVR_WEB_SERVER_H

#include <stdint.h>
#include <stdbool.h>

// HTTP method enum
typedef enum {
    HTTP_GET,
    HTTP_POST,
    HTTP_PUT,
    HTTP_DELETE,
    HTTP_OPTIONS
} http_method_t;

// HTTP request structure
typedef struct {
    http_method_t method;
    char path[256];
    char query_string[1024];
    char remote_addr[64];
    char user_agent[256];
    char content_type[128];
    uint64_t content_length;
    void *body;
    void *headers;  // Implementation-specific headers structure
    void *session;  // Implementation-specific session structure
} http_request_t;

// HTTP response structure
typedef struct {
    int status_code;
    char content_type[128];
    void *headers;  // Implementation-specific headers structure
    void *body;
    uint64_t body_length;
    bool needs_free;  // Flag to indicate if body needs to be freed
} http_response_t;

// Request handler function type
typedef void (*request_handler_t)(const http_request_t *request, http_response_t *response);

/**
 * Initialize the web server
 * 
 * @param port Port to listen on
 * @param web_root Path to web root directory
 * @return 0 on success, non-zero on failure
 */
int init_web_server(int port, const char *web_root);

/**
 * Shutdown the web server
 */
void shutdown_web_server(void);

/**
 * Register a request handler for a specific path
 * 
 * @param path URL path to handle
 * @param method HTTP method to handle (NULL for all methods)
 * @param handler Handler function
 * @return 0 on success, non-zero on failure
 */
int register_request_handler(const char *path, const char *method, request_handler_t handler);

/**
 * Set authentication requirements
 * 
 * @param enabled Whether authentication is required
 * @param username Username for basic auth
 * @param password Password for basic auth
 * @return 0 on success, non-zero on failure
 */
int set_authentication(bool enabled, const char *username, const char *password);

/**
 * Set CORS (Cross-Origin Resource Sharing) settings
 * 
 * @param enabled Whether CORS is enabled
 * @param allowed_origins Comma-separated list of allowed origins
 * @param allowed_methods Comma-separated list of allowed methods
 * @param allowed_headers Comma-separated list of allowed headers
 * @return 0 on success, non-zero on failure
 */
int set_cors_settings(bool enabled, const char *allowed_origins, 
                     const char *allowed_methods, const char *allowed_headers);

/**
 * Set SSL/TLS settings
 * 
 * @param enabled Whether SSL/TLS is enabled
 * @param cert_path Path to certificate file
 * @param key_path Path to private key file
 * @return 0 on success, non-zero on failure
 */
int set_ssl_settings(bool enabled, const char *cert_path, const char *key_path);

/**
 * Set maximum number of simultaneous connections
 * 
 * @param max_connections Maximum number of connections
 * @return 0 on success, non-zero on failure
 */
int set_max_connections(int max_connections);

/**
 * Set connection timeout
 * 
 * @param timeout_seconds Timeout in seconds
 * @return 0 on success, non-zero on failure
 */
int set_connection_timeout(int timeout_seconds);

/**
 * Create a JSON response
 * 
 * @param response Response structure to fill
 * @param status_code HTTP status code
 * @param json_data JSON data string
 * @return 0 on success, non-zero on failure
 */
int create_json_response(http_response_t *response, int status_code, const char *json_data);

/**
 * Create a file response
 * 
 * @param response Response structure to fill
 * @param status_code HTTP status code
 * @param file_path Path to the file
 * @param content_type Content type (NULL for auto-detect)
 * @return 0 on success, non-zero on failure
 */
int create_file_response(http_response_t *response, int status_code, 
                        const char *file_path, const char *content_type);

/**
 * Create a text response
 * 
 * @param response Response structure to fill
 * @param status_code HTTP status code
 * @param text Text data
 * @param content_type Content type (default: text/plain)
 * @return 0 on success, non-zero on failure
 */
int create_text_response(http_response_t *response, int status_code, 
                        const char *text, const char *content_type);

/**
 * Create a redirect response
 * 
 * @param response Response structure to fill
 * @param status_code HTTP status code (301 or 302)
 * @param location Redirect location
 * @return 0 on success, non-zero on failure
 */
int create_redirect_response(http_response_t *response, int status_code, const char *location);

/**
 * Get a query parameter from a request
 * 
 * @param request HTTP request
 * @param name Parameter name
 * @param value Buffer to store parameter value
 * @param value_size Size of value buffer
 * @return 0 on success, non-zero if parameter not found
 */
int get_query_param(const http_request_t *request, const char *name, 
                   char *value, size_t value_size);

/**
 * Get a form parameter from a request (for application/x-www-form-urlencoded)
 * 
 * @param request HTTP request
 * @param name Parameter name
 * @param value Buffer to store parameter value
 * @param value_size Size of value buffer
 * @return 0 on success, non-zero if parameter not found
 */
int get_form_param(const http_request_t *request, const char *name, 
                  char *value, size_t value_size);

/**
 * Get a header from a request
 * 
 * @param request HTTP request
 * @param name Header name
 * @param value Buffer to store header value
 * @param value_size Size of value buffer
 * @return 0 on success, non-zero if header not found
 */
int get_request_header(const http_request_t *request, const char *name, 
                      char *value, size_t value_size);

/**
 * Set a response header
 * 
 * @param response HTTP response
 * @param name Header name
 * @param value Header value
 * @return 0 on success, non-zero on failure
 */
int set_response_header(http_response_t *response, const char *name, const char *value);

/**
 * Get server statistics
 * 
 * @param active_connections Pointer to store active connection count
 * @param requests_per_second Pointer to store requests per second
 * @param bytes_sent Pointer to store total bytes sent
 * @param bytes_received Pointer to store total bytes received
 * @return 0 on success, non-zero on failure
 */
int get_web_server_stats(int *active_connections, double *requests_per_second, 
                        uint64_t *bytes_sent, uint64_t *bytes_received);


int url_decode(const char *src, char *dst, size_t dst_size);


static void *web_server_shutdown_monitor(void *arg);

#endif // LIGHTNVR_WEB_SERVER_H
