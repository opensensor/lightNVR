/*
 * SPDX-License-Identifier: GPL-3.0-or-later
 * Copyright (C) 2026 OpenSensor Engineering and LightNVR contributors
 *
 * This file is part of LightNVR <https://github.com/opensensor/lightNVR>.
 */

#ifndef LIGHTNVR_API_HANDLERS_LPR_H
#define LIGHTNVR_API_HANDLERS_LPR_H

#include "web/request_response.h"

/** Time-scoped protected search. Plate criteria are accepted only in JSON. */
void handle_post_lpr_search(const http_request_t *req, http_response_t *res);

/** Audited JSON export with the same mandatory camera/time scope. */
void handle_post_lpr_export(const http_request_t *req, http_response_t *res);

/** Permanently delete one protected read by opaque UUID. */
void handle_delete_lpr_read(const http_request_t *req, http_response_t *res);

#endif
