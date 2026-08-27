#define _POSIX_C_SOURCE 200809L

#include "database/db_eptz_presets.h"

#include <ctype.h>
#include <math.h>
#include <pthread.h>
#include <sqlite3.h>
#include <stdio.h>
#include <string.h>

#include "database/db_core.h"
#include "utils/strings.h"
#include "utils/uuid.h"

#define PRESET_FIELDS \
    "uuid,camera_uuid,COALESCE(owner_user_id,0),name,is_shared,mode," \
    "yaw,tilt,view_fov,secondary_yaw,secondary_tilt,secondary_view_fov," \
    "revision,created_at,updated_at"

static void bind_owner(sqlite3_stmt *statement, int parameter,
                       int64_t owner_user_id) {
    sqlite3_bind_int64(statement, parameter, owner_user_id);
}

static void copy_column(char *destination, size_t size,
                        sqlite3_stmt *statement, int column) {
    const char *value = (const char *)sqlite3_column_text(statement, column);
    safe_strcpy(destination, value ? value : "", size, 0);
}

static void populate(sqlite3_stmt *statement,
                     eptz_operator_preset_t *preset) {
    memset(preset, 0, sizeof(*preset));
    copy_column(preset->uuid, sizeof(preset->uuid), statement, 0);
    copy_column(preset->camera_uuid, sizeof(preset->camera_uuid), statement, 1);
    preset->owner_user_id = sqlite3_column_int64(statement, 2);
    copy_column(preset->name, sizeof(preset->name), statement, 3);
    preset->is_shared = sqlite3_column_int(statement, 4) != 0;
    copy_column(preset->mode, sizeof(preset->mode), statement, 5);
    preset->yaw = sqlite3_column_double(statement, 6);
    preset->tilt = sqlite3_column_double(statement, 7);
    preset->view_fov = sqlite3_column_double(statement, 8);
    preset->secondary_yaw = sqlite3_column_double(statement, 9);
    preset->secondary_tilt = sqlite3_column_double(statement, 10);
    preset->secondary_view_fov = sqlite3_column_double(statement, 11);
    preset->revision = sqlite3_column_int64(statement, 12);
    preset->created_at = sqlite3_column_int64(statement, 13);
    preset->updated_at = sqlite3_column_int64(statement, 14);
}

static bool valid_name(const char *name) {
    size_t length = name ? strnlen(name, EPTZ_PRESET_NAME_MAX) : 0;
    if (length == 0 || length == EPTZ_PRESET_NAME_MAX) return false;
    for (const unsigned char *cursor = (const unsigned char *)name; *cursor;
         cursor++) {
        if (iscntrl(*cursor)) return false;
    }
    return true;
}

static bool in_range(double value, double minimum, double maximum) {
    return isfinite(value) && value >= minimum && value <= maximum;
}

static bool valid_preset(const eptz_operator_preset_t *preset,
                         bool require_uuid) {
    return preset && lightnvr_uuid_is_valid(preset->camera_uuid) &&
        (!require_uuid || lightnvr_uuid_is_valid(preset->uuid)) &&
        preset->owner_user_id >= 0 && valid_name(preset->name) &&
        (strcmp(preset->mode, "raw") == 0 ||
         strcmp(preset->mode, "dewarp") == 0 ||
         strcmp(preset->mode, "panorama") == 0 ||
         strcmp(preset->mode, "dual") == 0) &&
        in_range(preset->yaw, -180, 180) &&
        in_range(preset->tilt, -90, 30) &&
        in_range(preset->view_fov, 20, 120) &&
        in_range(preset->secondary_yaw, -180, 180) &&
        in_range(preset->secondary_tilt, -90, 30) &&
        in_range(preset->secondary_view_fov, 20, 120);
}

