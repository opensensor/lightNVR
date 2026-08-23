#define _POSIX_C_SOURCE 200809L

#include <cjson/cJSON.h>
#include <ctype.h>
#include <pthread.h>
#include <sqlite3.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "core/camera_selector.h"
#include "core/logger.h"
#include "database/db_camera_collections.h"
#include "database/db_core.h"
#include "utils/strings.h"

#define COLLECTION_SELECT_FIELDS \
    "c.uuid, c.name, c.description, c.collection_type, c.selector_json, " \
    "c.is_shared, COALESCE(c.owner_user_id, 0), c.created_at, c.updated_at, " \
    "(SELECT count(*) FROM camera_collection_members m " \
    " WHERE m.collection_uuid = c.uuid) "

static bool valid_uuid(const char *value) {
    if (!value || strlen(value) != CAMERA_UUID_STRING_SIZE - 1) return false;
    for (int i = 0; i < CAMERA_UUID_STRING_SIZE - 1; i++) {
        unsigned char c = (unsigned char)value[i];
        if (i == 8 || i == 13 || i == 18 || i == 23) {
            if (c != '-') return false;
        } else if (!isxdigit(c)) {
            return false;
        }
    }
    return true;
}

static bool valid_name(const char *input, char *normalized, size_t size) {
    if (!input || copy_trimmed_value(normalized, size, input, 0) == 0) return false;
    for (const unsigned char *p = (const unsigned char *)normalized; *p; p++) {
        if (iscntrl(*p)) return false;
    }
    return true;
}

static bool valid_selector_json(const char *json) {
    if (!json || json[0] == '\0') return false;
    cJSON *root = cJSON_Parse(json);
    if (!root) return false;
    char error[FLEET_SELECTOR_ERROR_MAX] = {0};
    fleet_selector_t *selector =
        fleet_selector_parse(root, error, sizeof(error));
    bool valid = selector != NULL;
    fleet_selector_free(selector);
    cJSON_Delete(root);
    return valid;
}

static bool valid_collection(camera_collection_t *collection,
                             char *normalized_name, size_t name_size) {
    if (!collection ||
        !valid_name(collection->name, normalized_name, name_size)) return false;
    if (strcmp(collection->collection_type, "static") == 0) return true;
    if (strcmp(collection->collection_type, "smart") == 0) {
        return valid_selector_json(collection->selector_json);
    }
    return false;
}

static bool transaction_begin(sqlite3 *db) {
    return sqlite3_exec(db, "BEGIN IMMEDIATE;", NULL, NULL, NULL) == SQLITE_OK;
}

static bool transaction_finish(sqlite3 *db, bool success) {
    if (!success) {
        sqlite3_exec(db, "ROLLBACK;", NULL, NULL, NULL);
        return false;
    }
    if (sqlite3_exec(db, "COMMIT;", NULL, NULL, NULL) == SQLITE_OK) {
        return true;
    }
    sqlite3_exec(db, "ROLLBACK;", NULL, NULL, NULL);
    return false;
}

static void copy_column(char *destination, size_t destination_size,
                        sqlite3_stmt *stmt, int column) {
    const char *value = (const char *)sqlite3_column_text(stmt, column);
    safe_strcpy(destination, value ? value : "", destination_size, 0);
}

static void populate_collection(sqlite3_stmt *stmt,
                                camera_collection_t *collection) {
    memset(collection, 0, sizeof(*collection));
    copy_column(collection->uuid, sizeof(collection->uuid), stmt, 0);
    copy_column(collection->name, sizeof(collection->name), stmt, 1);
    copy_column(collection->description, sizeof(collection->description), stmt, 2);
    copy_column(collection->collection_type,
                sizeof(collection->collection_type), stmt, 3);
    copy_column(collection->selector_json,
                sizeof(collection->selector_json), stmt, 4);
    collection->is_shared = sqlite3_column_int(stmt, 5) != 0;
    collection->owner_user_id = sqlite3_column_int64(stmt, 6);
    collection->created_at = sqlite3_column_int64(stmt, 7);
    collection->updated_at = sqlite3_column_int64(stmt, 8);
    collection->member_count = sqlite3_column_int(stmt, 9);
}

