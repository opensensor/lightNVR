#define _XOPEN_SOURCE
#define _GNU_SOURCE

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <ctype.h>
#include <unistd.h>
#include <pthread.h>

#include "web/api_handlers.h"
#include "web/request_response.h"
#include "web/httpd_utils.h"
#include "core/logger.h"
#include "core/config.h"
#include "video/stream_manager.h"
#include "video/streams.h"
#include "video/stream_state.h"
#include "video/detection_stream.h"
#include "video/unified_detection_thread.h"
#include "database/database_manager.h"
#include "video/hls/hls_directory.h"
#include "video/hls/hls_api.h"
#include "video/onvif_device_management.h"
#include "video/go2rtc/go2rtc_stream.h"
#include "video/go2rtc/go2rtc_integration.h"
#include "video/go2rtc/go2rtc_api.h"
#include "video/mp4_recording.h"


/**
 * @brief Structure for PUT stream update task
 */
typedef struct {
    stream_handle_t stream;                    // Stream handle
    stream_config_t config;                    // Updated stream configuration
    char decoded_id[MAX_STREAM_NAME];          // Decoded stream ID
    char original_url[MAX_URL_LENGTH];         // Original URL before update
    stream_protocol_t original_protocol;       // Original protocol before update
    bool original_record_audio;                // Original record_audio before update
    bool config_changed;                       // Whether config changed
    bool requires_restart;                     // Whether restart is required
    bool is_running;                           // Whether stream was running
    bool onvif_test_performed;                 // Whether ONVIF test was performed
    bool onvif_test_success;                   // Whether ONVIF test succeeded
    bool original_detection_based_recording;   // Original detection state
    bool has_detection_based_recording;        // Whether detection_based_recording was provided
    bool detection_based_recording_value;      // Value of detection_based_recording
    bool has_detection_model;                  // Whether detection_model was provided
    bool has_detection_threshold;              // Whether detection_threshold was provided
    bool has_detection_interval;               // Whether detection_interval was provided
    bool has_record;                           // Whether record flag was provided
    bool has_streaming_enabled;                // Whether streaming_enabled flag was provided
    bool non_dynamic_config_changed;           // Whether non-dynamic fields changed
} put_stream_task_t;

/**
 * @brief Free a PUT stream task
 */
static void put_stream_task_free(put_stream_task_t *task) {
    if (task) {
        free(task);
    }
}

/**
 * @brief Worker function for PUT stream update
 *
 * This function performs the actual stream update work in a background thread.
 */
static void put_stream_worker(put_stream_task_t *task) {
    if (!task) {
        log_error("Invalid PUT stream task");
        return;
    }

    log_info("Processing PUT /api/streams/%s in worker thread", task->decoded_id);

    // Update stream configuration in database first
    if (update_stream_config(task->decoded_id, &task->config) != 0) {
        log_error("Failed to update stream configuration in database for %s", task->decoded_id);
        put_stream_task_free(task);
        return;
    }

    // Force update of stream configuration in memory to ensure it matches the database
    stream_config_t updated_config;
    if (get_stream_config(task->stream, &updated_config) != 0) {
        log_error("Failed to refresh stream configuration from database for stream %s", task->config.name);
        put_stream_task_free(task);
        return;
    }

    if (set_stream_detection_params(task->stream,
                                   task->config.detection_interval,
                                   task->config.detection_threshold,
                                   task->config.pre_detection_buffer,
                                   task->config.post_detection_buffer) != 0) {
        log_warn("Failed to update detection parameters for stream %s", task->config.name);
    }

    if (set_stream_detection_recording(task->stream,
                                      task->config.detection_based_recording,
                                      task->config.detection_model) != 0) {
        log_warn("Failed to update detection recording for stream %s", task->config.name);
    }

    // If detection settings were changed and the stream is running,
    // we need to restart the stream to apply the new detection settings
    if (task->config_changed &&
        (task->has_detection_based_recording || task->has_detection_model ||
         task->has_detection_threshold || task->has_detection_interval) &&
        task->is_running && !task->requires_restart) {
        log_info("Detection settings changed for stream %s, marking for restart to apply changes", task->config.name);
        task->requires_restart = true;
    }

    // Update other stream properties in memory (only if they were provided in the JSON)
    if (task->has_record) {
        log_info("PUT stream worker: About to call set_stream_recording for stream '%s' with value %d (%s)",
                task->config.name,
                task->config.record,
                task->config.record ? "enabled" : "disabled");
        if (set_stream_recording(task->stream, task->config.record) != 0) {
            log_warn("Failed to update recording setting for stream %s", task->config.name);
        }
    }

    if (task->has_streaming_enabled) {
        if (set_stream_streaming_enabled(task->stream, task->config.streaming_enabled) != 0) {
            log_warn("Failed to update streaming setting for stream %s", task->config.name);
        }
    }

    // Handle detection thread management based on detection_based_recording setting
    bool detection_was_enabled = false;
    bool detection_now_enabled = false;

    // Check if detection_based_recording was changed in this request
    if (task->has_detection_based_recording) {
        detection_was_enabled = task->original_detection_based_recording;
        detection_now_enabled = task->detection_based_recording_value;
    } else {
        detection_now_enabled = task->config.detection_based_recording;
        detection_was_enabled = is_unified_detection_running(task->config.name);
        if (detection_now_enabled && !detection_was_enabled) {
            log_info("Detection is enabled in config for stream %s but no thread is running", task->config.name);
            detection_was_enabled = false;
        }
    }

    // If detection was enabled and now disabled, stop the detection thread
    if (detection_was_enabled && !detection_now_enabled) {
        log_info("Detection disabled for stream %s, stopping unified detection thread", task->config.name);
        if (stop_unified_detection_thread(task->config.name) != 0) {
            log_warn("Failed to stop unified detection thread for stream %s", task->config.name);
        } else {
            log_info("Successfully stopped unified detection thread for stream %s", task->config.name);
        }
    }
    // If detection was disabled and now enabled, start the detection thread
    else if (!detection_was_enabled && detection_now_enabled) {
        if (task->config.detection_model[0] != '\0' && task->config.enabled) {
            // If continuous recording is also enabled, run detection in annotation-only mode
            bool annotation_only = task->config.record;
            log_info("Detection enabled for stream %s, starting unified detection thread with model %s (annotation_only=%s)",
                    task->config.name, task->config.detection_model, annotation_only ? "true" : "false");

            if (go2rtc_integration_reload_stream(task->config.name)) {
                log_info("Successfully ensured stream %s is registered with go2rtc", task->config.name);
            } else {
                log_warn("Failed to ensure stream %s is registered with go2rtc", task->config.name);
            }

            if (start_unified_detection_thread(task->config.name,
                                              task->config.detection_model,
                                              task->config.detection_threshold,
                                              task->config.pre_detection_buffer,
                                              task->config.post_detection_buffer,
                                              annotation_only) != 0) {
                log_warn("Failed to start unified detection thread for stream %s", task->config.name);
            } else {
                log_info("Successfully started unified detection thread for stream %s", task->config.name);
            }
        } else {
            log_warn("Detection enabled for stream %s but no model specified or stream disabled", task->config.name);
        }
    }
    // If detection settings changed but detection was already enabled, restart the thread
    else if (detection_now_enabled && (task->has_detection_model || task->has_detection_threshold || task->has_detection_interval)) {
        log_info("Detection settings changed for stream %s, restarting unified detection thread", task->config.name);

        if (stop_unified_detection_thread(task->config.name) != 0) {
            log_warn("Failed to stop existing unified detection thread for stream %s", task->config.name);
        }

        if (task->config.detection_model[0] != '\0' && task->config.enabled) {
            // If continuous recording is also enabled, run detection in annotation-only mode
            bool annotation_only = task->config.record;
            if (start_unified_detection_thread(task->config.name,
                                              task->config.detection_model,
                                              task->config.detection_threshold,
                                              task->config.pre_detection_buffer,
                                              task->config.post_detection_buffer,
                                              annotation_only) != 0) {
                log_warn("Failed to restart unified detection thread for stream %s", task->config.name);
            } else {
                log_info("Successfully restarted unified detection thread for stream %s", task->config.name);
            }
        }
    }

    log_info("Updated stream configuration in memory for stream %s", task->config.name);

    // Verify the update by reading back the configuration
    if (get_stream_config(task->stream, &updated_config) == 0) {
        log_info("Detection settings after update - Model: %s, Threshold: %.2f, Interval: %d, Pre-buffer: %d, Post-buffer: %d",
                 updated_config.detection_model, updated_config.detection_threshold, updated_config.detection_interval,
                 updated_config.pre_detection_buffer, updated_config.post_detection_buffer);
    }

    // Restart stream if configuration changed and either:
    // 1. Critical parameters requiring restart were changed (URL, protocol, record_audio)
    // 2. The stream is currently running AND non-dynamic parameters changed
    // Note: record and streaming_enabled can be toggled dynamically without restart
    // Note: retention and PTZ settings are metadata only and don't require restart

    if (task->config_changed && (task->requires_restart || (task->is_running && task->non_dynamic_config_changed))) {
        log_info("Restarting stream %s (requires_restart=%s, is_running=%s, non_dynamic_changed=%s)",
                task->config.name,
                task->requires_restart ? "true" : "false",
                task->is_running ? "true" : "false",
                task->non_dynamic_config_changed ? "true" : "false");

        bool url_changed = strcmp(task->original_url, task->config.url) != 0;
        bool protocol_changed = task->original_protocol != task->config.protocol;
        bool record_audio_changed = task->original_record_audio != task->config.record_audio;

        // First clear HLS segments if URL changed
        if (url_changed) {
            log_info("URL changed for stream %s, clearing HLS segments", task->config.name);
            if (clear_stream_hls_segments(task->config.name) != 0) {
                log_warn("Failed to clear HLS segments for stream %s", task->config.name);
            }
        }

        // Stop stream if it's running
        if (task->is_running) {
            log_info("Stopping stream %s for restart", task->config.name);

            if (stop_stream(task->stream) != 0) {
                log_error("Failed to stop stream: %s", task->decoded_id);
            }

            // Wait for stream to stop with increased timeout for critical parameter changes
            int timeout = task->requires_restart ? 50 : 30;
            while (get_stream_status(task->stream) != STREAM_STATUS_STOPPED && timeout > 0) {
                usleep(100000); // 100ms
                timeout--;
            }

            if (timeout == 0) {
                log_warn("Timeout waiting for stream %s to stop, continuing anyway", task->config.name);
            }
        }

        // If URL, protocol, or record_audio changed, update go2rtc stream registration
        if ((url_changed || protocol_changed || record_audio_changed)) {
            log_info("URL, protocol, or record_audio changed for stream %s, updating go2rtc registration", task->config.name);

            if (go2rtc_integration_reload_stream_config(task->config.name, task->config.url,
                                                        task->config.onvif_username[0] != '\0' ? task->config.onvif_username : NULL,
                                                        task->config.onvif_password[0] != '\0' ? task->config.onvif_password : NULL,
                                                        task->config.backchannel_enabled ? 1 : 0,
                                                        task->config.protocol,
                                                        task->config.record_audio ? 1 : 0)) {
                log_info("Successfully reloaded stream %s in go2rtc with updated config", task->config.name);
            } else {
                log_error("Failed to reload stream %s in go2rtc", task->config.name);
            }

            // Wait a moment for go2rtc to be ready
            usleep(500000); // 500ms
        }

        // Start stream if enabled (AFTER go2rtc has been updated)
        if (task->config.enabled) {
            log_info("Starting stream %s after configuration update", task->config.name);
            if (start_stream(task->stream) != 0) {
                log_error("Failed to restart stream: %s", task->decoded_id);
            }

            // Force restart the HLS stream thread if streaming is enabled and URL/protocol changed
            if ((url_changed || protocol_changed) && task->config.streaming_enabled) {
                log_info("Force restarting HLS stream thread for %s after go2rtc update", task->config.name);
                if (stream_restart_hls(task->config.name) != 0) {
                    log_warn("Failed to force restart HLS stream for %s", task->config.name);
                } else {
                    log_info("Successfully force restarted HLS stream for %s", task->config.name);
                }
            }
        }
    } else if (task->config_changed) {
        log_info("Configuration changed for stream %s but restart not required", task->config.name);
    }

    log_info("Successfully completed stream update for: %s", task->decoded_id);

    // Clean up
    put_stream_task_free(task);
}

