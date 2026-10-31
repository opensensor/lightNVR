/*
 * SPDX-License-Identifier: GPL-3.0-or-later
 * Copyright (C) 2025-2026 OpenSensor Engineering and LightNVR contributors
 *
 * This file is part of LightNVR <https://github.com/opensensor/lightNVR>.
 */

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
 * @brief Map log level strings to numeric values (case-insensitive).
 *
 * @param log_level The log level as a string
 * @return The numeric log_level_t enum, LOG_LEVEL_INFO if no match
 */
log_level_t parse_log_level_string(const char *log_level);

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

/**
 * Check if logger is available for use
 *
 * @return 1 if logger is available, 0 if shutting down or not initialized
 */
int is_logger_available(void);

/* -----------------------------------------------------------------------
 * Compile-time per-translation-unit context macros
 *
 * A source file may set a component label BEFORE including this header:
 *
 *   #define LOG_COMPONENT "WebAPI"
 *   #include "core/logger.h"
 *
 * Every log_* call in that TU will then carry a [WebAPI] prefix whenever
 * the calling thread has no TLS component set.  TLS set via
 * log_set_thread_context() always takes priority over LOG_COMPONENT.
 *
 * A per-scope stream name can be injected by redefining _log_stream_name
 * around the relevant log calls, e.g. inside a function that handles a
 * specific stream:
 *
 *   #undef  _log_stream_name
 *   #define _log_stream_name  stream_name   // local variable
 *   log_info("handling request");
 *   #undef  _log_stream_name
 *   #define _log_stream_name  ((const char *)NULL)   // restore default
 *
 * This keeps _log_stream_name as a pure macro so that -Wshadow warnings
 * are never triggered by the ubiquitous 'stream_name' parameter name.
 * ----------------------------------------------------------------------- */

/**
 * Internal variant called by the log_* macros below.
 *
 * Prefers TLS context (set via log_set_thread_context); falls back to
 * the explicit (component, stream) arguments when TLS is unset.
 *
 * Not intended to be called directly — use the log_* macros instead.
 */
void _log_message_ctx(log_level_t level, const char *component, const char *stream,
                      const char *format, ...)
     __attribute__((format(printf, 4, 5)));

/** Default component: NULL (no prefix).  Override per-file with
 *  #define LOG_COMPONENT "MySubsystem"  before #include "core/logger.h". */
#ifndef LOG_COMPONENT
#  define LOG_COMPONENT ((const char *)NULL)
#endif

/** Default stream: NULL (no stream prefix).
 *  Override per-scope:  #undef _log_stream_name / #define _log_stream_name var */
#ifndef _log_stream_name
#  define _log_stream_name ((const char *)NULL)
#endif

/* Redefine the public log_* names to call _log_message_ctx so that every
 * call site automatically picks up LOG_COMPONENT and _log_stream_name.
 *
 * Define LOG_DISABLE_CONTEXT_MACROS before including this header to opt
 * out (logger.c uses this to protect its own function definitions).       */
#ifndef LOG_DISABLE_CONTEXT_MACROS
#  undef  log_error
#  define log_error(...)        _log_message_ctx(LOG_LEVEL_ERROR, LOG_COMPONENT, \
                                                 _log_stream_name, __VA_ARGS__)
#  undef  log_warn
#  define log_warn(...)         _log_message_ctx(LOG_LEVEL_WARN,  LOG_COMPONENT, \
                                                 _log_stream_name, __VA_ARGS__)
#  undef  log_info
#  define log_info(...)         _log_message_ctx(LOG_LEVEL_INFO,  LOG_COMPONENT, \
                                                 _log_stream_name, __VA_ARGS__)
#  undef  log_debug
#  define log_debug(...)        _log_message_ctx(LOG_LEVEL_DEBUG, LOG_COMPONENT, \
                                                 _log_stream_name, __VA_ARGS__)
#  undef  log_message
#  define log_message(lvl, ...) _log_message_ctx((lvl), LOG_COMPONENT, \
                                                 _log_stream_name, __VA_ARGS__)
#endif /* LOG_DISABLE_CONTEXT_MACROS */

/* -----------------------------------------------------------------------
 * Per-thread logging context
 *
 * Each long-running thread may call log_set_thread_context() once at
 * startup so that every subsequent log_* call from that thread
 * automatically includes a [component] and, when applicable, a
 * [stream_name] field in the log line:
 *
 *   [timestamp] [LEVEL] [component] [stream] message
 *   [timestamp] [LEVEL] [component] message        <- no stream set
 *   [timestamp] [LEVEL] message                    <- no context set
 *
 * The implementation uses __thread storage; no mutex is required.
 * ----------------------------------------------------------------------- */

/**
 * Set the logging context for the current thread.
 *
 * @param component   Short label for the subsystem, e.g. "MP4Writer"
 *                    (max 63 chars, copied into thread-local storage).
 *                    Pass NULL or "" to clear.
 * @param stream_name Name of the stream this thread is handling, e.g.
 *                    "front_door" (max 127 chars, copied).
 *                    Pass NULL or "" when the thread is not stream-specific.
 */
void log_set_thread_context(const char *component, const char *stream_name);

/**
 * Clear the logging context for the current thread.
 * After this call log_* calls from the thread omit the context prefix.
 */
void log_clear_thread_context(void);

/**
 * Return the component label stored for the current thread.
 * Returns "" when no context has been set.
 */
const char *log_get_thread_component(void);

/**
 * Return the stream name stored for the current thread.
 * Returns "" when no stream context has been set.
 */
const char *log_get_thread_stream(void);

#endif // LIGHTNVR_LOGGER_H
