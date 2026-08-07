/**
 * @file api_handlers_motion.c
 * @brief External motion trigger REST endpoint.
 *
 * Exposes POST /api/motion/trigger so external automation (Home Assistant,
 * NodeRED, shell scripts) can drive the same motion-recording path that
 * ONVIF events normally drive. See discussion #375 for motivation.
 *
 * The endpoint:
 *   - authenticates the caller via httpd_get_authenticated_user() (for example:
 *     X-API-Key, Bearer token, session cookie, or HTTP Basic auth)
 *   - rejects USER_ROLE_VIEWER
 *   - validates the target stream exists
 *   - records any caller-supplied object classes as detections so the event is
 *     visible on the timeline and over MQTT (#466)
 *   - sets the UDT external_motion_trigger atomic for the target stream
 *   - propagates the event to any cross-stream-linked peers
 *   - for "pulse", schedules a deferred motion-ended via a detached worker
 *
 * Streams without detection-based recording are accepted rather than rejected:
 * a 24/7-recording stream has no UDT to arm, but the caller still wants the
 * event annotated onto the continuous recording it already has (#466). The
 * response reports recording_triggered=false in that case.
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>
#include <pthread.h>
#include <unistd.h>
#include <stdint.h>

#include <cjson/cJSON.h>

#define LOG_COMPONENT "MotionAPI"
#include "core/logger.h"
#include "core/config.h"
#include "utils/strings.h"

#include "web/api_handlers_motion.h"
#include "web/request_response.h"
#include "web/httpd_utils.h"

#include "core/mqtt_client.h"
#include "database/db_auth.h"
#include "database/db_detections.h"
#include "database/db_recording_tags.h"
#include "video/detection_result.h"
#include "video/mp4_recording.h"
#include "video/stream_manager.h"
#include "video/cross_stream_motion_trigger.h"
#include "video/unified_detection_thread.h"
#include "video/zone_filter.h"

#define MOTION_PULSE_DEFAULT_MS 2000
#define MOTION_PULSE_MAX_MS     600000   /* 10 minutes; matches UDT sanity bound */

/* Caller-supplied object classes are asserted facts, not model output, so they
 * default to full confidence unless the caller says otherwise. */
#define MOTION_DEFAULT_CONFIDENCE 1.0f

typedef enum {
    MOTION_ACTION_START,
    MOTION_ACTION_STOP,
    MOTION_ACTION_PULSE
} motion_action_t;

/*
 * Per-stream pulse generation counters.
 *
 * Every motion trigger action (start, stop, or pulse) increments the counter
 * for the target stream.  A pulse worker records the counter value at the
 * moment it is created and checks it again before emitting the deferred
 * motion-stop.  If a newer action has arrived in the meantime the counter
 * will have changed, and the worker silently discards the stop so it cannot
 * cut short an active recording.
 */
typedef struct {
    char     name[MAX_STREAM_NAME];
    uint64_t generation;
} pulse_gen_entry_t;

static pulse_gen_entry_t g_pulse_gen[MAX_STREAMS];
static int               g_pulse_gen_count = 0;
static pthread_mutex_t   g_pulse_gen_mutex = PTHREAD_MUTEX_INITIALIZER;

/* Return the current generation for stream_name (0 if not seen before).
 * Caller MUST hold g_pulse_gen_mutex. */
static uint64_t pulse_gen_get(const char *stream_name) {
    for (int i = 0; i < g_pulse_gen_count; i++) {
        if (strcmp(g_pulse_gen[i].name, stream_name) == 0)
            return g_pulse_gen[i].generation;
    }
    return 0;
}

/* Increment the generation counter for stream_name and return the new value.
 * Caller MUST hold g_pulse_gen_mutex. */
static uint64_t pulse_gen_increment(const char *stream_name) {
    for (int i = 0; i < g_pulse_gen_count; i++) {
        if (strcmp(g_pulse_gen[i].name, stream_name) == 0)
            return ++g_pulse_gen[i].generation;
    }
    /* First time we see this stream — add an entry. */
    if (g_pulse_gen_count < MAX_STREAMS) {
        int i = g_pulse_gen_count++;
        safe_strcpy(g_pulse_gen[i].name, stream_name,
                    sizeof(g_pulse_gen[i].name), 0);
        g_pulse_gen[i].generation = 1;
        return 1;
    }
    /* Table full (shouldn't happen with MAX_STREAMS=256 streams).
     * Return 0 — the reserved "generation tracking disabled" sentinel.
     * The pulse worker treats generation==0 as "always fire the stop",
     * which is safer than leaving the stream permanently in motion-active. */
    log_warn("pulse_gen table full; pulse stop for '%s' will fire unconditionally",
             stream_name);
    return 0;
}

