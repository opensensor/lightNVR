#define _POSIX_C_SOURCE 200809L
#define _XOPEN_SOURCE 700
#define _GNU_SOURCE

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>
#include <sqlite3.h>
#include <stdbool.h>
#include <math.h>

#include "database/db_detections.h"
#include "database/db_core.h"
#include "core/logger.h"
#include "utils/strings.h"
#include "video/detection_result.h"

/**
 * Store detection results in the database
 *
 * @param stream_name Stream name
 * @param result Detection results
 * @param timestamp Timestamp of the detection (0 for current time)
 * @param recording_id Recording ID to link detections to (0 for no link)
 * @return 0 on success, non-zero on failure
 */
static int store_detections_with_source(const char *stream_name,
                                        const detection_result_t *result,
                                        time_t timestamp,
                                        uint64_t recording_id,
                                        const char *source,
                                        bool open_interval) {
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
    
    log_debug("Storing %d detections in database for stream %s", result->count, stream_name);

    // Log the first detection for debugging
    if (result->count > 0) {
        log_debug("First detection: %s (%.2f%%) at [%.2f, %.2f, %.2f, %.2f]",
                result->detections[0].label,
                result->detections[0].confidence * 100.0f,
                result->detections[0].x,
                result->detections[0].y,
                result->detections[0].width,
                result->detections[0].height);
    }
    
    pthread_mutex_lock(db_mutex);
    
    // Note: detections table is created by SQL migrations (see db/migrations/)
    // No need for runtime table existence checks or fallback creation.
    char *err_msg = NULL;
    char **query_result;
    int rows, cols;

    // Begin transaction for better performance when inserting multiple detections
    rc = sqlite3_exec(db, "BEGIN TRANSACTION;", NULL, NULL, &err_msg);
    if (rc != SQLITE_OK) {
        log_error("Failed to begin transaction: %s", err_msg);
        sqlite3_free(err_msg);
        pthread_mutex_unlock(db_mutex);
        return -1;
    }
    
    const char *sql = "INSERT INTO detections (stream_name, timestamp, label, confidence, x, y, width, height, track_id, zone_id, recording_id, source, event_end_time) "
                      "VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?);";

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
        sqlite3_bind_int(stmt, 9, result->detections[i].track_id);
        sqlite3_bind_text(stmt, 10, result->detections[i].zone_id, -1, SQLITE_STATIC);

        // Bind recording_id - NULL if 0, otherwise the actual ID
        if (recording_id > 0) {
            sqlite3_bind_int64(stmt, 11, (sqlite3_int64)recording_id);
        } else {
            sqlite3_bind_null(stmt, 11);
        }
        sqlite3_bind_text(stmt, 12, source ? source : "", -1, SQLITE_STATIC);
        if (open_interval) {
            sqlite3_bind_null(stmt, 13);
        } else {
            sqlite3_bind_int64(stmt, 13, (sqlite3_int64)timestamp);
        }
        
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
    
    // Untrack and finalize the prepared statement
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
        int count = (int)strtol(query_result[1], NULL, 10); // First row, first column
        log_debug("Verified %d detections were stored in database for stream %s", count, stream_name);
        sqlite3_free_table(query_result);
    }

    pthread_mutex_unlock(db_mutex);

    log_debug("Successfully stored %d detections in database for stream %s", result->count, stream_name);
    return 0;
}

int store_detections_in_db(const char *stream_name,
                           const detection_result_t *result,
                           time_t timestamp,
                           uint64_t recording_id) {
    return store_detections_with_source(stream_name, result, timestamp,
                                        recording_id, "", false);
}

int store_external_motion_detections(const char *stream_name,
                                     const detection_result_t *result,
                                     time_t timestamp,
                                     uint64_t recording_id) {
    return store_detections_with_source(stream_name, result, timestamp,
                                        recording_id, "external_motion", true);
}