static db_eptz_preset_result_t get_locked(
    sqlite3 *db, const char *camera_uuid, int64_t owner_user_id,
    const char *uuid, bool visible, eptz_operator_preset_t *preset) {
    const char *visibility = visible
        ? "(is_shared=1 OR ((?=0 AND owner_user_id IS NULL) OR owner_user_id=?))"
        : "((?=0 AND owner_user_id IS NULL) OR owner_user_id=?)";
    char sql[768];
    int written = snprintf(
        sql, sizeof(sql), "SELECT " PRESET_FIELDS
        " FROM eptz_operator_presets WHERE camera_uuid=? AND uuid=? AND %s "
        "LIMIT 1;", visibility);
    if (written < 0 || (size_t)written >= sizeof(sql)) {
        return DB_EPTZ_PRESET_ERROR;
    }
    sqlite3_stmt *statement = NULL;
    int result = sqlite3_prepare_v2(db, sql, -1, &statement, NULL);
    if (result == SQLITE_OK) {
        sqlite3_bind_text(statement, 1, camera_uuid, -1, SQLITE_TRANSIENT);
        sqlite3_bind_text(statement, 2, uuid, -1, SQLITE_TRANSIENT);
        bind_owner(statement, 3, owner_user_id);
        bind_owner(statement, 4, owner_user_id);
        result = sqlite3_step(statement);
    }
    db_eptz_preset_result_t outcome = DB_EPTZ_PRESET_NOT_FOUND;
    if (result == SQLITE_ROW) {
        populate(statement, preset);
        outcome = DB_EPTZ_PRESET_OK;
    } else if (result != SQLITE_DONE) {
        outcome = DB_EPTZ_PRESET_ERROR;
    }
    if (statement) sqlite3_finalize(statement);
    return outcome;
}

int db_eptz_preset_list_visible(const char *camera_uuid,
                                int64_t owner_user_id,
                                eptz_operator_preset_t *presets,
                                int max_count) {
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    if (!db || !mutex || !presets || !lightnvr_uuid_is_valid(camera_uuid) ||
        owner_user_id < 0 || max_count < 1 ||
        max_count > EPTZ_PRESET_MAX_VISIBLE) return -1;
    const char *sql =
        "SELECT " PRESET_FIELDS " FROM eptz_operator_presets "
        "WHERE camera_uuid=? AND (is_shared=1 OR "
        "((?=0 AND owner_user_id IS NULL) OR owner_user_id=?)) "
        "ORDER BY is_shared DESC,name COLLATE NOCASE,uuid LIMIT ?;";
    pthread_mutex_lock(mutex);
    sqlite3_stmt *statement = NULL;
    int result = sqlite3_prepare_v2(db, sql, -1, &statement, NULL);
    if (result == SQLITE_OK) {
        sqlite3_bind_text(statement, 1, camera_uuid, -1, SQLITE_TRANSIENT);
        bind_owner(statement, 2, owner_user_id);
        bind_owner(statement, 3, owner_user_id);
        sqlite3_bind_int(statement, 4, max_count);
    }
    int count = 0;
    if (result == SQLITE_OK) {
        while (count < max_count &&
               (result = sqlite3_step(statement)) == SQLITE_ROW) {
            populate(statement, &presets[count++]);
        }
    }
    if (result != SQLITE_DONE && result != SQLITE_ROW) count = -1;
    if (statement) sqlite3_finalize(statement);
    pthread_mutex_unlock(mutex);
    return count;
}

db_eptz_preset_result_t db_eptz_preset_get_visible(
    const char *camera_uuid, int64_t owner_user_id, const char *uuid,
    eptz_operator_preset_t *preset) {
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    if (!db || !mutex || !preset || owner_user_id < 0 ||
        !lightnvr_uuid_is_valid(camera_uuid) ||
        !lightnvr_uuid_is_valid(uuid)) return DB_EPTZ_PRESET_INVALID;
    pthread_mutex_lock(mutex);
    db_eptz_preset_result_t result = get_locked(
        db, camera_uuid, owner_user_id, uuid, true, preset);
    pthread_mutex_unlock(mutex);
    return result;
}

