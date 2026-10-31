#define _POSIX_C_SOURCE 200809L
#include <float.h>
#include <stdarg.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <curl/curl.h>
#include <sqlite3.h>
#include "unity.h"
#include "database/db_core.h"
#include "database/db_query_builder.h"
#include "database/db_zones.h"
#include "video/go2rtc/go2rtc_api.h"

static sqlite3 *db;
static char *captured_url;
static unsigned requests;
static char scratch[8192], names[MAX_TRACKED_COLUMNS][64];
static sqlite3_stmt *statement;

/* Only replace the connection provider; schema checks, query assembly and
 * zone transactions/serialization are the actual production functions. */
sqlite3 *__wrap_get_db_handle(void) { return db; }

/* Real libcurl options and escaping, but no network or external camera.
 * A successful synthetic upstream records exactly the URL passed to CURL. */
CURLcode __real_curl_easy_getinfo(CURL *, CURLINFO, ...);
CURLcode __wrap_curl_easy_getinfo(CURL *curl, CURLINFO info, ...) {
    va_list args;
    va_start(args, info);
    void *value = va_arg(args, void *);
    va_end(args);
    if (info == CURLINFO_RESPONSE_CODE) {
        *(long *)value = 200;
        return CURLE_OK;
    }
    return __real_curl_easy_getinfo(curl, info, value);
}
CURLcode __wrap_curl_easy_perform(CURL *curl) {
    char *url = NULL;
    CURLcode rc = __real_curl_easy_getinfo(curl, CURLINFO_EFFECTIVE_URL, &url);
    if (rc != CURLE_OK || !url) return CURLE_URL_MALFORMAT;
    free(captured_url);
    captured_url = strdup(url);
    requests++;
    return captured_url ? CURLE_OK : CURLE_OUT_OF_MEMORY;
}
static void sql(const char *query) {
    TEST_ASSERT_EQUAL_INT_MESSAGE(SQLITE_OK,
        sqlite3_exec(db, query, NULL, NULL, NULL), sqlite3_errmsg(db));
}
void setUp(void) {
    db = NULL;
    statement = NULL;
    captured_url = NULL;
    requests = 0;
    TEST_ASSERT_EQUAL_INT(SQLITE_OK, sqlite3_open(":memory:", &db));
    sql("CREATE TABLE t(id INTEGER); INSERT INTO t VALUES(1);");
    sql("CREATE TABLE detection_zones(id TEXT PRIMARY KEY, stream_name TEXT, "
        "name TEXT, enabled INTEGER, color TEXT, polygon TEXT, filter_classes TEXT, "
        "min_confidence REAL, created_at INTEGER, updated_at INTEGER);");
    TEST_ASSERT_TRUE(go2rtc_api_init("127.0.0.1", 1984));
}
void tearDown(void) {
    sqlite3_finalize(statement);
    statement = NULL;
    if (db) sqlite3_close(db);
    db = NULL;
    go2rtc_api_cleanup();
    free(captured_url);
    captured_url = NULL;
}
static void builder(query_builder_t *qb) {
    TEST_ASSERT_EQUAL_INT(0, qb_init(qb, "t"));
    TEST_ASSERT_EQUAL_INT(0, qb_add_column(qb, "id", true));
}
static void assert_executable(const char *query, int columns) {
    TEST_ASSERT_NOT_NULL(query);
    TEST_ASSERT_EQUAL_INT(SQLITE_OK, sqlite3_prepare_v2(db, query, -1, &statement, NULL));
    TEST_ASSERT_EQUAL_INT(columns, sqlite3_column_count(statement));
    sqlite3_finalize(statement);
    statement = NULL;
}
static void test_query_valid_where_order(void) {
    query_builder_t qb;
    builder(&qb);
    const char *query = qb_build_select(&qb, "id=1", "id DESC");
    TEST_ASSERT_EQUAL_STRING("SELECT id FROM t WHERE id=1 ORDER BY id DESC;", query);
    assert_executable(query, 1);
    TEST_ASSERT_EQUAL_INT(0, qb_get_column_index(&qb, "id"));
}
static void query_where_boundary(size_t total_length, bool accepted) {
    struct { unsigned before; query_builder_t qb; unsigned after; } guarded;
    guarded.before = 0xaabbccdd;
    guarded.after = 0xaabbccdd;
    builder(&guarded.qb);
    const size_t fixed = strlen("SELECT id FROM t WHERE ;");
    size_t clause_len = total_length - fixed;
    memset(scratch, 'a', clause_len);
    memcpy(scratch, "id=1 /*", 7);
    memcpy(scratch + clause_len - 2, "*/", 2);
    scratch[clause_len] = '\0';
    const char *query = qb_build_select(&guarded.qb, scratch, NULL);
    TEST_ASSERT_EQUAL_HEX32(0xaabbccdd, guarded.before);
    TEST_ASSERT_EQUAL_HEX32(0xaabbccdd, guarded.after);
    if (accepted) {
        TEST_ASSERT_NOT_NULL(query);
        TEST_ASSERT_EQUAL_UINT(total_length, strlen(query));
        assert_executable(query, 1);
    } else TEST_ASSERT_NULL(query);
}
static void test_query_exact_buffer_fit(void) { query_where_boundary(MAX_QUERY_LEN - 1, true); }
static void test_query_one_byte_overflow_rejected(void) { query_where_boundary(MAX_QUERY_LEN, false); }
static void test_query_oversized_where_and_order_rejected(void) {
    query_builder_t qb;
    builder(&qb);
    memset(scratch, 'a', sizeof(scratch) - 1);
    scratch[sizeof(scratch) - 1] = '\0';
    TEST_ASSERT_NULL(qb_build_select(&qb, scratch, NULL));
    TEST_ASSERT_NULL(qb_build_select(&qb, NULL, scratch));
}
static void columns_table(query_builder_t *qb, int count) {
    size_t used = (size_t)snprintf(scratch, sizeof(scratch), "CREATE TABLE b(");
    for (int i = 0; i < count; i++) {
        memset(names[i], 'a', 63);
        names[i][0] = 'c';
        names[i][1] = (char)('0' + i / 10);
        names[i][2] = (char)('0' + i % 10);
        names[i][63] = '\0';
        int n = snprintf(scratch + used, sizeof(scratch) - used,
                         "%s%s INTEGER", i ? "," : "", names[i]);
        TEST_ASSERT_TRUE(n > 0 && (size_t)n < sizeof(scratch) - used);
        used += (size_t)n;
    }
    strcpy(scratch + used, ");");
    sql(scratch);
    TEST_ASSERT_EQUAL_INT(0, qb_init(qb, "b"));
    for (int i = 0; i < count; i++)
        TEST_ASSERT_EQUAL_INT(0, qb_add_column(qb, names[i], true));
}
static void test_query_long_column_list_that_fits(void) {
    query_builder_t qb;
    columns_table(&qb, 62);
    assert_executable(qb_build_select(&qb, NULL, NULL), 62);
}
static void test_query_column_list_overflow_rejected(void) {
    query_builder_t qb;
    columns_table(&qb, MAX_TRACKED_COLUMNS);
    TEST_ASSERT_NULL(qb_build_select(&qb, NULL, NULL));
}

