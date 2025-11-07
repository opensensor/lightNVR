#define _POSIX_C_SOURCE 200809L
#define _XOPEN_SOURCE 700
#define _GNU_SOURCE

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>
#include <sqlite3.h>
#include <stdbool.h>

#include "database/db_zones.h"
#include "database/db_core.h"
#include "core/logger.h"

/**
 * Initialize zones table
 */
int init_zones_table(void) {
    const char *sql =
        "CREATE TABLE IF NOT EXISTS detection_zones ("
        "id TEXT PRIMARY KEY,"
        "stream_name TEXT NOT NULL,"
        "name TEXT NOT NULL,"
        "enabled INTEGER DEFAULT 1,"
        "color TEXT DEFAULT '#3b82f6',"
        "polygon TEXT NOT NULL,"  // JSON array of points
        "filter_classes TEXT DEFAULT '',"
        "min_confidence REAL DEFAULT 0.0,"
        "created_at INTEGER NOT NULL,"
        "updated_at INTEGER NOT NULL,"
        "FOREIGN KEY (stream_name) REFERENCES streams(name) ON DELETE CASCADE"
        ");";

    sqlite3 *db = get_db_handle();
    if (!db) {
        log_error("Database not initialized");
        return -1;
    }

    char *err_msg = NULL;
    int rc = sqlite3_exec(db, sql, NULL, NULL, &err_msg);
    if (rc != SQLITE_OK) {
        log_error("Failed to create detection_zones table: %s", err_msg);
        sqlite3_free(err_msg);
        return -1;
    }

    // Create index on stream_name for faster lookups
    const char *index_sql = "CREATE INDEX IF NOT EXISTS idx_zones_stream ON detection_zones(stream_name);";
    rc = sqlite3_exec(db, index_sql, NULL, NULL, &err_msg);
    if (rc != SQLITE_OK) {
        log_error("Failed to create index on detection_zones: %s", err_msg);
        sqlite3_free(err_msg);
        return -1;
    }

    log_info("Detection zones table initialized successfully");
    return 0;
}

/**
 * Convert polygon points to JSON string
 */
static int polygon_to_json(const zone_point_t *polygon, int count, char *json_out, size_t max_len) {
    if (!polygon || count <= 0 || !json_out) {
        return -1;
    }

    int offset = 0;
    offset += snprintf(json_out + offset, max_len - offset, "[");

    for (int i = 0; i < count; i++) {
        if (i > 0) {
            offset += snprintf(json_out + offset, max_len - offset, ",");
        }
        offset += snprintf(json_out + offset, max_len - offset,
                          "{\"x\":%.6f,\"y\":%.6f}",
                          polygon[i].x, polygon[i].y);
    }

    offset += snprintf(json_out + offset, max_len - offset, "]");
    return 0;
}

/**
 * Parse JSON polygon string to points array
 */
static int json_to_polygon(const char *json, zone_point_t *polygon, int *count, int max_points) {
    if (!json || !polygon || !count) {
        return -1;
    }

    *count = 0;
    const char *p = json;

    // Skip opening bracket
    while (*p && *p != '[') p++;
    if (*p != '[') return -1;
    p++;

    // Parse points
    while (*p && *count < max_points) {
        // Skip whitespace and commas
        while (*p && (*p == ' ' || *p == ',' || *p == '\n' || *p == '\r')) p++;
        if (*p == ']') break;
        if (*p != '{') return -1;

        // Parse point
        float x, y;
        if (sscanf(p, "{\"x\":%f,\"y\":%f}", &x, &y) == 2) {
            polygon[*count].x = x;
            polygon[*count].y = y;
            (*count)++;

            // Skip to next point
            while (*p && *p != '}') p++;
            if (*p == '}') p++;
        } else {
            return -1;
        }
    }

    return 0;
}

/**
 * Save detection zones for a stream
 */
