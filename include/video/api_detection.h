/*
 * SPDX-License-Identifier: GPL-3.0-or-later
 * Copyright (C) 2025-2026 OpenSensor Engineering and LightNVR contributors
 *
 * This file is part of LightNVR <https://github.com/opensensor/lightNVR>.
 */

#ifndef LIGHTNVR_API_DETECTION_H
#define LIGHTNVR_API_DETECTION_H

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>
#include <time.h>

#include "core/config.h"
#include "core/observation_evidence.h"
#include "video/detection_result.h"

// Model type for API-based detection
#define MODEL_TYPE_API "api"

/**
 * Per-request options for the HTTP detection API.
 *
 * LightNVR speaks two wire formats:
 *
 *  - light-object-detect (default): multipart/form-data upload of the JPEG in a
 *    "file" part, with backend/confidence_threshold/return_image query
 *    parameters appended to the URL. The reply is {"detections":[{label,
 *    confidence 0-1, x_min/y_min/x_max/y_max normalized 0-1 (flat or under
 *    "bounding_box")}]}.
 *
 *  - doods2: JSON body {"id","detector_name","detect":{"*":percent},"data":
 *    base64 JPEG} posted verbatim to the URL. The reply is {"detections":[{
 *    top/left/bottom/right normalized 0-1, label, confidence 0-100}],"error"}.
 *
 * The global defaults come from [api_detection] in lightnvr.ini and may be
 * overridden per stream through the detection engine "config" JSON object
 * ({"format","backend","detector_name"}).
 */
typedef struct {
    api_detection_format_t format;
    char backend[API_DETECTION_BACKEND_MAX];             // light-object-detect: inference backend query param
    char detector_name[API_DETECTION_DETECTOR_NAME_MAX]; // doods2: "detector_name" request field
} api_detection_options_t;

/** Populate options from the global configuration. */
void api_detection_options_from_config(api_detection_options_t *options);

/**
 * Overlay per-engine overrides from a detection engine config JSON object.
 * Unknown keys are ignored. Returns 0 on success (including an empty or
 * absent config) and -1 when the JSON is malformed or names an unknown
 * format; options are left in a consistent state either way.
 */
int api_detection_options_apply_json(api_detection_options_t *options,
                                     const char *config_json);

/**
 * Build the request URL for the selected format. light-object-detect appends
 * its query parameters; doods2 uses the base URL verbatim. NULL options mean
 * the global configuration. Returns 0 on success, -1 on error.
 */
int api_detection_build_request_url(char *buffer, size_t buffer_size,
                                    const char *base_url, float threshold,
                                    const api_detection_options_t *options);

/**
 * Build the DOODS2 JSON request body for a JPEG snapshot. request_id is sent
 * back by DOODS2 in the response "id" field (stream name when available).
 * Returns a malloc'd string the caller frees, or NULL on failure.
 */
char *api_detection_build_doods2_body(const unsigned char *jpeg_data, size_t jpeg_size,
                                      float threshold, const char *request_id,
                                      const api_detection_options_t *options);

/**
 * Parse a detection API response into result (reset first). Both box shapes
 * are accepted regardless of format; the confidence scale follows the format
 * (0-1 for light-object-detect, 0-100 for doods2). Returns 0 on success and
 * -1 when the document is not a detection response or reports an error.
 */
int api_detection_parse_response(const char *json,
                                 const api_detection_options_t *options,
                                 detection_result_t *result);

/*
 * INT_01 L7: optional geometry and provenance a detection server may declare
 * next to "detections" (both wire formats). Absent fields keep today's
 * behaviour: boxes normalized to the submitted JPEG, transform mode none.
 *
 *   "letterbox": {
 *     "width": 640, "height": 640,       // letterboxed inference image (required, > 0)
 *     "scale": 0.5,                      // or "scale_x"/"scale_y": inference / submitted
 *                                        //   (absent = uniform min(width/W, height/H))
 *     "pad_x": 0, "pad_y": 140,          // inference-space left/top padding in pixels
 *                                        //   (absent = centred: (width - round(W*scale)) / 2)
 *     "box_units": "normalized"          // or "pixels": boxes are in inference space,
 *   },                                   //   normalized to width x height or in pixels
 *   "model": "yolov8n", "model_version": "8.3.0"
 *
 * When a letterbox is declared the boxes in "detections" are in inference
 * space; api_detection_apply_response_geometry() maps them onto the submitted
 * image (contract section 4.4: x = (u - pad_x) / scale_x).
 */
typedef struct {
    bool letterbox;                 /* a valid "letterbox" object was present */
    int inference_width, inference_height;
    double scale_x, scale_y;        /* <= 0: derive from the submitted size */
    double pad_x, pad_y;            /* < 0: derive (centred) */
    bool pixel_units;               /* box_units == "pixels" */
    char model[128];                /* "" = not declared */
    char model_version[64];         /* "" = not declared */
} api_detection_response_meta_t;

