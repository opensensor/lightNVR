#define _POSIX_C_SOURCE 200809L

#include <pthread.h>
#include <sqlite3.h>
#include <stdlib.h>
#include <string.h>

#include "core/logger.h"
#include "core/url_utils.h"
#include "database/db_core.h"
#include "database/db_fleet_query.h"
#include "telemetry/stream_metrics.h"
#include "utils/strings.h"

static void copy_column(char *destination, size_t destination_size,
                        sqlite3_stmt *stmt, int column) {
    const char *value = (const char *)sqlite3_column_text(stmt, column);
    safe_strcpy(destination, value ? value : "", destination_size, 0);
}

static int copy_safe_address(const char *raw_url, char *address,
                             size_t address_size) {
    char stripped[MAX_URL_LENGTH];
    if (url_strip_credentials(raw_url, stripped, sizeof(stripped)) != 0) {
        return -1;
    }
    const char *scheme_end = strstr(stripped, "://");
    if (!scheme_end) return -1;
    const char *authority = scheme_end + 3;
    const char *end = strpbrk(authority, "/?#");
    size_t length = end ? (size_t)(end - stripped) : strlen(stripped);
    if (length == 0 || length >= address_size) return -1;
    memcpy(address, stripped, length);
    address[length] = '\0';
    return 0;
}

static int load_location_ancestors(fleet_camera_t *camera,
                                   const char *ancestor_csv) {
    if (!ancestor_csv || ancestor_csv[0] == '\0') return 0;
    char copy[FLEET_CAMERA_MAX_LOCATION_DEPTH * CAMERA_UUID_STRING_SIZE];
    if (strlen(ancestor_csv) >= sizeof(copy)) return -1;
    safe_strcpy(copy, ancestor_csv, sizeof(copy), 0);
    char *saveptr = NULL;
    for (char *token = strtok_r(copy, ",", &saveptr);
         token != NULL;
         token = strtok_r(NULL, ",", &saveptr)) {
        if (camera->location_depth >= FLEET_CAMERA_MAX_LOCATION_DEPTH ||
            strlen(token) != CAMERA_UUID_STRING_SIZE - 1) {
            return -1;
        }
        safe_strcpy(camera->location_ancestor_uuids[camera->location_depth],
                    token, CAMERA_UUID_STRING_SIZE, 0);
        camera->location_depth++;
    }
    return 0;
}

