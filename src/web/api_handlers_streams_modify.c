/*
 * SPDX-License-Identifier: GPL-3.0-or-later
 * Copyright (C) 2025-2026 OpenSensor Engineering and LightNVR contributors
 *
 * This file is part of LightNVR <https://github.com/opensensor/lightNVR>.
 */

#define _XOPEN_SOURCE
#define _GNU_SOURCE

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <strings.h>
#include <ctype.h>
#include <unistd.h>
#include <pthread.h>
#include <time.h>
#include <stdint.h>

#include "web/api_handlers.h"
#include "web/request_response.h"
#include "web/httpd_utils.h"
#define LOG_COMPONENT "StreamsAPI"
#include "core/logger.h"
#include "core/config.h"
#include "core/path_utils.h"
#include "core/url_utils.h"
#include "utils/strings.h"
#include "video/stream_manager.h"
#include "video/streams.h"
#include "video/stream_state.h"
#include "video/detection_stream.h"
#include "video/unified_detection_thread.h"
#include "database/database_manager.h"
#include "database/db_auth.h"
#include "database/db_fleet_query.h"
#include "video/hls/hls_directory.h"
#include "video/hls/hls_api.h"
#include "video/onvif_device_management.h"
#include "video/go2rtc/go2rtc_stream.h"
#include "video/go2rtc/go2rtc_integration.h"
#include "video/go2rtc/go2rtc_api.h"
#include "video/go2rtc/go2rtc_lifecycle.h"
#include "video/mp4_recording.h"
#include "utils/yaml_validate.h"

static pthread_mutex_t g_put_stream_update_mutex = PTHREAD_MUTEX_INITIALIZER;

static bool check_stream_write_access(const http_request_t *req,
                                      http_response_t *res,
                                      stream_config_t *config) {
    if (!config) return httpd_authorize_global_action(req, res, AUTHZ_CAMERA_CONFIGURE) != 0;
    user_t user;
    fleet_camera_t camera;
    authorization_evaluation_t evaluation;
    if (!httpd_authorize_stream_action_with_context(req, res, AUTHZ_CAMERA_CONFIGURE,
            config->name, &user, &camera, &evaluation)) return false;
    /* Bind asynchronous work to the identity actually authorized, not a name
     * which can be reused before the worker is scheduled. */
    safe_strcpy(config->camera_uuid, camera.camera_uuid, sizeof(config->camera_uuid), 0);
    return true;
}

static bool stream_task_identity_matches(const char *name, const char *camera_uuid) {
    fleet_camera_t current;
    return camera_uuid[0] && db_fleet_camera_find_by_name_readonly(name, &current) == 0 &&
           strcmp(camera_uuid, current.camera_uuid) == 0;
}


/**
 * @brief Structure for PUT stream update task
 */
typedef struct {
    stream_config_t config;                    // Updated stream configuration
    char stream_id[MAX_STREAM_NAME];          // Decoded stream ID (durable key for worker-time lookup)
    char original_url[MAX_URL_LENGTH];         // Original URL before update
    stream_protocol_t original_protocol;       // Original protocol before update
    bool original_record_audio;                // Original record_audio before update
    bool original_backchannel_enabled;         // Original backchannel_enabled before update
    bool original_enabled;                     // Original enabled flag before update
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
    bool has_detection_url;                    // Whether detection_url was provided
    bool has_record;                           // Whether record flag was provided
    bool has_streaming_enabled;                // Whether streaming_enabled flag was provided
    bool non_dynamic_config_changed;           // Whether non-dynamic fields changed
    bool credentials_changed;                  // Whether ONVIF credentials changed
    bool go2rtc_override_changed;              // Whether go2rtc_source_override changed
    bool sub_stream_changed;                   // Whether sub_stream_url changed
    bool publish_changed;                      // Whether publish_url (RTMP restream) changed
} put_stream_task_t;

static void format_stream_capacity_error(char *buf, size_t buf_size,
                                         int runtime_capacity,
                                         int configured_capacity,
                                         int current_total) {
    if (!buf || buf_size == 0) {
        return;
    }

    if (configured_capacity > runtime_capacity) {
        snprintf(buf, buf_size,
                 "Max streams limit reached (%d/%d in the current runtime). "
                 "Settings are already saved for %d streams, but you must restart "
                 "LightNVR before adding more cameras.",
                 current_total, runtime_capacity, configured_capacity);
        return;
    }

    snprintf(buf, buf_size,
             "Max streams limit reached (%d/%d). Increase Settings -> Max Streams "
             "and restart LightNVR before adding more cameras.",
             current_total, runtime_capacity);
}

/**
 * @brief Free a PUT stream task
 */
static void put_stream_task_free(put_stream_task_t *task) {
    if (task) {
        free(task);
    }
}

static void normalize_stream_url_credentials(stream_config_t *config) {
    char extracted_username[sizeof(config->onvif_username)] = {0};
    char extracted_password[sizeof(config->onvif_password)] = {0};
    char stripped_url[MAX_URL_LENGTH];
    bool use_separate_credentials;

    if (!config) {
        return;
    }

    use_separate_credentials = config->is_onvif ||
                              config->onvif_username[0] != '\0' ||
                              config->onvif_password[0] != '\0';
    if (!use_separate_credentials) {
        return;
    }

    if (url_extract_credentials(config->url,
                                extracted_username, sizeof(extracted_username),
                                extracted_password, sizeof(extracted_password)) == 0) {
        if (config->onvif_username[0] == '\0' && extracted_username[0] != '\0') {
            safe_strcpy(config->onvif_username, extracted_username, sizeof(config->onvif_username), 0);
        }
        if (config->onvif_password[0] == '\0' && extracted_password[0] != '\0') {
            safe_strcpy(config->onvif_password, extracted_password, sizeof(config->onvif_password), 0);
        }
    }

    if (url_strip_credentials(config->url, stripped_url, sizeof(stripped_url)) == 0) {
        safe_strcpy(config->url, stripped_url, sizeof(config->url), 0);
    }
}

static int build_onvif_test_url(const stream_config_t *config, char *out_url, size_t out_size) {
    if (!config || !out_url || out_size == 0) {
        return -1;
    }

    return url_build_onvif_device_service_url(config->url, config->onvif_port,
                                              out_url, out_size);
}

static void redact_url_for_log(const char *url, char *out_url, size_t out_size) {
    if (url_redact_for_logging(url, out_url, out_size) != 0) {
        safe_strcpy(out_url, "[invalid-url]", out_size, 0);
    }
}

/**
 * @brief Worker function for PUT stream update
 *
 * This function performs the actual stream update work in a background thread.
 */
