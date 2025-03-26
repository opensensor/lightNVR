#define _POSIX_C_SOURCE 200809L
#define _XOPEN_SOURCE 700
#define _GNU_SOURCE

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>
#include <sqlite3.h>
#include <stdbool.h>

#include "database/db_detections.h"
#include "database/db_core.h"
#include "core/logger.h"
#include "video/detection_result.h"

/**
 * Store detection results in the database
 * 
 * @param stream_name Stream name
 * @param result Detection results
 * @param timestamp Timestamp of the detection (0 for current time)
 * @return 0 on success, non-zero on failure
 */
int store_detections_in_db(const char *stream_name, const detection_result_t *result, time_t timestamp) {
    int rc;
    sqlite3_stmt *stmt;
    
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *db_mutex = get_db_mutex();
    
    if (!db) {
        log_error("Database not initialized when trying to store detections");
        return -1;
    }
    
    if (!stream_name || !result) {
        log_error("Invalid parameters for store_detections_in_db: stream_name=%p, result=%p", 
                 stream_name, result);
        return -1;
    }
    
    // Use current time if timestamp is 0
    if (timestamp == 0) {
        timestamp = time(NULL);
    }
    
    log_info("Storing %d detections in database for stream %s", result->count, stream_name);
    
    // Log the first detection for debugging
    if (result->count > 0) {
        log_info("First detection: %s (%.2f%%) at [%.2f, %.2f, %.2f, %.2f]",
                result->detections[0].label,
                result->detections[0].confidence * 100.0f,
                result->detections[0].x,
                result->detections[0].y,
                result->detections[0].width,
                result->detections[0].height);
    }
    
    pthread_mutex_lock(db_mutex);
    
    // Check if detections table exists
    char *err_msg = NULL;
    char **query_result;
    int rows, cols;
    rc = sqlite3_get_table(db, 
                          "SELECT name FROM sqlite_master WHERE type='table' AND name='detections';", 
                          &query_result, &rows, &cols, &err_msg);
    
    if (rc != SQLITE_OK) {
        log_error("Failed to check if detections table exists: %s", err_msg);
        sqlite3_free(err_msg);
        pthread_mutex_unlock(db_mutex);
        return -1;
    }
    
    if (rows == 0) {
        log_error("Detections table does not exist, recreating it");
        sqlite3_free_table(query_result);
        
        // Create detections table
        const char *create_detections_table = 
            "CREATE TABLE IF NOT EXISTS detections ("
            "id INTEGER PRIMARY KEY AUTOINCREMENT,"
            "stream_name TEXT NOT NULL,"
            "timestamp INTEGER NOT NULL,"
            "label TEXT NOT NULL,"
            "confidence REAL NOT NULL,"
            "x REAL NOT NULL,"
            "y REAL NOT NULL,"
            "width REAL NOT NULL,"
            "height REAL NOT NULL"
            ");";
        
        rc = sqlite3_exec(db, create_detections_table, NULL, NULL, &err_msg);
        if (rc != SQLITE_OK) {
            log_error("Failed to create detections table: %s", err_msg);
            sqlite3_free(err_msg);
            pthread_mutex_unlock(db_mutex);
            return -1;
        }
        
        // Create indexes
        const char *create_indexes = 
            "CREATE INDEX IF NOT EXISTS idx_detections_stream_timestamp ON detections (stream_name, timestamp);"
            "CREATE INDEX IF NOT EXISTS idx_detections_timestamp ON detections (timestamp);";
        
        rc = sqlite3_exec(db, create_indexes, NULL, NULL, &err_msg);
        if (rc != SQLITE_OK) {
            log_error("Failed to create indexes: %s", err_msg);
            sqlite3_free(err_msg);
            pthread_mutex_unlock(db_mutex);
            return -1;
        }
    } else {
        sqlite3_free_table(query_result);
    }
    
    // Begin transaction for better performance when inserting multiple detections
    rc = sqlite3_exec(db, "BEGIN TRANSACTION;", NULL, NULL, &err_msg);
    if (rc != SQLITE_OK) {
        log_error("Failed to begin transaction: %s", err_msg);
        sqlite3_free(err_msg);
        pthread_mutex_unlock(db_mutex);
        return -1;
    }
    
    const char *sql = "INSERT INTO detections (stream_name, timestamp, label, confidence, x, y, width, height) "
                      "VALUES (?, ?, ?, ?, ?, ?, ?, ?);";
    
    rc = sqlite3_prepare_v2(db, sql, -1, &stmt, NULL);
    if (rc != SQLITE_OK) {
        log_error("Failed to prepare statement: %s", sqlite3_errmsg(db));
        sqlite3_exec(db, "ROLLBACK;", NULL, NULL, NULL);
        pthread_mutex_unlock(db_mutex);
        return -1;
    }
    
    // Insert each detection
    for (int i = 0; i < result->count; i++) {
        // Bind parameters
        sqlite3_bind_text(stmt, 1, stream_name, -1, SQLITE_STATIC);
        sqlite3_bind_int64(stmt, 2, (sqlite3_int64)timestamp);
        sqlite3_bind_text(stmt, 3, result->detections[i].label, -1, SQLITE_STATIC);
        sqlite3_bind_double(stmt, 4, result->detections[i].confidence);
        sqlite3_bind_double(stmt, 5, result->detections[i].x);
        sqlite3_bind_double(stmt, 6, result->detections[i].y);
        sqlite3_bind_double(stmt, 7, result->detections[i].width);
        sqlite3_bind_double(stmt, 8, result->detections[i].height);
        
        // Execute statement
        rc = sqlite3_step(stmt);
        if (rc != SQLITE_DONE) {
            log_error("Failed to insert detection %d: %s", i, sqlite3_errmsg(db));
            sqlite3_finalize(stmt);
            sqlite3_exec(db, "ROLLBACK;", NULL, NULL, NULL);
            pthread_mutex_unlock(db_mutex);
            return -1;
        }
        
        // Reset statement and clear bindings for next detection
        sqlite3_reset(stmt);
        sqlite3_clear_bindings(stmt);
    }
    
    sqlite3_finalize(stmt);
    
    // Commit transaction
    rc = sqlite3_exec(db, "COMMIT;", NULL, NULL, &err_msg);
    if (rc != SQLITE_OK) {
        log_error("Failed to commit transaction: %s", err_msg);
        sqlite3_free(err_msg);
        sqlite3_exec(db, "ROLLBACK;", NULL, NULL, NULL);
        pthread_mutex_unlock(db_mutex);
        return -1;
    }
    
    // Verify the detections were stored
    char verify_sql[256];
    snprintf(verify_sql, sizeof(verify_sql), 
            "SELECT COUNT(*) FROM detections WHERE stream_name = '%s' AND timestamp = %lld;",
            stream_name, (long long)timestamp);
    
    rc = sqlite3_get_table(db, verify_sql, &query_result, &rows, &cols, &err_msg);
    if (rc != SQLITE_OK) {
        log_error("Failed to verify detections were stored: %s", err_msg);
        sqlite3_free(err_msg);
    } else if (rows > 0 && cols > 0) {
        int count = atoi(query_result[1]); // First row, first column
        log_info("Verified %d detections were stored in database for stream %s", count, stream_name);
        sqlite3_free_table(query_result);
    }
    
    pthread_mutex_unlock(db_mutex);
    
    log_info("Successfully stored %d detections in database for stream %s", result->count, stream_name);
    return 0;
}

