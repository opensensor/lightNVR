/*
 * SPDX-License-Identifier: GPL-3.0-or-later
 * Copyright (C) 2026 OpenSensor Engineering and LightNVR contributors
 *
 * This file is part of LightNVR <https://github.com/opensensor/lightNVR>.
 */

#ifndef LIGHTNVR_API_HANDLERS_OBSERVATIONS_H
#define LIGHTNVR_API_HANDLERS_OBSERVATIONS_H

#include <stddef.h>
#include <stdint.h>

#include <cjson/cJSON.h>

#include "database/db_observations.h"
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

/* ---- Shared serialization (also used by L14) ------------------------- */

#define OBS_WRITER_NAMESPACE_SIZE 33   /* ^[a-z0-9][a-z0-9_.-]{0,31}$ */
#define OBS_TIME_TEXT_SIZE 32          /* 2026-11-08T14:03:12.481Z */

/* UTC RFC 3339 with milliseconds and a Z suffix (contract section 2). */
int observation_format_time(int64_t ms, char *out, size_t out_size);
/* Parse RFC 3339 (fractional seconds optional, Z or +-hh:mm offset). 0 on success. */
int observation_parse_time(const char *text, int64_t *ms);

/* Contract section 10.2 / D3: normalized username. Returns 0 and fills out
 * when the principal has a valid, non-reserved namespace; otherwise returns
 * -1 (invalid) or -2 (reserved) and leaves out empty. */
int observation_writer_namespace(const char *username, char out[OBS_WRITER_NAMESPACE_SIZE]);

/* Observation object (contract section 3) for one DB record. Never emits the
 * internal asset path. Returns NULL on allocation failure. */
cJSON *observation_record_to_json(const observation_record_t *record, const char *source);

#endif /* LIGHTNVR_API_HANDLERS_OBSERVATIONS_H */