/**
 * @brief Handler function for PUT stream
 *
 * This function is called by the multithreading system.
 */
// put_stream_handler removed - work done via update_stream_task_func

/**
 * @brief Direct handler for POST /api/streams
 */
void handle_post_stream(const http_request_t *req, http_response_t *res) {
    log_info("Handling POST /api/streams request");

    // Parse JSON from request body
    cJSON *stream_json = httpd_parse_json_body(req);
    if (!stream_json) {
        log_error("Failed to parse stream JSON from request body");
        http_response_set_json_error(res, 400, "Invalid JSON in request body");
        return;
    }

    // Extract stream configuration
    stream_config_t config;
    memset(&config, 0, sizeof(config));

    // Required fields
    cJSON *name = cJSON_GetObjectItem(stream_json, "name");
    cJSON *url = cJSON_GetObjectItem(stream_json, "url");

    if (!name || !cJSON_IsString(name) || !url || !cJSON_IsString(url)) {
        log_error("Missing required fields in stream configuration");
        cJSON_Delete(stream_json);
        http_response_set_json_error(res, 400, "Missing required fields (name, url)");
        return;
    }

    // Copy name and URL
    strncpy(config.name, name->valuestring, sizeof(config.name) - 1);
    strncpy(config.url, url->valuestring, sizeof(config.url) - 1);

    // Optional fields with defaults
    config.enabled = true;
    config.streaming_enabled = true;
    config.width = 0;   // Auto-detected from stream
    config.height = 0;  // Auto-detected from stream
    config.fps = 0;     // Auto-detected from stream
    config.codec[0] = '\0'; // Auto-detected from stream
    config.priority = 5;
    config.record = true;
    config.segment_duration = 60;
    config.detection_based_recording = false;
    config.detection_interval = 10;
    config.detection_threshold = 0.5f;
    config.pre_detection_buffer = 5;
    config.post_detection_buffer = 5;
    config.protocol = STREAM_PROTOCOL_TCP;
    config.record_audio = true; // Default to true for new streams

    // Override with provided values
    cJSON *enabled = cJSON_GetObjectItem(stream_json, "enabled");
    if (enabled && cJSON_IsBool(enabled)) {
        config.enabled = cJSON_IsTrue(enabled);
    }

    cJSON *streaming_enabled = cJSON_GetObjectItem(stream_json, "streaming_enabled");
    if (streaming_enabled && cJSON_IsBool(streaming_enabled)) {
        config.streaming_enabled = cJSON_IsTrue(streaming_enabled);
    }

    // Note: width, height, fps, and codec are auto-detected from the stream
    // and are not accepted from API input. They remain at their defaults (0/empty)
    // until populated by stream probing or ONVIF discovery.

    cJSON *priority = cJSON_GetObjectItem(stream_json, "priority");
    if (priority && cJSON_IsNumber(priority)) {
        config.priority = priority->valueint;
    }

    cJSON *record = cJSON_GetObjectItem(stream_json, "record");
    if (record && cJSON_IsBool(record)) {
        config.record = cJSON_IsTrue(record);
    }

    cJSON *segment_duration = cJSON_GetObjectItem(stream_json, "segment_duration");
    if (segment_duration && cJSON_IsNumber(segment_duration)) {
        config.segment_duration = segment_duration->valueint;
    }

    cJSON *detection_based_recording = cJSON_GetObjectItem(stream_json, "detection_based_recording");
    if (detection_based_recording && cJSON_IsBool(detection_based_recording)) {
        config.detection_based_recording = cJSON_IsTrue(detection_based_recording);
    }

    cJSON *detection_model = cJSON_GetObjectItem(stream_json, "detection_model");
    if (detection_model && cJSON_IsString(detection_model)) {
        strncpy(config.detection_model, detection_model->valuestring, sizeof(config.detection_model) - 1);
    }

    cJSON *detection_threshold = cJSON_GetObjectItem(stream_json, "detection_threshold");
    if (detection_threshold && cJSON_IsNumber(detection_threshold)) {
        // Convert from percentage (0-100) to float (0.0-1.0)
        config.detection_threshold = (float)(detection_threshold->valuedouble / 100.0);
    }

    cJSON *detection_interval = cJSON_GetObjectItem(stream_json, "detection_interval");
    if (detection_interval && cJSON_IsNumber(detection_interval)) {
        config.detection_interval = detection_interval->valueint;
    }

    cJSON *pre_detection_buffer = cJSON_GetObjectItem(stream_json, "pre_detection_buffer");
    if (pre_detection_buffer && cJSON_IsNumber(pre_detection_buffer)) {
        config.pre_detection_buffer = pre_detection_buffer->valueint;
    }

    cJSON *post_detection_buffer = cJSON_GetObjectItem(stream_json, "post_detection_buffer");
    if (post_detection_buffer && cJSON_IsNumber(post_detection_buffer)) {
        config.post_detection_buffer = post_detection_buffer->valueint;
    }

    cJSON *detection_object_filter = cJSON_GetObjectItem(stream_json, "detection_object_filter");
    if (detection_object_filter && cJSON_IsString(detection_object_filter)) {
        strncpy(config.detection_object_filter, detection_object_filter->valuestring, sizeof(config.detection_object_filter) - 1);
        config.detection_object_filter[sizeof(config.detection_object_filter) - 1] = '\0';
    }

    cJSON *detection_object_filter_list = cJSON_GetObjectItem(stream_json, "detection_object_filter_list");
    if (detection_object_filter_list && cJSON_IsString(detection_object_filter_list)) {
        strncpy(config.detection_object_filter_list, detection_object_filter_list->valuestring, sizeof(config.detection_object_filter_list) - 1);
        config.detection_object_filter_list[sizeof(config.detection_object_filter_list) - 1] = '\0';
    }

    cJSON *protocol = cJSON_GetObjectItem(stream_json, "protocol");
    if (protocol && cJSON_IsNumber(protocol)) {
        config.protocol = (stream_protocol_t)protocol->valueint;
    }

    cJSON *record_audio = cJSON_GetObjectItem(stream_json, "record_audio");
    if (record_audio && cJSON_IsBool(record_audio)) {
        config.record_audio = cJSON_IsTrue(record_audio);
        log_info("Audio recording %s for stream %s",
                config.record_audio ? "enabled" : "disabled", config.name);
    }

    // Check if backchannel_enabled flag is set in the request
    cJSON *backchannel_enabled = cJSON_GetObjectItem(stream_json, "backchannel_enabled");
    if (backchannel_enabled && cJSON_IsBool(backchannel_enabled)) {
        config.backchannel_enabled = cJSON_IsTrue(backchannel_enabled);
        log_info("Backchannel audio %s for stream %s",
                config.backchannel_enabled ? "enabled" : "disabled", config.name);
    }

    // Parse retention policy settings
    cJSON *retention_days = cJSON_GetObjectItem(stream_json, "retention_days");
    if (retention_days && cJSON_IsNumber(retention_days)) {
        config.retention_days = retention_days->valueint;
    }

    cJSON *detection_retention_days = cJSON_GetObjectItem(stream_json, "detection_retention_days");
    if (detection_retention_days && cJSON_IsNumber(detection_retention_days)) {
        config.detection_retention_days = detection_retention_days->valueint;
    }

    cJSON *max_storage_mb = cJSON_GetObjectItem(stream_json, "max_storage_mb");
    if (max_storage_mb && cJSON_IsNumber(max_storage_mb)) {
        config.max_storage_mb = max_storage_mb->valueint;
    }

    // Parse tier multiplier settings
    cJSON *tier_critical_multiplier = cJSON_GetObjectItem(stream_json, "tier_critical_multiplier");
    if (tier_critical_multiplier && cJSON_IsNumber(tier_critical_multiplier)) {
        config.tier_critical_multiplier = tier_critical_multiplier->valuedouble;
    }

    cJSON *tier_important_multiplier = cJSON_GetObjectItem(stream_json, "tier_important_multiplier");
    if (tier_important_multiplier && cJSON_IsNumber(tier_important_multiplier)) {
        config.tier_important_multiplier = tier_important_multiplier->valuedouble;
    }

    cJSON *tier_ephemeral_multiplier = cJSON_GetObjectItem(stream_json, "tier_ephemeral_multiplier");
    if (tier_ephemeral_multiplier && cJSON_IsNumber(tier_ephemeral_multiplier)) {
        config.tier_ephemeral_multiplier = tier_ephemeral_multiplier->valuedouble;
    }

    cJSON *storage_priority = cJSON_GetObjectItem(stream_json, "storage_priority");
    if (storage_priority && cJSON_IsNumber(storage_priority)) {
        config.storage_priority = storage_priority->valueint;
    }

    // Parse PTZ settings
    cJSON *ptz_enabled = cJSON_GetObjectItem(stream_json, "ptz_enabled");
    if (ptz_enabled && cJSON_IsBool(ptz_enabled)) {
        config.ptz_enabled = cJSON_IsTrue(ptz_enabled);
        log_info("PTZ %s for stream %s",
                config.ptz_enabled ? "enabled" : "disabled", config.name);
    }

    cJSON *ptz_max_x = cJSON_GetObjectItem(stream_json, "ptz_max_x");
    if (ptz_max_x && cJSON_IsNumber(ptz_max_x)) {
        config.ptz_max_x = ptz_max_x->valueint;
    }

    cJSON *ptz_max_y = cJSON_GetObjectItem(stream_json, "ptz_max_y");
    if (ptz_max_y && cJSON_IsNumber(ptz_max_y)) {
        config.ptz_max_y = ptz_max_y->valueint;
    }

    cJSON *ptz_max_z = cJSON_GetObjectItem(stream_json, "ptz_max_z");
    if (ptz_max_z && cJSON_IsNumber(ptz_max_z)) {
        config.ptz_max_z = ptz_max_z->valueint;
    }

    cJSON *ptz_has_home = cJSON_GetObjectItem(stream_json, "ptz_has_home");
    if (ptz_has_home && cJSON_IsBool(ptz_has_home)) {
        config.ptz_has_home = cJSON_IsTrue(ptz_has_home);
    }

    // Parse recording schedule settings
    cJSON *record_on_schedule = cJSON_GetObjectItem(stream_json, "record_on_schedule");
    if (record_on_schedule && cJSON_IsBool(record_on_schedule)) {
        config.record_on_schedule = cJSON_IsTrue(record_on_schedule);
    }

    cJSON *recording_schedule = cJSON_GetObjectItem(stream_json, "recording_schedule");
    if (recording_schedule && cJSON_IsArray(recording_schedule)) {
        int arr_size = cJSON_GetArraySize(recording_schedule);
        if (arr_size == 168) {
            for (int j = 0; j < 168; j++) {
                cJSON *item = cJSON_GetArrayItem(recording_schedule, j);
                config.recording_schedule[j] = (item && cJSON_IsTrue(item)) ? 1 : 0;
            }
        }
    } else {
        // Default: all hours enabled
        memset(config.recording_schedule, 1, sizeof(config.recording_schedule));
    }

    // Parse tags setting (comma-separated list, e.g. "outdoor,critical,entrance")
    cJSON *tags_post = cJSON_GetObjectItem(stream_json, "tags");
    if (tags_post && cJSON_IsString(tags_post)) {
        strncpy(config.tags, tags_post->valuestring, sizeof(config.tags) - 1);
        config.tags[sizeof(config.tags) - 1] = '\0';
    } else {
        config.tags[0] = '\0';
    }

    // Check if isOnvif flag is set in the request
    cJSON *is_onvif = cJSON_GetObjectItem(stream_json, "isOnvif");
    if (is_onvif && cJSON_IsBool(is_onvif)) {
        config.is_onvif = cJSON_IsTrue(is_onvif);
    } else {
        // Fall back to URL-based detection if not explicitly set
        config.is_onvif = (strstr(config.url, "onvif") != NULL);
    }

    log_info("ONVIF flag for stream %s: %s", config.name, config.is_onvif ? "true" : "false");

    // If ONVIF flag is set, test the connection
    bool onvif_test_success = true;
    bool onvif_test_performed = false;
    if (config.is_onvif) {
        onvif_test_performed = true;
        log_info("Testing ONVIF capabilities for stream %s", config.name);

        // Extract username and password if provided
        cJSON *onvif_username = cJSON_GetObjectItem(stream_json, "onvif_username");
        cJSON *onvif_password = cJSON_GetObjectItem(stream_json, "onvif_password");

        if (onvif_username && cJSON_IsString(onvif_username)) {
            strncpy(config.onvif_username, onvif_username->valuestring, sizeof(config.onvif_username) - 1);
            config.onvif_username[sizeof(config.onvif_username) - 1] = '\0';
        }

        if (onvif_password && cJSON_IsString(onvif_password)) {
            strncpy(config.onvif_password, onvif_password->valuestring, sizeof(config.onvif_password) - 1);
            config.onvif_password[sizeof(config.onvif_password) - 1] = '\0';
        }

        // Test ONVIF connection
        int result = test_onvif_connection(config.url,
                                          config.onvif_username[0] ? config.onvif_username : NULL,
                                          config.onvif_password[0] ? config.onvif_password : NULL);

        onvif_test_success = (result == 0);

        // If ONVIF test fails, keep user selection but report status
        if (!onvif_test_success) {
            log_warn("ONVIF test failed for stream %s; keeping user-selected ONVIF flag", config.name);
            // Do not override config.is_onvif here; persist as provided by user
        }
    }

    // Clean up JSON
    cJSON_Delete(stream_json);

    // Check if stream already exists
    stream_handle_t existing_stream = get_stream_by_name(config.name);
    if (existing_stream) {
        log_error("Stream already exists: %s", config.name);
        http_response_set_json_error(res, 409, "Stream already exists");
        return;
    }

    // Add stream to database
    uint64_t stream_id = add_stream_config(&config);
    if (stream_id == 0) {
        log_error("Failed to add stream configuration to database");
        http_response_set_json_error(res, 500, "Failed to add stream configuration");
        return;
    }

    // Create stream in memory from the database configuration
    // This also registers the stream with go2rtc via go2rtc_integration_register_stream()
    stream_handle_t stream = add_stream(&config);
    if (!stream) {
        log_error("Failed to create stream: %s", config.name);
        // Delete from database since we couldn't create it in memory
        delete_stream_config(config.name);
        http_response_set_json_error(res, 500, "Failed to create stream");
        return;
    }

    // Note: Stream is already registered with go2rtc by add_stream()
    // No need to call go2rtc_sync_streams_from_database() which would sync ALL streams

    // Start stream if enabled
    if (config.enabled) {
        if (start_stream(stream) != 0) {
            log_error("Failed to start stream: %s", config.name);
            // Continue anyway, stream is created
        }

        // Start detection thread if detection is enabled and we have a model
        if (config.detection_based_recording && config.detection_model[0] != '\0') {
            // If continuous recording is also enabled, run detection in annotation-only mode
            bool annotation_only = config.record;
            log_info("Detection enabled for new stream %s, starting unified detection thread with model %s (annotation_only=%s)",
                    config.name, config.detection_model, annotation_only ? "true" : "false");

            // Start unified detection thread
            if (start_unified_detection_thread(config.name,
                                              config.detection_model,
                                              config.detection_threshold,
                                              config.pre_detection_buffer,
                                              config.post_detection_buffer,
                                              annotation_only) != 0) {
                log_warn("Failed to start unified detection thread for new stream %s", config.name);
            } else {
                log_info("Successfully started unified detection thread for new stream %s", config.name);
            }
        }
    }

    // Create success response using cJSON
    cJSON *success = cJSON_CreateObject();
    if (!success) {
        log_error("Failed to create success JSON object");
        http_response_set_json_error(res, 500, "Failed to create success JSON");
        return;
    }

    cJSON_AddBoolToObject(success, "success", true);

    // Add ONVIF detection result if applicable
    if (onvif_test_performed) {
        if (onvif_test_success) {
            cJSON_AddStringToObject(success, "onvif_status", "success");
            cJSON_AddStringToObject(success, "onvif_message", "ONVIF capabilities detected successfully");
        } else {
            cJSON_AddStringToObject(success, "onvif_status", "error");
            cJSON_AddStringToObject(success, "onvif_message", "Failed to detect ONVIF capabilities");
        }
    }

    // Convert to string
    char *json_str = cJSON_PrintUnformatted(success);
    if (!json_str) {
        log_error("Failed to convert success JSON to string");
        cJSON_Delete(success);
        http_response_set_json_error(res, 500, "Failed to convert success JSON to string");
        return;
    }

    // Send response
    http_response_set_json(res, 201, json_str);

    // Clean up
    free(json_str);
    cJSON_Delete(success);

    log_info("Successfully created stream: %s", config.name);
}

