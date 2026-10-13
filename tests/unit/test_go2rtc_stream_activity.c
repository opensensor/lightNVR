#define _POSIX_C_SOURCE 200809L

#include <arpa/inet.h>
#include <pthread.h>
#include <string.h>
#include <sys/socket.h>
#include <unistd.h>
#include <curl/curl.h>

#include "unity.h"
#include "core/camera_selector.h"
#include "video/go2rtc/go2rtc_api.h"
#include "web/fleet_health.h"

void setUp(void) {}
void tearDown(void) {}

void test_parser_recognizes_video_packets_and_substreams(void) {
    const char *json =
        "{"
        "\"camera-a\":{\"producers\":[{\"receivers\":[{"
          "\"codec\":{\"codec_name\":\"h264\",\"codec_type\":\"video\"},"
          "\"bytes\":8192,\"packets\":64}]}],\"consumers\":[{}]},"
        "\"camera-b\":{\"producers\":[{\"url\":\"wyze://idle\"}]},"
        "\"camera-b_sub\":{\"producers\":[{\"bytes_recv\":4096,"
          "\"medias\":[\"video, recvonly, H264\"]}]},"
        "\"camera-c\":{\"producers\":[{\"receivers\":[{"
          "\"codec\":{\"codec_name\":\"opus\",\"codec_type\":\"audio\"},"
          "\"bytes\":2048,\"packets\":32}]}]},"
        "\"camera-d\":{\"producers\":[{\"receivers\":[{"
          "\"codec\":{\"codec_name\":\"h264\",\"codec_type\":\"video\"},"
          "\"bytes\":0,\"packets\":0}]}]},"
        "\"camera-e\":{\"producers\":[],\"consumers\":[{"
          "\"bytes_send\":8192}]}"
        "}";
    go2rtc_stream_activity_t activity[] = {
        { .stream_name = "camera-a" },
        { .stream_name = "camera-b",
          .alternate_stream_name = "camera-b_sub" },
        { .stream_name = "camera-c" },
        { .stream_name = "camera-d" },
        { .stream_name = "camera-e" },
        { .stream_name = "missing", .video_active = true },
    };

    TEST_ASSERT_TRUE(go2rtc_api_parse_stream_activity_json(
        json, activity, sizeof(activity) / sizeof(activity[0])));
    TEST_ASSERT_TRUE(activity[0].video_active);
    TEST_ASSERT_TRUE(activity[1].video_active);
    TEST_ASSERT_FALSE(activity[2].video_active);
    TEST_ASSERT_FALSE(activity[3].video_active);
    TEST_ASSERT_FALSE(activity[4].video_active);
    TEST_ASSERT_FALSE(activity[5].video_active);
}

void test_parser_rejects_invalid_json_and_clears_results(void) {
    go2rtc_stream_activity_t activity = {
        .stream_name = "camera-a",
        .video_active = true,
    };
    TEST_ASSERT_FALSE(go2rtc_api_parse_stream_activity_json(
        "not json", &activity, 1));
    TEST_ASSERT_FALSE(activity.video_active);
}

static fleet_camera_t camera_with_health(const char *name,
                                         fleet_health_state_t health) {
    fleet_camera_t camera;
    memset(&camera, 0, sizeof(camera));
    strncpy(camera.name, name, sizeof(camera.name) - 1);
    camera.enabled = true;
    camera.streaming_enabled = true;
    camera.health = health;
    camera.availability = FLEET_AVAILABILITY_NEVER_CONNECTED;
    return camera;
}

void test_merge_promotes_only_unknown_active_cameras(void) {
    fleet_camera_t cameras[] = {
        camera_with_health("active-main", FLEET_HEALTH_UNKNOWN),
        camera_with_health("active-sub", FLEET_HEALTH_UNKNOWN),
        camera_with_health("already-degraded", FLEET_HEALTH_DEGRADED),
        camera_with_health("inactive", FLEET_HEALTH_UNKNOWN),
    };
    cameras[1].first_video_at = 123;
    go2rtc_stream_activity_t activity[] = {
        { .stream_name = "active-main", .video_active = true },
        { .stream_name = "active-sub", .video_active = true },
        { .stream_name = "already-degraded", .video_active = true },
        { .stream_name = "inactive", .video_active = false },
    };

    fleet_health_apply_go2rtc_activity(
        cameras, 4, activity, 4, (time_t)456);

    TEST_ASSERT_EQUAL_INT(FLEET_HEALTH_UP, cameras[0].health);
    TEST_ASSERT_EQUAL_INT(FLEET_AVAILABILITY_LIVE, cameras[0].availability);
    TEST_ASSERT_EQUAL_INT64(456, cameras[0].last_frame_ts);
    TEST_ASSERT_EQUAL_INT64(456, cameras[0].first_video_at);
    TEST_ASSERT_EQUAL_INT64(456, cameras[0].health_changed_at);

    TEST_ASSERT_EQUAL_INT(FLEET_HEALTH_UP, cameras[1].health);
    TEST_ASSERT_EQUAL_INT64(123, cameras[1].first_video_at);
    TEST_ASSERT_EQUAL_INT(FLEET_HEALTH_DEGRADED, cameras[2].health);
    TEST_ASSERT_EQUAL_INT(FLEET_HEALTH_UNKNOWN, cameras[3].health);
}

