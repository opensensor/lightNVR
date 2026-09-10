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
 *
 * If `ffmpeg`/`ffprobe` aren't on PATH (e.g. an environment that only
 * installs the FFmpeg *-dev libraries this project links against, not the
 * CLI tools), every test here skips via TEST_IGNORE rather than hard-
 * failing, matching test_go2rtc_two_config_merge.c's pattern for its own
 * optional external binary.
 */

#define _POSIX_C_SOURCE 200809L
#define _GNU_SOURCE

#include <pthread.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/stat.h>
#include <sys/wait.h>
#include <time.h>
#include <unistd.h>

#include "unity.h"
#include "video/recording_transcode.h"

#define TEST_DIR_TEMPLATE "/tmp/lightnvr_unit_recording_transcodeXXXXXX"

static char g_test_dir[256];
static char g_hevc_fixture[320];
static char g_h264_fixture[320];
static char g_hevc_with_audio_fixture[320];
static char *g_original_path;
static int s_skip = 0;

/* Matches the skip pattern in test_go2rtc_two_config_merge.c: an
 * environment that can build this test target but doesn't have the
 * optional external binaries (e.g. sanitizer.yml, which installs only the
 * FFmpeg *-dev libraries this project links against, not the ffmpeg/ffprobe
 * CLI tools) should skip gracefully rather than hard-fail the whole suite. */
static bool binaries_available(void) {
    return system("command -v ffmpeg >/dev/null 2>&1 && "
                   "command -v ffprobe >/dev/null 2>&1") == 0;
}

static int run_ffmpeg(const char *args_joined_by_space_command) {
    /* Test-only helper: build known-good fixture files with the real
     * ffmpeg binary. Not a mock of production behavior. */
    char cmd[1024];
    snprintf(cmd, sizeof(cmd), "ffmpeg -hide_banner -loglevel error -y %s",
             args_joined_by_space_command);
    int rc = system(cmd);
    if (rc == -1) {
        return -1;
    }
    return WIFEXITED(rc) ? WEXITSTATUS(rc) : -1;
}

static bool file_exists_nonempty(const char *path) {
    struct stat st;
    return stat(path, &st) == 0 && S_ISREG(st.st_mode) && st.st_size > 0;
}

void setUp(void) {
    s_skip = !binaries_available();
    if (s_skip) {
        g_test_dir[0] = '\0';
        return;
    }

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

    /* mp4_writer.c already transcodes any PCM audio a camera sends (e.g.
     * pcm_alaw) to AAC before writing the MP4 — MP4 can't mux raw PCM at
     * all — so a real recording's audio track is always AAC by the time
     * this feature ever sees it. Match that here. */
    snprintf(g_hevc_with_audio_fixture, sizeof(g_hevc_with_audio_fixture),
             "%s/hevc_with_audio_source.mp4", g_test_dir);
    snprintf(args, sizeof(args),
             "-f lavfi -i testsrc=size=64x64:rate=1:duration=1 "
             "-f lavfi -i sine=frequency=1000:duration=1 "
             "-c:v libx265 -pix_fmt yuv420p -c:a aac \"%s\"",
             g_hevc_with_audio_fixture);
    TEST_ASSERT_EQUAL_INT(0, run_ffmpeg(args));
    TEST_ASSERT_TRUE(file_exists_nonempty(g_hevc_with_audio_fixture));
}

/* Test-only helper: read back the first audio stream's codec name via
 * ffprobe. Returns an empty string if the file has no audio stream. */
static void probe_audio_codec_name(const char *path, char *out, size_t out_size) {
    char cmd[512];
    snprintf(cmd, sizeof(cmd),
             "ffprobe -v error -select_streams a:0 -show_entries stream=codec_name "
             "-of default=noprint_wrappers=1:nokey=1 \"%s\"", path);
    out[0] = '\0';
    FILE *p = popen(cmd, "r");
    if (!p) return;
    if (fgets(out, (int)out_size, p)) {
        size_t len = strlen(out);
        if (len > 0 && out[len - 1] == '\n') out[len - 1] = '\0';
    }
    pclose(p);
}

