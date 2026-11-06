/*
 * SPDX-License-Identifier: GPL-3.0-or-later
 * Copyright (C) 2026 OpenSensor Engineering and LightNVR contributors
 *
 * This file is part of LightNVR <https://github.com/opensensor/lightNVR>.
 */

/**
 * @file api_handlers_totp.c
 * @brief TOTP MFA API handlers
 *
 * Provides endpoints for TOTP setup, verification, disable, and login.
 */

#define _XOPEN_SOURCE
#define _GNU_SOURCE

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdint.h>
#include <time.h>
#include <ctype.h>
#include <sqlite3.h>
#include <cjson/cJSON.h>

#include "core/totp.h"
#include <errno.h>
#include <curl/curl.h>

#include "web/api_handlers_totp.h"
#include "web/httpd_utils.h"
#include "web/request_response.h"
#include "web/audit_log.h"
#define LOG_COMPONENT "AuthAPI"
#include "core/logger.h"
#include "core/config.h"
#include "core/authorization.h"
#include "database/db_auth.h"
#include "database/db_core.h"

/* ========== Helper: Extract user ID from TOTP URL path ========== */

static int extract_totp_user_id(const http_request_t *req, const char *suffix, int64_t *user_id) {
    const char *prefix = "/api/auth/users/";
    if (!req || strncmp(req->path, prefix, strlen(prefix))) return -1;
    const char *text = req->path + strlen(prefix);
    if (*text < '0' || *text > '9') return -1;
    errno = 0;
    char *end = NULL;
    long long value = strtoll(text, &end, 10);
    if (errno == ERANGE || value <= 0 || value > INT64_MAX || strncmp(end, suffix, strlen(suffix))) return -1;
    end += strlen(suffix);
    while (*end == '/') ++end;
    if (*end && *end != '?') return -1;
    *user_id = (int64_t)value;
    return 0;
}

static void identity_error(http_response_t *res, int result) {
    if (result == DB_AUTH_IDENTITY_LIMITED) {
        http_response_set_json_error(res, 429, "Too many TOTP attempts. Please try again later.");
    } else if (result == DB_AUTH_IDENTITY_CONFLICT) {
        http_response_set_json_error(res, 409, "TOTP state changed or setup expired. Check status and start setup again if needed.");
    } else if (result == DB_AUTH_IDENTITY_INVALID) {
        http_response_set_json_error(res, 401, "Invalid or already used TOTP code");
    } else {
        http_response_set_json_error(res, 500, "Failed to update authentication state");
    }
}

static void current_owner_session(const http_request_t *req, const user_t *current,
                                   int64_t target, char session[64]) {
    session[0] = '\0';
    if (current->id == target && !current->authenticated_via_scoped_token)
        (void)httpd_get_session_token(req, session, 64);
}

/**
 * @brief Check if the current user can manage TOTP for the target user
 * Admins can manage any user's TOTP; users can manage their own
 */
static int check_totp_permission(const http_request_t *req, http_response_t *res,
                                  int64_t target_user_id, user_t *current_user) {
    if (!httpd_check_action_access(req, current_user)) {
        http_response_set_json_error(res, 401, "Unauthorized");
        return 0;
    }
    authorization_evaluation_t evaluation;
    bool can_manage_users =
        authorization_evaluate(current_user, AUTHZ_USERS_MANAGE, NULL,
                               &evaluation) == 0 &&
        evaluation.decision == AUTHZ_DECISION_ALLOW;
    bool is_session_self = !current_user->authenticated_via_scoped_token &&
                           current_user->id == target_user_id;
    if (!can_manage_users && !is_session_self) {
        http_response_set_json_error(res, 403, "Forbidden: Can only manage your own MFA");
        return 0;
    }
    return 1;  /* Permission granted */
}



/* ========== API Handlers ========== */

/**
 * @brief POST /api/auth/users/:id/totp/setup
 * Generate TOTP secret and return otpauth:// URI
 */
