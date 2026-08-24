#ifndef LIGHTNVR_API_HANDLERS_STORAGE_POLICIES_H
#define LIGHTNVR_API_HANDLERS_STORAGE_POLICIES_H

#include "web/request_response.h"

void handle_get_storage_policies(const http_request_t *req,
                                 http_response_t *res);
void handle_post_storage_policy(const http_request_t *req,
                                http_response_t *res);
void handle_post_storage_policy_preview(const http_request_t *req,
                                        http_response_t *res);
void handle_get_storage_policy(const http_request_t *req,
                               http_response_t *res);
void handle_put_storage_policy(const http_request_t *req,
                               http_response_t *res);
void handle_delete_storage_policy(const http_request_t *req,
                                  http_response_t *res);

#endif /* LIGHTNVR_API_HANDLERS_STORAGE_POLICIES_H */