int save_detection_zones(const char *stream_name, const detection_zone_t *zones, int count) {
    if (!stream_name || !zones || count < 0) {
        log_error("Invalid parameters for save_detection_zones");
        return -1;
    }

    sqlite3 *db = get_db_handle();
    if (!db) {
        log_error("Database not initialized");
        return -1;
    }

    // Start transaction
    char *err_msg = NULL;
    if (sqlite3_exec(db, "BEGIN TRANSACTION;", NULL, NULL, &err_msg) != SQLITE_OK) {
        log_error("Failed to begin transaction: %s", err_msg);
        sqlite3_free(err_msg);
        return -1;
    }

    // Delete existing zones for this stream
    const char *delete_sql = "DELETE FROM detection_zones WHERE stream_name = ?;";
    sqlite3_stmt *stmt = NULL;
    int rc = sqlite3_prepare_v2(db, delete_sql, -1, &stmt, NULL);
    if (rc != SQLITE_OK) {
        log_error("Failed to prepare delete statement: %s", sqlite3_errmsg(db));
        sqlite3_exec(db, "ROLLBACK;", NULL, NULL, NULL);
        return -1;
    }

    sqlite3_bind_text(stmt, 1, stream_name, -1, SQLITE_STATIC);
    rc = sqlite3_step(stmt);
    sqlite3_finalize(stmt);

    if (rc != SQLITE_DONE) {
        log_error("Failed to delete existing zones: %s", sqlite3_errmsg(db));
        sqlite3_exec(db, "ROLLBACK;", NULL, NULL, NULL);
        return -1;
    }

    // Insert new zones
    const char *insert_sql =
        "INSERT INTO detection_zones (id, stream_name, name, enabled, color, polygon, "
        "filter_classes, min_confidence, created_at, updated_at) "
        "VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?);";

    time_t now = time(NULL);

    for (int i = 0; i < count; i++) {
        const detection_zone_t *zone = &zones[i];

        rc = sqlite3_prepare_v2(db, insert_sql, -1, &stmt, NULL);
        if (rc != SQLITE_OK) {
            log_error("Failed to prepare insert statement: %s", sqlite3_errmsg(db));
            sqlite3_exec(db, "ROLLBACK;", NULL, NULL, NULL);
            return -1;
        }

        // Convert polygon to JSON
        char polygon_json[2048];
        if (polygon_to_json(zone->polygon, zone->polygon_count, polygon_json, sizeof(polygon_json)) != 0) {
            log_error("Failed to convert polygon to JSON for zone %s", zone->id);
            sqlite3_finalize(stmt);
            sqlite3_exec(db, "ROLLBACK;", NULL, NULL, NULL);
            return -1;
        }

        sqlite3_bind_text(stmt, 1, zone->id, -1, SQLITE_STATIC);
        sqlite3_bind_text(stmt, 2, stream_name, -1, SQLITE_STATIC);
        sqlite3_bind_text(stmt, 3, zone->name, -1, SQLITE_STATIC);
        sqlite3_bind_int(stmt, 4, zone->enabled ? 1 : 0);
        sqlite3_bind_text(stmt, 5, zone->color, -1, SQLITE_STATIC);
        sqlite3_bind_text(stmt, 6, polygon_json, -1, SQLITE_TRANSIENT);
        sqlite3_bind_text(stmt, 7, zone->filter_classes, -1, SQLITE_STATIC);
        sqlite3_bind_double(stmt, 8, zone->min_confidence);
        sqlite3_bind_int64(stmt, 9, now);
        sqlite3_bind_int64(stmt, 10, now);

        rc = sqlite3_step(stmt);
        sqlite3_finalize(stmt);

        if (rc != SQLITE_DONE) {
            log_error("Failed to insert zone %s: %s", zone->id, sqlite3_errmsg(db));
            sqlite3_exec(db, "ROLLBACK;", NULL, NULL, NULL);
            return -1;
        }
    }

    // Commit transaction
    if (sqlite3_exec(db, "COMMIT;", NULL, NULL, &err_msg) != SQLITE_OK) {
        log_error("Failed to commit transaction: %s", err_msg);
        sqlite3_free(err_msg);
        return -1;
    }

    log_info("Saved %d detection zones for stream %s", count, stream_name);
    return 0;
}

/**
 * Get detection zones for a stream
 */