void handle_totp_setup(const http_request_t *req, http_response_t *res) {
    log_info("Handling POST /api/auth/users/:id/totp/setup");

    int64_t target_user_id;
    if (extract_totp_user_id(req, "/totp/setup", &target_user_id) != 0) {
        http_response_set_json_error(res, 400, "Invalid user ID in path");
        return;
    }

    user_t current_user;
    if (!check_totp_permission(req, res, target_user_id, &current_user)) {
        return;
    }

    /* Get the target user to include username in URI */
    user_t target_user;
    if (db_auth_get_user_by_id(target_user_id, &target_user) != 0) {
        http_response_set_json_error(res, 404, "User not found");
        return;
    }

    /* Generate a new TOTP secret */
    char secret_b32[64] = {0};
    if (totp_generate_secret(secret_b32, sizeof(secret_b32)) != 0) {
        http_response_set_json_error(res, 500, "Failed to generate TOTP secret");
        return;
    }

    /* Build otpauth:// URI */
    char otpauth_uri[512];
    char *label = curl_easy_escape(NULL, target_user.username, 0);
    if (!label) { http_response_set_json_error(res, 500, "Failed to encode TOTP label"); return; }
    snprintf(otpauth_uri, sizeof(otpauth_uri),
             "otpauth://totp/LightNVR:%s?secret=%s&issuer=LightNVR&algorithm=SHA1&digits=6&period=30",
             label, secret_b32);
    curl_free(label);
    /* Store the secret in the database (not yet enabled) */
    int result = db_auth_set_totp_secret(target_user_id, secret_b32);
    if (result != 0) {
        identity_error(res, result);
        return;
    }

    http_response_add_header(res, "Cache-Control", "private, no-store");

    /* Return the URI and secret */
    cJSON *response = cJSON_CreateObject();
    cJSON_AddBoolToObject(response, "success", true);
    cJSON_AddStringToObject(response, "secret", secret_b32);
    cJSON_AddStringToObject(response, "otpauth_uri", otpauth_uri);
    cJSON_AddStringToObject(response, "message", "Scan the QR code with your authenticator app, then verify with a code");

    char *json_str = cJSON_PrintUnformatted(response);
    http_response_set_json(res, 200, json_str);
    free(json_str);
    cJSON_Delete(response);

    log_info("TOTP setup initiated for user %lld (%s)", (long long)target_user_id, target_user.username);
}

/**
 * @brief POST /api/auth/users/:id/totp/verify
 * Verify a TOTP code and enable TOTP for the user
 * Body: { "code": "123456" }
 */
void handle_totp_verify(const http_request_t *req, http_response_t *res) {
    log_info("Handling POST /api/auth/users/:id/totp/verify");

    int64_t target_user_id;
    if (extract_totp_user_id(req, "/totp/verify", &target_user_id) != 0) {
        http_response_set_json_error(res, 400, "Invalid user ID in path");
        return;
    }

    user_t current_user;
    if (!check_totp_permission(req, res, target_user_id, &current_user)) {
        return;
    }

    /* Parse the TOTP code from request body */
    cJSON *body = httpd_parse_json_body(req);
    if (!body) {
        http_response_set_json_error(res, 400, "Invalid JSON body");
        return;
    }

    cJSON *code_json = cJSON_GetObjectItem(body, "code");
    if (!code_json || !cJSON_IsString(code_json)) {
        cJSON_Delete(body);
        http_response_set_json_error(res, 400, "Missing or invalid 'code' field");
        return;
    }

    char keep_session[64];
    current_owner_session(req, &current_user, target_user_id, keep_session);
    int result = db_auth_verify_totp_enrollment(target_user_id, code_json->valuestring, keep_session);
    cJSON_Delete(body);
    if (result != 0) { identity_error(res, result); return; }
    if (current_user.id == target_user_id) httpd_clear_trusted_device_cookie(res);

    cJSON *response = cJSON_CreateObject();
    cJSON_AddBoolToObject(response, "success", true);
    cJSON_AddStringToObject(response, "message", "TOTP MFA enabled successfully");

    char *json_str = cJSON_PrintUnformatted(response);
    http_response_set_json(res, 200, json_str);
    free(json_str);
    cJSON_Delete(response);

    log_info("TOTP enabled for user %lld", (long long)target_user_id);
}

/**
 * @brief POST /api/auth/users/:id/totp/disable
 * Disable TOTP for a user
 */
void handle_totp_disable(const http_request_t *req, http_response_t *res) {
    log_info("Handling POST /api/auth/users/:id/totp/disable");

    int64_t target_user_id;
    if (extract_totp_user_id(req, "/totp/disable", &target_user_id) != 0) {
        http_response_set_json_error(res, 400, "Invalid user ID in path");
        return;
    }

    user_t current_user;
    if (!check_totp_permission(req, res, target_user_id, &current_user)) {
        return;
    }

    char keep_session[64];
    current_owner_session(req, &current_user, target_user_id, keep_session);
    int result = db_auth_disable_totp(target_user_id, keep_session);
    if (result != 0) { identity_error(res, result); return; }
    if (current_user.id == target_user_id) httpd_clear_trusted_device_cookie(res);

    cJSON *response = cJSON_CreateObject();
    cJSON_AddBoolToObject(response, "success", true);
    cJSON_AddStringToObject(response, "message", "TOTP MFA disabled successfully");

    char *json_str = cJSON_PrintUnformatted(response);
    http_response_set_json(res, 200, json_str);
    free(json_str);
    cJSON_Delete(response);

    log_info("TOTP disabled for user %lld", (long long)target_user_id);
}

/**
 * @brief GET /api/auth/users/:id/totp/status
 * Get TOTP status for a user
 */