static db_camera_collection_result_t get_locked(
    sqlite3 *db, const char *uuid, camera_collection_t *collection) {
    const char *sql =
        "SELECT " COLLECTION_SELECT_FIELDS
        "FROM camera_collections c WHERE c.uuid = ? LIMIT 1;";
    sqlite3_stmt *stmt = NULL;
    int rc = sqlite3_prepare_v2(db, sql, -1, &stmt, NULL);
    if (rc != SQLITE_OK) return DB_CAMERA_COLLECTION_ERROR;
    sqlite3_bind_text(stmt, 1, uuid, -1, SQLITE_TRANSIENT);
    db_camera_collection_result_t result = DB_CAMERA_COLLECTION_NOT_FOUND;
    rc = sqlite3_step(stmt);
    if (rc == SQLITE_ROW) {
        populate_collection(stmt, collection);
        result = DB_CAMERA_COLLECTION_OK;
    } else if (rc != SQLITE_DONE) {
        result = DB_CAMERA_COLLECTION_ERROR;
    }
    sqlite3_finalize(stmt);
    return result;
}

static bool row_exists_locked(sqlite3 *db, const char *sql, const char *value) {
    sqlite3_stmt *stmt = NULL;
    if (sqlite3_prepare_v2(db, sql, -1, &stmt, NULL) != SQLITE_OK) return false;
    sqlite3_bind_text(stmt, 1, value, -1, SQLITE_TRANSIENT);
    bool exists = sqlite3_step(stmt) == SQLITE_ROW;
    sqlite3_finalize(stmt);
    return exists;
}

int db_camera_collection_count(void) {
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    if (!db) return -1;
    pthread_mutex_lock(mutex);
    sqlite3_stmt *stmt = NULL;
    int count = -1;
    if (sqlite3_prepare_v2(db, "SELECT count(*) FROM camera_collections;", -1,
                           &stmt, NULL) == SQLITE_OK &&
        sqlite3_step(stmt) == SQLITE_ROW) {
        count = sqlite3_column_int(stmt, 0);
    }
    if (stmt) sqlite3_finalize(stmt);
    pthread_mutex_unlock(mutex);
    return count;
}

int db_camera_collection_list(camera_collection_t *collections, int max_count) {
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    if (!db || !collections || max_count <= 0) return -1;
    const char *sql =
        "SELECT " COLLECTION_SELECT_FIELDS
        "FROM camera_collections c ORDER BY c.name COLLATE NOCASE;";
    pthread_mutex_lock(mutex);
    sqlite3_stmt *stmt = NULL;
    int rc = sqlite3_prepare_v2(db, sql, -1, &stmt, NULL);
    if (rc != SQLITE_OK) {
        pthread_mutex_unlock(mutex);
        return -1;
    }
    int count = 0;
    while (count < max_count && (rc = sqlite3_step(stmt)) == SQLITE_ROW) {
        populate_collection(stmt, &collections[count++]);
    }
    if (rc != SQLITE_ROW && rc != SQLITE_DONE) count = -1;
    sqlite3_finalize(stmt);
    pthread_mutex_unlock(mutex);
    return count;
}

db_camera_collection_result_t db_camera_collection_get(
    const char *uuid, camera_collection_t *collection) {
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    if (!db || !collection || !valid_uuid(uuid)) {
        return DB_CAMERA_COLLECTION_INVALID;
    }
    pthread_mutex_lock(mutex);
    db_camera_collection_result_t result = get_locked(db, uuid, collection);
    pthread_mutex_unlock(mutex);
    return result;
}

