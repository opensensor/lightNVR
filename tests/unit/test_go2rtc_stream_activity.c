#include <string.h>

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

int main(void) {
    UNITY_BEGIN();
    RUN_TEST(test_parser_recognizes_video_packets_and_substreams);
    RUN_TEST(test_parser_rejects_invalid_json_and_clears_results);
    RUN_TEST(test_merge_promotes_only_unknown_active_cameras);
    RUN_TEST(test_merge_preserves_administratively_disabled_camera);
    return UNITY_END();
}
