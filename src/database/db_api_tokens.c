#define _POSIX_C_SOURCE 200809L

#include <cjson/cJSON.h>
#include <ctype.h>
#include <mbedtls/ctr_drbg.h>
#include <mbedtls/entropy.h>
#include <mbedtls/sha256.h>
#include <pthread.h>
#include <sqlite3.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>

#include "core/authorization.h"
#include "core/camera_selector.h"
#include "database/db_api_tokens.h"
#include "database/db_core.h"
#include "utils/memory.h"
#include "utils/strings.h"

#define TOKEN_RANDOM_BYTES 32
#define TOKEN_HASH_HEX_SIZE 65

static void copy_column(char *destination, size_t destination_size,
                        sqlite3_stmt *stmt, int column) {
    const char *value = (const char *)sqlite3_column_text(stmt, column);
    safe_strcpy(destination, value ? value : "", destination_size, 0);
}

static void populate_token(sqlite3_stmt *stmt, api_token_t *token) {
    memset(token, 0, sizeof(*token));
    copy_column(token->uuid, sizeof(token->uuid), stmt, 0);
    token->user_id = sqlite3_column_int64(stmt, 1);
    token->created_by_user_id = sqlite3_column_int64(stmt, 2);
    copy_column(token->description, sizeof(token->description), stmt, 3);
    copy_column(token->token_prefix, sizeof(token->token_prefix), stmt, 4);
    token->action_mask = (uint64_t)sqlite3_column_int64(stmt, 5);
    copy_column(token->scope_type, sizeof(token->scope_type), stmt, 6);
    copy_column(token->selector_json, sizeof(token->selector_json), stmt, 7);
    copy_column(token->collection_uuid, sizeof(token->collection_uuid), stmt, 8);
    token->expires_at = sqlite3_column_int64(stmt, 9);
    token->revoked_at = sqlite3_column_int64(stmt, 10);
    token->last_used_at = sqlite3_column_int64(stmt, 11);
    token->created_at = sqlite3_column_int64(stmt, 12);
}

static const char *token_select_fields(void) {
    return "uuid,user_id,COALESCE(created_by_user_id,0),description,"
           "token_prefix,action_mask,scope_type,COALESCE(selector_json,''),"
           "COALESCE(collection_uuid,''),expires_at,COALESCE(revoked_at,0),"
           "COALESCE(last_used_at,0),created_at";
}

static bool valid_selector(const char *serialized) {
    if (!serialized || serialized[0] == '\0' ||
        strlen(serialized) >= API_TOKEN_SELECTOR_MAX) {
        return false;
    }
    cJSON *json = cJSON_Parse(serialized);
    if (!json) return false;
    char error[FLEET_SELECTOR_ERROR_MAX] = {0};
    fleet_selector_t *selector =
        fleet_selector_parse(json, error, sizeof(error));
    cJSON_Delete(json);
    if (!selector) return false;
    fleet_selector_free(selector);
    return true;
}

static bool valid_scope(const api_token_create_t *input) {
    if (strcmp(input->scope_type, "all") == 0) {
        return !input->selector_json && !input->collection_uuid;
    }
    if (strcmp(input->scope_type, "selector") == 0) {
        return !input->collection_uuid && valid_selector(input->selector_json);
    }
    return strcmp(input->scope_type, "collection") == 0 &&
           !input->selector_json && input->collection_uuid &&
           input->collection_uuid[0] != '\0';
}

static bool shared_collection_exists_locked(sqlite3 *db, const char *uuid) {
    sqlite3_stmt *stmt = NULL;
    int rc = sqlite3_prepare_v2(
        db,
        "SELECT 1 FROM camera_collections WHERE uuid=? AND is_shared=1;",
        -1, &stmt, NULL);
    if (rc == SQLITE_OK) {
        sqlite3_bind_text(stmt, 1, uuid, -1, SQLITE_TRANSIENT);
        rc = sqlite3_step(stmt);
    }
    if (stmt) sqlite3_finalize(stmt);
    return rc == SQLITE_ROW;
}