static detection_zone_t zone(int count) {
    detection_zone_t value = {0};
    strcpy(value.id, "zone");
    strcpy(value.name, "Original");
    strcpy(value.color, "#123456");
    value.enabled = true;
    value.polygon_count = count;
    for (int i = 0; i < MAX_ZONE_POINTS; i++) {
        value.polygon[i].x = (float)(i % 2);
        value.polygon[i].y = (float)((i / 2) % 2);
    }
    return value;
}
static void assert_original_zone(void) {
    detection_zone_t result = {0};
    TEST_ASSERT_EQUAL_INT(1, get_detection_zones("camera", &result, 1));
    TEST_ASSERT_EQUAL_STRING("Original", result.name);
    TEST_ASSERT_EQUAL_INT(3, result.polygon_count);
}
static void test_zone_maximum_normalized_points_round_trip(void) {
    detection_zone_t input = zone(MAX_ZONE_POINTS), result = {0};
    TEST_ASSERT_EQUAL_INT(0, save_detection_zones("camera", &input, 1));
    TEST_ASSERT_EQUAL_INT(1, get_detection_zones("camera", &result, 1));
    TEST_ASSERT_EQUAL_INT(MAX_ZONE_POINTS, result.polygon_count);
    for (int i = 0; i < MAX_ZONE_POINTS; i++) {
        TEST_ASSERT_EQUAL_FLOAT(input.polygon[i].x, result.polygon[i].x);
        TEST_ASSERT_EQUAL_FLOAT(input.polygon[i].y, result.polygon[i].y);
    }
}
static void test_zone_serialized_overflow_rolls_back(void) {
    detection_zone_t input = zone(3);
    TEST_ASSERT_EQUAL_INT(0, save_detection_zones("camera", &input, 1));
    strcpy(input.name, "Replacement");
    input.polygon_count = MAX_ZONE_POINTS;
    for (int i = 0; i < MAX_ZONE_POINTS; i++)
        input.polygon[i].x = input.polygon[i].y = FLT_MAX;
    TEST_ASSERT_EQUAL_INT(-1, save_detection_zones("camera", &input, 1));
    assert_original_zone();
}
static void test_zone_zero_negative_counts_roll_back(void) {
    detection_zone_t input = zone(3);
    TEST_ASSERT_EQUAL_INT(0, save_detection_zones("camera", &input, 1));
    input.polygon_count = 0;
    TEST_ASSERT_EQUAL_INT(-1, save_detection_zones("camera", &input, 1));
    assert_original_zone();
    input.polygon_count = -1;
    TEST_ASSERT_EQUAL_INT(-1, save_detection_zones("camera", &input, 1));
    assert_original_zone();
}
static void test_zone_excessive_point_count_rejected(void) {
    detection_zone_t input = zone(MAX_ZONE_POINTS + 1);
    TEST_ASSERT_EQUAL_INT(-1, save_detection_zones("camera", &input, 1));
}

