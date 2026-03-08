#define _POSIX_C_SOURCE 200809L
#define _GNU_SOURCE

#include <dirent.h>
#include <fcntl.h>
#include <limits.h>
#include <sqlite3.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/stat.h>
#include <time.h>
#include <unistd.h>

#include "unity.h"
#include "database/db_core.h"
#include "database/db_recordings.h"
#include "database/db_streams.h"
#include "storage/storage_manager.h"

#define TEST_DB_PATH "/tmp/lightnvr_unit_storage_manager_retention.db"

static char g_storage_root[PATH_MAX];

static void clear_tables(void) {
    sqlite3 *db = get_db_handle();
    sqlite3_exec(db, "DELETE FROM recordings;", NULL, NULL, NULL);
    sqlite3_exec(db, "DELETE FROM streams;", NULL, NULL, NULL);
}

static void remove_tree(const char *path) {
    DIR *dir = opendir(path);
    if (!dir) { rmdir(path); return; }
    struct dirent *entry;
    while ((entry = readdir(dir)) != NULL) {
        if (strcmp(entry->d_name, ".") == 0 || strcmp(entry->d_name, "..") == 0) continue;
        char child[PATH_MAX];
        snprintf(child, sizeof(child), "%s/%s", path, entry->d_name);
        struct stat st;
        if (stat(child, &st) == 0 && S_ISDIR(st.st_mode)) remove_tree(child); else unlink(child);
    }
    closedir(dir);
    rmdir(path);
}

static void mp4_path(char *out, size_t out_size, const char *name) {
    size_t root_len = strlen(g_storage_root);
    size_t name_len = strlen(name);
    TEST_ASSERT_TRUE(root_len + 6 + name_len <= out_size);
    memcpy(out, g_storage_root, root_len);
    memcpy(out + root_len, "/mp4/", 5);
    memcpy(out + root_len + 5, name, name_len + 1);
}

static void create_mp4_dir(void) {
    char dir[PATH_MAX];
    size_t root_len = strlen(g_storage_root);
    TEST_ASSERT_TRUE(root_len + 5 <= sizeof(dir));
    memcpy(dir, g_storage_root, root_len);
    memcpy(dir + root_len, "/mp4", 5);
    TEST_ASSERT_TRUE(mkdir(dir, 0755) == 0 || access(dir, F_OK) == 0);
}

static void create_file(const char *path, size_t size) {
    int fd = open(path, O_CREAT | O_TRUNC | O_WRONLY, 0644);
    TEST_ASSERT_TRUE(fd >= 0);
    TEST_ASSERT_EQUAL_INT(0, ftruncate(fd, (off_t)size));
    close(fd);
}

static stream_config_t make_stream(const char *name) {
    stream_config_t s;
    memset(&s, 0, sizeof(s));
    strncpy(s.name, name, sizeof(s.name) - 1);
    strncpy(s.url, "rtsp://camera/stream", sizeof(s.url) - 1);
    strncpy(s.codec, "h264", sizeof(s.codec) - 1);
    s.enabled = true;
    s.streaming_enabled = true;
    s.record = true;
    s.width = 1920;
    s.height = 1080;
    s.fps = 25;
    s.segment_duration = 60;
    return s;
}

static void add_stream_with_quota(const char *name, int max_storage_mb) {
    stream_config_t s = make_stream(name);
    TEST_ASSERT_NOT_EQUAL(0, add_stream_config(&s));
    stream_retention_config_t cfg = {.retention_days = 0, .detection_retention_days = 0, .max_storage_mb = max_storage_mb};
    TEST_ASSERT_EQUAL_INT(0, set_stream_retention_config(name, &cfg));
}

static recording_metadata_t make_recording(const char *stream, const char *path, time_t start, uint64_t size_bytes) {
    recording_metadata_t m;
    memset(&m, 0, sizeof(m));
    strncpy(m.stream_name, stream, sizeof(m.stream_name) - 1);
    strncpy(m.file_path, path, sizeof(m.file_path) - 1);
    strncpy(m.codec, "h264", sizeof(m.codec) - 1);
    strncpy(m.trigger_type, "scheduled", sizeof(m.trigger_type) - 1);
    m.start_time = start;
    m.end_time = start + 60;
    m.size_bytes = size_bytes;
    m.width = 1920;
    m.height = 1080;
    m.fps = 30;
    m.is_complete = true;
    m.retention_override_days = -1;
    m.retention_tier = RETENTION_TIER_STANDARD;
    m.disk_pressure_eligible = true;
    return m;
}

