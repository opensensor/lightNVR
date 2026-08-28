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

/*
 * Catalog order defines the persisted action-mask bit positions (see
 * authorization_action_bit). Append new actions at the end of the enum in
 * core/authorization.h; reordering silently re-maps every issued API token.
 */
static const authorization_action_metadata_t action_catalog[] = {
    {AUTHZ_LIVE_VIEW, "live.view", "Live video",
     "View a camera live stream", true, false, true},
    {AUTHZ_AUDIO_LISTEN, "audio.listen", "Live video",
     "Listen to camera audio", true, false, false},
    {AUTHZ_AUDIO_TALK, "audio.talk", "Live video",
     "Transmit audio to a camera", true, false, false},
    {AUTHZ_RECORDINGS_REPLAY, "recordings.replay", "Recordings",
     "Replay recorded video", true, false, true},
    {AUTHZ_RECORDINGS_EXPORT, "recordings.export", "Recordings",
     "Download or export recorded video", true, false, true},
    {AUTHZ_SNAPSHOT_CREATE, "snapshot.create", "Recordings",
     "Create or download a camera snapshot", true, false, true},
    {AUTHZ_PTZ_CONTROL, "ptz.control", "Camera operation",
     "Move PTZ cameras and manage presets", true, false, true},
    {AUTHZ_EVIDENCE_PROTECT, "evidence.protect", "Recordings",
     "Protect or release recordings from retention", true, true, true},
    {AUTHZ_RECORDING_DELETE, "recording.delete", "Recordings",
     "Permanently delete recordings", true, true, true},
    {AUTHZ_CAMERA_CONFIGURE, "camera.configure", "Camera administration",
     "Add, change, or remove camera configuration", true, true, true},
    {AUTHZ_FLEET_EXECUTE_JOB, "fleet.execute_job", "Camera administration",
     "Execute a bulk fleet operation", true, true, false},
    {AUTHZ_STORAGE_CONFIGURE, "storage.configure", "System administration",
     "Change storage and retention configuration", false, true, true},
    {AUTHZ_EVENTS_CONFIGURE, "events.configure", "System administration",
     "Change event routes and destinations", false, true, true},
    {AUTHZ_USERS_MANAGE, "users.manage", "System administration",
     "Manage users, roles, and grants", false, true, true},
    {AUTHZ_SYSTEM_ADMIN, "system.admin", "System administration",
     "Change or control the lightNVR system", false, true, true},
    {AUTHZ_LPR_READ, "lpr.read", "License plates",
     "View protected plate values", true, false, true},
    {AUTHZ_LPR_SEARCH, "lpr.search", "License plates",
     "Search protected plate reads", true, false, true},
    {AUTHZ_LPR_EXPORT, "lpr.export", "License plates",
     "Export protected plate reads", true, false, true},
    {AUTHZ_LPR_DELETE, "lpr.delete", "License plates",
     "Permanently delete protected plate reads", true, true, true},
};

_Static_assert(sizeof(action_catalog) / sizeof(action_catalog[0]) ==
                   AUTHZ_ACTION_COUNT,
               "authorization action catalog must match the enum");
_Static_assert(AUTHZ_ACTION_COUNT <= 64,
               "action masks are persisted as 64-bit integers");

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

