#ifndef REQUEST_RESPONSE_H
#define REQUEST_RESPONSE_H

#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>

#include "core/config.h"

// Maximum headers in HTTP requests/responses
#define MAX_HEADERS 50

// Maximum query string parameters
#define MAX_QUERY_PARAMS 32

// HTTP methods (prefixed with HTTP_METHOD_ to avoid conflicts with llhttp)
typedef enum {
    HTTP_METHOD_GET,
    HTTP_METHOD_POST,
    HTTP_METHOD_PUT,
    HTTP_METHOD_DELETE,
    HTTP_METHOD_OPTIONS,
    HTTP_METHOD_HEAD,
    HTTP_METHOD_PATCH,
    HTTP_METHOD_UNKNOWN
} http_method_t;

// HTTP header structure
typedef struct {
    char name[128];   // Header name
    char value[1024]; // Header value
} http_header_t;

// HTTP request structure
typedef struct {
    http_method_t method;                  // Request method (enum)
    char method_str[16];                   // Request method as string
    char path[MAX_PATH_LENGTH];            // Request path (without query string)
    char uri[MAX_PATH_LENGTH + 1024];      // Full URI (path + query string)
    char query_string[1024];               // Query string (after ?)
    char content_type[128];                // Content-Type header
    uint64_t content_length;               // Content-Length header
    char user_agent[256];                  // User-Agent header
    void *body;                            // Request body (may not be null-terminated)
    size_t body_len;                       // Length of request body
    http_header_t headers[MAX_HEADERS];    // Array of headers (inline, no alloc needed)
    int num_headers;                       // Number of headers
    char client_ip[64];                    // Client IP address
    void *user_data;                       // User data pointer (e.g., http_server_t*)
} http_request_t;

// HTTP response structure
typedef struct {
    int status_code;                       // Response status code
    char content_type[128];                // Content-Type header
    size_t body_length;                    // Content length
    void *body;                            // Response body (caller manages memory)
    bool body_allocated;                   // Whether body was allocated by response helpers
    http_header_t headers[MAX_HEADERS];    // Array of headers (inline, no alloc needed)
    int num_headers;                       // Number of headers
    void *user_data;                       // User data pointer
} http_response_t;

// Request handler function type (backend-agnostic)
typedef void (*request_handler_t)(const http_request_t *request, http_response_t *response);

/**
 * @brief Get a header value from the request
 * @param req HTTP request
 * @param name Header name (case-insensitive)
 * @return Header value or NULL if not found
 */
const char* http_request_get_header(const http_request_t *req, const char *name);

/**
 * @brief Get a query parameter value from the request
 * @param req HTTP request
 * @param name Parameter name
 * @param value Buffer to store the value
 * @param value_len Size of value buffer
 * @return Length of value on success, -1 if not found
 */
int http_request_get_query_param(const http_request_t *req, const char *name, char *value, size_t value_len);

/**
 * @brief Get the request body as a null-terminated string
 * @param req HTTP request
 * @param buf Buffer to store the body string
 * @param buf_len Size of buffer
 * @return 0 on success, -1 on error
 */
int http_request_get_body_str(const http_request_t *req, char *buf, size_t buf_len);

/**
 * @brief Extract a path parameter from the URI (e.g., /api/streams/:id)
 * @param req HTTP request
 * @param prefix URL prefix to strip (e.g., "/api/streams/")
 * @param param_buf Buffer to store the extracted parameter
 * @param buf_size Size of the buffer
 * @return 0 on success, -1 on error
 */
int http_request_extract_path_param(const http_request_t *req, const char *prefix,
                                     char *param_buf, size_t buf_size);

/**
 * @brief Add a header to the response
 * @param res HTTP response
 * @param name Header name
 * @param value Header value
 * @return 0 on success, -1 if headers full
 */
int http_response_add_header(http_response_t *res, const char *name, const char *value);

/**
 * @brief Set the response body from a string (makes a copy)
 * @param res HTTP response
 * @param body Body string
 * @return 0 on success, -1 on error
 */
int http_response_set_body(http_response_t *res, const char *body);

/**
 * @brief Set JSON body on the response with appropriate content-type and CORS headers
 * @param res HTTP response
 * @param status_code HTTP status code
 * @param json_str JSON string to set as body
 * @return 0 on success, -1 on error
 */
int http_response_set_json(http_response_t *res, int status_code, const char *json_str);

/**
 * @brief Set a JSON error response
 * @param res HTTP response
 * @param status_code HTTP status code
 * @param error_message Error message string
 * @return 0 on success, -1 on error
 */
int http_response_set_json_error(http_response_t *res, int status_code, const char *error_message);

/**
 * @brief Add standard CORS headers to a response
 * @param res HTTP response
 */
void http_response_add_cors_headers(http_response_t *res);

/**
 * @brief Initialize an http_request_t struct to safe defaults
 * @param req Request to initialize
 */
void http_request_init(http_request_t *req);

/**
 * @brief Initialize an http_response_t struct to safe defaults
 * @param res Response to initialize
 */
void http_response_init(http_response_t *res);

/**
 * @brief Free any allocated resources in a response
 * @param res Response to clean up
 */
void http_response_free(http_response_t *res);

/**
 * @brief URL decode a string
 * @param src Source string
 * @param dst Destination buffer
 * @param dst_size Destination buffer size
 * @return 0 on success, -1 on error
 */
int url_decode(const char *src, char *dst, size_t dst_size);

/**
 * @brief Serve a file with range request support (backend-agnostic)
 *
 * This function serves a file using the libuv backend.
 * It automatically handles:
 * - Range requests for video seeking
 * - MIME type detection
 * - Proper HTTP status codes (200, 206, 404, 416)
 * - CORS headers
 *
 * @param req HTTP request (used to get Range header and backend connection)
 * @param res HTTP response (may be modified for error responses)
 * @param file_path Path to the file to serve
 * @param content_type MIME type (or NULL for auto-detection)
 * @param extra_headers Additional headers to include (or NULL)
 * @return 0 on success, -1 on error
 */
int http_serve_file(const http_request_t *req, const http_response_t *res,
                    const char *file_path, const char *content_type,
                    const char *extra_headers);

#endif /* REQUEST_RESPONSE_H */