db_camera_collection_result_t db_camera_collection_create(
    camera_collection_t *collection) {
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    char normalized_name[CAMERA_COLLECTION_NAME_MAX];
    if (!db || !valid_collection(collection, normalized_name,
                                 sizeof(normalized_name))) {
        return DB_CAMERA_COLLECTION_INVALID;
    }
    if (strcmp(collection->collection_type, "static") == 0) {
        collection->selector_json[0] = '\0';
    }
    const char *sql =
        "INSERT INTO camera_collections "
        "(uuid, name, description, collection_type, selector_json, is_shared, "
        " owner_user_id) VALUES ("
        "lower(hex(randomblob(4)) || '-' || hex(randomblob(2)) || '-4' || "
        "substr(hex(randomblob(2)), 2) || '-' || "
        "substr('89ab', (abs(random()) % 4) + 1, 1) || "
        "substr(hex(randomblob(2)), 2) || '-' || hex(randomblob(6))), "
        "?, ?, ?, ?, ?, ?);";
    pthread_mutex_lock(mutex);
    sqlite3_stmt *stmt = NULL;
    int rc = sqlite3_prepare_v2(db, sql, -1, &stmt, NULL);
    if (rc == SQLITE_OK) {
        sqlite3_bind_text(stmt, 1, normalized_name, -1, SQLITE_TRANSIENT);
        sqlite3_bind_text(stmt, 2, collection->description, -1,
                          SQLITE_TRANSIENT);
        sqlite3_bind_text(stmt, 3, collection->collection_type, -1,
                          SQLITE_TRANSIENT);
        sqlite3_bind_text(stmt, 4, collection->selector_json, -1,
                          SQLITE_TRANSIENT);
        sqlite3_bind_int(stmt, 5, collection->is_shared ? 1 : 0);
        if (collection->owner_user_id > 0) {
            sqlite3_bind_int64(stmt, 6, collection->owner_user_id);
        } else {
            sqlite3_bind_null(stmt, 6);
        }
        rc = sqlite3_step(stmt);
    }
    if (stmt) {
        sqlite3_finalize(stmt);
        stmt = NULL;
    }
    if (rc != SQLITE_DONE) {
        db_camera_collection_result_t result =
            rc == SQLITE_CONSTRAINT ? DB_CAMERA_COLLECTION_CONFLICT
                                    : DB_CAMERA_COLLECTION_ERROR;
        pthread_mutex_unlock(mutex);
        return result;
    }
    char row_id[32];
    snprintf(row_id, sizeof(row_id), "%lld",
             (long long)sqlite3_last_insert_rowid(db));
    rc = sqlite3_prepare_v2(db,
                            "SELECT uuid FROM camera_collections "
                            "WHERE id = ? LIMIT 1;", -1, &stmt, NULL);
    if (rc == SQLITE_OK) {
        sqlite3_bind_text(stmt, 1, row_id, -1, SQLITE_TRANSIENT);
        rc = sqlite3_step(stmt);
        if (rc == SQLITE_ROW) {
            char uuid[CAMERA_UUID_STRING_SIZE];
            copy_column(uuid, sizeof(uuid), stmt, 0);
            sqlite3_finalize(stmt);
            stmt = NULL;
            db_camera_collection_result_t result =
                get_locked(db, uuid, collection);
            pthread_mutex_unlock(mutex);
            return result;
        }
    }
    if (stmt) sqlite3_finalize(stmt);
    pthread_mutex_unlock(mutex);
    return DB_CAMERA_COLLECTION_ERROR;
}

