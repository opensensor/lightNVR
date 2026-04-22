#define _POSIX_C_SOURCE 200809L
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>
#include <sqlite3.h>

#include "database/db_system_settings.h"
#include "database/db_core.h"
#include "core/logger.h"
#include "utils/strings.h"

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
            safe_strcpy(out, val, out_len, 0);
            rc = 0;
        }
    }
    sqlite3_finalize(stmt);
    return rc;
}

int db_get_system_setting_alloc(const char *key, char **out, size_t *out_len) {
    if (!key || !out || !out_len) return -1;

    /* Normalize outputs so early-return paths are consistent for callers. */
    *out = NULL;
    *out_len = 0;

    sqlite3 *db = get_db_handle();
    if (!db) {
        log_error("db_get_system_setting_alloc: no db handle");
        return -1;
    }

    const char *sql = "SELECT value FROM system_settings WHERE key = ? LIMIT 1;";
    sqlite3_stmt *stmt = NULL;
    if (sqlite3_prepare_v2(db, sql, -1, &stmt, NULL) != SQLITE_OK) {
        log_error("db_get_system_setting_alloc: prepare failed: %s", sqlite3_errmsg(db));
        return -1;
    }
    sqlite3_bind_text(stmt, 1, key, -1, SQLITE_STATIC);

    int rc;
    int step = sqlite3_step(stmt);
    if (step == SQLITE_ROW) {
        /* sqlite3_column_bytes must be called AFTER any text coercion; call
         * sqlite3_column_text first so bytes reflects the UTF-8 length. */
        const unsigned char *val = sqlite3_column_text(stmt, 0);
        int nbytes = sqlite3_column_bytes(stmt, 0);
        if (!val || nbytes < 0) {
            /* Treat NULL column as "not found" — there is nothing to copy. */
            rc = 1;
        } else {
            char *buf = malloc((size_t)nbytes + 1);
            if (!buf) {
                log_error("db_get_system_setting_alloc: malloc(%d) failed", nbytes + 1);
                rc = -1;
            } else {
                if (nbytes > 0) {
                    memcpy(buf, val, (size_t)nbytes);
                }
                buf[nbytes] = '\0';
                *out = buf;
                *out_len = (size_t)nbytes;
                rc = 0;
            }
        }
    } else if (step == SQLITE_DONE) {
        rc = 1; /* key not present */
    } else {
        log_error("db_get_system_setting_alloc: step failed (rc=%d): %s", step, sqlite3_errmsg(db));
        rc = -1;
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

