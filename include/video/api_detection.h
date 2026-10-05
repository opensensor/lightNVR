#ifndef LIGHTNVR_API_DETECTION_H
#define LIGHTNVR_API_DETECTION_H

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>
#include <time.h>

#include "core/config.h"
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

#endif /* LIGHTNVR_API_DETECTION_H */
