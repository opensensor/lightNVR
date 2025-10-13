#include "database/db_motion_config.h"
#include "database/db_core.h"
#include "core/logger.h"
#include <sqlite3.h>
#include <string.h>
#include <time.h>

/**
 * Motion Recording Configuration Database Implementation
 */

// Initialize the motion recording configuration table
// NOTE: This function is called from within a database transaction during schema migration,
// so it should NOT lock/unlock the database mutex (the transaction already holds the lock)
int init_motion_config_table(void) {
    sqlite3 *db = get_db_handle();
    char *err_msg = NULL;
    int rc;

    if (!db) {
        log_error("Database not initialized");
        return -1;
    }

    // DO NOT lock the mutex here - this is called from within a transaction
    // that already holds the lock. Locking again would cause a deadlock.

    // Create motion_recording_config table
    const char *create_config_table =
        "CREATE TABLE IF NOT EXISTS motion_recording_config ("
        "id INTEGER PRIMARY KEY AUTOINCREMENT,"
        "stream_name TEXT NOT NULL UNIQUE,"
        "enabled INTEGER DEFAULT 1,"
        "pre_buffer_seconds INTEGER DEFAULT 5,"
        "post_buffer_seconds INTEGER DEFAULT 5,"
        "max_file_duration INTEGER DEFAULT 300,"
        "codec TEXT DEFAULT 'h264',"
        "quality TEXT DEFAULT 'medium',"
        "retention_days INTEGER DEFAULT 7,"
        "max_storage_mb INTEGER DEFAULT 0,"  // 0 = unlimited
        "created_at INTEGER NOT NULL,"
        "updated_at INTEGER NOT NULL"
        ");";

    rc = sqlite3_exec(db, create_config_table, NULL, NULL, &err_msg);
    if (rc != SQLITE_OK) {
        log_error("Failed to create motion_recording_config table: %s", err_msg);
        sqlite3_free(err_msg);
        return -1;
    }

    // Create index on stream_name for faster lookups
    const char *create_index =
        "CREATE INDEX IF NOT EXISTS idx_motion_config_stream ON motion_recording_config(stream_name);";

    rc = sqlite3_exec(db, create_index, NULL, NULL, &err_msg);
    if (rc != SQLITE_OK) {
        log_error("Failed to create index on motion_recording_config: %s", err_msg);
        sqlite3_free(err_msg);
        return -1;
    }

    // Create motion_recordings table (separate from regular recordings)
    const char *create_recordings_table =
        "CREATE TABLE IF NOT EXISTS motion_recordings ("
        "id INTEGER PRIMARY KEY AUTOINCREMENT,"
        "stream_name TEXT NOT NULL,"
        "file_path TEXT NOT NULL UNIQUE,"
        "start_time INTEGER NOT NULL,"
        "end_time INTEGER,"
        "size_bytes INTEGER DEFAULT 0,"
        "width INTEGER,"
        "height INTEGER,"
        "fps INTEGER,"
        "codec TEXT,"
        "is_complete INTEGER DEFAULT 0,"
        "created_at INTEGER NOT NULL"
        ");";

    rc = sqlite3_exec(db, create_recordings_table, NULL, NULL, &err_msg);
    if (rc != SQLITE_OK) {
        log_error("Failed to create motion_recordings table: %s", err_msg);
        sqlite3_free(err_msg);
        return -1;
    }

    // Create indexes for faster queries
    const char *create_stream_index =
        "CREATE INDEX IF NOT EXISTS idx_motion_recordings_stream ON motion_recordings(stream_name);";

    rc = sqlite3_exec(db, create_stream_index, NULL, NULL, &err_msg);
    if (rc != SQLITE_OK) {
        log_error("Failed to create stream index: %s", err_msg);
        sqlite3_free(err_msg);
        return -1;
    }

    const char *create_time_index =
        "CREATE INDEX IF NOT EXISTS idx_motion_recordings_time ON motion_recordings(start_time);";

    rc = sqlite3_exec(db, create_time_index, NULL, NULL, &err_msg);
    if (rc != SQLITE_OK) {
        log_error("Failed to create time index: %s", err_msg);
        sqlite3_free(err_msg);
        return -1;
    }

    log_info("Motion recording configuration tables initialized successfully");
    return 0;
}

