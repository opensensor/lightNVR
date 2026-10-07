#define _POSIX_C_SOURCE 200809L

#include "database/db_camera_favorites.h"

#include <pthread.h>
#include <sqlite3.h>
#include <string.h>

#include "core/logger.h"
#include "database/db_core.h"
#include "utils/strings.h"
#include "utils/uuid.h"

/* Parameter 1 is the caller's user id, with 0 selecting the NULL-owner set. */
#define OWNER_MATCH "((?1=0 AND user_id IS NULL) OR user_id=?1)"

static int64_t scope_id(int64_t user_id) {
    return user_id > 0 ? user_id : 0;
}

int db_camera_favorites_list(int64_t user_id, camera_favorite_t *favorites,
                             int max_count) {
    if (!favorites || max_count < 1) return -1;
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    if (!db || !mutex) return -1;

    static const char *sql =
        "SELECT camera_uuid,created_at FROM camera_favorites "
        "WHERE " OWNER_MATCH " ORDER BY created_at,camera_uuid LIMIT ?2;";

    pthread_mutex_lock(mutex);
    sqlite3_stmt *statement = NULL;
    int count = 0;
    int rc = sqlite3_prepare_v2(db, sql, -1, &statement, NULL);
    if (rc == SQLITE_OK) {
        sqlite3_bind_int64(statement, 1, scope_id(user_id));
        sqlite3_bind_int(statement, 2, max_count);
        while ((rc = sqlite3_step(statement)) == SQLITE_ROW && count < max_count) {
            const char *uuid = (const char *)sqlite3_column_text(statement, 0);
            safe_strcpy(favorites[count].camera_uuid, uuid ? uuid : "",
                        sizeof(favorites[count].camera_uuid), 0);
            favorites[count].created_at = sqlite3_column_int64(statement, 1);
            count++;
        }
        if (rc == SQLITE_ROW) rc = SQLITE_DONE;
    }
    if (rc != SQLITE_DONE) {
        log_error("Failed to list camera favorites: %s", sqlite3_errmsg(db));
    }
    if (statement) sqlite3_finalize(statement);
    pthread_mutex_unlock(mutex);
    return rc == SQLITE_DONE ? count : -1;
}

db_camera_favorite_result_t db_camera_favorite_add(int64_t user_id,
                                                   const char *camera_uuid,
                                                   camera_favorite_t *favorite) {
    if (!lightnvr_uuid_is_valid(camera_uuid)) return DB_CAMERA_FAVORITE_INVALID;
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    if (!db || !mutex) return DB_CAMERA_FAVORITE_ERROR;

    /* OR IGNORE swallows the unique-index hit on a repeat add but still
     * raises the foreign key failure for a camera that does not exist. */
    static const char *insert_sql =
        "INSERT OR IGNORE INTO camera_favorites(user_id,camera_uuid) VALUES(?1,?2);";
    static const char *select_sql =
        "SELECT camera_uuid,created_at FROM camera_favorites "
        "WHERE " OWNER_MATCH " AND camera_uuid=?2;";

    db_camera_favorite_result_t result = DB_CAMERA_FAVORITE_ERROR;
    pthread_mutex_lock(mutex);
    sqlite3_stmt *statement = NULL;
    int rc = sqlite3_prepare_v2(db, insert_sql, -1, &statement, NULL);
    if (rc == SQLITE_OK) {
        if (user_id > 0) sqlite3_bind_int64(statement, 1, user_id);
        else sqlite3_bind_null(statement, 1);
        sqlite3_bind_text(statement, 2, camera_uuid, -1, SQLITE_TRANSIENT);
        rc = sqlite3_step(statement);
    }
    if (statement) sqlite3_finalize(statement);
    statement = NULL;

    if (rc == SQLITE_CONSTRAINT) {
        result = DB_CAMERA_FAVORITE_NOT_FOUND;
    } else if (rc != SQLITE_DONE) {
        log_error("Failed to add camera favorite: %s", sqlite3_errmsg(db));
    } else {
        rc = sqlite3_prepare_v2(db, select_sql, -1, &statement, NULL);
        if (rc == SQLITE_OK) {
            sqlite3_bind_int64(statement, 1, scope_id(user_id));
            sqlite3_bind_text(statement, 2, camera_uuid, -1, SQLITE_TRANSIENT);
            rc = sqlite3_step(statement);
        }
        if (rc == SQLITE_ROW) {
            if (favorite) {
                memset(favorite, 0, sizeof(*favorite));
                const char *uuid = (const char *)sqlite3_column_text(statement, 0);
                safe_strcpy(favorite->camera_uuid, uuid ? uuid : "",
                            sizeof(favorite->camera_uuid), 0);
                favorite->created_at = sqlite3_column_int64(statement, 1);
            }
            result = DB_CAMERA_FAVORITE_OK;
        } else {
            log_error("Failed to read back camera favorite: %s", sqlite3_errmsg(db));
        }
        if (statement) sqlite3_finalize(statement);
    }
    pthread_mutex_unlock(mutex);
    return result;
}

db_camera_favorite_result_t db_camera_favorite_remove(int64_t user_id,
                                                      const char *camera_uuid,
                                                      bool *removed) {
    if (removed) *removed = false;
    if (!lightnvr_uuid_is_valid(camera_uuid)) return DB_CAMERA_FAVORITE_INVALID;
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    if (!db || !mutex) return DB_CAMERA_FAVORITE_ERROR;

    static const char *sql =
        "DELETE FROM camera_favorites WHERE " OWNER_MATCH " AND camera_uuid=?2;";

    pthread_mutex_lock(mutex);
    sqlite3_stmt *statement = NULL;
    int rc = sqlite3_prepare_v2(db, sql, -1, &statement, NULL);
    if (rc == SQLITE_OK) {
        sqlite3_bind_int64(statement, 1, scope_id(user_id));
        sqlite3_bind_text(statement, 2, camera_uuid, -1, SQLITE_TRANSIENT);
        rc = sqlite3_step(statement);
    }
    if (rc == SQLITE_DONE) {
        if (removed) *removed = sqlite3_changes(db) > 0;
    } else {
        log_error("Failed to remove camera favorite: %s", sqlite3_errmsg(db));
    }
    if (statement) sqlite3_finalize(statement);
    pthread_mutex_unlock(mutex);
    return rc == SQLITE_DONE ? DB_CAMERA_FAVORITE_OK : DB_CAMERA_FAVORITE_ERROR;
}
