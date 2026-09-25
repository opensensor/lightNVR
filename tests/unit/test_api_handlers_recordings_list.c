/**
 * @file test_api_handlers_recordings_list.c
 * @brief Layer 2 — GET /api/recordings exact-total cache and handler integration
 *
 * The page total of /api/recordings is an exact COUNT(*) that is cached for
 * 10 s per filter signature and dropped whenever db_recordings_generation()
 * moves. The cache takes its clock and generation as parameters so the first
 * four tests drive it deterministically; the last one exercises the handler
 * end to end with authentication disabled.
 */

#define _POSIX_C_SOURCE 200809L
#define _GNU_SOURCE

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>
#include <unistd.h>
#include <sqlite3.h>
#include <cjson/cJSON.h>

#include "unity.h"
#include "core/config.h"
#include "database/db_core.h"
#include "database/db_recordings.h"
#include "database/db_streams.h"
#include "utils/strings.h"
#include "web/api_handlers.h"
#include "web/api_handlers_recordings.h"
#include "web/request_response.h"

#define TEST_DB_PATH "/tmp/lightnvr_unit_recordings_list_test.db"

extern config_t g_config;

void setUp(void) {}
void tearDown(void) {}

static const char *scope_ab[] = {"cam_a", "cam_b"};

static char *base_key(void) {
    return recordings_count_cache_build_key(7, 1000, 2000, "cam_a", 0, NULL, -1,
                                            scope_ab, 2, NULL, NULL);
}

static void assert_key_differs_from(const char *base, char *variant) {
    TEST_ASSERT_NOT_NULL(variant);
    TEST_ASSERT_FALSE_MESSAGE(strcmp(base, variant) == 0,
                              "variant key must differ from base key");
    free(variant);
}

/* ---- cache in isolation ---- */

void test_count_cache_key_covers_every_filter_component_and_scope(void) {
    char *base = base_key();
    TEST_ASSERT_NOT_NULL(base);
    char *same = base_key();
    TEST_ASSERT_EQUAL_STRING(base, same);
    free(same);

    const char *scope_ac[] = {"cam_a", "cam_c"};
    assert_key_differs_from(base, recordings_count_cache_build_key(
        8, 1000, 2000, "cam_a", 0, NULL, -1, scope_ab, 2, NULL, NULL));
    assert_key_differs_from(base, recordings_count_cache_build_key(
        7, 1001, 2000, "cam_a", 0, NULL, -1, scope_ab, 2, NULL, NULL));
    assert_key_differs_from(base, recordings_count_cache_build_key(
        7, 1000, 2001, "cam_a", 0, NULL, -1, scope_ab, 2, NULL, NULL));
    assert_key_differs_from(base, recordings_count_cache_build_key(
        7, 1000, 2000, "cam_b", 0, NULL, -1, scope_ab, 2, NULL, NULL));
    assert_key_differs_from(base, recordings_count_cache_build_key(
        7, 1000, 2000, "cam_a", 1, NULL, -1, scope_ab, 2, NULL, NULL));
    assert_key_differs_from(base, recordings_count_cache_build_key(
        7, 1000, 2000, "cam_a", 0, "person", -1, scope_ab, 2, NULL, NULL));
    assert_key_differs_from(base, recordings_count_cache_build_key(
        7, 1000, 2000, "cam_a", 0, NULL, 1, scope_ab, 2, NULL, NULL));
    assert_key_differs_from(base, recordings_count_cache_build_key(
        7, 1000, 2000, "cam_a", 0, NULL, -1, scope_ab, 2, "review", NULL));
    assert_key_differs_from(base, recordings_count_cache_build_key(
        7, 1000, 2000, "cam_a", 0, NULL, -1, scope_ab, 2, NULL, "manual"));
    assert_key_differs_from(base, recordings_count_cache_build_key(
        7, 1000, 2000, "cam_a", 0, NULL, -1, scope_ab, 1, NULL, NULL));
    assert_key_differs_from(base, recordings_count_cache_build_key(
        7, 1000, 2000, "cam_a", 0, NULL, -1, scope_ac, 2, NULL, NULL));

    /* Fields are length-prefixed: different splits of the same bytes and
     * swapping a value between the stream predicate and the scope must not
     * collide. */
    const char *split_one[] = {"a,b", "c"};
    const char *split_two[] = {"a", "b,c"};
    char *k1 = recordings_count_cache_build_key(7, 0, 0, NULL, 0, NULL, -1,
                                                split_one, 2, NULL, NULL);
    char *k2 = recordings_count_cache_build_key(7, 0, 0, NULL, 0, NULL, -1,
                                                split_two, 2, NULL, NULL);
    TEST_ASSERT_NOT_NULL(k1);
    TEST_ASSERT_NOT_NULL(k2);
    TEST_ASSERT_FALSE(strcmp(k1, k2) == 0);
    const char *only_a[] = {"a"};
    const char *only_c[] = {"c"};
    char *k3 = recordings_count_cache_build_key(7, 0, 0, "c", 0, NULL, -1,
                                                only_a, 1, NULL, NULL);
    char *k4 = recordings_count_cache_build_key(7, 0, 0, "a", 0, NULL, -1,
                                                only_c, 1, NULL, NULL);
    TEST_ASSERT_NOT_NULL(k3);
    TEST_ASSERT_NOT_NULL(k4);
    TEST_ASSERT_FALSE(strcmp(k3, k4) == 0);
    free(k1);
    free(k2);
    free(k3);
    free(k4);

    /* Inputs that produce the same COUNT predicate share a key: NULL and ""
     * text filters, and a NULL scope versus a zero-length one. */
    char *k5 = recordings_count_cache_build_key(7, 0, 0, NULL, 0, NULL, -1,
                                                NULL, 0, NULL, NULL);
    char *k6 = recordings_count_cache_build_key(7, 0, 0, "", 0, "", -1,
                                                scope_ab, 0, "", "");
    TEST_ASSERT_NOT_NULL(k5);
    TEST_ASSERT_NOT_NULL(k6);
    TEST_ASSERT_EQUAL_STRING(k5, k6);
    free(k5);
    free(k6);
    free(base);
}

