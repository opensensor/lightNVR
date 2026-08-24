#define _GNU_SOURCE

#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <stdbool.h>
#include <ctype.h>
#include <string.h>
#include <strings.h>
#include <arpa/inet.h>

#include "web/httpd_utils.h"
#include "web/request_response.h"
#define LOG_COMPONENT "HTTP"
#include "core/logger.h"
#include "core/config.h"
#include "utils/strings.h"
#include "database/db_auth.h"
#include "database/db_api_tokens.h"
#include "database/db_fleet_query.h"
#include "database/db_streams.h"
#include "web/audit_log.h"

cJSON* httpd_parse_json_body(const http_request_t *req) {
    if (!req || !req->body || req->body_len == 0) {
        return NULL;
    }

    // Make a null-terminated copy of the body
    char *body = malloc(req->body_len + 1);
    if (!body) {
        log_error("Failed to allocate memory for request body");
        return NULL;
    }

    memcpy(body, req->body, req->body_len);
    body[req->body_len] = '\0';

    cJSON *json = cJSON_Parse(body);
    free(body);

    if (!json) {
        log_error("Failed to parse JSON from request body");
        return NULL;
    }

    return json;
}

/**
 * Base64 decode helper for Basic Auth
 */
static int base64_decode(const char *src, size_t src_len, char *dst, size_t dst_size) {
    static const unsigned char table[256] = {
        ['A']=0,['B']=1,['C']=2,['D']=3,['E']=4,['F']=5,['G']=6,['H']=7,
        ['I']=8,['J']=9,['K']=10,['L']=11,['M']=12,['N']=13,['O']=14,['P']=15,
        ['Q']=16,['R']=17,['S']=18,['T']=19,['U']=20,['V']=21,['W']=22,['X']=23,
        ['Y']=24,['Z']=25,['a']=26,['b']=27,['c']=28,['d']=29,['e']=30,['f']=31,
        ['g']=32,['h']=33,['i']=34,['j']=35,['k']=36,['l']=37,['m']=38,['n']=39,
        ['o']=40,['p']=41,['q']=42,['r']=43,['s']=44,['t']=45,['u']=46,['v']=47,
        ['w']=48,['x']=49,['y']=50,['z']=51,['0']=52,['1']=53,['2']=54,['3']=55,
        ['4']=56,['5']=57,['6']=58,['7']=59,['8']=60,['9']=61,['+']=62,['/']=63
    };

    size_t j = 0;
    unsigned int accum = 0;
    int bits = 0;

    for (size_t i = 0; i < src_len && src[i] != '='; i++) {
        unsigned char c = (unsigned char)src[i];
        if (c == ' ' || c == '\n' || c == '\r') continue;
        accum = (accum << 6) | table[c];
        bits += 6;
        if (bits >= 8) {
            bits -= 8;
            if (j < dst_size - 1) {
                dst[j++] = (char)((accum >> bits) & 0xFF);
            }
        }
    }
    dst[j] = '\0';
    return (int)j;
}

static bool normalize_ip_literal(const char *input, char *normalized, size_t normalized_size) {
    if (!input || !normalized || normalized_size == 0) {
        return false;
    }

    normalized[0] = '\0';

    char candidate_buffer[INET6_ADDRSTRLEN + 8];
    if (strlen(input) >= sizeof(candidate_buffer)) {
        return false;
    }
    safe_strcpy(candidate_buffer, input, sizeof(candidate_buffer), 0);

    char *candidate = trim_ascii_whitespace(candidate_buffer);
    if (!candidate || candidate[0] == '\0') {
        return false;
    }

    if (candidate[0] == '[') {
        char *closing = strchr(candidate, ']');
        if (!closing || closing[1] != '\0') {
            return false;
        }
        *closing = '\0';
        candidate++;
    }

    unsigned char addr[sizeof(struct in6_addr)] = {0};
    if (inet_pton(AF_INET, candidate, addr) == 1) {
        return inet_ntop(AF_INET, addr, normalized, (socklen_t)normalized_size) != NULL;
    }
    if (inet_pton(AF_INET6, candidate, addr) == 1) {
        return inet_ntop(AF_INET6, addr, normalized, (socklen_t)normalized_size) != NULL;
    }
    return false;
}

