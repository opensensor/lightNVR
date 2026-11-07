#define _POSIX_C_SOURCE 200809L
#include "../../src/web/api_handlers_system_go2rtc.c"
#include "unity.h"

enum fault_kind { NONE, STAT_ERROR, ALLOC_ERROR, READ_ERROR, EARLY_EOF,
                  CLOSE_ERROR, INTERRUPTED_SHORT_READ, NEGATIVE_SIZE };
static enum fault_kind fault;
static unsigned read_calls;
static char path[128];
static const char *diagnostic_path;
static const char *db_value;
static size_t db_length;
static int db_status;
static bool process_available;
static int process_pid;
static const char secret[] = "fixture-secret";

int __real_fstat(int fd, struct stat *st);
int __wrap_fstat(int fd, struct stat *st) {
    if (fault == STAT_ERROR) { errno = EACCES; return -1; }
    int result = __real_fstat(fd, st);
    if (result == 0 && fault == NEGATIVE_SIZE) st->st_size = -1;
    return result;
}
void *__real_malloc(size_t size);
void *__wrap_malloc(size_t size) {
    if (fault == ALLOC_ERROR) { fault = NONE; errno = 0; return NULL; }
    return __real_malloc(size);
}
ssize_t __real_read(int fd, void *buffer, size_t count);
ssize_t __wrap_read(int fd, void *buffer, size_t count) {
    if (fault == READ_ERROR) { errno = EIO; return -1; }
    if (fault == EARLY_EOF) return 0;
    if (fault == INTERRUPTED_SHORT_READ) {
        if (read_calls++ == 0) { errno = EINTR; return -1; }
        if (count > 2) count = 2;
    }
    return __real_read(fd, buffer, count);
}
#ifdef __GLIBC__
/* Optimized fortified builds call this entry point instead of read(). Keep
 * the same injected faults without disabling production fortification. */
ssize_t __real___read_chk(int fd, void *buffer, size_t count, size_t capacity);
ssize_t __wrap___read_chk(int fd, void *buffer, size_t count, size_t capacity) {
    if (count > capacity) return __real___read_chk(fd, buffer, count, capacity);
    return __wrap_read(fd, buffer, count);
}
#endif
int __real_close(int fd);
int __wrap_close(int fd) {
    int result = __real_close(fd);
    if (fault == CLOSE_ERROR) { errno = EIO; return -1; }
    errno = EBADF; // A successful cleanup must not replace the actual failure.
    return result;
}
cJSON *__real_cJSON_AddNumberToObject(cJSON *object, const char *name, double value);
cJSON *__wrap_cJSON_AddNumberToObject(cJSON *object, const char *name, double value) {
    cJSON *item = __real_cJSON_AddNumberToObject(object, name, value);
    errno = ENOSPC; // The handler must not depend on errno after JSON operations.
    return item;
}
const char *__wrap_go2rtc_process_get_override_path(void) { return diagnostic_path; }
bool __wrap_go2rtc_process_try_get_pid(int *pid) { *pid = process_pid; return process_available; }
int __wrap_db_get_system_setting_alloc(const char *key, char **value, size_t *length) {
    (void)key;
    if (db_status != 0) {
        *value = NULL;
        *length = 0;
        errno = EDOM;
        return db_status;
    }
    *length = db_length;
    *value = malloc(db_length + 1);
    if (*value == NULL) {
        *length = 0;
        errno = ENOMEM;
        return -1;
    }
    if (db_value != NULL && db_length > 0) memcpy(*value, db_value, db_length);
    (*value)[db_length] = '\0';
    return 0;
}
int __wrap_httpd_authorize_global_action(const http_request_t *req, http_response_t *res,
                                         authorization_action_t action) {
    (void)req; (void)res; (void)action; return 1;
}

void setUp(void) {
    fault = NONE;
    read_calls = 0;
    db_value = NULL;
    db_length = 0;
    db_status = 1;
    process_available = true;
    process_pid = -1;
    strcpy(path, "/tmp/lightnvr-override-read-XXXXXX");
    int fd = mkstemp(path);
    TEST_ASSERT_GREATER_OR_EQUAL_INT(0, fd);
    TEST_ASSERT_EQUAL_INT(sizeof(secret) - 1, write(fd, secret, sizeof(secret) - 1));
    close(fd);
    diagnostic_path = path;
}
void tearDown(void) {
    fault = NONE;
    db_value = NULL;
    db_length = 0;
    db_status = 1;
    process_available = true;
    process_pid = -1;
    unlink(path);
    rmdir(path);
}

static void assert_failure(int expected_error) {
    char *content = NULL;
    size_t length = 0;
    int read_error = 0;
    errno = ENOSPC;
    int result = try_read_file(path, &content, &length, &read_error);
    fault = NONE;
    TEST_ASSERT_EQUAL_INT_MESSAGE(-1, result, strerror(expected_error));
    TEST_ASSERT_EQUAL_INT(expected_error, read_error);
    TEST_ASSERT_NULL(content);
    TEST_ASSERT_EQUAL_UINT(0, length);
}

