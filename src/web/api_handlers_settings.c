#define _XOPEN_SOURCE
#define _GNU_SOURCE

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <ctype.h>
#include <unistd.h>
#include <syslog.h>
#include <pthread.h>
#include <cjson/cJSON.h>

#include "web/api_handlers.h"
#include "web/api_handlers_common.h"
#include "web/request_response.h"
#include "web/httpd_utils.h"
#include "core/logger.h"
#include "core/config.h"
#include "database/db_core.h"
#include "database/db_streams.h"
#include "database/db_auth.h"
#include "video/stream_manager.h"
#include "video/streams.h"
#include "video/mp4_recording.h"
#include "video/go2rtc/go2rtc_process.h"
#include "video/go2rtc/go2rtc_stream.h"
#include "video/go2rtc/go2rtc_integration.h"
#include "video/hls/hls_api.h"
#include "core/mqtt_client.h"

/**
 * @brief Background task for MQTT reinit after settings change.
 *
 * MQTT cleanup and reconnection can take several seconds due to timeouts,
 * so we run it in a detached thread (same pattern as go2rtc below).
 */
typedef struct {
    bool mqtt_now_enabled;  // Whether MQTT is enabled after the settings change
} mqtt_settings_task_t;

static void mqtt_settings_worker(mqtt_settings_task_t *task) {
    if (!task) return;

    log_info("MQTT settings worker: reinitializing MQTT client...");

    int rc = mqtt_reinit(&g_config);
    if (rc != 0) {
        log_error("MQTT settings worker: reinit failed (rc=%d)", rc);
    } else {
        log_info("MQTT settings worker: reinit complete (mqtt_enabled=%s)",
                 task->mqtt_now_enabled ? "true" : "false");
    }

    free(task);
}

/**
 * @brief Background task for go2rtc start/stop after settings change.
 *
 * The actual go2rtc start/stop involves multiple sleep() calls (5-15+ seconds)
 * so we run it in a detached thread to avoid blocking the API response.
 */
typedef struct {
    bool becoming_enabled;   // true = user enabled go2rtc, false = user disabled it
} go2rtc_settings_task_t;

static void go2rtc_settings_worker(go2rtc_settings_task_t *task) {
    if (!task) return;

    stream_config_t all_streams[MAX_STREAMS];
    int stream_count = get_all_stream_configs(all_streams, MAX_STREAMS);

    if (!task->becoming_enabled) {
        // go2rtc is being DISABLED — stop health monitor, stop go2rtc, start native HLS
        log_info("go2rtc settings worker: disabling go2rtc...");

        // Stop the health monitor first so it doesn't restart go2rtc
        go2rtc_integration_cleanup();

        if (go2rtc_process_is_running()) {
            // Stop HLS streams that were routed through go2rtc
            for (int i = 0; i < stream_count; i++) {
                if (all_streams[i].name[0] != '\0' && all_streams[i].enabled &&
                    all_streams[i].streaming_enabled) {
                    stop_hls_stream(all_streams[i].name);
                }
            }

            // Stop MP4 recordings that were using go2rtc RTSP URLs
            // They will be restarted below with direct camera URLs
            for (int i = 0; i < stream_count; i++) {
                if (all_streams[i].name[0] != '\0' && all_streams[i].enabled &&
                    all_streams[i].record) {
                    log_info("Stopping MP4 recording for %s before go2rtc shutdown", all_streams[i].name);
                    stop_mp4_recording(all_streams[i].name);
                }
            }

            if (!go2rtc_process_stop()) {
                log_warn("Failed to stop go2rtc process cleanly");
            }
            sleep(2);
        }

        // Fully reset the stream module (process manager + API client) so that
        // a subsequent re-enable gets a completely fresh go2rtc_stream_init().
        // Without this, the stream module remains "initialized" with potentially
        // stale state, and go2rtc_integration_full_start() skips the init step.
        go2rtc_stream_cleanup();

        // Restart MP4 recordings with direct camera URLs (go2rtc is now disabled)
        for (int i = 0; i < stream_count; i++) {
            if (all_streams[i].name[0] != '\0' && all_streams[i].enabled &&
                all_streams[i].record) {
                log_info("Restarting MP4 recording for %s with direct camera URL", all_streams[i].name);
                if (start_mp4_recording(all_streams[i].name) != 0) {
                    log_warn("Failed to restart MP4 recording for stream %s", all_streams[i].name);
                }
            }
        }

        // Start native HLS threads
        for (int i = 0; i < stream_count; i++) {
            if (all_streams[i].name[0] != '\0' && all_streams[i].enabled &&
                all_streams[i].streaming_enabled) {
                if (start_hls_stream(all_streams[i].name) != 0) {
                    log_warn("Failed to start native HLS for stream %s", all_streams[i].name);
                }
            }
        }

        log_info("go2rtc settings worker: go2rtc disabled, native HLS and recordings restarted");
    } else {
        // go2rtc is being ENABLED (or config changed) — restart go2rtc
        log_info("go2rtc settings worker: enabling go2rtc...");

        // Stop any existing HLS threads first
        for (int i = 0; i < stream_count; i++) {
            if (all_streams[i].name[0] != '\0' && all_streams[i].enabled &&
                all_streams[i].streaming_enabled) {
                stop_hls_stream(all_streams[i].name);
            }
        }

        // Stop MP4 recordings so they can be restarted with go2rtc RTSP URLs
        for (int i = 0; i < stream_count; i++) {
            if (all_streams[i].name[0] != '\0' && all_streams[i].enabled &&
                all_streams[i].record) {
                log_info("Stopping MP4 recording for %s before go2rtc startup", all_streams[i].name);
                stop_mp4_recording(all_streams[i].name);
            }
        }

        // Stop go2rtc if already running
        if (go2rtc_process_is_running()) {
            if (!go2rtc_process_stop()) {
                log_warn("Failed to stop go2rtc process cleanly, continuing anyway");
            }
            sleep(2);
        }

        // Full go2rtc startup: init, start, register streams
        if (!go2rtc_integration_full_start()) {
            log_error("Failed to start go2rtc integration");

            // go2rtc failed to start — restart recordings with direct URLs as fallback
            for (int i = 0; i < stream_count; i++) {
                if (all_streams[i].name[0] != '\0' && all_streams[i].enabled &&
                    all_streams[i].record) {
                    log_info("Restarting MP4 recording for %s with direct URL (go2rtc failed)", all_streams[i].name);
                    if (start_mp4_recording(all_streams[i].name) != 0) {
                        log_warn("Failed to restart MP4 recording for stream %s", all_streams[i].name);
                    }
                }
            }
        } else {
            // Restart MP4 recordings routed through go2rtc
            for (int i = 0; i < stream_count; i++) {
                if (all_streams[i].name[0] != '\0' && all_streams[i].enabled &&
                    all_streams[i].record) {
                    log_info("Restarting MP4 recording for %s via go2rtc", all_streams[i].name);
                    if (go2rtc_integration_start_recording(all_streams[i].name) != 0) {
                        log_warn("Failed to restart MP4 recording for stream %s via go2rtc", all_streams[i].name);
                    }
                }
            }

            // Start HLS streams routed through go2rtc
            for (int i = 0; i < stream_count; i++) {
                if (all_streams[i].name[0] != '\0' && all_streams[i].enabled &&
                    all_streams[i].streaming_enabled) {
                    if (stream_start_hls(all_streams[i].name) != 0) {
                        log_warn("Failed to start HLS for stream %s", all_streams[i].name);
                    }
                }
            }
            log_info("go2rtc settings worker: go2rtc enabled, streams and recordings restarted");
        }
    }

    free(task);
}

/**
 * @brief Direct handler for GET /api/settings
 */
