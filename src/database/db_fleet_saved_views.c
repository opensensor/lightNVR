#define _POSIX_C_SOURCE 200809L

#include "database/db_fleet_saved_views.h"

#include <cjson/cJSON.h>
#include <ctype.h>
#include <pthread.h>
#include <sqlite3.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "core/camera_selector.h"
#include "database/db_core.h"
#include "utils/strings.h"
#include "utils/uuid.h"

#define SAVED_VIEW_SELECT_FIELDS \
    "v.uuid,COALESCE(v.owner_user_id,0),v.name,v.is_shared," \
    "v.selector_json,v.search_text,COALESCE(v.collection_uuid,'')," \
    "v.columns_json,v.sort_by,v.sort_order,v.revision,v.created_at," \
    "v.updated_at "

static void bind_owner(sqlite3_stmt *statement, int parameter,
                       int64_t owner_user_id) {
    sqlite3_bind_int64(statement, parameter, owner_user_id);
}

static void copy_column(char *destination, size_t size,
                        sqlite3_stmt *statement, int column) {
    const char *value = (const char *)sqlite3_column_text(statement, column);
    safe_strcpy(destination, value ? value : "", size, 0);
}

static void populate(sqlite3_stmt *statement, fleet_saved_view_t *view) {
    memset(view, 0, sizeof(*view));
    copy_column(view->uuid, sizeof(view->uuid), statement, 0);
    view->owner_user_id = sqlite3_column_int64(statement, 1);
    copy_column(view->name, sizeof(view->name), statement, 2);
    view->is_shared = sqlite3_column_int(statement, 3) != 0;
    copy_column(view->selector_json, sizeof(view->selector_json), statement, 4);
    copy_column(view->search, sizeof(view->search), statement, 5);
    copy_column(view->collection_uuid, sizeof(view->collection_uuid),
                statement, 6);
    copy_column(view->columns_json, sizeof(view->columns_json), statement, 7);
    copy_column(view->sort_by, sizeof(view->sort_by), statement, 8);
    copy_column(view->sort_order, sizeof(view->sort_order), statement, 9);
    view->revision = sqlite3_column_int64(statement, 10);
    view->created_at = sqlite3_column_int64(statement, 11);
    view->updated_at = sqlite3_column_int64(statement, 12);
}

static bool valid_text(const char *value, size_t maximum, bool allow_empty) {
    if (!value) return false;
    size_t length = strnlen(value, maximum);
    if (length == maximum || (!allow_empty && length == 0)) return false;
    for (const unsigned char *cursor = (const unsigned char *)value; *cursor;
         cursor++) {
        if (iscntrl(*cursor)) return false;
    }
    return true;
}

static bool valid_sort(const fleet_saved_view_t *view) {
    bool field = strcmp(view->sort_by, "name") == 0 ||
        strcmp(view->sort_by, "camera_uuid") == 0 ||
        strcmp(view->sort_by, "location") == 0 ||
        strcmp(view->sort_by, "health") == 0 ||
        strcmp(view->sort_by, "enabled") == 0 ||
        strcmp(view->sort_by, "recording_mode") == 0 ||
        strcmp(view->sort_by, "address") == 0;
    return field && (strcmp(view->sort_order, "asc") == 0 ||
                     strcmp(view->sort_order, "desc") == 0);
}

static bool valid_columns(const char *encoded) {
    static const char *const allowed[] = {
        "camera", "health", "location", "tags", "recording", "actions"
    };
    cJSON *columns = cJSON_Parse(encoded);
    int count = cJSON_IsArray(columns) ? cJSON_GetArraySize(columns) : 0;
    bool valid = count > 0 && count <= 16;
    for (int index = 0; valid && index < count; index++) {
        const cJSON *column = cJSON_GetArrayItem(columns, index);
        valid = cJSON_IsString(column) && column->valuestring;
        bool found = false;
        for (size_t option = 0; valid && option <
             sizeof(allowed) / sizeof(allowed[0]); option++) {
            if (strcmp(column->valuestring, allowed[option]) == 0) {
                found = true;
                break;
            }
        }
        if (!found) valid = false;
        for (int previous = 0; valid && previous < index; previous++) {
            const cJSON *other = cJSON_GetArrayItem(columns, previous);
            if (strcmp(column->valuestring, other->valuestring) == 0) {
                valid = false;
            }
        }
    }
    cJSON_Delete(columns);
    return valid;
}

static bool valid_selector(const char *encoded) {
    cJSON *json = cJSON_Parse(encoded);
    fleet_selector_t *selector = cJSON_IsObject(json)
        ? fleet_selector_parse(json, NULL, 0) : NULL;
    bool valid = selector != NULL;
    fleet_selector_free(selector);
    cJSON_Delete(json);
    return valid;
}