static void remove_tree(const char *path) {
    char cmd[512];
    snprintf(cmd, sizeof(cmd), "rm -rf \"%s\"", path);
    system(cmd);
}

void tearDown(void) {
    if (g_original_path) {
        setenv("PATH", g_original_path, 1);
        free(g_original_path);
        g_original_path = NULL;
        unsetenv("LIGHTNVR_TEST_REAL_FFMPEG");
        unsetenv("LIGHTNVR_TEST_TRANSCODE_DIR");
    }
    if (g_test_dir[0] != '\0') {
        remove_tree(g_test_dir);
        g_test_dir[0] = '\0';
    }
}

/* ================================================================
 * recording_needs_hevc_transcode
 * ================================================================ */

void test_needs_transcode_true_for_hevc_file(void) {
    if (s_skip) { TEST_IGNORE_MESSAGE("ffmpeg/ffprobe not found in PATH"); return; }
    TEST_ASSERT_TRUE(recording_needs_hevc_transcode(g_hevc_fixture));
}

void test_needs_transcode_false_for_h264_file(void) {
    if (s_skip) { TEST_IGNORE_MESSAGE("ffmpeg/ffprobe not found in PATH"); return; }
    TEST_ASSERT_FALSE(recording_needs_hevc_transcode(g_h264_fixture));
}

void test_needs_transcode_fails_open_for_missing_file(void) {
    if (s_skip) { TEST_IGNORE_MESSAGE("ffmpeg/ffprobe not found in PATH"); return; }
    char missing[320];
    snprintf(missing, sizeof(missing), "%s/does_not_exist.mp4", g_test_dir);
    TEST_ASSERT_FALSE(recording_needs_hevc_transcode(missing));
}

void test_needs_transcode_fails_open_for_null_path(void) {
    if (s_skip) { TEST_IGNORE_MESSAGE("ffmpeg/ffprobe not found in PATH"); return; }
    TEST_ASSERT_FALSE(recording_needs_hevc_transcode(NULL));
}

/* ================================================================
 * ensure_recording_transcode_cache
 * ================================================================ */

void test_ensure_cache_transcodes_hevc_source_to_playable_h264(void) {
    if (s_skip) { TEST_IGNORE_MESSAGE("ffmpeg/ffprobe not found in PATH"); return; }
    char cache_path[320];
    snprintf(cache_path, sizeof(cache_path), "%s/cache_out.mp4", g_test_dir);

    TEST_ASSERT_EQUAL_INT(0, ensure_recording_transcode_cache(g_hevc_fixture, cache_path));
    TEST_ASSERT_TRUE(file_exists_nonempty(cache_path));
    /* The whole point: the produced file must no longer need transcoding. */
    TEST_ASSERT_FALSE(recording_needs_hevc_transcode(cache_path));
}

