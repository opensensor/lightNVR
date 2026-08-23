#ifndef LIGHTNVR_AUTHORIZATION_H
#define LIGHTNVR_AUTHORIZATION_H

#include <stdbool.h>
#include <stdint.h>

#include "core/camera_selector.h"
#include "database/db_auth.h"

#define AUTHORIZATION_ACTION_KEY_MAX 64
#define AUTHORIZATION_CATEGORY_MAX 64
#define AUTHORIZATION_DESCRIPTION_MAX 256
#define AUTHORIZATION_EXPLANATION_MAX 256

typedef enum {
    AUTHZ_ACTION_INVALID = -1,
    AUTHZ_LIVE_VIEW = 0,
    AUTHZ_AUDIO_LISTEN,
    AUTHZ_AUDIO_TALK,
    AUTHZ_RECORDINGS_REPLAY,
    AUTHZ_RECORDINGS_EXPORT,
    AUTHZ_SNAPSHOT_CREATE,
    AUTHZ_PTZ_CONTROL,
    AUTHZ_EVIDENCE_PROTECT,
    AUTHZ_RECORDING_DELETE,
    AUTHZ_CAMERA_CONFIGURE,
    AUTHZ_FLEET_EXECUTE_JOB,
    AUTHZ_STORAGE_CONFIGURE,
    AUTHZ_EVENTS_CONFIGURE,
    AUTHZ_USERS_MANAGE,
    AUTHZ_SYSTEM_ADMIN,
    AUTHZ_ACTION_COUNT
} authorization_action_t;

typedef struct {
    authorization_action_t action;
    const char *key;
    const char *category;
    const char *description;
    bool camera_scoped;
    bool destructive;
} authorization_action_metadata_t;

typedef enum {
    AUTHZ_DECISION_DENY = 0,
    AUTHZ_DECISION_ALLOW = 1
} authorization_decision_t;

typedef enum {
    AUTHZ_SOURCE_NONE = 0,
    AUTHZ_SOURCE_LEGACY_ROLE,
    AUTHZ_SOURCE_POLICY_GRANT
} authorization_decision_source_t;

typedef struct {
    authorization_decision_t decision;
    authorization_decision_source_t source;
    int64_t policy_version;
    char grant_uuid[CAMERA_UUID_STRING_SIZE];
    char role_uuid[CAMERA_UUID_STRING_SIZE];
    char role_name[128];
    char explanation[AUTHORIZATION_EXPLANATION_MAX];
} authorization_evaluation_t;

const authorization_action_metadata_t *authorization_action_catalog(
    int *action_count);
const authorization_action_metadata_t *authorization_action_metadata(
    authorization_action_t action);
authorization_action_t authorization_action_from_key(const char *key);
const char *authorization_decision_source_name(
    authorization_decision_source_t source);

/*
 * Evaluate one user/action/resource tuple. A NULL camera is valid for global
 * actions and for an all-fleet grant, but never matches a selector-backed grant.
 * Returns 0 for an allow/deny decision and -1 for an evaluation failure. Callers
 * must fail closed when this function returns -1.
 */
int authorization_evaluate(const user_t *user, authorization_action_t action,
                           const fleet_camera_t *camera,
                           authorization_evaluation_t *evaluation);

#endif /* LIGHTNVR_AUTHORIZATION_H */