void handle_get_settings(const http_request_t *req, http_response_t *res) {
    log_info("Handling GET /api/settings request");

    // Check authentication if enabled
    // In demo mode, allow unauthenticated viewer access to read settings
    if (g_config.web_auth_enabled) {
        user_t user;
        if (g_config.demo_mode) {
            if (!httpd_check_viewer_access(req, &user)) {
                log_error("Authentication failed for GET /api/settings request");
                http_response_set_json_error(res, 401, "Unauthorized");
                return;
            }
        } else {
            if (!httpd_get_authenticated_user(req, &user)) {
                log_error("Authentication failed for GET /api/settings request");
                http_response_set_json_error(res, 401, "Unauthorized");
                return;
            }
        }
    }

    // Get global configuration
    // Create JSON object
    cJSON *settings = cJSON_CreateObject();
    if (!settings) {
        log_error("Failed to create settings JSON object");
        http_response_set_json_error(res, 500, "Failed to create settings JSON");
        return;
    }
    
    // Add settings properties
    cJSON_AddNumberToObject(settings, "web_port", g_config.web_port);
    cJSON_AddStringToObject(settings, "web_root", g_config.web_root);
    cJSON_AddBoolToObject(settings, "web_auth_enabled", g_config.web_auth_enabled);
    cJSON_AddBoolToObject(settings, "demo_mode", g_config.demo_mode);
    // Note: web_username and web_password removed - user management is now fully database-based
    cJSON_AddBoolToObject(settings, "webrtc_disabled", g_config.webrtc_disabled);
    
    cJSON_AddStringToObject(settings, "storage_path", g_config.storage_path);
    cJSON_AddStringToObject(settings, "storage_path_hls", g_config.storage_path_hls);
    cJSON_AddNumberToObject(settings, "max_storage_size", g_config.max_storage_size);
    cJSON_AddNumberToObject(settings, "retention_days", g_config.retention_days);
    cJSON_AddBoolToObject(settings, "auto_delete_oldest", g_config.auto_delete_oldest);
    cJSON_AddBoolToObject(settings, "generate_thumbnails", g_config.generate_thumbnails);
    cJSON_AddNumberToObject(settings, "max_streams", MAX_STREAMS);
    cJSON_AddStringToObject(settings, "log_file", g_config.log_file);
    cJSON_AddNumberToObject(settings, "log_level", g_config.log_level);
    cJSON_AddStringToObject(settings, "pid_file", g_config.pid_file);
    cJSON_AddStringToObject(settings, "db_path", g_config.db_path);
    cJSON_AddStringToObject(settings, "models_path", g_config.models_path);
    cJSON_AddNumberToObject(settings, "buffer_size", g_config.buffer_size);
    cJSON_AddBoolToObject(settings, "use_swap", g_config.use_swap);
    cJSON_AddNumberToObject(settings, "swap_size", g_config.swap_size / (1024 * 1024)); // Convert bytes to MB

    // Syslog settings
    cJSON_AddBoolToObject(settings, "syslog_enabled", g_config.syslog_enabled);
    cJSON_AddStringToObject(settings, "syslog_ident", g_config.syslog_ident);
    // Convert facility number to name for the API
    {
        const char *facility_name = "LOG_USER";
        switch (g_config.syslog_facility) {
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
        cJSON_AddStringToObject(settings, "syslog_facility", facility_name);
    }

    // API detection settings
    cJSON_AddStringToObject(settings, "api_detection_url", g_config.api_detection_url);
    cJSON_AddStringToObject(settings, "api_detection_backend", g_config.api_detection_backend);

    // Detection defaults
    cJSON_AddNumberToObject(settings, "default_detection_threshold", g_config.default_detection_threshold);
    cJSON_AddNumberToObject(settings, "pre_detection_buffer", g_config.default_pre_detection_buffer);
    cJSON_AddNumberToObject(settings, "post_detection_buffer", g_config.default_post_detection_buffer);
    cJSON_AddStringToObject(settings, "buffer_strategy", g_config.default_buffer_strategy);

    // Auth timeout
    cJSON_AddNumberToObject(settings, "auth_timeout_hours", g_config.auth_timeout_hours);

    // go2rtc settings
    cJSON_AddBoolToObject(settings, "go2rtc_enabled", g_config.go2rtc_enabled);
    cJSON_AddStringToObject(settings, "go2rtc_binary_path", g_config.go2rtc_binary_path);
    cJSON_AddStringToObject(settings, "go2rtc_config_dir", g_config.go2rtc_config_dir);
    cJSON_AddNumberToObject(settings, "go2rtc_api_port", g_config.go2rtc_api_port);
    cJSON_AddNumberToObject(settings, "go2rtc_rtsp_port", g_config.go2rtc_rtsp_port);
    cJSON_AddBoolToObject(settings, "go2rtc_webrtc_enabled", g_config.go2rtc_webrtc_enabled);
    cJSON_AddNumberToObject(settings, "go2rtc_webrtc_listen_port", g_config.go2rtc_webrtc_listen_port);
    cJSON_AddBoolToObject(settings, "go2rtc_stun_enabled", g_config.go2rtc_stun_enabled);
    cJSON_AddStringToObject(settings, "go2rtc_stun_server", g_config.go2rtc_stun_server);
    cJSON_AddStringToObject(settings, "go2rtc_external_ip", g_config.go2rtc_external_ip);
    cJSON_AddStringToObject(settings, "go2rtc_ice_servers", g_config.go2rtc_ice_servers);
    cJSON_AddBoolToObject(settings, "go2rtc_force_native_hls", g_config.go2rtc_force_native_hls);

    // MQTT settings
    cJSON_AddBoolToObject(settings, "mqtt_enabled", g_config.mqtt_enabled);
    cJSON_AddStringToObject(settings, "mqtt_broker_host", g_config.mqtt_broker_host);
    cJSON_AddNumberToObject(settings, "mqtt_broker_port", g_config.mqtt_broker_port);
    cJSON_AddStringToObject(settings, "mqtt_username", g_config.mqtt_username);
    // Don't include the password for security reasons
    cJSON_AddStringToObject(settings, "mqtt_password", g_config.mqtt_password[0] ? "********" : "");
    cJSON_AddStringToObject(settings, "mqtt_client_id", g_config.mqtt_client_id);
    cJSON_AddStringToObject(settings, "mqtt_topic_prefix", g_config.mqtt_topic_prefix);
    cJSON_AddBoolToObject(settings, "mqtt_tls_enabled", g_config.mqtt_tls_enabled);
    cJSON_AddNumberToObject(settings, "mqtt_keepalive", g_config.mqtt_keepalive);
    cJSON_AddNumberToObject(settings, "mqtt_qos", g_config.mqtt_qos);
    cJSON_AddBoolToObject(settings, "mqtt_retain", g_config.mqtt_retain);

    // Home Assistant MQTT auto-discovery settings
    cJSON_AddBoolToObject(settings, "mqtt_ha_discovery", g_config.mqtt_ha_discovery);
    cJSON_AddStringToObject(settings, "mqtt_ha_discovery_prefix", g_config.mqtt_ha_discovery_prefix);
    cJSON_AddNumberToObject(settings, "mqtt_ha_snapshot_interval", g_config.mqtt_ha_snapshot_interval);

    // TURN server settings for WebRTC relay
    cJSON_AddBoolToObject(settings, "turn_enabled", g_config.turn_enabled);
    cJSON_AddStringToObject(settings, "turn_server_url", g_config.turn_server_url);
    cJSON_AddStringToObject(settings, "turn_username", g_config.turn_username);
    // Don't include the password for security reasons
    cJSON_AddStringToObject(settings, "turn_password", g_config.turn_password[0] ? "********" : "");

    // ONVIF discovery settings
    cJSON_AddBoolToObject(settings, "onvif_discovery_enabled", g_config.onvif_discovery_enabled);
    cJSON_AddNumberToObject(settings, "onvif_discovery_interval", g_config.onvif_discovery_interval);
    cJSON_AddStringToObject(settings, "onvif_discovery_network", g_config.onvif_discovery_network);

    // Convert to string
    char *json_str = cJSON_PrintUnformatted(settings);
    if (!json_str) {
        log_error("Failed to convert settings JSON to string");
        cJSON_Delete(settings);
        http_response_set_json_error(res, 500, "Failed to convert settings JSON to string");
        return;
    }
    
    // Send response
    http_response_set_json(res, 200, json_str);
    
    // Clean up
    free(json_str);
    cJSON_Delete(settings);
    
    log_info("Successfully handled GET /api/settings request");
}

/**
 * @brief Direct handler for POST /api/settings
 */
void handle_post_settings(const http_request_t *req, http_response_t *res) {
    log_info("Handling POST /api/settings request");

    // Check if user has admin privileges to modify settings
    if (!httpd_check_admin_privileges(req, res)) {
        return;  // Error response already sent
    }

    // Parse JSON from request body
    cJSON *settings = httpd_parse_json_body(req);
    if (!settings) {
        log_error("Failed to parse settings JSON from request body");
        http_response_set_json_error(res, 400, "Invalid JSON in request body");
        return;
    }

    // Update settings
    bool settings_changed = false;
    bool go2rtc_config_changed = false;  // Track if go2rtc-related settings changed
    bool go2rtc_becoming_enabled = false;  // Track transition direction
    bool mqtt_config_changed = false;     // Track if MQTT-related settings changed

    // Snapshot current MQTT settings before parsing new values
    bool old_mqtt_enabled = g_config.mqtt_enabled;
    char old_mqtt_broker_host[256];
    strncpy(old_mqtt_broker_host, g_config.mqtt_broker_host, sizeof(old_mqtt_broker_host));
    int old_mqtt_broker_port = g_config.mqtt_broker_port;
    char old_mqtt_username[128];
    strncpy(old_mqtt_username, g_config.mqtt_username, sizeof(old_mqtt_username));
    char old_mqtt_password[128];
    strncpy(old_mqtt_password, g_config.mqtt_password, sizeof(old_mqtt_password));
    char old_mqtt_client_id[128];
    strncpy(old_mqtt_client_id, g_config.mqtt_client_id, sizeof(old_mqtt_client_id));
    char old_mqtt_topic_prefix[256];
    strncpy(old_mqtt_topic_prefix, g_config.mqtt_topic_prefix, sizeof(old_mqtt_topic_prefix));
    bool old_mqtt_tls_enabled = g_config.mqtt_tls_enabled;
    int old_mqtt_keepalive = g_config.mqtt_keepalive;
    int old_mqtt_qos = g_config.mqtt_qos;
    bool old_mqtt_retain = g_config.mqtt_retain;
    bool old_mqtt_ha_discovery = g_config.mqtt_ha_discovery;
    char old_mqtt_ha_discovery_prefix[128];
    strncpy(old_mqtt_ha_discovery_prefix, g_config.mqtt_ha_discovery_prefix, sizeof(old_mqtt_ha_discovery_prefix));
    int old_mqtt_ha_snapshot_interval = g_config.mqtt_ha_snapshot_interval;

    // Web port
    cJSON *web_port = cJSON_GetObjectItem(settings, "web_port");
    if (web_port && cJSON_IsNumber(web_port)) {
        g_config.web_port = web_port->valueint;
        settings_changed = true;
        log_info("Updated web_port: %d", g_config.web_port);
    }

    // Web root
    cJSON *web_root = cJSON_GetObjectItem(settings, "web_root");
    if (web_root && cJSON_IsString(web_root)) {
        strncpy(g_config.web_root, web_root->valuestring, sizeof(g_config.web_root) - 1);
        g_config.web_root[sizeof(g_config.web_root) - 1] = '\0';
        settings_changed = true;
        log_info("Updated web_root: %s", g_config.web_root);
    }

    // Web auth enabled
    cJSON *web_auth_enabled = cJSON_GetObjectItem(settings, "web_auth_enabled");
    if (web_auth_enabled && cJSON_IsBool(web_auth_enabled)) {
        g_config.web_auth_enabled = cJSON_IsTrue(web_auth_enabled);
        settings_changed = true;
        log_info("Updated web_auth_enabled: %s", g_config.web_auth_enabled ? "true" : "false");
    }

    // Demo mode
    cJSON *demo_mode = cJSON_GetObjectItem(settings, "demo_mode");
    if (demo_mode && cJSON_IsBool(demo_mode)) {
        g_config.demo_mode = cJSON_IsTrue(demo_mode);
        settings_changed = true;
        log_info("Updated demo_mode: %s", g_config.demo_mode ? "true" : "false");
    }

    // Note: web_username and web_password settings removed - user management is now fully database-based

    // WebRTC disabled - track old value to detect changes
    cJSON *webrtc_disabled = cJSON_GetObjectItem(settings, "webrtc_disabled");
    if (webrtc_disabled && cJSON_IsBool(webrtc_disabled)) {
        bool old_webrtc_disabled = g_config.webrtc_disabled;
        g_config.webrtc_disabled = cJSON_IsTrue(webrtc_disabled);
        settings_changed = true;
        log_info("Updated webrtc_disabled: %s", g_config.webrtc_disabled ? "true" : "false");

        // If webrtc_disabled changed, we need to restart go2rtc
        if (old_webrtc_disabled != g_config.webrtc_disabled) {
            go2rtc_config_changed = true;
            go2rtc_becoming_enabled = g_config.go2rtc_enabled;  // restart, not a toggle
            log_info("WebRTC disabled setting changed, will restart go2rtc");
        }
    }

    // Auth timeout hours
    cJSON *auth_timeout_hours = cJSON_GetObjectItem(settings, "auth_timeout_hours");
    if (auth_timeout_hours && cJSON_IsNumber(auth_timeout_hours)) {
        int value = auth_timeout_hours->valueint;
        if (value < 1) value = 1; // Minimum 1 hour
        g_config.auth_timeout_hours = value;
        settings_changed = true;
        log_info("Updated auth_timeout_hours: %d", g_config.auth_timeout_hours);
    }

    // Storage path
    cJSON *storage_path = cJSON_GetObjectItem(settings, "storage_path");
    if (storage_path && cJSON_IsString(storage_path)) {
        strncpy(g_config.storage_path, storage_path->valuestring, sizeof(g_config.storage_path) - 1);
        g_config.storage_path[sizeof(g_config.storage_path) - 1] = '\0';
        settings_changed = true;
        log_info("Updated storage_path: %s", g_config.storage_path);
    }
    
    // Storage path for HLS segments
    cJSON *storage_path_hls = cJSON_GetObjectItem(settings, "storage_path_hls");
    if (storage_path_hls && cJSON_IsString(storage_path_hls)) {
        strncpy(g_config.storage_path_hls, storage_path_hls->valuestring, sizeof(g_config.storage_path_hls) - 1);
        g_config.storage_path_hls[sizeof(g_config.storage_path_hls) - 1] = '\0';
        settings_changed = true;
        log_info("Updated storage_path_hls: %s", g_config.storage_path_hls);
    }
    
    // Max storage size
    cJSON *max_storage_size = cJSON_GetObjectItem(settings, "max_storage_size");
    if (max_storage_size && cJSON_IsNumber(max_storage_size)) {
        g_config.max_storage_size = max_storage_size->valueint;
        settings_changed = true;
        log_info("Updated max_storage_size: %d", g_config.max_storage_size);
    }
    
    // Retention days
    cJSON *retention_days = cJSON_GetObjectItem(settings, "retention_days");
    if (retention_days && cJSON_IsNumber(retention_days)) {
        g_config.retention_days = retention_days->valueint;
        settings_changed = true;
        log_info("Updated retention_days: %d", g_config.retention_days);
    }
    
    // Auto delete oldest
    cJSON *auto_delete_oldest = cJSON_GetObjectItem(settings, "auto_delete_oldest");
    if (auto_delete_oldest && cJSON_IsBool(auto_delete_oldest)) {
        g_config.auto_delete_oldest = cJSON_IsTrue(auto_delete_oldest);
        settings_changed = true;
        log_info("Updated auto_delete_oldest: %s", g_config.auto_delete_oldest ? "true" : "false");
    }

    // Generate thumbnails (grid view)
    cJSON *generate_thumbnails = cJSON_GetObjectItem(settings, "generate_thumbnails");
    if (generate_thumbnails && cJSON_IsBool(generate_thumbnails)) {
        g_config.generate_thumbnails = cJSON_IsTrue(generate_thumbnails);
        settings_changed = true;
        log_info("Updated generate_thumbnails: %s", g_config.generate_thumbnails ? "true" : "false");
    }

    // Models path
    cJSON *models_path = cJSON_GetObjectItem(settings, "models_path");
    if (models_path && cJSON_IsString(models_path)) {
        strncpy(g_config.models_path, models_path->valuestring, sizeof(g_config.models_path) - 1);
        g_config.models_path[sizeof(g_config.models_path) - 1] = '\0';
        settings_changed = true;
        log_info("Updated models_path: %s", g_config.models_path);
    }
    
    // max_streams is no longer configurable (compile-time MAX_STREAMS constant)

    // Log file
    cJSON *log_file = cJSON_GetObjectItem(settings, "log_file");
    if (log_file && cJSON_IsString(log_file)) {
        strncpy(g_config.log_file, log_file->valuestring, sizeof(g_config.log_file) - 1);
        g_config.log_file[sizeof(g_config.log_file) - 1] = '\0';
        settings_changed = true;
        log_info("Updated log_file: %s", g_config.log_file);
    }
    
    // Log level
    cJSON *log_level = cJSON_GetObjectItem(settings, "log_level");
    if (log_level && cJSON_IsNumber(log_level)) {
        g_config.log_level = log_level->valueint;
        set_log_level(g_config.log_level);
        settings_changed = true;
        log_info("Updated log_level: %d", g_config.log_level);
    }

    // Syslog enabled
    cJSON *syslog_enabled = cJSON_GetObjectItem(settings, "syslog_enabled");
    if (syslog_enabled && cJSON_IsBool(syslog_enabled)) {
        g_config.syslog_enabled = cJSON_IsTrue(syslog_enabled);
        settings_changed = true;
        log_info("Updated syslog_enabled: %s", g_config.syslog_enabled ? "true" : "false");
    }

    // Syslog ident
    cJSON *syslog_ident = cJSON_GetObjectItem(settings, "syslog_ident");
    if (syslog_ident && cJSON_IsString(syslog_ident)) {
        strncpy(g_config.syslog_ident, syslog_ident->valuestring, sizeof(g_config.syslog_ident) - 1);
        g_config.syslog_ident[sizeof(g_config.syslog_ident) - 1] = '\0';
        settings_changed = true;
        log_info("Updated syslog_ident: %s", g_config.syslog_ident);
    }

    // Syslog facility
    cJSON *syslog_facility = cJSON_GetObjectItem(settings, "syslog_facility");
    if (syslog_facility && cJSON_IsString(syslog_facility)) {
        const char *val = syslog_facility->valuestring;
        if (strcmp(val, "LOG_USER") == 0) g_config.syslog_facility = LOG_USER;
        else if (strcmp(val, "LOG_DAEMON") == 0) g_config.syslog_facility = LOG_DAEMON;
        else if (strcmp(val, "LOG_LOCAL0") == 0) g_config.syslog_facility = LOG_LOCAL0;
        else if (strcmp(val, "LOG_LOCAL1") == 0) g_config.syslog_facility = LOG_LOCAL1;
        else if (strcmp(val, "LOG_LOCAL2") == 0) g_config.syslog_facility = LOG_LOCAL2;
        else if (strcmp(val, "LOG_LOCAL3") == 0) g_config.syslog_facility = LOG_LOCAL3;
        else if (strcmp(val, "LOG_LOCAL4") == 0) g_config.syslog_facility = LOG_LOCAL4;
        else if (strcmp(val, "LOG_LOCAL5") == 0) g_config.syslog_facility = LOG_LOCAL5;
        else if (strcmp(val, "LOG_LOCAL6") == 0) g_config.syslog_facility = LOG_LOCAL6;
        else if (strcmp(val, "LOG_LOCAL7") == 0) g_config.syslog_facility = LOG_LOCAL7;
        else g_config.syslog_facility = LOG_USER;
        settings_changed = true;
        log_info("Updated syslog_facility: %d", g_config.syslog_facility);
    }

    // Buffer size
    cJSON *buffer_size = cJSON_GetObjectItem(settings, "buffer_size");
    if (buffer_size && cJSON_IsNumber(buffer_size)) {
        g_config.buffer_size = buffer_size->valueint;
        settings_changed = true;
        log_info("Updated buffer_size: %d", g_config.buffer_size);
    }
    
    // Use swap
    cJSON *use_swap = cJSON_GetObjectItem(settings, "use_swap");
    if (use_swap && cJSON_IsBool(use_swap)) {
        g_config.use_swap = cJSON_IsTrue(use_swap);
        settings_changed = true;
        log_info("Updated use_swap: %s", g_config.use_swap ? "true" : "false");
    }
    
    // Swap size
    cJSON *swap_size = cJSON_GetObjectItem(settings, "swap_size");
    if (swap_size && cJSON_IsNumber(swap_size)) {
        g_config.swap_size = swap_size->valueint * 1024 * 1024; // Convert MB to bytes
        settings_changed = true;
        log_info("Updated swap_size: %llu bytes", (unsigned long long)g_config.swap_size);
    }

    // Default detection threshold
    cJSON *default_detection_threshold = cJSON_GetObjectItem(settings, "default_detection_threshold");
    if (default_detection_threshold && cJSON_IsNumber(default_detection_threshold)) {
        int value = default_detection_threshold->valueint;
        // Clamp to valid range
        if (value < 0) value = 0;
        if (value > 100) value = 100;
        g_config.default_detection_threshold = value;
        settings_changed = true;
        log_info("Updated default_detection_threshold: %d%%", g_config.default_detection_threshold);
    }

    // Pre-detection buffer (default for new streams)
    cJSON *pre_detection_buffer = cJSON_GetObjectItem(settings, "pre_detection_buffer");
    if (pre_detection_buffer && cJSON_IsNumber(pre_detection_buffer)) {
        int value = pre_detection_buffer->valueint;
        // Clamp to valid range
        if (value < 0) value = 0;
        if (value > 60) value = 60;
        g_config.default_pre_detection_buffer = value;
        settings_changed = true;
        log_info("Updated default_pre_detection_buffer: %d seconds", g_config.default_pre_detection_buffer);
    }

    // Post-detection buffer (default for new streams)
    cJSON *post_detection_buffer = cJSON_GetObjectItem(settings, "post_detection_buffer");
    if (post_detection_buffer && cJSON_IsNumber(post_detection_buffer)) {
        int value = post_detection_buffer->valueint;
        // Clamp to valid range
        if (value < 0) value = 0;
        if (value > 300) value = 300;
        g_config.default_post_detection_buffer = value;
        settings_changed = true;
        log_info("Updated default_post_detection_buffer: %d seconds", g_config.default_post_detection_buffer);
    }

    // Buffer strategy (default for new streams)
    cJSON *buffer_strategy = cJSON_GetObjectItem(settings, "buffer_strategy");
    if (buffer_strategy && cJSON_IsString(buffer_strategy)) {
        strncpy(g_config.default_buffer_strategy, buffer_strategy->valuestring, sizeof(g_config.default_buffer_strategy) - 1);
        g_config.default_buffer_strategy[sizeof(g_config.default_buffer_strategy) - 1] = '\0';
        settings_changed = true;
        log_info("Updated default_buffer_strategy: %s", g_config.default_buffer_strategy);
    }

    // API detection URL
    cJSON *api_detection_url = cJSON_GetObjectItem(settings, "api_detection_url");
    if (api_detection_url && cJSON_IsString(api_detection_url)) {
        strncpy(g_config.api_detection_url, api_detection_url->valuestring, sizeof(g_config.api_detection_url) - 1);
        g_config.api_detection_url[sizeof(g_config.api_detection_url) - 1] = '\0';
        settings_changed = true;
        log_info("Updated api_detection_url: %s", g_config.api_detection_url);
    }

    // API detection backend
    cJSON *api_detection_backend = cJSON_GetObjectItem(settings, "api_detection_backend");
    if (api_detection_backend && cJSON_IsString(api_detection_backend)) {
        strncpy(g_config.api_detection_backend, api_detection_backend->valuestring, sizeof(g_config.api_detection_backend) - 1);
        g_config.api_detection_backend[sizeof(g_config.api_detection_backend) - 1] = '\0';
        settings_changed = true;
        log_info("Updated api_detection_backend: %s", g_config.api_detection_backend);
    }

    // go2rtc settings
    cJSON *go2rtc_enabled = cJSON_GetObjectItem(settings, "go2rtc_enabled");
    if (go2rtc_enabled && cJSON_IsBool(go2rtc_enabled)) {
        bool old_go2rtc_enabled = g_config.go2rtc_enabled;
        g_config.go2rtc_enabled = cJSON_IsTrue(go2rtc_enabled);
        settings_changed = true;
        log_info("Updated go2rtc_enabled: %s", g_config.go2rtc_enabled ? "true" : "false");

        // If go2rtc_enabled changed, we need to start/stop go2rtc
        if (old_go2rtc_enabled != g_config.go2rtc_enabled) {
            go2rtc_config_changed = true;
            go2rtc_becoming_enabled = g_config.go2rtc_enabled;
            log_info("go2rtc_enabled setting changed from %s to %s",
                    old_go2rtc_enabled ? "true" : "false",
                    g_config.go2rtc_enabled ? "true" : "false");
        }
    }

    cJSON *go2rtc_binary_path = cJSON_GetObjectItem(settings, "go2rtc_binary_path");
    if (go2rtc_binary_path && cJSON_IsString(go2rtc_binary_path)) {
        strncpy(g_config.go2rtc_binary_path, go2rtc_binary_path->valuestring, sizeof(g_config.go2rtc_binary_path) - 1);
        g_config.go2rtc_binary_path[sizeof(g_config.go2rtc_binary_path) - 1] = '\0';
        settings_changed = true;
        log_info("Updated go2rtc_binary_path: %s", g_config.go2rtc_binary_path);
    }

    cJSON *go2rtc_config_dir = cJSON_GetObjectItem(settings, "go2rtc_config_dir");
    if (go2rtc_config_dir && cJSON_IsString(go2rtc_config_dir)) {
        strncpy(g_config.go2rtc_config_dir, go2rtc_config_dir->valuestring, sizeof(g_config.go2rtc_config_dir) - 1);
        g_config.go2rtc_config_dir[sizeof(g_config.go2rtc_config_dir) - 1] = '\0';
        settings_changed = true;
        log_info("Updated go2rtc_config_dir: %s", g_config.go2rtc_config_dir);
    }

    cJSON *go2rtc_api_port = cJSON_GetObjectItem(settings, "go2rtc_api_port");
    if (go2rtc_api_port && cJSON_IsNumber(go2rtc_api_port)) {
        g_config.go2rtc_api_port = go2rtc_api_port->valueint;
        settings_changed = true;
        log_info("Updated go2rtc_api_port: %d", g_config.go2rtc_api_port);
    }

    cJSON *go2rtc_rtsp_port = cJSON_GetObjectItem(settings, "go2rtc_rtsp_port");
    if (go2rtc_rtsp_port && cJSON_IsNumber(go2rtc_rtsp_port)) {
        g_config.go2rtc_rtsp_port = go2rtc_rtsp_port->valueint;
        settings_changed = true;
        log_info("Updated go2rtc_rtsp_port: %d", g_config.go2rtc_rtsp_port);
    }

    cJSON *go2rtc_webrtc_enabled = cJSON_GetObjectItem(settings, "go2rtc_webrtc_enabled");
    if (go2rtc_webrtc_enabled && cJSON_IsBool(go2rtc_webrtc_enabled)) {
        g_config.go2rtc_webrtc_enabled = cJSON_IsTrue(go2rtc_webrtc_enabled);
        settings_changed = true;
        log_info("Updated go2rtc_webrtc_enabled: %s", g_config.go2rtc_webrtc_enabled ? "true" : "false");
    }

    cJSON *go2rtc_webrtc_listen_port = cJSON_GetObjectItem(settings, "go2rtc_webrtc_listen_port");
    if (go2rtc_webrtc_listen_port && cJSON_IsNumber(go2rtc_webrtc_listen_port)) {
        g_config.go2rtc_webrtc_listen_port = go2rtc_webrtc_listen_port->valueint;
        settings_changed = true;
        log_info("Updated go2rtc_webrtc_listen_port: %d", g_config.go2rtc_webrtc_listen_port);
    }

    cJSON *go2rtc_stun_enabled = cJSON_GetObjectItem(settings, "go2rtc_stun_enabled");
    if (go2rtc_stun_enabled && cJSON_IsBool(go2rtc_stun_enabled)) {
        g_config.go2rtc_stun_enabled = cJSON_IsTrue(go2rtc_stun_enabled);
        settings_changed = true;
        log_info("Updated go2rtc_stun_enabled: %s", g_config.go2rtc_stun_enabled ? "true" : "false");
    }

    cJSON *go2rtc_stun_server = cJSON_GetObjectItem(settings, "go2rtc_stun_server");
    if (go2rtc_stun_server && cJSON_IsString(go2rtc_stun_server)) {
        strncpy(g_config.go2rtc_stun_server, go2rtc_stun_server->valuestring, sizeof(g_config.go2rtc_stun_server) - 1);
        g_config.go2rtc_stun_server[sizeof(g_config.go2rtc_stun_server) - 1] = '\0';
        settings_changed = true;
        log_info("Updated go2rtc_stun_server: %s", g_config.go2rtc_stun_server);
    }

    cJSON *go2rtc_external_ip = cJSON_GetObjectItem(settings, "go2rtc_external_ip");
    if (go2rtc_external_ip && cJSON_IsString(go2rtc_external_ip)) {
        strncpy(g_config.go2rtc_external_ip, go2rtc_external_ip->valuestring, sizeof(g_config.go2rtc_external_ip) - 1);
        g_config.go2rtc_external_ip[sizeof(g_config.go2rtc_external_ip) - 1] = '\0';
        settings_changed = true;
        log_info("Updated go2rtc_external_ip: %s", g_config.go2rtc_external_ip);
    }

    cJSON *go2rtc_ice_servers = cJSON_GetObjectItem(settings, "go2rtc_ice_servers");
    if (go2rtc_ice_servers && cJSON_IsString(go2rtc_ice_servers)) {
        strncpy(g_config.go2rtc_ice_servers, go2rtc_ice_servers->valuestring, sizeof(g_config.go2rtc_ice_servers) - 1);
        g_config.go2rtc_ice_servers[sizeof(g_config.go2rtc_ice_servers) - 1] = '\0';
        settings_changed = true;
        log_info("Updated go2rtc_ice_servers: %s", g_config.go2rtc_ice_servers);
    }

    cJSON *go2rtc_force_native_hls = cJSON_GetObjectItem(settings, "go2rtc_force_native_hls");
    if (go2rtc_force_native_hls && cJSON_IsBool(go2rtc_force_native_hls)) {
        g_config.go2rtc_force_native_hls = cJSON_IsTrue(go2rtc_force_native_hls);
        settings_changed = true;
        log_info("Updated go2rtc_force_native_hls: %s", g_config.go2rtc_force_native_hls ? "true" : "false");
    }

    // MQTT enabled
    cJSON *mqtt_enabled = cJSON_GetObjectItem(settings, "mqtt_enabled");
    if (mqtt_enabled && cJSON_IsBool(mqtt_enabled)) {
        g_config.mqtt_enabled = cJSON_IsTrue(mqtt_enabled);
        settings_changed = true;
        log_info("Updated mqtt_enabled: %s", g_config.mqtt_enabled ? "true" : "false");
    }

    // MQTT broker host
    cJSON *mqtt_broker_host = cJSON_GetObjectItem(settings, "mqtt_broker_host");
    if (mqtt_broker_host && cJSON_IsString(mqtt_broker_host)) {
        strncpy(g_config.mqtt_broker_host, mqtt_broker_host->valuestring, sizeof(g_config.mqtt_broker_host) - 1);
        g_config.mqtt_broker_host[sizeof(g_config.mqtt_broker_host) - 1] = '\0';
        settings_changed = true;
        log_info("Updated mqtt_broker_host: %s", g_config.mqtt_broker_host);
    }

    // MQTT broker port
    cJSON *mqtt_broker_port = cJSON_GetObjectItem(settings, "mqtt_broker_port");
    if (mqtt_broker_port && cJSON_IsNumber(mqtt_broker_port)) {
        g_config.mqtt_broker_port = mqtt_broker_port->valueint;
        settings_changed = true;
        log_info("Updated mqtt_broker_port: %d", g_config.mqtt_broker_port);
    }

    // MQTT username
    cJSON *mqtt_username = cJSON_GetObjectItem(settings, "mqtt_username");
    if (mqtt_username && cJSON_IsString(mqtt_username)) {
        strncpy(g_config.mqtt_username, mqtt_username->valuestring, sizeof(g_config.mqtt_username) - 1);
        g_config.mqtt_username[sizeof(g_config.mqtt_username) - 1] = '\0';
        settings_changed = true;
        log_info("Updated mqtt_username: %s", g_config.mqtt_username);
    }

    // MQTT password (only update if not masked)
    cJSON *mqtt_password = cJSON_GetObjectItem(settings, "mqtt_password");
    if (mqtt_password && cJSON_IsString(mqtt_password) && strcmp(mqtt_password->valuestring, "********") != 0) {
        strncpy(g_config.mqtt_password, mqtt_password->valuestring, sizeof(g_config.mqtt_password) - 1);
        g_config.mqtt_password[sizeof(g_config.mqtt_password) - 1] = '\0';
        settings_changed = true;
        log_info("Updated mqtt_password");
    }

    // MQTT client ID
    cJSON *mqtt_client_id = cJSON_GetObjectItem(settings, "mqtt_client_id");
    if (mqtt_client_id && cJSON_IsString(mqtt_client_id)) {
        strncpy(g_config.mqtt_client_id, mqtt_client_id->valuestring, sizeof(g_config.mqtt_client_id) - 1);
        g_config.mqtt_client_id[sizeof(g_config.mqtt_client_id) - 1] = '\0';
        settings_changed = true;
        log_info("Updated mqtt_client_id: %s", g_config.mqtt_client_id);
    }

    // MQTT topic prefix
    cJSON *mqtt_topic_prefix = cJSON_GetObjectItem(settings, "mqtt_topic_prefix");
    if (mqtt_topic_prefix && cJSON_IsString(mqtt_topic_prefix)) {
        strncpy(g_config.mqtt_topic_prefix, mqtt_topic_prefix->valuestring, sizeof(g_config.mqtt_topic_prefix) - 1);
        g_config.mqtt_topic_prefix[sizeof(g_config.mqtt_topic_prefix) - 1] = '\0';
        settings_changed = true;
        log_info("Updated mqtt_topic_prefix: %s", g_config.mqtt_topic_prefix);
    }

    // MQTT TLS enabled
    cJSON *mqtt_tls_enabled = cJSON_GetObjectItem(settings, "mqtt_tls_enabled");
    if (mqtt_tls_enabled && cJSON_IsBool(mqtt_tls_enabled)) {
        g_config.mqtt_tls_enabled = cJSON_IsTrue(mqtt_tls_enabled);
        settings_changed = true;
        log_info("Updated mqtt_tls_enabled: %s", g_config.mqtt_tls_enabled ? "true" : "false");
    }

    // MQTT keepalive
    cJSON *mqtt_keepalive = cJSON_GetObjectItem(settings, "mqtt_keepalive");
    if (mqtt_keepalive && cJSON_IsNumber(mqtt_keepalive)) {
        g_config.mqtt_keepalive = mqtt_keepalive->valueint;
        settings_changed = true;
        log_info("Updated mqtt_keepalive: %d", g_config.mqtt_keepalive);
    }

    // MQTT QoS
    cJSON *mqtt_qos = cJSON_GetObjectItem(settings, "mqtt_qos");
    if (mqtt_qos && cJSON_IsNumber(mqtt_qos)) {
        int qos = mqtt_qos->valueint;
        if (qos < 0) qos = 0;
        if (qos > 2) qos = 2;
        g_config.mqtt_qos = qos;
        settings_changed = true;
        log_info("Updated mqtt_qos: %d", g_config.mqtt_qos);
    }

    // MQTT retain
    cJSON *mqtt_retain = cJSON_GetObjectItem(settings, "mqtt_retain");
    if (mqtt_retain && cJSON_IsBool(mqtt_retain)) {
        g_config.mqtt_retain = cJSON_IsTrue(mqtt_retain);
        settings_changed = true;
        log_info("Updated mqtt_retain: %s", g_config.mqtt_retain ? "true" : "false");
    }

    // MQTT HA discovery enabled
    cJSON *mqtt_ha_discovery = cJSON_GetObjectItem(settings, "mqtt_ha_discovery");
    if (mqtt_ha_discovery && cJSON_IsBool(mqtt_ha_discovery)) {
        g_config.mqtt_ha_discovery = cJSON_IsTrue(mqtt_ha_discovery);
        settings_changed = true;
        log_info("Updated mqtt_ha_discovery: %s", g_config.mqtt_ha_discovery ? "true" : "false");
    }

    // MQTT HA discovery prefix
    cJSON *mqtt_ha_discovery_prefix = cJSON_GetObjectItem(settings, "mqtt_ha_discovery_prefix");
    if (mqtt_ha_discovery_prefix && cJSON_IsString(mqtt_ha_discovery_prefix)) {
        strncpy(g_config.mqtt_ha_discovery_prefix, mqtt_ha_discovery_prefix->valuestring, sizeof(g_config.mqtt_ha_discovery_prefix) - 1);
        g_config.mqtt_ha_discovery_prefix[sizeof(g_config.mqtt_ha_discovery_prefix) - 1] = '\0';
        settings_changed = true;
        log_info("Updated mqtt_ha_discovery_prefix: %s", g_config.mqtt_ha_discovery_prefix);
    }

    // MQTT HA snapshot interval
    cJSON *mqtt_ha_snapshot_interval = cJSON_GetObjectItem(settings, "mqtt_ha_snapshot_interval");
    if (mqtt_ha_snapshot_interval && cJSON_IsNumber(mqtt_ha_snapshot_interval)) {
        int interval = mqtt_ha_snapshot_interval->valueint;
        if (interval < 0) interval = 0;
        if (interval > 300) interval = 300;
        g_config.mqtt_ha_snapshot_interval = interval;
        settings_changed = true;
        log_info("Updated mqtt_ha_snapshot_interval: %d", g_config.mqtt_ha_snapshot_interval);
    }

    // Detect if any MQTT setting actually changed
    if (old_mqtt_enabled != g_config.mqtt_enabled ||
        strcmp(old_mqtt_broker_host, g_config.mqtt_broker_host) != 0 ||
        old_mqtt_broker_port != g_config.mqtt_broker_port ||
        strcmp(old_mqtt_username, g_config.mqtt_username) != 0 ||
        strcmp(old_mqtt_password, g_config.mqtt_password) != 0 ||
        strcmp(old_mqtt_client_id, g_config.mqtt_client_id) != 0 ||
        strcmp(old_mqtt_topic_prefix, g_config.mqtt_topic_prefix) != 0 ||
        old_mqtt_tls_enabled != g_config.mqtt_tls_enabled ||
        old_mqtt_keepalive != g_config.mqtt_keepalive ||
        old_mqtt_qos != g_config.mqtt_qos ||
        old_mqtt_retain != g_config.mqtt_retain ||
        old_mqtt_ha_discovery != g_config.mqtt_ha_discovery ||
        strcmp(old_mqtt_ha_discovery_prefix, g_config.mqtt_ha_discovery_prefix) != 0 ||
        old_mqtt_ha_snapshot_interval != g_config.mqtt_ha_snapshot_interval) {
        mqtt_config_changed = true;
        log_info("MQTT settings changed, will reinitialize MQTT client");
    }

    // TURN server settings for WebRTC relay
    // Changes to TURN settings require go2rtc restart to regenerate config
    cJSON *turn_enabled = cJSON_GetObjectItem(settings, "turn_enabled");
    if (turn_enabled && cJSON_IsBool(turn_enabled)) {
        bool old_turn_enabled = g_config.turn_enabled;
        g_config.turn_enabled = cJSON_IsTrue(turn_enabled);
        settings_changed = true;
        log_info("Updated turn_enabled: %s", g_config.turn_enabled ? "true" : "false");
        if (old_turn_enabled != g_config.turn_enabled && g_config.go2rtc_enabled) {
            go2rtc_config_changed = true;
            go2rtc_becoming_enabled = true;  // Restart go2rtc to apply new TURN config
            log_info("TURN enabled setting changed, will restart go2rtc");
        }
    }

    cJSON *turn_server_url = cJSON_GetObjectItem(settings, "turn_server_url");
    if (turn_server_url && cJSON_IsString(turn_server_url)) {
        char old_turn_server_url[256];
        strncpy(old_turn_server_url, g_config.turn_server_url, sizeof(old_turn_server_url) - 1);
        old_turn_server_url[sizeof(old_turn_server_url) - 1] = '\0';
        strncpy(g_config.turn_server_url, turn_server_url->valuestring, sizeof(g_config.turn_server_url) - 1);
        g_config.turn_server_url[sizeof(g_config.turn_server_url) - 1] = '\0';
        settings_changed = true;
        log_info("Updated turn_server_url: %s", g_config.turn_server_url);
        if (strcmp(old_turn_server_url, g_config.turn_server_url) != 0 && g_config.go2rtc_enabled && g_config.turn_enabled) {
            go2rtc_config_changed = true;
            go2rtc_becoming_enabled = true;  // Restart go2rtc to apply new TURN config
            log_info("TURN server URL changed, will restart go2rtc");
        }
    }

    cJSON *turn_username = cJSON_GetObjectItem(settings, "turn_username");
    if (turn_username && cJSON_IsString(turn_username)) {
        char old_turn_username[128];
        strncpy(old_turn_username, g_config.turn_username, sizeof(old_turn_username) - 1);
        old_turn_username[sizeof(old_turn_username) - 1] = '\0';
        strncpy(g_config.turn_username, turn_username->valuestring, sizeof(g_config.turn_username) - 1);
        g_config.turn_username[sizeof(g_config.turn_username) - 1] = '\0';
        settings_changed = true;
        log_info("Updated turn_username: %s", g_config.turn_username);
        if (strcmp(old_turn_username, g_config.turn_username) != 0 && g_config.go2rtc_enabled && g_config.turn_enabled) {
            go2rtc_config_changed = true;
            go2rtc_becoming_enabled = true;  // Restart go2rtc to apply new TURN config
            log_info("TURN username changed, will restart go2rtc");
        }
    }

    cJSON *turn_password = cJSON_GetObjectItem(settings, "turn_password");
    if (turn_password && cJSON_IsString(turn_password)) {
        // Only update if not the masked value
        if (strcmp(turn_password->valuestring, "********") != 0) {
            char old_turn_password[128];
            strncpy(old_turn_password, g_config.turn_password, sizeof(old_turn_password) - 1);
            old_turn_password[sizeof(old_turn_password) - 1] = '\0';
            strncpy(g_config.turn_password, turn_password->valuestring, sizeof(g_config.turn_password) - 1);
            g_config.turn_password[sizeof(g_config.turn_password) - 1] = '\0';
            settings_changed = true;
            log_info("Updated turn_password");
            if (strcmp(old_turn_password, g_config.turn_password) != 0 && g_config.go2rtc_enabled && g_config.turn_enabled) {
                go2rtc_config_changed = true;
                go2rtc_becoming_enabled = true;  // Restart go2rtc to apply new TURN config
                log_info("TURN password changed, will restart go2rtc");
            }
        }
    }

    // ONVIF discovery enabled
    cJSON *onvif_discovery_enabled = cJSON_GetObjectItem(settings, "onvif_discovery_enabled");
    if (onvif_discovery_enabled && cJSON_IsBool(onvif_discovery_enabled)) {
        g_config.onvif_discovery_enabled = cJSON_IsTrue(onvif_discovery_enabled);
        settings_changed = true;
        log_info("Updated onvif_discovery_enabled: %s", g_config.onvif_discovery_enabled ? "true" : "false");
    }

    // ONVIF discovery interval
    cJSON *onvif_discovery_interval = cJSON_GetObjectItem(settings, "onvif_discovery_interval");
    if (onvif_discovery_interval && cJSON_IsNumber(onvif_discovery_interval)) {
        int value = onvif_discovery_interval->valueint;
        if (value < 30) value = 30;
        if (value > 3600) value = 3600;
        g_config.onvif_discovery_interval = value;
        settings_changed = true;
        log_info("Updated onvif_discovery_interval: %d", g_config.onvif_discovery_interval);
    }

    // ONVIF discovery network
    cJSON *onvif_discovery_network = cJSON_GetObjectItem(settings, "onvif_discovery_network");
    if (onvif_discovery_network && cJSON_IsString(onvif_discovery_network)) {
        strncpy(g_config.onvif_discovery_network, onvif_discovery_network->valuestring, sizeof(g_config.onvif_discovery_network) - 1);
        g_config.onvif_discovery_network[sizeof(g_config.onvif_discovery_network) - 1] = '\0';
        settings_changed = true;
        log_info("Updated onvif_discovery_network: %s", g_config.onvif_discovery_network);
    }

    // Database path
    cJSON *db_path = cJSON_GetObjectItem(settings, "db_path");
    if (db_path && cJSON_IsString(db_path) && 
        strcmp(g_config.db_path, db_path->valuestring) != 0) {
        
        char old_db_path[MAX_PATH_LENGTH];
        strncpy(old_db_path, g_config.db_path, sizeof(old_db_path) - 1);
        old_db_path[sizeof(old_db_path) - 1] = '\0';
        
        // Update the config with the new path
        strncpy(g_config.db_path, db_path->valuestring, sizeof(g_config.db_path) - 1);
        g_config.db_path[sizeof(g_config.db_path) - 1] = '\0';
        settings_changed = true;
        log_info("Database path changed from %s to %s", old_db_path, g_config.db_path);
        
        // First, stop all HLS streams explicitly to ensure they're properly shut down
        log_info("Stopping all HLS streams before changing database path...");
        
        // Get a list of all active streams
        char active_streams[MAX_STREAMS][MAX_STREAM_NAME];
        int active_stream_count = 0;
        
        log_info("Scanning for active streams...");
        for (int i = 0; i < MAX_STREAMS; i++) {
            log_info("Checking stream slot %d: name='%s', enabled=%d", 
                    i, g_config.streams[i].name, g_config.streams[i].enabled);
            
            if (g_config.streams[i].name[0] != '\0') {
                // Explicitly stop HLS streaming for all streams, even if they're not enabled
                log_info("Explicitly stopping HLS streaming for stream: %s", g_config.streams[i].name);
                stream_stop_hls(g_config.streams[i].name);
                
                // Only add enabled streams to the active list
                if (g_config.streams[i].enabled) {
                    // Copy the stream name for later use
                    strncpy(active_streams[active_stream_count], g_config.streams[i].name, MAX_STREAM_NAME - 1);
                    active_streams[active_stream_count][MAX_STREAM_NAME - 1] = '\0';
                    log_info("Added active stream %d: %s", active_stream_count, active_streams[active_stream_count]);
                    active_stream_count++;
                    
                    // Stop the stream
                    log_info("Stopping stream: %s", g_config.streams[i].name);
                    
                    // Get the stream handle
                    stream_handle_t stream = get_stream_by_name(g_config.streams[i].name);
                    if (stream) {
                        // Stop the stream
                        if (stop_stream(stream) == 0) {
                            log_info("Stream stopped: %s", g_config.streams[i].name);
                        } else {
                            log_warn("Failed to stop stream: %s", g_config.streams[i].name);
                        }
                    } else {
                        log_warn("Failed to get stream handle for: %s", g_config.streams[i].name);
                    }
                }
            }
        }
        
        log_info("Found %d active streams", active_stream_count);
        
        // Wait a bit to ensure all streams are fully stopped
        log_info("Waiting for streams to fully stop...");
        sleep(2);
        
        // We need to restart the database and stream manager
        log_info("Shutting down stream manager to change database path...");
        shutdown_stream_manager();
        
        log_info("Shutting down database...");
        shutdown_database();
        
        log_info("Initializing database with new path: %s", g_config.db_path);
        if (init_database(g_config.db_path) != 0) {
            log_error("Failed to initialize database with new path, reverting to old path");
            
            // Revert to the old path
            strncpy(g_config.db_path, old_db_path, sizeof(g_config.db_path) - 1);
            g_config.db_path[sizeof(g_config.db_path) - 1] = '\0';
            
            // Try to reinitialize with the old path
            if (init_database(g_config.db_path) != 0) {
                log_error("Failed to reinitialize database with old path, database may be unavailable");
            } else {
                log_info("Successfully reinitialized database with old path");
            }
            
            // Reinitialize stream manager
            if (init_stream_manager(MAX_STREAMS) != 0) {
                log_error("Failed to reinitialize stream manager");
            } else {
                log_info("Successfully reinitialized stream manager");
            }
            
            // Send error response
            cJSON_Delete(settings);
            http_response_set_json_error(res, 500, "Failed to initialize database with new path");
            return;
        }
        
        log_info("Reinitializing stream manager...");
        if (init_stream_manager(MAX_STREAMS) != 0) {
            log_error("Failed to reinitialize stream manager");
            
            // Send error response
            cJSON_Delete(settings);
            http_response_set_json_error(res, 500, "Failed to reinitialize stream manager");
            return;
        }
        
        // Restart streams from configuration
        log_info("Restarting streams from configuration...");
        
        // Restart all streams that were previously active
        log_info("Active stream count: %d", active_stream_count);
        for (int i = 0; i < active_stream_count; i++) {
            log_info("Processing active stream %d: %s", i, active_streams[i]);
            log_info("Restarting stream: %s", active_streams[i]);
            
            // Get the stream handle
            stream_handle_t stream = get_stream_by_name(active_streams[i]);
            if (stream) {
                // Get the stream configuration
                stream_config_t config;
                if (get_stream_config(stream, &config) != 0) {
                    log_error("Failed to get stream configuration for %s", active_streams[i]);
                    continue;
                }
                
                // Start the stream
                if (start_stream(stream) == 0) {
                    log_info("Stream restarted: %s", active_streams[i]);
                    
                    // Explicitly start HLS streaming if enabled
                    if (config.streaming_enabled) {
                        log_info("Starting HLS streaming for stream: %s", active_streams[i]);
                        
                        // Try multiple times to start HLS streaming
                        bool hls_started = false;
                        for (int retry = 0; retry < 3 && !hls_started; retry++) {
                            if (retry > 0) {
                                log_info("Retry %d starting HLS streaming for stream: %s", retry, active_streams[i]);
                                // Wait a bit before retrying
                                usleep(500000); // 500ms
                            }
                            
                            if (stream_start_hls(active_streams[i]) == 0) {
                                log_info("HLS streaming started for stream: %s", active_streams[i]);
                                hls_started = true;
                            } else {
                                log_warn("Failed to start HLS streaming for stream: %s (attempt %d/3)", 
                                        active_streams[i], retry + 1);
                            }
                        }
                        
                        if (!hls_started) {
                            log_error("Failed to start HLS streaming for stream: %s after multiple attempts", 
                                    active_streams[i]);
                        }
                    }
                } else {
                    log_warn("Failed to restart stream: %s", active_streams[i]);
                }
            } else {
                log_warn("Failed to get stream handle for: %s", active_streams[i]);
                
                // Try to find the stream configuration in the global config
                stream_config_t *config = NULL;
                for (int j = 0; j < MAX_STREAMS; j++) {
                    if (strcmp(g_config.streams[j].name, active_streams[i]) == 0) {
                        config = &g_config.streams[j];
                        break;
                    }
                }
                
                if (config) {
                    // Try to add the stream first
                    stream = add_stream(config);
                    if (stream) {
                        log_info("Added stream: %s", active_streams[i]);
                        
                        // Start the stream
                        if (start_stream(stream) == 0) {
                            log_info("Stream started: %s", active_streams[i]);
                            
                            // Explicitly start HLS streaming if enabled
                            if (config->streaming_enabled) {
                                log_info("Starting HLS streaming for stream: %s", active_streams[i]);
                                
                                // Try multiple times to start HLS streaming
                                bool hls_started = false;
                                for (int retry = 0; retry < 3 && !hls_started; retry++) {
                                    if (retry > 0) {
                                        log_info("Retry %d starting HLS streaming for stream: %s", retry, active_streams[i]);
                                        // Wait a bit before retrying
                                        usleep(500000); // 500ms
                                    }
                                    
                                    if (stream_start_hls(active_streams[i]) == 0) {
                                        log_info("HLS streaming started for stream: %s", active_streams[i]);
                                        hls_started = true;
                                    } else {
                                        log_warn("Failed to start HLS streaming for stream: %s (attempt %d/3)", 
                                                active_streams[i], retry + 1);
                                    }
                                }
                                
                                if (!hls_started) {
                                    log_error("Failed to start HLS streaming for stream: %s after multiple attempts", 
                                            active_streams[i]);
                                }
                            }
                        } else {
                            log_warn("Failed to start stream: %s", active_streams[i]);
                        }
                    } else {
                        log_error("Failed to add stream: %s", active_streams[i]);
                    }
                } else {
                    log_error("Failed to find configuration for stream: %s", active_streams[i]);
                }
            }
        }
        
        // Wait a bit to ensure all streams have time to start
        log_info("Waiting for streams to fully start...");
        sleep(2);
        
        // Force restart all HLS streams to ensure they're properly started
        log_info("Force restarting all HLS streams to ensure they're properly started...");
        for (int i = 0; i < active_stream_count; i++) {
            log_info("Force restarting HLS for active stream %d: %s", i, active_streams[i]);
            
            // Get the stream handle
            stream_handle_t stream = get_stream_by_name(active_streams[i]);
            if (stream) {
                // Get the stream configuration
                stream_config_t config;
                if (get_stream_config(stream, &config) != 0) {
                    log_error("Failed to get stream configuration for %s", active_streams[i]);
                    continue;
                }
                
                // Explicitly restart HLS streaming if enabled
                if (config.streaming_enabled) {
                    log_info("Force restarting HLS streaming for stream: %s", active_streams[i]);
                    
                    // First stop the HLS stream
                    if (stream_stop_hls(active_streams[i]) != 0) {
                        log_warn("Failed to stop HLS stream for restart: %s", active_streams[i]);
                    }

                    // Wait a bit to ensure the stream is fully stopped
                    usleep(500000); // 500ms

                    // Start the HLS stream again
                    if (stream_start_hls(active_streams[i]) == 0) {
                        log_info("HLS streaming force restarted for stream: %s", active_streams[i]);
                    } else {
                        log_warn("Failed to force restart HLS streaming for stream: %s", active_streams[i]);
                    }
                } else {
                    log_warn("Streaming not enabled for stream: %s", active_streams[i]);
                }
            } else {
                log_warn("Failed to get stream handle for force restart: %s", active_streams[i]);
            }
        }
        
        // Always start all streams from the database after changing the database path
        log_info("Starting all streams from the database after changing database path...");
        
        // Get all stream configurations from the database
        stream_config_t db_streams[MAX_STREAMS];
        int count = get_all_stream_configs(db_streams, MAX_STREAMS);
        
        if (count > 0) {
            log_info("Found %d streams in the database", count);
            
            // Start each stream
            for (int i = 0; i < count; i++) {
                if (db_streams[i].name[0] != '\0' && db_streams[i].enabled) {
                    log_info("Starting stream from database: %s (streaming_enabled=%d)", 
                            db_streams[i].name, db_streams[i].streaming_enabled);
                    
                    // Add the stream
                    stream_handle_t stream = add_stream(&db_streams[i]);
                    if (stream) {
                        log_info("Added stream from database: %s", db_streams[i].name);
                        
                        // Start the stream
                        if (start_stream(stream) == 0) {
                            log_info("Started stream from database: %s", db_streams[i].name);
                            
                            // Explicitly start HLS streaming if enabled
                            if (db_streams[i].streaming_enabled) {
                                log_info("Starting HLS streaming for database stream: %s", db_streams[i].name);
                                
                                // Try multiple times to start HLS streaming
                                bool hls_started = false;
                                for (int retry = 0; retry < 3 && !hls_started; retry++) {
                                    if (retry > 0) {
                                        log_info("Retry %d starting HLS streaming for database stream: %s", 
                                                retry, db_streams[i].name);
                                        // Wait a bit before retrying
                                        usleep(500000); // 500ms
                                    }
                                    
                                    if (stream_start_hls(db_streams[i].name) == 0) {
                                        log_info("HLS streaming started for database stream: %s", db_streams[i].name);
                                        hls_started = true;
                                    } else {
                                        log_warn("Failed to start HLS streaming for database stream: %s (attempt %d/3)", 
                                                db_streams[i].name, retry + 1);
                                    }
                                }
                                
                                if (!hls_started) {
                                    log_error("Failed to start HLS streaming for database stream: %s after multiple attempts", 
                                            db_streams[i].name);
                                }
                            } else {
                                log_warn("HLS streaming not enabled for database stream: %s", db_streams[i].name);
                            }
                        } else {
                            log_warn("Failed to start stream from database: %s", db_streams[i].name);
                        }
                    } else {
                        log_error("Failed to add stream from database: %s", db_streams[i].name);
                    }
                }
            }
            
            // Wait a bit to ensure all streams have time to start
            log_info("Waiting for database streams to fully start...");
            sleep(2);
            
            // Force restart all HLS streams to ensure they're properly started
            log_info("Force restarting all HLS streams from database to ensure they're properly started...");
            for (int i = 0; i < count; i++) {
                if (db_streams[i].name[0] != '\0' && db_streams[i].enabled && db_streams[i].streaming_enabled) {
                    log_info("Force restarting HLS for database stream: %s", db_streams[i].name);
                    
                    // First stop the HLS stream
                    if (stream_stop_hls(db_streams[i].name) != 0) {
                        log_warn("Failed to stop HLS stream for restart: %s", db_streams[i].name);
                    }

                    // Wait a bit to ensure the stream is fully stopped
                    usleep(500000); // 500ms

                    // Start the HLS stream again
                    if (stream_start_hls(db_streams[i].name) == 0) {
                        log_info("HLS streaming force restarted for database stream: %s", db_streams[i].name);
                    } else {
                        log_warn("Failed to force restart HLS streaming for database stream: %s", db_streams[i].name);
                    }
                }
            }
        } else {
            log_warn("No streams found in the database");
        }
        
        log_info("Database path changed successfully");
    }
    
        // Save settings if changed
        if (settings_changed) {
            // Use the loaded config path - save_config will handle this automatically
            const char* config_path = get_loaded_config_path();
            log_info("Saving configuration to file: %s", config_path ? config_path : "default path");
            
            // Print the current database path to verify it's set correctly
            log_info("Current database path before saving: %s", g_config.db_path);
            
            // Save to the specific config file path if available
            int save_result;
            if (config_path) {
                save_result = save_config(&g_config, config_path);
            } else {
                save_result = save_config(&g_config, NULL);
            }
            
            if (save_result != 0) {
                log_error("Failed to save configuration, error code: %d", save_result);
                cJSON_Delete(settings);
                http_response_set_json_error(res, 500, "Failed to save configuration");
                return;
            }
            
            log_info("Configuration saved successfully");

            // Reload the configuration to ensure changes are applied
            log_info("Reloading configuration after save");
            if (reload_config(&g_config) != 0) {
                log_warn("Failed to reload configuration after save, changes may not be applied until restart");
            } else {
                log_info("Configuration reloaded successfully");

                // Verify the database path after reload
                log_info("Database path after reload: %s", g_config.db_path);
            }

            // If go2rtc-related settings changed, spawn background thread
            // to handle start/stop (avoids blocking the API response for 5-15+ seconds)
            if (go2rtc_config_changed) {
                go2rtc_settings_task_t *task = calloc(1, sizeof(go2rtc_settings_task_t));
                if (task) {
                    task->becoming_enabled = go2rtc_becoming_enabled;

                    pthread_t thread_id;
                    pthread_attr_t attr;
                    pthread_attr_init(&attr);
                    pthread_attr_setdetachstate(&attr, PTHREAD_CREATE_DETACHED);

                    if (pthread_create(&thread_id, &attr,
                                       (void *(*)(void *))go2rtc_settings_worker, task) != 0) {
                        log_error("Failed to create go2rtc settings worker thread");
                        free(task);
                    } else {
                        log_info("go2rtc settings change dispatched to background thread (becoming_%s)",
                                 go2rtc_becoming_enabled ? "enabled" : "disabled");
                    }
                    pthread_attr_destroy(&attr);
                } else {
                    log_error("Failed to allocate go2rtc settings task");
                }
            }

            // If MQTT-related settings changed, spawn background thread
            // to handle cleanup + reinit (avoids blocking the API response)
            if (mqtt_config_changed) {
                mqtt_settings_task_t *task = calloc(1, sizeof(mqtt_settings_task_t));
                if (task) {
                    task->mqtt_now_enabled = g_config.mqtt_enabled;

                    pthread_t thread_id;
                    pthread_attr_t attr;
                    pthread_attr_init(&attr);
                    pthread_attr_setdetachstate(&attr, PTHREAD_CREATE_DETACHED);

                    if (pthread_create(&thread_id, &attr,
                                       (void *(*)(void *))mqtt_settings_worker, task) != 0) {
                        log_error("Failed to create MQTT settings worker thread");
                        free(task);
                    } else {
                        log_info("MQTT settings change dispatched to background thread");
                    }
                    pthread_attr_destroy(&attr);
                } else {
                    log_error("Failed to allocate MQTT settings task");
                }
            }
        } else {
            log_info("No settings changed");
        }
    
    // Clean up
    cJSON_Delete(settings);
    
    // Create success response using cJSON
    cJSON *success = cJSON_CreateObject();
    if (!success) {
        log_error("Failed to create success JSON object");
        http_response_set_json_error(res, 500, "Failed to create success JSON");
        return;
    }
    
    cJSON_AddBoolToObject(success, "success", true);
    
    // Convert to string
    char *json_str = cJSON_PrintUnformatted(success);
    if (!json_str) {
        log_error("Failed to convert success JSON to string");
        cJSON_Delete(success);
        http_response_set_json_error(res, 500, "Failed to convert success JSON to string");
        return;
    }
    
    // Send response
    http_response_set_json(res, 200, json_str);
    
    // Clean up
    free(json_str);
    cJSON_Delete(success);
    
    log_info("Successfully handled POST /api/settings request");
}
