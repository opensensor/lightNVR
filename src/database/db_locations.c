#define _POSIX_C_SOURCE 200809L

#include <pthread.h>
#include <sqlite3.h>
#include <stdbool.h>
#include <stdio.h>
#include <string.h>

#include "core/logger.h"
#include "database/db_core.h"
#include "database/db_locations.h"
#include "utils/strings.h"

#define LOCATION_SELECT_FIELDS \
    "l.uuid, l.parent_uuid, l.name, l.type, l.sort_order, l.metadata_json, " \
    "l.is_system, l.created_at, l.updated_at, " \
    "(SELECT count(*) FROM camera_locations c WHERE c.parent_uuid = l.uuid), " \
    "(SELECT count(*) FROM streams s WHERE s.location_uuid = l.uuid) "

static bool has_non_whitespace(const char *value) {
    if (!value) return false;
    while (*value == ' ' || *value == '\t' || *value == '\n' || *value == '\r') {
        value++;
    }
    return *value != '\0';
}

static bool valid_uuid_string(const char *uuid) {
    return uuid && strlen(uuid) == CAMERA_UUID_STRING_SIZE - 1;
}

static void copy_column(char *destination, size_t destination_size,
                        sqlite3_stmt *stmt, int column) {
    const char *value = (const char *)sqlite3_column_text(stmt, column);
    if (value) {
        safe_strcpy(destination, value, destination_size, 0);
    } else if (destination_size > 0) {
        destination[0] = '\0';
    }
}

static void populate_location(sqlite3_stmt *stmt, camera_location_t *location) {
    memset(location, 0, sizeof(*location));
    copy_column(location->uuid, sizeof(location->uuid), stmt, 0);
    copy_column(location->parent_uuid, sizeof(location->parent_uuid), stmt, 1);
    copy_column(location->name, sizeof(location->name), stmt, 2);
    copy_column(location->type, sizeof(location->type), stmt, 3);
    location->sort_order = sqlite3_column_int(stmt, 4);
    copy_column(location->metadata_json, sizeof(location->metadata_json), stmt, 5);
    location->is_system = sqlite3_column_int(stmt, 6);
    location->created_at = sqlite3_column_int64(stmt, 7);
    location->updated_at = sqlite3_column_int64(stmt, 8);
    location->direct_child_count = sqlite3_column_int(stmt, 9);
    location->direct_camera_count = sqlite3_column_int(stmt, 10);
}

static db_location_result_t prepare_error(sqlite3 *db, const char *operation) {
    log_error("Failed to %s camera location: %s", operation, sqlite3_errmsg(db));
    return DB_LOCATION_ERROR;
}

static db_location_result_t get_locked(sqlite3 *db, const char *where_clause,
                                       const char *value,
                                       camera_location_t *location) {
    char sql[768];
    int written = snprintf(sql, sizeof(sql),
                           "SELECT " LOCATION_SELECT_FIELDS
                           "FROM camera_locations l WHERE %s LIMIT 1;",
                           where_clause);
    if (written < 0 || (size_t)written >= sizeof(sql)) {
        return DB_LOCATION_ERROR;
    }

    sqlite3_stmt *stmt = NULL;
    int rc = sqlite3_prepare_v2(db, sql, -1, &stmt, NULL);
    if (rc != SQLITE_OK) return prepare_error(db, "read");
    if (value) sqlite3_bind_text(stmt, 1, value, -1, SQLITE_TRANSIENT);

    db_location_result_t result = DB_LOCATION_NOT_FOUND;
    rc = sqlite3_step(stmt);
    if (rc == SQLITE_ROW) {
        populate_location(stmt, location);
        result = DB_LOCATION_OK;
    } else if (rc != SQLITE_DONE) {
        result = prepare_error(db, "read");
    }
    sqlite3_finalize(stmt);
    return result;
}

db_location_result_t db_location_get_unassigned(camera_location_t *location) {
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    if (!db || !location) return DB_LOCATION_INVALID;

    pthread_mutex_lock(mutex);
    db_location_result_t result =
        get_locked(db, "l.is_system = 1", NULL, location);
    pthread_mutex_unlock(mutex);
    return result;
}

db_location_result_t db_location_get(const char *uuid,
                                     camera_location_t *location) {
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    if (!db || !location || !valid_uuid_string(uuid)) return DB_LOCATION_INVALID;

    pthread_mutex_lock(mutex);
    db_location_result_t result = get_locked(db, "l.uuid = ?", uuid, location);
    pthread_mutex_unlock(mutex);
    return result;
}