/**
 * @brief Direct handler for PUT /api/streams/:id
 */
void handle_put_stream(const http_request_t *req, http_response_t *res) {
    // Extract stream ID from URL
    char stream_id[MAX_STREAM_NAME];
    if (http_request_extract_path_param(req, "/api/streams/", stream_id, sizeof(stream_id)) != 0) {
        log_error("Failed to extract stream ID from URL");
        http_response_set_json_error(res, 400, "Invalid request path");
        return;
    }

    // URL-decode the stream identifier
    char decoded_id[MAX_STREAM_NAME];
    url_decode(stream_id, decoded_id, sizeof(decoded_id));

    log_info("Handling PUT /api/streams/%s request", decoded_id);

    // Find the stream by name
    stream_handle_t stream = get_stream_by_name(decoded_id);
    if (!stream) {
        log_error("Stream not found: %s", decoded_id);
        http_response_set_json_error(res, 404, "Stream not found");
        return;
    }

    // Get current stream configuration
    stream_config_t config;
    if (get_stream_config(stream, &config) != 0) {
        log_error("Failed to get stream configuration for: %s", decoded_id);
        http_response_set_json_error(res, 500, "Failed to get stream configuration");
        return;
    }

    // Parse JSON from request body
    cJSON *stream_json = httpd_parse_json_body(req);
    if (!stream_json) {
        log_error("Failed to parse stream JSON from request body");
        http_response_set_json_error(res, 400, "Invalid JSON in request body");
        return;
    }

    // Update configuration with provided values
    bool config_changed = false;
    bool requires_restart = false;  // Flag for changes that require stream restart
    bool has_record = false;  // Track if record flag was provided
    bool has_streaming_enabled = false;  // Track if streaming_enabled flag was provided
    bool non_dynamic_config_changed = false;  // Track if non-dynamic fields changed

    // Save original values for comparison
    char original_url[MAX_URL_LENGTH];
    strncpy(original_url, config.url, MAX_URL_LENGTH - 1);
    original_url[MAX_URL_LENGTH - 1] = '\0';

    stream_protocol_t original_protocol = config.protocol;
    bool original_record_audio = config.record_audio;

    cJSON *url = cJSON_GetObjectItem(stream_json, "url");
    if (url && cJSON_IsString(url)) {
        if (strcmp(config.url, url->valuestring) != 0) {
            strncpy(config.url, url->valuestring, sizeof(config.url) - 1);
            config_changed = true;
            requires_restart = true;  // URL changes always require restart
            log_info("URL changed from '%s' to '%s' - restart required", original_url, config.url);
        }
    }

    cJSON *enabled = cJSON_GetObjectItem(stream_json, "enabled");
    if (enabled && cJSON_IsBool(enabled)) {
        config.enabled = cJSON_IsTrue(enabled);
        config_changed = true;
        non_dynamic_config_changed = true;
    }

    cJSON *streaming_enabled = cJSON_GetObjectItem(stream_json, "streaming_enabled");
    if (streaming_enabled && cJSON_IsBool(streaming_enabled)) {
        config.streaming_enabled = cJSON_IsTrue(streaming_enabled);
        config_changed = true;
        has_streaming_enabled = true;
        // streaming_enabled can be toggled dynamically, don't set non_dynamic_config_changed
    }

    // Note: width, height, fps, and codec are auto-detected from the stream
    // and are not accepted from API input on update either.

    cJSON *priority = cJSON_GetObjectItem(stream_json, "priority");
    if (priority && cJSON_IsNumber(priority)) {
        config.priority = priority->valueint;
        config_changed = true;
        non_dynamic_config_changed = true;
    }

    cJSON *record = cJSON_GetObjectItem(stream_json, "record");
    if (record && cJSON_IsBool(record)) {
        config.record = cJSON_IsTrue(record);
        config_changed = true;
        has_record = true;
        log_info("PUT stream JSON parsing: record field parsed as %d (%s)",
                config.record,
                config.record ? "enabled" : "disabled");
        // record can be toggled dynamically, don't set non_dynamic_config_changed
    }

    cJSON *segment_duration = cJSON_GetObjectItem(stream_json, "segment_duration");
    if (segment_duration && cJSON_IsNumber(segment_duration)) {
        config.segment_duration = segment_duration->valueint;
        config_changed = true;
        non_dynamic_config_changed = true;
    }

    cJSON *detection_based_recording_json = cJSON_GetObjectItem(stream_json, "detection_based_recording");
    bool detection_based_recording_value = false;
    bool has_detection_based_recording = false;
    // Save the previous detection state BEFORE we overwrite it
    bool original_detection_based_recording = config.detection_based_recording;
    if (detection_based_recording_json && cJSON_IsBool(detection_based_recording_json)) {
        detection_based_recording_value = cJSON_IsTrue(detection_based_recording_json);
        has_detection_based_recording = true;
        config.detection_based_recording = detection_based_recording_value;
        config_changed = true;
        non_dynamic_config_changed = true;
    }

    cJSON *detection_model_json = cJSON_GetObjectItem(stream_json, "detection_model");
    char detection_model_value[256] = {0};
    bool has_detection_model = false;
    if (detection_model_json && cJSON_IsString(detection_model_json)) {
        strncpy(detection_model_value, detection_model_json->valuestring, sizeof(detection_model_value) - 1);
        has_detection_model = true;
        strncpy(config.detection_model, detection_model_value, sizeof(config.detection_model) - 1);
        config_changed = true;
        non_dynamic_config_changed = true;
    }

    cJSON *detection_threshold_json = cJSON_GetObjectItem(stream_json, "detection_threshold");
    float detection_threshold_value = 0.0f;
    bool has_detection_threshold = false;
    if (detection_threshold_json && cJSON_IsNumber(detection_threshold_json)) {
        // Convert from percentage (0-100) to float (0.0-1.0)
        detection_threshold_value = detection_threshold_json->valuedouble / 100.0f;
        has_detection_threshold = true;
        config.detection_threshold = detection_threshold_value;
        config_changed = true;
        non_dynamic_config_changed = true;
    }

    cJSON *detection_interval_json = cJSON_GetObjectItem(stream_json, "detection_interval");
    int detection_interval_value = 0;
    bool has_detection_interval = false;
    if (detection_interval_json && cJSON_IsNumber(detection_interval_json)) {
        detection_interval_value = detection_interval_json->valueint;
        has_detection_interval = true;
        config.detection_interval = detection_interval_value;
        config_changed = true;
        non_dynamic_config_changed = true;
    }

    cJSON *pre_detection_buffer = cJSON_GetObjectItem(stream_json, "pre_detection_buffer");
    if (pre_detection_buffer && cJSON_IsNumber(pre_detection_buffer)) {
        config.pre_detection_buffer = pre_detection_buffer->valueint;
        config_changed = true;
        non_dynamic_config_changed = true;
    }

    cJSON *post_detection_buffer = cJSON_GetObjectItem(stream_json, "post_detection_buffer");
    if (post_detection_buffer && cJSON_IsNumber(post_detection_buffer)) {
        config.post_detection_buffer = post_detection_buffer->valueint;
        config_changed = true;
        non_dynamic_config_changed = true;
    }

    cJSON *detection_object_filter = cJSON_GetObjectItem(stream_json, "detection_object_filter");
    if (detection_object_filter && cJSON_IsString(detection_object_filter)) {
        strncpy(config.detection_object_filter, detection_object_filter->valuestring, sizeof(config.detection_object_filter) - 1);
        config.detection_object_filter[sizeof(config.detection_object_filter) - 1] = '\0';
        config_changed = true;
    }

    cJSON *detection_object_filter_list = cJSON_GetObjectItem(stream_json, "detection_object_filter_list");
    if (detection_object_filter_list && cJSON_IsString(detection_object_filter_list)) {
        strncpy(config.detection_object_filter_list, detection_object_filter_list->valuestring, sizeof(config.detection_object_filter_list) - 1);
        config.detection_object_filter_list[sizeof(config.detection_object_filter_list) - 1] = '\0';
        config_changed = true;
    }

    cJSON *record_audio = cJSON_GetObjectItem(stream_json, "record_audio");
    if (record_audio && cJSON_IsBool(record_audio)) {
        bool original_record_audio = config.record_audio;
        config.record_audio = cJSON_IsTrue(record_audio);
        if (original_record_audio != config.record_audio) {
            config_changed = true;
            non_dynamic_config_changed = true;
            requires_restart = true;  // Audio recording changes require restart
            log_info("Audio recording changed from %s to %s - restart required",
                    original_record_audio ? "enabled" : "disabled",
                    config.record_audio ? "enabled" : "disabled");
        }
    }

    cJSON *backchannel_enabled = cJSON_GetObjectItem(stream_json, "backchannel_enabled");
    if (backchannel_enabled && cJSON_IsBool(backchannel_enabled)) {
        bool original_backchannel = config.backchannel_enabled;
        config.backchannel_enabled = cJSON_IsTrue(backchannel_enabled);
        if (original_backchannel != config.backchannel_enabled) {
            config_changed = true;
            non_dynamic_config_changed = true;
            log_info("Backchannel audio changed from %s to %s",
                    original_backchannel ? "enabled" : "disabled",
                    config.backchannel_enabled ? "enabled" : "disabled");
        }
    }

    // Parse retention policy settings - these are metadata only, don't require restart
    cJSON *retention_days = cJSON_GetObjectItem(stream_json, "retention_days");
    if (retention_days && cJSON_IsNumber(retention_days)) {
        int new_retention = retention_days->valueint;
        if (config.retention_days != new_retention) {
            config.retention_days = new_retention;
            config_changed = true;
            // Retention is metadata only, doesn't require restart
            log_info("Retention days changed to %d for stream %s", new_retention, config.name);
        }
    }

    cJSON *detection_retention_days = cJSON_GetObjectItem(stream_json, "detection_retention_days");
    if (detection_retention_days && cJSON_IsNumber(detection_retention_days)) {
        int new_detection_retention = detection_retention_days->valueint;
        if (config.detection_retention_days != new_detection_retention) {
            config.detection_retention_days = new_detection_retention;
            config_changed = true;
            // Retention is metadata only, doesn't require restart
            log_info("Detection retention days changed to %d for stream %s", new_detection_retention, config.name);
        }
    }

    cJSON *max_storage_mb = cJSON_GetObjectItem(stream_json, "max_storage_mb");
    if (max_storage_mb && cJSON_IsNumber(max_storage_mb)) {
        int new_max_storage = max_storage_mb->valueint;
        if (config.max_storage_mb != new_max_storage) {
            config.max_storage_mb = new_max_storage;
            config_changed = true;
            // Storage limit is metadata only, doesn't require restart
            log_info("Max storage MB changed to %d for stream %s", new_max_storage, config.name);
        }
    }

    // Parse tier multiplier settings - metadata only, don't require restart
    cJSON *tier_critical_multiplier = cJSON_GetObjectItem(stream_json, "tier_critical_multiplier");
    if (tier_critical_multiplier && cJSON_IsNumber(tier_critical_multiplier)) {
        double new_val = tier_critical_multiplier->valuedouble;
        if (config.tier_critical_multiplier != new_val) {
            config.tier_critical_multiplier = new_val;
            config_changed = true;
            log_info("Tier critical multiplier changed to %.2f for stream %s", new_val, config.name);
        }
    }

    cJSON *tier_important_multiplier = cJSON_GetObjectItem(stream_json, "tier_important_multiplier");
    if (tier_important_multiplier && cJSON_IsNumber(tier_important_multiplier)) {
        double new_val = tier_important_multiplier->valuedouble;
        if (config.tier_important_multiplier != new_val) {
            config.tier_important_multiplier = new_val;
            config_changed = true;
            log_info("Tier important multiplier changed to %.2f for stream %s", new_val, config.name);
        }
    }

    cJSON *tier_ephemeral_multiplier = cJSON_GetObjectItem(stream_json, "tier_ephemeral_multiplier");
    if (tier_ephemeral_multiplier && cJSON_IsNumber(tier_ephemeral_multiplier)) {
        double new_val = tier_ephemeral_multiplier->valuedouble;
        if (config.tier_ephemeral_multiplier != new_val) {
            config.tier_ephemeral_multiplier = new_val;
            config_changed = true;
            log_info("Tier ephemeral multiplier changed to %.2f for stream %s", new_val, config.name);
        }
    }

    cJSON *storage_priority = cJSON_GetObjectItem(stream_json, "storage_priority");
    if (storage_priority && cJSON_IsNumber(storage_priority)) {
        int new_val = storage_priority->valueint;
        if (config.storage_priority != new_val) {
            config.storage_priority = new_val;
            config_changed = true;
            log_info("Storage priority changed to %d for stream %s", new_val, config.name);
        }
    }

    // Parse PTZ settings - these are metadata only, don't require restart
    cJSON *ptz_enabled = cJSON_GetObjectItem(stream_json, "ptz_enabled");
    if (ptz_enabled && cJSON_IsBool(ptz_enabled)) {
        bool new_ptz_enabled = cJSON_IsTrue(ptz_enabled);
        if (config.ptz_enabled != new_ptz_enabled) {
            config.ptz_enabled = new_ptz_enabled;
            config_changed = true;
            // PTZ is metadata only, doesn't require restart
            log_info("PTZ %s for stream %s",
                    config.ptz_enabled ? "enabled" : "disabled", config.name);
        }
    }

    cJSON *ptz_max_x = cJSON_GetObjectItem(stream_json, "ptz_max_x");
    if (ptz_max_x && cJSON_IsNumber(ptz_max_x)) {
        int new_ptz_max_x = ptz_max_x->valueint;
        if (config.ptz_max_x != new_ptz_max_x) {
            config.ptz_max_x = new_ptz_max_x;
            config_changed = true;
            // PTZ is metadata only, doesn't require restart
        }
    }

    cJSON *ptz_max_y = cJSON_GetObjectItem(stream_json, "ptz_max_y");
    if (ptz_max_y && cJSON_IsNumber(ptz_max_y)) {
        int new_ptz_max_y = ptz_max_y->valueint;
        if (config.ptz_max_y != new_ptz_max_y) {
            config.ptz_max_y = new_ptz_max_y;
            config_changed = true;
            // PTZ is metadata only, doesn't require restart
        }
    }

    cJSON *ptz_max_z = cJSON_GetObjectItem(stream_json, "ptz_max_z");
    if (ptz_max_z && cJSON_IsNumber(ptz_max_z)) {
        int new_ptz_max_z = ptz_max_z->valueint;
        if (config.ptz_max_z != new_ptz_max_z) {
            config.ptz_max_z = new_ptz_max_z;
            config_changed = true;
            // PTZ is metadata only, doesn't require restart
        }
    }

    cJSON *ptz_has_home = cJSON_GetObjectItem(stream_json, "ptz_has_home");
    if (ptz_has_home && cJSON_IsBool(ptz_has_home)) {
        bool new_ptz_has_home = cJSON_IsTrue(ptz_has_home);
        if (config.ptz_has_home != new_ptz_has_home) {
            config.ptz_has_home = new_ptz_has_home;
            config_changed = true;
            // PTZ is metadata only, doesn't require restart
        }
    }

    // Parse recording schedule settings - metadata only, don't require restart
    cJSON *record_on_schedule = cJSON_GetObjectItem(stream_json, "record_on_schedule");
    if (record_on_schedule && cJSON_IsBool(record_on_schedule)) {
        bool new_ros = cJSON_IsTrue(record_on_schedule);
        if (config.record_on_schedule != new_ros) {
            config.record_on_schedule = new_ros;
            config_changed = true;
            log_info("Recording schedule mode changed to %s for stream %s",
                    new_ros ? "enabled" : "disabled", config.name);
        }
    }

    cJSON *recording_schedule = cJSON_GetObjectItem(stream_json, "recording_schedule");
    if (recording_schedule && cJSON_IsArray(recording_schedule)) {
        int arr_size = cJSON_GetArraySize(recording_schedule);
        if (arr_size == 168) {
            bool schedule_changed = false;
            for (int j = 0; j < 168; j++) {
                cJSON *item = cJSON_GetArrayItem(recording_schedule, j);
                uint8_t new_val = (item && cJSON_IsTrue(item)) ? 1 : 0;
                if (config.recording_schedule[j] != new_val) {
                    config.recording_schedule[j] = new_val;
                    schedule_changed = true;
                }
            }
            if (schedule_changed) {
                config_changed = true;
                log_info("Recording schedule updated for stream %s", config.name);
            }
        }
    }

    cJSON *protocol = cJSON_GetObjectItem(stream_json, "protocol");
    if (protocol && cJSON_IsNumber(protocol)) {
        stream_protocol_t new_protocol = (stream_protocol_t)protocol->valueint;
        if (config.protocol != new_protocol) {
            config.protocol = new_protocol;
            config_changed = true;
            non_dynamic_config_changed = true;
            requires_restart = true;  // Protocol changes always require restart
            log_info("Protocol changed from %d to %d - restart required",
                    original_protocol, config.protocol);
        }
    }

    // Parse tags setting (metadata only, no restart needed)
    cJSON *tags_put = cJSON_GetObjectItem(stream_json, "tags");
    if (tags_put && cJSON_IsString(tags_put)) {
        if (strncmp(config.tags, tags_put->valuestring, sizeof(config.tags) - 1) != 0) {
            strncpy(config.tags, tags_put->valuestring, sizeof(config.tags) - 1);
            config.tags[sizeof(config.tags) - 1] = '\0';
            config_changed = true;
            log_info("Tags changed to '%s' for stream %s", config.tags, config.name);
        }
    } else if (tags_put && cJSON_IsNull(tags_put)) {
        if (config.tags[0] != '\0') {
            config.tags[0] = '\0';
            config_changed = true;
        }
    }

    // Update is_onvif flag based on request or URL
    bool original_is_onvif = config.is_onvif;

    // Check if isOnvif flag is set in the request
    cJSON *is_onvif = cJSON_GetObjectItem(stream_json, "isOnvif");
    if (is_onvif && cJSON_IsBool(is_onvif)) {
        config.is_onvif = cJSON_IsTrue(is_onvif);
    } else {
        // Fall back to URL-based detection if not explicitly set
        config.is_onvif = (strstr(config.url, "onvif") != NULL);
    }

    if (original_is_onvif != config.is_onvif) {
        log_info("ONVIF flag changed from %s to %s",
                original_is_onvif ? "true" : "false",
                config.is_onvif ? "true" : "false");
        config_changed = true;
        non_dynamic_config_changed = true;
    }

    // If ONVIF flag is set, test the connection
    bool onvif_test_success = true;
    bool onvif_test_performed = false;

    if (config.is_onvif) {
        log_info("Testing ONVIF capabilities for stream %s", config.name);
        onvif_test_performed = true;

        // Extract username and password if provided
        cJSON *onvif_username = cJSON_GetObjectItem(stream_json, "onvif_username");
        cJSON *onvif_password = cJSON_GetObjectItem(stream_json, "onvif_password");

        if (onvif_username && cJSON_IsString(onvif_username)) {
            strncpy(config.onvif_username, onvif_username->valuestring, sizeof(config.onvif_username) - 1);
            config.onvif_username[sizeof(config.onvif_username) - 1] = '\0';
        }

        if (onvif_password && cJSON_IsString(onvif_password)) {
            strncpy(config.onvif_password, onvif_password->valuestring, sizeof(config.onvif_password) - 1);
            config.onvif_password[sizeof(config.onvif_password) - 1] = '\0';
        }

        // Test ONVIF connection
        int result = test_onvif_connection(config.url,
                                          config.onvif_username[0] ? config.onvif_username : NULL,
                                          config.onvif_password[0] ? config.onvif_password : NULL);

        onvif_test_success = (result == 0);
        // If ONVIF test fails, keep user selection but report status
        if (!onvif_test_success) {
            log_warn("ONVIF test failed for stream %s; keeping user-selected ONVIF flag", config.name);
            // Do not override config.is_onvif here; persist as provided by user
        }
    }

    // Check if there's a request to enable a disabled stream
    cJSON *enable_request = cJSON_GetObjectItem(stream_json, "enable_disabled");
    if (enable_request && cJSON_IsBool(enable_request) && cJSON_IsTrue(enable_request)) {
        // Request to enable a disabled stream
        log_info("Enable requested for disabled stream %s", decoded_id);

        // Check if the stream is currently disabled
        bool currently_disabled = false;
        sqlite3 *db = get_db_handle();
        if (db) {
            sqlite3_stmt *stmt;
            const char *sql = "SELECT enabled FROM streams WHERE name = ?;";
            if (sqlite3_prepare_v2(db, sql, -1, &stmt, NULL) == SQLITE_OK) {
                sqlite3_bind_text(stmt, 1, decoded_id, -1, SQLITE_STATIC);
                if (sqlite3_step(stmt) == SQLITE_ROW && sqlite3_column_type(stmt, 0) != SQLITE_NULL) {
                    currently_disabled = sqlite3_column_int(stmt, 0) == 0;
                }
                sqlite3_finalize(stmt);
            }
        }

        if (currently_disabled) {
            // Enable the stream by setting enabled to 1
            sqlite3 *db = get_db_handle();
            if (db) {
                sqlite3_stmt *stmt;
                const char *sql = "UPDATE streams SET enabled = 1 WHERE name = ?;";
                if (sqlite3_prepare_v2(db, sql, -1, &stmt, NULL) == SQLITE_OK) {
                    sqlite3_bind_text(stmt, 1, decoded_id, -1, SQLITE_STATIC);
                    if (sqlite3_step(stmt) != SQLITE_DONE) {
                        log_error("Failed to enable stream %s: %s", decoded_id, sqlite3_errmsg(db));
                    } else {
                        log_info("Successfully enabled stream %s", decoded_id);

                        // Get the stream configuration to register with go2rtc
                        stream_config_t stream_config;
                        if (get_stream_config_by_name(decoded_id, &stream_config) == 0) {
                            // Use centralized function to register the stream with go2rtc
                            if (go2rtc_integration_reload_stream(decoded_id)) {
                                log_info("Successfully registered stream %s with go2rtc", decoded_id);
                            } else {
                                log_warn("Failed to register stream %s with go2rtc (go2rtc may not be ready)", decoded_id);
                            }

                            // If detection is enabled for this stream, start the unified detection thread
                            if (stream_config.detection_based_recording && stream_config.detection_model[0] != '\0') {
                                // If continuous recording is also enabled, run detection in annotation-only mode
                                bool annotation_only = stream_config.record;
                                log_info("Starting unified detection thread for enabled stream %s (annotation_only=%s)",
                                         decoded_id, annotation_only ? "true" : "false");

                                // Start unified detection thread
                                if (start_unified_detection_thread(decoded_id,
                                                                  stream_config.detection_model,
                                                                  stream_config.detection_threshold,
                                                                  stream_config.pre_detection_buffer,
                                                                  stream_config.post_detection_buffer,
                                                                  annotation_only) != 0) {
                                    log_warn("Failed to start unified detection thread for stream %s", decoded_id);
                                } else {
                                    log_info("Successfully started unified detection thread for stream %s", decoded_id);
                                }
                            }
                        } else {
                            log_error("Failed to get configuration for stream %s", decoded_id);
                        }
                    }
                    sqlite3_finalize(stmt);
                }
            }
        }
    }

    // Clean up JSON
    cJSON_Delete(stream_json);

    // Check if stream is running - we'll need this information for detection settings changes
    stream_status_t status = get_stream_status(stream);
    bool is_running = (status == STREAM_STATUS_RUNNING || status == STREAM_STATUS_STARTING);

    // Create task structure for background processing
    put_stream_task_t *task = calloc(1, sizeof(put_stream_task_t));
    if (!task) {
        log_error("Failed to allocate memory for PUT stream task");
        http_response_set_json_error(res, 500, "Failed to allocate memory for task");
        return;
    }

    // Populate task with all necessary data
    task->stream = stream;
    memcpy(&task->config, &config, sizeof(stream_config_t));
    strncpy(task->decoded_id, decoded_id, MAX_STREAM_NAME - 1);
    strncpy(task->original_url, original_url, MAX_URL_LENGTH - 1);
    task->original_protocol = original_protocol;
    task->original_record_audio = original_record_audio;
    task->config_changed = config_changed;
    task->requires_restart = requires_restart;
    task->is_running = is_running;
    task->onvif_test_performed = onvif_test_performed;
    task->onvif_test_success = onvif_test_success;
    task->original_detection_based_recording = original_detection_based_recording;
    task->has_detection_based_recording = has_detection_based_recording;
    task->detection_based_recording_value = detection_based_recording_value;
    task->has_detection_model = has_detection_model;
    task->has_detection_threshold = has_detection_threshold;
    task->has_detection_interval = has_detection_interval;
    task->has_record = has_record;
    task->has_streaming_enabled = has_streaming_enabled;
    task->non_dynamic_config_changed = non_dynamic_config_changed;

    log_info("Detection settings before update - Model: %s, Threshold: %.2f, Interval: %d, Pre-buffer: %d, Post-buffer: %d",
             config.detection_model, config.detection_threshold, config.detection_interval,
             config.pre_detection_buffer, config.post_detection_buffer);

    // Send an immediate 202 Accepted response to the client
    // Include ONVIF test results if applicable
    cJSON *response = cJSON_CreateObject();
    if (!response) {
        log_error("Failed to create response JSON object");
        put_stream_task_free(task);
        http_response_set_json_error(res, 500, "Failed to create response JSON");
        return;
    }

    cJSON_AddBoolToObject(response, "success", true);
    cJSON_AddStringToObject(response, "message", "Stream update request accepted and processing");

    // Add ONVIF detection result if applicable
    if (onvif_test_performed) {
        if (onvif_test_success) {
            cJSON_AddStringToObject(response, "onvif_status", "success");
            cJSON_AddStringToObject(response, "onvif_message", "ONVIF capabilities detected successfully");
        } else {
            cJSON_AddStringToObject(response, "onvif_status", "error");
            cJSON_AddStringToObject(response, "onvif_message", "Failed to detect ONVIF capabilities");
        }
    }

    char *json_str = cJSON_PrintUnformatted(response);
    cJSON_Delete(response);

    if (!json_str) {
        log_error("Failed to convert response JSON to string");
        put_stream_task_free(task);
        http_response_set_json_error(res, 500, "Failed to convert response JSON");
        return;
    }

    http_response_set_json(res, 202, json_str);
    free(json_str);

    // Spawn background thread to perform the actual update work
    // This prevents blocking the web server event loop
    pthread_t thread_id;
    pthread_attr_t attr;
    pthread_attr_init(&attr);
    pthread_attr_setdetachstate(&attr, PTHREAD_CREATE_DETACHED);

    if (pthread_create(&thread_id, &attr, (void *(*)(void *))put_stream_worker, task) != 0) {
        log_error("Failed to create worker thread for PUT stream");
        put_stream_task_free(task);
        // Response already sent, just log the error
    } else {
        log_info("PUT stream task started in worker thread for: %s", decoded_id);
    }

    pthread_attr_destroy(&attr);
}