static bool ip_matches_cidr_list(const char *cidr_list, const char *ip) {
    if (!cidr_list || cidr_list[0] == '\0' || !ip || ip[0] == '\0') {
        return false;
    }

    user_t rules;
    memset(&rules, 0, sizeof(rules));
    safe_strcpy(rules.allowed_login_cidrs, cidr_list, sizeof(rules.allowed_login_cidrs), 0);
    rules.has_login_cidr_restriction = true;
    return db_auth_ip_allowed_for_user(&rules, ip);
}

static int resolve_client_ip_from_forwarded_headers(const http_request_t *req,
                                                    char *client_ip,
                                                    size_t client_ip_size) {
    const char *xff = http_request_get_header(req, "X-Forwarded-For");
    if (xff && xff[0] != '\0') {
        char xff_copy[1024];
        if (strlen(xff) >= sizeof(xff_copy)) {
            return -1;
        }
        safe_strcpy(xff_copy, xff, sizeof(xff_copy), 0);

        char normalized_tokens[16][INET6_ADDRSTRLEN] = {{0}};
        int token_count = 0;
        char *saveptr = NULL;
        for (char *token = strtok_r(xff_copy, ",", &saveptr);
             token != NULL && token_count < 16;
             token = strtok_r(NULL, ",", &saveptr)) {
            char normalized[INET6_ADDRSTRLEN] = {0};
            if (!normalize_ip_literal(token, normalized, sizeof(normalized))) {
                return -1;
            }
            safe_strcpy(normalized_tokens[token_count], normalized, sizeof(normalized_tokens[token_count]), 0);
            token_count++;
        }

        if (token_count > 0) {
            for (int i = token_count - 1; i >= 0; --i) {
                if (!ip_matches_cidr_list(g_config.trusted_proxy_cidrs, normalized_tokens[i])) {
                    safe_strcpy(client_ip, normalized_tokens[i], client_ip_size, 0);
                    return 0;
                }
            }
        }
    }

    const char *x_real_ip = http_request_get_header(req, "X-Real-IP");
    if (x_real_ip && x_real_ip[0] != '\0') {
        char normalized[INET6_ADDRSTRLEN] = {0};
        if (!normalize_ip_literal(x_real_ip, normalized, sizeof(normalized))) {
            return -1;
        }
        safe_strcpy(client_ip, normalized, client_ip_size, 0);
        return 0;
    }

    return -1;
}

int httpd_get_basic_auth_credentials(const http_request_t *req,
                                      char *username, size_t username_size,
                                      char *password, size_t password_size) {
    if (!req || !username || !password) return -1;

    username[0] = '\0';
    password[0] = '\0';

    const char *auth = http_request_get_header(req, "Authorization");
    if (!auth) return -1;

    // Check for "Basic " prefix
    if (strncasecmp(auth, "Basic ", 6) != 0) return -1;

    const char *encoded = auth + 6;
    while (*encoded == ' ') encoded++;

    // Decode base64
    char decoded[256] = {0};
    base64_decode(encoded, strlen(encoded), decoded, sizeof(decoded));

    // Split at ':'
    char *colon = strchr(decoded, ':');
    if (!colon) return -1;

    *colon = '\0';
    safe_strcpy(username, decoded, username_size, 0);
    safe_strcpy(password, colon + 1, password_size, 0);

    return 0;
}

int httpd_get_api_key(const http_request_t *req, char *api_key, size_t api_key_size) {
    if (!req || !api_key || api_key_size == 0) {
        return -1;
    }

    api_key[0] = '\0';

    const char *header_value = http_request_get_header(req, "X-API-Key");
    if (header_value && copy_trimmed_value(api_key, api_key_size, header_value, 0)) {
        return 0;
    }

    const char *auth = http_request_get_header(req, "Authorization");
    if (!auth || strncasecmp(auth, "Bearer ", 7) != 0) {
        return -1;
    }

    return copy_trimmed_value(api_key, api_key_size, auth + 7, 0) ? 0 : -1;
}

bool httpd_peer_is_trusted_proxy(const http_request_t *req) {
    if (!req || req->client_ip[0] == '\0') return false;
    if (g_config.trusted_proxy_cidrs[0] == '\0') return false;
    char peer_ip[INET6_ADDRSTRLEN] = {0};
    if (!normalize_ip_literal(req->client_ip, peer_ip, sizeof(peer_ip))) {
        return false;
    }
    return ip_matches_cidr_list(g_config.trusted_proxy_cidrs, peer_ip);
}

