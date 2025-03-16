#define _POSIX_C_SOURCE 200809L
#define _XOPEN_SOURCE 700
#define _GNU_SOURCE

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>
#include <sys/time.h>
#include <sqlite3.h>
#include <pthread.h>
#include <unistd.h>
#include <sys/stat.h>
#include <sys/types.h>
#include <errno.h>
#include <libgen.h>

#include "database/database_manager.h"
#include "core/logger.h"
#include "core/config.h"

// Database handle
static sqlite3 *db = NULL;

// Mutex for thread safety
static pthread_mutex_t db_mutex;

// Create directory if it doesn't exist
static int create_directory(const char *path) {
    struct stat st;
    
    // Check if directory already exists
    if (stat(path, &st) == 0) {
        if (S_ISDIR(st.st_mode)) {
            return 0; // Directory exists
        } else {
            return -1; // Path exists but is not a directory
        }
    }
    
    // Create directory with permissions 0755
    if (mkdir(path, 0755) != 0) {
        if (errno == ENOENT) {
            // Parent directory doesn't exist, try to create it recursively
            char *parent_path = strdup(path);
            if (!parent_path) {
                return -1;
            }
            
            char *parent_dir = dirname(parent_path);
            int ret = create_directory(parent_dir);
            free(parent_path);
            
            if (ret != 0) {
                return -1;
            }
            
            // Try again to create the directory
            if (mkdir(path, 0755) != 0) {
                return -1;
            }
        } else {
            return -1;
        }
    }
    
    return 0;
}

// Initialize the database
int init_database(const char *db_path) {
    int rc;
    char *err_msg = NULL;
    
    // Initialize mutex
    if (pthread_mutex_init(&db_mutex, NULL) != 0) {
        log_error("Failed to initialize database mutex");
        return -1;
    }
    
    // Create directory for database if needed
    char *dir_path = strdup(db_path);
    if (!dir_path) {
        log_error("Failed to allocate memory for database directory path");
        return -1;
    }
    
    char *dir = dirname(dir_path);
    if (create_directory(dir) != 0) {
        log_error("Failed to create database directory: %s", dir);
        free(dir_path);
        return -1;
    }
    free(dir_path);
    
    // Open database
    rc = sqlite3_open(db_path, &db);
    if (rc != SQLITE_OK) {
        log_error("Failed to open database: %s", sqlite3_errmsg(db));
        sqlite3_close(db);
        db = NULL;
        return -1;
    }
    
    // Enable foreign keys
    rc = sqlite3_exec(db, "PRAGMA foreign_keys = ON;", NULL, NULL, &err_msg);
    if (rc != SQLITE_OK) {
        log_error("Failed to enable foreign keys: %s", err_msg);
        sqlite3_free(err_msg);
        sqlite3_close(db);
        db = NULL;
        return -1;
    }
    
    // Create events table
    const char *create_events_table = 
        "CREATE TABLE IF NOT EXISTS events ("
        "id INTEGER PRIMARY KEY AUTOINCREMENT,"
        "type INTEGER NOT NULL,"
        "timestamp INTEGER NOT NULL,"
        "stream_name TEXT,"
        "description TEXT NOT NULL,"
        "details TEXT"
        ");";
    
    rc = sqlite3_exec(db, create_events_table, NULL, NULL, &err_msg);
    if (rc != SQLITE_OK) {
        log_error("Failed to create events table: %s", err_msg);
        sqlite3_free(err_msg);
        sqlite3_close(db);
        db = NULL;
        return -1;
    }
    
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
        sqlite3_close(db);
        db = NULL;
        return -1;
    }
    
    // Create recordings table
    const char *create_recordings_table = 
        "CREATE TABLE IF NOT EXISTS recordings ("
        "id INTEGER PRIMARY KEY AUTOINCREMENT,"
        "stream_name TEXT NOT NULL,"
        "file_path TEXT NOT NULL,"
        "start_time INTEGER NOT NULL,"
        "end_time INTEGER,"
        "size_bytes INTEGER,"
        "width INTEGER,"
        "height INTEGER,"
        "fps INTEGER,"
        "codec TEXT,"
        "is_complete INTEGER DEFAULT 0"
        ");";
        
    // Create streams table for storing stream configurations
    const char *create_streams_table = 
        "CREATE TABLE IF NOT EXISTS streams ("
        "id INTEGER PRIMARY KEY AUTOINCREMENT,"
        "name TEXT NOT NULL UNIQUE,"
        "url TEXT NOT NULL,"
        "enabled INTEGER DEFAULT 1,"
        "width INTEGER DEFAULT 1280,"
        "height INTEGER DEFAULT 720,"
        "fps INTEGER DEFAULT 30,"
        "codec TEXT DEFAULT 'h264',"
        "priority INTEGER DEFAULT 5,"
        "record INTEGER DEFAULT 1,"
        "segment_duration INTEGER DEFAULT 900"
        ");";
    
    rc = sqlite3_exec(db, create_recordings_table, NULL, NULL, &err_msg);
    if (rc != SQLITE_OK) {
        log_error("Failed to create recordings table: %s", err_msg);
        sqlite3_free(err_msg);
        sqlite3_close(db);
        db = NULL;
        return -1;
    }
    
    // Create streams table
    rc = sqlite3_exec(db, create_streams_table, NULL, NULL, &err_msg);
    if (rc != SQLITE_OK) {
        log_error("Failed to create streams table: %s", err_msg);
        sqlite3_free(err_msg);
        sqlite3_close(db);
        db = NULL;
        return -1;
    }
    
    // Create indexes for faster queries
    const char *create_indexes = 
        "CREATE INDEX IF NOT EXISTS idx_events_timestamp ON events (timestamp);"
        "CREATE INDEX IF NOT EXISTS idx_events_type ON events (type);"
        "CREATE INDEX IF NOT EXISTS idx_events_stream ON events (stream_name);"
        "CREATE INDEX IF NOT EXISTS idx_recordings_start_time ON recordings (start_time);"
        "CREATE INDEX IF NOT EXISTS idx_recordings_end_time ON recordings (end_time);"
        "CREATE INDEX IF NOT EXISTS idx_recordings_stream ON recordings (stream_name);"
        "CREATE INDEX IF NOT EXISTS idx_recordings_complete_stream_start ON recordings (is_complete, stream_name, start_time);"
        "CREATE INDEX IF NOT EXISTS idx_streams_name ON streams (name);"
        "CREATE INDEX IF NOT EXISTS idx_detections_stream_timestamp ON detections (stream_name, timestamp);"
        "CREATE INDEX IF NOT EXISTS idx_detections_timestamp ON detections (timestamp);";
    
    rc = sqlite3_exec(db, create_indexes, NULL, NULL, &err_msg);
    if (rc != SQLITE_OK) {
        log_error("Failed to create indexes: %s", err_msg);
        sqlite3_free(err_msg);
        sqlite3_close(db);
        db = NULL;
        return -1;
    }
    
    log_info("Database initialized successfully");
    return 0;
}

