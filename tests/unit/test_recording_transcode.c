/**
 * @file test_recording_transcode.c
 * @brief Layer 2 unit tests — on-demand HEVC->H.264 recording playback cache
 *
 * FrontDoor's recordings are stored as native HEVC (mp4_writer.c is a pure
 * stream-copy remuxer, so whatever codec the camera sends is what lands on
 * disk). Most browsers have no royalty-free HEVC decoder for an HTML5
 * <video> element, so playing such a recording from the recordings page
 * fails with "no video with supported format and MIME type found" even
 * though the same file plays fine in a local player. These tests cover the
 * codec probe and on-demand transcode-and-cache helpers that fix that for
 * in-browser playback without changing how recordings are stored.
 *
 * Fixture files are generated with the real `ffmpeg` binary at test setup
 * (software libx265/libx264 encodes of a one-frame test pattern) so the
 * probe and transcode functions run against real containers, not mocks.
 * ensure_recording_transcode_cache() itself prefers VAAPI hardware
 * transcode when /dev/dri/renderD128 exists and falls back to software
 * libx264 otherwise, so these tests pass identically on hosts with and
 * without a GPU (e.g. CI).
 */

#define _POSIX_C_SOURCE 200809L
#define _GNU_SOURCE

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/stat.h>
#include <sys/wait.h>
#include <unistd.h>

#include "unity.h"
#include "video/recording_transcode.h"

#define TEST_DIR_TEMPLATE "/tmp/lightnvr_unit_recording_transcodeXXXXXX"

static char g_test_dir[256];
static char g_hevc_fixture[320];
static char g_h264_fixture[320];

static int run_ffmpeg(const char *args_joined_by_space_command) {
    /* Test-only helper: build known-good fixture files with the real
     * ffmpeg binary. Not a mock of production behavior. */
    char cmd[1024];
    snprintf(cmd, sizeof(cmd), "ffmpeg -hide_banner -loglevel error -y %s",
             args_joined_by_space_command);
    int rc = system(cmd);
    return (rc == -1) ? -1 : WEXITSTATUS(rc);
}

static bool file_exists_nonempty(const char *path) {
    struct stat st;
    return stat(path, &st) == 0 && S_ISREG(st.st_mode) && st.st_size > 0;
}

void setUp(void) {
    strcpy(g_test_dir, TEST_DIR_TEMPLATE);
    TEST_ASSERT_NOT_NULL(mkdtemp(g_test_dir));

    snprintf(g_hevc_fixture, sizeof(g_hevc_fixture), "%s/hevc_source.mp4", g_test_dir);
    snprintf(g_h264_fixture, sizeof(g_h264_fixture), "%s/h264_source.mp4", g_test_dir);

    char args[512];
    snprintf(args, sizeof(args),
             "-f lavfi -i testsrc=size=64x64:rate=1:duration=1 "
             "-c:v libx265 -pix_fmt yuv420p \"%s\"", g_hevc_fixture);
    TEST_ASSERT_EQUAL_INT(0, run_ffmpeg(args));
    TEST_ASSERT_TRUE(file_exists_nonempty(g_hevc_fixture));

    snprintf(args, sizeof(args),
             "-f lavfi -i testsrc=size=64x64:rate=1:duration=1 "
             "-c:v libx264 -pix_fmt yuv420p \"%s\"", g_h264_fixture);
    TEST_ASSERT_EQUAL_INT(0, run_ffmpeg(args));
    TEST_ASSERT_TRUE(file_exists_nonempty(g_h264_fixture));
}

static void remove_tree(const char *path) {
    char cmd[512];
    snprintf(cmd, sizeof(cmd), "rm -rf \"%s\"", path);
    system(cmd);
}

void tearDown(void) {
    remove_tree(g_test_dir);
    g_test_dir[0] = '\0';
}

/* ================================================================
 * recording_needs_hevc_transcode
 * ================================================================ */

void test_needs_transcode_true_for_hevc_file(void) {
    TEST_ASSERT_TRUE(recording_needs_hevc_transcode(g_hevc_fixture));
}

void test_needs_transcode_false_for_h264_file(void) {
    TEST_ASSERT_FALSE(recording_needs_hevc_transcode(g_h264_fixture));
}

void test_needs_transcode_fails_open_for_missing_file(void) {
    char missing[320];
    snprintf(missing, sizeof(missing), "%s/does_not_exist.mp4", g_test_dir);
    TEST_ASSERT_FALSE(recording_needs_hevc_transcode(missing));
}

