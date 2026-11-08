/*
 * SPDX-License-Identifier: GPL-3.0-or-later
 * Copyright (C) 2026 OpenSensor Engineering and LightNVR contributors
 *
 * This file is part of LightNVR <https://github.com/opensensor/lightNVR>.
 */

/*
 * INT_01 L6: local detector -> observation evidence glue (contract:
 * docs/integrations/OBSERVATIONS_CONTRACT.md sections 3 and 4.4).
 *
 * A sidecar to detection_result_t (whose layout the generated Rust and the
 * tests depend on): the engine provenance, the per-camera evidence settings
 * and the inference transform travel alongside the result instead of inside
 * it.
 *
 * The capture happens on the thread that ran inference, while the decoded
 * frame's RGB24 buffer is still alive (inside detect_on_decoded_frame in
 * unified_detection_thread.c). That covers the main-stream keyframe path and
 * the detection-stream background thread alike: the detection-stream result
 * slot only carries the detection_result_t to the packet thread, and the
 * evidence has already been handed to the L3 queue against the detection
 * stream's own resolution by then.
 *
 * Nothing here touches the database or blocks: settings are cached in the
 * detection context, IDs come from the local UUID generator and the hand-off
 * is the non-blocking observation_evidence_submit().
 */

#ifndef LIGHTNVR_VIDEO_DETECTION_EVIDENCE_H
#define LIGHTNVR_VIDEO_DETECTION_EVIDENCE_H

#include <stdbool.h>
#include <stdint.h>
#include <time.h>

#include "core/observation_evidence.h"
#include "database/db_observations.h"
#include "video/detection_model.h"
#include "video/detection_result.h"

/* Detector provenance for one batch (contract section 3.3). */
typedef struct {
    const char *engine_key;   /* stream_detection_engines.engine_key; NULL/"" = "legacy-primary" */
    const char *engine_type;  /* must be "object" for local inference (motion never observes) */
    const char *model_path;   /* reduced to its basename; NULL/"" = model null */
    const char *model_version;/* NULL/"" = null */
} detection_evidence_engine_t;

/* Capture-time camera identity and its cached evidence settings. */
typedef struct {
    const char *camera_uuid;  /* streams.camera_uuid (capture-time identity) */
    const char *stream_name;  /* stream_name_at_capture (display only) */
    bool evidence_enabled;    /* streams.observation_evidence_enabled */
    const char *labels_csv;   /* streams.observation_labels; NULL/"" = every object label */
    int64_t recording_id;     /* recording open at capture (in-memory), 0 = none */
} detection_evidence_camera_t;

typedef enum {
    DETECTION_EVIDENCE_SUBMITTED = 0,     /* handed to observation_evidence_submit(); see *submit_result */
    DETECTION_EVIDENCE_NOT_ENABLED,       /* evidence off for the camera */
    DETECTION_EVIDENCE_NOT_ELIGIBLE,      /* no surviving object has an eligible label */
    DETECTION_EVIDENCE_NOT_OBJECT_ENGINE, /* motion / onvif / api: not this path's job */
    DETECTION_EVIDENCE_ERROR              /* invalid input or UUID failure */
} detection_evidence_status_t;

/* UUIDv4 generated once per process (contract section 3.4 track.session).
 * Thread-safe; returns "" only if the generator fails. */
const char *detection_evidence_boot_session(void);

/* True when label is eligible under the camera's observation_labels CSV:
 * an empty/NULL list admits every label; otherwise a case-insensitive match
 * against any comma-separated entry (surrounding blanks ignored). */
bool detection_evidence_label_eligible(const char *labels_csv, const char *label);

/* Clamp a normalized xywh box into [0,1] with x + width <= 1, y + height <= 1.
 * Returns false when nothing of positive area remains. */
bool detection_evidence_clamp_box(double x, double y, double w, double h,
                                  double *ox, double *oy, double *ow, double *oh);

/*
 * Build the observation batch for count surviving detections of one engine
 * (pure apart from the batch_id UUIDv4). Objects whose label is ineligible,
 * empty or too long, whose confidence is not finite, or whose box clamps to
 * nothing (that object then gets no box rather than being dropped) are
 * handled per contract; object_index is the position among the eligible
 * objects. Track IDs are emitted only for track_id >= 0 with namespace =
 * engine key and session = detection_evidence_boot_session().
 *
 * Returns the object count written to out (0 = nothing eligible), or -1 on
 * invalid input.
 */
int detection_evidence_build_batch(const detection_evidence_camera_t *camera,
                                   const detection_evidence_engine_t *engine,
                                   const detection_t *detections, int count,
                                   int64_t observed_at_ms,
                                   observation_batch_input_t *out);

/*
 * Inference transform for a frame of frame_w x frame_h that is stored as-is
 * (contract section 4.4; boxes from local engines are already normalized to
 * the full decoded frame, so the stored image is that frame):
 *   - net_w/net_h <= 0 or equal to the frame: mode none, scale 1;
 *   - letterbox false: mode resize, scale_x = net_w / frame_w, scale_y = net_h / frame_h
 *     (SOD: sod_cnn_prepare_image resizes independently per axis);
 *   - letterbox true: mode letterbox with the uniform scale and centred
 *     integer padding computed exactly as litert_engine.cc letterbox_rgb24().
 */
void detection_evidence_transform_compute(int frame_w, int frame_h,
                                          int net_w, int net_h, bool letterbox,
                                          observation_evidence_transform_t *out);

/* Model input geometry of a loaded embedded model: SOD CNN network size
 * (resize), LiteRT input tensor (letterbox). Returns 0 and fills the outputs,
 * or -1 when the model has no fixed input (RealNet, unknown, NULL) so the
 * caller records mode none. */
int detection_evidence_model_geometry(detection_model_t model, int *net_w, int *net_h,
                                      bool *letterbox);

/* Millisecond capture time for a frame stamped with whole seconds: the wall
 * clock when it is still inside frame_ts's second, else frame_ts * 1000. */
int64_t detection_evidence_observed_at_ms(time_t frame_ts);

/*
 * Build the batch and hand the owned RGB24 frame to the L3 queue. Takes
 * ownership of rgb24 on EVERY return (freed when not submitted). rgb24 must
 * carry RGB24_SWS_TAIL_PADDING slack (ffmpeg_frame_to_rgb24_padded()).
 * transform NULL = mode none at width x height. submit_result (optional)
 * receives the L3 result when DETECTION_EVIDENCE_SUBMITTED.
 */
detection_evidence_status_t detection_evidence_capture(
    const detection_evidence_camera_t *camera,
    const detection_evidence_engine_t *engine,
    const detection_t *detections, int count,
    int64_t observed_at_ms,
    uint8_t *rgb24, int width, int height,
    const observation_evidence_transform_t *transform,
    observation_evidence_result_t *submit_result,
    char asset_uuid_out[OBS_UUID_SIZE]);

#endif /* LIGHTNVR_VIDEO_DETECTION_EVIDENCE_H */