int close_external_motion_detections(const char *stream_name, time_t end_time) {
    if (!stream_name || stream_name[0] == '\0') return -1;
    if (end_time == 0) end_time = time(NULL);

    sqlite3 *db = get_db_handle();
    pthread_mutex_t *db_mutex = get_db_mutex();
    if (!db) return -1;

    pthread_mutex_lock(db_mutex);
    const char *sql =
        "UPDATE detections SET event_end_time = "
        "CASE WHEN ? < timestamp THEN timestamp ELSE ? END "
        "WHERE stream_name = ? AND source = 'external_motion' "
        "AND event_end_time IS NULL;";
    sqlite3_stmt *stmt = NULL;
    int rc = sqlite3_prepare_v2(db, sql, -1, &stmt, NULL);
    if (rc == SQLITE_OK) {
        sqlite3_bind_int64(stmt, 1, (sqlite3_int64)end_time);
        sqlite3_bind_int64(stmt, 2, (sqlite3_int64)end_time);
        sqlite3_bind_text(stmt, 3, stream_name, -1, SQLITE_STATIC);
        rc = sqlite3_step(stmt);
    }
    int changed = rc == SQLITE_DONE ? sqlite3_changes(db) : -1;
    if (stmt) sqlite3_finalize(stmt);
    pthread_mutex_unlock(db_mutex);
    return changed;
}

