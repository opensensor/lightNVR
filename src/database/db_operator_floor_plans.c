#define _POSIX_C_SOURCE 200809L

#include "database/db_operator_floor_plans.h"

#include <ctype.h>
#include <math.h>
#include <pthread.h>
#include <sqlite3.h>
#include <stdio.h>
#include <string.h>

#include "database/db_core.h"
#include "utils/strings.h"
#include "utils/uuid.h"

#define PLAN_FIELDS \
    "uuid,name,COALESCE(location_uuid,''),COALESCE(parent_plan_uuid,'')," \
    "canvas_width,canvas_height,COALESCE(background_mime,'')," \
    "revision,created_at,updated_at"

static void copy_column(char *destination, size_t size,
                        sqlite3_stmt *statement, int column) {
    const char *value = (const char *)sqlite3_column_text(statement, column);
    safe_strcpy(destination, value ? value : "", size, 0);
}

static void populate_plan(sqlite3_stmt *statement,
                          operator_floor_plan_t *plan) {
    memset(plan, 0, sizeof(*plan));
    copy_column(plan->uuid, sizeof(plan->uuid), statement, 0);
    copy_column(plan->name, sizeof(plan->name), statement, 1);
    copy_column(plan->location_uuid, sizeof(plan->location_uuid), statement, 2);
    copy_column(plan->parent_plan_uuid, sizeof(plan->parent_plan_uuid),
                statement, 3);
    plan->canvas_width = sqlite3_column_int(statement, 4);
    plan->canvas_height = sqlite3_column_int(statement, 5);
    copy_column(plan->background_mime, sizeof(plan->background_mime),
                statement, 6);
    plan->revision = sqlite3_column_int64(statement, 7);
    plan->created_at = sqlite3_column_int64(statement, 8);
    plan->updated_at = sqlite3_column_int64(statement, 9);
}

static bool valid_name(const char *name) {
    size_t length = name ? strnlen(name, OPERATOR_FLOOR_PLAN_NAME_MAX) : 0;
    if (length == 0 || length == OPERATOR_FLOOR_PLAN_NAME_MAX) return false;
    for (const unsigned char *cursor = (const unsigned char *)name; *cursor;
         cursor++) {
        if (iscntrl(*cursor)) return false;
    }
    return true;
}

static bool valid_background_mime(const char *background_mime) {
    return background_mime[0] == '\0' ||
        strcmp(background_mime, "image/png") == 0 ||
        strcmp(background_mime, "image/jpeg") == 0;
}

static bool valid_plan(const operator_floor_plan_t *plan, bool require_uuid) {
    return plan && (!require_uuid || lightnvr_uuid_is_valid(plan->uuid)) &&
        valid_name(plan->name) &&
        (plan->location_uuid[0] == '\0' ||
         lightnvr_uuid_is_valid(plan->location_uuid)) &&
        (plan->parent_plan_uuid[0] == '\0' ||
         lightnvr_uuid_is_valid(plan->parent_plan_uuid)) &&
        (plan->parent_plan_uuid[0] == '\0' || plan->uuid[0] == '\0' ||
         strcmp(plan->uuid, plan->parent_plan_uuid) != 0) &&
        plan->canvas_width >= 400 && plan->canvas_width <= 4000 &&
        plan->canvas_height >= 300 && plan->canvas_height <= 4000 &&
        valid_background_mime(plan->background_mime);
}

static bool valid_cameras(const operator_floor_plan_camera_t *cameras,
                          int count) {
    if (count < 0 || count > OPERATOR_FLOOR_PLAN_MAX_CAMERAS ||
        (count > 0 && !cameras)) return false;
    for (int index = 0; index < count; index++) {
        const operator_floor_plan_camera_t *camera = &cameras[index];
        if (!lightnvr_uuid_is_valid(camera->camera_uuid) ||
            !isfinite(camera->x) || camera->x < 0.0 || camera->x > 1.0 ||
            !isfinite(camera->y) || camera->y < 0.0 || camera->y > 1.0 ||
            !isfinite(camera->rotation) || camera->rotation < -180.0 ||
            camera->rotation > 180.0 || !isfinite(camera->fov) ||
            camera->fov < 1.0 || camera->fov > 180.0) return false;
        for (int previous = 0; previous < index; previous++) {
            if (strcmp(cameras[previous].camera_uuid,
                       camera->camera_uuid) == 0) return false;
        }
    }
    return true;
}