db_camera_collection_result_t db_camera_collection_update(
    camera_collection_t *collection) {
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    char normalized_name[CAMERA_COLLECTION_NAME_MAX];
    if (!db || !collection || !valid_uuid(collection->uuid) ||
        !valid_collection(collection, normalized_name, sizeof(normalized_name))) {
        return DB_CAMERA_COLLECTION_INVALID;
    }
    if (strcmp(collection->collection_type, "static") == 0) {
        collection->selector_json[0] = '\0';
    }

    pthread_mutex_lock(mutex);
    camera_collection_t existing;
    db_camera_collection_result_t result =
        get_locked(db, collection->uuid, &existing);
    if (result != DB_CAMERA_COLLECTION_OK) {
        pthread_mutex_unlock(mutex);
        return result;
    }
    if (!transaction_begin(db)) {
        pthread_mutex_unlock(mutex);
        return DB_CAMERA_COLLECTION_ERROR;
    }
    const char *sql =
        "UPDATE camera_collections SET name = ?, description = ?, "
        "collection_type = ?, selector_json = ?, is_shared = ?, "
        "updated_at = strftime('%s', 'now') WHERE uuid = ?;";
    sqlite3_stmt *stmt = NULL;
    int rc = sqlite3_prepare_v2(db, sql, -1, &stmt, NULL);
    if (rc == SQLITE_OK) {
        sqlite3_bind_text(stmt, 1, normalized_name, -1, SQLITE_TRANSIENT);
        sqlite3_bind_text(stmt, 2, collection->description, -1,
                          SQLITE_TRANSIENT);
        sqlite3_bind_text(stmt, 3, collection->collection_type, -1,
                          SQLITE_TRANSIENT);
        sqlite3_bind_text(stmt, 4, collection->selector_json, -1,
                          SQLITE_TRANSIENT);
        sqlite3_bind_int(stmt, 5, collection->is_shared ? 1 : 0);
        sqlite3_bind_text(stmt, 6, collection->uuid, -1, SQLITE_TRANSIENT);
        rc = sqlite3_step(stmt);
    }
    if (stmt) {
        sqlite3_finalize(stmt);
        stmt = NULL;
    }
    if (rc == SQLITE_DONE && strcmp(collection->collection_type, "smart") == 0) {
        rc = sqlite3_prepare_v2(
            db, "DELETE FROM camera_collection_members WHERE collection_uuid = ?;",
            -1, &stmt, NULL);
        if (rc == SQLITE_OK) {
            sqlite3_bind_text(stmt, 1, collection->uuid, -1, SQLITE_TRANSIENT);
            rc = sqlite3_step(stmt);
        }
        if (stmt) {
            sqlite3_finalize(stmt);
            stmt = NULL;
        }
    }
    bool success = transaction_finish(db, rc == SQLITE_DONE);
    if (!success) {
        result = rc == SQLITE_CONSTRAINT ? DB_CAMERA_COLLECTION_CONFLICT
                                        : DB_CAMERA_COLLECTION_ERROR;
        pthread_mutex_unlock(mutex);
        return result;
    }
    result = get_locked(db, collection->uuid, collection);
    pthread_mutex_unlock(mutex);
    return result;
}

db_camera_collection_result_t db_camera_collection_delete(const char *uuid) {
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    if (!db || !valid_uuid(uuid)) return DB_CAMERA_COLLECTION_INVALID;
    pthread_mutex_lock(mutex);
    if (!row_exists_locked(db,
                           "SELECT 1 FROM camera_collections WHERE uuid = ?;",
                           uuid)) {
        pthread_mutex_unlock(mutex);
        return DB_CAMERA_COLLECTION_NOT_FOUND;
    }
    sqlite3_stmt *stmt = NULL;
    int rc = sqlite3_prepare_v2(
        db, "DELETE FROM camera_collections WHERE uuid = ?;", -1, &stmt, NULL);
    if (rc == SQLITE_OK) {
        sqlite3_bind_text(stmt, 1, uuid, -1, SQLITE_TRANSIENT);
        rc = sqlite3_step(stmt);
    }
    if (stmt) sqlite3_finalize(stmt);
    pthread_mutex_unlock(mutex);
    return rc == SQLITE_DONE ? DB_CAMERA_COLLECTION_OK
                             : DB_CAMERA_COLLECTION_ERROR;
}

