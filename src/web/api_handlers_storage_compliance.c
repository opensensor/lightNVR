#define _POSIX_C_SOURCE 200809L

#include "web/api_handlers_storage_compliance.h"

#include <cjson/cJSON.h>
#include <pthread.h>
#include <sqlite3.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>

#include "core/authorization.h"
#include "database/db_core.h"
#include "database/db_storage_lifecycle.h"
#include "database/db_storage_policies.h"
#include "database/db_storage_pools.h"
#include "database/db_storage_targets.h"
#include "web/httpd_utils.h"

static bool authorize_storage(const http_request_t *req,
                              http_response_t *res, user_t *user) {
    authorization_evaluation_t evaluation;
    return httpd_authorize_action(req, res, AUTHZ_STORAGE_CONFIGURE, NULL,
                                  user, &evaluation) != 0;
}

static const char *confidence_for(double sample_days, int samples) {
    if (samples < 3 || sample_days < 3.0) return "low";
    if (sample_days < 14.0) return "medium";
    return "high";
}

static void add_nullable_number(cJSON *object, const char *name,
                                bool available, double value) {
    if (available) cJSON_AddNumberToObject(object, name, value);
    else cJSON_AddNullToObject(object, name);
}

static int target_history(const char *uuid, uint64_t *bytes,
                          int64_t *first, int64_t *last, int *samples) {
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    pthread_mutex_lock(mutex);
    sqlite3_stmt *statement = NULL;
    int result = sqlite3_prepare_v2(db,
        "SELECT COALESCE(SUM(size_bytes),0),MIN(observed_at),MAX(observed_at),"
        "count(*) FROM (SELECT size_bytes,start_time observed_at FROM recordings "
        "WHERE storage_target_uuid=? AND is_complete=1 AND "
        "start_time>=strftime('%s','now')-2592000 UNION ALL SELECT size_bytes,"
        "created_at FROM storage_recording_copies WHERE target_uuid=? AND "
        "created_at>=strftime('%s','now')-2592000);", -1, &statement, NULL);
    if (result == SQLITE_OK) {
        sqlite3_bind_text(statement, 1, uuid, -1, SQLITE_TRANSIENT);
        sqlite3_bind_text(statement, 2, uuid, -1, SQLITE_TRANSIENT);
        result = sqlite3_step(statement);
    }
    if (result == SQLITE_ROW) {
        *bytes = (uint64_t)sqlite3_column_int64(statement, 0);
        *first = sqlite3_column_int64(statement, 1);
        *last = sqlite3_column_int64(statement, 2);
        *samples = sqlite3_column_int(statement, 3);
    }
    if (statement) sqlite3_finalize(statement);
    pthread_mutex_unlock(mutex);
    return result == SQLITE_ROW ? 0 : -1;
}

static cJSON *target_forecast(const storage_target_t *target) {
    uint64_t bytes = 0;
    int64_t first = 0;
    int64_t last = 0;
    int samples = 0;
    if (target_history(target->uuid, &bytes, &first, &last, &samples) != 0)
        return NULL;
    double sample_days = first > 0 && last >= first
        ? (double)(last - first) / 86400.0 + 1.0 : 0.0;
    double daily_rate = sample_days > 0.0 ? (double)bytes / sample_days : 0.0;
    double threshold_free = target->capacity_bytes *
        (100.0 - target->high_watermark_pct) / 100.0;
    double headroom = target->available_bytes > threshold_free
        ? (double)target->available_bytes - threshold_free : 0.0;
    double usable = target->capacity_bytes * target->high_watermark_pct / 100.0;
    if (usable > (double)target->reserve_bytes) usable -= target->reserve_bytes;
    else usable = 0.0;
    cJSON *object = cJSON_CreateObject();
    if (!object) return NULL;
    cJSON_AddStringToObject(object, "target_uuid", target->uuid);
    cJSON_AddStringToObject(object, "target_name", target->name);
    cJSON_AddNumberToObject(object, "sample_window_days", sample_days);
    cJSON_AddNumberToObject(object, "sample_count", samples);
    cJSON_AddNumberToObject(object, "observed_bytes", (double)bytes);
    cJSON_AddNumberToObject(object, "observed_daily_bytes", daily_rate);
    cJSON_AddStringToObject(object, "confidence",
                            confidence_for(sample_days, samples));
    add_nullable_number(object, "days_to_high_watermark", daily_rate > 0.0,
                        daily_rate > 0.0 ? headroom / daily_rate : 0.0);
    add_nullable_number(object, "expected_retention_days", daily_rate > 0.0,
                        daily_rate > 0.0 ? usable / daily_rate : 0.0);
    return object;
}