/**
 * @brief Structure for DELETE stream background task
 */
typedef struct {
    stream_handle_t stream;                    // Stream handle
    char decoded_id[MAX_STREAM_NAME];          // Decoded stream ID
    bool permanent_delete;                     // Whether to permanently delete
} delete_stream_task_t;

/**
 * @brief Worker function for DELETE stream
 *
 * This function performs the actual stream deletion work in a background thread.
 * It handles stopping the stream, detection threads, removing from memory,
 * database deletion, and go2rtc unregistration.
 */
static void delete_stream_worker(delete_stream_task_t *task) {
    if (!task) {
        log_error("Invalid DELETE stream task");
        return;
    }

    log_info("Processing DELETE /api/streams/%s in worker thread", task->decoded_id);

    // Stop stream if it's running
    stream_status_t status = get_stream_status(task->stream);
    if (status == STREAM_STATUS_RUNNING || status == STREAM_STATUS_STARTING) {
        if (stop_stream(task->stream) != 0) {
            log_error("Failed to stop stream: %s", task->decoded_id);
            // Continue anyway
        }

        // Wait for stream to stop
        int timeout = 30; // 3 seconds
        while (get_stream_status(task->stream) != STREAM_STATUS_STOPPED && timeout > 0) {
            usleep(100000); // 100ms
            timeout--;
        }
    }

    // Stop any unified detection thread for this stream
    if (is_unified_detection_running(task->decoded_id)) {
        log_info("Stopping unified detection thread for stream %s", task->decoded_id);
        if (stop_unified_detection_thread(task->decoded_id) != 0) {
            log_warn("Failed to stop unified detection thread for stream %s", task->decoded_id);
            // Continue anyway
        } else {
            log_info("Successfully stopped unified detection thread for stream %s", task->decoded_id);
        }
    }

    // Delete stream from memory
    if (remove_stream(task->stream) != 0) {
        log_error("Failed to delete stream from memory: %s", task->decoded_id);
        free(task);
        return;
    }

    // Delete the stream from the database (permanently or just disable)
    if (delete_stream_config_internal(task->decoded_id, task->permanent_delete) != 0) {
        log_error("Failed to %s stream configuration in database for %s",
                task->permanent_delete ? "permanently delete" : "disable",
                task->decoded_id);
        free(task);
        return;
    }

    // Unregister stream from go2rtc
    if (!go2rtc_integration_unregister_stream(task->decoded_id)) {
        log_warn("Failed to unregister stream %s from go2rtc", task->decoded_id);
        // Continue anyway - stream is deleted from database
    }

    log_info("Successfully %s stream in worker thread: %s",
            task->permanent_delete ? "permanently deleted" : "disabled",
            task->decoded_id);

    free(task);
}

