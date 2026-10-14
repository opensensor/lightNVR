#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <dirent.h>
#include <sys/stat.h>
#include <fcntl.h>
#include <time.h>
#include <errno.h>

#include "core/logger.h"
#include "core/config.h"
#include "core/path_utils.h"
#include "utils/strings.h"
#include "video/streams.h"
#include "video/hls/hls_directory.h"
#include "video/hls/hls_context.h"
#include "video/hls/hls_cache_cleanup.h"

void cleanup_stale_hls_segments(void) {
    // Only storage paths are needed; avoid refreshing the full camera catalog
    // from the storage heartbeat or borrowing its shared mutable snapshot.
    const config_t *config = &g_config;
    const char *base = config->storage_path_hls[0]
        ? config->storage_path_hls : config->storage_path;
    if (!base[0]) return;
    char root[MAX_PATH_LENGTH];
    int length = snprintf(root, sizeof(root), "%s/hls", base);
    if (length < 0 || (size_t)length >= sizeof(root)) {
        log_error("HLS cache cleanup path exceeds maximum length");
        return;
    }
    hls_cache_cleanup_result_t result = {0};
    int status = hls_prune_cache(root, time(NULL), HLS_CACHE_MIN_AGE_SECONDS,
                                HLS_CACHE_MAX_REMOVALS, false, &result);
    if (result.removed) {
        log_info("HLS cache cleanup: removed=%u freed=%llu bytes",
                 result.removed, (unsigned long long)result.bytes);
    }
    if (status || result.skipped_directories) {
        log_warn("HLS cache cleanup incomplete: errors=%u skipped_directories=%u",
                 result.errors, result.skipped_directories);
    }
}

/**
 * Ensure the HLS output directory exists and is writable
 */