typedef struct {
    uint64_t recent_bytes;
    int64_t first_recent;
    int64_t last_recent;
    int samples;
    int total_recordings;
    uint64_t total_bytes;
    int64_t oldest;
    int copy_deficits;
    int active_violations;
} policy_history_t;

static int policy_history(const char *uuid, policy_history_t *history) {
    memset(history, 0, sizeof(*history));
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    pthread_mutex_lock(mutex);
    sqlite3_stmt *statement = NULL;
    const char *sql =
        "SELECT COALESCE(SUM(CASE WHEN r.start_time>=strftime('%s','now')-2592000 "
        "THEN r.size_bytes ELSE 0 END),0),"
        "MIN(CASE WHEN r.start_time>=strftime('%s','now')-2592000 THEN r.start_time END),"
        "MAX(CASE WHEN r.start_time>=strftime('%s','now')-2592000 THEN r.start_time END),"
        "SUM(CASE WHEN r.start_time>=strftime('%s','now')-2592000 THEN 1 ELSE 0 END),"
        "count(r.id),COALESCE(SUM(r.size_bytes),0),MIN(r.start_time),"
        "SUM(CASE WHEN r.id IS NOT NULL AND p.required_copy_count > "
        "1+(SELECT count(*) FROM storage_recording_copies c WHERE c.recording_id=r.id) "
        "THEN 1 ELSE 0 END),"
        "(SELECT count(*) FROM storage_policy_violations v WHERE "
        "v.policy_uuid=p.uuid AND v.resolved_at IS NULL) "
        "FROM storage_policies p LEFT JOIN recordings r ON "
        "p.uuid=substr(r.placement_reason,instr(r.placement_reason,':')+1) "
        "WHERE p.uuid=? GROUP BY p.uuid;";
    int result = sqlite3_prepare_v2(db, sql, -1, &statement, NULL);
    if (result == SQLITE_OK) {
        sqlite3_bind_text(statement, 1, uuid, -1, SQLITE_TRANSIENT);
        result = sqlite3_step(statement);
    }
    if (result == SQLITE_ROW) {
        history->recent_bytes = (uint64_t)sqlite3_column_int64(statement, 0);
        history->first_recent = sqlite3_column_int64(statement, 1);
        history->last_recent = sqlite3_column_int64(statement, 2);
        history->samples = sqlite3_column_int(statement, 3);
        history->total_recordings = sqlite3_column_int(statement, 4);
        history->total_bytes = (uint64_t)sqlite3_column_int64(statement, 5);
        history->oldest = sqlite3_column_int64(statement, 6);
        history->copy_deficits = sqlite3_column_int(statement, 7);
        history->active_violations = sqlite3_column_int(statement, 8);
    }
    if (statement) sqlite3_finalize(statement);
    pthread_mutex_unlock(mutex);
    return result == SQLITE_ROW ? 0 : -1;
}

static double policy_usable_capacity(const storage_policy_t *policy) {
    if (policy->primary_pool_uuid[0]) {
        storage_pool_t pool;
        if (db_storage_pool_get(policy->primary_pool_uuid, &pool) !=
            DB_STORAGE_POOL_OK) return 0.0;
        double total = 0.0;
        uint64_t devices[STORAGE_POOL_MEMBER_MAX] = {0};
        int device_count = 0;
        for (int index = 0; index < pool.member_count; index++) {
            storage_target_t target;
            if (db_storage_target_get(pool.members[index].target_uuid, &target) !=
                    DB_STORAGE_TARGET_OK || !target.enabled) continue;
            bool duplicate_device = false;
            if (target.filesystem_device != 0) {
                for (int seen = 0; seen < device_count; seen++) {
                    if (devices[seen] == target.filesystem_device) {
                        duplicate_device = true;
                        break;
                    }
                }
                if (!duplicate_device) devices[device_count++] =
                    target.filesystem_device;
            }
            if (duplicate_device) continue;
            double usable = target.capacity_bytes * target.high_watermark_pct / 100.0;
            if (usable > target.reserve_bytes) total += usable - target.reserve_bytes;
        }
        return total;
    }
    storage_target_t target;
    if (db_storage_target_get(policy->primary_target_uuid, &target) !=
        DB_STORAGE_TARGET_OK) return 0.0;
    double usable = target.capacity_bytes * target.high_watermark_pct / 100.0;
    return usable > target.reserve_bytes ? usable - target.reserve_bytes : 0.0;
}

