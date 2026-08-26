#define _POSIX_C_SOURCE 200809L

#include "web/api_handlers_storage_pools.h"

#include <cjson/cJSON.h>
#include <errno.h>
#include <math.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "core/authorization.h"
#include "database/db_storage_pools.h"
#include "utils/strings.h"
#include "utils/uuid.h"
#include "web/audit_log.h"
#include "web/httpd_utils.h"

static bool authorize_storage(const http_request_t *req,
                              http_response_t *res, user_t *user) {
    authorization_evaluation_t evaluation;
    return httpd_authorize_action(req, res, AUTHZ_STORAGE_CONFIGURE, NULL,
                                  user, &evaluation) != 0;
}

static void audit_pool(const http_request_t *req, const user_t *user,
                       const char *uuid, const char *operation,
                       const char *outcome) {
    audit_log_operation(req, user, "storage.configure", "storage_pool",
                        uuid, operation, outcome, NULL);
}

static cJSON *pool_to_json(const storage_pool_t *pool) {
    cJSON *object = cJSON_CreateObject();
    cJSON *members = cJSON_CreateArray();
    if (!object || !members) {
        cJSON_Delete(object);
        cJSON_Delete(members);
        return NULL;
    }
    cJSON_AddStringToObject(object, "uuid", pool->uuid);
    cJSON_AddStringToObject(object, "name", pool->name);
    cJSON_AddStringToObject(object, "strategy", pool->strategy);
    cJSON_AddBoolToObject(object, "enabled", pool->enabled);
    cJSON_AddNumberToObject(object, "revision", (double)pool->revision);
    cJSON_AddNumberToObject(object, "created_at", (double)pool->created_at);
    cJSON_AddNumberToObject(object, "updated_at", (double)pool->updated_at);
    cJSON_AddItemToObject(object, "members", members);
    for (int index = 0; index < pool->member_count; index++) {
        cJSON *member = cJSON_CreateObject();
        if (!member) {
            cJSON_Delete(object);
            return NULL;
        }
        cJSON_AddStringToObject(member, "target_uuid",
                               pool->members[index].target_uuid);
        cJSON_AddNumberToObject(member, "position", index);
        cJSON_AddNumberToObject(member, "weight",
                                pool->members[index].weight);
        cJSON_AddItemToArray(members, member);
    }
    return object;
}

static bool send_json(http_response_t *res, int status, cJSON *object) {
    char *body = object ? cJSON_PrintUnformatted(object) : NULL;
    cJSON_Delete(object);
    if (!body) {
        http_response_set_json_error(res, 500,
                                     "Failed to serialize storage pool response");
        return false;
    }
    http_response_set_json(res, status, body);
    free(body);
    return true;
}

static void set_db_error(http_response_t *res, db_storage_pool_result_t result,
                         const char *validation_error) {
    switch (result) {
        case DB_STORAGE_POOL_NOT_FOUND:
            http_response_set_json_error(res, 404, "Storage pool not found");
            break;
        case DB_STORAGE_POOL_CONFLICT:
            http_response_set_json_error(res, 409,
                                         "A storage pool already uses that name");
            break;
        case DB_STORAGE_POOL_STALE:
            http_response_set_json_error(res, 409,
                "Storage pool was changed by another administrator");
            break;
        case DB_STORAGE_POOL_IN_USE:
            http_response_set_json_error(res, 409,
                                         "Storage pool is used by a policy");
            break;
        case DB_STORAGE_POOL_LIMIT:
            http_response_set_json_error(res, 409, "Storage pool limit reached");
            break;
        case DB_STORAGE_POOL_INVALID:
            http_response_set_json_error(res, 400,
                validation_error && validation_error[0]
                    ? validation_error : "Invalid storage pool");
            break;
        default:
            http_response_set_json_error(res, 500,
                                         "Storage pool operation failed");
            break;
    }
}

static bool extract_uuid(const http_request_t *req,
                         char uuid[LIGHTNVR_UUID_STRING_SIZE],
                         http_response_t *res) {
    char value[MAX_PATH_LENGTH];
    if (http_request_extract_path_param(req, "/api/storage-pools/", value,
                                        sizeof(value)) != 0) {
        http_response_set_json_error(res, 400, "Invalid storage pool path");
        return false;
    }
    char *slash = strchr(value, '/');
    if (slash) *slash = '\0';
    if (!lightnvr_uuid_is_valid(value)) {
        http_response_set_json_error(res, 400, "Invalid storage pool UUID");
        return false;
    }
    safe_strcpy(uuid, value, LIGHTNVR_UUID_STRING_SIZE, 0);
    return true;
}

