#define _POSIX_C_SOURCE 200809L
#include "utils/uuid.h"
static int fixture_uuid_generate(char output[LIGHTNVR_UUID_STRING_SIZE]);
#define lightnvr_uuid_generate_v4 fixture_uuid_generate
#include "../../src/web/api_handlers_go2rtc_proxy.c"
#undef lightnvr_uuid_generate_v4
static int uuid_fault;
static int fixture_uuid_generate(char output[LIGHTNVR_UUID_STRING_SIZE]) {
    if (uuid_fault == 1 || uuid_fault == 2) {
        memset(output, 'z', LIGHTNVR_UUID_STRING_SIZE);
        if (uuid_fault == 1) output[36] = 0;
        return 0;
    }
    int result = lightnvr_uuid_generate_v4(output);
    if (result == 0 && uuid_fault == 3) output[0] = '0';
    return result;
}
#include "database/db_api_tokens.h"
#include "database/db_authorization.h"
#include "database/db_core.h"
#include "database/db_streams.h"
#include "unity.h"
#include <unistd.h>
#undef LOG_COMPONENT
#include "../../src/web/api_handlers_streaming.c"
static char path[128];
static int64_t user_id;
static stream_config_t camera;
static authorization_role_t role;
static http_request_t original, callback;
static http_response_t response;
static char lease[37];
static const char *media_fixture;
void setUp(void) {
    uuid_fault = 0;
    load_default_config(&g_config);
    g_config.web_auth_enabled = true;
    strcpy(path, "/tmp/lightnvr-media-authority-XXXXXX");
    int fd = mkstemp(path);
    TEST_ASSERT_GREATER_OR_EQUAL_INT(0, fd);
    close(fd);
    TEST_ASSERT_EQUAL_INT(0, init_database(path));
    TEST_ASSERT_EQUAL_INT(
        0, db_auth_create_user("media", "Password123!", NULL, USER_ROLE_VIEWER, true, &user_id));
    memset(&camera, 0, sizeof(camera));
    strcpy(camera.name, "camera");
    strcpy(camera.url, "rtsp://fixture.invalid/live");
    camera.enabled = true;
    camera.streaming_enabled = true;
    TEST_ASSERT_NOT_EQUAL(0, add_stream_config(&camera));
    TEST_ASSERT_EQUAL_INT(0, get_stream_config_by_name("camera", &camera));
    memset(&role, 0, sizeof(role));
    strcpy(role.name, "media-policy");
    role.action_mask = authorization_action_bit(AUTHZ_LIVE_VIEW);
    int64_t version;
    TEST_ASSERT_EQUAL_INT(0, db_authorization_get_policy_version(&version));
    TEST_ASSERT_EQUAL_INT(0, db_authorization_role_create(&role, version, &version));
    TEST_ASSERT_EQUAL_INT(0, db_authorization_set_user_mode(user_id, "policy"));
    char selector[128];
    snprintf(selector, sizeof(selector),
             "{\"version\":1,\"expression\":{\"op\":\"camera_uuid\",\"values\":[\"%s\"]}}",
             camera.camera_uuid);
    TEST_ASSERT_EQUAL_INT(0, db_authorization_create_user_grant(user_id, role.uuid, "selector",
                                                                selector, NULL, NULL));
    http_request_init(&original);
    strcpy(original.path, "/go2rtc/api/webrtc");
    strcpy(original.query_string, "src=camera_sub");
    strcpy(original.method_str, "POST");
    strcpy(original.client_ip, "127.0.0.1");
    strcpy(original.user_agent, "owned-fixture");
    original.num_headers = 1;
    strcpy(original.headers[0].name, "Authorization");
    strcpy(original.headers[0].value, "Basic bWVkaWE6UGFzc3dvcmQxMjMh");
    http_request_init(&callback);
    strcpy(callback.client_ip, "127.0.0.1");
    strcpy(callback.query_string, "src=camera_sub");
    callback.num_headers = 1;
    strcpy(callback.headers[0].name, "X-LightNVR-Media-Lease");
    http_response_init(&response);
}
void tearDown(void) {
    http_response_free(&response);
    for (int i = 0; i < MEDIA_LEASE_CAPACITY; i++) {
        if (media_leases[i].request) {
            memset(media_leases[i].request, 0, sizeof(http_request_t));
            free(media_leases[i].request);
        }
        memset(&media_leases[i], 0, sizeof(media_leases[i]));
    }
    shutdown_database();
    unlink(path);
}
static void issue(void) {
    bool authorized = go2rtc_proxy_authorize_request(&original, &response);
    if (!authorized)
        fprintf(stderr, "fixture authorization status=%d body=%s\n", response.status_code,
                response.body ? (char *)response.body : "");
    TEST_ASSERT_TRUE(authorized);
    TEST_ASSERT_TRUE(go2rtc_proxy_create_media_lease(&original, &response, lease));
    strcpy(callback.headers[0].value, lease);
}
static void check(int code, const char *json) {
    http_response_free(&response);
    http_response_init(&response);
    handle_internal_media_lease(&callback, &response);
    TEST_ASSERT_EQUAL_INT(code, response.status_code);
    if (json)
        TEST_ASSERT_EQUAL_STRING(json, response.body);
}
static void rights(uint64_t mask) {
    role.action_mask = mask;
    int64_t version;
    TEST_ASSERT_EQUAL_INT(0, db_authorization_get_policy_version(&version));
    TEST_ASSERT_EQUAL_INT(0, db_authorization_role_update(&role, version, &version));
}
void test_malformed_generated_lease_is_bounded_and_not_stored(void) {
    for (uuid_fault = 1; uuid_fault <= 2; ++uuid_fault) {
        TEST_ASSERT_TRUE(go2rtc_proxy_authorize_request(&original, &response));
        TEST_ASSERT_FALSE(go2rtc_proxy_create_media_lease(&original, &response, lease));
        TEST_ASSERT_EQUAL_INT(503, response.status_code);
        TEST_ASSERT_EQUAL_CHAR(0, lease[0]);
        for (int i = 0; i < MEDIA_LEASE_CAPACITY; ++i) {
            TEST_ASSERT_NULL(media_leases[i].request);
        }
    }
}
void test_issuance_rejects_missing_camera_and_invalid_principal(void) {
    strcpy(original.query_string, "src=missing_camera");
    TEST_ASSERT_FALSE(go2rtc_proxy_create_media_lease(&original, &response, lease));
    TEST_ASSERT_EQUAL_INT(404, response.status_code);
    TEST_ASSERT_EQUAL_CHAR(0, lease[0]);
    strcpy(original.query_string, "src=camera_sub");
    strcpy(original.headers[0].value, "Basic invalid");
    TEST_ASSERT_FALSE(go2rtc_proxy_create_media_lease(&original, &response, lease));
    TEST_ASSERT_EQUAL_INT(401, response.status_code);
    TEST_ASSERT_EQUAL_CHAR(0, lease[0]);
    TEST_ASSERT_NULL(media_leases[0].request);
}
void test_lease_capacity_fails_closed_and_reclaims_only_expired_slot(void) {
    /* Authentication-disabled deployments still have bounded media leases.
     * Avoid 256 password hashes while exercising real registry allocations. */
    g_config.web_auth_enabled = false;
    char first[LIGHTNVR_UUID_STRING_SIZE];
    for (int i = 0; i < MEDIA_LEASE_CAPACITY; ++i) {
        TEST_ASSERT_TRUE(go2rtc_proxy_create_media_lease(&original, &response, lease));
        if (i == 0) strcpy(first, lease);
    }
    TEST_ASSERT_FALSE(go2rtc_proxy_create_media_lease(&original, &response, lease));
    TEST_ASSERT_EQUAL_INT(503, response.status_code);
    TEST_ASSERT_EQUAL_CHAR(0, lease[0]);
    strcpy(callback.headers[0].value, first);
    check(200, NULL);
    media_leases[0].touched -= MEDIA_LEASE_IDLE_SECONDS + 1;
    TEST_ASSERT_TRUE(go2rtc_proxy_create_media_lease(&original, &response, lease));
    TEST_ASSERT_NOT_EQUAL(0, strcmp(first, lease));
    TEST_ASSERT_EQUAL_STRING(lease, media_leases[0].token);
    check(403, NULL);
    strcpy(callback.headers[0].value, lease);
    check(200, NULL);
    int occupied = 0;
    for (int i = 0; i < MEDIA_LEASE_CAPACITY; ++i) occupied += media_leases[i].request != NULL;
    TEST_ASSERT_EQUAL_INT(MEDIA_LEASE_CAPACITY, occupied);
}
void test_basic_audio_directions_and_live_revocation(void) {
    issue();
    check(200, "{\"view\":true,\"listen\":false,\"talk\":false}");
    rights(authorization_action_bit(AUTHZ_LIVE_VIEW) |
           authorization_action_bit(AUTHZ_AUDIO_LISTEN));
    check(200, "{\"view\":true,\"listen\":true,\"talk\":false}");
    rights(authorization_action_bit(AUTHZ_LIVE_VIEW) | authorization_action_bit(AUTHZ_AUDIO_TALK));
    check(200, "{\"view\":true,\"listen\":false,\"talk\":true}");
    rights(authorization_action_bit(AUTHZ_AUDIO_TALK));
    check(403, NULL);
}
void test_instance_audio_disable_overrides_granted_media_rights(void) {
  rights(authorization_action_bit(AUTHZ_LIVE_VIEW) |
         authorization_action_bit(AUTHZ_AUDIO_LISTEN) |
         authorization_action_bit(AUTHZ_AUDIO_TALK));
  issue();
  check(200, "{\"view\":true,\"listen\":true,\"talk\":true}");
  g_config.audio_disabled = true;
  check(200, "{\"view\":true,\"listen\":false,\"talk\":false}");
  g_config.audio_disabled = false;
  check(200, "{\"view\":true,\"listen\":true,\"talk\":true}");
}
void test_instance_audio_disable_remuxes_native_hls_despite_cached_grant(void) {
  TEST_ASSERT_NOT_NULL(media_fixture);
  rights(authorization_action_bit(AUTHZ_LIVE_VIEW) |
         authorization_action_bit(AUTHZ_AUDIO_LISTEN));
  /* Prime the normal 30-second media cache before the instance policy changes.
   */
  TEST_ASSERT_TRUE(httpd_authorize_media_stream_action(
      &original, &response, AUTHZ_AUDIO_LISTEN, "camera"));
  char directory[] = "/tmp/lightnvr-hls-policy-XXXXXX";
  TEST_ASSERT_NOT_NULL(mkdtemp(directory));
  char hls[256], camera_dir[256], segment[256], output[256];
  snprintf(hls, sizeof(hls), "%s/hls", directory);
  snprintf(camera_dir, sizeof(camera_dir), "%s/hls/camera", directory);
  snprintf(segment, sizeof(segment), "%s/hls/camera/av.ts", directory);
  snprintf(output, sizeof(output), "%s/video.ts", directory);
  TEST_ASSERT_EQUAL_INT(0, mkdir(hls, 0700));
  TEST_ASSERT_EQUAL_INT(0, mkdir(camera_dir, 0700));
  TEST_ASSERT_EQUAL_INT(0, symlink(media_fixture, segment));
  strcpy(g_config.storage_path_hls, directory);
  g_config.audio_disabled = true;
  strcpy(original.path, "/hls/camera/av.ts");
  handle_direct_hls_request(&original, &response);
  /* Remove owned fixture paths before assertions, including on regression. */
  unlink(segment);
  rmdir(camera_dir);
  rmdir(hls);
  if (!response.body) {
    rmdir(directory);
    TEST_FAIL_MESSAGE("Global audio disable must remux the actual HLS segment");
  }
  FILE *file = fopen(output, "wb");
  TEST_ASSERT_NOT_NULL(file);
  TEST_ASSERT_EQUAL_UINT(response.body_length,
                         fwrite(response.body, 1, response.body_length, file));
  fclose(file);
  AVFormatContext *input = NULL;
  TEST_ASSERT_EQUAL_INT(0, avformat_open_input(&input, output, NULL, NULL));
  TEST_ASSERT_GREATER_OR_EQUAL_INT(0, avformat_find_stream_info(input, NULL));
  TEST_ASSERT_GREATER_OR_EQUAL_INT(
      0, av_find_best_stream(input, AVMEDIA_TYPE_VIDEO, -1, -1, NULL, 0));
  TEST_ASSERT_LESS_THAN_INT(
      0, av_find_best_stream(input, AVMEDIA_TYPE_AUDIO, -1, -1, NULL, 0));
  avformat_close_input(&input);
  unlink(output);
  rmdir(directory);
}
void test_lease_binds_source_loopback_immutable_identity_and_expiry(void) {
    /* Reproduce a valid token beginning with zero deterministically. */
    uuid_fault = 3;
    issue();
    uuid_fault = 0;
    strcpy(callback.client_ip, "198.51.100.1");
    check(403, NULL);
    strcpy(callback.client_ip, "127.0.0.1");
    strcpy(callback.query_string, "src=another");
    check(403, NULL);
    strcpy(callback.query_string, "src=camera_sub");
    callback.headers[0].value[0] = lease[0] == '0' ? '1' : '0';
    check(403, NULL);
    strcpy(callback.headers[0].value, lease);
    check(200, NULL);
    media_leases[0].touched -= MEDIA_LEASE_IDLE_SECONDS + 1;
    check(403, NULL);
}
void test_existing_lease_does_not_follow_substream_name_reassignment(void) {
    TEST_ASSERT_EQUAL_INT(
        0, db_authorization_create_user_grant(user_id, role.uuid, "all", NULL, NULL, NULL));
    issue();
    check(200, NULL);
    stream_config_t replacement = {0};
    strcpy(replacement.name, "camera_sub");
    strcpy(replacement.url, "rtsp://fixture.invalid/other");
    replacement.enabled = true;
    TEST_ASSERT_NOT_EQUAL(0, add_stream_config(&replacement));
    check(403, NULL); /* Even the all-fleet principal must negotiate a new lease. */
}
void test_disabled_account_revokes_existing_basic_lease(void) {
    issue();
    check(200, NULL);
    TEST_ASSERT_EQUAL_INT(0, db_auth_update_user(user_id, NULL, NULL, -1, 0));
    check(403, NULL);
}
void test_session_and_legacy_key_revocation(void) {
    char session[128];
    TEST_ASSERT_EQUAL_INT(0,
                          db_auth_create_session(user_id, original.client_ip, original.user_agent,
                                                 60, session, sizeof(session)));
    strcpy(original.headers[0].name, "Cookie");
    snprintf(original.headers[0].value, sizeof(original.headers[0].value), "session=%s", session);
    issue();
    check(200, NULL);
    TEST_ASSERT_EQUAL_INT(0, db_auth_delete_session(session));
    check(403, NULL);
    strcpy(original.headers[0].name, "X-API-Key");
    TEST_ASSERT_EQUAL_INT(0, db_auth_generate_api_key(user_id, original.headers[0].value,
                                                      sizeof(original.headers[0].value)));
    issue();
    check(200, NULL);
    char replacement[64];
    TEST_ASSERT_EQUAL_INT(0, db_auth_generate_api_key(user_id, replacement, sizeof(replacement)));
    check(403, NULL);
}
void test_scoped_token_intersects_owner_and_observes_revocation(void) {
    rights(authorization_action_bit(AUTHZ_LIVE_VIEW) |
           authorization_action_bit(AUTHZ_AUDIO_LISTEN) |
           authorization_action_bit(AUTHZ_AUDIO_TALK));
    api_token_create_t input = {.user_id = user_id,
                                .created_by_user_id = user_id,
                                .description = "fixture",
                                .action_mask = authorization_action_bit(AUTHZ_LIVE_VIEW),
                                .scope_type = "all",
                                .expires_at = time(NULL) + 60};
    api_token_t token;
    char secret[API_TOKEN_SECRET_MAX];
    TEST_ASSERT_EQUAL_INT(0, db_api_token_create(&input, &token, secret));
    snprintf(original.headers[0].value, sizeof(original.headers[0].value), "Bearer %s", secret);
    issue();
    check(200, "{\"view\":true,\"listen\":false,\"talk\":false}");
    TEST_ASSERT_EQUAL_INT(0, db_api_token_revoke(user_id, token.uuid));
    check(403, NULL);
}
void test_owned_native_hls_segment_preserves_video_and_removes_audio(void) {
    TEST_ASSERT_NOT_NULL_MESSAGE(media_fixture, "Run with the owned FFmpeg fixture wrapper");
    AVFormatContext *input = NULL;
    TEST_ASSERT_EQUAL_INT(0, avformat_open_input(&input, media_fixture, NULL, NULL));
    TEST_ASSERT_GREATER_OR_EQUAL_INT(0, avformat_find_stream_info(input, NULL));
    TEST_ASSERT_GREATER_OR_EQUAL_INT(
        0, av_find_best_stream(input, AVMEDIA_TYPE_AUDIO, -1, -1, NULL, 0));
    avformat_close_input(&input);
    TEST_ASSERT_TRUE(serve_video_segment(media_fixture, &response));
    char output[180];
    snprintf(output, sizeof(output), "%s.ts", path);
    FILE *file = fopen(output, "wb");
    TEST_ASSERT_NOT_NULL(file);
    TEST_ASSERT_EQUAL_UINT(response.body_length,
                           fwrite(response.body, 1, response.body_length, file));
    fclose(file);
    TEST_ASSERT_EQUAL_INT(0, avformat_open_input(&input, output, NULL, NULL));
    TEST_ASSERT_GREATER_OR_EQUAL_INT(0, avformat_find_stream_info(input, NULL));
    TEST_ASSERT_GREATER_OR_EQUAL_INT(
        0, av_find_best_stream(input, AVMEDIA_TYPE_VIDEO, -1, -1, NULL, 0));
    TEST_ASSERT_LESS_THAN_INT(0, av_find_best_stream(input, AVMEDIA_TYPE_AUDIO, -1, -1, NULL, 0));
    avformat_close_input(&input);
    unlink(output);
}
int main(int argc, char **argv) {
    media_fixture = argc > 1 ? argv[1] : NULL;
    UNITY_BEGIN();
    RUN_TEST(test_malformed_generated_lease_is_bounded_and_not_stored);
    RUN_TEST(test_issuance_rejects_missing_camera_and_invalid_principal);
    RUN_TEST(test_lease_capacity_fails_closed_and_reclaims_only_expired_slot);
    RUN_TEST(test_basic_audio_directions_and_live_revocation);
    RUN_TEST(test_instance_audio_disable_overrides_granted_media_rights);
    RUN_TEST(test_instance_audio_disable_remuxes_native_hls_despite_cached_grant);
    RUN_TEST(test_lease_binds_source_loopback_immutable_identity_and_expiry);
    RUN_TEST(test_existing_lease_does_not_follow_substream_name_reassignment);
    RUN_TEST(test_disabled_account_revokes_existing_basic_lease);
    RUN_TEST(test_session_and_legacy_key_revocation);
    RUN_TEST(test_scoped_token_intersects_owner_and_observes_revocation);
    RUN_TEST(test_owned_native_hls_segment_preserves_video_and_removes_audio);
    return UNITY_END();
}
