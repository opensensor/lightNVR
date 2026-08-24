/**
 * @file api_handlers_investigations.c
 * @brief Multi-camera investigation timeline API.
 */

#include <math.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include <cjson/cJSON.h>

#include "web/api_handlers_investigations.h"
#include "web/api_handlers_timeline.h"
#include "web/audit_log.h"
#include "web/httpd_utils.h"
#include "core/authorization.h"
#include "core/config.h"
#define LOG_COMPONENT "InvestigationsAPI"
#include "core/logger.h"
#include "database/db_fleet_query.h"
#include "database/db_investigation_search.h"
#include "database/db_streams.h"
#include "utils/strings.h"

#define INVESTIGATION_MAX_RANGE_SECONDS (31 * 24 * 60 * 60)
#define INVESTIGATION_ACTIVE_DECODER_LIMIT 4
#define INVESTIGATION_DEFAULT_THUMBNAIL_SAMPLES 7
#define INVESTIGATION_MIN_THUMBNAIL_SAMPLES 3
#define INVESTIGATION_MAX_THUMBNAIL_SAMPLES 12

static bool parse_epoch_seconds(const cJSON *body, const char *name,
                                time_t *value) {
    const cJSON *item = cJSON_GetObjectItemCaseSensitive(body, name);
    if (!cJSON_IsNumber(item) || !isfinite(item->valuedouble) ||
        item->valuedouble < 1 || floor(item->valuedouble) != item->valuedouble) {
        return false;
    }
    *value = (time_t)item->valuedouble;
    return true;
}

static const char *segment_capture_method(const timeline_segment_t *segment) {
    if (!segment || segment->trigger_type[0] == '\0') return "scheduled";
    if (strcmp(segment->trigger_type, "scheduled") == 0 &&
        segment->schedule_restricted == 0) {
        return "continuous";
    }
    return segment->trigger_type;
}

static cJSON *segment_json(const timeline_segment_t *segment) {
    cJSON *item = cJSON_CreateObject();
    if (!item) return NULL;
    cJSON_AddNumberToObject(item, "id", (double)segment->id);
    cJSON_AddNumberToObject(item, "start_time",
                            (double)segment->start_time);
    cJSON_AddNumberToObject(item, "end_time", (double)segment->end_time);
    cJSON_AddNumberToObject(item, "duration",
                            (double)(segment->end_time - segment->start_time));
    cJSON_AddStringToObject(item, "capture_method",
                            segment_capture_method(segment));
    cJSON_AddBoolToObject(item, "has_detection", segment->has_detection);
    cJSON_AddBoolToObject(item, "media_available", true);
    return item;
}

static void set_json_response(http_response_t *response, cJSON *root) {
    char *encoded = cJSON_PrintUnformatted(root);
    if (!encoded) {
        http_response_set_json_error(response, 500,
                                     "Failed to encode investigation response");
        return;
    }
    http_response_set_json(response, 200, encoded);
    free(encoded);
}

static int resolve_authorized_cameras(
    const cJSON *body, const http_request_t *request,
    http_response_t *response, stream_config_t *cameras,
    fleet_camera_t *fleet_cameras, user_t *user) {
    const cJSON *camera_uuids =
        cJSON_GetObjectItemCaseSensitive(body, "camera_uuids");
    int camera_count = cJSON_IsArray(camera_uuids)
        ? cJSON_GetArraySize(camera_uuids) : 0;
    if (camera_count < 1 || camera_count > INVESTIGATION_MAX_CAMERAS) {
        http_response_set_json_error(
            response, 400,
            "camera_uuids must contain between 1 and 16 cameras");
        return -1;
    }
    if (!httpd_check_action_access(request, user)) {
        http_response_set_json_error(response, 401, "Unauthorized");
        return -1;
    }

    authorization_context_t *auth_context = authorization_context_create();
    if (!auth_context) {
        http_response_set_json_error(response, 500,
                                     "Authorization context unavailable");
        return -1;
    }
    for (int i = 0; i < camera_count; i++) {
        const cJSON *uuid = cJSON_GetArrayItem(camera_uuids, i);
        if (!cJSON_IsString(uuid) || !uuid->valuestring ||
            strlen(uuid->valuestring) != CAMERA_UUID_STRING_SIZE - 1) {
            authorization_context_free(auth_context);
            http_response_set_json_error(response, 400,
                                         "camera_uuids contains an invalid UUID");
            return -1;
        }
        for (int previous = 0; previous < i; previous++) {
            if (strcmp(cameras[previous].camera_uuid, uuid->valuestring) == 0) {
                authorization_context_free(auth_context);
                http_response_set_json_error(
                    response, 400, "camera_uuids contains duplicates");
                return -1;
            }
        }
        if (get_stream_config_by_uuid(uuid->valuestring, &cameras[i]) != 0 ||
            db_fleet_camera_find_by_name(cameras[i].name,
                                         &fleet_cameras[i]) != 0) {
            authorization_context_free(auth_context);
            http_response_set_json_error(response, 404, "Camera not found");
            return -1;
        }
        authorization_evaluation_t evaluation = {0};
        if (authorization_evaluate_in_context(
                auth_context, user, AUTHZ_RECORDINGS_REPLAY,
                &fleet_cameras[i], &evaluation) != 0) {
            audit_log_authorization(request, user, AUTHZ_RECORDINGS_REPLAY,
                                    &fleet_cameras[i], NULL, "error");
            authorization_context_free(auth_context);
            http_response_set_json_error(
                response, 500, "Authorization policy evaluation failed");
            return -1;
        }
        if (evaluation.decision != AUTHZ_DECISION_ALLOW) {
            audit_log_authorization(request, user, AUTHZ_RECORDINGS_REPLAY,
                                    &fleet_cameras[i], &evaluation, "denied");
            authorization_context_free(auth_context);
            http_response_set_json_error(response, 403, "Forbidden");
            return -1;
        }
    }
    authorization_context_free(auth_context);
    return camera_count;
}

