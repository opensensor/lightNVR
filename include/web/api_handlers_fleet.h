/*
 * SPDX-License-Identifier: GPL-3.0-or-later
 * Copyright (C) 2026 OpenSensor Engineering and LightNVR contributors
 *
 * This file is part of LightNVR <https://github.com/opensensor/lightNVR>.
 */

#ifndef LIGHTNVR_API_HANDLERS_FLEET_H
#define LIGHTNVR_API_HANDLERS_FLEET_H

#include "web/request_response.h"

void handle_post_fleet_camera_query(const http_request_t *req,
                                    http_response_t *res);
void handle_post_fleet_selector_preview(const http_request_t *req,
                                        http_response_t *res);

#endif /* LIGHTNVR_API_HANDLERS_FLEET_H */