/**
 * @brief Direct handler for DELETE /api/streams/:id
 *
 * Validates the request and spawns a background thread to perform the
 * blocking deletion work. Returns 202 Accepted immediately so the
 * event loop is not blocked.
 */
void handle_delete_stream(const http_request_t *req, http_response_t *res) {
    // Extract stream ID from URL
    char stream_id[MAX_STREAM_NAME];
    if (http_request_extract_path_param(req, "/api/streams/", stream_id, sizeof(stream_id)) != 0) {
        log_error("Failed to extract stream ID from URL");
        http_response_set_json_error(res, 400, "Invalid request path");
        return;
    }

    // URL-decode the stream identifier
    char decoded_id[MAX_STREAM_NAME];
    url_decode(stream_id, decoded_id, sizeof(decoded_id));

    log_info("Handling DELETE /api/streams/%s request", decoded_id);

    // Check if permanent delete is requested
    bool permanent_delete = false;
    char permanent_param[16] = {0};
    if (http_request_get_query_param(req, "permanent", permanent_param, sizeof(permanent_param)) > 0) {
        if (strcmp(permanent_param, "true") == 0) {
            permanent_delete = true;
            log_info("Permanent delete requested for stream: %s", decoded_id);
        }
    }

    // Find the stream by name
    stream_handle_t stream = get_stream_by_name(decoded_id);
    if (!stream) {
        log_error("Stream not found: %s", decoded_id);
        http_response_set_json_error(res, 404, "Stream not found");
        return;
    }

    // Allocate task for the background worker
    delete_stream_task_t *task = calloc(1, sizeof(delete_stream_task_t));
    if (!task) {
        log_error("Failed to allocate delete stream task");
        http_response_set_json_error(res, 500, "Failed to allocate delete task");
        return;
    }

    task->stream = stream;
    strncpy(task->decoded_id, decoded_id, sizeof(task->decoded_id) - 1);
    task->permanent_delete = permanent_delete;

    // Send 202 Accepted response immediately so we don't block the event loop
    cJSON *response_json = cJSON_CreateObject();
    if (!response_json) {
        log_error("Failed to create response JSON object");
        free(task);
        http_response_set_json_error(res, 500, "Failed to create response JSON");
        return;
    }

    cJSON_AddBoolToObject(response_json, "success", true);
    cJSON_AddBoolToObject(response_json, "permanent", permanent_delete);
    cJSON_AddStringToObject(response_json, "status", "accepted");

    char *json_str = cJSON_PrintUnformatted(response_json);
    cJSON_Delete(response_json);

    if (!json_str) {
        log_error("Failed to convert response JSON to string");
        free(task);
        http_response_set_json_error(res, 500, "Failed to convert response JSON to string");
        return;
    }

    http_response_set_json(res, 202, json_str);
    free(json_str);

    // Spawn background thread to perform the actual deletion work
    // This prevents blocking the web server event loop
    pthread_t thread_id;
    pthread_attr_t attr;
    pthread_attr_init(&attr);
    pthread_attr_setdetachstate(&attr, PTHREAD_CREATE_DETACHED);

    if (pthread_create(&thread_id, &attr, (void *(*)(void *))delete_stream_worker, task) != 0) {
        log_error("Failed to create worker thread for DELETE stream");
        free(task);
        // Response already sent, just log the error
    } else {
        log_info("DELETE stream task started in worker thread for: %s", decoded_id);
    }

    pthread_attr_destroy(&attr);
}

