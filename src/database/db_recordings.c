#define _POSIX_C_SOURCE 200809L
#define _XOPEN_SOURCE 700
#define _GNU_SOURCE

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>
#include <unistd.h>
#include <sqlite3.h>
#include <stdbool.h>

#include "database/db_recordings.h"
#include "database/db_core.h"
#include "core/logger.h"

// Add recording metadata to the database
uint64_t add_recording_metadata(const recording_metadata_t *metadata) {
    int rc;
    sqlite3_stmt *stmt;
    uint64_t recording_id = 0;

    sqlite3 *db = get_db_handle();
    pthread_mutex_t *db_mutex = get_db_mutex();

    if (!db) {
        log_error("Database not initialized");
        return 0;
    }

    if (!metadata) {
        log_error("Recording metadata is required");
        return 0;
    }

    pthread_mutex_lock(db_mutex);

    const char *sql = "INSERT INTO recordings (stream_name, file_path, start_time, end_time, "
                      "size_bytes, width, height, fps, codec, is_complete, trigger_type) "
                      "VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?);";

    rc = sqlite3_prepare_v2(db, sql, -1, &stmt, NULL);
    if (rc != SQLITE_OK) {
        log_error("Failed to prepare statement: %s", sqlite3_errmsg(db));
        pthread_mutex_unlock(db_mutex);
        return 0;
    }

    // No longer tracking statements - each function is responsible for finalizing its own statements

    // Bind parameters
    sqlite3_bind_text(stmt, 1, metadata->stream_name, -1, SQLITE_STATIC);
    sqlite3_bind_text(stmt, 2, metadata->file_path, -1, SQLITE_STATIC);
    sqlite3_bind_int64(stmt, 3, (sqlite3_int64)metadata->start_time);

    if (metadata->end_time > 0) {
        sqlite3_bind_int64(stmt, 4, (sqlite3_int64)metadata->end_time);
    } else {
        sqlite3_bind_null(stmt, 4);
    }

    sqlite3_bind_int64(stmt, 5, (sqlite3_int64)metadata->size_bytes);
    sqlite3_bind_int(stmt, 6, metadata->width);
    sqlite3_bind_int(stmt, 7, metadata->height);
    sqlite3_bind_int(stmt, 8, metadata->fps);
    sqlite3_bind_text(stmt, 9, metadata->codec, -1, SQLITE_STATIC);
    sqlite3_bind_int(stmt, 10, metadata->is_complete ? 1 : 0);

    // Bind trigger_type, default to 'scheduled' if not set
    const char *trigger_type = (metadata->trigger_type[0] != '\0') ? metadata->trigger_type : "scheduled";
    sqlite3_bind_text(stmt, 11, trigger_type, -1, SQLITE_STATIC);

    // Execute statement
    rc = sqlite3_step(stmt);
    if (rc != SQLITE_DONE) {
        log_error("Failed to add recording metadata: %s", sqlite3_errmsg(db));
    } else {
        recording_id = (uint64_t)sqlite3_last_insert_rowid(db);
        log_debug("Added recording metadata with ID %llu", (unsigned long long)recording_id);
    }

    // Finalize the prepared statement
    sqlite3_finalize(stmt);
    pthread_mutex_unlock(db_mutex);

    return recording_id;
}

// Update recording metadata in the database
int update_recording_metadata(uint64_t id, time_t end_time,
                             uint64_t size_bytes, bool is_complete) {
    int rc;
    sqlite3_stmt *stmt;

    sqlite3 *db = get_db_handle();
    pthread_mutex_t *db_mutex = get_db_mutex();

    if (!db) {
        log_error("Database not initialized");
        return -1;
    }

    pthread_mutex_lock(db_mutex);

    const char *sql = "UPDATE recordings SET end_time = ?, size_bytes = ?, is_complete = ? "
                      "WHERE id = ?;";

    rc = sqlite3_prepare_v2(db, sql, -1, &stmt, NULL);
    if (rc != SQLITE_OK) {
        log_error("Failed to prepare statement: %s", sqlite3_errmsg(db));
        pthread_mutex_unlock(db_mutex);
        return -1;
    }

    // No longer tracking statements - each function is responsible for finalizing its own statements

    // Bind parameters
    sqlite3_bind_int64(stmt, 1, (sqlite3_int64)end_time);
    sqlite3_bind_int64(stmt, 2, (sqlite3_int64)size_bytes);
    sqlite3_bind_int(stmt, 3, is_complete ? 1 : 0);
    sqlite3_bind_int64(stmt, 4, (sqlite3_int64)id);

    // Execute statement
    rc = sqlite3_step(stmt);
    if (rc != SQLITE_DONE) {
        log_error("Failed to update recording metadata: %s", sqlite3_errmsg(db));
        sqlite3_finalize(stmt);
        pthread_mutex_unlock(db_mutex);
        return -1;
    }

    // Finalize the prepared statement
    sqlite3_finalize(stmt);
    pthread_mutex_unlock(db_mutex);

    return 0;
}

