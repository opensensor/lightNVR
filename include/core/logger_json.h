/**
 * @file logger_json.h
 * @brief JSON logger for structured log output
 */

#ifndef LOGGER_JSON_H
#define LOGGER_JSON_H

#include <time.h>
#include "core/logger.h"

/**
 * @brief Initialize the JSON logger
 * 
 * @param filename Path to the JSON log file
 * @return int 0 on success, non-zero on error
 */
int init_json_logger(const char *filename);

/**
 * @brief Shutdown the JSON logger
 */
void shutdown_json_logger(void);

/**
 * @brief Write a log entry to the JSON log file
 * 
 * @param level Log level
 * @param timestamp Timestamp string
 * @param message Log message
 * @return int 0 on success, non-zero on error
 */
int write_json_log(log_level_t level, const char *timestamp, const char *message);

/**
 * @brief Get logs from the JSON log file with timestamp-based pagination
 * 
 * @param min_level Minimum log level to include
 * @param last_timestamp Last timestamp received by client (for pagination)
 * @param logs Pointer to array of log entries (will be allocated)
 * @param count Pointer to store number of logs
 * @return int 0 on success, non-zero on error
 */
int get_json_logs(const char *min_level, const char *last_timestamp, char ***logs, int *count);

/**
 * @brief Rotate JSON log file if it exceeds a certain size
 * 
 * @param max_size Maximum file size in bytes
 * @param max_files Maximum number of rotated files to keep
 * @return int 0 on success, non-zero on error
 */
int json_log_rotate(size_t max_size, int max_files);

#endif /* LOGGER_JSON_H */
