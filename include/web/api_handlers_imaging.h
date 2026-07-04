#ifndef API_HANDLERS_IMAGING_H
#define API_HANDLERS_IMAGING_H

#include "web/request_response.h"

void handle_imaging_get_settings(const http_request_t *req, http_response_t *res);
void handle_imaging_put_settings(const http_request_t *req, http_response_t *res);
void handle_imaging_get_options(const http_request_t *req, http_response_t *res);
void handle_daynight_get(const http_request_t *req, http_response_t *res);
void handle_daynight_put(const http_request_t *req, http_response_t *res);

#endif /* API_HANDLERS_IMAGING_H */