static int bytes_to_hex(const unsigned char *bytes, size_t byte_count,
                        char *output, size_t output_size) {
    static const char alphabet[] = "0123456789abcdef";
    if (!bytes || !output || output_size < byte_count * 2 + 1) return -1;
    for (size_t i = 0; i < byte_count; i++) {
        output[i * 2] = alphabet[bytes[i] >> 4];
        output[i * 2 + 1] = alphabet[bytes[i] & 0x0f];
    }
    output[byte_count * 2] = '\0';
    return 0;
}

static int generate_secret(char secret[API_TOKEN_SECRET_MAX]) {
    mbedtls_entropy_context entropy;
    mbedtls_ctr_drbg_context random;
    mbedtls_entropy_init(&entropy);
    mbedtls_ctr_drbg_init(&random);
    static const unsigned char personalization[] = "lightnvr-api-token";
    int rc = mbedtls_ctr_drbg_seed(
        &random, mbedtls_entropy_func, &entropy, personalization,
        sizeof(personalization) - 1);
    unsigned char bytes[TOKEN_RANDOM_BYTES];
    if (rc == 0) {
        rc = mbedtls_ctr_drbg_random(&random, bytes, sizeof(bytes));
    }
    char hex[TOKEN_RANDOM_BYTES * 2 + 1];
    if (rc == 0) rc = bytes_to_hex(bytes, sizeof(bytes), hex, sizeof(hex));
    if (rc == 0) {
        int written = snprintf(secret, API_TOKEN_SECRET_MAX, "lnvr_%s", hex);
        if (written < 0 || written >= API_TOKEN_SECRET_MAX) rc = -1;
    }
    memset(bytes, 0, sizeof(bytes));
    mbedtls_ctr_drbg_free(&random);
    mbedtls_entropy_free(&entropy);
    return rc == 0 ? 0 : -1;
}

static int hash_secret(const char *secret,
                       char hash[TOKEN_HASH_HEX_SIZE]) {
    if (!secret || secret[0] == '\0') return -1;
    unsigned char digest[32];
    if (mbedtls_sha256((const unsigned char *)secret, strlen(secret), digest,
                       0) != 0) {
        secure_zero_memory(digest, sizeof(digest));
        return -1;
    }
    int rc = bytes_to_hex(digest, sizeof(digest), hash, TOKEN_HASH_HEX_SIZE);
    secure_zero_memory(digest, sizeof(digest));
    return rc;
}

