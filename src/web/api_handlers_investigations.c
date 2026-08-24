/**
 * @file api_handlers_investigations.c
 * @brief Multi-camera investigation timeline API.
 */

#include <math.h>
#include <stdbool.h>
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
#include "database/db_streams.h"

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
                                     "Failed to encode investigation timeline");
        return;
    }
    http_response_set_json(response, 200, encoded);
    free(encoded);
}

void handle_post_investigation_timeline(const http_request_t *request,
                                        http_response_t *response) {
    if (!request || !response) return;

    cJSON *body = httpd_parse_json_body(request);
    if (!body) {
        http_response_set_json_error(response, 400, "Invalid JSON body");
        return;
    }

    const cJSON *camera_uuids =
        cJSON_GetObjectItemCaseSensitive(body, "camera_uuids");
    int camera_count = cJSON_IsArray(camera_uuids)
        ? cJSON_GetArraySize(camera_uuids) : 0;
    if (camera_count < 1 || camera_count > INVESTIGATION_MAX_CAMERAS) {
        cJSON_Delete(body);
        http_response_set_json_error(
            response, 400, "camera_uuids must contain between 1 and 16 cameras");
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

    user_t user;
    if (!httpd_check_action_access(request, &user)) {
        cJSON_Delete(body);
        http_response_set_json_error(response, 401, "Unauthorized");
        return;
    }

    stream_config_t cameras[INVESTIGATION_MAX_CAMERAS];
    fleet_camera_t fleet_cameras[INVESTIGATION_MAX_CAMERAS];
    memset(cameras, 0, sizeof(cameras));
    memset(fleet_cameras, 0, sizeof(fleet_cameras));

    authorization_context_t *auth_context = authorization_context_create();
    if (!auth_context) {
        cJSON_Delete(body);
        http_response_set_json_error(response, 500,
                                     "Authorization context unavailable");
        return;
    }

    /* Resolve and authorize the complete fixed camera list before returning any
     * timeline data. Explicit unauthorized UUIDs fail as one scoped request. */
    for (int i = 0; i < camera_count; i++) {
        const cJSON *uuid = cJSON_GetArrayItem(camera_uuids, i);
        if (!cJSON_IsString(uuid) || !uuid->valuestring ||
            strlen(uuid->valuestring) != CAMERA_UUID_STRING_SIZE - 1) {
            authorization_context_free(auth_context);
            cJSON_Delete(body);
            http_response_set_json_error(response, 400,
                                         "camera_uuids contains an invalid UUID");
            return;
        }
        for (int previous = 0; previous < i; previous++) {
            if (strcmp(cameras[previous].camera_uuid, uuid->valuestring) == 0) {
                authorization_context_free(auth_context);
                cJSON_Delete(body);
                http_response_set_json_error(response, 400,
                                             "camera_uuids contains duplicates");
                return;
            }
        }
        if (get_stream_config_by_uuid(uuid->valuestring, &cameras[i]) != 0 ||
            db_fleet_camera_find_by_name(cameras[i].name,
                                         &fleet_cameras[i]) != 0) {
            authorization_context_free(auth_context);
            cJSON_Delete(body);
            http_response_set_json_error(response, 404, "Camera not found");
            return;
        }
        authorization_evaluation_t evaluation;
        memset(&evaluation, 0, sizeof(evaluation));
        if (authorization_evaluate_in_context(
                auth_context, &user, AUTHZ_RECORDINGS_REPLAY,
                &fleet_cameras[i], &evaluation) != 0) {
            audit_log_authorization(request, &user, AUTHZ_RECORDINGS_REPLAY,
                                    &fleet_cameras[i], NULL, "error");
            authorization_context_free(auth_context);
            cJSON_Delete(body);
            http_response_set_json_error(
                response, 500, "Authorization policy evaluation failed");
            return;
        }
        if (evaluation.decision != AUTHZ_DECISION_ALLOW) {
            audit_log_authorization(request, &user, AUTHZ_RECORDINGS_REPLAY,
                                    &fleet_cameras[i], &evaluation, "denied");
            authorization_context_free(auth_context);
            cJSON_Delete(body);
            http_response_set_json_error(response, 403, "Forbidden");
            return;
        }
    }
    authorization_context_free(auth_context);

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
