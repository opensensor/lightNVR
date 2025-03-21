#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "web/mongoose_server_utils.h"
#include "core/logger.h"

// Include Mongoose
#include "mongoose.h"

/**
 * @brief Check if a path matches a pattern
 */
bool mongoose_server_path_matches(const char *pattern, const char *path) {
    // Special case: if pattern ends with "/*", it matches any path that starts with the prefix
    size_t pattern_len = strlen(pattern);
    if (pattern_len >= 2 && pattern[pattern_len-2] == '/' && pattern[pattern_len-1] == '*') {
        // Extract the prefix (everything before the "/*")
        size_t prefix_len = pattern_len - 2;
        
        // Check if the path starts with the prefix
        if (strncmp(pattern, path, prefix_len) == 0) {
            // Make sure the next character in path is either '/' or the end of string
            if (path[prefix_len] == '/' || path[prefix_len] == '\0') {
                return true;
            }
        }
        
        return false;
    }
    
    // Special case: if pattern contains "/*/", it matches any segment in the path
    const char *wildcard_pos = strstr(pattern, "/*/");
    if (wildcard_pos != NULL) {
        // Split the pattern into prefix and suffix
        size_t prefix_len = wildcard_pos - pattern;
        const char *suffix = wildcard_pos + 3; // Skip "/*/"
        
        // Check if the path starts with the prefix
        if (strncmp(pattern, path, prefix_len) == 0) {
            // Find the next '/' in the path after the prefix
            const char *path_after_prefix = path + prefix_len + 1;
            const char *next_slash = strchr(path_after_prefix, '/');
            
            if (next_slash) {
                // Check if the rest of the path matches the suffix
                if (strcmp(next_slash + 1, suffix) == 0) {
                    return true;
                }
            } else if (suffix[0] == '\0') {
                // If there's no next slash and the suffix is empty, it's a match
                return true;
            }
        }
    }

    // Check for exact match
    if (strcmp(pattern, path) == 0) {
        return true;
    }

    // Check if the path has a trailing slash but pattern doesn't
    size_t path_len = strlen(path);
    if (path_len > 0 && path[path_len-1] == '/' && 
        strncmp(pattern, path, path_len-1) == 0 && pattern[path_len-1] == '\0') {
        return true;
    }

    // Check if the pattern has a trailing slash but path doesn't
    if (pattern_len > 0 && pattern[pattern_len-1] == '/' && 
        strncmp(pattern, path, pattern_len-1) == 0 && path[pattern_len-1] == '\0') {
        return true;
    }

    return false;
}

/**
 * @brief Convert Mongoose HTTP message to HTTP request
 */
