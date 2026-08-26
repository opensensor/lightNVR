#ifndef LIGHTNVR_API_HANDLERS_STORAGE_POOLS_H
#define LIGHTNVR_API_HANDLERS_STORAGE_POOLS_H

#include "web/request_response.h"

void handle_get_storage_pools(const http_request_t *req, http_response_t *res);
void handle_post_storage_pool(const http_request_t *req, http_response_t *res);
void handle_get_storage_pool(const http_request_t *req, http_response_t *res);
void handle_put_storage_pool(const http_request_t *req, http_response_t *res);
void handle_delete_storage_pool(const http_request_t *req, http_response_t *res);

#endif