/**
 * @brief Structure for stream refresh task
 */
typedef struct {
    char stream_name[MAX_STREAM_NAME];  // Decoded stream name
} refresh_stream_task_t;

/**
 * @brief Worker function for stream refresh
 *
 * This function performs the actual go2rtc reload and detection thread
 * restart in a background thread so the event loop is not blocked.
 */
static void refresh_stream_worker(refresh_stream_task_t *task) {
    if (!task) {
        log_error("Invalid refresh stream task");
        return;
    }

    log_info("Processing stream refresh for %s in worker thread", task->stream_name);

    // If go2rtc integration is not initialized, try to start it
    if (!go2rtc_integration_is_initialized()) {
        log_info("go2rtc integration not initialized, attempting full start");
        if (!go2rtc_integration_full_start()) {
            log_error("Failed to start go2rtc integration for stream refresh: %s", task->stream_name);
            free(task);
            return;
        }
        log_info("go2rtc integration started successfully");
    }

    // Reload the stream with go2rtc
    bool success = go2rtc_integration_reload_stream(task->stream_name);

    if (success) {
        log_info("Successfully refreshed go2rtc registration for stream: %s", task->stream_name);

        // Signal the recording thread to reconnect cleanly rather than
        // discovering the stale RTSP connection through av_read_frame errors.
        signal_mp4_recording_reconnect(task->stream_name);

        // Also restart the unified detection thread if detection-based recording is enabled
        stream_handle_t stream = get_stream_by_name(task->stream_name);
        stream_config_t config;
        if (stream && get_stream_config(stream, &config) == 0) {
            if (config.detection_based_recording && config.detection_model[0] != '\0') {
                log_info("Restarting unified detection thread for stream: %s", task->stream_name);

                // Stop existing detection thread if running
                if (is_unified_detection_running(task->stream_name)) {
                    stop_unified_detection_thread(task->stream_name);
                    // Give it a moment to stop
                    usleep(500000);  // 500ms
                }

                // Start the detection thread
                // If continuous recording is also enabled, run detection in annotation-only mode
                bool annotation_only = config.record;
                if (start_unified_detection_thread(task->stream_name,
                                                   config.detection_model,
                                                   config.detection_threshold,
                                                   config.pre_detection_buffer,
                                                   config.post_detection_buffer,
                                                   annotation_only) != 0) {
                    log_warn("Failed to restart unified detection thread for stream: %s", task->stream_name);
                } else {
                    log_info("Successfully restarted unified detection thread for stream: %s", task->stream_name);
                }
            }
        }
    } else {
        log_error("Failed to refresh go2rtc registration for stream: %s", task->stream_name);
    }

    free(task);
}

