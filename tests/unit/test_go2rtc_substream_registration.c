#define _POSIX_C_SOURCE 200809L

#include <netinet/in.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/socket.h>
#include <unistd.h>

#include "unity.h"
#include "core/config.h"
#include "database/db_streams.h"
#include "video/go2rtc/go2rtc_api.h"
#include "video/go2rtc/go2rtc_stream.h"

static char config_dir[128];
static char registered_name[MAX_STREAM_NAME + 8];
static char registered_sources[3][2048];
static int source_count;
static bool registration_succeeds;

/* Capture the API boundary; URL preparation and fallback selection are real. */
bool __wrap_go2rtc_api_add_stream_multi(const char *name, const char **sources,
                                       int count) {
    TEST_ASSERT_LESS_OR_EQUAL_INT(3, count);
    snprintf(registered_name, sizeof(registered_name), "%s", name);
    source_count = count;
    for (int i = 0; i < count; i++) {
        snprintf(registered_sources[i], sizeof(registered_sources[i]), "%s", sources[i]);
    }
    return registration_succeeds;
}

bool __wrap_go2rtc_api_add_stream(const char *name, const char *source) {
    return __wrap_go2rtc_api_add_stream_multi(name, &source, 1);
}

int __wrap_get_stream_config_by_name(const char *name, stream_config_t *config) {
    (void)name;
    (void)config;
    return -1; /* No publish destination. */
}

void setUp(void) {
    source_count = 0;
    registration_succeeds = true;
    memset(registered_sources, 0, sizeof(registered_sources));
    load_default_config(&g_config);
    g_config.hw_accel_enabled = false;
    g_config.go2rtc_enabled = true;
    g_config.go2rtc_webrtc_enabled = false;

    int fd = socket(AF_INET, SOCK_STREAM, 0);
    TEST_ASSERT_GREATER_OR_EQUAL_INT(0, fd);
    struct sockaddr_in address = {0};
    address.sin_family = AF_INET;
    address.sin_addr.s_addr = htonl(INADDR_LOOPBACK);
    TEST_ASSERT_EQUAL_INT(0, bind(fd, (struct sockaddr *)&address, sizeof(address)));
    socklen_t size = sizeof(address);
    TEST_ASSERT_EQUAL_INT(0, getsockname(fd, (struct sockaddr *)&address, &size));
    int port = ntohs(address.sin_port);
    close(fd);

    char template[] = "/tmp/lightnvr-substream-XXXXXX";
    char *dir = mkdtemp(template);
    TEST_ASSERT_NOT_NULL(dir);
    snprintf(config_dir, sizeof(config_dir), "%s", dir);
    snprintf(g_config.log_file, sizeof(g_config.log_file), "%s/lightnvr.log", dir);
    g_config.go2rtc_api_port = port;

    char port_text[16];
    snprintf(port_text, sizeof(port_text), "%d", port);
    setenv("FAKE_GO2RTC_PORT", port_text, 1);
    TEST_ASSERT_TRUE(go2rtc_stream_init(FAKE_GO2RTC_PATH, dir, port));
    TEST_ASSERT_TRUE(go2rtc_stream_start_service());
}

void tearDown(void) {
    go2rtc_stream_cleanup();
    free(g_config.streams);
    g_config.streams = NULL;
    unsetenv("FAKE_GO2RTC_PORT");
    const char *files[] = {"go2rtc.yaml", "override.yaml", "go2rtc.log", "lightnvr.log"};
    for (size_t i = 0; i < sizeof(files) / sizeof(files[0]); i++) {
        char path[192];
        snprintf(path, sizeof(path), "%s/%s", config_dir, files[i]);
        unlink(path);
    }
    rmdir(config_dir);
}

void test_substream_keeps_video_fallback_after_h264_main_registration(void) {
    TEST_ASSERT_TRUE(go2rtc_stream_register("Front Gate", "rtsp://camera/stream1",
        NULL, NULL, false, STREAM_PROTOCOL_TCP, false, "h264"));
    TEST_ASSERT_EQUAL_INT(1, source_count);

    /* #579: the main is H.264, but /stream8 advertises JPEG. Its codec has
     * not been probed separately, so preserve a fallback on the sub relay. */
    TEST_ASSERT_TRUE(go2rtc_stream_register_substream("Front Gate_sub",
        "rtsp://camera/stream8", "viewer", "secret", STREAM_PROTOCOL_TCP));
    TEST_ASSERT_EQUAL_STRING("Front Gate_sub", registered_name);
    TEST_ASSERT_EQUAL_INT(2, source_count);
    TEST_ASSERT_EQUAL_STRING(
        "rtsp://viewer:secret@camera/stream8#transport=tcp#timeout=30",
        registered_sources[0]);
    TEST_ASSERT_EQUAL_STRING("ffmpeg:Front Gate_sub#video=h264", registered_sources[1]);
}

void test_substream_fallback_respects_hardware_opt_in_and_udp(void) {
    g_config.hw_accel_enabled = true;
    TEST_ASSERT_TRUE(go2rtc_stream_register_substream("camera_sub",
        "rtsp://camera/sub", NULL, NULL, STREAM_PROTOCOL_UDP));
    TEST_ASSERT_EQUAL_INT(2, source_count);
    TEST_ASSERT_EQUAL_STRING("rtsp://camera/sub#transport=udp", registered_sources[0]);
    TEST_ASSERT_EQUAL_STRING("ffmpeg:camera_sub#video=h264#hardware", registered_sources[1]);
}

void test_substream_fallback_preserves_instance_audio_policy(void) {
    g_config.audio_disabled = true;
    TEST_ASSERT_TRUE(go2rtc_stream_register_substream("camera_sub",
        "rtsp://camera/sub#media=audio", NULL, NULL, STREAM_PROTOCOL_TCP));
    TEST_ASSERT_EQUAL_INT(2, source_count);
    TEST_ASSERT_EQUAL_STRING("rtsp://camera/sub#transport=tcp#timeout=30#media=video",
                             registered_sources[0]);
    TEST_ASSERT_EQUAL_STRING("ffmpeg:camera_sub#video=h264", registered_sources[1]);
}

void test_main_stream_audio_and_video_fallbacks_are_preserved(void) {
    TEST_ASSERT_TRUE(go2rtc_stream_register("camera", "rtsp://camera/main",
        NULL, NULL, true, STREAM_PROTOCOL_TCP, true, "hevc"));
    TEST_ASSERT_EQUAL_INT(3, source_count);
    TEST_ASSERT_EQUAL_STRING("rtsp://camera/main#transport=tcp#timeout=30#backchannel=1",
                             registered_sources[0]);
    TEST_ASSERT_EQUAL_STRING("ffmpeg:camera#audio=aac#audio=opus", registered_sources[1]);
    TEST_ASSERT_EQUAL_STRING("ffmpeg:camera#video=h264", registered_sources[2]);
}

void test_substream_reports_registration_failure(void) {
    registration_succeeds = false;
    TEST_ASSERT_FALSE(go2rtc_stream_register_substream("camera_sub",
        "rtsp://camera/sub", NULL, NULL, STREAM_PROTOCOL_TCP));
}

int main(void) {
    UNITY_BEGIN();
    RUN_TEST(test_substream_keeps_video_fallback_after_h264_main_registration);
    RUN_TEST(test_substream_fallback_respects_hardware_opt_in_and_udp);
    RUN_TEST(test_substream_fallback_preserves_instance_audio_policy);
    RUN_TEST(test_main_stream_audio_and_video_fallbacks_are_preserved);
    RUN_TEST(test_substream_reports_registration_failure);
    return UNITY_END();
}
