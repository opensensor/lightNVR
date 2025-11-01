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
#include "core/logger.h"
#include "core/config.h"
#include "mongoose.h"


/**
 * @brief Get system logs
 *
 * @param logs Pointer to array of log strings (will be allocated)
 * @param count Pointer to store number of logs
 * @return int 0 on success, -1 on failure
 */
int get_system_logs(char ***logs, int *count) {
    // Initialize output parameters
    *logs = NULL;
    *count = 0;

    // Check if log file is set
    if (g_config.log_file[0] == '\0') {
        log_error("Log file not configured");
        return -1;
    }

    // Open log file
    FILE *log_file = fopen(g_config.log_file, "r");
    if (!log_file) {
        log_error("Failed to open log file: %s", g_config.log_file);
        return -1;
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
        return -1;
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

    // If no lines, return empty result
    if (line_count == 0) {
        free(buffer);
        *logs = NULL;
        *count = 0;
        return 0;
    }

    // Limit the maximum number of lines to prevent excessive memory usage
    const int max_lines = 500;
    if (line_count > max_lines) {
        log_info("Limiting log lines from %d to %d to prevent excessive memory usage", line_count, max_lines);
        line_count = max_lines;
    }

    // Allocate array of log strings
    char **log_lines = calloc(line_count, sizeof(char *));
    if (!log_lines) {
        log_error("Failed to allocate memory for log lines");
        free(buffer);
        return -1;
    }

    // Split buffer into lines
    char *saveptr;
    char *line = strtok_r(buffer, "\n", &saveptr);
    int log_index = 0;

    while (line != NULL && log_index < line_count) {
        // Skip empty lines
        if (*line == '\0') {
            line = strtok_r(NULL, "\n", &saveptr);
            continue;
        }

        // Allocate memory for the log line
        log_lines[log_index] = strdup(line);
        if (!log_lines[log_index]) {
            log_error("Failed to allocate memory for log line");

            // Free previously allocated lines
            for (int i = 0; i < line_count; i++) {
                if (log_lines[i]) {
                    free(log_lines[i]);
                    log_lines[i] = NULL;
                }
            }
            free(log_lines);
            free(buffer);
            return -1;
        }

        log_index++;
        line = strtok_r(NULL, "\n", &saveptr);
    }

    // If we didn't read as many lines as expected, adjust the count
    if (log_index < line_count) {
        // Ensure any unused slots are NULL
        for (int i = log_index; i < line_count; i++) {
            log_lines[i] = NULL;
        }
    }

    // Set output parameters
    *logs = log_lines;
    *count = log_index;

    // Free buffer
    free(buffer);

    return 0;
}

/**
 * @brief Check if a log level meets the minimum required level
 *
 * @param log_level The log level to check
 * @param min_level The minimum required level
 * @return int 1 if the log level meets the minimum, 0 otherwise
 */
int log_level_meets_minimum(const char *log_level, const char *min_level) {
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
    char level[16] = "debug";

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
    int count = 250;

    const int result = get_json_logs_tail(level, NULL, &logs, &count);

    if (result != 0 || !logs) {
        mg_send_json_error(c, 500, "Failed to get system logs");
        return;
    }

    // Create JSON object
    cJSON *logs_obj = cJSON_CreateObject();
    if (!logs_obj) {
        log_error("Failed to create logs JSON object");

        // Free logs
        if (logs) {
            for (int i = 0; i < count; i++) {
                if (logs[i]) {
                    free(logs[i]);
                }
            }
            free(logs);
        }

        mg_send_json_error(c, 500, "Failed to create logs JSON");
        return;
    }

    // Create logs array
    cJSON *logs_array = cJSON_CreateArray();
    if (!logs_array) {
        log_error("Failed to create logs array");

        // Free logs
        if (logs) {
            for (int i = 0; i < count; i++) {
                if (logs[i]) {
                    free(logs[i]);
                }
            }
            free(logs);
        }

        cJSON_Delete(logs_obj);
        mg_send_json_error(c, 500, "Failed to create logs array");
        return;
    }

    // Parse and add logs to array as objects
    // Note: get_json_logs_tail returns JSON-formatted strings, so we need to parse them
    for (int i = 0; i < count; i++) {
        if (logs[i] != NULL) {
            // Parse the JSON string
            cJSON *log_json = cJSON_Parse(logs[i]);
            if (!log_json) {
                log_error("Failed to parse log JSON: %s", logs[i]);
                continue;
            }

            // Extract fields from JSON
            cJSON *timestamp_json = cJSON_GetObjectItem(log_json, "timestamp");
            cJSON *level_json = cJSON_GetObjectItem(log_json, "level");
            cJSON *message_json = cJSON_GetObjectItem(log_json, "message");

            const char *timestamp = timestamp_json && cJSON_IsString(timestamp_json) ? timestamp_json->valuestring : "Unknown";
            const char *log_level = level_json && cJSON_IsString(level_json) ? level_json->valuestring : "info";
            const char *message = message_json && cJSON_IsString(message_json) ? message_json->valuestring : "";

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

            // Clean up parsed JSON
            cJSON_Delete(log_json);
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
    if (logs) {
        for (int i = 0; i < count; i++) {
            if (logs[i]) {
                free(logs[i]);
            }
        }
        free(logs);
    }
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
