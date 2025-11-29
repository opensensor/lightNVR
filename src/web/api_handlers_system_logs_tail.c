/**
 * @file api_handlers_system_logs_tail.c
 * @brief Optimized API handlers for system logs using tail
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
#include <cjson/cJSON.h>

/**
 * @brief Get system logs using tail command
 *
 * @param logs Pointer to array of log strings (will be allocated)
 * @param count Pointer to store number of logs
 * @param max_lines Maximum number of lines to return
 * @return int 0 on success, -1 on failure
 */
int get_system_logs_tail(char ***logs, int *count, int max_lines) {
    // Initialize output parameters
    *logs = NULL;
    *count = 0;

    // Check if log file is set
    if (g_config.log_file[0] == '\0') {
        log_error("Log file not configured");
        return -1;
    }

    // Validate max_lines
    if (max_lines <= 0) {
        max_lines = 500; // Default to 500 lines
    } else if (max_lines > 5000) {
        max_lines = 5000; // Cap at 5000 lines to prevent excessive memory usage
    }

    // Use tail command to get the last N lines
    // We request 2*max_lines to ensure we have enough lines after filtering by log level
    char command[512];
    snprintf(command, sizeof(command), "tail -n %d %s 2>/dev/null", max_lines * 2, g_config.log_file);
    
    FILE *fp = popen(command, "r");
    if (!fp) {
        log_error("Failed to execute tail command: %s", strerror(errno));
        return -1;
    }

    // Count lines first to allocate memory
    int line_count = 0;
    char line_buffer[4096]; // Increased buffer size for long log lines
    
    // Read all lines to count them
    while (fgets(line_buffer, sizeof(line_buffer), fp) != NULL && line_count < max_lines * 2) {
        line_count++;
    }
    
    // If no lines, return empty result
    if (line_count == 0) {
        pclose(fp);
        return 0;
    }
    
    // Allocate array of log strings
    char **log_lines = calloc(line_count, sizeof(char *));
    if (!log_lines) {
        log_error("Failed to allocate memory for log lines");
        pclose(fp);
        return -1;
    }
    
    // Reopen the command to read the lines again
    pclose(fp);
    fp = popen(command, "r");
    if (!fp) {
        log_error("Failed to execute tail command (second time): %s", strerror(errno));
        free(log_lines);
        return -1;
    }
    
    // Read lines and store them
    int log_index = 0;
    while (fgets(line_buffer, sizeof(line_buffer), fp) != NULL && log_index < line_count) {
        // Remove trailing newline
        size_t len = strlen(line_buffer);
        if (len > 0 && line_buffer[len - 1] == '\n') {
            line_buffer[len - 1] = '\0';
        }
        
        // Skip empty lines
        if (line_buffer[0] == '\0') {
            continue;
        }
        
        // Allocate memory for the log line
        log_lines[log_index] = strdup(line_buffer);
        if (!log_lines[log_index]) {
            log_error("Failed to allocate memory for log line");
            
            // Free previously allocated lines
            for (int i = 0; i < log_index; i++) {
                if (log_lines[i]) {
                    free(log_lines[i]);
                }
            }
            free(log_lines);
            pclose(fp);
            return -1;
        }
        
        log_index++;
    }
    
    // Close the command
    pclose(fp);
    
    // Set output parameters
    *logs = log_lines;
    *count = log_index;
    
    return 0;
}

/**
 * @brief Get JSON logs using tail command
 *
 * @param min_level Minimum log level to include
 * @param last_timestamp Last timestamp received by client (for pagination)
 * @param logs Pointer to array of log entries (will be allocated)
 * @param count Pointer to store number of logs
 * @return int 0 on success, -1 on error
 */
