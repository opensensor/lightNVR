/**
 * @file test_db_camera_favorites.c
 * @brief Per-user camera favorites persistence tests.
 */

#define _POSIX_C_SOURCE 200809L

#include <stdio.h>
#include <string.h>
#include <unistd.h>

#include <sqlite3.h>

#include "unity.h"
#include "database/db_auth.h"
#include "database/db_camera_favorites.h"
#include "database/db_core.h"
#include "database/db_streams.h"
#include "utils/strings.h"
#include "utils/uuid.h"

#define TEST_DB_PATH "/tmp/lightnvr_unit_db_camera_favorites.db"

/* Favorites reference users(id), so the tests need real user rows. */
static int64_t alice_id = 0;
static int64_t bob_id = 0;

static stream_config_t create_camera(const char *name) {
    stream_config_t stream;
    memset(&stream, 0, sizeof(stream));
    safe_strcpy(stream.name, name, sizeof(stream.name), 0);
    snprintf(stream.url, sizeof(stream.url), "rtsp://camera/%s", name);
    safe_strcpy(stream.codec, "h264", sizeof(stream.codec), 0);
    stream.enabled = true;
    stream.streaming_enabled = true;
    stream.width = 1920;
    stream.height = 1080;
    stream.fps = 25;
    TEST_ASSERT_NOT_EQUAL(0, add_stream_config(&stream));
    TEST_ASSERT_EQUAL_INT(0, get_stream_config_by_name(stream.name, &stream));
    return stream;
}

static bool listed(const camera_favorite_t *favorites, int count,
                   const char *camera_uuid) {
    for (int index = 0; index < count; index++) {
        if (strcmp(favorites[index].camera_uuid, camera_uuid) == 0) return true;
    }
    return false;
}

void setUp(void) {
    sqlite3_exec(get_db_handle(), "DELETE FROM camera_favorites;", NULL, NULL, NULL);
    sqlite3_exec(get_db_handle(), "DELETE FROM streams;", NULL, NULL, NULL);
}

void tearDown(void) {}

void test_add_is_idempotent_and_scoped_per_user(void) {
    stream_config_t lobby = create_camera("Lobby");
    stream_config_t yard = create_camera("Yard");
    camera_favorite_t first, repeat;

    TEST_ASSERT_EQUAL_INT(DB_CAMERA_FAVORITE_OK,
                          db_camera_favorite_add(alice_id, lobby.camera_uuid, &first));
    TEST_ASSERT_EQUAL_STRING(lobby.camera_uuid, first.camera_uuid);
    TEST_ASSERT_GREATER_THAN_INT64(0, first.created_at);

    TEST_ASSERT_EQUAL_INT(DB_CAMERA_FAVORITE_OK,
                          db_camera_favorite_add(alice_id, lobby.camera_uuid, &repeat));
    TEST_ASSERT_EQUAL_INT64(first.created_at, repeat.created_at);
    TEST_ASSERT_EQUAL_INT(DB_CAMERA_FAVORITE_OK,
                          db_camera_favorite_add(alice_id, yard.camera_uuid, NULL));

    camera_favorite_t favorites[8];
    int count = db_camera_favorites_list(alice_id, favorites, 8);
    TEST_ASSERT_EQUAL_INT(2, count);
    TEST_ASSERT_TRUE(listed(favorites, count, lobby.camera_uuid));
    TEST_ASSERT_TRUE(listed(favorites, count, yard.camera_uuid));

    /* Another user sees nothing. */
    TEST_ASSERT_EQUAL_INT(0, db_camera_favorites_list(bob_id, favorites, 8));
}

void test_remove_is_idempotent(void) {
    stream_config_t lobby = create_camera("Lobby");
    bool removed = true;

    TEST_ASSERT_EQUAL_INT(DB_CAMERA_FAVORITE_OK,
                          db_camera_favorite_add(alice_id, lobby.camera_uuid, NULL));
    TEST_ASSERT_EQUAL_INT(DB_CAMERA_FAVORITE_OK,
                          db_camera_favorite_remove(alice_id, lobby.camera_uuid, &removed));
    TEST_ASSERT_TRUE(removed);
    TEST_ASSERT_EQUAL_INT(DB_CAMERA_FAVORITE_OK,
                          db_camera_favorite_remove(alice_id, lobby.camera_uuid, &removed));
    TEST_ASSERT_FALSE(removed);

    camera_favorite_t favorites[8];
    TEST_ASSERT_EQUAL_INT(0, db_camera_favorites_list(alice_id, favorites, 8));
}

