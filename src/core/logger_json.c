#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <pthread.h>
#include <unistd.h>
#include <sys/stat.h>
#include <sys/types.h>
#include <errno.h>
#include <libgen.h>
#include <time.h>

#include "core/logger.h"
#include "core/logger_json.h"
#include <cjson/cJSON.h>

// JSON logger state
static struct {
    FILE *log_file;
    char log_filename[256];
    pthread_mutex_t mutex;
    int initialized;
} json_logger = {
    .log_file = NULL,
    .log_filename = "",
    .initialized = 0
};

// Log level strings (lowercase for JSON)
static const char *json_log_level_strings[] = {
    "error",
    "warning",
    "info",
    "debug"
};

// Create directory if it doesn't exist (copied from logger.c)
static int create_directory(const char *path) {
    struct stat st;
    
    // Check if directory already exists
    if (stat(path, &st) == 0) {
        if (S_ISDIR(st.st_mode)) {
            return 0; // Directory exists
        } else {
            return -1; // Path exists but is not a directory
        }
    }
    
    // Create directory with permissions 0755
    if (mkdir(path, 0755) != 0) {
        if (errno == ENOENT) {
            // Parent directory doesn't exist, try to create it recursively
            char *parent_path = strdup(path);
            if (!parent_path) {
                return -1;
            }
            
            char *parent_dir = dirname(parent_path);
            int ret = create_directory(parent_dir);
            free(parent_path);
            
            if (ret != 0) {
                return -1;
            }
            
            // Try again to create the directory
            if (mkdir(path, 0755) != 0) {
                return -1;
            }
        } else {
            return -1;
        }
    }
    
    return 0;
}

/**
 * @brief Initialize the JSON logger
 * 
 * @param filename Path to the JSON log file
 * @return int 0 on success, non-zero on error
 */
int init_json_logger(const char *filename) {
    if (!filename) return -1;
    
    // Initialize mutex
    if (pthread_mutex_init(&json_logger.mutex, NULL) != 0) {
        fprintf(stderr, "Failed to initialize JSON logger mutex\n");
        return -1;
    }
    
    pthread_mutex_lock(&json_logger.mutex);
    
    // Create directory for log file if needed
    char *dir_path = strdup(filename);
    if (!dir_path) {
        pthread_mutex_unlock(&json_logger.mutex);
        return -1;
    }
    
    char *dir = dirname(dir_path);
    if (create_directory(dir) != 0) {
        free(dir_path);
        pthread_mutex_unlock(&json_logger.mutex);
        return -1;
    }
    free(dir_path);
    
    // Open log file
    json_logger.log_file = fopen(filename, "a");
    if (!json_logger.log_file) {
        pthread_mutex_unlock(&json_logger.mutex);
        return -1;
    }
    
    // Store filename for potential log rotation
    strncpy(json_logger.log_filename, filename, sizeof(json_logger.log_filename) - 1);
    json_logger.log_filename[sizeof(json_logger.log_filename) - 1] = '\0';
    
    json_logger.initialized = 1;
    
    pthread_mutex_unlock(&json_logger.mutex);
    
    // Write a startup marker to the log file
    time_t now;
    struct tm *tm_info;
    char timestamp[32];
    
    time(&now);
    tm_info = localtime(&now);
    strftime(timestamp, sizeof(timestamp), "%Y-%m-%dT%H:%M:%S", tm_info);
    
    write_json_log(LOG_LEVEL_INFO, timestamp, "JSON logger initialized");
    
    return 0;
}

/**
 * @brief Shutdown the JSON logger
 */
void shutdown_json_logger(void) {
    pthread_mutex_lock(&json_logger.mutex);
    
    if (json_logger.log_file) {
        fclose(json_logger.log_file);
        json_logger.log_file = NULL;
    }
    
    json_logger.initialized = 0;
    
    pthread_mutex_unlock(&json_logger.mutex);
    pthread_mutex_destroy(&json_logger.mutex);
}

