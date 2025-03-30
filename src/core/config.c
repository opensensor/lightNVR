#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <errno.h>
#include <sys/stat.h>
#include <sys/types.h>
#include <libgen.h>
#include <ctype.h>

#include "ini.h"
#include "core/config.h"
#include "core/logger.h"
#include "database/database_manager.h"

// Global configuration variable
config_t g_config;

// Default configuration values
void load_default_config(config_t *config) {
    if (!config) return;
    
    // Clear the structure
    memset(config, 0, sizeof(config_t));
    
    // General settings
    snprintf(config->pid_file, MAX_PATH_LENGTH, "/var/run/lightnvr.pid");
    snprintf(config->log_file, MAX_PATH_LENGTH, "/var/log/lightnvr.log");
    config->log_level = LOG_LEVEL_INFO;
    
    // Storage settings
    snprintf(config->storage_path, MAX_PATH_LENGTH, "/var/lib/lightnvr/recordings");
    config->storage_path_hls[0] = '\0'; // Empty by default, will use storage_path if not specified
    config->max_storage_size = 0; // 0 means unlimited
    config->retention_days = 30;
    config->auto_delete_oldest = true;
    
    // Models settings
    snprintf(config->models_path, MAX_PATH_LENGTH, "/var/lib/lightnvr/models");
    
    // Database settings
    snprintf(config->db_path, MAX_PATH_LENGTH, "/var/lib/lightnvr/lightnvr.db");
    
    // Web server settings
    config->web_port = 8080;
    snprintf(config->web_root, MAX_PATH_LENGTH, "/var/lib/lightnvr/www");
    config->web_auth_enabled = true;
    snprintf(config->web_username, 32, "admin");
    snprintf(config->web_password, 32, "admin"); // Default password, should be changed
    config->connection_pool_threads = 8; // Default to 8 threads for the connection pool
    
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
    
    // go2rtc settings
    snprintf(config->go2rtc_binary_path, MAX_PATH_LENGTH, "/usr/local/bin/go2rtc");
    snprintf(config->go2rtc_config_dir, MAX_PATH_LENGTH, "/etc/lightnvr/go2rtc");
    config->go2rtc_api_port = 1984;
    
    // Initialize default values for detection-based recording in streams
    for (int i = 0; i < MAX_STREAMS; i++) {
        config->streams[i].detection_based_recording = false;
        config->streams[i].detection_model[0] = '\0';
        config->streams[i].detection_interval = 10; // Check every 10 frames
        config->streams[i].detection_threshold = 0.5f; // 50% confidence threshold
        config->streams[i].pre_detection_buffer = 5; // 5 seconds before detection
        config->streams[i].post_detection_buffer = 10; // 10 seconds after detection
        config->streams[i].streaming_enabled = true; // Enable streaming by default
        config->streams[i].record_audio = false; // Disable audio recording by default
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

// Ensure all required directories exist
static int ensure_directories(const config_t *config) {
    // Storage directory
    if (create_directory(config->storage_path) != 0) {
        log_error("Failed to create storage directory: %s", config->storage_path);
        return -1;
    }
    
    // HLS storage directory if specified
    if (config->storage_path_hls[0] != '\0') {
        if (create_directory(config->storage_path_hls) != 0) {
            log_error("Failed to create HLS storage directory: %s", config->storage_path_hls);
            return -1;
        }
        log_info("Created HLS storage directory: %s", config->storage_path_hls);
    }
    
    // Models directory
    if (create_directory(config->models_path) != 0) {
        log_error("Failed to create models directory: %s", config->models_path);
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
    
    // Ensure log file is writable
    FILE *test_file = fopen(config->log_file, "a");
    if (!test_file) {
        log_warn("Log file %s is not writable: %s", config->log_file, strerror(errno));
        // Try to create the directory with more permissive permissions
        if (chmod(dir, 0777) != 0) {
            log_warn("Failed to change log directory permissions: %s", strerror(errno));
        }
    } else {
        fclose(test_file);
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
    
    if (strlen(config->models_path) == 0) {
        log_error("Models path is required");
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

// Handler function for inih
static int config_ini_handler(void* user, const char* section, const char* name, const char* value) {
    config_t* config = (config_t*)user;
    
    // General settings
    if (strcmp(section, "general") == 0) {
        if (strcmp(name, "pid_file") == 0) {
            strncpy(config->pid_file, value, MAX_PATH_LENGTH - 1);
        } else if (strcmp(name, "log_file") == 0) {
            strncpy(config->log_file, value, MAX_PATH_LENGTH - 1);
        } else if (strcmp(name, "log_level") == 0) {
            config->log_level = atoi(value);
        }
    }
    // Storage settings
    else if (strcmp(section, "storage") == 0) {
        if (strcmp(name, "path") == 0) {
            strncpy(config->storage_path, value, MAX_PATH_LENGTH - 1);
        } else if (strcmp(name, "path_hls") == 0) {
            strncpy(config->storage_path_hls, value, MAX_PATH_LENGTH - 1);
        } else if (strcmp(name, "max_size") == 0) {
            config->max_storage_size = strtoull(value, NULL, 10);
        } else if (strcmp(name, "retention_days") == 0) {
            config->retention_days = atoi(value);
        } else if (strcmp(name, "auto_delete_oldest") == 0) {
            config->auto_delete_oldest = (strcmp(value, "true") == 0 || strcmp(value, "1") == 0);
        }
    }
    // Models settings
    else if (strcmp(section, "models") == 0) {
        if (strcmp(name, "path") == 0) {
            strncpy(config->models_path, value, MAX_PATH_LENGTH - 1);
        }
    }
    // Database settings
    else if (strcmp(section, "database") == 0) {
        if (strcmp(name, "path") == 0) {
            strncpy(config->db_path, value, MAX_PATH_LENGTH - 1);
        }
    }
    // Web server settings
    else if (strcmp(section, "web") == 0) {
        if (strcmp(name, "port") == 0) {
            config->web_port = atoi(value);
        } else if (strcmp(name, "root") == 0) {
            strncpy(config->web_root, value, MAX_PATH_LENGTH - 1);
        } else if (strcmp(name, "auth_enabled") == 0) {
            config->web_auth_enabled = (strcmp(value, "true") == 0 || strcmp(value, "1") == 0);
        } else if (strcmp(name, "username") == 0) {
            strncpy(config->web_username, value, 31);
        } else if (strcmp(name, "password") == 0) {
            strncpy(config->web_password, value, 31);
        } else if (strcmp(name, "connection_pool_threads") == 0) {
            config->connection_pool_threads = atoi(value);
        }
    }
    // Stream settings
    else if (strcmp(section, "streams") == 0) {
        if (strcmp(name, "max_streams") == 0) {
            config->max_streams = atoi(value);
        }
    }
    // Stream-specific settings (format: stream_name.setting)
    else if (strstr(section, "stream.") == section) {
        // Extract stream name from section (after "stream.")
        const char *stream_name = section + 7; // Skip "stream."
        
        // Find the stream with this name
        int stream_idx = -1;
        for (int i = 0; i < config->max_streams; i++) {
            if (strcmp(config->streams[i].name, stream_name) == 0) {
                stream_idx = i;
                break;
            }
        }
        
        // If stream not found, log warning and skip
        if (stream_idx == -1) {
            log_warn("Configuration for unknown stream: %s", stream_name);
            return 1; // Continue processing
        }
        
        // Parse stream-specific settings
        if (strcmp(name, "detection_based_recording") == 0) {
            config->streams[stream_idx].detection_based_recording = 
                (strcmp(value, "true") == 0 || strcmp(value, "1") == 0);
        } else if (strcmp(name, "detection_model") == 0) {
            strncpy(config->streams[stream_idx].detection_model, value, MAX_PATH_LENGTH - 1);
        } else if (strcmp(name, "detection_interval") == 0) {
            config->streams[stream_idx].detection_interval = atoi(value);
        } else if (strcmp(name, "detection_threshold") == 0) {
            config->streams[stream_idx].detection_threshold = atof(value);
        } else if (strcmp(name, "pre_detection_buffer") == 0) {
            config->streams[stream_idx].pre_detection_buffer = atoi(value);
        } else if (strcmp(name, "post_detection_buffer") == 0) {
            config->streams[stream_idx].post_detection_buffer = atoi(value);
        } else if (strcmp(name, "record_audio") == 0) {
            config->streams[stream_idx].record_audio = 
                (strcmp(value, "true") == 0 || strcmp(value, "1") == 0);
        }
    }
    // Memory optimization
    else if (strcmp(section, "memory") == 0) {
        if (strcmp(name, "buffer_size") == 0) {
            config->buffer_size = atoi(value);
        } else if (strcmp(name, "use_swap") == 0) {
            config->use_swap = (strcmp(value, "true") == 0 || strcmp(value, "1") == 0);
        } else if (strcmp(name, "swap_file") == 0) {
            strncpy(config->swap_file, value, MAX_PATH_LENGTH - 1);
        } else if (strcmp(name, "swap_size") == 0) {
            config->swap_size = strtoull(value, NULL, 10);
        }
    }
    // Hardware acceleration
    else if (strcmp(section, "hardware") == 0) {
        if (strcmp(name, "hw_accel_enabled") == 0) {
            config->hw_accel_enabled = (strcmp(value, "true") == 0 || strcmp(value, "1") == 0);
        } else if (strcmp(name, "hw_accel_device") == 0) {
            strncpy(config->hw_accel_device, value, 31);
        }
    }
    // go2rtc settings
    else if (strcmp(section, "go2rtc") == 0) {
        if (strcmp(name, "binary_path") == 0) {
            strncpy(config->go2rtc_binary_path, value, MAX_PATH_LENGTH - 1);
        } else if (strcmp(name, "config_dir") == 0) {
            strncpy(config->go2rtc_config_dir, value, MAX_PATH_LENGTH - 1);
        } else if (strcmp(name, "api_port") == 0) {
            config->go2rtc_api_port = atoi(value);
        }
    }
    
    return 1; // Return 1 to continue processing
}

// Load configuration from file using inih
static int load_config_from_file(const char *filename, config_t *config) {
    // Use inih to parse the INI file
    int result = ini_parse(filename, config_ini_handler, config);
    
    if (result < 0) {
        log_warn("Could not read config file %s", filename);
        return -1;
    } else if (result > 0) {
        log_warn("Error in config file %s at line %d", filename, result);
        return -1;
    }
    
    return 0;
}

// Load stream configurations from database
int load_stream_configs(config_t *config) {
    if (!config) return -1;
    
    // Clear existing stream configurations
    memset(config->streams, 0, sizeof(stream_config_t) * MAX_STREAMS);
    
    // Get stream count from database
    int count = count_stream_configs();
    if (count < 0) {
        log_error("Failed to count stream configurations in database");
        return -1;
    }
    
    if (count == 0) {
        log_info("No stream configurations found in database");
        return 0;
    }
    
    // Get stream configurations from database
    stream_config_t db_streams[MAX_STREAMS];
    int loaded = get_all_stream_configs(db_streams, MAX_STREAMS);
    if (loaded < 0) {
        log_error("Failed to load stream configurations from database");
        return -1;
    }
    
    // Copy stream configurations to config
    for (int i = 0; i < loaded && i < config->max_streams; i++) {
        memcpy(&config->streams[i], &db_streams[i], sizeof(stream_config_t));
    }
    
    log_info("Loaded %d stream configurations from database", loaded);
    return loaded;
}

// Save stream configurations to database with improved timeout protection
int save_stream_configs(const config_t *config) {
    if (!config) return -1;
    
    int saved = 0;
    int transaction_started = 0;
    
    // We don't set an alarm here anymore - the caller should handle timeouts
    // The alarm is now set in handle_post_settings with a proper signal handler
    
    // Begin transaction
    if (begin_transaction() != 0) {
        log_error("Failed to begin transaction for saving stream configurations");
        return -1;
    }
    
    transaction_started = 1;
    
    // Get existing stream configurations from database
    int count = count_stream_configs();
    if (count < 0) {
        log_error("Failed to count stream configurations in database");
        rollback_transaction();
        return -1;
    }
    
    // Skip stream configuration updates if there are no changes
    // This is a performance optimization and reduces the chance of locking issues
    if (count == 0 && config->max_streams == 0) {
        log_info("No stream configurations to save");
        commit_transaction();
        return 0;
    }
    
    if (count > 0) {
        // Get existing stream names
        stream_config_t db_streams[MAX_STREAMS];
        int loaded = get_all_stream_configs(db_streams, MAX_STREAMS);
        if (loaded < 0) {
            log_error("Failed to load stream configurations from database");
            rollback_transaction();
            return -1;
        }
        
        // Check if configurations are identical to avoid unnecessary updates
        int identical = 1;
        if (loaded == config->max_streams) {
            for (int i = 0; i < loaded && identical; i++) {
                if (strlen(config->streams[i].name) == 0 || 
                    strcmp(config->streams[i].name, db_streams[i].name) != 0) {
                    identical = 0;
                }
            }
            
            if (identical) {
                log_info("Stream configurations unchanged, skipping update");
                commit_transaction();
                return loaded;
            }
        }
        
        // Delete existing stream configurations
        for (int i = 0; i < loaded; i++) {
            if (delete_stream_config(db_streams[i].name) != 0) {
                log_error("Failed to delete stream configuration: %s", db_streams[i].name);
                rollback_transaction();
                return -1;
            }
        }
    }
    
    // Add stream configurations to database
    for (int i = 0; i < config->max_streams; i++) {
        if (strlen(config->streams[i].name) > 0) {
            uint64_t result = add_stream_config(&config->streams[i]);
            if (result == 0) {
                log_error("Failed to add stream configuration: %s", config->streams[i].name);
                rollback_transaction();
                return -1;
            }
            saved++;
        }
    }
    
    // Commit transaction
    if (commit_transaction() != 0) {
        log_error("Failed to commit transaction for saving stream configurations");
        // Try to rollback, but don't check the result since we're already in an error state
        rollback_transaction();
        return -1;
    }
    
    log_info("Saved %d stream configurations to database", saved);
    return saved;
}

// Global variable to store the custom config path
static char g_custom_config_path[MAX_PATH_LENGTH] = {0};

// Global variable to store the actual loaded config path
static char g_loaded_config_path[MAX_PATH_LENGTH] = {0};

// Function to set the custom config path
void set_custom_config_path(const char *path) {
    if (path && path[0] != '\0') {
        strncpy(g_custom_config_path, path, MAX_PATH_LENGTH - 1);
        g_custom_config_path[MAX_PATH_LENGTH - 1] = '\0';
        log_info("Custom config path set to: %s", g_custom_config_path);
    }
}

// Function to get the custom config path
const char* get_custom_config_path(void) {
    return g_custom_config_path[0] != '\0' ? g_custom_config_path : NULL;
}

// Function to get the actual loaded config path
const char* get_loaded_config_path(void) {
    return g_loaded_config_path[0] != '\0' ? g_loaded_config_path : NULL;
}

// Function to set the loaded config path
static void set_loaded_config_path(const char *path) {
    if (path && path[0] != '\0') {
        strncpy(g_loaded_config_path, path, MAX_PATH_LENGTH - 1);
        g_loaded_config_path[MAX_PATH_LENGTH - 1] = '\0';
        log_info("Loaded config path set to: %s", g_loaded_config_path);
    }
}

// Load configuration
int load_config(config_t *config) {
    if (!config) return -1;
    
    // Load default configuration
    load_default_config(config);
    
    int loaded = 0;
    
    // First try to load from custom config path if specified
    if (g_custom_config_path[0] != '\0') {
        if (access(g_custom_config_path, R_OK) == 0) {
            if (load_config_from_file(g_custom_config_path, config) == 0) {
                log_info("Loaded configuration from custom path: %s", g_custom_config_path);
                set_loaded_config_path(g_custom_config_path);
                loaded = 1;
            } else {
                log_error("Failed to load configuration from custom path: %s", g_custom_config_path);
            }
        } else {
            log_error("Custom config file not accessible: %s", g_custom_config_path);
        }
    }
    
    // If no custom config or failed to load, try default paths
    if (!loaded) {
        // Try to load from config file - ONLY use INI format
        const char *config_paths[] = {
            "./lightnvr.ini", // Current directory INI format
            "/etc/lightnvr/lightnvr.ini", // System directory INI format
            NULL
        };
        
        for (int i = 0; config_paths[i] != NULL && !loaded; i++) {
            if (access(config_paths[i], R_OK) == 0) {
                if (load_config_from_file(config_paths[i], config) == 0) {
                    log_info("Loaded configuration from %s", config_paths[i]);
                    set_loaded_config_path(config_paths[i]);
                    loaded = 1;
                    break;
                }
            }
        }
    }
    
    // If no INI config file was found, check for old format and convert if found
    if (!loaded) {
        const char *old_config_paths[] = {
            "./lightnvr.conf", // Old format
            "/etc/lightnvr/lightnvr.conf", // Old format
            NULL
        };
        
        for (int i = 0; old_config_paths[i] != NULL; i++) {
            if (access(old_config_paths[i], R_OK) == 0) {
                log_warn("Found old format configuration file: %s", old_config_paths[i]);
                log_info("Converting to INI format...");
                
                // Load the old format config
                if (load_config_from_file(old_config_paths[i], config) == 0) {
                    // Save in INI format
                    const char *ini_path = "./lightnvr.ini";
                    
                    // Check if /etc/lightnvr exists and is writable
                    struct stat st;
                    if (stat("/etc/lightnvr", &st) == 0 && S_ISDIR(st.st_mode) && access("/etc/lightnvr", W_OK) == 0) {
                        ini_path = "/etc/lightnvr/lightnvr.ini";
                    }
                    
                    if (save_config(config, ini_path) == 0) {
                        log_info("Successfully converted configuration to INI format: %s", ini_path);
                        loaded = 1;
                        break;
                    } else {
                        log_error("Failed to save converted configuration to INI format");
                    }
                }
            }
        }
    }
    
    if (!loaded) {
        log_warn("No configuration file found, using defaults");
    }

    // Set default web root if not specified
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
    
    // Initialize database
    if (init_database(config->db_path) != 0) {
        log_error("Failed to initialize database");
        return -1;
    }
    
    // Load stream configurations from database
    if (load_stream_configs(config) < 0) {
        log_error("Failed to load stream configurations from database");
        // Continue anyway, we'll use empty stream configurations
    }
    
    return 0;
}

/**
 * Reload configuration from disk
 * This is used to refresh the global config after settings changes
 * 
 * @param config Pointer to config structure to fill
 * @return 0 on success, non-zero on failure
 */
int reload_config(config_t *config) {
    if (!config) return -1;
    
    log_info("Reloading configuration from disk");
    
    // Save a copy of the current config for comparison
    config_t old_config;
    memcpy(&old_config, config, sizeof(config_t));
    
    // Load the configuration
    int result = load_config(config);
    if (result != 0) {
        log_error("Failed to reload configuration");
        return result;
    }
    
    // Log changes
    if (old_config.log_level != config->log_level) {
        log_info("Log level changed: %d -> %d", old_config.log_level, config->log_level);
    }
    
    if (old_config.web_port != config->web_port) {
        log_info("Web port changed: %d -> %d", old_config.web_port, config->web_port);
        log_warn("Web port change requires restart to take effect");
    }
    
    if (strcmp(old_config.storage_path, config->storage_path) != 0) {
        log_info("Storage path changed: %s -> %s", old_config.storage_path, config->storage_path);
    }
    
    // Log changes to storage_path_hls
    if (old_config.storage_path_hls[0] == '\0' && config->storage_path_hls[0] != '\0') {
        log_info("HLS storage path set: %s", config->storage_path_hls);
    } else if (old_config.storage_path_hls[0] != '\0' && config->storage_path_hls[0] == '\0') {
        log_info("HLS storage path cleared, will use storage_path");
    } else if (old_config.storage_path_hls[0] != '\0' && config->storage_path_hls[0] != '\0' && 
               strcmp(old_config.storage_path_hls, config->storage_path_hls) != 0) {
        log_info("HLS storage path changed: %s -> %s", old_config.storage_path_hls, config->storage_path_hls);
    }
    
    if (strcmp(old_config.models_path, config->models_path) != 0) {
        log_info("Models path changed: %s -> %s", old_config.models_path, config->models_path);
    }
    
    if (old_config.max_storage_size != config->max_storage_size) {
        log_info("Max storage size changed: %lu -> %lu bytes", 
                (unsigned long)old_config.max_storage_size, 
                (unsigned long)config->max_storage_size);
    }
    
    if (old_config.retention_days != config->retention_days) {
        log_info("Retention days changed: %d -> %d", old_config.retention_days, config->retention_days);
    }
    
    // Update global config
    memcpy(&g_config, config, sizeof(config_t));
    
    log_info("Configuration reloaded successfully");
    return 0;
}

// Save configuration to file in INI format
int save_config(const config_t *config, const char *path) {
    if (!config) {
        log_error("Invalid config parameter for save_config: config=%p", config);
        return -1;
    }
    
    // If no path is specified, use the loaded config path
    const char *save_path = path;
    if (!save_path || save_path[0] == '\0') {
        save_path = get_loaded_config_path();
        if (!save_path) {
            log_error("No path specified and no loaded config path available");
            return -1;
        }
    }
    
    log_info("Attempting to save configuration to %s", save_path);
    
    // Check if directory exists and is writable
    char dir_path[MAX_PATH_LENGTH];
    strncpy(dir_path, save_path, MAX_PATH_LENGTH - 1);
    dir_path[MAX_PATH_LENGTH - 1] = '\0';
    
    // Get directory part
    char *last_slash = strrchr(dir_path, '/');
    if (last_slash) {
        *last_slash = '\0'; // Truncate at last slash to get directory
        
        // Check if directory exists
        struct stat st;
        if (stat(dir_path, &st) != 0) {
            log_warn("Directory %s does not exist, attempting to create it", dir_path);
            if (create_directory(dir_path) != 0) {
                log_error("Failed to create directory %s: %s", dir_path, strerror(errno));
                return -1;
            }
        } else if (!S_ISDIR(st.st_mode)) {
            log_error("Path %s exists but is not a directory", dir_path);
            return -1;
        }
        
        // Check if directory is writable
        if (access(dir_path, W_OK) != 0) {
            log_error("Directory %s is not writable: %s", dir_path, strerror(errno));
            return -1;
        }
    }
    
    // Try to open the file for writing
    FILE *file = fopen(save_path, "w");
    if (!file) {
        log_error("Could not open config file for writing: %s (error: %s)", save_path, strerror(errno));
        return -1;
    }
    
    // Write header
    fprintf(file, "; LightNVR Configuration File (INI format)\n\n");
    
    // Write general settings
    fprintf(file, "[general]\n");
    fprintf(file, "pid_file = %s\n", config->pid_file);
    fprintf(file, "log_file = %s\n", config->log_file);
    fprintf(file, "log_level = %d  ; 0=ERROR, 1=WARN, 2=INFO, 3=DEBUG\n\n", config->log_level);
    
    // Write storage settings
    fprintf(file, "[storage]\n");
    fprintf(file, "path = %s\n", config->storage_path);
    
    // Write storage_path_hls if it's specified
    if (config->storage_path_hls[0] != '\0') {
        fprintf(file, "path_hls = %s  ; Dedicated path for HLS segments\n", config->storage_path_hls);
    }
    
    fprintf(file, "max_size = %llu  ; 0 means unlimited, otherwise bytes\n", (unsigned long long)config->max_storage_size);
    fprintf(file, "retention_days = %d\n", config->retention_days);
    fprintf(file, "auto_delete_oldest = %s\n\n", config->auto_delete_oldest ? "true" : "false");
    
    // Write models settings
    fprintf(file, "[models]\n");
    fprintf(file, "path = %s\n\n", config->models_path);
    
    // Write database settings
    fprintf(file, "[database]\n");
    fprintf(file, "path = %s\n\n", config->db_path);
    
    // Write web server settings
    fprintf(file, "[web]\n");
    fprintf(file, "port = %d\n", config->web_port);
    fprintf(file, "root = %s\n", config->web_root);
    fprintf(file, "auth_enabled = %s\n", config->web_auth_enabled ? "true" : "false");
    fprintf(file, "username = %s\n", config->web_username);
    fprintf(file, "password = %s  ; IMPORTANT: Change this default password!\n", config->web_password);
    fprintf(file, "connection_pool_threads = %d  ; Number of threads for handling concurrent connections\n\n", config->connection_pool_threads);
    
    // Write stream settings
    fprintf(file, "[streams]\n");
    fprintf(file, "max_streams = %d\n\n", config->max_streams);
    
    // Write memory optimization settings
    fprintf(file, "[memory]\n");
    fprintf(file, "buffer_size = %d  ; Buffer size in KB\n", config->buffer_size);
    fprintf(file, "use_swap = %s\n", config->use_swap ? "true" : "false");
    fprintf(file, "swap_file = %s\n", config->swap_file);
    fprintf(file, "swap_size = %llu  ; Size in bytes\n\n", (unsigned long long)config->swap_size);
    
    // Write hardware acceleration settings
    fprintf(file, "[hardware]\n");
    fprintf(file, "hw_accel_enabled = %s\n", config->hw_accel_enabled ? "true" : "false");
    fprintf(file, "hw_accel_device = %s\n\n", config->hw_accel_device);
    
    // Write go2rtc settings
    fprintf(file, "[go2rtc]\n");
    fprintf(file, "binary_path = %s\n", config->go2rtc_binary_path);
    fprintf(file, "config_dir = %s\n", config->go2rtc_config_dir);
    fprintf(file, "api_port = %d\n", config->go2rtc_api_port);
    
    // Write stream-specific settings
    for (int i = 0; i < config->max_streams; i++) {
        if (strlen(config->streams[i].name) > 0 && 
            (config->streams[i].detection_based_recording || config->streams[i].record_audio)) {
            fprintf(file, "\n[stream.%s]\n", config->streams[i].name);
            
            // Write detection-based recording settings if enabled
            if (config->streams[i].detection_based_recording) {
                fprintf(file, "detection_based_recording = %s\n", 
                        config->streams[i].detection_based_recording ? "true" : "false");
                
                if (config->streams[i].detection_model[0] != '\0') {
                    fprintf(file, "detection_model = %s\n", config->streams[i].detection_model);
                }
                
                fprintf(file, "detection_interval = %d\n", config->streams[i].detection_interval);
                fprintf(file, "detection_threshold = %.2f\n", config->streams[i].detection_threshold);
                fprintf(file, "pre_detection_buffer = %d\n", config->streams[i].pre_detection_buffer);
                fprintf(file, "post_detection_buffer = %d\n", config->streams[i].post_detection_buffer);
            }
            
            // Write audio recording setting
            fprintf(file, "record_audio = %s\n", config->streams[i].record_audio ? "true" : "false");
        }
    }
    
    // Note: Stream configurations are stored in the database
    fprintf(file, "\n; Note: Stream configurations are stored in the database\n");
    
    fclose(file);
    
    // We don't need to save stream configurations when saving settings
    // This was causing the server to hang due to database locks
    // Stream configurations are managed separately through the streams API
    
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
    if (config->storage_path_hls[0] != '\0') {
        printf("    HLS Storage Path: %s\n", config->storage_path_hls);
    }
    printf("    Max Storage Size: %llu bytes\n", (unsigned long long)config->max_storage_size);
    printf("    Retention Days: %d\n", config->retention_days);
    printf("    Auto Delete Oldest: %s\n", config->auto_delete_oldest ? "true" : "false");
    
    printf("  Models Settings:\n");
    printf("    Models Path: %s\n", config->models_path);
    
    printf("  Database Settings:\n");
    printf("    Database Path: %s\n", config->db_path);
    
    printf("  Web Server Settings:\n");
    printf("    Web Port: %d\n", config->web_port);
    printf("    Web Root: %s\n", config->web_root);
    printf("    Web Auth Enabled: %s\n", config->web_auth_enabled ? "true" : "false");
    printf("    Web Username: %s\n", config->web_username);
    printf("    Web Password: %s\n", "********");
    printf("    Connection Pool Threads: %d\n", config->connection_pool_threads);
    
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
            printf("      Detection-based Recording: %s\n", 
                   config->streams[i].detection_based_recording ? "true" : "false");
            printf("      Record Audio: %s\n",
                   config->streams[i].record_audio ? "true" : "false");
            
            if (config->streams[i].detection_based_recording) {
                printf("      Detection Model: %s\n", 
                       config->streams[i].detection_model[0] ? config->streams[i].detection_model : "None");
                printf("      Detection Interval: %d frames\n", config->streams[i].detection_interval);
                printf("      Detection Threshold: %.2f\n", config->streams[i].detection_threshold);
                printf("      Pre-detection Buffer: %d seconds\n", config->streams[i].pre_detection_buffer);
                printf("      Post-detection Buffer: %d seconds\n", config->streams[i].post_detection_buffer);
            }
        }
    }
}
