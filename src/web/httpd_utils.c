#define _GNU_SOURCE

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <strings.h>

#include "web/httpd_utils.h"
#include "web/request_response.h"
#include "core/logger.h"
#include "core/config.h"
#include "database/db_auth.h"

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
    strncpy(username, decoded, username_size - 1);
    username[username_size - 1] = '\0';
    strncpy(password, colon + 1, password_size - 1);
    password[password_size - 1] = '\0';

    return 0;
}

int httpd_get_session_token(const http_request_t *req, char *token, size_t token_size) {
    if (!req || !token || token_size == 0) return -1;

    token[0] = '\0';
    const char *cookie = http_request_get_header(req, "Cookie");
    if (!cookie) return -1;

    const char *session_start = strstr(cookie, "session=");
    if (!session_start) return -1;

    session_start += 8; // Skip "session="
    const char *session_end = strchr(session_start, ';');
    size_t len = session_end ? (size_t)(session_end - session_start) : strlen(session_start);

    if (len == 0 || len >= token_size) return -1;

    memcpy(token, session_start, len);
    token[len] = '\0';
    return 0;
}

int httpd_get_authenticated_user(const http_request_t *req, user_t *user) {
    if (!req || !user) return 0;

    // If authentication is disabled, return a dummy admin user
    if (!g_config.web_auth_enabled) {
        memset(user, 0, sizeof(user_t));
        strncpy(user->username, "admin", sizeof(user->username) - 1);
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
            if (rc == 0) {
                return 1;
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
                if (rc == 0) {
                    return 1;
                }
            }

            // Fall back to config-based authentication for backward compatibility
            if (strcmp(username, g_config.web_username) == 0 &&
                strcmp(password, g_config.web_password) == 0) {
                memset(user, 0, sizeof(user_t));
                strncpy(user->username, username, sizeof(user->username) - 1);
                user->role = USER_ROLE_ADMIN;
                user->is_active = true;
                return 1;
            }
        }
    }

    return 0;
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

