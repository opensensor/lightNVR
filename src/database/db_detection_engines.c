#include "database/db_detection_engines.h"

#include <ctype.h>
#include <sqlite3.h>
#include <stdio.h>
#include <string.h>

#include "cJSON.h"
#include "core/logger.h"
#include "database/db_core.h"

static int copy_column(char *dst, size_t dst_size, sqlite3_stmt *stmt, int column) {
    const unsigned char *value = sqlite3_column_text(stmt, column);
    if (!dst || dst_size == 0) return -1;
    snprintf(dst, dst_size, "%s", value ? (const char *)value : "");
    return 0;
}

static bool valid_key(const char *key) {
    if (!key || !key[0] || strlen(key) >= DETECTION_ENGINE_KEY_MAX) return false;
    for (const unsigned char *p = (const unsigned char *)key; *p; ++p) {
        if (!(islower(*p) || isdigit(*p) || *p == '-' || *p == '_')) return false;
    }
    return true;
}

static bool valid_type(const char *type) {
    static const char *types[] = {"motion", "object", "onvif", "api", "external"};
    for (size_t i = 0; i < sizeof(types) / sizeof(types[0]); ++i) {
        if (type && strcmp(type, types[i]) == 0) return true;
    }
    return false;
}

static int validation_error(char *error, size_t error_size, const char *message) {
    if (error && error_size > 0) snprintf(error, error_size, "%s", message);
    return -1;
}

int db_detection_engine_validate(const stream_detection_engine_t *engine,
                                 char *error, size_t error_size) {
    if (!engine) return validation_error(error, error_size, "engine is required");
    if (!valid_key(engine->engine_key))
        return validation_error(error, error_size, "invalid engine_key");
    if (strcmp(engine->engine_key, "legacy-primary") == 0)
        return validation_error(error, error_size, "legacy-primary is managed by stream settings");
    if (!valid_type(engine->engine_type))
        return validation_error(error, error_size, "invalid engine_type");
    if (engine->threshold < 0.0f || engine->threshold > 1.0f)
        return validation_error(error, error_size, "threshold must be between 0 and 1");
    if (engine->interval_seconds < 1 || engine->interval_seconds > 86400)
        return validation_error(error, error_size, "interval_seconds must be between 1 and 86400");
    if (strlen(engine->model_path) >= MAX_PATH_LENGTH)
        return validation_error(error, error_size, "model_path is too long");
    if (strcmp(engine->engine_type, "object") == 0 && engine->model_path[0] == '\0')
        return validation_error(error, error_size, "object engines require model_path");
    if (strcmp(engine->engine_type, "api") == 0 && engine->model_path[0] == '\0')
        return validation_error(error, error_size, "api engines require model_path");
    if (strcmp(engine->engine_type, "motion") == 0 && engine->model_path[0] != '\0' &&
        strcmp(engine->model_path, "motion") != 0)
        return validation_error(error, error_size, "motion model_path must be empty or motion");
    if (strcmp(engine->engine_type, "onvif") == 0 && engine->model_path[0] != '\0' &&
        strcmp(engine->model_path, "onvif") != 0)
        return validation_error(error, error_size, "onvif model_path must be empty or onvif");

    const char *json = engine->config_json[0] ? engine->config_json : "{}";
    cJSON *parsed = cJSON_Parse(json);
    if (!parsed || !cJSON_IsObject(parsed)) {
        cJSON_Delete(parsed);
        return validation_error(error, error_size, "config_json must be a JSON object");
    }
    cJSON_Delete(parsed);
    if (error && error_size > 0) error[0] = '\0';
    return 0;
}

