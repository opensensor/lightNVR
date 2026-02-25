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
#include "database/db_schema.h"
#include "database/db_schema_cache.h"
#include "core/logger.h"
#include "core/config.h"

/**
 * Serialize recording_schedule uint8_t[168] to comma-separated text string.
 * Output: "1,0,1,1,..." (168 values)
 * Buffer must be at least RECORDING_SCHEDULE_TEXT_SIZE bytes.
 */
static void serialize_recording_schedule(const uint8_t *schedule, char *buf, size_t buf_size) {
    if (!schedule || !buf || buf_size < 4) {
        if (buf && buf_size > 0) buf[0] = '\0';
        return;
    }
    int pos = 0;
    for (int i = 0; i < 168; i++) {
        int written = snprintf(buf + pos, buf_size - pos, "%s%d",
                               (i > 0) ? "," : "", schedule[i] ? 1 : 0);
        if (written < 0 || (size_t)(pos + written) >= buf_size) break;
        pos += written;
    }
}

/**
 * Deserialize recording_schedule from comma-separated text string to uint8_t[168].
 * If the text is NULL/empty/invalid, all hours default to 1 (always record).
 */
static void deserialize_recording_schedule(const char *text, uint8_t *schedule) {
    if (!schedule) return;
    if (!text || text[0] == '\0') {
        // Default: always record
        memset(schedule, 1, 168);
        return;
    }
    char buf[RECORDING_SCHEDULE_TEXT_SIZE];
    strncpy(buf, text, sizeof(buf) - 1);
    buf[sizeof(buf) - 1] = '\0';

    int idx = 0;
    char *saveptr = NULL;
    char *token = strtok_r(buf, ",", &saveptr);
    while (token && idx < 168) {
        schedule[idx++] = (uint8_t)(atoi(token) ? 1 : 0);
        token = strtok_r(NULL, ",", &saveptr);
    }
    // Fill remaining slots with 1 if string was short
    while (idx < 168) {
        schedule[idx++] = 1;
    }
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

    // Check if a stream with this name already exists but is disabled
    const char *check_sql = "SELECT id FROM streams WHERE name = ? AND enabled = 0;";
    sqlite3_stmt *check_stmt;

    rc = sqlite3_prepare_v2(db, check_sql, -1, &check_stmt, NULL);
    if (rc != SQLITE_OK) {
        log_error("Failed to prepare statement to check for disabled stream: %s", sqlite3_errmsg(db));
        pthread_mutex_unlock(db_mutex);
        return 0;
    }

    sqlite3_bind_text(check_stmt, 1, stream->name, -1, SQLITE_STATIC);

    if (sqlite3_step(check_stmt) == SQLITE_ROW) {
        // Stream exists but is disabled, enable it by updating
        uint64_t existing_id = (uint64_t)sqlite3_column_int64(check_stmt, 0);

        // Finalize the prepared statement
        if (check_stmt) {
            sqlite3_finalize(check_stmt);
            check_stmt = NULL;
        }

        const char *update_sql = "UPDATE streams SET "
                                "url = ?, enabled = ?, streaming_enabled = ?, width = ?, height = ?, "
                                "fps = ?, codec = ?, priority = ?, record = ?, segment_duration = ?, "
                                "detection_based_recording = ?, detection_model = ?, detection_threshold = ?, "
                                "detection_interval = ?, pre_detection_buffer = ?, post_detection_buffer = ?, "
                                "detection_api_url = ?, detection_object_filter = ?, detection_object_filter_list = ?, "
                                "protocol = ?, is_onvif = ?, record_audio = ?, "
                                "backchannel_enabled = ?, retention_days = ?, detection_retention_days = ?, max_storage_mb = ?, "
                                "tier_critical_multiplier = ?, tier_important_multiplier = ?, tier_ephemeral_multiplier = ?, storage_priority = ?, "
                                "ptz_enabled = ?, ptz_max_x = ?, ptz_max_y = ?, ptz_max_z = ?, ptz_has_home = ?, "
                                "onvif_username = ?, onvif_password = ?, onvif_profile = ?, "
                                "record_on_schedule = ?, recording_schedule = ?, tags = ? "
                                "WHERE id = ?;";

        rc = sqlite3_prepare_v2(db, update_sql, -1, &stmt, NULL);
        if (rc != SQLITE_OK) {
            log_error("Failed to prepare statement to update disabled stream: %s", sqlite3_errmsg(db));
            pthread_mutex_unlock(db_mutex);
            return 0;
        }

        // Bind parameters for basic stream settings
        sqlite3_bind_text(stmt, 1, stream->url, -1, SQLITE_STATIC);
        sqlite3_bind_int(stmt, 2, stream->enabled ? 1 : 0);
        sqlite3_bind_int(stmt, 3, stream->streaming_enabled ? 1 : 0);
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
        sqlite3_bind_text(stmt, 17, stream->detection_api_url, -1, SQLITE_STATIC);

        // Bind detection object filter parameters
        sqlite3_bind_text(stmt, 18, stream->detection_object_filter, -1, SQLITE_STATIC);
        sqlite3_bind_text(stmt, 19, stream->detection_object_filter_list, -1, SQLITE_STATIC);

        // Bind protocol parameter
        sqlite3_bind_int(stmt, 20, (int)stream->protocol);

        // Bind is_onvif parameter
        sqlite3_bind_int(stmt, 21, stream->is_onvif ? 1 : 0);

        // Bind record_audio parameter
        sqlite3_bind_int(stmt, 22, stream->record_audio ? 1 : 0);

        // Bind backchannel_enabled parameter
        sqlite3_bind_int(stmt, 23, stream->backchannel_enabled ? 1 : 0);

        // Bind retention policy parameters
        sqlite3_bind_int(stmt, 24, stream->retention_days);
        sqlite3_bind_int(stmt, 25, stream->detection_retention_days);
        sqlite3_bind_int(stmt, 26, stream->max_storage_mb);

        // Bind tier multiplier parameters
        sqlite3_bind_double(stmt, 27, stream->tier_critical_multiplier);
        sqlite3_bind_double(stmt, 28, stream->tier_important_multiplier);
        sqlite3_bind_double(stmt, 29, stream->tier_ephemeral_multiplier);
        sqlite3_bind_int(stmt, 30, stream->storage_priority);

        // Bind PTZ parameters
        sqlite3_bind_int(stmt, 31, stream->ptz_enabled ? 1 : 0);
        sqlite3_bind_int(stmt, 32, stream->ptz_max_x);
        sqlite3_bind_int(stmt, 33, stream->ptz_max_y);
        sqlite3_bind_int(stmt, 34, stream->ptz_max_z);
        sqlite3_bind_int(stmt, 35, stream->ptz_has_home ? 1 : 0);

        // Bind ONVIF credentials
        sqlite3_bind_text(stmt, 36, stream->onvif_username, -1, SQLITE_STATIC);
        sqlite3_bind_text(stmt, 37, stream->onvif_password, -1, SQLITE_STATIC);
        sqlite3_bind_text(stmt, 38, stream->onvif_profile, -1, SQLITE_STATIC);

        // Bind recording schedule parameters
        sqlite3_bind_int(stmt, 39, stream->record_on_schedule ? 1 : 0);
        char schedule_buf[RECORDING_SCHEDULE_TEXT_SIZE];
        serialize_recording_schedule(stream->recording_schedule, schedule_buf, sizeof(schedule_buf));
        sqlite3_bind_text(stmt, 40, schedule_buf, -1, SQLITE_TRANSIENT);

        // Bind tags parameter
        sqlite3_bind_text(stmt, 41, stream->tags, -1, SQLITE_STATIC);

        // Bind ID parameter
        sqlite3_bind_int64(stmt, 42, (sqlite3_int64)existing_id);

        // Execute statement
        rc = sqlite3_step(stmt);
        if (rc != SQLITE_DONE) {
            log_error("Failed to update disabled stream configuration: %s", sqlite3_errmsg(db));

            // Finalize the prepared statement
            if (stmt) {
                sqlite3_finalize(stmt);
                stmt = NULL;
            }
            pthread_mutex_unlock(db_mutex);
            return 0;
        }

        // Finalize the prepared statement
        if (stmt) {
            sqlite3_finalize(stmt);
            stmt = NULL;
        }

        log_info("Updated disabled stream configuration: name=%s, enabled=%s, detection=%s, model=%s",
                stream->name,
                stream->enabled ? "true" : "false",
                stream->detection_based_recording ? "true" : "false",
                stream->detection_model);

        pthread_mutex_unlock(db_mutex);
        return existing_id;
    }

    // Finalize the prepared statement
    if (check_stmt) {
        sqlite3_finalize(check_stmt);
        check_stmt = NULL;
    }

    // No disabled stream found, insert a new one
    const char *sql = "INSERT INTO streams (name, url, enabled, streaming_enabled, width, height, fps, codec, priority, record, segment_duration, "
          "detection_based_recording, detection_model, detection_threshold, detection_interval, "
          "pre_detection_buffer, post_detection_buffer, detection_api_url, "
          "detection_object_filter, detection_object_filter_list, "
          "protocol, is_onvif, record_audio, backchannel_enabled, "
          "retention_days, detection_retention_days, max_storage_mb, "
          "tier_critical_multiplier, tier_important_multiplier, tier_ephemeral_multiplier, storage_priority, "
          "ptz_enabled, ptz_max_x, ptz_max_y, ptz_max_z, ptz_has_home, "
          "onvif_username, onvif_password, onvif_profile, "
          "record_on_schedule, recording_schedule, tags) "
          "VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?);";

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
    sqlite3_bind_text(stmt, 18, stream->detection_api_url, -1, SQLITE_STATIC);

    // Bind detection object filter parameters
    sqlite3_bind_text(stmt, 19, stream->detection_object_filter, -1, SQLITE_STATIC);
    sqlite3_bind_text(stmt, 20, stream->detection_object_filter_list, -1, SQLITE_STATIC);

    // Bind protocol parameter
    sqlite3_bind_int(stmt, 21, (int)stream->protocol);

    // Bind is_onvif parameter
    sqlite3_bind_int(stmt, 22, stream->is_onvif ? 1 : 0);

    // Bind record_audio parameter
    sqlite3_bind_int(stmt, 23, stream->record_audio ? 1 : 0);

    // Bind backchannel_enabled parameter
    sqlite3_bind_int(stmt, 24, stream->backchannel_enabled ? 1 : 0);

    // Bind retention policy parameters
    sqlite3_bind_int(stmt, 25, stream->retention_days);
    sqlite3_bind_int(stmt, 26, stream->detection_retention_days);
    sqlite3_bind_int(stmt, 27, stream->max_storage_mb);

    // Bind tier multiplier parameters
    sqlite3_bind_double(stmt, 28, stream->tier_critical_multiplier);
    sqlite3_bind_double(stmt, 29, stream->tier_important_multiplier);
    sqlite3_bind_double(stmt, 30, stream->tier_ephemeral_multiplier);
    sqlite3_bind_int(stmt, 31, stream->storage_priority);

    // Bind PTZ parameters
    sqlite3_bind_int(stmt, 32, stream->ptz_enabled ? 1 : 0);
    sqlite3_bind_int(stmt, 33, stream->ptz_max_x);
    sqlite3_bind_int(stmt, 34, stream->ptz_max_y);
    sqlite3_bind_int(stmt, 35, stream->ptz_max_z);
    sqlite3_bind_int(stmt, 36, stream->ptz_has_home ? 1 : 0);

    // Bind ONVIF credentials
    sqlite3_bind_text(stmt, 37, stream->onvif_username, -1, SQLITE_STATIC);
    sqlite3_bind_text(stmt, 38, stream->onvif_password, -1, SQLITE_STATIC);
    sqlite3_bind_text(stmt, 39, stream->onvif_profile, -1, SQLITE_STATIC);

    // Bind recording schedule parameters
    sqlite3_bind_int(stmt, 40, stream->record_on_schedule ? 1 : 0);
    char insert_schedule_buf[RECORDING_SCHEDULE_TEXT_SIZE];
    serialize_recording_schedule(stream->recording_schedule, insert_schedule_buf, sizeof(insert_schedule_buf));
    sqlite3_bind_text(stmt, 41, insert_schedule_buf, -1, SQLITE_TRANSIENT);

    // Bind tags parameter
    sqlite3_bind_text(stmt, 42, stream->tags, -1, SQLITE_STATIC);

    // Execute statement
    rc = sqlite3_step(stmt);
    if (rc != SQLITE_DONE) {
        log_error("Failed to add stream configuration: %s", sqlite3_errmsg(db));
        // Continue to finalize the statement
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

    // Finalize the prepared statement
    if (stmt) {
        sqlite3_finalize(stmt);
        stmt = NULL;
    }
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

    // Schema migrations should have already been run during database initialization
    // No need to check for columns here anymore

    // Now update the stream with all fields including detection settings, protocol, is_onvif, record_audio, backchannel_enabled, retention settings, PTZ, ONVIF credentials, recording schedule, and tags
    const char *sql = "UPDATE streams SET "
                      "name = ?, url = ?, enabled = ?, streaming_enabled = ?, width = ?, height = ?, "
                      "fps = ?, codec = ?, priority = ?, record = ?, segment_duration = ?, "
                      "detection_based_recording = ?, detection_model = ?, detection_threshold = ?, "
                      "detection_interval = ?, pre_detection_buffer = ?, post_detection_buffer = ?, "
                      "detection_api_url = ?, detection_object_filter = ?, detection_object_filter_list = ?, "
                      "protocol = ?, is_onvif = ?, record_audio = ?, "
                      "backchannel_enabled = ?, retention_days = ?, detection_retention_days = ?, max_storage_mb = ?, "
                      "tier_critical_multiplier = ?, tier_important_multiplier = ?, tier_ephemeral_multiplier = ?, storage_priority = ?, "
                      "ptz_enabled = ?, ptz_max_x = ?, ptz_max_y = ?, ptz_max_z = ?, ptz_has_home = ?, "
                      "onvif_username = ?, onvif_password = ?, onvif_profile = ?, "
                      "record_on_schedule = ?, recording_schedule = ?, tags = ? "
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
    sqlite3_bind_text(stmt, 18, stream->detection_api_url, -1, SQLITE_STATIC);

    // Bind detection object filter parameters
    sqlite3_bind_text(stmt, 19, stream->detection_object_filter, -1, SQLITE_STATIC);
    sqlite3_bind_text(stmt, 20, stream->detection_object_filter_list, -1, SQLITE_STATIC);

    // Bind protocol parameter
    sqlite3_bind_int(stmt, 21, (int)stream->protocol);

    // Bind is_onvif parameter
    sqlite3_bind_int(stmt, 22, stream->is_onvif ? 1 : 0);

    // Bind record_audio parameter
    sqlite3_bind_int(stmt, 23, stream->record_audio ? 1 : 0);

    // Bind backchannel_enabled parameter
    sqlite3_bind_int(stmt, 24, stream->backchannel_enabled ? 1 : 0);

    // Bind retention policy parameters
    sqlite3_bind_int(stmt, 25, stream->retention_days);
    sqlite3_bind_int(stmt, 26, stream->detection_retention_days);
    sqlite3_bind_int(stmt, 27, stream->max_storage_mb);

    // Bind tier multiplier parameters
    sqlite3_bind_double(stmt, 28, stream->tier_critical_multiplier);
    sqlite3_bind_double(stmt, 29, stream->tier_important_multiplier);
    sqlite3_bind_double(stmt, 30, stream->tier_ephemeral_multiplier);
    sqlite3_bind_int(stmt, 31, stream->storage_priority);

    // Bind PTZ parameters
    sqlite3_bind_int(stmt, 32, stream->ptz_enabled ? 1 : 0);
    sqlite3_bind_int(stmt, 33, stream->ptz_max_x);
    sqlite3_bind_int(stmt, 34, stream->ptz_max_y);
    sqlite3_bind_int(stmt, 35, stream->ptz_max_z);
    sqlite3_bind_int(stmt, 36, stream->ptz_has_home ? 1 : 0);

    // Bind ONVIF credentials
    sqlite3_bind_text(stmt, 37, stream->onvif_username, -1, SQLITE_STATIC);
    sqlite3_bind_text(stmt, 38, stream->onvif_password, -1, SQLITE_STATIC);
    sqlite3_bind_text(stmt, 39, stream->onvif_profile, -1, SQLITE_STATIC);

    // Bind recording schedule parameters
    sqlite3_bind_int(stmt, 40, stream->record_on_schedule ? 1 : 0);
    char update_schedule_buf[RECORDING_SCHEDULE_TEXT_SIZE];
    serialize_recording_schedule(stream->recording_schedule, update_schedule_buf, sizeof(update_schedule_buf));
    sqlite3_bind_text(stmt, 41, update_schedule_buf, -1, SQLITE_TRANSIENT);

    // Bind tags parameter
    sqlite3_bind_text(stmt, 42, stream->tags, -1, SQLITE_STATIC);

    // Bind the WHERE clause parameter
    sqlite3_bind_text(stmt, 43, name, -1, SQLITE_STATIC);

    // Execute statement
    rc = sqlite3_step(stmt);
    if (rc != SQLITE_DONE) {
        log_error("Failed to update stream configuration: %s", sqlite3_errmsg(db));

        // Finalize the prepared statement
        if (stmt) {
            sqlite3_finalize(stmt);
            stmt = NULL;
        }
        pthread_mutex_unlock(db_mutex);
        return -1;
    }

    // Finalize the prepared statement
    if (stmt) {
        sqlite3_finalize(stmt);
        stmt = NULL;
    }

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
 * Update auto-detected video parameters for a stream.
 * Always updates to the detected values so the database stays in sync with the
 * actual stream resolution (which may change if the camera firmware is updated,
 * the user switches sub-stream / main-stream, etc.).
 *
 * A resolution change is logged at INFO level so operators can spot it.
 */
int update_stream_video_params(const char *stream_name, int width, int height, int fps, const char *codec) {
    int rc;
    sqlite3_stmt *stmt;

    sqlite3 *db = get_db_handle();
    pthread_mutex_t *db_mutex = get_db_mutex();

    if (!db) {
        log_error("Database not initialized");
        return -1;
    }

    if (!stream_name) {
        log_error("Stream name is required for video param update");
        return -1;
    }

    pthread_mutex_lock(db_mutex);

    // First, read the current values so we can log meaningful changes
    int old_width = 0, old_height = 0, old_fps = 0;
    char old_codec[16] = {0};

    const char *select_sql =
        "SELECT width, height, fps, codec FROM streams WHERE name = ?;";
    rc = sqlite3_prepare_v2(db, select_sql, -1, &stmt, NULL);
    if (rc == SQLITE_OK) {
        sqlite3_bind_text(stmt, 1, stream_name, -1, SQLITE_STATIC);
        if (sqlite3_step(stmt) == SQLITE_ROW) {
            old_width  = sqlite3_column_int(stmt, 0);
            old_height = sqlite3_column_int(stmt, 1);
            old_fps    = sqlite3_column_int(stmt, 2);
            const char *c = (const char *)sqlite3_column_text(stmt, 3);
            if (c) {
                strncpy(old_codec, c, sizeof(old_codec) - 1);
            }
        }
        sqlite3_finalize(stmt);
        stmt = NULL;
    }

    // Skip the UPDATE entirely if nothing changed — avoids unnecessary writes
    bool same = (old_width == width && old_height == height && old_fps == fps);
    if (same && codec) {
        same = (strcmp(old_codec, codec) == 0);
    }
    if (same) {
        pthread_mutex_unlock(db_mutex);
        return 0;   // nothing to do
    }

    // Always overwrite with the freshly-detected values
    const char *sql =
        "UPDATE streams SET "
        "width  = ?, "
        "height = ?, "
        "fps    = ?, "
        "codec  = ? "
        "WHERE name = ?;";

    rc = sqlite3_prepare_v2(db, sql, -1, &stmt, NULL);
    if (rc != SQLITE_OK) {
        log_error("Failed to prepare video params update: %s", sqlite3_errmsg(db));
        pthread_mutex_unlock(db_mutex);
        return -1;
    }

    sqlite3_bind_int(stmt, 1, width);
    sqlite3_bind_int(stmt, 2, height);
    sqlite3_bind_int(stmt, 3, fps);
    sqlite3_bind_text(stmt, 4, codec ? codec : "", -1, SQLITE_STATIC);
    sqlite3_bind_text(stmt, 5, stream_name, -1, SQLITE_STATIC);

    rc = sqlite3_step(stmt);
    if (rc != SQLITE_DONE) {
        log_error("Failed to update video params for stream %s: %s", stream_name, sqlite3_errmsg(db));
        sqlite3_finalize(stmt);
        pthread_mutex_unlock(db_mutex);
        return -1;
    }

    int changes = sqlite3_changes(db);
    sqlite3_finalize(stmt);
    pthread_mutex_unlock(db_mutex);

    if (changes > 0) {
        if (old_width != width || old_height != height) {
            log_info("Stream %s resolution changed: %dx%d -> %dx%d (fps %d->%d, codec %s->%s)",
                     stream_name, old_width, old_height, width, height,
                     old_fps, fps,
                     old_codec[0] ? old_codec : "?",
                     codec ? codec : "?");
        } else {
            log_info("Auto-detected video params for stream %s: %dx%d @ %d fps, codec=%s",
                     stream_name, width, height, fps, codec ? codec : "unknown");
        }
    }

    return 0;
}

/**
 * Delete a stream configuration from the database
 *
 * @param name Stream name to delete
 * @return 0 on success, non-zero on failure
 */
int delete_stream_config(const char *name) {
    return delete_stream_config_internal(name, false);
}

/**
 * Delete a stream configuration from the database with option for permanent deletion
 *
 * @param name Stream name to delete
 * @param permanent If true, permanently delete the stream; if false, just disable it
 * @return 0 on success, non-zero on failure
 */
int delete_stream_config_internal(const char *name, bool permanent) {
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

    const char *sql;
    if (permanent) {
        // Permanently delete the stream
        sql = "DELETE FROM streams WHERE name = ?;";
        log_info("Preparing to permanently delete stream: %s", name);
    } else {
        // Disable the stream by setting enabled = 0
        sql = "UPDATE streams SET enabled = 0 WHERE name = ?;";
        log_info("Preparing to disable stream: %s", name);
    }

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
        log_error("Failed to %s stream configuration: %s",
                permanent ? "permanently delete" : "disable",
                sqlite3_errmsg(db));

        // Finalize the prepared statement
        if (stmt) {
            sqlite3_finalize(stmt);
            stmt = NULL;
        }
        pthread_mutex_unlock(db_mutex);
        return -1;
    }

    // Finalize the prepared statement
    if (stmt) {
        sqlite3_finalize(stmt);
        stmt = NULL;
    }

    if (permanent) {
        log_info("Permanently deleted stream configuration: %s", name);
    } else {
        log_info("Disabled stream configuration: %s", name);
    }

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

    // After migrations, all columns are guaranteed to exist
    // Use a single query with all columns - column indices are fixed
    const char *sql =
        "SELECT name, url, enabled, streaming_enabled, width, height, fps, codec, priority, record, segment_duration, "
        "detection_based_recording, detection_model, detection_threshold, detection_interval, "
        "pre_detection_buffer, post_detection_buffer, detection_api_url, "
        "detection_object_filter, detection_object_filter_list, "
        "protocol, is_onvif, record_audio, backchannel_enabled, "
        "retention_days, detection_retention_days, max_storage_mb, "
        "tier_critical_multiplier, tier_important_multiplier, tier_ephemeral_multiplier, storage_priority, "
        "ptz_enabled, ptz_max_x, ptz_max_y, ptz_max_z, ptz_has_home, "
        "onvif_username, onvif_password, onvif_profile, "
        "record_on_schedule, recording_schedule, tags "
        "FROM streams WHERE name = ?;";

    // Column index constants for readability
    enum {
        COL_NAME = 0, COL_URL, COL_ENABLED, COL_STREAMING_ENABLED,
        COL_WIDTH, COL_HEIGHT, COL_FPS, COL_CODEC, COL_PRIORITY, COL_RECORD, COL_SEGMENT_DURATION,
        COL_DETECTION_BASED_RECORDING, COL_DETECTION_MODEL, COL_DETECTION_THRESHOLD, COL_DETECTION_INTERVAL,
        COL_PRE_DETECTION_BUFFER, COL_POST_DETECTION_BUFFER, COL_DETECTION_API_URL,
        COL_DETECTION_OBJECT_FILTER, COL_DETECTION_OBJECT_FILTER_LIST,
        COL_PROTOCOL, COL_IS_ONVIF, COL_RECORD_AUDIO, COL_BACKCHANNEL_ENABLED,
        COL_RETENTION_DAYS, COL_DETECTION_RETENTION_DAYS, COL_MAX_STORAGE_MB,
        COL_TIER_CRITICAL_MULTIPLIER, COL_TIER_IMPORTANT_MULTIPLIER, COL_TIER_EPHEMERAL_MULTIPLIER, COL_STORAGE_PRIORITY,
        COL_PTZ_ENABLED, COL_PTZ_MAX_X, COL_PTZ_MAX_Y, COL_PTZ_MAX_Z, COL_PTZ_HAS_HOME,
        COL_ONVIF_USERNAME, COL_ONVIF_PASSWORD, COL_ONVIF_PROFILE,
        COL_RECORD_ON_SCHEDULE, COL_RECORDING_SCHEDULE, COL_TAGS
    };

    rc = sqlite3_prepare_v2(db, sql, -1, &stmt, NULL);
    if (rc != SQLITE_OK) {
        log_error("Failed to prepare statement: %s", sqlite3_errmsg(db));
        pthread_mutex_unlock(db_mutex);
        return -1;
    }

    sqlite3_bind_text(stmt, 1, name, -1, SQLITE_STATIC);

    if (sqlite3_step(stmt) == SQLITE_ROW) {
        memset(stream, 0, sizeof(stream_config_t));

        // Basic stream settings
        const char *stream_name = (const char *)sqlite3_column_text(stmt, COL_NAME);
        if (stream_name) {
            strncpy(stream->name, stream_name, MAX_STREAM_NAME - 1);
            stream->name[MAX_STREAM_NAME - 1] = '\0';
        }

        const char *url = (const char *)sqlite3_column_text(stmt, COL_URL);
        if (url) {
            strncpy(stream->url, url, MAX_URL_LENGTH - 1);
            stream->url[MAX_URL_LENGTH - 1] = '\0';
        }

        stream->enabled = sqlite3_column_int(stmt, COL_ENABLED) != 0;
        stream->streaming_enabled = sqlite3_column_int(stmt, COL_STREAMING_ENABLED) != 0;
        stream->width = sqlite3_column_int(stmt, COL_WIDTH);
        stream->height = sqlite3_column_int(stmt, COL_HEIGHT);
        stream->fps = sqlite3_column_int(stmt, COL_FPS);

        const char *codec = (const char *)sqlite3_column_text(stmt, COL_CODEC);
        if (codec) {
            strncpy(stream->codec, codec, sizeof(stream->codec) - 1);
            stream->codec[sizeof(stream->codec) - 1] = '\0';
        }

        stream->priority = sqlite3_column_int(stmt, COL_PRIORITY);
        stream->record = sqlite3_column_int(stmt, COL_RECORD) != 0;
        stream->segment_duration = sqlite3_column_int(stmt, COL_SEGMENT_DURATION);

        // Detection settings
        stream->detection_based_recording = sqlite3_column_int(stmt, COL_DETECTION_BASED_RECORDING) != 0;

        const char *detection_model = (const char *)sqlite3_column_text(stmt, COL_DETECTION_MODEL);
        if (detection_model) {
            strncpy(stream->detection_model, detection_model, MAX_PATH_LENGTH - 1);
            stream->detection_model[MAX_PATH_LENGTH - 1] = '\0';
        }

        stream->detection_threshold = (sqlite3_column_type(stmt, COL_DETECTION_THRESHOLD) != SQLITE_NULL)
            ? (float)sqlite3_column_double(stmt, COL_DETECTION_THRESHOLD) : 0.5f;
        stream->detection_interval = (sqlite3_column_type(stmt, COL_DETECTION_INTERVAL) != SQLITE_NULL)
            ? sqlite3_column_int(stmt, COL_DETECTION_INTERVAL) : 10;
        stream->pre_detection_buffer = (sqlite3_column_type(stmt, COL_PRE_DETECTION_BUFFER) != SQLITE_NULL)
            ? sqlite3_column_int(stmt, COL_PRE_DETECTION_BUFFER) : 0;
        stream->post_detection_buffer = (sqlite3_column_type(stmt, COL_POST_DETECTION_BUFFER) != SQLITE_NULL)
            ? sqlite3_column_int(stmt, COL_POST_DETECTION_BUFFER) : 3;

        const char *detection_api_url = (const char *)sqlite3_column_text(stmt, COL_DETECTION_API_URL);
        if (detection_api_url) {
            strncpy(stream->detection_api_url, detection_api_url, MAX_URL_LENGTH - 1);
            stream->detection_api_url[MAX_URL_LENGTH - 1] = '\0';
        }

        // Detection object filter settings
        const char *detection_object_filter = (const char *)sqlite3_column_text(stmt, COL_DETECTION_OBJECT_FILTER);
        if (detection_object_filter) {
            strncpy(stream->detection_object_filter, detection_object_filter, sizeof(stream->detection_object_filter) - 1);
            stream->detection_object_filter[sizeof(stream->detection_object_filter) - 1] = '\0';
        } else {
            strncpy(stream->detection_object_filter, "none", sizeof(stream->detection_object_filter) - 1);
        }

        const char *detection_object_filter_list = (const char *)sqlite3_column_text(stmt, COL_DETECTION_OBJECT_FILTER_LIST);
        if (detection_object_filter_list) {
            strncpy(stream->detection_object_filter_list, detection_object_filter_list, sizeof(stream->detection_object_filter_list) - 1);
            stream->detection_object_filter_list[sizeof(stream->detection_object_filter_list) - 1] = '\0';
        }

        // Protocol and ONVIF
        stream->protocol = (sqlite3_column_type(stmt, COL_PROTOCOL) != SQLITE_NULL)
            ? (stream_protocol_t)sqlite3_column_int(stmt, COL_PROTOCOL) : STREAM_PROTOCOL_TCP;
        stream->is_onvif = sqlite3_column_int(stmt, COL_IS_ONVIF) != 0;
        stream->record_audio = sqlite3_column_int(stmt, COL_RECORD_AUDIO) != 0;
        stream->backchannel_enabled = sqlite3_column_int(stmt, COL_BACKCHANNEL_ENABLED) != 0;

        // Retention settings
        stream->retention_days = (sqlite3_column_type(stmt, COL_RETENTION_DAYS) != SQLITE_NULL)
            ? sqlite3_column_int(stmt, COL_RETENTION_DAYS) : 0;
        stream->detection_retention_days = (sqlite3_column_type(stmt, COL_DETECTION_RETENTION_DAYS) != SQLITE_NULL)
            ? sqlite3_column_int(stmt, COL_DETECTION_RETENTION_DAYS) : 0;
        stream->max_storage_mb = (sqlite3_column_type(stmt, COL_MAX_STORAGE_MB) != SQLITE_NULL)
            ? sqlite3_column_int(stmt, COL_MAX_STORAGE_MB) : 0;

        // Tier multiplier settings
        stream->tier_critical_multiplier = (sqlite3_column_type(stmt, COL_TIER_CRITICAL_MULTIPLIER) != SQLITE_NULL)
            ? sqlite3_column_double(stmt, COL_TIER_CRITICAL_MULTIPLIER) : 3.0;
        stream->tier_important_multiplier = (sqlite3_column_type(stmt, COL_TIER_IMPORTANT_MULTIPLIER) != SQLITE_NULL)
            ? sqlite3_column_double(stmt, COL_TIER_IMPORTANT_MULTIPLIER) : 2.0;
        stream->tier_ephemeral_multiplier = (sqlite3_column_type(stmt, COL_TIER_EPHEMERAL_MULTIPLIER) != SQLITE_NULL)
            ? sqlite3_column_double(stmt, COL_TIER_EPHEMERAL_MULTIPLIER) : 0.25;
        stream->storage_priority = (sqlite3_column_type(stmt, COL_STORAGE_PRIORITY) != SQLITE_NULL)
            ? sqlite3_column_int(stmt, COL_STORAGE_PRIORITY) : 5;

        // PTZ settings
        stream->ptz_enabled = sqlite3_column_int(stmt, COL_PTZ_ENABLED) != 0;
        stream->ptz_max_x = (sqlite3_column_type(stmt, COL_PTZ_MAX_X) != SQLITE_NULL)
            ? sqlite3_column_int(stmt, COL_PTZ_MAX_X) : 0;
        stream->ptz_max_y = (sqlite3_column_type(stmt, COL_PTZ_MAX_Y) != SQLITE_NULL)
            ? sqlite3_column_int(stmt, COL_PTZ_MAX_Y) : 0;
        stream->ptz_max_z = (sqlite3_column_type(stmt, COL_PTZ_MAX_Z) != SQLITE_NULL)
            ? sqlite3_column_int(stmt, COL_PTZ_MAX_Z) : 0;
        stream->ptz_has_home = sqlite3_column_int(stmt, COL_PTZ_HAS_HOME) != 0;

        // ONVIF credentials
        const char *onvif_username = (const char *)sqlite3_column_text(stmt, COL_ONVIF_USERNAME);
        if (onvif_username) {
            strncpy(stream->onvif_username, onvif_username, sizeof(stream->onvif_username) - 1);
            stream->onvif_username[sizeof(stream->onvif_username) - 1] = '\0';
        }

        const char *onvif_password = (const char *)sqlite3_column_text(stmt, COL_ONVIF_PASSWORD);
        if (onvif_password) {
            strncpy(stream->onvif_password, onvif_password, sizeof(stream->onvif_password) - 1);
            stream->onvif_password[sizeof(stream->onvif_password) - 1] = '\0';
        }

        const char *onvif_profile = (const char *)sqlite3_column_text(stmt, COL_ONVIF_PROFILE);
        if (onvif_profile) {
            strncpy(stream->onvif_profile, onvif_profile, sizeof(stream->onvif_profile) - 1);
            stream->onvif_profile[sizeof(stream->onvif_profile) - 1] = '\0';
        }

        // Recording schedule
        stream->record_on_schedule = sqlite3_column_int(stmt, COL_RECORD_ON_SCHEDULE) != 0;
        const char *recording_schedule_text = (const char *)sqlite3_column_text(stmt, COL_RECORDING_SCHEDULE);
        deserialize_recording_schedule(recording_schedule_text, stream->recording_schedule);

        // Tags
        const char *tags_val = (const char *)sqlite3_column_text(stmt, COL_TAGS);
        if (tags_val) {
            strncpy(stream->tags, tags_val, sizeof(stream->tags) - 1);
            stream->tags[sizeof(stream->tags) - 1] = '\0';
        } else {
            stream->tags[0] = '\0';
        }

        result = 0;
    }

    // Finalize the prepared statement
    if (stmt) {
        sqlite3_finalize(stmt);
        stmt = NULL;
    }
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

    // After migrations, all columns are guaranteed to exist
    const char *sql =
        "SELECT name, url, enabled, streaming_enabled, width, height, fps, codec, priority, record, segment_duration, "
        "detection_based_recording, detection_model, detection_threshold, detection_interval, "
        "pre_detection_buffer, post_detection_buffer, detection_api_url, "
        "detection_object_filter, detection_object_filter_list, "
        "protocol, is_onvif, record_audio, backchannel_enabled, "
        "retention_days, detection_retention_days, max_storage_mb, "
        "tier_critical_multiplier, tier_important_multiplier, tier_ephemeral_multiplier, storage_priority, "
        "ptz_enabled, ptz_max_x, ptz_max_y, ptz_max_z, ptz_has_home, "
        "record_on_schedule, recording_schedule, tags "
        "FROM streams ORDER BY name;";

    // Column index constants (same as get_stream_config_by_name, minus ONVIF creds)
    enum {
        COL_NAME = 0, COL_URL, COL_ENABLED, COL_STREAMING_ENABLED,
        COL_WIDTH, COL_HEIGHT, COL_FPS, COL_CODEC, COL_PRIORITY, COL_RECORD, COL_SEGMENT_DURATION,
        COL_DETECTION_BASED_RECORDING, COL_DETECTION_MODEL, COL_DETECTION_THRESHOLD, COL_DETECTION_INTERVAL,
        COL_PRE_DETECTION_BUFFER, COL_POST_DETECTION_BUFFER, COL_DETECTION_API_URL,
        COL_DETECTION_OBJECT_FILTER, COL_DETECTION_OBJECT_FILTER_LIST,
        COL_PROTOCOL, COL_IS_ONVIF, COL_RECORD_AUDIO, COL_BACKCHANNEL_ENABLED,
        COL_RETENTION_DAYS, COL_DETECTION_RETENTION_DAYS, COL_MAX_STORAGE_MB,
        COL_TIER_CRITICAL_MULTIPLIER, COL_TIER_IMPORTANT_MULTIPLIER, COL_TIER_EPHEMERAL_MULTIPLIER, COL_STORAGE_PRIORITY,
        COL_PTZ_ENABLED, COL_PTZ_MAX_X, COL_PTZ_MAX_Y, COL_PTZ_MAX_Z, COL_PTZ_HAS_HOME,
        COL_RECORD_ON_SCHEDULE, COL_RECORDING_SCHEDULE, COL_TAGS
    };

    rc = sqlite3_prepare_v2(db, sql, -1, &stmt, NULL);
    if (rc != SQLITE_OK) {
        log_error("Failed to prepare statement: %s", sqlite3_errmsg(db));
        pthread_mutex_unlock(db_mutex);
        return -1;
    }

    while (sqlite3_step(stmt) == SQLITE_ROW && count < max_count) {
        stream_config_t *s = &streams[count];
        memset(s, 0, sizeof(stream_config_t));

        // Basic settings
        const char *name = (const char *)sqlite3_column_text(stmt, COL_NAME);
        if (name) {
            strncpy(s->name, name, MAX_STREAM_NAME - 1);
            s->name[MAX_STREAM_NAME - 1] = '\0';
        }

        const char *url = (const char *)sqlite3_column_text(stmt, COL_URL);
        if (url) {
            strncpy(s->url, url, MAX_URL_LENGTH - 1);
            s->url[MAX_URL_LENGTH - 1] = '\0';
        }

        s->enabled = sqlite3_column_int(stmt, COL_ENABLED) != 0;
        s->streaming_enabled = sqlite3_column_int(stmt, COL_STREAMING_ENABLED) != 0;
        s->width = sqlite3_column_int(stmt, COL_WIDTH);
        s->height = sqlite3_column_int(stmt, COL_HEIGHT);
        s->fps = sqlite3_column_int(stmt, COL_FPS);

        const char *codec = (const char *)sqlite3_column_text(stmt, COL_CODEC);
        if (codec) {
            strncpy(s->codec, codec, sizeof(s->codec) - 1);
            s->codec[sizeof(s->codec) - 1] = '\0';
        }

        s->priority = sqlite3_column_int(stmt, COL_PRIORITY);
        s->record = sqlite3_column_int(stmt, COL_RECORD) != 0;
        s->segment_duration = sqlite3_column_int(stmt, COL_SEGMENT_DURATION);

        // Detection settings
        s->detection_based_recording = sqlite3_column_int(stmt, COL_DETECTION_BASED_RECORDING) != 0;

        const char *detection_model = (const char *)sqlite3_column_text(stmt, COL_DETECTION_MODEL);
        if (detection_model) {
            strncpy(s->detection_model, detection_model, MAX_PATH_LENGTH - 1);
            s->detection_model[MAX_PATH_LENGTH - 1] = '\0';
        }

        s->detection_threshold = (sqlite3_column_type(stmt, COL_DETECTION_THRESHOLD) != SQLITE_NULL)
            ? (float)sqlite3_column_double(stmt, COL_DETECTION_THRESHOLD) : 0.5f;
        s->detection_interval = (sqlite3_column_type(stmt, COL_DETECTION_INTERVAL) != SQLITE_NULL)
            ? sqlite3_column_int(stmt, COL_DETECTION_INTERVAL) : 10;
        s->pre_detection_buffer = (sqlite3_column_type(stmt, COL_PRE_DETECTION_BUFFER) != SQLITE_NULL)
            ? sqlite3_column_int(stmt, COL_PRE_DETECTION_BUFFER) : 0;
        s->post_detection_buffer = (sqlite3_column_type(stmt, COL_POST_DETECTION_BUFFER) != SQLITE_NULL)
            ? sqlite3_column_int(stmt, COL_POST_DETECTION_BUFFER) : 3;

        const char *detection_api_url = (const char *)sqlite3_column_text(stmt, COL_DETECTION_API_URL);
        if (detection_api_url) {
            strncpy(s->detection_api_url, detection_api_url, MAX_URL_LENGTH - 1);
            s->detection_api_url[MAX_URL_LENGTH - 1] = '\0';
        }

        // Detection object filter settings
        const char *detection_object_filter = (const char *)sqlite3_column_text(stmt, COL_DETECTION_OBJECT_FILTER);
        if (detection_object_filter) {
            strncpy(s->detection_object_filter, detection_object_filter, sizeof(s->detection_object_filter) - 1);
            s->detection_object_filter[sizeof(s->detection_object_filter) - 1] = '\0';
        } else {
            strncpy(s->detection_object_filter, "none", sizeof(s->detection_object_filter) - 1);
        }

        const char *detection_object_filter_list = (const char *)sqlite3_column_text(stmt, COL_DETECTION_OBJECT_FILTER_LIST);
        if (detection_object_filter_list) {
            strncpy(s->detection_object_filter_list, detection_object_filter_list, sizeof(s->detection_object_filter_list) - 1);
            s->detection_object_filter_list[sizeof(s->detection_object_filter_list) - 1] = '\0';
        }

        // Protocol and ONVIF
        s->protocol = (sqlite3_column_type(stmt, COL_PROTOCOL) != SQLITE_NULL)
            ? (stream_protocol_t)sqlite3_column_int(stmt, COL_PROTOCOL) : STREAM_PROTOCOL_TCP;
        s->is_onvif = sqlite3_column_int(stmt, COL_IS_ONVIF) != 0;
        s->record_audio = sqlite3_column_int(stmt, COL_RECORD_AUDIO) != 0;
        s->backchannel_enabled = sqlite3_column_int(stmt, COL_BACKCHANNEL_ENABLED) != 0;

        // Retention settings
        s->retention_days = (sqlite3_column_type(stmt, COL_RETENTION_DAYS) != SQLITE_NULL)
            ? sqlite3_column_int(stmt, COL_RETENTION_DAYS) : 0;
        s->detection_retention_days = (sqlite3_column_type(stmt, COL_DETECTION_RETENTION_DAYS) != SQLITE_NULL)
            ? sqlite3_column_int(stmt, COL_DETECTION_RETENTION_DAYS) : 0;
        s->max_storage_mb = (sqlite3_column_type(stmt, COL_MAX_STORAGE_MB) != SQLITE_NULL)
            ? sqlite3_column_int(stmt, COL_MAX_STORAGE_MB) : 0;

        // Tier multiplier settings
        s->tier_critical_multiplier = (sqlite3_column_type(stmt, COL_TIER_CRITICAL_MULTIPLIER) != SQLITE_NULL)
            ? sqlite3_column_double(stmt, COL_TIER_CRITICAL_MULTIPLIER) : 3.0;
        s->tier_important_multiplier = (sqlite3_column_type(stmt, COL_TIER_IMPORTANT_MULTIPLIER) != SQLITE_NULL)
            ? sqlite3_column_double(stmt, COL_TIER_IMPORTANT_MULTIPLIER) : 2.0;
        s->tier_ephemeral_multiplier = (sqlite3_column_type(stmt, COL_TIER_EPHEMERAL_MULTIPLIER) != SQLITE_NULL)
            ? sqlite3_column_double(stmt, COL_TIER_EPHEMERAL_MULTIPLIER) : 0.25;
        s->storage_priority = (sqlite3_column_type(stmt, COL_STORAGE_PRIORITY) != SQLITE_NULL)
            ? sqlite3_column_int(stmt, COL_STORAGE_PRIORITY) : 5;

        // PTZ settings
        s->ptz_enabled = sqlite3_column_int(stmt, COL_PTZ_ENABLED) != 0;
        s->ptz_max_x = (sqlite3_column_type(stmt, COL_PTZ_MAX_X) != SQLITE_NULL)
            ? sqlite3_column_int(stmt, COL_PTZ_MAX_X) : 0;
        s->ptz_max_y = (sqlite3_column_type(stmt, COL_PTZ_MAX_Y) != SQLITE_NULL)
            ? sqlite3_column_int(stmt, COL_PTZ_MAX_Y) : 0;
        s->ptz_max_z = (sqlite3_column_type(stmt, COL_PTZ_MAX_Z) != SQLITE_NULL)
            ? sqlite3_column_int(stmt, COL_PTZ_MAX_Z) : 0;
        s->ptz_has_home = sqlite3_column_int(stmt, COL_PTZ_HAS_HOME) != 0;

        // Recording schedule
        s->record_on_schedule = sqlite3_column_int(stmt, COL_RECORD_ON_SCHEDULE) != 0;
        const char *recording_schedule_text = (const char *)sqlite3_column_text(stmt, COL_RECORDING_SCHEDULE);
        deserialize_recording_schedule(recording_schedule_text, s->recording_schedule);

        // Tags
        const char *tags_val = (const char *)sqlite3_column_text(stmt, COL_TAGS);
        if (tags_val) {
            strncpy(s->tags, tags_val, sizeof(s->tags) - 1);
            s->tags[sizeof(s->tags) - 1] = '\0';
        } else {
            s->tags[0] = '\0';
        }

        count++;
    }

    // Finalize the prepared statement
    if (stmt) {
        sqlite3_finalize(stmt);
        stmt = NULL;
    }
    pthread_mutex_unlock(db_mutex);

    return count;
}

/**
 * Check if a stream is eligible for live streaming
 *
 * @param stream_name Name of the stream to check
 * @return 1 if eligible, 0 if not eligible, -1 on error
 */
int is_stream_eligible_for_live_streaming(const char *stream_name) {
    int rc;
    sqlite3_stmt *stmt;
    int result = -1;

    sqlite3 *db = get_db_handle();
    pthread_mutex_t *db_mutex = get_db_mutex();

    if (!db) {
        log_error("Database not initialized");
        return -1;
    }

    if (!stream_name) {
        log_error("Stream name is required");
        return -1;
    }

    pthread_mutex_lock(db_mutex);

    const char *sql = "SELECT enabled, streaming_enabled FROM streams WHERE name = ?;";

    rc = sqlite3_prepare_v2(db, sql, -1, &stmt, NULL);
    if (rc != SQLITE_OK) {
        log_error("Failed to prepare statement: %s", sqlite3_errmsg(db));
        pthread_mutex_unlock(db_mutex);
        return -1;
    }

    // Bind parameters
    sqlite3_bind_text(stmt, 1, stream_name, -1, SQLITE_STATIC);

    if (sqlite3_step(stmt) == SQLITE_ROW) {
        bool enabled = sqlite3_column_int(stmt, 0) != 0;
        bool streaming_enabled = sqlite3_column_int(stmt, 1) != 0;

        // Stream is eligible if it's enabled and streaming is enabled
        result = (enabled && streaming_enabled) ? 1 : 0;

        if (!enabled) {
            log_info("Stream %s is not eligible for live streaming: not enabled", stream_name);
        } else if (!streaming_enabled) {
            log_info("Stream %s is not eligible for live streaming: streaming not enabled", stream_name);
        }
    } else {
        log_error("Stream %s not found", stream_name);
        result = 0; // Not eligible if not found
    }

    // Finalize the prepared statement
    if (stmt) {
        sqlite3_finalize(stmt);
        stmt = NULL;
    }
    pthread_mutex_unlock(db_mutex);

    return result;
}

/**
 * Count the number of enabled stream configurations in the database
 *
 * @return Number of enabled streams, or -1 on error
 */
int get_enabled_stream_count(void) {
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

    const char *sql = "SELECT COUNT(*) FROM streams WHERE enabled = 1;";

    rc = sqlite3_prepare_v2(db, sql, -1, &stmt, NULL);
    if (rc != SQLITE_OK) {
        log_error("Failed to prepare statement: %s", sqlite3_errmsg(db));
        pthread_mutex_unlock(db_mutex);
        return -1;
    }

    if (sqlite3_step(stmt) == SQLITE_ROW) {
        count = sqlite3_column_int(stmt, 0);
    }

    // finalize the prepared statement
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

    // finalize the prepared statement
    sqlite3_finalize(stmt);
    pthread_mutex_unlock(db_mutex);

    return count;
}

/**
 * Get retention configuration for a stream
 *
 * @param stream_name Stream name
 * @param config Pointer to retention config structure to fill
 * @return 0 on success, non-zero on failure
 */
int get_stream_retention_config(const char *stream_name, stream_retention_config_t *config) {
    int rc;
    sqlite3_stmt *stmt;
    int result = -1;

    sqlite3 *db = get_db_handle();
    pthread_mutex_t *db_mutex = get_db_mutex();

    if (!db) {
        log_error("Database not initialized");
        return -1;
    }

    if (!stream_name || !config) {
        log_error("Stream name and config pointer are required");
        return -1;
    }

    // Set defaults
    config->retention_days = 30;
    config->detection_retention_days = 90;
    config->max_storage_mb = 0;

    pthread_mutex_lock(db_mutex);

    const char *sql = "SELECT retention_days, detection_retention_days, max_storage_mb "
                      "FROM streams WHERE name = ?;";

    rc = sqlite3_prepare_v2(db, sql, -1, &stmt, NULL);
    if (rc != SQLITE_OK) {
        log_error("Failed to prepare statement: %s", sqlite3_errmsg(db));
        pthread_mutex_unlock(db_mutex);
        return -1;
    }

    sqlite3_bind_text(stmt, 1, stream_name, -1, SQLITE_STATIC);

    if (sqlite3_step(stmt) == SQLITE_ROW) {
        if (sqlite3_column_type(stmt, 0) != SQLITE_NULL) {
            config->retention_days = sqlite3_column_int(stmt, 0);
        }
        if (sqlite3_column_type(stmt, 1) != SQLITE_NULL) {
            config->detection_retention_days = sqlite3_column_int(stmt, 1);
        }
        if (sqlite3_column_type(stmt, 2) != SQLITE_NULL) {
            config->max_storage_mb = (uint64_t)sqlite3_column_int64(stmt, 2);
        }
        result = 0;
    }

    sqlite3_finalize(stmt);
    pthread_mutex_unlock(db_mutex);

    return result;
}

/**
 * Set retention configuration for a stream
 *
 * @param stream_name Stream name
 * @param config Pointer to retention config structure with new values
 * @return 0 on success, non-zero on failure
 */
int set_stream_retention_config(const char *stream_name, const stream_retention_config_t *config) {
    int rc;
    sqlite3_stmt *stmt;

    sqlite3 *db = get_db_handle();
    pthread_mutex_t *db_mutex = get_db_mutex();

    if (!db) {
        log_error("Database not initialized");
        return -1;
    }

    if (!stream_name || !config) {
        log_error("Stream name and config are required");
        return -1;
    }

    pthread_mutex_lock(db_mutex);

    const char *sql = "UPDATE streams SET retention_days = ?, detection_retention_days = ?, "
                      "max_storage_mb = ? WHERE name = ?;";

    rc = sqlite3_prepare_v2(db, sql, -1, &stmt, NULL);
    if (rc != SQLITE_OK) {
        log_error("Failed to prepare statement: %s", sqlite3_errmsg(db));
        pthread_mutex_unlock(db_mutex);
        return -1;
    }

    sqlite3_bind_int(stmt, 1, config->retention_days);
    sqlite3_bind_int(stmt, 2, config->detection_retention_days);
    sqlite3_bind_int64(stmt, 3, (sqlite3_int64)config->max_storage_mb);
    sqlite3_bind_text(stmt, 4, stream_name, -1, SQLITE_STATIC);

    rc = sqlite3_step(stmt);
    sqlite3_finalize(stmt);
    pthread_mutex_unlock(db_mutex);

    if (rc != SQLITE_DONE) {
        log_error("Failed to update stream retention config: %s", sqlite3_errmsg(db));
        return -1;
    }

    log_info("Updated retention config for stream %s: retention_days=%d, detection_retention_days=%d, max_storage_mb=%lu",
             stream_name, config->retention_days, config->detection_retention_days,
             (unsigned long)config->max_storage_mb);

    return 0;
}

/**
 * Get all stream names for retention policy processing
 *
 * @param names Array of stream name buffers (each should be at least 64 chars)
 * @param max_count Maximum number of stream names to return
 * @return Number of streams found, or -1 on error
 */
int get_all_stream_names(char names[][64], int max_count) {
    int rc;
    sqlite3_stmt *stmt;
    int count = 0;

    sqlite3 *db = get_db_handle();
    pthread_mutex_t *db_mutex = get_db_mutex();

    if (!db) {
        log_error("Database not initialized");
        return -1;
    }

    if (!names || max_count <= 0) {
        log_error("Invalid parameters for get_all_stream_names");
        return -1;
    }

    pthread_mutex_lock(db_mutex);

    const char *sql = "SELECT name FROM streams WHERE enabled = 1 ORDER BY name;";

    rc = sqlite3_prepare_v2(db, sql, -1, &stmt, NULL);
    if (rc != SQLITE_OK) {
        log_error("Failed to prepare statement: %s", sqlite3_errmsg(db));
        pthread_mutex_unlock(db_mutex);
        return -1;
    }

    while (sqlite3_step(stmt) == SQLITE_ROW && count < max_count) {
        const char *name = (const char *)sqlite3_column_text(stmt, 0);
        if (name) {
            strncpy(names[count], name, 63);
            names[count][63] = '\0';
            count++;
        }
    }

    sqlite3_finalize(stmt);
    pthread_mutex_unlock(db_mutex);

    return count;
}

/**
 * Get storage usage for a stream in bytes
 *
 * @param stream_name Stream name
 * @return Total size in bytes, or 0 on error
 */
uint64_t get_stream_storage_usage_db(const char *stream_name) {
    int rc;
    sqlite3_stmt *stmt;
    uint64_t size_bytes = 0;

    sqlite3 *db = get_db_handle();
    pthread_mutex_t *db_mutex = get_db_mutex();

    if (!db) {
        log_error("Database not initialized");
        return 0;
    }

    if (!stream_name) {
        log_error("Stream name is required");
        return 0;
    }

    pthread_mutex_lock(db_mutex);

    const char *sql = "SELECT COALESCE(SUM(size_bytes), 0) FROM recordings "
                      "WHERE stream_name = ? AND is_complete = 1;";

    rc = sqlite3_prepare_v2(db, sql, -1, &stmt, NULL);
    if (rc != SQLITE_OK) {
        log_error("Failed to prepare statement: %s", sqlite3_errmsg(db));
        pthread_mutex_unlock(db_mutex);
        return 0;
    }

    sqlite3_bind_text(stmt, 1, stream_name, -1, SQLITE_STATIC);

    if (sqlite3_step(stmt) == SQLITE_ROW) {
        size_bytes = (uint64_t)sqlite3_column_int64(stmt, 0);
    }

    sqlite3_finalize(stmt);
    pthread_mutex_unlock(db_mutex);

    return size_bytes;
}