db_api_token_result_t db_api_token_create(
    const api_token_create_t *input, api_token_t *token,
    char secret[API_TOKEN_SECRET_MAX]) {
    int64_t now = (int64_t)time(NULL);
    uint64_t valid_actions = (UINT64_C(1) << AUTHZ_ACTION_COUNT) - 1;
    char description[API_TOKEN_DESCRIPTION_MAX];
    if (!input || !token || !secret || input->user_id <= 0 ||
        input->created_by_user_id <= 0 || !input->description ||
        strlen(input->description) >= sizeof(description) ||
        copy_trimmed_value(description, sizeof(description),
                           input->description, 0) == 0 ||
        input->action_mask == 0 ||
        (input->action_mask & ~valid_actions) != 0 || !input->scope_type ||
        !valid_scope(input) || input->expires_at <= now ||
        input->expires_at - now > API_TOKEN_MAX_LIFETIME_SECONDS) {
        return DB_API_TOKEN_INVALID;
    }
    for (const unsigned char *cursor = (const unsigned char *)description;
         *cursor; cursor++) {
        if (iscntrl(*cursor)) return DB_API_TOKEN_INVALID;
    }
    memset(token, 0, sizeof(*token));
    secret[0] = '\0';
    if (generate_secret(secret) != 0) return DB_API_TOKEN_ERROR;
    char hash[TOKEN_HASH_HEX_SIZE];
    if (hash_secret(secret, hash) != 0) {
        secure_zero_memory(secret, API_TOKEN_SECRET_MAX);
        return DB_API_TOKEN_ERROR;
    }

    sqlite3 *db = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    if (!db || !mutex) {
        secure_zero_memory(secret, API_TOKEN_SECRET_MAX);
        return DB_API_TOKEN_ERROR;
    }
    pthread_mutex_lock(mutex);
    if (sqlite3_exec(db, "BEGIN IMMEDIATE;", NULL, NULL, NULL) != SQLITE_OK) {
        pthread_mutex_unlock(mutex);
        secure_zero_memory(secret, API_TOKEN_SECRET_MAX);
        return DB_API_TOKEN_ERROR;
    }
    sqlite3_stmt *stmt = NULL;
    int rc = sqlite3_prepare_v2(
        db,
        "SELECT count(*) FROM authz_api_tokens WHERE user_id=? AND "
        "revoked_at IS NULL AND expires_at>strftime('%s','now');",
        -1, &stmt, NULL);
    if (rc == SQLITE_OK) {
        sqlite3_bind_int64(stmt, 1, input->user_id);
        rc = sqlite3_step(stmt);
    }
    int active_count = rc == SQLITE_ROW ? sqlite3_column_int(stmt, 0) : -1;
    if (stmt) sqlite3_finalize(stmt);
    if (active_count >= API_TOKEN_MAX_PER_USER) {
        sqlite3_exec(db, "ROLLBACK;", NULL, NULL, NULL);
        pthread_mutex_unlock(mutex);
        secure_zero_memory(secret, API_TOKEN_SECRET_MAX);
        return DB_API_TOKEN_LIMIT;
    }
    if (active_count < 0 ||
        (strcmp(input->scope_type, "collection") == 0 &&
         !shared_collection_exists_locked(db, input->collection_uuid))) {
        sqlite3_exec(db, "ROLLBACK;", NULL, NULL, NULL);
        pthread_mutex_unlock(mutex);
        secure_zero_memory(secret, API_TOKEN_SECRET_MAX);
        return active_count < 0 ? DB_API_TOKEN_ERROR : DB_API_TOKEN_INVALID;
    }

    const char *sql =
        "INSERT INTO authz_api_tokens "
        "(uuid,user_id,created_by_user_id,description,token_prefix,token_hash,"
        "action_mask,scope_type,selector_json,collection_uuid,expires_at) "
        "VALUES (lower(hex(randomblob(4))||'-'||hex(randomblob(2))||'-4'||"
        "substr(hex(randomblob(2)),2)||'-'||"
        "substr('89ab',(abs(random())%4)+1,1)||substr(hex(randomblob(2)),2)||"
        "'-'||hex(randomblob(6))),?,?,?,?,?,?,?,?,?,?);";
    stmt = NULL;
    rc = sqlite3_prepare_v2(db, sql, -1, &stmt, NULL);
    if (rc == SQLITE_OK) {
        sqlite3_bind_int64(stmt, 1, input->user_id);
        sqlite3_bind_int64(stmt, 2, input->created_by_user_id);
        sqlite3_bind_text(stmt, 3, description, -1, SQLITE_TRANSIENT);
        sqlite3_bind_text(stmt, 4, secret, 13, SQLITE_TRANSIENT);
        sqlite3_bind_text(stmt, 5, hash, -1, SQLITE_TRANSIENT);
        sqlite3_bind_int64(stmt, 6, (sqlite3_int64)input->action_mask);
        sqlite3_bind_text(stmt, 7, input->scope_type, -1, SQLITE_TRANSIENT);
        if (input->selector_json) {
            sqlite3_bind_text(stmt, 8, input->selector_json, -1,
                              SQLITE_TRANSIENT);
        } else {
            sqlite3_bind_null(stmt, 8);
        }
        if (input->collection_uuid) {
            sqlite3_bind_text(stmt, 9, input->collection_uuid, -1,
                              SQLITE_TRANSIENT);
        } else {
            sqlite3_bind_null(stmt, 9);
        }
        sqlite3_bind_int64(stmt, 10, input->expires_at);
        rc = sqlite3_step(stmt);
    }
    if (stmt) sqlite3_finalize(stmt);
    if (rc == SQLITE_DONE) {
        char query[512];
        snprintf(query, sizeof(query),
                 "SELECT %s FROM authz_api_tokens "
                 "WHERE rowid=last_insert_rowid();",
                 token_select_fields());
        stmt = NULL;
        rc = sqlite3_prepare_v2(db, query, -1, &stmt, NULL);
        if (rc == SQLITE_OK && sqlite3_step(stmt) == SQLITE_ROW) {
            populate_token(stmt, token);
            rc = SQLITE_DONE;
        }
        if (stmt) sqlite3_finalize(stmt);
    }
    bool committed = rc == SQLITE_DONE && token->uuid[0] != '\0' &&
        sqlite3_exec(db, "COMMIT;", NULL, NULL, NULL) == SQLITE_OK;
    if (!committed) sqlite3_exec(db, "ROLLBACK;", NULL, NULL, NULL);
    pthread_mutex_unlock(mutex);
    secure_zero_memory(hash, sizeof(hash));
    if (!committed) {
        secure_zero_memory(secret, API_TOKEN_SECRET_MAX);
        return DB_API_TOKEN_ERROR;
    }
    return DB_API_TOKEN_OK;
}

