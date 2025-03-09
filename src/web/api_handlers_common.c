#define _XOPEN_SOURCE
#define _GNU_SOURCE

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <ctype.h>

#include "web/api_handlers_common.h"
#include "core/config.h"
#include "core/logger.h"

// Helper function to create a simple JSON string
char* create_json_string(const config_t *config) {
    char *json = malloc(4096); // Allocate enough space for the JSON
    if (!json) {
        return NULL;
    }
    
    // Format the JSON string manually
    int len = snprintf(json, 4096,
        "{\n"
        "  \"log_level\": %d,\n"
        "  \"storage_path\": \"%s\",\n"
        "  \"max_storage\": %lu,\n"
        "  \"retention\": %d,\n"
        "  \"auto_delete\": %s,\n"
        "  \"web_port\": %d,\n"
        "  \"auth_enabled\": %s,\n"
        "  \"username\": \"%s\",\n"
        "  \"password\": \"********\",\n"
        "  \"buffer_size\": %d,\n"
        "  \"use_swap\": %s,\n"
        "  \"swap_size\": %lu\n"
        "}",
        config->log_level,
        config->storage_path,
        (unsigned long)(config->max_storage_size / (1024 * 1024 * 1024)), // Convert to GB
        config->retention_days,
        config->auto_delete_oldest ? "true" : "false",
        config->web_port,
        config->web_auth_enabled ? "true" : "false",
        config->web_username,
        config->buffer_size,
        config->use_swap ? "true" : "false",
        (unsigned long)(config->swap_size / (1024 * 1024)) // Convert to MB
    );
    
    if (len >= 4096) {
        // Buffer was too small
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