int httpd_get_effective_client_ip(const http_request_t *req, char *client_ip, size_t client_ip_size) {
    if (!req || !client_ip || client_ip_size == 0) {
        return -1;
    }

    client_ip[0] = '\0';
    if (req->client_ip[0] == '\0') {
        return -1;
    }

    char peer_ip[INET6_ADDRSTRLEN] = {0};
    if (!normalize_ip_literal(req->client_ip, peer_ip, sizeof(peer_ip))) {
        return copy_trimmed_value(client_ip, client_ip_size, req->client_ip, 0) ? 0 : -1;
    }

    if (g_config.trusted_proxy_cidrs[0] != '\0' &&
        ip_matches_cidr_list(g_config.trusted_proxy_cidrs, peer_ip) &&
        resolve_client_ip_from_forwarded_headers(req, client_ip, client_ip_size) == 0) {
        return 0;
    }

    safe_strcpy(client_ip, peer_ip, client_ip_size, 0);
    return 0;
}

int httpd_get_cookie_value(const http_request_t *req, const char *cookie_name,
                           char *value, size_t value_size) {
    if (!req || !cookie_name || !value || value_size == 0) return -1;

    value[0] = '\0';
    const char *cookie = http_request_get_header(req, "Cookie");
    if (!cookie) return -1;

    size_t name_len = strlen(cookie_name);
    const char *cursor = cookie;

    while (cursor && *cursor) {
        while (*cursor == ' ' || *cursor == ';') cursor++;
        if (strncmp(cursor, cookie_name, name_len) == 0 && cursor[name_len] == '=') {
            const char *value_start = cursor + name_len + 1;
            const char *value_end = strchr(value_start, ';');
            size_t len = value_end ? (size_t)(value_end - value_start) : strlen(value_start);
            if (len == 0 || len >= value_size) return -1;
            memcpy(value, value_start, len);
            value[len] = '\0';
            return 0;
        }
        cursor = strchr(cursor, ';');
        if (cursor) cursor++;
    }

    return -1;
}

int httpd_get_session_token(const http_request_t *req, char *token, size_t token_size) {
    return httpd_get_cookie_value(req, "session", token, token_size);
}

int httpd_auth_absolute_timeout_seconds(void) {
    int64_t seconds = (int64_t)g_config.auth_absolute_timeout_hours * 3600;
    if (seconds <= 0 || seconds > INT32_MAX) {
        return 604800;
    }
    return (int)seconds;
}

int httpd_trusted_device_lifetime_seconds(void) {
    int64_t seconds = (int64_t)g_config.trusted_device_days * 86400;
    if (seconds <= 0 || seconds > INT32_MAX) {
        return 0;
    }
    return (int)seconds;
}

void httpd_add_session_cookie(http_response_t *res, const char *token) {
    if (!res || !token || token[0] == '\0') {
        return;
    }

    char cookie_header[256];
    snprintf(cookie_header, sizeof(cookie_header),
             "session=%s; Path=/; Max-Age=%d; HttpOnly; SameSite=Lax",
             token, httpd_auth_absolute_timeout_seconds());
    http_response_add_header(res, "Set-Cookie", cookie_header);
}

void httpd_clear_session_cookie(http_response_t *res) {
    if (!res) {
        return;
    }
    http_response_add_header(res, "Set-Cookie",
                             "session=; Path=/; Max-Age=0; HttpOnly; SameSite=Lax");
}

void httpd_add_trusted_device_cookie(http_response_t *res, const char *token) {
    int lifetime = httpd_trusted_device_lifetime_seconds();
    if (!res || !token || token[0] == '\0' || lifetime <= 0) {
        return;
    }

    char cookie_header[256];
    snprintf(cookie_header, sizeof(cookie_header),
             "trusted_device=%s; Path=/; Max-Age=%d; HttpOnly; SameSite=Lax",
             token, lifetime);
    http_response_add_header(res, "Set-Cookie", cookie_header);
}

void httpd_clear_trusted_device_cookie(http_response_t *res) {
    if (!res) {
        return;
    }
    http_response_add_header(res, "Set-Cookie",
                             "trusted_device=; Path=/; Max-Age=0; HttpOnly; SameSite=Lax");
}

static bool request_allowed_during_required_password_change(
    const http_request_t *req, const user_t *user) {
    if (!req || !user || !user->must_change_password || g_config.demo_mode) {
        return true;
    }

    if (req->method == HTTP_METHOD_GET &&
        strcmp(req->path, "/api/auth/verify") == 0) {
        return true;
    }

    char password_path[128];
    int written = snprintf(password_path, sizeof(password_path),
                           "/api/auth/users/%lld/password",
                           (long long)user->id);
    return written > 0 && (size_t)written < sizeof(password_path) &&
           req->method == HTTP_METHOD_PUT &&
           strcmp(req->path, password_path) == 0;
}

