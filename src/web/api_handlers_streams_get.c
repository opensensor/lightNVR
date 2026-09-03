#define _XOPEN_SOURCE
#define _GNU_SOURCE

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <ctype.h>

#include "web/api_handlers.h"
#include "web/request_response.h"
#include "web/httpd_utils.h"
#include "web/fleet_health.h"
#define LOG_COMPONENT "StreamsAPI"
#include "core/logger.h"
#include "core/config.h"
#include "core/url_utils.h"
#include "utils/strings.h"
#include "video/stream_manager.h"
#include "video/streams.h"
#include "video/stream_state.h"
#include "video/detection_stream.h"
#include "database/database_manager.h"
#include "database/db_fleet_query.h"

#include "database/db_motion_config.h"
#include "database/db_auth.h"
#include "video/go2rtc/go2rtc_integration.h"
#include "video/unified_detection_thread.h"

/**
 * Resolve the effective stream status for API reporting.
 *
 * When go2rtc manages streams the stream-state manager stays at INACTIVE
 * (STOPPED) because start_stream_with_state() is not called on startup.
 * This helper consults the Unified Detection Thread (UDT) to obtain a more
 * accurate status. An open go2rtc session alone is not evidence of usable
 * video, so a missing/stopped producer remains STOPPED.
 */
static stream_status_t resolve_effective_stream_status(stream_status_t raw_status,
                                                       const char *stream_name,
                                                       bool stream_enabled) {
    if (raw_status == STREAM_STATUS_STOPPED && stream_enabled
            && go2rtc_integration_is_initialized()) {
        /* Ask the UDT for a more accurate status */
        stream_status_t udt_status = get_unified_detection_effective_status(stream_name);
        if (udt_status != STREAM_STATUS_STOPPED) {
            return udt_status;
        }
        return STREAM_STATUS_STOPPED;
    }
    return raw_status;
}

static void get_stream_api_credentials(const stream_config_t *config,
                                       char *safe_url, size_t safe_url_size,
                                       char *onvif_username, size_t onvif_username_size,
                                       char *onvif_password, size_t onvif_password_size,
                                       bool expose_sensitive_config) {
    char extracted_username[128] = {0};
    char extracted_password[128] = {0};
    bool use_separate_credentials;

    if (!config) {
        return;
    }

    safe_strcpy(safe_url, config->url, safe_url_size, 0);

    safe_strcpy(onvif_username, config->onvif_username, onvif_username_size, 0);
    safe_strcpy(onvif_password, config->onvif_password, onvif_password_size, 0);

    use_separate_credentials = config->is_onvif ||
                              config->onvif_username[0] != '\0' ||
                              config->onvif_password[0] != '\0';
    if (use_separate_credentials &&
        (onvif_username[0] == '\0' || onvif_password[0] == '\0') &&
        url_extract_credentials(config->url,
                                extracted_username, sizeof(extracted_username),
                                extracted_password, sizeof(extracted_password)) == 0) {
        if (onvif_username[0] == '\0' && extracted_username[0] != '\0') {
            safe_strcpy(onvif_username, extracted_username, onvif_username_size, 0);
        }
        if (onvif_password[0] == '\0' && extracted_password[0] != '\0') {
            safe_strcpy(onvif_password, extracted_password, onvif_password_size, 0);
        }
    }

    if (use_separate_credentials || !expose_sensitive_config) {
        if (url_strip_credentials(config->url, safe_url, safe_url_size) != 0) {
            /* A viewer must never receive a URL that may contain credentials.
             * Elevated users retain the legacy value so stream editing does
             * not lose unsupported/custom URL formats. */
            safe_strcpy(safe_url, expose_sensitive_config ? config->url : "",
                        safe_url_size, 0);
        }
    }

    if (!expose_sensitive_config) {
        onvif_username[0] = '\0';
        onvif_password[0] = '\0';
    }
}

static bool stream_user_can_modify(const user_t *user,
                                   const fleet_camera_t *camera) {
    authorization_evaluation_t evaluation;
    return user && camera &&
           authorization_evaluate(user, AUTHZ_CAMERA_CONFIGURE, camera,
                                  &evaluation) == 0 &&
           evaluation.decision == AUTHZ_DECISION_ALLOW;
}

static const fleet_camera_t *find_loaded_camera(
    const fleet_camera_t *cameras, int camera_count,
    const stream_config_t *config) {
    if (!cameras || !config) return NULL;
    if (config->camera_uuid[0] != '\0') {
        int low = 0;
        int high = camera_count - 1;
        while (low <= high) {
            int middle = low + (high - low) / 2;
            int comparison = strcmp(cameras[middle].camera_uuid,
                                    config->camera_uuid);
            if (comparison == 0) return &cameras[middle];
            if (comparison < 0) low = middle + 1;
            else high = middle - 1;
        }
    }
    /* Legacy rows may predate camera UUIDs. Keep the name fallback isolated
     * to those exceptional records instead of scanning for every camera. */
    for (int i = 0; config->camera_uuid[0] == '\0' && i < camera_count; i++) {
        if (strcmp(cameras[i].name, config->name) == 0) {
            return &cameras[i];
        }
    }
    return NULL;
}

typedef struct {
    const stream_config_t *config;
    const fleet_camera_t *camera;
    bool can_configure;
} stream_summary_row_t;

static _Thread_local char stream_summary_sort[24];
static _Thread_local bool stream_summary_descending;

static const char *summary_status(const fleet_camera_t *camera) {
    if (!camera || !camera->enabled) return "Stopped";
    switch (camera->health) {
        case FLEET_HEALTH_UP: return "Running";
        case FLEET_HEALTH_DEGRADED: return "Degraded";
        case FLEET_HEALTH_DOWN: return "Error";
        case FLEET_HEALTH_DISABLED: return "Stopped";
        case FLEET_HEALTH_UNKNOWN: default: return "Unknown";
    }
}

static int compare_stream_summaries(const void *left_ptr,
                                    const void *right_ptr) {
    const stream_summary_row_t *left = left_ptr;
    const stream_summary_row_t *right = right_ptr;
    int result = 0;
    if (strcmp(stream_summary_sort, "status") == 0) {
        result = strcasecmp(summary_status(left->camera),
                            summary_status(right->camera));
    } else if (strcmp(stream_summary_sort, "resolution") == 0) {
        int64_t left_pixels =
            (int64_t)left->config->width * left->config->height;
        int64_t right_pixels =
            (int64_t)right->config->width * right->config->height;
        result = left_pixels < right_pixels ? -1 : left_pixels > right_pixels;
    } else if (strcmp(stream_summary_sort, "fps") == 0) {
        result = left->config->fps - right->config->fps;
    } else if (strcmp(stream_summary_sort, "recording") == 0) {
        result = (int)left->config->record - (int)right->config->record;
    } else {
        result = strcasecmp(left->config->name, right->config->name);
    }
    if (result == 0) {
        result = strcasecmp(left->config->name, right->config->name);
    }
    return stream_summary_descending ? -result : result;
}

