/*
 * SPDX-License-Identifier: GPL-3.0-or-later
 * Copyright (C) 2025-2026 OpenSensor Engineering and LightNVR contributors
 *
 * This file is part of LightNVR <https://github.com/opensensor/lightNVR>.
 */

#include <sqlite3.h>
#include <string.h>
#include <time.h>

#include "database/db_motion_config.h"
#include "database/db_core.h"
#include "core/logger.h"
#include "utils/strings.h"
#include "video/streams.h"

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

    time_t cutoff_time = time(NULL) - ((time_t)retention_days * 24 * 60 * 60);

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

