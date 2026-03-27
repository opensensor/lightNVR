/**
 * @file test_db_recording_tags.c
 * @brief Layer 2 Unity tests for database/db_recording_tags.c
 *
 * Exercises the full recording-tag API:
 *   - add / get / remove a single tag
 *   - set (replace-all) tags
 *   - batch add / remove
 *   - get_all_unique
 *   - get_recordings_by_tag
 *   - whitespace trimming
 *   - duplicate-tag deduplication
 *   - null / empty input guards
 */

#define _POSIX_C_SOURCE 200809L
#define _GNU_SOURCE

#include <stdio.h>
#include <string.h>
#include <time.h>
#include <unistd.h>
#include <sqlite3.h>

#include "unity.h"
#include "database/db_core.h"
#include "database/db_recordings.h"
#include "database/db_recording_tags.h"

#define TEST_DB_PATH "/tmp/lightnvr_unit_recording_tags_test.db"

/* ------------------------------------------------------------------ helpers */

static recording_metadata_t make_rec(const char *stream, const char *path, time_t start) {
    recording_metadata_t m;
    memset(&m, 0, sizeof(m));
    strncpy(m.stream_name, stream, sizeof(m.stream_name) - 1);
    strncpy(m.file_path,   path,   sizeof(m.file_path)   - 1);
    strncpy(m.codec,       "h264", sizeof(m.codec)       - 1);
    strncpy(m.trigger_type, "scheduled", sizeof(m.trigger_type) - 1);
    m.start_time  = start;
    m.end_time    = start + 60;
    m.size_bytes  = 1024 * 1024;
    m.width = 1920; m.height = 1080; m.fps = 30;
    m.is_complete = true;
    m.protected   = false;
    m.retention_override_days = -1;
    m.disk_pressure_eligible  = true;
    return m;
}

static void clear_all(void) {
    sqlite3 *db = get_db_handle();
    sqlite3_exec(db, "DELETE FROM recording_tags;", NULL, NULL, NULL);
    sqlite3_exec(db, "DELETE FROM recordings;",    NULL, NULL, NULL);
}

void setUp(void)    { clear_all(); }
void tearDown(void) {}

/* ------------------------------------------------------------------ tests */

void test_add_tag_returns_zero(void) {
    time_t now = time(NULL);
    recording_metadata_t m = make_rec("cam1", "/rec/tag1.mp4", now);
    uint64_t id = add_recording_metadata(&m);
    TEST_ASSERT_NOT_EQUAL(0, id);

    int rc = db_recording_tag_add(id, "important");
    TEST_ASSERT_EQUAL_INT(0, rc);
}

void test_get_tags_returns_added_tag(void) {
    time_t now = time(NULL);
    recording_metadata_t m = make_rec("cam1", "/rec/tag2.mp4", now);
    uint64_t id = add_recording_metadata(&m);
    db_recording_tag_add(id, "review");

    char tags[10][MAX_TAG_LENGTH];
    int n = db_recording_tag_get(id, tags, 10);
    TEST_ASSERT_EQUAL_INT(1, n);
    TEST_ASSERT_EQUAL_STRING("review", tags[0]);
}

void test_duplicate_tag_ignored(void) {
    time_t now = time(NULL);
    recording_metadata_t m = make_rec("cam1", "/rec/tag3.mp4", now);
    uint64_t id = add_recording_metadata(&m);
    db_recording_tag_add(id, "dup");
    db_recording_tag_add(id, "dup"); /* second insert should be silently ignored */

    char tags[10][MAX_TAG_LENGTH];
    int n = db_recording_tag_get(id, tags, 10);
    TEST_ASSERT_EQUAL_INT(1, n);
}

void test_remove_tag(void) {
    time_t now = time(NULL);
    recording_metadata_t m = make_rec("cam1", "/rec/tag4.mp4", now);
    uint64_t id = add_recording_metadata(&m);
    db_recording_tag_add(id, "delete-me");

    int rc = db_recording_tag_remove(id, "delete-me");
    TEST_ASSERT_EQUAL_INT(0, rc);

    char tags[10][MAX_TAG_LENGTH];
    int n = db_recording_tag_get(id, tags, 10);
    TEST_ASSERT_EQUAL_INT(0, n);
}

