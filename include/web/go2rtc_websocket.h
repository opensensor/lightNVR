/*
 * SPDX-License-Identifier: GPL-3.0-or-later
 * Copyright (C) 2026 OpenSensor Engineering and LightNVR contributors
 *
 * This file is part of LightNVR <https://github.com/opensensor/lightNVR>.
 */

#ifndef GO2RTC_WEBSOCKET_H
#define GO2RTC_WEBSOCKET_H

#ifdef HTTP_BACKEND_LIBUV
#include "web/libuv_server.h"

/* Validate the upgrade and same-origin policy after camera authorization. */
bool go2rtc_websocket_validate(const http_request_t *req, http_response_t *res);
/* Event-loop only. The URL must come from go2rtc_proxy_build_url after auth. */
void go2rtc_websocket_start(libuv_connection_t *conn, const char *url);
void go2rtc_websocket_disconnected(libuv_connection_t *conn);
void go2rtc_websocket_shutdown(uv_loop_t *loop);
#endif
#endif