static int get_authenticated_user(const http_request_t *req, user_t *user,
                                  bool allow_scoped_token) {
    if (!req || !user) return 0;

    char effective_client_ip[64] = {0};
    if (httpd_get_effective_client_ip(req, effective_client_ip, sizeof(effective_client_ip)) != 0) {
        safe_strcpy(effective_client_ip, req->client_ip, sizeof(effective_client_ip), 0);
    }

    // If authentication is disabled, return a dummy admin user
    if (!g_config.web_auth_enabled) {
        memset(user, 0, sizeof(user_t));
        safe_strcpy(user->username, "admin", sizeof(user->username), 0);
        safe_strcpy(user->authentication_method, "auth_disabled",
                    sizeof(user->authentication_method), 0);
        user->role = USER_ROLE_ADMIN;
        user->is_active = true;
        return 1;
    }

    // First, check for session token in cookie
    char session_token[64] = {0};
    if (httpd_get_session_token(req, session_token, sizeof(session_token)) == 0) {
        int64_t user_id;
        int rc = db_auth_validate_session(session_token, &user_id);
        if (rc == 0) {
            rc = db_auth_get_user_by_id(user_id, user);
            if (rc == 0 && db_auth_ip_allowed_for_user(user, effective_client_ip)) {
                rc = db_auth_validate_session_with_context(session_token, &user_id,
                                                           effective_client_ip, req->user_agent);
                if (rc == 0) {
                    safe_strcpy(user->authentication_method, "session",
                                sizeof(user->authentication_method), 0);
                    if (request_allowed_during_required_password_change(req, user)) {
                        return 1;
                    }
                    log_warn("Session access restricted pending password change for user '%s'",
                             user->username);
                }
            } else if (rc == 0) {
                log_warn("Session auth blocked by allowed_login_cidrs for user '%s' from IP %s",
                         user->username, effective_client_ip[0] != '\0' ? effective_client_ip : "(unknown)");
            }
        }
    }

    // Fall back to HTTP Basic Auth
    char username[64] = {0};
    char password[64] = {0};
    if (httpd_get_basic_auth_credentials(req, username, sizeof(username),
                                          password, sizeof(password)) == 0) {
        if (username[0] != '\0' && password[0] != '\0') {
            int64_t user_id;
            int rc = db_auth_authenticate(username, password, &user_id);
            if (rc == 0) {
                rc = db_auth_get_user_by_id(user_id, user);
                if (rc == 0 && db_auth_ip_allowed_for_user(user, effective_client_ip)) {
                    safe_strcpy(user->authentication_method, "basic",
                                sizeof(user->authentication_method), 0);
                    if (request_allowed_during_required_password_change(req, user)) {
                        return 1;
                    }
                    log_warn("Basic auth access restricted pending password change for user '%s'",
                             user->username);
                } else if (rc == 0) {
                    log_warn("Basic auth blocked by allowed_login_cidrs for user '%s' from IP %s",
                             user->username, effective_client_ip[0] != '\0' ? effective_client_ip : "(unknown)");
                }
            }

            // Config-based authentication fallback removed - all auth is now database-based
        }
    }

    char api_key[128] = {0};
    if (httpd_get_api_key(req, api_key, sizeof(api_key)) == 0) {
        int rc = db_auth_get_user_by_api_key(api_key, user);
        if (rc == 0 && user->is_active && db_auth_ip_allowed_for_user(user, effective_client_ip)) {
            safe_strcpy(user->authentication_method, "legacy_api_key",
                        sizeof(user->authentication_method), 0);
            return 1;
        }
        if (rc == 0 && user->is_active) {
            log_warn("API key auth blocked by allowed_login_cidrs for user '%s' from IP %s",
                     user->username, effective_client_ip[0] != '\0' ? effective_client_ip : "(unknown)");
        }
        if (rc != 0 && allow_scoped_token) {
            int64_t user_id = 0;
            char token_uuid[CAMERA_UUID_STRING_SIZE] = {0};
            db_api_token_result_t token_result = db_api_token_authenticate(
                api_key, &user_id, token_uuid);
            if (token_result == DB_API_TOKEN_OK &&
                db_auth_get_user_by_id(user_id, user) == 0 &&
                user->is_active &&
                db_auth_ip_allowed_for_user(user, effective_client_ip)) {
                user->authenticated_via_scoped_token = true;
                safe_strcpy(user->authentication_method, "scoped_token",
                            sizeof(user->authentication_method), 0);
                safe_strcpy(user->api_token_uuid, token_uuid,
                            sizeof(user->api_token_uuid), 0);
                return 1;
            }
        }
    }

    return 0;
}

