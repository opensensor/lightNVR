/*
 * SPDX-License-Identifier: GPL-3.0-or-later
 * Copyright (C) 2025-2026 OpenSensor Engineering and LightNVR contributors
 *
 * This file is part of LightNVR <https://github.com/opensensor/lightNVR>.
 */

/* Prevent logger.h's macro layer from redefining our own function bodies. */
#define LOG_DISABLE_CONTEXT_MACROS

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdarg.h>
#include <time.h>
#include <pthread.h>
#include <unistd.h>
#include <fcntl.h>
#include <sys/stat.h>
#include <sys/types.h>
#include <errno.h>
#include <libgen.h>
#include <syslog.h>

#include "core/config.h"
#include "core/logger.h"
#include "core/logger_json.h"
#include "core/path_utils.h"
#include "utils/strings.h"

// Logger state
static struct {
    FILE *log_file;
    log_level_t log_level;
    int console_logging;
    char log_filename[256];
    pthread_mutex_t mutex;
    int syslog_enabled;
    char syslog_ident[64];
    volatile int shutdown;  // Flag to indicate logger is shut down (1) or shutting down (2)
    volatile int initialized;  // Flag to indicate logger is initialized
} logger = {
    .log_file = NULL,
    .log_level = LOG_LEVEL_INFO,
    .console_logging = 1,
    .log_filename = "",
    .mutex = PTHREAD_MUTEX_INITIALIZER,
    .syslog_enabled = 0,
    .syslog_ident = "",
    .shutdown = 0,
    .initialized = 0,
};

// Weak symbols for optional JSON logger (implemented in logger_json.c when linked)
extern __attribute__((weak)) int init_json_logger(const char *filename);
extern __attribute__((weak)) int write_json_log(log_level_t level, const char *timestamp, const char *message);

// -----------------------------------------------------------------------
// Per-thread logging context (thread-local storage)
// Each worker thread calls log_set_thread_context() once at startup.
// log_message_v() reads these to prepend [component] [stream] to lines.
// -----------------------------------------------------------------------
static __thread char tls_log_component[64]  = {0};
static __thread char tls_log_stream[128]    = {0};

void log_set_thread_context(const char *component, const char *stream_name) {
    safe_strcpy(tls_log_component, component ? component : "", sizeof(tls_log_component), 0);
    safe_strcpy(tls_log_stream, stream_name ? stream_name : "", sizeof(tls_log_stream), 0);
}

void log_clear_thread_context(void) {
    tls_log_component[0] = '\0';
    tls_log_stream[0]    = '\0';
}

const char *log_get_thread_component(void) { return tls_log_component; }
const char *log_get_thread_stream(void)    { return tls_log_stream;    }

// Log level strings
static const char *log_level_strings[] = {
    "ERROR",
    "WARN",
    "INFO",
    "DEBUG"
};

// Initialize the logging system
int init_logger(void) {
    // Check if already initialized
    if (logger.initialized) {
        return 0;
    }

    // Reset shutdown flag
    logger.shutdown = 0;

    // Initialize mutex
    if (pthread_mutex_init(&logger.mutex, NULL) != 0) {
        fprintf(stderr, "Failed to initialize logger mutex\n");
        return -1;
    }

    // Default to stderr if no log file is set
    if (logger.log_file == NULL && logger.log_filename[0] == '\0') {
        logger.log_file = stderr;
    }

    // Initialize JSON logger if log file is set and the function is available
    if (logger.log_filename[0] != '\0' && init_json_logger) {
        char json_log_filename[512];
        snprintf(json_log_filename, sizeof(json_log_filename), "%s.json", logger.log_filename);
        init_json_logger(json_log_filename);
    }

    // Mark as initialized
    logger.initialized = 1;

    return 0;
}

// Check if logger is available for use
int is_logger_available(void) {
    return logger.initialized && !logger.shutdown;
}