// Get recording metadata by ID
int get_recording_metadata_by_id(uint64_t id, recording_metadata_t *metadata) {
    int rc;
    sqlite3_stmt *stmt;
    int result = -1;

    sqlite3 *db = get_db_handle();
    pthread_mutex_t *db_mutex = get_db_mutex();

    if (!db) {
        log_error("Database not initialized");
        return -1;
    }

    if (!metadata) {
        log_error("Invalid parameters for get_recording_metadata_by_id");
        return -1;
    }

    pthread_mutex_lock(db_mutex);

    const char *sql = "SELECT id, stream_name, file_path, start_time, end_time, "
                      "size_bytes, width, height, fps, codec, is_complete, trigger_type "
                      "FROM recordings WHERE id = ?;";

    rc = sqlite3_prepare_v2(db, sql, -1, &stmt, NULL);
    if (rc != SQLITE_OK) {
        log_error("Failed to prepare statement: %s", sqlite3_errmsg(db));
        pthread_mutex_unlock(db_mutex);
        return -1;
    }

    // No longer tracking statements - each function is responsible for finalizing its own statements

    // Bind parameters
    sqlite3_bind_int64(stmt, 1, (sqlite3_int64)id);

    // Execute query and fetch result
    if (sqlite3_step(stmt) == SQLITE_ROW) {
        metadata->id = (uint64_t)sqlite3_column_int64(stmt, 0);

        const char *stream = (const char *)sqlite3_column_text(stmt, 1);
        if (stream) {
            strncpy(metadata->stream_name, stream, sizeof(metadata->stream_name) - 1);
            metadata->stream_name[sizeof(metadata->stream_name) - 1] = '\0';
        } else {
            metadata->stream_name[0] = '\0';
        }

        const char *path = (const char *)sqlite3_column_text(stmt, 2);
        if (path) {
            strncpy(metadata->file_path, path, sizeof(metadata->file_path) - 1);
            metadata->file_path[sizeof(metadata->file_path) - 1] = '\0';
        } else {
            metadata->file_path[0] = '\0';
        }

        metadata->start_time = (time_t)sqlite3_column_int64(stmt, 3);

        if (sqlite3_column_type(stmt, 4) != SQLITE_NULL) {
            metadata->end_time = (time_t)sqlite3_column_int64(stmt, 4);
        } else {
            metadata->end_time = 0;
        }

        metadata->size_bytes = (uint64_t)sqlite3_column_int64(stmt, 5);
        metadata->width = sqlite3_column_int(stmt, 6);
        metadata->height = sqlite3_column_int(stmt, 7);
        metadata->fps = sqlite3_column_int(stmt, 8);

        const char *codec = (const char *)sqlite3_column_text(stmt, 9);
        if (codec) {
            strncpy(metadata->codec, codec, sizeof(metadata->codec) - 1);
            metadata->codec[sizeof(metadata->codec) - 1] = '\0';
        } else {
            metadata->codec[0] = '\0';
        }

        metadata->is_complete = sqlite3_column_int(stmt, 10) != 0;

        const char *trigger_type = (const char *)sqlite3_column_text(stmt, 11);
        if (trigger_type) {
            strncpy(metadata->trigger_type, trigger_type, sizeof(metadata->trigger_type) - 1);
            metadata->trigger_type[sizeof(metadata->trigger_type) - 1] = '\0';
        } else {
            strncpy(metadata->trigger_type, "scheduled", sizeof(metadata->trigger_type) - 1);
        }

        result = 0; // Success
    }

    // Finalize the prepared statement
    sqlite3_finalize(stmt);
    pthread_mutex_unlock(db_mutex);

    return result;
}

// Get recording metadata by file path
int get_recording_metadata_by_path(const char *file_path, recording_metadata_t *metadata) {
    int rc;
    sqlite3_stmt *stmt;
    int result = -1;

    sqlite3 *db = get_db_handle();
    pthread_mutex_t *db_mutex = get_db_mutex();

    if (!db) {
        log_error("Database not initialized");
        return -1;
    }

    if (!file_path || !metadata) {
        log_error("Invalid parameters for get_recording_metadata_by_path");
        return -1;
    }

    pthread_mutex_lock(db_mutex);

    const char *sql = "SELECT id, stream_name, file_path, start_time, end_time, "
                      "size_bytes, width, height, fps, codec, is_complete, trigger_type, "
                      "protected, retention_override_days "
                      "FROM recordings WHERE file_path = ?;";

    rc = sqlite3_prepare_v2(db, sql, -1, &stmt, NULL);
    if (rc != SQLITE_OK) {
        log_error("Failed to prepare statement: %s", sqlite3_errmsg(db));
        pthread_mutex_unlock(db_mutex);
        return -1;
    }

    sqlite3_bind_text(stmt, 1, file_path, -1, SQLITE_STATIC);

    if (sqlite3_step(stmt) == SQLITE_ROW) {
        metadata->id = (uint64_t)sqlite3_column_int64(stmt, 0);

        const char *stream = (const char *)sqlite3_column_text(stmt, 1);
        if (stream) {
            strncpy(metadata->stream_name, stream, sizeof(metadata->stream_name) - 1);
            metadata->stream_name[sizeof(metadata->stream_name) - 1] = '\0';
        } else {
            metadata->stream_name[0] = '\0';
        }

        const char *path = (const char *)sqlite3_column_text(stmt, 2);
        if (path) {
            strncpy(metadata->file_path, path, sizeof(metadata->file_path) - 1);
            metadata->file_path[sizeof(metadata->file_path) - 1] = '\0';
        } else {
            metadata->file_path[0] = '\0';
        }

        metadata->start_time = (time_t)sqlite3_column_int64(stmt, 3);

        if (sqlite3_column_type(stmt, 4) != SQLITE_NULL) {
            metadata->end_time = (time_t)sqlite3_column_int64(stmt, 4);
        } else {
            metadata->end_time = 0;
        }

        metadata->size_bytes = (uint64_t)sqlite3_column_int64(stmt, 5);
        metadata->width = sqlite3_column_int(stmt, 6);
        metadata->height = sqlite3_column_int(stmt, 7);
        metadata->fps = sqlite3_column_int(stmt, 8);

        const char *codec = (const char *)sqlite3_column_text(stmt, 9);
        if (codec) {
            strncpy(metadata->codec, codec, sizeof(metadata->codec) - 1);
            metadata->codec[sizeof(metadata->codec) - 1] = '\0';
        } else {
            metadata->codec[0] = '\0';
        }

        metadata->is_complete = sqlite3_column_int(stmt, 10) != 0;

        const char *trigger_type = (const char *)sqlite3_column_text(stmt, 11);
        if (trigger_type) {
            strncpy(metadata->trigger_type, trigger_type, sizeof(metadata->trigger_type) - 1);
            metadata->trigger_type[sizeof(metadata->trigger_type) - 1] = '\0';
        } else {
            strncpy(metadata->trigger_type, "scheduled", sizeof(metadata->trigger_type) - 1);
        }

        metadata->protected = sqlite3_column_int(stmt, 12) != 0;

        if (sqlite3_column_type(stmt, 13) != SQLITE_NULL) {
            metadata->retention_override_days = sqlite3_column_int(stmt, 13);
        } else {
            metadata->retention_override_days = -1;
        }

        result = 0; // Success
    }

    sqlite3_finalize(stmt);
    pthread_mutex_unlock(db_mutex);

    return result;
}