void test_remove_nonexistent_tag_ok(void) {
    time_t now = time(NULL);
    recording_metadata_t m = make_rec("cam1", "/rec/tag5.mp4", now);
    uint64_t id = add_recording_metadata(&m);

    int rc = db_recording_tag_remove(id, "ghost");
    TEST_ASSERT_EQUAL_INT(0, rc);
}

void test_set_tags_replaces_all(void) {
    time_t now = time(NULL);
    recording_metadata_t m = make_rec("cam1", "/rec/tag6.mp4", now);
    uint64_t id = add_recording_metadata(&m);
    db_recording_tag_add(id, "old");

    const char *new_tags[] = {"alpha", "beta", "gamma"};
    int rc = db_recording_tag_set(id, new_tags, 3);
    TEST_ASSERT_EQUAL_INT(0, rc);

    char out[10][MAX_TAG_LENGTH];
    int n = db_recording_tag_get(id, out, 10);
    TEST_ASSERT_EQUAL_INT(3, n);
}

void test_set_tags_empty_clears_all(void) {
    time_t now = time(NULL);
    recording_metadata_t m = make_rec("cam1", "/rec/tag7.mp4", now);
    uint64_t id = add_recording_metadata(&m);
    db_recording_tag_add(id, "keep");

    int rc = db_recording_tag_set(id, NULL, 0);
    TEST_ASSERT_EQUAL_INT(0, rc);

    char out[10][MAX_TAG_LENGTH];
    int n = db_recording_tag_get(id, out, 10);
    TEST_ASSERT_EQUAL_INT(0, n);
}

void test_get_all_unique_tags(void) {
    time_t now = time(NULL);
    recording_metadata_t m1 = make_rec("cam1", "/rec/uniq1.mp4", now);
    recording_metadata_t m2 = make_rec("cam2", "/rec/uniq2.mp4", now + 60);
    uint64_t id1 = add_recording_metadata(&m1);
    uint64_t id2 = add_recording_metadata(&m2);

    db_recording_tag_add(id1, "shared");
    db_recording_tag_add(id2, "shared");
    db_recording_tag_add(id1, "unique1");
    db_recording_tag_add(id2, "unique2");

    char tags[20][MAX_TAG_LENGTH];
    int n = db_recording_tag_get_all_unique(tags, 20);
    TEST_ASSERT_EQUAL_INT(3, n); /* shared, unique1, unique2 */
}

void test_batch_add_tags(void) {
    time_t now = time(NULL);
    recording_metadata_t m1 = make_rec("cam1", "/rec/batch1.mp4", now);
    recording_metadata_t m2 = make_rec("cam2", "/rec/batch2.mp4", now + 60);
    uint64_t id1 = add_recording_metadata(&m1);
    uint64_t id2 = add_recording_metadata(&m2);

    uint64_t ids[] = {id1, id2};
    int added = db_recording_tag_batch_add(ids, 2, "batch-tag");
    TEST_ASSERT_EQUAL_INT(2, added);

    char tags[5][MAX_TAG_LENGTH];
    TEST_ASSERT_EQUAL_INT(1, db_recording_tag_get(id1, tags, 5));
    TEST_ASSERT_EQUAL_INT(1, db_recording_tag_get(id2, tags, 5));
}

void test_batch_remove_tags(void) {
    time_t now = time(NULL);
    recording_metadata_t m1 = make_rec("cam1", "/rec/brem1.mp4", now);
    recording_metadata_t m2 = make_rec("cam2", "/rec/brem2.mp4", now + 60);
    uint64_t id1 = add_recording_metadata(&m1);
    uint64_t id2 = add_recording_metadata(&m2);
    uint64_t ids[] = {id1, id2};
    db_recording_tag_batch_add(ids, 2, "remove-me");

    int removed = db_recording_tag_batch_remove(ids, 2, "remove-me");
    TEST_ASSERT_EQUAL_INT(2, removed);

    char tags[5][MAX_TAG_LENGTH];
    TEST_ASSERT_EQUAL_INT(0, db_recording_tag_get(id1, tags, 5));
}

