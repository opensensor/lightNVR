#ifndef LIGHTNVR_API_HANDLERS_MOTION_H
#define LIGHTNVR_API_HANDLERS_MOTION_H

#include <stddef.h>

#include <cjson/cJSON.h>

#include "database/db_recording_tags.h"
#include "video/detection_result.h"
#include "web/request_response.h"

/* Upper bound on `tags` entries accepted in one trigger request. */
#define MOTION_MAX_TAGS 16

/**
 * POST /api/motion/trigger
 *
 * External motion trigger endpoint. Lets an authenticated client (Home
 * Assistant, NodeRED, a shell script, etc.) drive the same motion-recording
 * path that ONVIF events normally drive. Intended for cameras whose native
 * ONVIF event stream is broken or missing.
 *
 * Request body (application/json):
 *   {
 *     "stream":      "<stream name>",          // required
 *     "action":      "start"|"stop"|"pulse",   // required
 *     "duration_ms": <int>,                     // pulse only, optional; default 2000, max 600000
 *     "label":       "<object class>",          // optional, e.g. "person"
 *     "objects":     [...],                     // optional; see motion_trigger_parse_objects
 *     "confidence":  <0..1>,                    // optional, default 1.0
 *     "tags":        ["<tag>", ...]             // optional; applied to the open recording
 *   }
 *
 * Auth: requires an authenticated user (X-API-Key, Authorization: Bearer, or
 * session cookie). Role must be ADMIN, USER, or API; VIEWER is rejected.
 *
 * Streams without detection_based_recording are accepted: there is no unified
 * detection thread to arm, so no recording is started, but the reported objects
 * and tags are still recorded against the stream's existing (e.g. 24/7)
 * recording. The response reports recording_triggered=false in that case.
 */
void handle_post_motion_trigger(const http_request_t *req, http_response_t *res);

/**
 * Parse the optional `label`, `confidence` and `objects` fields of a motion
 * trigger body into `result`.
 *
 * `objects` accepts bare strings (["person","vehicle"]) or per-entry
 * confidences ([{"label":"person","confidence":0.82}]), so callers can forward
 * detector output verbatim or keep it terse. `confidence` sets the default for
 * `label` and for any entry without its own. Entries past MAX_DETECTIONS are
 * ignored rather than rejected.
 *
 * Exposed (rather than static) so the validation rules can be unit-tested
 * without standing up an HTTP server and a stream.
 *
 * @param body    Parsed request body
 * @param result  Zeroed detection result to append to
 * @param err     Buffer for a caller-facing error message
 * @param err_sz  Size of `err`
 * @return 0 on success, -1 on a malformed field (with `err` populated)
 */
int motion_trigger_parse_objects(const cJSON *body, detection_result_t *result,
                                 char *err, size_t err_sz);

/**
 * Parse the optional `tags` array of strings from a motion trigger body.
 *
 * Exposed for unit testing alongside motion_trigger_parse_objects().
 *
 * @param tags    Output buffer for up to MOTION_MAX_TAGS tags
 * @return tag count on success, -1 on a malformed field (with `err` populated)
 */
int motion_trigger_parse_tags(const cJSON *body, char tags[][MAX_TAG_LENGTH],
                              char *err, size_t err_sz);

#endif /* LIGHTNVR_API_HANDLERS_MOTION_H */
