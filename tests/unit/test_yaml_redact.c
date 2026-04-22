/**
 * @file test_yaml_redact.c
 * @brief Unit tests for the YAML-aware secret redactor (T7).
 *
 * When LIGHTNVR_HAVE_LIBYAML is undefined, the redactor is a verbatim
 * pass-through; in that mode we only assert the structural API contract
 * (caller-owned, NUL-terminated, no crashes) and skip the masking checks.
 */

#define _POSIX_C_SOURCE 200809L

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "unity.h"
#include "utils/yaml_redact.h"

void setUp(void)    {}
void tearDown(void) {}

/* Helper: assert that @p needle does NOT appear in @p haystack (because it's
 * supposed to have been redacted out).  When the stub path is in use we
 * still want to fail loudly if the stub does something unexpected, so the
 * assertion only applies when redaction is actually available. */
static void assert_redacted_when_available(const char *haystack,
                                           const char *needle)
{
    if (yaml_redact_is_available()) {
        TEST_ASSERT_NULL_MESSAGE(strstr(haystack, needle),
                                 "expected secret to be redacted");
    }
}

void test_redactor_returns_caller_owned_buffer(void) {
    const char *src = "api: { password: secret }\n";
    size_t out_len = 0;
    char *out = yaml_redact_alloc(src, strlen(src), &out_len);
    TEST_ASSERT_NOT_NULL(out);
    TEST_ASSERT_EQUAL_INT8('\0', out[out_len]);
    free(out);
}

void test_redactor_handles_empty_input(void) {
    size_t out_len = 0;
    char *out = yaml_redact_alloc(NULL, 0, &out_len);
    TEST_ASSERT_NOT_NULL(out);
    TEST_ASSERT_EQUAL_INT(0, out_len);
    free(out);
}

void test_redactor_masks_api_password(void) {
    const char *src =
        "api:\n"
        "  password: super-secret-123\n"
        "  username: admin\n";
    char *out = yaml_redact_alloc(src, strlen(src), NULL);
    TEST_ASSERT_NOT_NULL(out);
    assert_redacted_when_available(out, "super-secret-123");
    assert_redacted_when_available(out, "admin");
    if (yaml_redact_is_available()) {
        TEST_ASSERT_NOT_NULL(strstr(out, "<redacted>"));
    }
    free(out);
}

void test_redactor_masks_block_scalar_password(void) {
    /* This is the case a regex-based redactor would miss. */
    const char *src =
        "mqtt:\n"
        "  password: |\n"
        "    block-scalar-secret\n";
    char *out = yaml_redact_alloc(src, strlen(src), NULL);
    TEST_ASSERT_NOT_NULL(out);
    assert_redacted_when_available(out, "block-scalar-secret");
    free(out);
}

void test_redactor_masks_ice_server_credential(void) {
    const char *src =
        "webrtc:\n"
        "  ice_servers:\n"
        "    - url: turn:turn.example.com\n"
        "      username: turnuser-keep-or-redact\n"
        "      credential: turn-secret-XYZ\n";
    char *out = yaml_redact_alloc(src, strlen(src), NULL);
    TEST_ASSERT_NOT_NULL(out);
    assert_redacted_when_available(out, "turn-secret-XYZ");
    assert_redacted_when_available(out, "turnuser-keep-or-redact");
    free(out);
}

void test_redactor_masks_url_userinfo_in_streams(void) {
    const char *src =
        "streams:\n"
        "  cam1: rtsp://admin:hunter2@192.168.1.10/stream\n";
    char *out = yaml_redact_alloc(src, strlen(src), NULL);
    TEST_ASSERT_NOT_NULL(out);
    assert_redacted_when_available(out, "hunter2");
    /* Host should still be visible. */
    if (yaml_redact_is_available()) {
        TEST_ASSERT_NOT_NULL(strstr(out, "192.168.1.10/stream"));
    }
    free(out);
}

void test_redactor_preserves_non_secret_keys(void) {
    const char *src =
        "api:\n"
        "  listen: \":1984\"\n"
        "log:\n"
        "  level: trace\n";
    char *out = yaml_redact_alloc(src, strlen(src), NULL);
    TEST_ASSERT_NOT_NULL(out);
    if (yaml_redact_is_available()) {
        TEST_ASSERT_NOT_NULL(strstr(out, "1984"));
        TEST_ASSERT_NOT_NULL(strstr(out, "trace"));
        TEST_ASSERT_NULL(strstr(out, "<redacted>"));
    }
    free(out);
}

int main(void) {
    UNITY_BEGIN();
    RUN_TEST(test_redactor_returns_caller_owned_buffer);
    RUN_TEST(test_redactor_handles_empty_input);
    RUN_TEST(test_redactor_masks_api_password);
    RUN_TEST(test_redactor_masks_block_scalar_password);
    RUN_TEST(test_redactor_masks_ice_server_credential);
    RUN_TEST(test_redactor_masks_url_userinfo_in_streams);
    RUN_TEST(test_redactor_preserves_non_secret_keys);
    return UNITY_END();
}
