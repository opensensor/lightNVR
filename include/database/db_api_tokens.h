#ifndef LIGHTNVR_DB_API_TOKENS_H
#define LIGHTNVR_DB_API_TOKENS_H

#include <stdbool.h>
#include <stdint.h>

#include "core/config.h"

#define API_TOKEN_DESCRIPTION_MAX 128
#define API_TOKEN_PREFIX_MAX 16
#define API_TOKEN_SECRET_MAX 80
#define API_TOKEN_SCOPE_TYPE_MAX 16
#define API_TOKEN_SELECTOR_MAX 8192
#define API_TOKEN_MAX_PER_USER 256
#define API_TOKEN_MAX_LIFETIME_SECONDS (INT64_C(366) * 24 * 60 * 60)

typedef enum {
    DB_API_TOKEN_OK = 0,
    DB_API_TOKEN_ERROR = -1,
    DB_API_TOKEN_NOT_FOUND = -2,
    DB_API_TOKEN_INVALID = -3,
    DB_API_TOKEN_LIMIT = -4,
    DB_API_TOKEN_EXPIRED = -5,
    DB_API_TOKEN_REVOKED = -6,
    DB_API_TOKEN_INACTIVE_OWNER = -7
} db_api_token_result_t;

typedef struct {
    char uuid[CAMERA_UUID_STRING_SIZE];
    int64_t user_id;
    int64_t created_by_user_id;
    char description[API_TOKEN_DESCRIPTION_MAX];
    char token_prefix[API_TOKEN_PREFIX_MAX];
    uint64_t action_mask;
    char scope_type[API_TOKEN_SCOPE_TYPE_MAX];
    char selector_json[API_TOKEN_SELECTOR_MAX];
    char collection_uuid[CAMERA_UUID_STRING_SIZE];
    int64_t expires_at;
    int64_t revoked_at;
    int64_t last_used_at;
    int64_t created_at;
} api_token_t;

typedef struct {
    int64_t user_id;
    int64_t created_by_user_id;
    const char *description;
    uint64_t action_mask;
    const char *scope_type;
    const char *selector_json;
    const char *collection_uuid;
    int64_t expires_at;
} api_token_create_t;

db_api_token_result_t db_api_token_create(
    const api_token_create_t *input, api_token_t *token,
    char secret[API_TOKEN_SECRET_MAX]);
db_api_token_result_t db_api_token_list(int64_t user_id,
                                        api_token_t **tokens,
                                        int *token_count);
db_api_token_result_t db_api_token_revoke(int64_t user_id,
                                          const char *token_uuid);

/* Resolve a secret to an active token owner. The secret is never persisted. */
db_api_token_result_t db_api_token_authenticate(
    const char *secret, int64_t *user_id,
    char token_uuid[CAMERA_UUID_STRING_SIZE], bool *usage_audit_due);

/* Reload one active token so authorization observes revocation and expiry. */
db_api_token_result_t db_api_token_get_active(const char *token_uuid,
                                              api_token_t *token);

#endif /* LIGHTNVR_DB_API_TOKENS_H */