// Get recording metadata from the database
int get_recording_metadata(time_t start_time, time_t end_time,
                          const char *stream_name, recording_metadata_t *metadata,
                          int max_count) {
    int rc;
    sqlite3_stmt *stmt;
    int count = 0;

    sqlite3 *db = get_db_handle();
    pthread_mutex_t *db_mutex = get_db_mutex();

    if (!db) {
        log_error("Database not initialized");
        return -1;
    }

    if (!metadata || max_count <= 0) {
        log_error("Invalid parameters for get_recording_metadata");
        return -1;
    }

    pthread_mutex_lock(db_mutex);

    // Build query based on filters
    char sql[1024];
    strcpy(sql, "SELECT id, stream_name, file_path, start_time, end_time, "
                 "size_bytes, width, height, fps, codec, is_complete "
                 "FROM recordings WHERE is_complete = 1 AND end_time IS NOT NULL"); // Only complete recordings with end_time set

    if (start_time > 0) {
        strcat(sql, " AND start_time >= ?");
    }

    if (end_time > 0) {
        strcat(sql, " AND start_time <= ?");
    }

    if (stream_name) {
        strcat(sql, " AND stream_name = ?");
    }

    strcat(sql, " ORDER BY start_time DESC LIMIT ?;");

    rc = sqlite3_prepare_v2(db, sql, -1, &stmt, NULL);
    if (rc != SQLITE_OK) {
        log_error("Failed to prepare statement: %s", sqlite3_errmsg(db));
        pthread_mutex_unlock(db_mutex);
        return -1;
    }

    // No longer tracking statements - each function is responsible for finalizing its own statements

    // Bind parameters
    int param_index = 1;

    if (start_time > 0) {
        sqlite3_bind_int64(stmt, param_index++, (sqlite3_int64)start_time);
    }

    if (end_time > 0) {
        sqlite3_bind_int64(stmt, param_index++, (sqlite3_int64)end_time);
    }

    if (stream_name) {
        sqlite3_bind_text(stmt, param_index++, stream_name, -1, SQLITE_STATIC);
    }

    sqlite3_bind_int(stmt, param_index, max_count);

    // Execute query and fetch results
    int rc_step;
    while ((rc_step = sqlite3_step(stmt)) == SQLITE_ROW && count < max_count) {
        // Safely copy data to metadata structure
        if (count < max_count) {
            metadata[count].id = (uint64_t)sqlite3_column_int64(stmt, 0);

            const char *stream = (const char *)sqlite3_column_text(stmt, 1);
            if (stream) {
                strncpy(metadata[count].stream_name, stream, sizeof(metadata[count].stream_name) - 1);
                metadata[count].stream_name[sizeof(metadata[count].stream_name) - 1] = '\0';
            } else {
                metadata[count].stream_name[0] = '\0';
            }

            const char *path = (const char *)sqlite3_column_text(stmt, 2);
            if (path) {
                strncpy(metadata[count].file_path, path, sizeof(metadata[count].file_path) - 1);
                metadata[count].file_path[sizeof(metadata[count].file_path) - 1] = '\0';
            } else {
                metadata[count].file_path[0] = '\0';
            }

            metadata[count].start_time = (time_t)sqlite3_column_int64(stmt, 3);

            if (sqlite3_column_type(stmt, 4) != SQLITE_NULL) {
                metadata[count].end_time = (time_t)sqlite3_column_int64(stmt, 4);
            } else {
                metadata[count].end_time = 0;
            }

            metadata[count].size_bytes = (uint64_t)sqlite3_column_int64(stmt, 5);
            metadata[count].width = sqlite3_column_int(stmt, 6);
            metadata[count].height = sqlite3_column_int(stmt, 7);
            metadata[count].fps = sqlite3_column_int(stmt, 8);

            const char *codec = (const char *)sqlite3_column_text(stmt, 9);
            if (codec) {
                strncpy(metadata[count].codec, codec, sizeof(metadata[count].codec) - 1);
                metadata[count].codec[sizeof(metadata[count].codec) - 1] = '\0';
            } else {
                metadata[count].codec[0] = '\0';
            }

            metadata[count].is_complete = sqlite3_column_int(stmt, 10) != 0;

            count++;
        }
    }

    if (rc_step != SQLITE_DONE && rc_step != SQLITE_ROW) {
        log_error("Error while fetching recordings: %s", sqlite3_errmsg(db));
    }

    // Finalize the prepared statement
    sqlite3_finalize(stmt);
    pthread_mutex_unlock(db_mutex);

    log_info("Found %d recordings in database matching criteria", count);
    return count;
}

// Get total count of recordings matching filter criteria
int get_recording_count(time_t start_time, time_t end_time,
                       const char *stream_name, int has_detection) {
    int rc;
    sqlite3_stmt *stmt;
    int count = 0;

    sqlite3 *db = get_db_handle();
    pthread_mutex_t *db_mutex = get_db_mutex();

    if (!db) {
        log_error("Database not initialized");
        return -1;
    }

    pthread_mutex_lock(db_mutex);

    // Build query based on filters
    char sql[1024];

    // Use trigger_type and/or detections table to filter detection-based recordings
    strcpy(sql, "SELECT COUNT(*) FROM recordings r WHERE r.is_complete = 1 AND r.end_time IS NOT NULL");

    if (has_detection) {
        // Filter by trigger_type = 'detection' OR existence of detections in the recording's time range
        strcat(sql, " AND (r.trigger_type = 'detection' OR EXISTS (SELECT 1 FROM detections d WHERE d.stream_name = r.stream_name AND d.timestamp >= r.start_time AND d.timestamp <= r.end_time))");
        log_debug("Adding detection filter (trigger_type OR detections table)");
    }

    if (start_time > 0) {
        strcat(sql, " AND r.start_time >= ?");
        log_debug("Adding start_time filter: %ld", (long)start_time);
    }

    if (end_time > 0) {
        strcat(sql, " AND r.start_time <= ?");
        log_debug("Adding end_time filter: %ld", (long)end_time);
    }

    if (stream_name) {
        strcat(sql, " AND r.stream_name = ?");
    }

    log_debug("SQL query for get_recording_count: %s", sql);

    rc = sqlite3_prepare_v2(db, sql, -1, &stmt, NULL);
    if (rc != SQLITE_OK) {
        log_error("Failed to prepare statement: %s", sqlite3_errmsg(db));
        pthread_mutex_unlock(db_mutex);
        return -1;
    }

    // No longer tracking statements - each function is responsible for finalizing its own statements

    // Bind parameters
    int param_index = 1;

    if (start_time > 0) {
        sqlite3_bind_int64(stmt, param_index++, (sqlite3_int64)start_time);
    }

    if (end_time > 0) {
        sqlite3_bind_int64(stmt, param_index++, (sqlite3_int64)end_time);
    }

    if (stream_name) {
        sqlite3_bind_text(stmt, param_index++, stream_name, -1, SQLITE_STATIC);
    }

    // Execute query and get count
    if (sqlite3_step(stmt) == SQLITE_ROW) {
        count = sqlite3_column_int(stmt, 0);
    } else {
        log_error("Error while getting recording count: %s", sqlite3_errmsg(db));
        count = -1;
    }

    // Finalize the prepared statement
    sqlite3_finalize(stmt);
    pthread_mutex_unlock(db_mutex);

    log_debug("Total count of recordings matching criteria: %d", count);
    return count;
}

