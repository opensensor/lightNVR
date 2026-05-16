#include "unity.h"

#include <string.h>
#include <time.h>

#include "core/logger.h"
#include "video/mp4_writer.h"
#include "video/mp4_writer_thread.h"

static mp4_writer_t writer;
static mp4_writer_thread_t thread_ctx;

void setUp(void) {
    memset(&writer, 0, sizeof(writer));
    memset(&thread_ctx, 0, sizeof(thread_ctx));

    strcpy(writer.stream_name, "watchdog-test");
    writer.creation_time = time(NULL);
    writer.thread_ctx = &thread_ctx;
    thread_ctx.running = 1;
}

void tearDown(void) {
}

static void test_null_writer_is_not_recording(void) {
    TEST_ASSERT_EQUAL_INT(0, mp4_writer_is_recording(NULL));
}

static void test_recent_activity_is_recording(void) {
    writer.last_packet_time = time(NULL);

    TEST_ASSERT_EQUAL_INT(1, mp4_writer_is_recording(&writer));
}

static void test_default_timeout_marks_stale_recording_dead(void) {
    writer.segment_duration = 30;
    writer.last_packet_time = time(NULL) - 46;

    TEST_ASSERT_EQUAL_INT(0, mp4_writer_is_recording(&writer));
}

static void test_long_segment_extends_watchdog_timeout(void) {
    writer.segment_duration = 60;
    writer.last_packet_time = time(NULL) - 50;

    TEST_ASSERT_EQUAL_INT(1, mp4_writer_is_recording(&writer));
}

static void test_long_segment_still_marks_truly_stale_recording_dead(void) {
    writer.segment_duration = 60;
    writer.last_packet_time = time(NULL) - 76;

    TEST_ASSERT_EQUAL_INT(0, mp4_writer_is_recording(&writer));
}

int main(void) {
    init_logger();

    UNITY_BEGIN();
    RUN_TEST(test_null_writer_is_not_recording);
    RUN_TEST(test_recent_activity_is_recording);
    RUN_TEST(test_default_timeout_marks_stale_recording_dead);
    RUN_TEST(test_long_segment_extends_watchdog_timeout);
    RUN_TEST(test_long_segment_still_marks_truly_stale_recording_dead);
    int result = UNITY_END();

    shutdown_logger();

    return result;
}
