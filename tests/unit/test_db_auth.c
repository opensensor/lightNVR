/**
 * @file test_db_auth.c
 * @brief Layer 2 — user authentication and session management via SQLite
 *
 * Tests db_auth_init, create/get user, authenticate, change_password,
 * create/validate/delete session, cleanup_sessions, role helpers,
 * generate_api_key, and TOTP set/get/enable round-trip.
 */

#define _POSIX_C_SOURCE 200809L
#define _GNU_SOURCE

#include <stdio.h>
#include <string.h>
#include <time.h>
#include <unistd.h>
#include <sqlite3.h>

#include "unity.h"
#include "database/db_core.h"
#include "database/db_auth.h"

#define TEST_DB_PATH "/tmp/lightnvr_unit_auth_test.db"

static void clear_users(void) {
    sqlite3 *db = get_db_handle();
    /* Remove non-admin users to keep db_auth_init's default admin */
    sqlite3_exec(db, "DELETE FROM users WHERE username != 'admin';", NULL, NULL, NULL);
    sqlite3_exec(db, "DELETE FROM sessions;", NULL, NULL, NULL);
    sqlite3_exec(db, "DELETE FROM trusted_devices;", NULL, NULL, NULL);
}

void setUp(void)    { clear_users(); }
void tearDown(void) {}

/* db_auth_init creates default admin */
void test_auth_init_creates_admin(void) {
    int rc = db_auth_init();
    TEST_ASSERT_EQUAL_INT(0, rc);

    user_t user;
    rc = db_auth_get_user_by_username("admin", &user);
    TEST_ASSERT_EQUAL_INT(0, rc);
    TEST_ASSERT_EQUAL_INT(USER_ROLE_ADMIN, user.role);
}

/* create_user and get_user_by_username round-trip */
void test_create_and_get_user(void) {
    int64_t uid = 0;
    int rc = db_auth_create_user("testuser", "password123", "test@example.com",
                                 USER_ROLE_USER, true, &uid);
    TEST_ASSERT_EQUAL_INT(0, rc);
    TEST_ASSERT_GREATER_THAN(0, uid);

    user_t user;
    rc = db_auth_get_user_by_username("testuser", &user);
    TEST_ASSERT_EQUAL_INT(0, rc);
    TEST_ASSERT_EQUAL_STRING("testuser", user.username);
    TEST_ASSERT_EQUAL_INT(USER_ROLE_USER, user.role);
    TEST_ASSERT_TRUE(user.is_active);
}

/* db_auth_authenticate success */
void test_authenticate_success(void) {
    db_auth_create_user("authuser", "secret", NULL, USER_ROLE_USER, true, NULL);
    int64_t uid = 0;
    int rc = db_auth_authenticate("authuser", "secret", &uid);
    TEST_ASSERT_EQUAL_INT(0, rc);
    TEST_ASSERT_GREATER_THAN(0, uid);
}

/* db_auth_authenticate failure */
void test_authenticate_wrong_password(void) {
    db_auth_create_user("authuser2", "correct", NULL, USER_ROLE_USER, true, NULL);
    int rc = db_auth_authenticate("authuser2", "wrong", NULL);
    TEST_ASSERT_NOT_EQUAL(0, rc);
}

/* db_auth_change_password */
void test_change_password(void) {
    int64_t uid = 0;
    db_auth_create_user("chgpwuser", "oldpass", NULL, USER_ROLE_USER, true, &uid);
    int rc = db_auth_change_password(uid, "newpass");
    TEST_ASSERT_EQUAL_INT(0, rc);

    /* New password works */
    rc = db_auth_authenticate("chgpwuser", "newpass", NULL);
    TEST_ASSERT_EQUAL_INT(0, rc);

    /* Old password fails */
    rc = db_auth_authenticate("chgpwuser", "oldpass", NULL);
    TEST_ASSERT_NOT_EQUAL(0, rc);
}

/* create_session and validate_session */
void test_create_and_validate_session(void) {
    int64_t uid = 0;
    db_auth_create_user("sessuser", "pass", NULL, USER_ROLE_USER, true, &uid);

    char token[128];
    int rc = db_auth_create_session(uid, "127.0.0.1", "TestAgent", 3600,
                                    token, sizeof(token));
    TEST_ASSERT_EQUAL_INT(0, rc);
    TEST_ASSERT_GREATER_THAN(0, strlen(token));

    int64_t out_uid = 0;
    rc = db_auth_validate_session(token, &out_uid);
    TEST_ASSERT_EQUAL_INT(0, rc);
    TEST_ASSERT_EQUAL_INT(uid, out_uid);
}

/* delete_session invalidates */
void test_delete_session(void) {
    int64_t uid = 0;
    db_auth_create_user("deluser", "pass", NULL, USER_ROLE_USER, true, &uid);

    char token[128];
    db_auth_create_session(uid, NULL, NULL, 3600, token, sizeof(token));

    int rc = db_auth_delete_session(token);
    TEST_ASSERT_EQUAL_INT(0, rc);

    rc = db_auth_validate_session(token, NULL);
    TEST_ASSERT_NOT_EQUAL(0, rc);
}

