#define _POSIX_C_SOURCE 200809L

#include "unity.h"
#include <unistd.h>
#include "database/db_core.h"
#include "database/db_streams.h"
/* Exercise the private cache invariants as well as the public query API. */
#include "../../src/web/api_handlers_go2rtc_proxy.c"

void setUp(void) {
    load_default_config(&g_config);
    memset(g_hls_sessions, 0, sizeof(g_hls_sessions));
}
void tearDown(void) {}

static void request_for(http_request_t *req, const char *path, const char *query) {
    http_request_init(req);
    strcpy(req->path, path);
    strcpy(req->query_string, query);
    strcpy(req->method_str, "GET");
}

void test_encoded_sources_are_decoded_before_length_checks_and_canonicalized(void) {
    const char *paths[] = {"/go2rtc/api/ws", "/go2rtc/api/webrtc",
                          "/go2rtc/api/frame.jpeg", "/go2rtc/api/stream.m3u8"};
    http_request_t req;
    char query[1024] = "src=", expected[MAX_STREAM_NAME], name[MAX_STREAM_NAME], url[2048];
    memset(expected, 'a', sizeof(expected) - 1);
    expected[sizeof(expected) - 1] = '\0';
    for (int i = 0; i < MAX_STREAM_NAME - 1; i++) strcat(query, "%61");
    for (size_t i = 0; i < sizeof(paths) / sizeof(paths[0]); i++) {
        request_for(&req, paths[i], query);
        TEST_ASSERT_TRUE(go2rtc_proxy_parse_source(&req, name, sizeof(name)));
        TEST_ASSERT_EQUAL_STRING(expected, name);
        TEST_ASSERT_TRUE(go2rtc_proxy_build_url(&req, url, sizeof(url)));
        TEST_ASSERT_EQUAL_STRING(expected, strstr(url, "?src=") + 5);
        strcat(req.query_string, "b");
        TEST_ASSERT_FALSE(go2rtc_proxy_parse_source(&req, name, sizeof(name)));
        TEST_ASSERT_FALSE(go2rtc_proxy_build_url(&req, url, sizeof(url)));
    }
    request_for(&req, paths[0], "src=Front+Gate%26src%3Devil");
    TEST_ASSERT_TRUE(go2rtc_proxy_parse_source(&req, name, sizeof(name)));
    TEST_ASSERT_EQUAL_STRING("Front Gate&src=evil", name);
    TEST_ASSERT_TRUE(go2rtc_proxy_build_url(&req, url, sizeof(url)));
    TEST_ASSERT_NOT_NULL(strstr(url, "?src=Front%20Gate%26src%3Devil"));
    TEST_ASSERT_FALSE(go2rtc_proxy_build_url(&req, url, 16));
}

void test_playback_queries_reject_duplicates_aliases_and_malformed_encoding(void) {
    const char *queries[] = {"src=cam&src=other", "src=cam&%73rc=other", "s%72c=cam",
        "src=cam&dst=other", "src=cam&name=other", "src=cam;src=other", "src=cam#other",
        "src=cam%00other", "src=cam%0a", "src=cam%", "src=cam%2", "src=cam%GG",
        "src=cam&", "src=cam&&video", "src=", ""};
    const char *paths[] = {"/go2rtc/api/ws", "/go2rtc/api/webrtc",
                          "/go2rtc/api/frame.jpeg", "/go2rtc/api/stream.m3u8"};
    http_request_t req;
    char url[2048];
    for (size_t p = 0; p < sizeof(paths) / sizeof(paths[0]); p++)
        for (size_t q = 0; q < sizeof(queries) / sizeof(queries[0]); q++) {
            request_for(&req, paths[p], queries[q]);
            TEST_ASSERT_FALSE_MESSAGE(go2rtc_proxy_build_url(&req, url, sizeof(url)), queries[q]);
        }
}

void test_browser_hls_and_snapshot_options_survive_canonical_forwarding(void) {
    http_request_t req;
    char url[2048], name[MAX_STREAM_NAME];
    request_for(&req, "/go2rtc/api/stream.m3u8", "mp4=flac&src=Front%20Gate_sub&video");
    TEST_ASSERT_TRUE(go2rtc_proxy_build_url(&req, url, sizeof(url)));
    TEST_ASSERT_NOT_NULL(strstr(url, "?mp4=flac&src=Front%20Gate_sub&video="));
    TEST_ASSERT_TRUE(go2rtc_proxy_parse_source(&req, name, sizeof(name)));
    TEST_ASSERT_EQUAL_STRING("Front Gate_sub", name);
    request_for(&req, "/go2rtc/api/frame.jpeg", "src=cam&cache=5s&t=1234567");
    TEST_ASSERT_TRUE(go2rtc_proxy_build_url(&req, url, sizeof(url)));
    TEST_ASSERT_NOT_NULL(strstr(url, "?src=cam&cache=5s&t=1234567"));
    request_for(&req, "/go2rtc/api/hls/segment.m4s", "id=abc123&n=7");
    TEST_ASSERT_TRUE(go2rtc_proxy_build_url(&req, url, sizeof(url)));
    TEST_ASSERT_NOT_NULL(strstr(url, "?id=abc123&n=7"));
    request_for(&req, "/go2rtc/api/streams", "src=Front+Gate");
    TEST_ASSERT_TRUE(go2rtc_proxy_build_url(&req, url, sizeof(url)));
    TEST_ASSERT_NOT_NULL(strstr(url, "?src=Front%20Gate"));
}

