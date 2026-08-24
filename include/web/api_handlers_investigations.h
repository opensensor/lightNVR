#ifndef API_HANDLERS_INVESTIGATIONS_H
#define API_HANDLERS_INVESTIGATIONS_H

#include "web/request_response.h"

#define INVESTIGATION_MAX_CAMERAS 16
#define INVESTIGATION_MAX_SEGMENTS_PER_CAMERA 2048

/** POST /api/investigations/timeline */
void handle_post_investigation_timeline(const http_request_t *request,
                                        http_response_t *response);

#endif /* API_HANDLERS_INVESTIGATIONS_H */
