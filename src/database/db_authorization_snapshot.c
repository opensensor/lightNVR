/* SPDX-License-Identifier: GPL-3.0-or-later */
#define _POSIX_C_SOURCE 200809L

#include <stdlib.h>
#include <string.h>
#include <time.h>

#include "database/db_authorization_snapshot.h"
#include "database/db_api_tokens.h"
#include "database/db_core.h"
#include "database/db_fleet_query.h"
#include "utils/strings.h"
#include "web/fleet_health.h"

/* These are exactly the fields mutated by fleet_camera_capture_runtime_health
 * and fleet_health_capture_go2rtc_activity. Inventory identity, tags, location,
 * capabilities and last_recording_at are immutable across the three views. */
typedef struct {
    fleet_health_state_t health;
    fleet_availability_state_t availability;
    int64_t health_changed_at;
    int64_t first_video_at;
    int64_t last_frame_ts;
    double current_fps;
    bool recording_active;
} camera_health_observation_t;

typedef struct {
    camera_health_observation_t raw;
    camera_health_observation_t runtime;
} camera_observations_t;

static camera_health_observation_t health_observation(const fleet_camera_t *camera) {
    return (camera_health_observation_t){
        .health = camera->health,
        .availability = camera->availability,
        .health_changed_at = camera->health_changed_at,
        .first_video_at = camera->first_video_at,
        .last_frame_ts = camera->last_frame_ts,
        .current_fps = camera->current_fps,
        .recording_active = camera->recording_active,
    };
}

static void apply_observation(fleet_camera_t *camera,
                              const camera_health_observation_t *observation) {
    camera->health = observation->health;
    camera->availability = observation->availability;
    camera->health_changed_at = observation->health_changed_at;
    camera->first_video_at = observation->first_video_at;
    camera->last_frame_ts = observation->last_frame_ts;
    camera->current_fps = observation->current_fps;
    camera->recording_active = observation->recording_active;
}

struct authorization_snapshot {
    sqlite3 *connection;
    authorization_context_t *context;
    user_t principal;
    /* Enriched in place; after capture it holds the final live observation. */
    fleet_camera_t *inventory;
    camera_observations_t *observations;
    int count;
    int64_t observed_at;
    struct timespec runtime_started;
    struct timespec runtime_finished;
};

void authorization_snapshot_free(authorization_snapshot_t *snapshot) {
    if (!snapshot) return;
    authorization_context_free(snapshot->context);
    free(snapshot->inventory);
    free(snapshot->observations);
    if (snapshot->connection) db_close_readonly_connection(snapshot->connection);
    free(snapshot);
}

static int load_principal(authorization_snapshot_t *snapshot, const user_t *input) {
    sqlite3_stmt *stmt = NULL;
    int rc = sqlite3_prepare_v2(snapshot->connection,
        "SELECT role,is_active,authorization_mode FROM users WHERE id=?;",
        -1, &stmt, NULL);
    if (rc != SQLITE_OK) return -1;
    sqlite3_bind_int64(stmt, 1, input->id);
    rc = sqlite3_step(stmt); /* First read pins all later policy reads. */
    int result = -1;
    if (rc == SQLITE_DONE) result = -2;
    else if (rc == SQLITE_ROW) {
        const char *mode = (const char *)sqlite3_column_text(stmt, 2);
        int role = sqlite3_column_int(stmt, 0);
        if (!sqlite3_column_int(stmt, 1)) result = -2;
        else if (!mode || (strcmp(mode, "legacy") && strcmp(mode, "policy")) ||
                 role < USER_ROLE_ADMIN || role > USER_ROLE_API) result = -1;
        else {
            user_t *user = &snapshot->principal;
            user->id = input->id;
            user->role = (user_role_t)role;
            user->is_active = true;
            safe_strcpy(user->authorization_mode, mode, sizeof(user->authorization_mode), 0);
            user->authenticated_via_scoped_token = input->authenticated_via_scoped_token;
            safe_strcpy(user->api_token_uuid, input->api_token_uuid, sizeof(user->api_token_uuid), 0);
            safe_strcpy(user->authentication_method, input->authentication_method,
                        sizeof(user->authentication_method), 0);
            result = 0;
        }
    }
    sqlite3_finalize(stmt);
    return result;
}

static int inventory_within_bound(sqlite3 *db) {
    sqlite3_stmt *stmt = NULL;
    int rc = sqlite3_prepare_v2(db, "SELECT count(*) FROM streams;", -1, &stmt, NULL);
    if (rc != SQLITE_OK) return -1;
    int result = -1;
    if (sqlite3_step(stmt) == SQLITE_ROW) {
        sqlite3_int64 count = sqlite3_column_int64(stmt, 0);
        if (count >= 0 && count <= MAX_STREAMS) result = 0;
    }
    sqlite3_finalize(stmt);
    return result;
}