// Shutdown the database
void shutdown_database(void) {
    pthread_mutex_lock(&db_mutex);
    
    if (db != NULL) {
        sqlite3_close(db);
        db = NULL;
    }
    
    pthread_mutex_unlock(&db_mutex);
    pthread_mutex_destroy(&db_mutex);
    
    log_info("Database shutdown");
}

// Add an event to the database
uint64_t add_event(event_type_t type, const char *stream_name, 
                  const char *description, const char *details) {
    int rc;
    sqlite3_stmt *stmt;
    uint64_t event_id = 0;
    
    if (!db) {
        log_error("Database not initialized");
        return 0;
    }
    
    if (!description) {
        log_error("Event description is required");
        return 0;
    }
    
    pthread_mutex_lock(&db_mutex);
    
    const char *sql = "INSERT INTO events (type, timestamp, stream_name, description, details) "
                      "VALUES (?, ?, ?, ?, ?);";
    
    rc = sqlite3_prepare_v2(db, sql, -1, &stmt, NULL);
    if (rc != SQLITE_OK) {
        log_error("Failed to prepare statement: %s", sqlite3_errmsg(db));
        pthread_mutex_unlock(&db_mutex);
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
    
    sqlite3_finalize(stmt);
    pthread_mutex_unlock(&db_mutex);
    
    return event_id;
}

// Get events from the database
int get_events(time_t start_time, time_t end_time, int type, 
              const char *stream_name, event_info_t *events, int max_count) {
    int rc;
    sqlite3_stmt *stmt;
    int count = 0;
    
    if (!db) {
        log_error("Database not initialized");
        return -1;
    }
    
    if (!events || max_count <= 0) {
        log_error("Invalid parameters for get_events");
        return -1;
    }
    
    pthread_mutex_lock(&db_mutex);
    
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
        pthread_mutex_unlock(&db_mutex);
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
    
    sqlite3_finalize(stmt);
    pthread_mutex_unlock(&db_mutex);
    
    log_info("Found %d recordings in database matching criteria", count);
    return count;
}

// Add recording metadata to the database
uint64_t add_recording_metadata(const recording_metadata_t *metadata) {
    int rc;
    sqlite3_stmt *stmt;
    uint64_t recording_id = 0;
    
    if (!db) {
        log_error("Database not initialized");
        return 0;
    }
    
    if (!metadata) {
        log_error("Recording metadata is required");
        return 0;
    }
    
    pthread_mutex_lock(&db_mutex);
    
    const char *sql = "INSERT INTO recordings (stream_name, file_path, start_time, end_time, "
                      "size_bytes, width, height, fps, codec, is_complete) "
                      "VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?);";
    
    rc = sqlite3_prepare_v2(db, sql, -1, &stmt, NULL);
    if (rc != SQLITE_OK) {
        log_error("Failed to prepare statement: %s", sqlite3_errmsg(db));
        pthread_mutex_unlock(&db_mutex);
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
    pthread_mutex_unlock(&db_mutex);
    
    return recording_id;
}

// Update recording metadata in the database
int update_recording_metadata(uint64_t id, time_t end_time, 
                             uint64_t size_bytes, bool is_complete) {
    int rc;
    sqlite3_stmt *stmt;
    
    if (!db) {
        log_error("Database not initialized");
        return -1;
    }
    
    pthread_mutex_lock(&db_mutex);
    
    const char *sql = "UPDATE recordings SET end_time = ?, size_bytes = ?, is_complete = ? "
                      "WHERE id = ?;";
    
    rc = sqlite3_prepare_v2(db, sql, -1, &stmt, NULL);
    if (rc != SQLITE_OK) {
        log_error("Failed to prepare statement: %s", sqlite3_errmsg(db));
        pthread_mutex_unlock(&db_mutex);
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
        pthread_mutex_unlock(&db_mutex);
        return -1;
    }
    
    sqlite3_finalize(stmt);
    pthread_mutex_unlock(&db_mutex);
    
    return 0;
}

// Get recording metadata by ID
int get_recording_metadata_by_id(uint64_t id, recording_metadata_t *metadata) {
    int rc;
    sqlite3_stmt *stmt;
    int result = -1;
    
    if (!db) {
        log_error("Database not initialized");
        return -1;
    }
    
    if (!metadata) {
        log_error("Invalid parameters for get_recording_metadata_by_id");
        return -1;
    }
    
    pthread_mutex_lock(&db_mutex);
    
    const char *sql = "SELECT id, stream_name, file_path, start_time, end_time, "
                      "size_bytes, width, height, fps, codec, is_complete "
                      "FROM recordings WHERE id = ?;";
    
    rc = sqlite3_prepare_v2(db, sql, -1, &stmt, NULL);
    if (rc != SQLITE_OK) {
        log_error("Failed to prepare statement: %s", sqlite3_errmsg(db));
        pthread_mutex_unlock(&db_mutex);
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
    pthread_mutex_unlock(&db_mutex);
    
    return result;
}

// Get recording metadata from the database
int get_recording_metadata(time_t start_time, time_t end_time, 
                          const char *stream_name, recording_metadata_t *metadata, 
                          int max_count) {
    int rc;
    sqlite3_stmt *stmt;
    int count = 0;
    
    if (!db) {
        log_error("Database not initialized");
        return -1;
    }
    
    if (!metadata || max_count <= 0) {
        log_error("Invalid parameters for get_recording_metadata");
        return -1;
    }
    
    pthread_mutex_lock(&db_mutex);
    
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
        pthread_mutex_unlock(&db_mutex);
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
    pthread_mutex_unlock(&db_mutex);
    
    log_info("Found %d recordings in database matching criteria", count);
    return count;
}

// Delete recording metadata from the database
int delete_recording_metadata(uint64_t id) {
    int rc;
    sqlite3_stmt *stmt;
    
    if (!db) {
        log_error("Database not initialized");
        return -1;
    }
    
    pthread_mutex_lock(&db_mutex);
    
    const char *sql = "DELETE FROM recordings WHERE id = ?;";
    
    rc = sqlite3_prepare_v2(db, sql, -1, &stmt, NULL);
    if (rc != SQLITE_OK) {
        log_error("Failed to prepare statement: %s", sqlite3_errmsg(db));
        pthread_mutex_unlock(&db_mutex);
        return -1;
    }
    
    // Bind parameters
    sqlite3_bind_int64(stmt, 1, (sqlite3_int64)id);
    
    // Execute statement
    rc = sqlite3_step(stmt);
    if (rc != SQLITE_DONE) {
        log_error("Failed to delete recording metadata: %s", sqlite3_errmsg(db));
        sqlite3_finalize(stmt);
        pthread_mutex_unlock(&db_mutex);
        return -1;
    }
    
    sqlite3_finalize(stmt);
    pthread_mutex_unlock(&db_mutex);
    
    return 0;
}

// Delete old events from the database
int delete_old_events(uint64_t max_age) {
    int rc;
    sqlite3_stmt *stmt;
    int deleted_count = 0;
    
    if (!db) {
        log_error("Database not initialized");
        return -1;
    }
    
    pthread_mutex_lock(&db_mutex);
    
    const char *sql = "DELETE FROM events WHERE timestamp < ?;";
    
    rc = sqlite3_prepare_v2(db, sql, -1, &stmt, NULL);
    if (rc != SQLITE_OK) {
        log_error("Failed to prepare statement: %s", sqlite3_errmsg(db));
        pthread_mutex_unlock(&db_mutex);
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
        pthread_mutex_unlock(&db_mutex);
        return -1;
    }
    
    deleted_count = sqlite3_changes(db);
    
    sqlite3_finalize(stmt);
    pthread_mutex_unlock(&db_mutex);
    
    return deleted_count;
}

// Delete old recording metadata from the database
int delete_old_recording_metadata(uint64_t max_age) {
    int rc;
    sqlite3_stmt *stmt;
    int deleted_count = 0;
    
    if (!db) {
        log_error("Database not initialized");
        return -1;
    }
    
    pthread_mutex_lock(&db_mutex);
    
    const char *sql = "DELETE FROM recordings WHERE end_time < ?;";
    
    rc = sqlite3_prepare_v2(db, sql, -1, &stmt, NULL);
    if (rc != SQLITE_OK) {
        log_error("Failed to prepare statement: %s", sqlite3_errmsg(db));
        pthread_mutex_unlock(&db_mutex);
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
        pthread_mutex_unlock(&db_mutex);
        return -1;
    }
    
    deleted_count = sqlite3_changes(db);
    
    sqlite3_finalize(stmt);
    pthread_mutex_unlock(&db_mutex);
    
    return deleted_count;
}

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
    
    if (!db) {
        log_error("Database not initialized");
        return -1;
    }
    
    if (!stream_name || !result) {
        log_error("Invalid parameters for store_detections_in_db");
        return -1;
    }
    
    // Use current time if timestamp is 0
    if (timestamp == 0) {
        timestamp = time(NULL);
    }
    
    pthread_mutex_lock(&db_mutex);
    
    // Begin transaction for better performance when inserting multiple detections
    char *err_msg = NULL;
    rc = sqlite3_exec(db, "BEGIN TRANSACTION;", NULL, NULL, &err_msg);
    if (rc != SQLITE_OK) {
        log_error("Failed to begin transaction: %s", err_msg);
        sqlite3_free(err_msg);
        pthread_mutex_unlock(&db_mutex);
        return -1;
    }
    
    const char *sql = "INSERT INTO detections (stream_name, timestamp, label, confidence, x, y, width, height) "
                      "VALUES (?, ?, ?, ?, ?, ?, ?, ?);";
    
    rc = sqlite3_prepare_v2(db, sql, -1, &stmt, NULL);
    if (rc != SQLITE_OK) {
        log_error("Failed to prepare statement: %s", sqlite3_errmsg(db));
        sqlite3_exec(db, "ROLLBACK;", NULL, NULL, NULL);
        pthread_mutex_unlock(&db_mutex);
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
            log_error("Failed to insert detection: %s", sqlite3_errmsg(db));
            sqlite3_finalize(stmt);
            sqlite3_exec(db, "ROLLBACK;", NULL, NULL, NULL);
            pthread_mutex_unlock(&db_mutex);
            return -1;
        }
        
        // Reset statement for next detection
        sqlite3_reset(stmt);
    }
    
    sqlite3_finalize(stmt);
    
    // Commit transaction
    rc = sqlite3_exec(db, "COMMIT;", NULL, NULL, &err_msg);
    if (rc != SQLITE_OK) {
        log_error("Failed to commit transaction: %s", err_msg);
        sqlite3_free(err_msg);
        sqlite3_exec(db, "ROLLBACK;", NULL, NULL, NULL);
        pthread_mutex_unlock(&db_mutex);
        return -1;
    }
    
    pthread_mutex_unlock(&db_mutex);
    
    log_info("Stored %d detections in database for stream %s", result->count, stream_name);
    return 0;
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
    int rc;
    sqlite3_stmt *stmt;
    
    if (!db) {
        log_error("Database not initialized");
        return -1;
    }
    
    if (!stream_name || !result) {
        log_error("Invalid parameters for get_detections_from_db");
        return -1;
    }
    
    // Initialize result
    memset(result, 0, sizeof(detection_result_t));
    
    pthread_mutex_lock(&db_mutex);
    
    // Build query based on max_age
    char sql[512];
    if (max_age > 0) {
        // Calculate cutoff time
        time_t cutoff_time = time(NULL) - max_age;
        
        snprintf(sql, sizeof(sql), 
                "SELECT label, confidence, x, y, width, height "
                "FROM detections "
                "WHERE stream_name = ? AND timestamp >= ? "
                "ORDER BY timestamp DESC "
                "LIMIT ?;");
        
        rc = sqlite3_prepare_v2(db, sql, -1, &stmt, NULL);
        if (rc != SQLITE_OK) {
            log_error("Failed to prepare statement: %s", sqlite3_errmsg(db));
            pthread_mutex_unlock(&db_mutex);
            return -1;
        }
        
        // Bind parameters
        sqlite3_bind_text(stmt, 1, stream_name, -1, SQLITE_STATIC);
        sqlite3_bind_int64(stmt, 2, (sqlite3_int64)cutoff_time);
        sqlite3_bind_int(stmt, 3, MAX_DETECTIONS);
    } else {
        // No age filter
        snprintf(sql, sizeof(sql), 
                "SELECT label, confidence, x, y, width, height "
                "FROM detections "
                "WHERE stream_name = ? "
                "ORDER BY timestamp DESC "
                "LIMIT ?;");
        
        rc = sqlite3_prepare_v2(db, sql, -1, &stmt, NULL);
        if (rc != SQLITE_OK) {
            log_error("Failed to prepare statement: %s", sqlite3_errmsg(db));
            pthread_mutex_unlock(&db_mutex);
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
    pthread_mutex_unlock(&db_mutex);
    
    log_info("Found %d detections in database for stream %s", count, stream_name);
    return count;
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
    
    if (!db) {
        log_error("Database not initialized");
        return -1;
    }
    
    pthread_mutex_lock(&db_mutex);
    
    const char *sql = "DELETE FROM detections WHERE timestamp < ?;";
    
    rc = sqlite3_prepare_v2(db, sql, -1, &stmt, NULL);
    if (rc != SQLITE_OK) {
        log_error("Failed to prepare statement: %s", sqlite3_errmsg(db));
        pthread_mutex_unlock(&db_mutex);
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
        pthread_mutex_unlock(&db_mutex);
        return -1;
    }
    
    deleted_count = sqlite3_changes(db);
    
    sqlite3_finalize(stmt);
    pthread_mutex_unlock(&db_mutex);
    
    log_info("Deleted %d old detections from database", deleted_count);
    return deleted_count;
}

// Begin a database transaction with improved error handling
int begin_transaction(void) {
    int rc;
    char *err_msg = NULL;
    
    if (!db) {
        log_error("Database not initialized");
        return -1;
    }
    
    // Set a timeout for acquiring the mutex
    struct timespec timeout;
    clock_gettime(CLOCK_REALTIME, &timeout);
    timeout.tv_sec += 5; // 5 second timeout
    
    int lock_result = pthread_mutex_timedlock(&db_mutex, &timeout);
    if (lock_result != 0) {
        log_error("Failed to acquire database mutex for transaction: %s", strerror(lock_result));
        return -1;
    }
    
    rc = sqlite3_exec(db, "BEGIN TRANSACTION;", NULL, NULL, &err_msg);
    if (rc != SQLITE_OK) {
        log_error("Failed to begin transaction: %s", err_msg);
        sqlite3_free(err_msg);
        pthread_mutex_unlock(&db_mutex);
        return -1;
    }
    
    // Note: We do NOT unlock the mutex here - it will be unlocked by commit_transaction or rollback_transaction
    return 0;
}

// Commit a database transaction with improved error handling
int commit_transaction(void) {
    int rc;
    char *err_msg = NULL;
    
    if (!db) {
        log_error("Database not initialized");
        pthread_mutex_unlock(&db_mutex); // Always unlock the mutex
        return -1;
    }
    
    rc = sqlite3_exec(db, "COMMIT;", NULL, NULL, &err_msg);
    if (rc != SQLITE_OK) {
        log_error("Failed to commit transaction: %s", err_msg);
        sqlite3_free(err_msg);
        
        // Try to rollback on commit failure
        char *rollback_err = NULL;
        sqlite3_exec(db, "ROLLBACK;", NULL, NULL, &rollback_err);
        if (rollback_err) {
            sqlite3_free(rollback_err);
        }
        
        pthread_mutex_unlock(&db_mutex);
        return -1;
    }
    
    pthread_mutex_unlock(&db_mutex);
    return 0;
}

// Rollback a database transaction with improved error handling
int rollback_transaction(void) {
    int rc;
    char *err_msg = NULL;
    
    if (!db) {
        log_error("Database not initialized");
        pthread_mutex_unlock(&db_mutex); // Always unlock the mutex
        return -1;
    }
    
    rc = sqlite3_exec(db, "ROLLBACK;", NULL, NULL, &err_msg);
    if (rc != SQLITE_OK) {
        log_error("Failed to rollback transaction: %s", err_msg);
        sqlite3_free(err_msg);
        // Continue to unlock the mutex even on error
    }
    
    pthread_mutex_unlock(&db_mutex);
    return (rc == SQLITE_OK) ? 0 : -1;
}

// Get the database size
int64_t get_database_size(void) {
    int rc;
    sqlite3_stmt *stmt;
    int64_t size = -1;
    
    if (!db) {
        log_error("Database not initialized");
        return -1;
    }
    
    pthread_mutex_lock(&db_mutex);
    
    const char *sql = "PRAGMA page_count;";
    
    rc = sqlite3_prepare_v2(db, sql, -1, &stmt, NULL);
    if (rc != SQLITE_OK) {
        log_error("Failed to prepare statement: %s", sqlite3_errmsg(db));
        pthread_mutex_unlock(&db_mutex);
        return -1;
    }
    
    if (sqlite3_step(stmt) == SQLITE_ROW) {
        int64_t page_count = sqlite3_column_int64(stmt, 0);
        sqlite3_finalize(stmt);
        
        sql = "PRAGMA page_size;";
        rc = sqlite3_prepare_v2(db, sql, -1, &stmt, NULL);
        if (rc != SQLITE_OK) {
            log_error("Failed to prepare statement: %s", sqlite3_errmsg(db));
            pthread_mutex_unlock(&db_mutex);
            return -1;
        }
        
        if (sqlite3_step(stmt) == SQLITE_ROW) {
            int64_t page_size = sqlite3_column_int64(stmt, 0);
            size = page_count * page_size;
        }
    }
    
    sqlite3_finalize(stmt);
    pthread_mutex_unlock(&db_mutex);
    
    return size;
}

// Vacuum the database to reclaim space
int vacuum_database(void) {
    int rc;
    char *err_msg = NULL;
    
    if (!db) {
        log_error("Database not initialized");
        return -1;
    }
    
    pthread_mutex_lock(&db_mutex);
    
    rc = sqlite3_exec(db, "VACUUM;", NULL, NULL, &err_msg);
    if (rc != SQLITE_OK) {
        log_error("Failed to vacuum database: %s", err_msg);
        sqlite3_free(err_msg);
        pthread_mutex_unlock(&db_mutex);
        return -1;
    }
    
    pthread_mutex_unlock(&db_mutex);
    return 0;
}

// Check database integrity
int check_database_integrity(void) {
    int rc;
    sqlite3_stmt *stmt;
    int result = -1;
    
    if (!db) {
        log_error("Database not initialized");
        return -1;
    }
    
    pthread_mutex_lock(&db_mutex);
    
    const char *sql = "PRAGMA integrity_check;";
    
    rc = sqlite3_prepare_v2(db, sql, -1, &stmt, NULL);
    if (rc != SQLITE_OK) {
        log_error("Failed to prepare statement: %s", sqlite3_errmsg(db));
        pthread_mutex_unlock(&db_mutex);
        return -1;
    }
    
    if (sqlite3_step(stmt) == SQLITE_ROW) {
        const char *integrity_result = (const char *)sqlite3_column_text(stmt, 0);
        if (integrity_result && strcmp(integrity_result, "ok") == 0) {
            result = 0; // Database is valid
        } else {
            log_error("Database integrity check failed: %s", integrity_result ? integrity_result : "unknown error");
            result = -1;
        }
    }
    
    sqlite3_finalize(stmt);
    pthread_mutex_unlock(&db_mutex);
    
    return result;
}

/**
 * Add a stream configuration to the database
 * 
 * @param stream Stream configuration to add
 * @return Stream ID on success, 0 on failure
 */
uint64_t add_stream_config(const stream_config_t *stream) {
    int rc;
    sqlite3_stmt *stmt;
    uint64_t stream_id = 0;
    
    if (!db) {
        log_error("Database not initialized");
        return 0;
    }
    
    if (!stream) {
        log_error("Stream configuration is required");
        return 0;
    }
    
    pthread_mutex_lock(&db_mutex);
    
    // First, check if we need to alter the table to add detection columns
    const char *check_column_sql = "PRAGMA table_info(streams);";
    sqlite3_stmt *check_stmt;
    
    rc = sqlite3_prepare_v2(db, check_column_sql, -1, &check_stmt, NULL);
    if (rc != SQLITE_OK) {
        log_error("Failed to prepare statement: %s", sqlite3_errmsg(db));
        pthread_mutex_unlock(&db_mutex);
        return 0;
    }
    
    bool has_detection_columns = false;
    while (sqlite3_step(check_stmt) == SQLITE_ROW) {
        const char *column_name = (const char *)sqlite3_column_text(check_stmt, 1);
        if (column_name && strcmp(column_name, "detection_based_recording") == 0) {
            has_detection_columns = true;
            break;
        }
    }
    
    sqlite3_finalize(check_stmt);
    
    // If detection columns don't exist, add them
    if (!has_detection_columns) {
        log_info("Adding detection columns to streams table");
        
        const char *alter_table_sql[] = {
            "ALTER TABLE streams ADD COLUMN detection_based_recording INTEGER DEFAULT 0;",
            "ALTER TABLE streams ADD COLUMN detection_model TEXT DEFAULT '';",
            "ALTER TABLE streams ADD COLUMN detection_threshold REAL DEFAULT 0.5;",
            "ALTER TABLE streams ADD COLUMN detection_interval INTEGER DEFAULT 10;",
            "ALTER TABLE streams ADD COLUMN pre_detection_buffer INTEGER DEFAULT 5;",
            "ALTER TABLE streams ADD COLUMN post_detection_buffer INTEGER DEFAULT 10;"
        };
        
        for (int i = 0; i < 6; i++) {
            char *err_msg = NULL;
            rc = sqlite3_exec(db, alter_table_sql[i], NULL, NULL, &err_msg);
            if (rc != SQLITE_OK) {
                log_error("Failed to alter table: %s", err_msg);
                sqlite3_free(err_msg);
                // Continue anyway, the column might already exist
            }
        }
    }
    
    // Now insert the stream with all fields including detection settings
    const char *sql = "INSERT INTO streams (name, url, enabled, width, height, fps, codec, priority, record, segment_duration, "
                      "detection_based_recording, detection_model, detection_threshold, detection_interval, "
                      "pre_detection_buffer, post_detection_buffer) "
                      "VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?);";
    
    rc = sqlite3_prepare_v2(db, sql, -1, &stmt, NULL);
    if (rc != SQLITE_OK) {
        log_error("Failed to prepare statement: %s", sqlite3_errmsg(db));
        pthread_mutex_unlock(&db_mutex);
        return 0;
    }
    
    // Bind parameters for basic stream settings
    sqlite3_bind_text(stmt, 1, stream->name, -1, SQLITE_STATIC);
    sqlite3_bind_text(stmt, 2, stream->url, -1, SQLITE_STATIC);
    sqlite3_bind_int(stmt, 3, stream->enabled ? 1 : 0);
    sqlite3_bind_int(stmt, 4, stream->width);
    sqlite3_bind_int(stmt, 5, stream->height);
    sqlite3_bind_int(stmt, 6, stream->fps);
    sqlite3_bind_text(stmt, 7, stream->codec, -1, SQLITE_STATIC);
    sqlite3_bind_int(stmt, 8, stream->priority);
    sqlite3_bind_int(stmt, 9, stream->record ? 1 : 0);
    sqlite3_bind_int(stmt, 10, stream->segment_duration);
    
    // Bind parameters for detection settings
    sqlite3_bind_int(stmt, 11, stream->detection_based_recording ? 1 : 0);
    sqlite3_bind_text(stmt, 12, stream->detection_model, -1, SQLITE_STATIC);
    sqlite3_bind_double(stmt, 13, stream->detection_threshold);
    sqlite3_bind_int(stmt, 14, stream->detection_interval);
    sqlite3_bind_int(stmt, 15, stream->pre_detection_buffer);
    sqlite3_bind_int(stmt, 16, stream->post_detection_buffer);
    
    // Execute statement
    rc = sqlite3_step(stmt);
    if (rc != SQLITE_DONE) {
        log_error("Failed to add stream configuration: %s", sqlite3_errmsg(db));
    } else {
        stream_id = (uint64_t)sqlite3_last_insert_rowid(db);
        log_debug("Added stream configuration with ID %llu", (unsigned long long)stream_id);
        
        // Log the addition
        log_info("Added stream configuration: name=%s, enabled=%s, detection=%s, model=%s", 
                stream->name, 
                stream->enabled ? "true" : "false",
                stream->detection_based_recording ? "true" : "false",
                stream->detection_model);
    }
    
    sqlite3_finalize(stmt);
    pthread_mutex_unlock(&db_mutex);
    
    return stream_id;
}

/**
 * Update a stream configuration in the database
 * 
 * @param name Stream name to update
 * @param stream Updated stream configuration
 * @return 0 on success, non-zero on failure
 */
int update_stream_config(const char *name, const stream_config_t *stream) {
    int rc;
    sqlite3_stmt *stmt;
    
    if (!db) {
        log_error("Database not initialized");
        return -1;
    }
    
    if (!name || !stream) {
        log_error("Stream name and configuration are required");
        return -1;
    }
    
    pthread_mutex_lock(&db_mutex);
    
    // First, check if we need to alter the table to add detection columns
    const char *check_column_sql = "PRAGMA table_info(streams);";
    sqlite3_stmt *check_stmt;
    
    rc = sqlite3_prepare_v2(db, check_column_sql, -1, &check_stmt, NULL);
    if (rc != SQLITE_OK) {
        log_error("Failed to prepare statement: %s", sqlite3_errmsg(db));
        pthread_mutex_unlock(&db_mutex);
        return -1;
    }
    
    bool has_detection_columns = false;
    while (sqlite3_step(check_stmt) == SQLITE_ROW) {
        const char *column_name = (const char *)sqlite3_column_text(check_stmt, 1);
        if (column_name && strcmp(column_name, "detection_based_recording") == 0) {
            has_detection_columns = true;
            break;
        }
    }
    
    sqlite3_finalize(check_stmt);
    
    // If detection columns don't exist, add them
    if (!has_detection_columns) {
        log_info("Adding detection columns to streams table");
        
        const char *alter_table_sql[] = {
            "ALTER TABLE streams ADD COLUMN detection_based_recording INTEGER DEFAULT 0;",
            "ALTER TABLE streams ADD COLUMN detection_model TEXT DEFAULT '';",
            "ALTER TABLE streams ADD COLUMN detection_threshold REAL DEFAULT 0.5;",
            "ALTER TABLE streams ADD COLUMN detection_interval INTEGER DEFAULT 10;",
            "ALTER TABLE streams ADD COLUMN pre_detection_buffer INTEGER DEFAULT 5;",
            "ALTER TABLE streams ADD COLUMN post_detection_buffer INTEGER DEFAULT 10;"
        };
        
        for (int i = 0; i < 6; i++) {
            char *err_msg = NULL;
            rc = sqlite3_exec(db, alter_table_sql[i], NULL, NULL, &err_msg);
            if (rc != SQLITE_OK) {
                log_error("Failed to alter table: %s", err_msg);
                sqlite3_free(err_msg);
                // Continue anyway, the column might already exist
            }
        }
    }
    
    // Now update the stream with all fields including detection settings
    const char *sql = "UPDATE streams SET "
                      "name = ?, url = ?, enabled = ?, width = ?, height = ?, "
                      "fps = ?, codec = ?, priority = ?, record = ?, segment_duration = ?, "
                      "detection_based_recording = ?, detection_model = ?, detection_threshold = ?, "
                      "detection_interval = ?, pre_detection_buffer = ?, post_detection_buffer = ? "
                      "WHERE name = ?;";
    
    rc = sqlite3_prepare_v2(db, sql, -1, &stmt, NULL);
    if (rc != SQLITE_OK) {
        log_error("Failed to prepare statement: %s", sqlite3_errmsg(db));
        pthread_mutex_unlock(&db_mutex);
        return -1;
    }
    
    // Bind parameters for basic stream settings
    sqlite3_bind_text(stmt, 1, stream->name, -1, SQLITE_STATIC);
    sqlite3_bind_text(stmt, 2, stream->url, -1, SQLITE_STATIC);
    sqlite3_bind_int(stmt, 3, stream->enabled ? 1 : 0);
    sqlite3_bind_int(stmt, 4, stream->width);
    sqlite3_bind_int(stmt, 5, stream->height);
    sqlite3_bind_int(stmt, 6, stream->fps);
    sqlite3_bind_text(stmt, 7, stream->codec, -1, SQLITE_STATIC);
    sqlite3_bind_int(stmt, 8, stream->priority);
    sqlite3_bind_int(stmt, 9, stream->record ? 1 : 0);
    sqlite3_bind_int(stmt, 10, stream->segment_duration);
    
    // Bind parameters for detection settings
    sqlite3_bind_int(stmt, 11, stream->detection_based_recording ? 1 : 0);
    sqlite3_bind_text(stmt, 12, stream->detection_model, -1, SQLITE_STATIC);
    sqlite3_bind_double(stmt, 13, stream->detection_threshold);
    sqlite3_bind_int(stmt, 14, stream->detection_interval);
    sqlite3_bind_int(stmt, 15, stream->pre_detection_buffer);
    sqlite3_bind_int(stmt, 16, stream->post_detection_buffer);
    
    // Bind the WHERE clause parameter
    sqlite3_bind_text(stmt, 17, name, -1, SQLITE_STATIC);
    
    // Execute statement
    rc = sqlite3_step(stmt);
    if (rc != SQLITE_DONE) {
        log_error("Failed to update stream configuration: %s", sqlite3_errmsg(db));
        sqlite3_finalize(stmt);
        pthread_mutex_unlock(&db_mutex);
        return -1;
    }
    
    sqlite3_finalize(stmt);
    
    // Log the update
    log_info("Updated stream configuration for %s: enabled=%s, detection=%s, model=%s", 
             stream->name, 
             stream->enabled ? "true" : "false",
             stream->detection_based_recording ? "true" : "false",
             stream->detection_model);
    
    pthread_mutex_unlock(&db_mutex);
    
    return 0;
}

/**
 * Delete a stream configuration from the database
 * 
 * @param name Stream name to delete
 * @return 0 on success, non-zero on failure
 */
int delete_stream_config(const char *name) {
    int rc;
    sqlite3_stmt *stmt;
    
    if (!db) {
        log_error("Database not initialized");
        return -1;
    }
    
    if (!name) {
        log_error("Stream name is required");
        return -1;
    }
    
    pthread_mutex_lock(&db_mutex);
    
    const char *sql = "DELETE FROM streams WHERE name = ?;";
    
    rc = sqlite3_prepare_v2(db, sql, -1, &stmt, NULL);
    if (rc != SQLITE_OK) {
        log_error("Failed to prepare statement: %s", sqlite3_errmsg(db));
        pthread_mutex_unlock(&db_mutex);
        return -1;
    }
    
    // Bind parameters
    sqlite3_bind_text(stmt, 1, name, -1, SQLITE_STATIC);
    
    // Execute statement
    rc = sqlite3_step(stmt);
    if (rc != SQLITE_DONE) {
        log_error("Failed to delete stream configuration: %s", sqlite3_errmsg(db));
        sqlite3_finalize(stmt);
        pthread_mutex_unlock(&db_mutex);
        return -1;
    }
    
    sqlite3_finalize(stmt);
    pthread_mutex_unlock(&db_mutex);
    
    return 0;
}

/**
 * Get a stream configuration from the database
 * 
 * @param name Stream name to get
 * @param stream Stream configuration to fill
 * @return 0 on success, non-zero on failure
 */
int get_stream_config_by_name(const char *name, stream_config_t *stream) {
    int rc;
    sqlite3_stmt *stmt;
    int result = -1;
    
    if (!db) {
        log_error("Database not initialized");
        return -1;
    }
    
    if (!name || !stream) {
        log_error("Stream name and configuration pointer are required");
        return -1;
    }
    
    pthread_mutex_lock(&db_mutex);
    
    // First, check if detection columns exist
    const char *check_column_sql = "PRAGMA table_info(streams);";
    sqlite3_stmt *check_stmt;
    
    rc = sqlite3_prepare_v2(db, check_column_sql, -1, &check_stmt, NULL);
    if (rc != SQLITE_OK) {
        log_error("Failed to prepare statement: %s", sqlite3_errmsg(db));
        pthread_mutex_unlock(&db_mutex);
        return -1;
    }
    
    bool has_detection_columns = false;
    while (sqlite3_step(check_stmt) == SQLITE_ROW) {
        const char *column_name = (const char *)sqlite3_column_text(check_stmt, 1);
        if (column_name && strcmp(column_name, "detection_based_recording") == 0) {
            has_detection_columns = true;
            break;
        }
    }
    
    sqlite3_finalize(check_stmt);
    
    // Prepare SQL based on whether detection columns exist
    const char *sql;
    if (has_detection_columns) {
        sql = "SELECT name, url, enabled, width, height, fps, codec, priority, record, segment_duration, "
              "detection_based_recording, detection_model, detection_threshold, detection_interval, "
              "pre_detection_buffer, post_detection_buffer "
              "FROM streams WHERE name = ?;";
    } else {
        sql = "SELECT name, url, enabled, width, height, fps, codec, priority, record, segment_duration "
              "FROM streams WHERE name = ?;";
    }
    
    rc = sqlite3_prepare_v2(db, sql, -1, &stmt, NULL);
    if (rc != SQLITE_OK) {
        log_error("Failed to prepare statement: %s", sqlite3_errmsg(db));
        pthread_mutex_unlock(&db_mutex);
        return -1;
    }
    
    // Bind parameters
    sqlite3_bind_text(stmt, 1, name, -1, SQLITE_STATIC);
    
    // Execute query and fetch result
    if (sqlite3_step(stmt) == SQLITE_ROW) {
        // Initialize stream with default values
        memset(stream, 0, sizeof(stream_config_t));
        
        // Set default values for detection settings
        stream->detection_threshold = 0.5f;
        stream->detection_interval = 10;
        stream->pre_detection_buffer = 5;
        stream->post_detection_buffer = 10;
        
        // Parse basic stream settings
        const char *stream_name = (const char *)sqlite3_column_text(stmt, 0);
        if (stream_name) {
            strncpy(stream->name, stream_name, MAX_STREAM_NAME - 1);
            stream->name[MAX_STREAM_NAME - 1] = '\0';
        }
        
        const char *url = (const char *)sqlite3_column_text(stmt, 1);
        if (url) {
            strncpy(stream->url, url, MAX_URL_LENGTH - 1);
            stream->url[MAX_URL_LENGTH - 1] = '\0';
        }
        
        stream->enabled = sqlite3_column_int(stmt, 2) != 0;
        stream->width = sqlite3_column_int(stmt, 3);
        stream->height = sqlite3_column_int(stmt, 4);
        stream->fps = sqlite3_column_int(stmt, 5);
        
        const char *codec = (const char *)sqlite3_column_text(stmt, 6);
        if (codec) {
            strncpy(stream->codec, codec, sizeof(stream->codec) - 1);
            stream->codec[sizeof(stream->codec) - 1] = '\0';
        }
        
        stream->priority = sqlite3_column_int(stmt, 7);
        stream->record = sqlite3_column_int(stmt, 8) != 0;
        stream->segment_duration = sqlite3_column_int(stmt, 9);
        
        // Parse detection settings if columns exist
        if (has_detection_columns && sqlite3_column_count(stmt) > 10) {
            stream->detection_based_recording = sqlite3_column_int(stmt, 10) != 0;
            
            const char *detection_model = (const char *)sqlite3_column_text(stmt, 11);
            if (detection_model) {
                strncpy(stream->detection_model, detection_model, MAX_PATH_LENGTH - 1);
                stream->detection_model[MAX_PATH_LENGTH - 1] = '\0';
            }
            
            if (sqlite3_column_type(stmt, 12) != SQLITE_NULL) {
                stream->detection_threshold = (float)sqlite3_column_double(stmt, 12);
            }
            
            if (sqlite3_column_type(stmt, 13) != SQLITE_NULL) {
                stream->detection_interval = sqlite3_column_int(stmt, 13);
            }
            
            if (sqlite3_column_type(stmt, 14) != SQLITE_NULL) {
                stream->pre_detection_buffer = sqlite3_column_int(stmt, 14);
            }
            
            if (sqlite3_column_type(stmt, 15) != SQLITE_NULL) {
                stream->post_detection_buffer = sqlite3_column_int(stmt, 15);
            }
        }
        
        result = 0; // Success
    }
    
    sqlite3_finalize(stmt);
    pthread_mutex_unlock(&db_mutex);
    
    return result;
}

/**
 * Get all stream configurations from the database
 * 
 * @param streams Array to fill with stream configurations
 * @param max_count Maximum number of streams to return
 * @return Number of streams found, or -1 on error
 */
int get_all_stream_configs(stream_config_t *streams, int max_count) {
    int rc;
    sqlite3_stmt *stmt;
    int count = 0;
    
    if (!db) {
        log_error("Database not initialized");
        return -1;
    }
    
    if (!streams || max_count <= 0) {
        log_error("Invalid parameters for get_all_stream_configs");
        return -1;
    }
    
    pthread_mutex_lock(&db_mutex);
    
    // First, check if detection columns exist
    const char *check_column_sql = "PRAGMA table_info(streams);";
    sqlite3_stmt *check_stmt;
    
    rc = sqlite3_prepare_v2(db, check_column_sql, -1, &check_stmt, NULL);
    if (rc != SQLITE_OK) {
        log_error("Failed to prepare statement: %s", sqlite3_errmsg(db));
        pthread_mutex_unlock(&db_mutex);
        return -1;
    }
    
    bool has_detection_columns = false;
    while (sqlite3_step(check_stmt) == SQLITE_ROW) {
        const char *column_name = (const char *)sqlite3_column_text(check_stmt, 1);
        if (column_name && strcmp(column_name, "detection_based_recording") == 0) {
            has_detection_columns = true;
            break;
        }
    }
    
    sqlite3_finalize(check_stmt);
    
    // Prepare SQL based on whether detection columns exist
    const char *sql;
    if (has_detection_columns) {
        sql = "SELECT name, url, enabled, width, height, fps, codec, priority, record, segment_duration, "
              "detection_based_recording, detection_model, detection_threshold, detection_interval, "
              "pre_detection_buffer, post_detection_buffer "
              "FROM streams ORDER BY name;";
    } else {
        sql = "SELECT name, url, enabled, width, height, fps, codec, priority, record, segment_duration "
              "FROM streams ORDER BY name;";
    }
    
    rc = sqlite3_prepare_v2(db, sql, -1, &stmt, NULL);
    if (rc != SQLITE_OK) {
        log_error("Failed to prepare statement: %s", sqlite3_errmsg(db));
        pthread_mutex_unlock(&db_mutex);
        return -1;
    }
    
    // Execute query and fetch results
    while (sqlite3_step(stmt) == SQLITE_ROW && count < max_count) {
        // Initialize stream with default values
        memset(&streams[count], 0, sizeof(stream_config_t));
        
        // Set default values for detection settings
        streams[count].detection_threshold = 0.5f;
        streams[count].detection_interval = 10;
        streams[count].pre_detection_buffer = 5;
        streams[count].post_detection_buffer = 10;
        
        // Parse basic stream settings
        const char *name = (const char *)sqlite3_column_text(stmt, 0);
        if (name) {
            strncpy(streams[count].name, name, MAX_STREAM_NAME - 1);
            streams[count].name[MAX_STREAM_NAME - 1] = '\0';
        }
        
        const char *url = (const char *)sqlite3_column_text(stmt, 1);
        if (url) {
            strncpy(streams[count].url, url, MAX_URL_LENGTH - 1);
            streams[count].url[MAX_URL_LENGTH - 1] = '\0';
        }
        
        streams[count].enabled = sqlite3_column_int(stmt, 2) != 0;
        streams[count].width = sqlite3_column_int(stmt, 3);
        streams[count].height = sqlite3_column_int(stmt, 4);
        streams[count].fps = sqlite3_column_int(stmt, 5);
        
        const char *codec = (const char *)sqlite3_column_text(stmt, 6);
        if (codec) {
            strncpy(streams[count].codec, codec, sizeof(streams[count].codec) - 1);
            streams[count].codec[sizeof(streams[count].codec) - 1] = '\0';
        }
        
        streams[count].priority = sqlite3_column_int(stmt, 7);
        streams[count].record = sqlite3_column_int(stmt, 8) != 0;
        streams[count].segment_duration = sqlite3_column_int(stmt, 9);
        
        // Parse detection settings if columns exist
        if (has_detection_columns && sqlite3_column_count(stmt) > 10) {
            streams[count].detection_based_recording = sqlite3_column_int(stmt, 10) != 0;
            
            const char *detection_model = (const char *)sqlite3_column_text(stmt, 11);
            if (detection_model) {
                strncpy(streams[count].detection_model, detection_model, MAX_PATH_LENGTH - 1);
                streams[count].detection_model[MAX_PATH_LENGTH - 1] = '\0';
            }
            
            if (sqlite3_column_type(stmt, 12) != SQLITE_NULL) {
                streams[count].detection_threshold = (float)sqlite3_column_double(stmt, 12);
            }
            
            if (sqlite3_column_type(stmt, 13) != SQLITE_NULL) {
                streams[count].detection_interval = sqlite3_column_int(stmt, 13);
            }
            
            if (sqlite3_column_type(stmt, 14) != SQLITE_NULL) {
                streams[count].pre_detection_buffer = sqlite3_column_int(stmt, 14);
            }
            
            if (sqlite3_column_type(stmt, 15) != SQLITE_NULL) {
                streams[count].post_detection_buffer = sqlite3_column_int(stmt, 15);
            }
        }
        
        count++;
    }
    
    sqlite3_finalize(stmt);
    pthread_mutex_unlock(&db_mutex);
    
    return count;
}

/**
 * Count the number of stream configurations in the database
 * 
 * @return Number of streams, or -1 on error
 */
int count_stream_configs(void) {
    int rc;
    sqlite3_stmt *stmt;
    int count = -1;
    
    if (!db) {
        log_error("Database not initialized");
        return -1;
    }
    
    pthread_mutex_lock(&db_mutex);
    
    const char *sql = "SELECT COUNT(*) FROM streams;";
    
    rc = sqlite3_prepare_v2(db, sql, -1, &stmt, NULL);
    if (rc != SQLITE_OK) {
        log_error("Failed to prepare statement: %s", sqlite3_errmsg(db));
        pthread_mutex_unlock(&db_mutex);
        return -1;
    }
    
    if (sqlite3_step(stmt) == SQLITE_ROW) {
        count = sqlite3_column_int(stmt, 0);
    }
    
    sqlite3_finalize(stmt);
    pthread_mutex_unlock(&db_mutex);
    
    return count;
}

/**
 * Execute a SQL query and get the results
 * 
 * @param sql SQL query to execute
 * @param result Pointer to store the result set
 * @param rows Pointer to store the number of rows
 * @param cols Pointer to store the number of columns
 * @return 0 on success, non-zero on failure
 */
int database_execute_query(const char *sql, void **result, int *rows, int *cols) {
    int rc;
    char *err_msg = NULL;
    
    if (!db) {
        log_error("Database not initialized");
        return -1;
    }
    
    if (!sql || !result || !rows || !cols) {
        log_error("Invalid parameters for database_execute_query");
        return -1;
    }
    
    pthread_mutex_lock(&db_mutex);
    
    // Execute the query and get results as a table
    rc = sqlite3_get_table(db, sql, (char ***)result, rows, cols, &err_msg);
    
    if (rc != SQLITE_OK) {
        log_error("SQL error: %s", err_msg);
        sqlite3_free(err_msg);
        pthread_mutex_unlock(&db_mutex);
        return -1;
    }
    
    pthread_mutex_unlock(&db_mutex);
    return 0;
}
