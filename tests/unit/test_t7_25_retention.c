#define _GNU_SOURCE
/* Exercise the private retention helper, without changing production visibility
 * or opening an application database. The production implementation is intact. */
#include "../../src/database/db_core.c"

#include "unity.h"

static char root[128], backups[256], moved[256], old_path[384], newer_path[384];
static char saved_old[256], target[256];
static unsigned inspected_files, swaps;
static int swap_kind;
static bool fixture_error, active;
static struct stat old_identity, newer_identity;
static unsigned seen_mask, candidate_stats, candidate_opens, fault_hits, pins_at_swap;
static int owned_fds[256], owned_roles[256], owned_count, private_fd;
static DIR *held_directory;
static bool repeat_name, repeated_name, repeat_ready;
static struct dirent repeat_entry;
static int fail_open_after, fail_stat_after, move_error;
static bool fail_realloc, fail_strdup, fail_read, fail_delete, restore_collision, change_after_move;

static unsigned role_of(int fd) {
    for (int i = 0; i < owned_count; ++i) if (owned_fds[i] == fd) return (unsigned)owned_roles[i];
    return 0;
}
static unsigned pin_count(void) {
    unsigned n = 0;
    for (int i = 0; i < owned_count; ++i) if (owned_roles[i] == 1) ++n;
    return n;
}

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
    pins_at_swap = pin_count();
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

static void inspected(const struct stat *info) {
    candidate_stats++;
    if (!swap_kind) return;
    unsigned bit = same_backup_inode(info, &old_identity) ? 1u :
                   same_backup_inode(info, &newer_identity) ? 2u : 0u;
    if (bit && !(seen_mask & bit)) {
        seen_mask |= bit; inspected_files++;
        if (seen_mask == 3) replace_after_inspection();
    }
}
int __real_stat(const char *, struct stat *);
int __wrap_stat(const char *path, struct stat *info) {
    int result = __real_stat(path, info);
    if (result == 0 && active && (!strcmp(path, old_path) || !strcmp(path, newer_path))) inspected(info);
    return result;
}
int __real_openat(int, const char *, int, ...);
int __wrap_openat(int fd, const char *name, int flags, ...) {
    mode_t mode = 0;
    if (flags & O_CREAT) { va_list a; va_start(a, flags); mode = va_arg(a, int); va_end(a); }
    bool candidate = active && has_suffix(name, ".sqlite3") && !(flags & O_DIRECTORY);
    if (candidate && fail_open_after >= 0 && (int)candidate_opens == fail_open_after) {
        fault_hits++; errno = EMFILE; return -1;
    }
    int result = __real_openat(fd, name, flags, mode);
    if (result >= 0 && active) {
        if (owned_count >= 256) { fixture_error = true; close(result); errno = EMFILE; return -1; }
        owned_fds[owned_count] = result;
        owned_roles[owned_count++] = candidate ? 1 : 2;
        if (candidate) candidate_opens++;
        if (flags & O_DIRECTORY) private_fd = result;
    }
    return result;
}
int __real_close(int);
int __wrap_close(int fd) {
    int result = __real_close(fd);
    for (int i = 0; i < owned_count; ++i) if (owned_fds[i] == fd) {
        owned_fds[i] = owned_fds[--owned_count]; owned_roles[i] = owned_roles[owned_count]; break;
    }
    if (fd == private_fd) private_fd = -1;
    return result;
}
int __real_fstat(int, struct stat *);
int __wrap_fstat(int fd, struct stat *info) {
    if (active && role_of(fd) == 1 && fail_stat_after >= 0 && (int)candidate_stats == fail_stat_after) {
        fault_hits++; errno = EIO; return -1;
    }
    int result = __real_fstat(fd, info);
    if (result == 0 && active && role_of(fd) == 1) inspected(info);
    return result;
}
DIR *__real_opendir(const char *);
DIR *__wrap_opendir(const char *path) {
    DIR *d = __real_opendir(path);
    if (active && d && !strcmp(path, backups)) held_directory = d;
    return d;
}
int __real_closedir(DIR *);
int __wrap_closedir(DIR *d) {
    if (d == held_directory) held_directory = NULL;
    return __real_closedir(d);
}
struct dirent *__real_readdir(DIR *);
struct dirent *__wrap_readdir(DIR *d) {
    if (active && fail_read && candidate_stats) { fault_hits++; errno = EIO; return NULL; }
    struct dirent *entry = __real_readdir(d);
    if (active && repeat_name) {
        if (entry && !strcmp(entry->d_name, "20240102T000000Z.sqlite3")) {
            repeat_entry = *entry; repeat_ready = true;
        } else if (!entry && repeat_ready && !repeated_name) {
            repeated_name = true; return &repeat_entry;
        }
    }
    return entry;
}
void *__real_realloc(void *, size_t);
void *__wrap_realloc(void *p, size_t n) {
    if (active && fail_realloc) { fail_realloc = false; fault_hits++; errno = ENOMEM; return NULL; }
    return __real_realloc(p, n);
}
char *__real_strdup(const char *);
char *__wrap_strdup(const char *p) {
    if (active && fail_strdup) { fail_strdup = false; fault_hits++; errno = ENOMEM; return NULL; }
    return __real_strdup(p);
}
long __real_syscall(long, ...);
long __wrap_syscall(long number, ...) {
    if (number != SYS_renameat2) { fixture_error = true; errno = ENOSYS; return -1; }
    va_list a; va_start(a, number);
    int from = va_arg(a, int); const char *name = va_arg(a, const char *);
    int to = va_arg(a, int); const char *dest = va_arg(a, const char *);
    unsigned flags = va_arg(a, unsigned); va_end(a);
    if (flags != 1U) { fixture_error = true; errno = EINVAL; return -1; }
    if (active && to == private_fd && move_error) { fault_hits++; errno = move_error; return -1; }
    long result = __real_syscall(number, from, name, to, dest, flags);
    if (active && result == 0 && to == private_fd) {
        if (restore_collision && write_bytes(old_path, "competing-path") != 0) fixture_error = true;
        if (change_after_move) {
            int file = __real_openat(to, dest, O_WRONLY | O_APPEND | O_CLOEXEC);
            if (file < 0) fixture_error = true;
            else { if (write(file, "-changed", 8) != 8) fixture_error = true; __real_close(file); }
        }
    }
    return result;
}
int __real_unlinkat(int, const char *, int);
int __wrap_unlinkat(int fd, const char *name, int flags) {
    if (active && fd == private_fd && flags == 0 && fail_delete) {
        fault_hits++; errno = EIO; return -1;
    }
    return __real_unlinkat(fd, name, flags);
}
static int prune(int count) {
    active = true;
    int result = prune_timestamped_backups(backups, count);
    active = false;
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
    swaps = inspected_files = seen_mask = candidate_stats = candidate_opens = fault_hits = pins_at_swap = 0;
    owned_count = 0; private_fd = -1; held_directory = NULL; active = false;
    fail_open_after = fail_stat_after = -1; move_error = 0;
    fail_realloc = fail_strdup = fail_read = fail_delete = restore_collision = change_after_move = false;
    repeat_name = repeated_name = repeat_ready = false;
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
    TEST_ASSERT_EQUAL_INT(0, lstat(old_path, &old_identity));
    TEST_ASSERT_EQUAL_INT(0, lstat(newer_path, &newer_identity));
}

