/*
 * SPDX-License-Identifier: GPL-3.0-or-later
 * Copyright (C) 2026 OpenSensor Engineering and LightNVR contributors
 *
 * This file is part of LightNVR <https://github.com/opensensor/lightNVR>.
 */

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
