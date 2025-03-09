#define _GNU_SOURCE

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <fcntl.h>
#include <errno.h>
#include <ctype.h>
#include <stdbool.h>
#include <sys/socket.h>

#include "web/request_response.h"
#include "web/web_server.h"
#include "core/logger.h"
#include "utils/memory.h"

#define MAX_HEADER_SIZE 8192
#define RECV_BUFFER_SIZE 4096

// Forward declarations
static int parse_headers(const char *buffer, size_t length, http_request_t *request);
static const char *get_status_message(int status_code);

// Parse HTTP request
int parse_request(int client_socket, http_request_t *request) {
    if (!request) {
        return -1;
    }

    // Initialize the request
    memset(request, 0, sizeof(http_request_t));

    // Buffer for receiving data
    char buffer[RECV_BUFFER_SIZE];
    char header_buffer[MAX_HEADER_SIZE] = {0};
    size_t header_size = 0;

    // Track the end of headers
    bool headers_complete = false;

    // Read data until we find the end of headers
    while (!headers_complete && header_size < MAX_HEADER_SIZE - 1) {
        ssize_t bytes_read = recv(client_socket, buffer, sizeof(buffer) - 1, 0);

        if (bytes_read <= 0) {
            if (bytes_read == 0) {
                // Connection closed
                log_debug("Client closed connection during header read");
                return -1;
            } else if (errno == EINTR) {
                // Interrupted, try again
                continue;
            } else if (errno == EAGAIN || errno == EWOULDBLOCK) {
                // Timeout - client too slow
                log_error("Timeout reading request headers");
                return -1;
            } else {
                // Other error
                log_error("Failed to read request: %s", strerror(errno));
                return -1;
            }
        }

        // Null-terminate the buffer
        buffer[bytes_read] = '\0';

        // Check if we have enough room for the new data
        if (header_size + bytes_read >= MAX_HEADER_SIZE - 1) {
            log_error("Request headers too large");
            return -1;
        }

        // Append to header buffer
        memcpy(header_buffer + header_size, buffer, bytes_read);
        header_size += bytes_read;
        header_buffer[header_size] = '\0';

        // Check for end of headers "\r\n\r\n" or "\n\n"
        char *end_of_headers = strstr(header_buffer, "\r\n\r\n");
        if (!end_of_headers) {
            end_of_headers = strstr(header_buffer, "\n\n");
            if (end_of_headers) {
                end_of_headers += 2; // Skip "\n\n"
            }
        } else {
            end_of_headers += 4; // Skip "\r\n\r\n"
        }

        if (end_of_headers) {
            headers_complete = true;

            // Calculate body start position and bytes already read
            size_t header_length = end_of_headers - header_buffer;
            size_t body_bytes_read = header_size - header_length;

            // Parse headers
            if (parse_headers(header_buffer, header_length, request) != 0) {
                log_error("Failed to parse request headers");
                return -1;
            }

            // Process the already read part of the body if any
            if (body_bytes_read > 0 && request->content_length > 0) {
                // Allocate memory for body
                request->body = malloc(request->content_length + 1);
                if (!request->body) {
                    log_error("Failed to allocate memory for request body");
                    return -1;
                }

                // Copy already read body data
                memcpy(request->body, end_of_headers, body_bytes_read);

                // Read the rest of the body if needed
                size_t total_body_read = body_bytes_read;

                while (total_body_read < request->content_length) {
                    // Calculate remaining bytes
                    size_t remaining = request->content_length - total_body_read;
                    size_t to_read = remaining < RECV_BUFFER_SIZE ? remaining : RECV_BUFFER_SIZE;

                    bytes_read = recv(client_socket, buffer, to_read, 0);

                    if (bytes_read <= 0) {
                        if (bytes_read == 0) {
                            // Connection closed
                            log_error("Client closed connection during body read");
                            free(request->body);
                            request->body = NULL;
                            return -1;
                        } else if (errno == EINTR) {
                            // Interrupted, try again
                            continue;
                        } else if (errno == EAGAIN || errno == EWOULDBLOCK) {
                            // Timeout
                            log_error("Timeout reading request body");
                            free(request->body);
                            request->body = NULL;
                            return -1;
                        } else {
                            // Other error
                            log_error("Failed to read request body: %s", strerror(errno));
                            free(request->body);
                            request->body = NULL;
                            return -1;
                        }
                    }

                    // Copy data to body
                    memcpy((char *)request->body + total_body_read, buffer, bytes_read);
                    total_body_read += bytes_read;
                }

                // Null-terminate body (for text content)
                ((char *)request->body)[request->content_length] = '\0';
            }
        }
    }

    // Check if we found the end of headers
    if (!headers_complete) {
        log_error("Invalid request format or headers too large");
        return -1;
    }

    return 0;
}