void tearDown(void) {
    swap_kind = 0; active = false;
    int leaked = owned_count;
    while (owned_count) __real_close(owned_fds[--owned_count]);
    bool dir_leaked = held_directory != NULL;
    if (held_directory) { __real_closedir(held_directory); held_directory = NULL; }
    int cleanup = remove_owned_tree(root);
    TEST_ASSERT_FALSE(fixture_error);
    TEST_ASSERT_EQUAL_INT(0, leaked);
    TEST_ASSERT_FALSE(dir_leaked);
    TEST_ASSERT_EQUAL_INT(0, cleanup);
}

void test_retention_keeps_newest_and_excludes_unpublished_artifacts(void) {
    char temporary[384], note[384];
    snprintf(temporary, sizeof(temporary), "%s/20230101T000000Z.sqlite3.partial", backups);
    snprintf(note, sizeof(note), "%s/operator-note.txt", backups);
    TEST_ASSERT_EQUAL_INT(0, write_bytes(temporary, "unfinished"));
    TEST_ASSERT_EQUAL_INT(0, write_bytes(note, "note"));
    TEST_ASSERT_EQUAL_INT(0, prune(1));
    TEST_ASSERT_FALSE(exists(old_path));
    assert_bytes(newer_path, "inspected-newer");
    assert_bytes(temporary, "unfinished");
    assert_bytes(note, "note");
}

void test_retention_within_limit_preserves_all_bytes(void) {
    TEST_ASSERT_EQUAL_INT(0, prune(2));
    assert_bytes(old_path, "inspected-old");
    assert_bytes(newer_path, "inspected-newer");
}

