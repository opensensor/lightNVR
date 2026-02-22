/**
 * @file test_packet_buffer.c
 * @brief Layer 3 Unity tests for video/packet_buffer.c
 *
 * Tests the packet buffer pool lifecycle, FIFO ordering, statistics,
 * flush callback, and clear operation.  Uses real AVPackets allocated
 * via av_new_packet() so the FFmpeg refcounting path is exercised.
 */

#define _POSIX_C_SOURCE 200809L
#define _GNU_SOURCE

#include <stdio.h>
#include <string.h>
#include <time.h>
#include <unistd.h>

#include <libavcodec/avcodec.h>
#include <libavformat/avformat.h>

#include "unity.h"
#include "video/packet_buffer.h"

/* ---- helpers ---- */

static AVPacket *make_pkt(int size_bytes, bool keyframe) {
    AVPacket *pkt = av_packet_alloc();
    if (!pkt) return NULL;
    if (av_new_packet(pkt, size_bytes) < 0) {
        av_packet_free(&pkt);
        return NULL;
    }
    memset(pkt->data, 0xAB, size_bytes);
    if (keyframe)
        pkt->flags |= AV_PKT_FLAG_KEY;
    pkt->pts = 0;
    pkt->dts = 0;
    pkt->stream_index = 0;
    return pkt;
}

/* flush callback: counts delivered packets */
static int count_cb(const AVPacket *pkt, void *user_data) {
    (void)pkt;
    int *n = (int *)user_data;
    (*n)++;
    return 0;
}

/* ---- Unity boilerplate ---- */
void setUp(void) {
    init_packet_buffer_pool(64); /* 64 MB limit */
}

void tearDown(void) {
    cleanup_packet_buffer_pool();
}

/* ================================================================
 * init / cleanup
 * ================================================================ */

void test_init_cleanup_cycle(void) {
    cleanup_packet_buffer_pool();
    int rc = init_packet_buffer_pool(32);
    TEST_ASSERT_EQUAL_INT(0, rc);
    cleanup_packet_buffer_pool();
    /* Re-init so tearDown works */
    init_packet_buffer_pool(64);
}

/* ================================================================
 * create / destroy
 * ================================================================ */

void test_create_buffer_returns_non_null(void) {
    packet_buffer_t *b = create_packet_buffer("cam1", 5, BUFFER_MODE_MEMORY);
    TEST_ASSERT_NOT_NULL(b);
    destroy_packet_buffer(b);
}

void test_create_buffer_invalid_seconds_returns_null(void) {
    /* MIN_BUFFER_SECONDS = 5 so 4 is too small */
    packet_buffer_t *b = create_packet_buffer("cam2", 4, BUFFER_MODE_MEMORY);
    TEST_ASSERT_NULL(b);
}

void test_create_buffer_null_name_returns_null(void) {
    packet_buffer_t *b = create_packet_buffer(NULL, 5, BUFFER_MODE_MEMORY);
    TEST_ASSERT_NULL(b);
}

void test_destroy_null_no_crash(void) {
    destroy_packet_buffer(NULL);
    TEST_PASS();
}

/* ================================================================
 * get_packet_buffer
 * ================================================================ */

void test_get_packet_buffer_finds_created(void) {
    packet_buffer_t *b = create_packet_buffer("lookup_cam", 5, BUFFER_MODE_MEMORY);
    TEST_ASSERT_NOT_NULL(b);

    packet_buffer_t *found = get_packet_buffer("lookup_cam");
    TEST_ASSERT_EQUAL_PTR(b, found);
    destroy_packet_buffer(b);
}

void test_get_packet_buffer_missing_returns_null(void) {
    packet_buffer_t *found = get_packet_buffer("ghost_cam");
    TEST_ASSERT_NULL(found);
}

/* ================================================================
 * add / pop / peek
 * ================================================================ */

void test_add_and_pop_single_packet(void) {
    packet_buffer_t *b = create_packet_buffer("fifo_cam", 5, BUFFER_MODE_MEMORY);
    TEST_ASSERT_NOT_NULL(b);

    AVPacket *pkt = make_pkt(128, false);
    TEST_ASSERT_NOT_NULL(pkt);
    int rc = packet_buffer_add_packet(b, pkt, time(NULL));
    TEST_ASSERT_EQUAL_INT(0, rc);
    av_packet_free(&pkt);

    AVPacket *out = NULL;
    rc = packet_buffer_pop_oldest(b, &out);
    TEST_ASSERT_EQUAL_INT(0, rc);
    TEST_ASSERT_NOT_NULL(out);
    TEST_ASSERT_EQUAL_INT(128, out->size);
    av_packet_free(&out);

    /* Buffer should now be empty */
    AVPacket *empty = NULL;
    rc = packet_buffer_pop_oldest(b, &empty);
    TEST_ASSERT_EQUAL_INT(-1, rc);

    destroy_packet_buffer(b);
}

void test_peek_does_not_remove_packet(void) {
    packet_buffer_t *b = create_packet_buffer("peek_cam", 5, BUFFER_MODE_MEMORY);
    TEST_ASSERT_NOT_NULL(b);

    AVPacket *pkt = make_pkt(64, false);
    packet_buffer_add_packet(b, pkt, time(NULL));
    av_packet_free(&pkt);

    AVPacket *peek1 = NULL, *peek2 = NULL;
    TEST_ASSERT_EQUAL_INT(0, packet_buffer_peek_oldest(b, &peek1));
    TEST_ASSERT_EQUAL_INT(0, packet_buffer_peek_oldest(b, &peek2));
    TEST_ASSERT_EQUAL_INT(64, peek1->size);
    TEST_ASSERT_EQUAL_INT(64, peek2->size);
    av_packet_free(&peek1);
    av_packet_free(&peek2);

    destroy_packet_buffer(b);
}

