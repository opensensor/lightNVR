#define _XOPEN_SOURCE
#define _GNU_SOURCE

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <ctype.h>
#include <unistd.h>
#include <sys/utsname.h>
#include <sys/sysinfo.h>
#include <sys/statvfs.h>

#include "web/api_handlers.h"
#include "web/mongoose_adapter.h"
#include "core/logger.h"
#include "core/config.h"
#include "core/version.h"
#include "../external/mongoose/mongoose.h"

/**
 * @brief Direct handler for GET /api/system/info
 */
void mg_handle_get_system_info(struct mg_connection *c, struct mg_http_message *hm) {
    log_info("Handling GET /api/system/info request");
    
    // Create JSON object
    cJSON *info = cJSON_CreateObject();
    if (!info) {
        log_error("Failed to create system info JSON object");
        mg_send_json_error(c, 500, "Failed to create system info JSON");
        return;
    }
    
    // Add version information
    cJSON_AddStringToObject(info, "version", LIGHTNVR_VERSION_STRING);
    cJSON_AddStringToObject(info, "build_date", LIGHTNVR_BUILD_DATE);
    
    // Get system information
    struct utsname system_info;
    if (uname(&system_info) == 0) {
        cJSON_AddStringToObject(info, "system", system_info.sysname);
        cJSON_AddStringToObject(info, "node", system_info.nodename);
        cJSON_AddStringToObject(info, "release", system_info.release);
        cJSON_AddStringToObject(info, "version", system_info.version);
        cJSON_AddStringToObject(info, "machine", system_info.machine);
    }
    
    // Get memory information
    struct sysinfo sys_info;
    if (sysinfo(&sys_info) == 0) {
        cJSON *memory = cJSON_CreateObject();
        if (memory) {
            cJSON_AddNumberToObject(memory, "total", sys_info.totalram * sys_info.mem_unit);
            cJSON_AddNumberToObject(memory, "free", sys_info.freeram * sys_info.mem_unit);
            cJSON_AddNumberToObject(memory, "used", (sys_info.totalram - sys_info.freeram) * sys_info.mem_unit);
            cJSON_AddItemToObject(info, "memory", memory);
        }
    }
    
    // Get disk information
    struct statvfs disk_info;
    if (statvfs("/", &disk_info) == 0) {
        cJSON *disk = cJSON_CreateObject();
        if (disk) {
            cJSON_AddNumberToObject(disk, "total", disk_info.f_blocks * disk_info.f_frsize);
            cJSON_AddNumberToObject(disk, "free", disk_info.f_bfree * disk_info.f_frsize);
            cJSON_AddNumberToObject(disk, "used", (disk_info.f_blocks - disk_info.f_bfree) * disk_info.f_frsize);
            cJSON_AddItemToObject(info, "disk", disk);
        }
    }
    
    // Get uptime
    cJSON_AddNumberToObject(info, "uptime", sys_info.uptime);
    
    // Get process information
    cJSON_AddNumberToObject(info, "pid", getpid());
    
    // Convert to string
    char *json_str = cJSON_PrintUnformatted(info);
    if (!json_str) {
        log_error("Failed to convert system info JSON to string");
        cJSON_Delete(info);
        mg_send_json_error(c, 500, "Failed to convert system info JSON to string");
        return;
    }
    
    // Send response
    mg_send_json_response(c, 200, json_str);
    
    // Clean up
    free(json_str);
    cJSON_Delete(info);
    
    log_info("Successfully handled GET /api/system/info request");
}

/**
 * @brief Direct handler for GET /api/system/logs
 */
