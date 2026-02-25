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
#include <unistd.h>
#include <sys/time.h>

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

// Role names
static const char *role_names[] = {
    "admin",
    "user",
    "viewer",
    "api"
};

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
        char byte[3] = {hex[i], hex[i + 1], '\0'};
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
    int param_count = 1;

    if (email) {
        strcat(query, ", email = ?");
        param_count++;
    }

    if (role >= 0) {
        strcat(query, ", role = ?");
        param_count++;
    }

    if (is_active >= 0) {
        strcat(query, ", is_active = ?");
        param_count++;
    }

    strcat(query, " WHERE id = ?;");

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

    // Check if TOTP and allowed_tags columns exist for backward compatibility
    bool has_totp = cached_column_exists("users", "totp_enabled");
    bool has_allowed_tags = cached_column_exists("users", "allowed_tags");

    // Query the user
    sqlite3_stmt *stmt;
    int rc;
    if (has_totp && has_allowed_tags) {
        rc = sqlite3_prepare_v2(db,
                                "SELECT id, username, email, role, api_key, created_at, "
                                "updated_at, last_login, is_active, password_change_locked, totp_enabled, allowed_tags "
                                "FROM users WHERE id = ?;",
                                -1, &stmt, NULL);
    } else if (has_totp) {
        rc = sqlite3_prepare_v2(db,
                                "SELECT id, username, email, role, api_key, created_at, "
                                "updated_at, last_login, is_active, password_change_locked, totp_enabled "
                                "FROM users WHERE id = ?;",
                                -1, &stmt, NULL);
    } else {
        rc = sqlite3_prepare_v2(db,
                                "SELECT id, username, email, role, api_key, created_at, "
                                "updated_at, last_login, is_active, password_change_locked "
                                "FROM users WHERE id = ?;",
                                -1, &stmt, NULL);
    }
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

    // Fill the user structure
    user->id = sqlite3_column_int64(stmt, 0);
    strncpy(user->username, (const char *)sqlite3_column_text(stmt, 1), sizeof(user->username) - 1);
    user->username[sizeof(user->username) - 1] = '\0';

    const char *email = (const char *)sqlite3_column_text(stmt, 2);
    if (email) {
        strncpy(user->email, email, sizeof(user->email) - 1);
        user->email[sizeof(user->email) - 1] = '\0';
    } else {
        user->email[0] = '\0';
    }

    user->role = (user_role_t)sqlite3_column_int(stmt, 3);

    const char *api_key = (const char *)sqlite3_column_text(stmt, 4);
    if (api_key) {
        strncpy(user->api_key, api_key, sizeof(user->api_key) - 1);
        user->api_key[sizeof(user->api_key) - 1] = '\0';
    } else {
        user->api_key[0] = '\0';
    }

    user->created_at = sqlite3_column_int64(stmt, 5);
    user->updated_at = sqlite3_column_int64(stmt, 6);
    user->last_login = sqlite3_column_int64(stmt, 7);
    user->is_active = sqlite3_column_int(stmt, 8) != 0;
    user->password_change_locked = sqlite3_column_int(stmt, 9) != 0;
    user->totp_enabled = has_totp ? (sqlite3_column_int(stmt, 10) != 0) : false;
    if (has_allowed_tags && has_totp) {
        const char *at = (const char *)sqlite3_column_text(stmt, 11);
        if (at && at[0] != '\0') {
            strncpy(user->allowed_tags, at, sizeof(user->allowed_tags) - 1);
            user->allowed_tags[sizeof(user->allowed_tags) - 1] = '\0';
            user->has_tag_restriction = true;
        } else {
            user->allowed_tags[0] = '\0';
            user->has_tag_restriction = (sqlite3_column_type(stmt, 11) != SQLITE_NULL && at && at[0] == '\0');
        }
    } else {
        user->allowed_tags[0] = '\0';
        user->has_tag_restriction = false;
    }

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

    // Check if TOTP and allowed_tags columns exist for backward compatibility
    bool has_totp = cached_column_exists("users", "totp_enabled");
    bool has_allowed_tags = cached_column_exists("users", "allowed_tags");

    // Query the user
    sqlite3_stmt *stmt;
    int rc;
    if (has_totp && has_allowed_tags) {
        rc = sqlite3_prepare_v2(db,
                                "SELECT id, username, email, role, api_key, created_at, "
                                "updated_at, last_login, is_active, password_change_locked, totp_enabled, allowed_tags "
                                "FROM users WHERE username = ?;",
                                -1, &stmt, NULL);
    } else if (has_totp) {
        rc = sqlite3_prepare_v2(db,
                                "SELECT id, username, email, role, api_key, created_at, "
                                "updated_at, last_login, is_active, password_change_locked, totp_enabled "
                                "FROM users WHERE username = ?;",
                                -1, &stmt, NULL);
    } else {
        rc = sqlite3_prepare_v2(db,
                                "SELECT id, username, email, role, api_key, created_at, "
                                "updated_at, last_login, is_active, password_change_locked "
                                "FROM users WHERE username = ?;",
                                -1, &stmt, NULL);
    }
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

    // Fill the user structure
    user->id = sqlite3_column_int64(stmt, 0);
    strncpy(user->username, (const char *)sqlite3_column_text(stmt, 1), sizeof(user->username) - 1);
    user->username[sizeof(user->username) - 1] = '\0';

    const char *email = (const char *)sqlite3_column_text(stmt, 2);
    if (email) {
        strncpy(user->email, email, sizeof(user->email) - 1);
        user->email[sizeof(user->email) - 1] = '\0';
    } else {
        user->email[0] = '\0';
    }

    user->role = (user_role_t)sqlite3_column_int(stmt, 3);

    const char *api_key = (const char *)sqlite3_column_text(stmt, 4);
    if (api_key) {
        strncpy(user->api_key, api_key, sizeof(user->api_key) - 1);
        user->api_key[sizeof(user->api_key) - 1] = '\0';
    } else {
        user->api_key[0] = '\0';
    }

    user->created_at = sqlite3_column_int64(stmt, 5);
    user->updated_at = sqlite3_column_int64(stmt, 6);
    user->last_login = sqlite3_column_int64(stmt, 7);
    user->is_active = sqlite3_column_int(stmt, 8) != 0;
    user->password_change_locked = sqlite3_column_int(stmt, 9) != 0;
    user->totp_enabled = has_totp ? (sqlite3_column_int(stmt, 10) != 0) : false;
    if (has_allowed_tags && has_totp) {
        const char *at = (const char *)sqlite3_column_text(stmt, 11);
        if (at && at[0] != '\0') {
            strncpy(user->allowed_tags, at, sizeof(user->allowed_tags) - 1);
            user->allowed_tags[sizeof(user->allowed_tags) - 1] = '\0';
            user->has_tag_restriction = true;
        } else {
            user->allowed_tags[0] = '\0';
            user->has_tag_restriction = false;
        }
    } else {
        user->allowed_tags[0] = '\0';
        user->has_tag_restriction = false;
    }

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

    // Check if TOTP and allowed_tags columns exist for backward compatibility
    bool has_totp = cached_column_exists("users", "totp_enabled");
    bool has_allowed_tags = cached_column_exists("users", "allowed_tags");

    // Query the user
    sqlite3_stmt *stmt;
    int rc;
    if (has_totp && has_allowed_tags) {
        rc = sqlite3_prepare_v2(db,
                                "SELECT id, username, email, role, api_key, created_at, "
                                "updated_at, last_login, is_active, password_change_locked, totp_enabled, allowed_tags "
                                "FROM users WHERE api_key = ?;",
                                -1, &stmt, NULL);
    } else if (has_totp) {
        rc = sqlite3_prepare_v2(db,
                                "SELECT id, username, email, role, api_key, created_at, "
                                "updated_at, last_login, is_active, password_change_locked, totp_enabled "
                                "FROM users WHERE api_key = ?;",
                                -1, &stmt, NULL);
    } else {
        rc = sqlite3_prepare_v2(db,
                                "SELECT id, username, email, role, api_key, created_at, "
                                "updated_at, last_login, is_active, password_change_locked "
                                "FROM users WHERE api_key = ?;",
                                -1, &stmt, NULL);
    }
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

    // Fill the user structure
    user->id = sqlite3_column_int64(stmt, 0);
    strncpy(user->username, (const char *)sqlite3_column_text(stmt, 1), sizeof(user->username) - 1);
    user->username[sizeof(user->username) - 1] = '\0';

    const char *email = (const char *)sqlite3_column_text(stmt, 2);
    if (email) {
        strncpy(user->email, email, sizeof(user->email) - 1);
        user->email[sizeof(user->email) - 1] = '\0';
    } else {
        user->email[0] = '\0';
    }

    user->role = (user_role_t)sqlite3_column_int(stmt, 3);

    const char *key = (const char *)sqlite3_column_text(stmt, 4);
    if (key) {
        strncpy(user->api_key, key, sizeof(user->api_key) - 1);
        user->api_key[sizeof(user->api_key) - 1] = '\0';
    } else {
        user->api_key[0] = '\0';
    }

    user->created_at = sqlite3_column_int64(stmt, 5);
    user->updated_at = sqlite3_column_int64(stmt, 6);
    user->last_login = sqlite3_column_int64(stmt, 7);
    user->is_active = sqlite3_column_int(stmt, 8) != 0;
    user->password_change_locked = sqlite3_column_int(stmt, 9) != 0;
    user->totp_enabled = has_totp ? (sqlite3_column_int(stmt, 10) != 0) : false;
    if (has_allowed_tags && has_totp) {
        const char *at = (const char *)sqlite3_column_text(stmt, 11);
        if (at && at[0] != '\0') {
            strncpy(user->allowed_tags, at, sizeof(user->allowed_tags) - 1);
            user->allowed_tags[sizeof(user->allowed_tags) - 1] = '\0';
            user->has_tag_restriction = true;
        } else {
            user->allowed_tags[0] = '\0';
            user->has_tag_restriction = false;
        }
    } else {
        user->allowed_tags[0] = '\0';
        user->has_tag_restriction = false;
    }

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

    // Calculate expiry time (use config value as default if not specified)
    int default_expiry = g_config.auth_timeout_hours > 0
                             ? g_config.auth_timeout_hours * 3600
                             : DEFAULT_SESSION_EXPIRY;
    time_t expires_at = now + (expiry_seconds > 0 ? expiry_seconds : default_expiry);

    // Some deployments may not yet have the optional ip_address/user_agent columns
    // (e.g., databases created before the 0018_add_session_tracking migration).
    // Use cached_column_exists to build a compatible INSERT that only references
    // columns that actually exist, so login still works on older schemas.
    bool has_ip_column = cached_column_exists("sessions", "ip_address");
    bool has_ua_column = cached_column_exists("sessions", "user_agent");

    const char *sql = NULL;
    if (has_ip_column && has_ua_column) {
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
int db_auth_validate_session(const char *token, int64_t *user_id) {
    if (!token) {
        log_error("Token is required");
        return -1;
    }

    sqlite3 *db = get_db_handle();
    if (!db) {
        log_error("Database not initialized");
        return -1;
    }

    // Query the session
    sqlite3_stmt *stmt;
    int rc = sqlite3_prepare_v2(db,
                               "SELECT s.id, s.user_id, s.expires_at, u.is_active "
                               "FROM sessions s "
                               "JOIN users u ON s.user_id = u.id "
                               "WHERE s.token = ?;",
                               -1, &stmt, NULL);
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

    // Check if the session has expired
    time_t expires_at = sqlite3_column_int64(stmt, 2);
    time_t now = time(NULL);

    if (now > expires_at) {
        log_debug("Session has expired");
        sqlite3_finalize(stmt);
        return -1;
    }

    // Check if the user is active
    int is_active = sqlite3_column_int(stmt, 3);
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

    return 0;
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

    // Delete expired sessions
    sqlite3_stmt *stmt;
    int rc = sqlite3_prepare_v2(db, "DELETE FROM sessions WHERE expires_at < ?;", -1, &stmt, NULL);
    if (rc != SQLITE_OK) {
        log_error("Failed to prepare statement: %s", sqlite3_errmsg(db));
        return -1;
    }

    time_t now = time(NULL);
    sqlite3_bind_int64(stmt, 1, now);

    rc = sqlite3_step(stmt);
    if (rc != SQLITE_DONE) {
        log_error("Failed to delete expired sessions: %s", sqlite3_errmsg(db));
        sqlite3_finalize(stmt);
        return -1;
    }

    int deleted = sqlite3_changes(db);
    sqlite3_finalize(stmt);

    log_info("Cleaned up %d expired sessions", deleted);
    return deleted;
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
            return i;
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