// Save motion recording configuration for a stream
int save_motion_config(const char *stream_name, const motion_recording_config_t *config) {
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *db_mutex = get_db_mutex();
    sqlite3_stmt *stmt = NULL;
    int rc;

    if (!db || !stream_name || !config) {
        log_error("Invalid parameters for save_motion_config");
        return -1;
    }

    pthread_mutex_lock(db_mutex);

    const char *sql =
        "INSERT OR REPLACE INTO motion_recording_config "
        "(stream_name, enabled, pre_buffer_seconds, post_buffer_seconds, "
        "max_file_duration, codec, quality, retention_days, max_storage_mb, "
        "created_at, updated_at) "
        "VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?);";

    rc = sqlite3_prepare_v2(db, sql, -1, &stmt, NULL);
    if (rc != SQLITE_OK) {
        log_error("Failed to prepare save_motion_config statement: %s", sqlite3_errmsg(db));
        pthread_mutex_unlock(db_mutex);
        return -1;
    }

    time_t now = time(NULL);

    sqlite3_bind_text(stmt, 1, stream_name, -1, SQLITE_STATIC);
    sqlite3_bind_int(stmt, 2, config->enabled ? 1 : 0);
    sqlite3_bind_int(stmt, 3, config->pre_buffer_seconds);
    sqlite3_bind_int(stmt, 4, config->post_buffer_seconds);
    sqlite3_bind_int(stmt, 5, config->max_file_duration);
    sqlite3_bind_text(stmt, 6, config->codec, -1, SQLITE_STATIC);
    sqlite3_bind_text(stmt, 7, config->quality, -1, SQLITE_STATIC);
    sqlite3_bind_int(stmt, 8, config->retention_days);
    sqlite3_bind_int(stmt, 9, 0);  // Default max storage: unlimited
    sqlite3_bind_int64(stmt, 10, now);
    sqlite3_bind_int64(stmt, 11, now);

    rc = sqlite3_step(stmt);
    sqlite3_finalize(stmt);

    if (rc != SQLITE_DONE) {
        log_error("Failed to save motion config for %s: %s", stream_name, sqlite3_errmsg(db));
        pthread_mutex_unlock(db_mutex);
        return -1;
    }

    pthread_mutex_unlock(db_mutex);
    log_info("Saved motion recording config for stream: %s", stream_name);
    return 0;
}

// Load motion recording configuration for a stream
int load_motion_config(const char *stream_name, motion_recording_config_t *config) {
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *db_mutex = get_db_mutex();
    sqlite3_stmt *stmt = NULL;
    int rc;

    if (!db || !stream_name || !config) {
        log_error("Invalid parameters for load_motion_config");
        return -1;
    }

    pthread_mutex_lock(db_mutex);

    const char *sql =
        "SELECT enabled, pre_buffer_seconds, post_buffer_seconds, "
        "max_file_duration, codec, quality, retention_days "
        "FROM motion_recording_config WHERE stream_name = ?;";

    rc = sqlite3_prepare_v2(db, sql, -1, &stmt, NULL);
    if (rc != SQLITE_OK) {
        log_error("Failed to prepare load_motion_config statement: %s", sqlite3_errmsg(db));
        pthread_mutex_unlock(db_mutex);
        return -1;
    }

    sqlite3_bind_text(stmt, 1, stream_name, -1, SQLITE_STATIC);

    rc = sqlite3_step(stmt);
    if (rc == SQLITE_ROW) {
        config->enabled = sqlite3_column_int(stmt, 0) != 0;
        config->pre_buffer_seconds = sqlite3_column_int(stmt, 1);
        config->post_buffer_seconds = sqlite3_column_int(stmt, 2);
        config->max_file_duration = sqlite3_column_int(stmt, 3);

        const char *codec = (const char *)sqlite3_column_text(stmt, 4);
        if (codec) {
            strncpy(config->codec, codec, sizeof(config->codec) - 1);
            config->codec[sizeof(config->codec) - 1] = '\0';
        }

        const char *quality = (const char *)sqlite3_column_text(stmt, 5);
        if (quality) {
            strncpy(config->quality, quality, sizeof(config->quality) - 1);
            config->quality[sizeof(config->quality) - 1] = '\0';
        }

        config->retention_days = sqlite3_column_int(stmt, 6);

        sqlite3_finalize(stmt);
        pthread_mutex_unlock(db_mutex);
        return 0;
    } else if (rc == SQLITE_DONE) {
        // No configuration found
        sqlite3_finalize(stmt);
        pthread_mutex_unlock(db_mutex);
        return -1;
    } else {
        log_error("Failed to load motion config for %s: %s", stream_name, sqlite3_errmsg(db));
        sqlite3_finalize(stmt);
        pthread_mutex_unlock(db_mutex);
        return -1;
    }
}

