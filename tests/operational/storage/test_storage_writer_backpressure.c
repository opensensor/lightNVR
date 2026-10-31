/* Exercise the production reader loop. Only pressure, pacing, configuration and
 * segment boundaries are deterministic seams; no server, clock sleep or media I/O. */
#define reserve_mp4_recording_path __wrap_reserve_mp4_recording_path
#define release_mp4_recording_path __wrap_release_mp4_recording_path
#include "../../../src/video/mp4_writer_thread.c"
#undef reserve_mp4_recording_path
#undef release_mp4_recording_path
#include "unity.h"

/* The segment itself is already a deterministic seam in this loop test. */
int __wrap_reserve_mp4_recording_path(char *path,size_t size,storage_placement_t *placement) {
    (void)path;(void)size;(void)placement;return 123;
}
int __wrap_release_mp4_recording_path(int descriptor,const char *path) {
    (void)descriptor;(void)path;return 0;
}

static mp4_writer_t writer;
static mp4_writer_thread_t reader;
static int pause_waits, segment_calls, cleanup_requests;
static bool stop_while_paused;

bool __wrap_storage_should_pause_recording(void) {
    return pause_waits < 2 || stop_while_paused;
}
void __wrap_trigger_storage_cleanup(bool aggressive) {
    TEST_ASSERT_TRUE(aggressive);
    cleanup_requests++;
}
int __wrap_av_usleep(unsigned usec) {
    TEST_ASSERT_EQUAL_UINT(1000000, usec);
    TEST_ASSERT_EQUAL_INT(0, segment_calls);
    pause_waits++;
    TEST_ASSERT_LESS_OR_EQUAL_INT(2, pause_waits);
    if (stop_while_paused && pause_waits == 2)
        atomic_store(&reader.shutdown_requested, 1);
    return 0;
}
int __wrap_get_stream_config_by_name(const char *name, stream_config_t *config) {
    (void)name; (void)config;
    return -1;
}
int __wrap_prepare_placed_mp4_recording_path(const config_t *config,
    const char *name, time_t timestamp, char *path, size_t size,
    storage_placement_t *placement) {
    (void)config; (void)name; (void)timestamp;
    /* Never opened: record_segment below is a boundary stub. */
    snprintf(path, size, "/nonexistent/lightnvr-t28-paused.mp4");
    memset(placement, 0, sizeof(*placement));
    return 0;
}
int __wrap_record_segment(const char *url, const char *path, int duration, int audio,
    AVFormatContext **input, segment_info_t *segment,
    record_segment_started_cb started, record_segment_activity_cb activity,
    void *opaque, atomic_int *shutdown_flag) {
    (void)url; (void)path; (void)duration; (void)audio; (void)input; (void)segment;
    (void)started; (void)activity; (void)opaque; (void)shutdown_flag;
    TEST_ASSERT_FALSE(stop_while_paused);
    TEST_ASSERT_EQUAL_INT(2, pause_waits);
    segment_calls++;
    reader.running = 0;
    return 0;
}
void setUp(void) {
    memset(&writer, 0, sizeof(writer));
    memset(&reader, 0, sizeof(reader));
    strcpy(writer.stream_name, "storage-pressure-test");
    strcpy(writer.output_path, "/nonexistent/lightnvr-t28-paused.mp4");
    strcpy(reader.rtsp_url, "rtsp://unused/storage-pressure-test");
    writer.segment_duration = 30;
    writer.last_rotation_time = time(NULL);
    writer.thread_ctx = &reader;
    reader.writer = &writer;
    pause_waits = segment_calls = cleanup_requests = 0;
    stop_while_paused = false;
}
void tearDown(void) {
    avformat_close_input(&reader.input_ctx);
    av_packet_free(&reader.segment_info.pending_video_keyframe);
}
void test_no_segment_opens_during_pressure_then_recording_resumes(void) {
    mp4_writer_rtsp_thread(&reader);
    TEST_ASSERT_EQUAL_INT(2, pause_waits);
    TEST_ASSERT_EQUAL_INT(1, cleanup_requests);
    TEST_ASSERT_EQUAL_INT(1, segment_calls);
}
void test_shutdown_during_pressure_never_opens_segment(void) {
    stop_while_paused = true;
    mp4_writer_rtsp_thread(&reader);
    TEST_ASSERT_EQUAL_INT(2, pause_waits);
    TEST_ASSERT_EQUAL_INT(1, cleanup_requests);
    TEST_ASSERT_EQUAL_INT(0, segment_calls);
}
int main(void) {
    UNITY_BEGIN();
    RUN_TEST(test_no_segment_opens_during_pressure_then_recording_resumes);
    RUN_TEST(test_shutdown_during_pressure_never_opens_segment);
    return UNITY_END();
}
