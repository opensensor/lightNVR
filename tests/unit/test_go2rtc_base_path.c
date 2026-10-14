#include <string.h>
#include "unity.h"
#include "utils/yaml_validate.h"
#include "video/go2rtc/go2rtc_api.h"

void setUp(void) { go2rtc_api_apply_base_path_override(NULL, 0); }
void tearDown(void) { go2rtc_api_apply_base_path_override(NULL, 0); }

static void apply(const char *yaml, const char *expected) {
    if (!yaml_validate_is_available()) TEST_IGNORE_MESSAGE("libyaml unavailable");
    TEST_ASSERT_TRUE(go2rtc_api_apply_base_path_override(yaml, strlen(yaml)));
    TEST_ASSERT_EQUAL_STRING(expected, go2rtc_api_get_base_path());
}

void test_override_forms_and_default(void) {
    apply("api:\n  base_path: /foo\n", "/foo");
    apply("api: {base_path: '/nested/go2rtc'}\n", "/nested/go2rtc");
    apply("api: {base_path: \"\"}\n", "");
    apply("api:\n  listen: ':1984'\n", "/go2rtc");
    apply("api: {base_path: null}\n", "/go2rtc");
    apply("api: {base_path: }\n", "/go2rtc");
    apply("streams: {base_path: /not-api}\n", "/go2rtc");
    apply("api: &api {base_path: /alias}\nother: *api\n", "/alias");
    TEST_ASSERT_TRUE(go2rtc_api_apply_base_path_override(NULL, 0));
    TEST_ASSERT_EQUAL_STRING("/go2rtc", go2rtc_api_get_base_path());
}

void test_invalid_override_preserves_active_path(void) {
    apply("api: {base_path: /foo}", "/foo");
    const char *invalid[] = {
        "api: {base_path: /foo/}", "api: {base_path: relative}",
        "api: {base_path: '/foo?query'}", "api: {base_path: ['/foo']}",
        "api: {base_path: '/foo/../admin'}", "api: {base_path: \"/foo\\r\\nHeader\"}",
        "api: {base_path: \"/foo\\0bar\"}", "api: ["
    };
    for (size_t i = 0; i < sizeof(invalid) / sizeof(invalid[0]); ++i) {
        TEST_ASSERT_FALSE(go2rtc_api_apply_base_path_override(invalid[i], strlen(invalid[i])));
        TEST_ASSERT_EQUAL_STRING("/foo", go2rtc_api_get_base_path());
    }
}

int main(void) {
    UNITY_BEGIN();
    RUN_TEST(test_override_forms_and_default);
    RUN_TEST(test_invalid_override_preserves_active_path);
    return UNITY_END();
}
