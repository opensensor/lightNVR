/**
 * @file api_handlers_totp.c
 * @brief TOTP MFA API handlers and algorithm implementation
 *
 * Implements RFC 6238 TOTP using HMAC-SHA1 via mbedTLS.
 * Provides endpoints for TOTP setup, verification, disable, and login.
 */

#define _XOPEN_SOURCE
#define _GNU_SOURCE

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>
#include <ctype.h>
#include <sqlite3.h>
#include <cjson/cJSON.h>

#include <mbedtls/md.h>
#include <mbedtls/entropy.h>
#include <mbedtls/ctr_drbg.h>

#include "web/api_handlers_totp.h"
#include "web/httpd_utils.h"
#include "web/request_response.h"
#include "core/logger.h"
#include "core/config.h"
#include "database/db_auth.h"
#include "database/db_core.h"

/* ========== Base32 Encoding/Decoding ========== */

static const char BASE32_ALPHABET[] = "ABCDEFGHIJKLMNOPQRSTUVWXYZ234567";

/**
 * @brief Encode binary data to base32 string
 */
static int base32_encode(const unsigned char *data, size_t data_len,
                         char *output, size_t output_size) {
    if (!data || !output || output_size == 0) return -1;

    size_t out_len = ((data_len * 8) + 4) / 5;  /* Ceiling division */
    if (out_len + 1 > output_size) return -1;

    size_t i, j = 0;
    unsigned int buffer = 0;
    int bits_left = 0;

    for (i = 0; i < data_len; i++) {
        buffer = (buffer << 8) | data[i];
        bits_left += 8;
        while (bits_left >= 5) {
            bits_left -= 5;
            output[j++] = BASE32_ALPHABET[(buffer >> bits_left) & 0x1F];
        }
    }
    if (bits_left > 0) {
        output[j++] = BASE32_ALPHABET[(buffer << (5 - bits_left)) & 0x1F];
    }
    output[j] = '\0';
    return (int)j;
}

/**
 * @brief Decode base32 string to binary data
 */
static int base32_decode(const char *input, unsigned char *output, size_t output_size) {
    if (!input || !output || output_size == 0) return -1;

    size_t input_len = strlen(input);
    size_t out_len = (input_len * 5) / 8;
    if (out_len > output_size) return -1;

    unsigned int buffer = 0;
    int bits_left = 0;
    size_t j = 0;

    for (size_t i = 0; i < input_len; i++) {
        char c = toupper((unsigned char)input[i]);
        if (c == '=' || c == ' ') continue;  /* Skip padding and spaces */

        int val;
        if (c >= 'A' && c <= 'Z') {
            val = c - 'A';
        } else if (c >= '2' && c <= '7') {
            val = c - '2' + 26;
        } else {
            return -1;  /* Invalid character */
        }

        buffer = (buffer << 5) | val;
        bits_left += 5;

        if (bits_left >= 8) {
            bits_left -= 8;
            if (j < output_size) {
                output[j++] = (unsigned char)(buffer >> bits_left);
            }
        }
    }
    return (int)j;
}

/* ========== TOTP Algorithm (RFC 6238) ========== */

/**
 * @brief Compute HMAC-SHA1
 */
static int hmac_sha1(const unsigned char *key, size_t key_len,
                     const unsigned char *data, size_t data_len,
                     unsigned char *output) {
    const mbedtls_md_info_t *md_info = mbedtls_md_info_from_type(MBEDTLS_MD_SHA1);
    if (!md_info) return -1;

    return mbedtls_md_hmac(md_info, key, key_len, data, data_len, output);
}

/**
 * @brief Generate a TOTP code for a given time step
 * @param secret_b32 Base32-encoded secret
 * @param time_step Time step counter
 * @return 6-digit TOTP code, or -1 on error
 */
static int totp_generate(const char *secret_b32, uint64_t time_step) {
    /* Decode the base32 secret */
    unsigned char secret[64];
    int secret_len = base32_decode(secret_b32, secret, sizeof(secret));
    if (secret_len < 0) {
        log_error("TOTP: Failed to decode base32 secret");
        return -1;
    }

    /* Convert time step to big-endian 8 bytes */
    unsigned char msg[8];
    for (int i = 7; i >= 0; i--) {
        msg[i] = (unsigned char)(time_step & 0xFF);
        time_step >>= 8;
    }

    /* Compute HMAC-SHA1 */
    unsigned char hash[20];
    if (hmac_sha1(secret, (size_t)secret_len, msg, 8, hash) != 0) {
        log_error("TOTP: HMAC-SHA1 failed");
        return -1;
    }

    /* Dynamic truncation (RFC 4226 section 5.4) */
    int offset = hash[19] & 0x0F;
    uint32_t code = ((uint32_t)(hash[offset] & 0x7F) << 24) |
                    ((uint32_t)(hash[offset + 1] & 0xFF) << 16) |
                    ((uint32_t)(hash[offset + 2] & 0xFF) << 8) |
                    ((uint32_t)(hash[offset + 3] & 0xFF));

    return (int)(code % 1000000);
}