static bool valid_view(const fleet_saved_view_t *view, bool require_uuid) {
    return view && view->owner_user_id >= 0 &&
        (!require_uuid || lightnvr_uuid_is_valid(view->uuid)) &&
        valid_text(view->name, sizeof(view->name), false) &&
        valid_text(view->search, sizeof(view->search), true) &&
        valid_text(view->selector_json, sizeof(view->selector_json), false) &&
        valid_text(view->columns_json, sizeof(view->columns_json), false) &&
        (view->collection_uuid[0] == '\0' ||
         lightnvr_uuid_is_valid(view->collection_uuid)) &&
        valid_selector(view->selector_json) &&
        valid_columns(view->columns_json) && valid_sort(view);
}

static db_fleet_saved_view_result_t get_locked(
    sqlite3 *db, int64_t owner_user_id, const char *uuid,
    bool visible, fleet_saved_view_t *view) {
    const char *visibility = visible
        ? "(v.is_shared=1 OR ((?=0 AND v.owner_user_id IS NULL) OR "
          "v.owner_user_id=?))"
        : "((?=0 AND v.owner_user_id IS NULL) OR v.owner_user_id=?)";
    char sql[768];
    int written = snprintf(sql, sizeof(sql),
                           "SELECT %sFROM fleet_saved_views v "
                           "WHERE v.uuid=? AND %s LIMIT 1;",
                           SAVED_VIEW_SELECT_FIELDS, visibility);
    if (written < 0 || (size_t)written >= sizeof(sql)) {
        return DB_FLEET_SAVED_VIEW_ERROR;
    }
    sqlite3_stmt *statement = NULL;
    int result = sqlite3_prepare_v2(db, sql, -1, &statement, NULL);
    if (result == SQLITE_OK) {
        sqlite3_bind_text(statement, 1, uuid, -1, SQLITE_TRANSIENT);
        bind_owner(statement, 2, owner_user_id);
        bind_owner(statement, 3, owner_user_id);
        result = sqlite3_step(statement);
    }
    db_fleet_saved_view_result_t outcome = DB_FLEET_SAVED_VIEW_NOT_FOUND;
    if (result == SQLITE_ROW) {
        populate(statement, view);
        outcome = DB_FLEET_SAVED_VIEW_OK;
    } else if (result != SQLITE_DONE) {
        outcome = DB_FLEET_SAVED_VIEW_ERROR;
    }
    if (statement) sqlite3_finalize(statement);
    return outcome;
}

int db_fleet_saved_view_list_visible(
    int64_t owner_user_id, fleet_saved_view_t *views, int max_count) {
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    if (!db || !mutex || !views || owner_user_id < 0 || max_count < 1 ||
        max_count > FLEET_SAVED_VIEW_MAX_VISIBLE) return -1;
    const char *sql =
        "SELECT " SAVED_VIEW_SELECT_FIELDS
        "FROM fleet_saved_views v WHERE v.is_shared=1 OR "
        "((?=0 AND v.owner_user_id IS NULL) OR v.owner_user_id=?) "
        "ORDER BY v.is_shared DESC,v.name COLLATE NOCASE,v.uuid LIMIT ?;";
    pthread_mutex_lock(mutex);
    sqlite3_stmt *statement = NULL;
    int result = sqlite3_prepare_v2(db, sql, -1, &statement, NULL);
    if (result == SQLITE_OK) {
        bind_owner(statement, 1, owner_user_id);
        bind_owner(statement, 2, owner_user_id);
        sqlite3_bind_int(statement, 3, max_count);
    }
    int count = 0;
    while (result == SQLITE_OK && count < max_count &&
           (result = sqlite3_step(statement)) == SQLITE_ROW) {
        populate(statement, &views[count++]);
    }
    if (result != SQLITE_DONE && result != SQLITE_ROW) count = -1;
    if (statement) sqlite3_finalize(statement);
    pthread_mutex_unlock(mutex);
    return count;
}

db_fleet_saved_view_result_t db_fleet_saved_view_get_visible(
    int64_t owner_user_id, const char *uuid, fleet_saved_view_t *view) {
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    if (!db || !mutex || !view || owner_user_id < 0 ||
        !lightnvr_uuid_is_valid(uuid)) return DB_FLEET_SAVED_VIEW_INVALID;
    pthread_mutex_lock(mutex);
    db_fleet_saved_view_result_t result = get_locked(
        db, owner_user_id, uuid, true, view);
    pthread_mutex_unlock(mutex);
    return result;
}