void handle_totp_status(const http_request_t *req, http_response_t *res) {
    log_info("Handling GET /api/auth/users/:id/totp/status");

    int64_t target_user_id;
    if (extract_totp_user_id(req, "/totp/status", &target_user_id) != 0) {
        http_response_set_json_error(res, 400, "Invalid user ID in path");
        return;
    }

    user_t current_user;
    if (!check_totp_permission(req, res, target_user_id, &current_user)) {
        return;
    }

    char secret[64] = {0};
    bool enabled = false;
    user_t target_user;
    if (db_auth_get_user_by_id(target_user_id, &target_user) != 0) {
        http_response_set_json_error(res, 404, "User not found");
        return;
    }
    if (db_auth_get_totp_info(target_user_id, secret, sizeof(secret), &enabled) != 0) {
        http_response_set_json_error(res, 500, "Failed to read TOTP status");
        return;
    }
    http_response_add_header(res, "Cache-Control", "private, no-store");

    cJSON *response = cJSON_CreateObject();
    cJSON_AddBoolToObject(response, "totp_enabled", enabled);
    cJSON_AddBoolToObject(response, "totp_configured", secret[0] != '\0');

    char *json_str = cJSON_PrintUnformatted(response);
    http_response_set_json(res, 200, json_str);
    free(json_str);
    cJSON_Delete(response);
}

/**
 * @brief POST /api/auth/login/totp
 * Complete two-step login by verifying TOTP code
 * Body: { "totp_token": "...", "code": "123456" }
 *
 * The totp_token is a short-lived session created during password authentication
 * when TOTP is enabled. It proves the password was already verified.
 */
void handle_auth_login_totp(const http_request_t *req, http_response_t *res) {
    log_info("Handling POST /api/auth/login/totp");

    char effective_client_ip[64] = {0};
    if (httpd_get_effective_client_ip(req, effective_client_ip,
                                      sizeof(effective_client_ip)) != 0) {
        snprintf(effective_client_ip, sizeof(effective_client_ip), "%s",
                 req->client_ip);
    }

    cJSON *body = httpd_parse_json_body(req);
    if (!body) {
        audit_log_login(req, NULL, "", "password_totp", "failure",
                        "invalid_request");
        http_response_set_json_error(res, 400, "Invalid JSON body");
        return;
    }

    cJSON *token_json = cJSON_GetObjectItem(body, "totp_token");
    cJSON *code_json = cJSON_GetObjectItem(body, "code");
    cJSON *remember_device_json = cJSON_GetObjectItem(body, "remember_device");
    bool remember_device = remember_device_json && cJSON_IsBool(remember_device_json) && cJSON_IsTrue(remember_device_json);

    if (!token_json || !cJSON_IsString(token_json) ||
        !code_json || !cJSON_IsString(code_json)) {
        cJSON_Delete(body);
        audit_log_login(req, NULL, "", "password_totp", "failure",
                        "invalid_request");
        http_response_set_json_error(res, 400, "Missing 'totp_token' or 'code'");
        return;
    }

    /* Validate the pending MFA session token */
    int64_t user_id;
    int rc = db_auth_validate_mfa_challenge(token_json->valuestring, &user_id,
                                                   effective_client_ip,
                                                   req->user_agent);
    if (rc != 0) {
        cJSON_Delete(body);
        audit_log_login(req, NULL, "", "password_totp", "denied",
                        "invalid_mfa_session");
        http_response_set_json_error(res, 401, "Invalid or expired MFA token. Please login again.");
        return;
    }

    user_t challenge_owner;
    if (db_auth_get_user_by_id(user_id, &challenge_owner) != 0 ||
        !db_auth_ip_allowed_for_user(&challenge_owner, effective_client_ip)) {
        db_auth_delete_session(token_json->valuestring);
        cJSON_Delete(body);
        http_response_set_json_error(res, 401, "Invalid or expired MFA token. Please login again.");
        return;
    }

    char session_token[33], trusted_token[33];
    int trusted_seconds = remember_device ? httpd_trusted_device_lifetime_seconds() : 0;
    rc = db_auth_complete_totp_login(user_id, code_json->valuestring, token_json->valuestring,
                                     NULL, effective_client_ip, req->user_agent,
                                     trusted_seconds, session_token, trusted_token);
    cJSON_Delete(body);
    if (rc != 0) {
        audit_log_login(req, &challenge_owner, NULL, "password_totp", "denied",
                        rc == DB_AUTH_IDENTITY_LIMITED ? "rate_limited" : "invalid_mfa_code");
        identity_error(res, rc);
        return;
    }
    httpd_add_session_cookie(res, session_token);
    if (trusted_token[0]) httpd_add_trusted_device_cookie(res, trusted_token);

    /* Return success */
    cJSON *response = cJSON_CreateObject();
    cJSON_AddBoolToObject(response, "success", true);
    cJSON_AddStringToObject(response, "redirect", "/index.html");

    char *json_str = cJSON_PrintUnformatted(response);
    http_response_set_json(res, 200, json_str);
    free(json_str);
    cJSON_Delete(response);

    log_info("TOTP login completed for user ID %lld", (long long)user_id);
    user_t user = {.id = user_id};
    (void)db_auth_get_user_by_id(user_id, &user);
    audit_log_login(req, &user, NULL, "password_totp", "success",
                    "session_created");
}
