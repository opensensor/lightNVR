#define _POSIX_C_SOURCE 200809L

#include <pthread.h>
#include <sqlite3.h>
#include <stdbool.h>
#include <ctype.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "core/logger.h"
#include "database/db_camera_tags.h"
#include "database/db_core.h"
#include "utils/strings.h"

#define TAG_SELECT_FIELDS \
    "t.uuid, t.label, t.color, t.description, t.created_at, t.updated_at, " \
    "(SELECT count(*) FROM camera_tag_assignments a WHERE a.tag_uuid = t.uuid) "

static bool valid_uuid_string(const char *uuid) {
    return uuid && strlen(uuid) == CAMERA_UUID_STRING_SIZE - 1;
}

static bool normalize_label(const char *input, char *output,
                            size_t output_size) {
    if (!input || copy_trimmed_value(output, output_size, input, 0) == 0 ||
        strchr(output, ',') != NULL) {
        return false;
    }
    for (const unsigned char *p = (const unsigned char *)output; *p; p++) {
        if (iscntrl(*p)) return false;
    }
    return true;
}

static bool transaction_begin(sqlite3 *db, bool *owns_transaction) {
    *owns_transaction = sqlite3_get_autocommit(db) != 0;
    if (!*owns_transaction) return true;
    return sqlite3_exec(db, "BEGIN IMMEDIATE;", NULL, NULL, NULL) == SQLITE_OK;
}