/** api_detection_parse_response() that also fills meta (may be NULL). Boxes
 *  are left exactly as the server sent them. */
int api_detection_parse_response_meta(const char *json,
                                      const api_detection_options_t *options,
                                      detection_result_t *result,
                                      api_detection_response_meta_t *meta);

/*
 * Map result's boxes onto the submitted image (image_w x image_h) according
 * to meta and fill transform (contract section 4.4; may be NULL):
 *   - no letterbox: boxes untouched; mode none, inference = image size;
 *   - letterbox: boxes mapped from inference space and re-normalized to the
 *     image (not clamped; the evidence layer clamps); mode letterbox with the
 *     declared or derived scale and padding.
 * source_w/source_h is the decoded frame the JPEG came from (<= 0 = the
 * image itself, e.g. a go2rtc snapshot). Returns 0, or -1 on bad input
 * (result untouched).
 */
int api_detection_apply_response_geometry(const api_detection_response_meta_t *meta,
                                          int image_w, int image_h,
                                          int source_w, int source_h,
                                          detection_result_t *result,
                                          observation_evidence_transform_t *transform);

/** Pixel size of a baseline/progressive JPEG from its SOFn marker. Returns 0,
 *  or -1 when the bytes are not a parseable JPEG. */
int api_detection_jpeg_dimensions(const uint8_t *jpeg, size_t size, int *width, int *height);

int init_api_detection_system(void);

void shutdown_api_detection_system(void);

int detect_objects_api(const char *api_url, const unsigned char *frame_data,
                      int width, int height, int channels, detection_result_t *result,
                      const char *stream_name, float threshold, uint64_t recording_id,
                      time_t frame_timestamp);

/** Same as detect_objects_api with explicit request options (NULL = global config). */
int detect_objects_api_with_options(const char *api_url, const unsigned char *frame_data,
                                    int width, int height, int channels,
                                    detection_result_t *result, const char *stream_name,
                                    float threshold, uint64_t recording_id,
                                    time_t frame_timestamp,
                                    const api_detection_options_t *options);

/*
 * INT_01 L7: capture-time identity and cached evidence settings for one API
 * detection call. Built by the caller (the detection thread copies them from
 * its context; no DB read per frame). NULL, or camera_uuid NULL/"", keeps the
 * legacy behaviour: no observation and stream-name keyed DB rows.
 *
 * With a camera_uuid the detections are stored and published against that
 * immutable identity. When evidence_enabled and a surviving detection
 * (zone + object filter, confidence >= threshold) has an eligible label, the
 * exact JPEG bytes submitted to the server become the observation evidence
 * (origin submitted_image); their ownership moves to the L3 queue.
 */
typedef struct {
    const char *camera_uuid;   /* streams.camera_uuid */
    const char *engine_key;    /* NULL/"" = "legacy-primary" */
    bool evidence_enabled;     /* streams.observation_evidence_enabled (cached) */
    const char *labels_csv;    /* streams.observation_labels (cached); NULL/"" = all */
    int64_t observed_at_ms;    /* inference-input capture time; 0 = derive at call start */
} api_detection_evidence_t;

/** detect_objects_api_with_options() plus observation evidence (NULL = none). */
int detect_objects_api_with_evidence(const char *api_url, const unsigned char *frame_data,
                                     int width, int height, int channels,
                                     detection_result_t *result, const char *stream_name,
                                     float threshold, uint64_t recording_id,
                                     time_t frame_timestamp,
                                     const api_detection_options_t *options,
                                     const api_detection_evidence_t *evidence);

bool api_detection_should_use_go2rtc_snapshot(const unsigned char *frame_data,
                                              int width,
                                              int height,
                                              int channels,
                                              const char *stream_name);

int detect_objects_api_snapshot(const char *api_url, const char *stream_name,
                                detection_result_t *result, float threshold,
                                uint64_t recording_id, time_t frame_timestamp);

/** Same as detect_objects_api_snapshot with explicit request options (NULL = global config). */
int detect_objects_api_snapshot_with_options(const char *api_url, const char *stream_name,
                                             detection_result_t *result, float threshold,
                                             uint64_t recording_id, time_t frame_timestamp,
                                             const api_detection_options_t *options);

/** detect_objects_api_snapshot_with_options() plus observation evidence: the
 *  go2rtc snapshot IS the inference input, so it is the evidence. */
int detect_objects_api_snapshot_with_evidence(const char *api_url, const char *stream_name,
                                              detection_result_t *result, float threshold,
                                              uint64_t recording_id, time_t frame_timestamp,
                                              const api_detection_options_t *options,
                                              const api_detection_evidence_t *evidence);

#endif /* LIGHTNVR_API_DETECTION_H */