static void assert_query_identity(const char *stream, const char **sources, int count) {
    TEST_ASSERT_NOT_NULL(captured_url);
    const char *query = strchr(captured_url, '?');
    TEST_ASSERT_NOT_NULL(query);
    int source_index = 0, stream_count = 0;
    for (const char *p = query + 1; *p;) {
        const char *end = strchr(p, '&');
        if (!end) end = p + strlen(p);
        const char *equal = memchr(p, '=', (size_t)(end - p));
        TEST_ASSERT_NOT_NULL(equal);
        int length = 0;
        char *decoded = curl_easy_unescape(NULL, equal + 1, (int)(end - equal - 1), &length);
        TEST_ASSERT_NOT_NULL(decoded);
        bool same = false;
        if (equal - p == 3 && strncmp(p, "src", 3) == 0) {
            same = source_index < count && strlen(sources[source_index]) == (size_t)length
                   && memcmp(sources[source_index], decoded, (size_t)length) == 0;
            if (!same && source_index < count)
                fprintf(stderr, "source %d: expected %zu bytes, captured %d\n",
                        source_index, strlen(sources[source_index]), length);
            source_index++;
        } else if (equal - p == 4 && strncmp(p, "name", 4) == 0) {
            same = strlen(stream) == (size_t)length && memcmp(stream, decoded, (size_t)length) == 0;
            if (!same) fprintf(stderr, "stream name: expected %zu bytes, captured %d\n",
                               strlen(stream), length);
            stream_count++;
        }
        curl_free(decoded);
        TEST_ASSERT_TRUE_MESSAGE(same, "upstream received a changed source/name or unexpected parameter");
        p = *end ? end + 1 : end;
    }
    TEST_ASSERT_EQUAL_INT_MESSAGE(count, source_index, "upstream lost a source");
    TEST_ASSERT_EQUAL_INT(1, stream_count);
}
static void submit(const char *stream, const char **sources, int count, bool must_accept) {
    bool ok = go2rtc_api_add_stream_multi(stream, sources, count);
    if (must_accept) TEST_ASSERT_TRUE(ok);
    if (!ok) {
        TEST_ASSERT_EQUAL_UINT_MESSAGE(0, requests, "truncated request must not be sent");
        return;
    }
    TEST_ASSERT_EQUAL_UINT(1, requests);
    assert_query_identity(stream, sources, count);
}
static void test_multi_source_valid_escaping_preserves_identity(void) {
    const char *sources[] = {"rtsp://camera/a?token=x&name=y", "ffmpeg:camera#audio=opus"};
    submit("camera & east", sources, 2, true);
}
static void test_multi_source_maximum_valid_stream_name(void) {
    char name[MAX_STREAM_NAME];
    memset(name, ' ', sizeof(name) - 1); name[sizeof(name) - 1] = '\0';
    const char *sources[] = {"rtsp://camera/a", "rtsp://camera/b"};
    submit(name, sources, 2, true);
}
static void test_multi_source_aggregate_overflow_never_sends_partial_sources(void) {
    memset(scratch, '/', 1023); scratch[1023] = '\0';
    const char *sources[] = {"rtsp://camera/first", scratch};
    submit("camera", sources, 2, false);
}
static void test_multi_source_encoder_overflow_preserves_identity_or_rejects(void) {
    memset(scratch, 'a', 3500); scratch[3500] = '\0';
    const char *sources[] = {scratch, "rtsp://camera/b", "rtsp://camera/c"};
    submit("camera", sources, 3, false);
}
static void test_multi_source_name_overflow_preserves_identity_or_rejects(void) {
    char name[MAX_STREAM_NAME + 1];
    memset(name, ' ', sizeof(name) - 1); name[sizeof(name) - 1] = '\0';
    const char *sources[] = {"rtsp://camera/a", "rtsp://camera/b"};
    submit(name, sources, 2, false);
}
static void test_multi_source_exact_aggregate_buffer_fit(void) {
    const char *sources[] = {scratch};
    /* All source bytes are unreserved, so encoded length equals raw length. */
    size_t fixed = strlen("http://127.0.0.1:1984") + strlen(go2rtc_api_get_base_path())
                 + strlen("/api/streams?src=&name=camera");
    size_t source_len = 2048 - 1 - fixed;
    memset(scratch, 'a', source_len); scratch[source_len] = '\0';
    submit("camera", sources, 1, true);
    TEST_ASSERT_EQUAL_UINT(2047, strlen(captured_url));
}
static void test_multi_source_one_byte_aggregate_overflow(void) {
    const char *sources[] = {scratch};
    size_t fixed = strlen("http://127.0.0.1:1984") + strlen(go2rtc_api_get_base_path())
                 + strlen("/api/streams?src=&name=camera");
    size_t source_len = 2048 - fixed;
    memset(scratch, 'a', source_len); scratch[source_len] = '\0';
    submit("camera", sources, 1, false);
}
int main(void) {
    if (curl_global_init(CURL_GLOBAL_DEFAULT) != CURLE_OK) return 2;
    UNITY_BEGIN();
    RUN_TEST(test_query_valid_where_order);
    RUN_TEST(test_query_exact_buffer_fit);
    RUN_TEST(test_query_one_byte_overflow_rejected);
    RUN_TEST(test_query_oversized_where_and_order_rejected);
    RUN_TEST(test_query_long_column_list_that_fits);
    RUN_TEST(test_query_column_list_overflow_rejected);
    RUN_TEST(test_zone_maximum_normalized_points_round_trip);
    RUN_TEST(test_zone_serialized_overflow_rolls_back);
    RUN_TEST(test_zone_zero_negative_counts_roll_back);
    RUN_TEST(test_zone_excessive_point_count_rejected);
    RUN_TEST(test_multi_source_valid_escaping_preserves_identity);
    RUN_TEST(test_multi_source_maximum_valid_stream_name);
    RUN_TEST(test_multi_source_aggregate_overflow_never_sends_partial_sources);
    RUN_TEST(test_multi_source_encoder_overflow_preserves_identity_or_rejects);
    RUN_TEST(test_multi_source_name_overflow_preserves_identity_or_rejects);
    RUN_TEST(test_multi_source_exact_aggregate_buffer_fit);
    RUN_TEST(test_multi_source_one_byte_aggregate_overflow);
    int result = UNITY_END();
    curl_global_cleanup();
    return result;
}
