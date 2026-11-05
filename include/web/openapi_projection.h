/* SPDX-License-Identifier: GPL-3.0-or-later */
#ifndef LIGHTNVR_OPENAPI_PROJECTION_H
#define LIGHTNVR_OPENAPI_PROJECTION_H
#include <cjson/cJSON.h>

/* 1 allowed, 0 denied, -1 evaluation failure. Predicates are validated before
 * invocation. Result is request-owned and canonical input is never modified. */
typedef int (*openapi_access_fn)(void *context, const cJSON *predicate);
int openapi_project(const cJSON *canonical, openapi_access_fn access,
                    void *context, cJSON **result);

/* Adapter for a borrowed authorization_snapshot_t. Uses the existing evaluator
 * for every requirement on ONE camera, including its exact token constraints. */
int openapi_snapshot_access(void *snapshot, const cJSON *predicate);
#endif