void test_list_sessions_and_trusted_devices(void) {
    int64_t uid = 0;
    db_auth_create_user("deviceuser", "pass", NULL, USER_ROLE_USER, true, &uid);

    char session_token[128];
    int rc = db_auth_create_session(uid, "127.0.0.1", "TestAgent", 3600,
                                    session_token, sizeof(session_token));
    TEST_ASSERT_EQUAL_INT(0, rc);

    session_t sessions[8];
    int session_count = db_auth_list_user_sessions(uid, sessions, 8);
    TEST_ASSERT_GREATER_THAN_INT(0, session_count);
    TEST_ASSERT_EQUAL_INT(uid, sessions[0].user_id);
    TEST_ASSERT_TRUE(sessions[0].last_activity_at >= sessions[0].created_at);
    TEST_ASSERT_TRUE(sessions[0].idle_expires_at <= sessions[0].expires_at);

    char trusted_token[128];
    rc = db_auth_create_trusted_device(uid, "127.0.0.1", "TestAgent", 86400,
                                       trusted_token, sizeof(trusted_token));
    TEST_ASSERT_EQUAL_INT(0, rc);
    TEST_ASSERT_EQUAL_INT(0, db_auth_validate_trusted_device(uid, trusted_token));

    trusted_device_t devices[8];
    int device_count = db_auth_list_trusted_devices(uid, devices, 8);
    TEST_ASSERT_GREATER_THAN_INT(0, device_count);
    TEST_ASSERT_EQUAL_INT(uid, devices[0].user_id);

    sqlite3 *db = get_db_handle();
    sqlite3_stmt *stmt = NULL;
    rc = sqlite3_prepare_v2(db, "SELECT token FROM trusted_devices WHERE id = ?;", -1, &stmt, NULL);
    TEST_ASSERT_EQUAL_INT(SQLITE_OK, rc);
    sqlite3_bind_int64(stmt, 1, devices[0].id);
    TEST_ASSERT_EQUAL_INT(SQLITE_ROW, sqlite3_step(stmt));
    const char *stored_token = (const char *)sqlite3_column_text(stmt, 0);
    TEST_ASSERT_NOT_NULL(stored_token);
    TEST_ASSERT_NOT_EQUAL(0, strcmp(stored_token, trusted_token));
    sqlite3_finalize(stmt);

    rc = db_auth_delete_trusted_device_by_id(uid, devices[0].id);
    TEST_ASSERT_EQUAL_INT(0, rc);
    TEST_ASSERT_NOT_EQUAL(0, db_auth_validate_trusted_device(uid, trusted_token));
}

/* role name / id conversions */
void test_role_name_conversions(void) {
    TEST_ASSERT_EQUAL_STRING("admin",  db_auth_get_role_name(USER_ROLE_ADMIN));
    TEST_ASSERT_EQUAL_STRING("user",   db_auth_get_role_name(USER_ROLE_USER));
    TEST_ASSERT_EQUAL_STRING("viewer", db_auth_get_role_name(USER_ROLE_VIEWER));

    TEST_ASSERT_EQUAL_INT(USER_ROLE_ADMIN,  db_auth_get_role_id("admin"));
    TEST_ASSERT_EQUAL_INT(USER_ROLE_USER,   db_auth_get_role_id("user"));
    TEST_ASSERT_EQUAL_INT(-1,               db_auth_get_role_id("unknown_role"));
}

/* generate_api_key and get_user_by_api_key */
void test_generate_and_use_api_key(void) {
    int64_t uid = 0;
    db_auth_create_user("apiuser", "pass", NULL, USER_ROLE_API, true, &uid);

    char api_key[64];
    int rc = db_auth_generate_api_key(uid, api_key, sizeof(api_key));
    TEST_ASSERT_EQUAL_INT(0, rc);
    TEST_ASSERT_GREATER_THAN(0, strlen(api_key));

    user_t found;
    rc = db_auth_get_user_by_api_key(api_key, &found);
    TEST_ASSERT_EQUAL_INT(0, rc);
    TEST_ASSERT_EQUAL_STRING("apiuser", found.username);
}

/* TOTP set/get/enable round-trip */
void test_totp_set_get_enable(void) {
    int64_t uid = 0;
    db_auth_create_user("totpuser", "pass", NULL, USER_ROLE_USER, true, &uid);

    int rc = db_auth_set_totp_secret(uid, "JBSWY3DPEHPK3PXP");
    TEST_ASSERT_EQUAL_INT(0, rc);

    rc = db_auth_enable_totp(uid, true);
    TEST_ASSERT_EQUAL_INT(0, rc);

    char secret[64];
    bool enabled = false;
    rc = db_auth_get_totp_info(uid, secret, sizeof(secret), &enabled);
    TEST_ASSERT_EQUAL_INT(0, rc);
    TEST_ASSERT_TRUE(enabled);
    TEST_ASSERT_EQUAL_STRING("JBSWY3DPEHPK3PXP", secret);
}

int main(void) {
    unlink(TEST_DB_PATH);
    if (init_database(TEST_DB_PATH) != 0) {
        fprintf(stderr, "FATAL: init_database failed\n");
        return 1;
    }
    db_auth_init();

    UNITY_BEGIN();
    RUN_TEST(test_auth_init_creates_admin);
    RUN_TEST(test_create_and_get_user);
    RUN_TEST(test_authenticate_success);
    RUN_TEST(test_authenticate_wrong_password);
    RUN_TEST(test_change_password);
    RUN_TEST(test_create_and_validate_session);
    RUN_TEST(test_delete_session);
    RUN_TEST(test_list_sessions_and_trusted_devices);
    RUN_TEST(test_role_name_conversions);
    RUN_TEST(test_generate_and_use_api_key);
    RUN_TEST(test_totp_set_get_enable);
    int result = UNITY_END();
    shutdown_database();
    unlink(TEST_DB_PATH);
    return result;
}

