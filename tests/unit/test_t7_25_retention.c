#define _GNU_SOURCE
/* Exercise the private retention helper, without changing production visibility
 * or opening an application database. The production implementation is intact. */
#include "../../src/database/db_core.c"

#include "unity.h"

static char root[128], backups[256], moved[256], old_path[384], newer_path[384];
static char saved_old[256], target[256];
static unsigned inspected_files, swaps;
static int swap_kind;
static bool fixture_error;

static int write_bytes(const char *path, const char *value) {
    FILE *file = fopen(path, "wb");
    if (!file) return -1;
    size_t length = strlen(value);
    int result = fwrite(value, 1, length, file) == length ? 0 : -1;
    if (fclose(file) != 0) result = -1;
    return result;
}

static void replace_after_inspection(void) {
    int kind = swap_kind;
    swap_kind = 0;
    swaps++;
    if (kind == 1) {
        if (rename(backups, moved) != 0 || mkdir(backups, 0700) != 0 ||
            write_bytes(old_path, "replacement-parent-old") != 0 ||
            write_bytes(newer_path, "replacement-parent-newer") != 0) fixture_error = true;
    } else {
        if (rename(old_path, saved_old) != 0) { fixture_error = true; return; }
        if (kind == 2) {
            if (write_bytes(old_path, "replacement-leaf") != 0) fixture_error = true;
        } else if (symlink(target, old_path) != 0) {
            fixture_error = true;
        }
    }
}

int __real_stat(const char *, struct stat *);
int __wrap_stat(const char *path, struct stat *info) {
    int result = __real_stat(path, info);
    if (result == 0 && swap_kind &&
        (!strcmp(path, old_path) || !strcmp(path, newer_path))) {
        /* Wait for both real candidate metadata reads, independent of directory
         * enumeration order. The caller receives their original identities. */
        inspected_files++;
        if (inspected_files == 2) replace_after_inspection();
    }
    return result;
}

static bool exists(const char *path) {
    struct stat info;
    return lstat(path, &info) == 0;
}

static void assert_bytes(const char *path, const char *expected) {
    FILE *file = fopen(path, "rb");
    TEST_ASSERT_NOT_NULL_MESSAGE(file, path);
    char bytes[128] = {0};
    size_t count = fread(bytes, 1, sizeof(bytes), file);
    int close_result = fclose(file);
    TEST_ASSERT_EQUAL_INT(0, close_result);
    TEST_ASSERT_EQUAL_size_t(strlen(expected), count);
    TEST_ASSERT_EQUAL_MEMORY(expected, bytes, count);
}

/* This tree is created by this process under mkdtemp; never follow symlinks. */
static int remove_owned_tree(const char *path) {
    DIR *dir = opendir(path);
    if (!dir) return -1;
    int result = 0;
    struct dirent *entry;
    while ((entry = readdir(dir))) {
        if (!strcmp(entry->d_name, ".") || !strcmp(entry->d_name, "..")) continue;
        char child[768];
        int length = snprintf(child, sizeof(child), "%s/%s", path, entry->d_name);
        if (length < 0 || (size_t)length >= sizeof(child)) { result = -1; continue; }
        struct stat info;
        if (lstat(child, &info) != 0) { result = -1; continue; }
        if (S_ISDIR(info.st_mode)) {
            if (remove_owned_tree(child) != 0) result = -1;
        } else if (unlink(child) != 0) result = -1;
    }
    if (closedir(dir) != 0) result = -1;
    if (rmdir(path) != 0) result = -1;
    return result;
}

void setUp(void) {
    swap_kind = 0;
    swaps = inspected_files = 0;
    fixture_error = false;
    strcpy(root, "/tmp/lightnvr-retention-identity-XXXXXX");
    TEST_ASSERT_NOT_NULL(mkdtemp(root));
    snprintf(backups, sizeof(backups), "%s/backups", root);
    snprintf(moved, sizeof(moved), "%s/moved-backups", root);
    snprintf(saved_old, sizeof(saved_old), "%s/inspected-old", root);
    snprintf(target, sizeof(target), "%s/symlink-target", root);
    snprintf(old_path, sizeof(old_path), "%s/20240101T000000Z.sqlite3", backups);
    snprintf(newer_path, sizeof(newer_path), "%s/20240102T000000Z.sqlite3", backups);
    TEST_ASSERT_EQUAL_INT(0, mkdir(backups, 0700));
    TEST_ASSERT_EQUAL_INT(0, write_bytes(old_path, "inspected-old"));
    TEST_ASSERT_EQUAL_INT(0, write_bytes(newer_path, "inspected-newer"));
    TEST_ASSERT_EQUAL_INT(0, write_bytes(target, "outside-target"));
}