// Update motion recording configuration for a stream
int update_motion_config(const char *stream_name, const motion_recording_config_t *config) {
    // For SQLite with INSERT OR REPLACE, update is the same as save
    return save_motion_config(stream_name, config);
}

// Delete motion recording configuration for a stream
int delete_motion_config(const char *stream_name) {
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *db_mutex = get_db_mutex();
    sqlite3_stmt *stmt = NULL;
    int rc;

    if (!db || !stream_name) {
        log_error("Invalid parameters for delete_motion_config");
        return -1;
    }

    pthread_mutex_lock(db_mutex);

    const char *sql = "DELETE FROM motion_recording_config WHERE stream_name = ?;";

    rc = sqlite3_prepare_v2(db, sql, -1, &stmt, NULL);
    if (rc != SQLITE_OK) {
        log_error("Failed to prepare delete_motion_config statement: %s", sqlite3_errmsg(db));
        pthread_mutex_unlock(db_mutex);
        return -1;
    }

    sqlite3_bind_text(stmt, 1, stream_name, -1, SQLITE_STATIC);

    rc = sqlite3_step(stmt);
    sqlite3_finalize(stmt);

    if (rc != SQLITE_DONE) {
        log_error("Failed to delete motion config for %s: %s", stream_name, sqlite3_errmsg(db));
        pthread_mutex_unlock(db_mutex);
        return -1;
    }

    pthread_mutex_unlock(db_mutex);
    log_info("Deleted motion recording config for stream: %s", stream_name);
    return 0;
}

// Load all motion recording configurations
int load_all_motion_configs(motion_recording_config_t *configs, char stream_names[][256], int max_count) {
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *db_mutex = get_db_mutex();
    sqlite3_stmt *stmt = NULL;
    int rc;
    int count = 0;

    if (!db || !configs || !stream_names || max_count <= 0) {
        log_error("Invalid parameters for load_all_motion_configs");
        return -1;
    }

    pthread_mutex_lock(db_mutex);

    const char *sql =
        "SELECT stream_name, enabled, pre_buffer_seconds, post_buffer_seconds, "
        "max_file_duration, codec, quality, retention_days "
        "FROM motion_recording_config ORDER BY stream_name;";

    rc = sqlite3_prepare_v2(db, sql, -1, &stmt, NULL);
    if (rc != SQLITE_OK) {
        log_error("Failed to prepare load_all_motion_configs statement: %s", sqlite3_errmsg(db));
        pthread_mutex_unlock(db_mutex);
        return -1;
    }

    while ((rc = sqlite3_step(stmt)) == SQLITE_ROW && count < max_count) {
        const char *stream_name = (const char *)sqlite3_column_text(stmt, 0);
        if (stream_name) {
            strncpy(stream_names[count], stream_name, 255);
            stream_names[count][255] = '\0';

            configs[count].enabled = sqlite3_column_int(stmt, 1) != 0;
            configs[count].pre_buffer_seconds = sqlite3_column_int(stmt, 2);
            configs[count].post_buffer_seconds = sqlite3_column_int(stmt, 3);
            configs[count].max_file_duration = sqlite3_column_int(stmt, 4);

            const char *codec = (const char *)sqlite3_column_text(stmt, 5);
            if (codec) {
                strncpy(configs[count].codec, codec, sizeof(configs[count].codec) - 1);
                configs[count].codec[sizeof(configs[count].codec) - 1] = '\0';
            }

            const char *quality = (const char *)sqlite3_column_text(stmt, 6);
            if (quality) {
                strncpy(configs[count].quality, quality, sizeof(configs[count].quality) - 1);
                configs[count].quality[sizeof(configs[count].quality) - 1] = '\0';
            }

            configs[count].retention_days = sqlite3_column_int(stmt, 7);

            count++;
        }
    }

    sqlite3_finalize(stmt);
    pthread_mutex_unlock(db_mutex);

    log_info("Loaded %d motion recording configurations", count);
    return count;
}