db_api_token_result_t db_api_token_list(int64_t user_id,
                                        api_token_t **tokens,
                                        int *token_count) {
    if (user_id <= 0 || !tokens || !token_count) return DB_API_TOKEN_INVALID;
    *tokens = NULL;
    *token_count = 0;
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    if (!db || !mutex) return DB_API_TOKEN_ERROR;
    pthread_mutex_lock(mutex);
    char sql[512];
    snprintf(sql, sizeof(sql),
             "SELECT %s FROM authz_api_tokens WHERE user_id=? "
             "ORDER BY created_at DESC,uuid LIMIT ?;",
             token_select_fields());
    sqlite3_stmt *stmt = NULL;
    int rc = sqlite3_prepare_v2(db, sql, -1, &stmt, NULL);
    if (rc == SQLITE_OK) {
        sqlite3_bind_int64(stmt, 1, user_id);
        sqlite3_bind_int(stmt, 2, API_TOKEN_MAX_PER_USER);
    }
    int capacity = 8;
    api_token_t *loaded = rc == SQLITE_OK
        ? calloc((size_t)capacity, sizeof(*loaded)) : NULL;
    if (rc == SQLITE_OK && !loaded) rc = SQLITE_NOMEM;
    int count = 0;
    if (rc == SQLITE_OK) {
        while ((rc = sqlite3_step(stmt)) == SQLITE_ROW) {
            if (count == capacity) {
                capacity *= 2;
                api_token_t *resized =
                    realloc(loaded, (size_t)capacity * sizeof(*loaded));
                if (!resized) {
                    rc = SQLITE_NOMEM;
                    break;
                }
                loaded = resized;
            }
            populate_token(stmt, &loaded[count++]);
        }
    }
    if (stmt) sqlite3_finalize(stmt);
    pthread_mutex_unlock(mutex);
    if (rc != SQLITE_DONE) {
        free(loaded);
        return DB_API_TOKEN_ERROR;
    }
    if (count == 0) {
        free(loaded);
        loaded = NULL;
    }
    *tokens = loaded;
    *token_count = count;
    return DB_API_TOKEN_OK;
}

db_api_token_result_t db_api_token_revoke(int64_t user_id,
                                          const char *token_uuid) {
    if (user_id <= 0 || !token_uuid || token_uuid[0] == '\0') {
        return DB_API_TOKEN_INVALID;
    }
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    if (!db || !mutex) return DB_API_TOKEN_ERROR;
    pthread_mutex_lock(mutex);
    sqlite3_stmt *stmt = NULL;
    int rc = sqlite3_prepare_v2(
        db,
        "UPDATE authz_api_tokens SET revoked_at=strftime('%s','now') "
        "WHERE uuid=? AND user_id=? AND revoked_at IS NULL;",
        -1, &stmt, NULL);
    if (rc == SQLITE_OK) {
        sqlite3_bind_text(stmt, 1, token_uuid, -1, SQLITE_TRANSIENT);
        sqlite3_bind_int64(stmt, 2, user_id);
        rc = sqlite3_step(stmt);
    }
    int changes = rc == SQLITE_DONE ? sqlite3_changes(db) : 0;
    if (stmt) sqlite3_finalize(stmt);
    pthread_mutex_unlock(mutex);
    if (rc != SQLITE_DONE) return DB_API_TOKEN_ERROR;
    return changes == 1 ? DB_API_TOKEN_OK : DB_API_TOKEN_NOT_FOUND;
}

