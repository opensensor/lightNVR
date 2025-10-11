#ifndef LIGHTNVR_LOGGER_H
#define LIGHTNVR_LOGGER_H

#include <stdarg.h>
#include <stddef.h>

// Log levels
// Change your logger.h enum to avoid conflicting with syslog.h
typedef enum {
    LOG_LEVEL_ERROR = 0,
    LOG_LEVEL_WARN  = 1,
    LOG_LEVEL_INFO  = 2,
    LOG_LEVEL_DEBUG = 3
} log_level_t;

/**
 * Initialize the logging system
 * 
 * @return 0 on success, non-zero on failure
 */
int init_logger(void);

/**
 * Shutdown the logging system
 */
void shutdown_logger(void);

/**
 * Set the log level
 * 
 * @param level The log level to set
 */
void set_log_level(log_level_t level);

/**
 * Set the log file
 * 
 * @param filename Path to the log file
 * @return 0 on success, non-zero on failure
 */
int set_log_file(const char *filename);

/**
 * Enable or disable console logging
 * 
 * Note: With tee behavior enabled, console logging is always active
 * This function is kept for API compatibility but has no effect on output
 * 
 * @param enable True to enable console logging, false to disable (no effect with tee behavior)
 */
void set_console_logging(int enable);

/**
 * Log a message at ERROR level
 * 
 * @param format Printf-style format string
 * @param ... Format arguments
 */
void log_error(const char *format, ...);

/**
 * Log a message at WARN level
 * 
 * @param format Printf-style format string
 * @param ... Format arguments
 */
void log_warn(const char *format, ...);

/**
 * Log a message at INFO level
 * 
 * @param format Printf-style format string
 * @param ... Format arguments
 */
void log_info(const char *format, ...);

/**
 * Log a message at DEBUG level
 * 
 * @param format Printf-style format string
 * @param ... Format arguments
 */
void log_debug(const char *format, ...);

/**
 * Log a message at the specified level
 * 
 * @param level Log level
 * @param format Printf-style format string
 * @param ... Format arguments
 */
void log_message(log_level_t level, const char *format, ...);

/**
 * Log a message at the specified level with va_list
 * 
 * @param level Log level
 * @param format Printf-style format string
 * @param args Format arguments as va_list
 */
void log_message_v(log_level_t level, const char *format, va_list args);

/**
 * Rotate log files if they exceed a certain size
 * 
 * @param max_size Maximum size in bytes before rotation
 * @param max_files Maximum number of rotated files to keep
 * @return 0 on success, non-zero on failure
 */
int log_rotate(size_t max_size, int max_files);

/**
 * Get the string representation of a log level
 *
 * @param level The log level
 * @return String representation of the log level, or "UNKNOWN" if invalid
 */
const char *get_log_level_string(log_level_t level);

/**
 * Enable syslog logging
 *
 * @param ident Syslog identifier (application name)
 * @param facility Syslog facility (e.g., LOG_USER, LOG_DAEMON, LOG_LOCAL0-7)
 * @return 0 on success, non-zero on failure
 */
int enable_syslog(const char *ident, int facility);

/**
 * Disable syslog logging
 */
void disable_syslog(void);

/**
 * Check if syslog is enabled
 *
 * @return 1 if syslog is enabled, 0 otherwise
 */
int is_syslog_enabled(void);

#endif // LIGHTNVR_LOGGER_H