static bool contains_case_insensitive(const char *text, const char *needle) {
    if (!needle || needle[0] == '\0') return true;
    if (!text) return false;
    size_t length = strlen(needle);
    for (const char *cursor = text; *cursor; cursor++) {
        if (strncasecmp(cursor, needle, length) == 0) return true;
    }
    return false;
}

static bool stream_summary_matches(const stream_config_t *config,
                                   const fleet_camera_t *camera,
                                   const char *search) {
    return !search || search[0] == '\0' ||
           contains_case_insensitive(config->name, search) ||
           contains_case_insensitive(config->camera_uuid, search) ||
           contains_case_insensitive(config->tags, search) ||
           (camera &&
            (contains_case_insensitive(camera->location_path, search) ||
             contains_case_insensitive(camera->address, search)));
}

static bool parse_summary_positive_int(const http_request_t *req,
                                       const char *name, int fallback,
                                       int maximum, int *value) {
    char text[32] = {0};
    if (http_request_get_query_param(req, name, text, sizeof(text)) < 0) {
        *value = fallback;
        return true;
    }
    char *end = NULL;
    long parsed = strtol(text, &end, 10);
    if (!text[0] || !end || *end != '\0' || parsed < 1 || parsed > maximum) {
        return false;
    }
    *value = (int)parsed;
    return true;
}

static int handle_get_stream_summaries(
    const http_request_t *req, http_response_t *res, const user_t *user,
    authorization_context_t *authz_context, stream_config_t *db_streams,
    int stream_count, fleet_camera_t *fleet_cameras, int fleet_camera_count) {
    int page = 1;
    int page_size = 50;
    if (!parse_summary_positive_int(req, "page", 1, 1000000, &page) ||
        !parse_summary_positive_int(req, "page_size", 50, 100, &page_size)) {
        http_response_set_json_error(
            res, 400, "page and page_size must be positive; page_size max is 100");
        return -1;
    }
    char search[256] = {0};
    char sort_by[24] = "name";
    char sort_order[8] = "asc";
    char surface[16] = "admin";
    char availability[24] = "all";
    char include_admin_url_param[8] = {0};
    http_request_get_query_param(req, "search", search, sizeof(search));
    http_request_get_query_param(req, "sort_by", sort_by, sizeof(sort_by));
    http_request_get_query_param(req, "sort_order", sort_order,
                                 sizeof(sort_order));
    http_request_get_query_param(req, "surface", surface, sizeof(surface));
    http_request_get_query_param(req, "availability", availability,
                                 sizeof(availability));
    bool include_admin_url =
        http_request_get_query_param(
            req, "include_admin_url", include_admin_url_param,
            sizeof(include_admin_url_param)) > 0 &&
        (strcmp(include_admin_url_param, "true") == 0 ||
         strcmp(include_admin_url_param, "1") == 0);
    if (strcmp(sort_by, "name") != 0 && strcmp(sort_by, "status") != 0 &&
        strcmp(sort_by, "resolution") != 0 && strcmp(sort_by, "fps") != 0 &&
        strcmp(sort_by, "recording") != 0) {
        http_response_set_json_error(res, 400, "Invalid stream summary sort field");
        return -1;
    }
    if (strcmp(sort_order, "asc") != 0 && strcmp(sort_order, "desc") != 0) {
        http_response_set_json_error(res, 400, "sort_order must be asc or desc");
        return -1;
    }
    bool live_surface = strcmp(surface, "live") == 0;
    if (!live_surface && strcmp(surface, "admin") != 0) {
        http_response_set_json_error(res, 400,
                                     "surface must be admin or live");
        return -1;
    }
    if (strcmp(availability, "all") != 0 &&
        strcmp(availability, "live") != 0 &&
        strcmp(availability, "offline") != 0 &&
        strcmp(availability, "never_connected") != 0 &&
        strcmp(availability, "disabled") != 0) {
        http_response_set_json_error(res, 400,
                                     "Invalid availability value");
        return -1;
    }

    stream_summary_row_t *rows = calloc((size_t)stream_count, sizeof(*rows));
    if (!rows) {
        http_response_set_json_error(res, 500, "Out of memory");
        return -1;
    }
    int row_count = 0;
    for (int i = 0; i < stream_count; i++) {
        const fleet_camera_t *camera = find_loaded_camera(
            fleet_cameras, fleet_camera_count, &db_streams[i]);
        authorization_evaluation_t live = {0};
        authorization_evaluation_t configure = {0};
        if (!camera || authorization_evaluate_in_context(
                authz_context, user, AUTHZ_LIVE_VIEW, camera, &live) != 0 ||
            authorization_evaluate_in_context(
                authz_context, user, AUTHZ_CAMERA_CONFIGURE, camera,
                &configure) != 0) {
            free(rows);
            http_response_set_json_error(
                res, 500, "Authorization policy evaluation failed");
            return -1;
        }
        if (live.decision != AUTHZ_DECISION_ALLOW ||
            !stream_summary_matches(&db_streams[i], camera, search) ||
            (strcmp(availability, "all") != 0 &&
             strcmp(availability, fleet_availability_state_name(
                        camera->availability)) != 0)) {
            continue;
        }
        rows[row_count++] = (stream_summary_row_t){
            .config = &db_streams[i],
            .camera = camera,
            .can_configure = configure.decision == AUTHZ_DECISION_ALLOW,
        };
    }
    safe_strcpy(stream_summary_sort, sort_by, sizeof(stream_summary_sort), 0);
    stream_summary_descending = strcmp(sort_order, "desc") == 0;
    if (row_count > 1) {
        qsort(rows, (size_t)row_count, sizeof(*rows),
              compare_stream_summaries);
    }

    cJSON *root = cJSON_CreateObject();
    cJSON *items = root ? cJSON_AddArrayToObject(root, "streams") : NULL;
    if (!root || !items) {
        cJSON_Delete(root);
        free(rows);
        http_response_set_json_error(res, 500, "Failed to create stream summaries");
        return -1;
    }
    int64_t offset64 = (int64_t)(page - 1) * page_size;
    int offset = offset64 > row_count ? row_count : (int)offset64;
    int end = offset + page_size;
    if (end > row_count) end = row_count;
    for (int i = offset; i < end; i++) {
        const stream_config_t *config = rows[i].config;
        const fleet_camera_t *camera = rows[i].camera;
        cJSON *item = cJSON_CreateObject();
        if (!item) continue;
        cJSON_AddStringToObject(item, "camera_uuid", config->camera_uuid);
        cJSON_AddStringToObject(item, "location_uuid", config->location_uuid);
        cJSON_AddStringToObject(item, "location_name", camera->location_name);
        cJSON_AddStringToObject(item, "location_path", camera->location_path);
        cJSON_AddStringToObject(item, "name", config->name);
        cJSON_AddBoolToObject(item, "enabled", config->enabled);
        cJSON_AddNumberToObject(item, "width", config->width);
        cJSON_AddNumberToObject(item, "height", config->height);
        cJSON_AddNumberToObject(item, "fps", config->fps);
        cJSON_AddStringToObject(item, "codec", config->codec);
        cJSON_AddNumberToObject(item, "protocol", (int)config->protocol);
        cJSON_AddBoolToObject(item, "record", config->record);
        cJSON_AddBoolToObject(item, "record_on_schedule",
                              config->record_on_schedule);
        cJSON_AddBoolToObject(item, "detection_based_recording",
                              config->detection_based_recording);
        cJSON_AddBoolToObject(item, "isOnvif", config->is_onvif);
        cJSON_AddStringToObject(item, "tags", config->tags);
        cJSON_AddStringToObject(item, "status", summary_status(camera));
        cJSON_AddStringToObject(
            item, "availability",
            fleet_availability_state_name(camera->availability));
        cJSON_AddNumberToObject(item, "first_video_at",
                                (double)camera->first_video_at);
        cJSON_AddNumberToObject(item, "last_frame_ts",
                                (double)camera->last_frame_ts);
        cJSON_AddNumberToObject(item, "current_fps", camera->current_fps);
        cJSON_AddBoolToObject(item, "recording_active",
                              camera->recording_active);
        cJSON_AddNumberToObject(item, "last_recording_at",
                                (double)camera->last_recording_at);
        cJSON_AddBoolToObject(item, "can_configure", rows[i].can_configure);
        if (!live_surface && include_admin_url) {
            cJSON_AddStringToObject(
                item, "admin_url",
                rows[i].can_configure ? config->admin_url : "");
        }
        if (live_surface) {
            cJSON_AddBoolToObject(item, "streaming_enabled",
                                  config->streaming_enabled);
            cJSON_AddStringToObject(
                item, "playback_transport",
                playback_transport_is_valid(config->playback_transport)
                    ? config->playback_transport : "auto");
            cJSON_AddStringToObject(item, "eptz_config", config->eptz_config);
            cJSON_AddStringToObject(item, "detection_model",
                                    config->detection_model);
            cJSON_AddBoolToObject(item, "privacy_mode", config->privacy_mode);
            cJSON_AddBoolToObject(item, "can_control_privacy",
                                  rows[i].can_configure);
            cJSON_AddBoolToObject(item, "has_sub_stream",
                                  config->sub_stream_url[0] != '\0');
            cJSON_AddBoolToObject(item, "ptz_enabled", config->ptz_enabled);
            cJSON_AddBoolToObject(item, "backchannel_enabled",
                                  config->backchannel_enabled);
            cJSON_AddBoolToObject(
                item, "go2rtc_hls_available",
                go2rtc_integration_is_using_go2rtc_for_hls(config->name));
        }
        cJSON_AddItemToArray(items, item);
    }
    cJSON_AddNumberToObject(root, "total", row_count);
    cJSON_AddNumberToObject(root, "page", page);
    cJSON_AddNumberToObject(root, "page_size", page_size);
    cJSON_AddNumberToObject(root, "total_pages",
                            row_count == 0 ? 0 :
                            (row_count + page_size - 1) / page_size);
    char *encoded = cJSON_PrintUnformatted(root);
    if (!encoded) {
        cJSON_Delete(root);
        free(rows);
        http_response_set_json_error(res, 500, "Failed to encode stream summaries");
        return -1;
    }
    http_response_set_json(res, 200, encoded);
    free(encoded);
    cJSON_Delete(root);
    free(rows);
    return 0;
}

