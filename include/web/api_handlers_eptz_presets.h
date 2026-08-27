#ifndef LIGHTNVR_API_HANDLERS_EPTZ_PRESETS_H
#define LIGHTNVR_API_HANDLERS_EPTZ_PRESETS_H

#include "web/request_response.h"

void handle_get_eptz_presets(const http_request_t *req,
                             http_response_t *res);
void handle_post_eptz_preset(const http_request_t *req,
                             http_response_t *res);
void handle_put_eptz_preset(const http_request_t *req,
                            http_response_t *res);
void handle_delete_eptz_preset(const http_request_t *req,
                               http_response_t *res);

#endif /* LIGHTNVR_API_HANDLERS_EPTZ_PRESETS_H */