void test_count_cache_hit_within_ttl_miss_after_ttl_or_clock_rewind(void) {
    recordings_count_cache_reset();
    char *key = base_key();
    TEST_ASSERT_NOT_NULL(key);
    int total = -1;

    TEST_ASSERT_EQUAL_INT(0, recordings_count_cache_lookup(key, 1000, 1, &total));

    recordings_count_cache_store(key, 1000, 1, 42);
    TEST_ASSERT_EQUAL_INT(1, recordings_count_cache_lookup(key, 1000, 1, &total));
    TEST_ASSERT_EQUAL_INT(42, total);
    TEST_ASSERT_EQUAL_INT(1, recordings_count_cache_lookup(key, 1009, 1, &total));
    /* TTL is 10 s, exclusive. */
    TEST_ASSERT_EQUAL_INT(0, recordings_count_cache_lookup(key, 1010, 1, &total));
    /* A clock that went backwards never serves the entry. */
    TEST_ASSERT_EQUAL_INT(0, recordings_count_cache_lookup(key, 999, 1, &total));

    /* Re-storing refreshes both timestamp and value in place. */
    recordings_count_cache_store(key, 1010, 1, 43);
    total = -1;
    TEST_ASSERT_EQUAL_INT(1, recordings_count_cache_lookup(key, 1019, 1, &total));
    TEST_ASSERT_EQUAL_INT(43, total);
    free(key);
}

void test_count_cache_invalidates_on_generation_change(void) {
    recordings_count_cache_reset();
    char *key = base_key();
    TEST_ASSERT_NOT_NULL(key);
    int total = -1;

    recordings_count_cache_store(key, 5000, 3, 10);
    TEST_ASSERT_EQUAL_INT(0, recordings_count_cache_lookup(key, 5001, 4, &total));
    TEST_ASSERT_EQUAL_INT(1, recordings_count_cache_lookup(key, 5001, 3, &total));
    TEST_ASSERT_EQUAL_INT(10, total);

    recordings_count_cache_store(key, 5001, 4, 11);
    total = -1;
    TEST_ASSERT_EQUAL_INT(1, recordings_count_cache_lookup(key, 5002, 4, &total));
    TEST_ASSERT_EQUAL_INT(11, total);
    TEST_ASSERT_EQUAL_INT(0, recordings_count_cache_lookup(key, 5002, 3, &total));
    free(key);
}

