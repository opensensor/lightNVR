#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <errno.h>
#include <sys/stat.h>
#include <sys/types.h>
#include <libgen.h>
#include <ctype.h>

#include "../../include/core/config.h"
#include "../../include/core/logger.h"

// Default configuration values
void load_default_config(config_t *config) {
    if (!config) return;
    
    // Clear the structure
    memset(config, 0, sizeof(config_t));
    
    // General settings
    snprintf(config->pid_file, MAX_PATH_LENGTH, "/var/run/lightnvr.pid");
    snprintf(config->log_file, MAX_PATH_LENGTH, "/var/log/lightnvr.log");
    config->log_level = LOG_INFO;
    
    // Storage settings
    snprintf(config->storage_path, MAX_PATH_LENGTH, "/var/lib/lightnvr/recordings");
    config->max_storage_size = 0; // 0 means unlimited
    config->retention_days = 30;
    config->auto_delete_oldest = true;
    
    // Database settings
    snprintf(config->db_path, MAX_PATH_LENGTH, "/var/lib/lightnvr/lightnvr.db");
    
    // Web server settings
    config->web_port = 8080;
    snprintf(config->web_root, MAX_PATH_LENGTH, "/var/lib/lightnvr/www");
    config->web_auth_enabled = true;
    snprintf(config->web_username, 32, "admin");
    snprintf(config->web_password, 32, "admin"); // Default password, should be changed
    
    // Stream settings
    config->max_streams = 16;
    
    // Memory optimization
    config->buffer_size = 1024; // 1MB buffer size
    config->use_swap = true;
    snprintf(config->swap_file, MAX_PATH_LENGTH, "/var/lib/lightnvr/swap");
    config->swap_size = 128 * 1024 * 1024; // 128MB swap
    
    // Hardware acceleration
    config->hw_accel_enabled = false;
    memset(config->hw_accel_device, 0, 32);
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

// Ensure all required directories exist
static int ensure_directories(const config_t *config) {
    // Storage directory
    if (create_directory(config->storage_path) != 0) {
        log_error("Failed to create storage directory: %s", config->storage_path);
        return -1;
    }
    
    // Database directory
    char db_dir[MAX_PATH_LENGTH];
    strncpy(db_dir, config->db_path, MAX_PATH_LENGTH);
    char *dir = dirname(db_dir);
    if (create_directory(dir) != 0) {
        log_error("Failed to create database directory: %s", dir);
        return -1;
    }
    
    // Web root directory
    if (create_directory(config->web_root) != 0) {
        log_error("Failed to create web root directory: %s", config->web_root);
        return -1;
    }
    
    // Log directory
    char log_dir[MAX_PATH_LENGTH];
    strncpy(log_dir, config->log_file, MAX_PATH_LENGTH);
    dir = dirname(log_dir);
    if (create_directory(dir) != 0) {
        log_error("Failed to create log directory: %s", dir);
        return -1;
    }
    
    return 0;
}

// Validate configuration values
int validate_config(const config_t *config) {
    if (!config) return -1;
    
    // Check for required paths
    if (strlen(config->storage_path) == 0) {
        log_error("Storage path is required");
        return -1;
    }
    
    if (strlen(config->db_path) == 0) {
        log_error("Database path is required");
        return -1;
    }
    
    if (strlen(config->web_root) == 0) {
        log_error("Web root path is required");
        return -1;
    }
    
    // Check web port
    if (config->web_port <= 0 || config->web_port > 65535) {
        log_error("Invalid web port: %d", config->web_port);
        return -1;
    }
    
    // Check max streams
    if (config->max_streams <= 0 || config->max_streams > MAX_STREAMS) {
        log_error("Invalid max streams: %d (must be 1-%d)", config->max_streams, MAX_STREAMS);
        return -1;
    }
    
    // Check buffer size
    if (config->buffer_size <= 0) {
        log_error("Invalid buffer size: %d", config->buffer_size);
        return -1;
    }
    
    // Check swap size
    if (config->use_swap && config->swap_size <= 0) {
        log_error("Invalid swap size: %llu", (unsigned long long)config->swap_size);
        return -1;
    }
    
    return 0;
}

// Load configuration from file
static int load_config_from_file(const char *filename, config_t *config) {
    FILE *file = fopen(filename, "r");
    if (!file) {
        log_warn("Could not open config file: %s", filename);
        return -1;
    }
    
    char line[1024];
    char key[256];
    char value[768];
    
    while (fgets(line, sizeof(line), file)) {
        // Skip comments and empty lines
        if (line[0] == '#' || line[0] == '\n') {
            continue;
        }
        
        // Parse key=value
        if (sscanf(line, "%255[^=]=%767[^\n]", key, value) == 2) {
            // Remove trailing whitespace from key
            char *end = key + strlen(key) - 1;
            while (end > key && isspace(*end)) {
                *end-- = '\0';
            }
            
            // Remove leading whitespace from value
            char *start = value;
            while (*start && isspace(*start)) {
                start++;
            }
            
            // Process key-value pair
            if (strcmp(key, "pid_file") == 0) {
                strncpy(config->pid_file, start, MAX_PATH_LENGTH - 1);
            } else if (strcmp(key, "log_file") == 0) {
                strncpy(config->log_file, start, MAX_PATH_LENGTH - 1);
            } else if (strcmp(key, "log_level") == 0) {
                config->log_level = atoi(start);
            } else if (strcmp(key, "storage_path") == 0) {
                strncpy(config->storage_path, start, MAX_PATH_LENGTH - 1);
            } else if (strcmp(key, "max_storage_size") == 0) {
                config->max_storage_size = strtoull(start, NULL, 10);
            } else if (strcmp(key, "retention_days") == 0) {
                config->retention_days = atoi(start);
            } else if (strcmp(key, "auto_delete_oldest") == 0) {
                config->auto_delete_oldest = (strcmp(start, "true") == 0 || strcmp(start, "1") == 0);
            } else if (strcmp(key, "db_path") == 0) {
                strncpy(config->db_path, start, MAX_PATH_LENGTH - 1);
            } else if (strcmp(key, "web_port") == 0) {
                config->web_port = atoi(start);
            } else if (strcmp(key, "web_root") == 0) {
                strncpy(config->web_root, start, MAX_PATH_LENGTH - 1);
            } else if (strcmp(key, "web_auth_enabled") == 0) {
                config->web_auth_enabled = (strcmp(start, "true") == 0 || strcmp(start, "1") == 0);
            } else if (strcmp(key, "web_username") == 0) {
                strncpy(config->web_username, start, 31);
            } else if (strcmp(key, "web_password") == 0) {
                strncpy(config->web_password, start, 31);
            } else if (strcmp(key, "max_streams") == 0) {
                config->max_streams = atoi(start);
            } else if (strcmp(key, "buffer_size") == 0) {
                config->buffer_size = atoi(start);
            } else if (strcmp(key, "use_swap") == 0) {
                config->use_swap = (strcmp(start, "true") == 0 || strcmp(start, "1") == 0);
            } else if (strcmp(key, "swap_file") == 0) {
                strncpy(config->swap_file, start, MAX_PATH_LENGTH - 1);
            } else if (strcmp(key, "swap_size") == 0) {
                config->swap_size = strtoull(start, NULL, 10);
            } else if (strcmp(key, "hw_accel_enabled") == 0) {
                config->hw_accel_enabled = (strcmp(start, "true") == 0 || strcmp(start, "1") == 0);
            } else if (strcmp(key, "hw_accel_device") == 0) {
                strncpy(config->hw_accel_device, start, 31);
            } else if (strncmp(key, "stream.", 7) == 0) {
                // Stream configuration
                char stream_key[256];
                int stream_index;
                
                if (sscanf(key, "stream.%d.%255s", &stream_index, stream_key) == 2) {
                    if (stream_index >= 0 && stream_index < MAX_STREAMS) {
                        if (strcmp(stream_key, "name") == 0) {
                            strncpy(config->streams[stream_index].name, start, MAX_STREAM_NAME - 1);
                        } else if (strcmp(stream_key, "url") == 0) {
                            strncpy(config->streams[stream_index].url, start, MAX_URL_LENGTH - 1);
                        } else if (strcmp(stream_key, "enabled") == 0) {
                            config->streams[stream_index].enabled = (strcmp(start, "true") == 0 || strcmp(start, "1") == 0);
                        } else if (strcmp(stream_key, "width") == 0) {
                            config->streams[stream_index].width = atoi(start);
                        } else if (strcmp(stream_key, "height") == 0) {
                            config->streams[stream_index].height = atoi(start);
                        } else if (strcmp(stream_key, "fps") == 0) {
                            config->streams[stream_index].fps = atoi(start);
                        } else if (strcmp(stream_key, "codec") == 0) {
                            strncpy(config->streams[stream_index].codec, start, 15);
                        } else if (strcmp(stream_key, "priority") == 0) {
                            config->streams[stream_index].priority = atoi(start);
                        } else if (strcmp(stream_key, "record") == 0) {
                            config->streams[stream_index].record = (strcmp(start, "true") == 0 || strcmp(start, "1") == 0);
                        } else if (strcmp(stream_key, "segment_duration") == 0) {
                            config->streams[stream_index].segment_duration = atoi(start);
                        }
                    }
                }
            }
        }
    }
    
    fclose(file);
    return 0;
}

// Load configuration
int load_config(config_t *config) {
    if (!config) return -1;
    
    // Load default configuration
    load_default_config(config);
    
    // Try to load from config file
    const char *config_paths[] = {
        "./lightnvr.conf",
        "/etc/lightnvr/lightnvr.conf",
        NULL
    };
    
    int loaded = 0;
    for (int i = 0; config_paths[i] != NULL; i++) {
        if (access(config_paths[i], R_OK) == 0) {
            if (load_config_from_file(config_paths[i], config) == 0) {
                log_info("Loaded configuration from %s", config_paths[i]);
                loaded = 1;
                break;
            }
        }
    }
    
    if (!loaded) {
        log_warn("No configuration file found, using defaults");
    }

    // In load_config() or wherever you're setting up config:
    if (strlen(config->web_root) == 0) {
        // Set a default web root path
        strcpy(config->web_root, "/var/www/lightnvr");  // or another appropriate default
    }

    // Add logging to debug
    log_info("Using web root: %s", config->web_root);

    // Make sure the directory exists
    struct stat st;
    if (stat(config->web_root, &st) != 0 || !S_ISDIR(st.st_mode)) {
        log_warn("Web root directory %s does not exist or is not a directory", config->web_root);
        // Possibly create it or use a fallback
    }
    
    // Validate configuration
    if (validate_config(config) != 0) {
        log_error("Invalid configuration");
        return -1;
    }
    
    // Ensure directories exist
    if (ensure_directories(config) != 0) {
        log_error("Failed to create required directories");
        return -1;
    }
    
    return 0;
}

// Save configuration to file
int save_config(const config_t *config, const char *path) {
    if (!config || !path) return -1;
    
    FILE *file = fopen(path, "w");
    if (!file) {
        log_error("Could not open config file for writing: %s", path);
        return -1;
    }
    
    // Write general settings
    fprintf(file, "# LightNVR Configuration File\n\n");
    fprintf(file, "# General Settings\n");
    fprintf(file, "pid_file=%s\n", config->pid_file);
    fprintf(file, "log_file=%s\n", config->log_file);
    fprintf(file, "log_level=%d\n", config->log_level);
    
    // Write storage settings
    fprintf(file, "\n# Storage Settings\n");
    fprintf(file, "storage_path=%s\n", config->storage_path);
    fprintf(file, "max_storage_size=%llu\n", (unsigned long long)config->max_storage_size);
    fprintf(file, "retention_days=%d\n", config->retention_days);
    fprintf(file, "auto_delete_oldest=%s\n", config->auto_delete_oldest ? "true" : "false");
    
    // Write database settings
    fprintf(file, "\n# Database Settings\n");
    fprintf(file, "db_path=%s\n", config->db_path);
    
    // Write web server settings
    fprintf(file, "\n# Web Server Settings\n");
    fprintf(file, "web_port=%d\n", config->web_port);
    fprintf(file, "web_root=%s\n", config->web_root);
    fprintf(file, "web_auth_enabled=%s\n", config->web_auth_enabled ? "true" : "false");
    fprintf(file, "web_username=%s\n", config->web_username);
    fprintf(file, "web_password=%s\n", config->web_password);
    
    // Write stream settings
    fprintf(file, "\n# Stream Settings\n");
    fprintf(file, "max_streams=%d\n", config->max_streams);
    
    // Write memory optimization settings
    fprintf(file, "\n# Memory Optimization\n");
    fprintf(file, "buffer_size=%d\n", config->buffer_size);
    fprintf(file, "use_swap=%s\n", config->use_swap ? "true" : "false");
    fprintf(file, "swap_file=%s\n", config->swap_file);
    fprintf(file, "swap_size=%llu\n", (unsigned long long)config->swap_size);
    
    // Write hardware acceleration settings
    fprintf(file, "\n# Hardware Acceleration\n");
    fprintf(file, "hw_accel_enabled=%s\n", config->hw_accel_enabled ? "true" : "false");
    fprintf(file, "hw_accel_device=%s\n", config->hw_accel_device);
    
    // Write stream configurations
    fprintf(file, "\n# Stream Configurations\n");
    for (int i = 0; i < config->max_streams; i++) {
        if (strlen(config->streams[i].name) > 0) {
            fprintf(file, "\n# Stream %d\n", i);
            fprintf(file, "stream.%d.name=%s\n", i, config->streams[i].name);
            fprintf(file, "stream.%d.url=%s\n", i, config->streams[i].url);
            fprintf(file, "stream.%d.enabled=%s\n", i, config->streams[i].enabled ? "true" : "false");
            fprintf(file, "stream.%d.width=%d\n", i, config->streams[i].width);
            fprintf(file, "stream.%d.height=%d\n", i, config->streams[i].height);
            fprintf(file, "stream.%d.fps=%d\n", i, config->streams[i].fps);
            fprintf(file, "stream.%d.codec=%s\n", i, config->streams[i].codec);
            fprintf(file, "stream.%d.priority=%d\n", i, config->streams[i].priority);
            fprintf(file, "stream.%d.record=%s\n", i, config->streams[i].record ? "true" : "false");
            fprintf(file, "stream.%d.segment_duration=%d\n", i, config->streams[i].segment_duration);
        }
    }
    
    fclose(file);
    return 0;
}

// Print configuration to stdout (for debugging)
void print_config(const config_t *config) {
    if (!config) return;
    
    printf("LightNVR Configuration:\n");
    printf("  General Settings:\n");
    printf("    PID File: %s\n", config->pid_file);
    printf("    Log File: %s\n", config->log_file);
    printf("    Log Level: %d\n", config->log_level);
    
    printf("  Storage Settings:\n");
    printf("    Storage Path: %s\n", config->storage_path);
    printf("    Max Storage Size: %llu bytes\n", (unsigned long long)config->max_storage_size);
    printf("    Retention Days: %d\n", config->retention_days);
    printf("    Auto Delete Oldest: %s\n", config->auto_delete_oldest ? "true" : "false");
    
    printf("  Database Settings:\n");
    printf("    Database Path: %s\n", config->db_path);
    
    printf("  Web Server Settings:\n");
    printf("    Web Port: %d\n", config->web_port);
    printf("    Web Root: %s\n", config->web_root);
    printf("    Web Auth Enabled: %s\n", config->web_auth_enabled ? "true" : "false");
    printf("    Web Username: %s\n", config->web_username);
    printf("    Web Password: %s\n", "********");
    
    printf("  Stream Settings:\n");
    printf("    Max Streams: %d\n", config->max_streams);
    
    printf("  Memory Optimization:\n");
    printf("    Buffer Size: %d KB\n", config->buffer_size);
    printf("    Use Swap: %s\n", config->use_swap ? "true" : "false");
    printf("    Swap File: %s\n", config->swap_file);
    printf("    Swap Size: %llu bytes\n", (unsigned long long)config->swap_size);
    
    printf("  Hardware Acceleration:\n");
    printf("    HW Accel Enabled: %s\n", config->hw_accel_enabled ? "true" : "false");
    printf("    HW Accel Device: %s\n", config->hw_accel_device);
    
    printf("  Stream Configurations:\n");
    for (int i = 0; i < config->max_streams; i++) {
        if (strlen(config->streams[i].name) > 0) {
            printf("    Stream %d:\n", i);
            printf("      Name: %s\n", config->streams[i].name);
            printf("      URL: %s\n", config->streams[i].url);
            printf("      Enabled: %s\n", config->streams[i].enabled ? "true" : "false");
            printf("      Resolution: %dx%d\n", config->streams[i].width, config->streams[i].height);
            printf("      FPS: %d\n", config->streams[i].fps);
            printf("      Codec: %s\n", config->streams[i].codec);
            printf("      Priority: %d\n", config->streams[i].priority);
            printf("      Record: %s\n", config->streams[i].record ? "true" : "false");
            printf("      Segment Duration: %d seconds\n", config->streams[i].segment_duration);
        }
    }
}
