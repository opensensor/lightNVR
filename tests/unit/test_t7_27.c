#define _GNU_SOURCE
#define _POSIX_C_SOURCE 200809L
#include <dirent.h>
#include <errno.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/stat.h>
#include <unistd.h>
#include "unity.h"
#include "core/config.h"
#include "core/path_utils.h"
#include "database/db_core.h"
#include "database/db_operator_floor_plans.h"
#include "utils/strings.h"
#include "web/api_handlers_operator_floor_plans.h"
#include "web/request_response.h"

static char root[128], storage[256], database[256];
static char path[512], backup[520], saved[256], sentinel[256], uploaded[256];
static char endpoint[128], uuid[CAMERA_UUID_STRING_SIZE];
static char original_storage[MAX_PATH_LENGTH];
static bool armed_access, injected, fail_publication, publication_competitor;
static bool fail_metadata, metadata_competitor;
static int replacement_kind;
static const unsigned char old_png[] = {0x89,'P','N','G',13,10,26,10,'o','l','d'};
static const unsigned char new_png[] = {0x89,'P','N','G',13,10,26,10,'n','e','w'};

static void put_bytes(const char *name, const void *data, size_t size) {
    TEST_ASSERT_EQUAL_INT(0, ensure_path(name));
    FILE *file = fopen(name, "wb"); TEST_ASSERT_NOT_NULL(file);
    size_t count = fwrite(data, 1, size, file); int result = fclose(file);
    TEST_ASSERT_EQUAL_UINT(size, count); TEST_ASSERT_EQUAL_INT(0, result);
}
static void put_text(const char *name, const char *text) { put_bytes(name, text, strlen(text)); }
static void assert_bytes(const char *name, const void *expected, size_t size) {
    unsigned char data[128]; FILE *file = fopen(name, "rb"); TEST_ASSERT_NOT_NULL(file);
    size_t count = fread(data, 1, sizeof(data), file); int result = fclose(file);
    TEST_ASSERT_EQUAL_INT(0, result); TEST_ASSERT_EQUAL_UINT(size, count);
    TEST_ASSERT_EQUAL_MEMORY(expected, data, size);
}
static void assert_text(const char *name, const char *text) { assert_bytes(name, text, strlen(text)); }
int __real_access(const char *, int);
int __real_rename(const char *, const char *);
int __wrap_access(const char *name, int mode) {
    int result = __real_access(name, mode); int saved_errno = errno;
    if (armed_access && !strcmp(name, path) && result == 0) {
        armed_access = false; injected = true;
        TEST_ASSERT_EQUAL_INT(0, __real_rename(path, saved));
        if (replacement_kind == 1) put_text(path, "unrelated replacement");
        else TEST_ASSERT_EQUAL_INT(0, symlink(sentinel, path));
    }
    errno = saved_errno; return result;
}
int __wrap_rename(const char *from, const char *to) {
    if (fail_publication && !strcmp(to, path) && strstr(from, ".tmp.")) {
        fail_publication = false; injected = true;
        if (publication_competitor) put_text(path, "publication rollback competitor");
        errno = ENOSPC; return -1;
    }
    return __real_rename(from, to);
}
db_operator_floor_plan_result_t __real_db_operator_floor_plan_set_background(const char *, const char *);
db_operator_floor_plan_result_t __wrap_db_operator_floor_plan_set_background(const char *id, const char *mime) {
    if (fail_metadata && mime && !strcmp(id, uuid)) {
        fail_metadata = false; injected = true;
        if (metadata_competitor) {
            TEST_ASSERT_EQUAL_INT(0, __real_rename(path, uploaded));
            put_text(path, "metadata rollback competitor");
        }
        return DB_OPERATOR_FLOOR_PLAN_ERROR;
    }
    return __real_db_operator_floor_plan_set_background(id, mime);
}
static void remove_owned_tree(const char *directory) {
    DIR *dir = opendir(directory); if (!dir) return;
    struct dirent *entry;
    while ((entry = readdir(dir))) {
        if (!strcmp(entry->d_name, ".") || !strcmp(entry->d_name, "..")) continue;
        char name[1024]; int size = snprintf(name, sizeof(name), "%s/%s", directory, entry->d_name);
        if (size < 0 || (size_t)size >= sizeof(name)) continue;
        struct stat info;
        if (lstat(name, &info) == 0 && S_ISDIR(info.st_mode)) remove_owned_tree(name);
        else unlink(name);
    }
    closedir(dir); rmdir(directory);
}
static int upload_background(void) {
    http_request_t request; http_response_t response;
    http_request_init(&request); http_response_init(&response);
    request.method = HTTP_METHOD_PUT;
    safe_strcpy(request.path, endpoint, sizeof(request.path), 0);
    strcpy(request.client_ip, "127.0.0.1");
    request.body = (void *)new_png; request.body_len = sizeof(new_png);
    strcpy(request.headers[0].name, "Content-Type");
    strcpy(request.headers[0].value, "image/png"); request.num_headers = 1;
    handle_put_operator_floor_plan_background(&request, &response);
    int status = response.status_code;
    http_response_free(&response); /* Release before assertions can longjmp. */
    return status;
}
static void assert_metadata_unchanged(void) {
    operator_floor_plan_t plan;
    TEST_ASSERT_EQUAL_INT(DB_OPERATOR_FLOOR_PLAN_OK, db_operator_floor_plan_get(uuid, &plan));
    TEST_ASSERT_EQUAL_STRING("image/png", plan.background_mime);
    TEST_ASSERT_EQUAL_INT64(1, plan.revision);
}
void setUp(void) {
    armed_access = injected = fail_publication = publication_competitor = false;
    fail_metadata = metadata_competitor = false; replacement_kind = 0;
    g_config.web_auth_enabled = false; g_config.demo_mode = false;
    safe_strcpy(g_config.storage_path, storage, sizeof(g_config.storage_path), 0);
    TEST_ASSERT_EQUAL_INT(SQLITE_OK, sqlite3_exec(get_db_handle(), "DELETE FROM operator_floor_plans;", NULL, NULL, NULL));
    remove_owned_tree(storage); TEST_ASSERT_EQUAL_INT(0, mkdir(storage, 0700));
    unlink(saved); unlink(sentinel); unlink(uploaded);
    operator_floor_plan_t plan = {0};
    strcpy(plan.name, "T7.27 private plan"); plan.canvas_width = 1000; plan.canvas_height = 700;
    TEST_ASSERT_EQUAL_INT(DB_OPERATOR_FLOOR_PLAN_OK, db_operator_floor_plan_create(&plan, NULL, 0, NULL));
    strcpy(uuid, plan.uuid);
    TEST_ASSERT_EQUAL_INT(DB_OPERATOR_FLOOR_PLAN_OK, db_operator_floor_plan_set_background(uuid, "image/png"));
    snprintf(path, sizeof(path), "%s/floor_plans/%s.png", storage, uuid);
    snprintf(backup, sizeof(backup), "%s.bak", path);
    snprintf(endpoint, sizeof(endpoint), "/api/live/plans/%s/background", uuid);
    put_bytes(path, old_png, sizeof(old_png)); put_text(sentinel, "sentinel unchanged");
}
void tearDown(void) {
    armed_access = fail_publication = fail_metadata = false;
    g_config.web_auth_enabled = false;
    safe_strcpy(g_config.storage_path, original_storage, sizeof(g_config.storage_path), 0);
}
static void test_normal_same_type_replacement_updates_bytes(void) {
    TEST_ASSERT_EQUAL_INT(200, upload_background());
    assert_bytes(path, new_png, sizeof(new_png)); assert_metadata_unchanged();
    TEST_ASSERT_EQUAL_INT(-1, access(backup, F_OK));
}
static void test_publication_failure_restores_original_without_competitor(void) {
    fail_publication = true;
    TEST_ASSERT_EQUAL_INT(500, upload_background()); TEST_ASSERT_TRUE(injected);
    assert_bytes(path, old_png, sizeof(old_png)); assert_metadata_unchanged();
}
static void test_metadata_failure_restores_original_without_competitor(void) {
    fail_metadata = true;
    TEST_ASSERT_EQUAL_INT(500, upload_background()); TEST_ASSERT_TRUE(injected);
    assert_bytes(path, old_png, sizeof(old_png)); assert_metadata_unchanged();
}
static void test_replacement_after_access_is_not_backed_up_and_deleted(void) {
    armed_access = true; replacement_kind = 1;
    int status = upload_background(); TEST_ASSERT_TRUE(injected);
    assert_bytes(saved, old_png, sizeof(old_png));
    assert_text(path, "unrelated replacement");
    TEST_ASSERT_TRUE(status >= 400); assert_metadata_unchanged();
}
static void test_symlink_after_access_is_preserved_without_touching_target(void) {
    armed_access = true; replacement_kind = 2;
    int status = upload_background(); TEST_ASSERT_TRUE(injected);
    assert_text(sentinel, "sentinel unchanged"); assert_bytes(saved, old_png, sizeof(old_png));
    struct stat info; TEST_ASSERT_EQUAL_INT(0, lstat(path, &info)); TEST_ASSERT_TRUE(S_ISLNK(info.st_mode));
    TEST_ASSERT_TRUE(status >= 400); assert_metadata_unchanged();
}
static void test_publication_rollback_does_not_overwrite_competitor(void) {
    fail_publication = publication_competitor = true;
    TEST_ASSERT_EQUAL_INT(500, upload_background()); TEST_ASSERT_TRUE(injected);
    assert_text(path, "publication rollback competitor");
    assert_bytes(backup, old_png, sizeof(old_png)); assert_metadata_unchanged();
}
static void test_metadata_rollback_does_not_unlink_competitor(void) {
    fail_metadata = metadata_competitor = true;
    TEST_ASSERT_EQUAL_INT(500, upload_background()); TEST_ASSERT_TRUE(injected);
    assert_text(path, "metadata rollback competitor");
    assert_bytes(backup, old_png, sizeof(old_png));
    assert_bytes(uploaded, new_png, sizeof(new_png)); assert_metadata_unchanged();
}
static void test_preexisting_backup_is_not_silently_deleted(void) {
    put_text(backup, "unknown preexisting backup");
    int status = upload_background();
    assert_text(backup, "unknown preexisting backup");
    TEST_ASSERT_TRUE(status == 200 || status >= 400);
}
static void test_unauthenticated_request_preserves_existing_file(void) {
    g_config.web_auth_enabled = true;
    TEST_ASSERT_EQUAL_INT(401, upload_background());
    assert_bytes(path, old_png, sizeof(old_png)); assert_metadata_unchanged();
}
int main(void) {
    strcpy(root, "/tmp/lightnvr-t7-27-XXXXXX"); if (!mkdtemp(root)) return 2;
    snprintf(storage, sizeof(storage), "%s/storage", root);
    snprintf(database, sizeof(database), "%s/database.sqlite", root);
    snprintf(saved, sizeof(saved), "%s/original", root);
    snprintf(uploaded, sizeof(uploaded), "%s/uploaded", root);
    snprintf(sentinel, sizeof(sentinel), "%s/sentinel", root);
    load_default_config(&g_config);
    safe_strcpy(original_storage, g_config.storage_path, sizeof(original_storage), 0);
    if (init_database_ex(database, DB_INIT_NO_BACKUP) != 0) {
        free(g_config.streams); remove_owned_tree(root); return 2;
    }
    UNITY_BEGIN();
    RUN_TEST(test_normal_same_type_replacement_updates_bytes);
    RUN_TEST(test_publication_failure_restores_original_without_competitor);
    RUN_TEST(test_metadata_failure_restores_original_without_competitor);
    RUN_TEST(test_replacement_after_access_is_not_backed_up_and_deleted);
    RUN_TEST(test_symlink_after_access_is_preserved_without_touching_target);
    RUN_TEST(test_publication_rollback_does_not_overwrite_competitor);
    RUN_TEST(test_metadata_rollback_does_not_unlink_competitor);
    RUN_TEST(test_preexisting_backup_is_not_silently_deleted);
    RUN_TEST(test_unauthenticated_request_preserves_existing_file);
    int result = UNITY_END();
    shutdown_database(); free(g_config.streams); g_config.streams = NULL;
    remove_owned_tree(root); return result;
}
