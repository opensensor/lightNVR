/**
 * @file api_handlers_system_logs.c
 * @brief API handlers for system logs
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <ctype.h>
#include <unistd.h>
#include <fcntl.h>
#include <errno.h>

#include "web/api_handlers.h"
#include "web/mongoose_adapter.h"
#include "web/api_handlers_system_ws.h"
#include "core/logger.h"
#include "core/config.h"
#include "mongoose.h"

// Forward declarations
static int log_level_meets_minimum(const char *log_level, const char *min_level);

/**
 * @brief Get system logs
 * 
 * @param logs Pointer to array of log strings (will be allocated)
 * @param count Pointer to store number of logs
 */
void get_system_logs(char ***logs, int *count) {
    // Initialize output parameters
    *logs = NULL;
    *count = 0;
    
    // Check if log file is set
    if (g_config.log_file[0] == '\0') {
        log_error("Log file not configured");
        return;
    }
    
    // Open log file
    FILE *log_file = fopen(g_config.log_file, "r");
    if (!log_file) {
        log_error("Failed to open log file: %s", g_config.log_file);
        return;
    }
    
    // Get file size
    fseek(log_file, 0, SEEK_END);
    long file_size = ftell(log_file);
    
    // Limit to last 100KB if file is larger
    const long max_size = 100 * 1024;
    long read_size = file_size;
    long offset = 0;
    
    if (file_size > max_size) {
        read_size = max_size;
        offset = file_size - max_size;
    }
    
    // Allocate buffer
    char *buffer = malloc(read_size + 1);
    if (!buffer) {
        log_error("Failed to allocate memory for log file");
        fclose(log_file);
        return;
    }
    
    // Read log file
    fseek(log_file, offset, SEEK_SET);
    size_t bytes_read = fread(buffer, 1, read_size, log_file);
    buffer[bytes_read] = '\0';
    
    // Close file
    fclose(log_file);
    
    // Count number of lines
    int line_count = 0;
    for (size_t i = 0; i < bytes_read; i++) {
        if (buffer[i] == '\n') {
            line_count++;
        }
    }
    
    // Add one more for the last line if it doesn't end with a newline
    if (bytes_read > 0 && buffer[bytes_read - 1] != '\n') {
        line_count++;
    }
    
    // Allocate array of log strings
    char **log_lines = malloc(line_count * sizeof(char *));
    if (!log_lines) {
        log_error("Failed to allocate memory for log lines");
        free(buffer);
        return;
    }
    
    // Split buffer into lines
    char *saveptr;
    char *line = strtok_r(buffer, "\n", &saveptr);
    int log_index = 0;
    
    while (line != NULL && log_index < line_count) {
        // Allocate memory for the log line
        log_lines[log_index] = strdup(line);
        if (!log_lines[log_index]) {
            log_error("Failed to allocate memory for log line");
            
            // Free previously allocated lines
            for (int i = 0; i < log_index; i++) {
                free(log_lines[i]);
            }
            free(log_lines);
            free(buffer);
            return;
        }
        
        log_index++;
        line = strtok_r(NULL, "\n", &saveptr);
    }
    
    // Set output parameters
    *logs = log_lines;
    *count = log_index;
    
    // Free buffer
    free(buffer);
}

/**
 * @brief Check if a log level meets the minimum required level
 * 
 * @param log_level The log level to check
 * @param min_level The minimum required level
 * @return int 1 if the log level meets the minimum, 0 otherwise
 */
static int log_level_meets_minimum(const char *log_level, const char *min_level) {
    // Convert log levels to numeric values for comparison
    int level_value = 2; // Default to INFO (2)
    int min_value = 2;   // Default to INFO (2)
    
    // Map log level strings to numeric values
    // ERROR = 0, WARNING = 1, INFO = 2, DEBUG = 3
    if (strcmp(log_level, "error") == 0) {
        level_value = 0;
    } else if (strcmp(log_level, "warning") == 0) {
        level_value = 1;
    } else if (strcmp(log_level, "info") == 0) {
        level_value = 2;
    } else if (strcmp(log_level, "debug") == 0) {
        level_value = 3;
    }
    
    if (strcmp(min_level, "error") == 0) {
        min_value = 0;
    } else if (strcmp(min_level, "warning") == 0) {
        min_value = 1;
    } else if (strcmp(min_level, "info") == 0) {
        min_value = 2;
    } else if (strcmp(min_level, "debug") == 0) {
        min_value = 3;
    }
    
    // IMPORTANT: The logic here is inverted from what you might expect
    // When min_level is "error" (0), we only want to include error logs (0)
    // When min_level is "warning" (1), we want to include error (0) and warning (1)
    // When min_level is "info" (2), we want to include error (0), warning (1), and info (2)
    // When min_level is "debug" (3), we want to include all logs
    
    // So we return true if the log level value is LESS THAN OR EQUAL TO the minimum level value
    return level_value <= min_value;
}

