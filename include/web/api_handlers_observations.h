/*
 * SPDX-License-Identifier: GPL-3.0-or-later
 * Copyright (C) 2026 OpenSensor Engineering and LightNVR contributors
 *
 * This file is part of LightNVR <https://github.com/opensensor/lightNVR>.
 */

#ifndef LIGHTNVR_API_HANDLERS_OBSERVATIONS_H
#define LIGHTNVR_API_HANDLERS_OBSERVATIONS_H

#include "web/request_response.h"

/*
 * INT_01 observation and integration API
 * (docs/integrations/OBSERVATIONS_CONTRACT.md). Operation IDs are in
 * docs/api/openapi.json.
 */

/** GET /api/integrations/capabilities (getIntegrationCapabilities). */
void handle_get_integration_capabilities(const http_request_t *req, http_response_t *res);

/** GET /api/observations (listObservations): cursor-paged history. */
void handle_get_observations(const http_request_t *req, http_response_t *res);

/** GET /api/observations/{id} (getObservation). */
void handle_get_observation(const http_request_t *req, http_response_t *res);

/** GET /api/observations/{id}/snapshot (getObservationSnapshot): evidence JPEG. */
void handle_get_observation_snapshot(const http_request_t *req, http_response_t *res);

/**
 * PUT /api/observations/{id}/classifications/{namespace}/{name}
 * (putObservationClassification, P2).
 */
void handle_put_observation_classification(const http_request_t *req, http_response_t *res);

#endif /* LIGHTNVR_API_HANDLERS_OBSERVATIONS_H */