void test_count_cache_skips_errors_bounds_entries_and_resets(void) {
    recordings_count_cache_reset();
    char *key = base_key();
    TEST_ASSERT_NOT_NULL(key);
    int total = -1;

    /* A failed COUNT (-1) is never cached. */
    recordings_count_cache_store(key, 100, 1, -1);
    TEST_ASSERT_EQUAL_INT(0, recordings_count_cache_lookup(key, 100, 1, &total));
    recordings_count_cache_store(key, 100, 1, 5);
    TEST_ASSERT_EQUAL_INT(1, recordings_count_cache_lookup(key, 100, 1, &total));

    recordings_count_cache_reset();
    TEST_ASSERT_EQUAL_INT(0, recordings_count_cache_lookup(key, 100, 1, &total));
    free(key);

    /* The table holds 32 entries; flooding it evicts the oldest and keeps the
     * most recent 32 without disturbing the rest. */
    char *keys[100];
    for (int i = 0; i < 100; i++) {
        keys[i] = recordings_count_cache_build_key(i, 0, 0, NULL, 0, NULL, -1,
                                                   NULL, 0, NULL, NULL);
        TEST_ASSERT_NOT_NULL(keys[i]);
        recordings_count_cache_store(keys[i], 200, 1, i);
    }
    TEST_ASSERT_EQUAL_INT(0, recordings_count_cache_lookup(keys[0], 200, 1, &total));
    TEST_ASSERT_EQUAL_INT(1, recordings_count_cache_lookup(keys[68], 200, 1, &total));
    TEST_ASSERT_EQUAL_INT(68, total);
    TEST_ASSERT_EQUAL_INT(1, recordings_count_cache_lookup(keys[99], 200, 1, &total));
    TEST_ASSERT_EQUAL_INT(99, total);
    for (int i = 0; i < 100; i++) free(keys[i]);
    recordings_count_cache_reset();
}

/* ---- handler integration ---- */

static void create_stream(const char *name) {
    stream_config_t stream;
    memset(&stream, 0, sizeof(stream));
    safe_strcpy(stream.name, name, sizeof(stream.name), 0);
    safe_strcpy(stream.url, "rtsp://camera.example/live", sizeof(stream.url), 0);
    safe_strcpy(stream.codec, "h264", sizeof(stream.codec), 0);
    stream.enabled = true;
    stream.streaming_enabled = true;
    stream.record = true;
    stream.width = 1920;
    stream.height = 1080;
    stream.fps = 25;
    stream.segment_duration = 60;
    TEST_ASSERT_NOT_EQUAL(0, add_stream_config(&stream));
}

static uint64_t create_recording(const char *stream, const char *path, time_t start) {
    recording_metadata_t m;
    memset(&m, 0, sizeof(m));
    safe_strcpy(m.stream_name, stream, sizeof(m.stream_name), 0);
    safe_strcpy(m.file_path, path, sizeof(m.file_path), 0);
    safe_strcpy(m.codec, "h264", sizeof(m.codec), 0);
    safe_strcpy(m.trigger_type, "scheduled", sizeof(m.trigger_type), 0);
    m.start_time = start;
    m.end_time = start + 60;
    m.size_bytes = 1024;
    m.is_complete = true;
    m.schedule_restricted = 1;
    m.disk_pressure_eligible = true;
    uint64_t id = add_recording_metadata(&m);
    TEST_ASSERT_NOT_EQUAL(0, id);
    return id;
}

/* Writes straight through the shared handle, bypassing db_recordings.c and
 * therefore its generation counter (an importer or migration would). */
static void raw_insert_recording(const char *stream, const char *path, time_t start) {
    sqlite3_stmt *stmt = NULL;
    TEST_ASSERT_EQUAL_INT(SQLITE_OK, sqlite3_prepare_v2(
        get_db_handle(),
        "INSERT INTO recordings (stream_name, file_path, start_time, end_time, "
        "size_bytes, codec, is_complete, trigger_type, schedule_restricted, "
        "disk_pressure_eligible) VALUES (?, ?, ?, ?, 1024, 'h264', 1, "
        "'scheduled', 1, 1);", -1, &stmt, NULL));
    sqlite3_bind_text(stmt, 1, stream, -1, SQLITE_STATIC);
    sqlite3_bind_text(stmt, 2, path, -1, SQLITE_STATIC);
    sqlite3_bind_int64(stmt, 3, (sqlite3_int64)start);
    sqlite3_bind_int64(stmt, 4, (sqlite3_int64)(start + 60));
    TEST_ASSERT_EQUAL_INT(SQLITE_DONE, sqlite3_step(stmt));
    sqlite3_finalize(stmt);
}

/* Issues GET /api/recordings?<query>, returns pagination.total and the number
 * of rows on the page. */