int db_location_count(void) {
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    if (!db) return -1;

    pthread_mutex_lock(mutex);
    sqlite3_stmt *stmt = NULL;
    int rc = sqlite3_prepare_v2(db, "SELECT count(*) FROM camera_locations;",
                                -1, &stmt, NULL);
    int count = -1;
    if (rc == SQLITE_OK && sqlite3_step(stmt) == SQLITE_ROW) {
        count = sqlite3_column_int(stmt, 0);
    } else {
        prepare_error(db, "count");
    }
    if (stmt) sqlite3_finalize(stmt);
    pthread_mutex_unlock(mutex);
    return count;
}

int db_location_list(camera_location_t *locations, int max_count) {
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    if (!db || !locations || max_count <= 0) return -1;

    const char *sql =
        "SELECT " LOCATION_SELECT_FIELDS
        "FROM camera_locations l "
        "ORDER BY CASE WHEN l.parent_uuid IS NULL THEN 0 ELSE 1 END, "
        "l.parent_uuid, l.sort_order, l.name COLLATE NOCASE;";

    pthread_mutex_lock(mutex);
    sqlite3_stmt *stmt = NULL;
    int rc = sqlite3_prepare_v2(db, sql, -1, &stmt, NULL);
    if (rc != SQLITE_OK) {
        prepare_error(db, "list");
        pthread_mutex_unlock(mutex);
        return -1;
    }

    int count = 0;
    while (count < max_count && (rc = sqlite3_step(stmt)) == SQLITE_ROW) {
        populate_location(stmt, &locations[count++]);
    }
    if (rc != SQLITE_ROW && rc != SQLITE_DONE) {
        prepare_error(db, "list");
        count = -1;
    }
    sqlite3_finalize(stmt);
    pthread_mutex_unlock(mutex);
    return count;
}

static bool row_exists_locked(sqlite3 *db, const char *sql, const char *value) {
    sqlite3_stmt *stmt = NULL;
    if (sqlite3_prepare_v2(db, sql, -1, &stmt, NULL) != SQLITE_OK) return false;
    sqlite3_bind_text(stmt, 1, value, -1, SQLITE_TRANSIENT);
    bool exists = sqlite3_step(stmt) == SQLITE_ROW;
    sqlite3_finalize(stmt);
    return exists;
}

db_location_result_t db_location_create(camera_location_t *location) {
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    if (!db || !location || !has_non_whitespace(location->name) ||
        (location->parent_uuid[0] && !valid_uuid_string(location->parent_uuid))) {
        return DB_LOCATION_INVALID;
    }

    const char *sql =
        "INSERT INTO camera_locations "
        "(uuid, parent_uuid, name, type, sort_order, metadata_json) VALUES ("
        "lower(hex(randomblob(4)) || '-' || hex(randomblob(2)) || '-4' || "
        "substr(hex(randomblob(2)), 2) || '-' || "
        "substr('89ab', (abs(random()) % 4) + 1, 1) || "
        "substr(hex(randomblob(2)), 2) || '-' || hex(randomblob(6))), "
        "?, ?, ?, ?, ?);";

    pthread_mutex_lock(mutex);
    if (location->parent_uuid[0] &&
        !row_exists_locked(db, "SELECT 1 FROM camera_locations WHERE uuid = ?;",
                           location->parent_uuid)) {
        pthread_mutex_unlock(mutex);
        return DB_LOCATION_NOT_FOUND;
    }

    sqlite3_stmt *stmt = NULL;
    int rc = sqlite3_prepare_v2(db, sql, -1, &stmt, NULL);
    if (rc != SQLITE_OK) {
        db_location_result_t result = prepare_error(db, "create");
        pthread_mutex_unlock(mutex);
        return result;
    }

    if (location->parent_uuid[0]) {
        sqlite3_bind_text(stmt, 1, location->parent_uuid, -1, SQLITE_STATIC);
    } else {
        sqlite3_bind_null(stmt, 1);
    }
    sqlite3_bind_text(stmt, 2, location->name, -1, SQLITE_STATIC);
    sqlite3_bind_text(stmt, 3,
                      location->type[0] ? location->type : "area",
                      -1, SQLITE_STATIC);
    sqlite3_bind_int(stmt, 4, location->sort_order);
    sqlite3_bind_text(stmt, 5,
                      location->metadata_json[0] ? location->metadata_json : "{}",
                      -1, SQLITE_STATIC);

    rc = sqlite3_step(stmt);
    sqlite3_finalize(stmt);
    if (rc != SQLITE_DONE) {
        db_location_result_t result =
            (rc == SQLITE_CONSTRAINT) ? DB_LOCATION_CONFLICT
                                      : prepare_error(db, "create");
        pthread_mutex_unlock(mutex);
        return result;
    }

    char row_id[32];
    snprintf(row_id, sizeof(row_id), "%lld",
             (long long)sqlite3_last_insert_rowid(db));
    db_location_result_t result = get_locked(db, "l.id = ?", row_id, location);
    pthread_mutex_unlock(mutex);
    return result;
}

