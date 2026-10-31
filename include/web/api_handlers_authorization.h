/*
 * SPDX-License-Identifier: GPL-3.0-or-later
 * Copyright (C) 2026 OpenSensor Engineering and LightNVR contributors
 *
 * This file is part of LightNVR <https://github.com/opensensor/lightNVR>.
 */

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
void handle_get_user_api_tokens(const http_request_t *req,
                                http_response_t *res);
void handle_post_user_api_token(const http_request_t *req,
                                http_response_t *res);
void handle_delete_user_api_token(const http_request_t *req,
                                  http_response_t *res);

#endif /* LIGHTNVR_API_HANDLERS_AUTHORIZATION_H */
