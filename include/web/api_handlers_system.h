#ifndef API_HANDLERS_SYSTEM_H
#define API_HANDLERS_SYSTEM_H

#include "web/web_server.h"

/**
 * Handle GET request for system information
 */
void handle_get_system_info(const http_request_t *request, http_response_t *response);

/**
 * Handle GET request for system logs
 */
void handle_get_system_logs(const http_request_t *request, http_response_t *response);

/**
 * Handle POST request to restart the service
 */
void handle_post_system_restart(const http_request_t *request, http_response_t *response);

/**
 * Handle POST request to shutdown the service
 */
void handle_post_system_shutdown(const http_request_t *request, http_response_t *response);

/**
 * Handle POST request to clear system logs
 */
void handle_post_system_clear_logs(const http_request_t *request, http_response_t *response);

/**
 * Handle POST request to backup configuration
 */
void handle_post_system_backup(const http_request_t *request, http_response_t *response);

/**
 * Handle GET request for system status
 */
void handle_get_system_status(const http_request_t *request, http_response_t *response);

/**
 * Handle GET /api/system/go2rtc/effective-config.
 *
 * Returns a JSON object with the redacted contents of the lightNVR-generated
 * base config and the user override file, in the order go2rtc loads them:
 *
 *   {
 *     "base": "<yaml>",
 *     "override": "<yaml>",
 *     "merged_source_order": ["go2rtc.yaml", "override.yaml"],
 *     "redaction_available": true|false,
 *     "go2rtc_initialized": true|false,
 *     "warnings": [...]    // populated when files are missing or unreadable
 *   }
 *
 * Sensitive scalar values (api/rtsp/mqtt passwords, ICE credentials, RTSP
 * URL userinfo) are masked via a YAML-aware walker so block scalars are
 * caught.  When libyaml is unavailable, redaction is skipped and the
 * `redaction_available` flag is false — the UI must surface that prominently.
 */
void handle_get_system_go2rtc_effective_config(const http_request_t *request,
                                                http_response_t *response);

/**
 * @brief GET /api/system/go2rtc/override-status
 *
 * Diagnostic summary of the go2rtc override pipeline: compares the DB value
 * in `go2rtc_config_override` against the on-disk `override.yaml` and reports
 * quarantine / upgrade-validation state. Answers issue #394-shaped
 * reports ("my override isn't taking effect") without shell access to the
 * container — operators can see exactly where the chain breaks (DB empty vs
 * file missing vs quarantined vs byte-for-byte mismatch).
 *
 * No YAML content is returned — only sizes + a content hash — so this is
 * safe to render without redaction.
 */
void handle_get_system_go2rtc_override_status(const http_request_t *request,
                                               http_response_t *response);

#endif /* API_HANDLERS_SYSTEM_H */