// Shutdown the logging system
void shutdown_logger(void) {
    // Check if already shutdown or not initialized
    if (logger.shutdown || !logger.initialized) {
        return;
    }

    // Mark as shutting down BEFORE acquiring mutex
    // This prevents new log calls from trying to acquire the mutex
    logger.shutdown = 1;

    // Memory barrier to ensure shutdown flag is visible
    __sync_synchronize();

    // Small delay to allow in-flight log operations to complete
    usleep(10000);  // 10ms

    pthread_mutex_lock(&logger.mutex);

    if (logger.log_file != NULL && logger.log_file != stdout && logger.log_file != stderr) {
        fclose(logger.log_file);
        logger.log_file = NULL;
    }

    // Close syslog if enabled
    if (logger.syslog_enabled) {
        closelog();
        logger.syslog_enabled = 0;
    }

    pthread_mutex_unlock(&logger.mutex);

    // Mark as fully shutdown before destroying mutex
    logger.shutdown = 2;
    logger.initialized = 0;

    // Memory barrier
    __sync_synchronize();

    pthread_mutex_destroy(&logger.mutex);

    // Shutdown JSON logger if the function is available
    extern __attribute__((weak)) void shutdown_json_logger(void);
    if (shutdown_json_logger) {
        shutdown_json_logger();
    }
}

// Set the log level
void set_log_level(log_level_t level) {
    if (level >= LOG_LEVEL_ERROR && level <= LOG_LEVEL_DEBUG) {
        pthread_mutex_lock(&logger.mutex);
        // Store old level for logging
        log_level_t old_level = logger.log_level;
        logger.log_level = level;
        pthread_mutex_unlock(&logger.mutex);

        // Log the change - but only if we're not setting the initial level
        // This avoids a potential recursive call during initialization
        if (old_level != LOG_LEVEL_ERROR || level != LOG_LEVEL_ERROR) {
            // Use fprintf directly to avoid potential recursion with log_* functions
            fprintf(stderr, "[LOG LEVEL CHANGE] %s -> %s\n",
                    log_level_strings[old_level],
                    log_level_strings[level]);
        }
    }
}

// Set the log file
int set_log_file(const char *filename) {
    if (!filename) return -1;

    pthread_mutex_lock(&logger.mutex);

    // Close existing log file if open
    if (logger.log_file != NULL && logger.log_file != stdout && logger.log_file != stderr) {
        fclose(logger.log_file);
        logger.log_file = NULL;
    }

    // Create directory for log file if needed
    char *dir_path = strdup(filename);
    if (!dir_path) {
        pthread_mutex_unlock(&logger.mutex);
        return -1;
    }

    const char *dir = dirname(dir_path);
    if (mkdir_recursive(dir) != 0) {
        free(dir_path);
        pthread_mutex_unlock(&logger.mutex);
        return -1;
    }
    free(dir_path);

    // Open new log file with restricted permissions (0640: owner rw, group r, others none)
    int log_fd = open(filename, O_WRONLY | O_CREAT | O_APPEND, 0640);
    logger.log_file = (log_fd >= 0) ? fdopen(log_fd, "a") : NULL;
    if (!logger.log_file) {
        if (log_fd >= 0) close(log_fd);
        pthread_mutex_unlock(&logger.mutex);
        return -1;
    }

    // Store filename for potential log rotation
    safe_strcpy(logger.log_filename, filename, sizeof(logger.log_filename), 0);

    pthread_mutex_unlock(&logger.mutex);

    // Initialize JSON logger with a corresponding JSON log file if the function is available
    if (init_json_logger) {
        char json_log_filename[512];
        snprintf(json_log_filename, sizeof(json_log_filename), "%s.json", filename);
        init_json_logger(json_log_filename);
    }

    return 0;
}

// Log a message at ERROR level
void log_error(const char *format, ...) {
    va_list args;
    va_start(args, format);
    log_message_v(LOG_LEVEL_ERROR, format, args);
    va_end(args);
}