static int list_recordings(const char *query, int *returned) {
    http_request_t req;
    http_request_init(&req);
    req.method = HTTP_METHOD_GET;
    safe_strcpy(req.path, "/api/recordings", sizeof(req.path), 0);
    safe_strcpy(req.query_string, query, sizeof(req.query_string), 0);
    safe_strcpy(req.client_ip, "127.0.0.1", sizeof(req.client_ip), 0);

    http_response_t res;
    http_response_init(&res);
    handle_get_recordings(&req, &res);
    TEST_ASSERT_EQUAL_INT(200, res.status_code);
    TEST_ASSERT_NOT_NULL(res.body);

    cJSON *root = cJSON_Parse((const char *)res.body);
    TEST_ASSERT_NOT_NULL(root);
    cJSON *pagination = cJSON_GetObjectItem(root, "pagination");
    TEST_ASSERT_NOT_NULL(pagination);
    cJSON *total = cJSON_GetObjectItem(pagination, "total");
    cJSON *items = cJSON_GetObjectItem(root, "recordings");
    TEST_ASSERT_TRUE(cJSON_IsNumber(total));
    TEST_ASSERT_TRUE(cJSON_IsArray(items));
    int result = total->valueint;
    if (returned) *returned = cJSON_GetArraySize(items);
    cJSON_Delete(root);
    http_response_free(&res);
    return result;
}

void test_handle_get_recordings_serves_cached_total_until_generation_or_ttl_changes(void) {
    g_config.web_auth_enabled = false;
    recordings_count_cache_reset();
    create_stream("list_cam");
    time_t now = time(NULL);
    for (int i = 0; i < 3; i++) {
        char path[64];
        snprintf(path, sizeof(path), "/tmp/list-%d.mp4", i);
        create_recording("list_cam", path, now - (i + 1) * 100);
    }

    int returned = 0;
    TEST_ASSERT_EQUAL_INT(3, list_recordings("stream=list_cam&limit=2", &returned));
    TEST_ASSERT_EQUAL_INT(2, returned);

    /* A row written behind the module's back (no generation bump) is on the
     * page immediately, but the cached total holds for the TTL. The key does
     * not include page/limit, so a different page size shares the entry. */
    raw_insert_recording("list_cam", "/tmp/list-raw.mp4", now);
    TEST_ASSERT_EQUAL_INT(3, list_recordings("stream=list_cam&limit=10", &returned));
    TEST_ASSERT_EQUAL_INT(4, returned);

    /* Once the entry is gone (reset here, TTL in production) the total is
     * exact again. */
    recordings_count_cache_reset();
    TEST_ASSERT_EQUAL_INT(4, list_recordings("stream=list_cam&limit=10", &returned));
    TEST_ASSERT_EQUAL_INT(4, returned);

    /* A mutation through db_recordings.c moves the generation, so the next
     * request recounts without waiting for the TTL. */
    create_recording("list_cam", "/tmp/list-4.mp4", now + 100);
    TEST_ASSERT_EQUAL_INT(5, list_recordings("stream=list_cam&limit=10", &returned));
    TEST_ASSERT_EQUAL_INT(5, returned);

    /* limit=all sizes the page from the total, so it always recounts, while
     * the paged request keeps serving the cached total within the TTL. */
    raw_insert_recording("list_cam", "/tmp/list-raw-2.mp4", now + 200);
    TEST_ASSERT_EQUAL_INT(6, list_recordings("stream=list_cam&limit=all", &returned));
    TEST_ASSERT_EQUAL_INT(6, returned);
    TEST_ASSERT_EQUAL_INT(5, list_recordings("stream=list_cam&limit=10", NULL));

    recordings_count_cache_reset();
}

int main(void) {
    unlink(TEST_DB_PATH);
    if (init_database(TEST_DB_PATH) != 0) {
        fprintf(stderr, "FATAL: init_database failed\n");
        return 1;
    }
    UNITY_BEGIN();
    RUN_TEST(test_count_cache_key_covers_every_filter_component_and_scope);
    RUN_TEST(test_count_cache_hit_within_ttl_miss_after_ttl_or_clock_rewind);
    RUN_TEST(test_count_cache_invalidates_on_generation_change);
    RUN_TEST(test_count_cache_skips_errors_bounds_entries_and_resets);
    RUN_TEST(test_handle_get_recordings_serves_cached_total_until_generation_or_ttl_changes);
    int result = UNITY_END();
    shutdown_database();
    unlink(TEST_DB_PATH);
    return result;
}
