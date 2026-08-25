#ifndef LIGHTNVR_API_HANDLERS_FLEET_VIEWS_H
#define LIGHTNVR_API_HANDLERS_FLEET_VIEWS_H

#include "web/request_response.h"

void handle_get_fleet_saved_views(const http_request_t *req,
                                  http_response_t *res);
void handle_post_fleet_saved_view(const http_request_t *req,
                                  http_response_t *res);
void handle_get_fleet_saved_view(const http_request_t *req,
                                 http_response_t *res);
void handle_put_fleet_saved_view(const http_request_t *req,
                                 http_response_t *res);
void handle_delete_fleet_saved_view(const http_request_t *req,
                                    http_response_t *res);

#endif /* LIGHTNVR_API_HANDLERS_FLEET_VIEWS_H */
