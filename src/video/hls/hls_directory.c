#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <dirent.h>
#include <sys/stat.h>
#include <time.h>
#include <errno.h>

#include "core/logger.h"
#include "core/config.h"
#include "video/streams.h"
#include "video/hls/hls_directory.h"
#include "video/hls/hls_context.h"

/**
 * Ensure the HLS output directory exists and is writable
 */
int ensure_hls_directory(const char *output_dir, const char *stream_name) {
    // Get the global config for storage path
    config_t *global_config = get_streaming_config();
    if (!global_config) {
        log_error("Failed to get global config for HLS directory");
        return -1;
    }

    //  Always use the consistent path structure for HLS
    // Always use the storage path from the config
    char safe_output_dir[MAX_PATH_LENGTH];
    snprintf(safe_output_dir, sizeof(safe_output_dir), "%s/hls/%s", 
            global_config->storage_path, stream_name);
    
    // Log if we're redirecting from a different path
    if (strcmp(output_dir, safe_output_dir) != 0) {
        log_warn("Redirecting HLS output from %s to %s to ensure consistent path structure", 
                output_dir, safe_output_dir);
    }
    
    // Always use the safe path
    output_dir = safe_output_dir;

    // Verify output directory exists and is writable
    struct stat st;
    if (stat(output_dir, &st) != 0 || !S_ISDIR(st.st_mode)) {
        log_warn("Output directory does not exist or is not a directory: %s", output_dir);

        // Recreate it as a last resort
        char mkdir_cmd[MAX_PATH_LENGTH * 2];
        snprintf(mkdir_cmd, sizeof(mkdir_cmd), "mkdir -p %s", output_dir);

        int ret_mkdir = system(mkdir_cmd);
        if (ret_mkdir != 0 || stat(output_dir, &st) != 0 || !S_ISDIR(st.st_mode)) {
            log_error("Failed to create output directory: %s (return code: %d)", output_dir, ret_mkdir);
            return -1;
        }

        // Set permissions - use 777 to ensure all processes can write
        snprintf(mkdir_cmd, sizeof(mkdir_cmd), "chmod -R 777 %s", output_dir);
        int ret_chmod = system(mkdir_cmd);
        if (ret_chmod != 0) {
            log_warn("Failed to set permissions on directory: %s (return code: %d)", output_dir, ret_chmod);
            
            // Try again with sudo if available
            snprintf(mkdir_cmd, sizeof(mkdir_cmd), "sudo chmod -R 777 %s 2>/dev/null", output_dir);
            ret_chmod = system(mkdir_cmd);
            if (ret_chmod != 0) {
                log_warn("Failed to set permissions with sudo on directory: %s", output_dir);
            }
        }
        
        log_info("Successfully created output directory: %s", output_dir);
    }

    // Check directory permissions
    if (access(output_dir, W_OK) != 0) {
        log_error("Output directory is not writable: %s", output_dir);

        // Try to fix permissions
        char chmod_cmd[MAX_PATH_LENGTH * 2];
        snprintf(chmod_cmd, sizeof(chmod_cmd), "chmod -R 777 %s", output_dir);
        int ret_chmod = system(chmod_cmd);
        if (ret_chmod != 0) {
            log_warn("Failed to set permissions on directory: %s (return code: %d)", output_dir, ret_chmod);
        }

        if (access(output_dir, W_OK) != 0) {
            log_error("Still unable to write to output directory: %s", output_dir);
            return -1;
        }
        
        log_info("Successfully fixed permissions for output directory: %s", output_dir);
    }

    // Create a parent directory check file to ensure the parent directory exists
    char parent_dir[MAX_PATH_LENGTH];
    const char *last_slash = strrchr(output_dir, '/');
    if (last_slash) {
        size_t parent_len = last_slash - output_dir;
        strncpy(parent_dir, output_dir, parent_len);
        parent_dir[parent_len] = '\0';
        
        // Create a test file in the parent directory
        char test_file[MAX_PATH_LENGTH];
        snprintf(test_file, sizeof(test_file), "%s/.hls_parent_check", parent_dir);
        FILE *fp = fopen(test_file, "w");
        if (fp) {
            fclose(fp);
            // Leave the file there as a marker
            log_info("Verified parent directory is writable: %s", parent_dir);
        } else {
            log_warn("Parent directory may not be writable: %s (error: %s)", 
                    parent_dir, strerror(errno));
            
            // Try to create parent directory with full permissions
            char parent_cmd[MAX_PATH_LENGTH * 2];
            snprintf(parent_cmd, sizeof(parent_cmd), "mkdir -p %s && chmod -R 777 %s", 
                    parent_dir, parent_dir);
            int ret_parent = system(parent_cmd);
            if (ret_parent != 0) {
                log_warn("Failed to create parent directory: %s (return code: %d)", parent_dir, ret_parent);
            }
            
            log_info("Attempted to recreate parent directory with full permissions: %s", parent_dir);
        }
    }

    return 0;
}

