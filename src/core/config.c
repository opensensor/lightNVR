#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <strings.h>
#include <stddef.h>
#include <unistd.h>
#include <fcntl.h>
#include <errno.h>
#include <sys/stat.h>
#include <sys/types.h>
#include <libgen.h>
#include <ctype.h>
#include <limits.h>
#include <syslog.h>

#include "ini.h"
#include "core/config.h"
#include "core/logger.h"
#include "database/database_manager.h"

// Global configuration variable
config_t g_config;

/**
 * Safe integer conversion from string using strtol.
 * Returns the converted value, or fallback on failure (empty string, non-numeric, overflow).
 * This replaces atoi() to satisfy cert-err34-c.
 */
static int safe_atoi(const char *str, int fallback) {
    if (!str || !*str) return fallback;
    char *end;
    errno = 0;
    long val = strtol(str, &end, 10);
    if (end == str || errno == ERANGE || val < INT_MIN || val > INT_MAX) {
        return fallback;
    }
    return (int)val;
}

// ============================================================================
// Environment Variable Override Support
// ============================================================================
// This allows LIGHTNVR_ prefixed environment variables to override config
// values that are not already set in the config file. This is particularly
// useful for container deployments where settings like TURN server credentials
// can be injected via environment variables.
//
// Environment variable naming convention:
//   LIGHTNVR_<SECTION>_<SETTING>
// Examples:
//   LIGHTNVR_TURN_ENABLED=true
//   LIGHTNVR_TURN_SERVER_URL=turn:example.com:3478
//   LIGHTNVR_TURN_USERNAME=myuser
//   LIGHTNVR_TURN_PASSWORD=mypassword
//   LIGHTNVR_WEB_PORT=8080
//   LIGHTNVR_GO2RTC_STUN_ENABLED=true
// ============================================================================

// Configuration field types
typedef enum {
    CONFIG_TYPE_BOOL,
    CONFIG_TYPE_INT,
    CONFIG_TYPE_STRING
} config_field_type_t;

// Environment variable to config field mapping entry
typedef struct {
    const char *env_name;           // Environment variable name (without LIGHTNVR_ prefix)
    config_field_type_t type;       // Field type
    size_t offset;                  // Offset in config_t structure
    size_t size;                    // Size of string fields (0 for non-strings)
    const char *default_str_value;  // Default string value to check against (NULL for non-strings or empty check)
    int default_int_value;          // Default int value to check against
    bool default_bool_value;        // Default bool value to check against
} env_config_mapping_t;

// Helper macro to calculate offset in config_t
#define CONFIG_OFFSET(field) offsetof(config_t, field)

// Environment variable mappings - add new mappings here as needed
static const env_config_mapping_t env_config_mappings[] = {
    // TURN server settings (primary use case for provisioning)
    {"TURN_ENABLED",       CONFIG_TYPE_BOOL,   CONFIG_OFFSET(turn_enabled),       0,   NULL, 0, false},
    {"TURN_SERVER_URL",    CONFIG_TYPE_STRING, CONFIG_OFFSET(turn_server_url),    256, "",   0, false},
    {"TURN_USERNAME",      CONFIG_TYPE_STRING, CONFIG_OFFSET(turn_username),      64,  "",   0, false},
    {"TURN_PASSWORD",      CONFIG_TYPE_STRING, CONFIG_OFFSET(turn_password),      64,  "",   0, false},

    // go2rtc WebRTC settings
    {"GO2RTC_ENABLED",         CONFIG_TYPE_BOOL,   CONFIG_OFFSET(go2rtc_enabled),           0,   NULL, 0, true},
    {"GO2RTC_WEBRTC_ENABLED",  CONFIG_TYPE_BOOL,   CONFIG_OFFSET(go2rtc_webrtc_enabled),    0,   NULL, 0, true},
    {"GO2RTC_STUN_ENABLED",    CONFIG_TYPE_BOOL,   CONFIG_OFFSET(go2rtc_stun_enabled),      0,   NULL, 0, true},
    {"GO2RTC_STUN_SERVER",     CONFIG_TYPE_STRING, CONFIG_OFFSET(go2rtc_stun_server),       256, "stun.l.google.com:19302", 0, false},
    {"GO2RTC_EXTERNAL_IP",     CONFIG_TYPE_STRING, CONFIG_OFFSET(go2rtc_external_ip),       64,  "",   0, false},
    {"GO2RTC_ICE_SERVERS",     CONFIG_TYPE_STRING, CONFIG_OFFSET(go2rtc_ice_servers),       512, "",   0, false},
    {"GO2RTC_WEBRTC_LISTEN_PORT", CONFIG_TYPE_INT, CONFIG_OFFSET(go2rtc_webrtc_listen_port), 0,  NULL, 8555, false},

    // Web server settings
    {"WEB_PORT",           CONFIG_TYPE_INT,    CONFIG_OFFSET(web_port),           0,   NULL, 8080, false},
    {"WEB_AUTH_ENABLED",   CONFIG_TYPE_BOOL,   CONFIG_OFFSET(web_auth_enabled),   0,   NULL, 0, true},
    {"WEB_USERNAME",       CONFIG_TYPE_STRING, CONFIG_OFFSET(web_username),       32,  "admin", 0, false},
    {"DEMO_MODE",          CONFIG_TYPE_BOOL,   CONFIG_OFFSET(demo_mode),          0,   NULL, 0, false},

    // General settings
    {"LOG_LEVEL",          CONFIG_TYPE_INT,    CONFIG_OFFSET(log_level),          0,   NULL, 2, false},

    // Storage settings
    {"STORAGE_PATH",       CONFIG_TYPE_STRING, CONFIG_OFFSET(storage_path),       MAX_PATH_LENGTH, "/var/lib/lightnvr/recordings", 0, false},
    {"RETENTION_DAYS",     CONFIG_TYPE_INT,    CONFIG_OFFSET(retention_days),     0,   NULL, 30, false},

    // Database settings
    {"DB_PATH",            CONFIG_TYPE_STRING, CONFIG_OFFSET(db_path),            MAX_PATH_LENGTH, "/var/lib/lightnvr/lightnvr.db", 0, false},

    // Sentinel to mark end of array
    {NULL, CONFIG_TYPE_BOOL, 0, 0, NULL, 0, false}
};

// Helper function to parse boolean environment variable value
static bool parse_env_bool(const char *value) {
    if (!value) return false;
    return (strcasecmp(value, "true") == 0 ||
            strcasecmp(value, "1") == 0 ||
            strcasecmp(value, "yes") == 0 ||
            strcasecmp(value, "on") == 0);
}