void test_merge_preserves_administratively_disabled_camera(void) {
    fleet_camera_t camera =
        camera_with_health("disabled", FLEET_HEALTH_UNKNOWN);
    camera.enabled = false;
    camera.streaming_enabled = false;
    camera.availability = FLEET_AVAILABILITY_DISABLED;
    go2rtc_stream_activity_t activity = {
        .stream_name = "disabled",
        .video_active = true,
    };

    fleet_health_apply_go2rtc_activity(
        &camera, 1, &activity, 1, (time_t)456);

    TEST_ASSERT_EQUAL_INT(FLEET_HEALTH_UNKNOWN, camera.health);
    TEST_ASSERT_EQUAL_INT(FLEET_AVAILABILITY_DISABLED, camera.availability);
    TEST_ASSERT_EQUAL_INT64(0, camera.last_frame_ts);
}

typedef struct {
    int fd;
    int delay_ms;
    bool passive_request;
} activity_server_t;

static void *serve_activity(void *arg) {
    activity_server_t *server = arg;
    int client = accept(server->fd, NULL, NULL);
    if (client < 0) return NULL;
    char request[2048] = {0};
    ssize_t received = read(client, request, sizeof(request) - 1);
    server->passive_request = received > 0 &&
        strstr(request, "GET " GO2RTC_BASE_PATH "/api/streams HTTP/") == request;
    struct timespec delay = {
        .tv_sec = server->delay_ms / 1000,
        .tv_nsec = (server->delay_ms % 1000) * 1000000L,
    };
    nanosleep(&delay, NULL);
    const char response[] =
        "HTTP/1.1 200 OK\r\nConnection: close\r\n\r\n"
        "{\"camera-a\":{\"producers\":[{\"bytes_recv\":4096,"
        "\"medias\":[\"video, recvonly, H264\"]}]}}";
    (void)send(client, response, sizeof(response) - 1, MSG_NOSIGNAL);
    close(client);
    return NULL;
}

static void check_activity_request(int delay_ms, bool expected_success) {
    TEST_ASSERT_EQUAL_INT(CURLE_OK, curl_global_init(CURL_GLOBAL_ALL));
    activity_server_t server = { .delay_ms = delay_ms };
    server.fd = socket(AF_INET, SOCK_STREAM, 0);
    TEST_ASSERT_GREATER_OR_EQUAL_INT(0, server.fd);
    struct sockaddr_in address = {
        .sin_family = AF_INET,
        .sin_addr.s_addr = htonl(INADDR_LOOPBACK),
    };
    TEST_ASSERT_EQUAL_INT(0, bind(server.fd, (struct sockaddr *)&address, sizeof(address)));
    TEST_ASSERT_EQUAL_INT(0, listen(server.fd, 1));
    socklen_t size = sizeof(address);
    TEST_ASSERT_EQUAL_INT(0, getsockname(server.fd, (struct sockaddr *)&address, &size));
    TEST_ASSERT_TRUE(go2rtc_api_init("127.0.0.1", ntohs(address.sin_port)));
    pthread_t thread;
    TEST_ASSERT_EQUAL_INT(0, pthread_create(&thread, NULL, serve_activity, &server));

    go2rtc_stream_activity_t activity = { .stream_name = "camera-a", .video_active = true };
    struct timespec before, after;
    clock_gettime(CLOCK_MONOTONIC, &before);
    bool success = go2rtc_api_get_stream_activity(&activity, 1);
    clock_gettime(CLOCK_MONOTONIC, &after);
    long elapsed_ms = (after.tv_sec - before.tv_sec) * 1000L +
        (after.tv_nsec - before.tv_nsec) / 1000000L;
    pthread_join(thread, NULL);
    close(server.fd);
    go2rtc_api_cleanup();
    curl_global_cleanup();

    TEST_ASSERT_EQUAL_INT(expected_success, success);
    TEST_ASSERT_EQUAL_INT(expected_success, activity.video_active);
    TEST_ASSERT_TRUE(server.passive_request);
    TEST_ASSERT_LESS_THAN_INT64(1000, elapsed_ms);
}

void test_activity_read_does_not_wait_for_busy_go2rtc(void) {
    check_activity_request(1200, false);
}

void test_activity_read_recognizes_video_without_dialing_camera(void) {
    check_activity_request(0, true);
}

int main(void) {
    UNITY_BEGIN();
    RUN_TEST(test_parser_recognizes_video_packets_and_substreams);
    RUN_TEST(test_parser_rejects_invalid_json_and_clears_results);
    RUN_TEST(test_merge_promotes_only_unknown_active_cameras);
    RUN_TEST(test_merge_preserves_administratively_disabled_camera);
    RUN_TEST(test_activity_read_does_not_wait_for_busy_go2rtc);
    RUN_TEST(test_activity_read_recognizes_video_without_dialing_camera);
    return UNITY_END();
}