void test_hls_session_ids_cannot_be_truncated_or_duplicated(void) {
    http_request_t req;
    char url[2048];
    const char *queries[] = {"id=allowed&id=restricted", "id=allowed&%69d=restricted",
        "id=allowed%00restricted", "id=allowed;id=restricted", "id=allowed&src=restricted"};
    for (size_t i = 0; i < sizeof(queries) / sizeof(queries[0]); i++) {
        request_for(&req, "/go2rtc/api/hls/segment.ts", queries[i]);
        TEST_ASSERT_FALSE(go2rtc_proxy_build_url(&req, url, sizeof(url)));
    }
    request_for(&req, "/go2rtc/api/hls/segment.ts", "id=");
    memset(req.query_string + 3, 'a', HLS_SESSION_ID_MAX);
    req.query_string[3 + HLS_SESSION_ID_MAX] = '\0';
    TEST_ASSERT_FALSE(go2rtc_proxy_build_url(&req, url, sizeof(url)));
}

void test_hls_cache_updates_existing_entry_even_with_an_earlier_hole(void) {
    cache_hls_session("first", "credential", "old-camera");
    cache_hls_session("shared", "credential", "old-camera");
    g_hls_sessions[0].occupied = false;
    cache_hls_session("shared", "credential", "new-camera");
    int matches = 0;
    for (int i = 0; i < HLS_SESSION_CACHE_CAPACITY; i++) {
        if (g_hls_sessions[i].occupied && strcmp(g_hls_sessions[i].session_id, "shared") == 0) {
            matches++;
            TEST_ASSERT_EQUAL_STRING("new-camera", g_hls_sessions[i].stream_name);
        }
    }
    TEST_ASSERT_EQUAL_INT(1, matches);
    TEST_ASSERT_FALSE(g_hls_sessions[0].occupied);
}

void test_hls_cache_evicts_the_oldest_entry_when_full(void) {
    for (int i = 0; i < HLS_SESSION_CACHE_CAPACITY; i++) {
        char id[32];
        snprintf(id, sizeof(id), "session-%d", i);
        cache_hls_session(id, "credential", "camera");
        g_hls_sessions[i].last_used = 100 + i;
    }
    g_hls_sessions[5].last_used = 1;
    cache_hls_session("new", "credential", "camera");
    TEST_ASSERT_EQUAL_STRING("new", g_hls_sessions[5].session_id);
}

void test_expired_hls_substream_session_rechecks_the_parent_camera(void) {
    char path[] = "/tmp/lightnvr-hls-renew-XXXXXX";
    int fd = mkstemp(path);
    TEST_ASSERT_GREATER_OR_EQUAL_INT(0, fd);
    close(fd);
    TEST_ASSERT_EQUAL_INT(0, init_database(path));
    g_config.web_auth_enabled = false;
    stream_config_t camera = {0};
    strcpy(camera.name, "renew-camera");
    strcpy(camera.url, "rtsp://test/live");
    camera.enabled = true;
    TEST_ASSERT_NOT_EQUAL(0, add_stream_config(&camera));
    http_request_t req;
    request_for(&req, "/go2rtc/api/hls/segment.m4s", "id=renew-session&n=1");
    strcpy(req.client_ip, "192.0.2.99");
    char fingerprint[65];
    TEST_ASSERT_EQUAL_INT(0, httpd_request_auth_fingerprint(&req, fingerprint));
    cache_hls_session("renew-session", fingerprint, "renew-camera_sub");
    g_hls_sessions[0].authorized_until = 0;
    http_response_t res;
    http_response_init(&res);
    bool allowed = go2rtc_proxy_authorize_request(&req, &res);
    http_response_free(&res);
    shutdown_database();
    unlink(path);
    TEST_ASSERT_TRUE(allowed);
    TEST_ASSERT_GREATER_THAN_INT64(0, g_hls_sessions[0].authorized_until);
}

int main(void) {
    UNITY_BEGIN();
    RUN_TEST(test_encoded_sources_are_decoded_before_length_checks_and_canonicalized);
    RUN_TEST(test_playback_queries_reject_duplicates_aliases_and_malformed_encoding);
    RUN_TEST(test_browser_hls_and_snapshot_options_survive_canonical_forwarding);
    RUN_TEST(test_hls_session_ids_cannot_be_truncated_or_duplicated);
    RUN_TEST(test_hls_cache_updates_existing_entry_even_with_an_earlier_hole);
    RUN_TEST(test_hls_cache_evicts_the_oldest_entry_when_full);
    RUN_TEST(test_expired_hls_substream_session_rechecks_the_parent_camera);
    return UNITY_END();
}
