#ifndef REQUEST_RESPONSE_H
#define REQUEST_RESPONSE_H

#include <stdint.h>
#include <stddef.h>

#include "core/config.h"

// Maximum headers in HTTP requests/responses
#define MAX_HEADERS 50

// HTTP methods
typedef enum {
    HTTP_GET,
    HTTP_POST,
    HTTP_PUT,
    HTTP_DELETE,
    HTTP_OPTIONS,
    HTTP_HEAD
} http_method_t;

// HTTP header structure
typedef struct {
    char name[128];   // Header name
    char value[1024]; // Header value
} http_header_t;

// HTTP request structure
typedef struct {
    http_method_t method;                  // Request method
    char path[MAX_PATH_LENGTH];            // Request path
    char query_string[1024];               // Query string
    char content_type[128];                // Content-Type header
    uint64_t content_length;               // Content-Length header
    char user_agent[256];                  // User-Agent header
    void *body;                            // Request body
    http_header_t *headers;                // Array of headers
    int num_headers;                       // Number of headers
    void *user_data;                       // User data pointer
} http_request_t;

// HTTP response structure
typedef struct {
    int status_code;                       // Response status code
    char content_type[128];                // Content-Type header
    size_t body_length;                    // Content length
    void *body;                            // Response body
    http_header_t *headers;                // Array of headers
    int num_headers;                       // Number of headers
    void *user_data;                       // User data pointer
} http_response_t;

// Request handler function type
typedef void (*request_handler_t)(const http_request_t *request, http_response_t *response);

// Request parsing and handling functions
int parse_request(int client_socket, http_request_t *request);
int send_response(int client_socket, const http_response_t *response);

#endif /* REQUEST_RESPONSE_H */