static db_operator_floor_plan_result_t get_locked(
    sqlite3 *database, const char *uuid, operator_floor_plan_t *plan) {
    sqlite3_stmt *statement = NULL;
    int result = sqlite3_prepare_v2(
        database, "SELECT " PLAN_FIELDS
        " FROM operator_floor_plans WHERE uuid=? LIMIT 1;",
        -1, &statement, NULL);
    if (result == SQLITE_OK) {
        sqlite3_bind_text(statement, 1, uuid, -1, SQLITE_TRANSIENT);
        result = sqlite3_step(statement);
    }
    db_operator_floor_plan_result_t outcome =
        DB_OPERATOR_FLOOR_PLAN_NOT_FOUND;
    if (result == SQLITE_ROW) {
        populate_plan(statement, plan);
        outcome = DB_OPERATOR_FLOOR_PLAN_OK;
    } else if (result != SQLITE_DONE) {
        outcome = DB_OPERATOR_FLOOR_PLAN_ERROR;
    }
    if (statement) sqlite3_finalize(statement);
    return outcome;
}

static int insert_cameras_locked(
    sqlite3 *database, const char *plan_uuid,
    const operator_floor_plan_camera_t *cameras, int camera_count) {
    sqlite3_stmt *statement = NULL;
    int result = sqlite3_prepare_v2(
        database,
        "INSERT INTO operator_floor_plan_cameras"
        "(plan_uuid,camera_uuid,x,y,rotation,fov) VALUES(?,?,?,?,?,?);",
        -1, &statement, NULL);
    for (int index = 0; result == SQLITE_OK && index < camera_count; index++) {
        sqlite3_bind_text(statement, 1, plan_uuid, -1, SQLITE_TRANSIENT);
        sqlite3_bind_text(statement, 2, cameras[index].camera_uuid, -1,
                          SQLITE_TRANSIENT);
        sqlite3_bind_double(statement, 3, cameras[index].x);
        sqlite3_bind_double(statement, 4, cameras[index].y);
        sqlite3_bind_double(statement, 5, cameras[index].rotation);
        sqlite3_bind_double(statement, 6, cameras[index].fov);
        result = sqlite3_step(statement);
        if (result == SQLITE_DONE && index + 1 < camera_count) {
            result = sqlite3_reset(statement);
            sqlite3_clear_bindings(statement);
        }
    }
    if (statement) sqlite3_finalize(statement);
    return camera_count == 0 || result == SQLITE_DONE ? SQLITE_OK : result;
}

static bool parent_would_cycle_locked(sqlite3 *database,
                                      const char *plan_uuid,
                                      const char *parent_uuid) {
    if (!parent_uuid || parent_uuid[0] == '\0') return false;
    sqlite3_stmt *statement = NULL;
    int result = sqlite3_prepare_v2(
        database,
        "WITH RECURSIVE ancestors(uuid) AS ("
        "SELECT ? UNION ALL SELECT p.parent_plan_uuid "
        "FROM operator_floor_plans p JOIN ancestors a ON p.uuid=a.uuid "
        "WHERE p.parent_plan_uuid IS NOT NULL) "
        "SELECT 1 FROM ancestors WHERE uuid=? LIMIT 1;",
        -1, &statement, NULL);
    if (result == SQLITE_OK) {
        sqlite3_bind_text(statement, 1, parent_uuid, -1, SQLITE_TRANSIENT);
        sqlite3_bind_text(statement, 2, plan_uuid, -1, SQLITE_TRANSIENT);
        result = sqlite3_step(statement);
    }
    bool cycle = result == SQLITE_ROW || result != SQLITE_DONE;
    if (statement) sqlite3_finalize(statement);
    return cycle;
}

int db_operator_floor_plan_list(operator_floor_plan_t *plans, int max_count) {
    sqlite3 *database = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    if (!database || !mutex || !plans || max_count < 1 ||
        max_count > OPERATOR_FLOOR_PLAN_MAX_VISIBLE) return -1;
    pthread_mutex_lock(mutex);
    sqlite3_stmt *statement = NULL;
    int result = sqlite3_prepare_v2(
        database, "SELECT " PLAN_FIELDS
        " FROM operator_floor_plans ORDER BY name COLLATE NOCASE,uuid LIMIT ?;",
        -1, &statement, NULL);
    if (result == SQLITE_OK) sqlite3_bind_int(statement, 1, max_count);
    int count = 0;
    if (result == SQLITE_OK) {
        while (count < max_count &&
               (result = sqlite3_step(statement)) == SQLITE_ROW) {
            populate_plan(statement, &plans[count++]);
        }
    }
    if (result != SQLITE_DONE && result != SQLITE_ROW) count = -1;
    if (statement) sqlite3_finalize(statement);
    pthread_mutex_unlock(mutex);
    return count;
}