// Get paginated recording metadata from the database with sorting
int get_recording_metadata_paginated(time_t start_time, time_t end_time,
                                   const char *stream_name, int has_detection,
                                   const char *sort_field, const char *sort_order,
                                   recording_metadata_t *metadata,
                                   int limit, int offset) {
    int rc;
    sqlite3_stmt *stmt;
    int count = 0;

    sqlite3 *db = get_db_handle();
    pthread_mutex_t *db_mutex = get_db_mutex();

    if (!db) {
        log_error("Database not initialized");
        return -1;
    }

    if (!metadata || limit <= 0) {
        log_error("Invalid parameters for get_recording_metadata_paginated");
        return -1;
    }

    pthread_mutex_lock(db_mutex);

    // Validate and sanitize sort field to prevent SQL injection
    char safe_sort_field[32] = "start_time"; // Default sort field
    if (sort_field) {
        if (strcmp(sort_field, "id") == 0 ||
            strcmp(sort_field, "stream_name") == 0 ||
            strcmp(sort_field, "start_time") == 0 ||
            strcmp(sort_field, "end_time") == 0 ||
            strcmp(sort_field, "size_bytes") == 0) {
            strncpy(safe_sort_field, sort_field, sizeof(safe_sort_field) - 1);
            safe_sort_field[sizeof(safe_sort_field) - 1] = '\0';
        } else {
            log_warn("Invalid sort field: %s, using default", sort_field);
        }
    }

    // Validate sort order
    char safe_sort_order[8] = "DESC"; // Default sort order
    if (sort_order) {
        if (strcasecmp(sort_order, "asc") == 0) {
            strcpy(safe_sort_order, "ASC");
        } else if (strcasecmp(sort_order, "desc") == 0) {
            strcpy(safe_sort_order, "DESC");
        } else {
            log_warn("Invalid sort order: %s, using default", sort_order);
        }
    }

    // Build query based on filters
    char sql[1536];

    // Use trigger_type and/or detections table to filter detection-based recordings
    snprintf(sql, sizeof(sql),
            "SELECT r.id, r.stream_name, r.file_path, r.start_time, r.end_time, "
            "r.size_bytes, r.width, r.height, r.fps, r.codec, r.is_complete, r.trigger_type "
            "FROM recordings r WHERE r.is_complete = 1 AND r.end_time IS NOT NULL");

    if (has_detection) {
        // Filter by trigger_type = 'detection' OR existence of detections in the recording's time range
        strcat(sql, " AND (r.trigger_type = 'detection' OR EXISTS (SELECT 1 FROM detections d WHERE d.stream_name = r.stream_name AND d.timestamp >= r.start_time AND d.timestamp <= r.end_time))");
        log_info("Adding detection filter (trigger_type OR detections table)");
    }

    if (start_time > 0) {
        strcat(sql, " AND r.start_time >= ?");
        log_info("Adding start_time filter to paginated query: %ld", (long)start_time);
    }

    if (end_time > 0) {
        strcat(sql, " AND r.start_time <= ?");
        log_info("Adding end_time filter to paginated query: %ld", (long)end_time);
    }

    if (stream_name) {
        strcat(sql, " AND r.stream_name = ?");
    }

    // Add ORDER BY clause with sanitized field and order
    char order_clause[64];
    snprintf(order_clause, sizeof(order_clause), " ORDER BY r.%s %s", safe_sort_field, safe_sort_order);
    strcat(sql, order_clause);

    // Add LIMIT and OFFSET for pagination
    char limit_clause[64];
    snprintf(limit_clause, sizeof(limit_clause), " LIMIT ? OFFSET ?");
    strcat(sql, limit_clause);

    log_debug("SQL query for get_recording_metadata_paginated: %s", sql);

    rc = sqlite3_prepare_v2(db, sql, -1, &stmt, NULL);
    if (rc != SQLITE_OK) {
        log_error("Failed to prepare statement: %s", sqlite3_errmsg(db));
        pthread_mutex_unlock(db_mutex);
        return -1;
    }

    // No longer tracking statements - each function is responsible for finalizing its own statements

    // Bind parameters
    int param_index = 1;

    if (start_time > 0) {
        sqlite3_bind_int64(stmt, param_index++, (sqlite3_int64)start_time);
    }

    if (end_time > 0) {
        sqlite3_bind_int64(stmt, param_index++, (sqlite3_int64)end_time);
    }

    if (stream_name) {
        sqlite3_bind_text(stmt, param_index++, stream_name, -1, SQLITE_STATIC);
    }

    // Bind LIMIT and OFFSET parameters
    sqlite3_bind_int(stmt, param_index++, limit);
    sqlite3_bind_int(stmt, param_index, offset);

    // Execute query and fetch results
    int rc_step;
    while ((rc_step = sqlite3_step(stmt)) == SQLITE_ROW && count < limit) {
        // Safely copy data to metadata structure
        if (count < limit) {
            metadata[count].id = (uint64_t)sqlite3_column_int64(stmt, 0);

            const char *stream = (const char *)sqlite3_column_text(stmt, 1);
            if (stream) {
                strncpy(metadata[count].stream_name, stream, sizeof(metadata[count].stream_name) - 1);
                metadata[count].stream_name[sizeof(metadata[count].stream_name) - 1] = '\0';
            } else {
                metadata[count].stream_name[0] = '\0';
            }

            const char *path = (const char *)sqlite3_column_text(stmt, 2);
            if (path) {
                strncpy(metadata[count].file_path, path, sizeof(metadata[count].file_path) - 1);
                metadata[count].file_path[sizeof(metadata[count].file_path) - 1] = '\0';
            } else {
                metadata[count].file_path[0] = '\0';
            }

            metadata[count].start_time = (time_t)sqlite3_column_int64(stmt, 3);

            if (sqlite3_column_type(stmt, 4) != SQLITE_NULL) {
                metadata[count].end_time = (time_t)sqlite3_column_int64(stmt, 4);
            } else {
                metadata[count].end_time = 0;
            }

            metadata[count].size_bytes = (uint64_t)sqlite3_column_int64(stmt, 5);
            metadata[count].width = sqlite3_column_int(stmt, 6);
            metadata[count].height = sqlite3_column_int(stmt, 7);
            metadata[count].fps = sqlite3_column_int(stmt, 8);

            const char *codec = (const char *)sqlite3_column_text(stmt, 9);
            if (codec) {
                strncpy(metadata[count].codec, codec, sizeof(metadata[count].codec) - 1);
                metadata[count].codec[sizeof(metadata[count].codec) - 1] = '\0';
            } else {
                metadata[count].codec[0] = '\0';
            }

            metadata[count].is_complete = sqlite3_column_int(stmt, 10) != 0;

            const char *trigger_type = (const char *)sqlite3_column_text(stmt, 11);
            if (trigger_type) {
                strncpy(metadata[count].trigger_type, trigger_type, sizeof(metadata[count].trigger_type) - 1);
                metadata[count].trigger_type[sizeof(metadata[count].trigger_type) - 1] = '\0';
            } else {
                strncpy(metadata[count].trigger_type, "scheduled", sizeof(metadata[count].trigger_type) - 1);
            }

            count++;
        }
    }

    if (rc_step != SQLITE_DONE && rc_step != SQLITE_ROW) {
        log_error("Error while fetching recordings: %s", sqlite3_errmsg(db));
    }

    // Finalize the prepared statement
    sqlite3_finalize(stmt);
    pthread_mutex_unlock(db_mutex);

    log_debug("Found %d recordings in database matching criteria (page %d, limit %d)",
             count, (offset / limit) + 1, limit);
    return count;
}

