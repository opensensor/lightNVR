/**
 * @file api_handlers_system_logs.c
 * @brief API handlers for system logs
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <ctype.h>
#include <unistd.h>
#include <sys/stat.h>
#include <fcntl.h>
#include <errno.h>
#include <stdbool.h>

#include "web/api_handlers.h"
#include "web/request_response.h"
#include "web/httpd_utils.h"
#include "core/logger.h"
#include "core/config.h"

/* Forward declaration: retrieves the most recent JSON log entries filtered by level and source. */
int get_json_logs_tail(const char *level, const char *source, char ***logs, int *log_count);

typedef enum {
    SYSLOG_LEVEL_ERROR   = 0,
    SYSLOG_LEVEL_WARNING = 1,
    SYSLOG_LEVEL_INFO    = 2,
    SYSLOG_LEVEL_DEBUG   = 3
} LogLevel;

static const char *DEFAULT_LOG_FILE = "/var/log/lightnvr.log";
static const char *FALLBACK_LOG_FILE = "./lightnvr.log";
static const long MAX_LOG_TAIL_SIZE = 100 * 1024; // 100KB
#define MAX_LOG_LEVEL_LENGTH 16
static const int DEFAULT_MAX_LOG_ENTRIES = 250;

/**
 * @brief Validate that the log file path does not contain suspicious path traversal components.
 *
 * This is a conservative check that rejects paths containing ".." as a path component.
 */
static bool is_safe_log_path(const char *path);

/**
 * @brief Check if the given path can be opened for writing.
 *
 * This attempts to open the file directly, avoiding TOCTOU issues
 * associated with using access() before open().
 */
static bool can_open_for_write(const char *path) {
    int fd;

    if (!is_safe_log_path(path)) {
        return false;
    }

    fd = open(path, O_WRONLY | O_APPEND | O_CLOEXEC);
    if (fd < 0) {
        return false;
    }

    close(fd);
    return true;
}

static bool is_safe_log_path(const char *path) {
    if (path == NULL || path[0] == '\0') {
        return false;
    }

    /* Reject any occurrence of "/../" */
    if (strstr(path, "/../") != NULL) {
        return false;
    }

    /* Reject paths ending with "/.." */
    size_t len = strlen(path);
    if (len >= 3 && strcmp(path + len - 3, "/..") == 0) {
        return false;
    }

    /* Reject paths starting with "../" */
    if (strncmp(path, "../", 3) == 0) {
        return false;
    }

    /* Reject path that is exactly ".." */
    if (strcmp(path, "..") == 0) {
        return false;
    }

    return true;
}

