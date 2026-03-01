#ifndef API_HANDLERS_SETUP_H
#define API_HANDLERS_SETUP_H

#include "web/http_server.h"
#include "web/request_response.h"

/**
 * @brief GET /api/setup/status
 *
 * Returns the current setup wizard state.  No authentication required so that
 * the frontend can always determine whether to show the wizard.
 *
 * Response:
 * {
 *   "complete": bool,
 *   "setup_completed_at": int | null   // epoch seconds, 0 when not complete
 * }
 */
void handle_get_setup_status(const http_request_t *req, http_response_t *res);

/**
 * @brief POST /api/setup/complete
 *
 * Marks the setup wizard as finished.  Accepts an empty body or:
 * { "complete": true }
 *
 * Response: { "success": true }
 */
void handle_post_setup_complete(const http_request_t *req, http_response_t *res);

#endif /* API_HANDLERS_SETUP_H */

