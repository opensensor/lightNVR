#ifndef LIGHTNVR_API_HANDLERS_AUTHORIZATION_H
#define LIGHTNVR_API_HANDLERS_AUTHORIZATION_H

#include "web/request_response.h"

void handle_get_authorization_actions(const http_request_t *req,
                                      http_response_t *res);
void handle_post_authorization_simulate(const http_request_t *req,
                                        http_response_t *res);
void handle_get_authorization_roles(const http_request_t *req,
                                    http_response_t *res);
void handle_post_authorization_role(const http_request_t *req,
                                    http_response_t *res);
void handle_put_authorization_role(const http_request_t *req,
                                   http_response_t *res);
void handle_delete_authorization_role(const http_request_t *req,
                                      http_response_t *res);
void handle_get_user_authorization(const http_request_t *req,
                                   http_response_t *res);
void handle_put_user_authorization(const http_request_t *req,
                                   http_response_t *res);

#endif /* LIGHTNVR_API_HANDLERS_AUTHORIZATION_H */
