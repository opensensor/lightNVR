#define _POSIX_C_SOURCE 200809L
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>
#include <sqlite3.h>

#include "database/db_system_settings.h"
#include "database/db_core.h"
#include "core/logger.h"

int db_get_system_setting(const char *key, char *out, int out_len) {
    if (!key || !out || out_len <= 0) return -1;

    sqlite3 *db = get_db_handle();
    if (!db) { log_error("db_get_system_setting: no db handle"); return -1; }

    const char *sql = "SELECT value FROM system_settings WHERE key = ? LIMIT 1;";
    sqlite3_stmt *stmt = NULL;
    if (sqlite3_prepare_v2(db, sql, -1, &stmt, NULL) != SQLITE_OK) {
        log_error("db_get_system_setting: prepare failed: %s", sqlite3_errmsg(db));
        return -1;
    }
    sqlite3_bind_text(stmt, 1, key, -1, SQLITE_STATIC);

    int rc = -1;
    if (sqlite3_step(stmt) == SQLITE_ROW) {
        const char *val = (const char *)sqlite3_column_text(stmt, 0);
        if (val) {
            strncpy(out, val, out_len - 1);
            out[out_len - 1] = '\0';
            rc = 0;
        }
    }
    sqlite3_finalize(stmt);
    return rc;
}

int db_set_system_setting(const char *key, const char *value) {
    if (!key || !value) return -1;

    sqlite3 *db = get_db_handle();
    if (!db) { log_error("db_set_system_setting: no db handle"); return -1; }

    const char *sql =
        "INSERT INTO system_settings (key, value, updated_at) VALUES (?, ?, strftime('%s','now')) "
        "ON CONFLICT(key) DO UPDATE SET value = excluded.value, updated_at = excluded.updated_at;";
    sqlite3_stmt *stmt = NULL;
    if (sqlite3_prepare_v2(db, sql, -1, &stmt, NULL) != SQLITE_OK) {
        log_error("db_set_system_setting: prepare failed: %s", sqlite3_errmsg(db));
        return -1;
    }
    sqlite3_bind_text(stmt, 1, key, -1, SQLITE_STATIC);
    sqlite3_bind_text(stmt, 2, value, -1, SQLITE_STATIC);

    int rc = (sqlite3_step(stmt) == SQLITE_DONE) ? 0 : -1;
    if (rc != 0)
        log_error("db_set_system_setting: step failed: %s", sqlite3_errmsg(db));
    sqlite3_finalize(stmt);
    return rc;
}

bool db_is_setup_complete(void) {
    char val[8] = {0};
    if (db_get_system_setting("setup_complete", val, sizeof(val)) != 0)
        return false;
    return (val[0] == '1');
}

int db_mark_setup_complete(void) {
    if (db_set_system_setting("setup_complete", "1") != 0) return -1;

    char ts[32];
    snprintf(ts, sizeof(ts), "%lld", (long long)time(NULL));
    db_set_system_setting("setup_completed_at", ts); /* best-effort */
    log_info("Setup marked as complete");
    return 0;
}