void test_reader_reports_io_and_allocation_failures_after_cleanup(void) {
    const struct { enum fault_kind kind; int error; } cases[] = {
        {STAT_ERROR, EACCES}, {ALLOC_ERROR, ENOMEM}, {READ_ERROR, EIO},
        {EARLY_EOF, EIO}, {CLOSE_ERROR, EIO}, {NEGATIVE_SIZE, EIO},
    };
    for (size_t i = 0; i < sizeof(cases) / sizeof(cases[0]); i++) {
        fault = cases[i].kind;
        assert_failure(cases[i].error);
    }
}

void test_reader_retries_interrupted_short_reads_and_handles_empty_and_missing_files(void) {
    fault = INTERRUPTED_SHORT_READ;
    char *content;
    size_t length;
    int read_error;
    int result = try_read_file(path, &content, &length, &read_error);
    fault = NONE;
    TEST_ASSERT_EQUAL_INT(1, result);
    TEST_ASSERT_EQUAL_INT(0, read_error);
    TEST_ASSERT_EQUAL_STRING(secret, content);
    TEST_ASSERT_EQUAL_UINT(sizeof(secret) - 1, length);
    free(content);
    TEST_ASSERT_EQUAL_INT(0, truncate(path, 0));
    TEST_ASSERT_EQUAL_INT(1, try_read_file(path, &content, &length, &read_error));
    TEST_ASSERT_EQUAL_STRING("", content);
    TEST_ASSERT_EQUAL_INT(0, read_error);
    free(content);
    unlink(path);
    TEST_ASSERT_EQUAL_INT(0, try_read_file(path, &content, &length, &read_error));
    TEST_ASSERT_EQUAL_INT(0, read_error);
    TEST_ASSERT_NULL(content);
}

static void assert_diagnostic(const char *error, bool present, bool matches) {
    http_request_t request;
    http_response_t response;
    http_request_init(&request);
    http_response_init(&response);
    handle_get_system_go2rtc_override_status(&request, &response);
    TEST_ASSERT_EQUAL_INT(200, response.status_code);
    TEST_ASSERT_NULL(strstr(response.body, secret));
    cJSON *json = cJSON_Parse(response.body);
    TEST_ASSERT_NOT_NULL(json);
    const cJSON *message = cJSON_GetObjectItemCaseSensitive(json, "file_read_error");
    if (error) {
        TEST_ASSERT_TRUE(cJSON_IsString(message));
        TEST_ASSERT_EQUAL_STRING(error, message->valuestring);
    } else TEST_ASSERT_NULL(message);
    TEST_ASSERT_EQUAL_INT(present, cJSON_IsTrue(cJSON_GetObjectItemCaseSensitive(json, "file_present")));
    TEST_ASSERT_EQUAL_INT(matches, cJSON_IsTrue(cJSON_GetObjectItemCaseSensitive(json, "content_matches")));
    cJSON_Delete(json);
    http_response_free(&response);
}

void test_semantic_failures_and_json_allocations_cannot_report_stale_errno(void) {
    TEST_ASSERT_EQUAL_INT(0, truncate(path, EFFECTIVE_CONFIG_FILE_CAP + 1));
    assert_failure(EFBIG);
    assert_diagnostic(strerror(EFBIG), false, false);
    unlink(path);
    TEST_ASSERT_EQUAL_INT(0, mkdir(path, 0700));
    assert_failure(EINVAL);
    assert_diagnostic(strerror(EINVAL), false, false);
    rmdir(path);
    // A diagnostic request must not block waiting for a FIFO writer.
    TEST_ASSERT_EQUAL_INT(0, mkfifo(path, 0600));
    assert_failure(EINVAL);
    unlink(path);
    TEST_ASSERT_EQUAL_INT(0, symlink("/dev/null", path));
    assert_failure(ELOOP);
}

void test_diagnostics_distinguish_absent_uninitialized_and_present_without_disclosing_content(void) {
    assert_diagnostic(NULL, true, false);
    unlink(path);
    assert_diagnostic(NULL, false, true);
    diagnostic_path = NULL;
    assert_diagnostic("no override path", false, false);
}

int main(void) {
    UNITY_BEGIN();
    RUN_TEST(test_reader_reports_io_and_allocation_failures_after_cleanup);
    RUN_TEST(test_reader_retries_interrupted_short_reads_and_handles_empty_and_missing_files);
    RUN_TEST(test_semantic_failures_and_json_allocations_cannot_report_stale_errno);
    RUN_TEST(test_diagnostics_distinguish_absent_uninitialized_and_present_without_disclosing_content);
    return UNITY_END();
}
