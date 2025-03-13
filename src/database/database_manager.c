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
        "CREATE INDEX IF NOT EXISTS idx_streams_name ON streams (name);";
    
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
    
    const char *sql = "INSERT INTO streams (name, url, enabled, width, height, fps, codec, priority, record, segment_duration) "
                      "VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?);";
    
    rc = sqlite3_prepare_v2(db, sql, -1, &stmt, NULL);
    if (rc != SQLITE_OK) {
        log_error("Failed to prepare statement: %s", sqlite3_errmsg(db));
        pthread_mutex_unlock(&db_mutex);
        return 0;
    }
    
    // Bind parameters
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
    
    // Execute statement
    rc = sqlite3_step(stmt);
    if (rc != SQLITE_DONE) {
        log_error("Failed to add stream configuration: %s", sqlite3_errmsg(db));
    } else {
        stream_id = (uint64_t)sqlite3_last_insert_rowid(db);
        log_debug("Added stream configuration with ID %llu", (unsigned long long)stream_id);
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
    
    const char *sql = "UPDATE streams SET "
                      "name = ?, url = ?, enabled = ?, width = ?, height = ?, "
                      "fps = ?, codec = ?, priority = ?, record = ?, segment_duration = ? "
                      "WHERE name = ?;";
    
    rc = sqlite3_prepare_v2(db, sql, -1, &stmt, NULL);
    if (rc != SQLITE_OK) {
        log_error("Failed to prepare statement: %s", sqlite3_errmsg(db));
        pthread_mutex_unlock(&db_mutex);
        return -1;
    }
    
    // Bind parameters
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
    sqlite3_bind_text(stmt, 11, name, -1, SQLITE_STATIC);
    
    // Execute statement
    rc = sqlite3_step(stmt);
    if (rc != SQLITE_DONE) {
        log_error("Failed to update stream configuration: %s", sqlite3_errmsg(db));
        sqlite3_finalize(stmt);
        pthread_mutex_unlock(&db_mutex);
        return -1;
    }
    
    sqlite3_finalize(stmt);
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
    
    const char *sql = "SELECT name, url, enabled, width, height, fps, codec, priority, record, segment_duration "
                      "FROM streams WHERE name = ?;";
    
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
        const char *stream_name = (const char *)sqlite3_column_text(stmt, 0);
        if (stream_name) {
            strncpy(stream->name, stream_name, MAX_STREAM_NAME - 1);
            stream->name[MAX_STREAM_NAME - 1] = '\0';
        } else {
            stream->name[0] = '\0';
        }
        
        const char *url = (const char *)sqlite3_column_text(stmt, 1);
        if (url) {
            strncpy(stream->url, url, MAX_URL_LENGTH - 1);
            stream->url[MAX_URL_LENGTH - 1] = '\0';
        } else {
            stream->url[0] = '\0';
        }
        
        stream->enabled = sqlite3_column_int(stmt, 2) != 0;
        stream->width = sqlite3_column_int(stmt, 3);
        stream->height = sqlite3_column_int(stmt, 4);
        stream->fps = sqlite3_column_int(stmt, 5);
        
        const char *codec = (const char *)sqlite3_column_text(stmt, 6);
        if (codec) {
            strncpy(stream->codec, codec, sizeof(stream->codec) - 1);
            stream->codec[sizeof(stream->codec) - 1] = '\0';
        } else {
            stream->codec[0] = '\0';
        }
        
        stream->priority = sqlite3_column_int(stmt, 7);
        stream->record = sqlite3_column_int(stmt, 8) != 0;
        stream->segment_duration = sqlite3_column_int(stmt, 9);
        
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
    
    const char *sql = "SELECT name, url, enabled, width, height, fps, codec, priority, record, segment_duration "
                      "FROM streams ORDER BY name;";
    
    rc = sqlite3_prepare_v2(db, sql, -1, &stmt, NULL);
    if (rc != SQLITE_OK) {
        log_error("Failed to prepare statement: %s", sqlite3_errmsg(db));
        pthread_mutex_unlock(&db_mutex);
        return -1;
    }
    
    // Execute query and fetch results
    while (sqlite3_step(stmt) == SQLITE_ROW && count < max_count) {
        const char *name = (const char *)sqlite3_column_text(stmt, 0);
        if (name) {
            strncpy(streams[count].name, name, MAX_STREAM_NAME - 1);
            streams[count].name[MAX_STREAM_NAME - 1] = '\0';
        } else {
            streams[count].name[0] = '\0';
        }
        
        const char *url = (const char *)sqlite3_column_text(stmt, 1);
        if (url) {
            strncpy(streams[count].url, url, MAX_URL_LENGTH - 1);
            streams[count].url[MAX_URL_LENGTH - 1] = '\0';
        } else {
            streams[count].url[0] = '\0';
        }
        
        streams[count].enabled = sqlite3_column_int(stmt, 2) != 0;
        streams[count].width = sqlite3_column_int(stmt, 3);
        streams[count].height = sqlite3_column_int(stmt, 4);
        streams[count].fps = sqlite3_column_int(stmt, 5);
        
        const char *codec = (const char *)sqlite3_column_text(stmt, 6);
        if (codec) {
            strncpy(streams[count].codec, codec, sizeof(streams[count].codec) - 1);
            streams[count].codec[sizeof(streams[count].codec) - 1] = '\0';
        } else {
            streams[count].codec[0] = '\0';
        }
        
        streams[count].priority = sqlite3_column_int(stmt, 7);
        streams[count].record = sqlite3_column_int(stmt, 8) != 0;
        streams[count].segment_duration = sqlite3_column_int(stmt, 9);
        
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
