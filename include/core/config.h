#ifndef LIGHTNVR_CONFIG_H
#define LIGHTNVR_CONFIG_H

#include <stdint.h>
#include <stdbool.h>

// Maximum length for path strings
#define MAX_PATH_LENGTH 512
// Maximum length for stream names
#define MAX_STREAM_NAME 256
// Maximum length for URLs
#define MAX_URL_LENGTH 512
// Maximum number of streams supported
#define MAX_STREAMS 16

// Stream protocol enum
typedef enum {
    STREAM_PROTOCOL_TCP = 0,
    STREAM_PROTOCOL_UDP = 1
} stream_protocol_t;

// Stream configuration structure
typedef struct {
    char name[MAX_STREAM_NAME];
    char url[MAX_URL_LENGTH];
    bool enabled;
    int width;
    int height;
    int fps;
    char codec[16];
    int priority; // 1-10, higher number = higher priority
    bool record;
    int segment_duration; // in seconds
    bool detection_based_recording; // Only record when detection occurs
    char detection_model[MAX_PATH_LENGTH]; // Path to detection model file
    int detection_interval; // Frames between detection checks
    float detection_threshold; // Confidence threshold for detection
    int pre_detection_buffer; // Seconds to keep before detection
    int post_detection_buffer; // Seconds to keep after detection
    bool streaming_enabled; // Whether HLS streaming is enabled for this stream
    stream_protocol_t protocol; // Stream protocol (TCP or UDP)
} stream_config_t;

// Main configuration structure
typedef struct {
    // General settings
    char pid_file[MAX_PATH_LENGTH];
    char log_file[MAX_PATH_LENGTH];
    int log_level; // 0=ERROR, 1=WARN, 2=INFO, 3=DEBUG
    
    // Storage settings
    char storage_path[MAX_PATH_LENGTH];
    uint64_t max_storage_size; // in bytes
    int retention_days;
    bool auto_delete_oldest;

    // New recording format options
    bool record_mp4_directly;        // Record directly to MP4 alongside HLS
    char mp4_storage_path[256];      // Path for MP4 recordings storage
    int mp4_segment_duration;        // Duration of each MP4 segment in seconds
    int mp4_retention_days;          // Number of days to keep MP4 recordings

    // Database settings
    char db_path[MAX_PATH_LENGTH];
    
    // Web server settings
    int web_port;
    char web_root[MAX_PATH_LENGTH];
    bool web_auth_enabled;
    char web_username[32];
    char web_password[32]; // Stored as hash in actual implementation
    
    // Stream settings
    int max_streams;
    stream_config_t streams[MAX_STREAMS];
    
    // Memory optimization
    int buffer_size; // in KB
    bool use_swap;
    char swap_file[MAX_PATH_LENGTH];
    uint64_t swap_size; // in bytes
    
    // Hardware acceleration
    bool hw_accel_enabled;
    char hw_accel_device[32];
} config_t;

/**
 * Load configuration from default locations
 * Searches in this order:
 * 1. ./lightnvr.ini
 * 2. /etc/lightnvr/lightnvr.ini
 * 3. If not found, looks for old format and converts it
 * 
 * @param config Pointer to config structure to fill
 * @return 0 on success, non-zero on failure
 */
int load_config(config_t *config);

/**
 * Reload configuration from disk
 * This is used to refresh the global config after settings changes
 * 
 * @param config Pointer to config structure to fill
 * @return 0 on success, non-zero on failure
 */
int reload_config(config_t *config);

/**
 * Save configuration to specified file
 * 
 * @param config Pointer to config structure to save
 * @param path Path to save configuration file
 * @return 0 on success, non-zero on failure
 */
int save_config(const config_t *config, const char *path);

/**
 * Load default configuration values
 * 
 * @param config Pointer to config structure to fill with defaults
 */
void load_default_config(config_t *config);

/**
 * Validate configuration values
 * 
 * @param config Pointer to config structure to validate
 * @return 0 if valid, non-zero if invalid
 */
int validate_config(const config_t *config);

/**
 * Print configuration to stdout (for debugging)
 * 
 * @param config Pointer to config structure to print
 */
void print_config(const config_t *config);

/**
 * Load stream configurations from database
 * 
 * @param config Pointer to config structure to fill with stream configurations
 * @return Number of stream configurations loaded, or -1 on error
 */
int load_stream_configs(config_t *config);

/**
 * Save stream configurations to database
 * 
 * @param config Pointer to config structure containing stream configurations to save
 * @return Number of stream configurations saved, or -1 on error
 */
int save_stream_configs(const config_t *config);

/**
 * Set a custom configuration file path
 * This path will be checked first when loading configuration
 * 
 * @param path Path to the custom configuration file
 */
void set_custom_config_path(const char *path);

/**
 * Get the current custom configuration file path
 * 
 * @return The custom configuration file path, or NULL if not set
 */
const char* get_custom_config_path(void);

#endif // LIGHTNVR_CONFIG_H
