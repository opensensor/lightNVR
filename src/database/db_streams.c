#define _POSIX_C_SOURCE 200809L
#define _XOPEN_SOURCE 700
#define _GNU_SOURCE

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>
#include <sqlite3.h>
#include <stdbool.h>

#include "database/db_streams.h"
#include "database/db_core.h"
#include "core/logger.h"
#include "core/config.h"

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
    
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *db_mutex = get_db_mutex();
    
    if (!db) {
        log_error("Database not initialized");
        return 0;
    }
    
    if (!stream) {
        log_error("Stream configuration is required");
        return 0;
    }
    
    pthread_mutex_lock(db_mutex);
    
    // First, check if we need to alter the table to add detection columns
    const char *check_column_sql = "PRAGMA table_info(streams);";
    sqlite3_stmt *check_stmt;
    
    rc = sqlite3_prepare_v2(db, check_column_sql, -1, &check_stmt, NULL);
    if (rc != SQLITE_OK) {
        log_error("Failed to prepare statement: %s", sqlite3_errmsg(db));
        pthread_mutex_unlock(db_mutex);
        return 0;
    }
    
    bool has_detection_columns = false;
    while (sqlite3_step(check_stmt) == SQLITE_ROW) {
        const char *column_name = (const char *)sqlite3_column_text(check_stmt, 1);
        if (column_name && strcmp(column_name, "detection_based_recording") == 0) {
            has_detection_columns = true;
            log_info("detection_based_recording column exists in streams table");
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
    const char *sql = "INSERT INTO streams (name, url, enabled, streaming_enabled, width, height, fps, codec, priority, record, segment_duration, "
                      "detection_based_recording, detection_model, detection_threshold, detection_interval, "
                      "pre_detection_buffer, post_detection_buffer) "
                      "VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?);";
    
    rc = sqlite3_prepare_v2(db, sql, -1, &stmt, NULL);
    if (rc != SQLITE_OK) {
        log_error("Failed to prepare statement: %s", sqlite3_errmsg(db));
        pthread_mutex_unlock(db_mutex);
        return 0;
    }
    
    // Bind parameters for basic stream settings
    sqlite3_bind_text(stmt, 1, stream->name, -1, SQLITE_STATIC);
    sqlite3_bind_text(stmt, 2, stream->url, -1, SQLITE_STATIC);
    sqlite3_bind_int(stmt, 3, stream->enabled ? 1 : 0);
    sqlite3_bind_int(stmt, 4, stream->streaming_enabled ? 1 : 0);
    sqlite3_bind_int(stmt, 5, stream->width);
    sqlite3_bind_int(stmt, 6, stream->height);
    sqlite3_bind_int(stmt, 7, stream->fps);
    sqlite3_bind_text(stmt, 8, stream->codec, -1, SQLITE_STATIC);
    sqlite3_bind_int(stmt, 9, stream->priority);
    sqlite3_bind_int(stmt, 10, stream->record ? 1 : 0);
    sqlite3_bind_int(stmt, 11, stream->segment_duration);
    
    // Bind parameters for detection settings
    sqlite3_bind_int(stmt, 12, stream->detection_based_recording ? 1 : 0);
    sqlite3_bind_text(stmt, 13, stream->detection_model, -1, SQLITE_STATIC);
    sqlite3_bind_double(stmt, 14, stream->detection_threshold);
    sqlite3_bind_int(stmt, 15, stream->detection_interval);
    sqlite3_bind_int(stmt, 16, stream->pre_detection_buffer);
    sqlite3_bind_int(stmt, 17, stream->post_detection_buffer);
    
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
    pthread_mutex_unlock(db_mutex);
    
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
    
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *db_mutex = get_db_mutex();
    
    if (!db) {
        log_error("Database not initialized");
        return -1;
    }
    
    if (!name || !stream) {
        log_error("Stream name and configuration are required");
        return -1;
    }
    
    pthread_mutex_lock(db_mutex);
    
    // First, check if we need to alter the table to add detection columns
    const char *check_column_sql = "PRAGMA table_info(streams);";
    sqlite3_stmt *check_stmt;
    
    rc = sqlite3_prepare_v2(db, check_column_sql, -1, &check_stmt, NULL);
    if (rc != SQLITE_OK) {
        log_error("Failed to prepare statement: %s", sqlite3_errmsg(db));
        pthread_mutex_unlock(db_mutex);
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
                      "name = ?, url = ?, enabled = ?, streaming_enabled = ?, width = ?, height = ?, "
                      "fps = ?, codec = ?, priority = ?, record = ?, segment_duration = ?, "
                      "detection_based_recording = ?, detection_model = ?, detection_threshold = ?, "
                      "detection_interval = ?, pre_detection_buffer = ?, post_detection_buffer = ? "
                      "WHERE name = ?;";
    
    rc = sqlite3_prepare_v2(db, sql, -1, &stmt, NULL);
    if (rc != SQLITE_OK) {
        log_error("Failed to prepare statement: %s", sqlite3_errmsg(db));
        pthread_mutex_unlock(db_mutex);
        return -1;
    }
    
    // Bind parameters for basic stream settings
    sqlite3_bind_text(stmt, 1, stream->name, -1, SQLITE_STATIC);
    sqlite3_bind_text(stmt, 2, stream->url, -1, SQLITE_STATIC);
    sqlite3_bind_int(stmt, 3, stream->enabled ? 1 : 0);
    sqlite3_bind_int(stmt, 4, stream->streaming_enabled ? 1 : 0);
    sqlite3_bind_int(stmt, 5, stream->width);
    sqlite3_bind_int(stmt, 6, stream->height);
    sqlite3_bind_int(stmt, 7, stream->fps);
    sqlite3_bind_text(stmt, 8, stream->codec, -1, SQLITE_STATIC);
    sqlite3_bind_int(stmt, 9, stream->priority);
    sqlite3_bind_int(stmt, 10, stream->record ? 1 : 0);
    sqlite3_bind_int(stmt, 11, stream->segment_duration);
    
    // Bind parameters for detection settings
    sqlite3_bind_int(stmt, 12, stream->detection_based_recording ? 1 : 0);
    sqlite3_bind_text(stmt, 13, stream->detection_model, -1, SQLITE_STATIC);
    sqlite3_bind_double(stmt, 14, stream->detection_threshold);
    sqlite3_bind_int(stmt, 15, stream->detection_interval);
    sqlite3_bind_int(stmt, 16, stream->pre_detection_buffer);
    sqlite3_bind_int(stmt, 17, stream->post_detection_buffer);
    
    // Bind the WHERE clause parameter
    sqlite3_bind_text(stmt, 18, name, -1, SQLITE_STATIC);
    
    // Execute statement
    rc = sqlite3_step(stmt);
    if (rc != SQLITE_DONE) {
        log_error("Failed to update stream configuration: %s", sqlite3_errmsg(db));
        sqlite3_finalize(stmt);
        pthread_mutex_unlock(db_mutex);
        return -1;
    }
    
    sqlite3_finalize(stmt);
    
    // Log the update
    log_info("Updated stream configuration for %s: enabled=%s, detection=%s, model=%s", 
             stream->name, 
             stream->enabled ? "true" : "false",
             stream->detection_based_recording ? "true" : "false",
             stream->detection_model);
    
    pthread_mutex_unlock(db_mutex);
    
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
    
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *db_mutex = get_db_mutex();
    
    if (!db) {
        log_error("Database not initialized");
        return -1;
    }
    
    if (!name) {
        log_error("Stream name is required");
        return -1;
    }
    
    pthread_mutex_lock(db_mutex);
    
    const char *sql = "DELETE FROM streams WHERE name = ?;";
    
    rc = sqlite3_prepare_v2(db, sql, -1, &stmt, NULL);
    if (rc != SQLITE_OK) {
        log_error("Failed to prepare statement: %s", sqlite3_errmsg(db));
        pthread_mutex_unlock(db_mutex);
        return -1;
    }
    
    // Bind parameters
    sqlite3_bind_text(stmt, 1, name, -1, SQLITE_STATIC);
    
    // Execute statement
    rc = sqlite3_step(stmt);
    if (rc != SQLITE_DONE) {
        log_error("Failed to delete stream configuration: %s", sqlite3_errmsg(db));
        sqlite3_finalize(stmt);
        pthread_mutex_unlock(db_mutex);
        return -1;
    }
    
    sqlite3_finalize(stmt);
    pthread_mutex_unlock(db_mutex);
    
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
    
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *db_mutex = get_db_mutex();
    
    if (!db) {
        log_error("Database not initialized");
        return -1;
    }
    
    if (!name || !stream) {
        log_error("Stream name and configuration pointer are required");
        return -1;
    }
    
    pthread_mutex_lock(db_mutex);
    
    // First, check if detection columns exist
    const char *check_column_sql = "PRAGMA table_info(streams);";
    sqlite3_stmt *check_stmt;
    
    rc = sqlite3_prepare_v2(db, check_column_sql, -1, &check_stmt, NULL);
    if (rc != SQLITE_OK) {
        log_error("Failed to prepare statement: %s", sqlite3_errmsg(db));
        pthread_mutex_unlock(db_mutex);
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
        sql = "SELECT name, url, enabled, streaming_enabled, width, height, fps, codec, priority, record, segment_duration, "
              "detection_based_recording, detection_model, detection_threshold, detection_interval, "
              "pre_detection_buffer, post_detection_buffer "
              "FROM streams WHERE name = ?;";
    } else {
        sql = "SELECT name, url, enabled, streaming_enabled, width, height, fps, codec, priority, record, segment_duration "
              "FROM streams WHERE name = ?;";
    }
    
    rc = sqlite3_prepare_v2(db, sql, -1, &stmt, NULL);
    if (rc != SQLITE_OK) {
        log_error("Failed to prepare statement: %s", sqlite3_errmsg(db));
        pthread_mutex_unlock(db_mutex);
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
        stream->streaming_enabled = sqlite3_column_int(stmt, 3) != 0;
        stream->width = sqlite3_column_int(stmt, 4);
        stream->height = sqlite3_column_int(stmt, 5);
        stream->fps = sqlite3_column_int(stmt, 6);
        
        const char *codec = (const char *)sqlite3_column_text(stmt, 7);
        if (codec) {
            strncpy(stream->codec, codec, sizeof(stream->codec) - 1);
            stream->codec[sizeof(stream->codec) - 1] = '\0';
        }
        
        stream->priority = sqlite3_column_int(stmt, 8);
        stream->record = sqlite3_column_int(stmt, 9) != 0;
        stream->segment_duration = sqlite3_column_int(stmt, 10);
        
        // Parse detection settings if columns exist
        if (has_detection_columns && sqlite3_column_count(stmt) > 11) {
            stream->detection_based_recording = sqlite3_column_int(stmt, 11) != 0;
            
            const char *detection_model = (const char *)sqlite3_column_text(stmt, 12);
            if (detection_model) {
                strncpy(stream->detection_model, detection_model, MAX_PATH_LENGTH - 1);
                stream->detection_model[MAX_PATH_LENGTH - 1] = '\0';
            }
            
            if (sqlite3_column_type(stmt, 13) != SQLITE_NULL) {
                stream->detection_threshold = (float)sqlite3_column_double(stmt, 13);
            }
            
            if (sqlite3_column_type(stmt, 14) != SQLITE_NULL) {
                stream->detection_interval = sqlite3_column_int(stmt, 14);
            }
            
            if (sqlite3_column_type(stmt, 15) != SQLITE_NULL) {
                stream->pre_detection_buffer = sqlite3_column_int(stmt, 15);
            }
            
            if (sqlite3_column_type(stmt, 16) != SQLITE_NULL) {
                stream->post_detection_buffer = sqlite3_column_int(stmt, 16);
            }
        }
        
        result = 0; // Success
    }
    
    sqlite3_finalize(stmt);
    pthread_mutex_unlock(db_mutex);
    
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
    
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *db_mutex = get_db_mutex();
    
    if (!db) {
        log_error("Database not initialized");
        return -1;
    }
    
    if (!streams || max_count <= 0) {
        log_error("Invalid parameters for get_all_stream_configs");
        return -1;
    }
    
    pthread_mutex_lock(db_mutex);
    
    // First, check if detection columns exist
    const char *check_column_sql = "PRAGMA table_info(streams);";
    sqlite3_stmt *check_stmt;
    
    rc = sqlite3_prepare_v2(db, check_column_sql, -1, &check_stmt, NULL);
    if (rc != SQLITE_OK) {
        log_error("Failed to prepare statement: %s", sqlite3_errmsg(db));
        pthread_mutex_unlock(db_mutex);
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
        sql = "SELECT name, url, enabled, streaming_enabled, width, height, fps, codec, priority, record, segment_duration, "
              "detection_based_recording, detection_model, detection_threshold, detection_interval, "
              "pre_detection_buffer, post_detection_buffer "
              "FROM streams ORDER BY name;";
    } else {
        sql = "SELECT name, url, enabled, streaming_enabled, width, height, fps, codec, priority, record, segment_duration "
              "FROM streams ORDER BY name;";
    }
    
    rc = sqlite3_prepare_v2(db, sql, -1, &stmt, NULL);
    if (rc != SQLITE_OK) {
        log_error("Failed to prepare statement: %s", sqlite3_errmsg(db));
        pthread_mutex_unlock(db_mutex);
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
        streams[count].streaming_enabled = sqlite3_column_int(stmt, 3) != 0;
        streams[count].width = sqlite3_column_int(stmt, 4);
        streams[count].height = sqlite3_column_int(stmt, 5);
        streams[count].fps = sqlite3_column_int(stmt, 6);
        
        const char *codec = (const char *)sqlite3_column_text(stmt, 7);
        if (codec) {
            strncpy(streams[count].codec, codec, sizeof(streams[count].codec) - 1);
            streams[count].codec[sizeof(streams[count].codec) - 1] = '\0';
        }
        
        streams[count].priority = sqlite3_column_int(stmt, 8);
        streams[count].record = sqlite3_column_int(stmt, 9) != 0;
        streams[count].segment_duration = sqlite3_column_int(stmt, 10);
        
        // Parse detection settings if columns exist
        if (has_detection_columns && sqlite3_column_count(stmt) > 11) {
            streams[count].detection_based_recording = sqlite3_column_int(stmt, 11) != 0;
            
            const char *detection_model = (const char *)sqlite3_column_text(stmt, 12);
            if (detection_model) {
                strncpy(streams[count].detection_model, detection_model, MAX_PATH_LENGTH - 1);
                streams[count].detection_model[MAX_PATH_LENGTH - 1] = '\0';
            }
            
            if (sqlite3_column_type(stmt, 13) != SQLITE_NULL) {
                streams[count].detection_threshold = (float)sqlite3_column_double(stmt, 13);
            }
            
            if (sqlite3_column_type(stmt, 14) != SQLITE_NULL) {
                streams[count].detection_interval = sqlite3_column_int(stmt, 14);
            }
            
            if (sqlite3_column_type(stmt, 15) != SQLITE_NULL) {
                streams[count].pre_detection_buffer = sqlite3_column_int(stmt, 15);
            }
            
            if (sqlite3_column_type(stmt, 16) != SQLITE_NULL) {
                streams[count].post_detection_buffer = sqlite3_column_int(stmt, 16);
            }
        }
        
        count++;
    }
    
    sqlite3_finalize(stmt);
    pthread_mutex_unlock(db_mutex);
    
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
    
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *db_mutex = get_db_mutex();
    
    if (!db) {
        log_error("Database not initialized");
        return -1;
    }
    
    pthread_mutex_lock(db_mutex);
    
    const char *sql = "SELECT COUNT(*) FROM streams;";
    
    rc = sqlite3_prepare_v2(db, sql, -1, &stmt, NULL);
    if (rc != SQLITE_OK) {
        log_error("Failed to prepare statement: %s", sqlite3_errmsg(db));
        pthread_mutex_unlock(db_mutex);
        return -1;
    }
    
    if (sqlite3_step(stmt) == SQLITE_ROW) {
        count = sqlite3_column_int(stmt, 0);
    }
    
    sqlite3_finalize(stmt);
    pthread_mutex_unlock(db_mutex);
    
    return count;
}
