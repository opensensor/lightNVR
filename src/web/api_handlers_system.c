 #define _XOPEN_SOURCE
#define _GNU_SOURCE

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>
#include <unistd.h>
#include <sys/sysinfo.h>
#include <sys/statvfs.h>
#include <signal.h>
#include <fcntl.h>
#include <errno.h>
#include <sys/stat.h>

#include "web/api_handlers_system.h"
#include "web/api_handlers_common.h"
#include "core/config.h"
#include "core/logger.h"
#include "core/version.h"
#include "video/stream_manager.h"
#include "storage/storage_manager.h"

// Access daemon_mode from main.c
extern bool daemon_mode;

// Define a local config variable to work with
static config_t local_config;

// Global variable to store system start time
static time_t system_start_time = 0;

// Forward declaration of helper function to get current configuration
static config_t* get_current_config(void);

// Helper function to get current configuration
static config_t* get_current_config(void) {
    // This should return a reference to the actual global config
    extern config_t global_config;  // Declared in streams.c
    return &global_config;
}

// Helper function to read logs from file
static char** read_logs_from_file(const char* log_file, int max_lines, int* num_lines) {
    FILE* file = fopen(log_file, "r");
    if (!file) {
        *num_lines = 0;
        return NULL;
    }
    
    // Count the number of lines in the file
    int line_count = 0;
    char buffer[1024];
    while (fgets(buffer, sizeof(buffer), file) != NULL) {
        line_count++;
    }
    
    // Limit to max_lines
    int lines_to_read = (max_lines > 0 && line_count > max_lines) ? max_lines : line_count;
    
    // Allocate memory for the lines
    char** lines = (char**)malloc(lines_to_read * sizeof(char*));
    if (!lines) {
        fclose(file);
        *num_lines = 0;
        return NULL;
    }
    
    // If we need to skip lines (because there are more lines than max_lines)
    int lines_to_skip = line_count - lines_to_read;
    
    // Rewind file to beginning
    rewind(file);
    
    // Skip lines if needed
    for (int i = 0; i < lines_to_skip; i++) {
        if (fgets(buffer, sizeof(buffer), file) == NULL) {
            break;
        }
    }
    
    // Read the lines we want
    for (int i = 0; i < lines_to_read; i++) {
        if (fgets(buffer, sizeof(buffer), file) == NULL) {
            break;
        }
        
        // Remove newline character if present
        size_t len = strlen(buffer);
        if (len > 0 && buffer[len-1] == '\n') {
            buffer[len-1] = '\0';
        }
        
        // Allocate memory for this line and copy it
        lines[i] = strdup(buffer);
        if (!lines[i]) {
            // If allocation fails, free previously allocated lines
            for (int j = 0; j < i; j++) {
                free(lines[j]);
            }
            free(lines);
            fclose(file);
            *num_lines = 0;
            return NULL;
        }
    }
    
    fclose(file);
    *num_lines = lines_to_read;
    return lines;
}

// Helper function to get system uptime
static long get_system_uptime(void) {
    if (system_start_time == 0) {
        // If start time not set, set it now
        system_start_time = time(NULL);
    }
    
    return (long)(time(NULL) - system_start_time);
}

// Helper function to get CPU usage
static int get_cpu_usage(void) {
    FILE* file = fopen("/proc/stat", "r");
    if (!file) {
        return 0;
    }
    
    unsigned long user, nice, system, idle, iowait, irq, softirq;
    if (fscanf(file, "cpu %lu %lu %lu %lu %lu %lu %lu", 
               &user, &nice, &system, &idle, &iowait, &irq, &softirq) != 7) {
        fclose(file);
        return 0;
    }
    
    fclose(file);
    
    // Calculate CPU usage (simplified)
    unsigned long total = user + nice + system + idle + iowait + irq + softirq;
    unsigned long active = user + nice + system + irq + softirq;
    
    // To be more accurate, we should compare with previous values
    // For simplicity, we'll just return a percentage of active time
    return (int)((active * 100) / total);
}