void mg_handle_get_system_logs(struct mg_connection *c, struct mg_http_message *hm) {
    log_info("Handling GET /api/system/logs request");
    
    // Get global configuration
    extern config_t global_config;
    
    // Check if log file is set
    if (global_config.log_file[0] == '\0') {
        mg_send_json_error(c, 404, "Log file not configured");
        return;
    }
    
    // Open log file
    FILE *log_file = fopen(global_config.log_file, "r");
    if (!log_file) {
        log_error("Failed to open log file: %s", global_config.log_file);
        mg_send_json_error(c, 500, "Failed to open log file");
        return;
    }
    
    // Get file size
    fseek(log_file, 0, SEEK_END);
    long file_size = ftell(log_file);
    
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
        log_error("Failed to allocate memory for log file");
        fclose(log_file);
        mg_send_json_error(c, 500, "Failed to allocate memory for log file");
        return;
    }
    
    // Read log file
    fseek(log_file, offset, SEEK_SET);
    size_t bytes_read = fread(buffer, 1, read_size, log_file);
    buffer[bytes_read] = '\0';
    
    // Close file
    fclose(log_file);
    
    // Create JSON object
    cJSON *logs = cJSON_CreateObject();
    if (!logs) {
        log_error("Failed to create logs JSON object");
        free(buffer);
        mg_send_json_error(c, 500, "Failed to create logs JSON");
        return;
    }
    
    // Add logs
    cJSON_AddStringToObject(logs, "file", global_config.log_file);
    cJSON_AddNumberToObject(logs, "size", file_size);
    cJSON_AddNumberToObject(logs, "offset", offset);
    cJSON_AddStringToObject(logs, "content", buffer);
    
    // Convert to string
    char *json_str = cJSON_PrintUnformatted(logs);
    
    // Clean up
    free(buffer);
    cJSON_Delete(logs);
    
    if (!json_str) {
        log_error("Failed to convert logs JSON to string");
        mg_send_json_error(c, 500, "Failed to convert logs JSON to string");
        return;
    }
    
    // Send response
    mg_send_json_response(c, 200, json_str);
    
    // Clean up
    free(json_str);
    
    log_info("Successfully handled GET /api/system/logs request");
}

/**
 * @brief Direct handler for POST /api/system/restart
 */
void mg_handle_post_system_restart(struct mg_connection *c, struct mg_http_message *hm) {
    log_info("Handling POST /api/system/restart request");
    
    // Send response before restarting
    mg_send_json_response(c, 200, "{\"success\": true, \"message\": \"System is restarting\"}");
    
    // Flush response
    c->is_resp = 0;
    
    // Log restart
    log_info("System restart requested via API");
    
    // Schedule restart
    extern volatile bool running;
    running = false;
    
    log_info("Successfully handled POST /api/system/restart request");
}

/**
 * @brief Direct handler for POST /api/system/shutdown
 */
void mg_handle_post_system_shutdown(struct mg_connection *c, struct mg_http_message *hm) {
    log_info("Handling POST /api/system/shutdown request");
    
    // Send response before shutting down
    mg_send_json_response(c, 200, "{\"success\": true, \"message\": \"System is shutting down\"}");
    
    // Flush response
    c->is_resp = 0;
    
    // Log shutdown
    log_info("System shutdown requested via API");
    
    // Schedule shutdown
    extern volatile bool running;
    running = false;
    
    log_info("Successfully handled POST /api/system/shutdown request");
}

/**
 * @brief Direct handler for POST /api/system/clear_logs
 */
void mg_handle_post_system_clear_logs(struct mg_connection *c, struct mg_http_message *hm) {
    log_info("Handling POST /api/system/clear_logs request");
    
    // Get global configuration
    extern config_t global_config;
    
    // Get log file path
    const char* log_file = "/var/log/lightnvr.log"; // Default log file path
    const char* fallback_log_file = "./lightnvr.log"; // Fallback log file in current directory
    
    // Check if config has a log file path
    if (global_config.log_file[0] != '\0') {
        log_file = global_config.log_file;
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
        
        mg_send_json_response(c, 200, "{\"success\": true, \"message\": \"Logs cleared successfully\"}");
    } else {
        log_error("Failed to clear log file %s: %s", log_file, strerror(errno));
        
        char error_json[256];
        snprintf(error_json, sizeof(error_json), 
                 "{\"success\": false, \"message\": \"Failed to clear logs: %s\"}", 
                 strerror(errno));
        mg_send_json_error(c, 500, error_json);
    }
    
    log_info("Successfully handled POST /api/system/clear_logs request");
}

/**
 * @brief Direct handler for POST /api/system/backup
 */