int ensure_hls_directory(char *output_dir, size_t output_dir_size, const char *stream_name) {
    if (output_dir == NULL || output_dir_size == 0 || stream_name == NULL) {
        return -1;
    }

    // Get the global config for storage path
    const config_t *global_config = get_streaming_config();
    if (!global_config) {
        log_error("Failed to get global config for HLS directory");
        return -1;
    }

    //  Always use the consistent path structure for HLS
    // Use storage_path_hls if specified, otherwise fall back to storage_path
    const char *base_storage_path = global_config->storage_path;

    // Check if storage_path_hls is specified and not empty
    if (global_config->storage_path_hls[0] != '\0') {
        base_storage_path = global_config->storage_path_hls;
        log_info("Using dedicated HLS storage path: %s", base_storage_path);
    }

    // Make sure we're using a valid path.
    char stream_path[MAX_STREAM_NAME];
    sanitize_stream_name(stream_name, stream_path, MAX_STREAM_NAME);

    snprintf(output_dir, output_dir_size, "%s/hls/%s",
            base_storage_path, stream_path);

    // Verify output directory exists and is writable
    if (mkdir_recursive(output_dir)) {
        log_error("Failed to create output directory: %s", output_dir);
        return -1;
    }

    // Ensure the directory is writable.
    if (chmod_path(output_dir, 0755) != 0) {
        log_warn("Failed to set permissions on output directory: %s", output_dir);
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

    const config_t *global_config = get_streaming_config();
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

    // Make sure we're using a valid path.
    char stream_path[MAX_STREAM_NAME];
    sanitize_stream_name(stream_name, stream_path, MAX_STREAM_NAME);

    char stream_hls_dir[MAX_PATH_LENGTH];
    snprintf(stream_hls_dir, MAX_PATH_LENGTH, "%s/hls/%s",
             base_storage_path, stream_path);

    // Check if the directory exists
    struct stat st;
    if (stat(stream_hls_dir, &st) != 0 || !S_ISDIR(st.st_mode)) {
        log_info("HLS directory for stream %s does not exist, nothing to clear", stream_name);
        return 0;
    }

    log_info("Clearing HLS segments for stream: %s in directory: %s", stream_name, stream_hls_dir);

    const char *to_remove[] = {".ts", ".m4s", ".m3u8"};

    // Remove all .ts segment files using direct C functions
    DIR *dir = opendir(stream_hls_dir);
    if (dir) {
        const struct dirent *entry;
        int removed_count = 0;

        while ((entry = readdir(dir)) != NULL) {
            bool match = false;
            // Check if any extension matches
            for (int i = 0; i < sizeof(to_remove)/sizeof(char *); i++) {
                if (ends_with(entry->d_name, to_remove[i])) {
                    match = true;
                    break;
                }
            }

            if (match) {
                char file_path[MAX_PATH_LENGTH];
                snprintf(file_path, sizeof(file_path), "%s/%s", stream_hls_dir, entry->d_name);

                if (unlink(file_path) == 0) {
                    removed_count++;
                } else {
                    log_warn("Failed to remove HLS file: %s (error: %s)",
                            file_path, strerror(errno));
                }
            }
        }

        closedir(dir);
        log_info("Removed %d HLS files in %s", removed_count, stream_hls_dir);
    } else {
        log_warn("Failed to open directory to remove HLS files: %s (error: %s)",
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

    // Ensure the directory has proper permissions
    if (chmod_path(stream_hls_dir, 0755) != 0) {
        log_warn("Failed to set permissions on directory: %s", stream_hls_dir);
    }

    return 0;
}

/**
 * Clean up HLS directories during shutdown
 */
void cleanup_hls_directories(void) {
    const config_t *global_config = get_streaming_config();

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
    const struct dirent *entry;
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
            for (int i = 0; i < g_config.max_streams; i++) {
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
                    time_t five_minutes_ago = current_time - ((time_t)5 * 60); // 5 minutes in seconds

                    int dir_fd = dirfd(stream_dir);
                    while (dir_fd != -1 && (file_entry = readdir(stream_dir)) != NULL) {
                        // Check if this is a .ts file
                        if (strstr(file_entry->d_name, ".ts") != NULL) {
                            // Use fstatat+unlinkat with the directory fd to eliminate
                            // the TOCTOU race between lstat() and unlink() (#30)
                            struct stat file_stat;
                            if (fstatat(dir_fd, file_entry->d_name, &file_stat, AT_SYMLINK_NOFOLLOW) == 0
                                    && S_ISREG(file_stat.st_mode)) {
                                if (file_stat.st_mtime < five_minutes_ago) {
                                    // File is older than 5 minutes, delete it
                                    if (unlinkat(dir_fd, file_entry->d_name, 0) == 0) {
                                        removed_count++;
                                    } else {
                                        log_warn("Failed to remove old .ts file: %s/%s (error: %s)",
                                                stream_hls_dir, file_entry->d_name, strerror(errno));
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
                    time_t five_minutes_ago = current_time - ((time_t)5 * 60); // 5 minutes in seconds

                    int dir_fd = dirfd(stream_dir);
                    while (dir_fd != -1 && (file_entry = readdir(stream_dir)) != NULL) {
                        // Check if this is a .m4s file
                        if (strstr(file_entry->d_name, ".m4s") != NULL) {
                            // Use fstatat+unlinkat with the directory fd to eliminate
                            // the TOCTOU race between lstat() and unlink() (#31)
                            struct stat file_stat;
                            if (fstatat(dir_fd, file_entry->d_name, &file_stat, AT_SYMLINK_NOFOLLOW) == 0
                                    && S_ISREG(file_stat.st_mode)) {
                                if (file_stat.st_mtime < five_minutes_ago) {
                                    // File is older than 5 minutes, delete it
                                    if (unlinkat(dir_fd, file_entry->d_name, 0) == 0) {
                                        removed_count++;
                                    } else {
                                        log_warn("Failed to remove old .m4s file: %s/%s (error: %s)",
                                                stream_hls_dir, file_entry->d_name, strerror(errno));
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

            // Ensure the directory has proper permissions
            if (chmod_path(stream_hls_dir, 0755) != 0) {
                log_warn("Failed to set permissions on directory: %s (error: %s)",
                        stream_hls_dir, strerror(errno));
            }
        }
    }

    closedir(dir);
    log_info("HLS directory cleanup completed");
}