static void put_stream_worker_locked(put_stream_task_t *task) {
    log_set_thread_context("StreamAPI", task->stream_id);
    log_info("Processing PUT /api/streams/%s in worker thread", task->stream_id);

    // Resolve the stream handle at worker time from the durable name key; the
    // stream may have been deleted while this task was queued.
    stream_handle_t stream = get_stream_by_name(task->stream_id);
    if (!stream) {
        log_warn("Stream %s no longer exists, dropping queued PUT update", task->stream_id);
        put_stream_task_free(task);
        return;
    }

    if (!stream_task_identity_matches(task->stream_id, task->config.camera_uuid)) {
        log_warn("Camera identity changed; dropping queued PUT for %s", task->stream_id);
        put_stream_task_free(task);
        return;
    }

    // Note: update_stream_config() never writes privacy_mode (owned by the
    // set_privacy_mode toggle), so a toggle made while this task was queued
    // is preserved.

    // Update stream configuration in database first
    if (update_stream_config(task->stream_id, &task->config) != 0) {
        log_error("Failed to update stream configuration in database for %s", task->stream_id);
        put_stream_task_free(task);
        return;
    }

    // Force update of stream configuration in memory to ensure it matches the database
    stream_config_t updated_config;
    if (get_stream_config(stream, &updated_config) != 0) {
        log_error("Failed to refresh stream configuration from database for stream %s", task->config.name);
        put_stream_task_free(task);
        return;
    }

    if (set_stream_detection_params(stream,
                                   task->config.detection_interval,
                                   task->config.detection_threshold,
                                   task->config.pre_detection_buffer,
                                   task->config.post_detection_buffer) != 0) {
        log_warn("Failed to update detection parameters for stream %s", task->config.name);
    }

    if (set_stream_detection_recording(stream,
                                      task->config.detection_based_recording,
                                      task->config.detection_model) != 0) {
        log_warn("Failed to update detection recording for stream %s", task->config.name);
    }

    // If detection settings were changed and the stream is running,
    // we need to restart the stream to apply the new detection settings
    if (task->config_changed &&
        (task->has_detection_based_recording || task->has_detection_model ||
         task->has_detection_threshold || task->has_detection_interval || task->has_detection_url) &&
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
        if (set_stream_recording(stream, task->config.record) != 0) {
            log_warn("Failed to update recording setting for stream %s", task->config.name);
        }
    }

    if (task->has_streaming_enabled) {
        if (set_stream_streaming_enabled(stream, task->config.streaming_enabled) != 0) {
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
        if (task->config.enabled &&
            is_detection_recording_scheduled(&task->config)) {
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
            log_info("Detection enabled for stream %s but inactive due to stream state or schedule",
                     task->config.name);
        }
    }
    // If detection settings changed but detection was already enabled, restart the thread
    else if (detection_now_enabled && (task->has_detection_model || task->has_detection_threshold || task->has_detection_interval || task->has_detection_url)) {
        log_info("Detection settings changed for stream %s, restarting unified detection thread", task->config.name);

        if (stop_unified_detection_thread(task->config.name) != 0) {
            log_warn("Failed to stop existing unified detection thread for stream %s", task->config.name);
        }

        if (task->config.enabled &&
            is_detection_recording_scheduled(&task->config)) {
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
    if (get_stream_config(stream, &updated_config) == 0) {
        log_info("Detection settings after update - Model: %s, Threshold: %.2f, Interval: %d, Pre-buffer: %d, Post-buffer: %d",
                 updated_config.detection_model, updated_config.detection_threshold, updated_config.detection_interval,
                 updated_config.pre_detection_buffer, updated_config.post_detection_buffer);
    }

    // Restart stream if configuration changed and either:
    // 1. Critical parameters requiring restart were changed (URL, protocol, record_audio)
    // 2. The stream is currently running AND non-dynamic parameters changed
    // 3. The stream was just enabled (it is not running, so (2) can't start it)
    // Note: record and streaming_enabled can be toggled dynamically without restart
    // Note: retention and PTZ settings are metadata only and don't require restart

    bool enabled_changed_to_true = (!task->original_enabled && task->config.enabled);

    if (task->config_changed && (enabled_changed_to_true || task->requires_restart || (task->is_running && task->non_dynamic_config_changed))) {
        log_info("Restarting stream %s (enabled_now=%s, requires_restart=%s, is_running=%s, non_dynamic_changed=%s)",
                task->config.name,
                enabled_changed_to_true ? "true" : "false",
                task->requires_restart ? "true" : "false",
                task->is_running ? "true" : "false",
                task->non_dynamic_config_changed ? "true" : "false");

        bool url_changed = strcmp(task->original_url, task->config.url) != 0;
        bool protocol_changed = task->original_protocol != task->config.protocol;
        bool record_audio_changed = task->original_record_audio != task->config.record_audio;
        bool backchannel_changed = task->original_backchannel_enabled != task->config.backchannel_enabled;

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

            if (stop_stream(stream) != 0) {
                log_error("Failed to stop stream: %s", task->stream_id);
            }

            // Wait for stream to stop with increased timeout for critical parameter changes
            int timeout = task->requires_restart ? 50 : 30;
            while (get_stream_status(stream) != STREAM_STATUS_STOPPED && timeout > 0) {
                usleep(100000); // 100ms
                timeout--;
            }

            if (timeout == 0) {
                log_warn("Timeout waiting for stream %s to stop, continuing anyway", task->config.name);
            }
        }

        // If URL, protocol, record_audio, backchannel, or credentials changed, update the dynamic
        // go2rtc API registration. Streams with a source override are YAML-backed;
        // those changes are applied by the go2rtc process restart below instead.
        bool go2rtc_reloaded = false;
        if (!task->go2rtc_override_changed &&
            task->config.go2rtc_source_override[0] == '\0' &&
            (url_changed || protocol_changed || record_audio_changed || backchannel_changed || task->credentials_changed)) {
            go2rtc_reloaded = true;
            log_info("URL, protocol, record_audio, backchannel, or credentials changed for stream %s, updating go2rtc registration", task->config.name);

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

        // If go2rtc_source_override changed, regenerate config and fully re-register.
        // The override is baked into go2rtc.yaml, so the running process must be
        // restarted to load it. Removing/re-adding the stream through the API
        // would either ignore the override or overwrite it with the normal URL.
        if (task->go2rtc_override_changed) {
            log_info("go2rtc source override changed for stream %s, restarting go2rtc to load generated YAML", task->config.name);
            if (go2rtc_integration_restart_process()) {
                log_info("Successfully restarted go2rtc after source override change for stream %s", task->config.name);
            } else {
                log_error("Failed to restart go2rtc after source override change for stream %s", task->config.name);
            }
            usleep(500000);
        }

        // If sub-stream URL changed, register or unregister the {name}_sub stream
        if (task->sub_stream_changed && !task->go2rtc_override_changed) {
            char sub_name[MAX_STREAM_NAME + 8];
            snprintf(sub_name, sizeof(sub_name), "%s_sub", task->config.name);
            // Always remove old sub-stream first
            go2rtc_api_remove_stream(sub_name);
            // Re-register if new sub-stream URL is set
            if (task->config.sub_stream_url[0] != '\0') {
                log_info("Registering updated sub-stream %s with go2rtc", sub_name);
                go2rtc_stream_register_substream(sub_name, task->config.sub_stream_url,
                                       task->config.onvif_username[0] != '\0' ? task->config.onvif_username : NULL,
                                       task->config.onvif_password[0] != '\0' ? task->config.onvif_password : NULL,
                                       task->config.protocol);
            } else {
                log_info("Sub-stream %s removed from go2rtc", sub_name);
            }
            usleep(500000);
        }

        // A stream that was just enabled is not registered with go2rtc yet
        // (disabled streams are skipped at startup), so register it before
        // starting. Override-backed streams were handled by the restart above.
        if (enabled_changed_to_true && !go2rtc_reloaded && !task->go2rtc_override_changed) {
            if (go2rtc_integration_reload_stream(task->config.name)) {
                log_info("Registered newly enabled stream %s with go2rtc", task->config.name);
            } else {
                log_warn("Failed to register newly enabled stream %s with go2rtc (go2rtc may not be ready)",
                         task->config.name);
            }
        }

        // Start stream if enabled (AFTER go2rtc has been updated)
        if (task->config.enabled) {
            log_info("Starting stream %s after configuration update", task->config.name);
            if (start_stream(stream) != 0) {
                log_error("Failed to restart stream: %s", task->stream_id);
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

    // If the RTMP publish (restream) URL changed, regenerate go2rtc.yaml (which
    // carries the publish: block) and restart go2rtc so it starts/stops the
    // RTMP push. This is independent of the stream restart decision above: a
    // restream edit must apply even when the stream is not running, and it does
    // not need the recording pipeline restarted. Skip if an override change
    // already restarted go2rtc (which regenerated the publish block too).
    if (task->config_changed && task->publish_changed && !task->go2rtc_override_changed) {
        log_info("Publish URL changed for stream %s, restarting go2rtc to apply restream config", task->config.name);
        if (go2rtc_integration_restart_process()) {
            log_info("Successfully restarted go2rtc after publish URL change for stream %s", task->config.name);
        } else {
            log_error("Failed to restart go2rtc after publish URL change for stream %s", task->config.name);
        }
    }

    log_info("Successfully completed stream update for: %s", task->stream_id);

    // Clean up
    put_stream_task_free(task);
}

/**
 * @brief Thread entry for PUT stream update.
 *
 * Applies are serialized so two queued updates never interleave their
 * DB write / stop / go2rtc reload / start sequences.
 */
static void *put_stream_worker(void *arg) {
    put_stream_task_t *task = (put_stream_task_t *)arg;
    if (!task) {
        log_error("Invalid PUT stream task");
        return NULL;
    }

    pthread_mutex_lock(&g_put_stream_update_mutex);
    put_stream_worker_locked(task);  // frees task on every path
    pthread_mutex_unlock(&g_put_stream_update_mutex);
    return NULL;
}

/**
 * @brief Handler function for PUT stream
 *
 * This function is called by the multithreading system.
 */
// put_stream_handler removed - work done via update_stream_task_func

/* Validate a non-blank go2rtc_source_override shared by POST and PUT.
 * Single-line values are URLs and are not YAML-checked; values containing a
 * newline are treated as YAML and validated so malformed lists are rejected
 * with line/column info instead of failing later inside go2rtc.
 * On failure the error response is set on `res` and false is returned; the
 * caller still owns (and must free) the request JSON that `src` points into. */
static bool validate_go2rtc_source_override(const char *src, size_t max_size,
                                            http_response_t *res) {
    size_t src_len = strlen(src);

    if (src_len >= max_size) {
        log_error("go2rtc_source_override too long (%zu bytes, max %zu)",
                  src_len, max_size - 1);
        http_response_set_json_error(res, 413,
            "go2rtc_source_override exceeds size limit");
        return false;
    }

    if (strchr(src, '\n') == NULL) {
        return true;
    }

    char err[256] = {0};
    if (yaml_validate_str(src, src_len, err, sizeof(err)) >= 0) {
        return true;
    }

    log_warn("Rejecting go2rtc_source_override: %s", err);
    /* Structured 400 carrying the validator's line/column message. */
    cJSON *response = cJSON_CreateObject();
    if (response) {
        cJSON_AddBoolToObject(response, "valid", false);
        cJSON *errobj = cJSON_CreateObject();
        if (errobj) {
            cJSON_AddStringToObject(errobj, "message", err);
            cJSON_AddItemToObject(response, "error", errobj);
        }
        char *body = cJSON_PrintUnformatted(response);
        cJSON_Delete(response);
        if (body) {
            http_response_set_json(res, 400, body);
            free(body);
            return false;
        }
    }
    http_response_set_json_error(res, 400,
        "go2rtc_source_override is not valid YAML");
    return false;
}

/* detection_url is user-supplied and passed to avformat_open_input in the
 * detection stream thread. Restrict the scheme to network protocols so
 * file://, concat:, subfile:, etc. can't be used to coerce server-side reads.
 * An empty string is allowed (means "fall back to main stream"). */
static bool is_allowed_detection_url(const char *u) {
    if (!u || u[0] == '\0') return true;
    static const char *const prefixes[] = {
        "http://", "https://", "rtsp://", "rtsps://", NULL
    };
    for (size_t i = 0; prefixes[i] != NULL; i++) {
        if (strncasecmp(u, prefixes[i], strlen(prefixes[i])) == 0) return true;
    }
    return false;
}

/* publish_url is a list of restream (publish) destinations handed to go2rtc's
 * RTMP publisher. Every destination must be rtmp/rtmps so it can't be abused
 * to write to arbitrary local URLs, and must be free of characters that could
 * break out of go2rtc.yaml. An empty string means "no restreaming". */
static bool is_allowed_publish_url(const char *u) {
    return publish_url_list_is_valid(u);
}

static bool eptz_number_in_range(cJSON *root, const char *name,
                                 double minimum, double maximum) {
    cJSON *value = cJSON_GetObjectItemCaseSensitive(root, name);
    return cJSON_IsNumber(value) && value->valuedouble >= minimum &&
           value->valuedouble <= maximum;
}

/* Validate the version-1 browser dewarping document before it reaches the DB.
 * Empty is the canonical disabled value. Keeping this document opaque to the
 * recorder lets later projection versions evolve without touching media code. */
static bool copy_valid_eptz_config(cJSON *value, char *destination,
                                   size_t destination_size,
                                   char *error, size_t error_size) {
    if (!value || !destination || destination_size == 0) return false;
    if (!cJSON_IsString(value) || !value->valuestring) {
        snprintf(error, error_size, "eptz_config must be a JSON string");
        return false;
    }

    const char *text = value->valuestring;
    if (text[0] == '\0') {
        destination[0] = '\0';
        return true;
    }
    if (strlen(text) >= destination_size) {
        snprintf(error, error_size, "eptz_config exceeds size limit");
        return false;
    }

    cJSON *root = cJSON_Parse(text);
    cJSON *version = root
        ? cJSON_GetObjectItemCaseSensitive(root, "version") : NULL;
    cJSON *projection = root
        ? cJSON_GetObjectItemCaseSensitive(root, "projection") : NULL;
    cJSON *mount = root
        ? cJSON_GetObjectItemCaseSensitive(root, "mount") : NULL;
    bool valid = cJSON_IsObject(root) && cJSON_IsNumber(version) &&
        version->valuedouble == 1.0 && cJSON_IsString(projection) &&
        strcmp(projection->valuestring, "equidistant") == 0 &&
        cJSON_IsString(mount) && strcmp(mount->valuestring, "ceiling") == 0 &&
        eptz_number_in_range(root, "centerX", 0.0, 1.0) &&
        eptz_number_in_range(root, "centerY", 0.0, 1.0) &&
        eptz_number_in_range(root, "radius", 0.05, 1.0) &&
        eptz_number_in_range(root, "fov", 160.0, 360.0) &&
        eptz_number_in_range(root, "rotation", -360.0, 360.0) &&
        eptz_number_in_range(root, "defaultYaw", -360.0, 360.0) &&
        eptz_number_in_range(root, "defaultTilt", -90.0, 30.0) &&
        eptz_number_in_range(root, "defaultViewFov", 20.0, 120.0);
    cJSON_Delete(root);

    if (!valid) {
        snprintf(error, error_size,
                 "eptz_config is not a supported version-1 equidistant ceiling configuration");
        return false;
    }

    safe_strcpy(destination, text, destination_size, 0);
    return true;
}

/**
 * @brief Direct handler for POST /api/streams
 */
void handle_post_stream(const http_request_t *req, http_response_t *res) {
    log_info("Handling POST /api/streams request");

    if (!check_stream_write_access(req, res, NULL)) {
        return;
    }

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

    // Validate stream name is not empty or whitespace-only
    const char *name_str = name->valuestring;
    while (*name_str == ' ' || *name_str == '\t') name_str++;
    if (*name_str == '\0') {
        log_error("Stream name is empty or whitespace-only");
        cJSON_Delete(stream_json);
        http_response_set_json_error(res, 400, "Stream name cannot be empty");
        return;
    }

    // Copy trimmed name (skip leading whitespace, then strip trailing whitespace)
    safe_strcpy(config.name, name_str, sizeof(config.name), 0);
    size_t name_len = strlen(config.name);
    while (name_len > 0 && (config.name[name_len - 1] == ' ' || config.name[name_len - 1] == '\t')) {
        config.name[--name_len] = '\0';
    }

    // Reject names with characters that would mismatch between URL-encoded
    // client requests (MSE/HLS/WebRTC) and the decoded key stored by go2rtc
    // and the filesystem sanitizer. See issue #396.
    if (!is_valid_stream_name(config.name)) {
        log_error("Invalid stream name: '%s' (allowed: letters, digits, '.', '-', '_')", config.name);
        cJSON_Delete(stream_json);
        http_response_set_json_error(res, 400,
            "Stream name can only contain letters, digits, '.', '-', and '_'");
        return;
    }

    safe_strcpy(config.url, url->valuestring, sizeof(config.url), 0);

    // Optional fields with defaults
    config.enabled = true;
    config.streaming_enabled = true;
    config.width = 0;   // Auto-detected from stream
    config.height = 0;  // Auto-detected from stream
    config.fps = 0;     // Auto-detected from stream
    config.codec[0] = '\0'; // Auto-detected from stream
    config.priority = 5;
    config.record = true;
    config.segment_duration = 30;
    config.detection_based_recording = false;
    config.detection_interval = 10;
    config.detection_threshold = 0.5f;
    config.pre_detection_buffer = 5;
    config.post_detection_buffer = 5;
    config.protocol = STREAM_PROTOCOL_TCP;
    config.record_audio = true; // Default to true for new streams
    config.retention_days = -1;
    config.detection_retention_days = -1;
    safe_strcpy(config.playback_transport,
                playback_transport_is_valid(g_config.default_playback_transport)
                    ? g_config.default_playback_transport : "auto",
                sizeof(config.playback_transport), 0);
    memset(config.recording_schedule, 1, sizeof(config.recording_schedule));
    memset(config.detection_recording_schedule, 1,
           sizeof(config.detection_recording_schedule));

    // Override with provided values
    cJSON *enabled = cJSON_GetObjectItem(stream_json, "enabled");
    if (enabled && cJSON_IsBool(enabled)) {
        config.enabled = cJSON_IsTrue(enabled);
    }

    cJSON *streaming_enabled = cJSON_GetObjectItem(stream_json, "streaming_enabled");
    if (streaming_enabled && cJSON_IsBool(streaming_enabled)) {
        config.streaming_enabled = cJSON_IsTrue(streaming_enabled);
    }

    cJSON *playback_transport =
        cJSON_GetObjectItem(stream_json, "playback_transport");
    if (playback_transport) {
        if (!cJSON_IsString(playback_transport) ||
            !playback_transport_is_valid(playback_transport->valuestring)) {
            cJSON_Delete(stream_json);
            http_response_set_json_error(res, 400,
                "playback_transport is invalid");
            return;
        }
        safe_strcpy(config.playback_transport, playback_transport->valuestring,
                    sizeof(config.playback_transport), 0);
    }

    cJSON *eptz_config = cJSON_GetObjectItem(stream_json, "eptz_config");
    if (eptz_config) {
        char eptz_error[160];
        if (!copy_valid_eptz_config(eptz_config, config.eptz_config,
                                    sizeof(config.eptz_config), eptz_error,
                                    sizeof(eptz_error))) {
            cJSON_Delete(stream_json);
            http_response_set_json_error(res, 400, eptz_error);
            return;
        }
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
        safe_strcpy(config.detection_model, detection_model->valuestring, sizeof(config.detection_model), 0);
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
        safe_strcpy(config.detection_object_filter, detection_object_filter->valuestring, sizeof(config.detection_object_filter), 0);
    }

    cJSON *detection_object_filter_list = cJSON_GetObjectItem(stream_json, "detection_object_filter_list");
    if (detection_object_filter_list && cJSON_IsString(detection_object_filter_list)) {
        safe_strcpy(config.detection_object_filter_list, detection_object_filter_list->valuestring, sizeof(config.detection_object_filter_list), 0);
    }

    cJSON *protocol = cJSON_GetObjectItem(stream_json, "protocol");
    if (protocol && cJSON_IsNumber(protocol)) {
        config.protocol = (stream_protocol_t)protocol->valueint;
    }

    /* codec is user-settable as a hint (select Auto / H.264 / H.265) so
     * lightNVR can make the right call on the initial go2rtc registration —
     * specifically whether to pre-add the ffmpeg H.264 fallback for WebRTC
     * (#374). The detection thread overwrites this with the real codec once
     * it reads a packet, and re-registers with go2rtc if the user was wrong. */
    cJSON *codec_hint = cJSON_GetObjectItem(stream_json, "codec");
    if (codec_hint && cJSON_IsString(codec_hint)) {
        safe_strcpy(config.codec, codec_hint->valuestring, sizeof(config.codec), 0);
    }

    cJSON *record_audio = cJSON_GetObjectItem(stream_json, "record_audio");
    if (record_audio && cJSON_IsBool(record_audio)) {
        config.record_audio = cJSON_IsTrue(record_audio);
        log_info("Audio recording %s for stream %s",
                config.record_audio ? "enabled" : "disabled", config.name);
    }

    // Opt-in voice-enhancement filter for recordings (discussion #395).
    // Defaults to false in stream_config_t zero-init; only flipped on if
    // the client explicitly requests it.
    cJSON *audio_voice_enhancement = cJSON_GetObjectItem(stream_json, "audio_voice_enhancement");
    if (audio_voice_enhancement && cJSON_IsBool(audio_voice_enhancement)) {
        config.audio_voice_enhancement = cJSON_IsTrue(audio_voice_enhancement);
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
        if (retention_days->valueint < -1 || retention_days->valueint > 365) {
            cJSON_Delete(stream_json);
            http_response_set_json_error(res, 400, "retention_days must be between -1 and 365");
            return;
        }
        config.retention_days = retention_days->valueint;
    }

    cJSON *detection_retention_days = cJSON_GetObjectItem(stream_json, "detection_retention_days");
    if (detection_retention_days && cJSON_IsNumber(detection_retention_days)) {
        if (detection_retention_days->valueint < -1 ||
            detection_retention_days->valueint > 365) {
            cJSON_Delete(stream_json);
            http_response_set_json_error(res, 400,
                "detection_retention_days must be between -1 and 365");
            return;
        }
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
    }

    cJSON *detection_record_on_schedule =
        cJSON_GetObjectItem(stream_json, "detection_record_on_schedule");
    if (detection_record_on_schedule && cJSON_IsBool(detection_record_on_schedule)) {
        config.detection_record_on_schedule =
            cJSON_IsTrue(detection_record_on_schedule);
    }
    cJSON *detection_recording_schedule =
        cJSON_GetObjectItem(stream_json, "detection_recording_schedule");
    if (detection_recording_schedule && cJSON_IsArray(detection_recording_schedule) &&
        cJSON_GetArraySize(detection_recording_schedule) == 168) {
        for (int j = 0; j < 168; j++) {
            cJSON *item = cJSON_GetArrayItem(detection_recording_schedule, j);
            config.detection_recording_schedule[j] =
                (item && cJSON_IsTrue(item)) ? 1 : 0;
        }
    }

    // Parse tags setting (comma-separated list, e.g. "outdoor,critical,entrance")
    cJSON *tags_post = cJSON_GetObjectItem(stream_json, "tags");
    if (tags_post && cJSON_IsString(tags_post)) {
        safe_strcpy(config.tags, tags_post->valuestring, sizeof(config.tags), 0);
    } else {
        config.tags[0] = '\0';
    }

    cJSON *admin_url_post = cJSON_GetObjectItem(stream_json, "admin_url");
    if (admin_url_post && cJSON_IsString(admin_url_post)) {
        safe_strcpy(config.admin_url, admin_url_post->valuestring, sizeof(config.admin_url), 0);
    } else {
        config.admin_url[0] = '\0';
    }

    // Parse cross-stream motion trigger source
    cJSON *motion_trigger_source_post = cJSON_GetObjectItem(stream_json, "motion_trigger_source");
    if (motion_trigger_source_post && cJSON_IsString(motion_trigger_source_post)) {
        safe_strcpy(config.motion_trigger_source, motion_trigger_source_post->valuestring,
                sizeof(config.motion_trigger_source), 0);
    } else {
        config.motion_trigger_source[0] = '\0';
    }

    // Parse go2rtc source override (T11)
    //
    // Accept either a single line (URL) or multi-line YAML.  When the value
    // contains a newline we treat it as YAML and run the syntax validator
    // — this rejects malformed lists with HTTP 400 carrying line/column
    // info instead of failing later in go2rtc with an opaque error.
    // Single-line URLs skip validation; they're not YAML by intent.
    cJSON *go2rtc_source_override_post = cJSON_GetObjectItem(stream_json, "go2rtc_source_override");
    if (go2rtc_source_override_post && cJSON_IsString(go2rtc_source_override_post)) {
        const char *src = go2rtc_source_override_post->valuestring;

        // Code editors commonly retain a final newline after their visible
        // content is deleted. Whitespace-only input means "clear override";
        // preserving it would generate a stream key with no producers.
        if (!string_has_non_whitespace(src)) {
            config.go2rtc_source_override[0] = '\0';
        } else {
            if (!validate_go2rtc_source_override(
                    src, sizeof(config.go2rtc_source_override), res)) {
                cJSON_Delete(stream_json);
                return;
            }

            safe_strcpy(config.go2rtc_source_override, src,
                    sizeof(config.go2rtc_source_override), 0);
        }
    } else {
        config.go2rtc_source_override[0] = '\0';
    }

    // Parse sub-stream URL
    cJSON *sub_stream_url_post = cJSON_GetObjectItem(stream_json, "sub_stream_url");
    if (sub_stream_url_post && cJSON_IsString(sub_stream_url_post)) {
        safe_strcpy(config.sub_stream_url, sub_stream_url_post->valuestring,
                sizeof(config.sub_stream_url), 0);
    } else {
        config.sub_stream_url[0] = '\0';
    }

    // Parse secondary detection stream URL
    cJSON *detection_url_post = cJSON_GetObjectItem(stream_json, "detection_url");
    if (detection_url_post && cJSON_IsString(detection_url_post)) {
        if (!is_allowed_detection_url(detection_url_post->valuestring)) {
            log_error("Rejected detection_url with disallowed scheme for stream %s",
                      config.name);
            cJSON_Delete(stream_json);
            http_response_set_json_error(res, 400,
                "detection_url must use http, https, rtsp, or rtsps");
            return;
        }
        if (strlen(detection_url_post->valuestring) >= sizeof(config.detection_url)) {
            log_error("detection_url too long (%zu bytes, max %zu) for stream %s",
                      strlen(detection_url_post->valuestring),
                      sizeof(config.detection_url) - 1, config.name);
            cJSON_Delete(stream_json);
            http_response_set_json_error(res, 413, "detection_url exceeds size limit");
            return;
        }
        safe_strcpy(config.detection_url, detection_url_post->valuestring,
                sizeof(config.detection_url), 0);
    } else {
        config.detection_url[0] = '\0';
    }

    // RTMP/RTMPS publish (restream) destination, e.g. YouTube Live ingest URL.
    cJSON *publish_url_post = cJSON_GetObjectItem(stream_json, "publish_url");
    if (publish_url_post && cJSON_IsString(publish_url_post)) {
        const char *pub = publish_url_post->valuestring;
        if (pub[0] != '\0' && !is_allowed_publish_url(pub)) {
            log_error("Rejected invalid publish_url for stream %s", config.name);
            cJSON_Delete(stream_json);
            http_response_set_json_error(res, 400, "publish_url destinations must be rtmp:// or rtmps:// URLs");
            return;
        }
        if (strlen(pub) >= sizeof(config.publish_url)) {
            log_error("publish_url too long (%zu bytes, max %zu) for stream %s",
                      strlen(pub), sizeof(config.publish_url) - 1, config.name);
            cJSON_Delete(stream_json);
            http_response_set_json_error(res, 413, "publish_url exceeds size limit");
            return;
        }
        safe_strcpy(config.publish_url, pub, sizeof(config.publish_url), 0);
    } else {
        config.publish_url[0] = '\0';
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

    // Parse ONVIF port if provided
    cJSON *onvif_port_json = cJSON_GetObjectItem(stream_json, "onvif_port");
    if (onvif_port_json && cJSON_IsNumber(onvif_port_json)) {
        config.onvif_port = onvif_port_json->valueint;
    }

    // If ONVIF flag is set, test the connection
    bool onvif_test_success = true;
    bool onvif_test_performed = false;
    if (config.is_onvif) {
        onvif_test_performed = true;
        log_info("Testing ONVIF capabilities for stream %s", config.name);

        // Extract username and password if provided
        cJSON *onvif_username = cJSON_GetObjectItem(stream_json, "onvif_username");
        cJSON *onvif_password = cJSON_GetObjectItem(stream_json, "onvif_password");
        cJSON *onvif_profile = cJSON_GetObjectItem(stream_json, "onvif_profile");

        if (onvif_username && cJSON_IsString(onvif_username)) {
            safe_strcpy(config.onvif_username, onvif_username->valuestring, sizeof(config.onvif_username), 0);
        }

        if (onvif_password && cJSON_IsString(onvif_password)) {
            safe_strcpy(config.onvif_password, onvif_password->valuestring, sizeof(config.onvif_password), 0);
        }

        if (onvif_profile && cJSON_IsString(onvif_profile)) {
            safe_strcpy(config.onvif_profile, onvif_profile->valuestring, sizeof(config.onvif_profile), 0);
        }

        normalize_stream_url_credentials(&config);

        // Build ONVIF device URL, using onvif_port if specified
        char onvif_device_url[MAX_URL_LENGTH];
        if (build_onvif_test_url(&config, onvif_device_url, sizeof(onvif_device_url)) != 0) {
            safe_strcpy(onvif_device_url, config.url, sizeof(onvif_device_url), 0);
        }

        // Test ONVIF connection
        int result = test_onvif_connection(onvif_device_url,
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

    int runtime_capacity = get_stream_capacity();
    int current_total = get_total_stream_count();
    if (runtime_capacity > 0 && current_total >= runtime_capacity) {
        char error_message[256];
        format_stream_capacity_error(error_message, sizeof(error_message),
                                     runtime_capacity, g_config.max_streams, current_total);
        log_warn("Rejecting stream '%s': %s", config.name, error_message);
        http_response_set_json_error(res, 409, error_message);
        return;
    }

    // Add stream to database
    uint64_t stream_id = add_stream_config(&config);
    if (stream_id == 0) {
        log_error("Failed to add stream configuration to database");
        http_response_set_json_error(res, 500, "Failed to add stream configuration");
        return;
    }

    /* camera_uuid is generated by SQLite so callers cannot choose or reuse an
     * identity accidentally. Load it back before placing the configuration in
     * the runtime stream manager. */
    stream_config_t *persisted_config = calloc(1, sizeof(*persisted_config));
    if (!persisted_config ||
        get_stream_config_by_name(config.name, persisted_config) != 0) {
        log_error("Failed to load generated camera UUID for stream: %s", config.name);
        free(persisted_config);
        delete_stream_config_internal(config.name, true);
        http_response_set_json_error(res, 500,
                                     "Failed to load created stream identity");
        return;
    }
    safe_strcpy(config.camera_uuid, persisted_config->camera_uuid,
                sizeof(config.camera_uuid), 0);
    safe_strcpy(config.location_uuid, persisted_config->location_uuid,
                sizeof(config.location_uuid), 0);
    free(persisted_config);

    // Create stream in memory from the database configuration
    // This also registers the stream with go2rtc via go2rtc_integration_register_stream()
    stream_handle_t stream = add_stream(&config);
    if (!stream) {
        log_error("Failed to create stream: %s", config.name);
        // Delete from database since we couldn't create it in memory
        delete_stream_config(config.name);

        runtime_capacity = get_stream_capacity();
        current_total = get_total_stream_count();
        if (runtime_capacity > 0 && current_total >= runtime_capacity) {
            char error_message[256];
            format_stream_capacity_error(error_message, sizeof(error_message),
                                         runtime_capacity, g_config.max_streams, current_total);
            http_response_set_json_error(res, 409, error_message);
            return;
        }

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

        // Start detection when enabled and currently permitted by its schedule.
        if (config.detection_based_recording &&
            is_detection_recording_scheduled(&config)) {
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
    cJSON_AddStringToObject(success, "camera_uuid", config.camera_uuid);
    cJSON_AddStringToObject(success, "location_uuid", config.location_uuid);

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

    log_info("Handling PUT /api/streams/%s request", stream_id);

    // Find the stream by name
    stream_handle_t stream = get_stream_by_name(stream_id);
    if (!stream) {
        log_error("Stream not found: %s", stream_id);
        http_response_set_json_error(res, 404, "Stream not found");
        return;
    }

    // Get current stream configuration
    stream_config_t config;
    if (get_stream_config(stream, &config) != 0) {
        log_error("Failed to get stream configuration for: %s", stream_id);
        http_response_set_json_error(res, 500, "Failed to get stream configuration");
        return;
    }
    if (!check_stream_write_access(req, res, &config)) {
        return;
    }

    // Parse JSON from request body
    cJSON *stream_json = httpd_parse_json_body(req);
    if (!stream_json) {
        log_error("Failed to parse stream JSON from request body");
        http_response_set_json_error(res, 400, "Invalid JSON in request body");
        return;
    }

    // Check if there's a request to set/clear privacy mode (lightweight toggle, no full restart).
    // Handled before any other field so a rejected or privacy-only request
    // never triggers ONVIF probes or other per-field work.
    cJSON *privacy_request = cJSON_GetObjectItem(stream_json, "set_privacy_mode");
    if (privacy_request && cJSON_IsBool(privacy_request)) {
        // Reject mixed updates to avoid silently discarding ordinary config
        // fields. Identifying metadata ("name") is tolerated and ignored.
        cJSON *field = NULL;
        cJSON_ArrayForEach(field, stream_json) {
            if (!field->string ||
                (strcmp(field->string, "set_privacy_mode") != 0 &&
                 strcmp(field->string, "name") != 0)) {
                cJSON_Delete(stream_json);
                http_response_set_json_error(
                    res, 400,
                    "set_privacy_mode cannot be combined with other update fields");
                return;
            }
        }

        bool enable_privacy = cJSON_IsTrue(privacy_request);
        log_info("Privacy mode %s requested for stream %s", enable_privacy ? "enable" : "disable", stream_id);

        if (set_stream_privacy_mode(stream_id, enable_privacy) != 0) {
            cJSON_Delete(stream_json);
            http_response_set_json_error(res, 500, "Failed to update privacy mode");
            return;
        }
        log_info("Successfully set privacy_mode=%d for stream %s", enable_privacy ? 1 : 0, stream_id);

        char sub_name[MAX_STREAM_NAME + 8];
        snprintf(sub_name, sizeof(sub_name), "%s_sub", stream_id);
        bool has_sub_stream = config.sub_stream_url[0] != '\0';
        // Parts of this stream that live in the generated go2rtc.yaml (a source
        // override, or an RTMP publish/restream entry) only change when go2rtc
        // restarts and regenerates its config from the saved privacy flag.
        bool yaml_backed =
            (!g_config.audio_disabled && config.go2rtc_source_override[0] != '\0') ||
            config.publish_url[0] != '\0';
        // Whether the stream is (enable) no longer / (disable) again served by go2rtc
        bool go2rtc_applied = true;

        if (enable_privacy) {
            // Unregister the main stream and its sub-stream so clients cannot
            // connect through either. If go2rtc is not running nothing is being
            // served, and config generation, startup sync and the health monitor
            // all skip privacy-mode streams, so only a failure against a live
            // go2rtc is reported.
            if (go2rtc_stream_is_ready()) {
                if (!go2rtc_integration_unregister_stream(stream_id)) {
                    go2rtc_applied = false;
                }
                if (has_sub_stream && !go2rtc_api_remove_stream(sub_name)) {
                    go2rtc_applied = false;
                }
                // Drop the YAML source override / RTMP publish entry, which an
                // API delete does not stop.
                if (yaml_backed && !go2rtc_integration_restart_process()) {
                    go2rtc_applied = false;
                }
            }
            if (go2rtc_applied) {
                log_info("Stream %s is no longer served by go2rtc (privacy mode)", stream_id);
            } else {
                log_error("Privacy mode saved for stream %s but go2rtc still serves it", stream_id);
            }
        } else if (!config.enabled) {
            // A disabled stream must stay unregistered; clearing privacy only
            // updates the saved flag. Enabling the stream registers it later.
            log_info("Privacy mode cleared for disabled stream %s; not registering with go2rtc", stream_id);
        } else if (yaml_backed) {
            // Regenerate go2rtc.yaml (override / publish entries) and restart;
            // the restart re-registers every enabled non-privacy stream,
            // including this one and its sub-stream.
            if (go2rtc_integration_restart_process()) {
                log_info("Restarted go2rtc to restore stream %s after privacy mode cleared", stream_id);
            } else {
                go2rtc_applied = false;
                log_warn("Failed to restart go2rtc to restore stream %s", stream_id);
            }
        } else {
            // Resume: re-register with go2rtc. If this fails the health monitor
            // re-adds missing enabled streams, so report it without failing.
            if (go2rtc_integration_reload_stream(stream_id)) {
                log_info("Re-registered stream %s with go2rtc after privacy mode cleared", stream_id);
            } else {
                go2rtc_applied = false;
                log_warn("Failed to re-register stream %s with go2rtc", stream_id);
            }
            if (has_sub_stream &&
                !go2rtc_stream_register_substream(
                    sub_name, config.sub_stream_url,
                    config.onvif_username[0] != '\0' ? config.onvif_username : NULL,
                    config.onvif_password[0] != '\0' ? config.onvif_password : NULL,
                    config.protocol)) {
                go2rtc_applied = false;
                log_warn("Failed to re-register sub-stream %s with go2rtc", sub_name);
            }
        }

        cJSON_Delete(stream_json);

        if (enable_privacy && !go2rtc_applied) {
            // The saved flag is on (and keeps go2rtc from re-adding the stream),
            // but the camera is still viewable right now. Do not report success;
            // retrying the request repeats the unregister.
            http_response_set_json_error(
                res, 502,
                "Privacy mode was saved but the stream could not be removed from go2rtc; retry");
            return;
        }

        cJSON *response = cJSON_CreateObject();
        if (response) {
            cJSON_AddBoolToObject(response, "success", true);
            cJSON_AddBoolToObject(response, "privacy_mode", enable_privacy);
            cJSON_AddBoolToObject(response, "stream_registered",
                                  !enable_privacy && config.enabled && go2rtc_applied);
            char *response_str = cJSON_PrintUnformatted(response);
            cJSON_Delete(response);
            if (response_str) {
                http_response_set_json(res, 200, response_str);
                free(response_str);
                return;
            }
        }
        http_response_set_json_error(res, 500, "Failed to build response");
        return;
    }

    // Update configuration with provided values
    bool config_changed = false;
    bool requires_restart = false;  // Flag for changes that require stream restart
    bool go2rtc_override_changed = false;  // Track go2rtc source override changes
    bool sub_stream_changed = false;  // Track sub-stream URL changes
    bool has_record = false;  // Track if record flag was provided
    bool has_streaming_enabled = false;  // Track if streaming_enabled flag was provided
    bool non_dynamic_config_changed = false;  // Track if non-dynamic fields changed
    bool credentials_changed = false;  // Track if ONVIF credentials changed

    // Save original values for comparison
    char original_url[MAX_URL_LENGTH];
    safe_strcpy(original_url, config.url, MAX_URL_LENGTH, 0);

    stream_protocol_t original_protocol = config.protocol;
    bool original_record_audio = config.record_audio;
    bool original_backchannel_enabled = config.backchannel_enabled;

    cJSON *url = cJSON_GetObjectItem(stream_json, "url");
    if (url && cJSON_IsString(url)) {
        if (strcmp(config.url, url->valuestring) != 0) {
            safe_strcpy(config.url, url->valuestring, sizeof(config.url), 0);
            config_changed = true;
            requires_restart = true;  // URL changes always require restart
            char safe_original_url[MAX_URL_LENGTH];
            char safe_new_url[MAX_URL_LENGTH];
            redact_url_for_log(original_url, safe_original_url, sizeof(safe_original_url));
            redact_url_for_log(config.url, safe_new_url, sizeof(safe_new_url));
            log_info("URL changed from '%s' to '%s' - restart required", safe_original_url, safe_new_url);
        }
    }

    bool original_enabled = config.enabled;
    cJSON *enabled = cJSON_GetObjectItem(stream_json, "enabled");
    if (enabled && cJSON_IsBool(enabled)) {
        bool new_enabled = cJSON_IsTrue(enabled);
        if (config.enabled != new_enabled) {
            config.enabled = new_enabled;
            config_changed = true;
            non_dynamic_config_changed = true;
        }
    }

    cJSON *streaming_enabled = cJSON_GetObjectItem(stream_json, "streaming_enabled");
    if (streaming_enabled && cJSON_IsBool(streaming_enabled)) {
        config.streaming_enabled = cJSON_IsTrue(streaming_enabled);
        config_changed = true;
        has_streaming_enabled = true;
        // streaming_enabled can be toggled dynamically, don't set non_dynamic_config_changed
    }

    cJSON *playback_transport =
        cJSON_GetObjectItem(stream_json, "playback_transport");
    if (playback_transport) {
        if (!cJSON_IsString(playback_transport) ||
            !playback_transport_is_valid(playback_transport->valuestring)) {
            cJSON_Delete(stream_json);
            http_response_set_json_error(res, 400,
                "playback_transport is invalid");
            return;
        }
        if (strcmp(config.playback_transport,
                   playback_transport->valuestring) != 0) {
            safe_strcpy(config.playback_transport,
                        playback_transport->valuestring,
                        sizeof(config.playback_transport), 0);
            config_changed = true;
        }
    }

    cJSON *eptz_config = cJSON_GetObjectItem(stream_json, "eptz_config");
    if (eptz_config) {
        char parsed_eptz_config[EPTZ_CONFIG_MAX];
        char eptz_error[160];
        if (!copy_valid_eptz_config(eptz_config, parsed_eptz_config,
                                    sizeof(parsed_eptz_config), eptz_error,
                                    sizeof(eptz_error))) {
            cJSON_Delete(stream_json);
            http_response_set_json_error(res, 400, eptz_error);
            return;
        }
        if (strcmp(config.eptz_config, parsed_eptz_config) != 0) {
            safe_strcpy(config.eptz_config, parsed_eptz_config,
                        sizeof(config.eptz_config), 0);
            config_changed = true;
        }
    }

    // Note: width, height, fps, and codec are auto-detected from the stream
    // and are not accepted from API input on update either.

    cJSON *priority = cJSON_GetObjectItem(stream_json, "priority");
    if (priority && cJSON_IsNumber(priority)) {
        if (config.priority != priority->valueint) {
            config.priority = priority->valueint;
            config_changed = true;
            non_dynamic_config_changed = true;
        }
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
    bool has_detection_model = false;
    if (detection_model_json && cJSON_IsString(detection_model_json)) {
        char detection_model_value[256];
        safe_strcpy(detection_model_value, detection_model_json->valuestring, sizeof(detection_model_value), 0);
        has_detection_model = true;
        safe_strcpy(config.detection_model, detection_model_value, sizeof(config.detection_model), 0);
        config_changed = true;
        non_dynamic_config_changed = true;
    }

    cJSON *detection_threshold_json = cJSON_GetObjectItem(stream_json, "detection_threshold");
    bool has_detection_threshold = false;
    if (detection_threshold_json && cJSON_IsNumber(detection_threshold_json)) {
        // Convert from percentage (0-100) to float (0.0-1.0)
        has_detection_threshold = true;
        config.detection_threshold = (float)(detection_threshold_json->valuedouble / 100.0);
        config_changed = true;
        non_dynamic_config_changed = true;
    }

    cJSON *detection_interval_json = cJSON_GetObjectItem(stream_json, "detection_interval");
    bool has_detection_interval = false;
    if (detection_interval_json && cJSON_IsNumber(detection_interval_json)) {
        has_detection_interval = true;
        config.detection_interval = detection_interval_json->valueint;
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
        safe_strcpy(config.detection_object_filter, detection_object_filter->valuestring, sizeof(config.detection_object_filter), 0);
        config_changed = true;
    }

    cJSON *detection_object_filter_list = cJSON_GetObjectItem(stream_json, "detection_object_filter_list");
    if (detection_object_filter_list && cJSON_IsString(detection_object_filter_list)) {
        safe_strcpy(config.detection_object_filter_list, detection_object_filter_list->valuestring, sizeof(config.detection_object_filter_list), 0);
        config_changed = true;
    }

    cJSON *record_audio = cJSON_GetObjectItem(stream_json, "record_audio");
    if (record_audio && cJSON_IsBool(record_audio)) {
        bool prev_record_audio = config.record_audio;
        config.record_audio = cJSON_IsTrue(record_audio);
        if (prev_record_audio != config.record_audio) {
            config_changed = true;
            non_dynamic_config_changed = true;
            requires_restart = true;  // Audio recording changes require restart
            log_info("Audio recording changed from %s to %s - restart required",
                    prev_record_audio ? "enabled" : "disabled",
                    config.record_audio ? "enabled" : "disabled");
        }
    }

    // Voice-enhancement is a runtime toggle — the recording thread picks it up
    // at the next segment and the transcoder builds or drains its filter graph
    // on the next packet, so flipping it doesn't require a stream restart.
    // (Discussion #395.)
    cJSON *audio_voice_enhancement = cJSON_GetObjectItem(stream_json, "audio_voice_enhancement");
    if (audio_voice_enhancement && cJSON_IsBool(audio_voice_enhancement)) {
        bool prev_avoe = config.audio_voice_enhancement;
        config.audio_voice_enhancement = cJSON_IsTrue(audio_voice_enhancement);
        if (prev_avoe != config.audio_voice_enhancement) {
            config_changed = true;
            log_info("Audio voice enhancement %s for stream %s",
                    config.audio_voice_enhancement ? "enabled" : "disabled", config.name);
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
        if (new_retention < -1 || new_retention > 365) {
            cJSON_Delete(stream_json);
            http_response_set_json_error(res, 400,
                "retention_days must be between -1 and 365");
            return;
        }
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
        if (new_detection_retention < -1 || new_detection_retention > 365) {
            cJSON_Delete(stream_json);
            http_response_set_json_error(res, 400,
                "detection_retention_days must be between -1 and 365");
            return;
        }
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

    cJSON *detection_record_on_schedule =
        cJSON_GetObjectItem(stream_json, "detection_record_on_schedule");
    if (detection_record_on_schedule && cJSON_IsBool(detection_record_on_schedule)) {
        bool new_value = cJSON_IsTrue(detection_record_on_schedule);
        if (config.detection_record_on_schedule != new_value) {
            config.detection_record_on_schedule = new_value;
            config_changed = true;
            log_info("Detection recording schedule mode changed to %s for stream %s",
                     new_value ? "enabled" : "disabled", config.name);
        }
    }

    cJSON *detection_recording_schedule =
        cJSON_GetObjectItem(stream_json, "detection_recording_schedule");
    if (detection_recording_schedule &&
        cJSON_IsArray(detection_recording_schedule) &&
        cJSON_GetArraySize(detection_recording_schedule) == 168) {
        bool changed = false;
        for (int j = 0; j < 168; j++) {
            cJSON *item = cJSON_GetArrayItem(detection_recording_schedule, j);
            uint8_t new_value = (item && cJSON_IsTrue(item)) ? 1 : 0;
            if (config.detection_recording_schedule[j] != new_value) {
                config.detection_recording_schedule[j] = new_value;
                changed = true;
            }
        }
        if (changed) {
            config_changed = true;
            log_info("Detection recording schedule updated for stream %s", config.name);
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

    /* codec hint (#374): lets the user declare H.264 vs H.265 so the initial
     * go2rtc registration picks the right source list. If the value changes,
     * require a stream restart so the go2rtc re-registration picks it up
     * immediately rather than waiting for the detection thread's next run. */
    cJSON *codec_put = cJSON_GetObjectItem(stream_json, "codec");
    if (codec_put && cJSON_IsString(codec_put)) {
        char new_codec[sizeof(config.codec)];
        safe_strcpy(new_codec, codec_put->valuestring, sizeof(new_codec), 0);
        if (strcasecmp(config.codec, new_codec) != 0) {
            safe_strcpy(config.codec, new_codec, sizeof(config.codec), 0);
            config_changed = true;
            non_dynamic_config_changed = true;
            requires_restart = true;
            log_info("Codec hint changed to '%s' for stream %s - restart required",
                     config.codec, config.name);
        }
    }

    // Parse tags setting (metadata only, no restart needed)
    cJSON *tags_put = cJSON_GetObjectItem(stream_json, "tags");
    if (tags_put && cJSON_IsString(tags_put)) {
        if (strncmp(config.tags, tags_put->valuestring, sizeof(config.tags) - 1) != 0) {
            safe_strcpy(config.tags, tags_put->valuestring, sizeof(config.tags), 0);
            config_changed = true;
            log_info("Tags changed to '%s' for stream %s", config.tags, config.name);
        }
    } else if (tags_put && cJSON_IsNull(tags_put)) {
        if (config.tags[0] != '\0') {
            config.tags[0] = '\0';
            config_changed = true;
        }
    }

    cJSON *admin_url_put = cJSON_GetObjectItem(stream_json, "admin_url");
    if (admin_url_put && cJSON_IsString(admin_url_put)) {
        if (strncmp(config.admin_url, admin_url_put->valuestring, sizeof(config.admin_url) - 1) != 0) {
            safe_strcpy(config.admin_url, admin_url_put->valuestring, sizeof(config.admin_url), 0);
            config_changed = true;
            log_info("Admin URL changed for stream %s", config.name);
        }
    } else if (admin_url_put && cJSON_IsNull(admin_url_put)) {
        if (config.admin_url[0] != '\0') {
            config.admin_url[0] = '\0';
            config_changed = true;
        }
    }

    // Parse cross-stream motion trigger source
    cJSON *motion_trigger_source_put = cJSON_GetObjectItem(stream_json, "motion_trigger_source");
    if (motion_trigger_source_put && cJSON_IsString(motion_trigger_source_put)) {
        if (strncmp(config.motion_trigger_source, motion_trigger_source_put->valuestring,
                    sizeof(config.motion_trigger_source) - 1) != 0) {
            safe_strcpy(config.motion_trigger_source, motion_trigger_source_put->valuestring,
                    sizeof(config.motion_trigger_source), 0);
            config_changed = true;
            log_info("Motion trigger source changed to '%s' for stream %s",
                     config.motion_trigger_source, config.name);
        }
    } else if (motion_trigger_source_put && cJSON_IsNull(motion_trigger_source_put)) {
        if (config.motion_trigger_source[0] != '\0') {
            config.motion_trigger_source[0] = '\0';
            config_changed = true;
        }
    }

    // Parse go2rtc source override
    cJSON *go2rtc_source_override_put = cJSON_GetObjectItem(stream_json, "go2rtc_source_override");
    if (go2rtc_source_override_put && cJSON_IsString(go2rtc_source_override_put)) {
        const char *src = go2rtc_source_override_put->valuestring;
        if (!string_has_non_whitespace(src)) {
            src = "";
        }

        if (src[0] != '\0' &&
            !validate_go2rtc_source_override(
                src, sizeof(config.go2rtc_source_override), res)) {
            cJSON_Delete(stream_json);
            return;
        }

        if (strncmp(config.go2rtc_source_override, src,
                    sizeof(config.go2rtc_source_override) - 1) != 0) {
            safe_strcpy(config.go2rtc_source_override, src,
                    sizeof(config.go2rtc_source_override), 0);
            config_changed = true;
            requires_restart = true;
            go2rtc_override_changed = true;
            log_info("go2rtc source override changed for stream %s", config.name);
        }
    } else if (go2rtc_source_override_put && cJSON_IsNull(go2rtc_source_override_put)) {
        if (config.go2rtc_source_override[0] != '\0') {
            config.go2rtc_source_override[0] = '\0';
            config_changed = true;
            requires_restart = true;
            go2rtc_override_changed = true;
            log_info("go2rtc source override cleared for stream %s", config.name);
        }
    }

    // Parse sub-stream URL
    cJSON *sub_stream_url_put = cJSON_GetObjectItem(stream_json, "sub_stream_url");
    if (sub_stream_url_put && cJSON_IsString(sub_stream_url_put)) {
        if (strncmp(config.sub_stream_url, sub_stream_url_put->valuestring,
                    sizeof(config.sub_stream_url) - 1) != 0) {
            safe_strcpy(config.sub_stream_url, sub_stream_url_put->valuestring,
                    sizeof(config.sub_stream_url), 0);
            config_changed = true;
            requires_restart = true;
            sub_stream_changed = true;
            log_info("Sub-stream URL changed for stream %s", config.name);
        }
    } else if (sub_stream_url_put && cJSON_IsNull(sub_stream_url_put)) {
        if (config.sub_stream_url[0] != '\0') {
            config.sub_stream_url[0] = '\0';
            config_changed = true;
            requires_restart = true;
            sub_stream_changed = true;
            log_info("Sub-stream URL cleared for stream %s", config.name);
        }
    }

    // Parse secondary detection stream URL
    bool has_detection_url = false;
    cJSON *detection_url_put = cJSON_GetObjectItem(stream_json, "detection_url");
    if (detection_url_put && cJSON_IsString(detection_url_put)) {
        if (!is_allowed_detection_url(detection_url_put->valuestring)) {
            log_error("Rejected detection_url with disallowed scheme for stream %s",
                      config.name);
            cJSON_Delete(stream_json);
            http_response_set_json_error(res, 400,
                "detection_url must use http, https, rtsp, or rtsps");
            return;
        }
        if (strlen(detection_url_put->valuestring) >= sizeof(config.detection_url)) {
            log_error("detection_url too long (%zu bytes, max %zu) for stream %s",
                      strlen(detection_url_put->valuestring),
                      sizeof(config.detection_url) - 1, config.name);
            cJSON_Delete(stream_json);
            http_response_set_json_error(res, 413, "detection_url exceeds size limit");
            return;
        }
        if (strncmp(config.detection_url, detection_url_put->valuestring,
                    sizeof(config.detection_url) - 1) != 0) {
            safe_strcpy(config.detection_url, detection_url_put->valuestring,
                    sizeof(config.detection_url), 0);
            has_detection_url = true;
            config_changed = true;
            non_dynamic_config_changed = true;
            log_info("Detection URL changed for stream %s", config.name);
        }
    } else if (detection_url_put && cJSON_IsNull(detection_url_put)) {
        if (config.detection_url[0] != '\0') {
            config.detection_url[0] = '\0';
            has_detection_url = true;
            config_changed = true;
            non_dynamic_config_changed = true;
            log_info("Detection URL cleared for stream %s", config.name);
        }
    }

    // RTMP/RTMPS publish (restream) destination. Changes require regenerating
    // go2rtc.yaml (which carries the publish: block) and restarting go2rtc.
    bool publish_changed = false;
    cJSON *publish_url_put = cJSON_GetObjectItem(stream_json, "publish_url");
    if (publish_url_put && cJSON_IsString(publish_url_put)) {
        const char *pub = publish_url_put->valuestring;
        if (pub[0] != '\0' && !is_allowed_publish_url(pub)) {
            log_error("Rejected invalid publish_url for stream %s", config.name);
            cJSON_Delete(stream_json);
            http_response_set_json_error(res, 400, "publish_url destinations must be rtmp:// or rtmps:// URLs");
            return;
        }
        if (strlen(pub) >= sizeof(config.publish_url)) {
            log_error("publish_url too long (%zu bytes, max %zu) for stream %s",
                      strlen(pub), sizeof(config.publish_url) - 1, config.name);
            cJSON_Delete(stream_json);
            http_response_set_json_error(res, 413, "publish_url exceeds size limit");
            return;
        }
        if (strncmp(config.publish_url, pub, sizeof(config.publish_url) - 1) != 0) {
            safe_strcpy(config.publish_url, pub, sizeof(config.publish_url), 0);
            publish_changed = true;
            config_changed = true;
            log_info("Publish (restream) URL changed for stream %s", config.name);
        }
    } else if (publish_url_put && cJSON_IsNull(publish_url_put)) {
        if (config.publish_url[0] != '\0') {
            config.publish_url[0] = '\0';
            publish_changed = true;
            config_changed = true;
            log_info("Publish (restream) URL cleared for stream %s", config.name);
        }
    }

    // Update is_onvif flag based on request or URL
    bool original_is_onvif = config.is_onvif;

    // Check if isOnvif flag is explicitly set in the request.
    // For PUT updates, preserve existing persisted value when omitted.
    cJSON *is_onvif = cJSON_GetObjectItem(stream_json, "isOnvif");
    if (is_onvif && cJSON_IsBool(is_onvif)) {
        config.is_onvif = cJSON_IsTrue(is_onvif);
    }

    if (original_is_onvif != config.is_onvif) {
        log_info("ONVIF flag changed from %s to %s",
                original_is_onvif ? "true" : "false",
                config.is_onvif ? "true" : "false");
        config_changed = true;
        non_dynamic_config_changed = true;
    }

    // Parse ONVIF port if provided
    cJSON *onvif_port_json = cJSON_GetObjectItem(stream_json, "onvif_port");
    if (onvif_port_json && cJSON_IsNumber(onvif_port_json)) {
        int new_port = onvif_port_json->valueint;
        if (new_port != config.onvif_port) {
            config.onvif_port = new_port;
            config_changed = true;
            non_dynamic_config_changed = true;
        }
    }

    // If ONVIF flag is set, test the connection
    bool onvif_test_success = true;
    bool onvif_test_performed = false;

    // Save original credentials before any update so we can detect changes
    char original_onvif_username[sizeof(config.onvif_username)];
    char original_onvif_password[sizeof(config.onvif_password)];
    char original_onvif_profile[sizeof(config.onvif_profile)];
    safe_strcpy(original_onvif_username, config.onvif_username, sizeof(original_onvif_username), 0);
    safe_strcpy(original_onvif_password, config.onvif_password, sizeof(original_onvif_password), 0);
    safe_strcpy(original_onvif_profile, config.onvif_profile, sizeof(original_onvif_profile), 0);

    if (config.is_onvif) {
        log_info("Testing ONVIF capabilities for stream %s", config.name);
        onvif_test_performed = true;

        // Extract username and password if provided
        cJSON *onvif_username = cJSON_GetObjectItem(stream_json, "onvif_username");
        cJSON *onvif_password = cJSON_GetObjectItem(stream_json, "onvif_password");
        cJSON *onvif_profile = cJSON_GetObjectItem(stream_json, "onvif_profile");

        if (onvif_username && cJSON_IsString(onvif_username)) {
            safe_strcpy(config.onvif_username, onvif_username->valuestring, sizeof(config.onvif_username), 0);
        }

        if (onvif_password && cJSON_IsString(onvif_password)) {
            safe_strcpy(config.onvif_password, onvif_password->valuestring, sizeof(config.onvif_password), 0);
        }

        if (onvif_profile && cJSON_IsString(onvif_profile)) {
            safe_strcpy(config.onvif_profile, onvif_profile->valuestring, sizeof(config.onvif_profile), 0);
        }

        normalize_stream_url_credentials(&config);

        // Detect if credentials actually changed (after normalisation) and mark for restart + go2rtc reload
        if (strcmp(original_onvif_username, config.onvif_username) != 0 ||
            strcmp(original_onvif_password, config.onvif_password) != 0 ||
            strcmp(original_onvif_profile, config.onvif_profile) != 0) {
            credentials_changed = true;
            config_changed = true;
            requires_restart = true;
            non_dynamic_config_changed = true;
            log_info("ONVIF credentials/profile changed for stream %s - restart and go2rtc reload required", config.name);
        }

        // Build ONVIF device URL, using onvif_port if specified
        char onvif_device_url[MAX_URL_LENGTH];
        if (build_onvif_test_url(&config, onvif_device_url, sizeof(onvif_device_url)) != 0) {
            safe_strcpy(onvif_device_url, config.url, sizeof(onvif_device_url), 0);
        }

        // Test ONVIF connection
        int result = test_onvif_connection(onvif_device_url,
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
        // An explicit "enabled": false in the same request contradicts this.
        if (enabled && cJSON_IsBool(enabled) && !cJSON_IsTrue(enabled)) {
            cJSON_Delete(stream_json);
            http_response_set_json_error(res, 400,
                                         "enable_disabled cannot be combined with enabled=false");
            return;
        }

        /*
         * Do not update SQLite directly here: the worker persists the whole
         * config loaded above, so a direct write would be overwritten by the
         * stale enabled=false. Route the change through the normal update path;
         * the worker sees original_enabled=false -> enabled=true, registers the
         * stream with go2rtc, starts it, and starts detection if configured.
         */
        if (!original_enabled) {
            log_info("Enable requested for disabled stream %s", stream_id);
            config.enabled = true;
            config_changed = true;
            non_dynamic_config_changed = true;
        } else {
            log_info("Enable requested for stream %s, which is already enabled", stream_id);
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
    memcpy(&task->config, &config, sizeof(stream_config_t));
    safe_strcpy(task->stream_id, stream_id, MAX_STREAM_NAME, 0);
    safe_strcpy(task->original_url, original_url, MAX_URL_LENGTH, 0);
    task->original_protocol = original_protocol;
    task->original_record_audio = original_record_audio;
    task->original_backchannel_enabled = original_backchannel_enabled;
    task->original_enabled = original_enabled;
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
    task->has_detection_url = has_detection_url;
    task->has_record = has_record;
    task->has_streaming_enabled = has_streaming_enabled;
    task->non_dynamic_config_changed = non_dynamic_config_changed;
    task->credentials_changed = credentials_changed;
    task->go2rtc_override_changed = go2rtc_override_changed;
    task->sub_stream_changed = sub_stream_changed;
    task->publish_changed = publish_changed;

    log_info("Detection settings before update - Model: %s, Threshold: %.2f, Interval: %d, Pre-buffer: %d, Post-buffer: %d",
             config.detection_model, config.detection_threshold, config.detection_interval,
             config.pre_detection_buffer, config.post_detection_buffer);

    // Prepare the accepted response; the backend sends it after this handler returns.
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

    // Spawn background thread to perform the actual update work
    // This prevents blocking the web server event loop
    pthread_t thread_id;
    pthread_attr_t attr;
    int attr_rc = pthread_attr_init(&attr);
    if (attr_rc != 0) {
        log_error("Failed to initialize thread attributes for PUT stream (rc=%d)", attr_rc);
        free(json_str);
        put_stream_task_free(task);
        http_response_set_json_error(res, 500, "Failed to start update task");
        return;
    }

    attr_rc = pthread_attr_setdetachstate(&attr, PTHREAD_CREATE_DETACHED);
    if (attr_rc != 0) {
        log_error("Failed to set detached thread attribute for PUT stream (rc=%d)", attr_rc);
        pthread_attr_destroy(&attr);
        free(json_str);
        put_stream_task_free(task);
        http_response_set_json_error(res, 500, "Failed to start update task");
        return;
    }

    if (pthread_create(&thread_id, &attr, put_stream_worker, task) != 0) {
        log_error("Failed to create worker thread for PUT stream");
        free(json_str);
        put_stream_task_free(task);
        http_response_set_json_error(res, 500, "Failed to start update task");
    } else {
        http_response_set_json(res, 202, json_str);
        free(json_str);
        log_info("PUT stream task started in worker thread for: %s", stream_id);
    }

    pthread_attr_destroy(&attr);
}

/**
 * @brief Structure for DELETE stream background task
 */
typedef struct {
    char stream_id[MAX_STREAM_NAME];          // Current name, checked against captured UUID
    char camera_uuid[CAMERA_UUID_STRING_SIZE];
    bool permanent_delete;                     // Whether to permanently delete
} delete_stream_task_t;

/**
 * @brief Worker function for DELETE stream
 *
 * This function performs the actual stream deletion work in a background thread.
 * It handles stopping the stream, detection threads, removing from memory,
 * database deletion, and go2rtc unregistration.
 */
static void *delete_stream_worker(void *arg) {
    delete_stream_task_t *task = (delete_stream_task_t *)arg;
    if (!task) {
        log_error("Invalid DELETE stream task");
        return NULL;
    }

    log_set_thread_context("StreamAPI", task->stream_id);
    log_info("Processing DELETE /api/streams/%s in worker thread", task->stream_id);

    // Re-resolve the handle by name: a handle captured in the request handler
    // is a slot pointer that may have been freed or reused by another stream
    // (e.g. a concurrent DELETE) before this worker runs.
    stream_handle_t stream = get_stream_by_name(task->stream_id);
    if (!stream) {
        log_warn("Stream %s no longer exists, nothing to delete", task->stream_id);
        free(task);
        return NULL;
    }

    if (!stream_task_identity_matches(task->stream_id, task->camera_uuid)) {
        log_warn("Camera identity changed; dropping queued DELETE for %s", task->stream_id);
        free(task);
        return NULL;
    }

    // Stop stream if it's running
    stream_status_t status = get_stream_status(stream);
    if (status == STREAM_STATUS_RUNNING || status == STREAM_STATUS_STARTING) {
        if (stop_stream(stream) != 0) {
            log_error("Failed to stop stream: %s", task->stream_id);
            // Continue anyway
        }

        // Wait for stream to stop
        int timeout = 30; // 3 seconds
        while (get_stream_status(stream) != STREAM_STATUS_STOPPED && timeout > 0) {
            usleep(100000); // 100ms
            timeout--;
        }
    }

    // Stop any unified detection thread for this stream
    if (is_unified_detection_running(task->stream_id)) {
        log_info("Stopping unified detection thread for stream %s", task->stream_id);
        if (stop_unified_detection_thread(task->stream_id) != 0) {
            log_warn("Failed to stop unified detection thread for stream %s", task->stream_id);
            // Continue anyway
        } else {
            log_info("Successfully stopped unified detection thread for stream %s", task->stream_id);
        }
    }

    // Delete stream from memory
    if (remove_stream(stream) != 0) {
        log_error("Failed to delete stream from memory: %s", task->stream_id);
        free(task);
        return NULL;
    }

    // Delete the stream from the database (permanently or just disable)
    if (delete_stream_config_internal(task->stream_id, task->permanent_delete) != 0) {
        log_error("Failed to %s stream configuration in database for %s",
                task->permanent_delete ? "permanently delete" : "disable",
                task->stream_id);
        free(task);
        return NULL;
    }

    // Unregister stream from go2rtc
    if (!go2rtc_integration_unregister_stream(task->stream_id)) {
        log_warn("Failed to unregister stream %s from go2rtc", task->stream_id);
        // Continue anyway - stream is deleted from database
    }

    log_info("Successfully %s stream in worker thread: %s",
            task->permanent_delete ? "permanently deleted" : "disabled",
            task->stream_id);

    free(task);
    return NULL;
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

    log_info("Handling DELETE /api/streams/%s request", stream_id);

    // Check if permanent delete is requested
    bool permanent_delete = false;
    char permanent_param[16] = {0};
    if (http_request_get_query_param(req, "permanent", permanent_param, sizeof(permanent_param)) > 0) {
        if (strcmp(permanent_param, "true") == 0) {
            permanent_delete = true;
            log_info("Permanent delete requested for stream: %s", stream_id);
        }
    }

    // Find the stream by name
    stream_handle_t stream = get_stream_by_name(stream_id);
    if (!stream) {
        log_error("Stream not found: %s", stream_id);
        http_response_set_json_error(res, 404, "Stream not found");
        return;
    }

    stream_config_t config;
    if (get_stream_config(stream, &config) != 0) {
        http_response_set_json_error(res, 500, "Failed to get stream configuration");
        return;
    }
    if (!check_stream_write_access(req, res, &config)) {
        return;
    }

    // Allocate task for the background worker
    delete_stream_task_t *task = calloc(1, sizeof(delete_stream_task_t));
    if (!task) {
        log_error("Failed to allocate delete stream task");
        http_response_set_json_error(res, 500, "Failed to allocate delete task");
        return;
    }

    safe_strcpy(task->stream_id, stream_id, sizeof(task->stream_id), 0);
    safe_strcpy(task->camera_uuid, config.camera_uuid, sizeof(task->camera_uuid), 0);
    task->permanent_delete = permanent_delete;

    // Prepare the accepted response; the backend sends it after this handler returns.
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

    // Spawn background thread to perform the actual deletion work
    // This prevents blocking the web server event loop
    pthread_t thread_id;
    pthread_attr_t attr;
    int attr_rc = pthread_attr_init(&attr);
    if (attr_rc != 0) {
        log_error("Failed to initialize pthread attributes for DELETE stream worker (rc=%d)", attr_rc);
        free(task);
        free(json_str);
        http_response_set_json_error(res, 500, "Failed to start delete task");
        return;
    }

    attr_rc = pthread_attr_setdetachstate(&attr, PTHREAD_CREATE_DETACHED);
    if (attr_rc != 0) {
        log_error("Failed to set detached state for DELETE stream worker (rc=%d)", attr_rc);
        pthread_attr_destroy(&attr);
        free(task);
        free(json_str);
        http_response_set_json_error(res, 500, "Failed to start delete task");
        return;
    }

    if (pthread_create(&thread_id, &attr, delete_stream_worker, task) != 0) {
        log_error("Failed to create worker thread for DELETE stream");
        free(task);
        free(json_str);
        http_response_set_json_error(res, 500, "Failed to start delete task");
    } else {
        http_response_set_json(res, 202, json_str);
        free(json_str);
        log_info("DELETE stream task started in worker thread for: %s", stream_id);
    }

    pthread_attr_destroy(&attr);
}

/**
 * @brief Structure for stream refresh task
 */
typedef struct {
    char stream_name[MAX_STREAM_NAME];  // Decoded stream name
    char camera_uuid[CAMERA_UUID_STRING_SIZE];
    uint64_t restart_generation;
} refresh_stream_task_t;

#define STREAM_REFRESH_COOLDOWN_SECONDS 30

typedef struct {
    bool used;
    bool in_progress;
    char stream_name[MAX_STREAM_NAME];
    struct timespec last_started;
} refresh_stream_guard_t;

static refresh_stream_guard_t refresh_stream_guards[MAX_STREAMS];
static pthread_mutex_t refresh_stream_guards_mutex = PTHREAD_MUTEX_INITIALIZER;

/* A player retry is cheap, but this endpoint restarts shared source state.
 * Coalesce duplicate requests so several browser cells cannot continuously
 * disrupt one another on a multi-view dashboard (#457). */
static bool reserve_stream_refresh(const char *stream_name,
                                   int *retry_after_seconds) {
    struct timespec now;
    if (clock_gettime(CLOCK_MONOTONIC, &now) != 0) {
        now.tv_sec = time(NULL);
        now.tv_nsec = 0;
    }
    if (retry_after_seconds) {
        *retry_after_seconds = 0;
    }

    pthread_mutex_lock(&refresh_stream_guards_mutex);
    int free_slot = -1;
    int reusable_slot = -1;
    time_t oldest_start = now.tv_sec;

    for (int i = 0; i < MAX_STREAMS; i++) {
        refresh_stream_guard_t *guard = &refresh_stream_guards[i];
        if (!guard->used) {
            if (free_slot < 0) free_slot = i;
            continue;
        }
        if (strcmp(guard->stream_name, stream_name) == 0) {
            time_t elapsed = now.tv_sec - guard->last_started.tv_sec;
            if (elapsed < 0) {
                guard->last_started = now;
                elapsed = 0;
            }
            if (guard->in_progress || elapsed < STREAM_REFRESH_COOLDOWN_SECONDS) {
                if (retry_after_seconds) {
                    *retry_after_seconds = guard->in_progress
                        ? 1
                        : STREAM_REFRESH_COOLDOWN_SECONDS - (int)elapsed;
                }
                pthread_mutex_unlock(&refresh_stream_guards_mutex);
                return false;
            }
            guard->in_progress = true;
            guard->last_started = now;
            pthread_mutex_unlock(&refresh_stream_guards_mutex);
            return true;
        }
        if (!guard->in_progress && guard->last_started.tv_sec <= oldest_start) {
            oldest_start = guard->last_started.tv_sec;
            reusable_slot = i;
        }
    }

    int slot = free_slot >= 0 ? free_slot : reusable_slot;
    if (slot < 0) {
        if (retry_after_seconds) *retry_after_seconds = 1;
        pthread_mutex_unlock(&refresh_stream_guards_mutex);
        return false;
    }

    refresh_stream_guard_t *guard = &refresh_stream_guards[slot];
    memset(guard, 0, sizeof(*guard));
    guard->used = true;
    guard->in_progress = true;
    guard->last_started = now;
    safe_strcpy(guard->stream_name, stream_name, sizeof(guard->stream_name), 0);
    pthread_mutex_unlock(&refresh_stream_guards_mutex);
    return true;
}

static void finish_stream_refresh(const char *stream_name, bool discard) {
    struct timespec now;
    if (clock_gettime(CLOCK_MONOTONIC, &now) != 0) {
        now.tv_sec = time(NULL);
        now.tv_nsec = 0;
    }
    pthread_mutex_lock(&refresh_stream_guards_mutex);
    for (int i = 0; i < MAX_STREAMS; i++) {
        refresh_stream_guard_t *guard = &refresh_stream_guards[i];
        if (guard->used && strcmp(guard->stream_name, stream_name) == 0) {
            if (discard) {
                memset(guard, 0, sizeof(*guard));
            } else {
                guard->in_progress = false;
                guard->last_started = now;
            }
            break;
        }
    }
    pthread_mutex_unlock(&refresh_stream_guards_mutex);
}

/**
 * @brief Worker function for stream refresh
 *
 * This function performs the actual go2rtc reload and detection thread
 * restart in a background thread so the event loop is not blocked.
 */
static void *refresh_stream_worker(void *arg) {
    refresh_stream_task_t *task = arg;
    if (!task) {
        log_error("Invalid refresh stream task");
        return NULL;
    }

    log_set_thread_context("StreamAPI", task->stream_name);
    log_info("Processing stream refresh for %s in worker thread", task->stream_name);

    if (!stream_task_identity_matches(task->stream_name, task->camera_uuid)) {
        log_warn("Camera identity changed; dropping queued refresh for %s", task->stream_name);
        finish_stream_refresh(task->stream_name, true);
        free(task);
        return NULL;
    }

    // If go2rtc integration is not initialized, try to start it
    if (!go2rtc_integration_is_initialized()) {
        log_info("go2rtc integration not initialized, attempting full start");
        if (!go2rtc_integration_full_start()) {
            log_error("Failed to start go2rtc integration for stream refresh: %s", task->stream_name);
            finish_stream_refresh(task->stream_name, false);
            free(task);
            return NULL;
        }
        log_info("go2rtc integration started successfully");
    }

    // Serialize refresh reconfiguration with the shared process lifecycle. If
    // a global restart completed while this task was queued/waiting, that
    // restart already regenerated YAML and re-registered every stream, so the
    // refresh is coalesced instead of disturbing the newly serving instance.
    go2rtc_lifecycle_guard_t lifecycle_guard;
    bool lifecycle_entered = go2rtc_lifecycle_begin(
        GO2RTC_LIFECYCLE_RECONFIGURE, false, true, &lifecycle_guard);
    bool success = false;
    if (!lifecycle_entered) {
        log_error("Failed to enter go2rtc lifecycle for stream refresh: %s",
                  task->stream_name);
    } else if (!stream_task_identity_matches(task->stream_name, task->camera_uuid)) {
        log_warn("Camera identity changed while refresh waited for lifecycle: %s", task->stream_name);
    } else if (lifecycle_guard.restart_generation != task->restart_generation) {
        log_info("Coalesced refresh for %s with completed global go2rtc restart",
                 task->stream_name);
        success = true;
    } else {
        success = go2rtc_integration_reload_stream(task->stream_name);
    }
    if (lifecycle_entered) {
        go2rtc_lifecycle_end(&lifecycle_guard, success);
    }

    if (success) {
        log_info("Successfully refreshed go2rtc registration for stream: %s", task->stream_name);

        // Signal the recording thread to reconnect cleanly rather than
        // discovering the stale RTSP connection through av_read_frame errors.
        signal_mp4_recording_reconnect(task->stream_name);

        // Also restart the unified detection thread if detection-based recording is enabled
        stream_handle_t stream = get_stream_by_name(task->stream_name);
        stream_config_t config;
        if (stream && get_stream_config(stream, &config) == 0) {
            if (config.detection_based_recording &&
                is_detection_recording_scheduled(&config)) {
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

    finish_stream_refresh(task->stream_name, false);
    free(task);
    return NULL;
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

    user_t user;
    fleet_camera_t authorized_camera;
    authorization_evaluation_t evaluation;
    if (!httpd_authorize_stream_action_with_context(req, res, AUTHZ_CAMERA_CONFIGURE,
            stream_name, &user, &authorized_camera, &evaluation)) return;

    log_info("Refreshing go2rtc registration for stream: %s", stream_name);

    // Check if the stream exists
    stream_handle_t stream = get_stream_by_name(stream_name);
    if (!stream) {
        log_error("Stream not found: %s", stream_name);
        http_response_set_json_error(res, 404, "Stream not found");
        return;
    }

    int retry_after_seconds = 0;
    if (!reserve_stream_refresh(stream_name, &retry_after_seconds)) {
        cJSON *response = cJSON_CreateObject();
        if (!response) {
            http_response_set_json_error(res, 500, "Failed to create response JSON");
            return;
        }
        cJSON_AddBoolToObject(response, "success", true);
        cJSON_AddBoolToObject(response, "coalesced", true);
        cJSON_AddStringToObject(response, "message",
                                "A recent stream refresh is already active or cooling down");
        cJSON_AddStringToObject(response, "stream", stream_name);
        cJSON_AddNumberToObject(response, "retry_after_seconds", retry_after_seconds);
        char *json_str = cJSON_PrintUnformatted(response);
        cJSON_Delete(response);
        if (!json_str) {
            http_response_set_json_error(res, 500, "Failed to convert response JSON");
            return;
        }
        http_response_set_json(res, 202, json_str);
        free(json_str);
        return;
    }

    // Allocate task for worker thread
    refresh_stream_task_t *task = calloc(1, sizeof(refresh_stream_task_t));
    if (!task) {
        log_error("Failed to allocate refresh stream task");
        finish_stream_refresh(stream_name, true);
        http_response_set_json_error(res, 500, "Internal server error");
        return;
    }
    safe_strcpy(task->stream_name, stream_name, MAX_STREAM_NAME, 0);
    safe_strcpy(task->camera_uuid, authorized_camera.camera_uuid, sizeof(task->camera_uuid), 0);
    task->restart_generation = go2rtc_lifecycle_restart_generation();

    // Send immediate 202 Accepted response
    cJSON *response = cJSON_CreateObject();
    if (!response) {
        log_error("Failed to create response JSON object");
        free(task);
        finish_stream_refresh(stream_name, true);
        http_response_set_json_error(res, 500, "Failed to create response JSON");
        return;
    }

    cJSON_AddBoolToObject(response, "success", true);
    cJSON_AddStringToObject(response, "message", "Stream refresh request accepted and processing");
    cJSON_AddStringToObject(response, "stream", stream_name);

    char *json_str = cJSON_PrintUnformatted(response);
    cJSON_Delete(response);

    if (!json_str) {
        log_error("Failed to convert response JSON to string");
        free(task);
        finish_stream_refresh(stream_name, true);
        http_response_set_json_error(res, 500, "Failed to convert response JSON");
        return;
    }

    // Spawn background thread to perform the actual refresh work
    // This prevents blocking the web server event loop
    pthread_t thread_id;
    pthread_attr_t attr;
    int attr_rc = pthread_attr_init(&attr);
    if (attr_rc != 0) {
        log_error("Failed to initialize pthread attributes for stream refresh");
        free(task);
        finish_stream_refresh(stream_name, true);
        free(json_str);
        http_response_set_json_error(res, 500, "Failed to start stream refresh");
        return;
    }

    attr_rc = pthread_attr_setdetachstate(&attr, PTHREAD_CREATE_DETACHED);
    if (attr_rc != 0) {
        log_error("Failed to configure detached pthread attributes for stream refresh");
        free(task);
        finish_stream_refresh(stream_name, true);
        free(json_str);
        pthread_attr_destroy(&attr);
        http_response_set_json_error(res, 500, "Failed to start stream refresh");
        return;
    }

    if (pthread_create(&thread_id, &attr, refresh_stream_worker, task) != 0) {
        log_error("Failed to create worker thread for stream refresh");
        free(task);
        finish_stream_refresh(stream_name, true);
        free(json_str);
        pthread_attr_destroy(&attr);
        http_response_set_json_error(res, 500, "Failed to start stream refresh");
        return;
    } else {
        log_info("Stream refresh task started in worker thread for: %s", stream_name);
    }

    pthread_attr_destroy(&attr);
    http_response_set_json(res, 202, json_str);
    free(json_str);
}