void test_ensure_cache_skips_retranscode_when_cache_already_exists(void) {
    if (s_skip) { TEST_IGNORE_MESSAGE("ffmpeg/ffprobe not found in PATH"); return; }
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

void test_ensure_cache_transcodes_audio_to_aac_when_source_has_audio(void) {
    if (s_skip) { TEST_IGNORE_MESSAGE("ffmpeg/ffprobe not found in PATH"); return; }
    char cache_path[320];
    snprintf(cache_path, sizeof(cache_path), "%s/cache_with_audio_out.mp4", g_test_dir);

    TEST_ASSERT_EQUAL_INT(0, ensure_recording_transcode_cache(g_hevc_with_audio_fixture, cache_path));
    TEST_ASSERT_TRUE(file_exists_nonempty(cache_path));

    char codec_name[64];
    probe_audio_codec_name(cache_path, codec_name, sizeof(codec_name));
    TEST_ASSERT_EQUAL_STRING("aac", codec_name);
}

void test_ensure_cache_creates_missing_parent_directory(void) {
    if (s_skip) { TEST_IGNORE_MESSAGE("ffmpeg/ffprobe not found in PATH"); return; }
    /* Production cache paths live under <storage_path>/transcoded/, which
     * won't exist on a fresh install — the function must create it. */
    char cache_path[320];
    snprintf(cache_path, sizeof(cache_path), "%s/transcoded/42.mp4", g_test_dir);

    TEST_ASSERT_EQUAL_INT(0, ensure_recording_transcode_cache(g_hevc_fixture, cache_path));
    TEST_ASSERT_TRUE(file_exists_nonempty(cache_path));
}

void test_ensure_cache_fails_gracefully_for_missing_source(void) {
    if (s_skip) { TEST_IGNORE_MESSAGE("ffmpeg/ffprobe not found in PATH"); return; }
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
 * concurrency
 * ================================================================ */

typedef struct {
    pthread_mutex_t mutex;
    pthread_cond_t cond;
    bool open;
} start_gate_t;

typedef struct {
    const char *original_path;
    const char *cache_path;
    int result;
    start_gate_t *gate;
} concurrent_call_args_t;

static void *concurrent_call_thread(void *arg) {
    concurrent_call_args_t *args = (concurrent_call_args_t *)arg;
    if (args->gate) {
        pthread_mutex_lock(&args->gate->mutex);
        while (!args->gate->open) {
            pthread_cond_wait(&args->gate->cond, &args->gate->mutex);
        }
        pthread_mutex_unlock(&args->gate->mutex);
    }
    args->result = ensure_recording_transcode_cache(args->original_path, args->cache_path);
    return NULL;
}

/* Regression test for a real bug: HTTP handlers are dispatched across
 * libuv's worker threads within one process, so two simultaneous playback
 * requests for the *same* recording used to compute the identical
 * getpid()-only tmp path and race on the same file. Drives two threads at
 * the same cache_path concurrently and asserts both succeed, the result is
 * a valid playable file, and no tmp file is left behind. */
void test_ensure_cache_concurrent_calls_for_same_recording_dont_corrupt_cache(void) {
    if (s_skip) { TEST_IGNORE_MESSAGE("ffmpeg/ffprobe not found in PATH"); return; }
    char cache_path[320];
    snprintf(cache_path, sizeof(cache_path), "%s/concurrent_out.mp4", g_test_dir);

    concurrent_call_args_t args_a = { g_hevc_fixture, cache_path, -1, NULL };
    concurrent_call_args_t args_b = { g_hevc_fixture, cache_path, -1, NULL };

    pthread_t thread_a, thread_b;
    TEST_ASSERT_EQUAL_INT(0, pthread_create(&thread_a, NULL, concurrent_call_thread, &args_a));
    TEST_ASSERT_EQUAL_INT(0, pthread_create(&thread_b, NULL, concurrent_call_thread, &args_b));
    pthread_join(thread_a, NULL);
    pthread_join(thread_b, NULL);

    TEST_ASSERT_EQUAL_INT(0, args_a.result);
    TEST_ASSERT_EQUAL_INT(0, args_b.result);
    TEST_ASSERT_TRUE(file_exists_nonempty(cache_path));
    TEST_ASSERT_FALSE(recording_needs_hevc_transcode(cache_path));

    char list_cmd[512];
    snprintf(list_cmd, sizeof(list_cmd), "ls \"%s\" | grep -c tmp", g_test_dir);
    FILE *p = popen(list_cmd, "r");
    TEST_ASSERT_NOT_NULL(p);
    char count_buf[16] = {0};
    TEST_ASSERT_NOT_NULL(fgets(count_buf, sizeof(count_buf), p));
    pclose(p);
    TEST_ASSERT_EQUAL_INT(0, atoi(count_buf));
}

/* Wrap the real encoder to observe subprocess concurrency, rather than only
 * checking the final cache (which passed even when duplicate jobs caused OOM).
 * Holding each invocation briefly makes overlapping requests exercise a cache
 * miss. Force software encoding so these checks also work on VAAPI hosts. */
static void track_ffmpeg_processes(void) {
    char real_ffmpeg[1024];
    FILE *pipe = popen("command -v ffmpeg", "r");
    TEST_ASSERT_NOT_NULL(pipe);
    char *found = fgets(real_ffmpeg, sizeof(real_ffmpeg), pipe);
    int status = pclose(pipe);
    TEST_ASSERT_NOT_NULL(found);
    TEST_ASSERT_EQUAL_INT(0, status);
    real_ffmpeg[strcspn(real_ffmpeg, "\n")] = '\0';

    char wrapper[320];
    snprintf(wrapper, sizeof(wrapper), "%s/ffmpeg", g_test_dir);
    FILE *file = fopen(wrapper, "w");
    TEST_ASSERT_NOT_NULL(file);
    fputs("#!/bin/sh\n"
          "for arg do\n"
          "  [ \"$arg\" != h264_vaapi ] || exit 1\n"
          "done\n"
          "printf 'start\\n' >> \"$LIGHTNVR_TEST_TRANSCODE_DIR/starts\"\n"
          "owned=0\n"
          "if mkdir \"$LIGHTNVR_TEST_TRANSCODE_DIR/active\"; then\n"
          "  owned=1\n"
          "else\n"
          "  touch \"$LIGHTNVR_TEST_TRANSCODE_DIR/overlap\"\n"
          "fi\n"
          "sleep 1\n"
          "\"$LIGHTNVR_TEST_REAL_FFMPEG\" \"$@\"\n"
          "rc=$?\n"
          "if [ \"$owned\" = 1 ]; then rmdir \"$LIGHTNVR_TEST_TRANSCODE_DIR/active\"; fi\n"
          "exit \"$rc\"\n", file);
    TEST_ASSERT_EQUAL_INT(0, fclose(file));
    TEST_ASSERT_EQUAL_INT(0, chmod(wrapper, 0700));

    g_original_path = strdup(getenv("PATH"));
    TEST_ASSERT_NOT_NULL(g_original_path);
    char *wrapped_path = NULL;
    TEST_ASSERT_TRUE(asprintf(&wrapped_path, "%s:%s", g_test_dir, g_original_path) > 0);
    TEST_ASSERT_EQUAL_INT(0, setenv("LIGHTNVR_TEST_REAL_FFMPEG", real_ffmpeg, 1));
    TEST_ASSERT_EQUAL_INT(0, setenv("LIGHTNVR_TEST_TRANSCODE_DIR", g_test_dir, 1));
    int rc = setenv("PATH", wrapped_path, 1);
    free(wrapped_path);
    TEST_ASSERT_EQUAL_INT(0, rc);
}

static void assert_ffmpeg_process_counts(int expected_starts) {
    char path[320];
    snprintf(path, sizeof(path), "%s/starts", g_test_dir);
    FILE *file = fopen(path, "r");
    TEST_ASSERT_NOT_NULL(file);
    int count = 0;
    for (int c; (c = fgetc(file)) != EOF;) {
        if (c == '\n') count++;
    }
    fclose(file);
    TEST_ASSERT_EQUAL_INT(expected_starts, count);
    snprintf(path, sizeof(path), "%s/overlap", g_test_dir);
    TEST_ASSERT_EQUAL_INT_MESSAGE(-1, access(path, F_OK), "FFmpeg processes overlapped");
    snprintf(path, sizeof(path), "%s/active", g_test_dir);
    TEST_ASSERT_EQUAL_INT(-1, access(path, F_OK));
}

static void run_concurrent_transcodes(bool same_recording) {
    track_ffmpeg_processes();
    start_gate_t gate = { PTHREAD_MUTEX_INITIALIZER, PTHREAD_COND_INITIALIZER, false };
    char caches[3][320];
    concurrent_call_args_t args[3];
    pthread_t threads[3];
    int created = 0;
    int create_rc = 0;
    for (int i = 0; i < 3; i++) {
        snprintf(caches[i], sizeof(caches[i]), "%s/cache_%d.mp4", g_test_dir,
                 same_recording ? 0 : i);
        args[i] = (concurrent_call_args_t){ g_hevc_fixture, caches[i], -1, &gate };
        create_rc = pthread_create(&threads[i], NULL, concurrent_call_thread, &args[i]);
        if (create_rc != 0) break;
        created++;
    }
    pthread_mutex_lock(&gate.mutex);
    gate.open = true;
    pthread_cond_broadcast(&gate.cond);
    pthread_mutex_unlock(&gate.mutex);
    for (int i = 0; i < created; i++) pthread_join(threads[i], NULL);
    pthread_cond_destroy(&gate.cond);
    pthread_mutex_destroy(&gate.mutex);

    TEST_ASSERT_EQUAL_INT(0, create_rc);
    for (int i = 0; i < 3; i++) {
        TEST_ASSERT_EQUAL_INT(0, args[i].result);
        TEST_ASSERT_TRUE(file_exists_nonempty(caches[i]));
        TEST_ASSERT_FALSE(recording_needs_hevc_transcode(caches[i]));
    }
    assert_ffmpeg_process_counts(same_recording ? 1 : 3);
}

void test_concurrent_playback_requests_share_one_transcode(void) {
    if (s_skip) { TEST_IGNORE_MESSAGE("ffmpeg/ffprobe not found in PATH"); return; }
    run_concurrent_transcodes(true);
}

void test_different_recordings_transcode_one_at_a_time(void) {
    if (s_skip) { TEST_IGNORE_MESSAGE("ffmpeg/ffprobe not found in PATH"); return; }
    run_concurrent_transcodes(false);
}

void test_failed_transcode_releases_slot_for_retry(void) {
    if (s_skip) { TEST_IGNORE_MESSAGE("ffmpeg/ffprobe not found in PATH"); return; }
    char bad_source[320], cache[320];
    snprintf(bad_source, sizeof(bad_source), "%s/invalid.mp4", g_test_dir);
    snprintf(cache, sizeof(cache), "%s/retry.mp4", g_test_dir);
    FILE *file = fopen(bad_source, "w");
    TEST_ASSERT_NOT_NULL(file);
    fputs("not a video", file);
    fclose(file);

    track_ffmpeg_processes();
    TEST_ASSERT_EQUAL_INT(-1, ensure_recording_transcode_cache(bad_source, cache));
    TEST_ASSERT_EQUAL_INT(-1, access(cache, F_OK));
    TEST_ASSERT_EQUAL_INT(0, ensure_recording_transcode_cache(g_hevc_fixture, cache));
    TEST_ASSERT_FALSE(recording_needs_hevc_transcode(cache));
    assert_ffmpeg_process_counts(2);
}

static double monotonic_seconds(void) {
    struct timespec now;
    clock_gettime(CLOCK_MONOTONIC, &now);
    return now.tv_sec + now.tv_nsec / 1e9;
}

static recording_transcode_status_t wait_for_cache(const char *source, const char *cache) {
    double deadline = monotonic_seconds() + 10;
    recording_transcode_status_t status;
    do {
        status = request_recording_transcode_cache(source, cache);
        if (status != RECORDING_TRANSCODE_PENDING) return status;
        usleep(10000);
    } while (monotonic_seconds() < deadline);
    return status;
}

void test_background_requests_return_while_encoder_is_running(void) {
    if (s_skip) { TEST_IGNORE_MESSAGE("ffmpeg/ffprobe not found in PATH"); return; }
    track_ffmpeg_processes();
    char cache[320], other[320];
    snprintf(cache, sizeof(cache), "%s/background.mp4", g_test_dir);
    snprintf(other, sizeof(other), "%s/other.mp4", g_test_dir);
    double start = monotonic_seconds();
    for (int i = 0; i < 20; i++) {
        TEST_ASSERT_EQUAL_INT(RECORDING_TRANSCODE_PENDING,
            request_recording_transcode_cache(g_hevc_fixture, cache));
        TEST_ASSERT_EQUAL_INT(RECORDING_TRANSCODE_PENDING,
            request_recording_transcode_cache(g_hevc_fixture, other));
        TEST_ASSERT_EQUAL_INT(RECORDING_TRANSCODE_READY,
            request_recording_transcode_cache(g_hevc_fixture, g_h264_fixture));
    }
    TEST_ASSERT_TRUE_MESSAGE(monotonic_seconds() - start < 0.5,
                             "Playback requests waited for FFmpeg");
    TEST_ASSERT_EQUAL_INT(RECORDING_TRANSCODE_READY, wait_for_cache(g_hevc_fixture, cache));
    TEST_ASSERT_FALSE(recording_needs_hevc_transcode(cache));
    assert_ffmpeg_process_counts(1);
    TEST_ASSERT_EQUAL_INT(RECORDING_TRANSCODE_READY, wait_for_cache(g_hevc_fixture, other));
    assert_ffmpeg_process_counts(2);
}

void test_background_failure_is_reported_without_restarting_encoder(void) {
    if (s_skip) { TEST_IGNORE_MESSAGE("ffmpeg/ffprobe not found in PATH"); return; }
    track_ffmpeg_processes();
    char bad[320], cache[320];
    snprintf(bad, sizeof(bad), "%s/invalid.mp4", g_test_dir);
    snprintf(cache, sizeof(cache), "%s/failed_background.mp4", g_test_dir);
    FILE *file = fopen(bad, "w");
    TEST_ASSERT_NOT_NULL(file);
    fputs("not a video", file);
    fclose(file);
    TEST_ASSERT_EQUAL_INT(RECORDING_TRANSCODE_FAILED, wait_for_cache(bad, cache));
    for (int i = 0; i < 10; i++) {
        TEST_ASSERT_EQUAL_INT(RECORDING_TRANSCODE_FAILED,
            request_recording_transcode_cache(bad, cache));
    }
    TEST_ASSERT_FALSE(file_exists_nonempty(cache));
    assert_ffmpeg_process_counts(1);
    TEST_ASSERT_EQUAL_INT(RECORDING_TRANSCODE_READY, wait_for_cache(g_hevc_fixture, cache));
    assert_ffmpeg_process_counts(2);
}

void test_shutdown_aborts_background_encoder(void) {
    if (s_skip) { TEST_IGNORE_MESSAGE("ffmpeg/ffprobe not found in PATH"); return; }
    track_ffmpeg_processes();
    char wrapper[320], cache[320];
    snprintf(wrapper, sizeof(wrapper), "%s/ffmpeg", g_test_dir);
    snprintf(cache, sizeof(cache), "%s/shutdown.mp4", g_test_dir);
    FILE *file = fopen(wrapper, "w");
    TEST_ASSERT_NOT_NULL(file);
    /* exec keeps the simulated slow encoder in the tracked child PID. */
    fputs("#!/bin/sh\nexec sleep 30\n", file);
    fclose(file);
    TEST_ASSERT_EQUAL_INT(RECORDING_TRANSCODE_PENDING,
        request_recording_transcode_cache(g_hevc_fixture, cache));
    usleep(200000);
    double start = monotonic_seconds();
    shutdown_recording_transcode();
    TEST_ASSERT_TRUE_MESSAGE(monotonic_seconds() - start < 1,
                             "Shutdown waited for the full encode");
    TEST_ASSERT_FALSE(file_exists_nonempty(cache));
    TEST_ASSERT_EQUAL_INT(RECORDING_TRANSCODE_FAILED,
        request_recording_transcode_cache(g_hevc_fixture, cache));
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
    RUN_TEST(test_ensure_cache_transcodes_audio_to_aac_when_source_has_audio);
    RUN_TEST(test_ensure_cache_creates_missing_parent_directory);
    RUN_TEST(test_ensure_cache_skips_retranscode_when_cache_already_exists);
    RUN_TEST(test_ensure_cache_fails_gracefully_for_missing_source);
    RUN_TEST(test_ensure_cache_concurrent_calls_for_same_recording_dont_corrupt_cache);
    RUN_TEST(test_concurrent_playback_requests_share_one_transcode);
    RUN_TEST(test_different_recordings_transcode_one_at_a_time);
    RUN_TEST(test_failed_transcode_releases_slot_for_retry);
    RUN_TEST(test_background_requests_return_while_encoder_is_running);
    RUN_TEST(test_background_failure_is_reported_without_restarting_encoder);
    RUN_TEST(test_shutdown_aborts_background_encoder);

    return UNITY_END();
}