void test_installation_scope_without_a_user(void) {
    stream_config_t lobby = create_camera("Lobby");
    camera_favorite_t favorites[8];

    TEST_ASSERT_EQUAL_INT(DB_CAMERA_FAVORITE_OK,
                          db_camera_favorite_add(0, lobby.camera_uuid, NULL));
    /* Any non-positive id addresses the same NULL-owner set. */
    TEST_ASSERT_EQUAL_INT(DB_CAMERA_FAVORITE_OK,
                          db_camera_favorite_add(-1, lobby.camera_uuid, NULL));
    TEST_ASSERT_EQUAL_INT(1, db_camera_favorites_list(0, favorites, 8));
    TEST_ASSERT_EQUAL_INT(0, db_camera_favorites_list(bob_id, favorites, 8));
}

void test_rejects_invalid_and_unknown_cameras(void) {
    char unknown[CAMERA_UUID_STRING_SIZE];
    TEST_ASSERT_EQUAL_INT(0, lightnvr_uuid_generate_v4(unknown));

    TEST_ASSERT_EQUAL_INT(DB_CAMERA_FAVORITE_INVALID,
                          db_camera_favorite_add(alice_id, "not-a-uuid", NULL));
    TEST_ASSERT_EQUAL_INT(DB_CAMERA_FAVORITE_INVALID,
                          db_camera_favorite_remove(alice_id, "", NULL));
    TEST_ASSERT_EQUAL_INT(DB_CAMERA_FAVORITE_NOT_FOUND,
                          db_camera_favorite_add(alice_id, unknown, NULL));
}

void test_deleting_a_user_drops_their_favorites(void) {
    stream_config_t lobby = create_camera("Lobby");
    camera_favorite_t favorites[8];
    int64_t carol_id = 0;

    TEST_ASSERT_EQUAL_INT(0, db_auth_create_user("carol", "password123", NULL,
                                                 USER_ROLE_USER, true, &carol_id));
    TEST_ASSERT_EQUAL_INT(DB_CAMERA_FAVORITE_OK,
                          db_camera_favorite_add(carol_id, lobby.camera_uuid, NULL));
    TEST_ASSERT_EQUAL_INT(DB_CAMERA_FAVORITE_OK,
                          db_camera_favorite_add(alice_id, lobby.camera_uuid, NULL));
    TEST_ASSERT_EQUAL_INT(0, db_auth_delete_user(carol_id));
    TEST_ASSERT_EQUAL_INT(0, db_camera_favorites_list(carol_id, favorites, 8));
    TEST_ASSERT_EQUAL_INT(1, db_camera_favorites_list(alice_id, favorites, 8));
}

void test_deleting_a_camera_drops_its_favorites(void) {
    stream_config_t lobby = create_camera("Lobby");
    camera_favorite_t favorites[8];

    TEST_ASSERT_EQUAL_INT(DB_CAMERA_FAVORITE_OK,
                          db_camera_favorite_add(alice_id, lobby.camera_uuid, NULL));
    TEST_ASSERT_EQUAL_INT(1, db_camera_favorites_list(alice_id, favorites, 8));

    /* A soft delete keeps the stream row (it can be restored), so the favorite
     * survives; the API hides it because the camera leaves the live scope. */
    TEST_ASSERT_EQUAL_INT(0, delete_stream_config(lobby.name));
    TEST_ASSERT_EQUAL_INT(1, db_camera_favorites_list(alice_id, favorites, 8));

    /* A permanent delete removes the row and the FK cascade drops the favorite. */
    TEST_ASSERT_EQUAL_INT(0, delete_stream_config_internal(lobby.name, true));
    TEST_ASSERT_EQUAL_INT(0, db_camera_favorites_list(alice_id, favorites, 8));
}

int main(void) {
    unlink(TEST_DB_PATH);
    if (init_database(TEST_DB_PATH) != 0) {
        fprintf(stderr, "FATAL: init_database failed\n");
        return 1;
    }
    if (db_auth_create_user("alice", "password123", NULL, USER_ROLE_USER, true,
                            &alice_id) != 0 ||
        db_auth_create_user("bob", "password123", NULL, USER_ROLE_USER, true,
                            &bob_id) != 0) {
        fprintf(stderr, "FATAL: could not create test users\n");
        return 1;
    }
    UNITY_BEGIN();
    RUN_TEST(test_add_is_idempotent_and_scoped_per_user);
    RUN_TEST(test_remove_is_idempotent);
    RUN_TEST(test_installation_scope_without_a_user);
    RUN_TEST(test_rejects_invalid_and_unknown_cameras);
    RUN_TEST(test_deleting_a_user_drops_their_favorites);
    RUN_TEST(test_deleting_a_camera_drops_its_favorites);
    int result = UNITY_END();
    shutdown_database();
    unlink(TEST_DB_PATH);
    return result;
}
