/**
 * @file api_handlers_system_logs.c
 * @brief API handlers for system logs
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <strings.h>
#include <ctype.h>
#include <unistd.h>
#include <sys/stat.h>
#include <fcntl.h>
#include <errno.h>
#include <stdbool.h>

#include "web/api_handlers.h"
#include "web/request_response.h"
#include "web/httpd_utils.h"
#define LOG_COMPONENT "SystemAPI"
#include "core/logger.h"
#include "core/config.h"

static const char *DEFAULT_LOG_FILE = "/var/log/lightnvr.log";
static const char *FALLBACK_LOG_FILE = "./lightnvr.log";
static const long MAX_LOG_TAIL_SIZE = 100L * 1024; // 100KB
#define MAX_LOG_LEVEL_LENGTH 16
static const int DEFAULT_MAX_LOG_ENTRIES = 500;

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
 * @brief Direct handler for GET /api/system/logs
 */
void handle_get_system_logs(const http_request_t *req, http_response_t *res) {
    log_info("Handling GET /api/system/logs request");

    // System logs contain sensitive information — restrict to admins
    if (!httpd_authorize_global_action(req, res, AUTHZ_SYSTEM_ADMIN)) {
        return;  // Error response already set
    }

    // Get query parameters
    log_level_t level = LOG_LEVEL_DEBUG;
    int max_lines = DEFAULT_MAX_LOG_ENTRIES;

    char param_buf[32] = {0};
    char *last_ts = NULL;
    // Extract log level from query parameters
    if (http_request_get_query_param(req, "level", param_buf, sizeof(param_buf)) > 0 && param_buf[0]) {
        level = parse_log_level_string(param_buf);
    }
    // Extract requested line count
    if (http_request_get_query_param(req, "count", param_buf, sizeof(param_buf)) > 0 && param_buf[0]) {
        max_lines = atoi(param_buf);
    }
    // Extract last timestamp. Leave the value in param_buf! Must be the last parameter parsed
    if (http_request_get_query_param(req, "last_ts", param_buf, sizeof(param_buf)) > 0 && param_buf[0]) {
        last_ts = param_buf;
    }

    // Create JSON object
    cJSON *logs_obj = cJSON_CreateObject();
    if (!logs_obj) {
        log_error("Failed to create logs JSON object");

        http_response_set_json_error(res, 500, "Failed to create logs JSON");
        return;
    }

    // Retrieve the cJSON array of the last `max_lines` lines after `last_ts`.
    cJSON *logs_array = get_json_logs_tail(level, last_ts, max_lines);

    if (logs_array == NULL) {
        log_error("Failed to get JSON logs");

        http_response_set_json_error(res, 500, "Failed to get system logs");
        cJSON_Delete(logs_obj);
        return;
    }

    // Add logs array to response
    cJSON_AddItemToObject(logs_obj, "logs", logs_array);

    // Add metadata
    cJSON_AddStringToObject(logs_obj, "file", g_config.log_file);
    cJSON_AddStringToObject(logs_obj, "level", get_log_level_string(level));

    // Convert to string
    char *json_str = cJSON_PrintUnformatted(logs_obj);

    // Clean up
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

    // Clearing logs is a destructive admin operation — require admin privileges
    if (!httpd_authorize_global_action(req, res, AUTHZ_SYSTEM_ADMIN)) {
        return;  // Error response already set
    }

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