// Delete recording metadata from the database
int delete_recording_metadata(uint64_t id) {
    int rc;
    sqlite3_stmt *stmt;

    sqlite3 *db = get_db_handle();
    pthread_mutex_t *db_mutex = get_db_mutex();

    if (!db) {
        log_error("Database not initialized");
        return -1;
    }

    pthread_mutex_lock(db_mutex);

    const char *sql = "DELETE FROM recordings WHERE id = ?;";

    rc = sqlite3_prepare_v2(db, sql, -1, &stmt, NULL);
    if (rc != SQLITE_OK) {
        log_error("Failed to prepare statement: %s", sqlite3_errmsg(db));
        pthread_mutex_unlock(db_mutex);
        return -1;
    }

    // No longer tracking statements - each function is responsible for finalizing its own statements

    // Bind parameters
    sqlite3_bind_int64(stmt, 1, (sqlite3_int64)id);

    // Execute statement
    rc = sqlite3_step(stmt);
    if (rc != SQLITE_DONE) {
        log_error("Failed to delete recording metadata: %s", sqlite3_errmsg(db));
        sqlite3_finalize(stmt);
        pthread_mutex_unlock(db_mutex);
        return -1;
    }

    // Finalize the prepared statement
    sqlite3_finalize(stmt);
    pthread_mutex_unlock(db_mutex);

    return 0;
}

// Delete old recording metadata from the database
int delete_old_recording_metadata(uint64_t max_age) {
    int rc;
    sqlite3_stmt *stmt;
    int deleted_count = 0;

    sqlite3 *db = get_db_handle();
    pthread_mutex_t *db_mutex = get_db_mutex();

    if (!db) {
        log_error("Database not initialized");
        return -1;
    }

    pthread_mutex_lock(db_mutex);

    const char *sql = "DELETE FROM recordings WHERE end_time < ?;";

    rc = sqlite3_prepare_v2(db, sql, -1, &stmt, NULL);
    if (rc != SQLITE_OK) {
        log_error("Failed to prepare statement: %s", sqlite3_errmsg(db));
        pthread_mutex_unlock(db_mutex);
        return -1;
    }

    // Calculate cutoff time
    time_t cutoff_time = time(NULL) - max_age;

    // Bind parameters
    sqlite3_bind_int64(stmt, 1, (sqlite3_int64)cutoff_time);

    // Execute statement
    rc = sqlite3_step(stmt);
    if (rc != SQLITE_DONE) {
        log_error("Failed to delete old recording metadata: %s", sqlite3_errmsg(db));
        sqlite3_finalize(stmt);
        pthread_mutex_unlock(db_mutex);
        return -1;
    }

    deleted_count = sqlite3_changes(db);

    sqlite3_finalize(stmt);
    pthread_mutex_unlock(db_mutex);

    return deleted_count;
}

/**
 * Set protection status for a recording
 *
 * @param id Recording ID
 * @param protected Whether to protect the recording
 * @return 0 on success, non-zero on failure
 */