void test_get_recordings_by_tag(void) {
    time_t now = time(NULL);
    recording_metadata_t m1 = make_rec("cam1", "/rec/bytag1.mp4", now);
    recording_metadata_t m2 = make_rec("cam2", "/rec/bytag2.mp4", now + 60);
    recording_metadata_t m3 = make_rec("cam3", "/rec/bytag3.mp4", now + 120);
    uint64_t id1 = add_recording_metadata(&m1);
    uint64_t id2 = add_recording_metadata(&m2);
    uint64_t id3 = add_recording_metadata(&m3);

    db_recording_tag_add(id1, "flagged");
    db_recording_tag_add(id2, "flagged");
    db_recording_tag_add(id3, "other");

    uint64_t out[10];
    int n = db_recording_tag_get_recordings_by_tag("flagged", out, 10);
    TEST_ASSERT_EQUAL_INT(2, n);
}

void test_whitespace_tag_trimmed(void) {
    time_t now = time(NULL);
    recording_metadata_t m = make_rec("cam1", "/rec/ws.mp4", now);
    uint64_t id = add_recording_metadata(&m);

    int rc = db_recording_tag_add(id, "  trimmed  ");
    TEST_ASSERT_EQUAL_INT(0, rc);

    char tags[5][MAX_TAG_LENGTH];
    int n = db_recording_tag_get(id, tags, 5);
    TEST_ASSERT_EQUAL_INT(1, n);
    TEST_ASSERT_EQUAL_STRING("trimmed", tags[0]);
}

void test_null_tag_add_fails(void) {
    TEST_ASSERT_EQUAL_INT(-1, db_recording_tag_add(1, NULL));
}

void test_empty_tag_add_fails(void) {
    time_t now = time(NULL);
    recording_metadata_t m = make_rec("cam1", "/rec/empty.mp4", now);
    uint64_t id = add_recording_metadata(&m);
    TEST_ASSERT_EQUAL_INT(-1, db_recording_tag_add(id, ""));
}

void test_whitespace_only_tag_add_fails(void) {
    time_t now = time(NULL);
    recording_metadata_t m = make_rec("cam1", "/rec/ws2.mp4", now);
    uint64_t id = add_recording_metadata(&m);
    TEST_ASSERT_EQUAL_INT(-1, db_recording_tag_add(id, "   "));
}

/* ------------------------------------------------------------------ main */

int main(void) {
    unlink(TEST_DB_PATH);
    if (init_database(TEST_DB_PATH) != 0) {
        fprintf(stderr, "FATAL: init_database failed\n");
        return 1;
    }

    UNITY_BEGIN();
    RUN_TEST(test_add_tag_returns_zero);
    RUN_TEST(test_get_tags_returns_added_tag);
    RUN_TEST(test_duplicate_tag_ignored);
    RUN_TEST(test_remove_tag);
    RUN_TEST(test_remove_nonexistent_tag_ok);
    RUN_TEST(test_set_tags_replaces_all);
    RUN_TEST(test_set_tags_empty_clears_all);
    RUN_TEST(test_get_all_unique_tags);
    RUN_TEST(test_batch_add_tags);
    RUN_TEST(test_batch_remove_tags);
    RUN_TEST(test_get_recordings_by_tag);
    RUN_TEST(test_whitespace_tag_trimmed);
    RUN_TEST(test_null_tag_add_fails);
    RUN_TEST(test_empty_tag_add_fails);
    RUN_TEST(test_whitespace_only_tag_add_fails);
    int result = UNITY_END();

    shutdown_database();
    unlink(TEST_DB_PATH);
    return result;
}