// Helper function to get memory usage
static void get_memory_info(unsigned long* used, unsigned long* total) {
    struct sysinfo info;
    
    if (sysinfo(&info) != 0) {
        *used = 0;
        *total = 0;
        return;
    }
    
    *total = info.totalram / (1024 * 1024); // Convert to MB
    *used = (info.totalram - info.freeram) / (1024 * 1024); // Convert to MB
}

// Helper function to get recording streams count
static int get_recording_streams_count(void) {
    // In a real implementation, this would count the number of streams that are recording
    // For now, we'll just return a placeholder value based on active streams
    int active = get_active_stream_count();
    return (active > 0) ? (active / 2 + 1) : 0;
}

// Helper function to get data received and recorded
static void get_data_stats(uint64_t* received, uint64_t* recorded) {
    // In a real implementation, this would track actual data received and recorded
    // For now, we'll just use placeholder values
    storage_stats_t stats;
    if (get_storage_stats(&stats) == 0) {
        *recorded = stats.total_recording_bytes / (1024 * 1024); // Convert to MB
    } else {
        *recorded = 0;
    }
    
    // Assume received is slightly more than recorded
    *received = *recorded * 1.2;
}

/**
 * Handle GET request for system information
 */
void handle_get_system_info(const http_request_t *request, http_response_t *response) {
    // Copy current config settings from global config
    memcpy(&local_config, get_current_config(), sizeof(config_t));
    
    // Get system information
    char json[1024];
    
    // Get real system information
    long uptime = get_system_uptime();
    int cpu_usage = get_cpu_usage();
    
    unsigned long memory_usage, memory_total;
    get_memory_info(&memory_usage, &memory_total);
    
    storage_stats_t storage_stats;
    if (get_storage_stats(&storage_stats) != 0) {
        // If storage stats fail, use defaults
        storage_stats.used_space = 0;
        storage_stats.total_space = 0;
    }
    
    // Convert storage to GB
    double storage_usage = (double)storage_stats.used_space / (1024 * 1024 * 1024);
    double storage_total = (double)storage_stats.total_space / (1024 * 1024 * 1024);
    
    int active_streams = get_active_stream_count();
    int recording_streams = get_recording_streams_count();
    
    uint64_t data_received, data_recorded;
    get_data_stats(&data_received, &data_recorded);
    
    snprintf(json, sizeof(json),
             "{"
             "\"version\": \"%s\","
             "\"uptime\": %ld,"
             "\"cpu_usage\": %d,"
             "\"memory_usage\": %lu,"
             "\"memory_total\": %lu,"
             "\"storage_usage\": %.2f,"
             "\"storage_total\": %.2f,"
             "\"active_streams\": %d,"
             "\"max_streams\": %d,"
             "\"recording_streams\": %d,"
             "\"data_received\": %lu,"
             "\"data_recorded\": %lu"
             "}",
             LIGHTNVR_VERSION_STRING,
             uptime,
             cpu_usage,
             memory_usage,
             memory_total,
             storage_usage,
             storage_total,
             active_streams,
             local_config.max_streams,
             recording_streams,
             (unsigned long)data_received,
             (unsigned long)data_recorded);
    
    create_json_response(response, 200, json);
}

/**
 * Handle GET request for system logs
 */