uint64_t authorization_action_bit(authorization_action_t action) {
    if (action < 0 || action >= AUTHZ_ACTION_COUNT) return 0;
    return UINT64_C(1) << action;
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

/* ------------------------------------------------------------------ */
/* Evaluation context                                                  */
/* ------------------------------------------------------------------ */

typedef struct {
    char uuid[CAMERA_UUID_STRING_SIZE];
    camera_collection_filter_t filter;
    bool failed;
} collection_cache_entry_t;

typedef struct {
    int64_t user_id;
    authorization_action_t action;
    authorization_grant_t *grants;
    fleet_selector_t **selectors;
    int grant_count;
    int64_t policy_version;
    bool failed;
} grant_cache_entry_t;

struct authorization_context {
    grant_cache_entry_t *grants;
    int grant_count;
    int grant_capacity;
    collection_cache_entry_t *collections;
    int collection_count;
    int collection_capacity;
    bool token_loaded;
    bool token_valid;
    api_token_t token;
};

authorization_context_t *authorization_context_create(void) {
    return calloc(1, sizeof(authorization_context_t));
}

void authorization_context_free(authorization_context_t *context) {
    if (!context) return;
    for (int i = 0; i < context->grant_count; i++) {
        grant_cache_entry_t *entry = &context->grants[i];
        if (entry->selectors) {
            for (int j = 0; j < entry->grant_count; j++) {
                fleet_selector_free(entry->selectors[j]);
            }
            free(entry->selectors);
        }
        free(entry->grants);
    }
    free(context->grants);
    for (int i = 0; i < context->collection_count; i++) {
        camera_collection_filter_free(&context->collections[i].filter);
    }
    free(context->collections);
    free(context);
}

static void *grow_array(void *items, int *capacity, size_t item_size) {
    int next_capacity = *capacity == 0 ? 4 : *capacity * 2;
    void *resized = realloc(items, (size_t)next_capacity * item_size);
    if (!resized) return NULL;
    *capacity = next_capacity;
    return resized;
}

/*
 * Resolve a shared collection into a reusable filter. Returns NULL when the
 * collection cannot be used as an authorization boundary, which the caller
 * must treat as an evaluation failure rather than a deny.
 */
static const camera_collection_filter_t *context_collection_filter(
    authorization_context_t *context, const char *collection_uuid) {
    if (!collection_uuid || collection_uuid[0] == '\0') return NULL;
    for (int i = 0; i < context->collection_count; i++) {
        if (strcmp(context->collections[i].uuid, collection_uuid) == 0) {
            return context->collections[i].failed
                ? NULL : &context->collections[i].filter;
        }
    }
    if (context->collection_count == context->collection_capacity) {
        void *resized = grow_array(context->collections,
                                   &context->collection_capacity,
                                   sizeof(*context->collections));
        if (!resized) return NULL;
        context->collections = resized;
    }
    collection_cache_entry_t *entry =
        &context->collections[context->collection_count++];
    memset(entry, 0, sizeof(*entry));
    safe_strcpy(entry->uuid, collection_uuid, sizeof(entry->uuid), 0);
    entry->failed = camera_collection_filter_load_for_authorization(
                        collection_uuid, &entry->filter) !=
                    CAMERA_COLLECTION_FILTER_OK;
    return entry->failed ? NULL : &entry->filter;
}

static fleet_selector_t *parse_selector(const char *serialized) {
    if (!serialized || serialized[0] == '\0') return NULL;
    cJSON *json = cJSON_Parse(serialized);
    if (!json) return NULL;
    char error[FLEET_SELECTOR_ERROR_MAX] = {0};
    fleet_selector_t *selector =
        fleet_selector_parse(json, error, sizeof(error));
    cJSON_Delete(json);
    return selector;
}

/*
 * Load, and memoize, the enabled grants that carry an action for a user. The
 * selectors are parsed once here instead of once per candidate camera.
 */
static const grant_cache_entry_t *context_grants(
    authorization_context_t *context, int64_t user_id,
    authorization_action_t action, const char *action_key) {
    for (int i = 0; i < context->grant_count; i++) {
        if (context->grants[i].user_id == user_id &&
            context->grants[i].action == action) {
            return context->grants[i].failed ? NULL : &context->grants[i];
        }
    }
    if (context->grant_count == context->grant_capacity) {
        void *resized = grow_array(context->grants, &context->grant_capacity,
                                   sizeof(*context->grants));
        if (!resized) return NULL;
        context->grants = resized;
    }
    grant_cache_entry_t *entry = &context->grants[context->grant_count++];
    memset(entry, 0, sizeof(*entry));
    entry->user_id = user_id;
    entry->action = action;

    if (db_authorization_load_user_grants(user_id, action_key, &entry->grants,
                                          &entry->grant_count,
                                          &entry->policy_version) != 0) {
        entry->failed = true;
        entry->grants = NULL;
        entry->grant_count = 0;
        return NULL;
    }
    if (entry->grant_count > 0) {
        entry->selectors = calloc((size_t)entry->grant_count,
                                  sizeof(*entry->selectors));
        if (!entry->selectors) {
            entry->failed = true;
            return NULL;
        }
        for (int i = 0; i < entry->grant_count; i++) {
            if (strcmp(entry->grants[i].scope_type, "selector") != 0) continue;
            entry->selectors[i] = parse_selector(entry->grants[i].selector_json);
            if (!entry->selectors[i]) {
                entry->failed = true;
                return NULL;
            }
        }
    }
    return entry;
}

/* Returns 0 with *matches set, or -1 when the scope cannot be resolved. */
static int grant_matches(authorization_context_t *context,
                         const authorization_grant_t *grant,
                         fleet_selector_t *selector,
                         const fleet_camera_t *camera, bool *matches) {
    *matches = false;
    if (strcmp(grant->scope_type, "all") == 0) {
        *matches = true;
        return 0;
    }
    if (strcmp(grant->scope_type, "collection") == 0) {
        if (!camera) return 0;
        const camera_collection_filter_t *filter =
            context_collection_filter(context, grant->collection_uuid);
        if (!filter) return -1;
        *matches = camera_collection_filter_matches(filter, camera);
        return 0;
    }
    if (strcmp(grant->scope_type, "selector") != 0) return -1;
    if (!camera || !selector) return 0;
    *matches = fleet_selector_matches(selector, camera, NULL);
    return 0;
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
    (void)camera;
    evaluation->decision = AUTHZ_DECISION_ALLOW;
    snprintf(evaluation->explanation, sizeof(evaluation->explanation),
             "Allowed by compatibility mapping for legacy %s role",
             db_auth_get_role_name(user->role));
    return 0;
}

static int evaluate_principal(authorization_context_t *context,
                              const user_t *user,
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

    const grant_cache_entry_t *entry =
        context_grants(context, user->id, action, metadata->key);
    if (!entry) return -1;
    evaluation->policy_version = entry->policy_version;

    for (int i = 0; i < entry->grant_count; i++) {
        bool matches = false;
        if (grant_matches(context, &entry->grants[i],
                          entry->selectors ? entry->selectors[i] : NULL,
                          camera, &matches) != 0) {
            return -1;
        }
        if (!matches) continue;
        evaluation->decision = AUTHZ_DECISION_ALLOW;
        evaluation->source = AUTHZ_SOURCE_POLICY_GRANT;
        safe_strcpy(evaluation->grant_uuid, entry->grants[i].uuid,
                    sizeof(evaluation->grant_uuid), 0);
        safe_strcpy(evaluation->role_uuid, entry->grants[i].role_uuid,
                    sizeof(evaluation->role_uuid), 0);
        safe_strcpy(evaluation->role_name, entry->grants[i].role_name,
                    sizeof(evaluation->role_name), 0);
        snprintf(evaluation->explanation, sizeof(evaluation->explanation),
                 "Allowed by %s role grant", entry->grants[i].role_name);
        return 0;
    }
    safe_strcpy(evaluation->explanation,
                metadata->camera_scoped && !camera
                    ? "No all-fleet grant allows this action"
                    : "No matching grant allows this action",
                sizeof(evaluation->explanation), 0);
    return 0;
}

static int token_scope_matches(authorization_context_t *context,
                               const api_token_t *token,
                               const fleet_camera_t *camera, bool *matches) {
    *matches = false;
    if (strcmp(token->scope_type, "all") == 0) {
        *matches = true;
        return 0;
    }
    if (!camera) return 0;
    if (strcmp(token->scope_type, "collection") == 0) {
        const camera_collection_filter_t *filter =
            context_collection_filter(context, token->collection_uuid);
        if (!filter) return -1;
        *matches = camera_collection_filter_matches(filter, camera);
        return 0;
    }
    if (strcmp(token->scope_type, "selector") != 0) return -1;
    fleet_selector_t *selector = parse_selector(token->selector_json);
    if (!selector) return -1;
    *matches = fleet_selector_matches(selector, camera, NULL);
    fleet_selector_free(selector);
    return 0;
}

static const api_token_t *context_token(authorization_context_t *context,
                                        const user_t *user) {
    if (context->token_loaded) {
        return context->token_valid ? &context->token : NULL;
    }
    context->token_loaded = true;
    db_api_token_result_t result =
        db_api_token_get_active(user->api_token_uuid, &context->token);
    context->token_valid =
        result == DB_API_TOKEN_OK && context->token.user_id == user->id;
    return context->token_valid ? &context->token : NULL;
}

int authorization_evaluate_in_context(authorization_context_t *context,
                                      const user_t *user,
                                      authorization_action_t action,
                                      const fleet_camera_t *camera,
                                      authorization_evaluation_t *evaluation) {
    authorization_context_t *owned = NULL;
    if (!context) {
        owned = authorization_context_create();
        if (!owned) return -1;
        context = owned;
    }

    int result = evaluate_principal(context, user, action, camera, evaluation);
    if (result != 0 || !user || !evaluation ||
        evaluation->decision != AUTHZ_DECISION_ALLOW ||
        !user->authenticated_via_scoped_token) {
        authorization_context_free(owned);
        return result;
    }

    const api_token_t *token = context_token(context, user);
    if (!token) {
        evaluation->decision = AUTHZ_DECISION_DENY;
        evaluation->source = AUTHZ_SOURCE_NONE;
        safe_strcpy(evaluation->explanation,
                    "Scoped API token is expired, revoked, or unavailable",
                    sizeof(evaluation->explanation), 0);
        authorization_context_free(owned);
        return 0;
    }
    safe_strcpy(evaluation->token_uuid, token->uuid,
                sizeof(evaluation->token_uuid), 0);
    if ((token->action_mask & authorization_action_bit(action)) == 0) {
        evaluation->decision = AUTHZ_DECISION_DENY;
        evaluation->source = AUTHZ_SOURCE_NONE;
        safe_strcpy(evaluation->explanation,
                    "Scoped API token does not include this action",
                    sizeof(evaluation->explanation), 0);
        authorization_context_free(owned);
        return 0;
    }
    bool matches = false;
    if (token_scope_matches(context, token, camera, &matches) != 0) {
        authorization_context_free(owned);
        return -1;
    }
    if (!matches) {
        evaluation->decision = AUTHZ_DECISION_DENY;
        evaluation->source = AUTHZ_SOURCE_NONE;
        safe_strcpy(evaluation->explanation,
                    camera ? "Camera is outside the scoped API token"
                           : "Scoped API token is not valid for global actions",
                    sizeof(evaluation->explanation), 0);
        authorization_context_free(owned);
        return 0;
    }
    safe_strcat(evaluation->explanation, "; narrowed by scoped API token",
                sizeof(evaluation->explanation));
    authorization_context_free(owned);
    return 0;
}

int authorization_evaluate(const user_t *user, authorization_action_t action,
                           const fleet_camera_t *camera,
                           authorization_evaluation_t *evaluation) {
    return authorization_evaluate_in_context(NULL, user, action, camera,
                                             evaluation);
}

int authorization_filter_cameras(const user_t *user,
                                 authorization_action_t action,
                                 fleet_camera_t *cameras, int *count) {
    if (!user || !count) return -1;
    const authorization_action_metadata_t *metadata =
        authorization_action_metadata(action);
    if (!metadata || !metadata->camera_scoped) return -1;
    if (!cameras || *count <= 0) return 0;
    /* With authentication off there is no principal to evaluate: handlers on
     * that path never populate a user, and the rest of the codebase treats the
     * request as an administrator. Filtering a zeroed user would instead hide
     * every camera. */
    if (!g_config.web_auth_enabled) return 0;

    /* One context for the whole inventory: the grants, their selectors, and
     * any collection they reference are loaded once instead of per camera. */
    authorization_context_t *context = authorization_context_create();
    if (!context) return -1;

    int visible = 0;
    for (int i = 0; i < *count; i++) {
        authorization_evaluation_t evaluation;
        if (authorization_evaluate_in_context(context, user, action,
                                              &cameras[i], &evaluation) != 0) {
            authorization_context_free(context);
            return -1;
        }
        if (evaluation.decision != AUTHZ_DECISION_ALLOW) continue;
        if (visible != i) cameras[visible] = cameras[i];
        visible++;
    }
    authorization_context_free(context);
    *count = visible;
    return 0;
}

int authorization_filter_visible_cameras(const user_t *user,
                                         fleet_camera_t *cameras, int *count) {
    return authorization_filter_cameras(user, AUTHZ_LIVE_VIEW, cameras, count);
}

int authorization_effective_action_mask(const user_t *user, uint64_t *mask) {
    if (!user || !mask) return -1;
    *mask = 0;
    if (!user->is_active) return 0;

    if (strcmp(user->authorization_mode, "policy") != 0) {
        for (int i = 0; i < AUTHZ_ACTION_COUNT; i++) {
            if (legacy_role_allows(user->role, action_catalog[i].action)) {
                *mask |= authorization_action_bit(action_catalog[i].action);
            }
        }
    } else {
        char mode[USER_AUTHORIZATION_MODE_MAX];
        authorization_grant_t *grants = NULL;
        int grant_count = 0;
        int64_t policy_version = 0;
        if (db_authorization_get_user_policy(user->id, mode, &grants,
                                             &grant_count, &policy_version) !=
            DB_AUTHORIZATION_OK) {
            return -1;
        }
        for (int i = 0; i < grant_count; i++) {
            if (!grants[i].enabled) continue;
            authorization_role_t role;
            if (db_authorization_role_get(grants[i].role_uuid, &role) !=
                DB_AUTHORIZATION_OK) {
                free(grants);
                return -1;
            }
            *mask |= role.action_mask;
        }
        free(grants);
    }

    /* A scoped token can never widen the authority it was minted from. */
    if (user->authenticated_via_scoped_token) {
        api_token_t token;
        db_api_token_result_t result =
            db_api_token_get_active(user->api_token_uuid, &token);
        if (result == DB_API_TOKEN_ERROR) return -1;
        *mask = (result == DB_API_TOKEN_OK && token.user_id == user->id)
            ? (*mask & token.action_mask) : 0;
    }
    return 0;
}
