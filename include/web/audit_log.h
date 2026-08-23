#ifndef LIGHTNVR_WEB_AUDIT_LOG_H
#define LIGHTNVR_WEB_AUDIT_LOG_H

#include <cjson/cJSON.h>

#include "core/authorization.h"
#include "database/db_auth.h"
#include "web/request_response.h"

/*
 * Append a redacted audit event using request and authenticated-principal
 * context. details must be a JSON object and remains owned by the caller.
 * Audit persistence is best effort: failures are logged and do not change the
 * operation's authorization result.
 */
void audit_log_append(const http_request_t *req, const user_t *user,
                      const char *action, const char *target_type,
                      const char *target_uuid, const char *outcome,
                      const cJSON *details);

/*
 * Record the outcome of an authorized operation. context remains owned by the
 * caller and is nested below a standard operation.outcome envelope before the
 * same recursive redaction used by audit_log_append is applied.
 */
void audit_log_operation(const http_request_t *req, const user_t *user,
                         const char *action, const char *target_type,
                         const char *target_uuid, const char *operation,
                         const char *outcome, const cJSON *context);

void audit_log_authorization(const http_request_t *req, const user_t *user,
                             authorization_action_t action,
                             const fleet_camera_t *camera,
                             const authorization_evaluation_t *evaluation,
                             const char *outcome);

/* Record a completed authentication step without retaining credentials. */
void audit_log_login(const http_request_t *req, const user_t *user,
                     const char *attempted_username,
                     const char *authentication_method,
                     const char *outcome, const char *reason);

#endif /* LIGHTNVR_WEB_AUDIT_LOG_H */