/**
 * @brief Direct handler for GET /api/system/logs
 */
void mg_handle_get_system_logs(struct mg_connection *c, struct mg_http_message *hm) {
    log_info("Handling GET /api/system/logs request");
    
    // Get query parameters
    char level[16] = "info"; // Default to info level
    
    // Extract log level from query parameters
    struct mg_str query = mg_str_n(mg_str_get_ptr(&hm->query), mg_str_get_len(&hm->query));
    char level_buf[16] = {0};
    int level_len = mg_http_get_var(&query, "level", level_buf, sizeof(level_buf) - 1);
    if (level_len > 0) {
        strncpy(level, level_buf, sizeof(level) - 1);
        level[sizeof(level) - 1] = '\0';
    }
    
    // Get system logs
    char **logs = NULL;
    int count = 0;
    
    get_system_logs(&logs, &count);
    
    if (!logs) {
        mg_send_json_error(c, 500, "Failed to get system logs");
        return;
    }
    
    // Create JSON object
    cJSON *logs_obj = cJSON_CreateObject();
    if (!logs_obj) {
        log_error("Failed to create logs JSON object");
        
        // Free logs
        for (int i = 0; i < count; i++) {
            free(logs[i]);
        }
        free(logs);
        
        mg_send_json_error(c, 500, "Failed to create logs JSON");
        return;
    }
    
    // Create logs array
    cJSON *logs_array = cJSON_CreateArray();
    if (!logs_array) {
        log_error("Failed to create logs array");
        
        // Free logs
        for (int i = 0; i < count; i++) {
            free(logs[i]);
        }
        free(logs);
        
        cJSON_Delete(logs_obj);
        mg_send_json_error(c, 500, "Failed to create logs array");
        return;
    }
    
    // Parse and add logs to array as objects (similar to WebSocket implementation)
    for (int i = 0; i < count; i++) {
        if (logs[i] != NULL) {
            // Parse log line (format: [TIMESTAMP] [LEVEL] MESSAGE)
            char timestamp[32] = "Unknown";
            char log_level[16] = "info";
            char *message = logs[i];
            
            // First, check if this is a standard format log
            if (logs[i][0] == '[') {
                char *timestamp_end = strchr(logs[i] + 1, ']');
                if (timestamp_end) {
                    size_t timestamp_len = timestamp_end - (logs[i] + 1);
                    if (timestamp_len < sizeof(timestamp)) {
                        memcpy(timestamp, logs[i] + 1, timestamp_len);
                        timestamp[timestamp_len] = '\0';
                        
                        // Look for level
                        char *level_start = strchr(timestamp_end + 1, '[');
                        if (level_start) {
                            char *level_end = strchr(level_start + 1, ']');
                            if (level_end) {
                                size_t level_len = level_end - (level_start + 1);
                                if (level_len < sizeof(log_level)) {
                                    memcpy(log_level, level_start + 1, level_len);
                                    log_level[level_len] = '\0';
                                    
                                    // Convert to lowercase for consistency
                                    for (int j = 0; log_level[j]; j++) {
                                        log_level[j] = tolower(log_level[j]);
                                    }
                                    
                                    // Message starts after level
                                    message = level_end + 1;
                                    if (message && *message) {
                                        while (*message == ' ') message++; // Skip leading spaces
                                    } else {
                                        message = ""; // Set to empty string if NULL or empty
                                    }
                                }
                            }
                        }
                    }
                }
            } 
            // Check for alternative format: TIMESTAMP LEVEL MESSAGE
            else if (isdigit(logs[i][0])) {
                // Try to parse YYYY-MM-DD HH:MM:SS format
                if (strlen(logs[i]) > 19 && logs[i][4] == '-' && logs[i][7] == '-' && logs[i][10] == ' ' && logs[i][13] == ':' && logs[i][16] == ':') {
                    strncpy(timestamp, logs[i], 19);
                    timestamp[19] = '\0';
                    
                    // Extract level - assume it follows timestamp immediately
                    char *level_start = logs[i] + 19;
                    while (*level_start == ' ') level_start++; // Skip spaces
                    
                    // Find end of level (next space)
                    char *level_end = strchr(level_start, ' ');
                    if (level_end) {
                        size_t level_len = level_end - level_start;
                        if (level_len < sizeof(log_level)) {
                            memcpy(log_level, level_start, level_len);
                            log_level[level_len] = '\0';
                            
                            // Convert to lowercase for consistency
                            for (int j = 0; log_level[j]; j++) {
                                log_level[j] = tolower(log_level[j]);
                            }
                            
                            // Message starts after level
                            message = level_end + 1;
                            if (message && *message) {
                                while (*message == ' ') message++; // Skip leading spaces
                            } else {
                                message = ""; // Set to empty string if NULL or empty
                            }
                        }
                    }
                }
            }
            
            // Normalize log level
            if (strcmp(log_level, "warn") == 0) {
                strcpy(log_level, "warning");
            }
            
            // Check if this log meets the minimum level
            if (log_level_meets_minimum(log_level, level)) {
                // Create log entry object
                cJSON *log_entry = cJSON_CreateObject();
                if (log_entry) {
                    cJSON_AddStringToObject(log_entry, "timestamp", timestamp);
                    cJSON_AddStringToObject(log_entry, "level", log_level);
                    cJSON_AddStringToObject(log_entry, "message", message);
                    
                    cJSON_AddItemToArray(logs_array, log_entry);
                }
            }
        }
    }
    
    // Add logs array to response
    cJSON_AddItemToObject(logs_obj, "logs", logs_array);
    
    // Add metadata
    cJSON_AddStringToObject(logs_obj, "file", g_config.log_file);
    cJSON_AddStringToObject(logs_obj, "level", level);
    
    // Convert to string
    char *json_str = cJSON_PrintUnformatted(logs_obj);
    
    // Clean up
    for (int i = 0; i < count; i++) {
        free(logs[i]);
    }
    free(logs);
    cJSON_Delete(logs_obj);
    
    if (!json_str) {
        log_error("Failed to convert logs JSON to string");
        mg_send_json_error(c, 500, "Failed to convert logs JSON to string");
        return;
    }
    
    // Send response
    mg_send_json_response(c, 200, json_str);
    
    // Clean up
    free(json_str);
    
    log_info("Successfully handled GET /api/system/logs request");
}