void mg_handle_post_system_backup(struct mg_connection *c, struct mg_http_message *hm) {
    log_info("Handling POST /api/system/backup request");
    
    // Get global configuration
    extern config_t global_config;
    
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
    snprintf(backup_path, sizeof(backup_path), "%s/backups", global_config.web_root);
    
    // Create backups directory if it doesn't exist
    mkdir(backup_path, 0755);
    
    // Append filename to path
    snprintf(backup_path, sizeof(backup_path), "%s/backups/%s", global_config.web_root, backup_filename);
    
    // Open backup file
    FILE* backup_file = fopen(backup_path, "w");
    if (!backup_file) {
        log_error("Failed to create backup file: %s", strerror(errno));
        
        char error_json[256];
        snprintf(error_json, sizeof(error_json), 
                 "{\"success\": false, \"message\": \"Failed to create backup: %s\"}", 
                 strerror(errno));
        mg_send_json_error(c, 500, error_json);
        return;
    }
    
    // Create JSON object for backup
    cJSON *backup = cJSON_CreateObject();
    if (!backup) {
        log_error("Failed to create backup JSON object");
        fclose(backup_file);
        mg_send_json_error(c, 500, "Failed to create backup JSON");
        return;
    }
    
    // Add version and timestamp
    cJSON_AddStringToObject(backup, "version", LIGHTNVR_VERSION_STRING);
    cJSON_AddStringToObject(backup, "timestamp", timestamp);
    
    // Add config object
    cJSON *config = cJSON_CreateObject();
    if (!config) {
        log_error("Failed to create config JSON object");
        cJSON_Delete(backup);
        fclose(backup_file);
        mg_send_json_error(c, 500, "Failed to create config JSON");
        return;
    }
    
    // Add config properties
    cJSON_AddNumberToObject(config, "web_port", global_config.web_port);
    cJSON_AddStringToObject(config, "web_root", global_config.web_root);
    cJSON_AddStringToObject(config, "log_file", global_config.log_file);
    cJSON_AddStringToObject(config, "pid_file", global_config.pid_file);
    cJSON_AddStringToObject(config, "db_path", global_config.db_path);
    cJSON_AddStringToObject(config, "storage_path", global_config.storage_path);
    cJSON_AddNumberToObject(config, "max_storage_size", global_config.max_storage_size);
    cJSON_AddNumberToObject(config, "max_streams", global_config.max_streams);
    
    // Add streams array
    cJSON *streams = cJSON_CreateArray();
    if (!streams) {
        log_error("Failed to create streams JSON array");
        cJSON_Delete(config);
        cJSON_Delete(backup);
        fclose(backup_file);
        mg_send_json_error(c, 500, "Failed to create streams JSON");
        return;
    }
    
    // Add streams to array
    for (int i = 0; i < global_config.max_streams; i++) {
        if (global_config.streams[i].name[0] != '\0') {
            cJSON *stream = cJSON_CreateObject();
            if (!stream) {
                log_error("Failed to create stream JSON object");
                continue;
            }
            
            cJSON_AddStringToObject(stream, "name", global_config.streams[i].name);
            cJSON_AddStringToObject(stream, "url", global_config.streams[i].url);
            cJSON_AddBoolToObject(stream, "enabled", global_config.streams[i].enabled);
            cJSON_AddNumberToObject(stream, "width", global_config.streams[i].width);
            cJSON_AddNumberToObject(stream, "height", global_config.streams[i].height);
            cJSON_AddNumberToObject(stream, "fps", global_config.streams[i].fps);
            cJSON_AddStringToObject(stream, "codec", global_config.streams[i].codec);
            cJSON_AddBoolToObject(stream, "record", global_config.streams[i].record);
            cJSON_AddNumberToObject(stream, "priority", global_config.streams[i].priority);
            cJSON_AddNumberToObject(stream, "segment_duration", global_config.streams[i].segment_duration);
            
            cJSON_AddItemToArray(streams, stream);
        }
    }
    
    // Add streams to config
    cJSON_AddItemToObject(config, "streams", streams);
    
    // Add config to backup
    cJSON_AddItemToObject(backup, "config", config);
    
    // Convert to string
    char *json_str = cJSON_Print(backup);
    if (!json_str) {
        log_error("Failed to convert backup JSON to string");
        cJSON_Delete(backup);
        fclose(backup_file);
        mg_send_json_error(c, 500, "Failed to convert backup JSON to string");
        return;
    }
    
    // Write to file
    fprintf(backup_file, "%s", json_str);
    
    // Clean up
    free(json_str);
    cJSON_Delete(backup);
    fclose(backup_file);
    
    log_info("Configuration backup created: %s", backup_path);
    
    // Create success response with download URL
    char json[512];
    snprintf(json, sizeof(json), 
             "{\"success\": true, \"message\": \"Backup created successfully\", \"backupUrl\": \"/backups/%s\", \"filename\": \"%s\"}",
             backup_filename, backup_filename);
    mg_send_json_response(c, 200, json);
    
    log_info("Successfully handled POST /api/system/backup request");
}

/**
 * @brief Direct handler for GET /api/system/status
 */
void mg_handle_get_system_status(struct mg_connection *c, struct mg_http_message *hm) {
    log_info("Handling GET /api/system/status request");
    
    // Simple status check - if we're responding, the system is running
    mg_send_json_response(c, 200, "{\"status\": \"ok\", \"message\": \"System running normally\"}");
    
    log_info("Successfully handled GET /api/system/status request");
}