// Check if motion recording is enabled for a stream in the database
int is_motion_recording_enabled_in_db(const char *stream_name) {
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *db_mutex = get_db_mutex();
    sqlite3_stmt *stmt = NULL;
    int rc;
    int enabled = 0;

    if (!db || !stream_name) {
        log_error("Invalid parameters for is_motion_recording_enabled_in_db");
        return -1;
    }

    pthread_mutex_lock(db_mutex);

    const char *sql = "SELECT enabled FROM motion_recording_config WHERE stream_name = ?;";

    rc = sqlite3_prepare_v2(db, sql, -1, &stmt, NULL);
    if (rc != SQLITE_OK) {
        log_error("Failed to prepare is_motion_recording_enabled_in_db statement: %s", sqlite3_errmsg(db));
        pthread_mutex_unlock(db_mutex);
        return -1;
    }

    sqlite3_bind_text(stmt, 1, stream_name, -1, SQLITE_STATIC);

    rc = sqlite3_step(stmt);
    if (rc == SQLITE_ROW) {
        enabled = sqlite3_column_int(stmt, 0);
    }

    sqlite3_finalize(stmt);
    pthread_mutex_unlock(db_mutex);

    return enabled;
}

// Add a motion recording to the database
uint64_t add_motion_recording(const char *stream_name,
                              const char *file_path,
                              time_t start_time,
                              int width,
                              int height,
                              int fps,
                              const char *codec) {
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *db_mutex = get_db_mutex();
    sqlite3_stmt *stmt = NULL;
    int rc;
    uint64_t recording_id = 0;

    if (!db || !stream_name || !file_path) {
        log_error("Invalid parameters for add_motion_recording");
        return 0;
    }

    pthread_mutex_lock(db_mutex);

    const char *sql =
        "INSERT INTO motion_recordings "
        "(stream_name, file_path, start_time, width, height, fps, codec, created_at) "
        "VALUES (?, ?, ?, ?, ?, ?, ?, ?);";

    rc = sqlite3_prepare_v2(db, sql, -1, &stmt, NULL);
    if (rc != SQLITE_OK) {
        log_error("Failed to prepare add_motion_recording statement: %s", sqlite3_errmsg(db));
        pthread_mutex_unlock(db_mutex);
        return 0;
    }

    time_t now = time(NULL);

    sqlite3_bind_text(stmt, 1, stream_name, -1, SQLITE_STATIC);
    sqlite3_bind_text(stmt, 2, file_path, -1, SQLITE_STATIC);
    sqlite3_bind_int64(stmt, 3, start_time);
    sqlite3_bind_int(stmt, 4, width);
    sqlite3_bind_int(stmt, 5, height);
    sqlite3_bind_int(stmt, 6, fps);
    sqlite3_bind_text(stmt, 7, codec ? codec : "h264", -1, SQLITE_STATIC);
    sqlite3_bind_int64(stmt, 8, now);

    rc = sqlite3_step(stmt);
    if (rc == SQLITE_DONE) {
        recording_id = sqlite3_last_insert_rowid(db);
    } else {
        log_error("Failed to add motion recording: %s", sqlite3_errmsg(db));
    }

    sqlite3_finalize(stmt);
    pthread_mutex_unlock(db_mutex);

    if (recording_id > 0) {
        log_debug("Added motion recording to database: %s (ID: %llu)", file_path, (unsigned long long)recording_id);
    }

    return recording_id;
}