int set_recording_protected(uint64_t id, bool protected) {
    int rc;
    sqlite3_stmt *stmt;

    sqlite3 *db = get_db_handle();
    pthread_mutex_t *db_mutex = get_db_mutex();

    if (!db) {
        log_error("Database not initialized");
        return -1;
    }

    pthread_mutex_lock(db_mutex);

    const char *sql = "UPDATE recordings SET protected = ? WHERE id = ?;";

    rc = sqlite3_prepare_v2(db, sql, -1, &stmt, NULL);
    if (rc != SQLITE_OK) {
        log_error("Failed to prepare statement: %s", sqlite3_errmsg(db));
        pthread_mutex_unlock(db_mutex);
        return -1;
    }

    sqlite3_bind_int(stmt, 1, protected ? 1 : 0);
    sqlite3_bind_int64(stmt, 2, (sqlite3_int64)id);

    rc = sqlite3_step(stmt);
    sqlite3_finalize(stmt);
    pthread_mutex_unlock(db_mutex);

    if (rc != SQLITE_DONE) {
        log_error("Failed to update recording protection: %s", sqlite3_errmsg(db));
        return -1;
    }

    log_info("Recording %llu protection set to %s", (unsigned long long)id, protected ? "true" : "false");
    return 0;
}

/**
 * Set custom retention override for a recording
 *
 * @param id Recording ID
 * @param days Custom retention days (-1 to remove override)
 * @return 0 on success, non-zero on failure
 */
int set_recording_retention_override(uint64_t id, int days) {
    int rc;
    sqlite3_stmt *stmt;

    sqlite3 *db = get_db_handle();
    pthread_mutex_t *db_mutex = get_db_mutex();

    if (!db) {
        log_error("Database not initialized");
        return -1;
    }

    pthread_mutex_lock(db_mutex);

    const char *sql = "UPDATE recordings SET retention_override_days = ? WHERE id = ?;";

    rc = sqlite3_prepare_v2(db, sql, -1, &stmt, NULL);
    if (rc != SQLITE_OK) {
        log_error("Failed to prepare statement: %s", sqlite3_errmsg(db));
        pthread_mutex_unlock(db_mutex);
        return -1;
    }

    if (days < 0) {
        sqlite3_bind_null(stmt, 1);
    } else {
        sqlite3_bind_int(stmt, 1, days);
    }
    sqlite3_bind_int64(stmt, 2, (sqlite3_int64)id);

    rc = sqlite3_step(stmt);
    sqlite3_finalize(stmt);
    pthread_mutex_unlock(db_mutex);

    if (rc != SQLITE_DONE) {
        log_error("Failed to update recording retention override: %s", sqlite3_errmsg(db));
        return -1;
    }

    log_info("Recording %llu retention override set to %d days", (unsigned long long)id, days);
    return 0;
}

/**
 * Get count of protected recordings for a stream
 *
 * @param stream_name Stream name (NULL for all streams)
 * @return Count of protected recordings, or -1 on error
 */
int get_protected_recordings_count(const char *stream_name) {
    int rc;
    sqlite3_stmt *stmt;
    int count = -1;

    sqlite3 *db = get_db_handle();
    pthread_mutex_t *db_mutex = get_db_mutex();

    if (!db) {
        log_error("Database not initialized");
        return -1;
    }

    pthread_mutex_lock(db_mutex);

    const char *sql;
    if (stream_name) {
        sql = "SELECT COUNT(*) FROM recordings WHERE protected = 1 AND stream_name = ?;";
    } else {
        sql = "SELECT COUNT(*) FROM recordings WHERE protected = 1;";
    }

    rc = sqlite3_prepare_v2(db, sql, -1, &stmt, NULL);
    if (rc != SQLITE_OK) {
        log_error("Failed to prepare statement: %s", sqlite3_errmsg(db));
        pthread_mutex_unlock(db_mutex);
        return -1;
    }

    if (stream_name) {
        sqlite3_bind_text(stmt, 1, stream_name, -1, SQLITE_STATIC);
    }

    if (sqlite3_step(stmt) == SQLITE_ROW) {
        count = sqlite3_column_int(stmt, 0);
    }

    sqlite3_finalize(stmt);
    pthread_mutex_unlock(db_mutex);

    return count;
}


/**
 * Get recordings eligible for deletion based on retention policy
 * Priority 1: Regular recordings past retention period
 * Priority 2: Detection recordings past detection retention period
 * Protected recordings are never returned
 *
 * @param stream_name Stream name to filter
 * @param retention_days Regular recordings retention in days
 * @param detection_retention_days Detection recordings retention in days
 * @param recordings Array to fill with recording metadata
 * @param max_count Maximum number of recordings to return
 * @return Number of recordings found, or -1 on error
 */
