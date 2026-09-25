/**
 * @file test_ffmpeg_rgb24_padding.c
 * @brief Regression test for issue #587: libswscale writes past a packed
 *        RGB24 buffer sized exactly width*height*3.
 *
 * libswscale's x86 SSSE3 yuv420p->rgb24 converter advances 16 pixels per
 * iteration but its wrapper only rounds the width up to 8, so for widths with
 * (w % 16) >= 8 the last row is written up to 24 bytes past the end. The
 * rgb24->yuv420p readers used by the MJPEG encoder also read up to 64 bytes
 * past the last row. A tight malloc'd buffer that sits below the arena's top
 * chunk then trips glibc's "malloc(): corrupted top size".
 *
 * These tests place a buffer carrying exactly RGB24_SWS_TAIL_PADDING bytes of
 * slack flush against a PROT_NONE guard page. Any access beyond the padding
 * faults, which is visible even though libswscale is not sanitizer
 * instrumented. They therefore prove the padding is sufficient on this
 * platform for both directions; they do not depend on the overrun occurring.
 */

#define _POSIX_C_SOURCE 200809L
#define _GNU_SOURCE

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <sys/mman.h>

#include <libavutil/frame.h>
#include <libavutil/pixdesc.h>
#include <libswscale/swscale.h>

#include "unity.h"
#include "core/logger.h"
#include "video/ffmpeg_utils.h"

typedef struct {
    int width;
    int height;
    enum AVPixelFormat format;
} geometry_t;

/* Widths chosen so (w % 16) covers 8 (1080, 1000, 856), 15 (1919), 0 (1920)
 * and an odd width (641). Portrait 1080x1920 is the reporter's doorbell case. */
static const geometry_t geometries[] = {
    {1080, 1920, AV_PIX_FMT_YUV420P},
    {1080, 1920, AV_PIX_FMT_YUVJ420P},
    {1000, 1000, AV_PIX_FMT_YUV420P},
    {856,  480,  AV_PIX_FMT_YUV420P},
    {1919, 1080, AV_PIX_FMT_YUV420P},
    {1920, 1080, AV_PIX_FMT_YUV420P},
    {641,  480,  AV_PIX_FMT_YUV420P},
    {1280, 720,  AV_PIX_FMT_YUV422P},
};

void setUp(void) {}
void tearDown(void) {}

/* Allocate n bytes so the block ends exactly where a PROT_NONE page begins. */
static uint8_t *guarded_alloc(size_t n, void **map, size_t *maplen) {
    size_t pg = (size_t)sysconf(_SC_PAGESIZE);
    size_t body = (n + pg - 1) / pg * pg;
    *maplen = body + pg;
    *map = mmap(NULL, *maplen, PROT_READ | PROT_WRITE, MAP_PRIVATE | MAP_ANONYMOUS, -1, 0);
    if (*map == MAP_FAILED) return NULL;
    if (mprotect((uint8_t *)*map + body, pg, PROT_NONE) != 0) {
        munmap(*map, *maplen);
        return NULL;
    }
    return (uint8_t *)*map + (body - n);
}

static AVFrame *make_frame(const geometry_t *g) {
    AVFrame *frame = av_frame_alloc();
    if (!frame) return NULL;
    frame->format = g->format;
    frame->width = g->width;
    frame->height = g->height;
    if (av_frame_get_buffer(frame, 0) < 0) {
        av_frame_free(&frame);
        return NULL;
    }
    /* Deterministic content so the JPEG encode below has something to do. */
    for (int plane = 0; plane < 3 && frame->data[plane]; plane++) {
        int rows = plane == 0 ? frame->height : (frame->height + 1) / 2;
        for (int y = 0; y < rows; y++) {
            memset(frame->data[plane] + (size_t)y * frame->linesize[plane],
                   (plane * 64 + y) & 0xFF, (size_t)frame->linesize[plane]);
        }
    }
    return frame;
}

/* Both conversions the detection fallback performs, on a buffer with exactly
 * RGB24_SWS_TAIL_PADDING bytes of slack before a guard page. */
