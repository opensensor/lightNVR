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
    /* Uppercase hex digits: RFC 3986 §2.1 says producers should normalize to
     * uppercase, which url_apply_credentials has done since 650c2987. */
    TEST_ASSERT_EQUAL_STRING("rtsp://new%40user:p%3Ass@camera/live", url);
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

/* ---- url_build_onvif_service_url ---- */

void test_url_build_onvif_service_url_rtsp_to_http_with_device_service(void) {
    char url[256];
    TEST_ASSERT_EQUAL_INT(0, url_build_onvif_service_url("rtsp://alice:secret@camera:554/live", 8080, "/onvif/device_service", url, sizeof(url)));
    TEST_ASSERT_EQUAL_STRING("http://camera:8080/onvif/device_service", url);
}

void test_url_build_onvif_service_url_rtsps_maps_to_https(void) {
    char url[256];
    TEST_ASSERT_EQUAL_INT(0, url_build_onvif_service_url("rtsps://camera:322/live", 0, "/onvif/ptz_service", url, sizeof(url)));
    /* rtsps + port 322 -> https + port 443 */
    TEST_ASSERT_EQUAL_STRING("https://camera:443/onvif/ptz_service", url);
}

void test_url_build_onvif_service_url_rtsp_port_554_maps_to_80(void) {
    char url[256];
    TEST_ASSERT_EQUAL_INT(0, url_build_onvif_service_url("rtsp://camera:554/live", 0, "/onvif/device_service", url, sizeof(url)));
    TEST_ASSERT_EQUAL_STRING("http://camera:80/onvif/device_service", url);
}

void test_url_build_onvif_service_url_explicit_onvif_port_overrides_mapping(void) {
    char url[256];
    TEST_ASSERT_EQUAL_INT(0, url_build_onvif_service_url("rtsp://user:pass@camera:554/stream", 8899, "/onvif/ptz_service", url, sizeof(url)));
    TEST_ASSERT_EQUAL_STRING("http://camera:8899/onvif/ptz_service", url);
}

void test_url_build_onvif_service_url_strips_credentials(void) {
    char url[256];
    TEST_ASSERT_EQUAL_INT(0, url_build_onvif_service_url("rtsp://admin:secret@192.168.1.1:8554/ch0", 80, "/onvif/device_service", url, sizeof(url)));
    TEST_ASSERT_EQUAL_STRING("http://192.168.1.1:80/onvif/device_service", url);
}

void test_url_build_onvif_service_url_null_path_returns_base_url(void) {
    char url[256];
    TEST_ASSERT_EQUAL_INT(0, url_build_onvif_service_url("rtsp://user:pass@camera:8080/stream", 8080, NULL, url, sizeof(url)));
    TEST_ASSERT_EQUAL_STRING("http://camera:8080", url);
}

void test_url_build_onvif_service_url_https_source_preserved(void) {
    char url[256];
    TEST_ASSERT_EQUAL_INT(0, url_build_onvif_service_url("https://camera:443/onvif/device_service", 0, "/onvif/ptz_service", url, sizeof(url)));
    TEST_ASSERT_EQUAL_STRING("https://camera:443/onvif/ptz_service", url);
}

void test_simple_url_escape(void) {
    char url[256];
    simple_url_escape("^.*$ /cgi-bin/httpreq.pl", url, 256);
    TEST_ASSERT_EQUAL_STRING("%5E.%2A%24%20%2Fcgi-bin%2Fhttpreq.pl", url);
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
    RUN_TEST(test_url_build_onvif_service_url_rtsp_to_http_with_device_service);
    RUN_TEST(test_url_build_onvif_service_url_rtsps_maps_to_https);
    RUN_TEST(test_url_build_onvif_service_url_rtsp_port_554_maps_to_80);
    RUN_TEST(test_url_build_onvif_service_url_explicit_onvif_port_overrides_mapping);
    RUN_TEST(test_url_build_onvif_service_url_strips_credentials);
    RUN_TEST(test_url_build_onvif_service_url_null_path_returns_base_url);
    RUN_TEST(test_url_build_onvif_service_url_https_source_preserved);
    RUN_TEST(test_simple_url_escape);
    return UNITY_END();
}