// Helper function to check if a string field has the default value
static bool is_string_default(const char *current, const char *default_val) {
    if (!default_val || default_val[0] == '\0') {
        // Default is empty string - check if current is also empty
        return !current || current[0] == '\0';
    }
    // Check if current value matches the default
    return current && strcmp(current, default_val) == 0;
}

/**
 * Apply environment variable overrides to configuration
 * This function reads LIGHTNVR_ prefixed environment variables and applies
 * them to config fields that are still at their default values.
 *
 * The logic ensures that:
 * 1. Values explicitly set in the config file are NOT overridden
 * 2. Values that are still at defaults CAN be overridden by env vars
 * 3. This allows container deployments to inject settings via env vars
 *
 * @param config Pointer to config structure to modify
 */
static void apply_env_overrides(config_t *config) {
    if (!config) return;

    log_info("Checking for LIGHTNVR_ environment variable overrides");

    for (int i = 0; env_config_mappings[i].env_name != NULL; i++) {
        const env_config_mapping_t *mapping = &env_config_mappings[i];

        // Build the full environment variable name
        char env_name[128];
        snprintf(env_name, sizeof(env_name), "LIGHTNVR_%s", mapping->env_name);

        // Get the environment variable value
        const char *env_value = getenv(env_name);
        if (!env_value) {
            continue; // Environment variable not set
        }

        // Get pointer to the config field
        void *field_ptr = (char *)config + mapping->offset;

        // Check if the current value is still at default (not set by config file)
        bool is_default = false;

        switch (mapping->type) {
            case CONFIG_TYPE_BOOL: {
                bool *bool_ptr = (bool *)field_ptr;
                is_default = (*bool_ptr == mapping->default_bool_value);
                break;
            }
            case CONFIG_TYPE_INT: {
                int *int_ptr = (int *)field_ptr;
                is_default = (*int_ptr == mapping->default_int_value);
                break;
            }
            case CONFIG_TYPE_STRING: {
                char *str_ptr = (char *)field_ptr;
                is_default = is_string_default(str_ptr, mapping->default_str_value);
                break;
            }
        }

        if (!is_default) {
            log_debug("Config field for %s already set, skipping env override", env_name);
            continue;
        }

        // Apply the environment variable value
        switch (mapping->type) {
            case CONFIG_TYPE_BOOL: {
                bool *bool_ptr = (bool *)field_ptr;
                bool new_value = parse_env_bool(env_value);
                *bool_ptr = new_value;
                log_info("Applied env override: %s=%s (bool: %s)",
                         env_name, env_value, new_value ? "true" : "false");
                break;
            }
            case CONFIG_TYPE_INT: {
                int *int_ptr = (int *)field_ptr;
                int new_value = safe_atoi(env_value, 0);
                *int_ptr = new_value;
                log_info("Applied env override: %s=%s (int: %d)",
                         env_name, env_value, new_value);
                break;
            }
            case CONFIG_TYPE_STRING: {
                char *str_ptr = (char *)field_ptr;
                strncpy(str_ptr, env_value, mapping->size - 1);
                str_ptr[mapping->size - 1] = '\0';
                // Log with masked value for sensitive fields
                if (strstr(mapping->env_name, "PASSWORD") != NULL ||
                    strstr(mapping->env_name, "SECRET") != NULL) {
                    log_info("Applied env override: %s=*** (string, masked)",
                             env_name);
                } else {
                    log_info("Applied env override: %s=%s (string)",
                             env_name, env_value);
                }
                break;
            }
        }
    }
}