void test_padding_covers_rgb24_write_and_jpeg_read_overruns(void) {
    for (size_t i = 0; i < sizeof(geometries) / sizeof(geometries[0]); i++) {
        const geometry_t *g = &geometries[i];
        size_t rgb_size = (size_t)g->width * g->height * 3;
        void *map = NULL;
        size_t maplen = 0;
        uint8_t *rgb = guarded_alloc(rgb_size + RGB24_SWS_TAIL_PADDING, &map, &maplen);
        TEST_ASSERT_NOT_NULL_MESSAGE(rgb, "guarded mmap failed");
        memset(rgb, 0, rgb_size + RGB24_SWS_TAIL_PADDING);

        AVFrame *frame = make_frame(g);
        TEST_ASSERT_NOT_NULL_MESSAGE(frame, "av_frame_get_buffer failed");

        /* Write side: same call the helper makes (packed stride, exact dims). */
        struct SwsContext *sws = sws_getContext(g->width, g->height, g->format,
                                                g->width, g->height, AV_PIX_FMT_RGB24,
                                                SWS_BILINEAR, NULL, NULL, NULL);
        TEST_ASSERT_NOT_NULL(sws);
        uint8_t *dst[4] = {rgb, NULL, NULL, NULL};
        int dst_linesize[4] = {g->width * 3, 0, 0, 0};
        int rows = sws_scale(sws, (const uint8_t * const *)frame->data, frame->linesize,
                             0, g->height, dst, dst_linesize);
        sws_freeContext(sws);
        TEST_ASSERT_EQUAL_INT(g->height, rows);

        /* Read side: the MJPEG encoder feeds the same packed buffer through
         * libswscale's rgb24->yuvj420p readers (ffmpeg_utils.c). Exercise that
         * conversion directly so the check does not depend on libavcodec. */
        AVFrame *yuv = av_frame_alloc();
        TEST_ASSERT_NOT_NULL(yuv);
        yuv->format = AV_PIX_FMT_YUVJ420P;
        yuv->width = g->width;
        yuv->height = g->height;
        TEST_ASSERT_EQUAL_INT(0, av_frame_get_buffer(yuv, 0));
        struct SwsContext *to_yuv = sws_getContext(g->width, g->height, AV_PIX_FMT_RGB24,
                                                   g->width, g->height, AV_PIX_FMT_YUVJ420P,
                                                   SWS_BILINEAR, NULL, NULL, NULL);
        TEST_ASSERT_NOT_NULL(to_yuv);
        const uint8_t *src[4] = {rgb, NULL, NULL, NULL};
        int src_linesize[4] = {g->width * 3, 0, 0, 0};
        rows = sws_scale(to_yuv, src, src_linesize, 0, g->height, yuv->data, yuv->linesize);
        sws_freeContext(to_yuv);
        TEST_ASSERT_EQUAL_INT(g->height, rows);
        av_frame_free(&yuv);

        av_frame_free(&frame);
        munmap(map, maplen);
    }
}

void test_helper_returns_payload_size_and_zeroed_slack(void) {
    geometry_t g = {1080, 1920, AV_PIX_FMT_YUV420P};
    AVFrame *frame = make_frame(&g);
    TEST_ASSERT_NOT_NULL(frame);

    size_t rgb_size = 0;
    uint8_t *rgb = ffmpeg_frame_to_rgb24_padded(frame, &rgb_size);
    TEST_ASSERT_NOT_NULL(rgb);
    TEST_ASSERT_EQUAL_size_t((size_t)1080 * 1920 * 3, rgb_size);

    /* The SIMD path may legitimately spill up to 24 bytes into the slack for
     * this width; everything beyond that must still be the helper's zeroes. */
    for (size_t i = 64; i < RGB24_SWS_TAIL_PADDING; i++) {
        TEST_ASSERT_EQUAL_UINT8(0, rgb[rgb_size + i]);
    }

    free(rgb);
    av_frame_free(&frame);
}

void test_helper_rejects_missing_or_empty_frames(void) {
    size_t rgb_size = 123;
    TEST_ASSERT_NULL(ffmpeg_frame_to_rgb24_padded(NULL, &rgb_size));
    TEST_ASSERT_EQUAL_size_t(0, rgb_size);

    AVFrame *frame = av_frame_alloc();
    TEST_ASSERT_NOT_NULL(frame);
    frame->format = AV_PIX_FMT_YUV420P;
    frame->width = 0;
    frame->height = 0;
    TEST_ASSERT_NULL(ffmpeg_frame_to_rgb24_padded(frame, NULL));
    av_frame_free(&frame);
}

int main(void) {
    init_logger();
    UNITY_BEGIN();
    RUN_TEST(test_padding_covers_rgb24_write_and_jpeg_read_overruns);
    RUN_TEST(test_helper_returns_payload_size_and_zeroed_slack);
    RUN_TEST(test_helper_rejects_missing_or_empty_frames);
    return UNITY_END();
}