// Mark a motion recording as complete in the database
int mark_motion_recording_complete(const char *file_path, time_t end_time, uint64_t size_bytes) {
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *db_mutex = get_db_mutex();
    sqlite3_stmt *stmt = NULL;
    int rc;

    if (!db || !file_path) {
        log_error("Invalid parameters for mark_motion_recording_complete");
        return -1;
    }

    pthread_mutex_lock(db_mutex);

    const char *sql =
        "UPDATE motion_recordings SET end_time = ?, size_bytes = ?, is_complete = 1 "
        "WHERE file_path = ?;";

    rc = sqlite3_prepare_v2(db, sql, -1, &stmt, NULL);
    if (rc != SQLITE_OK) {
        log_error("Failed to prepare mark_motion_recording_complete statement: %s", sqlite3_errmsg(db));
        pthread_mutex_unlock(db_mutex);
        return -1;
    }

    sqlite3_bind_int64(stmt, 1, end_time);
    sqlite3_bind_int64(stmt, 2, size_bytes);
    sqlite3_bind_text(stmt, 3, file_path, -1, SQLITE_STATIC);

    rc = sqlite3_step(stmt);
    sqlite3_finalize(stmt);

    if (rc != SQLITE_DONE) {
        log_error("Failed to mark motion recording complete: %s", sqlite3_errmsg(db));
        pthread_mutex_unlock(db_mutex);
        return -1;
    }

    pthread_mutex_unlock(db_mutex);
    log_debug("Marked motion recording complete: %s", file_path);
    return 0;
}

// Get motion recording statistics from database
int get_motion_recording_db_stats(const char *stream_name,
                                   uint64_t *total_recordings,
                                   uint64_t *total_size_bytes,
                                   time_t *oldest_recording,
                                   time_t *newest_recording) {
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *db_mutex = get_db_mutex();
    sqlite3_stmt *stmt = NULL;
    int rc;

    if (!db || !stream_name) {
        log_error("Invalid parameters for get_motion_recording_db_stats");
        return -1;
    }

    pthread_mutex_lock(db_mutex);

    const char *sql =
        "SELECT COUNT(*), COALESCE(SUM(size_bytes), 0), "
        "MIN(start_time), MAX(start_time) "
        "FROM motion_recordings WHERE stream_name = ? AND is_complete = 1;";

    rc = sqlite3_prepare_v2(db, sql, -1, &stmt, NULL);
    if (rc != SQLITE_OK) {
        log_error("Failed to prepare get_motion_recording_db_stats statement: %s", sqlite3_errmsg(db));
        pthread_mutex_unlock(db_mutex);
        return -1;
    }

    sqlite3_bind_text(stmt, 1, stream_name, -1, SQLITE_STATIC);

    rc = sqlite3_step(stmt);
    if (rc == SQLITE_ROW) {
        if (total_recordings) *total_recordings = sqlite3_column_int64(stmt, 0);
        if (total_size_bytes) *total_size_bytes = sqlite3_column_int64(stmt, 1);
        if (oldest_recording) *oldest_recording = sqlite3_column_int64(stmt, 2);
        if (newest_recording) *newest_recording = sqlite3_column_int64(stmt, 3);
    }

    sqlite3_finalize(stmt);
    pthread_mutex_unlock(db_mutex);

    return 0;
}

// Get list of motion recordings for a stream
int get_motion_recordings_list(const char *stream_name,
                               time_t start_time,
                               time_t end_time,
                               char paths[][512],
                               time_t *timestamps,
                               uint64_t *sizes,
                               int max_count) {
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *db_mutex = get_db_mutex();
    sqlite3_stmt *stmt = NULL;
    int rc;
    int count = 0;

    if (!db || !stream_name || !paths || !timestamps || !sizes || max_count <= 0) {
        log_error("Invalid parameters for get_motion_recordings_list");
        return -1;
    }

    pthread_mutex_lock(db_mutex);

    const char *sql;
    if (start_time > 0 && end_time > 0) {
        sql = "SELECT file_path, start_time, size_bytes FROM motion_recordings "
              "WHERE stream_name = ? AND start_time >= ? AND start_time <= ? "
              "ORDER BY start_time DESC LIMIT ?;";
    } else if (start_time > 0) {
        sql = "SELECT file_path, start_time, size_bytes FROM motion_recordings "
              "WHERE stream_name = ? AND start_time >= ? "
              "ORDER BY start_time DESC LIMIT ?;";
    } else {
        sql = "SELECT file_path, start_time, size_bytes FROM motion_recordings "
              "WHERE stream_name = ? "
              "ORDER BY start_time DESC LIMIT ?;";
    }

    rc = sqlite3_prepare_v2(db, sql, -1, &stmt, NULL);
    if (rc != SQLITE_OK) {
        log_error("Failed to prepare get_motion_recordings_list statement: %s", sqlite3_errmsg(db));
        pthread_mutex_unlock(db_mutex);
        return -1;
    }

    int param_idx = 1;
    sqlite3_bind_text(stmt, param_idx++, stream_name, -1, SQLITE_STATIC);
    if (start_time > 0) {
        sqlite3_bind_int64(stmt, param_idx++, start_time);
    }
    if (end_time > 0 && start_time > 0) {
        sqlite3_bind_int64(stmt, param_idx++, end_time);
    }
    sqlite3_bind_int(stmt, param_idx, max_count);

    while ((rc = sqlite3_step(stmt)) == SQLITE_ROW && count < max_count) {
        const char *file_path = (const char *)sqlite3_column_text(stmt, 0);
        if (file_path) {
            strncpy(paths[count], file_path, 511);
            paths[count][511] = '\0';
            timestamps[count] = sqlite3_column_int64(stmt, 1);
            sizes[count] = sqlite3_column_int64(stmt, 2);
            count++;
        }
    }

    sqlite3_finalize(stmt);
    pthread_mutex_unlock(db_mutex);

    return count;
}

