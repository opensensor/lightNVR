/*
 * SPDX-License-Identifier: GPL-3.0-or-later
 * Copyright (C) 2026 OpenSensor Engineering and LightNVR contributors
 *
 * This file is part of LightNVR <https://github.com/opensensor/lightNVR>.
 */

/**
 * @file api_handlers_metrics.h
 * @brief Prometheus metrics endpoint and player telemetry ingest handlers
 */

#ifndef LIGHTNVR_API_HANDLERS_METRICS_H
#define LIGHTNVR_API_HANDLERS_METRICS_H

#include "web/request_response.h"

/**
 * GET /api/metrics
 * Returns all stream health metrics in Prometheus exposition text format.
 */
void handle_get_metrics(const http_request_t *req, http_response_t *res);

/**
 * POST /api/telemetry/player
 * Accepts client-side playback QoE events from the web player.
 * Returns 204 No Content.
 */
void handle_post_player_telemetry(const http_request_t *req, http_response_t *res);

#endif /* LIGHTNVR_API_HANDLERS_METRICS_H */
