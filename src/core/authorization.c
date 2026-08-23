#define _POSIX_C_SOURCE 200809L

#include <cjson/cJSON.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "core/authorization.h"
#include "core/camera_collection_filter.h"
#include "database/db_api_tokens.h"
#include "database/db_authorization.h"
#include "utils/strings.h"

static const authorization_action_metadata_t action_catalog[] = {
    {AUTHZ_LIVE_VIEW, "live.view", "Live video",
     "View a camera live stream", true, false},
    {AUTHZ_AUDIO_LISTEN, "audio.listen", "Live video",
     "Listen to camera audio", true, false},
    {AUTHZ_AUDIO_TALK, "audio.talk", "Live video",
     "Transmit audio to a camera", true, false},
    {AUTHZ_RECORDINGS_REPLAY, "recordings.replay", "Recordings",
     "Replay recorded video", true, false},
    {AUTHZ_RECORDINGS_EXPORT, "recordings.export", "Recordings",
     "Download or export recorded video", true, false},
    {AUTHZ_SNAPSHOT_CREATE, "snapshot.create", "Recordings",
     "Create or download a camera snapshot", true, false},
    {AUTHZ_PTZ_CONTROL, "ptz.control", "Camera operation",
     "Move PTZ cameras and manage presets", true, false},
    {AUTHZ_EVIDENCE_PROTECT, "evidence.protect", "Recordings",
     "Protect or release recordings from retention", true, true},
    {AUTHZ_RECORDING_DELETE, "recording.delete", "Recordings",
     "Permanently delete recordings", true, true},
    {AUTHZ_CAMERA_CONFIGURE, "camera.configure", "Camera administration",
     "Add, change, or remove camera configuration", true, true},
    {AUTHZ_FLEET_EXECUTE_JOB, "fleet.execute_job", "Camera administration",
     "Execute a bulk fleet operation", true, true},
    {AUTHZ_STORAGE_CONFIGURE, "storage.configure", "System administration",
     "Change storage and retention configuration", false, true},
    {AUTHZ_EVENTS_CONFIGURE, "events.configure", "System administration",
     "Change event routes and destinations", false, true},
    {AUTHZ_USERS_MANAGE, "users.manage", "System administration",
     "Manage users, roles, and grants", false, true},
    {AUTHZ_SYSTEM_ADMIN, "system.admin", "System administration",
     "Change or control the lightNVR system", false, true},
};

_Static_assert(sizeof(action_catalog) / sizeof(action_catalog[0]) ==
                   AUTHZ_ACTION_COUNT,
               "authorization action catalog must match the enum");

const authorization_action_metadata_t *authorization_action_catalog(
    int *action_count) {
    if (action_count) *action_count = AUTHZ_ACTION_COUNT;
    return action_catalog;
}

const authorization_action_metadata_t *authorization_action_metadata(
    authorization_action_t action) {
    if (action < 0 || action >= AUTHZ_ACTION_COUNT) return NULL;
    return &action_catalog[action];
}

authorization_action_t authorization_action_from_key(const char *key) {
    if (!key) return AUTHZ_ACTION_INVALID;
    for (int i = 0; i < AUTHZ_ACTION_COUNT; i++) {
        if (strcmp(action_catalog[i].key, key) == 0) {
            return action_catalog[i].action;
        }
    }
    return AUTHZ_ACTION_INVALID;
}

const char *authorization_decision_source_name(
    authorization_decision_source_t source) {
    switch (source) {
        case AUTHZ_SOURCE_LEGACY_ROLE: return "legacy_role";
        case AUTHZ_SOURCE_POLICY_GRANT: return "policy_grant";
        default: return "none";
    }
}

static bool legacy_role_allows(user_role_t role,
                               authorization_action_t action) {
    if (role == USER_ROLE_ADMIN) return true;
    switch (action) {
        case AUTHZ_LIVE_VIEW:
        case AUTHZ_AUDIO_LISTEN:
        case AUTHZ_RECORDINGS_REPLAY:
        case AUTHZ_RECORDINGS_EXPORT:
        case AUTHZ_SNAPSHOT_CREATE:
            return role == USER_ROLE_USER || role == USER_ROLE_VIEWER ||
                   role == USER_ROLE_API;
        case AUTHZ_AUDIO_TALK:
        case AUTHZ_PTZ_CONTROL:
        case AUTHZ_EVIDENCE_PROTECT:
        case AUTHZ_RECORDING_DELETE:
        case AUTHZ_CAMERA_CONFIGURE:
            return role == USER_ROLE_USER || role == USER_ROLE_API;
        default:
            return false;
    }
}

