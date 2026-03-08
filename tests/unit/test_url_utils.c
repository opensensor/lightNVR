#define _POSIX_C_SOURCE 200809L

#include "unity.h"
#include "core/url_utils.h"

void setUp(void) {}
void tearDown(void) {}

void test_url_apply_credentials_injects_credentials(void) {
    char url[256];
    TEST_ASSERT_EQUAL_INT(0, url_apply_credentials("rtsp://camera/live", "alice", "secret", url, sizeof(url)));
    TEST_ASSERT_EQUAL_STRING("rtsp://alice:secret@camera/live", url);
}

void test_url_apply_credentials_replaces_existing_credentials(void) {
    char url[256];
    TEST_ASSERT_EQUAL_INT(0, url_apply_credentials("rtsp://old:creds@camera/live", "new@user", "p:ss", url, sizeof(url)));
    TEST_ASSERT_EQUAL_STRING("rtsp://new%40user:p%3ass@camera/live", url);
}

void test_url_apply_credentials_preserves_fragment_suffix(void) {
    char url[256];
    TEST_ASSERT_EQUAL_INT(0, url_apply_credentials("rtsp://camera/live#transport=tcp#timeout=30", "alice", "secret", url, sizeof(url)));
    TEST_ASSERT_EQUAL_STRING("rtsp://alice:secret@camera/live#transport=tcp#timeout=30", url);
}

void test_url_strip_credentials_preserves_suffix(void) {
    char url[256];
    TEST_ASSERT_EQUAL_INT(0, url_strip_credentials("rtsp://alice:secret@camera/live#transport=tcp#timeout=30", url, sizeof(url)));
    TEST_ASSERT_EQUAL_STRING("rtsp://camera/live#transport=tcp#timeout=30", url);
}

void test_url_extract_credentials_decodes_values(void) {
    char username[64];
    char password[64];
    TEST_ASSERT_EQUAL_INT(0, url_extract_credentials("rtsp://alice%40cam:p%3Ass@camera/live", username, sizeof(username), password, sizeof(password)));
    TEST_ASSERT_EQUAL_STRING("alice@cam", username);
    TEST_ASSERT_EQUAL_STRING("p:ss", password);
}

void test_url_build_onvif_device_service_url_overrides_port_and_strips_credentials(void) {
    char url[256];
    TEST_ASSERT_EQUAL_INT(0, url_build_onvif_device_service_url("rtsp://alice:secret@camera:554/live", 8899, url, sizeof(url)));
    TEST_ASSERT_EQUAL_STRING("http://camera:8899/onvif/device_service", url);
}

void test_url_build_onvif_device_service_url_preserves_https_scheme(void) {
    char url[256];
    TEST_ASSERT_EQUAL_INT(0, url_build_onvif_device_service_url("https://alice:secret@camera/onvif/device_service", 7443, url, sizeof(url)));
    TEST_ASSERT_EQUAL_STRING("https://camera:7443/onvif/device_service", url);
}

int main(void) {
    UNITY_BEGIN();
    RUN_TEST(test_url_apply_credentials_injects_credentials);
    RUN_TEST(test_url_apply_credentials_replaces_existing_credentials);
    RUN_TEST(test_url_apply_credentials_preserves_fragment_suffix);
    RUN_TEST(test_url_strip_credentials_preserves_suffix);
    RUN_TEST(test_url_extract_credentials_decodes_values);
    RUN_TEST(test_url_build_onvif_device_service_url_overrides_port_and_strips_credentials);
    RUN_TEST(test_url_build_onvif_device_service_url_preserves_https_scheme);
    return UNITY_END();
}