db_api_token_result_t db_api_token_authenticate(
    const char *secret, int64_t *user_id,
    char token_uuid[CAMERA_UUID_STRING_SIZE]) {
    if (!secret || strlen(secret) != 5 + TOKEN_RANDOM_BYTES * 2 ||
        strncmp(secret, "lnvr_", 5) != 0 || !user_id || !token_uuid) {
        return DB_API_TOKEN_NOT_FOUND;
    }
    *user_id = 0;
    token_uuid[0] = '\0';
    char hash[TOKEN_HASH_HEX_SIZE];
    if (hash_secret(secret, hash) != 0) return DB_API_TOKEN_ERROR;
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    if (!db || !mutex) return DB_API_TOKEN_ERROR;
    pthread_mutex_lock(mutex);
    sqlite3_stmt *stmt = NULL;
    int rc = sqlite3_prepare_v2(
        db,
        "SELECT t.uuid,t.user_id FROM authz_api_tokens t "
        "JOIN users u ON u.id=t.user_id "
        "WHERE t.token_hash=? AND t.revoked_at IS NULL "
        "AND t.expires_at>strftime('%s','now') AND u.is_active=1 LIMIT 1;",
        -1, &stmt, NULL);
    if (rc == SQLITE_OK) {
        sqlite3_bind_text(stmt, 1, hash, -1, SQLITE_TRANSIENT);
        rc = sqlite3_step(stmt);
    }
    if (rc == SQLITE_ROW) {
        copy_column(token_uuid, CAMERA_UUID_STRING_SIZE, stmt, 0);
        *user_id = sqlite3_column_int64(stmt, 1);
    }
    if (stmt) sqlite3_finalize(stmt);
    if (*user_id > 0) {
        rc = sqlite3_prepare_v2(
            db,
            "UPDATE authz_api_tokens SET last_used_at=strftime('%s','now') "
            "WHERE uuid=? AND (last_used_at IS NULL OR "
            "last_used_at<strftime('%s','now')-60);",
            -1, &stmt, NULL);
        if (rc == SQLITE_OK) {
            sqlite3_bind_text(stmt, 1, token_uuid, -1, SQLITE_TRANSIENT);
            rc = sqlite3_step(stmt);
        }
        if (stmt) sqlite3_finalize(stmt);
    }
    pthread_mutex_unlock(mutex);
    secure_zero_memory(hash, sizeof(hash));
    if (*user_id <= 0) return DB_API_TOKEN_NOT_FOUND;
    return rc == SQLITE_DONE ? DB_API_TOKEN_OK : DB_API_TOKEN_ERROR;
}

db_api_token_result_t db_api_token_get_active(const char *token_uuid,
                                              api_token_t *token) {
    if (!token_uuid || token_uuid[0] == '\0' || !token) {
        return DB_API_TOKEN_INVALID;
    }
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    if (!db || !mutex) return DB_API_TOKEN_ERROR;
    pthread_mutex_lock(mutex);
    char sql[640];
    snprintf(sql, sizeof(sql),
             "SELECT %s FROM authz_api_tokens WHERE uuid=? AND "
             "revoked_at IS NULL AND expires_at>strftime('%%s','now') LIMIT 1;",
             token_select_fields());
    sqlite3_stmt *stmt = NULL;
    int rc = sqlite3_prepare_v2(db, sql, -1, &stmt, NULL);
    if (rc == SQLITE_OK) {
        sqlite3_bind_text(stmt, 1, token_uuid, -1, SQLITE_TRANSIENT);
        rc = sqlite3_step(stmt);
    }
    db_api_token_result_t result = DB_API_TOKEN_NOT_FOUND;
    if (rc == SQLITE_ROW) {
        populate_token(stmt, token);
        result = DB_API_TOKEN_OK;
    } else if (rc != SQLITE_DONE) {
        result = DB_API_TOKEN_ERROR;
    }
    if (stmt) sqlite3_finalize(stmt);
    pthread_mutex_unlock(mutex);
    return result;
}