static bool apply_body(const cJSON *body, storage_pool_t *pool, bool create,
                       int64_t *revision, char *error, size_t error_size,
                       http_response_t *res) {
    if (!cJSON_IsObject(body)) {
        http_response_set_json_error(res, 400,
                                     "Request body must be a JSON object");
        return false;
    }
    const cJSON *name = cJSON_GetObjectItemCaseSensitive(body, "name");
    const cJSON *strategy = cJSON_GetObjectItemCaseSensitive(body, "strategy");
    const cJSON *enabled = cJSON_GetObjectItemCaseSensitive(body, "enabled");
    const cJSON *members = cJSON_GetObjectItemCaseSensitive(body, "members");
    if ((create || name) && (!cJSON_IsString(name) || !name->valuestring ||
        strnlen(name->valuestring, sizeof(pool->name)) >= sizeof(pool->name))) {
        http_response_set_json_error(res, 400, "name must be a valid string");
        return false;
    }
    if (name) safe_strcpy(pool->name, name->valuestring, sizeof(pool->name), 0);
    if (strategy) {
        if (!cJSON_IsString(strategy) || !strategy->valuestring ||
            strnlen(strategy->valuestring, sizeof(pool->strategy)) >=
                sizeof(pool->strategy)) {
            http_response_set_json_error(res, 400,
                                         "strategy must be a valid string");
            return false;
        }
        safe_strcpy(pool->strategy, strategy->valuestring,
                    sizeof(pool->strategy), 0);
    }
    if (enabled) {
        if (!cJSON_IsBool(enabled)) {
            http_response_set_json_error(res, 400, "enabled must be boolean");
            return false;
        }
        pool->enabled = cJSON_IsTrue(enabled);
    }
    if (create || members) {
        if (!cJSON_IsArray(members) || cJSON_GetArraySize(members) < 1 ||
            cJSON_GetArraySize(members) > STORAGE_POOL_MEMBER_MAX) {
            http_response_set_json_error(res, 400,
                                         "members must be a non-empty array");
            return false;
        }
        pool->member_count = 0;
        const cJSON *item = NULL;
        cJSON_ArrayForEach(item, members) {
            const cJSON *target_uuid = cJSON_GetObjectItemCaseSensitive(
                item, "target_uuid");
            const cJSON *weight = cJSON_GetObjectItemCaseSensitive(item, "weight");
            if (!cJSON_IsObject(item) || !cJSON_IsString(target_uuid) ||
                !target_uuid->valuestring ||
                !lightnvr_uuid_is_valid(target_uuid->valuestring) ||
                (weight && (!cJSON_IsNumber(weight) ||
                 floor(weight->valuedouble) != weight->valuedouble ||
                 weight->valueint < 1 || weight->valueint > 1000))) {
                http_response_set_json_error(res, 400, "Invalid pool member");
                return false;
            }
            storage_pool_member_t *member =
                &pool->members[pool->member_count++];
            safe_strcpy(member->target_uuid, target_uuid->valuestring,
                        sizeof(member->target_uuid), 0);
            member->position = pool->member_count - 1;
            member->weight = weight ? weight->valueint : 1;
        }
    }
    if (!create) {
        const cJSON *revision_item = cJSON_GetObjectItemCaseSensitive(
            body, "revision");
        if (!cJSON_IsNumber(revision_item) || revision_item->valuedouble < 1 ||
            floor(revision_item->valuedouble) != revision_item->valuedouble) {
            http_response_set_json_error(res, 400,
                                         "revision must be a positive integer");
            return false;
        }
        *revision = (int64_t)revision_item->valuedouble;
    }
    db_storage_pool_result_t validated = db_storage_pool_validate(
        pool, error, error_size);
    if (validated != DB_STORAGE_POOL_OK) {
        set_db_error(res, validated, error);
        return false;
    }
    return true;
}

void handle_get_storage_pools(const http_request_t *req, http_response_t *res) {
    user_t user;
    if (!authorize_storage(req, res, &user)) return;
    int total = db_storage_pool_count();
    storage_pool_t *pools = total > 0 && total <= STORAGE_POOL_MAX_COUNT
        ? calloc((size_t)total, sizeof(*pools)) : NULL;
    int count = total < 0 || total > STORAGE_POOL_MAX_COUNT ||
                (total > 0 && !pools)
        ? -1 : (total > 0 ? db_storage_pool_list(pools, total) : 0);
    cJSON *root = cJSON_CreateObject();
    cJSON *items = cJSON_CreateArray();
    if (count < 0 || !root || !items) {
        free(pools);
        cJSON_Delete(root);
        cJSON_Delete(items);
        set_db_error(res, DB_STORAGE_POOL_ERROR, NULL);
        return;
    }
    cJSON_AddNumberToObject(root, "count", count);
    cJSON_AddItemToObject(root, "pools", items);
    for (int index = 0; index < count; index++) {
        cJSON *item = pool_to_json(&pools[index]);
        if (!item) {
            free(pools);
            cJSON_Delete(root);
            set_db_error(res, DB_STORAGE_POOL_ERROR, NULL);
            return;
        }
        cJSON_AddItemToArray(items, item);
    }
    free(pools);
    send_json(res, 200, root);
}

