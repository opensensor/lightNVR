#define _POSIX_C_SOURCE 200809L
#define _XOPEN_SOURCE 700
#define _GNU_SOURCE

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>
#include <sqlite3.h>
#include <stdbool.h>
#include <stdint.h>
#include <stddef.h>
#include <ctype.h>
#include <unistd.h>
#include <sys/time.h>
#include <arpa/inet.h>

#include "database/db_auth.h"
#include "database/db_core.h"
#include "database/db_schema_cache.h"  // For cached_column_exists
#include "core/logger.h"
#include "core/config.h"

// For password hashing
#include <mbedtls/sha256.h>
#include <mbedtls/md.h>
#include <mbedtls/entropy.h>
#include <mbedtls/ctr_drbg.h>
#include <mbedtls/pkcs5.h>

// Salt length for password hashing
#define SALT_LENGTH 16
#define SHA256_DIGEST_LENGTH 32

// Default session expiry time (7 days)
#define DEFAULT_SESSION_EXPIRY 604800

// Default trusted device expiry time (30 days)
#define DEFAULT_TRUSTED_DEVICE_EXPIRY 2592000

// Role names
static const char *role_names[] = {
    "admin",
    "user",
    "viewer",
    "api"
};

static int default_session_absolute_expiry_seconds(void) {
    int64_t seconds = (int64_t)g_config.auth_absolute_timeout_hours * 3600;
    if (seconds <= 0) {
        return DEFAULT_SESSION_EXPIRY;
    }
    if (seconds > INT32_MAX) {
        return INT32_MAX;
    }
    return (int)seconds;
}

static int default_session_idle_expiry_seconds(void) {
    int64_t seconds = (int64_t)g_config.auth_timeout_hours * 3600;
    if (seconds <= 0) {
        return DEFAULT_SESSION_EXPIRY;
    }
    if (seconds > INT32_MAX) {
        return INT32_MAX;
    }
    return (int)seconds;
}

static int default_trusted_device_expiry_seconds(void) {
    int64_t seconds = (int64_t)g_config.trusted_device_days * 86400;
    if (seconds <= 0) {
        return DEFAULT_TRUSTED_DEVICE_EXPIRY;
    }
    if (seconds > INT32_MAX) {
        return INT32_MAX;
    }
    return (int)seconds;
}

static bool should_refresh_session_tracking(time_t now, time_t last_activity_at, time_t idle_expires_at) {
    const time_t refresh_interval = 60;

    if (last_activity_at <= 0) {
        return true;
    }
    if (now - last_activity_at >= refresh_interval) {
        return true;
    }
    if (idle_expires_at <= now + refresh_interval) {
        return true;
    }

    return false;
}