/**
 * @brief Verify a TOTP code with time window tolerance (±1 step)
 * @param secret_b32 Base32-encoded secret
 * @param code 6-digit code to verify
 * @return 0 if valid, -1 if invalid
 */
static int totp_verify(const char *secret_b32, const char *code_str) {
    if (!secret_b32 || !code_str || strlen(code_str) != 6) return -1;

    int provided_code = atoi(code_str);
    uint64_t current_time = (uint64_t)time(NULL);
    uint64_t time_step = current_time / 30;

    /* Check current step and ±1 window for clock skew tolerance */
    for (int i = -1; i <= 1; i++) {
        int expected = totp_generate(secret_b32, time_step + i);
        if (expected >= 0 && expected == provided_code) {
            return 0;  /* Valid */
        }
    }
    return -1;  /* Invalid */
}

/**
 * @brief Generate a random TOTP secret (160 bits = 20 bytes, base32 encoded)
 * @param secret_b32 Buffer for base32-encoded secret (at least 33 bytes)
 * @param secret_size Size of the buffer
 * @return 0 on success, -1 on failure
 */
static int totp_generate_secret(char *secret_b32, size_t secret_size) {
    unsigned char raw_secret[20];  /* 160 bits */

    mbedtls_entropy_context entropy;
    mbedtls_ctr_drbg_context ctr_drbg;
    const char *pers = "totp_secret_gen";

    mbedtls_entropy_init(&entropy);
    mbedtls_ctr_drbg_init(&ctr_drbg);

    int ret = mbedtls_ctr_drbg_seed(&ctr_drbg, mbedtls_entropy_func, &entropy,
                                     (const unsigned char *)pers, strlen(pers));
    if (ret != 0) {
        log_error("TOTP: Failed to seed RNG: -0x%04x", -ret);
        mbedtls_ctr_drbg_free(&ctr_drbg);
        mbedtls_entropy_free(&entropy);
        return -1;
    }

    ret = mbedtls_ctr_drbg_random(&ctr_drbg, raw_secret, sizeof(raw_secret));
    mbedtls_ctr_drbg_free(&ctr_drbg);
    mbedtls_entropy_free(&entropy);

    if (ret != 0) {
        log_error("TOTP: Failed to generate random secret: -0x%04x", -ret);
        return -1;
    }

    if (base32_encode(raw_secret, sizeof(raw_secret), secret_b32, secret_size) < 0) {
        log_error("TOTP: Failed to base32 encode secret");
        return -1;
    }

    return 0;
}

/* ========== Helper: Extract user ID from TOTP URL path ========== */

static int extract_totp_user_id(const http_request_t *req, int64_t *user_id) {
    char param[64] = {0};
    if (http_request_extract_path_param(req, "/api/auth/users/", param, sizeof(param)) != 0) {
        return -1;
    }
    /* param may be "123/totp/setup" - extract just the numeric part */
    char *slash = strchr(param, '/');
    if (slash) *slash = '\0';
    *user_id = strtoll(param, NULL, 10);
    return (*user_id > 0) ? 0 : -1;
}

/**
 * @brief Check if the current user can manage TOTP for the target user
 * Admins can manage any user's TOTP; users can manage their own
 */