/**
 * Clear HLS segments for a specific stream
 * This is used when a stream's URL is changed to ensure the player sees the new stream
 */
int clear_stream_hls_segments(const char *stream_name) {
    if (!stream_name) {
        log_error("Cannot clear HLS segments: stream name is NULL");
        return -1;
    }
    
    config_t *global_config = get_streaming_config();
    if (!global_config || !global_config->storage_path) {
        log_error("Cannot clear HLS segments: global config or storage path is NULL");
        return -1;
    }
    
    char stream_hls_dir[MAX_PATH_LENGTH];
    snprintf(stream_hls_dir, MAX_PATH_LENGTH, "%s/hls/%s", 
             global_config->storage_path, stream_name);
    
    // Check if the directory exists
    struct stat st;
    if (stat(stream_hls_dir, &st) != 0 || !S_ISDIR(st.st_mode)) {
        log_info("HLS directory for stream %s does not exist, nothing to clear", stream_name);
        return 0;
    }
    
    log_info("Clearing HLS segments for stream: %s in directory: %s", stream_name, stream_hls_dir);
    
    // Remove all .ts segment files
    char rm_cmd[MAX_PATH_LENGTH * 2];
    snprintf(rm_cmd, sizeof(rm_cmd), "rm -f %s/*.ts", stream_hls_dir);
    int ret = system(rm_cmd);
    if (ret != 0) {
        log_warn("Failed to remove HLS .ts segment files in %s (return code: %d)", 
                stream_hls_dir, ret);
    } else {
        log_info("Removed HLS .ts segment files in %s", stream_hls_dir);
    }
    
    // Remove all .m4s segment files (for fMP4)
    snprintf(rm_cmd, sizeof(rm_cmd), "rm -f %s/*.m4s", stream_hls_dir);
    ret = system(rm_cmd);
    if (ret != 0) {
        log_warn("Failed to remove HLS .m4s segment files in %s (return code: %d)", 
                stream_hls_dir, ret);
    } else {
        log_info("Removed HLS .m4s segment files in %s", stream_hls_dir);
    }
    
    // Remove init.mp4 file (for fMP4)
    snprintf(rm_cmd, sizeof(rm_cmd), "rm -f %s/init.mp4", stream_hls_dir);
    ret = system(rm_cmd);
    if (ret != 0) {
        log_warn("Failed to remove HLS init.mp4 file in %s (return code: %d)", 
                stream_hls_dir, ret);
    } else {
        log_info("Removed HLS init.mp4 file in %s", stream_hls_dir);
    }
    
    // Remove all .m3u8 playlist files
    snprintf(rm_cmd, sizeof(rm_cmd), "rm -f %s/*.m3u8*", stream_hls_dir);
    ret = system(rm_cmd);
    if (ret != 0) {
        log_warn("Failed to remove HLS playlist files in %s (return code: %d)", 
                stream_hls_dir, ret);
    } else {
        log_info("Removed HLS playlist files in %s", stream_hls_dir);
    }
    
    // Ensure the directory has proper permissions
    char chmod_cmd[MAX_PATH_LENGTH * 2];
    snprintf(chmod_cmd, sizeof(chmod_cmd), "chmod -R 777 %s", stream_hls_dir);
    int ret_chmod = system(chmod_cmd);
    if (ret_chmod != 0) {
        log_warn("Failed to set permissions on directory: %s (return code: %d)", 
                stream_hls_dir, ret_chmod);
    }
    
    return 0;
}

/**
 * Clean up HLS directories during shutdown
 */