int get_detection_zones(const char *stream_name, detection_zone_t *zones, int max_zones) {
    if (!stream_name || !zones || max_zones <= 0) {
        log_error("Invalid parameters for get_detection_zones");
        return -1;
    }

    sqlite3 *db = get_db_handle();
    if (!db) {
        log_error("Database not initialized");
        return -1;
    }

    const char *sql =
        "SELECT id, stream_name, name, enabled, color, polygon, filter_classes, min_confidence "
        "FROM detection_zones WHERE stream_name = ? ORDER BY created_at;";

    sqlite3_stmt *stmt = NULL;
    int rc = sqlite3_prepare_v2(db, sql, -1, &stmt, NULL);
    if (rc != SQLITE_OK) {
        log_error("Failed to prepare select statement: %s", sqlite3_errmsg(db));
        return -1;
    }

    sqlite3_bind_text(stmt, 1, stream_name, -1, SQLITE_STATIC);

    int count = 0;
    while (sqlite3_step(stmt) == SQLITE_ROW && count < max_zones) {
        detection_zone_t *zone = &zones[count];

        strncpy(zone->id, (const char *)sqlite3_column_text(stmt, 0), MAX_ZONE_ID - 1);
        strncpy(zone->stream_name, (const char *)sqlite3_column_text(stmt, 1), MAX_STREAM_NAME - 1);
        strncpy(zone->name, (const char *)sqlite3_column_text(stmt, 2), MAX_ZONE_NAME - 1);
        zone->enabled = sqlite3_column_int(stmt, 3) != 0;
        strncpy(zone->color, (const char *)sqlite3_column_text(stmt, 4), 7);
        zone->color[7] = '\0';

        const char *polygon_json = (const char *)sqlite3_column_text(stmt, 5);
        if (json_to_polygon(polygon_json, zone->polygon, &zone->polygon_count, MAX_ZONE_POINTS) != 0) {
            log_warn("Failed to parse polygon for zone %s", zone->id);
            continue;
        }

        const char *filter_classes = (const char *)sqlite3_column_text(stmt, 6);
        if (filter_classes) {
            strncpy(zone->filter_classes, filter_classes, 255);
            zone->filter_classes[255] = '\0';
        } else {
            zone->filter_classes[0] = '\0';
        }

        zone->min_confidence = sqlite3_column_double(stmt, 7);

        count++;
    }

    sqlite3_finalize(stmt);
    return count;
}

/**
 * Delete all zones for a stream
 */
int delete_detection_zones(const char *stream_name) {
    if (!stream_name) {
        return -1;
    }

    sqlite3 *db = get_db_handle();
    if (!db) {
        return -1;
    }

    const char *sql = "DELETE FROM detection_zones WHERE stream_name = ?;";
    sqlite3_stmt *stmt = NULL;

    int rc = sqlite3_prepare_v2(db, sql, -1, &stmt, NULL);
    if (rc != SQLITE_OK) {
        return -1;
    }

    sqlite3_bind_text(stmt, 1, stream_name, -1, SQLITE_STATIC);
    rc = sqlite3_step(stmt);
    sqlite3_finalize(stmt);

    return (rc == SQLITE_DONE) ? 0 : -1;
}

/**
 * Delete a specific zone
 */
int delete_detection_zone(const char *zone_id) {
    if (!zone_id) {
        return -1;
    }

    sqlite3 *db = get_db_handle();
    if (!db) {
        return -1;
    }

    const char *sql = "DELETE FROM detection_zones WHERE id = ?;";
    sqlite3_stmt *stmt = NULL;

    int rc = sqlite3_prepare_v2(db, sql, -1, &stmt, NULL);
    if (rc != SQLITE_OK) {
        return -1;
    }

    sqlite3_bind_text(stmt, 1, zone_id, -1, SQLITE_STATIC);
    rc = sqlite3_step(stmt);
    sqlite3_finalize(stmt);

    return (rc == SQLITE_DONE) ? 0 : -1;
}

/**
 * Update a zone's enabled status
 */
int update_zone_enabled(const char *zone_id, bool enabled) {
    if (!zone_id) {
        return -1;
    }

    sqlite3 *db = get_db_handle();
    if (!db) {
        return -1;
    }

    const char *sql = "UPDATE detection_zones SET enabled = ?, updated_at = ? WHERE id = ?;";
    sqlite3_stmt *stmt = NULL;

    int rc = sqlite3_prepare_v2(db, sql, -1, &stmt, NULL);
    if (rc != SQLITE_OK) {
        return -1;
    }

    sqlite3_bind_int(stmt, 1, enabled ? 1 : 0);
    sqlite3_bind_int64(stmt, 2, time(NULL));
    sqlite3_bind_text(stmt, 3, zone_id, -1, SQLITE_STATIC);

    rc = sqlite3_step(stmt);
    sqlite3_finalize(stmt);

    return (rc == SQLITE_DONE) ? 0 : -1;
}

