#ifndef LIGHTNVR_API_HANDLERS_MOTION_H
#define LIGHTNVR_API_HANDLERS_MOTION_H

#include "web/request_response.h"

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
 *     "duration_ms": <int>                      // pulse only, optional; default 2000, max 600000
 *   }
 *
 * Auth: requires an authenticated user (X-API-Key, Authorization: Bearer, or
 * session cookie). Role must be ADMIN, USER, or API; VIEWER is rejected.
 *
 * Target stream must have detection_based_recording enabled — otherwise the
 * unified detection thread is not running and the trigger has nothing to drive.
 */
void handle_post_motion_trigger(const http_request_t *req, http_response_t *res);

#endif /* LIGHTNVR_API_HANDLERS_MOTION_H */