/**
 * @brief Get system logs
 *
 * Legacy API retained for backward compatibility. New code should use
 * get_json_logs_tail() instead, which is the preferred way of obtaining
 * log data for API responses.
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
    if (fseek(log_file, 0, SEEK_END) != 0) {
        log_error("Failed to seek to end of log file: %s", g_config.log_file);
        fclose(log_file);
        return -1;
    }
    long file_size = ftell(log_file);
    if (file_size < 0) {
        log_error("Failed to determine size of log file: %s", g_config.log_file);
        fclose(log_file);
        return -1;
    }

    // Limit to last 100KB if file is larger
    const long max_size = MAX_LOG_TAIL_SIZE;
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
    if (fseek(log_file, offset, SEEK_SET) != 0) {
        log_error("Failed to seek to offset %ld in log file: %s", offset, g_config.log_file);
        free(buffer);
        fclose(log_file);
        return -1;
    }
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
    int lines_to_allocate = line_count;
    if (lines_to_allocate > max_lines) {
        log_info("Limiting log lines from %d to %d to prevent excessive memory usage", line_count, max_lines);
        lines_to_allocate = max_lines;
    }

    // Allocate array of log strings
    char **log_lines = calloc(lines_to_allocate, sizeof(char *));
    if (!log_lines) {
        log_error("Failed to allocate memory for log lines");
        free(buffer);
        return -1;
    }

    // Split buffer into lines
    char *saveptr;
    char *line = strtok_r(buffer, "\n", &saveptr);
    int log_index = 0;

    while (line != NULL && log_index < lines_to_allocate) {
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
            for (int i = 0; i < log_index; i++) {
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
    // Validate input pointers to avoid passing NULL to strcmp
    if (log_level == NULL || min_level == NULL) {
        log_error("log_level_meets_minimum called with NULL argument: log_level=%p, min_level=%p",
                  (void *)log_level, (void *)min_level);
        return 0;
    }

    // Convert log levels to numeric values for comparison
    int level_value = SYSLOG_LEVEL_INFO; // Default to INFO
    int min_value = SYSLOG_LEVEL_INFO;   // Default to INFO

    // Map log level strings to numeric values
    // ERROR = 0, WARNING = 1, INFO = 2, DEBUG = 3
    if (strcmp(log_level, "error") == 0) {
        level_value = SYSLOG_LEVEL_ERROR;
    } else if (strcmp(log_level, "warning") == 0) {
        level_value = SYSLOG_LEVEL_WARNING;
    } else if (strcmp(log_level, "info") == 0) {
        level_value = SYSLOG_LEVEL_INFO;
    } else if (strcmp(log_level, "debug") == 0) {
        level_value = SYSLOG_LEVEL_DEBUG;
    }

    if (strcmp(min_level, "error") == 0) {
        min_value = SYSLOG_LEVEL_ERROR;
    } else if (strcmp(min_level, "warning") == 0) {
        min_value = SYSLOG_LEVEL_WARNING;
    } else if (strcmp(min_level, "info") == 0) {
        min_value = SYSLOG_LEVEL_INFO;
    } else if (strcmp(min_level, "debug") == 0) {
        min_value = SYSLOG_LEVEL_DEBUG;
    }

    // NOTE: Lower numeric values represent *higher* severity:
    //   error   = SYSLOG_LEVEL_ERROR   (0, most severe)
    //   warning = SYSLOG_LEVEL_WARNING (1)
    //   info    = SYSLOG_LEVEL_INFO    (2)
    //   debug   = SYSLOG_LEVEL_DEBUG   (3, least severe)
    //
    // The min_level parameter represents the least severe messages we still
    // want to include. A log entry should be included if its severity is
    // greater than or equal to this minimum severity (i.e., numerically
    // less than or equal to min_value).
    //
    // Examples:
    //   min_level = "error"  (0) -> include only "error"      (0)
    //   min_level = "warning"(1) -> include "error"(0) and "warning"(1)
    //   min_level = "info"   (2) -> include error(0), warning(1), info(2)
    //   min_level = "debug"  (3) -> include all levels
    //
    // Therefore, we return true if the log level value is LESS THAN OR EQUAL
    // TO the minimum level value.
    return level_value <= min_value;
}

/**
 * @brief Direct handler for GET /api/system/logs
 */
void handle_get_system_logs(const http_request_t *req, http_response_t *res) {
    log_info("Handling GET /api/system/logs request");

    // Get query parameters
    char level[MAX_LOG_LEVEL_LENGTH] = "debug";

    // Extract log level from query parameters
    char level_buf[MAX_LOG_LEVEL_LENGTH] = {0};
    if (http_request_get_query_param(req, "level", level_buf, sizeof(level_buf)) > 0 && level_buf[0]) {
        strncpy(level, level_buf, sizeof(level) - 1);
        level[sizeof(level) - 1] = '\0';
    }

    // Get system logs
    char **logs = NULL;
    // max_log_count is the input (maximum number of logs requested)
    // actual_log_count is the output (actual number of logs returned) by get_json_logs_tail().
    int max_log_count = DEFAULT_MAX_LOG_ENTRIES;  // Input: maximum number of logs to return
    int actual_log_count = max_log_count;         // Will be updated by get_json_logs_tail()

    // Second argument is the optional source/filter/context; NULL means "no specific filter" (use default system logs)
    // Note: actual_log_count is updated by get_json_logs_tail() to the actual number of logs returned.
    const int result = get_json_logs_tail(level, NULL, &logs, &actual_log_count);

    if (result != 0 || !logs) {
        http_response_set_json_error(res, 500, "Failed to get system logs");
        return;
    }

    // Create JSON object
    cJSON *logs_obj = cJSON_CreateObject();
    if (!logs_obj) {
        log_error("Failed to create logs JSON object");

        // Free logs
        if (logs) {
            for (int i = 0; i < actual_log_count; i++) {
                if (logs[i]) {
                    free(logs[i]);
                }
            }
            free(logs);
        }

        http_response_set_json_error(res, 500, "Failed to create logs JSON");
        return;
    }

    // Create logs array
    cJSON *logs_array = cJSON_CreateArray();
    if (!logs_array) {
        log_error("Failed to create logs array");

        // Free logs
        if (logs) {
            for (int i = 0; i < actual_log_count; i++) {
                if (logs[i]) {
                    free(logs[i]);
                }
            }
            free(logs);
        }

        cJSON_Delete(logs_obj);
        http_response_set_json_error(res, 500, "Failed to create logs array");
        return;
    }

    // Parse and add logs to array as objects
    // Note: get_json_logs_tail returns JSON-formatted strings, so we need to parse them
    for (int i = 0; i < actual_log_count; i++) {
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
        for (int i = 0; i < actual_log_count; i++) {
            if (logs[i]) {
                free(logs[i]);
            }
        }
        free(logs);
    }
    cJSON_Delete(logs_obj);

    if (!json_str) {
        log_error("Failed to convert logs JSON to string");
        http_response_set_json_error(res, 500, "Failed to convert logs JSON to string");
        return;
    }

    // Send response
    http_response_set_json(res, 200, json_str);

    // Clean up
    free(json_str);
}

