/*
 * SPDX-License-Identifier: GPL-3.0-or-later
 * Copyright (C) 2026 OpenSensor Engineering and LightNVR contributors
 *
 * This file is part of LightNVR <https://github.com/opensensor/lightNVR>.
 */

/* INT_01 L6: local detector -> observation evidence glue. See
 * include/video/detection_evidence.h. No DB access and no blocking. */

#include "video/detection_evidence.h"

#include <ctype.h>
#include <math.h>
#include <pthread.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <strings.h>
#include <sys/time.h>

#include "core/logger.h"
#include "utils/strings.h"
#include "utils/uuid.h"
#include "video/detection_model_internal.h"
#include "video/sod_detection.h"
#ifdef HAVE_LITERT
#include "video/detection/litert_engine.h"
#endif

#define DEFAULT_ENGINE_KEY "legacy-primary"

/* ---- boot session ----------------------------------------------------- */

static pthread_once_t boot_once = PTHREAD_ONCE_INIT;
static char boot_session[OBS_UUID_SIZE];

static void boot_session_init(void) {
    if (lightnvr_uuid_generate_v4(boot_session) != 0) boot_session[0] = '\0';
}

const char *detection_evidence_boot_session(void) {
    pthread_once(&boot_once, boot_session_init);
    return boot_session;
}

/* ---- eligibility ------------------------------------------------------ */

bool detection_evidence_label_eligible(const char *labels_csv, const char *label) {
    if (!label || !label[0]) return false;
    if (!labels_csv) return true;

    const char *scan = labels_csv;
    while (*scan == ' ' || *scan == '\t') scan++;
    if (*scan == '\0') return true;

    size_t label_len = strlen(label);
    const char *cursor = labels_csv;
    for (;;) {
        const char *end = strchr(cursor, ',');
        const char *stop = end ? end : cursor + strlen(cursor);
        const char *first = cursor;
        const char *last = stop;
        while (first < last && (*first == ' ' || *first == '\t')) first++;
        while (last > first && (last[-1] == ' ' || last[-1] == '\t')) last--;
        if ((size_t)(last - first) == label_len &&
            strncasecmp(first, label, label_len) == 0) {
            return true;
        }
        if (!end) return false;
        cursor = end + 1;
    }
}

/* ---- boxes -------------------------------------------------------------- */

static double clamp01(double v) {
    if (!(v > 0.0)) return 0.0;  /* also maps NaN to 0 */
    return v > 1.0 ? 1.0 : v;
}

bool detection_evidence_clamp_box(double x, double y, double w, double h,
                                  double *ox, double *oy, double *ow, double *oh) {
    if (!isfinite(x) || !isfinite(y) || !isfinite(w) || !isfinite(h)) return false;
    double x0 = clamp01(x), y0 = clamp01(y);
    double x1 = clamp01(x + w), y1 = clamp01(y + h);
    /* L2 rounds to 6 decimals; anything that would round to zero area is no box. */
    if (x1 - x0 < 1e-6 || y1 - y0 < 1e-6) return false;
    if (ox) *ox = x0;
    if (oy) *oy = y0;
    if (ow) *ow = x1 - x0;
    if (oh) *oh = y1 - y0;
    return true;
}

/* ---- batch ---------------------------------------------------------------- */

static const char *basename_of(const char *path) {
    if (!path) return "";
    const char *slash = strrchr(path, '/');
    return slash ? slash + 1 : path;
}

