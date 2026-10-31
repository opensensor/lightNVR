/*
 * SPDX-License-Identifier: GPL-3.0-or-later
 * Copyright (C) 2026 OpenSensor Engineering and LightNVR contributors
 *
 * This file is part of LightNVR <https://github.com/opensensor/lightNVR>.
 */

#ifndef LIGHTNVR_API_HANDLERS_EVENT_DESTINATIONS_H
#define LIGHTNVR_API_HANDLERS_EVENT_DESTINATIONS_H

#include "web/request_response.h"

void handle_get_event_destinations(const http_request_t *req,
                                   http_response_t *res);
void handle_post_event_destination(const http_request_t *req,
                                   http_response_t *res);
void handle_get_event_destination(const http_request_t *req,
                                  http_response_t *res);
void handle_put_event_destination(const http_request_t *req,
                                  http_response_t *res);
void handle_delete_event_destination(const http_request_t *req,
                                     http_response_t *res);

#endif /* LIGHTNVR_API_HANDLERS_EVENT_DESTINATIONS_H */
