#ifndef LIGHTNVR_API_HANDLERS_LIVE_LAYOUTS_H
#define LIGHTNVR_API_HANDLERS_LIVE_LAYOUTS_H

#include "web/request_response.h"

void handle_get_live_layouts(const http_request_t *req,
                             http_response_t *res);
void handle_post_live_layout(const http_request_t *req,
                             http_response_t *res);
void handle_put_live_layout(const http_request_t *req,
                            http_response_t *res);
void handle_delete_live_layout(const http_request_t *req,
                               http_response_t *res);

#endif /* LIGHTNVR_API_HANDLERS_LIVE_LAYOUTS_H */