// Log a message at WARN level
void log_warn(const char *format, ...) {
    va_list args;
    va_start(args, format);
    log_message_v(LOG_LEVEL_WARN, format, args);
    va_end(args);
}

// Log a message at INFO level
void log_info(const char *format, ...) {
    va_list args;
    va_start(args, format);
    log_message_v(LOG_LEVEL_INFO, format, args);
    va_end(args);
}

// Log a message at DEBUG level
void log_debug(const char *format, ...) {
    va_list args;
    va_start(args, format);
    log_message_v(LOG_LEVEL_DEBUG, format, args);
    va_end(args);
}

// Log a message at the specified level
void log_message(log_level_t level, const char *format, ...) {
    va_list args;
    va_start(args, format);
    log_message_v(level, format, args);
    va_end(args);
}

/**
 * Sanitize a string for logging to prevent displaying non-printable characters
 * This function replaces non-printable characters with '?' and ensures the string is properly terminated
 *
 * @param str The string to sanitize
 * @param max_len The maximum length of the string
 * @return A pointer to a static buffer containing the sanitized string
 */
const char *sanitize_for_logging(const char *str, size_t max_len) {
    static char sanitized[4096]; // Static buffer for thread safety concerns, but large enough for most uses
    size_t i;

    if (!str) {
        return "(null)"; // Handle NULL strings
    }

    // Limit to buffer size - 1 (for null terminator)
    if (max_len > sizeof(sanitized) - 1) {
        max_len = sizeof(sanitized) - 1;
    }

    // Copy and sanitize the string
    for (i = 0; i < max_len && str[i] != '\0'; i++) {
        // Check if character is printable (ASCII 32-126 plus tab and newline)
        if ((str[i] >= 32 && str[i] <= 126) || str[i] == '\t' || str[i] == '\n') {
            sanitized[i] = str[i];
        } else {
            sanitized[i] = '?'; // Replace non-printable characters
        }
    }

    // Null-terminate the string
    sanitized[i] = '\0';

    return sanitized;
}

// Internal helper: build "[component] [stream] " prefix into buf (max bufsz bytes).
static void build_ctx_prefix(char *buf, size_t bufsz,
                              const char *component, const char *stream) {
    if (component && component[0]) {
        if (stream && stream[0]) {
            snprintf(buf, bufsz, "[%s] [%s] ", component, stream);
        } else {
            snprintf(buf, bufsz, "[%s] ", component);
        }
    } else {
        buf[0] = '\0';
    }
}