/**
 * @brief Write a log entry to the JSON log file
 * 
 * @param level Log level
 * @param timestamp Timestamp string
 * @param message Log message
 * @return int 0 on success, non-zero on error
 */
int write_json_log(log_level_t level, const char *timestamp, const char *message) {
    if (!json_logger.initialized || !json_logger.log_file) {
        return -1;
    }
    
    if (level < LOG_LEVEL_ERROR || level > LOG_LEVEL_DEBUG) {
        return -1;
    }
    
    // Create JSON object for log entry
    cJSON *log_entry = cJSON_CreateObject();
    if (!log_entry) {
        return -1;
    }
    
    // Add timestamp, level, and message
    cJSON_AddStringToObject(log_entry, "timestamp", timestamp);
    cJSON_AddStringToObject(log_entry, "level", json_log_level_strings[level]);
    cJSON_AddStringToObject(log_entry, "message", message);
    
    // Convert to string
    char *json_str = cJSON_PrintUnformatted(log_entry);
    cJSON_Delete(log_entry);
    
    if (!json_str) {
        return -1;
    }
    
    // Write to log file
    pthread_mutex_lock(&json_logger.mutex);
    
    int result = 0;
    if (fprintf(json_logger.log_file, "%s\n", json_str) < 0) {
        result = -1;
    }
    
    fflush(json_logger.log_file);
    
    pthread_mutex_unlock(&json_logger.mutex);
    
    free(json_str);
    
    return result;
}

/**
 * @brief Get logs from the JSON log file with timestamp-based pagination
 * 
 * @param min_level Minimum log level to include
 * @param last_timestamp Last timestamp received by client (for pagination)
 * @param logs Pointer to array of log entries (will be allocated)
 * @param count Pointer to store number of logs
 * @return int 0 on success, non-zero on error
 */
