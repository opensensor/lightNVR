#ifndef API_HANDLERS_INVESTIGATIONS_H
#define API_HANDLERS_INVESTIGATIONS_H

#include "web/request_response.h"

#define INVESTIGATION_MAX_CAMERAS 16
#define INVESTIGATION_MAX_SEGMENTS_PER_CAMERA 2048
#define INVESTIGATION_MAX_ACTION_RECORDINGS 200

/** POST /api/investigations/timeline */
void handle_post_investigation_timeline(const http_request_t *request,
                                        http_response_t *response);

/** POST /api/investigations/segment-at */
void handle_post_investigation_segment_at(const http_request_t *request,
                                          http_response_t *response);

/** POST /api/investigations/search */
void handle_post_investigation_search(const http_request_t *request,
                                      http_response_t *response);

/** POST /api/investigations/recordings/preview */
void handle_post_investigation_recording_preview(
    const http_request_t *request, http_response_t *response);

/** POST /api/investigations/thumbnail-samples */
void handle_post_investigation_thumbnail_samples(
    const http_request_t *request, http_response_t *response);

#endif /* API_HANDLERS_INVESTIGATIONS_H */