db_fleet_saved_view_result_t db_fleet_saved_view_create(
    fleet_saved_view_t *view) {
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    char normalized_name[FLEET_SAVED_VIEW_NAME_MAX];
    if (!db || !mutex || !valid_view(view, false) ||
        copy_trimmed_value(normalized_name, sizeof(normalized_name),
                           view->name, 0) == 0 ||
        lightnvr_uuid_generate_v4(view->uuid) != 0) {
        return DB_FLEET_SAVED_VIEW_INVALID;
    }
    pthread_mutex_lock(mutex);
    const char *count_sql =
        "SELECT count(*) FROM fleet_saved_views WHERE "
        "((?=0 AND owner_user_id IS NULL) OR owner_user_id=?);";
    sqlite3_stmt *statement = NULL;
    int result = sqlite3_prepare_v2(db, count_sql, -1, &statement, NULL);
    int count = -1;
    if (result == SQLITE_OK) {
        bind_owner(statement, 1, view->owner_user_id);
        bind_owner(statement, 2, view->owner_user_id);
        if (sqlite3_step(statement) == SQLITE_ROW) {
            count = sqlite3_column_int(statement, 0);
        }
    }
    if (statement) sqlite3_finalize(statement);
    if (count < 0 || count >= FLEET_SAVED_VIEW_MAX_VISIBLE) {
        pthread_mutex_unlock(mutex);
        return count < 0 ? DB_FLEET_SAVED_VIEW_ERROR
                         : DB_FLEET_SAVED_VIEW_LIMIT;
    }
    const char *sql =
        "INSERT INTO fleet_saved_views(uuid,owner_user_id,name,is_shared,"
        "selector_json,search_text,collection_uuid,columns_json,sort_by,"
        "sort_order) VALUES(?,?,?,?,?,?,?,?,?,?);";
    statement = NULL;
    result = sqlite3_prepare_v2(db, sql, -1, &statement, NULL);
    if (result == SQLITE_OK) {
        sqlite3_bind_text(statement, 1, view->uuid, -1, SQLITE_TRANSIENT);
        if (view->owner_user_id > 0) {
            sqlite3_bind_int64(statement, 2, view->owner_user_id);
        } else {
            sqlite3_bind_null(statement, 2);
        }
        sqlite3_bind_text(statement, 3, normalized_name, -1, SQLITE_TRANSIENT);
        sqlite3_bind_int(statement, 4, view->is_shared ? 1 : 0);
        sqlite3_bind_text(statement, 5, view->selector_json, -1,
                          SQLITE_TRANSIENT);
        sqlite3_bind_text(statement, 6, view->search, -1, SQLITE_TRANSIENT);
        if (view->collection_uuid[0]) {
            sqlite3_bind_text(statement, 7, view->collection_uuid, -1,
                              SQLITE_TRANSIENT);
        } else {
            sqlite3_bind_null(statement, 7);
        }
        sqlite3_bind_text(statement, 8, view->columns_json, -1,
                          SQLITE_TRANSIENT);
        sqlite3_bind_text(statement, 9, view->sort_by, -1, SQLITE_TRANSIENT);
        sqlite3_bind_text(statement, 10, view->sort_order, -1,
                          SQLITE_TRANSIENT);
        result = sqlite3_step(statement);
    }
    if (statement) sqlite3_finalize(statement);
    if (result != SQLITE_DONE) {
        pthread_mutex_unlock(mutex);
        return result == SQLITE_CONSTRAINT ? DB_FLEET_SAVED_VIEW_CONFLICT
                                           : DB_FLEET_SAVED_VIEW_ERROR;
    }
    safe_strcpy(view->name, normalized_name, sizeof(view->name), 0);
    db_fleet_saved_view_result_t outcome = get_locked(
        db, view->owner_user_id, view->uuid, false, view);
    pthread_mutex_unlock(mutex);
    return outcome;
}

