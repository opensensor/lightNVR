/* Exercise the actual private name classifier, never the server entry point or
 * PID-file/signal path. All /proc fopen requests are intercepted below. */
#define main t7_55_unexecuted_server_main
#include "../../src/core/main.c"
#undef main
#include "unity.h"

static FILE *fixture;
static bool armed, fail_open, wrong_stream, missing_terminator;
static unsigned opens, reads, closes;
static int requested_size;
static char requested_path[96];

FILE *__real_fopen(const char *, const char *);
char *__real_fgets(char *, int, FILE *);
int __real_fclose(FILE *);

FILE *__wrap_fopen(const char *path, const char *mode) {
    if (!armed) return __real_fopen(path, mode);
    opens++;
    snprintf(requested_path, sizeof(requested_path), "%s", path);
    if (strcmp(mode, "r") != 0) wrong_stream = true;
    if (fail_open) { errno = EACCES; return NULL; }
    return fixture;
}

char *__wrap_fgets(char *buffer, int size, FILE *file) {
    if (!armed) return __real_fgets(buffer, size, file);
    reads++;
    requested_size = size;
    if (file != fixture) wrong_stream = true;
    char *result = __real_fgets(buffer, size, file);
    if (result && (size <= 0 || !memchr(buffer, '\0', (size_t)size)))
        missing_terminator = true;
    return result;
}

int __wrap_fclose(FILE *file) {
    if (armed) {
        closes++;
        if (file != fixture) wrong_stream = true;
        if (file == fixture) fixture = NULL;
    }
    return __real_fclose(file);
}

void setUp(void) {
    fixture = NULL;
    armed = fail_open = wrong_stream = missing_terminator = false;
    opens = reads = closes = 0;
    requested_size = 0;
    requested_path[0] = '\0';
}

void tearDown(void) {
    armed = false;
    if (fixture) { TEST_ASSERT_EQUAL_INT(0, fclose(fixture)); fixture = NULL; }
}

static void classify(const void *bytes, size_t length, bool expected) {
    fixture = tmpfile();
    TEST_ASSERT_NOT_NULL(fixture);
    TEST_ASSERT_EQUAL_size_t(length, fwrite(bytes, 1, length, fixture));
    rewind(fixture);
    armed = true;
    bool result = is_lightnvr_process(INT_MAX);
    armed = false;
    TEST_ASSERT_EQUAL(expected, result);
    TEST_ASSERT_EQUAL_UINT(1, opens);
    TEST_ASSERT_EQUAL_UINT(1, reads);
    TEST_ASSERT_EQUAL_UINT(1, closes);
    TEST_ASSERT_EQUAL_INT(32, requested_size);
    TEST_ASSERT_EQUAL_STRING("/proc/2147483647/comm", requested_path);
    TEST_ASSERT_FALSE(wrong_stream);
    TEST_ASSERT_FALSE(missing_terminator);
    TEST_ASSERT_NULL(fixture);
}

static void test_exact_name_with_newline_matches(void) { classify("lightnvr\n", 9, true); }
static void test_exact_name_without_newline_matches(void) { classify("lightnvr", 8, true); }
static void test_unrelated_name_is_rejected(void) { classify("go2rtc\n", 7, false); }
static void test_name_prefix_is_not_an_exact_match(void) { classify("lightnvr-worker\n", 16, false); }
static void test_newline_only_is_rejected(void) { classify("\n", 1, false); }
static void test_empty_read_retains_existing_conservative_result(void) { classify("", 0, true); }
static void test_newline_stops_before_following_bytes(void) { classify("lightnvr\nother", 14, true); }
static void test_embedded_nul_stays_within_fgets_buffer(void) {
    static const char bytes[] = {'l','i','g','h','t','n','v','r','\0','x','\n'};
    classify(bytes, sizeof(bytes), true);
}
static void test_full_buffer_without_newline_is_terminated(void) {
    char bytes[31]; memset(bytes, 'x', sizeof(bytes));
    classify(bytes, sizeof(bytes), false);
}
static void test_overlong_input_is_bounded_and_rejected(void) {
    char bytes[4096]; memset(bytes, 'x', sizeof(bytes));
    classify(bytes, sizeof(bytes), false);
}
static void test_open_failure_preserves_conservative_result(void) {
    fail_open = armed = true;
    bool result = is_lightnvr_process(123);
    armed = false;
    TEST_ASSERT_TRUE(result);
    TEST_ASSERT_EQUAL_UINT(1, opens);
    TEST_ASSERT_EQUAL_UINT(0, reads);
    TEST_ASSERT_EQUAL_UINT(0, closes);
    TEST_ASSERT_EQUAL_STRING("/proc/123/comm", requested_path);
}
static ssize_t read_error(void *cookie, char *buffer, size_t size) {
    (void)cookie; (void)buffer; (void)size;
    errno = EIO;
    return -1;
}
static void test_read_error_closes_stream_and_preserves_conservative_result(void) {
    cookie_io_functions_t io = {.read = read_error};
    fixture = fopencookie(NULL, "r", io);
    TEST_ASSERT_NOT_NULL(fixture);
    armed = true;
    bool result = is_lightnvr_process(123);
    armed = false;
    TEST_ASSERT_TRUE(result);
    TEST_ASSERT_EQUAL_UINT(1, opens);
    TEST_ASSERT_EQUAL_UINT(1, reads);
    TEST_ASSERT_EQUAL_UINT(1, closes);
    TEST_ASSERT_FALSE(wrong_stream);
    TEST_ASSERT_NULL(fixture);
}

int main(void) {
    UNITY_BEGIN();
    RUN_TEST(test_exact_name_with_newline_matches);
    RUN_TEST(test_exact_name_without_newline_matches);
    RUN_TEST(test_unrelated_name_is_rejected);
    RUN_TEST(test_name_prefix_is_not_an_exact_match);
    RUN_TEST(test_newline_only_is_rejected);
    RUN_TEST(test_empty_read_retains_existing_conservative_result);
    RUN_TEST(test_newline_stops_before_following_bytes);
    RUN_TEST(test_embedded_nul_stays_within_fgets_buffer);
    RUN_TEST(test_full_buffer_without_newline_is_terminated);
    RUN_TEST(test_overlong_input_is_bounded_and_rejected);
    RUN_TEST(test_open_failure_preserves_conservative_result);
    RUN_TEST(test_read_error_closes_stream_and_preserves_conservative_result);
    return UNITY_END();
}
