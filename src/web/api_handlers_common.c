#define _XOPEN_SOURCE
#define _GNU_SOURCE

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <ctype.h>

#include "web/api_handlers_common.h"
#include "core/config.h"
#include "core/logger.h"

/**
 * Append to a dynamically allocated string
 * Returns 1 on success, 0 on failure
 */
static int append_to_string(char **str, size_t *size, size_t *capacity, const char *format, ...) {
    va_list args;
    va_start(args, format);

    // Calculate required space
    va_list args_copy;
    va_copy(args_copy, args);
    int required = vsnprintf(NULL, 0, format, args_copy);
    va_end(args_copy);

    if (required < 0) {
        va_end(args);
        return 0;
    }

    // Ensure we have space
    size_t new_size = *size + required + 1;
    if (new_size > *capacity) {
        size_t new_capacity = *capacity * 2;
        if (new_capacity < new_size) {
            new_capacity = new_size + 256;  // Some extra space
        }

        char *new_str = realloc(*str, new_capacity);
        if (!new_str) {
            va_end(args);
            return 0;
        }

        *str = new_str;
        *capacity = new_capacity;
    }

    // Append the formatted string
    int written = vsnprintf(*str + *size, *capacity - *size, format, args);
    va_end(args);

    if (written < 0) {
        return 0;
    }

    *size += written;
    return 1;
}

/**
 * Create a JSON string from a config structure
 * Returns a dynamically allocated string that must be freed by the caller
 */
char* create_json_string(const config_t *config) {
    if (!config) {
        return NULL;
    }

    size_t capacity = 1024;  // Initial capacity
    size_t size = 0;
    char *json = malloc(capacity);

    if (!json) {
        return NULL;
    }

    // Start the JSON object
    if (!append_to_string(&json, &size, &capacity, "{\n")) {
        free(json);
        return NULL;
    }

    // Add fields one by one, checking for errors
    if (!append_to_string(&json, &size, &capacity, "  \"log_level\": %d,\n", config->log_level)) {
        free(json);
        return NULL;
    }

    if (!append_to_string(&json, &size, &capacity, "  \"storage_path\": \"%s\",\n", config->storage_path)) {
        free(json);
        return NULL;
    }

    if (!append_to_string(&json, &size, &capacity, "  \"max_storage\": %lu,\n",
                         (unsigned long)(config->max_storage_size / (1024 * 1024 * 1024)))) {
        free(json);
        return NULL;
    }

    if (!append_to_string(&json, &size, &capacity, "  \"retention\": %d,\n", config->retention_days)) {
        free(json);
        return NULL;
    }

    if (!append_to_string(&json, &size, &capacity, "  \"auto_delete\": %s,\n",
                         config->auto_delete_oldest ? "true" : "false")) {
        free(json);
        return NULL;
    }

    if (!append_to_string(&json, &size, &capacity, "  \"web_port\": %d,\n", config->web_port)) {
        free(json);
        return NULL;
    }

    if (!append_to_string(&json, &size, &capacity, "  \"auth_enabled\": %s,\n",
                         config->web_auth_enabled ? "true" : "false")) {
        free(json);
        return NULL;
    }

    if (!append_to_string(&json, &size, &capacity, "  \"username\": \"%s\",\n", config->web_username)) {
        free(json);
        return NULL;
    }

    if (!append_to_string(&json, &size, &capacity, "  \"password\": \"********\",\n")) {
        free(json);
        return NULL;
    }

    if (!append_to_string(&json, &size, &capacity, "  \"buffer_size\": %d,\n", config->buffer_size)) {
        free(json);
        return NULL;
    }

    if (!append_to_string(&json, &size, &capacity, "  \"use_swap\": %s,\n",
                         config->use_swap ? "true" : "false")) {
        free(json);
        return NULL;
    }

    // Last field doesn't have a trailing comma
    if (!append_to_string(&json, &size, &capacity, "  \"swap_size\": %lu\n",
                         (unsigned long)(config->swap_size / (1024 * 1024)))) {
        free(json);
        return NULL;
    }

    // Close the JSON object
    if (!append_to_string(&json, &size, &capacity, "}\n")) {
        free(json);
        return NULL;
    }

    return json;
}

// Helper function to parse a JSON value
char* get_json_string_value(const char *json, const char *key) {
    char search_key[256];
    snprintf(search_key, sizeof(search_key), "\"%s\"", key);
    
    char *key_pos = strstr(json, search_key);
    if (!key_pos) {
        return NULL;
    }
    
    // Move past the key and colon
    key_pos += strlen(search_key);
    while (*key_pos && (*key_pos == ' ' || *key_pos == ':')) {
        key_pos++;
    }
    
    // Check if it's a string value
    if (*key_pos == '"') {
        key_pos++; // Skip opening quote
        
        // Find closing quote
        char *end_pos = key_pos;
        while (*end_pos && *end_pos != '"') {
            if (*end_pos == '\\' && *(end_pos + 1)) {
                end_pos += 2; // Skip escaped character
            } else {
                end_pos++;
            }
        }
        
        if (*end_pos == '"') {
            size_t len = end_pos - key_pos;
            char *value = malloc(len + 1);
            if (value) {
                strncpy(value, key_pos, len);
                value[len] = '\0';
                return value;
            }
        }
    }
    
    return NULL;
}

// Helper function to get a boolean value from JSON
int get_json_boolean_value(const char *json, const char *key, int default_value) {
    char search_key[256];
    snprintf(search_key, sizeof(search_key), "\"%s\"", key);
    
    char *key_pos = strstr(json, search_key);
    if (!key_pos) {
        return default_value;
    }
    
    // Move past the key and colon
    key_pos += strlen(search_key);
    while (*key_pos && (*key_pos == ' ' || *key_pos == ':')) {
        key_pos++;
    }
    
    if (strncmp(key_pos, "true", 4) == 0) {
        return 1;
    } else if (strncmp(key_pos, "false", 5) == 0) {
        return 0;
    }
    
    return default_value;
}

// Helper function to get an integer value from JSON
long long get_json_integer_value(const char *json, const char *key, long long default_value) {
    char search_key[256];
    snprintf(search_key, sizeof(search_key), "\"%s\"", key);
    
    char *key_pos = strstr(json, search_key);
    if (!key_pos) {
        return default_value;
    }
    
    // Move past the key and colon
    key_pos += strlen(search_key);
    while (*key_pos && (*key_pos == ' ' || *key_pos == ':')) {
        key_pos++;
    }
    
    // Check if it's a number
    if (isdigit(*key_pos) || *key_pos == '-') {
        return strtoll(key_pos, NULL, 10);
    }
    
    return default_value;
}

/**
 * Create error response with appropriate content type
 */
void create_stream_error_response(http_response_t *response, int status_code, const char *message) {
    char error_json[512];
    snprintf(error_json, sizeof(error_json), "{\"error\": \"%s\"}", message);

    response->status_code = status_code;

    // Use strncpy for the content type field which appears to be an array
    strncpy(response->content_type, "application/json", sizeof(response->content_type) - 1);
    response->content_type[sizeof(response->content_type) - 1] = '\0';

    // Allocate and set body
    response->body = strdup(error_json);

    // Add proper cleanup handling - your response structure likely has a different flag
    // for indicating the body should be freed
    // Check if your http_response_t has a needs_free or similar field
    // response->needs_free = 1;
}
