/**
 * @file test_api_detection.c
 * @brief Layer 2 Unity tests for API detection URL validation.
 */

#define _POSIX_C_SOURCE 200809L

#include <string.h>

#include "unity.h"
#include "core/logger.h"
#include "video/api_detection.h"

void setUp(void) {
    TEST_ASSERT_EQUAL_INT(0, init_api_detection_system());
}

void tearDown(void) {
    shutdown_api_detection_system();
}

static void assert_invalid_detection_url(const char *url) {
    detection_result_t result;
    memset(&result, 0xAB, sizeof(result));

    int rc = detect_objects_api(url, NULL, 0, 0, 0, &result, NULL, 0.5f, 0);

    TEST_ASSERT_EQUAL_INT(-1, rc);
    TEST_ASSERT_EQUAL_INT(0, result.count);
}

void test_detect_objects_api_rejects_url_with_space(void) {
    assert_invalid_detection_url("http://localhost:9001/api detect");
}

void test_detect_objects_api_rejects_url_with_userinfo(void) {
    assert_invalid_detection_url("http://user:pass@localhost:9001/detect");
}

void test_detect_objects_api_rejects_url_with_fragment(void) {
    assert_invalid_detection_url("http://localhost:9001/detect#frag");
}

void test_detect_objects_api_rejects_url_with_multiple_query_markers(void) {
    assert_invalid_detection_url("http://localhost:9001/detect?foo=1?bar=2");
}

void test_api_detection_uses_go2rtc_snapshot_only_without_decoded_frame(void) {
    TEST_ASSERT_TRUE(api_detection_should_use_go2rtc_snapshot(NULL, 0, 0, 0, "cam1"));

    const unsigned char frame_data[3] = {0, 0, 0};
    TEST_ASSERT_FALSE(api_detection_should_use_go2rtc_snapshot(frame_data, 1, 1, 3, "cam1"));
}

void test_api_detection_skips_go2rtc_snapshot_without_stream_name(void) {
    TEST_ASSERT_FALSE(api_detection_should_use_go2rtc_snapshot(NULL, 0, 0, 0, NULL));
    TEST_ASSERT_FALSE(api_detection_should_use_go2rtc_snapshot(NULL, 0, 0, 0, ""));
}

int main(void) {
    init_logger();
    UNITY_BEGIN();
    RUN_TEST(test_detect_objects_api_rejects_url_with_space);
    RUN_TEST(test_detect_objects_api_rejects_url_with_userinfo);
    RUN_TEST(test_detect_objects_api_rejects_url_with_fragment);
    RUN_TEST(test_detect_objects_api_rejects_url_with_multiple_query_markers);
    RUN_TEST(test_api_detection_uses_go2rtc_snapshot_only_without_decoded_frame);
    RUN_TEST(test_api_detection_skips_go2rtc_snapshot_without_stream_name);
    return UNITY_END();
}