// Core logging implementation shared by log_message_v and _log_message_ctx.
// component and stream are already resolved by the caller; NULL or "" means
// no prefix for that field.
static void do_log_internal(log_level_t level,
                             const char *component, const char *stream,
                             const char *format, va_list args) {
    // Only log messages at or below the configured log level.
    if (level > logger.log_level) {
        return;
    }

    // Create timestamp
    struct timespec now;
    clock_gettime(CLOCK_REALTIME, &now);

    struct tm tm_buf;
    localtime_r(&now.tv_sec, &tm_buf);

    char timestamp[32];
    size_t offset = strftime(timestamp, sizeof(timestamp), "%Y-%m-%d %H:%M:%S", &tm_buf);

    unsigned int msec = (unsigned int)(now.tv_nsec / 1000000UL);
    snprintf(timestamp + offset, sizeof(timestamp) - offset, ".%03u", msec);

    char message[4096];
    vsnprintf(message, sizeof(message), format, args);

    char ctx_prefix[224] = {0};
    build_ctx_prefix(ctx_prefix, sizeof(ctx_prefix), component, stream);

    // CRITICAL: Check if logger is shutting down or destroyed.
    // Write directly to console without the mutex to avoid use-after-destroy.
    if (logger.shutdown) {
        FILE *console = (level == LOG_LEVEL_ERROR) ? stderr : stdout;
        fprintf(console, "[%s] [%s] %s%s\n",
                timestamp, log_level_strings[level], ctx_prefix, message);
        fflush(console);
        return;
    }

    pthread_mutex_lock(&logger.mutex);

    if (logger.log_file && logger.log_file != stdout && logger.log_file != stderr) {
        fprintf(logger.log_file, "[%s] [%s] %s%s\n",
                timestamp, log_level_strings[level], ctx_prefix, message);
        fflush(logger.log_file);
    }

    FILE *console = (level == LOG_LEVEL_ERROR) ? stderr : stdout;
    fprintf(console, "[%s] [%s] %s%s\n",
            timestamp, log_level_strings[level], ctx_prefix, message);
    fflush(console);

    if (logger.syslog_enabled) {
        int syslog_priority;
        switch (level) {
            case LOG_LEVEL_ERROR: syslog_priority = LOG_ERR;     break;
            case LOG_LEVEL_WARN:  syslog_priority = LOG_WARNING; break;
            case LOG_LEVEL_INFO:  syslog_priority = LOG_INFO;    break;
            case LOG_LEVEL_DEBUG: syslog_priority = LOG_DEBUG;   break;
            default:              syslog_priority = LOG_INFO;    break;
        }
        syslog(syslog_priority, "%s", message);
    }

    pthread_mutex_unlock(&logger.mutex);

    if (write_json_log) {
        char iso_timestamp[32];

        strftime(iso_timestamp, sizeof(iso_timestamp), "%Y-%m-%dT%H:%M:%S",  &tm_buf);
        write_json_log(level, iso_timestamp, message);
    }
}

// Log a message at the specified level with va_list.
// Reads component/stream from the calling thread's TLS context.
void log_message_v(log_level_t level, const char *format, va_list args) {
    do_log_internal(level, tls_log_component, tls_log_stream, format, args);
}

// Internal variant called by the log_* macros.
// Prefers TLS context; falls back to (component, stream) when TLS is unset.
void _log_message_ctx(log_level_t level, const char *component, const char *stream,
                      const char *format, ...) {
    // TLS takes priority: it represents the most specific runtime context
    // (set explicitly for a thread or operation via log_set_thread_context).
    // The compile-time LOG_COMPONENT / _log_stream_name macros act as
    // fallbacks for threads that have not set a TLS context.
    const char *eff_comp   = (tls_log_component[0]) ? tls_log_component
                           : (component && component[0]) ? component : "";
    const char *eff_stream = (tls_log_stream[0]) ? tls_log_stream
                           : (stream    && stream[0])    ? stream    : "";
    va_list args;
    va_start(args, format);
    do_log_internal(level, eff_comp, eff_stream, format, args);
    va_end(args);
}

// Get the string representation of a log level
const char *get_log_level_string(log_level_t level) {
    if (level >= LOG_LEVEL_ERROR && level <= LOG_LEVEL_DEBUG) {
        return log_level_strings[level];
    }
    return "UNKNOWN";
}

// Parse the log level string and return the associated enum
log_level_t parse_log_level_string(const char *log_level) {
    int level_value = LOG_LEVEL_INFO; // Default to INFO

    if (log_level == NULL) {
        return level_value;
    }

    // "WARN" (written by the logger) is accepted as an alias for "WARNING".
    // ERROR = 0, WARNING = 1, INFO = 2, DEBUG = 3
    if (strcasecmp(log_level, "error") == 0) {
        level_value = LOG_LEVEL_ERROR;
    } else if (strcasecmp(log_level, "warning") == 0 || strcasecmp(log_level, "warn") == 0) {
        level_value = LOG_LEVEL_WARN;
    } else if (strcasecmp(log_level, "info") == 0) {
        level_value = LOG_LEVEL_INFO;
    } else if (strcasecmp(log_level, "debug") == 0) {
        level_value = LOG_LEVEL_DEBUG;
    }

    return level_value;
}