static int resolve_authorized_search_cameras(
    const cJSON *body, const http_request_t *request,
    http_response_t *response, fleet_camera_t *cameras, user_t *user,
    bool *selector_applied) {
    const cJSON *selector_json =
        cJSON_GetObjectItemCaseSensitive(body, "selector");
    const cJSON *camera_uuids =
        cJSON_GetObjectItemCaseSensitive(body, "camera_uuids");
    *selector_applied = selector_json != NULL;
    if (selector_json && camera_uuids) {
        http_response_set_json_error(
            response, 400,
            "Provide either camera_uuids or selector, not both");
        return -1;
    }
    if (!selector_json) {
        stream_config_t explicit_streams[INVESTIGATION_MAX_CAMERAS] = {0};
        fleet_camera_t explicit_cameras[INVESTIGATION_MAX_CAMERAS] = {0};
        int count = resolve_authorized_cameras(
            body, request, response, explicit_streams, explicit_cameras, user);
        if (count < 0) return -1;
        memcpy(cameras, explicit_cameras,
               (size_t)count * sizeof(*explicit_cameras));
        return count;
    }

    if (!httpd_check_action_access(request, user)) {
        http_response_set_json_error(response, 401, "Unauthorized");
        return -1;
    }
    char selector_error[FLEET_SELECTOR_ERROR_MAX] = {0};
    fleet_selector_t *selector = fleet_selector_parse(
        selector_json, selector_error, sizeof(selector_error));
    if (!selector) {
        http_response_set_json_error(
            response, 400,
            selector_error[0] ? selector_error : "Invalid selector");
        return -1;
    }
    fleet_camera_t *inventory = NULL;
    int inventory_count = 0;
    if (db_fleet_camera_load(&inventory, &inventory_count) != 0) {
        fleet_selector_free(selector);
        http_response_set_json_error(response, 500,
                                     "Failed to load fleet cameras");
        return -1;
    }
    authorization_context_t *auth_context = authorization_context_create();
    if (!auth_context) {
        free(inventory);
        fleet_selector_free(selector);
        http_response_set_json_error(response, 500,
                                     "Authorization context unavailable");
        return -1;
    }

    int allowed_count = 0;
    for (int i = 0; i < inventory_count; i++) {
        if (!fleet_selector_matches(selector, &inventory[i], NULL)) continue;
        authorization_evaluation_t evaluation = {0};
        if (authorization_evaluate_in_context(
                auth_context, user, AUTHZ_RECORDINGS_REPLAY,
                &inventory[i], &evaluation) != 0) {
            authorization_context_free(auth_context);
            free(inventory);
            fleet_selector_free(selector);
            http_response_set_json_error(
                response, 500, "Authorization policy evaluation failed");
            return -1;
        }
        /* Broad selectors omit denied cameras before counts or facets. */
        if (evaluation.decision != AUTHZ_DECISION_ALLOW) continue;
        if (allowed_count >= INVESTIGATION_SEARCH_MAX_CAMERAS) {
            authorization_context_free(auth_context);
            free(inventory);
            fleet_selector_free(selector);
            http_response_set_json_error(
                response, 400,
                "selector resolves to more than 64 authorized cameras");
            return -1;
        }
        cameras[allowed_count++] = inventory[i];
    }
    authorization_context_free(auth_context);
    free(inventory);
    fleet_selector_free(selector);
    return allowed_count;
}

void handle_post_investigation_timeline(const http_request_t *request,
                                        http_response_t *response) {
    if (!request || !response) return;

    cJSON *body = httpd_parse_json_body(request);
    if (!body) {
        http_response_set_json_error(response, 400, "Invalid JSON body");
        return;
    }

    time_t start_time = 0;
    time_t end_time = 0;
    if (!parse_epoch_seconds(body, "start_time", &start_time) ||
        !parse_epoch_seconds(body, "end_time", &end_time) ||
        end_time <= start_time ||
        end_time - start_time > INVESTIGATION_MAX_RANGE_SECONDS) {
        cJSON_Delete(body);
        http_response_set_json_error(
            response, 400,
            "start_time and end_time must define a range of at most 31 days");
        return;
    }

    stream_config_t cameras[INVESTIGATION_MAX_CAMERAS];
    fleet_camera_t fleet_cameras[INVESTIGATION_MAX_CAMERAS];
    memset(cameras, 0, sizeof(cameras));
    memset(fleet_cameras, 0, sizeof(fleet_cameras));
    user_t user = {0};
    int camera_count = resolve_authorized_cameras(
        body, request, response, cameras, fleet_cameras, &user);
    if (camera_count < 0) {
        cJSON_Delete(body);
        return;
    }

    cJSON *root = cJSON_CreateObject();
    cJSON *tracks = cJSON_CreateArray();
    if (!root || !tracks) {
        cJSON_Delete(root);
        cJSON_Delete(tracks);
        cJSON_Delete(body);
        http_response_set_json_error(response, 500,
                                     "Failed to create investigation timeline");
        return;
    }
    cJSON_AddNumberToObject(root, "start_time", (double)start_time);
    cJSON_AddNumberToObject(root, "end_time", (double)end_time);
    cJSON_AddNumberToObject(root, "camera_count", camera_count);
    cJSON_AddNumberToObject(root, "max_active_decoders",
                            INVESTIGATION_ACTIVE_DECODER_LIMIT);
    cJSON_AddItemToObject(root, "tracks", tracks);

    timeline_segment_t *segments = calloc(
        INVESTIGATION_MAX_SEGMENTS_PER_CAMERA, sizeof(*segments));
    if (!segments) {
        cJSON_Delete(root);
        cJSON_Delete(body);
        http_response_set_json_error(response, 500,
                                     "Failed to allocate timeline buffer");
        return;
    }

    for (int i = 0; i < camera_count; i++) {
        int segment_count = get_timeline_segments_by_camera_uuid(
            cameras[i].camera_uuid, start_time, end_time, segments,
            INVESTIGATION_MAX_SEGMENTS_PER_CAMERA);
        if (segment_count < 0) {
            free(segments);
            cJSON_Delete(root);
            cJSON_Delete(body);
            http_response_set_json_error(response, 500,
                                         "Failed to query investigation timeline");
            return;
        }

        cJSON *track = cJSON_CreateObject();
        cJSON *track_segments = cJSON_CreateArray();
        cJSON *coverage = cJSON_CreateObject();
        if (!track || !track_segments || !coverage) {
            cJSON_Delete(track);
            cJSON_Delete(track_segments);
            cJSON_Delete(coverage);
            free(segments);
            cJSON_Delete(root);
            cJSON_Delete(body);
            http_response_set_json_error(response, 500,
                                         "Failed to create timeline track");
            return;
        }
        cJSON_AddStringToObject(track, "camera_uuid", cameras[i].camera_uuid);
        cJSON_AddStringToObject(track, "name", cameras[i].name);
        cJSON_AddStringToObject(track, "stream_name", cameras[i].name);
        cJSON_AddNumberToObject(track, "segment_count", segment_count);
        cJSON_AddBoolToObject(
            track, "truncated",
            segment_count == INVESTIGATION_MAX_SEGMENTS_PER_CAMERA);
        cJSON_AddBoolToObject(coverage, "identity_resolved", true);
        cJSON_AddNumberToObject(coverage, "requested_start",
                                (double)start_time);
        cJSON_AddNumberToObject(coverage, "requested_end", (double)end_time);
        cJSON_AddItemToObject(track, "coverage", coverage);
        cJSON_AddItemToObject(track, "segments", track_segments);

        for (int j = 0; j < segment_count; j++) {
            cJSON *item = segment_json(&segments[j]);
            if (item) cJSON_AddItemToArray(track_segments, item);
        }
        cJSON_AddItemToArray(tracks, track);
    }

    free(segments);
    set_json_response(response, root);
    cJSON_Delete(root);
    cJSON_Delete(body);
}