int get_external_motion_detection_intervals(const char *stream_name,
                                            time_t start_time,
                                            time_t end_time,
                                            detection_interval_t *intervals,
                                            int max_intervals) {
    if (!stream_name || !intervals || max_intervals <= 0) return -1;
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *db_mutex = get_db_mutex();
    if (!db) return -1;

    pthread_mutex_lock(db_mutex);
    const char *sql =
        "SELECT timestamp, COALESCE(event_end_time, CAST(strftime('%s','now') AS INTEGER)) "
        "FROM detections WHERE stream_name = ? "
        "AND source = 'external_motion' AND timestamp <= ? "
        "AND COALESCE(event_end_time, CAST(strftime('%s','now') AS INTEGER)) >= ? "
        "GROUP BY timestamp, event_end_time ORDER BY timestamp ASC LIMIT ?;";
    sqlite3_stmt *stmt = NULL;
    int rc = sqlite3_prepare_v2(db, sql, -1, &stmt, NULL);
    if (rc != SQLITE_OK) {
        pthread_mutex_unlock(db_mutex);
        return -1;
    }
    sqlite3_bind_text(stmt, 1, stream_name, -1, SQLITE_STATIC);
    sqlite3_bind_int64(stmt, 2, (sqlite3_int64)end_time);
    sqlite3_bind_int64(stmt, 3, (sqlite3_int64)start_time);
    sqlite3_bind_int(stmt, 4, max_intervals);

    int count = 0;
    while (sqlite3_step(stmt) == SQLITE_ROW && count < max_intervals) {
        intervals[count].start_time = (time_t)sqlite3_column_int64(stmt, 0);
        intervals[count].end_time = (time_t)sqlite3_column_int64(stmt, 1);
        count++;
    }
    sqlite3_finalize(stmt);
    pthread_mutex_unlock(db_mutex);
    return count;
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
    char sql[2048];
    
    if (start_time > 0 && end_time > 0) {
        // Time range filter
        log_info("Getting detections for stream %s between %lld and %lld (MAX_DETECTIONS=%d)",
                stream_name, (long long)start_time, (long long)end_time, MAX_DETECTIONS);

        snprintf(sql, sizeof(sql),
                "SELECT label, confidence, x, y, width, height FROM ("
                "SELECT label, confidence, x, y, width, height, timestamp "
                "FROM detections WHERE stream_name = ? "
                "AND source != 'external_motion' AND timestamp >= ? AND timestamp <= ? "
                "UNION ALL "
                "SELECT label, confidence, x, y, width, height, timestamp "
                "FROM detections WHERE stream_name = ? "
                "AND source = 'external_motion' AND timestamp <= ? "
                "AND COALESCE(event_end_time, CAST(strftime('%%s','now') AS INTEGER)) >= ?"
                ") ORDER BY timestamp DESC LIMIT ?;");
        
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
        sqlite3_bind_text(stmt, 4, stream_name, -1, SQLITE_STATIC);
        sqlite3_bind_int64(stmt, 5, (sqlite3_int64)end_time);
        sqlite3_bind_int64(stmt, 6, (sqlite3_int64)start_time);
        sqlite3_bind_int(stmt, 7, MAX_DETECTIONS);
    } else if (start_time > 0) {
        // Start time filter only
        log_debug("Getting detections for stream %s from %lld",
                stream_name, (long long)start_time);
        
        snprintf(sql, sizeof(sql),
                "SELECT label, confidence, x, y, width, height FROM ("
                "SELECT label, confidence, x, y, width, height, timestamp "
                "FROM detections WHERE stream_name = ? "
                "AND source != 'external_motion' AND timestamp >= ? "
                "UNION ALL "
                "SELECT label, confidence, x, y, width, height, timestamp "
                "FROM detections WHERE stream_name = ? "
                "AND source = 'external_motion' "
                "AND COALESCE(event_end_time, CAST(strftime('%%s','now') AS INTEGER)) >= ?"
                ") ORDER BY timestamp DESC LIMIT ?;");
        
        rc = sqlite3_prepare_v2(db, sql, -1, &stmt, NULL);
        if (rc != SQLITE_OK) {
            log_error("Failed to prepare statement: %s", sqlite3_errmsg(db));
            pthread_mutex_unlock(db_mutex);
            return -1;
        }
        
        // Bind parameters
        sqlite3_bind_text(stmt, 1, stream_name, -1, SQLITE_STATIC);
        sqlite3_bind_int64(stmt, 2, (sqlite3_int64)start_time);
        sqlite3_bind_text(stmt, 3, stream_name, -1, SQLITE_STATIC);
        sqlite3_bind_int64(stmt, 4, (sqlite3_int64)start_time);
        sqlite3_bind_int(stmt, 5, MAX_DETECTIONS);
    } else if (end_time > 0) {
        // End time filter only
        log_debug("Getting detections for stream %s until %lld",
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
        time_t cutoff_time = time(NULL) - (time_t)max_age;

        log_debug("Getting detections for stream %s since %lld (max age %llu seconds)",
                stream_name, (long long)cutoff_time, (unsigned long long)max_age);

        // Get all detections within the time window (not just the latest timestamp)
        snprintf(sql, sizeof(sql),
                "SELECT label, confidence, x, y, width, height FROM ("
                "SELECT label, confidence, x, y, width, height, timestamp "
                "FROM detections WHERE stream_name = ? "
                "AND source != 'external_motion' AND timestamp >= ? "
                "UNION ALL "
                "SELECT label, confidence, x, y, width, height, timestamp "
                "FROM detections WHERE stream_name = ? "
                "AND source = 'external_motion' "
                "AND COALESCE(event_end_time, CAST(strftime('%%s','now') AS INTEGER)) >= ?"
                ") ORDER BY timestamp DESC LIMIT ?;");

        rc = sqlite3_prepare_v2(db, sql, -1, &stmt, NULL);
        if (rc != SQLITE_OK) {
            log_error("Failed to prepare statement: %s", sqlite3_errmsg(db));
            pthread_mutex_unlock(db_mutex);
            return -1;
        }

        // Bind parameters
        sqlite3_bind_text(stmt, 1, stream_name, -1, SQLITE_STATIC);
        sqlite3_bind_int64(stmt, 2, (sqlite3_int64)cutoff_time);
        sqlite3_bind_text(stmt, 3, stream_name, -1, SQLITE_STATIC);
        sqlite3_bind_int64(stmt, 4, (sqlite3_int64)cutoff_time);
        sqlite3_bind_int(stmt, 5, MAX_DETECTIONS);
    } else {
        // No filters, just get the latest detections
        log_debug("Getting latest detections for stream %s (no time filters)", stream_name);
        
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
            safe_strcpy(result->detections[count].label, label, MAX_LABEL_LENGTH, 0);
        } else {
            safe_strcpy(result->detections[count].label, "unknown", MAX_LABEL_LENGTH, 0);
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

    log_info("Found %d detections in database for stream %s (start=%lld, end=%lld)",
             count, stream_name, (long long)start_time, (long long)end_time);
    return count;
}

/**
 * Get timestamps for detections
 * 
 * @param stream_name Stream name
 * @param result Detection results to match
 * @param timestamps Array to store timestamps (must be same size as result->count)
 * @param max_age Maximum age in seconds (0 for all)
 * @param start_time Start time filter (0 for no filter)
 * @param end_time End time filter (0 for no filter)
 * @return 0 on success, non-zero on failure
 */
int get_detection_time_ranges(const char *stream_name,
                              const detection_result_t *result,
                              time_t *timestamps,
                              time_t *end_timestamps,
                              uint64_t max_age,
                              time_t start_time,
                              time_t end_time) {
    int rc;
    sqlite3_stmt *stmt;
    
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *db_mutex = get_db_mutex();
    
    if (!db) {
        log_error("Database not initialized");
        return -1;
    }
    
    if (!stream_name || !result || !timestamps) {
        log_error("Invalid parameters for get_detection_timestamps");
        return -1;
    }
    
    pthread_mutex_lock(db_mutex);
    
    // Build query based on filters
    char sql[2048];
    
    if (start_time > 0 && end_time > 0) {
        // Time range filter
        snprintf(sql, sizeof(sql),
                "SELECT timestamp, interval_end, label, confidence, x, y, width, height FROM ("
                "SELECT timestamp, COALESCE(event_end_time, timestamp) AS interval_end, "
                "label, confidence, x, y, width, height "
                "FROM detections WHERE stream_name = ? "
                "AND source != 'external_motion' AND timestamp >= ? AND timestamp <= ? "
                "UNION ALL "
                "SELECT timestamp, COALESCE(event_end_time, CAST(strftime('%%s','now') AS INTEGER)) AS interval_end, "
                "label, confidence, x, y, width, height "
                "FROM detections WHERE stream_name = ? "
                "AND source = 'external_motion' AND timestamp <= ? "
                "AND COALESCE(event_end_time, CAST(strftime('%%s','now') AS INTEGER)) >= ?"
                ") ORDER BY timestamp DESC LIMIT ?;");
        
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
        sqlite3_bind_text(stmt, 4, stream_name, -1, SQLITE_STATIC);
        sqlite3_bind_int64(stmt, 5, (sqlite3_int64)end_time);
        sqlite3_bind_int64(stmt, 6, (sqlite3_int64)start_time);
        sqlite3_bind_int(stmt, 7, MAX_DETECTIONS);
    } else if (start_time > 0) {
        // Start time filter only
        snprintf(sql, sizeof(sql),
                "SELECT timestamp, interval_end, label, confidence, x, y, width, height FROM ("
                "SELECT timestamp, COALESCE(event_end_time, timestamp) AS interval_end, "
                "label, confidence, x, y, width, height "
                "FROM detections WHERE stream_name = ? "
                "AND source != 'external_motion' AND timestamp >= ? "
                "UNION ALL "
                "SELECT timestamp, COALESCE(event_end_time, CAST(strftime('%%s','now') AS INTEGER)) AS interval_end, "
                "label, confidence, x, y, width, height "
                "FROM detections WHERE stream_name = ? "
                "AND source = 'external_motion' "
                "AND COALESCE(event_end_time, CAST(strftime('%%s','now') AS INTEGER)) >= ?"
                ") ORDER BY timestamp DESC LIMIT ?;");
        
        rc = sqlite3_prepare_v2(db, sql, -1, &stmt, NULL);
        if (rc != SQLITE_OK) {
            log_error("Failed to prepare statement: %s", sqlite3_errmsg(db));
            pthread_mutex_unlock(db_mutex);
            return -1;
        }
        
        // Bind parameters
        sqlite3_bind_text(stmt, 1, stream_name, -1, SQLITE_STATIC);
        sqlite3_bind_int64(stmt, 2, (sqlite3_int64)start_time);
        sqlite3_bind_text(stmt, 3, stream_name, -1, SQLITE_STATIC);
        sqlite3_bind_int64(stmt, 4, (sqlite3_int64)start_time);
        sqlite3_bind_int(stmt, 5, MAX_DETECTIONS);
    } else if (end_time > 0) {
        // End time filter only
        snprintf(sql, sizeof(sql),
                "SELECT timestamp, COALESCE(event_end_time, CAST(strftime('%%s','now') AS INTEGER)), label, confidence, x, y, width, height "
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
        time_t cutoff_time = time(NULL) - (time_t)max_age;
        
        // First get the latest timestamp
        snprintf(sql, sizeof(sql),
                "SELECT timestamp, interval_end, label, confidence, x, y, width, height FROM ("
                "SELECT timestamp, COALESCE(event_end_time, timestamp) AS interval_end, "
                "label, confidence, x, y, width, height "
                "FROM detections WHERE stream_name = ? "
                "AND source != 'external_motion' AND timestamp >= ? "
                "UNION ALL "
                "SELECT timestamp, COALESCE(event_end_time, CAST(strftime('%%s','now') AS INTEGER)) AS interval_end, "
                "label, confidence, x, y, width, height "
                "FROM detections WHERE stream_name = ? "
                "AND source = 'external_motion' "
                "AND COALESCE(event_end_time, CAST(strftime('%%s','now') AS INTEGER)) >= ?"
                ") ORDER BY timestamp DESC LIMIT ?;");
        
        rc = sqlite3_prepare_v2(db, sql, -1, &stmt, NULL);
        if (rc != SQLITE_OK) {
            log_error("Failed to prepare statement: %s", sqlite3_errmsg(db));
            pthread_mutex_unlock(db_mutex);
            return -1;
        }
        
        // Bind parameters
        sqlite3_bind_text(stmt, 1, stream_name, -1, SQLITE_STATIC);
        sqlite3_bind_int64(stmt, 2, (sqlite3_int64)cutoff_time);
        sqlite3_bind_text(stmt, 3, stream_name, -1, SQLITE_STATIC);
        sqlite3_bind_int64(stmt, 4, (sqlite3_int64)cutoff_time);
        sqlite3_bind_int(stmt, 5, MAX_DETECTIONS);
    } else {
        // No filters, just get the latest detections
        snprintf(sql, sizeof(sql),
                "SELECT timestamp, COALESCE(event_end_time, CAST(strftime('%%s','now') AS INTEGER)), label, confidence, x, y, width, height "
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
    // Both get_detections_from_db_time_range and this function use the same
    // ORDER BY timestamp DESC, so results are in the same order - just assign directly
    int count = 0;
    while (sqlite3_step(stmt) == SQLITE_ROW && count < result->count) {
        // Get timestamp and assign directly by index
        // The queries return results in the same order, so index matching works
        timestamps[count] = (time_t)sqlite3_column_int64(stmt, 0);
        if (end_timestamps) {
            end_timestamps[count] = (time_t)sqlite3_column_int64(stmt, 1);
        }
        count++;
    }

    sqlite3_finalize(stmt);
    pthread_mutex_unlock(db_mutex);

    return 0;
}

int get_detection_timestamps(const char *stream_name,
                             const detection_result_t *result,
                             time_t *timestamps,
                             uint64_t max_age,
                             time_t start_time,
                             time_t end_time) {
    return get_detection_time_ranges(stream_name, result, timestamps, NULL,
                                     max_age, start_time, end_time);
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
 * Check if there are any detections for a stream within a time range
 *
 * @param stream_name Stream name
 * @param start_time Start time (inclusive)
 * @param end_time End time (inclusive)
 * @return 1 if detections exist, 0 if none, -1 on error
 */
int has_detections_in_time_range(const char *stream_name, time_t start_time, time_t end_time) {
    int rc;
    sqlite3_stmt *stmt;
    int has_detections = 0;

    sqlite3 *db = get_db_handle();
    pthread_mutex_t *db_mutex = get_db_mutex();

    if (!db) {
        log_error("Database not initialized");
        return -1;
    }

    if (!stream_name) {
        log_error("Stream name is required for has_detections_in_time_range");
        return -1;
    }

    log_debug("Checking for detections: stream=%s, start=%lld, end=%lld",
             stream_name, (long long)start_time, (long long)end_time);

    pthread_mutex_lock(db_mutex);

    // Keep point detections and true intervals in separate EXISTS clauses so
    // SQLite can apply both timestamp bounds to the large point-detection set.
    const char *sql =
        "SELECT ("
        "EXISTS(SELECT 1 FROM detections WHERE stream_name = ? "
        "AND source != 'external_motion' AND timestamp >= ? AND timestamp <= ? LIMIT 1) "
        "OR EXISTS(SELECT 1 FROM detections WHERE stream_name = ? "
        "AND source = 'external_motion' AND timestamp <= ? "
        "AND COALESCE(event_end_time, CAST(strftime('%s','now') AS INTEGER)) >= ? LIMIT 1)"
        ");";

    rc = sqlite3_prepare_v2(db, sql, -1, &stmt, NULL);
    if (rc != SQLITE_OK) {
        log_error("Failed to prepare statement: %s", sqlite3_errmsg(db));
        pthread_mutex_unlock(db_mutex);
        return -1;
    }

    sqlite3_bind_text(stmt, 1, stream_name, -1, SQLITE_STATIC);
    sqlite3_bind_int64(stmt, 2, (sqlite3_int64)start_time);
    sqlite3_bind_int64(stmt, 3, (sqlite3_int64)end_time);
    sqlite3_bind_text(stmt, 4, stream_name, -1, SQLITE_STATIC);
    sqlite3_bind_int64(stmt, 5, (sqlite3_int64)end_time);
    sqlite3_bind_int64(stmt, 6, (sqlite3_int64)start_time);

    rc = sqlite3_step(stmt);
    if (rc == SQLITE_ROW) {
        has_detections = sqlite3_column_int(stmt, 0);
        log_debug("Detection check result for stream %s: %d", stream_name, has_detections);
    } else {
        log_error("Failed to check for detections: %s", sqlite3_errmsg(db));
        sqlite3_finalize(stmt);
        pthread_mutex_unlock(db_mutex);
        return -1;
    }

    sqlite3_finalize(stmt);
    pthread_mutex_unlock(db_mutex);

    return has_detections;
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
    
    const char *sql =
        "DELETE FROM detections WHERE "
        "CASE WHEN source = 'external_motion' AND event_end_time IS NULL "
        "THEN CAST(strftime('%s','now') AS INTEGER) "
        "ELSE COALESCE(event_end_time, timestamp) END < ?;";
    
    rc = sqlite3_prepare_v2(db, sql, -1, &stmt, NULL);
    if (rc != SQLITE_OK) {
        log_error("Failed to prepare statement: %s", sqlite3_errmsg(db));
        pthread_mutex_unlock(db_mutex);
        return -1;
    }
    
    // Calculate cutoff time
    time_t cutoff_time = time(NULL) - (time_t)max_age;
    
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

    // finalize the prepared statement
    sqlite3_finalize(stmt);
    pthread_mutex_unlock(db_mutex);

    log_info("Deleted %d old detections from database", deleted_count);
    return deleted_count;
}

/**
 * Get a summary of detection labels for a stream within a time range
 * Returns unique labels with their counts, sorted by count descending
 *
 * @param stream_name Stream name
 * @param start_time Start time (inclusive)
 * @param end_time End time (inclusive)
 * @param labels Array to store label summaries (must have space for max_labels entries)
 * @param max_labels Maximum number of labels to return
 * @return Number of unique labels found, or -1 on error
 */
int get_detection_labels_summary(const char *stream_name, time_t start_time, time_t end_time,
                                 detection_label_summary_t *labels, int max_labels) {
    int rc;
    sqlite3_stmt *stmt;
    int count = 0;

    sqlite3 *db = get_db_handle();
    pthread_mutex_t *db_mutex = get_db_mutex();

    if (!db) {
        log_error("Database not initialized");
        return -1;
    }

    if (!stream_name || !labels || max_labels <= 0) {
        log_error("Invalid parameters for get_detection_labels_summary");
        return -1;
    }

    // Initialize labels array
    memset(labels, 0, max_labels * sizeof(detection_label_summary_t));

    pthread_mutex_lock(db_mutex);

    // Aggregate point detections and true intervals independently. A single
    // overlap predicate prevents SQLite from using the lower timestamp bound
    // for millions of ordinary point detections.
    const char *sql =
        "SELECT label, SUM(cnt) AS cnt FROM ("
        "SELECT label, COUNT(*) AS cnt FROM detections "
        "WHERE stream_name = ? AND source != 'external_motion' "
        "AND timestamp >= ? AND timestamp <= ? GROUP BY label "
        "UNION ALL "
        "SELECT label, COUNT(*) AS cnt FROM detections "
        "WHERE stream_name = ? AND source = 'external_motion' "
        "AND timestamp <= ? "
        "AND COALESCE(event_end_time, CAST(strftime('%s','now') AS INTEGER)) >= ? "
        "GROUP BY label"
        ") GROUP BY label ORDER BY cnt DESC "
        "LIMIT ?;";

    rc = sqlite3_prepare_v2(db, sql, -1, &stmt, NULL);
    if (rc != SQLITE_OK) {
        log_error("Failed to prepare statement for get_detection_labels_summary: %s", sqlite3_errmsg(db));
        pthread_mutex_unlock(db_mutex);
        return -1;
    }

    // Bind parameters
    sqlite3_bind_text(stmt, 1, stream_name, -1, SQLITE_STATIC);
    sqlite3_bind_int64(stmt, 2, (sqlite3_int64)start_time);
    sqlite3_bind_int64(stmt, 3, (sqlite3_int64)end_time);
    sqlite3_bind_text(stmt, 4, stream_name, -1, SQLITE_STATIC);
    sqlite3_bind_int64(stmt, 5, (sqlite3_int64)end_time);
    sqlite3_bind_int64(stmt, 6, (sqlite3_int64)start_time);
    sqlite3_bind_int(stmt, 7, max_labels);

    // Execute query and fetch results
    while ((rc = sqlite3_step(stmt)) == SQLITE_ROW && count < max_labels) {
        const char *label = (const char *)sqlite3_column_text(stmt, 0);
        int label_count = sqlite3_column_int(stmt, 1);

        if (label) {
            safe_strcpy(labels[count].label, label, MAX_LABEL_LENGTH, 0);
            labels[count].count = label_count;
            count++;
        }
    }

    if (rc != SQLITE_DONE && rc != SQLITE_ROW) {
        log_error("Failed to fetch detection labels: %s", sqlite3_errmsg(db));
        sqlite3_finalize(stmt);
        pthread_mutex_unlock(db_mutex);
        return -1;
    }

    sqlite3_finalize(stmt);
    pthread_mutex_unlock(db_mutex);

    return count;
}

int get_all_unique_detection_labels(char labels[][MAX_LABEL_LENGTH], int max_labels) {
    int rc;
    sqlite3_stmt *stmt;
    int count = 0;

    sqlite3 *db = get_db_handle();
    pthread_mutex_t *db_mutex = get_db_mutex();

    if (!db) {
        log_error("Database not initialized");
        return -1;
    }

    if (!labels || max_labels <= 0) {
        log_error("Invalid parameters for get_all_unique_detection_labels");
        return -1;
    }

    memset(labels, 0, (size_t)max_labels * MAX_LABEL_LENGTH);

    pthread_mutex_lock(db_mutex);

    const char *sql =
        "SELECT DISTINCT label "
        "FROM detections "
        "WHERE label IS NOT NULL AND TRIM(label) <> '' "
        "ORDER BY label ASC "
        "LIMIT ?;";

    rc = sqlite3_prepare_v2(db, sql, -1, &stmt, NULL);
    if (rc != SQLITE_OK) {
        log_error("Failed to prepare statement for get_all_unique_detection_labels: %s", sqlite3_errmsg(db));
        pthread_mutex_unlock(db_mutex);
        return -1;
    }

    sqlite3_bind_int(stmt, 1, max_labels);

    while ((rc = sqlite3_step(stmt)) == SQLITE_ROW && count < max_labels) {
        const char *label = (const char *)sqlite3_column_text(stmt, 0);

        if (label) {
            safe_strcpy(labels[count], label, MAX_LABEL_LENGTH, 0);
            count++;
        }
    }

    if (rc != SQLITE_DONE && rc != SQLITE_ROW) {
        log_error("Failed to fetch unique detection labels: %s", sqlite3_errmsg(db));
        sqlite3_finalize(stmt);
        pthread_mutex_unlock(db_mutex);
        return -1;
    }

    sqlite3_finalize(stmt);
    pthread_mutex_unlock(db_mutex);

    return count;
}

/**
 * Update recent detections with a recording_id
 * This links detections that were stored before the recording was created to the recording.
 * Useful for detection-triggered recordings where the first detection triggers the recording.
 */
int update_detections_recording_id(const char *stream_name, uint64_t recording_id, time_t since_time) {
    int rc;
    sqlite3_stmt *stmt;

    sqlite3 *db = get_db_handle();
    pthread_mutex_t *db_mutex = get_db_mutex();

    if (!db) {
        log_error("Database not initialized");
        return -1;
    }

    if (!stream_name || recording_id == 0) {
        log_error("Invalid parameters for update_detections_recording_id");
        return -1;
    }

    pthread_mutex_lock(db_mutex);

    // Update detections where recording_id is NULL or 0 for the given stream and time range
    const char *sql =
        "UPDATE detections "
        "SET recording_id = ? "
        "WHERE stream_name = ? AND timestamp >= ? AND (recording_id IS NULL OR recording_id = 0);";

    rc = sqlite3_prepare_v2(db, sql, -1, &stmt, NULL);
    if (rc != SQLITE_OK) {
        log_error("Failed to prepare statement for update_detections_recording_id: %s", sqlite3_errmsg(db));
        pthread_mutex_unlock(db_mutex);
        return -1;
    }

    // Bind parameters
    sqlite3_bind_int64(stmt, 1, (sqlite3_int64)recording_id);
    sqlite3_bind_text(stmt, 2, stream_name, -1, SQLITE_STATIC);
    sqlite3_bind_int64(stmt, 3, (sqlite3_int64)since_time);

    rc = sqlite3_step(stmt);
    if (rc != SQLITE_DONE) {
        log_error("Failed to update detections with recording_id: %s", sqlite3_errmsg(db));
        sqlite3_finalize(stmt);
        pthread_mutex_unlock(db_mutex);
        return -1;
    }

    int updated = sqlite3_changes(db);

    sqlite3_finalize(stmt);
    pthread_mutex_unlock(db_mutex);

    if (updated > 0) {
        log_debug("Updated %d detections with recording_id %lu for stream %s",
                  updated, (unsigned long)recording_id, stream_name);
    }

    return updated;
}
