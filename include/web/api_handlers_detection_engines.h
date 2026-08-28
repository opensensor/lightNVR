#ifndef LIGHTNVR_API_HANDLERS_DETECTION_ENGINES_H
#define LIGHTNVR_API_HANDLERS_DETECTION_ENGINES_H

#include "web/request_response.h"

void handle_get_detection_engines(const http_request_t *req,
                                  http_response_t *res);
void handle_put_detection_engines(const http_request_t *req,
                                  http_response_t *res);

#endif