typedef struct {
    char     stream_name[MAX_STREAM_NAME];
    int      duration_ms;
    /* Generation counter snapshot taken when this pulse was scheduled.
     * 0 is the reserved sentinel meaning "generation tracking disabled":
     * the worker will fire the stop unconditionally in that case. */
    uint64_t generation;
    /* False when the stream has no detection-based recording: there is no UDT
     * to notify, so the worker only does the cross-stream fan-out. */
    bool     trigger_recording;
} motion_pulse_task_t;

/* Fire motion-start, sleep for duration_ms, fire motion-stop — but only if no
 * newer motion trigger has arrived since the pulse was scheduled.  Runs in a
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

    /* Check whether a newer action has superseded this pulse.
     * generation==0 means tracking was disabled (table-full fallback);
     * in that case always fire the stop rather than leave motion active. */
    bool still_current;
    if (task->generation == 0) {
        still_current = true;
    } else {
        pthread_mutex_lock(&g_pulse_gen_mutex);
        uint64_t current_gen = pulse_gen_get(task->stream_name);
        still_current = (current_gen == task->generation);
        pthread_mutex_unlock(&g_pulse_gen_mutex);
    }

    if (still_current) {
        time_t now = time(NULL);
        close_external_motion_detections(task->stream_name, now);
        if (task->trigger_recording) {
            unified_detection_notify_motion(task->stream_name, false);
        }
        process_motion_event(task->stream_name, false, now, false);
        log_info("Pulse ended for stream '%s' after %d ms",
                 task->stream_name, task->duration_ms);
    } else {
        log_info("Pulse for stream '%s' superseded by newer trigger; stop skipped",
                 task->stream_name);
    }

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

/* Validate an optional confidence value. Returns 0 and writes *out on success,
 * -1 if the field is present but not a number in [0,1]. A missing field leaves
 * *out untouched so the caller's default survives. */
static int parse_confidence(const cJSON *j, float *out) {
    if (!j) return 0;
    if (!cJSON_IsNumber(j)) return -1;
    double v = j->valuedouble;
    if (v < 0.0 || v > 1.0) return -1;
    *out = (float)v;
    return 0;
}

/* Append one externally reported object to `result`.
 *
 * An external trigger carries no spatial information, so the detection covers
 * the whole frame — the same convention the ONVIF event path uses when a camera
 * reports a smart-detection class without a bounding box. Returns -1 if the
 * label is empty or the result is full. */
static int append_detection(detection_result_t *result, const char *label, float confidence) {
    if (!result || !label || label[0] == '\0') return -1;
    if (result->count >= MAX_DETECTIONS) return -1;

    detection_t *d = &result->detections[result->count];
    safe_strcpy(d->label, label, MAX_LABEL_LENGTH, 0);
    d->confidence = confidence;
    d->x = 0.0f;
    d->y = 0.0f;
    d->width = 1.0f;
    d->height = 1.0f;
    result->count++;
    return 0;
}

/* Parse the optional `label`, `confidence` and `objects` fields into `result`.
 *
 * `objects` accepts either bare strings (["person","vehicle"]) or objects with
 * a per-entry confidence ([{"label":"person","confidence":0.82}]), so callers
 * can forward detector output verbatim or keep it terse.
 *
 * Returns 0 on success. On a malformed field returns -1 and writes a caller-
 * facing message into `err` (never NULL-terminated short of `err_sz`). */
int motion_trigger_parse_objects(const cJSON *body, detection_result_t *result,
                                 char *err, size_t err_sz) {
    float default_confidence = MOTION_DEFAULT_CONFIDENCE;
    const cJSON *j_conf = cJSON_GetObjectItemCaseSensitive(body, "confidence");
    if (parse_confidence(j_conf, &default_confidence) != 0) {
        safe_strcpy(err, "Field 'confidence' must be a number between 0 and 1", err_sz, 0);
        return -1;
    }

    const cJSON *j_label = cJSON_GetObjectItemCaseSensitive(body, "label");
    if (j_label) {
        if (!cJSON_IsString(j_label) || j_label->valuestring[0] == '\0') {
            safe_strcpy(err, "Field 'label' must be a non-empty string", err_sz, 0);
            return -1;
        }
        append_detection(result, j_label->valuestring, default_confidence);
    }

    const cJSON *j_objects = cJSON_GetObjectItemCaseSensitive(body, "objects");
    if (!j_objects) return 0;
    if (!cJSON_IsArray(j_objects)) {
        safe_strcpy(err, "Field 'objects' must be an array", err_sz, 0);
        return -1;
    }

    const cJSON *entry = NULL;
    cJSON_ArrayForEach(entry, j_objects) {
        const char *label = NULL;
        float confidence = default_confidence;

        if (cJSON_IsString(entry)) {
            label = entry->valuestring;
        } else if (cJSON_IsObject(entry)) {
            const cJSON *j_entry_label = cJSON_GetObjectItemCaseSensitive(entry, "label");
            if (!cJSON_IsString(j_entry_label)) {
                safe_strcpy(err, "Each 'objects' entry must be a string or an object with a 'label' string", err_sz, 0);
                return -1;
            }
            label = j_entry_label->valuestring;
            if (parse_confidence(cJSON_GetObjectItemCaseSensitive(entry, "confidence"),
                                 &confidence) != 0) {
                safe_strcpy(err, "Each 'objects' entry 'confidence' must be a number between 0 and 1", err_sz, 0);
                return -1;
            }
        } else {
            safe_strcpy(err, "Each 'objects' entry must be a string or an object with a 'label' string", err_sz, 0);
            return -1;
        }

        if (!label || label[0] == '\0') {
            safe_strcpy(err, "Each 'objects' entry must have a non-empty label", err_sz, 0);
            return -1;
        }
        /* Silently ignore anything past MAX_DETECTIONS rather than failing the
         * whole trigger — the motion event still matters more than the tail of
         * an over-long object list. The count is reported in the response. */
        append_detection(result, label, confidence);
    }

    return 0;
}