void handle_post_investigation_thumbnail_samples(
    const http_request_t *request, http_response_t *response) {
    if (!request || !response) return;

    cJSON *body = httpd_parse_json_body(request);
    if (!body) {
        http_response_set_json_error(response, 400, "Invalid JSON body");
        return;
    }
    const cJSON *camera_uuids =
        cJSON_GetObjectItemCaseSensitive(body, "camera_uuids");
    if (!cJSON_IsArray(camera_uuids) ||
        cJSON_GetArraySize(camera_uuids) != 1) {
        cJSON_Delete(body);
        http_response_set_json_error(
            response, 400,
            "camera_uuids must contain exactly one camera");
        return;
    }

    time_t start_time = 0;
    time_t end_time = 0;
    if (!parse_epoch_seconds(body, "start_time", &start_time) ||
        !parse_epoch_seconds(body, "end_time", &end_time) ||
        end_time <= start_time ||
        end_time - start_time > INVESTIGATION_MAX_RANGE_SECONDS) {
        cJSON_Delete(body);
        http_response_set_json_error(
            response, 400,
            "start_time and end_time must define a range of at most 31 days");
        return;
    }

    int requested_count = INVESTIGATION_DEFAULT_THUMBNAIL_SAMPLES;
    const cJSON *sample_count =
        cJSON_GetObjectItemCaseSensitive(body, "sample_count");
    if (sample_count) {
        if (!cJSON_IsNumber(sample_count) ||
            !isfinite(sample_count->valuedouble) ||
            floor(sample_count->valuedouble) != sample_count->valuedouble ||
            sample_count->valuedouble < INVESTIGATION_MIN_THUMBNAIL_SAMPLES ||
            sample_count->valuedouble > INVESTIGATION_MAX_THUMBNAIL_SAMPLES) {
            cJSON_Delete(body);
            http_response_set_json_error(
                response, 400, "sample_count must be an integer from 3 to 12");
            return;
        }
        requested_count = sample_count->valueint;
    }

    stream_config_t cameras[INVESTIGATION_MAX_CAMERAS] = {0};
    fleet_camera_t fleet_cameras[INVESTIGATION_MAX_CAMERAS] = {0};
    user_t user = {0};
    int camera_count = resolve_authorized_cameras(
        body, request, response, cameras, fleet_cameras, &user);
    if (camera_count < 0) {
        cJSON_Delete(body);
        return;
    }

    timeline_segment_t *segments = calloc(
        INVESTIGATION_MAX_SEGMENTS_PER_CAMERA, sizeof(*segments));
    if (!segments) {
        cJSON_Delete(body);
        http_response_set_json_error(response, 500,
                                     "Failed to allocate thumbnail samples");
        return;
    }
    int segment_count = get_timeline_segments_by_camera_uuid(
        cameras[0].camera_uuid, start_time, end_time, segments,
        INVESTIGATION_MAX_SEGMENTS_PER_CAMERA);
    if (segment_count < 0) {
        free(segments);
        cJSON_Delete(body);
        http_response_set_json_error(response, 500,
                                     "Failed to query recording coverage");
        return;
    }

    int actual_count = requested_count;
    time_t range_seconds = end_time - start_time;
    if ((time_t)(actual_count - 1) > range_seconds) {
        actual_count = (int)range_seconds + 1;
    }
    cJSON *root = cJSON_CreateObject();
    cJSON *camera = cJSON_CreateObject();
    cJSON *samples = cJSON_CreateArray();
    cJSON *coverage = cJSON_CreateObject();
    if (!root || !camera || !samples || !coverage) {
        cJSON_Delete(root);
        cJSON_Delete(camera);
        cJSON_Delete(samples);
        cJSON_Delete(coverage);
        free(segments);
        cJSON_Delete(body);
        http_response_set_json_error(response, 500,
                                     "Failed to create thumbnail samples");
        return;
    }

    cJSON_AddNumberToObject(root, "start_time", (double)start_time);
    cJSON_AddNumberToObject(root, "end_time", (double)end_time);
    cJSON_AddNumberToObject(root, "sample_count", actual_count);
    cJSON_AddStringToObject(camera, "camera_uuid", cameras[0].camera_uuid);
    cJSON_AddStringToObject(camera, "name", cameras[0].name);
    cJSON_AddItemToObject(root, "camera", camera);
    cJSON_AddItemToObject(root, "samples", samples);
    cJSON_AddItemToObject(root, "coverage", coverage);

    int available_count = 0;
    for (int i = 0; i < actual_count; i++) {
        time_t timestamp = start_time;
        if (actual_count > 1) {
            timestamp += (time_t)(((int64_t)range_seconds * i) /
                                  (actual_count - 1));
        }
        const timeline_segment_t *match = NULL;
        for (int j = 0; j < segment_count; j++) {
            if (timestamp >= segments[j].start_time &&
                timestamp <= segments[j].end_time) {
                match = &segments[j];
                break;
            }
        }

        cJSON *sample = cJSON_CreateObject();
        cJSON *thumbnail = cJSON_CreateObject();
        if (!sample || !thumbnail) {
            cJSON_Delete(sample);
            cJSON_Delete(thumbnail);
            cJSON_Delete(root);
            free(segments);
            cJSON_Delete(body);
            http_response_set_json_error(
                response, 500, "Failed to create thumbnail samples");
            return;
        }
        cJSON_AddNumberToObject(sample, "timestamp", (double)timestamp);
        cJSON_AddItemToObject(sample, "thumbnail", thumbnail);
        if (!match) {
            cJSON_AddStringToObject(sample, "media_status", "gap");
            cJSON_AddStringToObject(thumbnail, "status", "unavailable");
            cJSON_AddItemToArray(samples, sample);
            continue;
        }

        int64_t offset_ms =
            (int64_t)(timestamp - match->start_time) * 1000;
        char thumbnail_url[160];
        snprintf(thumbnail_url, sizeof(thumbnail_url),
                 "/api/investigations/thumbnail/%llu/%lld",
                 (unsigned long long)match->id, (long long)offset_ms);
        cJSON_AddStringToObject(sample, "media_status", "available");
        cJSON_AddNumberToObject(sample, "recording_id", (double)match->id);
        cJSON_AddNumberToObject(sample, "recording_start_time",
                                (double)match->start_time);
        cJSON_AddNumberToObject(sample, "recording_end_time",
                                (double)match->end_time);
        cJSON_AddNumberToObject(sample, "offset_ms", (double)offset_ms);
        // Mirror the drill-down endpoint's own guards: CPU-save mode (#364)
        // rejects generation, so advertising a URL here would just hand the UI
        // a set of images that 403.
        if (g_config.generate_thumbnails &&
            g_config.thumbnails_per_recording > 1) {
            cJSON_AddStringToObject(thumbnail, "status", "available");
            cJSON_AddStringToObject(thumbnail, "url", thumbnail_url);
        } else {
            cJSON_AddStringToObject(thumbnail, "status", "disabled");
        }
        cJSON_AddItemToArray(samples, sample);
        available_count++;
    }

    cJSON_AddBoolToObject(
        coverage, "segments_truncated",
        segment_count == INVESTIGATION_MAX_SEGMENTS_PER_CAMERA);
    cJSON_AddNumberToObject(coverage, "available_samples", available_count);
    cJSON_AddNumberToObject(coverage, "gap_samples",
                            actual_count - available_count);

    set_json_response(response, root);
    cJSON_Delete(root);
    free(segments);
    cJSON_Delete(body);
}

