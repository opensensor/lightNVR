#ifndef API_HANDLERS_LOCATIONS_H
#define API_HANDLERS_LOCATIONS_H

#include "web/request_response.h"

void handle_get_locations(const http_request_t *req, http_response_t *res);
void handle_post_location(const http_request_t *req, http_response_t *res);
void handle_get_location(const http_request_t *req, http_response_t *res);
void handle_put_location(const http_request_t *req, http_response_t *res);
void handle_delete_location(const http_request_t *req, http_response_t *res);
void handle_put_camera_location(const http_request_t *req,
                                http_response_t *res);

#endif /* API_HANDLERS_LOCATIONS_H */