static int count_recordings(void) {
    sqlite3_stmt *stmt = NULL;
    int count = -1;
    sqlite3_prepare_v2(get_db_handle(), "SELECT COUNT(*) FROM recordings;", -1, &stmt, NULL);
    if (sqlite3_step(stmt) == SQLITE_ROW) count = sqlite3_column_int(stmt, 0);
    sqlite3_finalize(stmt);
    return count;
}

void setUp(void) {
    clear_tables();
    strcpy(g_storage_root, "/tmp/lightnvr_storage_manager_retentionXXXXXX");
    TEST_ASSERT_NOT_NULL(mkdtemp(g_storage_root));
    TEST_ASSERT_EQUAL_INT(0, init_storage_manager(g_storage_root, 0));
    TEST_ASSERT_EQUAL_INT(0, set_retention_days(0));
    TEST_ASSERT_EQUAL_INT(0, set_max_storage_size(0));
}

void tearDown(void) {
    shutdown_storage_manager();
    remove_tree(g_storage_root);
    g_storage_root[0] = '\0';
}

void test_apply_retention_policy_enforces_quota_with_oldest_eligible_first(void) {
    time_t now = time(NULL);
    char protected_path[PATH_MAX], delete_path[PATH_MAX], keep_path[PATH_MAX];
    recording_metadata_t protected_rec, delete_rec, keep_rec;
    create_mp4_dir();
    add_stream_with_quota("quota_cam", 2);
    mp4_path(protected_path, sizeof(protected_path), "protected-oldest.mp4");
    mp4_path(delete_path, sizeof(delete_path), "delete-middle.mp4");
    mp4_path(keep_path, sizeof(keep_path), "keep-newest.mp4");
    create_file(protected_path, 1024 * 1024);
    create_file(delete_path, 1024 * 1024);
    create_file(keep_path, 1024 * 1024);

    protected_rec = make_recording("quota_cam", protected_path, now - 300, 1024 * 1024);
    delete_rec = make_recording("quota_cam", delete_path, now - 200, 1024 * 1024);
    keep_rec = make_recording("quota_cam", keep_path, now - 100, 1024 * 1024);

    uint64_t protected_id = add_recording_metadata(&protected_rec);
    uint64_t delete_id = add_recording_metadata(&delete_rec);
    uint64_t keep_id = add_recording_metadata(&keep_rec);
    TEST_ASSERT_NOT_EQUAL(0, protected_id);
    TEST_ASSERT_NOT_EQUAL(0, delete_id);
    TEST_ASSERT_NOT_EQUAL(0, keep_id);
    TEST_ASSERT_EQUAL_INT(0, set_recording_protected(protected_id, true));

    TEST_ASSERT_EQUAL_INT(1, apply_retention_policy());
    TEST_ASSERT_EQUAL_INT(-1, access(delete_path, F_OK));
    TEST_ASSERT_EQUAL_INT(0, access(protected_path, F_OK));
    TEST_ASSERT_EQUAL_INT(0, access(keep_path, F_OK));
    recording_metadata_t meta;
    TEST_ASSERT_NOT_EQUAL(0, get_recording_metadata_by_path(delete_path, &meta));
    TEST_ASSERT_EQUAL_INT(0, get_recording_metadata_by_path(protected_path, &meta));
    TEST_ASSERT_EQUAL_INT(0, get_recording_metadata_by_path(keep_path, &meta));
}

void test_apply_retention_policy_preserves_metadata_when_file_delete_fails(void) {
    time_t now = time(NULL);
    char blocked_path[PATH_MAX];
    recording_metadata_t rec, meta;
    struct stat st;

    create_mp4_dir();
    add_stream_with_quota("blocked_cam", 1);
    mp4_path(blocked_path, sizeof(blocked_path), "quota-blocked.mp4");
    TEST_ASSERT_EQUAL_INT(0, mkdir(blocked_path, 0755));

    rec = make_recording("blocked_cam", blocked_path, now - 300, 2 * 1024 * 1024);
    TEST_ASSERT_NOT_EQUAL(0, add_recording_metadata(&rec));

    TEST_ASSERT_EQUAL_INT(0, apply_retention_policy());
    TEST_ASSERT_EQUAL_INT(0, stat(blocked_path, &st));
    TEST_ASSERT_TRUE(S_ISDIR(st.st_mode));
    TEST_ASSERT_EQUAL_INT(0, get_recording_metadata_by_path(blocked_path, &meta));
}