void cleanup_hls_directories(void) {
    config_t *global_config = get_streaming_config();
    
    if (!global_config || !global_config->storage_path) {
        log_error("Cannot clean up HLS directories: global config or storage path is NULL");
        return;
    }
    
    char hls_base_dir[MAX_PATH_LENGTH];
    snprintf(hls_base_dir, MAX_PATH_LENGTH, "%s/hls", global_config->storage_path);
    
    // Check if HLS base directory exists
    struct stat st;
    if (stat(hls_base_dir, &st) != 0 || !S_ISDIR(st.st_mode)) {
        log_info("HLS base directory does not exist, nothing to clean up: %s", hls_base_dir);
        return;
    }
    
    log_info("Cleaning up HLS directories in: %s", hls_base_dir);
    
    // Open the HLS base directory
    DIR *dir = opendir(hls_base_dir);
    if (!dir) {
        log_error("Failed to open HLS base directory for cleanup: %s (error: %s)", 
                 hls_base_dir, strerror(errno));
        return;
    }
    
    // Iterate through each stream directory
    struct dirent *entry;
    while ((entry = readdir(dir)) != NULL) {
        // Skip . and .. directories
        if (strcmp(entry->d_name, ".") == 0 || strcmp(entry->d_name, "..") == 0) {
            continue;
        }
        
        // Form the full path to the stream's HLS directory
        char stream_hls_dir[MAX_PATH_LENGTH];
        snprintf(stream_hls_dir, MAX_PATH_LENGTH, "%s/%s", hls_base_dir, entry->d_name);
        
        // Check if it's a directory
        if (stat(stream_hls_dir, &st) == 0 && S_ISDIR(st.st_mode)) {
            log_info("Cleaning up HLS files for stream: %s", entry->d_name);
            
            // Check if this stream is currently active
            bool is_active = false;
            for (int i = 0; i < MAX_STREAMS; i++) {
                if (streaming_contexts[i] && 
                    strcmp(streaming_contexts[i]->config.name, entry->d_name) == 0 &&
                    streaming_contexts[i]->running) {
                    is_active = true;
                    break;
                }
            }

            if (is_active) {
                log_info("Stream %s is active, skipping cleanup of main playlist file", entry->d_name);
                
                // For active streams, only remove temporary files and old segments
                // but preserve the main index.m3u8 file
                
                // Remove temporary .m3u8.tmp files
                char rm_cmd[MAX_PATH_LENGTH * 2];
                snprintf(rm_cmd, sizeof(rm_cmd), "rm -f %s/*.m3u8.tmp", stream_hls_dir);
                int ret_tmp = system(rm_cmd);
                if (ret_tmp != 0) {
                    log_warn("Failed to remove temporary files in %s (return code: %d)", 
                            stream_hls_dir, ret_tmp);
                }
                
                // Only remove segments that are older than 5 minutes
                // This ensures we don't delete segments that might still be in use
                snprintf(rm_cmd, sizeof(rm_cmd), 
                        "find %s -name \"*.ts\" -type f -mmin +5 -delete", 
                        stream_hls_dir);
                int ret_find = system(rm_cmd);
                if (ret_find != 0) {
                    log_warn("Failed to remove old .ts segments in %s (return code: %d)", 
                            stream_hls_dir, ret_find);
                }
                
                // Also clean up old .m4s segments (for fMP4)
                snprintf(rm_cmd, sizeof(rm_cmd), 
                        "find %s -name \"*.m4s\" -type f -mmin +5 -delete", 
                        stream_hls_dir);
                ret_find = system(rm_cmd);
                if (ret_find != 0) {
                    log_warn("Failed to remove old .m4s segments in %s (return code: %d)", 
                            stream_hls_dir, ret_find);
                }
                
                log_info("Cleaned up temporary files for active stream: %s", entry->d_name);
            } else {
                // For inactive streams, we can safely remove all files
                log_info("Stream %s is inactive, removing all HLS files", entry->d_name);
                
                // Remove all .ts segment files
                char rm_cmd[MAX_PATH_LENGTH * 2];
                snprintf(rm_cmd, sizeof(rm_cmd), "rm -f %s/*.ts", stream_hls_dir);
                int ret = system(rm_cmd);
                if (ret != 0) {
                    log_warn("Failed to remove HLS .ts segment files in %s (return code: %d)", 
                            stream_hls_dir, ret);
                } else {
                    log_info("Removed HLS .ts segment files in %s", stream_hls_dir);
                }
                
                // Remove all .m4s segment files (for fMP4)
                snprintf(rm_cmd, sizeof(rm_cmd), "rm -f %s/*.m4s", stream_hls_dir);
                ret = system(rm_cmd);
                if (ret != 0) {
                    log_warn("Failed to remove HLS .m4s segment files in %s (return code: %d)", 
                            stream_hls_dir, ret);
                } else {
                    log_info("Removed HLS .m4s segment files in %s", stream_hls_dir);
                }
                
                // Remove init.mp4 file (for fMP4)
                snprintf(rm_cmd, sizeof(rm_cmd), "rm -f %s/init.mp4", stream_hls_dir);
                ret = system(rm_cmd);
                if (ret != 0) {
                    log_warn("Failed to remove HLS init.mp4 file in %s (return code: %d)", 
                            stream_hls_dir, ret);
                } else {
                    log_info("Removed HLS init.mp4 file in %s", stream_hls_dir);
                }
                
                // Remove all .m3u8 playlist files
                snprintf(rm_cmd, sizeof(rm_cmd), "rm -f %s/*.m3u8*", stream_hls_dir);
                ret = system(rm_cmd);
                if (ret != 0) {
                    log_warn("Failed to remove HLS playlist files in %s (return code: %d)", 
                            stream_hls_dir, ret);
                } else {
                    log_info("Removed HLS playlist files in %s", stream_hls_dir);
                }
            }
            
            // Ensure the directory has proper permissions
            char chmod_cmd[MAX_PATH_LENGTH * 2];
            snprintf(chmod_cmd, sizeof(chmod_cmd), "chmod -R 777 %s", stream_hls_dir);
            int ret_chmod = system(chmod_cmd);
            if (ret_chmod != 0) {
                log_warn("Failed to set permissions on directory: %s (return code: %d)", 
                        stream_hls_dir, ret_chmod);
            }
        }
    }
    
    closedir(dir);
    log_info("HLS directory cleanup completed");
}