int db_camera_collection_list_members(
    const char *collection_uuid,
    char camera_uuids[][CAMERA_UUID_STRING_SIZE], int max_count) {
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    if (!db || !valid_uuid(collection_uuid) || !camera_uuids || max_count <= 0) {
        return -1;
    }
    pthread_mutex_lock(mutex);
    camera_collection_t collection;
    db_camera_collection_result_t result =
        get_locked(db, collection_uuid, &collection);
    if (result == DB_CAMERA_COLLECTION_NOT_FOUND) {
        pthread_mutex_unlock(mutex);
        return -2;
    }
    if (result != DB_CAMERA_COLLECTION_OK) {
        pthread_mutex_unlock(mutex);
        return -1;
    }
    if (strcmp(collection.collection_type, "static") != 0) {
        pthread_mutex_unlock(mutex);
        return -3;
    }
    sqlite3_stmt *stmt = NULL;
    int rc = sqlite3_prepare_v2(
        db,
        "SELECT camera_uuid FROM camera_collection_members "
        "WHERE collection_uuid = ? ORDER BY camera_uuid;",
        -1, &stmt, NULL);
    if (rc != SQLITE_OK) {
        pthread_mutex_unlock(mutex);
        return -1;
    }
    sqlite3_bind_text(stmt, 1, collection_uuid, -1, SQLITE_TRANSIENT);
    int count = 0;
    while (count < max_count && (rc = sqlite3_step(stmt)) == SQLITE_ROW) {
        copy_column(camera_uuids[count], CAMERA_UUID_STRING_SIZE, stmt, 0);
        count++;
    }
    if (rc != SQLITE_ROW && rc != SQLITE_DONE) count = -1;
    sqlite3_finalize(stmt);
    pthread_mutex_unlock(mutex);
    return count;
}

db_camera_collection_result_t db_camera_collection_set_members(
    const char *collection_uuid, const char *const *camera_uuids,
    int camera_count) {
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    if (!db || !valid_uuid(collection_uuid) || camera_count < 0 ||
        camera_count > CAMERA_COLLECTION_MAX_MEMBERS ||
        (camera_count > 0 && !camera_uuids)) {
        return camera_count > CAMERA_COLLECTION_MAX_MEMBERS ?
               DB_CAMERA_COLLECTION_LIMIT : DB_CAMERA_COLLECTION_INVALID;
    }
    pthread_mutex_lock(mutex);
    camera_collection_t collection;
    db_camera_collection_result_t result =
        get_locked(db, collection_uuid, &collection);
    if (result != DB_CAMERA_COLLECTION_OK) {
        pthread_mutex_unlock(mutex);
        return result;
    }
    if (strcmp(collection.collection_type, "static") != 0) {
        pthread_mutex_unlock(mutex);
        return DB_CAMERA_COLLECTION_WRONG_TYPE;
    }
    for (int i = 0; i < camera_count; i++) {
        if (!valid_uuid(camera_uuids[i]) ||
            !row_exists_locked(db,
                               "SELECT 1 FROM streams WHERE camera_uuid = ?;",
                               camera_uuids[i])) {
            pthread_mutex_unlock(mutex);
            return DB_CAMERA_COLLECTION_NOT_FOUND;
        }
    }
    if (!transaction_begin(db)) {
        pthread_mutex_unlock(mutex);
        return DB_CAMERA_COLLECTION_ERROR;
    }
    sqlite3_stmt *stmt = NULL;
    int rc = sqlite3_prepare_v2(
        db, "DELETE FROM camera_collection_members WHERE collection_uuid = ?;",
        -1, &stmt, NULL);
    if (rc == SQLITE_OK) {
        sqlite3_bind_text(stmt, 1, collection_uuid, -1, SQLITE_TRANSIENT);
        rc = sqlite3_step(stmt);
    }
    if (stmt) {
        sqlite3_finalize(stmt);
        stmt = NULL;
    }
    for (int i = 0; rc == SQLITE_DONE && i < camera_count; i++) {
        rc = sqlite3_prepare_v2(
            db,
            "INSERT OR IGNORE INTO camera_collection_members "
            "(collection_uuid, camera_uuid) VALUES (?, ?);",
            -1, &stmt, NULL);
        if (rc != SQLITE_OK) break;
        sqlite3_bind_text(stmt, 1, collection_uuid, -1, SQLITE_TRANSIENT);
        sqlite3_bind_text(stmt, 2, camera_uuids[i], -1, SQLITE_TRANSIENT);
        rc = sqlite3_step(stmt);
        sqlite3_finalize(stmt);
        stmt = NULL;
    }
    if (stmt) sqlite3_finalize(stmt);
    bool success = transaction_finish(db, rc == SQLITE_DONE);
    pthread_mutex_unlock(mutex);
    return success ? DB_CAMERA_COLLECTION_OK : DB_CAMERA_COLLECTION_ERROR;
}
