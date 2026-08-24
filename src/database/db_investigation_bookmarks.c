#define _POSIX_C_SOURCE 200809L

#include <cjson/cJSON.h>
#include <pthread.h>
#include <sqlite3.h>
#include <stdbool.h>
#include <stdlib.h>
#include <string.h>

#include "database/db_core.h"
#include "database/db_investigation_bookmarks.h"
#include "utils/strings.h"
#include "utils/uuid.h"

#define BOOKMARK_SELECT_FIELDS \
    "b.uuid, COALESCE(b.owner_user_id, 0), b.title, b.note, " \
    "b.start_time, b.end_time, b.cursor_time, b.primary_camera_uuid, " \
    "b.filters_json, COALESCE(b.representative_result_json, ''), " \
    "b.revision, b.created_at, b.updated_at, " \
    "(SELECT count(*) FROM investigation_bookmark_cameras c " \
    " WHERE c.bookmark_uuid = b.uuid) "

static bool owner_predicate_bind(sqlite3_stmt *statement, int parameter,
                                 int64_t owner_user_id) {
    if (!statement || parameter < 1 || owner_user_id < 0) return false;
    sqlite3_bind_int64(statement, parameter, owner_user_id);
    return true;
}

static bool valid_json_object(const char *value, bool allow_empty) {
    if (!value || value[0] == '\0') return allow_empty;
    cJSON *root = cJSON_Parse(value);
    bool valid = cJSON_IsObject(root);
    cJSON_Delete(root);
    return valid;
}

static bool valid_cameras(
    const char camera_uuids[][CAMERA_UUID_STRING_SIZE], int camera_count,
    const char *primary_camera_uuid) {
    if (!camera_uuids || camera_count < 1 ||
        camera_count > INVESTIGATION_BOOKMARK_MAX_CAMERAS ||
        !lightnvr_uuid_is_valid(primary_camera_uuid)) {
        return false;
    }
    bool primary_found = false;
    for (int i = 0; i < camera_count; i++) {
        if (!lightnvr_uuid_is_valid(camera_uuids[i])) return false;
        if (strcmp(camera_uuids[i], primary_camera_uuid) == 0) {
            primary_found = true;
        }
        for (int previous = 0; previous < i; previous++) {
            if (strcmp(camera_uuids[previous], camera_uuids[i]) == 0) {
                return false;
            }
        }
    }
    return primary_found;
}

static bool valid_bookmark(const investigation_bookmark_t *bookmark,
                           bool require_uuid) {
    if (!bookmark || bookmark->owner_user_id < 0 ||
        (require_uuid && !lightnvr_uuid_is_valid(bookmark->uuid)) ||
        bookmark->title[0] == '\0' ||
        strnlen(bookmark->title, sizeof(bookmark->title)) >=
            sizeof(bookmark->title) ||
        strnlen(bookmark->note, sizeof(bookmark->note)) >=
            sizeof(bookmark->note) ||
        bookmark->start_time < 1 || bookmark->end_time <= bookmark->start_time ||
        bookmark->end_time - bookmark->start_time > 31LL * 24 * 60 * 60 ||
        bookmark->cursor_time < bookmark->start_time ||
        bookmark->cursor_time > bookmark->end_time ||
        !lightnvr_uuid_is_valid(bookmark->primary_camera_uuid) ||
        strnlen(bookmark->filters_json, sizeof(bookmark->filters_json)) >=
            sizeof(bookmark->filters_json) ||
        strnlen(bookmark->representative_result_json,
                sizeof(bookmark->representative_result_json)) >=
            sizeof(bookmark->representative_result_json) ||
        !valid_json_object(bookmark->filters_json, false) ||
        !valid_json_object(bookmark->representative_result_json, true)) {
        return false;
    }
    return true;
}

static void copy_column(char *destination, size_t destination_size,
                        sqlite3_stmt *statement, int column) {
    const char *value = (const char *)sqlite3_column_text(statement, column);
    safe_strcpy(destination, value ? value : "", destination_size, 0);
}