int db_fleet_camera_load(fleet_camera_t **cameras, int *count) {
    if (!cameras || !count) return -1;
    *cameras = NULL;
    *count = 0;

    sqlite3 *db = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    if (!db) return -1;

    pthread_mutex_lock(mutex);
    sqlite3_stmt *stmt = NULL;
    int rc = sqlite3_prepare_v2(db, "SELECT count(*) FROM streams;", -1,
                                &stmt, NULL);
    int camera_count = -1;
    if (rc == SQLITE_OK && sqlite3_step(stmt) == SQLITE_ROW) {
        camera_count = sqlite3_column_int(stmt, 0);
    }
    if (stmt) {
        sqlite3_finalize(stmt);
        stmt = NULL;
    }
    if (camera_count < 0) {
        log_error("Failed to count fleet cameras: %s", sqlite3_errmsg(db));
        pthread_mutex_unlock(mutex);
        return -1;
    }
    if (camera_count == 0) {
        pthread_mutex_unlock(mutex);
        return 0;
    }

    fleet_camera_t *loaded = calloc((size_t)camera_count, sizeof(*loaded));
    if (!loaded) {
        pthread_mutex_unlock(mutex);
        return -1;
    }

    const char *sql =
        "WITH RECURSIVE location_tree(uuid, parent_uuid, name, path, ancestors) AS ("
        " SELECT uuid, parent_uuid, name, name, uuid FROM camera_locations "
        " WHERE parent_uuid IS NULL "
        " UNION ALL "
        " SELECT child.uuid, child.parent_uuid, child.name, "
        "        parent.path || ' / ' || child.name, "
        "        parent.ancestors || ',' || child.uuid "
        " FROM camera_locations child "
        " JOIN location_tree parent ON child.parent_uuid = parent.uuid"
        ") "
        "SELECT s.camera_uuid, s.name, s.url, s.tags, s.enabled, s.record, "
        "       s.detection_based_recording, s.is_onvif, s.ptz_enabled, "
        "       s.backchannel_enabled, s.location_uuid, "
        "       COALESCE(loc.name, ''), COALESCE(loc.path, ''), "
        "       COALESCE(loc.ancestors, ''), "
        "       COALESCE(tag.uuid, ''), COALESCE(tag.label, '') "
        "FROM streams s "
        "LEFT JOIN location_tree loc ON loc.uuid = s.location_uuid "
        "LEFT JOIN camera_tag_assignments assignment "
        "       ON assignment.camera_uuid = s.camera_uuid "
        "LEFT JOIN camera_tags tag ON tag.uuid = assignment.tag_uuid "
        "ORDER BY s.camera_uuid, tag.label COLLATE NOCASE;";

    rc = sqlite3_prepare_v2(db, sql, -1, &stmt, NULL);
    if (rc != SQLITE_OK) {
        log_error("Failed to prepare fleet inventory query: %s",
                  sqlite3_errmsg(db));
        free(loaded);
        pthread_mutex_unlock(mutex);
        return -1;
    }

    int loaded_count = 0;
    fleet_camera_t *camera = NULL;
    while ((rc = sqlite3_step(stmt)) == SQLITE_ROW) {
        const char *camera_uuid = (const char *)sqlite3_column_text(stmt, 0);
        if (!camera_uuid) {
            rc = SQLITE_CORRUPT;
            break;
        }
        if (!camera || strcmp(camera->camera_uuid, camera_uuid) != 0) {
            if (loaded_count >= camera_count) {
                rc = SQLITE_TOOBIG;
                break;
            }
            camera = &loaded[loaded_count++];
            copy_column(camera->camera_uuid, sizeof(camera->camera_uuid), stmt, 0);
            copy_column(camera->name, sizeof(camera->name), stmt, 1);
            char raw_url[MAX_URL_LENGTH];
            copy_column(raw_url, sizeof(raw_url), stmt, 2);
            if (copy_safe_address(raw_url, camera->address,
                                  sizeof(camera->address)) != 0) {
                camera->address[0] = '\0';
            }
            copy_column(camera->legacy_tags, sizeof(camera->legacy_tags), stmt, 3);
            camera->enabled = sqlite3_column_int(stmt, 4) != 0;
            camera->record = sqlite3_column_int(stmt, 5) != 0;
            camera->detection_based_recording =
                sqlite3_column_int(stmt, 6) != 0;
            camera->is_onvif = sqlite3_column_int(stmt, 7) != 0;
            camera->ptz_enabled = sqlite3_column_int(stmt, 8) != 0;
            camera->backchannel_enabled = sqlite3_column_int(stmt, 9) != 0;
            copy_column(camera->location_uuid,
                        sizeof(camera->location_uuid), stmt, 10);
            copy_column(camera->location_name,
                        sizeof(camera->location_name), stmt, 11);
            copy_column(camera->location_path,
                        sizeof(camera->location_path), stmt, 12);
            const char *ancestors =
                (const char *)sqlite3_column_text(stmt, 13);
            if (load_location_ancestors(camera, ancestors) != 0) {
                rc = SQLITE_TOOBIG;
                break;
            }
            camera->health = camera->enabled ? FLEET_HEALTH_UNKNOWN
                                             : FLEET_HEALTH_DISABLED;
        }

        const char *tag_uuid = (const char *)sqlite3_column_text(stmt, 14);
        if (tag_uuid && tag_uuid[0] != '\0') {
            if (camera->tag_count >= FLEET_CAMERA_MAX_TAGS) {
                rc = SQLITE_TOOBIG;
                break;
            }
            fleet_camera_tag_t *tag = &camera->tags[camera->tag_count++];
            copy_column(tag->uuid, sizeof(tag->uuid), stmt, 14);
            copy_column(tag->label, sizeof(tag->label), stmt, 15);
        }
    }
    sqlite3_finalize(stmt);
    if (rc != SQLITE_DONE || loaded_count != camera_count) {
        log_error("Failed to load fleet inventory: %s", sqlite3_errmsg(db));
        free(loaded);
        pthread_mutex_unlock(mutex);
        return -1;
    }
    pthread_mutex_unlock(mutex);

    *cameras = loaded;
    *count = loaded_count;
    return 0;
}

void fleet_camera_enrich_runtime_health(fleet_camera_t *cameras, int count) {
    if (!cameras || count <= 0) return;
    int maximum = metrics_get_max_streams();
    if (maximum <= 0) return;
    stream_metrics_t *metrics = calloc((size_t)maximum, sizeof(*metrics));
    if (!metrics) return;
    int metric_count = metrics_snapshot_all(metrics, maximum);
    for (int i = 0; i < count; i++) {
        if (!cameras[i].enabled) {
            cameras[i].health = FLEET_HEALTH_DISABLED;
            continue;
        }
        for (int j = 0; j < metric_count; j++) {
            if (strcmp(cameras[i].name, metrics[j].stream_name) != 0) continue;
            switch ((stream_health_status_t)metrics[j].health_status) {
                case STREAM_HEALTH_UP:
                    cameras[i].health = FLEET_HEALTH_UP;
                    break;
                case STREAM_HEALTH_DEGRADED:
                    cameras[i].health = FLEET_HEALTH_DEGRADED;
                    break;
                case STREAM_HEALTH_DOWN:
                    cameras[i].health = FLEET_HEALTH_DOWN;
                    break;
            }
            cameras[i].last_frame_ts = (int64_t)metrics[j].last_frame_ts;
            cameras[i].current_fps = metrics[j].current_fps;
            cameras[i].recording_active = metrics[j].recording_active != 0;
            break;
        }
    }
    free(metrics);
}