int get_json_logs(const char *min_level, const char *last_timestamp, char ***logs, int *count) {
    if (!json_logger.initialized || !json_logger.log_file) {
        return -1;
    }
    
    // Initialize output parameters
    *logs = NULL;
    *count = 0;
    
    // Convert min_level to numeric value for comparison
    int min_value = 2; // Default to INFO (2)
    
    if (strcmp(min_level, "error") == 0) {
        min_value = 0;
    } else if (strcmp(min_level, "warning") == 0) {
        min_value = 1;
    } else if (strcmp(min_level, "info") == 0) {
        min_value = 2;
    } else if (strcmp(min_level, "debug") == 0) {
        min_value = 3;
    }
    
    // Open log file for reading
    FILE *file = fopen(json_logger.log_filename, "r");
    if (!file) {
        return -1;
    }
    
    // Get file size
    fseek(file, 0, SEEK_END);
    long file_size = ftell(file);
    
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
        fclose(file);
        return -1;
    }
    
    // Read log file
    fseek(file, offset, SEEK_SET);
    size_t bytes_read = fread(buffer, 1, read_size, file);
    buffer[bytes_read] = '\0';
    
    // Close file
    fclose(file);
    
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
        free(buffer);
        return -1;
    }
    
    // Split buffer into lines
    char *saveptr;
    char *line = strtok_r(buffer, "\n", &saveptr);
    int log_index = 0;
    
    // For timestamp comparison
    int compare_timestamp = (last_timestamp != NULL && last_timestamp[0] != '\0');
    
    while (line != NULL && log_index < line_count) {
        // Parse JSON log entry
        cJSON *log_entry = cJSON_Parse(line);
        if (log_entry) {
            // Extract timestamp, level, and message
            cJSON *timestamp_json = cJSON_GetObjectItem(log_entry, "timestamp");
            cJSON *level_json = cJSON_GetObjectItem(log_entry, "level");
            cJSON *message_json = cJSON_GetObjectItem(log_entry, "message");
            
            if (timestamp_json && cJSON_IsString(timestamp_json) &&
                level_json && cJSON_IsString(level_json) &&
                message_json && cJSON_IsString(message_json)) {
                
                const char *timestamp = timestamp_json->valuestring;
                const char *level = level_json->valuestring;
                const char *message = message_json->valuestring;
                
                // Skip entries with timestamp <= last_timestamp
                if (compare_timestamp && strcmp(timestamp, last_timestamp) <= 0) {
                    cJSON_Delete(log_entry);
                    line = strtok_r(NULL, "\n", &saveptr);
                    continue;
                }
                
                // Convert level to numeric value for comparison
                int level_value = 2; // Default to INFO (2)
                
                if (strcmp(level, "error") == 0) {
                    level_value = 0;
                } else if (strcmp(level, "warning") == 0) {
                    level_value = 1;
                } else if (strcmp(level, "info") == 0) {
                    level_value = 2;
                } else if (strcmp(level, "debug") == 0) {
                    level_value = 3;
                }
                
                // Only include logs that meet the minimum level
                // IMPORTANT: The logic here is inverted from what you might expect
                // When min_level is "error" (0), we only want to include error logs (0)
                // When min_level is "warning" (1), we want to include error (0) and warning (1)
                // When min_level is "info" (2), we want to include error (0), warning (1), and info (2)
                // When min_level is "debug" (3), we want to include all logs
                if (level_value <= min_value) {
                    // Include this log entry
                    log_lines[log_index] = strdup(line);
                    if (!log_lines[log_index]) {
                        // Free previously allocated lines
                        for (int i = 0; i < log_index; i++) {
                            free(log_lines[i]);
                        }
                        free(log_lines);
                        free(buffer);
                        cJSON_Delete(log_entry);
                        return -1;
                    }
                    
                    log_index++;
                }
            }
            
            cJSON_Delete(log_entry);
        }
        
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
 * @brief Rotate JSON log file if it exceeds a certain size
 * 
 * @param max_size Maximum file size in bytes
 * @param max_files Maximum number of rotated files to keep
 * @return int 0 on success, non-zero on error
 */
int json_log_rotate(size_t max_size, int max_files) {
    if (!json_logger.initialized || json_logger.log_filename[0] == '\0') {
        return -1;
    }
    
    pthread_mutex_lock(&json_logger.mutex);
    
    // Check current log file size
    struct stat st;
    if (stat(json_logger.log_filename, &st) != 0) {
        pthread_mutex_unlock(&json_logger.mutex);
        return -1;
    }
    
    // If file size is less than max_size, do nothing
    if ((size_t)st.st_size < max_size) {
        pthread_mutex_unlock(&json_logger.mutex);
        return 0;
    }
    
    // Close current log file
    if (json_logger.log_file) {
        fclose(json_logger.log_file);
        json_logger.log_file = NULL;
    }
    
    // Rotate log files
    char old_path[512];
    char new_path[512];
    
    // Remove oldest log file if it exists
    snprintf(old_path, sizeof(old_path), "%s.%d", json_logger.log_filename, max_files);
    unlink(old_path);
    
    // Shift existing log files
    for (int i = max_files - 1; i > 0; i--) {
        snprintf(old_path, sizeof(old_path), "%s.%d", json_logger.log_filename, i);
        snprintf(new_path, sizeof(new_path), "%s.%d", json_logger.log_filename, i + 1);
        rename(old_path, new_path);
    }
    
    // Rename current log file
    snprintf(new_path, sizeof(new_path), "%s.1", json_logger.log_filename);
    rename(json_logger.log_filename, new_path);
    
    // Open new log file
    json_logger.log_file = fopen(json_logger.log_filename, "a");
    if (!json_logger.log_file) {
        pthread_mutex_unlock(&json_logger.mutex);
        return -1;
    }
    
    pthread_mutex_unlock(&json_logger.mutex);
    return 0;
}