// Delete old motion recordings based on retention policy
int cleanup_old_motion_recordings(const char *stream_name, int retention_days) {
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *db_mutex = get_db_mutex();
    sqlite3_stmt *stmt = NULL;
    int rc;
    int deleted_count = 0;

    if (!db || retention_days < 0) {
        log_error("Invalid parameters for cleanup_old_motion_recordings");
        return -1;
    }

    time_t cutoff_time = time(NULL) - (retention_days * 24 * 60 * 60);

    pthread_mutex_lock(db_mutex);

    const char *sql;
    if (stream_name) {
        sql = "DELETE FROM motion_recordings WHERE stream_name = ? AND start_time < ?;";
    } else {
        sql = "DELETE FROM motion_recordings WHERE start_time < ?;";
    }

    rc = sqlite3_prepare_v2(db, sql, -1, &stmt, NULL);
    if (rc != SQLITE_OK) {
        log_error("Failed to prepare cleanup_old_motion_recordings statement: %s", sqlite3_errmsg(db));
        pthread_mutex_unlock(db_mutex);
        return -1;
    }

    if (stream_name) {
        sqlite3_bind_text(stmt, 1, stream_name, -1, SQLITE_STATIC);
        sqlite3_bind_int64(stmt, 2, cutoff_time);
    } else {
        sqlite3_bind_int64(stmt, 1, cutoff_time);
    }

    rc = sqlite3_step(stmt);
    if (rc == SQLITE_DONE) {
        deleted_count = sqlite3_changes(db);
    } else {
        log_error("Failed to cleanup old motion recordings: %s", sqlite3_errmsg(db));
        deleted_count = -1;
    }

    sqlite3_finalize(stmt);
    pthread_mutex_unlock(db_mutex);

    if (deleted_count > 0) {
        log_info("Cleaned up %d old motion recordings (retention: %d days)", deleted_count, retention_days);
    }

    return deleted_count;
}

// Get total disk space used by motion recordings
int64_t get_motion_recordings_disk_usage(const char *stream_name) {
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *db_mutex = get_db_mutex();
    sqlite3_stmt *stmt = NULL;
    int rc;
    int64_t total_size = 0;

    if (!db) {
        log_error("Database not initialized");
        return -1;
    }

    pthread_mutex_lock(db_mutex);

    const char *sql;
    if (stream_name) {
        sql = "SELECT COALESCE(SUM(size_bytes), 0) FROM motion_recordings WHERE stream_name = ?;";
    } else {
        sql = "SELECT COALESCE(SUM(size_bytes), 0) FROM motion_recordings;";
    }

    rc = sqlite3_prepare_v2(db, sql, -1, &stmt, NULL);
    if (rc != SQLITE_OK) {
        log_error("Failed to prepare get_motion_recordings_disk_usage statement: %s", sqlite3_errmsg(db));
        pthread_mutex_unlock(db_mutex);
        return -1;
    }

    if (stream_name) {
        sqlite3_bind_text(stmt, 1, stream_name, -1, SQLITE_STATIC);
    }

    rc = sqlite3_step(stmt);
    if (rc == SQLITE_ROW) {
        total_size = sqlite3_column_int64(stmt, 0);
    }

    sqlite3_finalize(stmt);
    pthread_mutex_unlock(db_mutex);

    return total_size;
}

