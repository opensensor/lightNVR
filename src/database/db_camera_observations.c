#define _POSIX_C_SOURCE 200809L

#include "database/db_camera_observations.h"

#include <pthread.h>
#include <sqlite3.h>
#include <string.h>

#include "database/db_core.h"
#include "utils/strings.h"

int db_camera_observation_record(const char *stream_name,
                                 int64_t video_at,
                                 int64_t recording_at) {
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    if (!db || !mutex || !stream_name || stream_name[0] == '\0' ||
        (video_at <= 0 && recording_at <= 0)) return -1;
    const char *sql =
        "INSERT INTO camera_observations"
        "(camera_uuid,first_video_at,last_video_at,last_recording_at) "
        "SELECT camera_uuid,?,?,? FROM streams WHERE name=? "
        "ON CONFLICT(camera_uuid) DO UPDATE SET "
        "first_video_at=CASE "
        " WHEN camera_observations.first_video_at=0 AND excluded.first_video_at>0"
        " THEN excluded.first_video_at "
        " ELSE camera_observations.first_video_at END,"
        "last_video_at=MAX(camera_observations.last_video_at,"
        "excluded.last_video_at),"
        "last_recording_at=MAX(camera_observations.last_recording_at,"
        "excluded.last_recording_at),"
        "updated_at=strftime('%s','now');";
    int64_t first_video_at = video_at > 0 ? video_at : 0;
    pthread_mutex_lock(mutex);
    sqlite3_stmt *statement = NULL;
    int result = sqlite3_prepare_v2(db, sql, -1, &statement, NULL);
    if (result == SQLITE_OK) {
        sqlite3_bind_int64(statement, 1, first_video_at);
        sqlite3_bind_int64(statement, 2, video_at > 0 ? video_at : 0);
        sqlite3_bind_int64(statement, 3,
                           recording_at > 0 ? recording_at : 0);
        sqlite3_bind_text(statement, 4, stream_name, -1, SQLITE_TRANSIENT);
        result = sqlite3_step(statement);
    }
    int changed = result == SQLITE_DONE ? sqlite3_changes(db) : 0;
    if (statement) sqlite3_finalize(statement);
    pthread_mutex_unlock(mutex);
    return result == SQLITE_DONE && changed > 0 ? 0 : -1;
}

int db_camera_observation_get(const char *camera_uuid,
                              camera_observation_t *observation) {
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    if (!db || !mutex || !camera_uuid || !observation) return -1;
    const char *sql =
        "SELECT camera_uuid,first_video_at,last_video_at,last_recording_at,"
        "updated_at FROM camera_observations WHERE camera_uuid=?;";
    memset(observation, 0, sizeof(*observation));
    pthread_mutex_lock(mutex);
    sqlite3_stmt *statement = NULL;
    int result = sqlite3_prepare_v2(db, sql, -1, &statement, NULL);
    if (result == SQLITE_OK) {
        sqlite3_bind_text(statement, 1, camera_uuid, -1, SQLITE_TRANSIENT);
        result = sqlite3_step(statement);
    }
    if (result == SQLITE_ROW) {
        const char *uuid = (const char *)sqlite3_column_text(statement, 0);
        safe_strcpy(observation->camera_uuid, uuid ? uuid : "",
                    sizeof(observation->camera_uuid), 0);
        observation->first_video_at = sqlite3_column_int64(statement, 1);
        observation->last_video_at = sqlite3_column_int64(statement, 2);
        observation->last_recording_at = sqlite3_column_int64(statement, 3);
        observation->updated_at = sqlite3_column_int64(statement, 4);
    }
    if (statement) sqlite3_finalize(statement);
    pthread_mutex_unlock(mutex);
    return result == SQLITE_ROW ? 0 : result == SQLITE_DONE ? 1 : -1;
}
