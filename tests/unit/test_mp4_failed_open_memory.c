#define _GNU_SOURCE

#include <malloc.h>
#include <string.h>

#include "unity.h"
#include "core/logger.h"
#include "video/mp4_segment_recorder.h"

void setUp(void) {}
void tearDown(void) {}

void test_unreachable_rtsp_retries_release_input_allocations(void) {
    AVFormatContext *input = NULL;
    segment_info_t segment = {0};
    size_t after_warmup = 0;

    /* Connection refused immediately, exercising the same open-failure path
     * as an unreachable camera without waiting for a network timeout. */
    for (int i = 0; i < 150; i++) {
        int rc = record_segment("rtsp://127.0.0.1:1/missing",
                                "/tmp/lightnvr-unreachable-test.mp4", 30, 0,
                                &input, &segment, NULL, NULL, NULL, NULL);
        TEST_ASSERT_LESS_THAN_INT(0, rc);
        TEST_ASSERT_NULL(input);
        if (i == 49) after_warmup = mallinfo2().uordblks;
    }

    size_t final_allocated = mallinfo2().uordblks;
    size_t growth = final_allocated > after_warmup
                  ? final_allocated - after_warmup : 0;
    TEST_ASSERT_LESS_THAN_UINT64(1024U * 1024U, (uint64_t)growth);
}

int main(void) {
    init_logger();
    UNITY_BEGIN();
    RUN_TEST(test_unreachable_rtsp_retries_release_input_allocations);
    return UNITY_END();
}
