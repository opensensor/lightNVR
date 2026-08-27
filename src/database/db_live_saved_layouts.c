#define _POSIX_C_SOURCE 200809L

#include "database/db_live_saved_layouts.h"

#include <cjson/cJSON.h>
#include <ctype.h>
#include <pthread.h>
#include <sqlite3.h>
#include <stdio.h>
#include <string.h>

#include "database/db_core.h"
#include "utils/strings.h"
#include "utils/uuid.h"

#define LAYOUT_FIELDS \
    "uuid,COALESCE(owner_user_id,0),name,is_shared," \
    "COALESCE(location_uuid,''),availability,columns,rows," \
    "camera_slots_json,revision,created_at,updated_at"

static void bind_owner(sqlite3_stmt *statement, int parameter,
                       int64_t owner_user_id) {
    sqlite3_bind_int64(statement, parameter, owner_user_id);
}

static void copy_column(char *destination, size_t size,
                        sqlite3_stmt *statement, int column) {
    const char *value = (const char *)sqlite3_column_text(statement, column);
    safe_strcpy(destination, value ? value : "", size, 0);
}

static void populate(sqlite3_stmt *statement, live_saved_layout_t *layout) {
    memset(layout, 0, sizeof(*layout));
    copy_column(layout->uuid, sizeof(layout->uuid), statement, 0);
    layout->owner_user_id = sqlite3_column_int64(statement, 1);
    copy_column(layout->name, sizeof(layout->name), statement, 2);
    layout->is_shared = sqlite3_column_int(statement, 3) != 0;
    copy_column(layout->location_uuid, sizeof(layout->location_uuid),
                statement, 4);
    copy_column(layout->availability, sizeof(layout->availability),
                statement, 5);
    layout->columns = sqlite3_column_int(statement, 6);
    layout->rows = sqlite3_column_int(statement, 7);
    copy_column(layout->camera_slots_json, sizeof(layout->camera_slots_json),
                statement, 8);
    layout->revision = sqlite3_column_int64(statement, 9);
    layout->created_at = sqlite3_column_int64(statement, 10);
    layout->updated_at = sqlite3_column_int64(statement, 11);
}

static bool valid_name(const char *name) {
    size_t length = name ? strnlen(name, LIVE_LAYOUT_NAME_MAX) : 0;
    if (length == 0 || length == LIVE_LAYOUT_NAME_MAX) return false;
    for (const unsigned char *cursor = (const unsigned char *)name; *cursor;
         cursor++) {
        if (iscntrl(*cursor)) return false;
    }
    return true;
}

static bool valid_availability(const char *value) {
    return value && (strcmp(value, "all") == 0 ||
        strcmp(value, "live") == 0 || strcmp(value, "offline") == 0 ||
        strcmp(value, "never_connected") == 0 ||
        strcmp(value, "disabled") == 0);
}

static bool valid_slots(const char *encoded, int capacity) {
    cJSON *slots = cJSON_Parse(encoded);
    int count = cJSON_IsArray(slots) ? cJSON_GetArraySize(slots) : -1;
    bool valid = count >= 0 && count <= capacity && count <= 36;
    for (int index = 0; valid && index < count; index++) {
        const cJSON *slot = cJSON_GetArrayItem(slots, index);
        const cJSON *camera = cJSON_IsObject(slot)
            ? cJSON_GetObjectItemCaseSensitive(slot, "camera_uuid") : NULL;
        valid = cJSON_IsString(camera) && camera->valuestring &&
            lightnvr_uuid_is_valid(camera->valuestring);
        for (int previous = 0; valid && previous < index; previous++) {
            const cJSON *other = cJSON_GetObjectItemCaseSensitive(
                cJSON_GetArrayItem(slots, previous), "camera_uuid");
            if (cJSON_IsString(other) &&
                strcmp(other->valuestring, camera->valuestring) == 0) {
                valid = false;
            }
        }
    }
    cJSON_Delete(slots);
    return valid;
}