static int evaluate_legacy(const user_t *user,
                           const authorization_action_metadata_t *metadata,
                           const fleet_camera_t *camera,
                           authorization_evaluation_t *evaluation) {
    evaluation->source = AUTHZ_SOURCE_LEGACY_ROLE;
    safe_strcpy(evaluation->role_name, db_auth_get_role_name(user->role),
                sizeof(evaluation->role_name), 0);

    if (!legacy_role_allows(user->role, metadata->action)) {
        snprintf(evaluation->explanation, sizeof(evaluation->explanation),
                 "Legacy %s role does not include %s",
                 db_auth_get_role_name(user->role), metadata->key);
        return 0;
    }
    if (metadata->camera_scoped && user->has_tag_restriction) {
        if (!camera) {
            safe_strcpy(evaluation->explanation,
                        "A camera is required to evaluate the legacy tag scope",
                        sizeof(evaluation->explanation), 0);
            return 0;
        }
        if (!db_auth_stream_allowed_for_user(user, camera->legacy_tags)) {
            safe_strcpy(evaluation->explanation,
                        "Camera is outside the user's legacy allowed-tags scope",
                        sizeof(evaluation->explanation), 0);
            return 0;
        }
    }
    evaluation->decision = AUTHZ_DECISION_ALLOW;
    snprintf(evaluation->explanation, sizeof(evaluation->explanation),
             "Allowed by compatibility mapping for legacy %s role",
             db_auth_get_role_name(user->role));
    return 0;
}

static int grant_matches(const authorization_grant_t *grant,
                         const fleet_camera_t *camera, bool *matches) {
    *matches = false;
    if (strcmp(grant->scope_type, "all") == 0) {
        *matches = true;
        return 0;
    }
    if (strcmp(grant->scope_type, "collection") == 0) {
        if (!camera || grant->collection_uuid[0] == '\0') return 0;
        camera_collection_filter_t filter;
        camera_collection_filter_result_t result =
            camera_collection_filter_load_for_authorization(
                grant->collection_uuid, &filter);
        if (result != CAMERA_COLLECTION_FILTER_OK) return -1;
        *matches = camera_collection_filter_matches(&filter, camera);
        camera_collection_filter_free(&filter);
        return 0;
    }
    if (strcmp(grant->scope_type, "selector") != 0 || !camera ||
        grant->selector_json[0] == '\0') {
        return strcmp(grant->scope_type, "selector") == 0 ? 0 : -1;
    }

    cJSON *selector_json = cJSON_Parse(grant->selector_json);
    if (!selector_json) return -1;
    char error[FLEET_SELECTOR_ERROR_MAX] = {0};
    fleet_selector_t *selector =
        fleet_selector_parse(selector_json, error, sizeof(error));
    cJSON_Delete(selector_json);
    if (!selector) return -1;
    *matches = fleet_selector_matches(selector, camera, NULL);
    fleet_selector_free(selector);
    return 0;
}

static int evaluate_principal(const user_t *user,
                              authorization_action_t action,
                              const fleet_camera_t *camera,
                              authorization_evaluation_t *evaluation) {
    if (!user || !evaluation) return -1;
    const authorization_action_metadata_t *metadata =
        authorization_action_metadata(action);
    if (!metadata) return -1;

    memset(evaluation, 0, sizeof(*evaluation));

    if (!user->is_active) {
        safe_strcpy(evaluation->explanation, "User is inactive",
                    sizeof(evaluation->explanation), 0);
        return 0;
    }

    if (strcmp(user->authorization_mode, "policy") != 0) {
        (void)db_authorization_get_policy_version(&evaluation->policy_version);
        return evaluate_legacy(user, metadata, camera, evaluation);
    }

    authorization_grant_t *grants = NULL;
    int grant_count = 0;
    if (db_authorization_load_user_grants(
            user->id, metadata->key, &grants, &grant_count,
            &evaluation->policy_version) != 0) {
        return -1;
    }
    for (int i = 0; i < grant_count; i++) {
        bool matches = false;
        if (grant_matches(&grants[i], camera, &matches) != 0) {
            free(grants);
            return -1;
        }
        if (!matches) continue;
        evaluation->decision = AUTHZ_DECISION_ALLOW;
        evaluation->source = AUTHZ_SOURCE_POLICY_GRANT;
        safe_strcpy(evaluation->grant_uuid, grants[i].uuid,
                    sizeof(evaluation->grant_uuid), 0);
        safe_strcpy(evaluation->role_uuid, grants[i].role_uuid,
                    sizeof(evaluation->role_uuid), 0);
        safe_strcpy(evaluation->role_name, grants[i].role_name,
                    sizeof(evaluation->role_name), 0);
        snprintf(evaluation->explanation, sizeof(evaluation->explanation),
                 "Allowed by %s role grant", grants[i].role_name);
        free(grants);
        return 0;
    }
    free(grants);
    safe_strcpy(evaluation->explanation,
                metadata->camera_scoped && !camera
                    ? "No all-fleet grant allows this action"
                    : "No matching grant allows this action",
                sizeof(evaluation->explanation), 0);
    return 0;
}

