#define _GNU_SOURCE
#include <errno.h>
#include <dirent.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/stat.h>
#include <unistd.h>
#include <cjson/cJSON.h>
#include "unity.h"
#include "core/config.h"
#include "core/logger.h"
#include "database/db_core.h"
#include "web/api_handlers.h"
#include "web/api_handlers_system.h"
#include "web/request_response.h"
#include "telemetry/collectors/linux_filesystem.h"

static char root[256], database[MAX_PATH_LENGTH], literal[MAX_PATH_LENGTH];
static config_t baseline, persisted;
static unsigned saves, shell_calls;
static http_response_t response;

static int remove_owned_backups(void) {
    char directory[MAX_PATH_LENGTH + 16];
    snprintf(directory, sizeof(directory), "%s.backups", database);
    DIR *dir = opendir(directory);
    if (!dir) return errno == ENOENT ? 0 : -1;
    int result = 0;
    struct dirent *entry;
    while ((entry = readdir(dir))) {
        if (!strcmp(entry->d_name, ".") || !strcmp(entry->d_name, "..")) continue;
        /* unlinkat removes only entries in our private backup directory;
         * it never follows a link or recursively removes an unknown tree. */
        if (unlinkat(dirfd(dir), entry->d_name, 0) != 0) result = -1;
    }
    if (closedir(dir) != 0) result = -1;
    if (rmdir(directory) != 0) result = -1;
    return result;
}

/* Capture the handler's persistence boundary; never write a host config file. */
int __wrap_save_config(const config_t *config, const char *path) {
    (void)path;
    persisted = *config;
    saves++;
    return 0;
}

int __wrap_system(const char *command) {
    (void)command;
    shell_calls++;
    errno = EACCES;
    return -1;
}

FILE *__wrap_popen(const char *command, const char *mode) {
    (void)command; (void)mode;
    shell_calls++;
    errno = EACCES;
    return NULL;
}

void setUp(void) {
    g_config = baseline;
    saves = shell_calls = 0;
    memset(&persisted, 0, sizeof(persisted));
    http_response_init(&response);
}

void tearDown(void) {
    http_response_free(&response);
}

static void post_path(const char *field, const char *value) {
    cJSON *body = cJSON_CreateObject();
    TEST_ASSERT_NOT_NULL(body);
    cJSON_AddStringToObject(body, field, value);
    char *json = cJSON_PrintUnformatted(body);
    cJSON_Delete(body);
    TEST_ASSERT_NOT_NULL(json);
    http_request_t request;
    http_request_init(&request);
    request.body = json;
    request.body_len = strlen(json);
    handle_post_settings(&request, &response);
    free(json);
}

static void assert_rejected(const char *field, const char *value) {
    g_config = baseline;
    http_response_free(&response);
    http_response_init(&response);
    post_path(field, value);
    TEST_ASSERT_EQUAL_INT_MESSAGE(400, response.status_code, value);
    TEST_ASSERT_EQUAL_UINT(0, saves);
    TEST_ASSERT_EQUAL_UINT(0, shell_calls);
    TEST_ASSERT_EQUAL_STRING(baseline.storage_path, g_config.storage_path);
    TEST_ASSERT_EQUAL_STRING(baseline.storage_path_hls, g_config.storage_path_hls);
    TEST_ASSERT_EQUAL_STRING(baseline.db_post_backup_script, g_config.db_post_backup_script);
}

void test_absolute_storage_path_with_spaces_is_persisted_without_shell(void) {
    char accepted[MAX_PATH_LENGTH];
    snprintf(accepted, sizeof(accepted), "%s/recording space", root);
    post_path("storage_path", accepted);
    TEST_ASSERT_EQUAL_INT(200, response.status_code);
    TEST_ASSERT_EQUAL_UINT(1, saves);
    TEST_ASSERT_EQUAL_STRING(accepted, persisted.storage_path);
    TEST_ASSERT_EQUAL_UINT(0, shell_calls);
}

void test_optional_hls_override_can_be_set_and_cleared(void) {
    post_path("storage_path_hls", root);
    TEST_ASSERT_EQUAL_INT(200, response.status_code);
    TEST_ASSERT_EQUAL_STRING(root, persisted.storage_path_hls);
    http_response_free(&response);
    http_response_init(&response);
    post_path("storage_path_hls", "");
    TEST_ASSERT_EQUAL_INT(200, response.status_code);
    TEST_ASSERT_EQUAL_UINT(2, saves);
    TEST_ASSERT_EQUAL_STRING("", persisted.storage_path_hls);
    TEST_ASSERT_EQUAL_UINT(0, shell_calls);
}

void test_backup_hook_path_is_saved_without_executing_it(void) {
    char accepted[MAX_PATH_LENGTH];
    snprintf(accepted, sizeof(accepted), "%s/nonexistent-hook", root);
    post_path("db_post_backup_script", accepted);
    TEST_ASSERT_EQUAL_INT(200, response.status_code);
    TEST_ASSERT_EQUAL_UINT(1, saves);
    TEST_ASSERT_EQUAL_STRING(accepted, persisted.db_post_backup_script);
    TEST_ASSERT_EQUAL_UINT(0, shell_calls);
}