db_eptz_preset_result_t db_eptz_preset_create(
    eptz_operator_preset_t *preset) {
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    char name[EPTZ_PRESET_NAME_MAX];
    if (!db || !mutex || !valid_preset(preset, false) ||
        copy_trimmed_value(name, sizeof(name), preset->name, 0) == 0 ||
        lightnvr_uuid_generate_v4(preset->uuid) != 0) {
        return DB_EPTZ_PRESET_INVALID;
    }
    pthread_mutex_lock(mutex);
    const char *count_sql =
        "SELECT count(*) FROM eptz_operator_presets WHERE camera_uuid=? AND "
        "((?=0 AND owner_user_id IS NULL) OR owner_user_id=?);";
    sqlite3_stmt *statement = NULL;
    int result = sqlite3_prepare_v2(db, count_sql, -1, &statement, NULL);
    int count = -1;
    if (result == SQLITE_OK) {
        sqlite3_bind_text(statement, 1, preset->camera_uuid, -1,
                          SQLITE_TRANSIENT);
        bind_owner(statement, 2, preset->owner_user_id);
        bind_owner(statement, 3, preset->owner_user_id);
        if (sqlite3_step(statement) == SQLITE_ROW) {
            count = sqlite3_column_int(statement, 0);
        }
    }
    if (statement) sqlite3_finalize(statement);
    if (count < 0 || count >= EPTZ_PRESET_MAX_VISIBLE) {
        pthread_mutex_unlock(mutex);
        return count < 0 ? DB_EPTZ_PRESET_ERROR : DB_EPTZ_PRESET_LIMIT;
    }
    const char *sql =
        "INSERT INTO eptz_operator_presets(uuid,camera_uuid,owner_user_id,"
        "name,is_shared,mode,yaw,tilt,view_fov,secondary_yaw,secondary_tilt,"
        "secondary_view_fov) VALUES(?,?,?,?,?,?,?,?,?,?,?,?);";
    statement = NULL;
    result = sqlite3_prepare_v2(db, sql, -1, &statement, NULL);
    if (result == SQLITE_OK) {
        sqlite3_bind_text(statement, 1, preset->uuid, -1, SQLITE_TRANSIENT);
        sqlite3_bind_text(statement, 2, preset->camera_uuid, -1,
                          SQLITE_TRANSIENT);
        if (preset->owner_user_id > 0) {
            sqlite3_bind_int64(statement, 3, preset->owner_user_id);
        } else {
            sqlite3_bind_null(statement, 3);
        }
        sqlite3_bind_text(statement, 4, name, -1, SQLITE_TRANSIENT);
        sqlite3_bind_int(statement, 5, preset->is_shared ? 1 : 0);
        sqlite3_bind_text(statement, 6, preset->mode, -1, SQLITE_TRANSIENT);
        sqlite3_bind_double(statement, 7, preset->yaw);
        sqlite3_bind_double(statement, 8, preset->tilt);
        sqlite3_bind_double(statement, 9, preset->view_fov);
        sqlite3_bind_double(statement, 10, preset->secondary_yaw);
        sqlite3_bind_double(statement, 11, preset->secondary_tilt);
        sqlite3_bind_double(statement, 12, preset->secondary_view_fov);
        result = sqlite3_step(statement);
    }
    if (statement) sqlite3_finalize(statement);
    if (result != SQLITE_DONE) {
        pthread_mutex_unlock(mutex);
        return result == SQLITE_CONSTRAINT ? DB_EPTZ_PRESET_CONFLICT
                                           : DB_EPTZ_PRESET_ERROR;
    }
    safe_strcpy(preset->name, name, sizeof(preset->name), 0);
    db_eptz_preset_result_t outcome = get_locked(
        db, preset->camera_uuid, preset->owner_user_id, preset->uuid, false,
        preset);
    pthread_mutex_unlock(mutex);
    return outcome;
}