void handle_get_system_logs(const http_request_t *request, http_response_t *response) {
    // Copy current config settings from global config
    memcpy(&local_config, get_current_config(), sizeof(config_t));
    
    // Get logs from the log file
    const char* log_file = "/var/log/lightnvr.log"; // Default log file path
    const char* fallback_log_file = "./lightnvr.log"; // Fallback log file in current directory
    
    // Check if config has a log file path
    if (local_config.log_file[0] != '\0') {
        log_file = local_config.log_file;
    }
    
    // Check if log file exists and is readable
    struct stat st;
    bool file_exists = (stat(log_file, &st) == 0);
    bool is_readable = (access(log_file, R_OK) == 0);
    
    // Log debug information
    log_info("Reading logs from file: %s", log_file);
    log_info("Log file exists: %s", file_exists ? "yes" : "no");
    log_info("Log file is readable: %s", is_readable ? "yes" : "no");
    if (file_exists) {
        log_info("Log file size: %ld bytes", (long)st.st_size);
    }
    
    // If the log file doesn't exist or isn't readable, try the fallback
    if (!file_exists || !is_readable) {
        log_info("Trying fallback log file: %s", fallback_log_file);
        file_exists = (stat(fallback_log_file, &st) == 0);
        is_readable = (access(fallback_log_file, R_OK) == 0);
        
        if (file_exists && is_readable) {
            log_info("Using fallback log file");
            log_file = fallback_log_file;
        }
    }
    
    int num_lines = 0;
    char** log_lines = read_logs_from_file(log_file, 100, &num_lines);
    
    // Build JSON response
    char* json = NULL;
    size_t json_size = 0;
    FILE* json_stream = open_memstream(&json, &json_size);
    
    if (!json_stream) {
        // If we can't create a memory stream, use a static response
        const char* error_json = "{\"logs\":[\"Error reading log file\"]}";
        create_json_response(response, 500, error_json);
        return;
    }
    
    fprintf(json_stream, "{\"logs\":[");
    
    if (log_lines && num_lines > 0) {
        // Add each log line as a JSON string
        for (int i = 0; i < num_lines; i++) {
            fprintf(json_stream, "\"%s\"", log_lines[i]);
            if (i < num_lines - 1) {
                fprintf(json_stream, ",");
            }
            free(log_lines[i]);
        }
        free(log_lines);
    } else {
        // No logs or error reading logs
        if (!file_exists) {
            fprintf(json_stream, "\"Log file does not exist: %s\"", log_file);
        } else if (!is_readable) {
            fprintf(json_stream, "\"Log file is not readable: %s\"", log_file);
        } else if (file_exists && st.st_size == 0) {
            fprintf(json_stream, "\"Log file is empty: %s\"", log_file);
        } else {
            fprintf(json_stream, "\"No logs available in: %s\"", log_file);
        }
    }
    
    fprintf(json_stream, "]}");
    fclose(json_stream);
    
    create_json_response(response, 200, json);
    free(json);
}

/**
 * Handle POST request to restart the service
 */
void handle_post_system_restart(const http_request_t *request, http_response_t *response) {
    // Check if this is just a daemon mode check
    bool check_only = false;
    
    // Parse request body if present
    if (request->body && request->content_length > 0) {
        // Simple JSON parsing to check for check_only parameter
        char* check_only_str = strstr(request->body, "\"check_only\"");
        if (check_only_str) {
            char* true_str = strstr(check_only_str, "true");
            if (true_str && true_str < (check_only_str + 20)) {
                check_only = true;
            }
        }
    }
    
    // Only allow restart if running in daemon mode
    if (!daemon_mode) {
        log_warn("Restart requested but not running in daemon mode");
        const char* error_json = "{\"success\":false,\"message\":\"Restart is only available when running in daemon mode\"}";
        create_json_response(response, 403, error_json);
        return;
    }
    
    // If this is just a check, return success without restarting
    if (check_only) {
        const char* json = "{\"success\":true,\"daemon_mode\":true}";
        create_json_response(response, 200, json);
        return;
    }
    
    // Log the restart request
    log_info("System restart requested via API");
    
    // Create success response
    const char* json = "{\"success\":true,\"message\":\"System restart initiated\"}";
    create_json_response(response, 200, json);
    
    // Schedule a restart after a short delay to allow the response to be sent
    pid_t pid = fork();
    if (pid == 0) {
        // Child process
        sleep(1); // Wait for response to be sent
        
        // Get the path to the current executable
        char exe_path[1024];
        ssize_t len = readlink("/proc/self/exe", exe_path, sizeof(exe_path) - 1);
        if (len != -1) {
            exe_path[len] = '\0';
            
            // Restart the service
            execl(exe_path, exe_path, "--daemon", NULL);
            
            // If execl fails
            exit(EXIT_FAILURE);
        }
        
        // If we can't get the executable path, just exit
        exit(EXIT_FAILURE);
    }
}