void test_relative_and_empty_primary_storage_paths_are_rejected(void) {
    assert_rejected("storage_path", "relative/path");
    assert_rejected("storage_path", "");
    assert_rejected("storage_path_hls", "relative/path");
    assert_rejected("db_post_backup_script", "relative/hook");
}

void test_shell_metacharacters_cannot_reach_path_persistence(void) {
    const char *fields[] = { "storage_path", "storage_path_hls", "db_post_backup_script" };
    const char characters[] = ";|&`$><\n\r\\\"'!{}()[]~*?";
    for (size_t f = 0; f < sizeof(fields) / sizeof(fields[0]); f++) {
        for (size_t i = 0; i < strlen(characters); i++) {
            char candidate[MAX_PATH_LENGTH];
            snprintf(candidate, sizeof(candidate), "%s/path%cmarker", root, characters[i]);
            assert_rejected(fields[f], candidate);
        }
    }
}

void test_cached_system_response_uses_no_shell(void) {
    /* This endpoint reads cached filesystem telemetry and DB recording sizes;
     * it does not itself sample this configured path. The next test exercises
     * the actual filesystem collector separately. */
    strcpy(g_config.storage_path, literal);
    http_request_t request;
    http_request_init(&request);
    handle_get_system_info(&request, &response);
    TEST_ASSERT_EQUAL_INT(200, response.status_code);
    TEST_ASSERT_EQUAL_UINT(0, shell_calls);
    TEST_ASSERT_EQUAL_UINT(0, saves);
    TEST_ASSERT_EQUAL_INT(0, access(literal, F_OK));
    cJSON *json = cJSON_Parse(response.body);
    TEST_ASSERT_NOT_NULL(json);
    TEST_ASSERT_TRUE(cJSON_IsObject(json));
    cJSON_Delete(json);
}

void test_filesystem_collector_samples_literal_path_without_shell(void) {
    linux_filesystem_resource_t resource = {0};
    strcpy(resource.logical_id, "recordings");
    strcpy(resource.path, literal);
    linux_filesystem_sample_t sample;
    TEST_ASSERT_EQUAL_INT(0, linux_filesystem_sample(&resource, "/proc/self/mountinfo", &sample));
    TEST_ASSERT_EQUAL_INT(SYSTEM_HEALTH_CAPABILITY_AVAILABLE, sample.capacity_bytes.capability);
    struct statvfs expected;
    TEST_ASSERT_EQUAL_INT(0, statvfs(literal, &expected));
    TEST_ASSERT_EQUAL_UINT64((uint64_t)expected.f_blocks * expected.f_frsize, sample.capacity_bytes.value);
    TEST_ASSERT_EQUAL_STRING("recordings", sample.logical_id);
    TEST_ASSERT_EQUAL_UINT(0, shell_calls);
    TEST_ASSERT_EQUAL_INT(0, access(literal, F_OK));
}

int main(void) {
    strcpy(root, "/tmp/lightnvr-settings-boundary-XXXXXX");
    if (!mkdtemp(root)) return 1;
    snprintf(database, sizeof(database), "%s/test.db", root);
    snprintf(literal, sizeof(literal), "%s/literal;echo not-a-command", root);
    if (mkdir(literal, 0700) != 0) return 1;
    init_logger();
    load_default_config(&g_config);
    g_config.web_auth_enabled = false;
    g_config.go2rtc_enabled = false;
    g_config.mqtt_enabled = false;
    strcpy(g_config.storage_path, root);
    strcpy(g_config.db_path, database);
    if (init_database(database) != 0) return 1;
    baseline = g_config;
    UNITY_BEGIN();
    RUN_TEST(test_absolute_storage_path_with_spaces_is_persisted_without_shell);
    RUN_TEST(test_optional_hls_override_can_be_set_and_cleared);
    RUN_TEST(test_backup_hook_path_is_saved_without_executing_it);
    RUN_TEST(test_relative_and_empty_primary_storage_paths_are_rejected);
    RUN_TEST(test_shell_metacharacters_cannot_reach_path_persistence);
    RUN_TEST(test_cached_system_response_uses_no_shell);
    RUN_TEST(test_filesystem_collector_samples_literal_path_without_shell);
    int result = UNITY_END();
    shutdown_database();
    const char *suffixes[] = { "", "-wal", "-shm", ".bak" };
    for (size_t i = 0; i < sizeof(suffixes) / sizeof(suffixes[0]); i++) {
        char file[MAX_PATH_LENGTH + 8];
        snprintf(file, sizeof(file), "%s%s", database, suffixes[i]);
        unlink(file);
    }
    free(g_config.streams);
    g_config.streams = NULL;
    if (remove_owned_backups() != 0) result = 1;
    if (rmdir(literal) != 0) result = 1;
    if (rmdir(root) != 0) result = 1;
    shutdown_logger();
    return result;
}