static bool valid_layout(const live_saved_layout_t *layout,
                         bool require_uuid) {
    int capacity = layout ? layout->columns * layout->rows : 0;
    return layout && layout->owner_user_id >= 0 &&
        (!require_uuid || lightnvr_uuid_is_valid(layout->uuid)) &&
        valid_name(layout->name) &&
        (layout->location_uuid[0] == '\0' ||
         lightnvr_uuid_is_valid(layout->location_uuid)) &&
        valid_availability(layout->availability) &&
        layout->columns >= 1 && layout->columns <= 9 &&
        layout->rows >= 1 && layout->rows <= 9 && capacity <= 36 &&
        valid_slots(layout->camera_slots_json, capacity);
}

static db_live_layout_result_t get_locked(
    sqlite3 *db, int64_t owner_user_id, const char *uuid, bool visible,
    live_saved_layout_t *layout) {
    const char *visibility = visible
        ? "(is_shared=1 OR ((?=0 AND owner_user_id IS NULL) OR owner_user_id=?))"
        : "((?=0 AND owner_user_id IS NULL) OR owner_user_id=?)";
    char sql[768];
    int written = snprintf(sql, sizeof(sql),
                           "SELECT " LAYOUT_FIELDS
                           " FROM live_saved_layouts WHERE uuid=? AND %s "
                           "LIMIT 1;", visibility);
    if (written < 0 || (size_t)written >= sizeof(sql)) {
        return DB_LIVE_LAYOUT_ERROR;
    }
    sqlite3_stmt *statement = NULL;
    int result = sqlite3_prepare_v2(db, sql, -1, &statement, NULL);
    if (result == SQLITE_OK) {
        sqlite3_bind_text(statement, 1, uuid, -1, SQLITE_TRANSIENT);
        bind_owner(statement, 2, owner_user_id);
        bind_owner(statement, 3, owner_user_id);
        result = sqlite3_step(statement);
    }
    db_live_layout_result_t outcome = DB_LIVE_LAYOUT_NOT_FOUND;
    if (result == SQLITE_ROW) {
        populate(statement, layout);
        outcome = DB_LIVE_LAYOUT_OK;
    } else if (result != SQLITE_DONE) {
        outcome = DB_LIVE_LAYOUT_ERROR;
    }
    if (statement) sqlite3_finalize(statement);
    return outcome;
}

int db_live_layout_list_visible(int64_t owner_user_id,
                                live_saved_layout_t *layouts,
                                int max_count) {
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    if (!db || !mutex || !layouts || owner_user_id < 0 || max_count < 1 ||
        max_count > LIVE_LAYOUT_MAX_VISIBLE) return -1;
    const char *sql =
        "SELECT " LAYOUT_FIELDS " FROM live_saved_layouts WHERE is_shared=1 "
        "OR ((?=0 AND owner_user_id IS NULL) OR owner_user_id=?) "
        "ORDER BY is_shared DESC,name COLLATE NOCASE,uuid LIMIT ?;";
    pthread_mutex_lock(mutex);
    sqlite3_stmt *statement = NULL;
    int result = sqlite3_prepare_v2(db, sql, -1, &statement, NULL);
    if (result == SQLITE_OK) {
        bind_owner(statement, 1, owner_user_id);
        bind_owner(statement, 2, owner_user_id);
        sqlite3_bind_int(statement, 3, max_count);
    }
    int count = 0;
    if (result == SQLITE_OK) {
        while (count < max_count &&
               (result = sqlite3_step(statement)) == SQLITE_ROW) {
            populate(statement, &layouts[count++]);
        }
    }
    if (result != SQLITE_DONE && result != SQLITE_ROW) count = -1;
    if (statement) sqlite3_finalize(statement);
    pthread_mutex_unlock(mutex);
    return count;
}

db_live_layout_result_t db_live_layout_get_visible(
    int64_t owner_user_id, const char *uuid, live_saved_layout_t *layout) {
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    if (!db || !mutex || !layout || owner_user_id < 0 ||
        !lightnvr_uuid_is_valid(uuid)) return DB_LIVE_LAYOUT_INVALID;
    pthread_mutex_lock(mutex);
    db_live_layout_result_t result = get_locked(
        db, owner_user_id, uuid, true, layout);
    pthread_mutex_unlock(mutex);
    return result;
}

