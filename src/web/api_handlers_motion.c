/**
 * @file api_handlers_motion.c
 * @brief External motion trigger REST endpoint.
 *
 * Exposes POST /api/motion/trigger so external automation (Home Assistant,
 * NodeRED, shell scripts) can drive the same motion-recording path that
 * ONVIF events normally drive. See discussion #375 for motivation.
 *
 * The endpoint:
 *   - authenticates the caller via X-API-Key / Bearer token / session cookie
 *   - rejects USER_ROLE_VIEWER
 *   - validates the target stream exists and has detection-based recording on
 *   - sets the UDT external_motion_trigger atomic for the target stream
 *   - propagates the event to any cross-stream-linked peers
 *   - for "pulse", schedules a deferred motion-ended via a detached worker
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>
#include <pthread.h>
#include <unistd.h>

#include <cjson/cJSON.h>

#define LOG_COMPONENT "MotionAPI"
#include "core/logger.h"
#include "core/config.h"
#include "utils/strings.h"

#include "web/api_handlers_motion.h"
#include "web/request_response.h"
#include "web/httpd_utils.h"

#include "database/db_auth.h"
#include "video/stream_manager.h"
#include "video/cross_stream_motion_trigger.h"
#include "video/unified_detection_thread.h"

#define MOTION_PULSE_DEFAULT_MS 2000
#define MOTION_PULSE_MAX_MS     600000   /* 10 minutes; matches UDT sanity bound */

typedef enum {
    MOTION_ACTION_START,
    MOTION_ACTION_STOP,
    MOTION_ACTION_PULSE
} motion_action_t;

typedef struct {
    char stream_name[MAX_STREAM_NAME];
    int  duration_ms;
} motion_pulse_task_t;

/* Fire motion-start, sleep for duration_ms, fire motion-stop. Runs in a
 * detached worker thread because the HTTP handler must return immediately.
 * Matches the deferred-work idiom already used in api_handlers_streams_modify.c. */
static void *motion_pulse_worker(void *arg) {
    motion_pulse_task_t *task = (motion_pulse_task_t *)arg;
    if (!task) return NULL;

    useconds_t usec = (useconds_t)task->duration_ms * 1000u;
    /* usleep is limited to <1s on some platforms; loop in 500ms chunks. */
    while (usec > 0) {
        useconds_t chunk = usec > 500000u ? 500000u : usec;
        usleep(chunk);
        usec -= chunk;
    }

    time_t now = time(NULL);
    unified_detection_notify_motion(task->stream_name, false);
    process_motion_event(task->stream_name, false, now, false);

    log_info("Pulse ended for stream '%s' after %d ms", task->stream_name, task->duration_ms);
    free(task);
    return NULL;
}

static int parse_action(const char *s, motion_action_t *out) {
    if (!s || !out) return -1;
    if (strcmp(s, "start") == 0) { *out = MOTION_ACTION_START; return 0; }
    if (strcmp(s, "stop")  == 0) { *out = MOTION_ACTION_STOP;  return 0; }
    if (strcmp(s, "pulse") == 0) { *out = MOTION_ACTION_PULSE; return 0; }
    return -1;
}