db_operator_floor_plan_result_t db_operator_floor_plan_get(
    const char *uuid, operator_floor_plan_t *plan) {
    sqlite3 *database = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    if (!database || !mutex || !plan || !lightnvr_uuid_is_valid(uuid)) {
        return DB_OPERATOR_FLOOR_PLAN_INVALID;
    }
    pthread_mutex_lock(mutex);
    db_operator_floor_plan_result_t result = get_locked(database, uuid, plan);
    pthread_mutex_unlock(mutex);
    return result;
}

int db_operator_floor_plan_camera_list(
    const char *plan_uuid, operator_floor_plan_camera_t *cameras,
    int max_count) {
    sqlite3 *database = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    if (!database || !mutex || !lightnvr_uuid_is_valid(plan_uuid) ||
        !cameras || max_count < 1 ||
        max_count > OPERATOR_FLOOR_PLAN_MAX_CAMERAS) return -1;
    pthread_mutex_lock(mutex);
    sqlite3_stmt *statement = NULL;
    int result = sqlite3_prepare_v2(
        database,
        "SELECT camera_uuid,x,y,rotation,fov "
        "FROM operator_floor_plan_cameras WHERE plan_uuid=? "
        "ORDER BY camera_uuid LIMIT ?;", -1, &statement, NULL);
    if (result == SQLITE_OK) {
        sqlite3_bind_text(statement, 1, plan_uuid, -1, SQLITE_TRANSIENT);
        sqlite3_bind_int(statement, 2, max_count);
    }
    int count = 0;
    if (result == SQLITE_OK) {
        while (count < max_count &&
               (result = sqlite3_step(statement)) == SQLITE_ROW) {
            operator_floor_plan_camera_t *camera = &cameras[count++];
            memset(camera, 0, sizeof(*camera));
            copy_column(camera->camera_uuid, sizeof(camera->camera_uuid),
                        statement, 0);
            camera->x = sqlite3_column_double(statement, 1);
            camera->y = sqlite3_column_double(statement, 2);
            camera->rotation = sqlite3_column_double(statement, 3);
            camera->fov = sqlite3_column_double(statement, 4);
        }
    }
    if (result != SQLITE_DONE && result != SQLITE_ROW) count = -1;
    if (statement) sqlite3_finalize(statement);
    pthread_mutex_unlock(mutex);
    return count;
}

db_operator_floor_plan_result_t db_operator_floor_plan_create(
    operator_floor_plan_t *plan,
    const operator_floor_plan_camera_t *cameras, int camera_count) {
    sqlite3 *database = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    char name[OPERATOR_FLOOR_PLAN_NAME_MAX];
    if (!database || !mutex || !valid_plan(plan, false) ||
        !valid_cameras(cameras, camera_count) ||
        copy_trimmed_value(name, sizeof(name), plan->name, 0) == 0 ||
        lightnvr_uuid_generate_v4(plan->uuid) != 0) {
        return DB_OPERATOR_FLOOR_PLAN_INVALID;
    }
    pthread_mutex_lock(mutex);
    sqlite3_stmt *statement = NULL;
    int result = sqlite3_prepare_v2(
        database, "SELECT COUNT(*) FROM operator_floor_plans;",
        -1, &statement, NULL);
    int plan_count = -1;
    if (result == SQLITE_OK && sqlite3_step(statement) == SQLITE_ROW) {
        plan_count = sqlite3_column_int(statement, 0);
    }
    if (statement) sqlite3_finalize(statement);
    if (plan_count < 0 || plan_count >= OPERATOR_FLOOR_PLAN_MAX_VISIBLE) {
        pthread_mutex_unlock(mutex);
        return plan_count < 0 ? DB_OPERATOR_FLOOR_PLAN_ERROR
                              : DB_OPERATOR_FLOOR_PLAN_LIMIT;
    }
    sqlite3_exec(database, "BEGIN IMMEDIATE;", NULL, NULL, NULL);
    statement = NULL;
    result = sqlite3_prepare_v2(
        database,
        "INSERT INTO operator_floor_plans"
        "(uuid,name,location_uuid,parent_plan_uuid,canvas_width,canvas_height) "
        "VALUES(?,?,?,?,?,?);", -1, &statement, NULL);
    if (result == SQLITE_OK) {
        sqlite3_bind_text(statement, 1, plan->uuid, -1, SQLITE_TRANSIENT);
        sqlite3_bind_text(statement, 2, name, -1, SQLITE_TRANSIENT);
        if (plan->location_uuid[0]) sqlite3_bind_text(
            statement, 3, plan->location_uuid, -1, SQLITE_TRANSIENT);
        else sqlite3_bind_null(statement, 3);
        if (plan->parent_plan_uuid[0]) sqlite3_bind_text(
            statement, 4, plan->parent_plan_uuid, -1, SQLITE_TRANSIENT);
        else sqlite3_bind_null(statement, 4);
        sqlite3_bind_int(statement, 5, plan->canvas_width);
        sqlite3_bind_int(statement, 6, plan->canvas_height);
        result = sqlite3_step(statement);
    }
    if (statement) sqlite3_finalize(statement);
    if (result == SQLITE_DONE) {
        result = insert_cameras_locked(database, plan->uuid, cameras,
                                       camera_count);
    }
    if (result != SQLITE_OK) {
        sqlite3_exec(database, "ROLLBACK;", NULL, NULL, NULL);
        pthread_mutex_unlock(mutex);
        return result == SQLITE_CONSTRAINT
            ? DB_OPERATOR_FLOOR_PLAN_CONFLICT
            : DB_OPERATOR_FLOOR_PLAN_ERROR;
    }
    sqlite3_exec(database, "COMMIT;", NULL, NULL, NULL);
    safe_strcpy(plan->name, name, sizeof(plan->name), 0);
    db_operator_floor_plan_result_t outcome = get_locked(
        database, plan->uuid, plan);
    pthread_mutex_unlock(mutex);
    return outcome;
}