void test_parent_replacement_cannot_redirect_retention_unlink(void) {
    swap_kind = 1;
    int result = prune(1);
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
    int result = prune(1);
    TEST_ASSERT_FALSE(fixture_error);
    TEST_ASSERT_EQUAL_UINT(1, swaps);
    assert_bytes(old_path, "replacement-leaf");
    assert_bytes(saved_old, "inspected-old");
    assert_bytes(newer_path, "inspected-newer");
    TEST_ASSERT_NOT_EQUAL(0, result);
}

void test_changed_symlink_leaf_and_target_are_retained(void) {
    swap_kind = 3;
    int result = prune(1);
    TEST_ASSERT_FALSE(fixture_error);
    TEST_ASSERT_EQUAL_UINT(1, swaps);
    struct stat info;
    TEST_ASSERT_EQUAL_INT(0, lstat(old_path, &info));
    TEST_ASSERT_TRUE(S_ISLNK(info.st_mode));
    assert_bytes(target, "outside-target");
    assert_bytes(saved_old, "inspected-old");
    TEST_ASSERT_NOT_EQUAL(0, result);
}

static void assert_both_originals(void) {
    assert_bytes(old_path, "inspected-old");
    assert_bytes(newer_path, "inspected-newer");
}
static void test_collection_emfile_preserves_all_and_closes_pins(void) {
    fail_open_after = 1;
    TEST_ASSERT_NOT_EQUAL(0, prune(1));
    TEST_ASSERT_EQUAL_UINT(1, fault_hits);
    TEST_ASSERT_EQUAL_UINT(1, candidate_opens);
    assert_both_originals();
}
static void test_collection_stat_failure_preserves_all(void) {
    fail_stat_after = 1;
    TEST_ASSERT_NOT_EQUAL(0, prune(1));
    TEST_ASSERT_EQUAL_UINT(1, fault_hits);
    assert_both_originals();
}
static void test_collection_realloc_failure_preserves_all(void) {
    fail_realloc = true;
    TEST_ASSERT_NOT_EQUAL(0, prune(1));
    TEST_ASSERT_EQUAL_UINT(1, fault_hits);
    assert_both_originals();
}
static void test_collection_strdup_failure_preserves_all(void) {
    fail_strdup = true;
    TEST_ASSERT_NOT_EQUAL(0, prune(1));
    TEST_ASSERT_EQUAL_UINT(1, fault_hits);
    assert_both_originals();
}
static void test_readdir_error_does_not_turn_partial_collection_into_retention(void) {
    fail_read = true;
    TEST_ASSERT_NOT_EQUAL(0, prune(0));
    TEST_ASSERT_EQUAL_UINT(1, fault_hits);
    assert_both_originals();
}
static void test_unsupported_move_preserves_published_backup(void) {
    move_error = ENOSYS;
    TEST_ASSERT_NOT_EQUAL(0, prune(1));
    TEST_ASSERT_EQUAL_UINT(1, fault_hits);
    assert_both_originals();
}
static void test_unlink_failure_restores_without_overwrite(void) {
    fail_delete = true;
    TEST_ASSERT_NOT_EQUAL(0, prune(1));
    TEST_ASSERT_EQUAL_UINT(1, fault_hits);
    assert_both_originals();
}
static void test_restore_collision_retains_quarantined_original(void) {
    fail_delete = true; restore_collision = true;
    TEST_ASSERT_NOT_EQUAL(0, prune(1));
    TEST_ASSERT_EQUAL_UINT(1, fault_hits);
    assert_bytes(old_path, "competing-path");
    assert_bytes(newer_path, "inspected-newer");
    DIR *d = opendir(backups); TEST_ASSERT_NOT_NULL(d);
    char retained[768] = {0}; unsigned found = 0; struct dirent *e;
    while ((e = readdir(d))) if (!strncmp(e->d_name, ".cleanup-", 9)) {
        snprintf(retained, sizeof(retained), "%s/%s/20240101T000000Z.sqlite3", backups, e->d_name); found++;
    }
    TEST_ASSERT_EQUAL_INT(0, closedir(d));
    TEST_ASSERT_EQUAL_UINT(1, found);
    assert_bytes(retained, "inspected-old");
}
static void test_changed_after_move_is_restored_and_pass_stops(void) {
    change_after_move = true;
    TEST_ASSERT_NOT_EQUAL(0, prune(0));
    assert_bytes(old_path, "inspected-old-changed");
    assert_bytes(newer_path, "inspected-newer");
}
static void test_both_selection_inodes_remain_pinned_during_replacement(void) {
    swap_kind = 2;
    TEST_ASSERT_NOT_EQUAL(0, prune(1));
    TEST_ASSERT_EQUAL_UINT(1, swaps);
    TEST_ASSERT_EQUAL_UINT(2, pins_at_swap);
    assert_bytes(old_path, "replacement-leaf");
    assert_bytes(saved_old, "inspected-old");
}
static void test_more_than_sixty_four_backups_preserve_configured_order(void) {
    for (unsigned i = 0; i < 70; ++i) {
        char path[384]; snprintf(path, sizeof(path), "%s/20230101T%06uZ.sqlite3", backups, i);
        TEST_ASSERT_EQUAL_INT(0, write_bytes(path, "older"));
    }
    TEST_ASSERT_EQUAL_INT(0, prune(2));
    assert_both_originals();
    for (unsigned i = 0; i < 70; ++i) {
        char path[384]; snprintf(path, sizeof(path), "%s/20230101T%06uZ.sqlite3", backups, i);
        TEST_ASSERT_FALSE(exists(path));
    }
}
static void test_published_suffix_symlink_and_fifo_are_not_counted_or_removed(void) {
    char link[384], fifo[384];
    snprintf(link, sizeof(link), "%s/19000101T000000Z.sqlite3", backups);
    snprintf(fifo, sizeof(fifo), "%s/19000102T000000Z.sqlite3", backups);
    TEST_ASSERT_EQUAL_INT(0, symlink(target, link));
    TEST_ASSERT_EQUAL_INT(0, mkfifo(fifo, 0600));
    TEST_ASSERT_EQUAL_INT(0, prune(2));
    assert_both_originals();
    struct stat st;
    TEST_ASSERT_EQUAL_INT(0, lstat(link, &st)); TEST_ASSERT_TRUE(S_ISLNK(st.st_mode));
    TEST_ASSERT_EQUAL_INT(0, lstat(fifo, &st)); TEST_ASSERT_TRUE(S_ISFIFO(st.st_mode));
    assert_bytes(target, "outside-target");
}