void test_pop_empty_buffer_returns_error(void) {
    packet_buffer_t *b = create_packet_buffer("empty_cam", 5, BUFFER_MODE_MEMORY);
    AVPacket *out = NULL;
    int rc = packet_buffer_pop_oldest(b, &out);
    TEST_ASSERT_EQUAL_INT(-1, rc);
    destroy_packet_buffer(b);
}

void test_add_multiple_packets_fifo_order(void) {
    packet_buffer_t *b = create_packet_buffer("order_cam", 5, BUFFER_MODE_MEMORY);
    TEST_ASSERT_NOT_NULL(b);

    /* Push 3 packets with distinct sizes */
    for (int sz = 10; sz <= 30; sz += 10) {
        AVPacket *p = make_pkt(sz, false);
        packet_buffer_add_packet(b, p, time(NULL));
        av_packet_free(&p);
    }

    /* Pop should return in FIFO (oldest = smallest size first) */
    for (int expected = 10; expected <= 30; expected += 10) {
        AVPacket *out = NULL;
        TEST_ASSERT_EQUAL_INT(0, packet_buffer_pop_oldest(b, &out));
        TEST_ASSERT_EQUAL_INT(expected, out->size);
        av_packet_free(&out);
    }

    destroy_packet_buffer(b);
}

/* ================================================================
 * get_stats
 * ================================================================ */

void test_get_stats_after_add(void) {
    packet_buffer_t *b = create_packet_buffer("stats_cam", 5, BUFFER_MODE_MEMORY);
    TEST_ASSERT_NOT_NULL(b);

    AVPacket *p1 = make_pkt(256, false);
    AVPacket *p2 = make_pkt(256, true);
    packet_buffer_add_packet(b, p1, time(NULL));
    packet_buffer_add_packet(b, p2, time(NULL));
    av_packet_free(&p1);
    av_packet_free(&p2);

    int count = 0;
    size_t mem = 0;
    int dur = 0;
    int rc = packet_buffer_get_stats(b, &count, &mem, &dur);
    TEST_ASSERT_EQUAL_INT(0, rc);
    TEST_ASSERT_EQUAL_INT(2, count);
    TEST_ASSERT_GREATER_THAN(0, (int)mem);

    destroy_packet_buffer(b);
}

/* ================================================================
 * flush
 * ================================================================ */

void test_flush_calls_callback_for_each_packet(void) {
    packet_buffer_t *b = create_packet_buffer("flush_cam", 5, BUFFER_MODE_MEMORY);
    TEST_ASSERT_NOT_NULL(b);

    for (int i = 0; i < 5; i++) {
        AVPacket *p = make_pkt(32, i == 0);
        packet_buffer_add_packet(b, p, time(NULL));
        av_packet_free(&p);
    }

    int called = 0;
    int n = packet_buffer_flush(b, count_cb, &called);
    TEST_ASSERT_EQUAL_INT(5, n);
    TEST_ASSERT_EQUAL_INT(5, called);

    destroy_packet_buffer(b);
}

void test_flush_null_callback_returns_error(void) {
    packet_buffer_t *b = create_packet_buffer("flush_null", 5, BUFFER_MODE_MEMORY);
    int rc = packet_buffer_flush(b, NULL, NULL);
    TEST_ASSERT_EQUAL_INT(-1, rc);
    destroy_packet_buffer(b);
}

/* ================================================================
 * clear
 * ================================================================ */

void test_clear_empties_buffer(void) {
    packet_buffer_t *b = create_packet_buffer("clear_cam", 5, BUFFER_MODE_MEMORY);
    TEST_ASSERT_NOT_NULL(b);

    AVPacket *p = make_pkt(64, false);
    packet_buffer_add_packet(b, p, time(NULL));
    av_packet_free(&p);

    packet_buffer_clear(b);

    int count = 0;
    size_t mem = 0;
    int dur = 0;
    packet_buffer_get_stats(b, &count, &mem, &dur);
    TEST_ASSERT_EQUAL_INT(0, count);

    destroy_packet_buffer(b);
}

/* ================================================================
 * estimate_packet_count
 * ================================================================ */

void test_estimate_packet_count_positive(void) {
    int n = packet_buffer_estimate_packet_count(30, 10);
    TEST_ASSERT_GREATER_THAN(0, n);
    /* 30 fps * 10 s * 1.2 overhead = 360 */
    TEST_ASSERT_EQUAL_INT(360, n);
}

/* ================================================================
 * main
 * ================================================================ */

int main(void) {
    UNITY_BEGIN();
    RUN_TEST(test_init_cleanup_cycle);
    RUN_TEST(test_create_buffer_returns_non_null);
    RUN_TEST(test_create_buffer_invalid_seconds_returns_null);
    RUN_TEST(test_create_buffer_null_name_returns_null);
    RUN_TEST(test_destroy_null_no_crash);
    RUN_TEST(test_get_packet_buffer_finds_created);
    RUN_TEST(test_get_packet_buffer_missing_returns_null);
    RUN_TEST(test_add_and_pop_single_packet);
    RUN_TEST(test_peek_does_not_remove_packet);
    RUN_TEST(test_pop_empty_buffer_returns_error);
    RUN_TEST(test_add_multiple_packets_fifo_order);
    RUN_TEST(test_get_stats_after_add);
    RUN_TEST(test_flush_calls_callback_for_each_packet);
    RUN_TEST(test_flush_null_callback_returns_error);
    RUN_TEST(test_clear_empties_buffer);
    RUN_TEST(test_estimate_packet_count_positive);
    return UNITY_END();
}