/* Parse the optional `tags` array of strings into a fixed-size buffer.
 * Returns the tag count on success, -1 on a malformed field. */
int motion_trigger_parse_tags(const cJSON *body, char tags[][MAX_TAG_LENGTH],
                              char *err, size_t err_sz) {
    const cJSON *j_tags = cJSON_GetObjectItemCaseSensitive(body, "tags");
    if (!j_tags) return 0;
    if (!cJSON_IsArray(j_tags)) {
        safe_strcpy(err, "Field 'tags' must be an array of strings", err_sz, 0);
        return -1;
    }

    int count = 0;
    const cJSON *entry = NULL;
    cJSON_ArrayForEach(entry, j_tags) {
        if (!cJSON_IsString(entry) || entry->valuestring[0] == '\0') {
            safe_strcpy(err, "Each 'tags' entry must be a non-empty string", err_sz, 0);
            return -1;
        }
        if (count >= MOTION_MAX_TAGS) break;
        safe_strcpy(tags[count], entry->valuestring, MAX_TAG_LENGTH, 0);
        count++;
    }
    return count;
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

    /* ---- Optional detection metadata (#466) ------------------------------ */
    /* Object classes and tags let an external detector (a camera's own smart
     * events, Frigate, a HA automation) say *what* it saw, not just that
     * something moved, so the event lands on the timeline and in MQTT with the
     * same shape as a model detection. */
    detection_result_t detections;
    memset(&detections, 0, sizeof(detections));
    char tags[MOTION_MAX_TAGS][MAX_TAG_LENGTH];
    char parse_err[160] = {0};

    if (motion_trigger_parse_objects(body, &detections, parse_err, sizeof(parse_err)) != 0) {
        cJSON_Delete(body);
        http_response_set_json_error(res, 400, parse_err);
        return;
    }

    int tag_count = motion_trigger_parse_tags(body, tags, parse_err, sizeof(parse_err));
    if (tag_count < 0) {
        cJSON_Delete(body);
        http_response_set_json_error(res, 400, parse_err);
        return;
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

    /* A stream without detection-based recording has no UDT to arm, so the
     * recording-trigger half of this request cannot do anything. That used to
     * be a 409, which also blocked the annotation half — leaving 24/7-recording
     * users with no way to put an external event on their timeline (#466).
     * Accept the request, do everything that *is* possible (detections, tags,
     * MQTT, cross-stream fan-out), and report what was skipped. */
    bool schedule_allows_trigger = is_detection_recording_scheduled(&cfg);
    bool can_trigger_recording =
        cfg.detection_based_recording && schedule_allows_trigger;
    if (!cfg.detection_based_recording) {
        log_info("Stream '%s' has no detection-based recording; recording trigger skipped "
                 "(detections/tags still recorded)", stream_name);
    } else if (!schedule_allows_trigger) {
        log_info("Stream '%s' is outside its detection recording schedule; "
                 "recording trigger skipped (detections/tags still recorded)",
                 stream_name);
    }

    /* ---- Dispatch -------------------------------------------------------- */
    time_t now = time(NULL);
    bool active = (action != MOTION_ACTION_STOP);

    /* Increment the per-stream generation counter before dispatching.  This
     * invalidates any in-flight pulse worker for this stream so it will not
     * emit a spurious stop that could cut short the new motion activity. */
    pthread_mutex_lock(&g_pulse_gen_mutex);
    uint64_t new_gen = pulse_gen_increment(stream_name);
    pthread_mutex_unlock(&g_pulse_gen_mutex);

    /* Record the reported objects before arming motion so a detection-triggered
     * recording can pick them up via the existing update_detections_recording_id
     * linkage. Only meaningful on the leading edge — a "stop" reports nothing
     * new about what was seen. */
    int stored_detections = 0;
    uint64_t current_recording_id =
        get_current_recording_id_for_stream(stream_name);
    if (active) {
        /* A new leading edge supersedes an unclosed prior session. This keeps
         * repeated Home Assistant "start" messages from leaving overlapping
         * open intervals. */
        close_external_motion_detections(stream_name, now);

        bool caller_supplied_objects = detections.count > 0;
        /* Honour zone filtering so an external trigger respects the same zone
         * configuration a model detection would. */
        if (detections.count > 0 &&
            filter_detections_by_zones(stream_name, &detections) != 0) {
            log_warn("Failed to filter detections by zones for '%s'; storing all", stream_name);
        }

        /* A plain motion start still needs a persisted interval for accurate
         * timeline highlighting. Use a zero-area generic motion detection;
         * callers that supplied objects but had every object excluded by zone
         * filtering remain excluded. */
        if (!caller_supplied_objects) {
            detections.count = 1;
            safe_strcpy(detections.detections[0].label, "motion",
                        sizeof(detections.detections[0].label), 0);
            detections.detections[0].confidence = 1.0f;
            detections.detections[0].track_id = -1;
        }
        if (detections.count > 0) {
            if (store_external_motion_detections(stream_name, &detections, now,
                                                 current_recording_id) == 0) {
                stored_detections = detections.count;
            } else {
                log_warn("Failed to store external detections for stream '%s'", stream_name);
            }
            if (caller_supplied_objects) {
                mqtt_publish_detection(stream_name, &detections, now);
            }
        }
    } else {
        close_external_motion_detections(stream_name, now);
    }

    /* Tags attach to whatever recording is currently open for the stream. With
     * 24/7 recording that is the segment the event falls inside — the case this
     * exists for. With detection-based recording no segment is open yet at this
     * instant, so tag_count is reported as applied=0 rather than guessed at. */
    int tags_applied = 0;
    if (active && tag_count > 0) {
        if (current_recording_id != 0) {
            for (int i = 0; i < tag_count; i++) {
                if (db_recording_tag_add(current_recording_id, tags[i]) == 0) {
                    tags_applied++;
                }
            }
        } else {
            log_info("No active recording for stream '%s'; %d tag(s) not applied",
                     stream_name, tag_count);
        }
    }

    if (can_trigger_recording) {
        unified_detection_notify_motion(stream_name, active);
    }
    process_motion_event(stream_name, active, now, false);

    const char *action_str =
        (action == MOTION_ACTION_START) ? "start" :
        (action == MOTION_ACTION_STOP)  ? "stop"  : "pulse";

    if (action == MOTION_ACTION_PULSE) {
        motion_pulse_task_t *task = calloc(1, sizeof(*task));
        if (!task) {
            /* We already armed motion-start; unwind so the stream doesn't
             * sit in RECORDING until external_motion_trigger is reset. */
            if (can_trigger_recording) {
                unified_detection_notify_motion(stream_name, false);
            }
            close_external_motion_detections(stream_name, now);
            process_motion_event(stream_name, false, now, false);
            http_response_set_json_error(res, 500, "Out of memory");
            return;
        }
        safe_strcpy(task->stream_name, stream_name, sizeof(task->stream_name), 0);
        task->duration_ms       = duration_ms;
        task->generation        = new_gen;
        task->trigger_recording = can_trigger_recording;

        pthread_t tid;
        pthread_attr_t attr;
        pthread_attr_init(&attr);
        pthread_attr_setdetachstate(&attr, PTHREAD_CREATE_DETACHED);
        int rc = pthread_create(&tid, &attr, motion_pulse_worker, task);
        pthread_attr_destroy(&attr);

        if (rc != 0) {
            if (can_trigger_recording) {
                unified_detection_notify_motion(stream_name, false);
            }
            close_external_motion_detections(stream_name, now);
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
    /* Report what actually happened so callers can tell an annotated event on a
     * 24/7 stream from one that also started a detection-based recording, and
     * can see when zone filtering or a missing open recording dropped work. */
    cJSON_AddBoolToObject(resp, "recording_triggered", can_trigger_recording);
    cJSON_AddNumberToObject(resp, "detections_stored", stored_detections);
    cJSON_AddNumberToObject(resp, "tags_applied", tags_applied);
    if (!cfg.detection_based_recording) {
        cJSON_AddStringToObject(resp, "warning",
            "Stream does not have detection-based recording enabled; "
            "the event was recorded but no recording was triggered");
    } else if (!schedule_allows_trigger) {
        cJSON_AddStringToObject(resp, "warning",
            "The event was recorded, but no recording was triggered because "
            "the stream is outside its detection recording schedule");
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