db_eptz_preset_result_t db_eptz_preset_update(
    eptz_operator_preset_t *preset, int64_t expected_revision) {
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    char name[EPTZ_PRESET_NAME_MAX];
    if (!db || !mutex || expected_revision < 1 ||
        !valid_preset(preset, true) ||
        copy_trimmed_value(name, sizeof(name), preset->name, 0) == 0) {
        return DB_EPTZ_PRESET_INVALID;
    }
    pthread_mutex_lock(mutex);
    eptz_operator_preset_t existing;
    db_eptz_preset_result_t outcome = get_locked(
        db, preset->camera_uuid, preset->owner_user_id, preset->uuid, false,
        &existing);
    if (outcome != DB_EPTZ_PRESET_OK || existing.revision != expected_revision) {
        pthread_mutex_unlock(mutex);
        return outcome == DB_EPTZ_PRESET_OK ? DB_EPTZ_PRESET_STALE : outcome;
    }
    const char *sql =
        "UPDATE eptz_operator_presets SET name=?,is_shared=?,mode=?,yaw=?,"
        "tilt=?,view_fov=?,secondary_yaw=?,secondary_tilt=?,"
        "secondary_view_fov=?,revision=revision+1,"
        "updated_at=strftime('%s','now') WHERE camera_uuid=? AND uuid=? AND "
        "revision=? AND ((?=0 AND owner_user_id IS NULL) OR owner_user_id=?);";
    sqlite3_stmt *statement = NULL;
    int result = sqlite3_prepare_v2(db, sql, -1, &statement, NULL);
    if (result == SQLITE_OK) {
        sqlite3_bind_text(statement, 1, name, -1, SQLITE_TRANSIENT);
        sqlite3_bind_int(statement, 2, preset->is_shared ? 1 : 0);
        sqlite3_bind_text(statement, 3, preset->mode, -1, SQLITE_TRANSIENT);
        sqlite3_bind_double(statement, 4, preset->yaw);
        sqlite3_bind_double(statement, 5, preset->tilt);
        sqlite3_bind_double(statement, 6, preset->view_fov);
        sqlite3_bind_double(statement, 7, preset->secondary_yaw);
        sqlite3_bind_double(statement, 8, preset->secondary_tilt);
        sqlite3_bind_double(statement, 9, preset->secondary_view_fov);
        sqlite3_bind_text(statement, 10, preset->camera_uuid, -1,
                          SQLITE_TRANSIENT);
        sqlite3_bind_text(statement, 11, preset->uuid, -1, SQLITE_TRANSIENT);
        sqlite3_bind_int64(statement, 12, expected_revision);
        bind_owner(statement, 13, preset->owner_user_id);
        bind_owner(statement, 14, preset->owner_user_id);
        result = sqlite3_step(statement);
    }
    int changed = result == SQLITE_DONE ? sqlite3_changes(db) : 0;
    if (statement) sqlite3_finalize(statement);
    if (result != SQLITE_DONE || changed != 1) {
        pthread_mutex_unlock(mutex);
        if (result == SQLITE_CONSTRAINT) return DB_EPTZ_PRESET_CONFLICT;
        return result == SQLITE_DONE ? DB_EPTZ_PRESET_STALE
                                     : DB_EPTZ_PRESET_ERROR;
    }
    outcome = get_locked(db, preset->camera_uuid, preset->owner_user_id,
                         preset->uuid, false, preset);
    pthread_mutex_unlock(mutex);
    return outcome;
}

db_eptz_preset_result_t db_eptz_preset_delete(
    const char *camera_uuid, int64_t owner_user_id, const char *uuid,
    int64_t expected_revision) {
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    if (!db || !mutex || owner_user_id < 0 || expected_revision < 1 ||
        !lightnvr_uuid_is_valid(camera_uuid) ||
        !lightnvr_uuid_is_valid(uuid)) return DB_EPTZ_PRESET_INVALID;
    pthread_mutex_lock(mutex);
    eptz_operator_preset_t existing;
    db_eptz_preset_result_t outcome = get_locked(
        db, camera_uuid, owner_user_id, uuid, false, &existing);
    if (outcome != DB_EPTZ_PRESET_OK || existing.revision != expected_revision) {
        pthread_mutex_unlock(mutex);
        return outcome == DB_EPTZ_PRESET_OK ? DB_EPTZ_PRESET_STALE : outcome;
    }
    const char *sql =
        "DELETE FROM eptz_operator_presets WHERE camera_uuid=? AND uuid=? AND "
        "revision=? AND ((?=0 AND owner_user_id IS NULL) OR owner_user_id=?);";
    sqlite3_stmt *statement = NULL;
    int result = sqlite3_prepare_v2(db, sql, -1, &statement, NULL);
    if (result == SQLITE_OK) {
        sqlite3_bind_text(statement, 1, camera_uuid, -1, SQLITE_TRANSIENT);
        sqlite3_bind_text(statement, 2, uuid, -1, SQLITE_TRANSIENT);
        sqlite3_bind_int64(statement, 3, expected_revision);
        bind_owner(statement, 4, owner_user_id);
        bind_owner(statement, 5, owner_user_id);
        result = sqlite3_step(statement);
    }
    int changed = result == SQLITE_DONE ? sqlite3_changes(db) : 0;
    if (statement) sqlite3_finalize(statement);
    pthread_mutex_unlock(mutex);
    if (result != SQLITE_DONE) return DB_EPTZ_PRESET_ERROR;
    return changed == 1 ? DB_EPTZ_PRESET_OK : DB_EPTZ_PRESET_STALE;
}