/**
 * @brief Backend-agnostic handler for GET /api/streams
 */
void handle_get_streams(const http_request_t *req, http_response_t *res) {
	log_info("Handling GET /api/streams request");

	// Capture the authenticated principal once; resource decisions below use a
    // shared context so selector grants do not cause one grant load per row.
	user_t auth_user;
	memset(&auth_user, 0, sizeof(auth_user));
    if (!httpd_check_action_access(req, &auth_user)) {
        http_response_set_json_error(res, 401, "Unauthorized");
        return;
    }
    authorization_context_t *authz_context = authorization_context_create();
    if (!authz_context) {
        http_response_set_json_error(res, 500, "Out of memory");
        return;
    }

    // Get all stream configurations from database (heap-allocated)
    stream_config_t *db_streams = calloc(g_config.max_streams, sizeof(stream_config_t));
    if (!db_streams) {
        log_error("handle_get_streams: out of memory");
        http_response_set_json_error(res, 500, "Internal error");
        authorization_context_free(authz_context);
        return;
    }
    int count = get_all_stream_configs(db_streams, g_config.max_streams);

    if (count < 0) {
        log_error("Failed to get stream configurations from database");
        free(db_streams);
        authorization_context_free(authz_context);
        http_response_set_json_error(res, 500, "Failed to get stream configurations");
        return;
    }

    // Load the credential-free inventory once. At large fleet sizes a lookup
    // per stream would otherwise turn this endpoint into N database queries.
    fleet_camera_t *fleet_cameras = NULL;
    int fleet_camera_count = 0;
    if (db_fleet_camera_load(&fleet_cameras, &fleet_camera_count) != 0) {
        free(db_streams);
        authorization_context_free(authz_context);
        http_response_set_json_error(res, 500, "Failed to load camera inventory");
        return;
    }
    /* Build runtime health once for both the summary and compatibility
     * contracts. This is keyed by stream name internally, so the legacy list
     * no longer performs a linear runtime lookup for every camera. */
    fleet_camera_enrich_runtime_health(fleet_cameras, fleet_camera_count);
    fleet_health_enrich_go2rtc_activity(fleet_cameras, fleet_camera_count);

    char summary_param[8] = {0};
    bool summary_request =
        http_request_get_query_param(req, "summary", summary_param,
                                     sizeof(summary_param)) > 0 &&
        (strcmp(summary_param, "true") == 0 ||
         strcmp(summary_param, "1") == 0);
    if (summary_request) {
        handle_get_stream_summaries(
            req, res, &auth_user, authz_context, db_streams, count,
            fleet_cameras, fleet_camera_count);
        free(fleet_cameras);
        free(db_streams);
        authorization_context_free(authz_context);
        return;
    }

    // Create JSON array
    cJSON *streams_array = cJSON_CreateArray();
    if (!streams_array) {
        log_error("Failed to create streams JSON array");
        free(fleet_cameras);
        free(db_streams);
        authorization_context_free(authz_context);
        http_response_set_json_error(res, 500, "Failed to create streams JSON");
        return;
    }

    // Add only streams allowed by the centralized live.view evaluator.
    for (int i = 0; i < count; i++) {
        const fleet_camera_t *camera =
            find_loaded_camera(fleet_cameras, fleet_camera_count,
                               &db_streams[i]);
        authorization_evaluation_t live_evaluation;
        authorization_evaluation_t configure_evaluation;
        if (!camera ||
            authorization_evaluate_in_context(
                authz_context, &auth_user, AUTHZ_LIVE_VIEW, camera,
                &live_evaluation) != 0 ||
            authorization_evaluate_in_context(
                authz_context, &auth_user, AUTHZ_CAMERA_CONFIGURE, camera,
                &configure_evaluation) != 0) {
            cJSON_Delete(streams_array);
            free(fleet_cameras);
            free(db_streams);
            authorization_context_free(authz_context);
            http_response_set_json_error(
                res, 500, "Authorization policy evaluation failed");
            return;
        }
        if (live_evaluation.decision != AUTHZ_DECISION_ALLOW) continue;
        const bool expose_sensitive_config =
            configure_evaluation.decision == AUTHZ_DECISION_ALLOW;

        cJSON *stream_obj = cJSON_CreateObject();
        if (!stream_obj) {
            log_error("Failed to create stream JSON object");
            cJSON_Delete(streams_array);
            free(fleet_cameras);
            free(db_streams);
            authorization_context_free(authz_context);
            http_response_set_json_error(res, 500, "Failed to create stream JSON");
            return;
        }

        char safe_url[MAX_URL_LENGTH];
        char api_onvif_username[sizeof(db_streams[i].onvif_username)];
        char api_onvif_password[sizeof(db_streams[i].onvif_password)];
        get_stream_api_credentials(&db_streams[i], safe_url, sizeof(safe_url),
                                   api_onvif_username, sizeof(api_onvif_username),
                                   api_onvif_password, sizeof(api_onvif_password),
                                   expose_sensitive_config);

        // Add stream properties
        cJSON_AddStringToObject(stream_obj, "camera_uuid", db_streams[i].camera_uuid);
        cJSON_AddStringToObject(stream_obj, "location_uuid", db_streams[i].location_uuid);
        cJSON_AddStringToObject(stream_obj, "name", db_streams[i].name);
        cJSON_AddStringToObject(stream_obj, "url", safe_url);
        cJSON_AddBoolToObject(stream_obj, "enabled", db_streams[i].enabled);
        cJSON_AddBoolToObject(stream_obj, "streaming_enabled", db_streams[i].streaming_enabled);
        cJSON_AddStringToObject(stream_obj, "playback_transport",
                                playback_transport_is_valid(db_streams[i].playback_transport)
                                    ? db_streams[i].playback_transport : "auto");
        cJSON_AddStringToObject(stream_obj, "eptz_config", db_streams[i].eptz_config);
        cJSON_AddNumberToObject(stream_obj, "width", db_streams[i].width);
        cJSON_AddNumberToObject(stream_obj, "height", db_streams[i].height);
        cJSON_AddNumberToObject(stream_obj, "fps", db_streams[i].fps);
        cJSON_AddStringToObject(stream_obj, "codec", db_streams[i].codec);
        cJSON_AddNumberToObject(stream_obj, "priority", db_streams[i].priority);
        cJSON_AddBoolToObject(stream_obj, "record", db_streams[i].record);
        cJSON_AddNumberToObject(stream_obj, "segment_duration", db_streams[i].segment_duration);

        // Add detection settings
        cJSON_AddBoolToObject(stream_obj, "detection_based_recording", db_streams[i].detection_based_recording);
        cJSON_AddStringToObject(stream_obj, "detection_model", db_streams[i].detection_model);

        // Convert threshold from 0.0-1.0 to percentage (0-100)
        int threshold_percent = (int)(db_streams[i].detection_threshold * 100.0f);
        cJSON_AddNumberToObject(stream_obj, "detection_threshold", threshold_percent);

        cJSON_AddNumberToObject(stream_obj, "detection_interval", db_streams[i].detection_interval);
        cJSON_AddNumberToObject(stream_obj, "pre_detection_buffer", db_streams[i].pre_detection_buffer);
        cJSON_AddNumberToObject(stream_obj, "post_detection_buffer", db_streams[i].post_detection_buffer);
        cJSON_AddStringToObject(stream_obj, "detection_object_filter", db_streams[i].detection_object_filter);
        cJSON_AddStringToObject(stream_obj, "detection_object_filter_list", db_streams[i].detection_object_filter_list);
        cJSON_AddNumberToObject(stream_obj, "protocol", (int)db_streams[i].protocol);
        cJSON_AddBoolToObject(stream_obj, "record_audio", db_streams[i].record_audio);
        cJSON_AddBoolToObject(stream_obj, "audio_voice_enhancement", db_streams[i].audio_voice_enhancement);
        cJSON_AddBoolToObject(stream_obj, "isOnvif", db_streams[i].is_onvif);
        cJSON_AddBoolToObject(stream_obj, "backchannel_enabled", db_streams[i].backchannel_enabled);
        cJSON_AddNumberToObject(stream_obj, "retention_days", db_streams[i].retention_days);
        cJSON_AddNumberToObject(stream_obj, "detection_retention_days", db_streams[i].detection_retention_days);
        cJSON_AddNumberToObject(stream_obj, "max_storage_mb", db_streams[i].max_storage_mb);
        cJSON_AddNumberToObject(stream_obj, "tier_critical_multiplier", db_streams[i].tier_critical_multiplier);
        cJSON_AddNumberToObject(stream_obj, "tier_important_multiplier", db_streams[i].tier_important_multiplier);
        cJSON_AddNumberToObject(stream_obj, "tier_ephemeral_multiplier", db_streams[i].tier_ephemeral_multiplier);
        cJSON_AddNumberToObject(stream_obj, "storage_priority", db_streams[i].storage_priority);
        cJSON_AddBoolToObject(stream_obj, "ptz_enabled", db_streams[i].ptz_enabled);
        cJSON_AddNumberToObject(stream_obj, "ptz_max_x", db_streams[i].ptz_max_x);
        cJSON_AddNumberToObject(stream_obj, "ptz_max_y", db_streams[i].ptz_max_y);
        cJSON_AddNumberToObject(stream_obj, "ptz_max_z", db_streams[i].ptz_max_z);
        cJSON_AddBoolToObject(stream_obj, "ptz_has_home", db_streams[i].ptz_has_home);
        cJSON_AddStringToObject(stream_obj, "onvif_username", api_onvif_username);
        cJSON_AddStringToObject(stream_obj, "onvif_password", api_onvif_password);
        cJSON_AddStringToObject(stream_obj, "onvif_profile", db_streams[i].onvif_profile);
        cJSON_AddNumberToObject(stream_obj, "onvif_port", db_streams[i].onvif_port);
        cJSON_AddBoolToObject(stream_obj, "record_on_schedule", db_streams[i].record_on_schedule);
        cJSON *schedule_arr_i = cJSON_CreateArray();
        if (schedule_arr_i) {
            for (int j = 0; j < 168; j++) {
                cJSON_AddItemToArray(schedule_arr_i,
                    cJSON_CreateBool(db_streams[i].recording_schedule[j] != 0));
            }
            cJSON_AddItemToObject(stream_obj, "recording_schedule", schedule_arr_i);
        }
        cJSON_AddBoolToObject(stream_obj, "detection_record_on_schedule",
                              db_streams[i].detection_record_on_schedule);
        cJSON *detection_schedule_arr_i = cJSON_CreateArray();
        if (detection_schedule_arr_i) {
            for (int j = 0; j < 168; j++) {
                cJSON_AddItemToArray(detection_schedule_arr_i,
                    cJSON_CreateBool(db_streams[i].detection_recording_schedule[j] != 0));
            }
            cJSON_AddItemToObject(stream_obj, "detection_recording_schedule",
                                  detection_schedule_arr_i);
        }
        cJSON_AddStringToObject(stream_obj, "tags", db_streams[i].tags);
        cJSON_AddStringToObject(stream_obj, "admin_url",
                                expose_sensitive_config ? db_streams[i].admin_url : "");
        cJSON_AddBoolToObject(stream_obj, "privacy_mode", db_streams[i].privacy_mode);
        cJSON_AddBoolToObject(stream_obj, "can_control_privacy", expose_sensitive_config);
        cJSON_AddStringToObject(stream_obj, "motion_trigger_source", db_streams[i].motion_trigger_source);
        cJSON_AddStringToObject(stream_obj, "go2rtc_source_override",
                                expose_sensitive_config ? db_streams[i].go2rtc_source_override : "");
        cJSON_AddStringToObject(stream_obj, "sub_stream_url",
                                expose_sensitive_config ? db_streams[i].sub_stream_url : "");
        cJSON_AddBoolToObject(stream_obj, "has_sub_stream",
                              db_streams[i].sub_stream_url[0] != '\0');
        cJSON_AddStringToObject(stream_obj, "detection_url",
                                expose_sensitive_config ? db_streams[i].detection_url : "");
        cJSON_AddStringToObject(stream_obj, "publish_url",
                                expose_sensitive_config ? db_streams[i].publish_url : "");

        /* Prefer frame-derived fleet health. An open session (or advancing
         * audio alone) is not sufficient evidence that video is usable. */
        const char *status = summary_status(camera);
        if (camera->health == FLEET_HEALTH_UNKNOWN && db_streams[i].enabled) {
            stream_handle_t stream = get_stream_by_name(db_streams[i].name);
            status = "Unknown";
            if (stream) {
                stream_status_t stream_status = get_stream_status(stream);

                // Resolve effective status when go2rtc manages the stream.
                stream_status = resolve_effective_stream_status(
                    stream_status, db_streams[i].name, db_streams[i].enabled);

                switch (stream_status) {
                    case STREAM_STATUS_STOPPED:       status = "Stopped";      break;
                    case STREAM_STATUS_STARTING:      status = "Starting";     break;
                    case STREAM_STATUS_RUNNING:       status = "Running";      break;
                    case STREAM_STATUS_STOPPING:      status = "Stopping";     break;
                    case STREAM_STATUS_ERROR:         status = "Error";        break;
                    case STREAM_STATUS_RECONNECTING:  status = "Reconnecting"; break;
                    default:                          status = "Unknown";      break;
                }
            }
        }
        cJSON_AddStringToObject(stream_obj, "status", status);

        // Surface the specific cause of an Error state (e.g. failed to load
        // a detection model) so the UI can show it in a tooltip.
        stream_state_manager_t *sm_err = get_stream_state_by_name(db_streams[i].name);
        if (sm_err && strcmp(status, "Error") == 0) {
            char err_msg[STREAM_ERROR_MESSAGE_MAX];
            pthread_mutex_lock(&sm_err->mutex);
            safe_strcpy(err_msg, sm_err->last_error_message, sizeof(err_msg), 0);
            pthread_mutex_unlock(&sm_err->mutex);
            if (err_msg[0] != '\0') {
                cJSON_AddStringToObject(stream_obj, "error_message", err_msg);
            }
        }

        // Add go2rtc HLS availability - tells frontend whether go2rtc is providing HLS for this stream
        cJSON_AddBoolToObject(stream_obj, "go2rtc_hls_available",
            go2rtc_integration_is_using_go2rtc_for_hls(db_streams[i].name));

        // Add stream to array
        cJSON_AddItemToArray(streams_array, stream_obj);
    }

    // Convert to string
    char *json_str = cJSON_PrintUnformatted(streams_array);
    if (!json_str) {
        log_error("Failed to convert streams JSON to string");
        cJSON_Delete(streams_array);
        free(fleet_cameras);
        free(db_streams);
        authorization_context_free(authz_context);
        http_response_set_json_error(res, 500, "Failed to convert streams JSON to string");
        return;
    }

    // Send response
    http_response_set_json(res, 200, json_str);

    // Clean up
    free(json_str);
    free(fleet_cameras);
    free(db_streams);
    authorization_context_free(authz_context);
    cJSON_Delete(streams_array);

    log_info("Successfully handled GET /api/streams request");
}