int httpd_get_authenticated_user(const http_request_t *req, user_t *user) {
    return get_authenticated_user(req, user, false);
}

int httpd_check_admin_privileges(const http_request_t *req, http_response_t *res) {
    // If authentication is disabled, grant admin access to all requests
    if (!g_config.web_auth_enabled) {
        return 1;
    }

    user_t user;
    if (httpd_get_authenticated_user(req, &user)) {
        if (user.role == USER_ROLE_ADMIN) {
            return 1;
        }
        log_warn("Access denied: User '%s' (role: %s) attempted admin action",
                 user.username, db_auth_get_role_name(user.role));
        http_response_set_json_error(res, 403, "Forbidden: Admin privileges required");
        return 0;
    }
    log_warn("Access denied: Unauthenticated request attempted admin action");
    http_response_set_json_error(res, 401, "Unauthorized: Authentication required");
    return 0;
}

int httpd_is_demo_mode(void) {
    return g_config.demo_mode ? 1 : 0;
}

static int check_viewer_access(const http_request_t *req, user_t *user,
                               bool allow_scoped_token) {
    if (!user) return 0;

    // First, try to get an authenticated user
    if (get_authenticated_user(req, user, allow_scoped_token)) {
        // User is authenticated - they have at least viewer access
        return 1;
    }

    // Authentication being disabled is already handled inside
    // get_authenticated_user(), which returns a dummy admin, so there is no
    // unauthenticated fall-through for that case to handle here.

    // If demo mode is enabled, grant viewer access to unauthenticated users
    if (g_config.demo_mode) {
        // Create a demo viewer pseudo-user
        memset(user, 0, sizeof(user_t));
        safe_strcpy(user->username, "demo", sizeof(user->username), 0);
        safe_strcpy(user->authentication_method, "demo",
                    sizeof(user->authentication_method), 0);
        user->role = USER_ROLE_VIEWER;
        user->is_active = true;
        log_debug("Demo mode: granting viewer access to unauthenticated user");
        return 1;
    }

    // Not authenticated and not in demo mode
    return 0;
}


int httpd_check_viewer_access(const http_request_t *req, user_t *user) {
    return check_viewer_access(req, user, false);
}

int httpd_check_action_access(const http_request_t *req, user_t *user) {
    return check_viewer_access(req, user, true);
}

int httpd_authorize_action(const http_request_t *req, http_response_t *res,
                           authorization_action_t action,
                           const fleet_camera_t *camera, user_t *user,
                           authorization_evaluation_t *evaluation) {
    if (!req || !res || !user || !evaluation) return 0;
    memset(user, 0, sizeof(*user));
    memset(evaluation, 0, sizeof(*evaluation));
    if (!httpd_check_action_access(req, user)) {
        audit_log_authorization(req, NULL, action, camera, NULL, "denied");
        http_response_set_json_error(res, 401, "Unauthorized");
        return 0;
    }
    if (authorization_evaluate(user, action, camera, evaluation) != 0) {
        audit_log_authorization(req, user, action, camera, NULL, "error");
        log_error("Authorization evaluation failed for user '%s' and action %d",
                  user->username, (int)action);
        http_response_set_json_error(res, 500,
                                     "Authorization policy evaluation failed");
        return 0;
    }
    if (evaluation->decision != AUTHZ_DECISION_ALLOW) {
        audit_log_authorization(req, user, action, camera, evaluation, "denied");
        log_warn("Access denied: User '%s' action %d: %s", user->username,
                 (int)action, evaluation->explanation);
        http_response_set_json_error(res, 403, "Forbidden");
        return 0;
    }
    audit_log_authorization(req, user, action, camera, evaluation, "allowed");
    return 1;
}

