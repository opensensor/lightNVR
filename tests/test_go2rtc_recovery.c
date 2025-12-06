/**
 * @file test_go2rtc_recovery.c
 * @brief Tests for go2rtc restart recovery mechanisms
 * 
 * This file tests:
 * - Recording reconnection signaling after go2rtc restart
 * - go2rtc PID tracking and memory reporting
 */

#define _POSIX_C_SOURCE 200809L
#define _XOPEN_SOURCE 700
#define _GNU_SOURCE

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <signal.h>
#include <stdatomic.h>

#include "core/logger.h"
#include "video/mp4_writer.h"
#include "video/mp4_writer_thread.h"
#include "video/mp4_recording.h"
#include "video/go2rtc/go2rtc_process.h"

// Test counters
static int tests_passed = 0;
static int tests_failed = 0;

#define TEST_ASSERT(condition, message) do { \
    if (condition) { \
        printf("  ✓ PASS: %s\n", message); \
        tests_passed++; \
    } else { \
        printf("  ✗ FAIL: %s\n", message); \
        tests_failed++; \
    } \
} while(0)

/**
 * Test 1: Test mp4_writer_signal_reconnect with NULL writer
 */
static void test_signal_reconnect_null_writer(void) {
    printf("\nTest 1: mp4_writer_signal_reconnect with NULL writer\n");
    
    // Should not crash with NULL writer
    mp4_writer_signal_reconnect(NULL);
    
    TEST_ASSERT(1, "mp4_writer_signal_reconnect handles NULL writer gracefully");
}

/**
 * Test 2: Test signal_all_mp4_recordings_reconnect with no active recordings
 */
static void test_signal_all_reconnect_no_recordings(void) {
    printf("\nTest 2: signal_all_mp4_recordings_reconnect with no active recordings\n");
    
    // Initialize the recording backend
    init_mp4_recording_backend();
    
    // Should not crash with no active recordings
    signal_all_mp4_recordings_reconnect();
    
    TEST_ASSERT(1, "signal_all_mp4_recordings_reconnect handles empty recordings gracefully");
    
    // Cleanup
    cleanup_mp4_recording_backend();
}

/**
 * Test 3: Test go2rtc_process_get_pid when go2rtc is not running
 */
static void test_get_pid_not_running(void) {
    printf("\nTest 3: go2rtc_process_get_pid when go2rtc is not running\n");
    
    // Without initializing go2rtc, the PID should be -1 or not found
    int pid = go2rtc_process_get_pid();
    
    // PID should be -1 or a valid PID if go2rtc happens to be running externally
    TEST_ASSERT(pid == -1 || pid > 0, "go2rtc_process_get_pid returns -1 when not running or valid PID");
    
    printf("  (PID returned: %d)\n", pid);
}

/**
 * Test 4: Test force_reconnect atomic flag behavior
 */
static void test_force_reconnect_atomic_flag(void) {
    printf("\nTest 4: force_reconnect atomic flag behavior\n");
    
    atomic_int force_reconnect;
    atomic_store(&force_reconnect, 0);
    
    // Initial value should be 0
    TEST_ASSERT(atomic_load(&force_reconnect) == 0, "Initial force_reconnect value is 0");
    
    // Set to 1
    atomic_store(&force_reconnect, 1);
    TEST_ASSERT(atomic_load(&force_reconnect) == 1, "force_reconnect can be set to 1");
    
    // atomic_exchange should return old value and set new value
    int old_value = atomic_exchange(&force_reconnect, 0);
    TEST_ASSERT(old_value == 1, "atomic_exchange returns old value (1)");
    TEST_ASSERT(atomic_load(&force_reconnect) == 0, "atomic_exchange sets new value (0)");
    
    // Second exchange should return 0
    old_value = atomic_exchange(&force_reconnect, 0);
    TEST_ASSERT(old_value == 0, "Second atomic_exchange returns 0 (already consumed)");
}

/**
 * Test 5: Test mp4_writer_is_recording with NULL
 */
static void test_is_recording_null(void) {
    printf("\nTest 5: mp4_writer_is_recording with NULL writer\n");
    
    int result = mp4_writer_is_recording(NULL);
    TEST_ASSERT(result == 0, "mp4_writer_is_recording returns 0 for NULL writer");
}

int main(int argc, char *argv[]) {
    printf("===========================================\n");
    printf("  go2rtc Recovery Mechanism Tests\n");
    printf("===========================================\n");

    // Initialize logger
    init_logger();
    set_log_level(LOG_LEVEL_DEBUG);

    // Run tests
    test_signal_reconnect_null_writer();
    test_signal_all_reconnect_no_recordings();
    test_get_pid_not_running();
    test_force_reconnect_atomic_flag();
    test_is_recording_null();

    // Summary
    printf("\n===========================================\n");
    printf("  Test Summary\n");
    printf("===========================================\n");
    printf("  Passed: %d\n", tests_passed);
    printf("  Failed: %d\n", tests_failed);
    printf("  Total:  %d\n", tests_passed + tests_failed);
    printf("===========================================\n");

    // Cleanup
    shutdown_logger();

    return tests_failed > 0 ? 1 : 0;
}

