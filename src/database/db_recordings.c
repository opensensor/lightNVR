#define _POSIX_C_SOURCE 200809L
#define _XOPEN_SOURCE 700
#define _GNU_SOURCE

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>
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
                      "size_bytes, width, height, fps, codec, is_complete) "
                      "VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?);";
    
    rc = sqlite3_prepare_v2(db, sql, -1, &stmt, NULL);
    if (rc != SQLITE_OK) {
        log_error("Failed to prepare statement: %s", sqlite3_errmsg(db));
        pthread_mutex_unlock(db_mutex);
        return 0;
    }
    
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
    
    // Execute statement
    rc = sqlite3_step(stmt);
    if (rc != SQLITE_DONE) {
        log_error("Failed to add recording metadata: %s", sqlite3_errmsg(db));
    } else {
        recording_id = (uint64_t)sqlite3_last_insert_rowid(db);
        log_debug("Added recording metadata with ID %llu", (unsigned long long)recording_id);
    }
    
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
                      "size_bytes, width, height, fps, codec, is_complete "
                      "FROM recordings WHERE id = ?;";
    
    rc = sqlite3_prepare_v2(db, sql, -1, &stmt, NULL);
    if (rc != SQLITE_OK) {
        log_error("Failed to prepare statement: %s", sqlite3_errmsg(db));
        pthread_mutex_unlock(db_mutex);
        return -1;
    }
    
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
                 "FROM recordings WHERE is_complete = 1"); // Only complete recordings
    
    if (start_time > 0) {
        strcat(sql, " AND (end_time >= ? OR end_time IS NULL)");
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
    
    sqlite3_finalize(stmt);
    pthread_mutex_unlock(db_mutex);
    
    log_info("Found %d recordings in database matching criteria", count);
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