// Parse headers from buffer
static int parse_headers(const char *buffer, size_t length, http_request_t *request) {
    // Create a temporary copy of the buffer for parsing
    char *temp_buffer = malloc(length + 1);
    if (!temp_buffer) {
        log_error("Failed to allocate memory for header parsing");
        return -1;
    }

    memcpy(temp_buffer, buffer, length);
    temp_buffer[length] = '\0';

    // Parse request line
    char *line_end = strstr(temp_buffer, "\r\n");
    if (!line_end) {
        line_end = strchr(temp_buffer, '\n');
    }

    if (!line_end) {
        log_error("Invalid request format: no line endings found");
        free(temp_buffer);
        return -1;
    }

    *line_end = '\0';
    char *request_line = temp_buffer;

    // Parse method, path, and version
    char method[16] = {0};
    char path[256] = {0};
    char version[16] = {0};

    if (sscanf(request_line, "%15s %255s %15s", method, path, version) != 3) {
        log_error("Invalid request line: %s", request_line);
        free(temp_buffer);
        return -1;
    }

    // Set request method
    if (strcmp(method, "GET") == 0) {
        request->method = HTTP_GET;
    } else if (strcmp(method, "POST") == 0) {
        request->method = HTTP_POST;
    } else if (strcmp(method, "PUT") == 0) {
        request->method = HTTP_PUT;
    } else if (strcmp(method, "DELETE") == 0) {
        request->method = HTTP_DELETE;
    } else if (strcmp(method, "OPTIONS") == 0) {
        request->method = HTTP_OPTIONS;
    } else if (strcmp(method, "HEAD") == 0) {
        request->method = HTTP_HEAD;
    } else {
        log_error("Unsupported method: %s", method);
        free(temp_buffer);
        return -1;
    }

    // Parse path and query string
    char *query = strchr(path, '?');
    if (query) {
        *query = '\0';
        strncpy(request->query_string, query + 1, sizeof(request->query_string) - 1);
        request->query_string[sizeof(request->query_string) - 1] = '\0';
    }

    // URL decode path
    url_decode(path, request->path, sizeof(request->path));

    // Start parsing headers
    char *header_start = line_end + 1;
    if (*header_start == '\r') header_start++; // Skip \r\n or \n

    // Parse each header
    int header_count = 0;
    http_header_t headers[MAX_HEADERS];

    char *saveptr;
    char *line = strtok_r(header_start, "\r\n", &saveptr);

    while (line && header_count < MAX_HEADERS) {
        // Skip empty lines
        if (*line == '\0') {
            line = strtok_r(NULL, "\r\n", &saveptr);
            continue;
        }

        // Find the colon
        char *colon = strchr(line, ':');
        if (!colon) {
            log_warn("Invalid header format: %s", line);
            line = strtok_r(NULL, "\r\n", &saveptr);
            continue;
        }

        // Split the header name and value
        *colon = '\0';
        char *name = line;
        char *value = colon + 1;

        // Trim leading and trailing whitespace
        while (isspace(*value)) value++;

        // Copy header name and value
        strncpy(headers[header_count].name, name, sizeof(headers[header_count].name) - 1);
        headers[header_count].name[sizeof(headers[header_count].name) - 1] = '\0';

        strncpy(headers[header_count].value, value, sizeof(headers[header_count].value) - 1);
        headers[header_count].value[sizeof(headers[header_count].value) - 1] = '\0';

        // Check for important headers
        if (strcasecmp(name, "Content-Type") == 0) {
            strncpy(request->content_type, value, sizeof(request->content_type) - 1);
            request->content_type[sizeof(request->content_type) - 1] = '\0';
        } else if (strcasecmp(name, "Content-Length") == 0) {
            request->content_length = strtoull(value, NULL, 10);
        } else if (strcasecmp(name, "User-Agent") == 0) {
            strncpy(request->user_agent, value, sizeof(request->user_agent) - 1);
            request->user_agent[sizeof(request->user_agent) - 1] = '\0';
        }

        header_count++;
        line = strtok_r(NULL, "\r\n", &saveptr);
    }

    free(temp_buffer);

    // Store headers in request if we have any
    if (header_count > 0) {
        request->headers = malloc(header_count * sizeof(http_header_t));
        if (!request->headers) {
            log_error("Failed to allocate memory for headers");
            return -1;
        }

        memcpy(request->headers, headers, header_count * sizeof(http_header_t));
        request->num_headers = header_count;
    }

    return 0;
}