void test_apply_retention_policy_skips_orphan_cleanup_when_ratio_is_too_high(void) {
    time_t now = time(NULL);
    create_mp4_dir();
    add_stream_with_quota("orphan_cam", 0);

    for (int i = 0; i < 10; i++) {
        char path[PATH_MAX], name[32];
        recording_metadata_t rec;
        snprintf(name, sizeof(name), "high-ratio-%02d.mp4", i);
        mp4_path(path, sizeof(path), name);
        if (i < 4) create_file(path, 128);
        rec = make_recording("orphan_cam", path, now - (1000 + i), 128);
        TEST_ASSERT_NOT_EQUAL(0, add_recording_metadata(&rec));
    }

    TEST_ASSERT_EQUAL_INT(0, apply_retention_policy());
    TEST_ASSERT_EQUAL_INT(10, count_recordings());
}

void test_apply_retention_policy_cleans_low_ratio_orphans_when_storage_is_healthy(void) {
    time_t now = time(NULL);
    char missing_path[PATH_MAX] = "", existing_path[PATH_MAX] = "";
    create_mp4_dir();
    add_stream_with_quota("healthy_cam", 0);

    for (int i = 0; i < 10; i++) {
        char path[PATH_MAX], name[32];
        recording_metadata_t rec;
        snprintf(name, sizeof(name), "low-ratio-%02d.mp4", i);
        mp4_path(path, sizeof(path), name);
        if (i < 8) create_file(path, 128);
        if (i == 1) memcpy(existing_path, path, strlen(path) + 1);
        if (i == 9) memcpy(missing_path, path, strlen(path) + 1);
        rec = make_recording("healthy_cam", path, now - (1000 + i), 128);
        TEST_ASSERT_NOT_EQUAL(0, add_recording_metadata(&rec));
    }

    TEST_ASSERT_EQUAL_INT(0, apply_retention_policy());
    TEST_ASSERT_EQUAL_INT(8, count_recordings());
    recording_metadata_t meta;
    TEST_ASSERT_EQUAL_INT(0, get_recording_metadata_by_path(existing_path, &meta));
    TEST_ASSERT_NOT_EQUAL(0, get_recording_metadata_by_path(missing_path, &meta));
}

void test_apply_retention_policy_skips_orphans_when_mp4_storage_is_inaccessible(void) {
    time_t now = time(NULL);
    add_stream_with_quota("offline_cam", 0);

    for (int i = 0; i < 10; i++) {
        char path[PATH_MAX], name[32];
        recording_metadata_t rec;
        snprintf(name, sizeof(name), "offline-%02d.mp4", i);
        mp4_path(path, sizeof(path), name);
        rec = make_recording("offline_cam", path, now - (1000 + i), 128);
        TEST_ASSERT_NOT_EQUAL(0, add_recording_metadata(&rec));
    }

    TEST_ASSERT_EQUAL_INT(0, apply_retention_policy());
    TEST_ASSERT_EQUAL_INT(10, count_recordings());
}

int main(void) {
    unlink(TEST_DB_PATH);
    if (init_database(TEST_DB_PATH) != 0) return 1;

    UNITY_BEGIN();
    RUN_TEST(test_apply_retention_policy_enforces_quota_with_oldest_eligible_first);
    RUN_TEST(test_apply_retention_policy_preserves_metadata_when_file_delete_fails);
    RUN_TEST(test_apply_retention_policy_skips_orphan_cleanup_when_ratio_is_too_high);
    RUN_TEST(test_apply_retention_policy_cleans_low_ratio_orphans_when_storage_is_healthy);
    RUN_TEST(test_apply_retention_policy_skips_orphans_when_mp4_storage_is_inaccessible);
    int result = UNITY_END();

    shutdown_database();
    unlink(TEST_DB_PATH);
    return result;
}