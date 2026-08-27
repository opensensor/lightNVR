#define _POSIX_C_SOURCE 200809L

#include "database/db_workspace_preferences.h"

#include <pthread.h>
#include <sqlite3.h>
#include <string.h>

#include "core/logger.h"
#include "database/db_core.h"

void db_workspace_preferences_defaults(workspace_preferences_t *preferences) {
    if (!preferences) return;
    preferences->live_navigator_visible = true;
    preferences->investigation_visible = true;
}

static void bind_owner(sqlite3_stmt *statement, int parameter,
                       int64_t owner_user_id) {
    sqlite3_bind_int64(statement, parameter, owner_user_id);
}

int db_workspace_preferences_get(int64_t owner_user_id,
                                 workspace_preferences_t *preferences) {
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    if (!db || !mutex || !preferences || owner_user_id < 0) return -1;
    db_workspace_preferences_defaults(preferences);

    const char *sql =
        "SELECT workspace_key,is_visible FROM user_workspace_preferences "
        "WHERE (?=0 AND owner_user_id IS NULL) OR owner_user_id=?;";
    pthread_mutex_lock(mutex);
    sqlite3_stmt *statement = NULL;
    int result = sqlite3_prepare_v2(db, sql, -1, &statement, NULL);
    if (result == SQLITE_OK) {
        bind_owner(statement, 1, owner_user_id);
        bind_owner(statement, 2, owner_user_id);
    }
    if (result == SQLITE_OK) {
        while ((result = sqlite3_step(statement)) == SQLITE_ROW) {
            const char *key =
                (const char *)sqlite3_column_text(statement, 0);
            bool visible = sqlite3_column_int(statement, 1) != 0;
            if (key && strcmp(key, WORKSPACE_KEY_LIVE_NAVIGATOR) == 0) {
                preferences->live_navigator_visible = visible;
            } else if (key && strcmp(key, WORKSPACE_KEY_INVESTIGATION) == 0) {
                preferences->investigation_visible = visible;
            }
        }
    }
    if (statement) sqlite3_finalize(statement);
    if (result != SQLITE_DONE) {
        log_error("Failed to load workspace preferences: %s",
                  sqlite3_errmsg(db));
    }
    pthread_mutex_unlock(mutex);
    return result == SQLITE_DONE ? 0 : -1;
}

static int replace_one(sqlite3 *db, int64_t owner_user_id, const char *key,
                       bool visible) {
    const char *update_sql =
        "UPDATE user_workspace_preferences SET is_visible=?,"
        "updated_at=strftime('%s','now') WHERE workspace_key=? AND "
        "((?=0 AND owner_user_id IS NULL) OR owner_user_id=?);";
    sqlite3_stmt *statement = NULL;
    int result = sqlite3_prepare_v2(db, update_sql, -1, &statement, NULL);
    if (result == SQLITE_OK) {
        sqlite3_bind_int(statement, 1, visible ? 1 : 0);
        sqlite3_bind_text(statement, 2, key, -1, SQLITE_STATIC);
        bind_owner(statement, 3, owner_user_id);
        bind_owner(statement, 4, owner_user_id);
        result = sqlite3_step(statement);
    }
    if (statement) sqlite3_finalize(statement);
    if (result != SQLITE_DONE) return -1;
    if (sqlite3_changes(db) > 0) return 0;

    const char *insert_sql =
        "INSERT INTO user_workspace_preferences"
        "(owner_user_id,workspace_key,is_visible) VALUES(?,?,?);";
    statement = NULL;
    result = sqlite3_prepare_v2(db, insert_sql, -1, &statement, NULL);
    if (result == SQLITE_OK) {
        if (owner_user_id > 0) {
            sqlite3_bind_int64(statement, 1, owner_user_id);
        } else {
            sqlite3_bind_null(statement, 1);
        }
        sqlite3_bind_text(statement, 2, key, -1, SQLITE_STATIC);
        sqlite3_bind_int(statement, 3, visible ? 1 : 0);
        result = sqlite3_step(statement);
    }
    if (statement) sqlite3_finalize(statement);
    return result == SQLITE_DONE ? 0 : -1;
}

int db_workspace_preferences_replace(
    int64_t owner_user_id, const workspace_preferences_t *preferences) {
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    if (!db || !mutex || !preferences || owner_user_id < 0) return -1;

    pthread_mutex_lock(mutex);
    int result = sqlite3_exec(db, "BEGIN IMMEDIATE;", NULL, NULL, NULL);
    if (result == SQLITE_OK &&
        replace_one(db, owner_user_id, WORKSPACE_KEY_LIVE_NAVIGATOR,
                    preferences->live_navigator_visible) == 0 &&
        replace_one(db, owner_user_id, WORKSPACE_KEY_INVESTIGATION,
                    preferences->investigation_visible) == 0) {
        result = sqlite3_exec(db, "COMMIT;", NULL, NULL, NULL);
    } else {
        sqlite3_exec(db, "ROLLBACK;", NULL, NULL, NULL);
        result = SQLITE_ERROR;
    }
    pthread_mutex_unlock(mutex);
    return result == SQLITE_OK ? 0 : -1;
}