/**
 * @brief Direct handler for POST /api/system/logs/clear
 */
void handle_post_system_logs_clear(const http_request_t *req, http_response_t *res) {
    log_info("Handling POST /api/system/logs/clear request");

    // Get log file path
    const char* log_file = DEFAULT_LOG_FILE; // Default log file path
    const char* fallback_log_file = FALLBACK_LOG_FILE; // Fallback log file in current directory

    // Check if config has a log file path
    if (g_config.log_file[0] != '\0') {
        log_file = g_config.log_file;
    }

    // Validate the selected log file path to prevent path traversal
    if (!is_safe_log_path(log_file)) {
        log_error("Configured log file path is unsafe: %s", log_file);
        // Fall back to a known-safe default log file path
        log_file = fallback_log_file;
        if (!is_safe_log_path(log_file)) {
            log_error("Fallback log file path is also unsafe: %s", log_file);
            http_response_set_json_error(res, 500, "Invalid log file configuration");
            return;
        }
    }

    // Check if log file exists and is writable
    struct stat st;
    bool file_exists = (stat(log_file, &st) == 0);
    bool is_writable = can_open_for_write(log_file);

    // If the log file doesn't exist or isn't writable, try the fallback
    if (!file_exists || !is_writable) {
        log_info("Primary log file not accessible, trying fallback: %s", fallback_log_file);
        if (!is_safe_log_path(fallback_log_file)) {
            log_error("Fallback log file path is unsafe: %s", fallback_log_file);
            http_response_set_json_error(res, 500, "Invalid fallback log file configuration");
            return;
        }

        file_exists = (stat(fallback_log_file, &st) == 0);
        is_writable = can_open_for_write(fallback_log_file);

        if (file_exists && is_writable) {
            log_info("Using fallback log file for clearing");
            log_file = fallback_log_file;
        }
    }

    // Clear the log file by truncating it
    int fd = open(log_file, O_WRONLY | O_TRUNC | O_CREAT | O_CLOEXEC, 0600);
    if (fd >= 0) {
        close(fd);
        log_info("Log file cleared via API: %s", log_file);

        // Create success response using cJSON
        cJSON *success = cJSON_CreateObject();
        if (!success) {
            log_error("Failed to create success JSON object");
            http_response_set_json_error(res, 500, "Failed to create success JSON");
            return;
        }

        cJSON_AddBoolToObject(success, "success", true);
        cJSON_AddStringToObject(success, "message", "Logs cleared successfully");

        // Convert to string
        char *json_str = cJSON_PrintUnformatted(success);
        if (!json_str) {
            log_error("Failed to convert success JSON to string");
            cJSON_Delete(success);
            http_response_set_json_error(res, 500, "Failed to convert success JSON to string");
            return;
        }

        // Send response
        http_response_set_json(res, 200, json_str);

        // Clean up
        free(json_str);
        cJSON_Delete(success);
    } else {
        log_error("Failed to clear log file %s: %s", log_file, strerror(errno));

        // Create error response using cJSON
        cJSON *error = cJSON_CreateObject();
        if (!error) {
            log_error("Failed to create error JSON object");
            http_response_set_json_error(res, 500, "Failed to create error JSON");
            return;
        }

        cJSON_AddBoolToObject(error, "success", false);
        cJSON_AddStringToObject(error, "message", "Failed to clear logs");

        // Convert to string
        char *json_str = cJSON_PrintUnformatted(error);
        if (!json_str) {
            log_error("Failed to convert error JSON to string");
            cJSON_Delete(error);
            http_response_set_json_error(res, 500, "Failed to convert error JSON to string");
            return;
        }

        // Send response
        http_response_set_json(res, 500, json_str);

        // Clean up
        free(json_str);
        cJSON_Delete(error);
    }

    log_info("Successfully handled POST /api/system/logs/clear request");
}
