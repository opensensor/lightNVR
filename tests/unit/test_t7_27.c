#define _GNU_SOURCE
#define _POSIX_C_SOURCE 200809L
#include <dirent.h>
#include <errno.h>
#include <fcntl.h>
#include <stdbool.h>
#include <stdarg.h>
#include <sys/syscall.h>
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
static int replacement_kind, binding_error, publication_race_kind, publication_error, restore_error;
static char observed_upload[1024], observed_quarantine[1024];
static int recovery_error;
static bool recovery_restore_collision, replace_upload_directory;
static char displaced_private[256], replaced_private[1024];
static bool fail_backup_creation, tamper_backup_directory, mismatch_restore_collision;
static char altered_directory[1024];
static char observed_backup[1024];
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
static bool before_upload_publication(void) {
    if (publication_race_kind) {
        injected = true;
        if (publication_race_kind == 1) put_text(path, "publication competitor");
        else TEST_ASSERT_EQUAL_INT(0, symlink(sentinel, path));
        publication_race_kind = 0;
    }
    if (fail_publication || publication_error) {
        fail_publication = false; injected = true;
        if (publication_competitor) put_text(path, "publication rollback competitor");
        errno = publication_error ? publication_error : ENOSPC;
        return false;
    }
    return true;
}
int __real_access(const char *, int);
int __real_rename(const char *, const char *);
static void replace_inspected_background(void) {
    armed_access = false; injected = true;
    TEST_ASSERT_EQUAL_INT(0, __real_rename(path, saved));
    if (replacement_kind == 1) put_text(path, "unrelated replacement");
    else TEST_ASSERT_EQUAL_INT(0, symlink(sentinel, path));
}
int __wrap_access(const char *name, int mode) {
    int result = __real_access(name, mode); int saved_errno = errno;
    if (armed_access && !strcmp(name, path) && result == 0) replace_inspected_background();
    errno = saved_errno; return result;
}
int __real_fstat(int, struct stat *);
int __wrap_fstat(int fd, struct stat *info) {
    int result = __real_fstat(fd, info); int saved_errno = errno;
    if (armed_access && result == 0 && S_ISREG(info->st_mode)) {
        char proc[64], name[1024]; snprintf(proc, sizeof(proc), "/proc/self/fd/%d", fd);
        ssize_t size = readlink(proc, name, sizeof(name)-1);
        if (size > 0) { name[size] = 0; if (!strcmp(name, path)) replace_inspected_background(); }
    }
    errno = saved_errno; return result;
}
long __real_syscall(long, ...);
long __wrap_syscall(long number, ...) {
    if (number != SYS_renameat2) { TEST_FAIL_MESSAGE("Unexpected background syscall"); errno = ENOSYS; return -1; }
    va_list args; va_start(args, number);
    int from = va_arg(args, int); const char *oldname = va_arg(args, const char *);
    int to = va_arg(args, int); const char *newname = va_arg(args, const char *);
    unsigned flags = va_arg(args, unsigned); va_end(args);
    if (!strcmp(newname, "original")) {
        char proc[64], name[1024]; snprintf(proc, sizeof(proc), "/proc/self/fd/%d", to);
        ssize_t size = readlink(proc, name, sizeof(name)-1); TEST_ASSERT_TRUE(size > 0); name[size] = 0;
        int length = snprintf(observed_backup, sizeof(observed_backup), "%s/original", name);
        TEST_ASSERT_TRUE(length > 0 && (size_t)length < sizeof(observed_backup));
        if (binding_error) { errno = binding_error; return -1; }
    }
    if (strstr(oldname, ".tmp.") && !strcmp(newname, strrchr(path, '/') + 1)) {
        char proc[64], directory[1024]; snprintf(proc, sizeof(proc), "/proc/self/fd/%d", from);
        ssize_t size = readlink(proc, directory, sizeof(directory)-1); TEST_ASSERT_TRUE(size > 0); directory[size] = 0;
        int length = snprintf(observed_upload, sizeof(observed_upload), "%s/%s", directory, oldname);
        TEST_ASSERT_TRUE(length > 0 && (size_t)length < sizeof(observed_upload));
        if (!before_upload_publication()) return -1;
    }
    if (!strcmp(newname, "failed-upload")) {
        char proc[64], directory[1024]; snprintf(proc, sizeof(proc), "/proc/self/fd/%d", to);
        ssize_t size = readlink(proc, directory, sizeof(directory)-1); TEST_ASSERT_TRUE(size > 0); directory[size] = 0;
        int length = snprintf(observed_quarantine, sizeof(observed_quarantine), "%s/failed-upload", directory);
        TEST_ASSERT_TRUE(length > 0 && (size_t)length < sizeof(observed_quarantine));
        if (recovery_error) { errno = recovery_error; return -1; }
    }
    if (recovery_restore_collision && !strcmp(oldname, "failed-upload"))
        put_text(path, "recovery restore competitor");
    if (restore_error && !strcmp(oldname, "original") && strcmp(newname, "original")) {
        errno = restore_error; return -1;
    }
    if (mismatch_restore_collision && !strcmp(oldname, "original") && strcmp(newname, "original"))
        put_text(path, "identity restore competitor");
    return __real_syscall(number, from, oldname, to, newname, flags);
}
static void substitute_upload_directory(const char *directory) {
    replace_upload_directory = false; injected = true;
    safe_strcpy(replaced_private, directory, sizeof(replaced_private), 0);
    TEST_ASSERT_EQUAL_INT(0, __real_rename(directory, displaced_private));
    TEST_ASSERT_EQUAL_INT(0, mkdir(directory, 0700));
    char marker[1200]; int length = snprintf(marker, sizeof(marker), "%s/marker", directory);
    TEST_ASSERT_TRUE(length > 0 && (size_t)length < sizeof(marker));
    put_text(marker, "replacement directory marker");
}
int __real_mkstemp(char *);
int __wrap_mkstemp(char *name) {
    if (replace_upload_directory && strstr(name, "/upload.tmp.")) {
        char directory[1024]; safe_strcpy(directory, name, sizeof(directory), 0);
        *strrchr(directory, '/') = 0; substitute_upload_directory(directory);
    }
    return __real_mkstemp(name);
}
int __real_openat(int, const char *, int, ...);
int __wrap_openat(int fd, const char *name, int flags, ...) {
    mode_t mode = 0;
    if (flags & O_CREAT) { va_list args; va_start(args, flags); mode = (mode_t)va_arg(args, int); va_end(args); }
    if (replace_upload_directory && !strncmp(name, "upload.tmp.", 11) && (flags & O_CREAT)) {
        char proc[64], directory[1024]; snprintf(proc, sizeof(proc), "/proc/self/fd/%d", fd);
        ssize_t size = readlink(proc, directory, sizeof(directory)-1); TEST_ASSERT_TRUE(size > 0); directory[size] = 0;
        substitute_upload_directory(directory);
    }
    return flags & O_CREAT ? __real_openat(fd, name, flags, mode) : __real_openat(fd, name, flags);
}
char *__real_mkdtemp(char *);
char *__wrap_mkdtemp(char *name) {
    if (fail_backup_creation && strstr(name, ".backup.")) { errno = EACCES; return NULL; }
    char *result = __real_mkdtemp(name);
    if (result && tamper_backup_directory && strstr(name, ".backup.")) {
        TEST_ASSERT_EQUAL_INT(0, chmod(name, 0755));
        safe_strcpy(altered_directory, name, sizeof(altered_directory), 0);
    }
    return result;
}
int __wrap_rename(const char *from, const char *to) {
    if (!strcmp(from, path) && (strstr(to, ".bak") || strstr(to, "/original")))
        safe_strcpy(observed_backup, to, sizeof(observed_backup), 0);
    if (!strcmp(to, path) && strstr(from, ".tmp.")) {
        safe_strcpy(observed_upload, from, sizeof(observed_upload), 0);
        if (!before_upload_publication()) return -1;
    }
    if (restore_error && !strcmp(from, observed_backup) && !strcmp(to, path)) {
        errno = restore_error; return -1;
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
    fail_metadata = metadata_competitor = fail_backup_creation = false; replacement_kind = binding_error = publication_race_kind = publication_error = restore_error = 0; observed_upload[0] = observed_quarantine[0] = 0; recovery_error = 0; recovery_restore_collision = replace_upload_directory = false; replaced_private[0] = 0; observed_backup[0] = 0; tamper_backup_directory = mismatch_restore_collision = false; altered_directory[0] = 0;
    g_config.web_auth_enabled = false; g_config.demo_mode = false;
    safe_strcpy(g_config.storage_path, storage, sizeof(g_config.storage_path), 0);
    TEST_ASSERT_EQUAL_INT(SQLITE_OK, sqlite3_exec(get_db_handle(), "DELETE FROM operator_floor_plans;", NULL, NULL, NULL));
    remove_owned_tree(storage); remove_owned_tree(displaced_private); TEST_ASSERT_EQUAL_INT(0, mkdir(storage, 0700));
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
    TEST_ASSERT_TRUE(observed_backup[0] != 0);
    assert_bytes(observed_backup, old_png, sizeof(old_png)); assert_metadata_unchanged();
}
static void test_metadata_rollback_does_not_unlink_competitor(void) {
    fail_metadata = metadata_competitor = true;
    TEST_ASSERT_EQUAL_INT(500, upload_background()); TEST_ASSERT_TRUE(injected);
    assert_text(path, "metadata rollback competitor");
    TEST_ASSERT_TRUE(observed_backup[0] != 0);
    assert_bytes(observed_backup, old_png, sizeof(old_png));
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
static void test_preexisting_backup_symlink_is_preserved(void) {
    TEST_ASSERT_EQUAL_INT(0, symlink(sentinel, backup));
    TEST_ASSERT_EQUAL_INT(200, upload_background());
    struct stat info; TEST_ASSERT_EQUAL_INT(0, lstat(backup, &info));
    TEST_ASSERT_TRUE(S_ISLNK(info.st_mode)); assert_text(sentinel, "sentinel unchanged");
    assert_bytes(path, new_png, sizeof(new_png));
}
static void test_private_backup_creation_failure_preserves_old_file(void) {
    fail_backup_creation = true;
    TEST_ASSERT_EQUAL_INT(500, upload_background());
    assert_bytes(path, old_png, sizeof(old_png)); assert_metadata_unchanged();
}
static void test_invalid_private_directory_is_retained_without_touching_background(void) {
    tamper_backup_directory = true;
    TEST_ASSERT_EQUAL_INT(500, upload_background());
    assert_bytes(path, old_png, sizeof(old_png)); assert_metadata_unchanged();
    struct stat info; TEST_ASSERT_TRUE(altered_directory[0] != 0);
    TEST_ASSERT_EQUAL_INT(0, lstat(altered_directory, &info));
    TEST_ASSERT_TRUE(S_ISDIR(info.st_mode));
}
static void test_initial_upload_without_old_background_succeeds(void) {
    TEST_ASSERT_EQUAL_INT(0, unlink(path));
    TEST_ASSERT_EQUAL_INT(DB_OPERATOR_FLOOR_PLAN_OK, db_operator_floor_plan_set_background(uuid, NULL));
    TEST_ASSERT_EQUAL_INT(200, upload_background());
    assert_bytes(path, new_png, sizeof(new_png)); assert_metadata_unchanged();
}
static void test_preexisting_symlink_background_is_rejected(void) {
    TEST_ASSERT_EQUAL_INT(0, unlink(path)); TEST_ASSERT_EQUAL_INT(0, symlink(sentinel, path));
    TEST_ASSERT_EQUAL_INT(500, upload_background());
    struct stat info; TEST_ASSERT_EQUAL_INT(0, lstat(path, &info)); TEST_ASSERT_TRUE(S_ISLNK(info.st_mode));
    assert_text(sentinel, "sentinel unchanged"); assert_metadata_unchanged();
}
static void test_unsupported_old_file_binding_preserves_background(void) {
    binding_error = ENOSYS;
    TEST_ASSERT_EQUAL_INT(500, upload_background());
    assert_bytes(path, old_png, sizeof(old_png)); assert_metadata_unchanged();
}
static void test_identity_restore_collision_preserves_all_entries(void) {
    armed_access = true; replacement_kind = 1; mismatch_restore_collision = true;
    TEST_ASSERT_EQUAL_INT(500, upload_background()); TEST_ASSERT_TRUE(injected);
    assert_bytes(saved, old_png, sizeof(old_png));
    assert_text(path, "identity restore competitor");
    assert_text(observed_backup, "unrelated replacement"); assert_metadata_unchanged();
}
static void test_upload_publication_does_not_overwrite_new_competitor(void) {
    publication_race_kind = 1;
    TEST_ASSERT_EQUAL_INT(500, upload_background()); TEST_ASSERT_TRUE(injected);
    assert_text(path, "publication competitor");
    assert_bytes(observed_backup, old_png, sizeof(old_png));
    assert_bytes(observed_upload, new_png, sizeof(new_png)); assert_metadata_unchanged();
}
static void test_upload_publication_preserves_competing_symlink(void) {
    publication_race_kind = 2;
    TEST_ASSERT_EQUAL_INT(500, upload_background()); TEST_ASSERT_TRUE(injected);
    struct stat info; TEST_ASSERT_EQUAL_INT(0, lstat(path, &info)); TEST_ASSERT_TRUE(S_ISLNK(info.st_mode));
    assert_text(sentinel, "sentinel unchanged");
    assert_bytes(observed_backup, old_png, sizeof(old_png)); assert_metadata_unchanged();
}
static void test_unsupported_publication_restores_old_background(void) {
    publication_error = ENOSYS;
    TEST_ASSERT_EQUAL_INT(500, upload_background()); TEST_ASSERT_TRUE(injected);
    assert_bytes(path, old_png, sizeof(old_png)); assert_metadata_unchanged();
}
static void test_unsupported_restore_retains_original_private_backup(void) {
    fail_publication = true; restore_error = ENOSYS;
    TEST_ASSERT_EQUAL_INT(500, upload_background()); TEST_ASSERT_TRUE(injected);
    TEST_ASSERT_EQUAL_INT(-1, access(path, F_OK));
    assert_bytes(observed_backup, old_png, sizeof(old_png)); assert_metadata_unchanged();
}
static void test_failed_recovery_preserves_upload_and_original(void) {
    fail_metadata = true; recovery_error = ENOSYS;
    TEST_ASSERT_EQUAL_INT(500, upload_background()); TEST_ASSERT_TRUE(injected);
    assert_bytes(path, new_png, sizeof(new_png));
    assert_bytes(observed_backup, old_png, sizeof(old_png)); assert_metadata_unchanged();
}
static void test_changed_upload_restore_collision_preserves_every_entry(void) {
    fail_metadata = metadata_competitor = recovery_restore_collision = true;
    TEST_ASSERT_EQUAL_INT(500, upload_background()); TEST_ASSERT_TRUE(injected);
    assert_text(path, "recovery restore competitor");
    assert_text(observed_quarantine, "metadata rollback competitor");
    assert_bytes(observed_backup, old_png, sizeof(old_png));
    assert_bytes(uploaded, new_png, sizeof(new_png)); assert_metadata_unchanged();
}
static int descriptor_count(void) {
    DIR *directory = opendir("/proc/self/fd"); TEST_ASSERT_NOT_NULL(directory);
    int count = 0; struct dirent *entry;
    while ((entry = readdir(directory))) if (strcmp(entry->d_name, ".") && strcmp(entry->d_name, "..")) ++count;
    closedir(directory); return count;
}
static void test_repeated_ordinary_failure_paths_release_descriptors(void) {
    int before = descriptor_count();
    for (int index = 0; index < 9; ++index) {
        binding_error = index % 3 == 0 ? ENOSYS : 0;
        publication_error = index % 3 == 1 ? ENOSYS : 0;
        fail_metadata = index % 3 == 2;
        TEST_ASSERT_EQUAL_INT(500, upload_background());
        assert_bytes(path, old_png, sizeof(old_png)); assert_metadata_unchanged();
    }
    TEST_ASSERT_EQUAL_INT(before, descriptor_count());
}
static void test_upload_creation_uses_held_private_directory(void) {
    replace_upload_directory = true;
    TEST_ASSERT_EQUAL_INT(500, upload_background()); TEST_ASSERT_TRUE(injected);
    assert_bytes(path, old_png, sizeof(old_png)); assert_metadata_unchanged();
    DIR *directory = opendir(replaced_private); TEST_ASSERT_NOT_NULL(directory);
    int unexpected = 0; struct dirent *entry;
    while ((entry = readdir(directory)))
        if (strcmp(entry->d_name, ".") && strcmp(entry->d_name, "..") && strcmp(entry->d_name, "marker")) ++unexpected;
    closedir(directory);
    TEST_ASSERT_EQUAL_INT(0, unexpected);
    char marker[1200]; snprintf(marker, sizeof(marker), "%s/marker", replaced_private);
    assert_text(marker, "replacement directory marker");
}
int main(void) {
    strcpy(root, "/tmp/lightnvr-t7-27-XXXXXX"); if (!mkdtemp(root)) return 2;
    snprintf(storage, sizeof(storage), "%s/storage", root);
    snprintf(database, sizeof(database), "%s/database.sqlite", root);
    snprintf(saved, sizeof(saved), "%s/original", root);
    snprintf(uploaded, sizeof(uploaded), "%s/uploaded", root);
    snprintf(displaced_private, sizeof(displaced_private), "%s/displaced-private", root);
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
    RUN_TEST(test_preexisting_backup_symlink_is_preserved);
    RUN_TEST(test_private_backup_creation_failure_preserves_old_file);
    RUN_TEST(test_invalid_private_directory_is_retained_without_touching_background);
    RUN_TEST(test_initial_upload_without_old_background_succeeds);
    RUN_TEST(test_preexisting_symlink_background_is_rejected);
    RUN_TEST(test_unsupported_old_file_binding_preserves_background);
    RUN_TEST(test_identity_restore_collision_preserves_all_entries);
    RUN_TEST(test_upload_publication_does_not_overwrite_new_competitor);
    RUN_TEST(test_upload_publication_preserves_competing_symlink);
    RUN_TEST(test_unsupported_publication_restores_old_background);
    RUN_TEST(test_unsupported_restore_retains_original_private_backup);
    RUN_TEST(test_failed_recovery_preserves_upload_and_original);
    RUN_TEST(test_changed_upload_restore_collision_preserves_every_entry);
    RUN_TEST(test_repeated_ordinary_failure_paths_release_descriptors);
    RUN_TEST(test_upload_creation_uses_held_private_directory);
    int result = UNITY_END();
    shutdown_database(); free(g_config.streams); g_config.streams = NULL;
    remove_owned_tree(root); return result;
}