int mongoose_server_mg_to_request(struct mg_connection *conn, struct mg_http_message *msg, 
                                 http_request_t *request) {
    if (!conn || !msg || !request) {
        return -1;
    }

    // Initialize request
    memset(request, 0, sizeof(http_request_t));

    // Parse method
    if (mg_match(msg->method, mg_str("GET"), NULL)) {
        request->method = HTTP_GET;
    } else if (mg_match(msg->method, mg_str("POST"), NULL)) {
        request->method = HTTP_POST;
    } else if (mg_match(msg->method, mg_str("PUT"), NULL)) {
        request->method = HTTP_PUT;
    } else if (mg_match(msg->method, mg_str("DELETE"), NULL)) {
        request->method = HTTP_DELETE;
    } else if (mg_match(msg->method, mg_str("OPTIONS"), NULL)) {
        request->method = HTTP_OPTIONS;
    } else if (mg_match(msg->method, mg_str("HEAD"), NULL)) {
        request->method = HTTP_HEAD;
    } else {
        log_warn("Unsupported HTTP method");
        return -1;
    }

    // Parse URI
    char uri[MAX_PATH_LENGTH];
    size_t uri_len = msg->uri.len < sizeof(uri) - 1 ? msg->uri.len : sizeof(uri) - 1;
    memcpy(uri, msg->uri.buf, uri_len);
    uri[uri_len] = '\0';

    // Split URI into path and query string
    char *query = strchr(uri, '?');
    if (query) {
        *query = '\0';
        strncpy(request->query_string, query + 1, sizeof(request->query_string) - 1);
        request->query_string[sizeof(request->query_string) - 1] = '\0';
    }

    // Copy path
    strncpy(request->path, uri, sizeof(request->path) - 1);
    request->path[sizeof(request->path) - 1] = '\0';

    // Parse headers
    request->headers = calloc(MAX_HEADERS, sizeof(http_header_t));
    if (!request->headers) {
        log_error("Failed to allocate memory for request headers");
        return -1;
    }

    request->num_headers = 0;
    for (int i = 0; i < MG_MAX_HTTP_HEADERS && i < MAX_HEADERS; i++) {
        if (msg->headers[i].name.len == 0) {
            break;
        }

        // Copy header name
        size_t name_len = msg->headers[i].name.len < sizeof(request->headers[0].name) - 1 ?
                         msg->headers[i].name.len : sizeof(request->headers[0].name) - 1;
        memcpy(request->headers[request->num_headers].name, msg->headers[i].name.buf, name_len);
        request->headers[request->num_headers].name[name_len] = '\0';

        // Copy header value
        size_t value_len = msg->headers[i].value.len < sizeof(request->headers[0].value) - 1 ?
                          msg->headers[i].value.len : sizeof(request->headers[0].value) - 1;
        memcpy(request->headers[request->num_headers].value, msg->headers[i].value.buf, value_len);
        request->headers[request->num_headers].value[value_len] = '\0';

        // Check for special headers
        if (strcasecmp(request->headers[request->num_headers].name, "Content-Type") == 0) {
            strncpy(request->content_type, request->headers[request->num_headers].value, 
                   sizeof(request->content_type) - 1);
            request->content_type[sizeof(request->content_type) - 1] = '\0';
        } else if (strcasecmp(request->headers[request->num_headers].name, "Content-Length") == 0) {
            request->content_length = strtoull(request->headers[request->num_headers].value, NULL, 10);
        } else if (strcasecmp(request->headers[request->num_headers].name, "User-Agent") == 0) {
            strncpy(request->user_agent, request->headers[request->num_headers].value, 
                   sizeof(request->user_agent) - 1);
            request->user_agent[sizeof(request->user_agent) - 1] = '\0';
        }

        request->num_headers++;
    }

    // Copy body if present
    if (msg->body.len > 0) {
        request->body = malloc(msg->body.len + 1);
        if (!request->body) {
            log_error("Failed to allocate memory for request body");
            free(request->headers);
            return -1;
        }

        memcpy(request->body, msg->body.buf, msg->body.len);
        ((char *)request->body)[msg->body.len] = '\0';
        request->content_length = msg->body.len;
    }

    return 0;
}

/**
 * @brief Send HTTP response using Mongoose
 */
int mongoose_server_send_response(struct mg_connection *conn, const http_response_t *response) {
    if (!conn || !response) {
        return -1;
    }

    // Start response
    mg_printf(conn, "HTTP/1.1 %d %s\r\n", response->status_code, 
             response->status_code == 200 ? "OK" : "Error");

    // Add Content-Type header
    mg_printf(conn, "Content-Type: %s\r\n", response->content_type);

    // Add Content-Length header
    mg_printf(conn, "Content-Length: %zu\r\n", response->body_length);

    // Add custom headers
    if (response->headers && response->num_headers > 0) {
        for (int i = 0; i < response->num_headers; i++) {
            mg_printf(conn, "%s: %s\r\n", response->headers[i].name, response->headers[i].value);
        }
    }

    // End headers
    mg_printf(conn, "\r\n");

    // Send body if present
    if (response->body && response->body_length > 0) {
        mg_send(conn, response->body, response->body_length);
    }

    return 0;
}
