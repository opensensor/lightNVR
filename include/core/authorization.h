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
    /*
     * True once at least one request handler routes this action through the
     * centralized evaluator. Unenforced actions are still grantable so that a
     * policy can be authored ahead of the enforcement work, but they must be
     * reported as unenforced so operators are not told a boundary exists
     * before it does. See docs/internal/AUTHORIZATION_ENDPOINT_INVENTORY.md.
     */
    bool enforced;
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
    char token_uuid[CAMERA_UUID_STRING_SIZE];
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
 * Bit position of an action inside a persisted action mask. The mask is stored
 * in authz_api_tokens.action_mask, so the position must never change for an
 * action that has shipped. db_authorization_verify_action_catalog() checks the
 * running catalog against the authz_actions table at startup.
 */
uint64_t authorization_action_bit(authorization_action_t action);

/*
 * Union of every action reachable by a principal, ignoring resource scope.
 * Used to stop a policy manager from minting authority it does not itself
 * hold. Returns 0 on success and -1 on evaluation failure.
 */
int authorization_effective_action_mask(const user_t *user, uint64_t *mask);

/*
 * Reusable evaluation state for callers that authorize many resources in one
 * request. The context memoizes the grants loaded for a user/action, the
 * selectors parsed from those grants, the collection filters they reference,
 * and the scoped API token, none of which change while a request is running.
 * A context is owned by one thread and must not be shared between requests.
 */
typedef struct authorization_context authorization_context_t;

authorization_context_t *authorization_context_create(void);
void authorization_context_free(authorization_context_t *context);

int authorization_evaluate_in_context(authorization_context_t *context,
                                      const user_t *user,
                                      authorization_action_t action,
                                      const fleet_camera_t *camera,
                                      authorization_evaluation_t *evaluation);

/*
 * Reduce a loaded fleet inventory in place to the cameras a user may see.
 *
 * Applies the same centralized live.view decision the per-camera endpoints
 * use, so list handlers cannot disclose cameras outside a policy grant while
 * computing totals, facets, or collection membership. Legacy-mode principals
 * keep their allowed-tags behaviour because the evaluator applies it for them.
 * Uses one shared context, so the cost is one grant load for the whole page.
 *
 * Returns 0 with *count reduced, or -1 on evaluation failure. Callers must
 * fail closed rather than serving an unfiltered list.
 */
int authorization_filter_visible_cameras(const user_t *user,
                                         fleet_camera_t *cameras, int *count);

/* Generic list filtering for camera-scoped actions such as recordings.replay
 * and snapshot.create. Totals must be calculated from the reduced inventory. */
int authorization_filter_cameras(const user_t *user,
                                 authorization_action_t action,
                                 fleet_camera_t *cameras, int *count);

/*
 * Evaluate one user/action/resource tuple. A NULL camera is valid for global
 * actions and for an all-fleet grant, but never matches a selector-backed grant.
 * Scoped API tokens are intersected with the user's effective decision.
 * Returns 0 for an allow/deny decision and -1 for an evaluation failure. Callers
 * must fail closed when this function returns -1.
 */
int authorization_evaluate(const user_t *user, authorization_action_t action,
                           const fleet_camera_t *camera,
                           authorization_evaluation_t *evaluation);

#endif /* LIGHTNVR_AUTHORIZATION_H */