/**
 * @brief Backend-agnostic handler for GET /api/streams/:id
 */
void handle_get_stream(const http_request_t *req, http_response_t *res) {
    // Extract stream ID from URL
    char stream_id[MAX_STREAM_NAME];
    if (http_request_extract_path_param(req, "/api/streams/", stream_id, sizeof(stream_id)) != 0) {
        log_error("Failed to extract stream ID from URL");
        http_response_set_json_error(res, 400, "Invalid request path");
        return;
    }

    log_info("Handling GET /api/streams/%s request", stream_id);

    // Find the stream by name
    stream_handle_t stream = get_stream_by_name(stream_id);
    if (!stream) {
        log_error("Stream not found: %s", stream_id);
        http_response_set_json_error(res, 404, "Stream not found");
        return;
    }

    // Get stream configuration
    stream_config_t config;
    if (get_stream_config(stream, &config) != 0) {
        log_error("Failed to get stream configuration for: %s", stream_id);
        http_response_set_json_error(res, 500, "Failed to get stream configuration");
        return;
    }
    user_t auth_user;
    fleet_camera_t camera;
    authorization_evaluation_t live_evaluation;
    if (!httpd_authorize_stream_action_with_context(
            req, res, AUTHZ_LIVE_VIEW, stream_id, &auth_user, &camera,
            &live_evaluation)) {
        return;
    }
    fleet_camera_enrich_runtime_health(&camera, 1);
    const bool expose_sensitive_config =
        stream_user_can_modify(&auth_user, &camera);

    // Create JSON object
    cJSON *stream_obj = cJSON_CreateObject();
    if (!stream_obj) {
        log_error("Failed to create stream JSON object");
        http_response_set_json_error(res, 500, "Failed to create stream JSON");
        return;
    }

    char safe_url[MAX_URL_LENGTH];
    char api_onvif_username[sizeof(config.onvif_username)];
    char api_onvif_password[sizeof(config.onvif_password)];
    get_stream_api_credentials(&config, safe_url, sizeof(safe_url),
                               api_onvif_username, sizeof(api_onvif_username),
                               api_onvif_password, sizeof(api_onvif_password),
                               expose_sensitive_config);

    // Add stream properties
    cJSON_AddStringToObject(stream_obj, "camera_uuid", config.camera_uuid);
    cJSON_AddStringToObject(stream_obj, "location_uuid", config.location_uuid);
    cJSON_AddStringToObject(stream_obj, "name", config.name);
    cJSON_AddStringToObject(stream_obj, "url", safe_url);
    cJSON_AddBoolToObject(stream_obj, "enabled", config.enabled);
    cJSON_AddBoolToObject(stream_obj, "streaming_enabled", config.streaming_enabled);
    cJSON_AddStringToObject(stream_obj, "playback_transport",
                            playback_transport_is_valid(config.playback_transport)
                                ? config.playback_transport : "auto");
    cJSON_AddStringToObject(stream_obj, "eptz_config", config.eptz_config);
    cJSON_AddNumberToObject(stream_obj, "width", config.width);
    cJSON_AddNumberToObject(stream_obj, "height", config.height);
    cJSON_AddNumberToObject(stream_obj, "fps", config.fps);
    cJSON_AddStringToObject(stream_obj, "codec", config.codec);
    cJSON_AddNumberToObject(stream_obj, "priority", config.priority);
    cJSON_AddBoolToObject(stream_obj, "record", config.record);
    cJSON_AddNumberToObject(stream_obj, "segment_duration", config.segment_duration);

    // Add detection settings
    cJSON_AddBoolToObject(stream_obj, "detection_based_recording", config.detection_based_recording);
    cJSON_AddStringToObject(stream_obj, "detection_model", config.detection_model);

    // Convert threshold from 0.0-1.0 to percentage (0-100)
    int threshold_percent = (int)(config.detection_threshold * 100.0f);
    cJSON_AddNumberToObject(stream_obj, "detection_threshold", threshold_percent);

    cJSON_AddNumberToObject(stream_obj, "detection_interval", config.detection_interval);
    cJSON_AddNumberToObject(stream_obj, "pre_detection_buffer", config.pre_detection_buffer);
    cJSON_AddNumberToObject(stream_obj, "post_detection_buffer", config.post_detection_buffer);
    cJSON_AddStringToObject(stream_obj, "detection_object_filter", config.detection_object_filter);
    cJSON_AddStringToObject(stream_obj, "detection_object_filter_list", config.detection_object_filter_list);
    cJSON_AddNumberToObject(stream_obj, "protocol", (int)config.protocol);
    cJSON_AddBoolToObject(stream_obj, "record_audio", config.record_audio);
    cJSON_AddBoolToObject(stream_obj, "audio_voice_enhancement", config.audio_voice_enhancement);
    cJSON_AddBoolToObject(stream_obj, "isOnvif", config.is_onvif);
    cJSON_AddBoolToObject(stream_obj, "backchannel_enabled", config.backchannel_enabled);
    cJSON_AddNumberToObject(stream_obj, "retention_days", config.retention_days);
    cJSON_AddNumberToObject(stream_obj, "detection_retention_days", config.detection_retention_days);
    cJSON_AddNumberToObject(stream_obj, "max_storage_mb", config.max_storage_mb);
    cJSON_AddNumberToObject(stream_obj, "tier_critical_multiplier", config.tier_critical_multiplier);
    cJSON_AddNumberToObject(stream_obj, "tier_important_multiplier", config.tier_important_multiplier);
    cJSON_AddNumberToObject(stream_obj, "tier_ephemeral_multiplier", config.tier_ephemeral_multiplier);
    cJSON_AddNumberToObject(stream_obj, "storage_priority", config.storage_priority);
    cJSON_AddBoolToObject(stream_obj, "ptz_enabled", config.ptz_enabled);
    cJSON_AddNumberToObject(stream_obj, "ptz_max_x", config.ptz_max_x);
    cJSON_AddNumberToObject(stream_obj, "ptz_max_y", config.ptz_max_y);
    cJSON_AddNumberToObject(stream_obj, "ptz_max_z", config.ptz_max_z);
    cJSON_AddBoolToObject(stream_obj, "ptz_has_home", config.ptz_has_home);
    cJSON_AddStringToObject(stream_obj, "onvif_username", api_onvif_username);
    cJSON_AddStringToObject(stream_obj, "onvif_password", api_onvif_password);
    cJSON_AddStringToObject(stream_obj, "onvif_profile", config.onvif_profile);
    cJSON_AddNumberToObject(stream_obj, "onvif_port", config.onvif_port);
    cJSON_AddBoolToObject(stream_obj, "record_on_schedule", config.record_on_schedule);
    cJSON *schedule_arr_get = cJSON_CreateArray();
    if (schedule_arr_get) {
        for (int j = 0; j < 168; j++) {
            cJSON_AddItemToArray(schedule_arr_get,
                cJSON_CreateBool(config.recording_schedule[j] != 0));
        }
        cJSON_AddItemToObject(stream_obj, "recording_schedule", schedule_arr_get);
    }
    cJSON_AddBoolToObject(stream_obj, "detection_record_on_schedule",
                          config.detection_record_on_schedule);
    cJSON *detection_schedule_arr_get = cJSON_CreateArray();
    if (detection_schedule_arr_get) {
        for (int j = 0; j < 168; j++) {
            cJSON_AddItemToArray(detection_schedule_arr_get,
                cJSON_CreateBool(config.detection_recording_schedule[j] != 0));
        }
        cJSON_AddItemToObject(stream_obj, "detection_recording_schedule",
                              detection_schedule_arr_get);
    }
    cJSON_AddStringToObject(stream_obj, "tags", config.tags);
    cJSON_AddStringToObject(stream_obj, "admin_url",
                            expose_sensitive_config ? config.admin_url : "");
    cJSON_AddBoolToObject(stream_obj, "privacy_mode", config.privacy_mode);
    cJSON_AddBoolToObject(stream_obj, "can_control_privacy", expose_sensitive_config);
    cJSON_AddStringToObject(stream_obj, "motion_trigger_source", config.motion_trigger_source);
    cJSON_AddStringToObject(stream_obj, "go2rtc_source_override",
                            expose_sensitive_config ? config.go2rtc_source_override : "");
    cJSON_AddStringToObject(stream_obj, "sub_stream_url",
                            expose_sensitive_config ? config.sub_stream_url : "");
    cJSON_AddBoolToObject(stream_obj, "has_sub_stream",
                          config.sub_stream_url[0] != '\0');
    cJSON_AddStringToObject(stream_obj, "detection_url",
                            expose_sensitive_config ? config.detection_url : "");
    cJSON_AddStringToObject(stream_obj, "publish_url",
                            expose_sensitive_config ? config.publish_url : "");

    // Get stream status — resolve using UDT state so that go2rtc-managed
    // streams (which stay INACTIVE in the state manager) report accurately.
    const char *status = summary_status(&camera);
    if (camera.health == FLEET_HEALTH_UNKNOWN && config.enabled) {
        stream_status_t stream_status = get_stream_status(stream);
        stream_status = resolve_effective_stream_status(
            stream_status, config.name, config.enabled);
        switch (stream_status) {
            case STREAM_STATUS_STOPPED:       status = "Stopped";      break;
            case STREAM_STATUS_STARTING:      status = "Starting";     break;
            case STREAM_STATUS_RUNNING:       status = "Running";      break;
            case STREAM_STATUS_STOPPING:      status = "Stopping";     break;
            case STREAM_STATUS_ERROR:         status = "Error";        break;
            case STREAM_STATUS_RECONNECTING:  status = "Reconnecting"; break;
            default:                          status = "Unknown";      break;
        }
    }
    cJSON_AddStringToObject(stream_obj, "status", status);

    // Surface the specific cause of an Error state (e.g. failed to load
    // a detection model) so the UI can show it in a tooltip.
    stream_state_manager_t *sm_err = get_stream_state_by_name(config.name);
    if (sm_err && strcmp(status, "Error") == 0) {
        char err_msg[STREAM_ERROR_MESSAGE_MAX];
        pthread_mutex_lock(&sm_err->mutex);
        safe_strcpy(err_msg, sm_err->last_error_message, sizeof(err_msg), 0);
        pthread_mutex_unlock(&sm_err->mutex);
        if (err_msg[0] != '\0') {
            cJSON_AddStringToObject(stream_obj, "error_message", err_msg);
        }
    }

    // Add go2rtc HLS availability - tells frontend whether go2rtc is providing HLS for this stream
    cJSON_AddBoolToObject(stream_obj, "go2rtc_hls_available",
        go2rtc_integration_is_using_go2rtc_for_hls(config.name));

    // Convert to string
    char *json_str = cJSON_PrintUnformatted(stream_obj);
    if (!json_str) {
        log_error("Failed to convert stream JSON to string");
        cJSON_Delete(stream_obj);
        http_response_set_json_error(res, 500, "Failed to convert stream JSON to string");
        return;
    }

    // Send response
    http_response_set_json(res, 200, json_str);

    // Clean up
    free(json_str);
    cJSON_Delete(stream_obj);

    log_info("Successfully handled GET /api/streams/%s request", stream_id);
}