/**
 * Get detection results from the database
 * 
 * @param stream_name Stream name
 * @param result Detection results to fill
 * @param max_age Maximum age in seconds (0 for all)
 * @param start_time Start time filter (0 for no filter)
 * @param end_time End time filter (0 for no filter)
 * @return Number of detections found, or -1 on error
 */
int get_detections_from_db_time_range(const char *stream_name, detection_result_t *result, 
                                     uint64_t max_age, time_t start_time, time_t end_time) {
    int rc;
    sqlite3_stmt *stmt;
    
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *db_mutex = get_db_mutex();
    
    if (!db) {
        log_error("Database not initialized");
        return -1;
    }
    
    if (!stream_name || !result) {
        log_error("Invalid parameters for get_detections_from_db_time_range");
        return -1;
    }
    
    // Initialize result
    memset(result, 0, sizeof(detection_result_t));
    
    pthread_mutex_lock(db_mutex);
    
    // Build query based on filters
    char sql[512];
    
    if (start_time > 0 && end_time > 0) {
        // Time range filter
        log_info("Getting detections for stream %s between %lld and %lld", 
                stream_name, (long long)start_time, (long long)end_time);
        
        snprintf(sql, sizeof(sql), 
                "SELECT label, confidence, x, y, width, height "
                "FROM detections "
                "WHERE stream_name = ? AND timestamp >= ? AND timestamp <= ? "
                "ORDER BY timestamp DESC "
                "LIMIT ?;");
        
        rc = sqlite3_prepare_v2(db, sql, -1, &stmt, NULL);
        if (rc != SQLITE_OK) {
            log_error("Failed to prepare statement: %s", sqlite3_errmsg(db));
            pthread_mutex_unlock(db_mutex);
            return -1;
        }
        
        // Bind parameters
        sqlite3_bind_text(stmt, 1, stream_name, -1, SQLITE_STATIC);
        sqlite3_bind_int64(stmt, 2, (sqlite3_int64)start_time);
        sqlite3_bind_int64(stmt, 3, (sqlite3_int64)end_time);
        sqlite3_bind_int(stmt, 4, MAX_DETECTIONS);
    } else if (start_time > 0) {
        // Start time filter only
        log_info("Getting detections for stream %s from %lld", 
                stream_name, (long long)start_time);
        
        snprintf(sql, sizeof(sql), 
                "SELECT label, confidence, x, y, width, height "
                "FROM detections "
                "WHERE stream_name = ? AND timestamp >= ? "
                "ORDER BY timestamp DESC "
                "LIMIT ?;");
        
        rc = sqlite3_prepare_v2(db, sql, -1, &stmt, NULL);
        if (rc != SQLITE_OK) {
            log_error("Failed to prepare statement: %s", sqlite3_errmsg(db));
            pthread_mutex_unlock(db_mutex);
            return -1;
        }
        
        // Bind parameters
        sqlite3_bind_text(stmt, 1, stream_name, -1, SQLITE_STATIC);
        sqlite3_bind_int64(stmt, 2, (sqlite3_int64)start_time);
        sqlite3_bind_int(stmt, 3, MAX_DETECTIONS);
    } else if (end_time > 0) {
        // End time filter only
        log_info("Getting detections for stream %s until %lld", 
                stream_name, (long long)end_time);
        
        snprintf(sql, sizeof(sql), 
                "SELECT label, confidence, x, y, width, height "
                "FROM detections "
                "WHERE stream_name = ? AND timestamp <= ? "
                "ORDER BY timestamp DESC "
                "LIMIT ?;");
        
        rc = sqlite3_prepare_v2(db, sql, -1, &stmt, NULL);
        if (rc != SQLITE_OK) {
            log_error("Failed to prepare statement: %s", sqlite3_errmsg(db));
            pthread_mutex_unlock(db_mutex);
            return -1;
        }
        
        // Bind parameters
        sqlite3_bind_text(stmt, 1, stream_name, -1, SQLITE_STATIC);
        sqlite3_bind_int64(stmt, 2, (sqlite3_int64)end_time);
        sqlite3_bind_int(stmt, 3, MAX_DETECTIONS);
    } else if (max_age > 0) {
        // Max age filter
        // Calculate cutoff time
        time_t cutoff_time = time(NULL) - max_age;
        
        log_info("Getting detections for stream %s since %lld (max age %llu seconds)", 
                stream_name, (long long)cutoff_time, (unsigned long long)max_age);
        
        // First get the latest timestamp
        snprintf(sql, sizeof(sql), 
                "SELECT MAX(timestamp) "
                "FROM detections "
                "WHERE stream_name = ? AND timestamp >= ?;");
        
        rc = sqlite3_prepare_v2(db, sql, -1, &stmt, NULL);
        if (rc != SQLITE_OK) {
            log_error("Failed to prepare statement: %s", sqlite3_errmsg(db));
            pthread_mutex_unlock(db_mutex);
            return -1;
        }
        
        // Bind parameters
        sqlite3_bind_text(stmt, 1, stream_name, -1, SQLITE_STATIC);
        sqlite3_bind_int64(stmt, 2, (sqlite3_int64)cutoff_time);
        
        // Execute query to get latest timestamp
        time_t latest_timestamp = 0;
        if (sqlite3_step(stmt) == SQLITE_ROW) {
            latest_timestamp = (time_t)sqlite3_column_int64(stmt, 0);
        }
        
        sqlite3_finalize(stmt);
        
        // If no timestamp found, return empty result
        if (latest_timestamp == 0) {
            pthread_mutex_unlock(db_mutex);
            log_info("No recent detections found for stream %s", stream_name);
            return 0;
        }
        
        // Now get all detections at that timestamp
        snprintf(sql, sizeof(sql), 
                "SELECT label, confidence, x, y, width, height "
                "FROM detections "
                "WHERE stream_name = ? AND timestamp = ? "
                "LIMIT ?;");
        
        rc = sqlite3_prepare_v2(db, sql, -1, &stmt, NULL);
        if (rc != SQLITE_OK) {
            log_error("Failed to prepare statement: %s", sqlite3_errmsg(db));
            pthread_mutex_unlock(db_mutex);
            return -1;
        }
        
        // Bind parameters
        sqlite3_bind_text(stmt, 1, stream_name, -1, SQLITE_STATIC);
        sqlite3_bind_int64(stmt, 2, (sqlite3_int64)latest_timestamp);
        sqlite3_bind_int(stmt, 3, MAX_DETECTIONS);
    } else {
        // No filters, just get the latest detections
        log_info("Getting latest detections for stream %s (no time filters)", stream_name);
        
        snprintf(sql, sizeof(sql), 
                "SELECT label, confidence, x, y, width, height "
                "FROM detections "
                "WHERE stream_name = ? "
                "ORDER BY timestamp DESC "
                "LIMIT ?;");
        
        rc = sqlite3_prepare_v2(db, sql, -1, &stmt, NULL);
        if (rc != SQLITE_OK) {
            log_error("Failed to prepare statement: %s", sqlite3_errmsg(db));
            pthread_mutex_unlock(db_mutex);
            return -1;
        }
        
        // Bind parameters
        sqlite3_bind_text(stmt, 1, stream_name, -1, SQLITE_STATIC);
        sqlite3_bind_int(stmt, 2, MAX_DETECTIONS);
    }
    
    // Execute query and fetch results
    int count = 0;
    while (sqlite3_step(stmt) == SQLITE_ROW && count < MAX_DETECTIONS) {
        // Get detection data
        const char *label = (const char *)sqlite3_column_text(stmt, 0);
        float confidence = (float)sqlite3_column_double(stmt, 1);
        float x = (float)sqlite3_column_double(stmt, 2);
        float y = (float)sqlite3_column_double(stmt, 3);
        float width = (float)sqlite3_column_double(stmt, 4);
        float height = (float)sqlite3_column_double(stmt, 5);
        
        // Store in result
        if (label) {
            strncpy(result->detections[count].label, label, MAX_LABEL_LENGTH - 1);
            result->detections[count].label[MAX_LABEL_LENGTH - 1] = '\0';
        } else {
            strncpy(result->detections[count].label, "unknown", MAX_LABEL_LENGTH - 1);
        }
        
        result->detections[count].confidence = confidence;
        result->detections[count].x = x;
        result->detections[count].y = y;
        result->detections[count].width = width;
        result->detections[count].height = height;
        
        count++;
    }
    
    result->count = count;
    
    sqlite3_finalize(stmt);
    pthread_mutex_unlock(db_mutex);
    
    log_info("Found %d detections in database for stream %s", count, stream_name);
    return count;
}

/**
 * Get detection results from the database
 * 
 * @param stream_name Stream name
 * @param result Detection results to fill
 * @param max_age Maximum age in seconds (0 for all)
 * @return Number of detections found, or -1 on error
 */
int get_detections_from_db(const char *stream_name, detection_result_t *result, uint64_t max_age) {
    // Call the new function with no time range filters
    return get_detections_from_db_time_range(stream_name, result, max_age, 0, 0);
}

/**
 * Delete old detections from the database
 * 
 * @param max_age Maximum age in seconds
 * @return Number of detections deleted, or -1 on error
 */
int delete_old_detections(uint64_t max_age) {
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
    
    const char *sql = "DELETE FROM detections WHERE timestamp < ?;";
    
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
        log_error("Failed to delete old detections: %s", sqlite3_errmsg(db));
        sqlite3_finalize(stmt);
        pthread_mutex_unlock(db_mutex);
        return -1;
    }
    
    deleted_count = sqlite3_changes(db);
    
    sqlite3_finalize(stmt);
    pthread_mutex_unlock(db_mutex);
    
    log_info("Deleted %d old detections from database", deleted_count);
    return deleted_count;
}