/**
 * Handle POST request to shutdown the service
 */
void handle_post_system_shutdown(const http_request_t *request, http_response_t *response) {
    // Log the shutdown request
    log_info("System shutdown requested via API");
    
    // Create success response
    const char* json = "{\"success\":true,\"message\":\"System shutdown initiated\"}";
    create_json_response(response, 200, json);
    
    // Schedule a shutdown after a short delay to allow the response to be sent
    pid_t pid = fork();
    if (pid == 0) {
        // Child process
        sleep(1); // Wait for response to be sent
        
        // Send SIGTERM to parent process
        kill(getppid(), SIGTERM);
        
        // Exit child process
        exit(EXIT_SUCCESS);
    }
}

/**
 * Handle POST request to clear system logs
 */
void handle_post_system_clear_logs(const http_request_t *request, http_response_t *response) {
    // Get log file path
    const char* log_file = "/var/log/lightnvr.log"; // Default log file path
    const char* fallback_log_file = "./lightnvr.log"; // Fallback log file in current directory
    
    // Check if config has a log file path
    if (local_config.log_file[0] != '\0') {
        log_file = local_config.log_file;
    }
    
    // Check if log file exists and is writable
    struct stat st;
    bool file_exists = (stat(log_file, &st) == 0);
    bool is_writable = (access(log_file, W_OK) == 0);
    
    // If the log file doesn't exist or isn't writable, try the fallback
    if (!file_exists || !is_writable) {
        log_info("Primary log file not accessible, trying fallback: %s", fallback_log_file);
        file_exists = (stat(fallback_log_file, &st) == 0);
        is_writable = (access(fallback_log_file, W_OK) == 0);
        
        if (file_exists || is_writable) {
            log_info("Using fallback log file for clearing");
            log_file = fallback_log_file;
        }
    }
    
    // Clear the log file by truncating it
    int fd = open(log_file, O_WRONLY | O_TRUNC | O_CREAT, 0644);
    if (fd >= 0) {
        close(fd);
        log_info("Log file cleared via API: %s", log_file);
        
        const char* json = "{\"success\":true,\"message\":\"Logs cleared successfully\"}";
        create_json_response(response, 200, json);
    } else {
        log_error("Failed to clear log file %s: %s", log_file, strerror(errno));
        
        char error_json[256];
        snprintf(error_json, sizeof(error_json), 
                 "{\"success\":false,\"message\":\"Failed to clear logs: %s\"}", 
                 strerror(errno));
        create_json_response(response, 500, error_json);
    }
}

/**
 * Handle POST request to backup configuration
 */