static bool parse_filter_strings(
    const cJSON *filters, const char *name, char *output, size_t stride,
    int max_count, int *count) {
    *count = 0;
    if (!filters) return true;
    const cJSON *array = cJSON_GetObjectItemCaseSensitive(filters, name);
    if (!array) return true;
    if (!cJSON_IsArray(array) || cJSON_GetArraySize(array) > max_count) {
        return false;
    }
    cJSON *item = NULL;
    cJSON_ArrayForEach(item, array) {
        if (!cJSON_IsString(item) || !item->valuestring ||
            item->valuestring[0] == '\0' ||
            strlen(item->valuestring) >= stride) {
            return false;
        }
        for (const unsigned char *cursor =
                 (const unsigned char *)item->valuestring;
             *cursor; cursor++) {
            if (*cursor < 0x20 || *cursor == 0x7f) return false;
        }
        for (int previous = 0; previous < *count; previous++) {
            if (strcmp(output + ((size_t)previous * stride),
                       item->valuestring) == 0) {
                return false;
            }
        }
        safe_strcpy(output + ((size_t)(*count) * stride), item->valuestring,
                    stride, 0);
        (*count)++;
    }
    return true;
}

static bool parse_confidence_filter(const cJSON *filters, const char *name,
                                    bool *present, double *value) {
    *present = false;
    if (!filters) return true;
    const cJSON *item = cJSON_GetObjectItemCaseSensitive(filters, name);
    if (!item) return true;
    if (!cJSON_IsNumber(item) || !isfinite(item->valuedouble) ||
        item->valuedouble < 0.0 || item->valuedouble > 1.0) {
        return false;
    }
    *present = true;
    *value = item->valuedouble;
    return true;
}

static bool filter_values_allowed(
    char values[][INVESTIGATION_SEARCH_VALUE_MAX], int count,
    const char *const *allowed, int allowed_count) {
    for (int i = 0; i < count; i++) {
        bool found = false;
        for (int j = 0; j < allowed_count; j++) {
            if (strcmp(values[i], allowed[j]) == 0) {
                found = true;
                break;
            }
        }
        if (!found) return false;
    }
    return true;
}

static bool parse_protected_filter(const cJSON *filters, int *value) {
    *value = -1;
    if (!filters) return true;
    const cJSON *item =
        cJSON_GetObjectItemCaseSensitive(filters, "protected");
    if (!item) return true;
    if (!cJSON_IsBool(item)) return false;
    *value = cJSON_IsTrue(item) ? 1 : 0;
    return true;
}