db_live_layout_result_t db_live_layout_create(live_saved_layout_t *layout) {
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    char name[LIVE_LAYOUT_NAME_MAX];
    if (!db || !mutex || !valid_layout(layout, false) ||
        copy_trimmed_value(name, sizeof(name), layout->name, 0) == 0 ||
        lightnvr_uuid_generate_v4(layout->uuid) != 0) {
        return DB_LIVE_LAYOUT_INVALID;
    }
    pthread_mutex_lock(mutex);
    const char *count_sql =
        "SELECT count(*) FROM live_saved_layouts WHERE "
        "((?=0 AND owner_user_id IS NULL) OR owner_user_id=?);";
    sqlite3_stmt *statement = NULL;
    int result = sqlite3_prepare_v2(db, count_sql, -1, &statement, NULL);
    int count = -1;
    if (result == SQLITE_OK) {
        bind_owner(statement, 1, layout->owner_user_id);
        bind_owner(statement, 2, layout->owner_user_id);
        if (sqlite3_step(statement) == SQLITE_ROW) {
            count = sqlite3_column_int(statement, 0);
        }
    }
    if (statement) sqlite3_finalize(statement);
    if (count < 0 || count >= LIVE_LAYOUT_MAX_VISIBLE) {
        pthread_mutex_unlock(mutex);
        return count < 0 ? DB_LIVE_LAYOUT_ERROR : DB_LIVE_LAYOUT_LIMIT;
    }
    const char *sql =
        "INSERT INTO live_saved_layouts(uuid,owner_user_id,name,is_shared,"
        "location_uuid,availability,columns,rows,camera_slots_json) "
        "VALUES(?,?,?,?,?,?,?,?,?);";
    statement = NULL;
    result = sqlite3_prepare_v2(db, sql, -1, &statement, NULL);
    if (result == SQLITE_OK) {
        sqlite3_bind_text(statement, 1, layout->uuid, -1, SQLITE_TRANSIENT);
        if (layout->owner_user_id > 0) {
            sqlite3_bind_int64(statement, 2, layout->owner_user_id);
        } else {
            sqlite3_bind_null(statement, 2);
        }
        sqlite3_bind_text(statement, 3, name, -1, SQLITE_TRANSIENT);
        sqlite3_bind_int(statement, 4, layout->is_shared ? 1 : 0);
        if (layout->location_uuid[0]) {
            sqlite3_bind_text(statement, 5, layout->location_uuid, -1,
                              SQLITE_TRANSIENT);
        } else {
            sqlite3_bind_null(statement, 5);
        }
        sqlite3_bind_text(statement, 6, layout->availability, -1,
                          SQLITE_TRANSIENT);
        sqlite3_bind_int(statement, 7, layout->columns);
        sqlite3_bind_int(statement, 8, layout->rows);
        sqlite3_bind_text(statement, 9, layout->camera_slots_json, -1,
                          SQLITE_TRANSIENT);
        result = sqlite3_step(statement);
    }
    if (statement) sqlite3_finalize(statement);
    if (result != SQLITE_DONE) {
        pthread_mutex_unlock(mutex);
        return result == SQLITE_CONSTRAINT ? DB_LIVE_LAYOUT_CONFLICT
                                           : DB_LIVE_LAYOUT_ERROR;
    }
    safe_strcpy(layout->name, name, sizeof(layout->name), 0);
    db_live_layout_result_t outcome = get_locked(
        db, layout->owner_user_id, layout->uuid, false, layout);
    pthread_mutex_unlock(mutex);
    return outcome;
}

