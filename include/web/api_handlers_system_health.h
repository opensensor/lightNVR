/** @file api_handlers_system_health.h Operational host-health API handlers. */

#ifndef LIGHTNVR_API_HANDLERS_SYSTEM_HEALTH_H
#define LIGHTNVR_API_HANDLERS_SYSTEM_HEALTH_H

#include "web/request_response.h"

/** Admin-only current operational health. This is not a liveness endpoint. */
void handle_get_system_health(const http_request_t *req, http_response_t *res);

/** Admin-only, bounded, cursor-paginated incident history. */
void handle_get_system_health_incidents(const http_request_t *req,
                                        http_response_t *res);

#endif /* LIGHTNVR_API_HANDLERS_SYSTEM_HEALTH_H */
