/** Regression coverage for the private system-info parsers. */
#include "../../src/web/api_handlers_system.c"
#include "unity.h"

void setUp(void) {}
void tearDown(void) {}

static void test_os_release_values(void) {
    const char *inputs[] = {"", " \t\n", "Ubuntu", "  Ubuntu  \n", "\"Ubuntu 26.04\"\n",
                            "'Debian GNU/Linux'", "\"\"", "''", "x", "\"x\""};
    const char *expected[] = {"", "", "Ubuntu", "Ubuntu", "Ubuntu 26.04",
                              "Debian GNU/Linux", "", "", "x", "x"};
    for (size_t i = 0; i < sizeof(inputs) / sizeof(inputs[0]); ++i) {
        char output[64];
        trim_copy_value(output, sizeof(output), inputs[i]);
        TEST_ASSERT_EQUAL_STRING(expected[i], output);
    }
    char short_output[4];
    trim_copy_value(short_output, sizeof(short_output), "\"Ubuntu\"");
    TEST_ASSERT_EQUAL_STRING("Ubu", short_output);
    trim_copy_value(short_output, 1, "x");
    TEST_ASSERT_EQUAL_STRING("", short_output);
    trim_copy_value(short_output, sizeof(short_output), NULL);
    TEST_ASSERT_EQUAL_STRING("", short_output);
}

static void test_detector_name_uses_complete_first_argument(void) {
    const char binary[] = "/opt/light-object-detect\0--port\08000\0";
    const char bare[] = "light-object-detect\0";
    const char argument[] = "bash\0-c\0cat light-object-detect.log\0";
    const char prefix[] = "light-object-detect-helper\0";
    const char directory[] = "/opt/light-object-detect/other\0";
    const char truncated[] = "light-object-detect";
    TEST_ASSERT_TRUE(is_detector_command(binary, sizeof(binary)));
    TEST_ASSERT_TRUE(is_detector_command(bare, sizeof(bare)));
    TEST_ASSERT_FALSE(is_detector_command(argument, sizeof(argument)));
    TEST_ASSERT_FALSE(is_detector_command(prefix, sizeof(prefix)));
    TEST_ASSERT_FALSE(is_detector_command(directory, sizeof(directory)));
    TEST_ASSERT_FALSE(is_detector_command(truncated, sizeof(truncated) - 1));
    TEST_ASSERT_FALSE(is_detector_command(bare, 0));
    TEST_ASSERT_FALSE(is_detector_command(NULL, 0));
}

int main(void) {
    UNITY_BEGIN();
    RUN_TEST(test_os_release_values);
    RUN_TEST(test_detector_name_uses_complete_first_argument);
    return UNITY_END();
}
