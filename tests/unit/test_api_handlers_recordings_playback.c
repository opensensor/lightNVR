#define _GNU_SOURCE
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/stat.h>
#include <unistd.h>

#include "unity.h"
#include "database/db_recordings.h"
#include "video/recording_path.h"
#include "video/recording_transcode.h"
#include "web/api_handlers_recordings_playback.h"
#include "web/httpd_utils.h"

static char directory[] = "/tmp/lightnvr_playback_apiXXXXXX";
static recording_metadata_t recording;
static http_request_t request;
static http_response_t response;
static bool hevc, allowed;
static int probe_calls, prepare_calls, serve_calls;
static recording_transcode_status_t transcode_status;
static char served_path[MAX_PATH_LENGTH];

int __wrap_get_recording_metadata_by_id(uint64_t id, recording_metadata_t *out) {
    *out = recording;
    return id == 42 ? 0 : -1;
}

int __wrap_httpd_authorize_camera_identity_action_with_context(
    const http_request_t *req, http_response_t *res,
    authorization_action_t action, const char *camera_uuid,
    const char *legacy_stream_name, user_t *user, fleet_camera_t *camera,
    authorization_evaluation_t *evaluation) {
    (void)req; (void)camera_uuid; (void)legacy_stream_name;
    (void)user; (void)camera; (void)evaluation;
    TEST_ASSERT_EQUAL_INT(AUTHZ_RECORDINGS_REPLAY, action);
    if (!allowed) http_response_set_json_error(res, 403, "Forbidden");
    return allowed;
}

bool __wrap_recording_needs_hevc_transcode(const char *path) {
    TEST_ASSERT_EQUAL_STRING(recording.file_path, path);
    probe_calls++;
    return hevc;
}

recording_transcode_status_t __wrap_request_recording_transcode_cache(
    const char *source, const char *cache) {
    TEST_ASSERT_EQUAL_STRING(recording.file_path, source);
    TEST_ASSERT_NOT_NULL(strstr(cache, "/transcoded/42.mp4"));
    prepare_calls++;
    return transcode_status;
}

int __wrap_http_serve_file(const http_request_t *req, const http_response_t *res,
                         const char *path, const char *type, const char *headers) {
    (void)res;
    TEST_ASSERT_EQUAL_STRING("video/mp4", type);
    TEST_ASSERT_NOT_NULL(strstr(headers, "Accept-Ranges: bytes"));
    TEST_ASSERT_EQUAL_STRING("bytes=0-1023", http_request_get_header(req, "Range"));
    snprintf(served_path, sizeof(served_path), "%s", path);
    serve_calls++;
    return 0;
}

void setUp(void) {
    http_request_init(&request);
    http_response_init(&response);
    strcpy(request.path, "/api/recordings/play/42");
    strcpy(request.query_string, "prepare=1");
    strcpy(request.headers[0].name, "Range");
    strcpy(request.headers[0].value, "bytes=0-1023");
    request.num_headers = 1;
    hevc = allowed = true;
    probe_calls = prepare_calls = serve_calls = 0;
    transcode_status = RECORDING_TRANSCODE_PENDING;
}

void tearDown(void) { http_response_free(&response); }

static const char *response_header(const char *name) {
    for (int i = 0; i < response.num_headers; i++) {
        if (strcasecmp(response.headers[i].name, name) == 0) return response.headers[i].value;
    }
    return NULL;
}

void test_hevc_preparation_returns_retryable_json_without_serving_media(void) {
    handle_recordings_playback(&request, &response);
    TEST_ASSERT_EQUAL_INT(202, response.status_code);
    TEST_ASSERT_EQUAL_STRING("{\"status\":\"preparing\"}", response.body);
    TEST_ASSERT_EQUAL_STRING("2", response_header("Retry-After"));
    TEST_ASSERT_NOT_NULL(strstr(response_header("Cache-Control"), "no-store"));
    TEST_ASSERT_EQUAL_INT(1, prepare_calls);
    TEST_ASSERT_EQUAL_INT(0, serve_calls);
}

void test_uncached_media_request_returns_prompt_retry_instead_of_waiting(void) {
    strcpy(request.query_string, "transcode=1");
    handle_recordings_playback(&request, &response);
    TEST_ASSERT_EQUAL_INT(503, response.status_code);
    TEST_ASSERT_EQUAL_STRING("2", response_header("Retry-After"));
    TEST_ASSERT_EQUAL_INT(0, serve_calls);
}