void handle_post_motion_trigger(const http_request_t *req, http_response_t *res) {
    log_info("POST /api/motion/trigger");

    /* ---- Auth ------------------------------------------------------------ */
    /* The setup-wizard endpoints are deliberately unauthenticated; everything
     * else in this codebase either respects g_config.web_auth_enabled or is
     * admin-only. External motion trigger is a write operation from
     * (potentially) the public network, so require auth unconditionally when
     * it is globally enabled, and reject read-only viewers. */
    user_t user;
    memset(&user, 0, sizeof(user));
    if (g_config.web_auth_enabled) {
        if (!httpd_get_authenticated_user(req, &user)) {
            http_response_set_json_error(res, 401, "Unauthorized");
            return;
        }
        if (user.role == USER_ROLE_VIEWER) {
            http_response_set_json_error(res, 403,
                "Viewer role cannot trigger motion events");
            return;
        }
    }

    /* ---- Body parsing ---------------------------------------------------- */
    cJSON *body = httpd_parse_json_body(req);
    if (!body) {
        http_response_set_json_error(res, 400, "Invalid or missing JSON body");
        return;
    }

    const cJSON *j_stream   = cJSON_GetObjectItemCaseSensitive(body, "stream");
    const cJSON *j_action   = cJSON_GetObjectItemCaseSensitive(body, "action");
    const cJSON *j_duration = cJSON_GetObjectItemCaseSensitive(body, "duration_ms");

    if (!cJSON_IsString(j_stream) || j_stream->valuestring[0] == '\0') {
        cJSON_Delete(body);
        http_response_set_json_error(res, 400, "Field 'stream' is required");
        return;
    }
    if (!cJSON_IsString(j_action)) {
        cJSON_Delete(body);
        http_response_set_json_error(res, 400,
            "Field 'action' is required (start|stop|pulse)");
        return;
    }

    motion_action_t action;
    if (parse_action(j_action->valuestring, &action) != 0) {
        cJSON_Delete(body);
        http_response_set_json_error(res, 400,
            "Field 'action' must be one of: start, stop, pulse");
        return;
    }

    int duration_ms = MOTION_PULSE_DEFAULT_MS;
    if (action == MOTION_ACTION_PULSE && j_duration) {
        if (!cJSON_IsNumber(j_duration)) {
            cJSON_Delete(body);
            http_response_set_json_error(res, 400,
                "Field 'duration_ms' must be a positive integer");
            return;
        }

        double duration_value = j_duration->valuedouble;
        if (duration_value <= 0 || duration_value > MOTION_PULSE_MAX_MS) {
            cJSON_Delete(body);
            http_response_set_json_error(res, 400,
                "Field 'duration_ms' must be between 1 and 600000");
            return;
        }

        int d = (int)duration_value;
        if ((double)d != duration_value) {
            cJSON_Delete(body);
            http_response_set_json_error(res, 400,
                "Field 'duration_ms' must be a positive integer");
            return;
        }
        duration_ms = d;
    }

    char stream_name[MAX_STREAM_NAME];
    safe_strcpy(stream_name, j_stream->valuestring, sizeof(stream_name), 0);
    cJSON_Delete(body);

    /* ---- Validate target stream ----------------------------------------- */
    stream_handle_t handle = get_stream_by_name(stream_name);
    if (!handle) {
        http_response_set_json_error(res, 404, "Stream not found");
        return;
    }

    stream_config_t cfg;
    memset(&cfg, 0, sizeof(cfg));
    if (get_stream_config(handle, &cfg) != 0) {
        http_response_set_json_error(res, 500, "Failed to read stream config");
        return;
    }

    if (!cfg.detection_based_recording) {
        /* The UDT is not running for this stream, so the trigger would land
         * in the void. 409 Conflict surfaces the misconfiguration instead of
         * silently succeeding. */
        http_response_set_json_error(res, 409,
            "Stream does not have detection-based recording enabled");
        return;
    }

    /* ---- Dispatch -------------------------------------------------------- */
    time_t now = time(NULL);
    bool active = (action != MOTION_ACTION_STOP);

    unified_detection_notify_motion(stream_name, active);
    process_motion_event(stream_name, active, now, false);

    const char *action_str =
        (action == MOTION_ACTION_START) ? "start" :
        (action == MOTION_ACTION_STOP)  ? "stop"  : "pulse";

    if (action == MOTION_ACTION_PULSE) {
        motion_pulse_task_t *task = calloc(1, sizeof(*task));
        if (!task) {
            /* We already armed motion-start; unwind so the stream doesn't
             * sit in RECORDING until external_motion_trigger is reset. */
            unified_detection_notify_motion(stream_name, false);
            process_motion_event(stream_name, false, now, false);
            http_response_set_json_error(res, 500, "Out of memory");
            return;
        }
        safe_strcpy(task->stream_name, stream_name, sizeof(task->stream_name), 0);
        task->duration_ms = duration_ms;

        pthread_t tid;
        pthread_attr_t attr;
        pthread_attr_init(&attr);
        pthread_attr_setdetachstate(&attr, PTHREAD_CREATE_DETACHED);
        int rc = pthread_create(&tid, &attr, motion_pulse_worker, task);
        pthread_attr_destroy(&attr);

        if (rc != 0) {
            unified_detection_notify_motion(stream_name, false);
            process_motion_event(stream_name, false, now, false);
            free(task);
            http_response_set_json_error(res, 500,
                "Failed to schedule pulse end");
            return;
        }
        log_info("Pulse started for stream '%s' (%d ms)", stream_name, duration_ms);
    } else {
        log_info("Motion %s for stream '%s'", action_str, stream_name);
    }

    /* ---- Response -------------------------------------------------------- */
    cJSON *resp = cJSON_CreateObject();
    if (!resp) {
        http_response_set_json_error(res, 500, "Failed to build response");
        return;
    }
    cJSON_AddBoolToObject(resp, "success", true);
    cJSON_AddStringToObject(resp, "stream", stream_name);
    cJSON_AddStringToObject(resp, "action", action_str);
    if (action == MOTION_ACTION_PULSE) {
        cJSON_AddNumberToObject(resp, "duration_ms", duration_ms);
    }

    char *json_str = cJSON_PrintUnformatted(resp);
    cJSON_Delete(resp);
    if (!json_str) {
        http_response_set_json_error(res, 500, "Failed to serialize response");
        return;
    }
    http_response_set_json(res, 202, json_str);
    free(json_str);
}