/**
 * @brief Direct handler for POST /api/system/logs/clear
 */
void mg_handle_post_system_logs_clear(struct mg_connection *c, struct mg_http_message *hm) {
    log_info("Handling POST /api/system/logs/clear request");
    
    // Get log file path
    const char* log_file = "/var/log/lightnvr.log"; // Default log file path
    const char* fallback_log_file = "./lightnvr.log"; // Fallback log file in current directory
    
    // Check if config has a log file path
    if (g_config.log_file[0] != '\0') {
        log_file = g_config.log_file;
    }
    
    // Check if log file exists and is writable
    struct stat st;
    bool file_exists = (stat(log_file, &st) == 0);
    bool is_writable = (access(log_file, W_OK) == 0);
    
    // If the log file doesn't exist or isn't writable, try the fallback
    if (!file_exists || !is_writable) {
        log_info("Primary log file not accessible, trying fallback: %s", fallback_log_file);
        file_exists = (stat(fallback_log_file, &st) == 0);
        is_writable = (access(fallback_log_file, W_OK) == 0);
        
        if (file_exists || is_writable) {
            log_info("Using fallback log file for clearing");
            log_file = fallback_log_file;
        }
    }
    
    // Clear the log file by truncating it
    int fd = open(log_file, O_WRONLY | O_TRUNC | O_CREAT, 0644);
    if (fd >= 0) {
        close(fd);
        log_info("Log file cleared via API: %s", log_file);
        
        // Create success response using cJSON
        cJSON *success = cJSON_CreateObject();
        if (!success) {
            log_error("Failed to create success JSON object");
            mg_send_json_error(c, 500, "Failed to create success JSON");
            return;
        }
        
        cJSON_AddBoolToObject(success, "success", true);
        cJSON_AddStringToObject(success, "message", "Logs cleared successfully");
        
        // Convert to string
        char *json_str = cJSON_PrintUnformatted(success);
        if (!json_str) {
            log_error("Failed to convert success JSON to string");
            cJSON_Delete(success);
            mg_send_json_error(c, 500, "Failed to convert success JSON to string");
            return;
        }
        
        // Send response
        mg_send_json_response(c, 200, json_str);
        
        // Clean up
        free(json_str);
        cJSON_Delete(success);
    } else {
        log_error("Failed to clear log file %s: %s", log_file, strerror(errno));
        
        // Create error response using cJSON
        cJSON *error = cJSON_CreateObject();
        if (!error) {
            log_error("Failed to create error JSON object");
            mg_send_json_error(c, 500, "Failed to create error JSON");
            return;
        }
        
        cJSON_AddBoolToObject(error, "success", false);
        cJSON_AddStringToObject(error, "message", "Failed to clear logs");
        
        // Convert to string
        char *json_str = cJSON_PrintUnformatted(error);
        if (!json_str) {
            log_error("Failed to convert error JSON to string");
            cJSON_Delete(error);
            mg_send_json_error(c, 500, "Failed to convert error JSON to string");
            return;
        }
        
        // Send response
        mg_send_json_error(c, 500, json_str);
        
        // Clean up
        free(json_str);
        cJSON_Delete(error);
    }
    
    log_info("Successfully handled POST /api/system/logs/clear request");
}
