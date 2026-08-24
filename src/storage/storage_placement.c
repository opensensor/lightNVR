#define _POSIX_C_SOURCE 200809L

#include "storage/storage_placement.h"

#include <cjson/cJSON.h>
#include <pthread.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>

#include "core/camera_selector.h"
#include "core/logger.h"
#include "database/db_fleet_query.h"
#include "database/db_storage_policies.h"
#include "database/db_storage_targets.h"
#include "utils/strings.h"

#define PLACEMENT_ASSIGNMENT_TTL_SECONDS 60

typedef struct {
    storage_policy_t policy;
    fleet_selector_t *selector;
} cached_policy_t;

typedef struct {
    bool used;
    char stream_name[MAX_STREAM_NAME];
    int policy_index;
    time_t evaluated_at;
} cached_assignment_t;

static pthread_mutex_t placement_cache_mutex = PTHREAD_MUTEX_INITIALIZER;
static cached_policy_t policy_cache[STORAGE_POLICY_MAX_COUNT];
static int policy_cache_count = 0;
static uint64_t policy_cache_generation = 0;
static cached_assignment_t assignment_cache[MAX_STREAMS];

static void clear_policy_cache_locked(void) {
    for (int index = 0; index < policy_cache_count; index++) {
        fleet_selector_free(policy_cache[index].selector);
        policy_cache[index].selector = NULL;
    }
    memset(policy_cache, 0, sizeof(policy_cache));
    policy_cache_count = 0;
    memset(assignment_cache, 0, sizeof(assignment_cache));
}

void storage_placement_cache_invalidate(void) {
    pthread_mutex_lock(&placement_cache_mutex);
    clear_policy_cache_locked();
    policy_cache_generation = 0;
    pthread_mutex_unlock(&placement_cache_mutex);
}

static int reload_policies_locked(uint64_t generation) {
    clear_policy_cache_locked();
    storage_policy_t *policies = calloc(STORAGE_POLICY_MAX_COUNT,
                                        sizeof(*policies));
    if (!policies) return -1;
    int count = db_storage_policy_list(
        policies, STORAGE_POLICY_MAX_COUNT, true);
    if (count < 0) {
        free(policies);
        return -1;
    }
    for (int index = 0; index < count; index++) {
        cJSON *json = cJSON_Parse(policies[index].selector_json);
        char error[FLEET_SELECTOR_ERROR_MAX] = {0};
        fleet_selector_t *selector = json
            ? fleet_selector_parse(json, error, sizeof(error)) : NULL;
        cJSON_Delete(json);
        if (!selector) {
            log_error("Skipping invalid storage policy %s: %s",
                      policies[index].uuid,
                      error[0] ? error : "invalid selector JSON");
            continue;
        }
        policy_cache[policy_cache_count].policy = policies[index];
        policy_cache[policy_cache_count].selector = selector;
        policy_cache_count++;
    }
    free(policies);
    policy_cache_generation = generation;
    return 0;
}

static int find_assignment_locked(const char *stream_name, time_t now) {
    for (int index = 0; index < MAX_STREAMS; index++) {
        if (!assignment_cache[index].used ||
            strcmp(assignment_cache[index].stream_name, stream_name) != 0) {
            continue;
        }
        if (now - assignment_cache[index].evaluated_at <=
            PLACEMENT_ASSIGNMENT_TTL_SECONDS) {
            return assignment_cache[index].policy_index;
        }
        assignment_cache[index].used = false;
        return -2;
    }
    return -2;
}

static void store_assignment_locked(const char *stream_name,
                                    int policy_index, time_t now) {
    int slot = -1;
    time_t oldest = now;
    for (int index = 0; index < MAX_STREAMS; index++) {
        if (assignment_cache[index].used &&
            strcmp(assignment_cache[index].stream_name, stream_name) == 0) {
            slot = index;
            break;
        }
        if (!assignment_cache[index].used) {
            slot = index;
            break;
        }
        if (assignment_cache[index].evaluated_at <= oldest) {
            oldest = assignment_cache[index].evaluated_at;
            slot = index;
        }
    }
    if (slot < 0) return;
    assignment_cache[slot].used = true;
    safe_strcpy(assignment_cache[slot].stream_name, stream_name,
                sizeof(assignment_cache[slot].stream_name), 0);
    assignment_cache[slot].policy_index = policy_index;
    assignment_cache[slot].evaluated_at = now;
}

