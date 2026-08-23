#ifndef LIGHTNVR_API_HANDLERS_EVENT_ROUTES_H
#define LIGHTNVR_API_HANDLERS_EVENT_ROUTES_H

#include "web/request_response.h"

void handle_get_event_catalog(const http_request_t *req,
                              http_response_t *res);
void handle_get_event_routes(const http_request_t *req,
                             http_response_t *res);
void handle_post_event_route(const http_request_t *req,
                             http_response_t *res);
void handle_post_event_route_preview(const http_request_t *req,
                                     http_response_t *res);
void handle_get_event_route(const http_request_t *req,
                            http_response_t *res);
void handle_put_event_route(const http_request_t *req,
                            http_response_t *res);
void handle_delete_event_route(const http_request_t *req,
                               http_response_t *res);

#endif /* LIGHTNVR_API_HANDLERS_EVENT_ROUTES_H */