void test_hevc_media_plays_original_without_probe_or_preparation(void) {
    request.query_string[0] = '\0';
    handle_recordings_playback(&request, &response);
    TEST_ASSERT_EQUAL_INT(200, response.status_code);
    TEST_ASSERT_EQUAL_INT(1, serve_calls);
    TEST_ASSERT_EQUAL_STRING(recording.file_path, served_path);
    TEST_ASSERT_EQUAL_INT(0, probe_calls);
    TEST_ASSERT_EQUAL_INT(0, prepare_calls);
}

void test_h264_preparation_is_ready_without_a_transcode(void) {
    hevc = false;
    handle_recordings_playback(&request, &response);
    TEST_ASSERT_EQUAL_INT(200, response.status_code);
    TEST_ASSERT_EQUAL_STRING("{\"status\":\"ready\"}", response.body);
    TEST_ASSERT_EQUAL_INT(0, prepare_calls);
    TEST_ASSERT_EQUAL_INT(0, serve_calls);
}

void test_h264_media_uses_original_with_range_support(void) {
    hevc = false;
    request.query_string[0] = '\0';
    handle_recordings_playback(&request, &response);
    TEST_ASSERT_EQUAL_INT(1, serve_calls);
    TEST_ASSERT_EQUAL_STRING(recording.file_path, served_path);
}

void test_failed_transcode_reports_error_instead_of_unplayable_original(void) {
    transcode_status = RECORDING_TRANSCODE_FAILED;
    handle_recordings_playback(&request, &response);
    TEST_ASSERT_EQUAL_INT(500, response.status_code);
    TEST_ASSERT_NOT_NULL(strstr(response.body, "Unable to prepare recording"));
    TEST_ASSERT_EQUAL_INT(0, serve_calls);
}

void test_preparation_requires_recording_replay_authorization(void) {
    allowed = false;
    handle_recordings_playback(&request, &response);
    TEST_ASSERT_EQUAL_INT(403, response.status_code);
    TEST_ASSERT_EQUAL_INT(0, probe_calls);
    TEST_ASSERT_EQUAL_INT(0, prepare_calls);
    TEST_ASSERT_EQUAL_INT(0, serve_calls);
}

void test_cached_hevc_media_skips_probe_and_conversion(void) {
    char cache[MAX_PATH_LENGTH], cache_dir[MAX_PATH_LENGTH];
    snprintf(cache_dir, sizeof(cache_dir), "%s/transcoded", directory);
    TEST_ASSERT_EQUAL_INT(0, mkdir(cache_dir, 0700));
    TEST_ASSERT_EQUAL_INT(0, build_recording_transcode_cache_path(directory, 42, cache, sizeof(cache)));
    FILE *file = fopen(cache, "w");
    TEST_ASSERT_NOT_NULL(file);
    fputs("cached video", file);
    fclose(file);
    strcpy(request.query_string, "transcode=1");
    handle_recordings_playback(&request, &response);
    unlink(cache);
    rmdir(cache_dir);
    TEST_ASSERT_EQUAL_INT(1, serve_calls);
    TEST_ASSERT_EQUAL_STRING(cache, served_path);
    TEST_ASSERT_EQUAL_INT(0, probe_calls);
    TEST_ASSERT_EQUAL_INT(0, prepare_calls);
}

int main(void) {
    if (!mkdtemp(directory)) return 1;
    strcpy(g_config.storage_path, directory);
    snprintf(recording.file_path, sizeof(recording.file_path), "%s/original.mp4", directory);
    FILE *file = fopen(recording.file_path, "w");
    if (!file) return 1;
    fputs("source video", file);
    fclose(file);
    UNITY_BEGIN();
    RUN_TEST(test_hevc_preparation_returns_retryable_json_without_serving_media);
    RUN_TEST(test_uncached_media_request_returns_prompt_retry_instead_of_waiting);
    RUN_TEST(test_hevc_media_plays_original_without_probe_or_preparation);
    RUN_TEST(test_h264_preparation_is_ready_without_a_transcode);
    RUN_TEST(test_h264_media_uses_original_with_range_support);
    RUN_TEST(test_failed_transcode_reports_error_instead_of_unplayable_original);
    RUN_TEST(test_preparation_requires_recording_replay_authorization);
    RUN_TEST(test_cached_hevc_media_skips_probe_and_conversion);
    int result = UNITY_END();
    unlink(recording.file_path);
    rmdir(directory);
    return result;
}