int authorization_snapshot_create(const user_t *principal,
                                 authorization_snapshot_t **out) {
    if (!out) return -1;
    *out = NULL;
    if (!principal || principal->id <= 0) return -2;
    authorization_snapshot_t *snapshot = calloc(1, sizeof(*snapshot));
    if (!snapshot) return -1;
    int result = -1;
    if (db_open_readonly_connection(&snapshot->connection) != 0) goto fail;
    sqlite3_busy_timeout(snapshot->connection, 1000);
    if (sqlite3_exec(snapshot->connection, "BEGIN;", NULL, NULL, NULL) != SQLITE_OK) goto fail;
    result = load_principal(snapshot, principal);
    if (result != 0) goto fail;
    result = -1;
    snapshot->observed_at = (int64_t)time(NULL);
    if (snapshot->observed_at < 0) goto fail;
    if (snapshot->principal.authenticated_via_scoped_token) {
        api_token_t token;
        db_api_token_result_t token_result = db_api_token_get_active_on_connection(
            snapshot->connection, snapshot->observed_at,
            snapshot->principal.api_token_uuid, &token);
        if (token_result != DB_API_TOKEN_OK || token.user_id != principal->id) {
            result = token_result == DB_API_TOKEN_ERROR ? -1 : -2;
            goto fail;
        }
    }
    snapshot->context = authorization_context_create_on_connection(
        snapshot->connection, snapshot->observed_at);
    if (!snapshot->context || inventory_within_bound(snapshot->connection) != 0 ||
        db_fleet_camera_load_on_connection(snapshot->connection,
            &snapshot->inventory, &snapshot->count) != 0) goto fail;
    if (snapshot->count > 0) {
        snapshot->observations = calloc((size_t)snapshot->count,
                                         sizeof(*snapshot->observations));
        if (!snapshot->observations) goto fail;
        for (int i = 0; i < snapshot->count; i++) {
            snapshot->observations[i].raw = health_observation(&snapshot->inventory[i]);
        }
        if (clock_gettime(CLOCK_MONOTONIC, &snapshot->runtime_started) != 0 ||
            fleet_camera_capture_runtime_health(snapshot->inventory, snapshot->count) != 0) goto fail;
        for (int i = 0; i < snapshot->count; i++) {
            snapshot->observations[i].runtime = health_observation(&snapshot->inventory[i]);
        }
        if (fleet_health_capture_go2rtc_activity(snapshot->inventory, snapshot->count) != 0 ||
            clock_gettime(CLOCK_MONOTONIC, &snapshot->runtime_finished) != 0) goto fail;
    }
    *out = snapshot;
    return 0;
fail:
    authorization_snapshot_free(snapshot);
    return result;
}

bool authorization_snapshot_allows_non_scoped_identity(
    const authorization_snapshot_t *snapshot) {
    return snapshot && !snapshot->principal.authenticated_via_scoped_token;
}

int authorization_snapshot_setup_complete(const authorization_snapshot_t *snapshot) {
    if (!snapshot || !snapshot->connection) return -1;
    sqlite3_stmt *statement = NULL;
    int result = -1;
    if (sqlite3_prepare_v2(snapshot->connection,
        "SELECT (SELECT value FROM system_settings WHERE key='setup_complete');",
        -1, &statement, NULL) == SQLITE_OK && sqlite3_step(statement) == SQLITE_ROW) {
        const char *value = (const char *)sqlite3_column_text(statement, 0);
        result = value && value[0] == '1' ? 1 : 0;
    }
    sqlite3_finalize(statement);
    return result;
}

int authorization_snapshot_camera_count(const authorization_snapshot_t *snapshot) {
    return snapshot ? snapshot->count : 0;
}

int authorization_snapshot_evaluate(authorization_snapshot_t *snapshot,
    authorization_action_t action, int camera_index, const char *resource,
    authorization_evaluation_t *evaluation) {
    if (evaluation) memset(evaluation, 0, sizeof(*evaluation));
    if (!snapshot || !resource || !evaluation || camera_index < -1 ||
        camera_index >= snapshot->count) return -1;
    bool raw = strcmp(resource, "raw") == 0;
    bool runtime = strcmp(resource, "runtime") == 0;
    if (!raw && !runtime && strcmp(resource, "live") != 0) return -1;
    if (camera_index == -1) {
        return authorization_evaluate_in_context(snapshot->context, &snapshot->principal,
            action, NULL, evaluation);
    }
    /* Evaluation is synchronous and retains no camera pointer. Its context
     * caches policy/grants/collection membership/token, never camera identity
     * by address. Reconstruct on the stack without mutating the frozen inventory. */
    fleet_camera_t camera = snapshot->inventory[camera_index];
    if (raw) apply_observation(&camera, &snapshot->observations[camera_index].raw);
    else if (runtime) apply_observation(&camera, &snapshot->observations[camera_index].runtime);
    return authorization_evaluate_in_context(snapshot->context, &snapshot->principal,
        action, &camera, evaluation);
}