int db_detection_engines_list(const char *stream_name,
                              stream_detection_engine_t *engines,
                              size_t capacity) {
    if (!stream_name || !stream_name[0] || (!engines && capacity > 0)) return -1;

    sqlite3 *db = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    if (!db || !mutex) return -1;

    const char *sql =
        "SELECT e.id, e.engine_key, e.engine_type, e.model_path, e.enabled, "
        "e.threshold, e.interval_seconds, e.sort_order, e.config_json "
        "FROM stream_detection_engines e "
        "JOIN streams s ON s.id = e.stream_id "
        "WHERE s.name = ? "
        "ORDER BY e.sort_order, e.id LIMIT ?;";

    pthread_mutex_lock(mutex);
    sqlite3_stmt *stmt = NULL;
    int rc = sqlite3_prepare_v2(db, sql, -1, &stmt, NULL);
    if (rc != SQLITE_OK) {
        log_error("Failed to prepare detection engine list: %s", sqlite3_errmsg(db));
        pthread_mutex_unlock(mutex);
        return -1;
    }
    sqlite3_bind_text(stmt, 1, stream_name, -1, SQLITE_TRANSIENT);
    sqlite3_bind_int64(stmt, 2, (sqlite3_int64)capacity);

    int count = 0;
    while ((rc = sqlite3_step(stmt)) == SQLITE_ROW && (size_t)count < capacity) {
        stream_detection_engine_t *engine = &engines[count++];
        memset(engine, 0, sizeof(*engine));
        engine->id = (uint64_t)sqlite3_column_int64(stmt, 0);
        copy_column(engine->engine_key, sizeof(engine->engine_key), stmt, 1);
        copy_column(engine->engine_type, sizeof(engine->engine_type), stmt, 2);
        copy_column(engine->model_path, sizeof(engine->model_path), stmt, 3);
        engine->enabled = sqlite3_column_int(stmt, 4) != 0;
        engine->threshold = (float)sqlite3_column_double(stmt, 5);
        engine->interval_seconds = sqlite3_column_int(stmt, 6);
        engine->sort_order = sqlite3_column_int(stmt, 7);
        copy_column(engine->config_json, sizeof(engine->config_json), stmt, 8);
    }
    if (rc != SQLITE_DONE) count = -1;
    sqlite3_finalize(stmt);
    pthread_mutex_unlock(mutex);
    return count;
}

int db_detection_engine_upsert(const char *stream_name,
                               const stream_detection_engine_t *engine) {
    char error[128];
    if (!stream_name || !stream_name[0] ||
        db_detection_engine_validate(engine, error, sizeof(error)) != 0) {
        if (engine) log_warn("Rejected detection engine configuration: %s", error);
        return -1;
    }

    sqlite3 *db = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    if (!db || !mutex) return -1;

    const char *sql =
        "INSERT INTO stream_detection_engines ("
        "stream_id, engine_key, engine_type, model_path, enabled, threshold, "
        "interval_seconds, sort_order, config_json) "
        "SELECT id, ?, ?, ?, ?, ?, ?, ?, ? FROM streams WHERE name = ? "
        "ON CONFLICT(stream_id, engine_key) DO UPDATE SET "
        "engine_type=excluded.engine_type, model_path=excluded.model_path, "
        "enabled=excluded.enabled, threshold=excluded.threshold, "
        "interval_seconds=excluded.interval_seconds, sort_order=excluded.sort_order, "
        "config_json=excluded.config_json, updated_at=strftime('%s','now');";

    pthread_mutex_lock(mutex);
    sqlite3_stmt *stmt = NULL;
    int rc = sqlite3_prepare_v2(db, sql, -1, &stmt, NULL);
    if (rc == SQLITE_OK) {
        sqlite3_bind_text(stmt, 1, engine->engine_key, -1, SQLITE_TRANSIENT);
        sqlite3_bind_text(stmt, 2, engine->engine_type, -1, SQLITE_TRANSIENT);
        sqlite3_bind_text(stmt, 3, engine->model_path, -1, SQLITE_TRANSIENT);
        sqlite3_bind_int(stmt, 4, engine->enabled ? 1 : 0);
        sqlite3_bind_double(stmt, 5, engine->threshold);
        sqlite3_bind_int(stmt, 6, engine->interval_seconds);
        sqlite3_bind_int(stmt, 7, engine->sort_order);
        sqlite3_bind_text(stmt, 8,
                          engine->config_json[0] ? engine->config_json : "{}",
                          -1, SQLITE_TRANSIENT);
        sqlite3_bind_text(stmt, 9, stream_name, -1, SQLITE_TRANSIENT);
        rc = sqlite3_step(stmt);
    }
    int changed = sqlite3_changes(db);
    if (stmt) sqlite3_finalize(stmt);
    pthread_mutex_unlock(mutex);
    return (rc == SQLITE_DONE && changed == 1) ? 0 : -1;
}

int db_detection_engine_delete(const char *stream_name,
                               const char *engine_key) {
    if (!stream_name || !stream_name[0] || !valid_key(engine_key) ||
        strcmp(engine_key, "legacy-primary") == 0) return -1;

    sqlite3 *db = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    if (!db || !mutex) return -1;

    const char *sql =
        "DELETE FROM stream_detection_engines "
        "WHERE engine_key = ? AND stream_id = "
        "(SELECT id FROM streams WHERE name = ?);";
    pthread_mutex_lock(mutex);
    sqlite3_stmt *stmt = NULL;
    int rc = sqlite3_prepare_v2(db, sql, -1, &stmt, NULL);
    if (rc == SQLITE_OK) {
        sqlite3_bind_text(stmt, 1, engine_key, -1, SQLITE_TRANSIENT);
        sqlite3_bind_text(stmt, 2, stream_name, -1, SQLITE_TRANSIENT);
        rc = sqlite3_step(stmt);
    }
    int changed = sqlite3_changes(db);
    if (stmt) sqlite3_finalize(stmt);
    pthread_mutex_unlock(mutex);
    return (rc == SQLITE_DONE && changed == 1) ? 0 : -1;
}

