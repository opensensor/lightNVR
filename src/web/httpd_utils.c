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

static char *trim_ascii_whitespace(char *value) {
    if (!value) {
        return NULL;
    }

    while (*value && isspace((unsigned char)*value)) {
        value++;
    }

    size_t len = strlen(value);
    while (len > 0 && isspace((unsigned char)value[len - 1])) {
        value[--len] = '\0';
    }

    return value;
}

static bool copy_trimmed_value(const char *input, char *output, size_t output_size) {
    if (!input || !output || output_size == 0) {
        return false;
    }

    output[0] = '\0';

    char buffer[1024] = {0};
    if (strlen(input) >= sizeof(buffer)) {
        return false;
    }
    strncpy(buffer, input, sizeof(buffer) - 1);

    char *trimmed = trim_ascii_whitespace(buffer);
    if (!trimmed || trimmed[0] == '\0' || strlen(trimmed) >= output_size) {
        return false;
    }

    strncpy(output, trimmed, output_size - 1);
    output[output_size - 1] = '\0';
    return true;
}

static bool normalize_ip_literal(const char *input, char *normalized, size_t normalized_size) {
    if (!input || !normalized || normalized_size == 0) {
        return false;
    }

    normalized[0] = '\0';

    char candidate_buffer[INET6_ADDRSTRLEN + 8] = {0};
    if (strlen(input) >= sizeof(candidate_buffer)) {
        return false;
    }
    strncpy(candidate_buffer, input, sizeof(candidate_buffer) - 1);

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
    strncpy(rules.allowed_login_cidrs, cidr_list, sizeof(rules.allowed_login_cidrs) - 1);
    rules.allowed_login_cidrs[sizeof(rules.allowed_login_cidrs) - 1] = '\0';
    rules.has_login_cidr_restriction = true;
    return db_auth_ip_allowed_for_user(&rules, ip);
}

static int resolve_client_ip_from_forwarded_headers(const http_request_t *req,
                                                    char *client_ip,
                                                    size_t client_ip_size) {
    const char *xff = http_request_get_header(req, "X-Forwarded-For");
    if (xff && xff[0] != '\0') {
        char xff_copy[1024] = {0};
        if (strlen(xff) >= sizeof(xff_copy)) {
            return -1;
        }
        strncpy(xff_copy, xff, sizeof(xff_copy) - 1);

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
            strncpy(normalized_tokens[token_count], normalized, sizeof(normalized_tokens[token_count]) - 1);
            token_count++;
        }

        if (token_count > 0) {
            for (int i = token_count - 1; i >= 0; --i) {
                if (!ip_matches_cidr_list(g_config.trusted_proxy_cidrs, normalized_tokens[i])) {
                    strncpy(client_ip, normalized_tokens[i], client_ip_size - 1);
                    client_ip[client_ip_size - 1] = '\0';
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
        strncpy(client_ip, normalized, client_ip_size - 1);
        client_ip[client_ip_size - 1] = '\0';
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
    strncpy(username, decoded, username_size - 1);
    username[username_size - 1] = '\0';
    strncpy(password, colon + 1, password_size - 1);
    password[password_size - 1] = '\0';

    return 0;
}

int httpd_get_api_key(const http_request_t *req, char *api_key, size_t api_key_size) {
    if (!req || !api_key || api_key_size == 0) {
        return -1;
    }

    api_key[0] = '\0';

    const char *header_value = http_request_get_header(req, "X-API-Key");
    if (header_value && copy_trimmed_value(header_value, api_key, api_key_size)) {
        return 0;
    }

    const char *auth = http_request_get_header(req, "Authorization");
    if (!auth || strncasecmp(auth, "Bearer ", 7) != 0) {
        return -1;
    }

    return copy_trimmed_value(auth + 7, api_key, api_key_size) ? 0 : -1;
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
        return copy_trimmed_value(req->client_ip, client_ip, client_ip_size) ? 0 : -1;
    }

    if (g_config.trusted_proxy_cidrs[0] != '\0' &&
        ip_matches_cidr_list(g_config.trusted_proxy_cidrs, peer_ip) &&
        resolve_client_ip_from_forwarded_headers(req, client_ip, client_ip_size) == 0) {
        return 0;
    }

    strncpy(client_ip, peer_ip, client_ip_size - 1);
    client_ip[client_ip_size - 1] = '\0';
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

int httpd_get_authenticated_user(const http_request_t *req, user_t *user) {
    if (!req || !user) return 0;

    char effective_client_ip[64] = {0};
    if (httpd_get_effective_client_ip(req, effective_client_ip, sizeof(effective_client_ip)) != 0) {
        strncpy(effective_client_ip, req->client_ip, sizeof(effective_client_ip) - 1);
        effective_client_ip[sizeof(effective_client_ip) - 1] = '\0';
    }

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
            if (rc == 0 && db_auth_ip_allowed_for_user(user, effective_client_ip)) {
                rc = db_auth_validate_session_with_context(session_token, &user_id,
                                                           effective_client_ip, req->user_agent);
                if (rc == 0) {
                    return 1;
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
                    return 1;
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
            return 1;
        }
        if (rc == 0 && user->is_active) {
            log_warn("API key auth blocked by allowed_login_cidrs for user '%s' from IP %s",
                     user->username, effective_client_ip[0] != '\0' ? effective_client_ip : "(unknown)");
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

int httpd_is_demo_mode(void) {
    return g_config.demo_mode ? 1 : 0;
}

int httpd_check_viewer_access(const http_request_t *req, user_t *user) {
    if (!user) return 0;

    // First, try to get an authenticated user
    if (httpd_get_authenticated_user(req, user)) {
        // User is authenticated - they have at least viewer access
        return 1;
    }

    // If authentication is disabled entirely, grant viewer access
    if (!g_config.web_auth_enabled) {
        // Create a pseudo-user for unauthenticated access
        memset(user, 0, sizeof(user_t));
        strncpy(user->username, "anonymous", sizeof(user->username) - 1);
        user->role = USER_ROLE_VIEWER;
        user->is_active = true;
        return 1;
    }

    // If demo mode is enabled, grant viewer access to unauthenticated users
    if (g_config.demo_mode) {
        // Create a demo viewer pseudo-user
        memset(user, 0, sizeof(user_t));
        strncpy(user->username, "demo", sizeof(user->username) - 1);
        user->role = USER_ROLE_VIEWER;
        user->is_active = true;
        log_debug("Demo mode: granting viewer access to unauthenticated user");
        return 1;
    }

    // Not authenticated and not in demo mode
    return 0;
}