static cJSON *policy_forecast(const storage_policy_t *policy) {
    policy_history_t history;
    if (policy_history(policy->uuid, &history) != 0) return NULL;
    double sample_days = history.first_recent > 0 &&
                         history.last_recent >= history.first_recent
        ? (double)(history.last_recent - history.first_recent) / 86400.0 + 1.0
        : 0.0;
    double daily_rate = sample_days > 0.0
        ? (double)history.recent_bytes / sample_days : 0.0;
    double expected_days = daily_rate > 0.0
        ? policy_usable_capacity(policy) / daily_rate : 0.0;
    double achieved_days = history.oldest > 0
        ? difftime(time(NULL), (time_t)history.oldest) / 86400.0 : 0.0;
    cJSON *object = cJSON_CreateObject();
    if (!object) return NULL;
    cJSON_AddStringToObject(object, "policy_uuid", policy->uuid);
    cJSON_AddStringToObject(object, "policy_name", policy->name);
    cJSON_AddNumberToObject(object, "recording_count", history.total_recordings);
    cJSON_AddNumberToObject(object, "recording_bytes", (double)history.total_bytes);
    cJSON_AddNumberToObject(object, "sample_window_days", sample_days);
    cJSON_AddNumberToObject(object, "observed_daily_bytes", daily_rate);
    cJSON_AddStringToObject(object, "confidence",
                            confidence_for(sample_days, history.samples));
    cJSON_AddNumberToObject(object, "achieved_retention_days", achieved_days);
    add_nullable_number(object, "expected_retention_days", daily_rate > 0.0,
                        expected_days);
    cJSON_AddNumberToObject(object, "minimum_retention_days",
                            policy->minimum_retention_days);
    cJSON_AddNumberToObject(object, "desired_retention_days",
                            policy->desired_retention_days);
    cJSON_AddNumberToObject(object, "maximum_retention_days",
                            policy->maximum_retention_days);
    cJSON_AddNumberToObject(object, "required_copy_count",
                            policy->required_copy_count);
    cJSON_AddNumberToObject(object, "copy_deficit_recordings",
                            history.copy_deficits);
    cJSON_AddNumberToObject(object, "active_violation_count",
                            history.active_violations);
    cJSON_AddBoolToObject(object, "minimum_forecast_met",
        policy->minimum_retention_days == 0 || daily_rate == 0.0 ||
        expected_days >= policy->minimum_retention_days);
    return object;
}

