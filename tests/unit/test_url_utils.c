#define _POSIX_C_SOURCE 200809L

#include <ctype.h>
#include <string.h>

#include "unity.h"
#include "core/url_utils.h"

void setUp(void) {}
void tearDown(void) {}

/* Percent-encoding hex digits are case-insensitive (RFC 3986 §2.1), and
 * url_apply_credentials() delegates encoding to libcurl, whose case varies
 * by version (e.g. libcurl 8.14.1 emits "%3a"). Normalize only the escape
 * digits so the rest of the URL is still compared exactly. */
static void uppercase_percent_escapes(char *url) {
    for (char *p = url; *p; p++) {
        if (p[0] == '%' && isxdigit((unsigned char)p[1]) && isxdigit((unsigned char)p[2])) {
            p[1] = (char)toupper((unsigned char)p[1]);
            p[2] = (char)toupper((unsigned char)p[2]);
            p += 2;
        }
    }
}

void test_url_apply_credentials_injects_credentials(void) {
    char url[256];
    TEST_ASSERT_EQUAL_INT(0, url_apply_credentials("rtsp://camera/live", "alice", "secret", url, sizeof(url)));
    TEST_ASSERT_EQUAL_STRING("rtsp://alice:secret@camera/live", url);
}

void test_url_apply_credentials_replaces_existing_credentials(void) {
    char url[256];
    TEST_ASSERT_EQUAL_INT(0, url_apply_credentials("rtsp://old:creds@camera/live", "new@user", "p:ss", url, sizeof(url)));
    uppercase_percent_escapes(url);
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

void test_url_redact_for_logging_removes_camera_credentials(void) {
    char url[256];
    TEST_ASSERT_EQUAL_INT(0, url_redact_for_logging(
        "rtsp://camera-user:p%40ssword@camera.local:8554/live?profile=main#transport=tcp",
        url, sizeof(url)));
    TEST_ASSERT_EQUAL_STRING(
        "rtsp://camera.local:8554/live?profile=main#transport=tcp", url);
    TEST_ASSERT_NULL(strstr(url, "camera-user"));
    TEST_ASSERT_NULL(strstr(url, "p%40ssword"));
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

void test_publish_url_next_splits_on_commas_and_whitespace(void) {
    char dest[PUBLISH_URL_MAX_LENGTH];
    const char *cursor = " rtmp://a/live/k1,rtmps://b/app/k2\n  rtmp://c/x ,";

    cursor = publish_url_next(cursor, dest, sizeof(dest));
    TEST_ASSERT_NOT_NULL(cursor);
    TEST_ASSERT_EQUAL_STRING("rtmp://a/live/k1", dest);
    cursor = publish_url_next(cursor, dest, sizeof(dest));
    TEST_ASSERT_NOT_NULL(cursor);
    TEST_ASSERT_EQUAL_STRING("rtmps://b/app/k2", dest);
    cursor = publish_url_next(cursor, dest, sizeof(dest));
    TEST_ASSERT_NOT_NULL(cursor);
    TEST_ASSERT_EQUAL_STRING("rtmp://c/x", dest);
    TEST_ASSERT_NULL(publish_url_next(cursor, dest, sizeof(dest)));
}

void test_publish_url_next_empty_list(void) {
    char dest[PUBLISH_URL_MAX_LENGTH];
    TEST_ASSERT_NULL(publish_url_next("", dest, sizeof(dest)));
    TEST_ASSERT_NULL(publish_url_next(" , ", dest, sizeof(dest)));
}

void test_publish_url_list_is_valid(void) {
    TEST_ASSERT_TRUE(publish_url_list_is_valid(""));
    TEST_ASSERT_TRUE(publish_url_list_is_valid(NULL));
    TEST_ASSERT_TRUE(publish_url_list_is_valid("rtmp://a.rtmp.youtube.com/live2/abcd-1234"));
    TEST_ASSERT_TRUE(publish_url_list_is_valid("rtmp://a/live/k1, RTMPS://b/app/k2"));

    TEST_ASSERT_FALSE(publish_url_list_is_valid("rtmp://a/live/k1, file:///etc/passwd"));
    TEST_ASSERT_FALSE(publish_url_list_is_valid("http://example.com/x"));
    TEST_ASSERT_FALSE(publish_url_list_is_valid("rtmp://"));
    TEST_ASSERT_FALSE(publish_url_list_is_valid("rtmp://a/\"k"));
    TEST_ASSERT_FALSE(publish_url_list_is_valid("rtmp://a/k\x01"));
}

void test_publish_url_newline_cannot_smuggle_yaml(void) {
    /* A newline is a separator, so the injected "line" becomes its own
     * destination and fails the scheme check. */
    TEST_ASSERT_FALSE(publish_url_list_is_valid("rtmp://a/k\nstreams:\n  x: exec:id"));
}

void test_publish_url_rejects_overlong_destination(void) {
    char list[PUBLISH_URL_MAX_LENGTH + 16];
    memset(list, 'a', sizeof(list) - 1);
    memcpy(list, "rtmp://", 7);
    list[sizeof(list) - 1] = '\0';
    TEST_ASSERT_FALSE(publish_url_list_is_valid(list));
}

int main(void) {
    UNITY_BEGIN();
    RUN_TEST(test_url_apply_credentials_injects_credentials);
    RUN_TEST(test_url_apply_credentials_replaces_existing_credentials);
    RUN_TEST(test_url_apply_credentials_preserves_fragment_suffix);
    RUN_TEST(test_url_strip_credentials_preserves_suffix);
    RUN_TEST(test_url_redact_for_logging_removes_camera_credentials);
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
    RUN_TEST(test_publish_url_next_splits_on_commas_and_whitespace);
    RUN_TEST(test_publish_url_next_empty_list);
    RUN_TEST(test_publish_url_list_is_valid);
    RUN_TEST(test_publish_url_newline_cannot_smuggle_yaml);
    RUN_TEST(test_publish_url_rejects_overlong_destination);
    return UNITY_END();
}