int get_recordings_for_retention(const char *stream_name,
                                 int retention_days,
                                 int detection_retention_days,
                                 recording_metadata_t *recordings,
                                 int max_count) {
    int rc;
    sqlite3_stmt *stmt;
    int count = 0;

    sqlite3 *db = get_db_handle();
    pthread_mutex_t *db_mutex = get_db_mutex();

    if (!db) {
        log_error("Database not initialized");
        return -1;
    }

    if (!stream_name || !recordings || max_count <= 0) {
        log_error("Invalid parameters for get_recordings_for_retention");
        return -1;
    }

    // Calculate cutoff times
    time_t now = time(NULL);
    time_t regular_cutoff = (retention_days > 0) ? now - (retention_days * 86400) : 0;
    time_t detection_cutoff = (detection_retention_days > 0) ? now - (detection_retention_days * 86400) : 0;

    pthread_mutex_lock(db_mutex);

    // Query for recordings past retention, ordered by priority (regular first, then detection)
    // and by start_time (oldest first)
    // Protected recordings are excluded
    // Also exclude recordings with retention_override_days that haven't expired yet
    const char *sql =
        "SELECT id, stream_name, file_path, start_time, end_time, "
        "size_bytes, width, height, fps, codec, is_complete, trigger_type, protected, retention_override_days "
        "FROM recordings "
        "WHERE stream_name = ? "
        "AND protected = 0 "
        "AND is_complete = 1 "
        "AND ("
        "  (trigger_type != 'detection' AND ? > 0 AND start_time < ?) "
        "  OR "
        "  (trigger_type = 'detection' AND ? > 0 AND start_time < ?)"
        ") "
        "AND ("
        "  retention_override_days IS NULL "
        "  OR start_time < (strftime('%s', 'now') - retention_override_days * 86400)"
        ") "
        "ORDER BY "
        "  CASE WHEN trigger_type = 'detection' THEN 1 ELSE 0 END ASC, "
        "  start_time ASC "
        "LIMIT ?;";

    rc = sqlite3_prepare_v2(db, sql, -1, &stmt, NULL);
    if (rc != SQLITE_OK) {
        log_error("Failed to prepare statement: %s", sqlite3_errmsg(db));
        pthread_mutex_unlock(db_mutex);
        return -1;
    }

    sqlite3_bind_text(stmt, 1, stream_name, -1, SQLITE_STATIC);
    sqlite3_bind_int(stmt, 2, retention_days);
    sqlite3_bind_int64(stmt, 3, (sqlite3_int64)regular_cutoff);
    sqlite3_bind_int(stmt, 4, detection_retention_days);
    sqlite3_bind_int64(stmt, 5, (sqlite3_int64)detection_cutoff);
    sqlite3_bind_int(stmt, 6, max_count);

    while (sqlite3_step(stmt) == SQLITE_ROW && count < max_count) {
        recordings[count].id = (uint64_t)sqlite3_column_int64(stmt, 0);

        const char *stream = (const char *)sqlite3_column_text(stmt, 1);
        if (stream) {
            strncpy(recordings[count].stream_name, stream, sizeof(recordings[count].stream_name) - 1);
            recordings[count].stream_name[sizeof(recordings[count].stream_name) - 1] = '\0';
        } else {
            recordings[count].stream_name[0] = '\0';
        }

        const char *path = (const char *)sqlite3_column_text(stmt, 2);
        if (path) {
            strncpy(recordings[count].file_path, path, sizeof(recordings[count].file_path) - 1);
            recordings[count].file_path[sizeof(recordings[count].file_path) - 1] = '\0';
        } else {
            recordings[count].file_path[0] = '\0';
        }

        recordings[count].start_time = (time_t)sqlite3_column_int64(stmt, 3);

        if (sqlite3_column_type(stmt, 4) != SQLITE_NULL) {
            recordings[count].end_time = (time_t)sqlite3_column_int64(stmt, 4);
        } else {
            recordings[count].end_time = 0;
        }

        recordings[count].size_bytes = (uint64_t)sqlite3_column_int64(stmt, 5);
        recordings[count].width = sqlite3_column_int(stmt, 6);
        recordings[count].height = sqlite3_column_int(stmt, 7);
        recordings[count].fps = sqlite3_column_int(stmt, 8);

        const char *codec = (const char *)sqlite3_column_text(stmt, 9);
        if (codec) {
            strncpy(recordings[count].codec, codec, sizeof(recordings[count].codec) - 1);
            recordings[count].codec[sizeof(recordings[count].codec) - 1] = '\0';
        } else {
            recordings[count].codec[0] = '\0';
        }

        recordings[count].is_complete = sqlite3_column_int(stmt, 10) != 0;

        const char *trigger_type = (const char *)sqlite3_column_text(stmt, 11);
        if (trigger_type) {
            strncpy(recordings[count].trigger_type, trigger_type, sizeof(recordings[count].trigger_type) - 1);
            recordings[count].trigger_type[sizeof(recordings[count].trigger_type) - 1] = '\0';
        } else {
            strncpy(recordings[count].trigger_type, "scheduled", sizeof(recordings[count].trigger_type) - 1);
        }

        recordings[count].protected = sqlite3_column_int(stmt, 12) != 0;

        if (sqlite3_column_type(stmt, 13) != SQLITE_NULL) {
            recordings[count].retention_override_days = sqlite3_column_int(stmt, 13);
        } else {
            recordings[count].retention_override_days = -1;
        }

        count++;
    }

    sqlite3_finalize(stmt);
    pthread_mutex_unlock(db_mutex);

    return count;
}

/**
 * Get recordings for quota enforcement (oldest unprotected first)
 *
 * @param stream_name Stream name
 * @param recordings Array to fill with recording metadata
 * @param max_count Maximum number of recordings to return
 * @return Number of recordings found, or -1 on error
 */