int db_detection_engines_replace_custom(
    const char *stream_name, const stream_detection_engine_t *engines,
    size_t count) {
    if (!stream_name || !stream_name[0] || (!engines && count > 0) ||
        count >= MAX_DETECTION_ENGINES_PER_STREAM) return -1;
    for (size_t i = 0; i < count; ++i) {
        if (db_detection_engine_validate(&engines[i], NULL, 0) != 0) return -1;
        for (size_t previous = 0; previous < i; ++previous) {
            if (strcmp(engines[i].engine_key,
                       engines[previous].engine_key) == 0) return -1;
        }
    }

    sqlite3 *db = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    if (!db || !mutex) return -1;
    pthread_mutex_lock(mutex);
    int rc = sqlite3_exec(db, "BEGIN IMMEDIATE;", NULL, NULL, NULL);
    sqlite3_stmt *stream_stmt = NULL;
    sqlite3_stmt *delete_stmt = NULL;
    sqlite3_stmt *insert_stmt = NULL;
    sqlite3_int64 stream_id = 0;
    if (rc == SQLITE_OK) {
        rc = sqlite3_prepare_v2(db, "SELECT id FROM streams WHERE name=?;", -1,
                                &stream_stmt, NULL);
    }
    if (rc == SQLITE_OK) {
        sqlite3_bind_text(stream_stmt, 1, stream_name, -1, SQLITE_TRANSIENT);
        rc = sqlite3_step(stream_stmt);
        if (rc == SQLITE_ROW) {
            stream_id = sqlite3_column_int64(stream_stmt, 0);
            rc = SQLITE_OK;
        } else {
            rc = SQLITE_NOTFOUND;
        }
    }
    if (stream_stmt) sqlite3_finalize(stream_stmt);

    if (rc == SQLITE_OK) {
        rc = sqlite3_prepare_v2(
            db, "DELETE FROM stream_detection_engines "
                "WHERE stream_id=? AND engine_key<>'legacy-primary';",
            -1, &delete_stmt, NULL);
    }
    if (rc == SQLITE_OK) {
        sqlite3_bind_int64(delete_stmt, 1, stream_id);
        rc = sqlite3_step(delete_stmt) == SQLITE_DONE ? SQLITE_OK : SQLITE_ERROR;
    }
    if (delete_stmt) sqlite3_finalize(delete_stmt);

    const char *insert_sql =
        "INSERT INTO stream_detection_engines(stream_id,engine_key,engine_type,"
        "model_path,enabled,threshold,interval_seconds,sort_order,config_json) "
        "VALUES(?,?,?,?,?,?,?,?,?);";
    if (rc == SQLITE_OK && count > 0)
        rc = sqlite3_prepare_v2(db, insert_sql, -1, &insert_stmt, NULL);
    for (size_t i = 0; rc == SQLITE_OK && i < count; ++i) {
        sqlite3_reset(insert_stmt);
        sqlite3_clear_bindings(insert_stmt);
        sqlite3_bind_int64(insert_stmt, 1, stream_id);
        sqlite3_bind_text(insert_stmt, 2, engines[i].engine_key, -1, SQLITE_TRANSIENT);
        sqlite3_bind_text(insert_stmt, 3, engines[i].engine_type, -1, SQLITE_TRANSIENT);
        sqlite3_bind_text(insert_stmt, 4, engines[i].model_path, -1, SQLITE_TRANSIENT);
        sqlite3_bind_int(insert_stmt, 5, engines[i].enabled ? 1 : 0);
        sqlite3_bind_double(insert_stmt, 6, engines[i].threshold);
        sqlite3_bind_int(insert_stmt, 7, engines[i].interval_seconds);
        sqlite3_bind_int(insert_stmt, 8, engines[i].sort_order);
        sqlite3_bind_text(insert_stmt, 9,
                          engines[i].config_json[0] ? engines[i].config_json : "{}",
                          -1, SQLITE_TRANSIENT);
        if (sqlite3_step(insert_stmt) != SQLITE_DONE) rc = SQLITE_ERROR;
    }
    if (insert_stmt) sqlite3_finalize(insert_stmt);
    if (rc == SQLITE_OK) {
        if (sqlite3_exec(db, "COMMIT;", NULL, NULL, NULL) != SQLITE_OK)
            rc = SQLITE_ERROR;
    } else {
        sqlite3_exec(db, "ROLLBACK;", NULL, NULL, NULL);
    }
    pthread_mutex_unlock(mutex);
    return rc == SQLITE_OK ? 0 : -1;
}
