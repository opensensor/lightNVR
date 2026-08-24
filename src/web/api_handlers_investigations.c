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

    investigation_search_query_t query = {0};
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

    stream_config_t cameras[INVESTIGATION_MAX_CAMERAS] = {0};
    fleet_camera_t fleet_cameras[INVESTIGATION_MAX_CAMERAS] = {0};
    user_t user = {0};
    int camera_count = resolve_authorized_cameras(
        body, request, response, cameras, fleet_cameras, &user);
    if (camera_count < 0) {
        cJSON_Delete(body);
        return;
    }
    query.camera_count = camera_count;
    for (int i = 0; i < camera_count; i++) {
        safe_strcpy(query.camera_uuids[i], cameras[i].camera_uuid,
                    sizeof(query.camera_uuids[i]), 0);
        safe_strcpy(query.legacy_stream_names[i], cameras[i].name,
                    sizeof(query.legacy_stream_names[i]), 0);
    }

    const cJSON *filters = cJSON_GetObjectItemCaseSensitive(body, "filters");
    if (filters && !cJSON_IsObject(filters)) {
        cJSON_Delete(body);
        http_response_set_json_error(response, 400,
                                     "filters must be an object");
        return;
    }
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
        !parse_confidence_filter(filters, "min_confidence",
                                 &query.has_min_confidence,
                                 &query.min_confidence) ||
        !parse_confidence_filter(filters, "max_confidence",
                                 &query.has_max_confidence,
                                 &query.max_confidence) ||
        (query.has_min_confidence && query.has_max_confidence &&
         query.min_confidence > query.max_confidence) ||
        !parse_search_cursor(body, &query)) {
        cJSON_Delete(body);
        http_response_set_json_error(response, 400,
                                     "Invalid investigation search filters");
        return;
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
    if (db_investigation_search(&query, results, summary) != 0) {
        free(results);
        free(summary);
        cJSON_Delete(body);
        http_response_set_json_error(response, 500,
                                     "Investigation search failed");
        return;
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
    for (int i = 0; i < summary->result_count; i++) {
        const investigation_search_result_t *result = &results[i];
        const fleet_camera_t *camera = find_camera_context(
            fleet_cameras, camera_count, result->camera_uuid);
        cJSON *item = cJSON_CreateObject();
        cJSON *camera_context = cJSON_CreateObject();
        cJSON *detection = cJSON_CreateObject();
        cJSON *thumbnail = cJSON_CreateObject();
        if (!item || !camera_context || !detection || !thumbnail) {
            cJSON_Delete(item);
            cJSON_Delete(camera_context);
            cJSON_Delete(detection);
            cJSON_Delete(thumbnail);
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
        } else {
            cJSON_AddNullToObject(item, "recording_id");
            cJSON_AddStringToObject(thumbnail, "status", "unavailable");
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
    cJSON *reasons = cJSON_AddArrayToObject(coverage, "incomplete_reasons");
    if (summary->unresolved_legacy_count > 0) {
        cJSON_AddItemToArray(
            reasons, cJSON_CreateString("legacy_camera_identity_unresolved"));
    }
    cJSON_AddBoolToObject(coverage, "complete",
                          summary->unresolved_legacy_count == 0);

    set_json_response(response, root);
    cJSON_Delete(root);
    free(results);
    free(summary);
    cJSON_Delete(body);
}
