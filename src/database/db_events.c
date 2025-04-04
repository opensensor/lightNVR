#define _POSIX_C_SOURCE 200809L
#define _XOPEN_SOURCE 700
#define _GNU_SOURCE

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>
#include <sqlite3.h>
#include <stdbool.h>

#include "database/db_events.h"
#include "database/db_core.h"
#include "core/logger.h"

// Add an event to the database
uint64_t add_event(event_type_t type, const char *stream_name, 
                  const char *description, const char *details) {
    int rc;
    sqlite3_stmt *stmt;
    uint64_t event_id = 0;
    
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *db_mutex = get_db_mutex();
    
    if (!db) {
        log_error("Database not initialized");
        return 0;
    }
    
    if (!description) {
        log_error("Event description is required");
        return 0;
    }
    
    pthread_mutex_lock(db_mutex);
    
    const char *sql = "INSERT INTO events (type, timestamp, stream_name, description, details) "
                      "VALUES (?, ?, ?, ?, ?);";
    
    rc = sqlite3_prepare_v2(db, sql, -1, &stmt, NULL);
    if (rc != SQLITE_OK) {
        log_error("Failed to prepare statement: %s", sqlite3_errmsg(db));
        pthread_mutex_unlock(db_mutex);
        return 0;
    }
    
    // Bind parameters
    sqlite3_bind_int(stmt, 1, (int)type);
    sqlite3_bind_int64(stmt, 2, (sqlite3_int64)time(NULL));
    
    if (stream_name) {
        sqlite3_bind_text(stmt, 3, stream_name, -1, SQLITE_STATIC);
    } else {
        sqlite3_bind_null(stmt, 3);
    }
    
    sqlite3_bind_text(stmt, 4, description, -1, SQLITE_STATIC);
    
    if (details) {
        sqlite3_bind_text(stmt, 5, details, -1, SQLITE_STATIC);
    } else {
        sqlite3_bind_null(stmt, 5);
    }
    
    // Execute statement
    rc = sqlite3_step(stmt);
    if (rc != SQLITE_DONE) {
        log_error("Failed to add event: %s", sqlite3_errmsg(db));
    } else {
        event_id = (uint64_t)sqlite3_last_insert_rowid(db);
        log_debug("Added event with ID %llu", (unsigned long long)event_id);
    }
    
    // finalize the prepared statement
    sqlite3_finalize(stmt);
    pthread_mutex_unlock(db_mutex);
    
    return event_id;
}

// Get events from the database
int get_events(time_t start_time, time_t end_time, int type, 
              const char *stream_name, event_info_t *events, int max_count) {
    int rc;
    sqlite3_stmt *stmt;
    int count = 0;
    
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *db_mutex = get_db_mutex();
    
    if (!db) {
        log_error("Database not initialized");
        return -1;
    }
    
    if (!events || max_count <= 0) {
        log_error("Invalid parameters for get_events");
        return -1;
    }
    
    pthread_mutex_lock(db_mutex);
    
    // Build query based on filters
    char sql[1024];
    strcpy(sql, "SELECT id, type, timestamp, stream_name, description, details FROM events WHERE 1=1");
    
    if (start_time > 0) {
        strcat(sql, " AND timestamp >= ?");
    }
    
    if (end_time > 0) {
        strcat(sql, " AND timestamp <= ?");
    }
    
    if (type >= 0) {
        strcat(sql, " AND type = ?");
    }
    
    if (stream_name) {
        strcat(sql, " AND stream_name = ?");
    }
    
    strcat(sql, " ORDER BY timestamp DESC LIMIT ?;");
    
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
    
    if (type >= 0) {
        sqlite3_bind_int(stmt, param_index++, type);
    }
    
    if (stream_name) {
        sqlite3_bind_text(stmt, param_index++, stream_name, -1, SQLITE_STATIC);
    }
    
    sqlite3_bind_int(stmt, param_index, max_count);
    
    // Execute query and fetch results
    while (sqlite3_step(stmt) == SQLITE_ROW && count < max_count) {
        events[count].id = (uint64_t)sqlite3_column_int64(stmt, 0);
        events[count].type = (event_type_t)sqlite3_column_int(stmt, 1);
        events[count].timestamp = (time_t)sqlite3_column_int64(stmt, 2);
        
        const char *stream = (const char *)sqlite3_column_text(stmt, 3);
        if (stream) {
            strncpy(events[count].stream_name, stream, sizeof(events[count].stream_name) - 1);
            events[count].stream_name[sizeof(events[count].stream_name) - 1] = '\0';
        } else {
            events[count].stream_name[0] = '\0';
        }
        
        const char *desc = (const char *)sqlite3_column_text(stmt, 4);
        if (desc) {
            strncpy(events[count].description, desc, sizeof(events[count].description) - 1);
            events[count].description[sizeof(events[count].description) - 1] = '\0';
        } else {
            events[count].description[0] = '\0';
        }
        
        const char *details = (const char *)sqlite3_column_text(stmt, 5);
        if (details) {
            strncpy(events[count].details, details, sizeof(events[count].details) - 1);
            events[count].details[sizeof(events[count].details) - 1] = '\0';
        } else {
            events[count].details[0] = '\0';
        }
        
        count++;
    }
    
    // finalize the prepared statement
    sqlite3_finalize(stmt);
    pthread_mutex_unlock(db_mutex);
    
    log_info("Found %d events in database matching criteria", count);
    return count;
}

// Delete old events from the database
int delete_old_events(uint64_t max_age) {
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
    
    const char *sql = "DELETE FROM events WHERE timestamp < ?;";
    
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
        log_error("Failed to delete old events: %s", sqlite3_errmsg(db));
        sqlite3_finalize(stmt);
        pthread_mutex_unlock(db_mutex);
        return -1;
    }
    
    deleted_count = sqlite3_changes(db);
    
    // finalize the prepared statement
    sqlite3_finalize(stmt);
    pthread_mutex_unlock(db_mutex);
    
    return deleted_count;
}