/**
 * @brief Handler for POST /api/streams/{stream_name}/refresh
 *
 * This endpoint triggers a re-registration of the stream with go2rtc.
 * It's useful when WebRTC connections fail and the stream needs to be refreshed
 * without changing any configuration.
 *
 * Returns 202 Accepted immediately and performs the blocking go2rtc reload
 * work in a background thread so the event loop is not blocked.
 *
 * @param req HTTP request
 * @param res HTTP response
 */
void handle_post_stream_refresh(const http_request_t *req, http_response_t *res) {
    log_info("Handling POST /api/streams/:name/refresh request");

    // Extract stream name from URL
    char stream_name[MAX_STREAM_NAME] = {0};
    if (http_request_extract_path_param(req, "/api/streams/", stream_name, sizeof(stream_name)) != 0) {
        log_error("Failed to extract stream name from URL");
        http_response_set_json_error(res, 400, "Invalid stream name in URL");
        return;
    }

    // Remove /refresh suffix if present
    char *suffix = strstr(stream_name, "/refresh");
    if (suffix) {
        *suffix = '\0';
    }

    // URL decode the stream name
    char decoded_name[MAX_STREAM_NAME] = {0};
    url_decode(stream_name, decoded_name, sizeof(decoded_name));

    log_info("Refreshing go2rtc registration for stream: %s", decoded_name);

    // Check if the stream exists
    stream_handle_t stream = get_stream_by_name(decoded_name);
    if (!stream) {
        log_error("Stream not found: %s", decoded_name);
        http_response_set_json_error(res, 404, "Stream not found");
        return;
    }

    // Allocate task for worker thread
    refresh_stream_task_t *task = calloc(1, sizeof(refresh_stream_task_t));
    if (!task) {
        log_error("Failed to allocate refresh stream task");
        http_response_set_json_error(res, 500, "Internal server error");
        return;
    }
    strncpy(task->stream_name, decoded_name, MAX_STREAM_NAME - 1);

    // Send immediate 202 Accepted response
    cJSON *response = cJSON_CreateObject();
    if (!response) {
        log_error("Failed to create response JSON object");
        free(task);
        http_response_set_json_error(res, 500, "Failed to create response JSON");
        return;
    }

    cJSON_AddBoolToObject(response, "success", true);
    cJSON_AddStringToObject(response, "message", "Stream refresh request accepted and processing");
    cJSON_AddStringToObject(response, "stream", decoded_name);

    char *json_str = cJSON_PrintUnformatted(response);
    cJSON_Delete(response);

    if (!json_str) {
        log_error("Failed to convert response JSON to string");
        free(task);
        http_response_set_json_error(res, 500, "Failed to convert response JSON");
        return;
    }

    http_response_set_json(res, 202, json_str);
    free(json_str);

    // Spawn background thread to perform the actual refresh work
    // This prevents blocking the web server event loop
    pthread_t thread_id;
    pthread_attr_t attr;
    pthread_attr_init(&attr);
    pthread_attr_setdetachstate(&attr, PTHREAD_CREATE_DETACHED);

    if (pthread_create(&thread_id, &attr, (void *(*)(void *))refresh_stream_worker, task) != 0) {
        log_error("Failed to create worker thread for stream refresh");
        free(task);
        // Response already sent, just log the error
    } else {
        log_info("Stream refresh task started in worker thread for: %s", decoded_name);
    }

    pthread_attr_destroy(&attr);
}