db_location_result_t db_location_update(camera_location_t *location) {
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    if (!db || !location || !valid_uuid_string(location->uuid) ||
        !has_non_whitespace(location->name) ||
        (location->parent_uuid[0] && !valid_uuid_string(location->parent_uuid))) {
        return DB_LOCATION_INVALID;
    }

    pthread_mutex_lock(mutex);
    camera_location_t existing;
    db_location_result_t result = get_locked(db, "l.uuid = ?", location->uuid,
                                             &existing);
    if (result != DB_LOCATION_OK) {
        pthread_mutex_unlock(mutex);
        return result;
    }
    if (existing.is_system) {
        pthread_mutex_unlock(mutex);
        return DB_LOCATION_INVALID;
    }

    if (location->parent_uuid[0]) {
        if (!row_exists_locked(db,
                               "SELECT 1 FROM camera_locations WHERE uuid = ?;",
                               location->parent_uuid)) {
            pthread_mutex_unlock(mutex);
            return DB_LOCATION_NOT_FOUND;
        }

        const char *cycle_sql =
            "WITH RECURSIVE ancestors(uuid, parent_uuid) AS ("
            "SELECT uuid, parent_uuid FROM camera_locations WHERE uuid = ? "
            "UNION ALL "
            "SELECT l.uuid, l.parent_uuid FROM camera_locations l "
            "JOIN ancestors a ON l.uuid = a.parent_uuid"
            ") SELECT 1 FROM ancestors WHERE uuid = ? LIMIT 1;";
        sqlite3_stmt *cycle_stmt = NULL;
        int rc = sqlite3_prepare_v2(db, cycle_sql, -1, &cycle_stmt, NULL);
        if (rc != SQLITE_OK) {
            result = prepare_error(db, "validate");
            pthread_mutex_unlock(mutex);
            return result;
        }
        sqlite3_bind_text(cycle_stmt, 1, location->parent_uuid, -1,
                          SQLITE_STATIC);
        sqlite3_bind_text(cycle_stmt, 2, location->uuid, -1, SQLITE_STATIC);
        bool creates_cycle = sqlite3_step(cycle_stmt) == SQLITE_ROW;
        sqlite3_finalize(cycle_stmt);
        if (creates_cycle) {
            pthread_mutex_unlock(mutex);
            return DB_LOCATION_CONFLICT;
        }
    }

    const char *update_sql =
        "UPDATE camera_locations SET parent_uuid = ?, name = ?, type = ?, "
        "sort_order = ?, metadata_json = ?, updated_at = strftime('%s', 'now') "
        "WHERE uuid = ? AND is_system = 0;";
    sqlite3_stmt *stmt = NULL;
    int rc = sqlite3_prepare_v2(db, update_sql, -1, &stmt, NULL);
    if (rc != SQLITE_OK) {
        result = prepare_error(db, "update");
        pthread_mutex_unlock(mutex);
        return result;
    }
    if (location->parent_uuid[0]) {
        sqlite3_bind_text(stmt, 1, location->parent_uuid, -1, SQLITE_STATIC);
    } else {
        sqlite3_bind_null(stmt, 1);
    }
    sqlite3_bind_text(stmt, 2, location->name, -1, SQLITE_STATIC);
    sqlite3_bind_text(stmt, 3,
                      location->type[0] ? location->type : "area",
                      -1, SQLITE_STATIC);
    sqlite3_bind_int(stmt, 4, location->sort_order);
    sqlite3_bind_text(stmt, 5,
                      location->metadata_json[0] ? location->metadata_json : "{}",
                      -1, SQLITE_STATIC);
    sqlite3_bind_text(stmt, 6, location->uuid, -1, SQLITE_STATIC);
    rc = sqlite3_step(stmt);
    sqlite3_finalize(stmt);
    if (rc != SQLITE_DONE) {
        result = (rc == SQLITE_CONSTRAINT) ? DB_LOCATION_CONFLICT
                                           : prepare_error(db, "update");
        pthread_mutex_unlock(mutex);
        return result;
    }

    result = get_locked(db, "l.uuid = ?", location->uuid, location);
    pthread_mutex_unlock(mutex);
    return result;
}

