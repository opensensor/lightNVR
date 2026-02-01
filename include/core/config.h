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
    char detection_api_url[MAX_URL_LENGTH]; // Per-stream detection API URL override (empty = use global)
    char buffer_strategy[32];  // Pre-detection buffer strategy: "auto", "go2rtc", "hls_segment", "memory_packet", "mmap_hybrid"
    bool streaming_enabled; // Whether HLS streaming is enabled for this stream
    stream_protocol_t protocol; // Stream protocol (TCP, UDP, or ONVIF)
    bool record_audio; // Whether to record audio with video

    // ONVIF specific fields
    char onvif_username[64];
    char onvif_password[64];
    char onvif_profile[64];
    bool onvif_discovery_enabled; // Whether this camera should be included in discovery
    bool is_onvif; // Whether this stream is an ONVIF camera

    // Two-way audio (backchannel) support
    bool backchannel_enabled; // Whether two-way audio is enabled for this stream

    // Per-stream retention policy settings
    int retention_days;              // Regular recordings retention (0 = use global)
    int detection_retention_days;    // Detection recordings retention (0 = use global)
    int max_storage_mb;              // Storage quota in MB (0 = unlimited)

    // PTZ (Pan-Tilt-Zoom) configuration
    bool ptz_enabled;                // Whether PTZ is enabled for this stream
    int ptz_max_x;                   // Maximum X (pan) position (0 = no limit)
    int ptz_max_y;                   // Maximum Y (tilt) position (0 = no limit)
    int ptz_max_z;                   // Maximum Z (zoom) position (0 = no limit)
    bool ptz_has_home;               // Whether the camera supports home position
} stream_config_t;

// Main configuration structure
typedef struct {
    // General settings
    char pid_file[MAX_PATH_LENGTH];
    char log_file[MAX_PATH_LENGTH];
    int log_level; // 0=ERROR, 1=WARN, 2=INFO, 3=DEBUG

    // Syslog settings
    bool syslog_enabled;           // Whether to log to syslog
    char syslog_ident[64];         // Syslog identifier (default: "lightnvr")
    int syslog_facility;           // Syslog facility (default: LOG_USER)
    
    // Storage settings
    char storage_path[MAX_PATH_LENGTH];
    char storage_path_hls[MAX_PATH_LENGTH]; // Path for HLS segments, overrides storage_path/hls when specified
    uint64_t max_storage_size; // in bytes
    int retention_days;
    bool auto_delete_oldest;

    // New recording format options
    bool record_mp4_directly;        // Record directly to MP4 alongside HLS
    char mp4_storage_path[256];      // Path for MP4 recordings storage
    int mp4_segment_duration;        // Duration of each MP4 segment in seconds
    int mp4_retention_days;          // Number of days to keep MP4 recordings
    
    // Models settings
    char models_path[MAX_PATH_LENGTH]; // Path to detection models directory
    
    // API detection settings
    char api_detection_url[MAX_URL_LENGTH]; // URL for the detection API
    char api_detection_backend[32];        // Backend to use: onnx, tflite, opencv (default: onnx)

    // Global detection buffer defaults (used when per-stream settings are not specified)
    int default_pre_detection_buffer;      // Default seconds to keep before detection (0-60)
    int default_post_detection_buffer;     // Default seconds to keep after detection (0-300)
    char default_buffer_strategy[32];      // Default buffer strategy: auto, go2rtc, hls_segment, memory_packet, mmap_hybrid

    // Database settings
    char db_path[MAX_PATH_LENGTH];
    
    // Web server settings
    int web_port;
    char web_root[MAX_PATH_LENGTH];
    bool web_auth_enabled;
    char web_username[32];
    char web_password[32]; // Stored as hash in actual implementation
    bool webrtc_disabled;  // Whether WebRTC is disabled (use HLS only)
    int auth_timeout_hours; // Session timeout in hours (default: 24)
    
    // Web optimization settings
    bool web_compression_enabled;    // Whether to enable gzip compression for text-based responses
    bool web_use_minified_assets;    // Whether to use minified assets (JS/CSS)
    int web_cache_max_age_html;      // Cache max-age for HTML files (in seconds)
    int web_cache_max_age_css;       // Cache max-age for CSS files (in seconds)
    int web_cache_max_age_js;        // Cache max-age for JS files (in seconds)
    int web_cache_max_age_images;    // Cache max-age for image files (in seconds)
    int web_cache_max_age_fonts;     // Cache max-age for font files (in seconds)
    int web_cache_max_age_default;   // Default cache max-age for other files (in seconds)
    
    // ONVIF settings
    bool onvif_discovery_enabled;    // Whether ONVIF discovery is enabled
    int onvif_discovery_interval;    // Interval in seconds between discovery attempts
    char onvif_discovery_network[64]; // Network to scan for ONVIF devices (e.g., "192.168.1.0/24")
    
    // Stream settings
    int max_streams;
    stream_config_t streams[MAX_STREAMS];
    
    // Memory optimization
    int buffer_size; // in KB
    bool use_swap;
    char swap_file[MAX_PATH_LENGTH];
    uint64_t swap_size; // in bytes
    bool memory_constrained; // Flag for memory-constrained devices
    
    // Hardware acceleration
    bool hw_accel_enabled;
    char hw_accel_device[32];
    
    // go2rtc settings
    char go2rtc_binary_path[MAX_PATH_LENGTH];
    char go2rtc_config_dir[MAX_PATH_LENGTH];
    int go2rtc_api_port;
    int go2rtc_rtsp_port;                 // RTSP listen port (default: 8554)

    // go2rtc WebRTC settings for NAT traversal
    bool go2rtc_webrtc_enabled;           // Enable WebRTC (default: true)
    int go2rtc_webrtc_listen_port;        // WebRTC listen port (default: 8555)
    bool go2rtc_stun_enabled;             // Enable STUN servers (default: true)
    char go2rtc_stun_server[256];         // Primary STUN server (default: stun.l.google.com:19302)
    char go2rtc_external_ip[64];          // Optional: External IP for NAT (empty = auto-detect)
    char go2rtc_ice_servers[512];         // Optional: Custom ICE servers (comma-separated)

    // MQTT settings for detection event streaming
    bool mqtt_enabled;                    // Enable MQTT event publishing (default: false)
    char mqtt_broker_host[256];           // MQTT broker hostname or IP
    int mqtt_broker_port;                 // MQTT broker port (default: 1883)
    char mqtt_username[64];               // MQTT username (optional)
    char mqtt_password[128];              // MQTT password (optional)
    char mqtt_client_id[64];              // MQTT client ID (default: lightnvr)
    char mqtt_topic_prefix[128];          // MQTT topic prefix (default: lightnvr)
    bool mqtt_tls_enabled;                // Enable TLS for MQTT connection (default: false)
    int mqtt_keepalive;                   // MQTT keepalive interval in seconds (default: 60)
    int mqtt_qos;                         // MQTT QoS level 0, 1, or 2 (default: 1)
    bool mqtt_retain;                     // Retain detection messages (default: false)
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

/**
 * Get the actual loaded configuration file path
 * 
 * @return The loaded configuration file path, or NULL if not set
 */
const char* get_loaded_config_path(void);

// Global configuration variable
extern config_t g_config;

/**
 * Get a pointer to the global streaming configuration
 * 
 * @return Pointer to the global configuration
 */
config_t* get_streaming_config(void);

#endif /* LIGHTNVR_CONFIG_H */
