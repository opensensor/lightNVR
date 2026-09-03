#include "unity.h"

#include "telemetry/collectors/linux_restart.h"

#include <stdio.h>
#include <string.h>

#ifndef TEST_CLOCK_FIXTURE_DIR
#define TEST_CLOCK_FIXTURE_DIR "tests/fixtures/system_health/clock"
#endif

void setUp(void) {}
void tearDown(void) {}

static linux_restart_evidence_t evidence(const char *boot, const char *run) {
    linux_restart_evidence_t value;
    memset(&value, 0, sizeof(value));
    value.capability = SYSTEM_HEALTH_CAPABILITY_AVAILABLE;
    strncpy(value.boot_id, boot, sizeof(value.boot_id) - 1);
    strncpy(value.run_id, run, sizeof(value.run_id) - 1);
    return value;
}

static void test_ids_are_bounded_and_validated(void) {
    TEST_ASSERT_TRUE(linux_restart_id_valid("11111111-1111-4111-8111-111111111111"));
    TEST_ASSERT_FALSE(linux_restart_id_valid("not-a-boot-id"));
    TEST_ASSERT_FALSE(linux_restart_id_valid("11111111-1111-4111-8111-111111111111x"));
}

static void test_restart_kinds_distinguish_boot_and_process(void) {
    linux_restart_evidence_t old = evidence(
        "11111111-1111-4111-8111-111111111111",
        "aaaaaaaa-aaaa-4aaa-8aaa-aaaaaaaaaaaa");
    linux_restart_evidence_t same = old;
    TEST_ASSERT_EQUAL(LINUX_RESTART_SAME_RUN,
                      linux_restart_classify(&old, &same));
    linux_restart_evidence_t process = evidence(
        old.boot_id, "bbbbbbbb-bbbb-4bbb-8bbb-bbbbbbbbbbbb");
    TEST_ASSERT_EQUAL(LINUX_RESTART_PROCESS_RESTART,
                      linux_restart_classify(&old, &process));
    linux_restart_evidence_t reboot = evidence(
        "22222222-2222-4222-8222-222222222222", process.run_id);
    TEST_ASSERT_EQUAL(LINUX_RESTART_HOST_REBOOT,
                      linux_restart_classify(&old, &reboot));
}

static void test_restart_evidence_reads_injected_proc_root(void) {
    char root[512];
    snprintf(root, sizeof(root), "%s/normal", TEST_CLOCK_FIXTURE_DIR);
    linux_restart_evidence_t value;
    TEST_ASSERT_EQUAL_INT(0, linux_restart_read_evidence(
        root, "aaaaaaaa-aaaa-4aaa-8aaa-aaaaaaaaaaaa", 1234, &value));
    TEST_ASSERT_EQUAL_STRING("11111111-1111-4111-8111-111111111111",
                             value.boot_id);
    TEST_ASSERT_EQUAL_STRING("aaaaaaaa-aaaa-4aaa-8aaa-aaaaaaaaaaaa",
                             value.run_id);
    TEST_ASSERT_FLOAT_WITHIN(0.001f, 4321.25f,
                             (float)value.host_uptime_seconds);
    TEST_ASSERT_EQUAL_UINT64(1234, value.process_start_monotonic_ms);
}

static void test_restart_evidence_rejects_nonfinite_uptime(void) {
    char root[512];
    snprintf(root, sizeof(root), "%s/malformed", TEST_CLOCK_FIXTURE_DIR);
    linux_restart_evidence_t value;
    TEST_ASSERT_EQUAL_INT(-1, linux_restart_read_evidence(
        root, "aaaaaaaa-aaaa-4aaa-8aaa-aaaaaaaaaaaa", 1234, &value));
    TEST_ASSERT_EQUAL(SYSTEM_HEALTH_CAPABILITY_ERROR, value.capability);
}

int main(void) {
    UNITY_BEGIN();
    RUN_TEST(test_ids_are_bounded_and_validated);
    RUN_TEST(test_restart_kinds_distinguish_boot_and_process);
    RUN_TEST(test_restart_evidence_reads_injected_proc_root);
    RUN_TEST(test_restart_evidence_rejects_nonfinite_uptime);
    return UNITY_END();
}