int httpd_evaluate_stream_action(const user_t *user,
                                 authorization_action_t action,
                                 const char *stream_name,
                                 authorization_evaluation_t *evaluation) {
    if (!user || !stream_name || !evaluation) return -1;
    fleet_camera_t camera;
    memset(&camera, 0, sizeof(camera));
    int result = db_fleet_camera_find_by_name(stream_name, &camera);
    if (result != 0) return result;
    return authorization_evaluate(user, action, &camera, evaluation);
}

int httpd_authorize_stream_action(const http_request_t *req,
                                  http_response_t *res,
                                  authorization_action_t action,
                                  const char *stream_name) {
    user_t user;
    fleet_camera_t camera;
    authorization_evaluation_t evaluation;
    return httpd_authorize_stream_action_with_context(
        req, res, action, stream_name, &user, &camera, &evaluation);
}

int httpd_authorize_stream_action_with_context(
    const http_request_t *req, http_response_t *res,
    authorization_action_t action, const char *stream_name, user_t *user,
    fleet_camera_t *camera, authorization_evaluation_t *evaluation) {
    if (!req || !res || !stream_name || !user || !camera || !evaluation) {
        return 0;
    }
    memset(user, 0, sizeof(*user));
    memset(camera, 0, sizeof(*camera));
    memset(evaluation, 0, sizeof(*evaluation));
    if (!httpd_check_action_access(req, user)) {
        audit_log_authorization(req, NULL, action, NULL, NULL, "denied");
        http_response_set_json_error(res, 401, "Unauthorized");
        return 0;
    }
    int result = db_fleet_camera_find_by_name(stream_name, camera);
    if (result > 0) {
        const authorization_action_metadata_t *metadata =
            authorization_action_metadata(action);
        cJSON *details = cJSON_CreateObject();
        if (details) {
            cJSON_AddStringToObject(details, "event_type",
                                    "authorization.resource_not_found");
            cJSON_AddStringToObject(details, "stream_name", stream_name);
        }
        audit_log_append(req, user, metadata ? metadata->key : "unknown",
                         "camera", NULL, "failure", details);
        cJSON_Delete(details);
        http_response_set_json_error(res, 404, "Camera not found");
        return 0;
    }
    if (result < 0 ||
        authorization_evaluate(user, action, camera, evaluation) != 0) {
        audit_log_authorization(req, user, action, camera,
                                result < 0 ? NULL : evaluation, "error");
        log_error("Failed to evaluate authorization for stream '%s'",
                  stream_name);
        http_response_set_json_error(
            res, 500, "Authorization policy evaluation failed");
        return 0;
    }
    if (evaluation->decision != AUTHZ_DECISION_ALLOW) {
        audit_log_authorization(req, user, action, camera, evaluation,
                                "denied");
        log_warn("Access denied: User '%s' action %d on stream '%s': %s",
                 user->username, (int)action, stream_name,
                 evaluation->explanation);
        http_response_set_json_error(res, 403, "Forbidden");
        return 0;
    }
    audit_log_authorization(req, user, action, camera, evaluation,
                            "allowed");
    return 1;
}

int httpd_authorize_camera_identity_action_with_context(
    const http_request_t *req, http_response_t *res,
    authorization_action_t action, const char *camera_uuid,
    const char *legacy_stream_name, user_t *user, fleet_camera_t *camera,
    authorization_evaluation_t *evaluation) {
    if (camera_uuid && camera_uuid[0] != '\0') {
        stream_config_t stream;
        memset(&stream, 0, sizeof(stream));
        if (get_stream_config_by_uuid(camera_uuid, &stream) != 0) {
            http_response_set_json_error(res, 404, "Camera not found");
            return 0;
        }
        return httpd_authorize_stream_action_with_context(
            req, res, action, stream.name, user, camera, evaluation);
    }
    return httpd_authorize_stream_action_with_context(
        req, res, action, legacy_stream_name, user, camera, evaluation);
}

void httpd_sanitize_attachment_filename(const char *input, char *output,
                                        size_t output_size) {
    if (!output || output_size == 0) return;
    output[0] = '\0';
    if (!input) return;
    const char *basename = input;
    for (const char *cursor = input; *cursor; cursor++) {
        if (*cursor == '/' || *cursor == '\\') basename = cursor + 1;
    }
    size_t written = 0;
    for (const unsigned char *cursor = (const unsigned char *)basename;
         *cursor && written + 1 < output_size; cursor++) {
        unsigned char character = *cursor;
        output[written++] = (isalnum(character) || character == '.' ||
                             character == '-' || character == '_')
            ? (char)character : '_';
    }
    output[written] = '\0';
}
