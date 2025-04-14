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
    // Use storage_path_hls if specified, otherwise fall back to storage_path
    char safe_output_dir[MAX_PATH_LENGTH];
    const char *base_storage_path = global_config->storage_path;

    // Check if storage_path_hls is specified and not empty
    if (global_config->storage_path_hls[0] != '\0') {
        base_storage_path = global_config->storage_path_hls;
        log_info("Using dedicated HLS storage path: %s", base_storage_path);
    }

    snprintf(safe_output_dir, sizeof(safe_output_dir), "%s/hls/%s",
            base_storage_path, stream_name);

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

        // Recreate it using direct C functions to handle paths with spaces
        char temp_path[MAX_PATH_LENGTH];
        strncpy(temp_path, output_dir, MAX_PATH_LENGTH - 1);
        temp_path[MAX_PATH_LENGTH - 1] = '\0';

        // Create parent directories one by one
        for (char *p = temp_path + 1; *p; p++) {
            if (*p == '/') {
                *p = '\0';
                if (mkdir(temp_path, 0777) != 0 && errno != EEXIST) {
                    log_warn("Failed to create parent directory: %s (error: %s)", temp_path, strerror(errno));
                }
                *p = '/';
            }
        }

        // Create the final directory
        if (mkdir(temp_path, 0777) != 0 && errno != EEXIST) {
            log_error("Failed to create output directory: %s (error: %s)", temp_path, strerror(errno));
            return -1;
        }

        // Verify the directory was created
        if (stat(output_dir, &st) != 0 || !S_ISDIR(st.st_mode)) {
            log_error("Failed to verify output directory: %s", output_dir);
            return -1;
        }

        // Set permissions directly using chmod
        if (chmod(output_dir, 0777) != 0) {
            log_warn("Failed to set permissions on directory: %s (error: %s)", output_dir, strerror(errno));
        }

        log_info("Successfully created output directory: %s", output_dir);
    }

    // Check directory permissions
    if (access(output_dir, W_OK) != 0) {
        log_error("Output directory is not writable: %s", output_dir);

        // Try to fix permissions using direct chmod
        if (chmod(output_dir, 0777) != 0) {
            log_warn("Failed to set permissions on directory: %s (error: %s)", output_dir, strerror(errno));
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

        // Create parent directory using direct C functions
        char temp_path[MAX_PATH_LENGTH];
        strncpy(temp_path, parent_dir, MAX_PATH_LENGTH - 1);
        temp_path[MAX_PATH_LENGTH - 1] = '\0';

        // Create parent directories one by one
        for (char *p = temp_path + 1; *p; p++) {
            if (*p == '/') {
                *p = '\0';
                if (mkdir(temp_path, 0755) != 0 && errno != EEXIST) {
                    log_warn("Failed to create parent directory: %s (error: %s)", temp_path, strerror(errno));
                }
                *p = '/';
            }
        }

        // Create the final directory
        if (mkdir(temp_path, 0755) != 0 && errno != EEXIST) {
            log_warn("Failed to create parent directory: %s (error: %s)", temp_path, strerror(errno));
        }

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

            // Try to create parent directory with full permissions using direct C functions
            char temp_path[MAX_PATH_LENGTH];
            strncpy(temp_path, parent_dir, MAX_PATH_LENGTH - 1);
            temp_path[MAX_PATH_LENGTH - 1] = '\0';

            // Create parent directories one by one
            for (char *p = temp_path + 1; *p; p++) {
                if (*p == '/') {
                    *p = '\0';
                    if (mkdir(temp_path, 0755) != 0 && errno != EEXIST) {
                        log_warn("Failed to create parent directory: %s (error: %s)", temp_path, strerror(errno));
                    } else {
                        // Set permissions
                        chmod(temp_path, 0777);
                    }
                    *p = '/';
                }
            }

            // Create the final directory
            if (mkdir(temp_path, 0777) != 0 && errno != EEXIST) {
                log_warn("Failed to create parent directory: %s (error: %s)", temp_path, strerror(errno));
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

    // Use storage_path_hls if specified, otherwise fall back to storage_path
    const char *base_storage_path = global_config->storage_path;
    if (global_config->storage_path_hls[0] != '\0') {
        base_storage_path = global_config->storage_path_hls;
        log_info("Using dedicated HLS storage path for clearing segments: %s", base_storage_path);
    }

    char stream_hls_dir[MAX_PATH_LENGTH];
    snprintf(stream_hls_dir, MAX_PATH_LENGTH, "%s/hls/%s",
             base_storage_path, stream_name);

    // Check if the directory exists
    struct stat st;
    if (stat(stream_hls_dir, &st) != 0 || !S_ISDIR(st.st_mode)) {
        log_info("HLS directory for stream %s does not exist, nothing to clear", stream_name);
        return 0;
    }

    log_info("Clearing HLS segments for stream: %s in directory: %s", stream_name, stream_hls_dir);

    // Remove all .ts segment files using direct C functions
    DIR *dir = opendir(stream_hls_dir);
    if (dir) {
        struct dirent *entry;
        int removed_count = 0;

        while ((entry = readdir(dir)) != NULL) {
            // Check if this is a .ts file
            if (strstr(entry->d_name, ".ts") != NULL) {
                char file_path[MAX_PATH_LENGTH];
                snprintf(file_path, sizeof(file_path), "%s/%s", stream_hls_dir, entry->d_name);

                if (unlink(file_path) == 0) {
                    removed_count++;
                } else {
                    log_warn("Failed to remove HLS .ts file: %s (error: %s)",
                            file_path, strerror(errno));
                }
            }
        }

        closedir(dir);
        log_info("Removed %d HLS .ts segment files in %s", removed_count, stream_hls_dir);
    } else {
        log_warn("Failed to open directory to remove .ts files: %s (error: %s)",
                stream_hls_dir, strerror(errno));
    }

    // Remove all .m4s segment files (for fMP4) using direct C functions
    dir = opendir(stream_hls_dir);
    if (dir) {
        struct dirent *entry;
        int removed_count = 0;

        while ((entry = readdir(dir)) != NULL) {
            // Check if this is a .m4s file
            if (strstr(entry->d_name, ".m4s") != NULL) {
                char file_path[MAX_PATH_LENGTH];
                snprintf(file_path, sizeof(file_path), "%s/%s", stream_hls_dir, entry->d_name);

                if (unlink(file_path) == 0) {
                    removed_count++;
                } else {
                    log_warn("Failed to remove HLS .m4s file: %s (error: %s)",
                            file_path, strerror(errno));
                }
            }
        }

        closedir(dir);
        log_info("Removed %d HLS .m4s segment files in %s", removed_count, stream_hls_dir);
    } else {
        log_warn("Failed to open directory to remove .m4s files: %s (error: %s)",
                stream_hls_dir, strerror(errno));
    }

    // Remove init.mp4 file (for fMP4) using direct C function
    char init_file_path[MAX_PATH_LENGTH];
    snprintf(init_file_path, sizeof(init_file_path), "%s/init.mp4", stream_hls_dir);

    if (unlink(init_file_path) == 0) {
        log_info("Removed HLS init.mp4 file in %s", stream_hls_dir);
    } else if (errno != ENOENT) { // Only log if error is not "file not found"
        log_warn("Failed to remove HLS init.mp4 file in %s (error: %s)",
                stream_hls_dir, strerror(errno));
    }

    // Remove all .m3u8 playlist files using direct C functions
    dir = opendir(stream_hls_dir);
    if (dir) {
        struct dirent *entry;
        int removed_count = 0;

        while ((entry = readdir(dir)) != NULL) {
            // Check if this is a .m3u8 file
            if (strstr(entry->d_name, ".m3u8") != NULL) {
                char file_path[MAX_PATH_LENGTH];
                snprintf(file_path, sizeof(file_path), "%s/%s", stream_hls_dir, entry->d_name);

                if (unlink(file_path) == 0) {
                    removed_count++;
                } else {
                    log_warn("Failed to remove HLS .m3u8 file: %s (error: %s)",
                            file_path, strerror(errno));
                }
            }
        }

        closedir(dir);
        log_info("Removed %d HLS .m3u8 playlist files in %s", removed_count, stream_hls_dir);
    } else {
        log_warn("Failed to open directory to remove .m3u8 files: %s (error: %s)",
                stream_hls_dir, strerror(errno));
    }

    // Ensure the directory has proper permissions using direct chmod
    if (chmod(stream_hls_dir, 0777) != 0) {
        log_warn("Failed to set permissions on directory: %s (error: %s)",
                stream_hls_dir, strerror(errno));
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

    // Use storage_path_hls if specified, otherwise fall back to storage_path
    const char *base_storage_path = global_config->storage_path;
    if (global_config->storage_path_hls[0] != '\0') {
        base_storage_path = global_config->storage_path_hls;
        log_info("Using dedicated HLS storage path for cleanup: %s", base_storage_path);
    }

    char hls_base_dir[MAX_PATH_LENGTH];
    snprintf(hls_base_dir, MAX_PATH_LENGTH, "%s/hls", base_storage_path);

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

                // Remove temporary .m3u8.tmp files using direct C functions
                DIR *stream_dir = opendir(stream_hls_dir);
                if (stream_dir) {
                    struct dirent *file_entry;
                    int removed_count = 0;

                    while ((file_entry = readdir(stream_dir)) != NULL) {
                        // Check if this is a .m3u8.tmp file
                        if (strstr(file_entry->d_name, ".m3u8.tmp") != NULL) {
                            char file_path[MAX_PATH_LENGTH];
                            snprintf(file_path, sizeof(file_path), "%s/%s", stream_hls_dir, file_entry->d_name);

                            if (unlink(file_path) == 0) {
                                removed_count++;
                            } else {
                                log_warn("Failed to remove temporary file: %s (error: %s)",
                                        file_path, strerror(errno));
                            }
                        }
                    }

                    closedir(stream_dir);
                    log_info("Removed %d temporary .m3u8.tmp files in %s", removed_count, stream_hls_dir);
                } else {
                    log_warn("Failed to open directory to remove temporary files: %s (error: %s)",
                            stream_hls_dir, strerror(errno));
                }

                // Only remove segments that are older than 5 minutes using direct C functions
                // This ensures we don't delete segments that might still be in use
                stream_dir = opendir(stream_hls_dir);
                if (stream_dir) {
                    struct dirent *file_entry;
                    int removed_count = 0;
                    time_t current_time = time(NULL);
                    time_t five_minutes_ago = current_time - (5 * 60); // 5 minutes in seconds

                    while ((file_entry = readdir(stream_dir)) != NULL) {
                        // Check if this is a .ts file
                        if (strstr(file_entry->d_name, ".ts") != NULL) {
                            char file_path[MAX_PATH_LENGTH];
                            snprintf(file_path, sizeof(file_path), "%s/%s", stream_hls_dir, file_entry->d_name);

                            // Check file modification time
                            struct stat file_stat;
                            if (stat(file_path, &file_stat) == 0) {
                                if (file_stat.st_mtime < five_minutes_ago) {
                                    // File is older than 5 minutes, delete it
                                    if (unlink(file_path) == 0) {
                                        removed_count++;
                                    } else {
                                        log_warn("Failed to remove old .ts file: %s (error: %s)",
                                                file_path, strerror(errno));
                                    }
                                }
                            }
                        }
                    }

                    closedir(stream_dir);
                    log_info("Removed %d old .ts segment files in %s", removed_count, stream_hls_dir);
                } else {
                    log_warn("Failed to open directory to remove old .ts files: %s (error: %s)",
                            stream_hls_dir, strerror(errno));
                }

                // Also clean up old .m4s segments (for fMP4) using direct C functions
                stream_dir = opendir(stream_hls_dir);
                if (stream_dir) {
                    struct dirent *file_entry;
                    int removed_count = 0;
                    time_t current_time = time(NULL);
                    time_t five_minutes_ago = current_time - (5 * 60); // 5 minutes in seconds

                    while ((file_entry = readdir(stream_dir)) != NULL) {
                        // Check if this is a .m4s file
                        if (strstr(file_entry->d_name, ".m4s") != NULL) {
                            char file_path[MAX_PATH_LENGTH];
                            snprintf(file_path, sizeof(file_path), "%s/%s", stream_hls_dir, file_entry->d_name);

                            // Check file modification time
                            struct stat file_stat;
                            if (stat(file_path, &file_stat) == 0) {
                                if (file_stat.st_mtime < five_minutes_ago) {
                                    // File is older than 5 minutes, delete it
                                    if (unlink(file_path) == 0) {
                                        removed_count++;
                                    } else {
                                        log_warn("Failed to remove old .m4s file: %s (error: %s)",
                                                file_path, strerror(errno));
                                    }
                                }
                            }
                        }
                    }

                    closedir(stream_dir);
                    log_info("Removed %d old .m4s segment files in %s", removed_count, stream_hls_dir);
                } else {
                    log_warn("Failed to open directory to remove old .m4s files: %s (error: %s)",
                            stream_hls_dir, strerror(errno));
                }

                log_info("Cleaned up temporary files for active stream: %s", entry->d_name);
            } else {
                // For inactive streams, we can safely remove all files
                log_info("Stream %s is inactive, removing all HLS files", entry->d_name);

                // Remove all .ts segment files using direct C functions
                DIR *stream_dir = opendir(stream_hls_dir);
                if (stream_dir) {
                    struct dirent *file_entry;
                    int removed_count = 0;

                    while ((file_entry = readdir(stream_dir)) != NULL) {
                        // Check if this is a .ts file
                        if (strstr(file_entry->d_name, ".ts") != NULL) {
                            char file_path[MAX_PATH_LENGTH];
                            snprintf(file_path, sizeof(file_path), "%s/%s", stream_hls_dir, file_entry->d_name);

                            if (unlink(file_path) == 0) {
                                removed_count++;
                            } else {
                                log_warn("Failed to remove HLS .ts file: %s (error: %s)",
                                        file_path, strerror(errno));
                            }
                        }
                    }

                    closedir(stream_dir);
                    log_info("Removed %d HLS .ts segment files in %s", removed_count, stream_hls_dir);
                } else {
                    log_warn("Failed to open directory to remove .ts files: %s (error: %s)",
                            stream_hls_dir, strerror(errno));
                }

                // Remove all .m4s segment files (for fMP4) using direct C functions
                stream_dir = opendir(stream_hls_dir);
                if (stream_dir) {
                    struct dirent *file_entry;
                    int removed_count = 0;

                    while ((file_entry = readdir(stream_dir)) != NULL) {
                        // Check if this is a .m4s file
                        if (strstr(file_entry->d_name, ".m4s") != NULL) {
                            char file_path[MAX_PATH_LENGTH];
                            snprintf(file_path, sizeof(file_path), "%s/%s", stream_hls_dir, file_entry->d_name);

                            if (unlink(file_path) == 0) {
                                removed_count++;
                            } else {
                                log_warn("Failed to remove HLS .m4s file: %s (error: %s)",
                                        file_path, strerror(errno));
                            }
                        }
                    }

                    closedir(stream_dir);
                    log_info("Removed %d HLS .m4s segment files in %s", removed_count, stream_hls_dir);
                } else {
                    log_warn("Failed to open directory to remove .m4s files: %s (error: %s)",
                            stream_hls_dir, strerror(errno));
                }

                // Remove init.mp4 file (for fMP4) using direct C function
                char init_file_path[MAX_PATH_LENGTH];
                snprintf(init_file_path, sizeof(init_file_path), "%s/init.mp4", stream_hls_dir);

                if (unlink(init_file_path) == 0) {
                    log_info("Removed HLS init.mp4 file in %s", stream_hls_dir);
                } else if (errno != ENOENT) { // Only log if error is not "file not found"
                    log_warn("Failed to remove HLS init.mp4 file in %s (error: %s)",
                            stream_hls_dir, strerror(errno));
                }

                // Remove all .m3u8 playlist files using direct C functions
                stream_dir = opendir(stream_hls_dir);
                if (stream_dir) {
                    struct dirent *file_entry;
                    int removed_count = 0;

                    while ((file_entry = readdir(stream_dir)) != NULL) {
                        // Check if this is a .m3u8 file
                        if (strstr(file_entry->d_name, ".m3u8") != NULL) {
                            char file_path[MAX_PATH_LENGTH];
                            snprintf(file_path, sizeof(file_path), "%s/%s", stream_hls_dir, file_entry->d_name);

                            if (unlink(file_path) == 0) {
                                removed_count++;
                            } else {
                                log_warn("Failed to remove HLS .m3u8 file: %s (error: %s)",
                                        file_path, strerror(errno));
                            }
                        }
                    }

                    closedir(stream_dir);
                    log_info("Removed %d HLS .m3u8 playlist files in %s", removed_count, stream_hls_dir);
                } else {
                    log_warn("Failed to open directory to remove .m3u8 files: %s (error: %s)",
                            stream_hls_dir, strerror(errno));
                }
            }

            // Ensure the directory has proper permissions using direct chmod
            if (chmod(stream_hls_dir, 0777) != 0) {
                log_warn("Failed to set permissions on directory: %s (error: %s)",
                        stream_hls_dir, strerror(errno));
            }
        }
    }

    closedir(dir);
    log_info("HLS directory cleanup completed");
}