static int resolve_policy(const char *stream_name, storage_policy_t *policy,
                          bool *matched) {
    *matched = false;
    time_t now = time(NULL);
    uint64_t generation = db_storage_policy_generation();
    pthread_mutex_lock(&placement_cache_mutex);
    if (policy_cache_generation != generation &&
        reload_policies_locked(generation) != 0) {
        pthread_mutex_unlock(&placement_cache_mutex);
        return -1;
    }
    int cached_index = find_assignment_locked(stream_name, now);
    if (cached_index >= 0 && cached_index < policy_cache_count) {
        *policy = policy_cache[cached_index].policy;
        *matched = true;
        pthread_mutex_unlock(&placement_cache_mutex);
        return 0;
    }
    if (cached_index == -1) {
        pthread_mutex_unlock(&placement_cache_mutex);
        return 0;
    }
    pthread_mutex_unlock(&placement_cache_mutex);

    fleet_camera_t camera;
    int camera_result = db_fleet_camera_find_by_name(stream_name, &camera);
    int selected = -1;
    if (camera_result == 0) {
        /* Keep runtime-health selectors consistent with Fleet previews. */
        fleet_camera_enrich_runtime_health(&camera, 1);
        pthread_mutex_lock(&placement_cache_mutex);
        generation = db_storage_policy_generation();
        if (policy_cache_generation != generation &&
            reload_policies_locked(generation) != 0) {
            pthread_mutex_unlock(&placement_cache_mutex);
            return -1;
        }
        for (int index = 0; index < policy_cache_count; index++) {
            if (fleet_selector_matches(policy_cache[index].selector,
                                       &camera, NULL)) {
                selected = index;
                break;
            }
        }
        store_assignment_locked(stream_name, selected, now);
        if (selected >= 0) {
            *policy = policy_cache[selected].policy;
            *matched = true;
        }
        pthread_mutex_unlock(&placement_cache_mutex);
        return 0;
    }
    if (camera_result < 0) return -1;

    /* A recording without fleet inventory safely follows the default target. */
    pthread_mutex_lock(&placement_cache_mutex);
    store_assignment_locked(stream_name, -1, now);
    pthread_mutex_unlock(&placement_cache_mutex);
    return 0;
}

static bool target_is_eligible(const char *uuid, storage_target_t *target) {
    if (db_storage_target_get(uuid, target) != DB_STORAGE_TARGET_OK) {
        return false;
    }
    if (!target->enabled) return false;
    if (target->mount_required) {
        static const char missing_mount_error[] =
            "Required mount is absent:";
        bool was_missing = strncmp(target->last_error, missing_mount_error,
                                   sizeof(missing_mount_error) - 1) == 0;
        if (!db_storage_target_mount_guard_active(target)) {
            /* Persist the transition once without touching the target path. */
            if (!was_missing) {
                (void)db_storage_target_probe(uuid, false, target);
            }
            return false;
        }
        if (was_missing &&
            db_storage_target_probe(uuid, false, target) !=
                DB_STORAGE_TARGET_OK) {
            return false;
        }
    }
    return strcmp(target->health_status, "healthy") == 0;
}

static bool default_target(storage_target_t *target) {
    storage_target_t configured;
    if (db_storage_target_get_default(&configured) != DB_STORAGE_TARGET_OK) {
        return false;
    }
    return target_is_eligible(configured.uuid, target);
}

static void ready(storage_placement_t *placement,
                  const storage_target_t *target, const char *reason) {
    placement->status = STORAGE_PLACEMENT_READY;
    placement->target_is_default = target->is_default;
    safe_strcpy(placement->target_uuid, target->uuid,
                sizeof(placement->target_uuid), 0);
    safe_strcpy(placement->target_root, target->root_path,
                sizeof(placement->target_root), 0);
    safe_strcpy(placement->reason, reason, sizeof(placement->reason), 0);
}

int storage_placement_select(const char *stream_name,
                             storage_placement_t *placement) {
    if (!stream_name || stream_name[0] == '\0' || !placement) return -1;
    memset(placement, 0, sizeof(*placement));
    placement->status = STORAGE_PLACEMENT_ERROR;

    storage_policy_t policy;
    bool matched = false;
    if (resolve_policy(stream_name, &policy, &matched) != 0) {
        /* Callers log this, so name the failure instead of an empty string. */
        safe_strcpy(placement->reason, "policy-lookup-failed",
                    sizeof(placement->reason), 0);
        return -1;
    }

    storage_target_t target;
    if (!matched) {
        if (!default_target(&target)) {
            placement->status = STORAGE_PLACEMENT_PAUSED;
            safe_strcpy(placement->reason, "default-unavailable",
                        sizeof(placement->reason), 0);
            return 0;
        }
        ready(placement, &target, "default-target");
        return 0;
    }

    safe_strcpy(placement->policy_uuid, policy.uuid,
                sizeof(placement->policy_uuid), 0);
    placement->policy_version = policy.revision;
    if (target_is_eligible(policy.primary_target_uuid, &target)) {
        char reason[64];
        snprintf(reason, sizeof(reason), "policy-primary:%s", policy.uuid);
        ready(placement, &target, reason);
        return 0;
    }

    if (strcmp(policy.fallback_mode, "pause") == 0) {
        placement->status = STORAGE_PLACEMENT_PAUSED;
        safe_strcpy(placement->reason, "policy-pause",
                    sizeof(placement->reason), 0);
        return 0;
    }
    if (strcmp(policy.fallback_mode, "fail") == 0) {
        placement->status = STORAGE_PLACEMENT_FAILED;
        safe_strcpy(placement->reason, "policy-fail",
                    sizeof(placement->reason), 0);
        return 0;
    }

    bool available = strcmp(policy.fallback_mode, "target") == 0
        ? target_is_eligible(policy.fallback_target_uuid, &target)
        : default_target(&target);
    if (!available) {
        placement->status = STORAGE_PLACEMENT_PAUSED;
        safe_strcpy(placement->reason, "fallback-unavailable",
                    sizeof(placement->reason), 0);
        return 0;
    }
    char reason[64];
    snprintf(reason, sizeof(reason),
             strcmp(policy.fallback_mode, "target") == 0
                ? "policy-fallback:%s" : "policy-default:%s",
             policy.uuid);
    ready(placement, &target, reason);
    return 0;
}
