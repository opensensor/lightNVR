#ifndef LIGHTNVR_DB_AUTHORIZATION_H
#define LIGHTNVR_DB_AUTHORIZATION_H

#include <stdint.h>

#include "core/config.h"

#define AUTHORIZATION_ROLE_NAME_MAX 128
#define AUTHORIZATION_SCOPE_TYPE_MAX 16
#define AUTHORIZATION_SELECTOR_MAX 8192
#define AUTHORIZATION_MAX_USER_GRANTS 256

typedef struct {
    char uuid[CAMERA_UUID_STRING_SIZE];
    char role_uuid[CAMERA_UUID_STRING_SIZE];
    char role_name[AUTHORIZATION_ROLE_NAME_MAX];
    char scope_type[AUTHORIZATION_SCOPE_TYPE_MAX];
    char selector_json[AUTHORIZATION_SELECTOR_MAX];
} authorization_grant_t;

/* Load enabled grants whose role contains action_key. The caller owns *grants. */
int db_authorization_load_user_grants(int64_t user_id, const char *action_key,
                                      authorization_grant_t **grants,
                                      int *grant_count,
                                      int64_t *policy_version);

/* Monotonic version incremented by supported policy-table changes. */
int db_authorization_get_policy_version(int64_t *version);

/* Create a validated user grant and bump the policy version atomically. */
int db_authorization_create_user_grant(
    int64_t user_id, const char *role_uuid, const char *scope_type,
    const char *selector_json,
    char grant_uuid[CAMERA_UUID_STRING_SIZE]);

/* Switch a user only after valid grants have been created and previewed. */
int db_authorization_set_user_mode(int64_t user_id, const char *mode);

#endif /* LIGHTNVR_DB_AUTHORIZATION_H */