static bool parse_region_filter(const cJSON *body,
                                investigation_search_query_t *query) {
    const cJSON *region =
        cJSON_GetObjectItemCaseSensitive(body, "region");
    if (!region) return true;
    if (!cJSON_IsObject(region)) return false;

    const cJSON *camera_uuid =
        cJSON_GetObjectItemCaseSensitive(region, "camera_uuid");
    const cJSON *x = cJSON_GetObjectItemCaseSensitive(region, "x");
    const cJSON *y = cJSON_GetObjectItemCaseSensitive(region, "y");
    const cJSON *width =
        cJSON_GetObjectItemCaseSensitive(region, "width");
    const cJSON *height =
        cJSON_GetObjectItemCaseSensitive(region, "height");
    const cJSON *match =
        cJSON_GetObjectItemCaseSensitive(region, "match");
    const cJSON *minimum =
        cJSON_GetObjectItemCaseSensitive(region, "min_intersection");
    if (!cJSON_IsString(camera_uuid) || !camera_uuid->valuestring ||
        strlen(camera_uuid->valuestring) != CAMERA_UUID_STRING_SIZE - 1 ||
        !cJSON_IsNumber(x) || !isfinite(x->valuedouble) ||
        !cJSON_IsNumber(y) || !isfinite(y->valuedouble) ||
        !cJSON_IsNumber(width) || !isfinite(width->valuedouble) ||
        !cJSON_IsNumber(height) || !isfinite(height->valuedouble) ||
        x->valuedouble < 0.0 || y->valuedouble < 0.0 ||
        width->valuedouble <= 0.0 || height->valuedouble <= 0.0 ||
        x->valuedouble + width->valuedouble > 1.0 ||
        y->valuedouble + height->valuedouble > 1.0 ||
        (match && (!cJSON_IsString(match) || !match->valuestring))) {
        return false;
    }

    const char *match_value = match ? match->valuestring : "center";
    investigation_region_match_t match_type = INVESTIGATION_REGION_NONE;
    if (strcmp(match_value, "center") == 0) {
        match_type = INVESTIGATION_REGION_CENTER;
    } else if (strcmp(match_value, "intersects") == 0) {
        match_type = INVESTIGATION_REGION_INTERSECTS;
    } else if (strcmp(match_value, "minimum_intersection") == 0) {
        match_type = INVESTIGATION_REGION_MIN_INTERSECTION;
    } else {
        return false;
    }
    double min_intersection = 0.25;
    if (minimum) {
        if (!cJSON_IsNumber(minimum) || !isfinite(minimum->valuedouble) ||
            minimum->valuedouble <= 0.0 || minimum->valuedouble > 1.0) {
            return false;
        }
        min_intersection = minimum->valuedouble;
    }

    query->has_region = true;
    safe_strcpy(query->region_camera_uuid, camera_uuid->valuestring,
                sizeof(query->region_camera_uuid), 0);
    query->region_x = x->valuedouble;
    query->region_y = y->valuedouble;
    query->region_width = width->valuedouble;
    query->region_height = height->valuedouble;
    query->region_match = match_type;
    query->region_min_intersection = min_intersection;
    return true;
}

static const char *region_match_name(investigation_region_match_t match) {
    switch (match) {
        case INVESTIGATION_REGION_CENTER:
            return "center";
        case INVESTIGATION_REGION_INTERSECTS:
            return "intersects";
        case INVESTIGATION_REGION_MIN_INTERSECTION:
            return "minimum_intersection";
        case INVESTIGATION_REGION_NONE:
            return "none";
    }
    return "none";
}

static bool camera_matches_location_filters(
    const fleet_camera_t *camera,
    char locations[][INVESTIGATION_SEARCH_VALUE_MAX], int location_count) {
    if (location_count == 0) return true;
    const char *value = camera->location_uuid[0] != '\0'
        ? camera->location_uuid : "unassigned";
    for (int i = 0; i < location_count; i++) {
        if (strcmp(value, locations[i]) == 0) return true;
    }
    return false;
}

static bool parse_search_cursor(const cJSON *body,
                                investigation_search_query_t *query) {
    const cJSON *cursor = cJSON_GetObjectItemCaseSensitive(body, "cursor");
    if (!cursor || cJSON_IsNull(cursor)) return true;
    if (!cJSON_IsString(cursor) || !cursor->valuestring) return false;
    unsigned long long timestamp = 0;
    unsigned long long id = 0;
    char trailing = '\0';
    if (sscanf(cursor->valuestring, "v1-%llx-%llx%c", &timestamp, &id,
               &trailing) != 2 || timestamp == 0 || id == 0 ||
        timestamp > INT64_MAX || id > INT64_MAX) {
        return false;
    }
    query->has_cursor = true;
    query->cursor_timestamp = (time_t)timestamp;
    query->cursor_id = (uint64_t)id;
    return true;
}

static void add_facet_array(cJSON *facets, const char *name,
                            const investigation_search_facet_t *values,
                            int count, const fleet_camera_t *cameras,
                            int camera_count) {
    cJSON *array = cJSON_AddArrayToObject(facets, name);
    if (!array) return;
    for (int i = 0; i < count; i++) {
        cJSON *item = cJSON_CreateObject();
        if (!item) continue;
        cJSON_AddStringToObject(item, "value", values[i].value);
        if (cameras) {
            const char *label = values[i].value;
            for (int camera_index = 0; camera_index < camera_count;
                 camera_index++) {
                if (strcmp(cameras[camera_index].camera_uuid,
                           values[i].value) == 0) {
                    label = cameras[camera_index].name;
                    break;
                }
            }
            cJSON_AddStringToObject(item, "label", label);
        }
        cJSON_AddNumberToObject(item, "count", (double)values[i].count);
        cJSON_AddItemToArray(array, item);
    }
}

static const fleet_camera_t *find_camera_context(
    const fleet_camera_t *cameras, int camera_count, const char *camera_uuid);

static void add_location_facet_array(
    cJSON *facets, const investigation_search_facet_t *camera_values,
    int camera_value_count, const fleet_camera_t *cameras,
    int camera_count) {
    investigation_search_facet_t values[INVESTIGATION_SEARCH_MAX_FACETS] = {0};
    char labels[INVESTIGATION_SEARCH_MAX_FACETS]
               [FLEET_LOCATION_PATH_MAX] = {{0}};
    int count = 0;
    for (int i = 0; i < camera_value_count; i++) {
        const fleet_camera_t *camera = find_camera_context(
            cameras, camera_count, camera_values[i].value);
        if (!camera) continue;
        const char *value = camera->location_uuid[0] != '\0'
            ? camera->location_uuid : "unassigned";
        const char *label = camera->location_path[0] != '\0'
            ? camera->location_path : "Unassigned";
        int index = -1;
        for (int j = 0; j < count; j++) {
            if (strcmp(values[j].value, value) == 0) {
                index = j;
                break;
            }
        }
        if (index < 0 && count < INVESTIGATION_SEARCH_MAX_FACETS) {
            index = count++;
            safe_strcpy(values[index].value, value,
                        sizeof(values[index].value), 0);
            safe_strcpy(labels[index], label, sizeof(labels[index]), 0);
        }
        if (index >= 0) values[index].count += camera_values[i].count;
    }

    cJSON *array = cJSON_AddArrayToObject(facets, "locations");
    if (!array) return;
    for (int i = 0; i < count; i++) {
        cJSON *item = cJSON_CreateObject();
        if (!item) continue;
        cJSON_AddStringToObject(item, "value", values[i].value);
        cJSON_AddStringToObject(item, "label", labels[i]);
        cJSON_AddNumberToObject(item, "count", (double)values[i].count);
        cJSON_AddItemToArray(array, item);
    }
}

