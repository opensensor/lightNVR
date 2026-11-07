/* SPDX-License-Identifier: GPL-3.0-or-later */
#include <stdlib.h>
#include <stdio.h>
#include <stdint.h>
#include <string.h>
#include <strings.h>
#include "web/api_handlers_openapi.h"
#include "web/httpd_utils.h"
#include "web/openapi_projection.h"
#include "web/openapi_document_generated.h"
#include "database/db_authorization_snapshot.h"
#include "utils/strings.h"

/* Static errors also work when allocation has failed. The response object owns
 * only the successful serialized result, never the embedded canonical bytes. */
static void error_response(http_response_t *response, int status, const char *body) {
    if (response->body_allocated) free(response->body);
    response->status_code = status;
    response->body = (void *)body;
    response->body_length = strlen(body);
    response->body_allocated = false;
}
static int private_headers(http_response_t *response) {
    safe_strcpy(response->content_type, "application/json", sizeof(response->content_type), 0);
    /* Avoid multiple contradictory Cache-Control headers if a wrapper added one. */
    for (int i = 0; i < response->num_headers; ) {
        if (!strcasecmp(response->headers[i].name, "Cache-Control") ||
            !strcasecmp(response->headers[i].name, "ETag")) {
            memmove(&response->headers[i], &response->headers[i+1],
                    (size_t)(response->num_headers-i-1) * sizeof(response->headers[0]));
            response->num_headers--;
        } else i++;
    }
    return http_response_add_header(response, "Cache-Control", "private, no-store");
}
/* Preserve exact large integer schema bounds through cJSON's formatter,
 * which can otherwise round 16-digit integers to 15 significant digits. */
static int preserve_exact_integer_bounds(cJSON *parent) {
    for (cJSON *item = parent->child, *next; item; item = next) {
        next = item->next;
        double value = item->valuedouble;
        if (cJSON_IsNumber(item) &&
            (value >= 1000000000000000.0 || value <= -1000000000000000.0) &&
            value >= -9007199254740991.0 && value <= 9007199254740991.0 &&
            value == (double)(int64_t)value) {
            char decimal[32];
            snprintf(decimal, sizeof(decimal), "%.0f", value);
            cJSON *exact = cJSON_CreateRaw(decimal);
            if (!exact) return -1;
            int replaced = cJSON_IsObject(parent)
                ? cJSON_ReplaceItemInObjectCaseSensitive(parent, item->string, exact)
                : cJSON_ReplaceItemViaPointer(parent, item, exact);
            if (!replaced) { cJSON_Delete(exact); return -1; }
        } else if (item->child && preserve_exact_integer_bounds(item) != 0) {
            return -1;
        }
    }
    return 0;
}

void handle_get_openapi(const http_request_t *request, http_response_t *response) {
    if (!response) return;
    error_response(response, 500, "{\"error\":\"API discovery is unavailable\"}");
    if (private_headers(response) != 0 || !request) return;
    user_t principal = {0};
    if (!httpd_check_action_access(request, &principal)) {
        error_response(response, 401, "{\"error\":\"Authentication required\"}");
        return;
    }
    if (!strcmp(principal.authentication_method, "auth_disabled") ||
        !strcmp(principal.authentication_method, "demo")) {
        error_response(response, 403, "{\"error\":\"API discovery requires an authenticated account; anonymous demo and disabled authentication are unsupported\"}");
        return;
    }
    authorization_snapshot_t *snapshot = NULL;
    int captured = authorization_snapshot_create(&principal, &snapshot);
    if (captured != 0) {
        if (captured == -2) error_response(response, 401, "{\"error\":\"Authentication required\"}");
        return;
    }
    cJSON *canonical = cJSON_ParseWithLength((const char *)lightnvr_openapi_document,
                                            lightnvr_openapi_document_length + 1);
    cJSON *projected = NULL;
    char *serialized = NULL;
    if (!canonical || openapi_project(canonical, openapi_snapshot_access, snapshot, &projected) != 0) goto done;
    cJSON *coverage = cJSON_GetObjectItemCaseSensitive(projected, "x-lightnvr-coverage");
    cJSON *revision = cJSON_CreateString(lightnvr_openapi_document_revision);
    if (!revision) goto done;
    if (!cJSON_IsObject(coverage) ||
        !cJSON_ReplaceItemInObjectCaseSensitive(coverage, "canonical_revision", revision)) {
        cJSON_Delete(revision);
        goto done;
    }
    if (preserve_exact_integer_bounds(projected) != 0) goto done;
    serialized = cJSON_PrintUnformatted(projected);
    if (!serialized) goto done;
    if (http_response_set_body(response, serialized) != 0) {
        error_response(response, 500, "{\"error\":\"API discovery is unavailable\"}");
        goto done;
    }
    response->status_code = 200;
 done:
    cJSON_free(serialized);
    cJSON_Delete(projected);
    cJSON_Delete(canonical);
    authorization_snapshot_free(snapshot);
}