void handle_post_storage_pool(const http_request_t *req, http_response_t *res) {
    user_t user;
    if (!authorize_storage(req, res, &user)) return;
    storage_pool_t pool;
    memset(&pool, 0, sizeof(pool));
    pool.enabled = true;
    safe_strcpy(pool.strategy, "most_free", sizeof(pool.strategy), 0);
    cJSON *body = httpd_parse_json_body(req);
    int64_t revision = 0;
    char error[STORAGE_TARGET_ERROR_MAX] = {0};
    if (!apply_body(body, &pool, true, &revision, error, sizeof(error), res)) {
        cJSON_Delete(body);
        audit_pool(req, &user, NULL, "pool_create", "failure");
        return;
    }
    cJSON_Delete(body);
    db_storage_pool_result_t result = db_storage_pool_create(&pool);
    if (result != DB_STORAGE_POOL_OK) {
        set_db_error(res, result, error);
        audit_pool(req, &user, NULL, "pool_create", "failure");
        return;
    }
    audit_pool(req, &user, pool.uuid, "pool_create", "success");
    send_json(res, 201, pool_to_json(&pool));
}

void handle_get_storage_pool(const http_request_t *req, http_response_t *res) {
    user_t user;
    char uuid[LIGHTNVR_UUID_STRING_SIZE];
    if (!authorize_storage(req, res, &user) || !extract_uuid(req, uuid, res)) return;
    storage_pool_t pool;
    db_storage_pool_result_t result = db_storage_pool_get(uuid, &pool);
    if (result != DB_STORAGE_POOL_OK) {
        set_db_error(res, result, NULL);
        return;
    }
    send_json(res, 200, pool_to_json(&pool));
}

void handle_put_storage_pool(const http_request_t *req, http_response_t *res) {
    user_t user;
    char uuid[LIGHTNVR_UUID_STRING_SIZE];
    if (!authorize_storage(req, res, &user) || !extract_uuid(req, uuid, res)) return;
    storage_pool_t pool;
    db_storage_pool_result_t result = db_storage_pool_get(uuid, &pool);
    if (result != DB_STORAGE_POOL_OK) {
        set_db_error(res, result, NULL);
        return;
    }
    cJSON *body = httpd_parse_json_body(req);
    int64_t revision = 0;
    char error[STORAGE_TARGET_ERROR_MAX] = {0};
    if (!apply_body(body, &pool, false, &revision, error, sizeof(error), res)) {
        cJSON_Delete(body);
        audit_pool(req, &user, uuid, "pool_update", "failure");
        return;
    }
    cJSON_Delete(body);
    result = db_storage_pool_update(&pool, revision);
    if (result != DB_STORAGE_POOL_OK) {
        set_db_error(res, result, error);
        audit_pool(req, &user, uuid, "pool_update", "failure");
        return;
    }
    audit_pool(req, &user, uuid, "pool_update", "success");
    send_json(res, 200, pool_to_json(&pool));
}

void handle_delete_storage_pool(const http_request_t *req,
                                http_response_t *res) {
    user_t user;
    char uuid[LIGHTNVR_UUID_STRING_SIZE];
    if (!authorize_storage(req, res, &user) || !extract_uuid(req, uuid, res)) return;
    char value[64];
    if (http_request_get_query_param(req, "revision", value, sizeof(value)) < 0) {
        http_response_set_json_error(res, 400,
                                     "revision query parameter is required");
        return;
    }
    errno = 0;
    char *end = NULL;
    long long revision = strtoll(value, &end, 10);
    if (errno == ERANGE || !end || *end != '\0' || revision < 1) {
        http_response_set_json_error(res, 400,
                                     "revision must be a positive integer");
        return;
    }
    db_storage_pool_result_t result = db_storage_pool_delete(uuid, revision);
    if (result != DB_STORAGE_POOL_OK) {
        set_db_error(res, result, NULL);
        audit_pool(req, &user, uuid, "pool_delete", "failure");
        return;
    }
    audit_pool(req, &user, uuid, "pool_delete", "success");
    http_response_set_json(res, 200, "{\"success\":true}");
}