db_fleet_saved_view_result_t db_fleet_saved_view_update(
    fleet_saved_view_t *view, int64_t expected_revision) {
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    char normalized_name[FLEET_SAVED_VIEW_NAME_MAX];
    if (!db || !mutex || expected_revision < 1 || !valid_view(view, true) ||
        copy_trimmed_value(normalized_name, sizeof(normalized_name),
                           view->name, 0) == 0) {
        return DB_FLEET_SAVED_VIEW_INVALID;
    }
    pthread_mutex_lock(mutex);
    fleet_saved_view_t existing;
    db_fleet_saved_view_result_t outcome = get_locked(
        db, view->owner_user_id, view->uuid, false, &existing);
    if (outcome != DB_FLEET_SAVED_VIEW_OK) {
        pthread_mutex_unlock(mutex);
        return outcome;
    }
    if (existing.revision != expected_revision) {
        pthread_mutex_unlock(mutex);
        return DB_FLEET_SAVED_VIEW_STALE;
    }
    const char *sql =
        "UPDATE fleet_saved_views SET name=?,is_shared=?,selector_json=?,"
        "search_text=?,collection_uuid=?,columns_json=?,sort_by=?,sort_order=?,"
        "revision=revision+1,updated_at=strftime('%s','now') WHERE uuid=? AND "
        "revision=? AND ((?=0 AND owner_user_id IS NULL) OR owner_user_id=?);";
    sqlite3_stmt *statement = NULL;
    int result = sqlite3_prepare_v2(db, sql, -1, &statement, NULL);
    if (result == SQLITE_OK) {
        sqlite3_bind_text(statement, 1, normalized_name, -1, SQLITE_TRANSIENT);
        sqlite3_bind_int(statement, 2, view->is_shared ? 1 : 0);
        sqlite3_bind_text(statement, 3, view->selector_json, -1,
                          SQLITE_TRANSIENT);
        sqlite3_bind_text(statement, 4, view->search, -1, SQLITE_TRANSIENT);
        if (view->collection_uuid[0]) {
            sqlite3_bind_text(statement, 5, view->collection_uuid, -1,
                              SQLITE_TRANSIENT);
        } else {
            sqlite3_bind_null(statement, 5);
        }
        sqlite3_bind_text(statement, 6, view->columns_json, -1,
                          SQLITE_TRANSIENT);
        sqlite3_bind_text(statement, 7, view->sort_by, -1, SQLITE_TRANSIENT);
        sqlite3_bind_text(statement, 8, view->sort_order, -1,
                          SQLITE_TRANSIENT);
        sqlite3_bind_text(statement, 9, view->uuid, -1, SQLITE_TRANSIENT);
        sqlite3_bind_int64(statement, 10, expected_revision);
        bind_owner(statement, 11, view->owner_user_id);
        bind_owner(statement, 12, view->owner_user_id);
        result = sqlite3_step(statement);
    }
    int changed = result == SQLITE_DONE ? sqlite3_changes(db) : 0;
    if (statement) sqlite3_finalize(statement);
    if (result != SQLITE_DONE || changed != 1) {
        pthread_mutex_unlock(mutex);
        if (result == SQLITE_CONSTRAINT) return DB_FLEET_SAVED_VIEW_CONFLICT;
        return result == SQLITE_DONE ? DB_FLEET_SAVED_VIEW_STALE
                                     : DB_FLEET_SAVED_VIEW_ERROR;
    }
    outcome = get_locked(db, view->owner_user_id, view->uuid, false, view);
    pthread_mutex_unlock(mutex);
    return outcome;
}

db_fleet_saved_view_result_t db_fleet_saved_view_delete(
    int64_t owner_user_id, const char *uuid, int64_t expected_revision) {
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    if (!db || !mutex || owner_user_id < 0 || expected_revision < 1 ||
        !lightnvr_uuid_is_valid(uuid)) return DB_FLEET_SAVED_VIEW_INVALID;
    pthread_mutex_lock(mutex);
    fleet_saved_view_t existing;
    db_fleet_saved_view_result_t outcome = get_locked(
        db, owner_user_id, uuid, false, &existing);
    if (outcome != DB_FLEET_SAVED_VIEW_OK ||
        existing.revision != expected_revision) {
        pthread_mutex_unlock(mutex);
        return outcome == DB_FLEET_SAVED_VIEW_OK
            ? DB_FLEET_SAVED_VIEW_STALE : outcome;
    }
    const char *sql =
        "DELETE FROM fleet_saved_views WHERE uuid=? AND revision=? AND "
        "((?=0 AND owner_user_id IS NULL) OR owner_user_id=?);";
    sqlite3_stmt *statement = NULL;
    int result = sqlite3_prepare_v2(db, sql, -1, &statement, NULL);
    if (result == SQLITE_OK) {
        sqlite3_bind_text(statement, 1, uuid, -1, SQLITE_TRANSIENT);
        sqlite3_bind_int64(statement, 2, expected_revision);
        bind_owner(statement, 3, owner_user_id);
        bind_owner(statement, 4, owner_user_id);
        result = sqlite3_step(statement);
    }
    int changed = result == SQLITE_DONE ? sqlite3_changes(db) : 0;
    if (statement) sqlite3_finalize(statement);
    pthread_mutex_unlock(mutex);
    if (result != SQLITE_DONE) return DB_FLEET_SAVED_VIEW_ERROR;
    return changed == 1 ? DB_FLEET_SAVED_VIEW_OK
                        : DB_FLEET_SAVED_VIEW_STALE;
}