// Rotate log files if they exceed a certain size
int log_rotate(size_t max_size, int max_files) {
    if (logger.log_filename[0] == '\0') {
        return -1; // No log file set
    }

    pthread_mutex_lock(&logger.mutex);

    // Check current log file size
    struct stat st;
    if (stat(logger.log_filename, &st) != 0) {
        pthread_mutex_unlock(&logger.mutex);
        return -1;
    }

    // If file size is less than max_size, do nothing
    if ((size_t)st.st_size < max_size) {
        pthread_mutex_unlock(&logger.mutex);
        return 0;
    }

    // Close current log file
    if (logger.log_file != NULL && logger.log_file != stdout && logger.log_file != stderr) {
        fclose(logger.log_file);
        logger.log_file = NULL;
    }

    // Rotate log files
    char old_path[MAX_PATH_LENGTH];
    char new_path[MAX_PATH_LENGTH];

    // Remove oldest log file if it exists
    snprintf(old_path, sizeof(old_path), "%s.%d", logger.log_filename, max_files);
    unlink(old_path);

    // Shift existing log files
    for (int i = max_files - 1; i > 0; i--) {
        snprintf(old_path, sizeof(old_path), "%s.%d", logger.log_filename, i);
        snprintf(new_path, sizeof(new_path), "%s.%d", logger.log_filename, i + 1);
        rename(old_path, new_path);
    }

    // Rename current log file
    snprintf(new_path, sizeof(new_path), "%s.1", logger.log_filename);
    rename(logger.log_filename, new_path);

    // Open new log file with restricted permissions (0640: owner rw, group r, others none)
    int rot_fd = open(logger.log_filename, O_WRONLY | O_CREAT | O_APPEND, 0640);
    logger.log_file = (rot_fd >= 0) ? fdopen(rot_fd, "a") : NULL;
    if (!logger.log_file) {
        if (rot_fd >= 0) close(rot_fd);
        pthread_mutex_unlock(&logger.mutex);
        return -1;
    }

    pthread_mutex_unlock(&logger.mutex);

    // Also rotate JSON log file if the function is available
    extern __attribute__((weak)) int json_log_rotate(size_t max_size, int max_files);
    if (json_log_rotate) {
        char json_log_filename[512];
        snprintf(json_log_filename, sizeof(json_log_filename), "%s.json", logger.log_filename);
        json_log_rotate(max_size, max_files);
    }

    return 0;
}

// Enable syslog logging
int enable_syslog(const char *ident, int facility) {
    if (!ident || ident[0] == '\0') {
        return -1;
    }

    pthread_mutex_lock(&logger.mutex);

    // Close existing syslog connection if any
    if (logger.syslog_enabled) {
        closelog();
    }

    // Store the identifier
    safe_strcpy(logger.syslog_ident, ident, sizeof(logger.syslog_ident), 0);

    // Open syslog connection
    // LOG_PID: include PID with each message
    // LOG_CONS: write to console if there's an error writing to syslog
    openlog(logger.syslog_ident, LOG_PID | LOG_CONS, facility);

    logger.syslog_enabled = 1;

    pthread_mutex_unlock(&logger.mutex);

    log_info("Syslog logging enabled (ident: %s, facility: %d)", ident, facility);

    return 0;
}

// Disable syslog logging
void disable_syslog(void) {
    int was_enabled;

    pthread_mutex_lock(&logger.mutex);

    was_enabled = logger.syslog_enabled;
    if (logger.syslog_enabled) {
        closelog();
        logger.syslog_enabled = 0;
    }

    pthread_mutex_unlock(&logger.mutex);

    // Log after releasing the mutex to avoid deadlock
    if (was_enabled) {
        log_info("Syslog logging disabled");
    }
}

// Check if syslog is enabled
int is_syslog_enabled(void) {
    int enabled;
    pthread_mutex_lock(&logger.mutex);
    enabled = logger.syslog_enabled;
    pthread_mutex_unlock(&logger.mutex);
    return enabled;
}
