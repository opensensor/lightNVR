/* Exercise the actual reader loop: a failed segment must keep each recording row
 * paired with its own file. Swapping the output path while the row was still open
 * left the retried file with no row and gave the old row the wrong size. */
#include "../../src/video/mp4_writer_thread.c"
#include "unity.h"

#define MAX_ROWS 4

typedef struct {
    char path[MAX_PATH_LENGTH];
    uint64_t size;
    bool complete;
} row_t;

static mp4_writer_t writer;
static mp4_writer_thread_t reader;
static char test_dir[64];
static row_t rows[MAX_ROWS];
static int row_count, segment_calls, paths_prepared, placement_failures, placement_waits;

static void write_file(const char *path, size_t size) {
    FILE *file = fopen(path, "wb");
    TEST_ASSERT_NOT_NULL(file);
    for (size_t i = 0; i < size; i++) fputc('x', file);
    fclose(file);
}

uint64_t __wrap_add_recording_metadata(const recording_metadata_t *metadata) {
    TEST_ASSERT_LESS_THAN_INT(MAX_ROWS, row_count);
    safe_strcpy(rows[row_count].path, metadata->file_path, MAX_PATH_LENGTH, 0);
    return (uint64_t)++row_count;
}

int __wrap_update_recording_metadata(uint64_t id, time_t end_time, uint64_t size, bool complete) {
    (void)end_time;
    TEST_ASSERT_TRUE(id >= 1 && id <= (uint64_t)row_count);
    rows[id - 1].size = size;
    rows[id - 1].complete = complete;
    return 0;
}

int __wrap_update_stream_storage_cache_add_recording(const char *name, uint64_t size) {
    (void)name; (void)size;
    return 0;
}

int __wrap_av_usleep(unsigned usec) {
    if (usec == 1000000) placement_waits++;
    return 0;
}

int __wrap_get_stream_config_by_name(const char *name, stream_config_t *config) {
    (void)name; (void)config;
    return -1;
}

bool __wrap_storage_should_pause_recording(void) { return false; }

int __wrap_prepare_placed_mp4_recording_path(const config_t *config, const char *name,
    time_t timestamp, char *path, size_t size, storage_placement_t *placement) {
    (void)config; (void)name; (void)timestamp;
    memset(placement, 0, sizeof(*placement));
    if (placement_failures > 0) {
        placement_failures--;
        return -1;
    }
    snprintf(path, size, "%s/segment_%d.mp4", test_dir, ++paths_prepared);
    return 0;
}

int __wrap_record_segment(const char *url, const char *path, int duration, int audio,
    AVFormatContext **input, segment_info_t *segment,
    record_segment_started_cb started, record_segment_activity_cb activity,
    void *opaque, atomic_int *shutdown_flag) {
    (void)url; (void)duration; (void)audio; (void)input; (void)segment;
    (void)activity; (void)shutdown_flag;
    segment_calls++;
    TEST_ASSERT_LESS_OR_EQUAL_INT(2, segment_calls);
    started(opaque);
    if (segment_calls == 1) {
        write_file(path, 10);
        return AVERROR(ENOSPC);
    }
    write_file(path, 20);
    reader.running = 0;
    return 0;
}

void setUp(void) {
    g_config.max_streams = MAX_STREAMS;
    strcpy(test_dir, "/tmp/lightnvr_retry_pairing_XXXXXX");
    TEST_ASSERT_NOT_NULL(mkdtemp(test_dir));
    memset(&writer, 0, sizeof(writer));
    memset(&reader, 0, sizeof(reader));
    memset(rows, 0, sizeof(rows));
    row_count = segment_calls = paths_prepared = placement_failures = placement_waits = 0;
    strcpy(writer.stream_name, "retry-pairing");
    snprintf(writer.output_path, sizeof(writer.output_path), "%s/segment_0.mp4", test_dir);
    writer.segment_duration = 30;
    writer.last_rotation_time = time(NULL);
    writer.thread_ctx = &reader;
    reader.writer = &writer;
    reader.running = 1;
    strcpy(reader.rtsp_url, "rtsp://unused/test");
}

void tearDown(void) {
    char path[MAX_PATH_LENGTH];
    for (int i = 0; i <= paths_prepared; i++) {
        snprintf(path, sizeof(path), "%s/segment_%d.mp4", test_dir, i);
        unlink(path);
    }
    rmdir(test_dir);
}

static void assert_each_file_kept_its_own_row(void) {
    TEST_ASSERT_EQUAL_INT(2, segment_calls);
    TEST_ASSERT_EQUAL_INT(2, row_count);
    TEST_ASSERT_TRUE(strcmp(rows[0].path, rows[1].path) != 0);

    TEST_ASSERT_TRUE(rows[0].complete);
    TEST_ASSERT_EQUAL_UINT64(10, rows[0].size);
    TEST_ASSERT_EQUAL_UINT64(20, rows[1].size);
}

static void test_failed_segment_finalizes_its_row_before_the_retry_file(void) {
    mp4_writer_rtsp_thread(&reader);
    assert_each_file_kept_its_own_row();
}

static void test_blocked_placement_waits_and_keeps_the_rows_paired(void) {
    placement_failures = 2;
    mp4_writer_rtsp_thread(&reader);
    assert_each_file_kept_its_own_row();
    TEST_ASSERT_EQUAL_INT(2, placement_waits);
}

int main(void) {
    UNITY_BEGIN();
    RUN_TEST(test_failed_segment_finalizes_its_row_before_the_retry_file);
    RUN_TEST(test_blocked_placement_waits_and_keeps_the_rows_paired);
    return UNITY_END();
}