static void populate_bookmark(sqlite3_stmt *statement,
                              investigation_bookmark_t *bookmark) {
    memset(bookmark, 0, sizeof(*bookmark));
    copy_column(bookmark->uuid, sizeof(bookmark->uuid), statement, 0);
    bookmark->owner_user_id = sqlite3_column_int64(statement, 1);
    copy_column(bookmark->title, sizeof(bookmark->title), statement, 2);
    copy_column(bookmark->note, sizeof(bookmark->note), statement, 3);
    bookmark->start_time = sqlite3_column_int64(statement, 4);
    bookmark->end_time = sqlite3_column_int64(statement, 5);
    bookmark->cursor_time = sqlite3_column_int64(statement, 6);
    copy_column(bookmark->primary_camera_uuid,
                sizeof(bookmark->primary_camera_uuid), statement, 7);
    copy_column(bookmark->filters_json, sizeof(bookmark->filters_json),
                statement, 8);
    copy_column(bookmark->representative_result_json,
                sizeof(bookmark->representative_result_json), statement, 9);
    bookmark->revision = sqlite3_column_int64(statement, 10);
    bookmark->created_at = sqlite3_column_int64(statement, 11);
    bookmark->updated_at = sqlite3_column_int64(statement, 12);
    bookmark->camera_count = sqlite3_column_int(statement, 13);
}

static db_investigation_bookmark_result_t get_locked(
    sqlite3 *db, int64_t owner_user_id, const char *uuid,
    investigation_bookmark_t *bookmark) {
    const char *sql =
        "SELECT " BOOKMARK_SELECT_FIELDS
        "FROM investigation_bookmarks b WHERE b.uuid=? AND "
        "((?=0 AND b.owner_user_id IS NULL) OR b.owner_user_id=?) LIMIT 1;";
    sqlite3_stmt *statement = NULL;
    int result = sqlite3_prepare_v2(db, sql, -1, &statement, NULL);
    if (result != SQLITE_OK) return DB_INVESTIGATION_BOOKMARK_ERROR;
    sqlite3_bind_text(statement, 1, uuid, -1, SQLITE_TRANSIENT);
    owner_predicate_bind(statement, 2, owner_user_id);
    owner_predicate_bind(statement, 3, owner_user_id);
    result = sqlite3_step(statement);
    db_investigation_bookmark_result_t outcome =
        DB_INVESTIGATION_BOOKMARK_NOT_FOUND;
    if (result == SQLITE_ROW) {
        populate_bookmark(statement, bookmark);
        outcome = DB_INVESTIGATION_BOOKMARK_OK;
    } else if (result != SQLITE_DONE) {
        outcome = DB_INVESTIGATION_BOOKMARK_ERROR;
    }
    sqlite3_finalize(statement);
    return outcome;
}

