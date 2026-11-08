/*
 * SPDX-License-Identifier: GPL-3.0-or-later
 * Copyright (C) 2026 OpenSensor Engineering and LightNVR contributors
 *
 * This file is part of LightNVR <https://github.com/opensensor/lightNVR>.
 */

/**
 * INT_01 observation and integration API.
 *
 * Contract: docs/integrations/OBSERVATIONS_CONTRACT.md. Every route is
 * registered now so the canonical OpenAPI operations map to native routes.
 * Until the implementations land (L8 for the GET routes, L14 for the PUT
 * route), each route authenticates the caller and then answers
 * 501 {"error", "code":"not_implemented"} (contract §11). The stubs read no
 * observation data, so no camera authorization is needed yet.
 */

#include "web/api_handlers_observations.h"

#include <cjson/cJSON.h>
#include <string.h>

#include "database/db_auth.h"
#include "web/httpd_utils.h"

static void respond_not_implemented(const http_request_t *req, http_response_t *res) {
    user_t user;
    memset(&user, 0, sizeof(user));
    if (!httpd_check_action_access(req, &user)) {
        http_response_set_json_error(res, 401, "Unauthorized");
        return;
    }
    cJSON *body = cJSON_CreateObject();
    char *json = NULL;
    if (body &&
        cJSON_AddStringToObject(body, "error", "Not implemented in this version") &&
        cJSON_AddStringToObject(body, "code", "not_implemented")) {
        json = cJSON_PrintUnformatted(body);
    }
    cJSON_Delete(body);
    if (!json) {
        http_response_set_json_error(res, 500, "Internal server error");
        return;
    }
    http_response_set_json(res, 501, json);
    cJSON_free(json);
}

void handle_get_integration_capabilities(const http_request_t *req, http_response_t *res) {
    respond_not_implemented(req, res);
}

void handle_get_observations(const http_request_t *req, http_response_t *res) {
    respond_not_implemented(req, res);
}

void handle_get_observation(const http_request_t *req, http_response_t *res) {
    respond_not_implemented(req, res);
}

void handle_get_observation_snapshot(const http_request_t *req, http_response_t *res) {
    respond_not_implemented(req, res);
}

void handle_put_observation_classification(const http_request_t *req, http_response_t *res) {
    respond_not_implemented(req, res);
}