db_live_layout_result_t db_live_layout_update(live_saved_layout_t *layout,
                                              int64_t expected_revision) {
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    char name[LIVE_LAYOUT_NAME_MAX];
    if (!db || !mutex || expected_revision < 1 ||
        !valid_layout(layout, true) ||
        copy_trimmed_value(name, sizeof(name), layout->name, 0) == 0) {
        return DB_LIVE_LAYOUT_INVALID;
    }
    pthread_mutex_lock(mutex);
    live_saved_layout_t existing;
    db_live_layout_result_t outcome = get_locked(
        db, layout->owner_user_id, layout->uuid, false, &existing);
    if (outcome != DB_LIVE_LAYOUT_OK || existing.revision != expected_revision) {
        pthread_mutex_unlock(mutex);
        return outcome == DB_LIVE_LAYOUT_OK ? DB_LIVE_LAYOUT_STALE : outcome;
    }
    const char *sql =
        "UPDATE live_saved_layouts SET name=?,is_shared=?,location_uuid=?,"
        "availability=?,columns=?,rows=?,camera_slots_json=?,"
        "revision=revision+1,updated_at=strftime('%s','now') "
        "WHERE uuid=? AND revision=? AND "
        "((?=0 AND owner_user_id IS NULL) OR owner_user_id=?);";
    sqlite3_stmt *statement = NULL;
    int result = sqlite3_prepare_v2(db, sql, -1, &statement, NULL);
    if (result == SQLITE_OK) {
        sqlite3_bind_text(statement, 1, name, -1, SQLITE_TRANSIENT);
        sqlite3_bind_int(statement, 2, layout->is_shared ? 1 : 0);
        if (layout->location_uuid[0]) {
            sqlite3_bind_text(statement, 3, layout->location_uuid, -1,
                              SQLITE_TRANSIENT);
        } else {
            sqlite3_bind_null(statement, 3);
        }
        sqlite3_bind_text(statement, 4, layout->availability, -1,
                          SQLITE_TRANSIENT);
        sqlite3_bind_int(statement, 5, layout->columns);
        sqlite3_bind_int(statement, 6, layout->rows);
        sqlite3_bind_text(statement, 7, layout->camera_slots_json, -1,
                          SQLITE_TRANSIENT);
        sqlite3_bind_text(statement, 8, layout->uuid, -1, SQLITE_TRANSIENT);
        sqlite3_bind_int64(statement, 9, expected_revision);
        bind_owner(statement, 10, layout->owner_user_id);
        bind_owner(statement, 11, layout->owner_user_id);
        result = sqlite3_step(statement);
    }
    int changed = result == SQLITE_DONE ? sqlite3_changes(db) : 0;
    if (statement) sqlite3_finalize(statement);
    if (result != SQLITE_DONE || changed != 1) {
        pthread_mutex_unlock(mutex);
        if (result == SQLITE_CONSTRAINT) return DB_LIVE_LAYOUT_CONFLICT;
        return result == SQLITE_DONE ? DB_LIVE_LAYOUT_STALE
                                     : DB_LIVE_LAYOUT_ERROR;
    }
    outcome = get_locked(db, layout->owner_user_id, layout->uuid, false,
                         layout);
    pthread_mutex_unlock(mutex);
    return outcome;
}

db_live_layout_result_t db_live_layout_delete(int64_t owner_user_id,
                                              const char *uuid,
                                              int64_t expected_revision) {
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    if (!db || !mutex || owner_user_id < 0 || expected_revision < 1 ||
        !lightnvr_uuid_is_valid(uuid)) return DB_LIVE_LAYOUT_INVALID;
    pthread_mutex_lock(mutex);
    live_saved_layout_t existing;
    db_live_layout_result_t outcome = get_locked(
        db, owner_user_id, uuid, false, &existing);
    if (outcome != DB_LIVE_LAYOUT_OK || existing.revision != expected_revision) {
        pthread_mutex_unlock(mutex);
        return outcome == DB_LIVE_LAYOUT_OK ? DB_LIVE_LAYOUT_STALE : outcome;
    }
    const char *sql =
        "DELETE FROM live_saved_layouts WHERE uuid=? AND revision=? AND "
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
    if (result != SQLITE_DONE) return DB_LIVE_LAYOUT_ERROR;
    return changed == 1 ? DB_LIVE_LAYOUT_OK : DB_LIVE_LAYOUT_STALE;
}