static int check_totp_permission(const http_request_t *req, http_response_t *res,
                                  int64_t target_user_id, user_t *current_user) {
    if (!httpd_get_authenticated_user(req, current_user)) {
        http_response_set_json_error(res, 401, "Unauthorized");
        return 0;
    }
    if (current_user->role != USER_ROLE_ADMIN && current_user->id != target_user_id) {
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
    if (extract_totp_user_id(req, &target_user_id) != 0) {
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

    /* Store the secret in the database (not yet enabled) */
    if (db_auth_set_totp_secret(target_user_id, secret_b32) != 0) {
        http_response_set_json_error(res, 500, "Failed to store TOTP secret");
        return;
    }

    /* Build otpauth:// URI */
    char otpauth_uri[512];
    snprintf(otpauth_uri, sizeof(otpauth_uri),
             "otpauth://totp/LightNVR:%s?secret=%s&issuer=LightNVR&algorithm=SHA1&digits=6&period=30",
             target_user.username, secret_b32);

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
    if (extract_totp_user_id(req, &target_user_id) != 0) {
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

    /* Get the stored secret */
    char secret[64] = {0};
    bool enabled = false;
    if (db_auth_get_totp_info(target_user_id, secret, sizeof(secret), &enabled) != 0) {
        cJSON_Delete(body);
        http_response_set_json_error(res, 400, "TOTP not set up for this user. Run setup first.");
        return;
    }

    /* Verify the code */
    if (totp_verify(secret, code_json->valuestring) != 0) {
        cJSON_Delete(body);
        http_response_set_json_error(res, 401, "Invalid TOTP code");
        return;
    }

    cJSON_Delete(body);

    /* Enable TOTP for this user */
    if (db_auth_enable_totp(target_user_id, true) != 0) {
        http_response_set_json_error(res, 500, "Failed to enable TOTP");
        return;
    }

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
    if (extract_totp_user_id(req, &target_user_id) != 0) {
        http_response_set_json_error(res, 400, "Invalid user ID in path");
        return;
    }

    user_t current_user;
    if (!check_totp_permission(req, res, target_user_id, &current_user)) {
        return;
    }

    /* Disable TOTP and clear the secret */
    if (db_auth_enable_totp(target_user_id, false) != 0) {
        http_response_set_json_error(res, 500, "Failed to disable TOTP");
        return;
    }

    /* Clear the secret from database */
    if (db_auth_set_totp_secret(target_user_id, NULL) != 0) {
        log_warn("Failed to clear TOTP secret for user %lld", (long long)target_user_id);
    }

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
    if (extract_totp_user_id(req, &target_user_id) != 0) {
        http_response_set_json_error(res, 400, "Invalid user ID in path");
        return;
    }

    user_t current_user;
    if (!check_totp_permission(req, res, target_user_id, &current_user)) {
        return;
    }

    char secret[64] = {0};
    bool enabled = false;
    db_auth_get_totp_info(target_user_id, secret, sizeof(secret), &enabled);

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

    cJSON *body = httpd_parse_json_body(req);
    if (!body) {
        http_response_set_json_error(res, 400, "Invalid JSON body");
        return;
    }

    cJSON *token_json = cJSON_GetObjectItem(body, "totp_token");
    cJSON *code_json = cJSON_GetObjectItem(body, "code");

    if (!token_json || !cJSON_IsString(token_json) ||
        !code_json || !cJSON_IsString(code_json)) {
        cJSON_Delete(body);
        http_response_set_json_error(res, 400, "Missing 'totp_token' or 'code'");
        return;
    }

    /* Validate the pending MFA session token */
    int64_t user_id;
    int rc = db_auth_validate_session(token_json->valuestring, &user_id);
    if (rc != 0) {
        cJSON_Delete(body);
        http_response_set_json_error(res, 401, "Invalid or expired MFA token. Please login again.");
        return;
    }

    /* Get TOTP secret for this user */
    char secret[64] = {0};
    bool enabled = false;
    if (db_auth_get_totp_info(user_id, secret, sizeof(secret), &enabled) != 0 || !enabled) {
        cJSON_Delete(body);
        /* Delete the pending session */
        db_auth_delete_session(token_json->valuestring);
        http_response_set_json_error(res, 400, "TOTP not enabled for this user");
        return;
    }

    /* Verify the TOTP code */
    if (totp_verify(secret, code_json->valuestring) != 0) {
        cJSON_Delete(body);
        http_response_set_json_error(res, 401, "Invalid TOTP code");
        return;
    }

    /* Delete the pending short-lived session */
    db_auth_delete_session(token_json->valuestring);

    cJSON_Delete(body);

    /* Create a real full session */
    int session_timeout_seconds = g_config.auth_timeout_hours * 3600;
    char session_token[33];
    rc = db_auth_create_session(user_id, NULL, NULL, session_timeout_seconds,
                                session_token, sizeof(session_token));
    if (rc != 0) {
        http_response_set_json_error(res, 500, "Failed to create session");
        return;
    }

    /* Set session cookie */
    char cookie_header[256];
    snprintf(cookie_header, sizeof(cookie_header),
             "session=%s; Path=/; Max-Age=%d; HttpOnly; SameSite=Lax",
             session_token, session_timeout_seconds);
    http_response_add_header(res, "Set-Cookie", cookie_header);

    /* Return success */
    cJSON *response = cJSON_CreateObject();
    cJSON_AddBoolToObject(response, "success", true);
    cJSON_AddStringToObject(response, "redirect", "/index.html");

    char *json_str = cJSON_PrintUnformatted(response);
    http_response_set_json(res, 200, json_str);
    free(json_str);
    cJSON_Delete(response);

    log_info("TOTP login completed for user ID %lld", (long long)user_id);
}