int detection_evidence_build_batch(const detection_evidence_camera_t *camera,
                                   const detection_evidence_engine_t *engine,
                                   const detection_t *detections, int count,
                                   int64_t observed_at_ms,
                                   observation_batch_input_t *out) {
    if (!camera || !engine || !out || count < 0 || (count > 0 && !detections) ||
        !camera->camera_uuid || !camera->camera_uuid[0] || observed_at_ms <= 0) {
        return -1;
    }

    memset(out, 0, sizeof(*out));
    safe_strcpy(out->camera_uuid, camera->camera_uuid, sizeof(out->camera_uuid), 0);
    if (camera->stream_name) {
        safe_strcpy(out->stream_name, camera->stream_name, sizeof(out->stream_name), 0);
    }
    out->observed_at_ms = observed_at_ms;
    out->recording_id = camera->recording_id > 0 ? camera->recording_id : 0;
    const char *key = (engine->engine_key && engine->engine_key[0])
        ? engine->engine_key : DEFAULT_ENGINE_KEY;
    safe_strcpy(out->engine_key, key, sizeof(out->engine_key), 0);
    safe_strcpy(out->engine_type,
                engine->engine_type && engine->engine_type[0] ? engine->engine_type : "object",
                sizeof(out->engine_type), 0);
    const char *model = basename_of(engine->model_path);
    if (strlen(model) < sizeof(out->model)) {
        safe_strcpy(out->model, model, sizeof(out->model), 0);
    }
    if (engine->model_version &&
        strlen(engine->model_version) < sizeof(out->model_version)) {
        safe_strcpy(out->model_version, engine->model_version, sizeof(out->model_version), 0);
    }

    const char *session = NULL;
    int n = 0;
    for (int i = 0; i < count && n < OBS_MAX_BATCH; ++i) {
        const detection_t *d = &detections[i];
        size_t label_len = strnlen(d->label, sizeof(d->label));
        if (label_len == 0 || label_len >= OBS_LABEL_SIZE) continue;
        if (!detection_evidence_label_eligible(camera->labels_csv, d->label)) continue;
        if (!isfinite(d->confidence)) continue;

        observation_object_input_t *o = &out->objects[n];
        o->object_index = n;
        memcpy(o->label, d->label, label_len);
        o->label[label_len] = '\0';
        o->confidence = clamp01((double)d->confidence);
        o->has_box = detection_evidence_clamp_box(d->x, d->y, d->width, d->height,
                                                  &o->box_x, &o->box_y,
                                                  &o->box_width, &o->box_height);
        if (d->track_id >= 0) {
            if (!session) session = detection_evidence_boot_session();
            if (session[0]) {
                o->has_track = true;
                snprintf(o->track_id, sizeof(o->track_id), "%d", d->track_id);
                safe_strcpy(o->track_namespace, out->engine_key, sizeof(o->track_namespace), 0);
                safe_strcpy(o->track_session, session, sizeof(o->track_session), 0);
            }
        }
        ++n;
    }
    out->object_count = n;
    if (n == 0) return 0;

    if (lightnvr_uuid_generate_v4(out->batch_id) != 0) return -1;
    return n;
}

/* ---- transform ------------------------------------------------------------ */

void detection_evidence_transform_compute(int frame_w, int frame_h,
                                          int net_w, int net_h, bool letterbox,
                                          observation_evidence_transform_t *out) {
    if (!out) return;
    memset(out, 0, sizeof(*out));
    out->mode = OBS_TRANSFORM_NONE;
    out->source_width = out->inference_width = frame_w;
    out->source_height = out->inference_height = frame_h;
    out->scale_x = out->scale_y = 1.0;
    if (frame_w <= 0 || frame_h <= 0 || net_w <= 0 || net_h <= 0 ||
        (net_w == frame_w && net_h == frame_h)) {
        return;
    }

    out->inference_width = net_w;
    out->inference_height = net_h;
    if (!letterbox) {
        out->mode = OBS_TRANSFORM_RESIZE;
        out->scale_x = (double)net_w / (double)frame_w;
        out->scale_y = (double)net_h / (double)frame_h;
        return;
    }

    /* Mirror litert_engine.cc letterbox_rgb24() exactly (float math). */
    float sx = (float)net_w / (float)frame_w;
    float sy = (float)net_h / (float)frame_h;
    float scale = sx < sy ? sx : sy;
    long new_w = lroundf((float)frame_w * scale);
    long new_h = lroundf((float)frame_h * scale);
    if (new_w < 1) new_w = 1;
    if (new_h < 1) new_h = 1;
    out->mode = OBS_TRANSFORM_LETTERBOX;
    out->scale_x = out->scale_y = (double)scale;
    out->pad_x = (double)((net_w - (int)new_w) / 2);
    out->pad_y = (double)((net_h - (int)new_h) / 2);
    if (out->pad_x < 0) out->pad_x = 0;
    if (out->pad_y < 0) out->pad_y = 0;
}