/**
 * @brief Backend-agnostic handler for GET /api/streams/:id/full
 * Returns both stream config and motion recording config in one response
 */
void handle_get_stream_full(const http_request_t *req, http_response_t *res) {
    // Extract stream ID from URL
    char stream_id[MAX_STREAM_NAME];
    if (http_request_extract_path_param(req, "/api/streams/", stream_id, sizeof(stream_id)) != 0) {
        log_error("Failed to extract stream ID from URL");
        http_response_set_json_error(res, 400, "Invalid request path");
        return;
    }

    // If the router matched '/api/streams/#/full', decoded_id may include the trailing segment
    // (e.g., "Cam01/full"). Trim anything after the first '/'.
    char *slash = strchr(stream_id, '/');
    if (slash) {
        *slash = '\0';
    }

    log_info("Handling GET /api/streams/%s/full request", stream_id);

    // Find the stream by name
    stream_handle_t stream = get_stream_by_name(stream_id);
    if (!stream) {
        log_error("Stream not found: %s", stream_id);
        http_response_set_json_error(res, 404, "Stream not found");
        return;
    }

    // Get stream configuration
    stream_config_t config;
    if (get_stream_config(stream, &config) != 0) {
        log_error("Failed to get stream configuration for: %s", stream_id);
        http_response_set_json_error(res, 500, "Failed to get stream configuration");
        return;
    }
    user_t auth_user;
    fleet_camera_t camera;
    authorization_evaluation_t live_evaluation;
    if (!httpd_authorize_stream_action_with_context(
            req, res, AUTHZ_LIVE_VIEW, stream_id, &auth_user, &camera,
            &live_evaluation)) {
        return;
    }
    fleet_camera_enrich_runtime_health(&camera, 1);
    const bool expose_sensitive_config =
        stream_user_can_modify(&auth_user, &camera);

    // Build stream JSON object (same as handle_get_stream)
    cJSON *stream_obj = cJSON_CreateObject();
    if (!stream_obj) {
        log_error("Failed to create stream JSON object");
        http_response_set_json_error(res, 500, "Failed to create stream JSON");
        return;
    }

    char safe_url_full[MAX_URL_LENGTH];
    char api_onvif_username_full[sizeof(config.onvif_username)];
    char api_onvif_password_full[sizeof(config.onvif_password)];
    get_stream_api_credentials(&config, safe_url_full, sizeof(safe_url_full),
                               api_onvif_username_full, sizeof(api_onvif_username_full),
                               api_onvif_password_full, sizeof(api_onvif_password_full),
                               expose_sensitive_config);

    cJSON_AddStringToObject(stream_obj, "camera_uuid", config.camera_uuid);
    cJSON_AddStringToObject(stream_obj, "location_uuid", config.location_uuid);
    cJSON_AddStringToObject(stream_obj, "name", config.name);
    cJSON_AddStringToObject(stream_obj, "url", safe_url_full);
    cJSON_AddBoolToObject(stream_obj, "enabled", config.enabled);
    cJSON_AddBoolToObject(stream_obj, "streaming_enabled", config.streaming_enabled);
    cJSON_AddStringToObject(stream_obj, "playback_transport",
                            playback_transport_is_valid(config.playback_transport)
                                ? config.playback_transport : "auto");
    cJSON_AddStringToObject(stream_obj, "eptz_config", config.eptz_config);
    cJSON_AddNumberToObject(stream_obj, "width", config.width);
    cJSON_AddNumberToObject(stream_obj, "height", config.height);
    cJSON_AddNumberToObject(stream_obj, "fps", config.fps);
    cJSON_AddStringToObject(stream_obj, "codec", config.codec);
    cJSON_AddNumberToObject(stream_obj, "priority", config.priority);
    cJSON_AddBoolToObject(stream_obj, "record", config.record);
    cJSON_AddNumberToObject(stream_obj, "segment_duration", config.segment_duration);

    // Detection settings
    cJSON_AddBoolToObject(stream_obj, "detection_based_recording", config.detection_based_recording);
    cJSON_AddStringToObject(stream_obj, "detection_model", config.detection_model);
    int threshold_percent = (int)(config.detection_threshold * 100.0f);
    cJSON_AddNumberToObject(stream_obj, "detection_threshold", threshold_percent);
    cJSON_AddNumberToObject(stream_obj, "detection_interval", config.detection_interval);
    cJSON_AddNumberToObject(stream_obj, "pre_detection_buffer", config.pre_detection_buffer);
    cJSON_AddNumberToObject(stream_obj, "post_detection_buffer", config.post_detection_buffer);
    cJSON_AddStringToObject(stream_obj, "detection_object_filter", config.detection_object_filter);
    cJSON_AddStringToObject(stream_obj, "detection_object_filter_list", config.detection_object_filter_list);
    cJSON_AddNumberToObject(stream_obj, "protocol", (int)config.protocol);
    cJSON_AddBoolToObject(stream_obj, "record_audio", config.record_audio);
    cJSON_AddBoolToObject(stream_obj, "audio_voice_enhancement", config.audio_voice_enhancement);
    cJSON_AddBoolToObject(stream_obj, "isOnvif", config.is_onvif);
    cJSON_AddBoolToObject(stream_obj, "backchannel_enabled", config.backchannel_enabled);
    cJSON_AddNumberToObject(stream_obj, "retention_days", config.retention_days);
    cJSON_AddNumberToObject(stream_obj, "detection_retention_days", config.detection_retention_days);
    cJSON_AddNumberToObject(stream_obj, "max_storage_mb", config.max_storage_mb);
    cJSON_AddNumberToObject(stream_obj, "tier_critical_multiplier", config.tier_critical_multiplier);
    cJSON_AddNumberToObject(stream_obj, "tier_important_multiplier", config.tier_important_multiplier);
    cJSON_AddNumberToObject(stream_obj, "tier_ephemeral_multiplier", config.tier_ephemeral_multiplier);
    cJSON_AddNumberToObject(stream_obj, "storage_priority", config.storage_priority);
    cJSON_AddBoolToObject(stream_obj, "ptz_enabled", config.ptz_enabled);
    cJSON_AddNumberToObject(stream_obj, "ptz_max_x", config.ptz_max_x);
    cJSON_AddNumberToObject(stream_obj, "ptz_max_y", config.ptz_max_y);
    cJSON_AddNumberToObject(stream_obj, "ptz_max_z", config.ptz_max_z);
    cJSON_AddBoolToObject(stream_obj, "ptz_has_home", config.ptz_has_home);
    cJSON_AddStringToObject(stream_obj, "onvif_username", api_onvif_username_full);
    cJSON_AddStringToObject(stream_obj, "onvif_password", api_onvif_password_full);
    cJSON_AddStringToObject(stream_obj, "onvif_profile", config.onvif_profile);
    cJSON_AddNumberToObject(stream_obj, "onvif_port", config.onvif_port);
    cJSON_AddBoolToObject(stream_obj, "record_on_schedule", config.record_on_schedule);
    cJSON *schedule_arr_full = cJSON_CreateArray();
    if (schedule_arr_full) {
        for (int j = 0; j < 168; j++) {
            cJSON_AddItemToArray(schedule_arr_full,
                cJSON_CreateBool(config.recording_schedule[j] != 0));
        }
        cJSON_AddItemToObject(stream_obj, "recording_schedule", schedule_arr_full);
    }
    cJSON_AddBoolToObject(stream_obj, "detection_record_on_schedule",
                          config.detection_record_on_schedule);
    cJSON *detection_schedule_arr_full = cJSON_CreateArray();
    if (detection_schedule_arr_full) {
        for (int j = 0; j < 168; j++) {
            cJSON_AddItemToArray(detection_schedule_arr_full,
                cJSON_CreateBool(config.detection_recording_schedule[j] != 0));
        }
        cJSON_AddItemToObject(stream_obj, "detection_recording_schedule",
                              detection_schedule_arr_full);
    }
    cJSON_AddStringToObject(stream_obj, "tags", config.tags);
    cJSON_AddStringToObject(stream_obj, "admin_url",
                            expose_sensitive_config ? config.admin_url : "");
    cJSON_AddBoolToObject(stream_obj, "privacy_mode", config.privacy_mode);
    cJSON_AddBoolToObject(stream_obj, "can_control_privacy", expose_sensitive_config);
    cJSON_AddStringToObject(stream_obj, "motion_trigger_source", config.motion_trigger_source);
    cJSON_AddStringToObject(stream_obj, "go2rtc_source_override",
                            expose_sensitive_config ? config.go2rtc_source_override : "");
    cJSON_AddStringToObject(stream_obj, "sub_stream_url",
                            expose_sensitive_config ? config.sub_stream_url : "");
    cJSON_AddBoolToObject(stream_obj, "has_sub_stream",
                          config.sub_stream_url[0] != '\0');
    cJSON_AddStringToObject(stream_obj, "detection_url",
                            expose_sensitive_config ? config.detection_url : "");
    cJSON_AddStringToObject(stream_obj, "publish_url",
                            expose_sensitive_config ? config.publish_url : "");

    // Status — resolve using UDT state for accurate reporting when go2rtc
    // manages the stream (state manager stays INACTIVE/STOPPED at startup).
    const char *status = summary_status(&camera);
    if (camera.health == FLEET_HEALTH_UNKNOWN && config.enabled) {
        stream_status_t stream_status = get_stream_status(stream);
        stream_status = resolve_effective_stream_status(
            stream_status, config.name, config.enabled);
        switch (stream_status) {
            case STREAM_STATUS_STOPPED:       status = "Stopped";      break;
            case STREAM_STATUS_STARTING:      status = "Starting";     break;
            case STREAM_STATUS_RUNNING:       status = "Running";      break;
            case STREAM_STATUS_STOPPING:      status = "Stopping";     break;
            case STREAM_STATUS_ERROR:         status = "Error";        break;
            case STREAM_STATUS_RECONNECTING:  status = "Reconnecting"; break;
            default:                          status = "Unknown";      break;
        }
    }
    cJSON_AddStringToObject(stream_obj, "status", status);

    // Surface the specific cause of an Error state.
    stream_state_manager_t *sm_err2 = get_stream_state_by_name(config.name);
    if (sm_err2 && strcmp(status, "Error") == 0) {
        char err_msg[STREAM_ERROR_MESSAGE_MAX];
        pthread_mutex_lock(&sm_err2->mutex);
        safe_strcpy(err_msg, sm_err2->last_error_message, sizeof(err_msg), 0);
        pthread_mutex_unlock(&sm_err2->mutex);
        if (err_msg[0] != '\0') {
            cJSON_AddStringToObject(stream_obj, "error_message", err_msg);
        }
    }

    // Add go2rtc HLS availability - tells frontend whether go2rtc is providing HLS for this stream
    cJSON_AddBoolToObject(stream_obj, "go2rtc_hls_available",
        go2rtc_integration_is_using_go2rtc_for_hls(config.name));

    // Build response wrapper
    cJSON *response = cJSON_CreateObject();
    if (!response) {
        cJSON_Delete(stream_obj);
        http_response_set_json_error(res, 500, "Failed to create JSON response");
        return;
    }
    cJSON_AddItemToObject(response, "stream", stream_obj);

    char *json_str = cJSON_PrintUnformatted(response);
    if (!json_str) {
        cJSON_Delete(response);
        http_response_set_json_error(res, 500, "Failed to serialize JSON");
        return;
    }

    http_response_set_json(res, 200, json_str);
    free(json_str);
    cJSON_Delete(response);
}