// Send HTTP response
int send_response(int client_socket, const http_response_t *response) {
    if (!response) {
        return -1;
    }

    // Format status line and headers
    char header_buffer[4096];
    int offset = 0;

    // Add status line
    offset += snprintf(header_buffer + offset, sizeof(header_buffer) - offset,
                     "HTTP/1.1 %d %s\r\n",
                     response->status_code,
                     get_status_message(response->status_code));

    // Add Content-Type
    if (response->content_type[0] != '\0') {
        offset += snprintf(header_buffer + offset, sizeof(header_buffer) - offset,
                         "Content-Type: %s\r\n", response->content_type);
    }

    // Add Content-Length
    offset += snprintf(header_buffer + offset, sizeof(header_buffer) - offset,
                     "Content-Length: %zu\r\n", response->body_length);

    // Add Server header
    offset += snprintf(header_buffer + offset, sizeof(header_buffer) - offset,
                     "Server: LightNVR\r\n");

    // Add custom headers
    if (response->headers && response->num_headers > 0) {
        for (int i = 0; i < response->num_headers; i++) {
            offset += snprintf(header_buffer + offset, sizeof(header_buffer) - offset,
                             "%s: %s\r\n",
                             response->headers[i].name,
                             response->headers[i].value);
        }
    }

    // End headers with empty line
    offset += snprintf(header_buffer + offset, sizeof(header_buffer) - offset, "\r\n");

    // Send headers
    ssize_t sent = 0;
    ssize_t total_sent = 0;
    size_t to_send = offset;

    while (total_sent < to_send) {
        sent = send(client_socket, header_buffer + total_sent, to_send - total_sent, 0);

        if (sent <= 0) {
            if (sent < 0 && (errno == EINTR || errno == EAGAIN || errno == EWOULDBLOCK)) {
                // Retry on these errors
                continue;
            }
            log_error("Failed to send response headers: %s", strerror(errno));
            return -1;
        }

        total_sent += sent;
    }

    // Send body if present
    if (response->body && response->body_length > 0) {
        total_sent = 0;
        to_send = response->body_length;

        while (total_sent < to_send) {
            // Send in chunks to avoid blocking for large files
            size_t chunk_size = to_send - total_sent > 65536 ? 65536 : to_send - total_sent;

            sent = send(client_socket, (char *)response->body + total_sent, chunk_size, 0);

            if (sent <= 0) {
                if (sent < 0 && (errno == EINTR || errno == EAGAIN || errno == EWOULDBLOCK)) {
                    // Retry on these errors
                    continue;
                }
                log_error("Failed to send response body: %s", strerror(errno));
                return -1;
            }

            total_sent += sent;
        }
    }

    return 0;
}

// Get a header from a request
const char *get_request_header(const http_request_t *request, const char *name) {
    if (!request || !name) {
        return NULL;
    }

    // Check for common headers with special handling
    if (strcasecmp(name, "Content-Type") == 0 && request->content_type[0] != '\0') {
        return request->content_type;
    }
    else if (strcasecmp(name, "User-Agent") == 0 && request->user_agent[0] != '\0') {
        return request->user_agent;
    }
    else if (strcasecmp(name, "Content-Length") == 0 && request->content_length > 0) {
        // Convert content length to string and return it
        // This is a simplified approach using a static buffer
        static char content_length_str[32];
        snprintf(content_length_str, sizeof(content_length_str), "%llu",
                (unsigned long long)request->content_length);
        return content_length_str;
    }

    // Search for the header in the headers array
    if (request->headers && request->num_headers > 0) {
        for (int i = 0; i < request->num_headers; i++) {
            if (strcasecmp(request->headers[i].name, name) == 0) {
                return request->headers[i].value;
            }
        }
    }

    return NULL;
}