static bool tracking_value_differs(const char *stored_value, const char *current_value) {
    if (!current_value) {
        return false;
    }

    const char *normalized_stored = stored_value ? stored_value : "";
    return strcmp(normalized_stored, current_value) != 0;
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

static int parse_cidr_entry(const char *cidr, int *family, unsigned char *network, int *prefix_len) {
    if (!cidr || !family || !network || !prefix_len) {
        return -1;
    }

    char entry[INET6_ADDRSTRLEN + 5] = {0};
    if (strlen(cidr) >= sizeof(entry)) {
        return -1;
    }
    strncpy(entry, cidr, sizeof(entry) - 1);

    char *trimmed = trim_ascii_whitespace(entry);
    if (!trimmed || trimmed[0] == '\0') {
        return -1;
    }

    char *slash = strrchr(trimmed, '/');
    if (!slash || slash == trimmed || slash[1] == '\0') {
        return -1;
    }

    *slash = '\0';
    char *address = trim_ascii_whitespace(trimmed);
    char *prefix_text = trim_ascii_whitespace(slash + 1);
    if (!address || address[0] == '\0' || !prefix_text || prefix_text[0] == '\0') {
        return -1;
    }

    char *endptr = NULL;
    long prefix = strtol(prefix_text, &endptr, 10);
    if (endptr == prefix_text || *endptr != '\0') {
        return -1;
    }

    if (inet_pton(AF_INET, address, network) == 1) {
        if (prefix < 0 || prefix > 32) {
            return -1;
        }
        *family = AF_INET;
        *prefix_len = (int)prefix;
        return 0;
    }

    if (inet_pton(AF_INET6, address, network) == 1) {
        if (prefix < 0 || prefix > 128) {
            return -1;
        }
        *family = AF_INET6;
        *prefix_len = (int)prefix;
        return 0;
    }

    return -1;
}

static int normalize_allowed_login_cidrs(const char *allowed_login_cidrs,
                                         char *normalized,
                                         size_t normalized_size,
                                         bool *has_entries) {
    if (!normalized || normalized_size == 0 || !has_entries) {
        return -1;
    }

    normalized[0] = '\0';
    *has_entries = false;

    if (!allowed_login_cidrs) {
        return 0;
    }

    if (strlen(allowed_login_cidrs) >= USER_ALLOWED_LOGIN_CIDRS_MAX) {
        return -1;
    }

    char input[USER_ALLOWED_LOGIN_CIDRS_MAX] = {0};
    strncpy(input, allowed_login_cidrs, sizeof(input) - 1);

    char *saveptr = NULL;
    for (char *token = strtok_r(input, ",\n", &saveptr);
         token != NULL;
         token = strtok_r(NULL, ",\n", &saveptr)) {
        char *trimmed = trim_ascii_whitespace(token);
        if (!trimmed || trimmed[0] == '\0') {
            continue;
        }

        int family = 0;
        int prefix_len = 0;
        unsigned char network[sizeof(struct in6_addr)] = {0};
        if (parse_cidr_entry(trimmed, &family, network, &prefix_len) != 0) {
            (void)family;
            (void)prefix_len;
            return -1;
        }

        size_t current_len = strlen(normalized);
        size_t token_len = strlen(trimmed);
        size_t separator_len = *has_entries ? 1 : 0;
        if (current_len + separator_len + token_len + 1 > normalized_size) {
            return -1;
        }

        if (*has_entries) {
            normalized[current_len++] = '\n';
            normalized[current_len] = '\0';
        }

        memcpy(normalized + current_len, trimmed, token_len + 1);
        *has_entries = true;
    }

    return 0;
}

static bool ip_matches_cidr(const char *client_ip, const char *cidr) {
    if (!client_ip || !cidr) {
        return false;
    }

    unsigned char ip[sizeof(struct in6_addr)] = {0};
    int ip_family = 0;
    if (inet_pton(AF_INET, client_ip, ip) == 1) {
        ip_family = AF_INET;
    } else if (inet_pton(AF_INET6, client_ip, ip) == 1) {
        ip_family = AF_INET6;
    } else {
        return false;
    }

    unsigned char network[sizeof(struct in6_addr)] = {0};
    int cidr_family = 0;
    int prefix_len = 0;
    if (parse_cidr_entry(cidr, &cidr_family, network, &prefix_len) != 0 || cidr_family != ip_family) {
        return false;
    }

    int full_bytes = prefix_len / 8;
    int remaining_bits = prefix_len % 8;
    if (full_bytes > 0 && memcmp(ip, network, (size_t)full_bytes) != 0) {
        return false;
    }

    if (remaining_bits == 0) {
        return true;
    }

    unsigned char mask = (unsigned char)(0xFFu << (8 - remaining_bits));
    return (ip[full_bytes] & mask) == (network[full_bytes] & mask);
}

static int prepare_user_lookup_stmt(sqlite3 *db, const char *where_clause, sqlite3_stmt **stmt) {
    if (!db || !where_clause || !stmt) {
        return SQLITE_MISUSE;
    }

    bool has_totp = cached_column_exists("users", "totp_enabled");
    bool has_allowed_tags = cached_column_exists("users", "allowed_tags");
    bool has_allowed_login_cidrs = cached_column_exists("users", "allowed_login_cidrs");

    char sql[768] = {0};
    int written = snprintf(sql, sizeof(sql),
                           "SELECT id, username, email, role, api_key, created_at, "
                           "updated_at, last_login, is_active, password_change_locked, %s, %s, %s "
                           "FROM users %s;",
                           has_totp ? "totp_enabled" : "0",
                           has_allowed_tags ? "allowed_tags" : "NULL",
                           has_allowed_login_cidrs ? "allowed_login_cidrs" : "NULL",
                           where_clause);
    if (written < 0 || (size_t)written >= sizeof(sql)) {
        return SQLITE_TOOBIG;
    }

    return sqlite3_prepare_v2(db, sql, -1, stmt, NULL);
}

static void populate_user_from_stmt(sqlite3_stmt *stmt, user_t *user) {
    memset(user, 0, sizeof(*user));

    user->id = sqlite3_column_int64(stmt, 0);
    strncpy(user->username, (const char *)sqlite3_column_text(stmt, 1), sizeof(user->username) - 1);
    user->username[sizeof(user->username) - 1] = '\0';

    const char *email = (const char *)sqlite3_column_text(stmt, 2);
    if (email) {
        strncpy(user->email, email, sizeof(user->email) - 1);
        user->email[sizeof(user->email) - 1] = '\0';
    }

    user->role = (user_role_t)sqlite3_column_int(stmt, 3);

    const char *api_key = (const char *)sqlite3_column_text(stmt, 4);
    if (api_key) {
        strncpy(user->api_key, api_key, sizeof(user->api_key) - 1);
        user->api_key[sizeof(user->api_key) - 1] = '\0';
    }

    user->created_at = sqlite3_column_int64(stmt, 5);
    user->updated_at = sqlite3_column_int64(stmt, 6);
    user->last_login = sqlite3_column_int64(stmt, 7);
    user->is_active = sqlite3_column_int(stmt, 8) != 0;
    user->password_change_locked = sqlite3_column_int(stmt, 9) != 0;
    user->totp_enabled = sqlite3_column_int(stmt, 10) != 0;

    const char *allowed_tags = (const char *)sqlite3_column_text(stmt, 11);
    if (allowed_tags && allowed_tags[0] != '\0') {
        strncpy(user->allowed_tags, allowed_tags, sizeof(user->allowed_tags) - 1);
        user->allowed_tags[sizeof(user->allowed_tags) - 1] = '\0';
        user->has_tag_restriction = true;
    }

    const char *allowed_login_cidrs = (const char *)sqlite3_column_text(stmt, 12);
    if (allowed_login_cidrs && allowed_login_cidrs[0] != '\0') {
        strncpy(user->allowed_login_cidrs, allowed_login_cidrs, sizeof(user->allowed_login_cidrs) - 1);
        user->allowed_login_cidrs[sizeof(user->allowed_login_cidrs) - 1] = '\0';
        user->has_login_cidr_restriction = true;
    }
}

/**
 * Generate a random string
 *
 * @param buffer Buffer to store the random string
 * @param length Length of the random string
 * @return 0 on success, non-zero on failure
 */
static int generate_random_string(char *buffer, size_t length) {
    const char charset[] = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";

    // Initialize mbedTLS entropy and random number generator
    mbedtls_entropy_context entropy;
    mbedtls_ctr_drbg_context ctr_drbg;

    mbedtls_entropy_init(&entropy);
    mbedtls_ctr_drbg_init(&ctr_drbg);

    // Seed the random number generator
    if (mbedtls_ctr_drbg_seed(&ctr_drbg, mbedtls_entropy_func, &entropy,
                              (const unsigned char *)"lightnvr", 8) != 0) {
        log_error("Failed to seed random number generator");
        mbedtls_ctr_drbg_free(&ctr_drbg);
        mbedtls_entropy_free(&entropy);
        return -1;
    }

    // Generate random bytes
    unsigned char random_bytes[length];
    if (mbedtls_ctr_drbg_random(&ctr_drbg, random_bytes, length) != 0) {
        log_error("Failed to generate random bytes");
        mbedtls_ctr_drbg_free(&ctr_drbg);
        mbedtls_entropy_free(&entropy);
        return -1;
    }

    // Convert to alphanumeric characters
    for (size_t i = 0; i < length; i++) {
        buffer[i] = charset[random_bytes[i] % (sizeof(charset) - 1)];
    }

    buffer[length] = '\0';

    // Clean up
    mbedtls_ctr_drbg_free(&ctr_drbg);
    mbedtls_entropy_free(&entropy);

    return 0;
}

/**
 * Hash a password with a salt using PBKDF2
 *
 * @param password Password to hash
 * @param salt Salt to use for hashing
 * @param salt_length Length of the salt
 * @param hash Buffer to store the hash
 * @param hash_length Length of the hash buffer
 * @return 0 on success, non-zero on failure
 */
static int hash_password(const char *password, const unsigned char *salt, size_t salt_length,
                        unsigned char *hash, size_t hash_length) {
    // Use mbedTLS to implement PBKDF2 with SHA-256
    mbedtls_md_context_t ctx;
    const mbedtls_md_info_t *md_info;

    mbedtls_md_init(&ctx);
    md_info = mbedtls_md_info_from_type(MBEDTLS_MD_SHA256);

    if (md_info == NULL) {
        log_error("Failed to get SHA-256 info");
        return -1;
    }

    if (mbedtls_md_setup(&ctx, md_info, 1) != 0) {
        log_error("Failed to set up message digest context");
        mbedtls_md_free(&ctx);
        return -1;
    }

    // Perform PBKDF2 key derivation
    if (mbedtls_pkcs5_pbkdf2_hmac(&ctx,
                                 (const unsigned char *)password, strlen(password),
                                 salt, salt_length,
                                 10000, // iterations
                                 hash_length, hash) != 0) {
        log_error("Failed to hash password");
        mbedtls_md_free(&ctx);
        return -1;
    }

    mbedtls_md_free(&ctx);
    return 0;
}

/**
 * Convert binary data to hexadecimal string
 *
 * @param data Binary data
 * @param data_length Length of the binary data
 * @param hex Buffer to store the hexadecimal string
 * @param hex_length Length of the hex buffer
 * @return 0 on success, non-zero on failure
 */
static int bin_to_hex(const unsigned char *data, size_t data_length,
                     char *hex, size_t hex_length) {
    if (hex_length < data_length * 2 + 1) {
        log_error("Hex buffer too small");
        return -1;
    }

    for (size_t i = 0; i < data_length; i++) {
        snprintf(hex + i * 2, 3, "%02x", data[i]);
    }

    return 0;
}

static int hash_token_identifier(const char *token, char *token_hash, size_t token_hash_size) {
    if (!token || token[0] == '\0') {
        return -1;
    }
    if (!token_hash || token_hash_size < (SHA256_DIGEST_LENGTH * 2 + 1)) {
        log_error("Token hash buffer too small");
        return -1;
    }

    unsigned char digest[SHA256_DIGEST_LENGTH];
    mbedtls_sha256((const unsigned char *)token, strlen(token), digest, 0);

    return bin_to_hex(digest, SHA256_DIGEST_LENGTH, token_hash, token_hash_size);
}

/**
 * Convert hexadecimal string to binary data
 *
 * @param hex Hexadecimal string
 * @param data Buffer to store the binary data
 * @param data_length Length of the data buffer
 * @return 0 on success, non-zero on failure
 */
static int hex_to_bin(const char *hex, unsigned char *data, size_t data_length) {
    size_t hex_length = strlen(hex);

    if (hex_length / 2 > data_length) {
        log_error("Data buffer too small");
        return -1;
    }

    for (size_t i = 0; i < hex_length; i += 2) {
        const char byte[3] = {hex[i], hex[i + 1], '\0'};
        data[i / 2] = (unsigned char)strtol(byte, NULL, 16);
    }

    return 0;
}

/**
 * Initialize the authentication system
 */
int db_auth_init(void) {
    log_info("Initializing authentication system");

    // Check if the default admin user exists
    user_t user;
    int rc = db_auth_get_user_by_username("admin", &user);

    if (rc == 0) {
        log_info("Default admin user already exists");
        return 0;
    }

    // Create the default admin user
    // If a password is set in the config file, use it; otherwise use "admin"
    const char *initial_password = NULL;

    if (g_config.web_password[0] != '\0') {
        initial_password = g_config.web_password;
        log_info("Creating default admin user with password from config file");
    } else {
        initial_password = "admin";
        log_info("Creating default admin user with default password");
    }

    rc = db_auth_create_user("admin", initial_password, NULL, USER_ROLE_ADMIN, true, NULL);
    if (rc != 0) {
        log_error("Failed to create default admin user");
        return -1;
    }

    log_info("********************************************************");
    log_info("***    Default admin user created successfully       ***");
    log_info("***    Username: admin                               ***");
    log_info("***    Password: admin                               ***");
    log_info("***    PLEASE CHANGE THIS PASSWORD IMMEDIATELY!      ***");
    log_info("********************************************************");

    return 0;
}

/**
 * Create a new user
 */
int db_auth_create_user(const char *username, const char *password, const char *email,
                       user_role_t role, bool is_active, int64_t *user_id) {
    if (!username || !password) {
        log_error("Username and password are required");
        return -1;
    }

    sqlite3 *db = get_db_handle();
    if (!db) {
        log_error("Database not initialized");
        return -1;
    }

    // Check if the username already exists
    sqlite3_stmt *stmt;
    int rc = sqlite3_prepare_v2(db, "SELECT id FROM users WHERE username = ?;", -1, &stmt, NULL);
    if (rc != SQLITE_OK) {
        log_error("Failed to prepare statement: %s", sqlite3_errmsg(db));
        return -1;
    }

    sqlite3_bind_text(stmt, 1, username, -1, SQLITE_STATIC);

    if (sqlite3_step(stmt) == SQLITE_ROW) {
        log_error("Username already exists: %s", username);
        sqlite3_finalize(stmt);
        return -1;
    }

    sqlite3_finalize(stmt);

    // Generate a salt
    unsigned char salt[SALT_LENGTH];

    // Initialize mbedTLS entropy and random number generator
    mbedtls_entropy_context entropy;
    mbedtls_ctr_drbg_context ctr_drbg;

    mbedtls_entropy_init(&entropy);
    mbedtls_ctr_drbg_init(&ctr_drbg);

    // Seed the random number generator
    if (mbedtls_ctr_drbg_seed(&ctr_drbg, mbedtls_entropy_func, &entropy,
                              (const unsigned char *)"lightnvr", 8) != 0) {
        log_error("Failed to seed random number generator");
        mbedtls_ctr_drbg_free(&ctr_drbg);
        mbedtls_entropy_free(&entropy);
        return -1;
    }

    // Generate random bytes for salt
    if (mbedtls_ctr_drbg_random(&ctr_drbg, salt, SALT_LENGTH) != 0) {
        log_error("Failed to generate salt");
        mbedtls_ctr_drbg_free(&ctr_drbg);
        mbedtls_entropy_free(&entropy);
        return -1;
    }

    // Clean up
    mbedtls_ctr_drbg_free(&ctr_drbg);
    mbedtls_entropy_free(&entropy);

    // Convert salt to hex
    char salt_hex[SALT_LENGTH * 2 + 1];
    if (bin_to_hex(salt, SALT_LENGTH, salt_hex, sizeof(salt_hex)) != 0) {
        log_error("Failed to convert salt to hex");
        return -1;
    }

    // Hash the password
    unsigned char hash[SHA256_DIGEST_LENGTH];
    if (hash_password(password, salt, SALT_LENGTH, hash, SHA256_DIGEST_LENGTH) != 0) {
        log_error("Failed to hash password");
        return -1;
    }

    // Convert hash to hex
    char hash_hex[SHA256_DIGEST_LENGTH * 2 + 1];
    if (bin_to_hex(hash, SHA256_DIGEST_LENGTH, hash_hex, sizeof(hash_hex)) != 0) {
        log_error("Failed to convert hash to hex");
        return -1;
    }

    // Get current timestamp
    time_t now = time(NULL);

    // Insert the user
    rc = sqlite3_prepare_v2(db,
                           "INSERT INTO users (username, password_hash, salt, role, email, "
                           "created_at, updated_at, is_active) "
                           "VALUES (?, ?, ?, ?, ?, ?, ?, ?);",
                           -1, &stmt, NULL);
    if (rc != SQLITE_OK) {
        log_error("Failed to prepare statement: %s", sqlite3_errmsg(db));
        return -1;
    }

    sqlite3_bind_text(stmt, 1, username, -1, SQLITE_STATIC);
    sqlite3_bind_text(stmt, 2, hash_hex, -1, SQLITE_STATIC);
    sqlite3_bind_text(stmt, 3, salt_hex, -1, SQLITE_STATIC);
    sqlite3_bind_int(stmt, 4, role);
    sqlite3_bind_text(stmt, 5, email ? email : "", -1, SQLITE_STATIC);
    sqlite3_bind_int64(stmt, 6, now);
    sqlite3_bind_int64(stmt, 7, now);
    sqlite3_bind_int(stmt, 8, is_active ? 1 : 0);

    rc = sqlite3_step(stmt);
    if (rc != SQLITE_DONE) {
        log_error("Failed to insert user: %s", sqlite3_errmsg(db));
        sqlite3_finalize(stmt);
        return -1;
    }

    // Get the user ID
    int64_t id = sqlite3_last_insert_rowid(db);
    if (user_id) {
        *user_id = id;
    }

    sqlite3_finalize(stmt);

    log_info("User created successfully: %s (ID: %lld)", username, (long long)id);
    return 0;
}

/**
 * Update a user
 */
int db_auth_update_user(int64_t user_id, const char *email, int role, int is_active) {
    sqlite3 *db = get_db_handle();
    if (!db) {
        log_error("Database not initialized");
        return -1;
    }

    // Check if the user exists
    sqlite3_stmt *stmt;
    int rc = sqlite3_prepare_v2(db, "SELECT id FROM users WHERE id = ?;", -1, &stmt, NULL);
    if (rc != SQLITE_OK) {
        log_error("Failed to prepare statement: %s", sqlite3_errmsg(db));
        return -1;
    }

    sqlite3_bind_int64(stmt, 1, user_id);

    if (sqlite3_step(stmt) != SQLITE_ROW) {
        log_error("User not found: %lld", (long long)user_id);
        sqlite3_finalize(stmt);
        return -1;
    }

    sqlite3_finalize(stmt);

    // Build the update query
    char query[512] = "UPDATE users SET updated_at = ?";

    if (email) {
        strncat(query, ", email = ?", sizeof(query) - strlen(query) - 1);
    }

    if (role >= 0) {
        strncat(query, ", role = ?", sizeof(query) - strlen(query) - 1);
    }

    if (is_active >= 0) {
        strncat(query, ", is_active = ?", sizeof(query) - strlen(query) - 1);
    }

    strncat(query, " WHERE id = ?;", sizeof(query) - strlen(query) - 1);

    // Prepare the statement
    rc = sqlite3_prepare_v2(db, query, -1, &stmt, NULL);
    if (rc != SQLITE_OK) {
        log_error("Failed to prepare statement: %s", sqlite3_errmsg(db));
        return -1;
    }

    // Bind parameters
    time_t now = time(NULL);
    sqlite3_bind_int64(stmt, 1, now);

    int param_index = 2;

    if (email) {
        sqlite3_bind_text(stmt, param_index++, email, -1, SQLITE_STATIC);
    }

    if (role >= 0) {
        sqlite3_bind_int(stmt, param_index++, role);
    }

    if (is_active >= 0) {
        sqlite3_bind_int(stmt, param_index++, is_active ? 1 : 0);
    }

    sqlite3_bind_int64(stmt, param_index, user_id);

    // Execute the statement
    rc = sqlite3_step(stmt);
    if (rc != SQLITE_DONE) {
        log_error("Failed to update user: %s", sqlite3_errmsg(db));
        sqlite3_finalize(stmt);
        return -1;
    }

    sqlite3_finalize(stmt);

    log_info("User updated successfully: %lld", (long long)user_id);
    return 0;
}

/**
 * Change a user's password
 */
int db_auth_change_password(int64_t user_id, const char *new_password) {
    if (!new_password) {
        log_error("New password is required");
        return -1;
    }

    sqlite3 *db = get_db_handle();
    if (!db) {
        log_error("Database not initialized");
        return -1;
    }

    // Check if the user exists and if password changes are locked
    sqlite3_stmt *stmt;
    int rc = sqlite3_prepare_v2(db, "SELECT id, password_change_locked FROM users WHERE id = ?;", -1, &stmt, NULL);
    if (rc != SQLITE_OK) {
        log_error("Failed to prepare statement: %s", sqlite3_errmsg(db));
        return -1;
    }

    sqlite3_bind_int64(stmt, 1, user_id);

    if (sqlite3_step(stmt) != SQLITE_ROW) {
        log_error("User not found: %lld", (long long)user_id);
        sqlite3_finalize(stmt);
        return -1;
    }

    // Check if password changes are locked for this user
    bool password_locked = sqlite3_column_int(stmt, 1) != 0;
    if (password_locked) {
        log_warn("Password changes are locked for user: %lld", (long long)user_id);
        sqlite3_finalize(stmt);
        return -2; // Special error code for locked password
    }

    sqlite3_finalize(stmt);

    // Generate a new salt
    unsigned char salt[SALT_LENGTH];

    // Initialize mbedTLS entropy and random number generator
    mbedtls_entropy_context entropy;
    mbedtls_ctr_drbg_context ctr_drbg;

    mbedtls_entropy_init(&entropy);
    mbedtls_ctr_drbg_init(&ctr_drbg);

    // Seed the random number generator
    if (mbedtls_ctr_drbg_seed(&ctr_drbg, mbedtls_entropy_func, &entropy,
                              (const unsigned char *)"lightnvr", 8) != 0) {
        log_error("Failed to seed random number generator");
        mbedtls_ctr_drbg_free(&ctr_drbg);
        mbedtls_entropy_free(&entropy);
        return -1;
    }

    // Generate random bytes for salt
    if (mbedtls_ctr_drbg_random(&ctr_drbg, salt, SALT_LENGTH) != 0) {
        log_error("Failed to generate salt");
        mbedtls_ctr_drbg_free(&ctr_drbg);
        mbedtls_entropy_free(&entropy);
        return -1;
    }

    // Clean up
    mbedtls_ctr_drbg_free(&ctr_drbg);
    mbedtls_entropy_free(&entropy);

    // Convert salt to hex
    char salt_hex[SALT_LENGTH * 2 + 1];
    if (bin_to_hex(salt, SALT_LENGTH, salt_hex, sizeof(salt_hex)) != 0) {
        log_error("Failed to convert salt to hex");
        return -1;
    }

    // Hash the password
    unsigned char hash[SHA256_DIGEST_LENGTH];
    if (hash_password(new_password, salt, SALT_LENGTH, hash, SHA256_DIGEST_LENGTH) != 0) {
        log_error("Failed to hash password");
        return -1;
    }

    // Convert hash to hex
    char hash_hex[SHA256_DIGEST_LENGTH * 2 + 1];
    if (bin_to_hex(hash, SHA256_DIGEST_LENGTH, hash_hex, sizeof(hash_hex)) != 0) {
        log_error("Failed to convert hash to hex");
        return -1;
    }

    // Update the password
    rc = sqlite3_prepare_v2(db,
                           "UPDATE users SET password_hash = ?, salt = ?, updated_at = ? "
                           "WHERE id = ?;",
                           -1, &stmt, NULL);
    if (rc != SQLITE_OK) {
        log_error("Failed to prepare statement: %s", sqlite3_errmsg(db));
        return -1;
    }

    time_t now = time(NULL);
    sqlite3_bind_text(stmt, 1, hash_hex, -1, SQLITE_STATIC);
    sqlite3_bind_text(stmt, 2, salt_hex, -1, SQLITE_STATIC);
    sqlite3_bind_int64(stmt, 3, now);
    sqlite3_bind_int64(stmt, 4, user_id);

    rc = sqlite3_step(stmt);
    if (rc != SQLITE_DONE) {
        log_error("Failed to update password: %s", sqlite3_errmsg(db));
        sqlite3_finalize(stmt);
        return -1;
    }

    sqlite3_finalize(stmt);

    log_info("Password changed successfully for user: %lld", (long long)user_id);
    return 0;
}

/**
 * Delete a user
 */
int db_auth_delete_user(int64_t user_id) {
    sqlite3 *db = get_db_handle();
    if (!db) {
        log_error("Database not initialized");
        return -1;
    }

    // Check if the user exists
    sqlite3_stmt *stmt;
    int rc = sqlite3_prepare_v2(db, "SELECT id FROM users WHERE id = ?;", -1, &stmt, NULL);
    if (rc != SQLITE_OK) {
        log_error("Failed to prepare statement: %s", sqlite3_errmsg(db));
        return -1;
    }

    sqlite3_bind_int64(stmt, 1, user_id);

    if (sqlite3_step(stmt) != SQLITE_ROW) {
        log_error("User not found: %lld", (long long)user_id);
        sqlite3_finalize(stmt);
        return -1;
    }

    sqlite3_finalize(stmt);

    // Delete the user
    rc = sqlite3_prepare_v2(db, "DELETE FROM users WHERE id = ?;", -1, &stmt, NULL);
    if (rc != SQLITE_OK) {
        log_error("Failed to prepare statement: %s", sqlite3_errmsg(db));
        return -1;
    }

    sqlite3_bind_int64(stmt, 1, user_id);

    rc = sqlite3_step(stmt);
    if (rc != SQLITE_DONE) {
        log_error("Failed to delete user: %s", sqlite3_errmsg(db));
        sqlite3_finalize(stmt);
        return -1;
    }

    sqlite3_finalize(stmt);

    log_info("User deleted successfully: %lld", (long long)user_id);
    return 0;
}

/**
 * Get a user by ID
 */
int db_auth_get_user_by_id(int64_t user_id, user_t *user) {
    if (!user) {
        log_error("User pointer is NULL");
        return -1;
    }

    sqlite3 *db = get_db_handle();
    if (!db) {
        log_error("Database not initialized");
        return -1;
    }

    sqlite3_stmt *stmt;
    int rc = prepare_user_lookup_stmt(db, "WHERE id = ?", &stmt);
    if (rc != SQLITE_OK) {
        log_error("Failed to prepare statement: %s", sqlite3_errmsg(db));
        return -1;
    }

    sqlite3_bind_int64(stmt, 1, user_id);

    if (sqlite3_step(stmt) != SQLITE_ROW) {
        log_error("User not found: %lld", (long long)user_id);
        sqlite3_finalize(stmt);
        return -1;
    }

    populate_user_from_stmt(stmt, user);

    sqlite3_finalize(stmt);

    return 0;
}

/**
 * Get a user by username
 */
int db_auth_get_user_by_username(const char *username, user_t *user) {
    if (!username || !user) {
        log_error("Username and user pointer are required");
        return -1;
    }

    sqlite3 *db = get_db_handle();
    if (!db) {
        log_error("Database not initialized");
        return -1;
    }

    sqlite3_stmt *stmt;
    int rc = prepare_user_lookup_stmt(db, "WHERE username = ?", &stmt);
    if (rc != SQLITE_OK) {
        log_error("Failed to prepare statement: %s", sqlite3_errmsg(db));
        return -1;
    }

    sqlite3_bind_text(stmt, 1, username, -1, SQLITE_STATIC);

    if (sqlite3_step(stmt) != SQLITE_ROW) {
        log_debug("User not found: %s", username);
        sqlite3_finalize(stmt);
        return -1;
    }

    populate_user_from_stmt(stmt, user);

    sqlite3_finalize(stmt);

    return 0;
}

/**
 * Get a user by API key
 */
int db_auth_get_user_by_api_key(const char *api_key, user_t *user) {
    if (!api_key || !user) {
        log_error("API key and user pointer are required");
        return -1;
    }

    sqlite3 *db = get_db_handle();
    if (!db) {
        log_error("Database not initialized");
        return -1;
    }

    sqlite3_stmt *stmt;
    int rc = prepare_user_lookup_stmt(db, "WHERE api_key = ?", &stmt);
    if (rc != SQLITE_OK) {
        log_error("Failed to prepare statement: %s", sqlite3_errmsg(db));
        return -1;
    }

    sqlite3_bind_text(stmt, 1, api_key, -1, SQLITE_STATIC);

    if (sqlite3_step(stmt) != SQLITE_ROW) {
        log_debug("User not found for API key");
        sqlite3_finalize(stmt);
        return -1;
    }

    populate_user_from_stmt(stmt, user);

    sqlite3_finalize(stmt);

    return 0;
}

/**
 * Generate a new API key for a user
 */
int db_auth_generate_api_key(int64_t user_id, char *api_key, size_t api_key_size) {
    if (!api_key || api_key_size < 33) {
        log_error("API key buffer is too small");
        return -1;
    }

    sqlite3 *db = get_db_handle();
    if (!db) {
        log_error("Database not initialized");
        return -1;
    }

    // Check if the user exists
    sqlite3_stmt *stmt;
    int rc = sqlite3_prepare_v2(db, "SELECT id FROM users WHERE id = ?;", -1, &stmt, NULL);
    if (rc != SQLITE_OK) {
        log_error("Failed to prepare statement: %s", sqlite3_errmsg(db));
        return -1;
    }

    sqlite3_bind_int64(stmt, 1, user_id);

    if (sqlite3_step(stmt) != SQLITE_ROW) {
        log_error("User not found: %lld", (long long)user_id);
        sqlite3_finalize(stmt);
        return -1;
    }

    sqlite3_finalize(stmt);

    // Generate a random API key
    if (generate_random_string(api_key, 32) != 0) {
        log_error("Failed to generate API key");
        return -1;
    }

    // Update the user
    rc = sqlite3_prepare_v2(db,
                           "UPDATE users SET api_key = ?, updated_at = ? WHERE id = ?;",
                           -1, &stmt, NULL);
    if (rc != SQLITE_OK) {
        log_error("Failed to prepare statement: %s", sqlite3_errmsg(db));
        return -1;
    }

    time_t now = time(NULL);
    sqlite3_bind_text(stmt, 1, api_key, -1, SQLITE_STATIC);
    sqlite3_bind_int64(stmt, 2, now);
    sqlite3_bind_int64(stmt, 3, user_id);

    rc = sqlite3_step(stmt);
    if (rc != SQLITE_DONE) {
        log_error("Failed to update API key: %s", sqlite3_errmsg(db));
        sqlite3_finalize(stmt);
        return -1;
    }

    sqlite3_finalize(stmt);

    log_info("API key generated successfully for user: %lld", (long long)user_id);
    return 0;
}

/**
 * Authenticate a user with username and password
 */
int db_auth_authenticate(const char *username, const char *password, int64_t *user_id) {
    if (!username || !password) {
        log_error("Username and password are required");
        return -1;
    }

    sqlite3 *db = get_db_handle();
    if (!db) {
        log_error("Database not initialized");
        return -1;
    }

    // Query the user
    sqlite3_stmt *stmt;
    int rc = sqlite3_prepare_v2(db,
                               "SELECT id, password_hash, salt, is_active FROM users WHERE username = ?;",
                               -1, &stmt, NULL);
    if (rc != SQLITE_OK) {
        log_error("Failed to prepare statement: %s", sqlite3_errmsg(db));
        return -1;
    }

    sqlite3_bind_text(stmt, 1, username, -1, SQLITE_STATIC);

    if (sqlite3_step(stmt) != SQLITE_ROW) {
        log_warn("Authentication failed: User not found: %s", username);
        sqlite3_finalize(stmt);
        return -1;
    }

    // Check if the user is active
    int is_active = sqlite3_column_int(stmt, 3);
    if (!is_active) {
        log_warn("Authentication failed: User is inactive: %s", username);
        sqlite3_finalize(stmt);
        return -1;
    }

    // Get the password hash and salt
    const char *hash_hex = (const char *)sqlite3_column_text(stmt, 1);
    const char *salt_hex = (const char *)sqlite3_column_text(stmt, 2);

    if (!hash_hex || !salt_hex) {
        log_error("Authentication failed: Invalid password hash or salt for user: %s", username);
        sqlite3_finalize(stmt);
        return -1;
    }

    // Convert salt from hex to binary
    unsigned char salt[SALT_LENGTH];
    if (hex_to_bin(salt_hex, salt, SALT_LENGTH) != 0) {
        log_error("Authentication failed: Failed to convert salt from hex for user: %s", username);
        sqlite3_finalize(stmt);
        return -1;
    }

    // Hash the provided password
    unsigned char hash[SHA256_DIGEST_LENGTH];
    if (hash_password(password, salt, SALT_LENGTH, hash, SHA256_DIGEST_LENGTH) != 0) {
        log_error("Authentication failed: Failed to hash password for user: %s", username);
        sqlite3_finalize(stmt);
        return -1;
    }

    // Convert hash to hex
    char computed_hash_hex[SHA256_DIGEST_LENGTH * 2 + 1];
    if (bin_to_hex(hash, SHA256_DIGEST_LENGTH, computed_hash_hex, sizeof(computed_hash_hex)) != 0) {
        log_error("Authentication failed: Failed to convert hash to hex for user: %s", username);
        sqlite3_finalize(stmt);
        return -1;
    }

    // Compare the hashes
    if (strcmp(hash_hex, computed_hash_hex) != 0) {
        log_warn("Authentication failed: Invalid password for user: %s", username);
        sqlite3_finalize(stmt);
        return -1;
    }

    // Authentication successful
    int64_t id = sqlite3_column_int64(stmt, 0);
    if (user_id) {
        *user_id = id;
    }

    // Update last login time
    sqlite3_finalize(stmt);

    rc = sqlite3_prepare_v2(db,
                           "UPDATE users SET last_login = ? WHERE id = ?;",
                           -1, &stmt, NULL);
    if (rc != SQLITE_OK) {
        log_error("Failed to prepare statement: %s", sqlite3_errmsg(db));
        // Still return success since authentication was successful
        return 0;
    }

    time_t now = time(NULL);
    sqlite3_bind_int64(stmt, 1, now);
    sqlite3_bind_int64(stmt, 2, id);

    rc = sqlite3_step(stmt);
    if (rc != SQLITE_DONE) {
        log_error("Failed to update last login time: %s", sqlite3_errmsg(db));
        // Still return success since authentication was successful
    }

    sqlite3_finalize(stmt);

    log_info("Authentication successful for user: %s (ID: %lld)", username, (long long)id);
    return 0;
}

/**
 * Verify a password for a specific user ID
 */
int db_auth_verify_password(int64_t user_id, const char *password) {
    if (!password) {
        log_error("Password is required");
        return -1;
    }

    sqlite3 *db = get_db_handle();
    if (!db) {
        log_error("Database not initialized");
        return -1;
    }

    // Query the user
    sqlite3_stmt *stmt;
    int rc = sqlite3_prepare_v2(db,
                               "SELECT password_hash, salt, is_active FROM users WHERE id = ?;",
                               -1, &stmt, NULL);
    if (rc != SQLITE_OK) {
        log_error("Failed to prepare statement: %s", sqlite3_errmsg(db));
        return -1;
    }

    sqlite3_bind_int64(stmt, 1, user_id);

    if (sqlite3_step(stmt) != SQLITE_ROW) {
        log_warn("Password verification failed: User not found: %lld", (long long)user_id);
        sqlite3_finalize(stmt);
        return -1;
    }

    // Check if the user is active
    int is_active = sqlite3_column_int(stmt, 2);
    if (!is_active) {
        log_warn("Password verification failed: User is inactive: %lld", (long long)user_id);
        sqlite3_finalize(stmt);
        return -1;
    }

    // Get the password hash and salt
    const char *hash_hex = (const char *)sqlite3_column_text(stmt, 0);
    const char *salt_hex = (const char *)sqlite3_column_text(stmt, 1);

    if (!hash_hex || !salt_hex) {
        log_error("Password verification failed: Invalid password hash or salt for user: %lld", (long long)user_id);
        sqlite3_finalize(stmt);
        return -1;
    }

    // Convert salt from hex to binary
    unsigned char salt[SALT_LENGTH];
    if (hex_to_bin(salt_hex, salt, SALT_LENGTH) != 0) {
        log_error("Password verification failed: Failed to convert salt from hex for user: %lld", (long long)user_id);
        sqlite3_finalize(stmt);
        return -1;
    }

    // Hash the provided password
    unsigned char hash[SHA256_DIGEST_LENGTH];
    if (hash_password(password, salt, SALT_LENGTH, hash, SHA256_DIGEST_LENGTH) != 0) {
        log_error("Password verification failed: Failed to hash password for user: %lld", (long long)user_id);
        sqlite3_finalize(stmt);
        return -1;
    }

    // Convert hash to hex
    char computed_hash_hex[SHA256_DIGEST_LENGTH * 2 + 1];
    if (bin_to_hex(hash, SHA256_DIGEST_LENGTH, computed_hash_hex, sizeof(computed_hash_hex)) != 0) {
        log_error("Password verification failed: Failed to convert hash to hex for user: %lld", (long long)user_id);
        sqlite3_finalize(stmt);
        return -1;
    }

    // Compare the hashes
    if (strcmp(hash_hex, computed_hash_hex) != 0) {
        log_warn("Password verification failed: Invalid password for user: %lld", (long long)user_id);
        sqlite3_finalize(stmt);
        return -1;
    }

    sqlite3_finalize(stmt);

    log_debug("Password verification successful for user: %lld", (long long)user_id);
    return 0;
}

/**
 * Set the password change lock status for a user
 */
int db_auth_set_password_lock(int64_t user_id, bool locked) {
    sqlite3 *db = get_db_handle();
    if (!db) {
        log_error("Database not initialized");
        return -1;
    }

    // Update the password_change_locked field
    sqlite3_stmt *stmt;
    int rc = sqlite3_prepare_v2(db,
                               "UPDATE users SET password_change_locked = ?, updated_at = ? WHERE id = ?;",
                               -1, &stmt, NULL);
    if (rc != SQLITE_OK) {
        log_error("Failed to prepare statement: %s", sqlite3_errmsg(db));
        return -1;
    }

    time_t now = time(NULL);
    sqlite3_bind_int(stmt, 1, locked ? 1 : 0);
    sqlite3_bind_int64(stmt, 2, now);
    sqlite3_bind_int64(stmt, 3, user_id);

    rc = sqlite3_step(stmt);
    if (rc != SQLITE_DONE) {
        log_error("Failed to update password lock status: %s", sqlite3_errmsg(db));
        sqlite3_finalize(stmt);
        return -1;
    }

    sqlite3_finalize(stmt);

    log_info("Password lock status updated for user: %lld (locked: %d)", (long long)user_id, locked);
    return 0;
}

/**
 * Create a new session for a user
 */
int db_auth_create_session(int64_t user_id, const char *ip_address, const char *user_agent,
                          int expiry_seconds, char *token, size_t token_size) {
    if (!token || token_size < 33) {
        log_error("Token buffer is too small");
        return -1;
    }

    sqlite3 *db = get_db_handle();
    if (!db) {
        log_error("Database not initialized");
        return -1;
    }

    // Check if the user exists
    sqlite3_stmt *stmt;
    int rc = sqlite3_prepare_v2(db, "SELECT id FROM users WHERE id = ?;", -1, &stmt, NULL);
    if (rc != SQLITE_OK) {
        log_error("Failed to prepare statement: %s", sqlite3_errmsg(db));
        return -1;
    }

    sqlite3_bind_int64(stmt, 1, user_id);

    if (sqlite3_step(stmt) != SQLITE_ROW) {
        log_error("User not found: %lld", (long long)user_id);
        sqlite3_finalize(stmt);
        return -1;
    }

    sqlite3_finalize(stmt);

    // Generate a random token
    if (generate_random_string(token, 32) != 0) {
        log_error("Failed to generate session token");
        return -1;
    }

    // Get current timestamp
    time_t now = time(NULL);

    // Calculate idle and absolute expiry times.
    int absolute_expiry = expiry_seconds > 0 ? expiry_seconds : default_session_absolute_expiry_seconds();
    int idle_expiry = expiry_seconds > 0 ? expiry_seconds : default_session_idle_expiry_seconds();
    time_t expires_at = now + absolute_expiry;
    time_t idle_expires_at = now + idle_expiry;

    // Some deployments may not yet have the optional ip_address/user_agent columns
    // (e.g., databases created before the 0018_add_session_tracking migration).
    // Use cached_column_exists to build a compatible INSERT that only references
    // columns that actually exist, so login still works on older schemas.
    bool has_ip_column = cached_column_exists("sessions", "ip_address");
    bool has_ua_column = cached_column_exists("sessions", "user_agent");
    bool has_last_activity_column = cached_column_exists("sessions", "last_activity_at");
    bool has_idle_expires_column = cached_column_exists("sessions", "idle_expires_at");

    const char *sql = NULL;
    if (has_ip_column && has_ua_column && has_last_activity_column && has_idle_expires_column) {
        sql = "INSERT INTO sessions (user_id, token, created_at, last_activity_at, idle_expires_at, expires_at, ip_address, user_agent) "
              "VALUES (?, ?, ?, ?, ?, ?, ?, ?);";
    } else if (has_ip_column && has_ua_column) {
        sql = "INSERT INTO sessions (user_id, token, created_at, expires_at, ip_address, user_agent) "
              "VALUES (?, ?, ?, ?, ?, ?);";
    } else if (has_ip_column && !has_ua_column) {
        sql = "INSERT INTO sessions (user_id, token, created_at, expires_at, ip_address) "
              "VALUES (?, ?, ?, ?, ?);";
    } else if (!has_ip_column && has_ua_column) {
        sql = "INSERT INTO sessions (user_id, token, created_at, expires_at, user_agent) "
              "VALUES (?, ?, ?, ?, ?);";
    } else {
        sql = "INSERT INTO sessions (user_id, token, created_at, expires_at) "
              "VALUES (?, ?, ?, ?);";
    }

    rc = sqlite3_prepare_v2(db, sql, -1, &stmt, NULL);
    if (rc != SQLITE_OK) {
        log_error("Failed to prepare session insert statement: %s", sqlite3_errmsg(db));
        return -1;
    }

    int param_index = 1;
    sqlite3_bind_int64(stmt, param_index++, user_id);
    sqlite3_bind_text(stmt, param_index++, token, -1, SQLITE_STATIC);
    sqlite3_bind_int64(stmt, param_index++, now);
    if (has_last_activity_column && has_idle_expires_column) {
        sqlite3_bind_int64(stmt, param_index++, now);
        sqlite3_bind_int64(stmt, param_index++, idle_expires_at);
    }
    sqlite3_bind_int64(stmt, param_index++, expires_at);

    if (has_ip_column) {
        sqlite3_bind_text(stmt, param_index++, ip_address ? ip_address : "", -1, SQLITE_STATIC);
    }

    if (has_ua_column) {
        sqlite3_bind_text(stmt, param_index++, user_agent ? user_agent : "", -1, SQLITE_STATIC);
    }

    rc = sqlite3_step(stmt);
    if (rc != SQLITE_DONE) {
        log_error("Failed to insert session: %s", sqlite3_errmsg(db));
        sqlite3_finalize(stmt);
        return -1;
    }

    sqlite3_finalize(stmt);

    log_info("Session created successfully for user: %lld", (long long)user_id);
    return 0;
}

/**
 * Validate a session token
 */
int db_auth_validate_session_with_context(const char *token, int64_t *user_id,
                                          const char *ip_address, const char *user_agent) {
    if (!token) {
        log_error("Token is required");
        return -1;
    }

    sqlite3 *db = get_db_handle();
    if (!db) {
        log_error("Database not initialized");
        return -1;
    }

    bool has_idle_expires_column = cached_column_exists("sessions", "idle_expires_at");
    bool has_last_activity_column = cached_column_exists("sessions", "last_activity_at");
    bool has_ip_column = cached_column_exists("sessions", "ip_address");
    bool has_ua_column = cached_column_exists("sessions", "user_agent");
    bool has_tracking_columns = has_idle_expires_column && has_last_activity_column;

    // Query the session
    sqlite3_stmt *stmt;
    const char *sql = has_tracking_columns
        ? "SELECT s.id, s.user_id, s.expires_at, s.idle_expires_at, COALESCE(s.last_activity_at, s.created_at), u.is_active "
          "FROM sessions s "
          "JOIN users u ON s.user_id = u.id "
          "WHERE s.token = ?;"
        : has_idle_expires_column
        ? "SELECT s.id, s.user_id, s.expires_at, s.idle_expires_at, u.is_active "
          "FROM sessions s "
          "JOIN users u ON s.user_id = u.id "
          "WHERE s.token = ?;"
        : "SELECT s.id, s.user_id, s.expires_at, u.is_active "
          "FROM sessions s "
          "JOIN users u ON s.user_id = u.id "
          "WHERE s.token = ?;";
    int rc = sqlite3_prepare_v2(db, sql, -1, &stmt, NULL);
    if (rc != SQLITE_OK) {
        log_error("Failed to prepare statement: %s", sqlite3_errmsg(db));
        return -1;
    }

    sqlite3_bind_text(stmt, 1, token, -1, SQLITE_STATIC);

    if (sqlite3_step(stmt) != SQLITE_ROW) {
        log_debug("Session not found for token");
        sqlite3_finalize(stmt);
        return -1;
    }

    int64_t session_id = sqlite3_column_int64(stmt, 0);

    // Check if the session has expired
    time_t expires_at = sqlite3_column_int64(stmt, 2);
    time_t idle_expires_at = has_idle_expires_column ? sqlite3_column_int64(stmt, 3) : expires_at;
    time_t last_activity_at = has_tracking_columns ? sqlite3_column_int64(stmt, 4) : 0;
    time_t now = time(NULL);

    if (now > expires_at || now > idle_expires_at) {
        log_debug("Session has expired");
        sqlite3_finalize(stmt);
        return -1;
    }

    // Check if the user is active
    int is_active = sqlite3_column_int(stmt, has_tracking_columns ? 5 : (has_idle_expires_column ? 4 : 3));
    if (!is_active) {
        log_debug("User is inactive");
        sqlite3_finalize(stmt);
        return -1;
    }

    // Session is valid
    int64_t id = sqlite3_column_int64(stmt, 1);
    if (user_id) {
        *user_id = id;
    }

    sqlite3_finalize(stmt);

    bool update_ip = false;
    bool update_ua = false;
    if ((has_ip_column && ip_address) || (has_ua_column && user_agent)) {
        const char *tracking_sql = has_ip_column && has_ua_column
            ? "SELECT COALESCE(ip_address, ''), COALESCE(user_agent, '') FROM sessions WHERE id = ?;"
            : has_ip_column
            ? "SELECT COALESCE(ip_address, '') FROM sessions WHERE id = ?;"
            : has_ua_column
            ? "SELECT COALESCE(user_agent, '') FROM sessions WHERE id = ?;"
            : NULL;
        if (tracking_sql) {
            rc = sqlite3_prepare_v2(db, tracking_sql, -1, &stmt, NULL);
            if (rc == SQLITE_OK) {
                sqlite3_bind_int64(stmt, 1, session_id);
                if (sqlite3_step(stmt) == SQLITE_ROW) {
                    int column_index = 0;
                    const char *stored_ip = has_ip_column
                        ? (const char *)sqlite3_column_text(stmt, column_index++)
                        : "";
                    const char *stored_ua = has_ua_column
                        ? (const char *)sqlite3_column_text(stmt, column_index)
                        : "";
                    update_ip = has_ip_column && tracking_value_differs(stored_ip, ip_address);
                    update_ua = has_ua_column && tracking_value_differs(stored_ua, user_agent);
                }
                sqlite3_finalize(stmt);
            } else {
                log_warn("Failed to prepare session client-context lookup for session %lld: %s",
                         (long long)session_id, sqlite3_errmsg(db));
            }
        }
    }

    bool refresh_tracking = has_tracking_columns && should_refresh_session_tracking(now, last_activity_at, idle_expires_at);
    if (refresh_tracking || update_ip || update_ua) {
        time_t new_idle_expires_at = now + default_session_idle_expiry_seconds();
        if (new_idle_expires_at > expires_at) {
            new_idle_expires_at = expires_at;
        }

        char update_sql[256] = "UPDATE sessions SET ";
        size_t sql_len = strlen(update_sql);
        bool need_comma = false;
        if (refresh_tracking) {
            int written = snprintf(update_sql + sql_len, sizeof(update_sql) - sql_len,
                                   "%slast_activity_at = ?, idle_expires_at = ?",
                                   need_comma ? ", " : "");
            if (written < 0 || (size_t)written >= sizeof(update_sql) - sql_len) {
                log_warn("Failed to build session refresh SQL for session %lld", (long long)session_id);
                return 0;
            }
            sql_len += (size_t)written;
            need_comma = true;
        }
        if (update_ip) {
            int written = snprintf(update_sql + sql_len, sizeof(update_sql) - sql_len,
                                   "%sip_address = ?", need_comma ? ", " : "");
            if (written < 0 || (size_t)written >= sizeof(update_sql) - sql_len) {
                log_warn("Failed to build session IP refresh SQL for session %lld", (long long)session_id);
                return 0;
            }
            sql_len += (size_t)written;
            need_comma = true;
        }
        if (update_ua) {
            int written = snprintf(update_sql + sql_len, sizeof(update_sql) - sql_len,
                                   "%suser_agent = ?", need_comma ? ", " : "");
            if (written < 0 || (size_t)written >= sizeof(update_sql) - sql_len) {
                log_warn("Failed to build session user-agent refresh SQL for session %lld", (long long)session_id);
                return 0;
            }
            sql_len += (size_t)written;
        }
        int written = snprintf(update_sql + sql_len, sizeof(update_sql) - sql_len, " WHERE id = ?;");
        if (written < 0 || (size_t)written >= sizeof(update_sql) - sql_len) {
            log_warn("Failed to finalize session refresh SQL for session %lld", (long long)session_id);
            return 0;
        }

        rc = sqlite3_prepare_v2(db, update_sql, -1, &stmt, NULL);
        if (rc == SQLITE_OK) {
            int param = 1;
            if (refresh_tracking) {
                sqlite3_bind_int64(stmt, param++, now);
                sqlite3_bind_int64(stmt, param++, new_idle_expires_at);
            }
            if (update_ip) {
                sqlite3_bind_text(stmt, param++, ip_address ? ip_address : "", -1, SQLITE_STATIC);
            }
            if (update_ua) {
                sqlite3_bind_text(stmt, param++, user_agent ? user_agent : "", -1, SQLITE_STATIC);
            }
            sqlite3_bind_int64(stmt, param, session_id);
            if (sqlite3_step(stmt) != SQLITE_DONE) {
                log_warn("Failed to refresh tracking for session %lld: %s",
                         (long long)session_id, sqlite3_errmsg(db));
            }
            sqlite3_finalize(stmt);
        } else {
            log_warn("Failed to prepare tracking refresh for session %lld: %s",
                     (long long)session_id, sqlite3_errmsg(db));
        }
    }

    return 0;
}

int db_auth_validate_session(const char *token, int64_t *user_id) {
    return db_auth_validate_session_with_context(token, user_id, NULL, NULL);
}

/**
 * Delete a session
 */
int db_auth_delete_session(const char *token) {
    if (!token) {
        log_error("Token is required");
        return -1;
    }

    sqlite3 *db = get_db_handle();
    if (!db) {
        log_error("Database not initialized");
        return -1;
    }

    // Delete the session
    sqlite3_stmt *stmt;
    int rc = sqlite3_prepare_v2(db, "DELETE FROM sessions WHERE token = ?;", -1, &stmt, NULL);
    if (rc != SQLITE_OK) {
        log_error("Failed to prepare statement: %s", sqlite3_errmsg(db));
        return -1;
    }

    sqlite3_bind_text(stmt, 1, token, -1, SQLITE_STATIC);

    rc = sqlite3_step(stmt);
    if (rc != SQLITE_DONE) {
        log_error("Failed to delete session: %s", sqlite3_errmsg(db));
        sqlite3_finalize(stmt);
        return -1;
    }

    sqlite3_finalize(stmt);

    log_info("Session deleted successfully");
    return 0;
}

/**
 * Delete all sessions for a user
 */
int db_auth_delete_user_sessions(int64_t user_id) {
    sqlite3 *db = get_db_handle();
    if (!db) {
        log_error("Database not initialized");
        return -1;
    }

    // Delete the sessions
    sqlite3_stmt *stmt;
    int rc = sqlite3_prepare_v2(db, "DELETE FROM sessions WHERE user_id = ?;", -1, &stmt, NULL);
    if (rc != SQLITE_OK) {
        log_error("Failed to prepare statement: %s", sqlite3_errmsg(db));
        return -1;
    }

    sqlite3_bind_int64(stmt, 1, user_id);

    rc = sqlite3_step(stmt);
    if (rc != SQLITE_DONE) {
        log_error("Failed to delete sessions: %s", sqlite3_errmsg(db));
        sqlite3_finalize(stmt);
        return -1;
    }

    sqlite3_finalize(stmt);

    log_info("Sessions deleted successfully for user: %lld", (long long)user_id);
    return 0;
}

/**
 * Clean up expired sessions
 */
int db_auth_cleanup_sessions(void) {
    sqlite3 *db = get_db_handle();
    if (!db) {
        log_error("Database not initialized");
        return -1;
    }

    // Delete expired or idle-expired sessions
    sqlite3_stmt *stmt;
    const char *sql = cached_column_exists("sessions", "idle_expires_at")
        ? "DELETE FROM sessions WHERE expires_at < ? OR idle_expires_at < ?;"
        : "DELETE FROM sessions WHERE expires_at < ?;";
    int rc = sqlite3_prepare_v2(db, sql, -1, &stmt, NULL);
    if (rc != SQLITE_OK) {
        log_error("Failed to prepare statement: %s", sqlite3_errmsg(db));
        return -1;
    }

    time_t now = time(NULL);
    sqlite3_bind_int64(stmt, 1, now);
    if (cached_column_exists("sessions", "idle_expires_at")) {
        sqlite3_bind_int64(stmt, 2, now);
    }

    rc = sqlite3_step(stmt);
    if (rc != SQLITE_DONE) {
        log_error("Failed to delete expired sessions: %s", sqlite3_errmsg(db));
        sqlite3_finalize(stmt);
        return -1;
    }

    int deleted = sqlite3_changes(db);
    sqlite3_finalize(stmt);

    log_info("Cleaned up %d expired sessions", deleted);

    rc = sqlite3_prepare_v2(db, "DELETE FROM trusted_devices WHERE expires_at < ?;", -1, &stmt, NULL);
    if (rc == SQLITE_OK) {
        sqlite3_bind_int64(stmt, 1, now);
        if (sqlite3_step(stmt) != SQLITE_DONE) {
            log_warn("Failed to clean up expired trusted devices: %s", sqlite3_errmsg(db));
        }
        sqlite3_finalize(stmt);
    }

    return deleted;
}

int db_auth_list_user_sessions(int64_t user_id, session_t *sessions, int max_count) {
    if (!sessions || max_count <= 0) {
        return 0;
    }

    sqlite3 *db = get_db_handle();
    if (!db) {
        log_error("Database not initialized");
        return -1;
    }

    bool has_idle_expires_column = cached_column_exists("sessions", "idle_expires_at");
    bool has_last_activity_column = cached_column_exists("sessions", "last_activity_at");
    time_t now = time(NULL);

    const char *sql = has_idle_expires_column && has_last_activity_column
        ? "SELECT id, user_id, token, created_at, "
          "COALESCE(last_activity_at, created_at), "
          "COALESCE(idle_expires_at, expires_at), "
          "expires_at, COALESCE(ip_address, ''), COALESCE(user_agent, '') "
          "FROM sessions WHERE user_id = ? "
          "AND expires_at >= ? "
          "AND (idle_expires_at IS NULL OR idle_expires_at >= ?) "
          "ORDER BY COALESCE(last_activity_at, created_at) DESC LIMIT ?;"
        : has_idle_expires_column
        ? "SELECT id, user_id, token, created_at, "
          "created_at, COALESCE(idle_expires_at, expires_at), "
          "expires_at, COALESCE(ip_address, ''), COALESCE(user_agent, '') "
          "FROM sessions WHERE user_id = ? "
          "AND expires_at >= ? "
          "AND (idle_expires_at IS NULL OR idle_expires_at >= ?) "
          "ORDER BY created_at DESC LIMIT ?;"
        : "SELECT id, user_id, token, created_at, "
          "created_at, expires_at, expires_at, COALESCE(ip_address, ''), COALESCE(user_agent, '') "
          "FROM sessions WHERE user_id = ? "
          "AND expires_at >= ? "
          "ORDER BY created_at DESC LIMIT ?;";

    sqlite3_stmt *stmt;
    int rc = sqlite3_prepare_v2(db, sql, -1, &stmt, NULL);
    if (rc != SQLITE_OK) {
        log_error("Failed to prepare statement: %s", sqlite3_errmsg(db));
        return -1;
    }

    int param = 1;
    sqlite3_bind_int64(stmt, param++, user_id);
    sqlite3_bind_int64(stmt, param++, now);
    if (has_idle_expires_column) {
        sqlite3_bind_int64(stmt, param++, now);
    }
    sqlite3_bind_int(stmt, param, max_count);

    int count = 0;
    while (sqlite3_step(stmt) == SQLITE_ROW && count < max_count) {
        session_t *session = &sessions[count++];
        memset(session, 0, sizeof(*session));
        session->id = sqlite3_column_int64(stmt, 0);
        session->user_id = sqlite3_column_int64(stmt, 1);
        const char *token = (const char *)sqlite3_column_text(stmt, 2);
        const char *ip = (const char *)sqlite3_column_text(stmt, 7);
        const char *ua = (const char *)sqlite3_column_text(stmt, 8);
        if (token) strncpy(session->token, token, sizeof(session->token) - 1);
        if (ip) strncpy(session->ip_address, ip, sizeof(session->ip_address) - 1);
        if (ua) strncpy(session->user_agent, ua, sizeof(session->user_agent) - 1);
        session->created_at = sqlite3_column_int64(stmt, 3);
        session->last_activity_at = sqlite3_column_int64(stmt, 4);
        session->idle_expires_at = sqlite3_column_int64(stmt, 5);
        session->expires_at = sqlite3_column_int64(stmt, 6);
    }

    sqlite3_finalize(stmt);
    return count;
}

int db_auth_delete_session_by_id(int64_t user_id, int64_t session_id) {
    sqlite3 *db = get_db_handle();
    if (!db) {
        log_error("Database not initialized");
        return -1;
    }

    sqlite3_stmt *stmt;
    int rc = sqlite3_prepare_v2(db,
                               "DELETE FROM sessions WHERE id = ? AND user_id = ?;",
                               -1, &stmt, NULL);
    if (rc != SQLITE_OK) {
        log_error("Failed to prepare statement: %s", sqlite3_errmsg(db));
        return -1;
    }

    sqlite3_bind_int64(stmt, 1, session_id);
    sqlite3_bind_int64(stmt, 2, user_id);
    rc = sqlite3_step(stmt);
    int changes = (rc == SQLITE_DONE) ? sqlite3_changes(db) : 0;
    sqlite3_finalize(stmt);
    return (rc == SQLITE_DONE && changes > 0) ? 0 : -1;
}

int db_auth_create_trusted_device(int64_t user_id, const char *ip_address, const char *user_agent,
                                  int expiry_seconds, char *token, size_t token_size) {
    if (!token || token_size < 33) {
        log_error("Token buffer is too small");
        return -1;
    }

    sqlite3 *db = get_db_handle();
    if (!db) {
        log_error("Database not initialized");
        return -1;
    }

    if (generate_random_string(token, 32) != 0) {
        return -1;
    }

    char token_hash[SHA256_DIGEST_LENGTH * 2 + 1];
    if (hash_token_identifier(token, token_hash, sizeof(token_hash)) != 0) {
        return -1;
    }

    time_t now = time(NULL);
    int lifetime = expiry_seconds > 0 ? expiry_seconds : default_trusted_device_expiry_seconds();

    sqlite3_stmt *stmt;
    int rc = sqlite3_prepare_v2(db,
                               "INSERT INTO trusted_devices (user_id, token, ip_address, user_agent, created_at, last_used_at, expires_at) "
                               "VALUES (?, ?, ?, ?, ?, ?, ?);",
                               -1, &stmt, NULL);
    if (rc != SQLITE_OK) {
        log_error("Failed to prepare statement: %s", sqlite3_errmsg(db));
        return -1;
    }

    sqlite3_bind_int64(stmt, 1, user_id);
    sqlite3_bind_text(stmt, 2, token_hash, -1, SQLITE_TRANSIENT);
    sqlite3_bind_text(stmt, 3, ip_address ? ip_address : "", -1, SQLITE_STATIC);
    sqlite3_bind_text(stmt, 4, user_agent ? user_agent : "", -1, SQLITE_STATIC);
    sqlite3_bind_int64(stmt, 5, now);
    sqlite3_bind_int64(stmt, 6, now);
    sqlite3_bind_int64(stmt, 7, now + lifetime);

    rc = sqlite3_step(stmt);
    sqlite3_finalize(stmt);
    return rc == SQLITE_DONE ? 0 : -1;
}

static int lookup_trusted_device(sqlite3 *db, int64_t user_id, const char *token,
                                 int64_t *trusted_id, time_t *expires_at) {
    if (!db || !token || token[0] == '\0') {
        return -1;
    }

    char token_hash[SHA256_DIGEST_LENGTH * 2 + 1];
    if (hash_token_identifier(token, token_hash, sizeof(token_hash)) != 0) {
        return -1;
    }

    sqlite3_stmt *stmt;
    int rc = sqlite3_prepare_v2(db,
                               "SELECT id, expires_at FROM trusted_devices WHERE user_id = ? AND token = ?;",
                               -1, &stmt, NULL);
    if (rc != SQLITE_OK) {
        log_error("Failed to prepare statement: %s", sqlite3_errmsg(db));
        return -1;
    }

    sqlite3_bind_int64(stmt, 1, user_id);
    sqlite3_bind_text(stmt, 2, token_hash, -1, SQLITE_TRANSIENT);

    if (sqlite3_step(stmt) != SQLITE_ROW) {
        sqlite3_finalize(stmt);
        return -1;
    }

    if (trusted_id) {
        *trusted_id = sqlite3_column_int64(stmt, 0);
    }
    if (expires_at) {
        *expires_at = sqlite3_column_int64(stmt, 1);
    }
    sqlite3_finalize(stmt);
    return 0;
}

int db_auth_get_trusted_device_id(int64_t user_id, const char *token, int64_t *trusted_device_id) {
    sqlite3 *db = get_db_handle();
    if (!db) {
        log_error("Database not initialized");
        return -1;
    }

    int64_t resolved_id = 0;
    time_t expires_at = 0;
    if (lookup_trusted_device(db, user_id, token, &resolved_id, &expires_at) != 0) {
        return -1;
    }

    if (time(NULL) > expires_at) {
        db_auth_delete_trusted_device_by_id(user_id, resolved_id);
        return -1;
    }

    if (trusted_device_id) {
        *trusted_device_id = resolved_id;
    }

    return 0;
}

int db_auth_validate_trusted_device(int64_t user_id, const char *token) {
    if (!token || token[0] == '\0') {
        return -1;
    }

    sqlite3 *db = get_db_handle();
    if (!db) {
        log_error("Database not initialized");
        return -1;
    }

    int64_t trusted_id = 0;
    if (db_auth_get_trusted_device_id(user_id, token, &trusted_id) != 0) {
        return -1;
    }

    sqlite3_stmt *stmt;
    int rc = sqlite3_prepare_v2(db,
                           "UPDATE trusted_devices SET last_used_at = ? WHERE id = ?;",
                           -1, &stmt, NULL);
    if (rc == SQLITE_OK) {
        sqlite3_bind_int64(stmt, 1, time(NULL));
        sqlite3_bind_int64(stmt, 2, trusted_id);
        sqlite3_step(stmt);
        sqlite3_finalize(stmt);
    }

    return 0;
}

int db_auth_list_trusted_devices(int64_t user_id, trusted_device_t *devices, int max_count) {
    if (!devices || max_count <= 0) {
        return 0;
    }

    sqlite3 *db = get_db_handle();
    if (!db) {
        log_error("Database not initialized");
        return -1;
    }

    time_t now = time(NULL);

    sqlite3_stmt *stmt;
    int rc = sqlite3_prepare_v2(db,
                               "SELECT id, user_id, created_at, COALESCE(last_used_at, created_at), expires_at, "
                               "COALESCE(ip_address, ''), COALESCE(user_agent, '') "
                               "FROM trusted_devices WHERE user_id = ? AND expires_at >= ? "
                               "ORDER BY COALESCE(last_used_at, created_at) DESC LIMIT ?;",
                               -1, &stmt, NULL);
    if (rc != SQLITE_OK) {
        log_error("Failed to prepare statement: %s", sqlite3_errmsg(db));
        return -1;
    }

    sqlite3_bind_int64(stmt, 1, user_id);
    sqlite3_bind_int64(stmt, 2, now);
    sqlite3_bind_int(stmt, 3, max_count);

    int count = 0;
    while (sqlite3_step(stmt) == SQLITE_ROW && count < max_count) {
        trusted_device_t *device = &devices[count++];
        memset(device, 0, sizeof(*device));
        device->id = sqlite3_column_int64(stmt, 0);
        device->user_id = sqlite3_column_int64(stmt, 1);
        const char *ip = (const char *)sqlite3_column_text(stmt, 5);
        const char *ua = (const char *)sqlite3_column_text(stmt, 6);
        if (ip) strncpy(device->ip_address, ip, sizeof(device->ip_address) - 1);
        if (ua) strncpy(device->user_agent, ua, sizeof(device->user_agent) - 1);
        device->created_at = sqlite3_column_int64(stmt, 2);
        device->last_used_at = sqlite3_column_int64(stmt, 3);
        device->expires_at = sqlite3_column_int64(stmt, 4);
    }

    sqlite3_finalize(stmt);
    return count;
}

int db_auth_delete_trusted_device_by_id(int64_t user_id, int64_t trusted_device_id) {
    sqlite3 *db = get_db_handle();
    if (!db) {
        log_error("Database not initialized");
        return -1;
    }

    sqlite3_stmt *stmt;
    int rc = sqlite3_prepare_v2(db,
                               "DELETE FROM trusted_devices WHERE id = ? AND user_id = ?;",
                               -1, &stmt, NULL);
    if (rc != SQLITE_OK) {
        log_error("Failed to prepare statement: %s", sqlite3_errmsg(db));
        return -1;
    }

    sqlite3_bind_int64(stmt, 1, trusted_device_id);
    sqlite3_bind_int64(stmt, 2, user_id);
    rc = sqlite3_step(stmt);
    int changes = (rc == SQLITE_DONE) ? sqlite3_changes(db) : 0;
    sqlite3_finalize(stmt);
    return (rc == SQLITE_DONE && changes > 0) ? 0 : -1;
}

/**
 * Get the role name for a role ID
 */
const char *db_auth_get_role_name(user_role_t role) {
    if (role < 0 || role >= sizeof(role_names) / sizeof(role_names[0])) {
        return "unknown";
    }

    return role_names[role];
}

/**
 * Get the role ID for a role name
 */
int db_auth_get_role_id(const char *role_name) {
    if (!role_name) {
        return -1;
    }

    for (size_t i = 0; i < sizeof(role_names) / sizeof(role_names[0]); i++) {
        if (strcmp(role_names[i], role_name) == 0) {
            return (int)i;
        }
    }

    return -1;
}


/* ========== TOTP MFA Database Functions ========== */

/**
 * Get TOTP info for a user
 */
int db_auth_get_totp_info(int64_t user_id, char *secret, size_t secret_size, bool *enabled) {
    if (!secret || !enabled || secret_size == 0) {
        log_error("Invalid parameters for db_auth_get_totp_info");
        return -1;
    }

    secret[0] = '\0';
    *enabled = false;

    sqlite3 *db = get_db_handle();
    if (!db) {
        log_error("Database not initialized");
        return -1;
    }

    /* Check if TOTP columns exist */
    if (!cached_column_exists("users", "totp_secret") ||
        !cached_column_exists("users", "totp_enabled")) {
        log_debug("TOTP columns not yet available in users table");
        return -1;
    }

    sqlite3_stmt *stmt;
    int rc = sqlite3_prepare_v2(db,
        "SELECT totp_secret, totp_enabled FROM users WHERE id = ?;",
        -1, &stmt, NULL);
    if (rc != SQLITE_OK) {
        log_error("Failed to prepare TOTP query: %s", sqlite3_errmsg(db));
        return -1;
    }

    sqlite3_bind_int64(stmt, 1, user_id);

    if (sqlite3_step(stmt) != SQLITE_ROW) {
        sqlite3_finalize(stmt);
        return -1;
    }

    const char *db_secret = (const char *)sqlite3_column_text(stmt, 0);
    if (db_secret && db_secret[0] != '\0') {
        strncpy(secret, db_secret, secret_size - 1);
        secret[secret_size - 1] = '\0';
    }

    *enabled = sqlite3_column_int(stmt, 1) != 0;
    sqlite3_finalize(stmt);

    /* Return -1 if no secret is configured */
    if (secret[0] == '\0') return -1;
    return 0;
}

/**
 * Set or clear the TOTP secret for a user
 */
int db_auth_set_totp_secret(int64_t user_id, const char *secret) {
    sqlite3 *db = get_db_handle();
    if (!db) {
        log_error("Database not initialized");
        return -1;
    }

    if (!cached_column_exists("users", "totp_secret")) {
        log_error("totp_secret column not available");
        return -1;
    }

    sqlite3_stmt *stmt;
    int rc = sqlite3_prepare_v2(db,
        "UPDATE users SET totp_secret = ?, updated_at = ? WHERE id = ?;",
        -1, &stmt, NULL);
    if (rc != SQLITE_OK) {
        log_error("Failed to prepare TOTP update: %s", sqlite3_errmsg(db));
        return -1;
    }

    if (secret) {
        sqlite3_bind_text(stmt, 1, secret, -1, SQLITE_STATIC);
    } else {
        sqlite3_bind_null(stmt, 1);
    }
    sqlite3_bind_int64(stmt, 2, (int64_t)time(NULL));
    sqlite3_bind_int64(stmt, 3, user_id);

    rc = sqlite3_step(stmt);
    sqlite3_finalize(stmt);

    if (rc != SQLITE_DONE) {
        log_error("Failed to update TOTP secret: %s", sqlite3_errmsg(db));
        return -1;
    }

    return 0;
}

/**
 * Enable or disable TOTP for a user
 */
int db_auth_enable_totp(int64_t user_id, bool enabled) {
    sqlite3 *db = get_db_handle();
    if (!db) {
        log_error("Database not initialized");
        return -1;
    }

    if (!cached_column_exists("users", "totp_enabled")) {
        log_error("totp_enabled column not available");
        return -1;
    }

    sqlite3_stmt *stmt;
    int rc = sqlite3_prepare_v2(db,
        "UPDATE users SET totp_enabled = ?, updated_at = ? WHERE id = ?;",
        -1, &stmt, NULL);
    if (rc != SQLITE_OK) {
        log_error("Failed to prepare TOTP enable update: %s", sqlite3_errmsg(db));
        return -1;
    }

    sqlite3_bind_int(stmt, 1, enabled ? 1 : 0);
    sqlite3_bind_int64(stmt, 2, (int64_t)time(NULL));
    sqlite3_bind_int64(stmt, 3, user_id);

    rc = sqlite3_step(stmt);
    sqlite3_finalize(stmt);

    if (rc != SQLITE_DONE) {
        log_error("Failed to update TOTP enabled: %s", sqlite3_errmsg(db));
        return -1;
    }

    log_info("TOTP %s for user %lld", enabled ? "enabled" : "disabled", (long long)user_id);
    return 0;
}

/**
 * Set the allowed_tags restriction for a user.
 * Pass NULL to remove any tag restriction (user can see all streams).
 * Pass an empty string to restrict to streams with NO tags (edge-case, generally use NULL for unrestricted).
 */
int db_auth_set_allowed_tags(int64_t user_id, const char *allowed_tags) {
    sqlite3 *db = get_db_handle();
    if (!db) {
        log_error("Database not initialized");
        return -1;
    }

    if (!cached_column_exists("users", "allowed_tags")) {
        log_error("allowed_tags column not available");
        return -1;
    }

    sqlite3_stmt *stmt;
    int rc = sqlite3_prepare_v2(db,
        "UPDATE users SET allowed_tags = ?, updated_at = ? WHERE id = ?;",
        -1, &stmt, NULL);
    if (rc != SQLITE_OK) {
        log_error("Failed to prepare allowed_tags update: %s", sqlite3_errmsg(db));
        return -1;
    }

    if (allowed_tags) {
        sqlite3_bind_text(stmt, 1, allowed_tags, -1, SQLITE_STATIC);
    } else {
        sqlite3_bind_null(stmt, 1);
    }
    sqlite3_bind_int64(stmt, 2, (int64_t)time(NULL));
    sqlite3_bind_int64(stmt, 3, user_id);

    rc = sqlite3_step(stmt);
    sqlite3_finalize(stmt);

    if (rc != SQLITE_DONE) {
        log_error("Failed to update allowed_tags: %s", sqlite3_errmsg(db));
        return -1;
    }

    log_info("allowed_tags updated for user %lld: %s", (long long)user_id,
             allowed_tags ? allowed_tags : "(unrestricted)");
    return 0;
}

int db_auth_validate_allowed_login_cidrs(const char *allowed_login_cidrs) {
    char normalized[USER_ALLOWED_LOGIN_CIDRS_MAX] = {0};
    bool has_entries = false;
    return normalize_allowed_login_cidrs(allowed_login_cidrs, normalized, sizeof(normalized), &has_entries);
}

int db_auth_set_allowed_login_cidrs(int64_t user_id, const char *allowed_login_cidrs) {
    sqlite3 *db = get_db_handle();
    if (!db) {
        log_error("Database not initialized");
        return -1;
    }

    if (!cached_column_exists("users", "allowed_login_cidrs")) {
        log_error("allowed_login_cidrs column not available");
        return -1;
    }

    char normalized[USER_ALLOWED_LOGIN_CIDRS_MAX] = {0};
    bool has_entries = false;
    if (normalize_allowed_login_cidrs(allowed_login_cidrs, normalized, sizeof(normalized), &has_entries) != 0) {
        log_error("Invalid allowed_login_cidrs value for user %lld", (long long)user_id);
        return -1;
    }

    sqlite3_stmt *stmt;
    int rc = sqlite3_prepare_v2(db,
        "UPDATE users SET allowed_login_cidrs = ?, updated_at = ? WHERE id = ?;",
        -1, &stmt, NULL);
    if (rc != SQLITE_OK) {
        log_error("Failed to prepare allowed_login_cidrs update: %s", sqlite3_errmsg(db));
        return -1;
    }

    if (has_entries) {
        sqlite3_bind_text(stmt, 1, normalized, -1, SQLITE_TRANSIENT);
    } else {
        sqlite3_bind_null(stmt, 1);
    }
    sqlite3_bind_int64(stmt, 2, (int64_t)time(NULL));
    sqlite3_bind_int64(stmt, 3, user_id);

    rc = sqlite3_step(stmt);
    sqlite3_finalize(stmt);

    if (rc != SQLITE_DONE) {
        log_error("Failed to update allowed_login_cidrs: %s", sqlite3_errmsg(db));
        return -1;
    }

    log_info("allowed_login_cidrs updated for user %lld: %s", (long long)user_id,
             has_entries ? normalized : "(unrestricted)");
    return 0;
}

bool db_auth_ip_allowed_for_user(const user_t *user, const char *client_ip) {
    if (!user) {
        return false;
    }

    if (!user->has_login_cidr_restriction) {
        return true;
    }

    if (!client_ip || client_ip[0] == '\0') {
        return false;
    }

    char cidr_list[USER_ALLOWED_LOGIN_CIDRS_MAX] = {0};
    size_t cidr_len = strnlen(user->allowed_login_cidrs, sizeof(cidr_list) - 1);
    memcpy(cidr_list, user->allowed_login_cidrs, cidr_len);
    cidr_list[cidr_len] = '\0';

    char *saveptr = NULL;
    for (char *token = strtok_r(cidr_list, "\n", &saveptr);
         token != NULL;
         token = strtok_r(NULL, "\n", &saveptr)) {
        char *trimmed = trim_ascii_whitespace(token);
        if (!trimmed || trimmed[0] == '\0') {
            continue;
        }
        if (ip_matches_cidr(client_ip, trimmed)) {
            return true;
        }
    }

    return false;
}

/**
 * Check whether a stream's tags satisfy a user's allowed_tags restriction.
 *
 * Returns true when:
 *   - The user has no tag restriction (has_tag_restriction == false), OR
 *   - The stream's tag list contains at least one tag that appears in the user's allowed_tags list
 */
bool db_auth_stream_allowed_for_user(const user_t *user, const char *stream_tags) {
    if (!user) return false;

    // No restriction: allow all streams
    if (!user->has_tag_restriction) return true;

    // Stream has no tags: deny access (user requires at least one matching tag)
    if (!stream_tags || stream_tags[0] == '\0') return false;

    // Tokenize stream_tags and check each against user's allowed_tags
    char stream_copy[256];
    strncpy(stream_copy, stream_tags, sizeof(stream_copy) - 1);
    stream_copy[sizeof(stream_copy) - 1] = '\0';

    char *saveptr = NULL;
    char *token = strtok_r(stream_copy, ",", &saveptr);
    while (token) {
        // Trim leading/trailing whitespace from token
        while (*token == ' ') token++;
        char *end = token + strlen(token) - 1;
        while (end > token && *end == ' ') { *end = '\0'; end--; }

        // Check if this stream tag appears in allowed_tags
        const char *p = user->allowed_tags;
        size_t tlen = strlen(token);
        while (*p) {
            // Skip leading spaces in allowed list
            while (*p == ' ') p++;
            // Compare tag
            if (strncmp(p, token, tlen) == 0) {
                char next = p[tlen];
                if (next == ',' || next == '\0' || next == ' ') {
                    return true;
                }
            }
            // Advance to next comma
            while (*p && *p != ',') p++;
            if (*p == ',') p++;
        }

        token = strtok_r(NULL, ",", &saveptr);
    }

    return false;
}