static int token_scope_matches(const api_token_t *token,
                               const fleet_camera_t *camera, bool *matches) {
    *matches = false;
    if (strcmp(token->scope_type, "all") == 0) {
        *matches = true;
        return 0;
    }
    if (!camera) return 0;
    if (strcmp(token->scope_type, "collection") == 0) {
        camera_collection_filter_t filter;
        camera_collection_filter_result_t result =
            camera_collection_filter_load_for_authorization(
                token->collection_uuid, &filter);
        if (result != CAMERA_COLLECTION_FILTER_OK) return -1;
        *matches = camera_collection_filter_matches(&filter, camera);
        camera_collection_filter_free(&filter);
        return 0;
    }
    if (strcmp(token->scope_type, "selector") != 0 ||
        token->selector_json[0] == '\0') {
        return -1;
    }
    cJSON *json = cJSON_Parse(token->selector_json);
    if (!json) return -1;
    char error[FLEET_SELECTOR_ERROR_MAX] = {0};
    fleet_selector_t *selector =
        fleet_selector_parse(json, error, sizeof(error));
    cJSON_Delete(json);
    if (!selector) return -1;
    *matches = fleet_selector_matches(selector, camera, NULL);
    fleet_selector_free(selector);
    return 0;
}

int authorization_evaluate(const user_t *user, authorization_action_t action,
                           const fleet_camera_t *camera,
                           authorization_evaluation_t *evaluation) {
    int result = evaluate_principal(user, action, camera, evaluation);
    if (result != 0 || !user || !evaluation ||
        evaluation->decision != AUTHZ_DECISION_ALLOW ||
        !user->authenticated_via_scoped_token) {
        return result;
    }

    api_token_t token;
    db_api_token_result_t token_result =
        db_api_token_get_active(user->api_token_uuid, &token);
    if (token_result == DB_API_TOKEN_ERROR) return -1;
    if (token_result != DB_API_TOKEN_OK || token.user_id != user->id) {
        evaluation->decision = AUTHZ_DECISION_DENY;
        evaluation->source = AUTHZ_SOURCE_NONE;
        safe_strcpy(evaluation->explanation,
                    "Scoped API token is expired, revoked, or unavailable",
                    sizeof(evaluation->explanation), 0);
        return 0;
    }
    safe_strcpy(evaluation->token_uuid, token.uuid,
                sizeof(evaluation->token_uuid), 0);
    if ((token.action_mask & (UINT64_C(1) << action)) == 0) {
        evaluation->decision = AUTHZ_DECISION_DENY;
        evaluation->source = AUTHZ_SOURCE_NONE;
        safe_strcpy(evaluation->explanation,
                    "Scoped API token does not include this action",
                    sizeof(evaluation->explanation), 0);
        return 0;
    }
    bool matches = false;
    if (token_scope_matches(&token, camera, &matches) != 0) return -1;
    if (!matches) {
        evaluation->decision = AUTHZ_DECISION_DENY;
        evaluation->source = AUTHZ_SOURCE_NONE;
        safe_strcpy(evaluation->explanation,
                    camera ? "Camera is outside the scoped API token"
                           : "Scoped API token is not valid for global actions",
                    sizeof(evaluation->explanation), 0);
        return 0;
    }
    char principal_explanation[AUTHORIZATION_EXPLANATION_MAX];
    safe_strcpy(principal_explanation, evaluation->explanation,
                sizeof(principal_explanation), 0);
    safe_strcpy(evaluation->explanation, principal_explanation,
                sizeof(evaluation->explanation), 0);
    safe_strcat(evaluation->explanation, "; narrowed by scoped API token",
                sizeof(evaluation->explanation));
    return 0;
}