db_operator_floor_plan_result_t db_operator_floor_plan_update(
    operator_floor_plan_t *plan,
    const operator_floor_plan_camera_t *cameras, int camera_count,
    int64_t expected_revision) {
    sqlite3 *database = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    char name[OPERATOR_FLOOR_PLAN_NAME_MAX];
    if (!database || !mutex || expected_revision < 1 ||
        !valid_plan(plan, true) || !valid_cameras(cameras, camera_count) ||
        copy_trimmed_value(name, sizeof(name), plan->name, 0) == 0) {
        return DB_OPERATOR_FLOOR_PLAN_INVALID;
    }
    pthread_mutex_lock(mutex);
    operator_floor_plan_t existing;
    db_operator_floor_plan_result_t outcome = get_locked(
        database, plan->uuid, &existing);
    if (outcome != DB_OPERATOR_FLOOR_PLAN_OK ||
        existing.revision != expected_revision) {
        pthread_mutex_unlock(mutex);
        return outcome == DB_OPERATOR_FLOOR_PLAN_OK
            ? DB_OPERATOR_FLOOR_PLAN_STALE : outcome;
    }
    if (parent_would_cycle_locked(database, plan->uuid,
                                  plan->parent_plan_uuid)) {
        pthread_mutex_unlock(mutex);
        return DB_OPERATOR_FLOOR_PLAN_CONFLICT;
    }
    sqlite3_exec(database, "BEGIN IMMEDIATE;", NULL, NULL, NULL);
    sqlite3_stmt *statement = NULL;
    int result = sqlite3_prepare_v2(
        database,
        "UPDATE operator_floor_plans SET name=?,location_uuid=?,"
        "parent_plan_uuid=?,canvas_width=?,canvas_height=?,"
        "revision=revision+1,updated_at=strftime('%s','now') "
        "WHERE uuid=? AND revision=?;", -1, &statement, NULL);
    if (result == SQLITE_OK) {
        sqlite3_bind_text(statement, 1, name, -1, SQLITE_TRANSIENT);
        if (plan->location_uuid[0]) sqlite3_bind_text(
            statement, 2, plan->location_uuid, -1, SQLITE_TRANSIENT);
        else sqlite3_bind_null(statement, 2);
        if (plan->parent_plan_uuid[0]) sqlite3_bind_text(
            statement, 3, plan->parent_plan_uuid, -1, SQLITE_TRANSIENT);
        else sqlite3_bind_null(statement, 3);
        sqlite3_bind_int(statement, 4, plan->canvas_width);
        sqlite3_bind_int(statement, 5, plan->canvas_height);
        sqlite3_bind_text(statement, 6, plan->uuid, -1, SQLITE_TRANSIENT);
        sqlite3_bind_int64(statement, 7, expected_revision);
        result = sqlite3_step(statement);
    }
    int changed = result == SQLITE_DONE ? sqlite3_changes(database) : 0;
    if (statement) sqlite3_finalize(statement);
    if (result == SQLITE_DONE && changed != 1) {
        sqlite3_exec(database, "ROLLBACK;", NULL, NULL, NULL);
        pthread_mutex_unlock(mutex);
        return DB_OPERATOR_FLOOR_PLAN_STALE;
    }
    if (result == SQLITE_DONE) {
        result = sqlite3_prepare_v2(
            database,
            "DELETE FROM operator_floor_plan_cameras WHERE plan_uuid=?;",
            -1, &statement, NULL);
        if (result == SQLITE_OK) {
            sqlite3_bind_text(statement, 1, plan->uuid, -1,
                              SQLITE_TRANSIENT);
            result = sqlite3_step(statement);
        }
        if (statement) sqlite3_finalize(statement);
        statement = NULL;
    }
    if (result == SQLITE_DONE) {
        result = insert_cameras_locked(database, plan->uuid, cameras,
                                       camera_count);
    }
    if (result != SQLITE_OK) {
        sqlite3_exec(database, "ROLLBACK;", NULL, NULL, NULL);
        pthread_mutex_unlock(mutex);
        return result == SQLITE_CONSTRAINT
            ? DB_OPERATOR_FLOOR_PLAN_CONFLICT
            : DB_OPERATOR_FLOOR_PLAN_ERROR;
    }
    sqlite3_exec(database, "COMMIT;", NULL, NULL, NULL);
    safe_strcpy(plan->name, name, sizeof(plan->name), 0);
    outcome = get_locked(database, plan->uuid, plan);
    pthread_mutex_unlock(mutex);
    return outcome;
}