// Set a response header
int set_response_header(http_response_t *response, const char *name, const char *value) {
    if (!response || !name || !value) {
        log_error("Invalid parameters for set_response_header");
        return -1;
    }

    // Handle special headers with direct field access
    if (strcasecmp(name, "Content-Type") == 0) {
        strncpy(response->content_type, value, sizeof(response->content_type) - 1);
        response->content_type[sizeof(response->content_type) - 1] = '\0';
        return 0;
    }

    // Create or resize headers array
    if (!response->headers) {
        response->headers = malloc(sizeof(http_header_t));
        if (!response->headers) {
            log_error("Failed to allocate memory for response headers");
            return -1;
        }
        response->num_headers = 0;
    } else if (response->num_headers >= MAX_HEADERS) {
        log_error("Too many response headers");
        return -1;
    } else if (response->num_headers > 0) {
        // Check if header already exists - update it
        for (int i = 0; i < response->num_headers; i++) {
            if (strcasecmp(response->headers[i].name, name) == 0) {
                strncpy(response->headers[i].value, value, sizeof(response->headers[i].value) - 1);
                response->headers[i].value[sizeof(response->headers[i].value) - 1] = '\0';
                return 0;
            }
        }

        // Resize array to add new header
        http_header_t *new_headers = realloc(response->headers,
                                            (response->num_headers + 1) * sizeof(http_header_t));
        if (!new_headers) {
            log_error("Failed to resize response headers array");
            return -1;
        }
        response->headers = new_headers;
    }

    // Add the new header
    strncpy(response->headers[response->num_headers].name, name,
           sizeof(response->headers[0].name) - 1);
    response->headers[response->num_headers].name[sizeof(response->headers[0].name) - 1] = '\0';

    strncpy(response->headers[response->num_headers].value, value,
           sizeof(response->headers[0].value) - 1);
    response->headers[response->num_headers].value[sizeof(response->headers[0].value) - 1] = '\0';

    response->num_headers++;

    return 0;
}

// Get status message for status code
static const char *get_status_message(int status_code) {
    // Common status codes
    switch (status_code) {
        case 200: return "OK";
        case 201: return "Created";
        case 204: return "No Content";
        case 301: return "Moved Permanently";
        case 302: return "Found";
        case 304: return "Not Modified";
        case 400: return "Bad Request";
        case 401: return "Unauthorized";
        case 403: return "Forbidden";
        case 404: return "Not Found";
        case 405: return "Method Not Allowed";
        case 500: return "Internal Server Error";
        case 501: return "Not Implemented";
        case 503: return "Service Unavailable";
        default: return "Unknown";
    }
}

// Helper function to get query parameter
int get_query_param(const http_request_t *request, const char *param_name, char *value, size_t value_size) {
    if (!request || !request->query_string || !param_name || !value) {
        return -1;
    }

    // Find the parameter in the query string
    const char *query = request->query_string;
    size_t param_len = strlen(param_name);
    char search[param_len + 2];

    // Format as "param=" or "param&"
    snprintf(search, sizeof(search), "%s=", param_name);

    const char *param_start = strstr(query, search);
    if (!param_start) {
        // Try at the beginning of the query string
        if (strncmp(query, search, strlen(search)) == 0) {
            param_start = query;
        } else {
            return -1; // Parameter not found
        }
    }

    // Move past "param="
    param_start += strlen(search);

    // Find the end of the parameter value
    const char *param_end = strchr(param_start, '&');
    if (!param_end) {
        param_end = param_start + strlen(param_start);
    }

    // Calculate value length
    size_t value_len = param_end - param_start;
    if (value_len >= value_size) {
        value_len = value_size - 1; // Ensure space for null terminator
    }

    // Copy value
    strncpy(value, param_start, value_len);
    value[value_len] = '\0';

    return 0;
}

// Helper function to get form parameter
int get_form_param(const http_request_t *request, const char *name,
                  char *value, size_t value_size) {
    if (!request || !name || !value || value_size == 0) {
        return -1;
    }

    if (!request->body || request->content_length == 0 ||
        strcmp(request->content_type, "application/x-www-form-urlencoded") != 0) {
        return -1;
    }

    char body_copy[4096];
    size_t copy_size = request->content_length < sizeof(body_copy) - 1 ?
                       request->content_length : sizeof(body_copy) - 1;

    memcpy(body_copy, request->body, copy_size);
    body_copy[copy_size] = '\0';

    char *token = strtok(body_copy, "&");
    while (token) {
        char *eq = strchr(token, '=');
        if (eq) {
            *eq = '\0';
            if (strcmp(token, name) == 0) {
                char *val = eq + 1;
                url_decode(val, value, value_size);
                return 0;
            }
        }
        token = strtok(NULL, "&");
    }

    return -1;
}

// URL decode function
int url_decode(const char *src, char *dst, size_t dst_size) {
    size_t src_len = strlen(src);
    size_t i, j = 0;

    for (i = 0; i < src_len && j < dst_size - 1; i++) {
        if (src[i] == '%' && i + 2 < src_len) {
            int value;
            if (sscanf(src + i + 1, "%2x", &value) == 1) {
                dst[j++] = value;
                i += 2;
            } else {
                dst[j++] = src[i];
            }
        } else if (src[i] == '+') {
            dst[j++] = ' ';
        } else {
            dst[j++] = src[i];
        }
    }

    dst[j] = '\0';
    return 0;
}