static const fleet_camera_t *find_camera_context(
    const fleet_camera_t *cameras, int camera_count, const char *camera_uuid) {
    for (int i = 0; i < camera_count; i++) {
        if (strcmp(cameras[i].camera_uuid, camera_uuid) == 0) {
            return &cameras[i];
        }
    }
    return NULL;
}

void handle_post_investigation_search(const http_request_t *request,
                                      http_response_t *response) {
    if (!request || !response) return;
    cJSON *body = httpd_parse_json_body(request);
    if (!body) {
        http_response_set_json_error(response, 400, "Invalid JSON body");
        return;
    }

    investigation_search_query_t query = {.protected_filter = -1};
    if (!parse_epoch_seconds(body, "start_time", &query.start_time) ||
        !parse_epoch_seconds(body, "end_time", &query.end_time) ||
        query.end_time <= query.start_time ||
        query.end_time - query.start_time > INVESTIGATION_MAX_RANGE_SECONDS) {
        cJSON_Delete(body);
        http_response_set_json_error(
            response, 400,
            "start_time and end_time must define a range of at most 31 days");
        return;
    }

    fleet_camera_t fleet_cameras[INVESTIGATION_SEARCH_MAX_CAMERAS] = {0};
    user_t user = {0};
    bool selector_applied = false;
    int camera_count = resolve_authorized_search_cameras(
        body, request, response, fleet_cameras, &user, &selector_applied);
    if (camera_count < 0) {
        cJSON_Delete(body);
        return;
    }

    const cJSON *filters = cJSON_GetObjectItemCaseSensitive(body, "filters");
    if (filters && !cJSON_IsObject(filters)) {
        cJSON_Delete(body);
        http_response_set_json_error(response, 400,
                                     "filters must be an object");
        return;
    }
    char locations[INVESTIGATION_SEARCH_MAX_FILTER_VALUES]
                  [INVESTIGATION_SEARCH_VALUE_MAX] = {{0}};
    int location_count = 0;
    static const char *const allowed_event_types[] = {
        "detection", "motion"
    };
    static const char *const allowed_capture_methods[] = {
        "continuous", "scheduled", "detection", "motion", "manual"
    };
    if (!parse_filter_strings(
            filters, "labels", (char *)query.labels,
            sizeof(query.labels[0]), INVESTIGATION_SEARCH_MAX_FILTER_VALUES,
            &query.label_count) ||
        !parse_filter_strings(
            filters, "zones", (char *)query.zones,
            sizeof(query.zones[0]), INVESTIGATION_SEARCH_MAX_FILTER_VALUES,
            &query.zone_count) ||
        !parse_filter_strings(
            filters, "sources", (char *)query.sources,
            sizeof(query.sources[0]), INVESTIGATION_SEARCH_MAX_FILTER_VALUES,
            &query.source_count) ||
        !parse_filter_strings(
            filters, "event_types", (char *)query.event_types,
            sizeof(query.event_types[0]),
            INVESTIGATION_SEARCH_MAX_FILTER_VALUES,
            &query.event_type_count) ||
        !parse_filter_strings(
            filters, "capture_methods", (char *)query.capture_methods,
            sizeof(query.capture_methods[0]),
            INVESTIGATION_SEARCH_MAX_FILTER_VALUES,
            &query.capture_method_count) ||
        !parse_filter_strings(
            filters, "recording_tags", (char *)query.recording_tags,
            sizeof(query.recording_tags[0]),
            INVESTIGATION_SEARCH_MAX_FILTER_VALUES,
            &query.recording_tag_count) ||
        !parse_filter_strings(
            filters, "locations", (char *)locations,
            sizeof(locations[0]), INVESTIGATION_SEARCH_MAX_FILTER_VALUES,
            &location_count) ||
        !parse_confidence_filter(filters, "min_confidence",
                                 &query.has_min_confidence,
                                 &query.min_confidence) ||
        !parse_confidence_filter(filters, "max_confidence",
                                 &query.has_max_confidence,
                                 &query.max_confidence) ||
        !parse_protected_filter(filters, &query.protected_filter) ||
        !filter_values_allowed(
            query.event_types, query.event_type_count, allowed_event_types,
            (int)(sizeof(allowed_event_types) /
                  sizeof(allowed_event_types[0]))) ||
        !filter_values_allowed(
            query.capture_methods, query.capture_method_count,
            allowed_capture_methods,
            (int)(sizeof(allowed_capture_methods) /
                  sizeof(allowed_capture_methods[0]))) ||
        (query.has_min_confidence && query.has_max_confidence &&
         query.min_confidence > query.max_confidence) ||
        !parse_region_filter(body, &query) ||
        !parse_search_cursor(body, &query)) {
        cJSON_Delete(body);
        http_response_set_json_error(response, 400,
                                     "Invalid investigation search filters");
        return;
    }

    int filtered_camera_count = 0;
    for (int i = 0; i < camera_count; i++) {
        if (!camera_matches_location_filters(
                &fleet_cameras[i], locations, location_count)) {
            continue;
        }
        if (filtered_camera_count != i) {
            fleet_cameras[filtered_camera_count] = fleet_cameras[i];
        }
        filtered_camera_count++;
    }
    camera_count = filtered_camera_count;
    if (query.has_region &&
        !find_camera_context(fleet_cameras, camera_count,
                             query.region_camera_uuid)) {
        cJSON_Delete(body);
        http_response_set_json_error(
            response, 400,
            "region camera must be in the authorized search scope");
        return;
    }
    query.camera_count = camera_count;
    for (int i = 0; i < camera_count; i++) {
        safe_strcpy(query.camera_uuids[i], fleet_cameras[i].camera_uuid,
                    sizeof(query.camera_uuids[i]), 0);
        safe_strcpy(query.legacy_stream_names[i], fleet_cameras[i].name,
                    sizeof(query.legacy_stream_names[i]), 0);
    }

    query.limit = 100;
    const cJSON *limit = cJSON_GetObjectItemCaseSensitive(body, "limit");
    if (limit) {
        if (!cJSON_IsNumber(limit) || !isfinite(limit->valuedouble) ||
            floor(limit->valuedouble) != limit->valuedouble ||
            limit->valueint < 1 ||
            limit->valueint > INVESTIGATION_SEARCH_MAX_RESULTS) {
            cJSON_Delete(body);
            http_response_set_json_error(response, 400,
                                         "limit must be between 1 and 500");
            return;
        }
        query.limit = limit->valueint;
    }

    investigation_search_result_t *results = calloc(
        (size_t)query.limit, sizeof(*results));
    investigation_search_summary_t *summary = calloc(1, sizeof(*summary));
    if (!results || !summary) {
        free(results);
        free(summary);
        cJSON_Delete(body);
        http_response_set_json_error(response, 500,
                                     "Failed to allocate search response");
        return;
    }
    if (query.camera_count > 0 &&
        db_investigation_search(&query, results, summary) != 0) {
        free(results);
        free(summary);
        cJSON_Delete(body);
        http_response_set_json_error(response, 500,
                                     "Investigation search failed");
        return;
    }
    if (query.camera_count == 0) {
        int64_t range = (int64_t)query.end_time -
                        (int64_t)query.start_time + 1;
        summary->histogram_bucket_seconds = (int)(
            (range + INVESTIGATION_SEARCH_MAX_HISTOGRAM_BUCKETS - 1) /
            INVESTIGATION_SEARCH_MAX_HISTOGRAM_BUCKETS);
    }

    cJSON *root = cJSON_CreateObject();
    cJSON *items = root ? cJSON_AddArrayToObject(root, "results") : NULL;
    if (!root || !items) {
        cJSON_Delete(root);
        free(results);
        free(summary);
        cJSON_Delete(body);
        http_response_set_json_error(response, 500,
                                     "Failed to create search response");
        return;
    }
    cJSON_AddNumberToObject(root, "camera_count", camera_count);
    cJSON_AddBoolToObject(root, "selector_applied", selector_applied);
    for (int i = 0; i < summary->result_count; i++) {
        const investigation_search_result_t *result = &results[i];
        const fleet_camera_t *camera = find_camera_context(
            fleet_cameras, camera_count, result->camera_uuid);
        cJSON *item = cJSON_CreateObject();
        cJSON *camera_context = cJSON_CreateObject();
        cJSON *detection = cJSON_CreateObject();
        cJSON *thumbnail = cJSON_CreateObject();
        cJSON *recording = cJSON_CreateObject();
        if (!item || !camera_context || !detection || !thumbnail ||
            !recording) {
            cJSON_Delete(item);
            cJSON_Delete(camera_context);
            cJSON_Delete(detection);
            cJSON_Delete(thumbnail);
            cJSON_Delete(recording);
            continue;
        }
        char result_id[64];
        snprintf(result_id, sizeof(result_id), "detection:%llu",
                 (unsigned long long)result->detection_id);
        cJSON_AddStringToObject(item, "result_id", result_id);
        cJSON_AddStringToObject(item, "event_type",
            strcmp(result->source, "external_motion") == 0
                ? "motion" : "detection");
        cJSON_AddStringToObject(item, "camera_uuid", result->camera_uuid);
        cJSON_AddNumberToObject(item, "start_time", (double)result->start_time);
        cJSON_AddNumberToObject(item, "end_time", (double)result->end_time);
        cJSON_AddBoolToObject(item, "media_available",
                              result->media_available);
        cJSON_AddBoolToObject(item, "known_gap", !result->media_available);
        if (result->recording_id > 0) {
            cJSON_AddNumberToObject(item, "recording_id",
                                    (double)result->recording_id);
            char thumbnail_url[128];
            snprintf(thumbnail_url, sizeof(thumbnail_url),
                     "/api/recordings/thumbnail/%llu/0",
                     (unsigned long long)result->recording_id);
            cJSON_AddStringToObject(thumbnail, "status",
                result->media_available ? "available" : "unavailable");
            if (result->media_available) {
                cJSON_AddStringToObject(thumbnail, "url", thumbnail_url);
            }
            cJSON_AddStringToObject(recording, "capture_method",
                                    result->capture_method);
            cJSON_AddBoolToObject(recording, "protected",
                                  result->recording_protected);
        } else {
            cJSON_AddNullToObject(item, "recording_id");
            cJSON_AddStringToObject(thumbnail, "status", "unavailable");
            cJSON_AddNullToObject(recording, "capture_method");
            cJSON_AddNullToObject(recording, "protected");
        }
        cJSON_AddStringToObject(camera_context, "name",
                                camera ? camera->name : result->legacy_stream_name);
        if (camera && camera->location_uuid[0] != '\0') {
            cJSON_AddStringToObject(camera_context, "location_uuid",
                                    camera->location_uuid);
            cJSON_AddStringToObject(camera_context, "location_name",
                                    camera->location_name);
            cJSON_AddStringToObject(camera_context, "location_path",
                                    camera->location_path);
        } else {
            cJSON_AddNullToObject(camera_context, "location_uuid");
            cJSON_AddNullToObject(camera_context, "location_name");
            cJSON_AddNullToObject(camera_context, "location_path");
        }
        cJSON_AddNumberToObject(detection, "id",
                                (double)result->detection_id);
        cJSON_AddStringToObject(detection, "label", result->label);
        cJSON_AddNumberToObject(detection, "confidence", result->confidence);
        cJSON_AddStringToObject(detection, "source", result->source);
        if (result->zone_uuid[0] != '\0') {
            cJSON_AddStringToObject(detection, "zone_uuid", result->zone_uuid);
        } else {
            cJSON_AddNullToObject(detection, "zone_uuid");
        }
        if (result->track_id >= 0) {
            cJSON_AddNumberToObject(detection, "track_id", result->track_id);
        } else {
            cJSON_AddNullToObject(detection, "track_id");
        }
        if (result->has_box) {
            cJSON *box = cJSON_AddObjectToObject(detection, "bounding_box");
            if (box) {
                cJSON_AddNumberToObject(box, "x", result->x);
                cJSON_AddNumberToObject(box, "y", result->y);
                cJSON_AddNumberToObject(box, "width", result->width);
                cJSON_AddNumberToObject(box, "height", result->height);
                cJSON_AddBoolToObject(box, "normalized", true);
            }
        } else {
            cJSON_AddNullToObject(detection, "bounding_box");
        }
        cJSON_AddItemToObject(item, "camera", camera_context);
        cJSON_AddItemToObject(item, "detection", detection);
        cJSON_AddItemToObject(item, "thumbnail", thumbnail);
        cJSON_AddItemToObject(item, "recording", recording);
        cJSON_AddItemToArray(items, item);
    }

    cJSON *page = cJSON_AddObjectToObject(root, "page");
    cJSON_AddNumberToObject(page, "limit", query.limit);
    cJSON_AddNumberToObject(page, "returned", summary->result_count);
    cJSON_AddNumberToObject(page, "total", (double)summary->total_count);
    cJSON_AddBoolToObject(page, "has_more", summary->has_more);
    if (summary->has_more && summary->result_count > 0) {
        const investigation_search_result_t *last =
            &results[summary->result_count - 1];
        char next_cursor[64];
        snprintf(next_cursor, sizeof(next_cursor), "v1-%016llx-%016llx",
                 (unsigned long long)last->start_time,
                 (unsigned long long)last->detection_id);
        cJSON_AddStringToObject(page, "next_cursor", next_cursor);
    } else {
        cJSON_AddNullToObject(page, "next_cursor");
    }

    cJSON *facets = cJSON_AddObjectToObject(root, "facets");
    add_facet_array(facets, "cameras", summary->facets.cameras,
                    summary->facets.camera_count, fleet_cameras, camera_count);
    add_facet_array(facets, "labels", summary->facets.labels,
                    summary->facets.label_count, NULL, 0);
    add_facet_array(facets, "zones", summary->facets.zones,
                    summary->facets.zone_count, NULL, 0);
    add_facet_array(facets, "sources", summary->facets.sources,
                    summary->facets.source_count, NULL, 0);
    add_facet_array(facets, "event_types", summary->facets.event_types,
                    summary->facets.event_type_count, NULL, 0);
    add_location_facet_array(
        facets, summary->facets.cameras, summary->facets.camera_count,
        fleet_cameras, camera_count);
    add_facet_array(facets, "capture_methods",
                    summary->facets.capture_methods,
                    summary->facets.capture_method_count, NULL, 0);
    add_facet_array(facets, "recording_tags",
                    summary->facets.recording_tags,
                    summary->facets.recording_tag_count, NULL, 0);
    add_facet_array(facets, "protection", summary->facets.protection,
                    summary->facets.protection_count, NULL, 0);

    cJSON *histogram = cJSON_AddObjectToObject(root, "histogram");
    cJSON_AddNumberToObject(histogram, "bucket_seconds",
                            summary->histogram_bucket_seconds);
    cJSON *buckets = cJSON_AddArrayToObject(histogram, "buckets");
    for (int i = 0; i < summary->histogram_count; i++) {
        cJSON *bucket = cJSON_CreateObject();
        if (!bucket) continue;
        cJSON_AddNumberToObject(bucket, "start_time",
            (double)summary->histogram[i].start_time);
        cJSON_AddNumberToObject(bucket, "end_time",
            (double)summary->histogram[i].end_time);
        cJSON_AddNumberToObject(bucket, "count",
            (double)summary->histogram[i].count);
        cJSON_AddItemToArray(buckets, bucket);
    }

    cJSON *coverage = cJSON_AddObjectToObject(root, "coverage");
    cJSON_AddNumberToObject(coverage, "requested_start",
                            (double)query.start_time);
    cJSON_AddNumberToObject(coverage, "requested_end",
                            (double)query.end_time);
    cJSON_AddNumberToObject(coverage, "searched_start",
                            (double)query.start_time);
    cJSON_AddNumberToObject(coverage, "searched_end",
                            (double)query.end_time);
    cJSON_AddNumberToObject(coverage, "unresolved_legacy_rows",
                            (double)summary->unresolved_legacy_count);
    cJSON *spatial = cJSON_AddObjectToObject(coverage, "spatial_metadata");
    cJSON_AddBoolToObject(spatial, "requested", query.has_region);
    if (query.has_region) {
        cJSON_AddStringToObject(spatial, "search_type", "metadata_search");
        cJSON_AddStringToObject(spatial, "camera_uuid",
                                query.region_camera_uuid);
        cJSON_AddStringToObject(spatial, "match",
                                region_match_name(query.region_match));
        cJSON_AddNumberToObject(spatial, "rows_with_boxes",
                                (double)summary->spatial_metadata_rows);
        cJSON_AddNumberToObject(spatial, "rows_without_boxes",
                                (double)summary->spatial_missing_rows);
        cJSON_AddBoolToObject(spatial, "complete",
                              summary->spatial_missing_rows == 0);
        cJSON *rectangle = cJSON_AddObjectToObject(spatial, "rectangle");
        cJSON_AddNumberToObject(rectangle, "x", query.region_x);
        cJSON_AddNumberToObject(rectangle, "y", query.region_y);
        cJSON_AddNumberToObject(rectangle, "width", query.region_width);
        cJSON_AddNumberToObject(rectangle, "height", query.region_height);
        if (query.region_match == INVESTIGATION_REGION_MIN_INTERSECTION) {
            cJSON_AddNumberToObject(spatial, "min_intersection",
                                    query.region_min_intersection);
        }
    }
    cJSON *reasons = cJSON_AddArrayToObject(coverage, "incomplete_reasons");
    if (summary->unresolved_legacy_count > 0) {
        cJSON_AddItemToArray(
            reasons, cJSON_CreateString("legacy_camera_identity_unresolved"));
    }
    if (query.has_region && summary->spatial_missing_rows > 0) {
        cJSON_AddItemToArray(
            reasons, cJSON_CreateString("spatial_metadata_missing"));
    }
    cJSON_AddBoolToObject(coverage, "complete",
                          summary->unresolved_legacy_count == 0 &&
                          summary->spatial_missing_rows == 0);

    set_json_response(response, root);
    cJSON_Delete(root);
    free(results);
    free(summary);
    cJSON_Delete(body);
}
