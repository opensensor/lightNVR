/**
 * @file api_handlers_system_logs_tail.c
 * @brief Optimized API handlers for system logs
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <ctype.h>
#include <unistd.h>
#include <fcntl.h>
#include <errno.h>

#include "web/api_handlers.h"
#define LOG_COMPONENT "SystemAPI"
#include "core/logger.h"
#include "core/config.h"
#include <cjson/cJSON.h>


/**
 * @brief Get JSON logs by parsing log file
 *
 * @param max_verbosity Maximum log level to include
 * @param last_timestamp Last timestamp received by client (for pagination)
 * @param max_lines Number of log lines to retrieve
 * @return The allocated cJSON array of log entries
 */
cJSON * get_json_logs_tail(int max_verbosity, const char *last_timestamp, int max_lines) {
    // Check if log file is set
    if (g_config.log_file[0] == '\0') {
        log_error("Log file not configured");
        return NULL;
    }
    
    // Default to 500 lines if not specified
    if (max_lines == 0) {
        max_lines = 500;
    }

    // Open the log file directly — no shell or popen needed
    FILE *fp = fopen(g_config.log_file, "r");
    if (!fp) {
        log_error("Failed to open log file %s: %s", g_config.log_file, strerror(errno));
        return NULL;
    }

    // Seek to an approximate position that should contain the last max_lines*2 lines.
    long bytes_needed = (long)(max_lines * 2) * 120;
    if (fseek(fp, 0, SEEK_END) == 0) {
        long file_size = ftell(fp);
        if (file_size > bytes_needed) {
            fseek(fp, -bytes_needed, SEEK_END);
            // Skip forward to the next newline so we start on a clean line boundary
            int ch;
            while ((ch = fgetc(fp)) != EOF && ch != '\n');
        } else {
            (void)fseek(fp, 0, SEEK_SET);
        }
    } else {
        (void)fseek(fp, 0, SEEK_SET);
    }

    // Create logs array
    cJSON *logs_array = cJSON_CreateArray();
    if (!logs_array) {
        log_error("Failed to create logs array");

        fclose(fp);
        return NULL;
    }

    // Read lines and store them
    char line_buffer[4096]; // Increased buffer size for long log lines

    // NOLINTNEXTLINE(clang-analyzer-unix.Stream)
    while (fgets(line_buffer, sizeof(line_buffer), fp) != NULL) {
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
        log_level_t log_level = LOG_LEVEL_INFO;
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
                            char level[16] = "";
                            size_t level_len = level_end - (level_start + 1);
                            if (level_len < sizeof(level)) {
                                memcpy(level, level_start + 1, level_len);
                                level[level_len] = '\0';
                                message = level_end + 2;
                                log_level = parse_log_level_string(level);
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
        if (log_level > max_verbosity) {
            continue;
        }
        
        // Create JSON object log entry
        cJSON *log_entry = cJSON_CreateObject();
        if (!log_entry) {
            log_error("Failed to create JSON object for log entry");

            // Free previously allocated lines
            cJSON_Delete(logs_array);
            fclose(fp);
            return NULL;
        }

        // Reference constant strings rather than allocating for each line
        const char *level = get_log_level_string(log_level);
        cJSON *json_lvl = cJSON_CreateStringReference(level);

        cJSON_AddStringToObject(log_entry, "timestamp", timestamp[0] ? timestamp : "Unknown");
        // json_lvl is already a reference node. Attach that node directly so
        // the parent owns and frees it; AddItemReferenceToObject would create
        // a second node and leak the original one for every log entry.
        cJSON_AddItemToObject(log_entry, "level", json_lvl);
        cJSON_AddStringToObject(log_entry, "message", message);

        cJSON_AddItemToArray(logs_array, log_entry);
    }

    // Close the file
    fclose(fp);

    return logs_array;
}
