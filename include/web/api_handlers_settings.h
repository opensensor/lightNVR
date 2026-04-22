#ifndef API_HANDLERS_SETTINGS_H
#define API_HANDLERS_SETTINGS_H

#include "web/web_server.h"

/**
 * Handle GET request for settings
 */
void handle_get_settings(const http_request_t *request, http_response_t *response);

/**
 * Handle POST request for settings
 */
void handle_post_settings(const http_request_t *request, http_response_t *response);

/**
 * Handle POST /api/settings/go2rtc/validate.
 *
 * Body: { "override": "<yaml-string>" }
 * Response (200): { valid: bool, error?: { line, column, message },
 *                    warnings: [string], libyaml_available: bool }
 *
 * The same validator runs inside handle_post_settings before saving the
 * `go2rtc_config_override` field, so this endpoint exists to give the UI
 * a way to validate without persisting (blur events, "Save" pre-flight).
 */
void handle_post_settings_go2rtc_validate(const http_request_t *request,
                                          http_response_t *response);

#endif /* API_HANDLERS_SETTINGS_H */
