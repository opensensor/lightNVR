#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdarg.h>
#include <time.h>
#include <pthread.h>
#include <unistd.h>
#include <sys/stat.h>
#include <sys/types.h>
#include <errno.h>
#include <libgen.h>

#include "core/logger.h"

// Logger state
static struct {
    FILE *log_file;
    log_level_t log_level;
    int console_logging;
    char log_filename[256];
    pthread_mutex_t mutex;
} logger = {
    .log_file = NULL,
    .log_level = LOG_LEVEL_INFO,
    .console_logging = 1,
    .log_filename = "",
};

// Log level strings
static const char *log_level_strings[] = {
    "ERROR",
    "WARN",
    "INFO",
    "DEBUG"
};

// Initialize the logging system
int init_logger(void) {
    // Initialize mutex
    if (pthread_mutex_init(&logger.mutex, NULL) != 0) {
        fprintf(stderr, "Failed to initialize logger mutex\n");
        return -1;
    }
    
    // Default to stderr if no log file is set
    if (logger.log_file == NULL && logger.log_filename[0] == '\0') {
        logger.log_file = stderr;
    }
    
    return 0;
}

// Shutdown the logging system
void shutdown_logger(void) {
    pthread_mutex_lock(&logger.mutex);
    
    if (logger.log_file != NULL && logger.log_file != stdout && logger.log_file != stderr) {
        fclose(logger.log_file);
        logger.log_file = NULL;
    }
    
    pthread_mutex_unlock(&logger.mutex);
    pthread_mutex_destroy(&logger.mutex);
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

// Create directory if it doesn't exist
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
    
    char *dir = dirname(dir_path);
    if (create_directory(dir) != 0) {
        free(dir_path);
        pthread_mutex_unlock(&logger.mutex);
        return -1;
    }
    free(dir_path);
    
    // Open new log file
    logger.log_file = fopen(filename, "a");
    if (!logger.log_file) {
        pthread_mutex_unlock(&logger.mutex);
        return -1;
    }
    
    // Store filename for potential log rotation
    strncpy(logger.log_filename, filename, sizeof(logger.log_filename) - 1);
    logger.log_filename[sizeof(logger.log_filename) - 1] = '\0';
    
    pthread_mutex_unlock(&logger.mutex);
    return 0;
}

// Enable or disable console logging
void set_console_logging(int enable) {
    pthread_mutex_lock(&logger.mutex);
    logger.console_logging = enable;
    pthread_mutex_unlock(&logger.mutex);
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

// Log a message at the specified level with va_list
void log_message_v(log_level_t level, const char *format, va_list args) {
    // Only log messages at or below the configured log level
    // For example, if log_level is INFO (2), we log ERROR (0), WARN (1), and INFO (2), but not DEBUG (3)
    if (level > logger.log_level) {
        return;
    }
    
    time_t now;
    struct tm *tm_info;
    char timestamp[20];
    
    // Get current time
    time(&now);
    tm_info = localtime(&now);
    strftime(timestamp, sizeof(timestamp), "%Y-%m-%d %H:%M:%S", tm_info);
    
    // Format the log message
    char message[4096];
    vsnprintf(message, sizeof(message), format, args);
    
    pthread_mutex_lock(&logger.mutex);
    
    // Write to log file if available
    if (logger.log_file) {
        fprintf(logger.log_file, "[%s] [%s] %s\n", timestamp, log_level_strings[level], message);
        fflush(logger.log_file);
    }
    
    // Write to console if enabled
    if (logger.console_logging) {
        FILE *console = (level == LOG_LEVEL_ERROR) ? stderr : stdout;
        fprintf(console, "[%s] [%s] %s\n", timestamp, log_level_strings[level], message);
        fflush(console);
    }
    
    pthread_mutex_unlock(&logger.mutex);
}

// Get the string representation of a log level
const char *get_log_level_string(log_level_t level) {
    if (level >= LOG_LEVEL_ERROR && level <= LOG_LEVEL_DEBUG) {
        return log_level_strings[level];
    }
    return "UNKNOWN";
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
    char old_path[512];
    char new_path[512];
    
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
    
    // Open new log file
    logger.log_file = fopen(logger.log_filename, "a");
    if (!logger.log_file) {
        pthread_mutex_unlock(&logger.mutex);
        return -1;
    }
    
    pthread_mutex_unlock(&logger.mutex);
    return 0;
}