void tearDown(void) {
    swap_kind = 0;
    TEST_ASSERT_EQUAL_INT(0, remove_owned_tree(root));
}

void test_retention_keeps_newest_and_excludes_unpublished_artifacts(void) {
    char temporary[384], note[384];
    snprintf(temporary, sizeof(temporary), "%s/20230101T000000Z.sqlite3.partial", backups);
    snprintf(note, sizeof(note), "%s/operator-note.txt", backups);
    TEST_ASSERT_EQUAL_INT(0, write_bytes(temporary, "unfinished"));
    TEST_ASSERT_EQUAL_INT(0, write_bytes(note, "note"));
    TEST_ASSERT_EQUAL_INT(0, prune_timestamped_backups(backups, 1));
    TEST_ASSERT_FALSE(exists(old_path));
    assert_bytes(newer_path, "inspected-newer");
    assert_bytes(temporary, "unfinished");
    assert_bytes(note, "note");
}

void test_retention_within_limit_preserves_all_bytes(void) {
    TEST_ASSERT_EQUAL_INT(0, prune_timestamped_backups(backups, 2));
    assert_bytes(old_path, "inspected-old");
    assert_bytes(newer_path, "inspected-newer");
}

void test_parent_replacement_cannot_redirect_retention_unlink(void) {
    swap_kind = 1;
    int result = prune_timestamped_backups(backups, 1);
    TEST_ASSERT_FALSE(fixture_error);
    TEST_ASSERT_EQUAL_UINT(1, swaps);
    assert_bytes(old_path, "replacement-parent-old");
    assert_bytes(newer_path, "replacement-parent-newer");
    char inspected[384];
    snprintf(inspected, sizeof(inspected), "%s/20240101T000000Z.sqlite3", moved);
    TEST_ASSERT_FALSE(exists(inspected));
    snprintf(inspected, sizeof(inspected), "%s/20240102T000000Z.sqlite3", moved);
    assert_bytes(inspected, "inspected-newer");
    TEST_ASSERT_EQUAL_INT(0, result);
}

void test_changed_regular_leaf_is_retained_after_inspection(void) {
    swap_kind = 2;
    int result = prune_timestamped_backups(backups, 1);
    TEST_ASSERT_FALSE(fixture_error);
    TEST_ASSERT_EQUAL_UINT(1, swaps);
    assert_bytes(old_path, "replacement-leaf");
    assert_bytes(saved_old, "inspected-old");
    assert_bytes(newer_path, "inspected-newer");
    TEST_ASSERT_NOT_EQUAL(0, result);
}

void test_changed_symlink_leaf_and_target_are_retained(void) {
    swap_kind = 3;
    int result = prune_timestamped_backups(backups, 1);
    TEST_ASSERT_FALSE(fixture_error);
    TEST_ASSERT_EQUAL_UINT(1, swaps);
    struct stat info;
    TEST_ASSERT_EQUAL_INT(0, lstat(old_path, &info));
    TEST_ASSERT_TRUE(S_ISLNK(info.st_mode));
    assert_bytes(target, "outside-target");
    assert_bytes(saved_old, "inspected-old");
    TEST_ASSERT_NOT_EQUAL(0, result);
}

int main(void) {
    init_logger();
    UNITY_BEGIN();
    RUN_TEST(test_retention_keeps_newest_and_excludes_unpublished_artifacts);
    RUN_TEST(test_retention_within_limit_preserves_all_bytes);
    RUN_TEST(test_parent_replacement_cannot_redirect_retention_unlink);
    RUN_TEST(test_changed_regular_leaf_is_retained_after_inspection);
    RUN_TEST(test_changed_symlink_leaf_and_target_are_retained);
    int result = UNITY_END();
    shutdown_logger();
    return result;
}
