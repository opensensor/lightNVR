#define _XOPEN_SOURCE
#define _GNU_SOURCE

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <ctype.h>

#include "web/web_server.h"
#include "core/logger.h"

/**
 * @brief Create a JSON response
 * 
 * @param response HTTP response
 * @param status_code HTTP status code
 * @param json_data JSON data as string
 * @return int 0 on success, non-zero on error
 */
int create_json_response(http_response_t *response, int status_code, const char *json_data) {
    if (!response || !json_data) {
        return -1;
    }
    
    // Set response status code
    response->status_code = status_code;
    
    // Set content type
    strncpy(response->content_type, "application/json", sizeof(response->content_type) - 1);
    response->content_type[sizeof(response->content_type) - 1] = '\0';
    
    // Free any existing response body
    if (response->body) {
        free(response->body);
    }
    
    // Allocate and copy JSON data
    response->body = strdup(json_data);
    if (!response->body) {
        log_error("Failed to allocate memory for response body");
        return -1;
    }
    
    // Set body length
    response->body_length = strlen(json_data);
    
    return 0;
}

/**
 * @brief Create a file response
 * 
 * @param response HTTP response
 * @param status_code HTTP status code
 * @param file_path Path to file
 * @param content_type Content type
 * @return int 0 on success, non-zero on error
 */
int create_file_response(http_response_t *response, int status_code, const char *file_path, const char *content_type) {
    if (!response || !file_path || !content_type) {
        return -1;
    }
    
    // Open file
    FILE *fp = fopen(file_path, "rb");
    if (!fp) {
        log_error("Failed to open file: %s", file_path);
        return -1;
    }
    
    // Get file size
    fseek(fp, 0, SEEK_END);
    long file_size = ftell(fp);
    fseek(fp, 0, SEEK_SET);
    
    // Allocate memory for file content
    void *content = malloc(file_size);
    if (!content) {
        log_error("Failed to allocate memory for file content");
        fclose(fp);
        return -1;
    }
    
    // Read file content
    size_t bytes_read = fread(content, 1, file_size, fp);
    fclose(fp);
    
    if (bytes_read != (size_t)file_size) {
        log_error("Failed to read file content");
        free(content);
        return -1;
    }
    
    // Set response
    response->status_code = status_code;
    strncpy(response->content_type, content_type, sizeof(response->content_type) - 1);
    response->content_type[sizeof(response->content_type) - 1] = '\0';
    
    // Free any existing response body
    if (response->body) {
        free(response->body);
    }
    
    response->body = content;
    response->body_length = file_size;
    
    return 0;
}

/**
 * @brief Create a text response
 * 
 * @param response HTTP response
 * @param status_code HTTP status code
 * @param text Text content
 * @param content_type Content type
 * @return int 0 on success, non-zero on error
 */
int create_text_response(http_response_t *response, int status_code, const char *text, const char *content_type) {
    if (!response || !text || !content_type) {
        return -1;
    }
    
    // Set response status code
    response->status_code = status_code;
    
    // Set content type
    strncpy(response->content_type, content_type, sizeof(response->content_type) - 1);
    response->content_type[sizeof(response->content_type) - 1] = '\0';
    
    // Free any existing response body
    if (response->body) {
        free(response->body);
    }
    
    // Allocate and copy text
    response->body = strdup(text);
    if (!response->body) {
        log_error("Failed to allocate memory for response body");
        return -1;
    }
    
    // Set body length
    response->body_length = strlen(text);
    
    return 0;
}

/**
 * @brief Create a redirect response
 * 
 * @param response HTTP response
 * @param status_code HTTP status code (should be 301, 302, 303, or 307)
 * @param location Redirect location
 * @return int 0 on success, non-zero on error
 */
int create_redirect_response(http_response_t *response, int status_code, const char *location) {
    if (!response || !location) {
        return -1;
    }
    
    // Set response status code
    response->status_code = status_code;
    
    // Set content type
    strncpy(response->content_type, "text/html", sizeof(response->content_type) - 1);
    response->content_type[sizeof(response->content_type) - 1] = '\0';
    
    // Set Location header
    http_header_t *headers = malloc(sizeof(http_header_t));
    if (!headers) {
        log_error("Failed to allocate memory for response headers");
        return -1;
    }
    
    strncpy(headers[0].name, "Location", sizeof(headers[0].name) - 1);
    headers[0].name[sizeof(headers[0].name) - 1] = '\0';
    
    strncpy(headers[0].value, location, sizeof(headers[0].value) - 1);
    headers[0].value[sizeof(headers[0].value) - 1] = '\0';
    
    // Free any existing headers
    if (response->headers) {
        free(response->headers);
    }
    
    response->headers = headers;
    response->num_headers = 1;
    
    // Create a simple HTML body
    char body[256];
    snprintf(body, sizeof(body), 
             "<html><head><title>Redirect</title></head><body>"
             "<h1>Redirect</h1><p>Redirecting to <a href=\"%s\">%s</a></p>"
             "</body></html>", 
             location, location);
    
    // Free any existing response body
    if (response->body) {
        free(response->body);
    }
    
    // Allocate and copy body
    response->body = strdup(body);
    if (!response->body) {
        log_error("Failed to allocate memory for response body");
        return -1;
    }
    
    // Set body length
    response->body_length = strlen(body);
    
    return 0;
}