int get_recordings_for_quota_enforcement(const char *stream_name,
                                         recording_metadata_t *recordings,
                                         int max_count) {
    int rc;
    sqlite3_stmt *stmt;
    int count = 0;

    sqlite3 *db = get_db_handle();
    pthread_mutex_t *db_mutex = get_db_mutex();

    if (!db) {
        log_error("Database not initialized");
        return -1;
    }

    if (!stream_name || !recordings || max_count <= 0) {
        log_error("Invalid parameters for get_recordings_for_quota_enforcement");
        return -1;
    }

    pthread_mutex_lock(db_mutex);

    // Get oldest unprotected recordings first
    // Order by protection status (unprotected first), then by start_time (oldest first)
    const char *sql =
        "SELECT id, stream_name, file_path, start_time, end_time, "
        "size_bytes, width, height, fps, codec, is_complete, trigger_type, protected, retention_override_days "
        "FROM recordings "
        "WHERE stream_name = ? "
        "AND protected = 0 "
        "AND is_complete = 1 "
        "ORDER BY start_time ASC "
        "LIMIT ?;";

    rc = sqlite3_prepare_v2(db, sql, -1, &stmt, NULL);
    if (rc != SQLITE_OK) {
        log_error("Failed to prepare statement: %s", sqlite3_errmsg(db));
        pthread_mutex_unlock(db_mutex);
        return -1;
    }

    sqlite3_bind_text(stmt, 1, stream_name, -1, SQLITE_STATIC);
    sqlite3_bind_int(stmt, 2, max_count);

    while (sqlite3_step(stmt) == SQLITE_ROW && count < max_count) {
        recordings[count].id = (uint64_t)sqlite3_column_int64(stmt, 0);

        const char *stream = (const char *)sqlite3_column_text(stmt, 1);
        if (stream) {
            strncpy(recordings[count].stream_name, stream, sizeof(recordings[count].stream_name) - 1);
            recordings[count].stream_name[sizeof(recordings[count].stream_name) - 1] = '\0';
        } else {
            recordings[count].stream_name[0] = '\0';
        }

        const char *path = (const char *)sqlite3_column_text(stmt, 2);
        if (path) {
            strncpy(recordings[count].file_path, path, sizeof(recordings[count].file_path) - 1);
            recordings[count].file_path[sizeof(recordings[count].file_path) - 1] = '\0';
        } else {
            recordings[count].file_path[0] = '\0';
        }

        recordings[count].start_time = (time_t)sqlite3_column_int64(stmt, 3);

        if (sqlite3_column_type(stmt, 4) != SQLITE_NULL) {
            recordings[count].end_time = (time_t)sqlite3_column_int64(stmt, 4);
        } else {
            recordings[count].end_time = 0;
        }

        recordings[count].size_bytes = (uint64_t)sqlite3_column_int64(stmt, 5);
        recordings[count].width = sqlite3_column_int(stmt, 6);
        recordings[count].height = sqlite3_column_int(stmt, 7);
        recordings[count].fps = sqlite3_column_int(stmt, 8);

        const char *codec = (const char *)sqlite3_column_text(stmt, 9);
        if (codec) {
            strncpy(recordings[count].codec, codec, sizeof(recordings[count].codec) - 1);
            recordings[count].codec[sizeof(recordings[count].codec) - 1] = '\0';
        } else {
            recordings[count].codec[0] = '\0';
        }

        recordings[count].is_complete = sqlite3_column_int(stmt, 10) != 0;

        const char *trigger_type = (const char *)sqlite3_column_text(stmt, 11);
        if (trigger_type) {
            strncpy(recordings[count].trigger_type, trigger_type, sizeof(recordings[count].trigger_type) - 1);
            recordings[count].trigger_type[sizeof(recordings[count].trigger_type) - 1] = '\0';
        } else {
            strncpy(recordings[count].trigger_type, "scheduled", sizeof(recordings[count].trigger_type) - 1);
        }

        recordings[count].protected = sqlite3_column_int(stmt, 12) != 0;

        if (sqlite3_column_type(stmt, 13) != SQLITE_NULL) {
            recordings[count].retention_override_days = sqlite3_column_int(stmt, 13);
        } else {
            recordings[count].retention_override_days = -1;
        }

        count++;
    }

    sqlite3_finalize(stmt);
    pthread_mutex_unlock(db_mutex);

    return count;
}

/**
 * Get orphaned recording entries (DB entries without files)
 *
 * @param recordings Array to fill with recording metadata
 * @param max_count Maximum number of recordings to return
 * @return Number of recordings found, or -1 on error
 */
int get_orphaned_db_entries(recording_metadata_t *recordings, int max_count) {
    int rc;
    sqlite3_stmt *stmt;
    int count = 0;
    int checked = 0;

    sqlite3 *db = get_db_handle();
    pthread_mutex_t *db_mutex = get_db_mutex();

    if (!db) {
        log_error("Database not initialized");
        return -1;
    }

    if (!recordings || max_count <= 0) {
        log_error("Invalid parameters for get_orphaned_db_entries");
        return -1;
    }

    pthread_mutex_lock(db_mutex);

    // Get all recordings and check if files exist
    const char *sql =
        "SELECT id, stream_name, file_path, start_time, end_time, "
        "size_bytes, width, height, fps, codec, is_complete, trigger_type "
        "FROM recordings "
        "WHERE is_complete = 1 "
        "ORDER BY start_time ASC;";

    rc = sqlite3_prepare_v2(db, sql, -1, &stmt, NULL);
    if (rc != SQLITE_OK) {
        log_error("Failed to prepare statement: %s", sqlite3_errmsg(db));
        pthread_mutex_unlock(db_mutex);
        return -1;
    }

    while (sqlite3_step(stmt) == SQLITE_ROW && count < max_count) {
        const char *path = (const char *)sqlite3_column_text(stmt, 2);

        // Check if file exists
        if (path && access(path, F_OK) != 0) {
            // File doesn't exist - this is an orphaned entry
            recordings[count].id = (uint64_t)sqlite3_column_int64(stmt, 0);

            const char *stream = (const char *)sqlite3_column_text(stmt, 1);
            if (stream) {
                strncpy(recordings[count].stream_name, stream, sizeof(recordings[count].stream_name) - 1);
                recordings[count].stream_name[sizeof(recordings[count].stream_name) - 1] = '\0';
            } else {
                recordings[count].stream_name[0] = '\0';
            }

            strncpy(recordings[count].file_path, path, sizeof(recordings[count].file_path) - 1);
            recordings[count].file_path[sizeof(recordings[count].file_path) - 1] = '\0';

            recordings[count].start_time = (time_t)sqlite3_column_int64(stmt, 3);

            if (sqlite3_column_type(stmt, 4) != SQLITE_NULL) {
                recordings[count].end_time = (time_t)sqlite3_column_int64(stmt, 4);
            } else {
                recordings[count].end_time = 0;
            }

            recordings[count].size_bytes = (uint64_t)sqlite3_column_int64(stmt, 5);
            recordings[count].width = sqlite3_column_int(stmt, 6);
            recordings[count].height = sqlite3_column_int(stmt, 7);
            recordings[count].fps = sqlite3_column_int(stmt, 8);

            const char *codec = (const char *)sqlite3_column_text(stmt, 9);
            if (codec) {
                strncpy(recordings[count].codec, codec, sizeof(recordings[count].codec) - 1);
                recordings[count].codec[sizeof(recordings[count].codec) - 1] = '\0';
            } else {
                recordings[count].codec[0] = '\0';
            }

            recordings[count].is_complete = sqlite3_column_int(stmt, 10) != 0;

            const char *trigger_type = (const char *)sqlite3_column_text(stmt, 11);
            if (trigger_type) {
                strncpy(recordings[count].trigger_type, trigger_type, sizeof(recordings[count].trigger_type) - 1);
                recordings[count].trigger_type[sizeof(recordings[count].trigger_type) - 1] = '\0';
            } else {
                strncpy(recordings[count].trigger_type, "scheduled", sizeof(recordings[count].trigger_type) - 1);
            }

            count++;
        }
        checked++;
    }

    sqlite3_finalize(stmt);
    pthread_mutex_unlock(db_mutex);

    log_info("Checked %d recordings, found %d orphaned DB entries", checked, count);
    return count;
}