int get_json_logs_tail(const char *min_level, const char *last_timestamp, char ***logs, int *count) {
    // Initialize output parameters
    *logs = NULL;
    *count = 0;
    
    // Check if log file is set
    if (g_config.log_file[0] == '\0') {
        log_error("Log file not configured");
        return -1;
    }
    
    // Default to 500 lines if not specified
    int max_lines = 500;
    
    // Use tail command to get the last N lines
    // We request 2*max_lines to ensure we have enough lines after filtering by log level
    char command[512];
    snprintf(command, sizeof(command), "tail -n %d %s 2>/dev/null", max_lines * 2, g_config.log_file);
    
    FILE *fp = popen(command, "r");
    if (!fp) {
        log_error("Failed to execute tail command: %s", strerror(errno));
        return -1;
    }
    
    // Allocate initial array of log strings
    // We'll resize it if needed
    int capacity = 100;
    char **log_lines = calloc(capacity, sizeof(char *));
    if (!log_lines) {
        log_error("Failed to allocate memory for log lines");
        pclose(fp);
        return -1;
    }
    
    // Read lines and store them
    int log_index = 0;
    char line_buffer[4096]; // Increased buffer size for long log lines
    
    while (fgets(line_buffer, sizeof(line_buffer), fp) != NULL && log_index < max_lines) {
        // Remove trailing newline
        size_t len = strlen(line_buffer);
        if (len > 0 && line_buffer[len - 1] == '\n') {
            line_buffer[len - 1] = '\0';
        }
        
        // Skip empty lines
        if (line_buffer[0] == '\0') {
            continue;
        }

        // Parse log line (format: [TIMESTAMP] [LEVEL] MESSAGE)
        char timestamp[32] = "";
        char level[16] = "";
        char *message = line_buffer;

        // Extract timestamp and level if line starts with [
        if (line_buffer[0] == '[') {
            char *timestamp_end = strchr(line_buffer + 1, ']');
            if (timestamp_end) {
                size_t timestamp_len = timestamp_end - (line_buffer + 1);
                if (timestamp_len < sizeof(timestamp)) {
                    memcpy(timestamp, line_buffer + 1, timestamp_len);
                    timestamp[timestamp_len] = '\0';

                    // Skip space after timestamp
                    char *level_start = timestamp_end + 2;
                    if (level_start[0] == '[') {
                        char *level_end = strchr(level_start + 1, ']');
                        if (level_end) {
                            size_t level_len = level_end - (level_start + 1);
                            if (level_len < sizeof(level)) {
                                memcpy(level, level_start + 1, level_len);
                                level[level_len] = '\0';
                                message = level_end + 2;
                            }
                        }
                    }
                }
            }
        }

        // Skip if timestamp filtering is enabled and this log is older
        if (last_timestamp && last_timestamp[0] && strcmp(timestamp, last_timestamp) <= 0) {
            continue;
        }

        // Skip if doesn't meet minimum level
        if (!log_level_meets_minimum(level, min_level)) {
            continue;
        }
        
        // Check if we need to resize the array
        if (log_index >= capacity) {
            capacity *= 2;
            char **new_lines = realloc(log_lines, capacity * sizeof(char *));
            if (!new_lines) {
                log_error("Failed to resize log lines array");
                
                // Free previously allocated lines
                for (int i = 0; i < log_index; i++) {
                    if (log_lines[i]) {
                        free(log_lines[i]);
                    }
                }
                free(log_lines);
                pclose(fp);
                return -1;
            }
            log_lines = new_lines;
        }

        // Create JSON format log entry using cJSON to properly escape strings
        cJSON *log_entry = cJSON_CreateObject();
        if (!log_entry) {
            log_error("Failed to create JSON object for log entry");

            // Free previously allocated lines
            for (int i = 0; i < log_index; i++) {
                if (log_lines[i]) {
                    free(log_lines[i]);
                }
            }
            free(log_lines);
            pclose(fp);
            return -1;
        }

        cJSON_AddStringToObject(log_entry, "timestamp", timestamp[0] ? timestamp : "Unknown");
        cJSON_AddStringToObject(log_entry, "level", level[0] ? level : "info");
        cJSON_AddStringToObject(log_entry, "message", message);

        char *json_str = cJSON_PrintUnformatted(log_entry);
        cJSON_Delete(log_entry);

        if (!json_str) {
            log_error("Failed to convert log entry to JSON string");

            // Free previously allocated lines
            for (int i = 0; i < log_index; i++) {
                if (log_lines[i]) {
                    free(log_lines[i]);
                }
            }
            free(log_lines);
            pclose(fp);
            return -1;
        }

        // Store the JSON string (already allocated by cJSON)
        log_lines[log_index] = json_str;
        log_index++;
    }
    
    // Close the command
    pclose(fp);
    
    // Set output parameters
    *logs = log_lines;
    *count = log_index;
    
    return 0;
}