static bool transaction_finish(sqlite3 *db, bool owns_transaction,
                               bool success) {
    if (!owns_transaction) return success;
    const char *sql = success ? "COMMIT;" : "ROLLBACK;";
    return sqlite3_exec(db, sql, NULL, NULL, NULL) == SQLITE_OK && success;
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

static void populate_tag(sqlite3_stmt *stmt, camera_tag_t *tag) {
    memset(tag, 0, sizeof(*tag));
    copy_column(tag->uuid, sizeof(tag->uuid), stmt, 0);
    copy_column(tag->label, sizeof(tag->label), stmt, 1);
    copy_column(tag->color, sizeof(tag->color), stmt, 2);
    copy_column(tag->description, sizeof(tag->description), stmt, 3);
    tag->created_at = sqlite3_column_int64(stmt, 4);
    tag->updated_at = sqlite3_column_int64(stmt, 5);
    tag->camera_count = sqlite3_column_int(stmt, 6);
}

static db_camera_tag_result_t get_locked(sqlite3 *db, const char *uuid,
                                         camera_tag_t *tag) {
    const char *sql =
        "SELECT " TAG_SELECT_FIELDS
        "FROM camera_tags t WHERE t.uuid = ? LIMIT 1;";
    sqlite3_stmt *stmt = NULL;
    int rc = sqlite3_prepare_v2(db, sql, -1, &stmt, NULL);
    if (rc != SQLITE_OK) return DB_CAMERA_TAG_ERROR;
    sqlite3_bind_text(stmt, 1, uuid, -1, SQLITE_TRANSIENT);
    db_camera_tag_result_t result = DB_CAMERA_TAG_NOT_FOUND;
    rc = sqlite3_step(stmt);
    if (rc == SQLITE_ROW) {
        populate_tag(stmt, tag);
        result = DB_CAMERA_TAG_OK;
    } else if (rc != SQLITE_DONE) {
        result = DB_CAMERA_TAG_ERROR;
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

static int ensure_tag_for_label_locked(sqlite3 *db, const char *label,
                                       char *tag_uuid, size_t tag_uuid_size) {
    const char *insert_sql =
        "INSERT OR IGNORE INTO camera_tags (uuid, label) VALUES ("
        "lower(hex(randomblob(4)) || '-' || hex(randomblob(2)) || '-4' || "
        "substr(hex(randomblob(2)), 2) || '-' || "
        "substr('89ab', (abs(random()) % 4) + 1, 1) || "
        "substr(hex(randomblob(2)), 2) || '-' || hex(randomblob(6))), ?);";
    sqlite3_stmt *stmt = NULL;
    int rc = sqlite3_prepare_v2(db, insert_sql, -1, &stmt, NULL);
    if (rc != SQLITE_OK) return -1;
    sqlite3_bind_text(stmt, 1, label, -1, SQLITE_TRANSIENT);
    rc = sqlite3_step(stmt);
    sqlite3_finalize(stmt);
    if (rc != SQLITE_DONE) return -1;

    rc = sqlite3_prepare_v2(db,
                            "SELECT uuid FROM camera_tags "
                            "WHERE label = ? COLLATE NOCASE LIMIT 1;",
                            -1, &stmt, NULL);
    if (rc != SQLITE_OK) return -1;
    sqlite3_bind_text(stmt, 1, label, -1, SQLITE_TRANSIENT);
    rc = sqlite3_step(stmt);
    if (rc == SQLITE_ROW) {
        const char *uuid = (const char *)sqlite3_column_text(stmt, 0);
        safe_strcpy(tag_uuid, uuid, tag_uuid_size, 0);
    }
    sqlite3_finalize(stmt);
    return rc == SQLITE_ROW ? 0 : -1;
}

static int rebuild_legacy_for_camera_locked(sqlite3 *db,
                                            const char *camera_uuid) {
    const char *select_sql =
        "SELECT t.label FROM camera_tag_assignments a "
        "JOIN camera_tags t ON t.uuid = a.tag_uuid "
        "WHERE a.camera_uuid = ? ORDER BY t.label COLLATE NOCASE;";
    sqlite3_stmt *stmt = NULL;
    int rc = sqlite3_prepare_v2(db, select_sql, -1, &stmt, NULL);
    if (rc != SQLITE_OK) return -1;
    sqlite3_bind_text(stmt, 1, camera_uuid, -1, SQLITE_TRANSIENT);

    char legacy_tags[256] = {0};
    size_t used = 0;
    while ((rc = sqlite3_step(stmt)) == SQLITE_ROW) {
        const char *label = (const char *)sqlite3_column_text(stmt, 0);
        size_t label_length = label ? strlen(label) : 0;
        size_t required = label_length + (used > 0 ? 1 : 0);
        if (!label || label_length == 0 || used + required >= sizeof(legacy_tags)) {
            sqlite3_finalize(stmt);
            return -2;
        }
        if (used > 0) legacy_tags[used++] = ',';
        memcpy(legacy_tags + used, label, label_length);
        used += label_length;
        legacy_tags[used] = '\0';
    }
    sqlite3_finalize(stmt);
    if (rc != SQLITE_DONE) return -1;

    rc = sqlite3_prepare_v2(db,
                            "UPDATE streams SET tags = ? WHERE camera_uuid = ?;",
                            -1, &stmt, NULL);
    if (rc != SQLITE_OK) return -1;
    sqlite3_bind_text(stmt, 1, legacy_tags, -1, SQLITE_TRANSIENT);
    sqlite3_bind_text(stmt, 2, camera_uuid, -1, SQLITE_TRANSIENT);
    rc = sqlite3_step(stmt);
    sqlite3_finalize(stmt);
    return rc == SQLITE_DONE ? 0 : -1;
}

static int rebuild_all_legacy_locked(sqlite3 *db) {
    sqlite3_stmt *stmt = NULL;
    int rc = sqlite3_prepare_v2(db, "SELECT camera_uuid FROM streams;",
                                -1, &stmt, NULL);
    if (rc != SQLITE_OK) return -1;

    int camera_count = 0;
    int camera_capacity = 32;
    char (*camera_uuids)[CAMERA_UUID_STRING_SIZE] =
        calloc((size_t)camera_capacity, sizeof(*camera_uuids));
    if (!camera_uuids) {
        sqlite3_finalize(stmt);
        return -1;
    }
    while ((rc = sqlite3_step(stmt)) == SQLITE_ROW) {
        if (camera_count == camera_capacity) {
            int new_capacity = camera_capacity * 2;
            void *resized = realloc(camera_uuids,
                                    (size_t)new_capacity * sizeof(*camera_uuids));
            if (!resized) {
                free(camera_uuids);
                sqlite3_finalize(stmt);
                return -1;
            }
            camera_uuids = resized;
            camera_capacity = new_capacity;
        }
        safe_strcpy(camera_uuids[camera_count],
                    (const char *)sqlite3_column_text(stmt, 0),
                    CAMERA_UUID_STRING_SIZE, 0);
        camera_count++;
    }
    sqlite3_finalize(stmt);
    if (rc != SQLITE_DONE) {
        free(camera_uuids);
        return -1;
    }

    int result = 0;
    for (int i = 0; i < camera_count; i++) {
        int rebuild_rc = rebuild_legacy_for_camera_locked(db, camera_uuids[i]);
        if (rebuild_rc != 0) {
            result = rebuild_rc;
            break;
        }
    }
    free(camera_uuids);
    return result;
}

static int sync_legacy_locked(sqlite3 *db, const char *camera_uuid,
                              const char *legacy_tags) {
    sqlite3_stmt *stmt = NULL;
    int rc = sqlite3_prepare_v2(
        db, "DELETE FROM camera_tag_assignments WHERE camera_uuid = ?;",
        -1, &stmt, NULL);
    if (rc != SQLITE_OK) return -1;
    sqlite3_bind_text(stmt, 1, camera_uuid, -1, SQLITE_TRANSIENT);
    rc = sqlite3_step(stmt);
    sqlite3_finalize(stmt);
    if (rc != SQLITE_DONE) return -1;

    if (!legacy_tags || legacy_tags[0] == '\0') return 0;
    char tags_copy[256];
    safe_strcpy(tags_copy, legacy_tags, sizeof(tags_copy), 0);
    char *saveptr = NULL;
    for (char *token = strtok_r(tags_copy, ",", &saveptr);
         token != NULL;
         token = strtok_r(NULL, ",", &saveptr)) {
        char label[CAMERA_TAG_LABEL_MAX];
        if (!normalize_label(token, label, sizeof(label))) continue;

        char tag_uuid[CAMERA_UUID_STRING_SIZE];
        if (ensure_tag_for_label_locked(db, label, tag_uuid,
                                        sizeof(tag_uuid)) != 0) {
            return -1;
        }

        rc = sqlite3_prepare_v2(
            db,
            "INSERT OR IGNORE INTO camera_tag_assignments "
            "(camera_uuid, tag_uuid) VALUES (?, ?);",
            -1, &stmt, NULL);
        if (rc != SQLITE_OK) return -1;
        sqlite3_bind_text(stmt, 1, camera_uuid, -1, SQLITE_TRANSIENT);
        sqlite3_bind_text(stmt, 2, tag_uuid, -1, SQLITE_TRANSIENT);
        rc = sqlite3_step(stmt);
        sqlite3_finalize(stmt);
        if (rc != SQLITE_DONE) return -1;
    }
    return 0;
}

int db_camera_tags_sync_legacy_by_name_locked(sqlite3 *db,
                                              const char *stream_name,
                                              const char *legacy_tags) {
    if (!db || !stream_name) return -1;
    sqlite3_stmt *stmt = NULL;
    int rc = sqlite3_prepare_v2(db,
                                "SELECT camera_uuid FROM streams "
                                "WHERE name = ? LIMIT 1;",
                                -1, &stmt, NULL);
    if (rc != SQLITE_OK) return -1;
    sqlite3_bind_text(stmt, 1, stream_name, -1, SQLITE_TRANSIENT);
    rc = sqlite3_step(stmt);
    char camera_uuid[CAMERA_UUID_STRING_SIZE] = {0};
    if (rc == SQLITE_ROW) {
        const char *value = (const char *)sqlite3_column_text(stmt, 0);
        safe_strcpy(camera_uuid, value, sizeof(camera_uuid), 0);
    }
    sqlite3_finalize(stmt);
    if (rc != SQLITE_ROW || !valid_uuid_string(camera_uuid)) return -1;
    bool owns_transaction = false;
    if (!transaction_begin(db, &owns_transaction)) return -1;
    int result = sync_legacy_locked(db, camera_uuid, legacy_tags);
    return transaction_finish(db, owns_transaction, result == 0) ? 0 : -1;
}

int db_camera_tags_backfill_legacy(void) {
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    if (!db) return -1;

    pthread_mutex_lock(mutex);
    bool owns_transaction = false;
    if (!transaction_begin(db, &owns_transaction)) {
        pthread_mutex_unlock(mutex);
        return -1;
    }

    sqlite3_stmt *stmt = NULL;
    int rc = sqlite3_prepare_v2(db,
                                "SELECT camera_uuid, tags FROM streams;",
                                -1, &stmt, NULL);
    int result = rc == SQLITE_OK ? 0 : -1;
    while (result == 0 && (rc = sqlite3_step(stmt)) == SQLITE_ROW) {
        char camera_uuid[CAMERA_UUID_STRING_SIZE];
        char legacy_tags[256];
        safe_strcpy(camera_uuid,
                    (const char *)sqlite3_column_text(stmt, 0),
                    sizeof(camera_uuid), 0);
        const char *legacy_value =
            (const char *)sqlite3_column_text(stmt, 1);
        safe_strcpy(legacy_tags, legacy_value ? legacy_value : "",
                    sizeof(legacy_tags), 0);
        if (sync_legacy_locked(db, camera_uuid, legacy_tags) != 0) result = -1;
    }
    if (stmt) sqlite3_finalize(stmt);
    if (rc != SQLITE_DONE && result == 0) result = -1;
    bool success = transaction_finish(db, owns_transaction, result == 0);
    pthread_mutex_unlock(mutex);
    if (!success) {
        log_error("Failed to backfill normalized camera tags: %s",
                  sqlite3_errmsg(db));
        return -1;
    }
    return 0;
}

int db_camera_tag_count(void) {
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    if (!db) return -1;
    pthread_mutex_lock(mutex);
    sqlite3_stmt *stmt = NULL;
    int count = -1;
    if (sqlite3_prepare_v2(db, "SELECT count(*) FROM camera_tags;", -1,
                           &stmt, NULL) == SQLITE_OK &&
        sqlite3_step(stmt) == SQLITE_ROW) {
        count = sqlite3_column_int(stmt, 0);
    }
    if (stmt) sqlite3_finalize(stmt);
    pthread_mutex_unlock(mutex);
    return count;
}

int db_camera_tag_list(camera_tag_t *tags, int max_count) {
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    if (!db || !tags || max_count <= 0) return -1;
    const char *sql =
        "SELECT " TAG_SELECT_FIELDS
        "FROM camera_tags t ORDER BY t.label COLLATE NOCASE;";
    pthread_mutex_lock(mutex);
    sqlite3_stmt *stmt = NULL;
    int rc = sqlite3_prepare_v2(db, sql, -1, &stmt, NULL);
    if (rc != SQLITE_OK) {
        pthread_mutex_unlock(mutex);
        return -1;
    }
    int count = 0;
    while (count < max_count && (rc = sqlite3_step(stmt)) == SQLITE_ROW) {
        populate_tag(stmt, &tags[count++]);
    }
    if (rc != SQLITE_ROW && rc != SQLITE_DONE) count = -1;
    sqlite3_finalize(stmt);
    pthread_mutex_unlock(mutex);
    return count;
}

db_camera_tag_result_t db_camera_tag_get(const char *uuid, camera_tag_t *tag) {
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    if (!db || !tag || !valid_uuid_string(uuid)) return DB_CAMERA_TAG_INVALID;
    pthread_mutex_lock(mutex);
    db_camera_tag_result_t result = get_locked(db, uuid, tag);
    pthread_mutex_unlock(mutex);
    return result;
}

db_camera_tag_result_t db_camera_tag_create(camera_tag_t *tag) {
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    if (!db || !tag) return DB_CAMERA_TAG_INVALID;
    char label[CAMERA_TAG_LABEL_MAX];
    if (!normalize_label(tag->label, label, sizeof(label))) {
        return DB_CAMERA_TAG_INVALID;
    }

    const char *sql =
        "INSERT INTO camera_tags (uuid, label, color, description) VALUES ("
        "lower(hex(randomblob(4)) || '-' || hex(randomblob(2)) || '-4' || "
        "substr(hex(randomblob(2)), 2) || '-' || "
        "substr('89ab', (abs(random()) % 4) + 1, 1) || "
        "substr(hex(randomblob(2)), 2) || '-' || hex(randomblob(6))), ?, ?, ?);";
    pthread_mutex_lock(mutex);
    sqlite3_stmt *stmt = NULL;
    int rc = sqlite3_prepare_v2(db, sql, -1, &stmt, NULL);
    if (rc == SQLITE_OK) {
        sqlite3_bind_text(stmt, 1, label, -1, SQLITE_TRANSIENT);
        sqlite3_bind_text(stmt, 2, tag->color, -1, SQLITE_TRANSIENT);
        sqlite3_bind_text(stmt, 3, tag->description, -1, SQLITE_TRANSIENT);
        rc = sqlite3_step(stmt);
    }
    if (stmt) sqlite3_finalize(stmt);
    if (rc != SQLITE_DONE) {
        db_camera_tag_result_t result =
            (rc == SQLITE_CONSTRAINT) ? DB_CAMERA_TAG_CONFLICT
                                      : DB_CAMERA_TAG_ERROR;
        pthread_mutex_unlock(mutex);
        return result;
    }
    char row_id[32];
    snprintf(row_id, sizeof(row_id), "%lld",
             (long long)sqlite3_last_insert_rowid(db));
    rc = sqlite3_prepare_v2(
        db, "SELECT uuid FROM camera_tags WHERE id = ? LIMIT 1;",
        -1, &stmt, NULL);
    if (rc == SQLITE_OK) {
        sqlite3_bind_text(stmt, 1, row_id, -1, SQLITE_TRANSIENT);
        rc = sqlite3_step(stmt);
        if (rc == SQLITE_ROW) {
            char uuid[CAMERA_UUID_STRING_SIZE];
            safe_strcpy(uuid, (const char *)sqlite3_column_text(stmt, 0),
                        sizeof(uuid), 0);
            sqlite3_finalize(stmt);
            stmt = NULL;
            db_camera_tag_result_t result = get_locked(db, uuid, tag);
            pthread_mutex_unlock(mutex);
            return result;
        }
    }
    if (stmt) sqlite3_finalize(stmt);
    pthread_mutex_unlock(mutex);
    return DB_CAMERA_TAG_ERROR;
}

db_camera_tag_result_t db_camera_tag_update(camera_tag_t *tag) {
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    if (!db || !tag || !valid_uuid_string(tag->uuid)) {
        return DB_CAMERA_TAG_INVALID;
    }
    char label[CAMERA_TAG_LABEL_MAX];
    if (!normalize_label(tag->label, label, sizeof(label))) {
        return DB_CAMERA_TAG_INVALID;
    }

    pthread_mutex_lock(mutex);
    if (!row_exists_locked(db, "SELECT 1 FROM camera_tags WHERE uuid = ?;",
                           tag->uuid)) {
        pthread_mutex_unlock(mutex);
        return DB_CAMERA_TAG_NOT_FOUND;
    }
    bool owns_transaction = false;
    if (!transaction_begin(db, &owns_transaction)) {
        pthread_mutex_unlock(mutex);
        return DB_CAMERA_TAG_ERROR;
    }
    sqlite3_stmt *stmt = NULL;
    int rc = sqlite3_prepare_v2(
        db,
        "UPDATE camera_tags SET label = ?, color = ?, description = ?, "
        "updated_at = strftime('%s', 'now') WHERE uuid = ?;",
        -1, &stmt, NULL);
    if (rc == SQLITE_OK) {
        sqlite3_bind_text(stmt, 1, label, -1, SQLITE_TRANSIENT);
        sqlite3_bind_text(stmt, 2, tag->color, -1, SQLITE_TRANSIENT);
        sqlite3_bind_text(stmt, 3, tag->description, -1, SQLITE_TRANSIENT);
        sqlite3_bind_text(stmt, 4, tag->uuid, -1, SQLITE_TRANSIENT);
        rc = sqlite3_step(stmt);
    }
    if (stmt) sqlite3_finalize(stmt);
    int rebuild_rc = rc == SQLITE_DONE ? rebuild_all_legacy_locked(db) : -1;
    bool success = rc == SQLITE_DONE && rebuild_rc == 0;
    success = transaction_finish(db, owns_transaction, success);
    if (!success) {
        db_camera_tag_result_t result =
            (rc == SQLITE_CONSTRAINT) ? DB_CAMERA_TAG_CONFLICT :
            (rebuild_rc == -2 ? DB_CAMERA_TAG_LIMIT : DB_CAMERA_TAG_ERROR);
        pthread_mutex_unlock(mutex);
        return result;
    }
    db_camera_tag_result_t result = get_locked(db, tag->uuid, tag);
    pthread_mutex_unlock(mutex);
    return result;
}

db_camera_tag_result_t db_camera_tag_delete(const char *uuid) {
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    if (!db || !valid_uuid_string(uuid)) return DB_CAMERA_TAG_INVALID;
    pthread_mutex_lock(mutex);
    if (!row_exists_locked(db, "SELECT 1 FROM camera_tags WHERE uuid = ?;", uuid)) {
        pthread_mutex_unlock(mutex);
        return DB_CAMERA_TAG_NOT_FOUND;
    }
    bool owns_transaction = false;
    if (!transaction_begin(db, &owns_transaction)) {
        pthread_mutex_unlock(mutex);
        return DB_CAMERA_TAG_ERROR;
    }
    sqlite3_stmt *stmt = NULL;
    int rc = sqlite3_prepare_v2(db, "DELETE FROM camera_tags WHERE uuid = ?;",
                                -1, &stmt, NULL);
    if (rc == SQLITE_OK) {
        sqlite3_bind_text(stmt, 1, uuid, -1, SQLITE_TRANSIENT);
        rc = sqlite3_step(stmt);
    }
    if (stmt) sqlite3_finalize(stmt);
    bool success = rc == SQLITE_DONE && rebuild_all_legacy_locked(db) == 0;
    success = transaction_finish(db, owns_transaction, success);
    pthread_mutex_unlock(mutex);
    return success ? DB_CAMERA_TAG_OK : DB_CAMERA_TAG_ERROR;
}

db_camera_tag_result_t db_camera_tag_merge(const char *source_uuid,
                                           const char *target_uuid) {
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    if (!db || !valid_uuid_string(source_uuid) ||
        !valid_uuid_string(target_uuid) || strcmp(source_uuid, target_uuid) == 0) {
        return DB_CAMERA_TAG_INVALID;
    }
    pthread_mutex_lock(mutex);
    if (!row_exists_locked(db, "SELECT 1 FROM camera_tags WHERE uuid = ?;",
                           source_uuid) ||
        !row_exists_locked(db, "SELECT 1 FROM camera_tags WHERE uuid = ?;",
                           target_uuid)) {
        pthread_mutex_unlock(mutex);
        return DB_CAMERA_TAG_NOT_FOUND;
    }
    bool owns_transaction = false;
    if (!transaction_begin(db, &owns_transaction)) {
        pthread_mutex_unlock(mutex);
        return DB_CAMERA_TAG_ERROR;
    }
    sqlite3_stmt *stmt = NULL;
    int rc = sqlite3_prepare_v2(
        db,
        "INSERT OR IGNORE INTO camera_tag_assignments (camera_uuid, tag_uuid) "
        "SELECT camera_uuid, ? FROM camera_tag_assignments WHERE tag_uuid = ?;",
        -1, &stmt, NULL);
    if (rc == SQLITE_OK) {
        sqlite3_bind_text(stmt, 1, target_uuid, -1, SQLITE_TRANSIENT);
        sqlite3_bind_text(stmt, 2, source_uuid, -1, SQLITE_TRANSIENT);
        rc = sqlite3_step(stmt);
    }
    if (stmt) {
        sqlite3_finalize(stmt);
        stmt = NULL;
    }
    if (rc == SQLITE_DONE) {
        rc = sqlite3_prepare_v2(db, "DELETE FROM camera_tags WHERE uuid = ?;",
                                -1, &stmt, NULL);
        if (rc == SQLITE_OK) {
            sqlite3_bind_text(stmt, 1, source_uuid, -1, SQLITE_TRANSIENT);
            rc = sqlite3_step(stmt);
        }
        if (stmt) sqlite3_finalize(stmt);
    }
    bool success = rc == SQLITE_DONE && rebuild_all_legacy_locked(db) == 0;
    success = transaction_finish(db, owns_transaction, success);
    pthread_mutex_unlock(mutex);
    return success ? DB_CAMERA_TAG_OK : DB_CAMERA_TAG_ERROR;
}

int db_camera_tag_list_for_camera(const char *camera_uuid, camera_tag_t *tags,
                                  int max_count) {
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    if (!db || !tags || max_count <= 0 || !valid_uuid_string(camera_uuid)) {
        return -1;
    }
    const char *sql =
        "SELECT " TAG_SELECT_FIELDS
        "FROM camera_tags t JOIN camera_tag_assignments assigned "
        "ON assigned.tag_uuid = t.uuid WHERE assigned.camera_uuid = ? "
        "ORDER BY t.label COLLATE NOCASE;";
    pthread_mutex_lock(mutex);
    if (!row_exists_locked(db, "SELECT 1 FROM streams WHERE camera_uuid = ?;",
                           camera_uuid)) {
        pthread_mutex_unlock(mutex);
        return -2;
    }
    sqlite3_stmt *stmt = NULL;
    int rc = sqlite3_prepare_v2(db, sql, -1, &stmt, NULL);
    if (rc != SQLITE_OK) {
        pthread_mutex_unlock(mutex);
        return -1;
    }
    sqlite3_bind_text(stmt, 1, camera_uuid, -1, SQLITE_TRANSIENT);
    int count = 0;
    while (count < max_count && (rc = sqlite3_step(stmt)) == SQLITE_ROW) {
        populate_tag(stmt, &tags[count++]);
    }
    if (rc != SQLITE_ROW && rc != SQLITE_DONE) count = -1;
    sqlite3_finalize(stmt);
    pthread_mutex_unlock(mutex);
    return count;
}

db_camera_tag_result_t db_camera_tag_set_for_camera(
    const char *camera_uuid, const char *const *tag_uuids, int tag_count) {
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    if (!db || !valid_uuid_string(camera_uuid) || tag_count < 0 ||
        tag_count > CAMERA_TAG_MAX_ASSIGNMENTS || (tag_count > 0 && !tag_uuids)) {
        return DB_CAMERA_TAG_INVALID;
    }

    pthread_mutex_lock(mutex);
    if (!row_exists_locked(db, "SELECT 1 FROM streams WHERE camera_uuid = ?;",
                           camera_uuid)) {
        pthread_mutex_unlock(mutex);
        return DB_CAMERA_TAG_NOT_FOUND;
    }
    for (int i = 0; i < tag_count; i++) {
        if (!valid_uuid_string(tag_uuids[i]) ||
            !row_exists_locked(db, "SELECT 1 FROM camera_tags WHERE uuid = ?;",
                               tag_uuids[i])) {
            pthread_mutex_unlock(mutex);
            return DB_CAMERA_TAG_NOT_FOUND;
        }
    }

    bool owns_transaction = false;
    if (!transaction_begin(db, &owns_transaction)) {
        pthread_mutex_unlock(mutex);
        return DB_CAMERA_TAG_ERROR;
    }
    sqlite3_stmt *stmt = NULL;
    int rc = sqlite3_prepare_v2(
        db, "DELETE FROM camera_tag_assignments WHERE camera_uuid = ?;",
        -1, &stmt, NULL);
    if (rc == SQLITE_OK) {
        sqlite3_bind_text(stmt, 1, camera_uuid, -1, SQLITE_TRANSIENT);
        rc = sqlite3_step(stmt);
    }
    if (stmt) {
        sqlite3_finalize(stmt);
        stmt = NULL;
    }

    for (int i = 0; rc == SQLITE_DONE && i < tag_count; i++) {
        rc = sqlite3_prepare_v2(
            db,
            "INSERT OR IGNORE INTO camera_tag_assignments "
            "(camera_uuid, tag_uuid) VALUES (?, ?);",
            -1, &stmt, NULL);
        if (rc != SQLITE_OK) break;
        sqlite3_bind_text(stmt, 1, camera_uuid, -1, SQLITE_TRANSIENT);
        sqlite3_bind_text(stmt, 2, tag_uuids[i], -1, SQLITE_TRANSIENT);
        rc = sqlite3_step(stmt);
        sqlite3_finalize(stmt);
        stmt = NULL;
    }
    if (stmt) sqlite3_finalize(stmt);

    int rebuild_rc = rc == SQLITE_DONE ?
        rebuild_legacy_for_camera_locked(db, camera_uuid) : -1;
    bool success = rc == SQLITE_DONE && rebuild_rc == 0;
    success = transaction_finish(db, owns_transaction, success);
    pthread_mutex_unlock(mutex);
    if (success) return DB_CAMERA_TAG_OK;
    return rebuild_rc == -2 ? DB_CAMERA_TAG_LIMIT : DB_CAMERA_TAG_ERROR;
}