db_location_result_t db_location_delete(const char *uuid) {
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    if (!db || !valid_uuid_string(uuid)) return DB_LOCATION_INVALID;

    pthread_mutex_lock(mutex);
    camera_location_t existing;
    db_location_result_t result = get_locked(db, "l.uuid = ?", uuid, &existing);
    if (result != DB_LOCATION_OK) {
        pthread_mutex_unlock(mutex);
        return result;
    }
    if (existing.is_system) {
        pthread_mutex_unlock(mutex);
        return DB_LOCATION_INVALID;
    }
    if (existing.direct_child_count > 0 || existing.direct_camera_count > 0) {
        pthread_mutex_unlock(mutex);
        return DB_LOCATION_CONFLICT;
    }

    sqlite3_stmt *stmt = NULL;
    int rc = sqlite3_prepare_v2(db,
                                "DELETE FROM camera_locations WHERE uuid = ? "
                                "AND is_system = 0;",
                                -1, &stmt, NULL);
    if (rc == SQLITE_OK) {
        sqlite3_bind_text(stmt, 1, uuid, -1, SQLITE_STATIC);
        rc = sqlite3_step(stmt);
    }
    if (stmt) sqlite3_finalize(stmt);
    if (rc != SQLITE_DONE) {
        result = (rc == SQLITE_CONSTRAINT) ? DB_LOCATION_CONFLICT
                                           : prepare_error(db, "delete");
    } else {
        result = DB_LOCATION_OK;
    }
    pthread_mutex_unlock(mutex);
    return result;
}

db_location_result_t db_location_assign_camera(const char *camera_uuid,
                                               const char *location_uuid) {
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    if (!db || !valid_uuid_string(camera_uuid) ||
        !valid_uuid_string(location_uuid)) {
        return DB_LOCATION_INVALID;
    }

    pthread_mutex_lock(mutex);
    if (!row_exists_locked(db, "SELECT 1 FROM camera_locations WHERE uuid = ?;",
                           location_uuid) ||
        !row_exists_locked(db, "SELECT 1 FROM streams WHERE camera_uuid = ?;",
                           camera_uuid)) {
        pthread_mutex_unlock(mutex);
        return DB_LOCATION_NOT_FOUND;
    }

    sqlite3_stmt *stmt = NULL;
    int rc = sqlite3_prepare_v2(db,
                                "UPDATE streams SET location_uuid = ? "
                                "WHERE camera_uuid = ?;",
                                -1, &stmt, NULL);
    if (rc == SQLITE_OK) {
        sqlite3_bind_text(stmt, 1, location_uuid, -1, SQLITE_STATIC);
        sqlite3_bind_text(stmt, 2, camera_uuid, -1, SQLITE_STATIC);
        rc = sqlite3_step(stmt);
    }
    if (stmt) sqlite3_finalize(stmt);
    db_location_result_t result =
        (rc == SQLITE_DONE) ? DB_LOCATION_OK : prepare_error(db, "assign");
    pthread_mutex_unlock(mutex);
    return result;
}

db_location_result_t db_location_get_for_camera(const char *camera_uuid,
                                                camera_location_t *location) {
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    if (!db || !location || !valid_uuid_string(camera_uuid)) {
        return DB_LOCATION_INVALID;
    }

    const char *sql =
        "SELECT " LOCATION_SELECT_FIELDS
        "FROM camera_locations l JOIN streams s ON s.location_uuid = l.uuid "
        "WHERE s.camera_uuid = ? LIMIT 1;";
    pthread_mutex_lock(mutex);
    sqlite3_stmt *stmt = NULL;
    int rc = sqlite3_prepare_v2(db, sql, -1, &stmt, NULL);
    if (rc != SQLITE_OK) {
        db_location_result_t result = prepare_error(db, "read camera");
        pthread_mutex_unlock(mutex);
        return result;
    }
    sqlite3_bind_text(stmt, 1, camera_uuid, -1, SQLITE_STATIC);
    db_location_result_t result = DB_LOCATION_NOT_FOUND;
    rc = sqlite3_step(stmt);
    if (rc == SQLITE_ROW) {
        populate_location(stmt, location);
        result = DB_LOCATION_OK;
    } else if (rc != SQLITE_DONE) {
        result = prepare_error(db, "read camera");
    }
    sqlite3_finalize(stmt);
    pthread_mutex_unlock(mutex);
    return result;
}
