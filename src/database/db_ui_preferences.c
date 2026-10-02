#define _POSIX_C_SOURCE 200809L

#include "database/db_ui_preferences.h"

#include <stdbool.h>
#include <pthread.h>
#include <sqlite3.h>
#include <string.h>

#include "database/db_core.h"
#include "utils/strings.h"
#include "utils/uuid.h"

static bool valid_mode(const char *mode) {
    return mode && (strcmp(mode, "auto") == 0 ||
        strcmp(mode, "mobile") == 0 || strcmp(mode, "desktop") == 0);
}

int db_user_ui_mode_get(int64_t user_id, char *mode, unsigned int mode_size) {
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    if (!db || !mutex || user_id <= 0 || !mode || mode_size < 8) return -1;
    safe_strcpy(mode, "auto", mode_size, 0);
    pthread_mutex_lock(mutex);
    sqlite3_stmt *stmt = NULL;
    int rc = sqlite3_prepare_v2(db,
        "SELECT json_extract(value_json,'$') FROM user_preferences "
        "WHERE user_id=? AND "
        "preference_key='ui_mode';", -1, &stmt, NULL);
    if (rc == SQLITE_OK) {
        sqlite3_bind_int64(stmt, 1, user_id);
        rc = sqlite3_step(stmt);
        if (rc == SQLITE_ROW) {
            const char *value = (const char *)sqlite3_column_text(stmt, 0);
            if (!valid_mode(value)) rc = SQLITE_ERROR;
            else safe_strcpy(mode, value, mode_size, 0);
        }
    }
    if (stmt) sqlite3_finalize(stmt);
    pthread_mutex_unlock(mutex);
    return (rc == SQLITE_ROW || rc == SQLITE_DONE) ? 0 : -1;
}

int db_user_ui_mode_set(int64_t user_id, const char *mode) {
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    if (!db || !mutex || user_id <= 0 || !valid_mode(mode)) return -1;
    pthread_mutex_lock(mutex);
    sqlite3_stmt *stmt = NULL;
    if (strcmp(mode, "auto") == 0) {
        int rc = sqlite3_prepare_v2(db,
            "DELETE FROM user_preferences WHERE user_id=? AND "
            "preference_key='ui_mode';", -1, &stmt, NULL);
        if (rc == SQLITE_OK) {
            sqlite3_bind_int64(stmt, 1, user_id);
            rc = sqlite3_step(stmt);
        }
        if (stmt) sqlite3_finalize(stmt);
        pthread_mutex_unlock(mutex);
        return rc == SQLITE_DONE ? 0 : -1;
    }
    int rc = sqlite3_prepare_v2(db,
        "INSERT INTO user_preferences(user_id,preference_key,value_json) "
        "VALUES(?,'ui_mode',json_quote(?)) ON CONFLICT(user_id,preference_key) "
        "DO UPDATE SET value_json=excluded.value_json,"
        "updated_at=strftime('%s','now');", -1, &stmt, NULL);
    if (rc == SQLITE_OK) {
        sqlite3_bind_int64(stmt, 1, user_id);
        sqlite3_bind_text(stmt, 2, mode, -1, SQLITE_TRANSIENT);
        rc = sqlite3_step(stmt);
    }
    if (stmt) sqlite3_finalize(stmt);
    pthread_mutex_unlock(mutex);
    return rc == SQLITE_DONE ? 0 : -1;
}

int db_user_favorites_list(int64_t user_id, user_favorite_t *favorites,
                           int max_count) {
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    if (!db || !mutex || user_id <= 0 || !favorites || max_count < 1) return -1;
    pthread_mutex_lock(mutex);
    sqlite3_stmt *stmt = NULL;
    int rc = sqlite3_prepare_v2(db,
        "SELECT camera_uuid,created_at FROM user_favorites WHERE user_id=? "
        "ORDER BY created_at,camera_uuid LIMIT ?;", -1, &stmt, NULL);
    if (rc == SQLITE_OK) {
        sqlite3_bind_int64(stmt, 1, user_id);
        sqlite3_bind_int(stmt, 2, max_count);
    }
    int count = 0;
    if (rc == SQLITE_OK) {
        while ((rc = sqlite3_step(stmt)) == SQLITE_ROW) {
            if (count >= max_count) break;
            const char *uuid = (const char *)sqlite3_column_text(stmt, 0);
            safe_strcpy(favorites[count].camera_uuid, uuid ? uuid : "",
                        sizeof(favorites[count].camera_uuid), 0);
            favorites[count++].created_at = sqlite3_column_int64(stmt, 1);
        }
    }
    if (stmt) sqlite3_finalize(stmt);
    pthread_mutex_unlock(mutex);
    return rc == SQLITE_DONE ? count : -1;
}

int db_user_favorite_add(int64_t user_id, const char *camera_uuid,
                         int64_t *created_at) {
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    if (!db || !mutex || user_id <= 0 || !lightnvr_uuid_is_valid(camera_uuid)) return -1;
    pthread_mutex_lock(mutex);
    sqlite3_stmt *stmt = NULL;
    int rc = sqlite3_prepare_v2(db,
        "INSERT INTO user_favorites(user_id,camera_uuid) VALUES(?,?) "
        "ON CONFLICT(user_id,camera_uuid) DO NOTHING;", -1, &stmt, NULL);
    if (rc == SQLITE_OK) {
        sqlite3_bind_int64(stmt, 1, user_id);
        sqlite3_bind_text(stmt, 2, camera_uuid, -1, SQLITE_TRANSIENT);
        rc = sqlite3_step(stmt);
    }
    if (stmt) sqlite3_finalize(stmt);
    if (rc == SQLITE_DONE && created_at) {
        stmt = NULL;
        rc = sqlite3_prepare_v2(db,
            "SELECT created_at FROM user_favorites WHERE user_id=? AND "
            "camera_uuid=?;", -1, &stmt, NULL);
        if (rc == SQLITE_OK) {
            sqlite3_bind_int64(stmt, 1, user_id);
            sqlite3_bind_text(stmt, 2, camera_uuid, -1, SQLITE_TRANSIENT);
            rc = sqlite3_step(stmt);
            if (rc == SQLITE_ROW) *created_at = sqlite3_column_int64(stmt, 0);
        }
        if (stmt) sqlite3_finalize(stmt);
    }
    pthread_mutex_unlock(mutex);
    return rc == SQLITE_ROW || rc == SQLITE_DONE ? 0 : -1;
}

int db_user_favorite_delete(int64_t user_id, const char *camera_uuid) {
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    if (!db || !mutex || user_id <= 0 || !lightnvr_uuid_is_valid(camera_uuid)) return -1;
    pthread_mutex_lock(mutex);
    sqlite3_stmt *stmt = NULL;
    int rc = sqlite3_prepare_v2(db,
        "DELETE FROM user_favorites WHERE user_id=? AND camera_uuid=?;",
        -1, &stmt, NULL);
    if (rc == SQLITE_OK) {
        sqlite3_bind_int64(stmt, 1, user_id);
        sqlite3_bind_text(stmt, 2, camera_uuid, -1, SQLITE_TRANSIENT);
        rc = sqlite3_step(stmt);
    }
    if (stmt) sqlite3_finalize(stmt);
    pthread_mutex_unlock(mutex);
    return rc == SQLITE_DONE ? 0 : -1;
}
