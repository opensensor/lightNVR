#ifndef LIGHTNVR_DB_AUTHORIZATION_H
#define LIGHTNVR_DB_AUTHORIZATION_H

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>

#include "core/config.h"
#include "database/db_auth.h"

#define AUTHORIZATION_ROLE_NAME_MAX 128
#define AUTHORIZATION_SCOPE_TYPE_MAX 16
#define AUTHORIZATION_SELECTOR_MAX 8192
#define AUTHORIZATION_MAX_USER_GRANTS 256
#define AUTHORIZATION_ROLE_DESCRIPTION_MAX 256

typedef enum {
    DB_AUTHORIZATION_OK = 0,
    DB_AUTHORIZATION_ERROR = -1,
    DB_AUTHORIZATION_NOT_FOUND = -2,
    DB_AUTHORIZATION_CONFLICT = -3,
    DB_AUTHORIZATION_IMMUTABLE = -4,
    DB_AUTHORIZATION_IN_USE = -5,
    DB_AUTHORIZATION_STALE = -6,
    DB_AUTHORIZATION_INVALID = -7
} db_authorization_result_t;

typedef struct {
    char uuid[CAMERA_UUID_STRING_SIZE];
    char name[AUTHORIZATION_ROLE_NAME_MAX];
    char description[AUTHORIZATION_ROLE_DESCRIPTION_MAX];
    bool is_builtin;
    uint64_t action_mask;
    int64_t created_at;
    int64_t updated_at;
} authorization_role_t;

typedef struct {
    char uuid[CAMERA_UUID_STRING_SIZE];
    int64_t user_id;
    char role_uuid[CAMERA_UUID_STRING_SIZE];
    char role_name[AUTHORIZATION_ROLE_NAME_MAX];
    char scope_type[AUTHORIZATION_SCOPE_TYPE_MAX];
    char selector_json[AUTHORIZATION_SELECTOR_MAX];
    char collection_uuid[CAMERA_UUID_STRING_SIZE];
    bool enabled;
    int64_t created_at;
    int64_t updated_at;
} authorization_grant_t;

typedef struct {
    char role_uuid[CAMERA_UUID_STRING_SIZE];
    char scope_type[AUTHORIZATION_SCOPE_TYPE_MAX];
    char selector_json[AUTHORIZATION_SELECTOR_MAX];
    char collection_uuid[CAMERA_UUID_STRING_SIZE];
} authorization_grant_input_t;

int db_authorization_role_count(void);
int db_authorization_role_list(authorization_role_t *roles, int max_count);
db_authorization_result_t db_authorization_load_roles(
    authorization_role_t **roles, int *role_count, int64_t *policy_version);
db_authorization_result_t db_authorization_role_get(
    const char *uuid, authorization_role_t *role);
db_authorization_result_t db_authorization_role_create(
    authorization_role_t *role, int64_t expected_version,
    int64_t *new_version);
db_authorization_result_t db_authorization_role_update(
    const authorization_role_t *role, int64_t expected_version,
    int64_t *new_version);
db_authorization_result_t db_authorization_role_delete(
    const char *uuid, int64_t expected_version, int64_t *new_version);

/* Load all grants for a user, independent of the action evaluator. */
db_authorization_result_t db_authorization_get_user_policy(
    int64_t user_id, char mode[USER_AUTHORIZATION_MODE_MAX],
    authorization_grant_t **grants, int *grant_count,
    int64_t *policy_version);

/* Atomically replace grants and mode if expected_version is still current. */
db_authorization_result_t db_authorization_replace_user_policy(
    int64_t user_id, const char *mode,
    const authorization_grant_input_t *grants, int grant_count,
    int64_t expected_version, int64_t *new_version);

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
    const char *selector_json, const char *collection_uuid,
    char grant_uuid[CAMERA_UUID_STRING_SIZE]);

/* Switch a user only after valid grants have been created and previewed. */
int db_authorization_set_user_mode(int64_t user_id, const char *mode);

#endif /* LIGHTNVR_DB_AUTHORIZATION_H */
