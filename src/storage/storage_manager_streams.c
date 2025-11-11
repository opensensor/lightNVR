/**
 * @file storage_manager_streams.c
 * @brief Implementation of stream-specific storage management functions
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <dirent.h>
#include <sys/stat.h>
#include <sys/types.h>
#include <unistd.h>
#include <errno.h>

#include "storage/storage_manager.h"
#include "storage/storage_manager_streams.h"
#include "core/logger.h"
#include "core/config.h"
#include <cjson/cJSON.h>

/**
 * Get storage usage per stream
 * 
 * @param storage_path Base storage path
 * @param stream_info Array to fill with stream storage information
 * @param max_streams Maximum number of streams to return
 * @return Number of streams found, or -1 on error
 */
int get_stream_storage_usage(const char *storage_path, stream_storage_info_t *stream_info, int max_streams) {
    if (!storage_path || !stream_info || max_streams <= 0) {
        log_error("Invalid parameters for get_stream_storage_usage");
        return -1;
    }
    
    // Based on user feedback, the recordings are in /var/lib/lightnvr/recordings/mp4/[stream_name]/
    // Construct the mp4 directory path
    char mp4_path[512];
    snprintf(mp4_path, sizeof(mp4_path), "%s/mp4", storage_path);
    
    // Open the mp4 directory
    DIR *dir = opendir(mp4_path);
    if (!dir) {
        log_error("Failed to open mp4 directory: %s (error: %s)", mp4_path, strerror(errno));
        return -1;
    }
    
    // Initialize stream count
    int stream_count = 0;
    
    // Iterate through directory entries (stream directories)
    struct dirent *entry;
    while ((entry = readdir(dir)) != NULL && stream_count < max_streams) {
        // Skip . and ..
        if (strcmp(entry->d_name, ".") == 0 || strcmp(entry->d_name, "..") == 0) {
            continue;
        }
        
        // Check if it's a directory (stream directory)
        char stream_path[512];
        snprintf(stream_path, sizeof(stream_path), "%s/%s", mp4_path, entry->d_name);
        
        struct stat st;
        if (stat(stream_path, &st) == 0 && S_ISDIR(st.st_mode)) {
            // Initialize stream info
            strncpy(stream_info[stream_count].name, entry->d_name, sizeof(stream_info[stream_count].name) - 1);
            stream_info[stream_count].name[sizeof(stream_info[stream_count].name) - 1] = '\0';
            stream_info[stream_count].size_bytes = 0;
            stream_info[stream_count].recording_count = 0;
            
            // Use du command to get directory size
            char du_cmd[768];
            snprintf(du_cmd, sizeof(du_cmd), "du -sb %s 2>/dev/null | cut -f1", stream_path);
            FILE *fp = popen(du_cmd, "r");
            if (fp) {
                if (fscanf(fp, "%lu", &stream_info[stream_count].size_bytes) == 1) {
                    // Use find to count MP4 files
                    char find_cmd[768];
                    snprintf(find_cmd, sizeof(find_cmd), "find %s -type f -name \"*.mp4\" | wc -l", stream_path);
                    FILE *fp_count = popen(find_cmd, "r");
                    if (fp_count) {
                        if (fscanf(fp_count, "%d", &stream_info[stream_count].recording_count) == 1) {
                            if (stream_info[stream_count].recording_count > 0) {
                                stream_count++;
                            }
                        }
                        pclose(fp_count);
                    }
                }
                pclose(fp);
            }
        }
    }
    
    closedir(dir);
    return stream_count;
}

/**
 * Get storage usage for all streams
 * 
 * @param stream_info Pointer to array that will be allocated and filled with stream storage information
 * @return Number of streams found, or -1 on error
 */
int get_all_stream_storage_usage(stream_storage_info_t **stream_info) {
    if (!stream_info) {
        log_error("Invalid parameter for get_all_stream_storage_usage");
        return -1;
    }
    
    // Get storage path from config
    const char *storage_path = g_config.storage_path;
    if (!storage_path || storage_path[0] == '\0') {
        log_error("Storage path not configured");
        return -1;
    }
    
    // Construct the mp4 directory path
    char mp4_path[512];
    snprintf(mp4_path, sizeof(mp4_path), "%s/mp4", storage_path);
    
    // First, count the number of stream directories in the mp4 directory
    DIR *dir = opendir(mp4_path);
    if (!dir) {
        log_error("Failed to open mp4 directory: %s (error: %s)", mp4_path, strerror(errno));
        return -1;
    }
    
    int stream_count = 0;
    struct dirent *entry;
    while ((entry = readdir(dir)) != NULL) {
        // Skip . and ..
        if (strcmp(entry->d_name, ".") == 0 || strcmp(entry->d_name, "..") == 0) {
            continue;
        }
        
        // Check if it's a directory (stream directory)
        char stream_path[512];
        snprintf(stream_path, sizeof(stream_path), "%s/%s", mp4_path, entry->d_name);
        
        struct stat st;
        if (stat(stream_path, &st) == 0 && S_ISDIR(st.st_mode)) {
            stream_count++;
        }
    }
    
    closedir(dir);
    
    // Allocate memory for stream info array
    *stream_info = (stream_storage_info_t *)malloc(stream_count * sizeof(stream_storage_info_t));
    if (!*stream_info) {
        log_error("Failed to allocate memory for stream storage info");
        return -1;
    }
    
    // Get stream storage usage
    int actual_count = get_stream_storage_usage(storage_path, *stream_info, stream_count);
    
    // If no streams found, free memory
    if (actual_count <= 0) {
        free(*stream_info);
        *stream_info = NULL;
    }
    
    return actual_count;
}

/**
 * Add stream storage usage to JSON object
 * 
 * @param json_obj JSON object to add stream storage usage to
 * @return 0 on success, -1 on error
 */
int add_stream_storage_usage_to_json(cJSON *json_obj) {
    if (!json_obj) {
        log_error("Invalid JSON object for add_stream_storage_usage_to_json");
        return -1;
    }
    
    // Create stream storage array
    cJSON *stream_storage_array = cJSON_CreateArray();
    if (!stream_storage_array) {
        log_error("Failed to create stream storage JSON array");
        return -1;
    }
    
    // Get stream storage usage
    stream_storage_info_t *stream_info = NULL;
    int stream_count = get_all_stream_storage_usage(&stream_info);
    
    if (stream_count <= 0 || !stream_info) {
        log_warn("No stream storage usage information available");
        // Still add the empty array to the JSON object
        cJSON_AddItemToObject(json_obj, "streamStorage", stream_storage_array);
        return 0;
    }
    
    // Add stream storage info to array
    for (int i = 0; i < stream_count; i++) {
        cJSON *stream_obj = cJSON_CreateObject();
        if (stream_obj) {
            cJSON_AddStringToObject(stream_obj, "name", stream_info[i].name);
            cJSON_AddNumberToObject(stream_obj, "size", stream_info[i].size_bytes);
            cJSON_AddNumberToObject(stream_obj, "count", stream_info[i].recording_count);
            
            cJSON_AddItemToArray(stream_storage_array, stream_obj);
        }
    }
    
    // Add stream storage array to JSON object
    cJSON_AddItemToObject(json_obj, "streamStorage", stream_storage_array);
    
    // Free memory
    free(stream_info);
    
    return 0;
}