void handle_post_system_backup(const http_request_t *request, http_response_t *response) {
    // Create a timestamp for the backup filename
    time_t now = time(NULL);
    struct tm* tm_info = localtime(&now);
    char timestamp[20];
    strftime(timestamp, sizeof(timestamp), "%Y%m%d_%H%M%S", tm_info);
    
    // Create backup filename
    char backup_filename[256];
    snprintf(backup_filename, sizeof(backup_filename), "lightnvr_backup_%s.json", timestamp);
    
    // Create backup path in the web root directory
    char backup_path[512];
    snprintf(backup_path, sizeof(backup_path), "%s/backups", local_config.web_root);
    
    // Create backups directory if it doesn't exist
    mkdir(backup_path, 0755);
    
    // Append filename to path
    snprintf(backup_path, sizeof(backup_path), "%s/backups/%s", local_config.web_root, backup_filename);
    
    // Open backup file
    FILE* backup_file = fopen(backup_path, "w");
    if (!backup_file) {
        log_error("Failed to create backup file: %s", strerror(errno));
        
        char error_json[256];
        snprintf(error_json, sizeof(error_json), 
                 "{\"success\":false,\"message\":\"Failed to create backup: %s\"}", 
                 strerror(errno));
        create_json_response(response, 500, error_json);
        return;
    }
    
    // Write configuration to backup file as JSON
    fprintf(backup_file, "{\n");
    fprintf(backup_file, "  \"version\": \"%s\",\n", LIGHTNVR_VERSION_STRING);
    fprintf(backup_file, "  \"timestamp\": \"%s\",\n", timestamp);
    fprintf(backup_file, "  \"config\": {\n");
    fprintf(backup_file, "    \"web_port\": %d,\n", local_config.web_port);
    fprintf(backup_file, "    \"web_root\": \"%s\",\n", local_config.web_root);
    fprintf(backup_file, "    \"log_file\": \"%s\",\n", local_config.log_file);
    fprintf(backup_file, "    \"pid_file\": \"%s\",\n", local_config.pid_file);
    fprintf(backup_file, "    \"db_path\": \"%s\",\n", local_config.db_path);
    fprintf(backup_file, "    \"storage_path\": \"%s\",\n", local_config.storage_path);
    fprintf(backup_file, "    \"max_storage_size\": %lu,\n", local_config.max_storage_size);
    fprintf(backup_file, "    \"max_streams\": %d,\n", local_config.max_streams);
    
    // Add streams configuration
    fprintf(backup_file, "    \"streams\": [\n");
    for (int i = 0; i < local_config.max_streams; i++) {
        if (local_config.streams[i].name[0] != '\0') {
            fprintf(backup_file, "      {\n");
            fprintf(backup_file, "        \"name\": \"%s\",\n", local_config.streams[i].name);
            fprintf(backup_file, "        \"url\": \"%s\",\n", local_config.streams[i].url);
            fprintf(backup_file, "        \"enabled\": %s,\n", local_config.streams[i].enabled ? "true" : "false");
            fprintf(backup_file, "        \"width\": %d,\n", local_config.streams[i].width);
            fprintf(backup_file, "        \"height\": %d,\n", local_config.streams[i].height);
            fprintf(backup_file, "        \"fps\": %d,\n", local_config.streams[i].fps);
            fprintf(backup_file, "        \"codec\": \"%s\",\n", local_config.streams[i].codec);
            fprintf(backup_file, "        \"record\": %s,\n", local_config.streams[i].record ? "true" : "false");
            fprintf(backup_file, "        \"priority\": %d,\n", local_config.streams[i].priority);
            fprintf(backup_file, "        \"segment_duration\": %d\n", local_config.streams[i].segment_duration);
            fprintf(backup_file, "      }%s\n", (i < local_config.max_streams - 1 && 
                                               local_config.streams[i+1].name[0] != '\0') ? "," : "");
        }
    }
    fprintf(backup_file, "    ]\n");
    fprintf(backup_file, "  }\n");
    fprintf(backup_file, "}\n");
    
    fclose(backup_file);
    
    log_info("Configuration backup created: %s", backup_path);
    
    // Create success response with download URL
    char json[512];
    snprintf(json, sizeof(json), 
             "{\"success\":true,\"message\":\"Backup created successfully\",\"backupUrl\":\"/backups/%s\",\"filename\":\"%s\"}",
             backup_filename, backup_filename);
    create_json_response(response, 200, json);
}

/**
 * Handle GET request for system status
 */
void handle_get_system_status(const http_request_t *request, http_response_t *response) {
    // Simple status check - if we're responding, the system is running
    const char* json = "{\"status\":\"ok\",\"message\":\"System running normally\"}";
    create_json_response(response, 200, json);
}