void test_needs_transcode_fails_open_for_null_path(void) {
    TEST_ASSERT_FALSE(recording_needs_hevc_transcode(NULL));
}

/* ================================================================
 * ensure_recording_transcode_cache
 * ================================================================ */

void test_ensure_cache_transcodes_hevc_source_to_playable_h264(void) {
    char cache_path[320];
    snprintf(cache_path, sizeof(cache_path), "%s/cache_out.mp4", g_test_dir);

    TEST_ASSERT_EQUAL_INT(0, ensure_recording_transcode_cache(g_hevc_fixture, cache_path));
    TEST_ASSERT_TRUE(file_exists_nonempty(cache_path));
    /* The whole point: the produced file must no longer need transcoding. */
    TEST_ASSERT_FALSE(recording_needs_hevc_transcode(cache_path));
}

void test_ensure_cache_skips_retranscode_when_cache_already_exists(void) {
    char cache_path[320];
    snprintf(cache_path, sizeof(cache_path), "%s/precached.mp4", g_test_dir);

    /* Deliberately not a real video — proves a cache hit is served as-is
     * rather than re-invoking ffmpeg, without needing to mock the process. */
    FILE *f = fopen(cache_path, "wb");
    TEST_ASSERT_NOT_NULL(f);
    fputs("not-a-real-video", f);
    fclose(f);

    TEST_ASSERT_EQUAL_INT(0, ensure_recording_transcode_cache(g_hevc_fixture, cache_path));

    FILE *check = fopen(cache_path, "rb");
    TEST_ASSERT_NOT_NULL(check);
    char buf[64] = {0};
    size_t n = fread(buf, 1, sizeof(buf) - 1, check);
    fclose(check);
    TEST_ASSERT_TRUE(n > 0);
    TEST_ASSERT_EQUAL_STRING("not-a-real-video", buf);
}

void test_ensure_cache_creates_missing_parent_directory(void) {
    /* Production cache paths live under <storage_path>/transcoded/, which
     * won't exist on a fresh install — the function must create it. */
    char cache_path[320];
    snprintf(cache_path, sizeof(cache_path), "%s/transcoded/42.mp4", g_test_dir);

    TEST_ASSERT_EQUAL_INT(0, ensure_recording_transcode_cache(g_hevc_fixture, cache_path));
    TEST_ASSERT_TRUE(file_exists_nonempty(cache_path));
}

void test_ensure_cache_fails_gracefully_for_missing_source(void) {
    char missing[320];
    char cache_path[320];
    snprintf(missing, sizeof(missing), "%s/does_not_exist.mp4", g_test_dir);
    snprintf(cache_path, sizeof(cache_path), "%s/should_not_appear.mp4", g_test_dir);

    TEST_ASSERT_EQUAL_INT(-1, ensure_recording_transcode_cache(missing, cache_path));
    TEST_ASSERT_FALSE(file_exists_nonempty(cache_path));

    /* No leaked temp file either. */
    char list_cmd[512];
    snprintf(list_cmd, sizeof(list_cmd), "ls \"%s\" | grep -c tmp", g_test_dir);
    FILE *p = popen(list_cmd, "r");
    TEST_ASSERT_NOT_NULL(p);
    char count_buf[16] = {0};
    TEST_ASSERT_NOT_NULL(fgets(count_buf, sizeof(count_buf), p));
    pclose(p);
    TEST_ASSERT_EQUAL_INT(0, atoi(count_buf));
}

/* ================================================================
 * main
 * ================================================================ */

int main(void) {
    UNITY_BEGIN();

    RUN_TEST(test_needs_transcode_true_for_hevc_file);
    RUN_TEST(test_needs_transcode_false_for_h264_file);
    RUN_TEST(test_needs_transcode_fails_open_for_missing_file);
    RUN_TEST(test_needs_transcode_fails_open_for_null_path);

    RUN_TEST(test_ensure_cache_transcodes_hevc_source_to_playable_h264);
    RUN_TEST(test_ensure_cache_creates_missing_parent_directory);
    RUN_TEST(test_ensure_cache_skips_retranscode_when_cache_already_exists);
    RUN_TEST(test_ensure_cache_fails_gracefully_for_missing_source);

    return UNITY_END();
}
