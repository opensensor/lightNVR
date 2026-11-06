/*
 * SPDX-License-Identifier: GPL-3.0-or-later
 * Copyright (C) 2026 OpenSensor Engineering and LightNVR contributors
 *
 * This file is part of LightNVR <https://github.com/opensensor/lightNVR>.
 */

#ifndef LIGHTNVR_RTSP_PROBE_H
#define LIGHTNVR_RTSP_PROBE_H

#include <stddef.h>

/* Probe server availability, not a specific stream or authentication. Sends a
 * constant OPTIONS * request: URL userinfo, paths and tokens never go on wire.
 * Any well-formed RTSP/1.0 status (including 401/404/405) establishes availability;
 * FFmpeg performs the actual resource/authentication exchange. Returns 0 or -1.
 * The socket exchange is bounded to two seconds after hostname resolution. */
int rtsp_probe_server(const char *url, char *host, size_t host_size, int *port);

#endif
