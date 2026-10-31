/*
 * SPDX-License-Identifier: GPL-3.0-or-later
 * Copyright (C) 2026 OpenSensor Engineering and LightNVR contributors
 *
 * This file is part of LightNVR <https://github.com/opensensor/lightNVR>.
 */

#ifndef LIGHTNVR_API_HANDLERS_AUDIT_H
#define LIGHTNVR_API_HANDLERS_AUDIT_H

#include "web/request_response.h"

void handle_get_audit_events(const http_request_t *req, http_response_t *res);
void handle_get_audit_export(const http_request_t *req, http_response_t *res);
void handle_get_audit_settings(const http_request_t *req, http_response_t *res);
void handle_put_audit_settings(const http_request_t *req, http_response_t *res);

#endif /* LIGHTNVR_API_HANDLERS_AUDIT_H */