// Default configuration values
void load_default_config(config_t *config) {
    if (!config) return;

    // Free any existing dynamic streams array before zeroing the struct
    if (config->streams) {
        free(config->streams);
        config->streams = NULL;
    }

    // Clear the structure
    memset(config, 0, sizeof(config_t));

    // --- Runtime stream limit ---
    config->max_streams = 32; // default; overridden by [streams] max_streams in INI
    config->streams = calloc(config->max_streams, sizeof(stream_config_t));
    if (!config->streams) {
        // Fatal: we can't run without a streams array. Caller will detect NULL.
        log_error("load_default_config: failed to allocate streams array");
        return;
    }

    // --- Web thread pool default: 2x online CPUs, clamped [2, 128] ---
    {
        int num_cores = (int)sysconf(_SC_NPROCESSORS_ONLN);
        config->web_thread_pool_size = (num_cores > 0) ? (num_cores * 2) : 8;
        if (config->web_thread_pool_size < 2)   config->web_thread_pool_size = 2;
        if (config->web_thread_pool_size > 128) config->web_thread_pool_size = 128;
    }
    
    // General settings
    snprintf(config->pid_file, MAX_PATH_LENGTH, "/var/run/lightnvr.pid");
    snprintf(config->log_file, MAX_PATH_LENGTH, "/var/log/lightnvr.log");
    config->log_level = LOG_LEVEL_INFO;

    // Syslog settings
    config->syslog_enabled = false;
    snprintf(config->syslog_ident, sizeof(config->syslog_ident), "lightnvr");
    config->syslog_facility = LOG_USER;

    // Storage settings
    snprintf(config->storage_path, MAX_PATH_LENGTH, "/var/lib/lightnvr/recordings");
    config->storage_path_hls[0] = '\0'; // Empty by default, will use storage_path if not specified
    config->max_storage_size = 0; // 0 means unlimited
    config->retention_days = 30;
    config->auto_delete_oldest = true;

    // Thumbnail/grid view settings
    config->generate_thumbnails = true;

    // MP4 recording settings
    config->record_mp4_directly = false;
    snprintf(config->mp4_storage_path, sizeof(config->mp4_storage_path), "/var/lib/lightnvr/recordings/mp4");
    config->mp4_segment_duration = 900; // 15 minutes
    config->mp4_retention_days = 30;

    // Models settings
    snprintf(config->models_path, MAX_PATH_LENGTH, "/var/lib/lightnvr/models");
    
    // API detection settings
    snprintf(config->api_detection_url, MAX_URL_LENGTH, "http://localhost:8000/detect");
    snprintf(config->api_detection_backend, 32, "onnx"); // Default to ONNX backend

    // Global detection defaults
    config->default_detection_threshold = 50;  // 50% confidence threshold
    config->default_pre_detection_buffer = 5;   // 5 seconds before detection
    config->default_post_detection_buffer = 10; // 10 seconds after detection
    snprintf(config->default_buffer_strategy, 32, "auto"); // Auto-select buffer strategy

    // Database settings
    snprintf(config->db_path, MAX_PATH_LENGTH, "/var/lib/lightnvr/lightnvr.db");
    
    // Web server settings
    config->web_port = 8080;
    snprintf(config->web_root, MAX_PATH_LENGTH, "/var/lib/lightnvr/www");
    config->web_auth_enabled = true;
    snprintf(config->web_username, 32, "admin");
    // No default password - will be generated randomly on first run
    config->web_password[0] = '\0';
    config->webrtc_disabled = false; // WebRTC is enabled by default
    config->auth_timeout_hours = 24; // Default session timeout: 24 hours
    config->demo_mode = false; // Demo mode disabled by default

    // Security settings
    config->force_mfa_on_login = false;           // Force MFA disabled by default
    config->login_rate_limit_enabled = true;       // Rate limiting enabled by default
    config->login_rate_limit_max_attempts = 5;     // 5 attempts before lockout
    config->login_rate_limit_window_seconds = 300; // 5 minute window

    // Web optimization settings
    config->web_compression_enabled = true;
    config->web_use_minified_assets = true;
    config->web_cache_max_age_html = 3600;        // 1 hour for HTML
    config->web_cache_max_age_css = 604800;       // 1 week for CSS
    config->web_cache_max_age_js = 604800;        // 1 week for JS
    config->web_cache_max_age_images = 2592000;   // 30 days for images
    config->web_cache_max_age_fonts = 2592000;    // 30 days for fonts
    config->web_cache_max_age_default = 86400;    // 1 day default
    
    // Memory optimization
    config->buffer_size = 1024; // 1MB buffer size
    config->use_swap = true;
    snprintf(config->swap_file, MAX_PATH_LENGTH, "/var/lib/lightnvr/swap");
    config->swap_size = (uint64_t)128 * 1024 * 1024; // 128MB swap
    
    // Hardware acceleration
    config->hw_accel_enabled = false;
    memset(config->hw_accel_device, 0, 32);
    
    // go2rtc settings
    config->go2rtc_enabled = true;  // Enable go2rtc by default
    snprintf(config->go2rtc_binary_path, MAX_PATH_LENGTH, "/usr/local/bin/go2rtc");
    snprintf(config->go2rtc_config_dir, MAX_PATH_LENGTH, "/etc/lightnvr/go2rtc");
    config->go2rtc_api_port = 1984;
    config->go2rtc_rtsp_port = 8554;  // Default RTSP listen port
    config->go2rtc_force_native_hls = false;  // Use go2rtc HLS by default
    config->go2rtc_proxy_max_inflight = 16;  // Default: 16 concurrent proxy requests

    // go2rtc WebRTC settings for NAT traversal
    config->go2rtc_webrtc_enabled = true;  // Enable WebRTC by default
    config->go2rtc_webrtc_listen_port = 8555;  // Default WebRTC listen port
    config->go2rtc_stun_enabled = true;  // Enable STUN by default for NAT traversal
    snprintf(config->go2rtc_stun_server, sizeof(config->go2rtc_stun_server), "stun.l.google.com:19302");
    config->go2rtc_external_ip[0] = '\0';  // Empty by default (auto-detect)
    config->go2rtc_ice_servers[0] = '\0';  // Empty by default (use STUN server)

    // TURN server settings for WebRTC relay (exposed to browser)
    config->turn_enabled = false;  // Disabled by default
    config->turn_server_url[0] = '\0';  // Empty by default
    config->turn_username[0] = '\0';  // Empty by default
    config->turn_password[0] = '\0';  // Empty by default

    // ONVIF discovery settings
    config->onvif_discovery_enabled = false;  // Disabled by default
    config->onvif_discovery_interval = 300;   // 5 minutes between scans
    snprintf(config->onvif_discovery_network, sizeof(config->onvif_discovery_network), "auto");

    // Initialize default values for detection-based recording in streams
    for (int i = 0; i < config->max_streams; i++) {
        config->streams[i].detection_based_recording = false;
        config->streams[i].detection_model[0] = '\0';
        config->streams[i].detection_interval = 10; // Check every 10 seconds
        config->streams[i].detection_threshold = 0.5f; // 50% confidence threshold
        config->streams[i].pre_detection_buffer = 5; // 5 seconds before detection
        config->streams[i].post_detection_buffer = 10; // 10 seconds after detection
        config->streams[i].detection_api_url[0] = '\0'; // Empty = use global config
        strncpy(config->streams[i].detection_object_filter, "none", sizeof(config->streams[i].detection_object_filter) - 1);
        config->streams[i].detection_object_filter_list[0] = '\0';
        config->streams[i].streaming_enabled = true; // Enable streaming by default
        config->streams[i].record_audio = false; // Disable audio recording by default

        // Tiered retention defaults
        config->streams[i].tier_critical_multiplier = 3.0;
        config->streams[i].tier_important_multiplier = 2.0;
        config->streams[i].tier_ephemeral_multiplier = 0.25;
        config->streams[i].storage_priority = 5;
    }

    // MQTT settings for detection event streaming
    config->mqtt_enabled = false;               // Disabled by default
    config->mqtt_broker_host[0] = '\0';         // Must be configured
    config->mqtt_broker_port = 1883;            // Default MQTT port
    config->mqtt_username[0] = '\0';            // Optional
    config->mqtt_password[0] = '\0';            // Optional
    snprintf(config->mqtt_client_id, sizeof(config->mqtt_client_id), "lightnvr");
    snprintf(config->mqtt_topic_prefix, sizeof(config->mqtt_topic_prefix), "lightnvr");
    config->mqtt_tls_enabled = false;           // No TLS by default
    config->mqtt_keepalive = 60;                // 60 seconds keepalive
    config->mqtt_qos = 1;                       // QoS 1 (at least once)
    config->mqtt_retain = false;                // Don't retain messages by default

    // Home Assistant MQTT auto-discovery settings
    config->mqtt_ha_discovery = false;          // Disabled by default
    snprintf(config->mqtt_ha_discovery_prefix, sizeof(config->mqtt_ha_discovery_prefix), "homeassistant");
    config->mqtt_ha_snapshot_interval = 30;     // 30 seconds default
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
            config->log_level = safe_atoi(value, 0);
        } else if (strcmp(name, "syslog_enabled") == 0) {
            config->syslog_enabled = (strcmp(value, "true") == 0 || strcmp(value, "1") == 0);
        } else if (strcmp(name, "syslog_ident") == 0) {
            strncpy(config->syslog_ident, value, sizeof(config->syslog_ident) - 1);
        } else if (strcmp(name, "syslog_facility") == 0) {
            // Parse syslog facility - support both numeric and string values
            if (isdigit(value[0])) {
                config->syslog_facility = safe_atoi(value, 0);
            } else {
                // Map facility names to values
                if (strcmp(value, "LOG_USER") == 0) config->syslog_facility = LOG_USER;
                else if (strcmp(value, "LOG_DAEMON") == 0) config->syslog_facility = LOG_DAEMON;
                else if (strcmp(value, "LOG_LOCAL0") == 0) config->syslog_facility = LOG_LOCAL0;
                else if (strcmp(value, "LOG_LOCAL1") == 0) config->syslog_facility = LOG_LOCAL1;
                else if (strcmp(value, "LOG_LOCAL2") == 0) config->syslog_facility = LOG_LOCAL2;
                else if (strcmp(value, "LOG_LOCAL3") == 0) config->syslog_facility = LOG_LOCAL3;
                else if (strcmp(value, "LOG_LOCAL4") == 0) config->syslog_facility = LOG_LOCAL4;
                else if (strcmp(value, "LOG_LOCAL5") == 0) config->syslog_facility = LOG_LOCAL5;
                else if (strcmp(value, "LOG_LOCAL6") == 0) config->syslog_facility = LOG_LOCAL6;
                else if (strcmp(value, "LOG_LOCAL7") == 0) config->syslog_facility = LOG_LOCAL7;
                else config->syslog_facility = LOG_USER; // Default
            }
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
            config->retention_days = safe_atoi(value, 0);
        } else if (strcmp(name, "auto_delete_oldest") == 0) {
            config->auto_delete_oldest = (strcmp(value, "true") == 0 || strcmp(value, "1") == 0);
        } else if (strcmp(name, "record_mp4_directly") == 0) {
            config->record_mp4_directly = (strcmp(value, "true") == 0 || strcmp(value, "1") == 0);
        } else if (strcmp(name, "mp4_path") == 0) {
            strncpy(config->mp4_storage_path, value, sizeof(config->mp4_storage_path) - 1);
            config->mp4_storage_path[sizeof(config->mp4_storage_path) - 1] = '\0';
        } else if (strcmp(name, "mp4_segment_duration") == 0) {
            config->mp4_segment_duration = safe_atoi(value, 0);
        } else if (strcmp(name, "mp4_retention_days") == 0) {
            config->mp4_retention_days = safe_atoi(value, 0);
        } else if (strcmp(name, "generate_thumbnails") == 0) {
            config->generate_thumbnails = (strcmp(value, "true") == 0 || strcmp(value, "1") == 0);
        }
    }
    // Models settings
    else if (strcmp(section, "models") == 0) {
        if (strcmp(name, "path") == 0) {
            strncpy(config->models_path, value, MAX_PATH_LENGTH - 1);
        }
    }
    // API detection settings
    else if (strcmp(section, "api_detection") == 0) {
        if (strcmp(name, "url") == 0) {
            strncpy(config->api_detection_url, value, MAX_URL_LENGTH - 1);
        } else if (strcmp(name, "backend") == 0) {
            strncpy(config->api_detection_backend, value, 31);
            config->api_detection_backend[31] = '\0';
        } else if (strcmp(name, "detection_threshold") == 0) {
            config->default_detection_threshold = safe_atoi(value, 0);
            // Clamp to valid range
            if (config->default_detection_threshold < 0) config->default_detection_threshold = 0;
            if (config->default_detection_threshold > 100) config->default_detection_threshold = 100;
        } else if (strcmp(name, "pre_detection_buffer") == 0) {
            config->default_pre_detection_buffer = safe_atoi(value, 0);
            // Clamp to valid range
            if (config->default_pre_detection_buffer < 0) config->default_pre_detection_buffer = 0;
            if (config->default_pre_detection_buffer > 60) config->default_pre_detection_buffer = 60;
        } else if (strcmp(name, "post_detection_buffer") == 0) {
            config->default_post_detection_buffer = safe_atoi(value, 0);
            // Clamp to valid range
            if (config->default_post_detection_buffer < 0) config->default_post_detection_buffer = 0;
            if (config->default_post_detection_buffer > 300) config->default_post_detection_buffer = 300;
        } else if (strcmp(name, "buffer_strategy") == 0) {
            strncpy(config->default_buffer_strategy, value, 31);
            config->default_buffer_strategy[31] = '\0';
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
            config->web_port = safe_atoi(value, 0);
        } else if (strcmp(name, "root") == 0) {
            strncpy(config->web_root, value, MAX_PATH_LENGTH - 1);
        } else if (strcmp(name, "auth_enabled") == 0) {
            config->web_auth_enabled = (strcmp(value, "true") == 0 || strcmp(value, "1") == 0);
        } else if (strcmp(name, "username") == 0) {
            strncpy(config->web_username, value, 31);
        } else if (strcmp(name, "password") == 0) {
            strncpy(config->web_password, value, 31);
        } else if (strcmp(name, "webrtc_disabled") == 0) {
            config->webrtc_disabled = (strcmp(value, "true") == 0 || strcmp(value, "1") == 0);
        } else if (strcmp(name, "auth_timeout_hours") == 0) {
            config->auth_timeout_hours = safe_atoi(value, 0);
            if (config->auth_timeout_hours < 1) {
                config->auth_timeout_hours = 1; // Minimum 1 hour
            }
        } else if (strcmp(name, "demo_mode") == 0) {
            config->demo_mode = (strcmp(value, "true") == 0 || strcmp(value, "1") == 0);
        } else if (strcmp(name, "force_mfa_on_login") == 0) {
            config->force_mfa_on_login = (strcmp(value, "true") == 0 || strcmp(value, "1") == 0);
        } else if (strcmp(name, "login_rate_limit_enabled") == 0) {
            config->login_rate_limit_enabled = (strcmp(value, "true") == 0 || strcmp(value, "1") == 0);
        } else if (strcmp(name, "login_rate_limit_max_attempts") == 0) {
            config->login_rate_limit_max_attempts = safe_atoi(value, 0);
            if (config->login_rate_limit_max_attempts < 1) {
                config->login_rate_limit_max_attempts = 1;
            }
        } else if (strcmp(name, "login_rate_limit_window_seconds") == 0) {
            config->login_rate_limit_window_seconds = safe_atoi(value, 0);
            if (config->login_rate_limit_window_seconds < 10) {
                config->login_rate_limit_window_seconds = 10; // Minimum 10 seconds
            }
        } else if (strcmp(name, "web_thread_pool_size") == 0) {
            int v = safe_atoi(value, 0);
            if (v < 2)   v = 2;
            if (v > 128) v = 128;
            config->web_thread_pool_size = v;
        }
    }
    // Stream settings
    else if (strcmp(section, "streams") == 0) {
        if (strcmp(name, "max_streams") == 0) {
            int new_max = safe_atoi(value, 0);
            if (new_max < 1)          new_max = 1;
            if (new_max > MAX_STREAMS) new_max = MAX_STREAMS;
            if (new_max != config->max_streams) {
                stream_config_t *p = realloc(config->streams, new_max * sizeof(stream_config_t));
                if (p) {
                    // Zero-initialise newly added slots when expanding
                    if (new_max > config->max_streams) {
                        memset(p + config->max_streams, 0,
                               (new_max - config->max_streams) * sizeof(stream_config_t));
                    }
                    config->streams    = p;
                    config->max_streams = new_max;
                    log_info("max_streams set to %d", new_max);
                } else {
                    log_error("Failed to realloc streams array for max_streams=%d, keeping %d",
                              new_max, config->max_streams);
                }
            }
        }
    }
    // Stream-specific [stream.X] sections are no longer read from the INI file.
    // All stream configuration is stored exclusively in the database.
    // Warn once per section so operators know to clean up stale config files.
    else if (strstr(section, "stream.") == section) {
        static char last_warned_section[256] = {0};
        if (strcmp(last_warned_section, section) != 0) {
            log_warn("Ignoring stale INI section [%s]: stream config lives in the database. "
                     "Remove this section from the config file.", section);
            strncpy(last_warned_section, section, sizeof(last_warned_section) - 1);
        }
    }
    // Memory optimization
    else if (strcmp(section, "memory") == 0) {
        if (strcmp(name, "buffer_size") == 0) {
            config->buffer_size = safe_atoi(value, 0);
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
        if (strcmp(name, "enabled") == 0) {
            config->go2rtc_enabled = (strcmp(value, "true") == 0 || strcmp(value, "1") == 0);
        } else if (strcmp(name, "binary_path") == 0) {
            strncpy(config->go2rtc_binary_path, value, MAX_PATH_LENGTH - 1);
        } else if (strcmp(name, "config_dir") == 0) {
            strncpy(config->go2rtc_config_dir, value, MAX_PATH_LENGTH - 1);
        } else if (strcmp(name, "api_port") == 0) {
            config->go2rtc_api_port = safe_atoi(value, 0);
        } else if (strcmp(name, "rtsp_port") == 0) {
            config->go2rtc_rtsp_port = safe_atoi(value, 0);
        } else if (strcmp(name, "webrtc_port") == 0) {
            config->go2rtc_webrtc_listen_port = safe_atoi(value, 0);
        } else if (strcmp(name, "webrtc_enabled") == 0) {
            config->go2rtc_webrtc_enabled = (strcmp(value, "true") == 0 || strcmp(value, "1") == 0);
        } else if (strcmp(name, "webrtc_listen_port") == 0) {
            config->go2rtc_webrtc_listen_port = safe_atoi(value, 0);
        } else if (strcmp(name, "stun_enabled") == 0) {
            config->go2rtc_stun_enabled = (strcmp(value, "true") == 0 || strcmp(value, "1") == 0);
        } else if (strcmp(name, "stun_server") == 0) {
            strncpy(config->go2rtc_stun_server, value, sizeof(config->go2rtc_stun_server) - 1);
            config->go2rtc_stun_server[sizeof(config->go2rtc_stun_server) - 1] = '\0';
        } else if (strcmp(name, "external_ip") == 0) {
            strncpy(config->go2rtc_external_ip, value, sizeof(config->go2rtc_external_ip) - 1);
            config->go2rtc_external_ip[sizeof(config->go2rtc_external_ip) - 1] = '\0';
        } else if (strcmp(name, "ice_servers") == 0) {
            strncpy(config->go2rtc_ice_servers, value, sizeof(config->go2rtc_ice_servers) - 1);
            config->go2rtc_ice_servers[sizeof(config->go2rtc_ice_servers) - 1] = '\0';
        } else if (strcmp(name, "force_native_hls") == 0) {
            config->go2rtc_force_native_hls = (strcmp(value, "true") == 0 || strcmp(value, "1") == 0);
        } else if (strcmp(name, "proxy_max_inflight") == 0) {
            config->go2rtc_proxy_max_inflight = safe_atoi(value, 0);
            if (config->go2rtc_proxy_max_inflight < 1) {
                config->go2rtc_proxy_max_inflight = 1;  // Minimum 1
            }
            if (config->go2rtc_proxy_max_inflight > 128) {
                config->go2rtc_proxy_max_inflight = 128;  // Maximum 128
            }
        } else if (strcmp(name, "turn_enabled") == 0) {
            config->turn_enabled = (strcmp(value, "true") == 0 || strcmp(value, "1") == 0);
        } else if (strcmp(name, "turn_server_url") == 0) {
            strncpy(config->turn_server_url, value, sizeof(config->turn_server_url) - 1);
            config->turn_server_url[sizeof(config->turn_server_url) - 1] = '\0';
        } else if (strcmp(name, "turn_username") == 0) {
            strncpy(config->turn_username, value, sizeof(config->turn_username) - 1);
            config->turn_username[sizeof(config->turn_username) - 1] = '\0';
        } else if (strcmp(name, "turn_password") == 0) {
            strncpy(config->turn_password, value, sizeof(config->turn_password) - 1);
            config->turn_password[sizeof(config->turn_password) - 1] = '\0';
        }
    }
    // ONVIF settings
    else if (strcmp(section, "onvif") == 0) {
        if (strcmp(name, "discovery_enabled") == 0) {
            config->onvif_discovery_enabled = (strcmp(value, "true") == 0 || strcmp(value, "1") == 0);
        } else if (strcmp(name, "discovery_interval") == 0) {
            config->onvif_discovery_interval = safe_atoi(value, 0);
            // Clamp to reasonable range (30 seconds to 1 hour)
            if (config->onvif_discovery_interval < 30) {
                config->onvif_discovery_interval = 30;
            }
            if (config->onvif_discovery_interval > 3600) {
                config->onvif_discovery_interval = 3600;
            }
        } else if (strcmp(name, "discovery_network") == 0) {
            strncpy(config->onvif_discovery_network, value, sizeof(config->onvif_discovery_network) - 1);
            config->onvif_discovery_network[sizeof(config->onvif_discovery_network) - 1] = '\0';
        }
    }
    // MQTT settings for detection event streaming
    else if (strcmp(section, "mqtt") == 0) {
        if (strcmp(name, "enabled") == 0) {
            config->mqtt_enabled = (strcmp(value, "true") == 0 || strcmp(value, "1") == 0);
        } else if (strcmp(name, "broker_host") == 0) {
            strncpy(config->mqtt_broker_host, value, sizeof(config->mqtt_broker_host) - 1);
            config->mqtt_broker_host[sizeof(config->mqtt_broker_host) - 1] = '\0';
        } else if (strcmp(name, "broker_port") == 0) {
            config->mqtt_broker_port = safe_atoi(value, 0);
            if (config->mqtt_broker_port <= 0 || config->mqtt_broker_port > 65535) {
                config->mqtt_broker_port = 1883; // Default port
            }
        } else if (strcmp(name, "username") == 0) {
            strncpy(config->mqtt_username, value, sizeof(config->mqtt_username) - 1);
            config->mqtt_username[sizeof(config->mqtt_username) - 1] = '\0';
        } else if (strcmp(name, "password") == 0) {
            strncpy(config->mqtt_password, value, sizeof(config->mqtt_password) - 1);
            config->mqtt_password[sizeof(config->mqtt_password) - 1] = '\0';
        } else if (strcmp(name, "client_id") == 0) {
            strncpy(config->mqtt_client_id, value, sizeof(config->mqtt_client_id) - 1);
            config->mqtt_client_id[sizeof(config->mqtt_client_id) - 1] = '\0';
        } else if (strcmp(name, "topic_prefix") == 0) {
            strncpy(config->mqtt_topic_prefix, value, sizeof(config->mqtt_topic_prefix) - 1);
            config->mqtt_topic_prefix[sizeof(config->mqtt_topic_prefix) - 1] = '\0';
        } else if (strcmp(name, "tls_enabled") == 0) {
            config->mqtt_tls_enabled = (strcmp(value, "true") == 0 || strcmp(value, "1") == 0);
        } else if (strcmp(name, "keepalive") == 0) {
            config->mqtt_keepalive = safe_atoi(value, 0);
            if (config->mqtt_keepalive < 5) {
                config->mqtt_keepalive = 5; // Minimum 5 seconds
            }
            if (config->mqtt_keepalive > 3600) {
                config->mqtt_keepalive = 3600; // Maximum 1 hour
            }
        } else if (strcmp(name, "qos") == 0) {
            config->mqtt_qos = safe_atoi(value, 0);
            if (config->mqtt_qos < 0 || config->mqtt_qos > 2) {
                config->mqtt_qos = 1; // Default to QoS 1
            }
        } else if (strcmp(name, "retain") == 0) {
            config->mqtt_retain = (strcmp(value, "true") == 0 || strcmp(value, "1") == 0);
        } else if (strcmp(name, "ha_discovery") == 0 || strcmp(name, "ha_discovery_enabled") == 0) {
            config->mqtt_ha_discovery = (strcmp(value, "true") == 0 || strcmp(value, "1") == 0);
        } else if (strcmp(name, "ha_discovery_prefix") == 0) {
            strncpy(config->mqtt_ha_discovery_prefix, value, sizeof(config->mqtt_ha_discovery_prefix) - 1);
            config->mqtt_ha_discovery_prefix[sizeof(config->mqtt_ha_discovery_prefix) - 1] = '\0';
        } else if (strcmp(name, "ha_snapshot_interval") == 0) {
            config->mqtt_ha_snapshot_interval = safe_atoi(value, 0);
            if (config->mqtt_ha_snapshot_interval < 0) {
                config->mqtt_ha_snapshot_interval = 0; // 0 = disabled
            }
            if (config->mqtt_ha_snapshot_interval > 300) {
                config->mqtt_ha_snapshot_interval = 300; // Maximum 5 minutes
            }
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
    if (!config || !config->streams) return -1;

    // Clear existing stream configurations
    memset(config->streams, 0, sizeof(stream_config_t) * config->max_streams);

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

    // Heap-allocate temporary buffer (stream_config_t is ~2 KB; stack array at 256 overflows)
    stream_config_t *db_streams = calloc(config->max_streams, sizeof(stream_config_t));
    if (!db_streams) {
        log_error("load_stream_configs: out of memory");
        return -1;
    }
    int loaded = get_all_stream_configs(db_streams, config->max_streams);
    if (loaded < 0) {
        log_error("Failed to load stream configurations from database");
        free(db_streams);
        return -1;
    }

    // Copy stream configurations to config
    for (int i = 0; i < loaded && i < config->max_streams; i++) {
        memcpy(&config->streams[i], &db_streams[i], sizeof(stream_config_t));
    }
    free(db_streams);

    log_info("Loaded %d stream configurations from database", loaded);
    return loaded;
}

// Save stream configurations to database with improved timeout protection
int save_stream_configs(const config_t *config) {
    if (!config) return -1;
    
    int saved = 0;

    // We don't set an alarm here anymore - the caller should handle timeouts
    // The alarm is now set in handle_post_settings with a proper signal handler

    // Begin transaction
    if (begin_transaction() != 0) {
        log_error("Failed to begin transaction for saving stream configurations");
        return -1;
    }

    // Get existing stream configurations from database
    int count = count_stream_configs();
    if (count < 0) {
        log_error("Failed to count stream configurations in database");
        rollback_transaction();
        return -1;
    }
    
    // Skip stream configuration updates if there are no changes
    // This is a performance optimization and reduces the chance of locking issues
    if (count == 0) {
        log_info("No stream configurations to save");
        commit_transaction();
        return 0;
    }
    
    if (count > 0) {
        // Get existing stream names (heap-allocated to avoid large stack frames)
        stream_config_t *db_streams = calloc(config->max_streams, sizeof(stream_config_t));
        if (!db_streams) {
            log_error("save_stream_configs: out of memory");
            rollback_transaction();
            return -1;
        }
        int loaded = get_all_stream_configs(db_streams, config->max_streams);
        if (loaded < 0) {
            log_error("Failed to load stream configurations from database");
            free(db_streams);
            rollback_transaction();
            return -1;
        }

        // Check if configurations are identical to avoid unnecessary updates
        int identical = 1;
        if (loaded == count) {
            for (int i = 0; i < loaded && identical; i++) {
                if (strlen(config->streams[i].name) == 0 ||
                    strcmp(config->streams[i].name, db_streams[i].name) != 0) {
                    identical = 0;
                }
            }

            if (identical) {
                log_info("Stream configurations unchanged, skipping update");
                free(db_streams);
                commit_transaction();
                return loaded;
            }
        }

        // Delete existing stream configurations
        for (int i = 0; i < loaded; i++) {
            if (delete_stream_config(db_streams[i].name) != 0) {
                log_error("Failed to delete stream configuration: %s", db_streams[i].name);
                free(db_streams);
                rollback_transaction();
                return -1;
            }
        }
        free(db_streams);
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
        // Try to load from config file - INI format only
        const char *config_paths[] = {
            "./lightnvr.ini",             // Current directory INI format
            "/etc/lightnvr/lightnvr.ini", // System directory INI format
            "./config/lightnvr.ini",      // Subdirectory (dev/repo layout)
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

    if (!loaded) {
        log_warn("No configuration file found, using defaults");
    }

    // Apply environment variable overrides
    // This allows LIGHTNVR_ prefixed env vars to set config values
    // that are not already set in the config file (useful for container deployments)
    apply_env_overrides(config);

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
    
    // If no path is specified, try tracked paths then well-known defaults
    const char *save_path = path;
    if (!save_path || save_path[0] == '\0') {
        save_path = get_loaded_config_path();
        if (!save_path) {
            save_path = get_custom_config_path();
        }
        if (!save_path) {
            // Fall back to well-known default locations (writable check)
            static const char *defaults[] = {
                "./lightnvr.ini",
                "/etc/lightnvr/lightnvr.ini",
                "./config/lightnvr.ini",
                NULL
            };
            for (int i = 0; defaults[i]; i++) {
                if (access(defaults[i], W_OK) == 0) {
                    save_path = defaults[i];
                    log_info("Using fallback config path for save: %s", save_path);
                    break;
                }
            }
        }
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
    
    // Open the config file with 0600 permissions so passwords written to it
    // are not world-readable. Using open()+fdopen() instead of fopen() lets us
    // specify the mode explicitly and avoid relying on the process umask.
    int config_fd = open(save_path, O_WRONLY | O_CREAT | O_TRUNC, 0600);
    if (config_fd < 0) {
        log_error("Could not open config file for writing: %s (error: %s)", save_path, strerror(errno));
        return -1;
    }
    FILE *file = fdopen(config_fd, "w");
    if (!file) {
        log_error("Could not open config file for writing: %s (error: %s)", save_path, strerror(errno));
        close(config_fd);
        return -1;
    }
    
    // Write header
    fprintf(file, "; LightNVR Configuration File (INI format)\n\n");
    
    // Write general settings
    fprintf(file, "[general]\n");
    fprintf(file, "pid_file = %s\n", config->pid_file);
    fprintf(file, "log_file = %s\n", config->log_file);
    fprintf(file, "log_level = %d  ; 0=ERROR, 1=WARN, 2=INFO, 3=DEBUG\n", config->log_level);
    fprintf(file, "syslog_enabled = %s\n", config->syslog_enabled ? "true" : "false");
    fprintf(file, "syslog_ident = %s\n", config->syslog_ident);

    // Convert facility number to name for readability
    const char *facility_name = "LOG_USER";
    switch (config->syslog_facility) {
        case LOG_USER: facility_name = "LOG_USER"; break;
        case LOG_DAEMON: facility_name = "LOG_DAEMON"; break;
        case LOG_LOCAL0: facility_name = "LOG_LOCAL0"; break;
        case LOG_LOCAL1: facility_name = "LOG_LOCAL1"; break;
        case LOG_LOCAL2: facility_name = "LOG_LOCAL2"; break;
        case LOG_LOCAL3: facility_name = "LOG_LOCAL3"; break;
        case LOG_LOCAL4: facility_name = "LOG_LOCAL4"; break;
        case LOG_LOCAL5: facility_name = "LOG_LOCAL5"; break;
        case LOG_LOCAL6: facility_name = "LOG_LOCAL6"; break;
        case LOG_LOCAL7: facility_name = "LOG_LOCAL7"; break;
    }
    fprintf(file, "syslog_facility = %s  ; Syslog facility for system logging\n\n", facility_name);
    
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

    // Write MP4 recording settings
    fprintf(file, "; New recording format options\n");
    fprintf(file, "record_mp4_directly = %s\n", config->record_mp4_directly ? "true" : "false");
    if (config->mp4_storage_path[0] != '\0') {
        fprintf(file, "mp4_path = %s\n", config->mp4_storage_path);
    }
    fprintf(file, "mp4_segment_duration = %d\n", config->mp4_segment_duration);
    fprintf(file, "mp4_retention_days = %d\n\n", config->mp4_retention_days);

    // Write thumbnail/grid view settings
    fprintf(file, "; Thumbnail preview settings\n");
    fprintf(file, "generate_thumbnails = %s\n\n", config->generate_thumbnails ? "true" : "false");

    // Write models settings
    fprintf(file, "[models]\n");
    fprintf(file, "path = %s\n\n", config->models_path);
    
    // Write API detection settings
    fprintf(file, "[api_detection]\n");
    fprintf(file, "url = %s\n", config->api_detection_url);
    fprintf(file, "backend = %s\n", config->api_detection_backend);
    fprintf(file, "detection_threshold = %d  ; Default confidence threshold (0-100%%)\n", config->default_detection_threshold);
    fprintf(file, "pre_detection_buffer = %d\n", config->default_pre_detection_buffer);
    fprintf(file, "post_detection_buffer = %d\n", config->default_post_detection_buffer);
    fprintf(file, "buffer_strategy = %s\n\n", config->default_buffer_strategy);

    // Write database settings
    fprintf(file, "[database]\n");
    fprintf(file, "path = %s\n\n", config->db_path);
    
    // Write web server settings
    fprintf(file, "[web]\n");
    fprintf(file, "web_thread_pool_size = %d  ; libuv UV_THREADPOOL_SIZE (default: 2x CPU cores; requires restart)\n", config->web_thread_pool_size);
    fprintf(file, "port = %d\n", config->web_port);
    fprintf(file, "root = %s\n", config->web_root);
    fprintf(file, "auth_enabled = %s\n", config->web_auth_enabled ? "true" : "false");
    fprintf(file, "username = %s\n", config->web_username);
    // Note: web_password is no longer saved to config - user passwords are managed in the database
    fprintf(file, "webrtc_disabled = %s\n", config->webrtc_disabled ? "true" : "false");
    fprintf(file, "auth_timeout_hours = %d  ; Session timeout in hours (default: 24)\n", config->auth_timeout_hours);
    fprintf(file, "demo_mode = %s  ; Demo mode: allows unauthenticated viewer access\n", config->demo_mode ? "true" : "false");
    fprintf(file, "force_mfa_on_login = %s  ; Require TOTP code with password at login (prevents password-only brute force)\n", config->force_mfa_on_login ? "true" : "false");
    fprintf(file, "login_rate_limit_enabled = %s  ; Enable login rate limiting\n", config->login_rate_limit_enabled ? "true" : "false");
    fprintf(file, "login_rate_limit_max_attempts = %d  ; Max login attempts before lockout (default: 5)\n", config->login_rate_limit_max_attempts);
    fprintf(file, "login_rate_limit_window_seconds = %d  ; Rate limit window in seconds (default: 300)\n", config->login_rate_limit_window_seconds);
    fprintf(file, "\n");

    // Write stream settings
    fprintf(file, "[streams]\n");
    fprintf(file, "max_streams = %d  ; Runtime stream slot limit (default: 32, ceiling: %d; requires restart)\n\n",
            config->max_streams, MAX_STREAMS);
    
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
    fprintf(file, "enabled = %s\n", config->go2rtc_enabled ? "true" : "false");
    fprintf(file, "binary_path = %s\n", config->go2rtc_binary_path);
    fprintf(file, "config_dir = %s\n", config->go2rtc_config_dir);
    fprintf(file, "api_port = %d\n", config->go2rtc_api_port);
    fprintf(file, "rtsp_port = %d\n", config->go2rtc_rtsp_port);
    fprintf(file, "webrtc_enabled = %s\n", config->go2rtc_webrtc_enabled ? "true" : "false");
    fprintf(file, "webrtc_listen_port = %d\n", config->go2rtc_webrtc_listen_port);
    fprintf(file, "stun_enabled = %s\n", config->go2rtc_stun_enabled ? "true" : "false");
    fprintf(file, "stun_server = %s\n", config->go2rtc_stun_server);
    if (config->go2rtc_external_ip[0] != '\0') {
        fprintf(file, "external_ip = %s\n", config->go2rtc_external_ip);
    }
    if (config->go2rtc_ice_servers[0] != '\0') {
        fprintf(file, "ice_servers = %s\n", config->go2rtc_ice_servers);
    }
    fprintf(file, "force_native_hls = %s\n", config->go2rtc_force_native_hls ? "true" : "false");
    fprintf(file, "proxy_max_inflight = %d\n", config->go2rtc_proxy_max_inflight);
    // TURN server settings
    fprintf(file, "turn_enabled = %s\n", config->turn_enabled ? "true" : "false");
    if (config->turn_server_url[0] != '\0') {
        fprintf(file, "turn_server_url = %s\n", config->turn_server_url);
    }
    if (config->turn_username[0] != '\0') {
        fprintf(file, "turn_username = %s\n", config->turn_username);
    }
    if (config->turn_password[0] != '\0') {
        fprintf(file, "turn_password = %s\n", config->turn_password); // codeql[cpp/cleartext-storage-file] - user-configured credential stored in 0600 config file
    }

    // Write MQTT settings
    fprintf(file, "\n[mqtt]\n");
    fprintf(file, "enabled = %s\n", config->mqtt_enabled ? "true" : "false");
    if (config->mqtt_broker_host[0] != '\0') {
        fprintf(file, "broker_host = %s\n", config->mqtt_broker_host);
    }
    fprintf(file, "broker_port = %d\n", config->mqtt_broker_port);
    if (config->mqtt_username[0] != '\0') {
        fprintf(file, "username = %s\n", config->mqtt_username);
    }
    if (config->mqtt_password[0] != '\0') {
        fprintf(file, "password = %s\n", config->mqtt_password); // codeql[cpp/cleartext-storage-file] - user-configured credential stored in 0600 config file
    }
    fprintf(file, "client_id = %s\n", config->mqtt_client_id);
    fprintf(file, "topic_prefix = %s\n", config->mqtt_topic_prefix);
    fprintf(file, "tls_enabled = %s\n", config->mqtt_tls_enabled ? "true" : "false");
    fprintf(file, "keepalive = %d\n", config->mqtt_keepalive);
    fprintf(file, "qos = %d\n", config->mqtt_qos);
    fprintf(file, "retain = %s\n", config->mqtt_retain ? "true" : "false");
    fprintf(file, "; Home Assistant MQTT auto-discovery\n");
    fprintf(file, "ha_discovery = %s\n", config->mqtt_ha_discovery ? "true" : "false");
    fprintf(file, "ha_discovery_prefix = %s\n", config->mqtt_ha_discovery_prefix);
    fprintf(file, "ha_snapshot_interval = %d\n", config->mqtt_ha_snapshot_interval);

    // Write ONVIF settings
    fprintf(file, "\n[onvif]\n");
    fprintf(file, "discovery_enabled = %s\n", config->onvif_discovery_enabled ? "true" : "false");
    fprintf(file, "discovery_interval = %d\n", config->onvif_discovery_interval);
    fprintf(file, "discovery_network = %s\n", config->onvif_discovery_network);

    // Stream configurations are stored exclusively in the database.
    // Do NOT write [stream.X] sections here.
    
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
    
    printf("  API Detection Settings:\n");
    printf("    API URL: %s\n", config->api_detection_url);
    
    printf("  Database Settings:\n");
    printf("    Database Path: %s\n", config->db_path);
    
    printf("  Web Server Settings:\n");
    printf("    Web Port: %d\n", config->web_port);
    printf("    Web Root: %s\n", config->web_root);
    printf("    Web Auth Enabled: %s\n", config->web_auth_enabled ? "true" : "false");
    printf("    Web Username: %s\n", config->web_username);
    printf("    Web Password: %s\n", "********");
    printf("    WebRTC Disabled: %s\n", config->webrtc_disabled ? "true" : "false");
    printf("    Auth Timeout: %d hours\n", config->auth_timeout_hours);
    printf("    Demo Mode: %s\n", config->demo_mode ? "true" : "false");
    printf("    Force MFA on Login: %s\n", config->force_mfa_on_login ? "true" : "false");
    printf("    Login Rate Limit: %s\n", config->login_rate_limit_enabled ? "true" : "false");
    printf("    Login Rate Limit Max Attempts: %d\n", config->login_rate_limit_max_attempts);
    printf("    Login Rate Limit Window: %d seconds\n", config->login_rate_limit_window_seconds);

    printf("  Stream Settings:\n");
    printf("    Max Streams: %d (runtime) / %d (compile-time ceiling)\n",
           config->max_streams, MAX_STREAMS);
    printf("  Web Thread Pool Size: %d\n", config->web_thread_pool_size);
    
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