static int count_locked(sqlite3 *db, int64_t owner_user_id) {
    const char *sql =
        "SELECT count(*) FROM investigation_bookmarks WHERE "
        "((?=0 AND owner_user_id IS NULL) OR owner_user_id=?);";
    sqlite3_stmt *statement = NULL;
    if (sqlite3_prepare_v2(db, sql, -1, &statement, NULL) != SQLITE_OK) {
        return -1;
    }
    owner_predicate_bind(statement, 1, owner_user_id);
    owner_predicate_bind(statement, 2, owner_user_id);
    int count = sqlite3_step(statement) == SQLITE_ROW
        ? sqlite3_column_int(statement, 0) : -1;
    sqlite3_finalize(statement);
    return count;
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

int db_investigation_bookmark_count(int64_t owner_user_id) {
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    if (!db || !mutex || owner_user_id < 0) return -1;
    pthread_mutex_lock(mutex);
    int count = count_locked(db, owner_user_id);
    pthread_mutex_unlock(mutex);
    return count;
}

int db_investigation_bookmark_list(
    int64_t owner_user_id, investigation_bookmark_t *bookmarks,
    int max_count) {
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    if (!db || !mutex || !bookmarks || max_count <= 0 || owner_user_id < 0) {
        return -1;
    }
    const char *sql =
        "SELECT " BOOKMARK_SELECT_FIELDS
        "FROM investigation_bookmarks b WHERE "
        "((?=0 AND b.owner_user_id IS NULL) OR b.owner_user_id=?) "
        "ORDER BY b.updated_at DESC, b.uuid LIMIT ?;";
    pthread_mutex_lock(mutex);
    sqlite3_stmt *statement = NULL;
    int result = sqlite3_prepare_v2(db, sql, -1, &statement, NULL);
    if (result != SQLITE_OK) {
        pthread_mutex_unlock(mutex);
        return -1;
    }
    owner_predicate_bind(statement, 1, owner_user_id);
    owner_predicate_bind(statement, 2, owner_user_id);
    sqlite3_bind_int(statement, 3, max_count);
    int count = 0;
    while (count < max_count &&
           (result = sqlite3_step(statement)) == SQLITE_ROW) {
        populate_bookmark(statement, &bookmarks[count++]);
    }
    if (result != SQLITE_DONE && result != SQLITE_ROW) count = -1;
    sqlite3_finalize(statement);
    pthread_mutex_unlock(mutex);
    return count;
}

db_investigation_bookmark_result_t db_investigation_bookmark_get(
    int64_t owner_user_id, const char *uuid,
    investigation_bookmark_t *bookmark) {
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    if (!db || !mutex || !bookmark || owner_user_id < 0 ||
        !lightnvr_uuid_is_valid(uuid)) {
        return DB_INVESTIGATION_BOOKMARK_INVALID;
    }
    pthread_mutex_lock(mutex);
    db_investigation_bookmark_result_t result =
        get_locked(db, owner_user_id, uuid, bookmark);
    pthread_mutex_unlock(mutex);
    return result;
}

int db_investigation_bookmark_list_cameras(
    const char *bookmark_uuid,
    char camera_uuids[][CAMERA_UUID_STRING_SIZE], int max_count) {
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    if (!db || !mutex || !camera_uuids || max_count <= 0 ||
        !lightnvr_uuid_is_valid(bookmark_uuid)) return -1;
    const char *sql =
        "SELECT camera_uuid FROM investigation_bookmark_cameras "
        "WHERE bookmark_uuid=? ORDER BY sort_order LIMIT ?;";
    pthread_mutex_lock(mutex);
    sqlite3_stmt *statement = NULL;
    int result = sqlite3_prepare_v2(db, sql, -1, &statement, NULL);
    if (result != SQLITE_OK) {
        pthread_mutex_unlock(mutex);
        return -1;
    }
    sqlite3_bind_text(statement, 1, bookmark_uuid, -1, SQLITE_TRANSIENT);
    sqlite3_bind_int(statement, 2, max_count);
    int count = 0;
    while (count < max_count &&
           (result = sqlite3_step(statement)) == SQLITE_ROW) {
        copy_column(camera_uuids[count], CAMERA_UUID_STRING_SIZE,
                    statement, 0);
        count++;
    }
    if (result != SQLITE_DONE && result != SQLITE_ROW) count = -1;
    sqlite3_finalize(statement);
    pthread_mutex_unlock(mutex);
    return count;
}

db_investigation_bookmark_result_t db_investigation_bookmark_create(
    investigation_bookmark_t *bookmark,
    const char camera_uuids[][CAMERA_UUID_STRING_SIZE], int camera_count) {
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    if (!db || !mutex || !valid_bookmark(bookmark, false) ||
        !valid_cameras(camera_uuids, camera_count,
                       bookmark->primary_camera_uuid) ||
        lightnvr_uuid_generate_v4(bookmark->uuid) != 0) {
        return DB_INVESTIGATION_BOOKMARK_INVALID;
    }
    char normalized_title[INVESTIGATION_BOOKMARK_TITLE_MAX];
    if (copy_trimmed_value(normalized_title, sizeof(normalized_title),
                           bookmark->title, 0) == 0) {
        return DB_INVESTIGATION_BOOKMARK_INVALID;
    }

    pthread_mutex_lock(mutex);
    int existing_count = count_locked(db, bookmark->owner_user_id);
    if (existing_count < 0) {
        pthread_mutex_unlock(mutex);
        return DB_INVESTIGATION_BOOKMARK_ERROR;
    }
    if (existing_count >= INVESTIGATION_BOOKMARK_MAX_PER_OWNER) {
        pthread_mutex_unlock(mutex);
        return DB_INVESTIGATION_BOOKMARK_LIMIT;
    }
    if (sqlite3_exec(db, "BEGIN IMMEDIATE;", NULL, NULL, NULL) != SQLITE_OK) {
        pthread_mutex_unlock(mutex);
        return DB_INVESTIGATION_BOOKMARK_ERROR;
    }

    const char *insert_bookmark =
        "INSERT INTO investigation_bookmarks("
        "uuid,owner_user_id,title,note,start_time,end_time,cursor_time,"
        "primary_camera_uuid,filters_json,representative_result_json) "
        "VALUES(?,?,?,?,?,?,?,?,?,?);";
    sqlite3_stmt *statement = NULL;
    int result = sqlite3_prepare_v2(
        db, insert_bookmark, -1, &statement, NULL);
    if (result == SQLITE_OK) {
        sqlite3_bind_text(statement, 1, bookmark->uuid, -1,
                          SQLITE_TRANSIENT);
        if (bookmark->owner_user_id > 0) {
            sqlite3_bind_int64(statement, 2, bookmark->owner_user_id);
        } else {
            sqlite3_bind_null(statement, 2);
        }
        sqlite3_bind_text(statement, 3, normalized_title, -1,
                          SQLITE_TRANSIENT);
        sqlite3_bind_text(statement, 4, bookmark->note, -1, SQLITE_TRANSIENT);
        sqlite3_bind_int64(statement, 5, bookmark->start_time);
        sqlite3_bind_int64(statement, 6, bookmark->end_time);
        sqlite3_bind_int64(statement, 7, bookmark->cursor_time);
        sqlite3_bind_text(statement, 8, bookmark->primary_camera_uuid, -1,
                          SQLITE_TRANSIENT);
        sqlite3_bind_text(statement, 9, bookmark->filters_json, -1,
                          SQLITE_TRANSIENT);
        if (bookmark->representative_result_json[0]) {
            sqlite3_bind_text(statement, 10,
                              bookmark->representative_result_json, -1,
                              SQLITE_TRANSIENT);
        } else {
            sqlite3_bind_null(statement, 10);
        }
        result = sqlite3_step(statement);
    }
    if (statement) sqlite3_finalize(statement);

    const char *insert_camera =
        "INSERT INTO investigation_bookmark_cameras("
        "bookmark_uuid,camera_uuid,sort_order) VALUES(?,?,?);";
    for (int i = 0; result == SQLITE_DONE && i < camera_count; i++) {
        statement = NULL;
        result = sqlite3_prepare_v2(db, insert_camera, -1, &statement, NULL);
        if (result == SQLITE_OK) {
            sqlite3_bind_text(statement, 1, bookmark->uuid, -1,
                              SQLITE_TRANSIENT);
            sqlite3_bind_text(statement, 2, camera_uuids[i], -1,
                              SQLITE_TRANSIENT);
            sqlite3_bind_int(statement, 3, i);
            result = sqlite3_step(statement);
        }
        if (statement) sqlite3_finalize(statement);
    }

    if (!transaction_finish(db, result == SQLITE_DONE)) {
        pthread_mutex_unlock(mutex);
        return DB_INVESTIGATION_BOOKMARK_ERROR;
    }
    db_investigation_bookmark_result_t outcome = get_locked(
        db, bookmark->owner_user_id, bookmark->uuid, bookmark);
    pthread_mutex_unlock(mutex);
    return outcome;
}

db_investigation_bookmark_result_t db_investigation_bookmark_update_metadata(
    investigation_bookmark_t *bookmark, int64_t expected_revision) {
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    if (!db || !mutex || !valid_bookmark(bookmark, true) ||
        expected_revision < 1) {
        return DB_INVESTIGATION_BOOKMARK_INVALID;
    }
    char normalized_title[INVESTIGATION_BOOKMARK_TITLE_MAX];
    if (copy_trimmed_value(normalized_title, sizeof(normalized_title),
                           bookmark->title, 0) == 0) {
        return DB_INVESTIGATION_BOOKMARK_INVALID;
    }
    pthread_mutex_lock(mutex);
    investigation_bookmark_t existing;
    db_investigation_bookmark_result_t outcome = get_locked(
        db, bookmark->owner_user_id, bookmark->uuid, &existing);
    if (outcome != DB_INVESTIGATION_BOOKMARK_OK) {
        pthread_mutex_unlock(mutex);
        return outcome;
    }
    if (existing.revision != expected_revision) {
        pthread_mutex_unlock(mutex);
        return DB_INVESTIGATION_BOOKMARK_STALE;
    }
    const char *sql =
        "UPDATE investigation_bookmarks SET title=?,note=?,"
        "revision=revision+1,updated_at=strftime('%s','now') "
        "WHERE uuid=? AND revision=? AND "
        "((?=0 AND owner_user_id IS NULL) OR owner_user_id=?);";
    sqlite3_stmt *statement = NULL;
    int result = sqlite3_prepare_v2(db, sql, -1, &statement, NULL);
    if (result == SQLITE_OK) {
        sqlite3_bind_text(statement, 1, normalized_title, -1,
                          SQLITE_TRANSIENT);
        sqlite3_bind_text(statement, 2, bookmark->note, -1, SQLITE_TRANSIENT);
        sqlite3_bind_text(statement, 3, bookmark->uuid, -1,
                          SQLITE_TRANSIENT);
        sqlite3_bind_int64(statement, 4, expected_revision);
        owner_predicate_bind(statement, 5, bookmark->owner_user_id);
        owner_predicate_bind(statement, 6, bookmark->owner_user_id);
        result = sqlite3_step(statement);
    }
    int changed = result == SQLITE_DONE ? sqlite3_changes(db) : 0;
    if (statement) sqlite3_finalize(statement);
    if (result != SQLITE_DONE || changed != 1) {
        pthread_mutex_unlock(mutex);
        return result == SQLITE_DONE ? DB_INVESTIGATION_BOOKMARK_STALE
                                     : DB_INVESTIGATION_BOOKMARK_ERROR;
    }
    outcome = get_locked(db, bookmark->owner_user_id, bookmark->uuid,
                         bookmark);
    pthread_mutex_unlock(mutex);
    return outcome;
}

db_investigation_bookmark_result_t db_investigation_bookmark_delete(
    int64_t owner_user_id, const char *uuid, int64_t expected_revision) {
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    if (!db || !mutex || owner_user_id < 0 || expected_revision < 1 ||
        !lightnvr_uuid_is_valid(uuid)) {
        return DB_INVESTIGATION_BOOKMARK_INVALID;
    }
    pthread_mutex_lock(mutex);
    investigation_bookmark_t existing;
    db_investigation_bookmark_result_t outcome = get_locked(
        db, owner_user_id, uuid, &existing);
    if (outcome != DB_INVESTIGATION_BOOKMARK_OK) {
        pthread_mutex_unlock(mutex);
        return outcome;
    }
    if (existing.revision != expected_revision) {
        pthread_mutex_unlock(mutex);
        return DB_INVESTIGATION_BOOKMARK_STALE;
    }
    const char *sql =
        "DELETE FROM investigation_bookmarks WHERE uuid=? AND revision=? AND "
        "((?=0 AND owner_user_id IS NULL) OR owner_user_id=?);";
    sqlite3_stmt *statement = NULL;
    int result = sqlite3_prepare_v2(db, sql, -1, &statement, NULL);
    if (result == SQLITE_OK) {
        sqlite3_bind_text(statement, 1, uuid, -1, SQLITE_TRANSIENT);
        sqlite3_bind_int64(statement, 2, expected_revision);
        owner_predicate_bind(statement, 3, owner_user_id);
        owner_predicate_bind(statement, 4, owner_user_id);
        result = sqlite3_step(statement);
    }
    int changed = result == SQLITE_DONE ? sqlite3_changes(db) : 0;
    if (statement) sqlite3_finalize(statement);
    pthread_mutex_unlock(mutex);
    if (result != SQLITE_DONE) return DB_INVESTIGATION_BOOKMARK_ERROR;
    return changed == 1 ? DB_INVESTIGATION_BOOKMARK_OK
                        : DB_INVESTIGATION_BOOKMARK_STALE;
}