static void test_repeated_selected_name_aborts_before_any_deletion(void) {
    repeat_name = true;
    int result = prune(1);
    TEST_ASSERT_TRUE(repeated_name);
    assert_both_originals();
    TEST_ASSERT_NOT_EQUAL(0, result);
}
static void test_distinct_hardlinked_names_keep_filename_count_semantics(void) {
    char additional[384];
    snprintf(additional, sizeof(additional), "%s/20240103T000000Z.sqlite3", backups);
    TEST_ASSERT_EQUAL_INT(0, link(newer_path, additional));
    TEST_ASSERT_EQUAL_INT(0, prune(1));
    TEST_ASSERT_FALSE(exists(old_path));
    TEST_ASSERT_FALSE(exists(newer_path));
    assert_bytes(additional, "inspected-newer");
}

int main(void) {
    init_logger();
    UNITY_BEGIN();
    RUN_TEST(test_retention_keeps_newest_and_excludes_unpublished_artifacts);
    RUN_TEST(test_retention_within_limit_preserves_all_bytes);
    RUN_TEST(test_parent_replacement_cannot_redirect_retention_unlink);
    RUN_TEST(test_changed_regular_leaf_is_retained_after_inspection);
    RUN_TEST(test_changed_symlink_leaf_and_target_are_retained);
    RUN_TEST(test_collection_emfile_preserves_all_and_closes_pins);
    RUN_TEST(test_collection_stat_failure_preserves_all);
    RUN_TEST(test_collection_realloc_failure_preserves_all);
    RUN_TEST(test_collection_strdup_failure_preserves_all);
    RUN_TEST(test_readdir_error_does_not_turn_partial_collection_into_retention);
    RUN_TEST(test_unsupported_move_preserves_published_backup);
    RUN_TEST(test_unlink_failure_restores_without_overwrite);
    RUN_TEST(test_restore_collision_retains_quarantined_original);
    RUN_TEST(test_changed_after_move_is_restored_and_pass_stops);
    RUN_TEST(test_both_selection_inodes_remain_pinned_during_replacement);
    RUN_TEST(test_more_than_sixty_four_backups_preserve_configured_order);
    RUN_TEST(test_published_suffix_symlink_and_fifo_are_not_counted_or_removed);
    RUN_TEST(test_repeated_selected_name_aborts_before_any_deletion);
    RUN_TEST(test_distinct_hardlinked_names_keep_filename_count_semantics);
    int result = UNITY_END();
    shutdown_logger();
    return result;
}
