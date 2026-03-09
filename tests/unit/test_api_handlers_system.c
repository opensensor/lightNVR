/**
 * @file test_api_handlers_system.c
 * @brief Layer 2 Unity tests for web/api_handlers_system.c
 */

#define _POSIX_C_SOURCE 200809L

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <sys/stat.h>

#include <cjson/cJSON.h>

#include "unity.h"
#include "core/config.h"
#include "core/logger.h"
#include "database/db_core.h"
#include "web/api_handlers_system.h"
#include "web/request_response.h"

extern config_t g_config;

static char g_tmp_root[256];
static char g_db_path[320];
static char g_storage_path[320];

static cJSON *parse_response_json(const http_response_t *res) {
    TEST_ASSERT_NOT_NULL(res);
    TEST_ASSERT_NOT_NULL(res->body);
    cJSON *json = cJSON_Parse((const char *)res->body);
    TEST_ASSERT_NOT_NULL(json);
    return json;
}

static cJSON *find_version_item(cJSON *items, const char *name) {
    cJSON *item = NULL;
    cJSON_ArrayForEach(item, items) {
        cJSON *item_name = cJSON_GetObjectItemCaseSensitive(item, "name");
        if (cJSON_IsString(item_name) && strcmp(item_name->valuestring, name) == 0) {
            return item;
        }
    }
    return NULL;
}

void setUp(void) {
    g_config.web_auth_enabled = false;
}

void tearDown(void) {}

void test_handle_get_system_info_includes_versions_summary(void) {
    http_request_t req;
    http_response_t res;
    http_request_init(&req);
    http_response_init(&res);

    handle_get_system_info(&req, &res);

    TEST_ASSERT_EQUAL_INT(200, res.status_code);

    cJSON *root = parse_response_json(&res);
    cJSON *versions = cJSON_GetObjectItemCaseSensitive(root, "versions");
    cJSON *items = cJSON_GetObjectItemCaseSensitive(versions, "items");

    TEST_ASSERT_TRUE(cJSON_IsObject(versions));
    TEST_ASSERT_TRUE(cJSON_IsArray(items));
    TEST_ASSERT_GREATER_OR_EQUAL_INT(10, cJSON_GetArraySize(items));
    TEST_ASSERT_NOT_NULL(find_version_item(items, "LightNVR"));
    TEST_ASSERT_NOT_NULL(find_version_item(items, "Base OS"));
    TEST_ASSERT_NOT_NULL(find_version_item(items, "SQLite"));
    TEST_ASSERT_NOT_NULL(find_version_item(items, "libcurl"));
    TEST_ASSERT_NOT_NULL(find_version_item(items, "mbedTLS"));
    TEST_ASSERT_NOT_NULL(find_version_item(items, "libuv"));
    TEST_ASSERT_NOT_NULL(find_version_item(items, "llhttp"));
    TEST_ASSERT_NOT_NULL(find_version_item(items, "libavformat"));

    cJSON_Delete(root);
    http_response_free(&res);
}

void test_handle_get_system_info_includes_empty_stream_storage_array(void) {
    http_request_t req;
    http_response_t res;
    http_request_init(&req);
    http_response_init(&res);

    handle_get_system_info(&req, &res);

    TEST_ASSERT_EQUAL_INT(200, res.status_code);

    cJSON *root = parse_response_json(&res);
    cJSON *stream_storage = cJSON_GetObjectItemCaseSensitive(root, "streamStorage");

    TEST_ASSERT_TRUE(cJSON_IsArray(stream_storage));
    TEST_ASSERT_EQUAL_INT(0, cJSON_GetArraySize(stream_storage));

    cJSON_Delete(root);
    http_response_free(&res);
}

int main(void) {
    init_logger();
    load_default_config(&g_config);

    snprintf(g_tmp_root, sizeof(g_tmp_root), "/tmp/lightnvr_system_handler_%d", (int)getpid());
    snprintf(g_db_path, sizeof(g_db_path), "%s/lightnvr.db", g_tmp_root);
    snprintf(g_storage_path, sizeof(g_storage_path), "%s/storage", g_tmp_root);

    mkdir(g_tmp_root, 0755);
    mkdir(g_storage_path, 0755);
    strncpy(g_config.storage_path, g_storage_path, sizeof(g_config.storage_path) - 1);

    if (init_database(g_db_path) != 0) {
        fprintf(stderr, "FATAL: init_database failed\n");
        return 1;
    }

    UNITY_BEGIN();
    RUN_TEST(test_handle_get_system_info_includes_versions_summary);
    RUN_TEST(test_handle_get_system_info_includes_empty_stream_storage_array);
    int result = UNITY_END();

    shutdown_database();
    unlink(g_db_path);
    snprintf(g_db_path, sizeof(g_db_path), "%s/lightnvr.db-wal", g_tmp_root);
    unlink(g_db_path);
    snprintf(g_db_path, sizeof(g_db_path), "%s/lightnvr.db-shm", g_tmp_root);
    unlink(g_db_path);
    snprintf(g_db_path, sizeof(g_db_path), "%s/lightnvr.db.bak", g_tmp_root);
    unlink(g_db_path);
    free(g_config.streams);
    g_config.streams = NULL;
    rmdir(g_storage_path);
    rmdir(g_tmp_root);
    shutdown_logger();
    return result;
}