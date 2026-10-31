#define _POSIX_C_SOURCE 200809L
#include <stdbool.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <cjson/cJSON.h>
#include "unity.h"
#include "core/logger.h"
#include "core/logger_json.h"
#define READ_LIMIT (100U * 1024U)
static char root[128], path[256];
static char **logs;
static int count, initialized;
static bool monitor, bad_tell, oversized_return;
static size_t short_limit, requested_read, first_allocation;
static unsigned allocations, reads, dup_calls, fail_allocation, fail_dup;
static const char entry[] = "{\"timestamp\":\"2026-01-01T00:00:00\",\"level\":\"info\",\"message\":\"tail\"}";
void *__real_malloc(size_t);
void *__wrap_malloc(size_t bytes) {
    if (monitor) {
        allocations++;
        if (allocations == 1) first_allocation = bytes;
        if (allocations == fail_allocation) return NULL;
    }
    return __real_malloc(bytes);
}
char *__real_strdup(const char *);
char *__wrap_strdup(const char *text) {
    if (monitor && ++dup_calls == fail_dup) return NULL;
    return __real_strdup(text);
}
long __real_ftell(FILE *);
long __wrap_ftell(FILE *file) { return monitor && bad_tell ? -1 : __real_ftell(file); }
size_t __real_fread(void *, size_t, size_t, FILE *);
size_t __wrap_fread(void *buffer, size_t width, size_t number, FILE *file) {
    if (!monitor) return __real_fread(buffer, width, number, file);
    reads++; requested_read = number;
    size_t result = __real_fread(buffer, width, number < short_limit ? number : short_limit, file);
    /* Impossible libc return-value seam, without writing outside requested bytes. */
    return oversized_return ? SIZE_MAX : result;
}
void setUp(void) {
    strcpy(root, "/tmp/lightnvr-t7-54-XXXXXX"); TEST_ASSERT_NOT_NULL(mkdtemp(root));
    snprintf(path, sizeof(path), "%s/log.json", root);
    logs = NULL; count = 0; initialized = 0;
    monitor = bad_tell = oversized_return = false;
    short_limit = SIZE_MAX; requested_read = first_allocation = 0;
    allocations = reads = dup_calls = fail_allocation = fail_dup = 0;
    TEST_ASSERT_EQUAL_INT(0, init_json_logger(path)); initialized = 1;
    TEST_ASSERT_EQUAL_INT(0, truncate(path, 0));
    log_clear_thread_context();
}
void tearDown(void) {
    monitor = false;
    for (int i = 0; i < count; i++) free(logs[i]);
    free(logs); logs = NULL; count = 0;
    log_clear_thread_context();
    if (initialized) shutdown_json_logger();
    unlink(path); TEST_ASSERT_EQUAL_INT(0, rmdir(root));
}
static void raw(const char *text) {
    FILE *file = fopen(path, "w"); TEST_ASSERT_NOT_NULL(file);
    TEST_ASSERT_EQUAL_UINT(strlen(text), fwrite(text, 1, strlen(text), file));
    TEST_ASSERT_EQUAL_INT(0, fclose(file));
}
static void retrieve(int expected, int expected_count) {
    monitor = true;
    int result = get_json_logs("debug", "", &logs, &count);
    monitor = false;
    TEST_ASSERT_EQUAL_INT(expected, result); TEST_ASSERT_EQUAL_INT(expected_count, count);
    if (expected != 0) TEST_ASSERT_NULL(logs);
    TEST_ASSERT_LESS_OR_EQUAL_UINT(READ_LIMIT, requested_read);
}
static void tail(void) {
    TEST_ASSERT_EQUAL_INT(1, count);
    cJSON *object = cJSON_Parse(logs[0]); TEST_ASSERT_NOT_NULL(object);
    const cJSON *message = cJSON_GetObjectItemCaseSensitive(object, "message");
    bool valid = cJSON_IsString(message) && strcmp(message->valuestring, "tail") == 0;
    cJSON_Delete(object); TEST_ASSERT_TRUE(valid);
}
static void test_empty_file_returns_no_allocation(void) {
    retrieve(0, 0); TEST_ASSERT_NULL(logs); TEST_ASSERT_EQUAL_UINT(0, allocations);
}
static void test_one_byte_invalid_json_is_terminated_and_ignored(void) {
    raw("x"); retrieve(0, 0); TEST_ASSERT_EQUAL_UINT(2, first_allocation);
}
static void test_final_line_without_newline_is_retained(void) {
    raw(entry); retrieve(0, 1); tail();
}
static void test_blank_and_malformed_lines_do_not_change_valid_record(void) {
    char text[512]; snprintf(text, sizeof(text), "\ninvalid\n{}\n%s\n\n", entry);
    raw(text); retrieve(0, 1); tail();
}
static void test_json_escape_and_thread_context_round_trip(void) {
    const char *message = "quote\" slash\\ newline\n tab\t control\001 utf8\xc3\xa9";
    log_set_thread_context("component\"", "camera\nname");
    TEST_ASSERT_EQUAL_INT(0, write_json_log(LOG_LEVEL_WARN, "2026-01-01", message));
    retrieve(0, 1);
    cJSON *object = cJSON_Parse(logs[0]); TEST_ASSERT_NOT_NULL(object);
    bool valid = strcmp(cJSON_GetObjectItem(object, "message")->valuestring, message) == 0 &&
        strcmp(cJSON_GetObjectItem(object, "component")->valuestring, "component\"") == 0 &&
        strcmp(cJSON_GetObjectItem(object, "stream")->valuestring, "camera\nname") == 0 &&
        strcmp(cJSON_GetObjectItem(object, "level")->valuestring, "warning") == 0;
    cJSON_Delete(object); TEST_ASSERT_TRUE(valid);
    TEST_ASSERT_NULL(strchr(logs[0], '\n'));
}
static void test_upstream_formatter_keeps_existing_4095_byte_truncation(void) {
    char message[5001]; memset(message, 'a', sizeof(message)-1); message[5000] = '\0';
    log_message(LOG_LEVEL_INFO, "%s", message);
    retrieve(0, 1);
    cJSON *object = cJSON_Parse(logs[0]); TEST_ASSERT_NOT_NULL(object);
    cJSON *value = cJSON_GetObjectItem(object, "message");
    bool valid = cJSON_IsString(value) && strlen(value->valuestring) == 4095 &&
                 strspn(value->valuestring, "a") == 4095;
    cJSON_Delete(object); TEST_ASSERT_TRUE(valid);
}
static void sized_file(size_t length) {
    FILE *file = fopen(path, "w"); TEST_ASSERT_NOT_NULL(file);
    size_t padding = length - strlen(entry) - 1;
    for (size_t i = 0; i < padding; i++) fputc(' ', file);
    fputs(entry, file); fputc('\n', file); TEST_ASSERT_EQUAL_INT(0, fclose(file));
    retrieve(0, 1); tail();
    size_t bounded = length < READ_LIMIT ? length : READ_LIMIT;
    TEST_ASSERT_EQUAL_UINT(bounded, requested_read);
    TEST_ASSERT_EQUAL_UINT(bounded + 1, first_allocation);
}
static void test_one_below_read_limit(void) { sized_file(READ_LIMIT - 1); }
static void test_exact_read_limit(void) { sized_file(READ_LIMIT); }
static void test_one_past_read_limit(void) { sized_file(READ_LIMIT + 1); }
static void test_tail_window_skips_partial_leading_record(void) {
    FILE *file = fopen(path, "w"); TEST_ASSERT_NOT_NULL(file);
    for (unsigned i = 0; i < READ_LIMIT; i++) fputc('x', file);
    fprintf(file, "\n%s\n", entry); TEST_ASSERT_EQUAL_INT(0, fclose(file));
    retrieve(0, 1); tail(); TEST_ASSERT_EQUAL_UINT(READ_LIMIT + 1, first_allocation);
}
static void test_short_read_terminates_at_actual_read(void) {
    char text[512]; snprintf(text, sizeof(text), "%s\n%s\n", entry, entry); raw(text);
    short_limit = strlen(entry) + 1; retrieve(0, 1); tail();
}
static void test_zero_read_returns_no_records(void) {
    raw(entry); short_limit = 0; retrieve(0, 0); TEST_ASSERT_NULL(logs);
}
static void test_failed_tell_rejects_before_allocation(void) {
    raw(entry); bad_tell = true; retrieve(-1, 0);
    TEST_ASSERT_EQUAL_UINT(0, allocations); TEST_ASSERT_EQUAL_UINT(0, reads);
}
static void test_impossible_oversized_read_return_is_clamped(void) {
    raw(entry); oversized_return = true; retrieve(0, 1); tail();
}
static void test_read_buffer_allocation_failure_clears_outputs(void) {
    raw(entry); fail_allocation = 1; retrieve(-1, 0); TEST_ASSERT_EQUAL_UINT(0, reads);
}
static void test_lines_allocation_failure_releases_buffer(void) {
    raw(entry); fail_allocation = 2; retrieve(-1, 0);
}
static void test_second_line_copy_failure_releases_previous_line(void) {
    char text[512]; snprintf(text, sizeof(text), "%s\n%s\n", entry, entry); raw(text);
    fail_dup = 2; retrieve(-1, 0); TEST_ASSERT_EQUAL_UINT(2, dup_calls);
}
int main(void) {
    init_logger();
    UNITY_BEGIN();
    RUN_TEST(test_empty_file_returns_no_allocation);
    RUN_TEST(test_one_byte_invalid_json_is_terminated_and_ignored);
    RUN_TEST(test_final_line_without_newline_is_retained);
    RUN_TEST(test_blank_and_malformed_lines_do_not_change_valid_record);
    RUN_TEST(test_json_escape_and_thread_context_round_trip);
    RUN_TEST(test_upstream_formatter_keeps_existing_4095_byte_truncation);
    RUN_TEST(test_one_below_read_limit);
    RUN_TEST(test_exact_read_limit);
    RUN_TEST(test_one_past_read_limit);
    RUN_TEST(test_tail_window_skips_partial_leading_record);
    RUN_TEST(test_short_read_terminates_at_actual_read);
    RUN_TEST(test_zero_read_returns_no_records);
    RUN_TEST(test_failed_tell_rejects_before_allocation);
    RUN_TEST(test_impossible_oversized_read_return_is_clamped);
    RUN_TEST(test_read_buffer_allocation_failure_clears_outputs);
    RUN_TEST(test_lines_allocation_failure_releases_buffer);
    RUN_TEST(test_second_line_copy_failure_releases_previous_line);
    int result = UNITY_END(); shutdown_logger(); return result;
}