db_operator_floor_plan_result_t db_operator_floor_plan_delete(
    const char *uuid, int64_t expected_revision) {
    sqlite3 *database = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    if (!database || !mutex || !lightnvr_uuid_is_valid(uuid) ||
        expected_revision < 1) return DB_OPERATOR_FLOOR_PLAN_INVALID;
    pthread_mutex_lock(mutex);
    sqlite3_stmt *statement = NULL;
    int result = sqlite3_prepare_v2(
        database,
        "DELETE FROM operator_floor_plans WHERE uuid=? AND revision=?;",
        -1, &statement, NULL);
    if (result == SQLITE_OK) {
        sqlite3_bind_text(statement, 1, uuid, -1, SQLITE_TRANSIENT);
        sqlite3_bind_int64(statement, 2, expected_revision);
        result = sqlite3_step(statement);
    }
    int changed = result == SQLITE_DONE ? sqlite3_changes(database) : 0;
    if (statement) sqlite3_finalize(statement);
    pthread_mutex_unlock(mutex);
    if (result != SQLITE_DONE) return DB_OPERATOR_FLOOR_PLAN_ERROR;
    return changed == 1 ? DB_OPERATOR_FLOOR_PLAN_OK
                        : DB_OPERATOR_FLOOR_PLAN_STALE;
}

db_operator_floor_plan_result_t db_operator_floor_plan_set_background(
    const char *uuid, const char *background_mime) {
    sqlite3 *database = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    if (!database || !mutex || !lightnvr_uuid_is_valid(uuid) ||
        !valid_background_mime(background_mime ? background_mime : "")) {
        return DB_OPERATOR_FLOOR_PLAN_INVALID;
    }
    // The in-memory representation uses an empty string for SQL NULL.
    // Accept that same representation here so callers do not accidentally
    // attempt to store a value rejected by the table CHECK constraint.
    if (background_mime && background_mime[0] == '\0') background_mime = NULL;
    pthread_mutex_lock(mutex);
    operator_floor_plan_t existing;
    db_operator_floor_plan_result_t outcome =
        get_locked(database, uuid, &existing);
    if (outcome == DB_OPERATOR_FLOOR_PLAN_OK) {
        sqlite3_stmt *statement = NULL;
        int result = sqlite3_prepare_v2(
            database,
            "UPDATE operator_floor_plans SET background_mime=?,"
            "updated_at=strftime('%s','now') WHERE uuid=?;",
            -1, &statement, NULL);
        if (result == SQLITE_OK) {
            if (background_mime) sqlite3_bind_text(
                statement, 1, background_mime, -1, SQLITE_TRANSIENT);
            else sqlite3_bind_null(statement, 1);
            sqlite3_bind_text(statement, 2, uuid, -1, SQLITE_TRANSIENT);
            result = sqlite3_step(statement);
        }
        if (statement) sqlite3_finalize(statement);
        outcome = result == SQLITE_DONE ? DB_OPERATOR_FLOOR_PLAN_OK
                                        : DB_OPERATOR_FLOOR_PLAN_ERROR;
    }
    pthread_mutex_unlock(mutex);
    return outcome;
}