int detection_evidence_model_geometry(detection_model_t model, int *net_w, int *net_h,
                                      bool *letterbox) {
    if (!model || !net_w || !net_h || !letterbox) return -1;
    const model_t *m = (const model_t *)model;
    if (strcmp(m->type, MODEL_TYPE_SOD) == 0) {
        if (sod_model_input_size(model, net_w, net_h) != 0) return -1;
        *letterbox = false;
        return 0;
    }
#ifdef HAVE_LITERT
    if (strcmp(m->type, MODEL_TYPE_TFLITE) == 0 && m->tflite.engine) {
        const litert_engine_t *engine = (const litert_engine_t *)m->tflite.engine;
        *net_w = litert_engine_input_width(engine);
        *net_h = litert_engine_input_height(engine);
        if (*net_w <= 0 || *net_h <= 0) return -1;
        *letterbox = true;
        return 0;
    }
#endif
    /* RealNet scans the frame at its own resolution; others are unknown. */
    return -1;
}

/* ---- time ------------------------------------------------------------------ */

int64_t detection_evidence_observed_at_ms(time_t frame_ts) {
    struct timeval tv;
    gettimeofday(&tv, NULL);
    if (frame_ts <= 0) return (int64_t)tv.tv_sec * 1000 + tv.tv_usec / 1000;
    if ((time_t)tv.tv_sec == frame_ts) {
        return (int64_t)tv.tv_sec * 1000 + tv.tv_usec / 1000;
    }
    return (int64_t)frame_ts * 1000;
}

/* ---- capture ---------------------------------------------------------------- */

detection_evidence_status_t detection_evidence_capture(
    const detection_evidence_camera_t *camera,
    const detection_evidence_engine_t *engine,
    const detection_t *detections, int count,
    int64_t observed_at_ms,
    uint8_t *rgb24, int width, int height,
    const observation_evidence_transform_t *transform,
    observation_evidence_result_t *submit_result,
    char asset_uuid_out[OBS_UUID_SIZE]) {
    if (asset_uuid_out) asset_uuid_out[0] = '\0';
    if (!camera || !engine) {
        free(rgb24);
        return DETECTION_EVIDENCE_ERROR;
    }
    if (!camera->evidence_enabled) {
        free(rgb24);
        return DETECTION_EVIDENCE_NOT_ENABLED;
    }
    /* Contract section 3: only object engines observe here. Motion never
     * creates observations; API/ONVIF have their own paths (L7). */
    if (!engine->engine_type || strcmp(engine->engine_type, "object") != 0) {
        free(rgb24);
        return DETECTION_EVIDENCE_NOT_OBJECT_ENGINE;
    }
    if (!rgb24 || width <= 0 || height <= 0) {
        free(rgb24);
        return DETECTION_EVIDENCE_ERROR;
    }

    observation_batch_input_t *batch = malloc(sizeof(*batch));
    if (!batch) {
        free(rgb24);
        return DETECTION_EVIDENCE_ERROR;
    }
    int n = detection_evidence_build_batch(camera, engine, detections, count,
                                           observed_at_ms, batch);
    if (n <= 0) {
        free(batch);
        free(rgb24);
        return n == 0 ? DETECTION_EVIDENCE_NOT_ELIGIBLE : DETECTION_EVIDENCE_ERROR;
    }

    observation_evidence_result_t r = observation_evidence_submit(
        batch, rgb24, width, height, transform, asset_uuid_out);
    if (submit_result) *submit_result = r;
    if (r != OBS_EVIDENCE_ACCEPTED) {
        log_debug("[%s] Observation evidence not accepted (result %d, %d object(s))",
                  camera->stream_name ? camera->stream_name : "?", (int)r, n);
    }
    free(batch);
    return DETECTION_EVIDENCE_SUBMITTED;
}