static cJSON *active_violations(void) {
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    cJSON *items = cJSON_CreateArray();
    if (!items) return NULL;
    pthread_mutex_lock(mutex);
    sqlite3_stmt *statement = NULL;
    int result = sqlite3_prepare_v2(db,
        "SELECT v.uuid,v.policy_uuid,p.name,COALESCE(v.recording_id,0),"
        "v.violation_type,v.details,v.first_seen_at,v.last_seen_at "
        "FROM storage_policy_violations v JOIN storage_policies p ON "
        "p.uuid=v.policy_uuid WHERE v.resolved_at IS NULL "
        "ORDER BY v.first_seen_at,v.uuid;", -1, &statement, NULL);
    while (result == SQLITE_OK && sqlite3_step(statement) == SQLITE_ROW) {
        cJSON *item = cJSON_CreateObject();
        if (!item) {
            cJSON_Delete(items);
            items = NULL;
            break;
        }
        cJSON_AddStringToObject(item, "uuid",
            (const char *)sqlite3_column_text(statement, 0));
        cJSON_AddStringToObject(item, "policy_uuid",
            (const char *)sqlite3_column_text(statement, 1));
        cJSON_AddStringToObject(item, "policy_name",
            (const char *)sqlite3_column_text(statement, 2));
        sqlite3_int64 recording_id = sqlite3_column_int64(statement, 3);
        if (recording_id > 0)
            cJSON_AddNumberToObject(item, "recording_id", (double)recording_id);
        else cJSON_AddNullToObject(item, "recording_id");
        cJSON_AddStringToObject(item, "type",
            (const char *)sqlite3_column_text(statement, 4));
        cJSON_AddStringToObject(item, "details",
            (const char *)sqlite3_column_text(statement, 5));
        cJSON_AddNumberToObject(item, "first_seen_at",
            (double)sqlite3_column_int64(statement, 6));
        cJSON_AddNumberToObject(item, "last_seen_at",
            (double)sqlite3_column_int64(statement, 7));
        cJSON_AddItemToArray(items, item);
    }
    if (statement) sqlite3_finalize(statement);
    pthread_mutex_unlock(mutex);
    return items;
}

void handle_get_storage_compliance(const http_request_t *req,
                                   http_response_t *res) {
    user_t user;
    if (!authorize_storage(req, res, &user)) return;
    if (db_storage_lifecycle_reconcile() < 0) {
        http_response_set_json_error(res, 500,
                                     "Failed to refresh lifecycle compliance");
        return;
    }
    int target_total = db_storage_target_count();
    int policy_total = db_storage_policy_count();
    storage_target_t *targets = target_total > 0 &&
        target_total <= STORAGE_TARGET_MAX_COUNT
            ? calloc((size_t)target_total, sizeof(*targets)) : NULL;
    storage_policy_t *policies = policy_total > 0 &&
        policy_total <= STORAGE_POLICY_MAX_COUNT
            ? calloc((size_t)policy_total, sizeof(*policies)) : NULL;
    int target_count = target_total < 0 ||
            target_total > STORAGE_TARGET_MAX_COUNT ||
            (target_total > 0 && !targets)
        ? -1 : (target_total > 0
            ? db_storage_target_list(targets, target_total) : 0);
    int policy_count = policy_total < 0 ||
            policy_total > STORAGE_POLICY_MAX_COUNT ||
            (policy_total > 0 && !policies)
        ? -1 : (policy_total > 0
            ? db_storage_policy_list(policies, policy_total, false) : 0);
    cJSON *root = cJSON_CreateObject();
    cJSON *target_items = cJSON_CreateArray();
    cJSON *policy_items = cJSON_CreateArray();
    cJSON *violations = active_violations();
    if (target_count < 0 || policy_count < 0 || !root || !target_items ||
        !policy_items || !violations) {
        free(targets);
        free(policies);
        cJSON_Delete(root);
        cJSON_Delete(target_items);
        cJSON_Delete(policy_items);
        cJSON_Delete(violations);
        http_response_set_json_error(res, 500,
                                     "Failed to build lifecycle compliance");
        return;
    }
    cJSON_AddNumberToObject(root, "forecast_window_days", 30);
    cJSON_AddStringToObject(root, "forecast_note",
        "Observed write rate; sparse or changing workloads reduce confidence.");
    cJSON_AddItemToObject(root, "targets", target_items);
    cJSON_AddItemToObject(root, "policies", policy_items);
    cJSON_AddItemToObject(root, "active_violations", violations);
    for (int index = 0; index < target_count; index++) {
        cJSON *item = target_forecast(&targets[index]);
        if (item) cJSON_AddItemToArray(target_items, item);
    }
    for (int index = 0; index < policy_count; index++) {
        cJSON *item = policy_forecast(&policies[index]);
        if (item) cJSON_AddItemToArray(policy_items, item);
    }
    free(targets);
    free(policies);
    char *body = cJSON_PrintUnformatted(root);
    cJSON_Delete(root);
    if (!body) {
        http_response_set_json_error(res, 500,
                                     "Failed to serialize lifecycle compliance");
        return;
    }
    http_response_set_json(res, 200, body);
    free(body);
}
