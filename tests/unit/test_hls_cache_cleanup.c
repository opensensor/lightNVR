#define _POSIX_C_SOURCE 200809L

#include <dirent.h>
#include <fcntl.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/stat.h>
#include <unistd.h>

#include "unity.h"
#include "video/hls/hls_cache_cleanup.h"

static char root[128];
static const time_t NOW = 100000;

static void path(char *out, size_t capacity, const char *name) {
    snprintf(out, capacity, "%s/camera/%s", root, name);
}

static void create_file(const char *name, const char *content, time_t timestamp) {
    char file[512];
    path(file, sizeof(file), name);
    FILE *f = fopen(file, "w");
    TEST_ASSERT_NOT_NULL(f);
    TEST_ASSERT_GREATER_OR_EQUAL_INT(0, fputs(content, f));
    TEST_ASSERT_EQUAL_INT(0, fclose(f));
    struct timespec times[2] = {{timestamp, 0}, {timestamp, 0}};
    TEST_ASSERT_EQUAL_INT(0, utimensat(AT_FDCWD, file, times, 0));
}

static bool exists(const char *name) {
    char file[512];
    path(file, sizeof(file), name);
    struct stat st;
    return lstat(file, &st) == 0;
}

static hls_cache_cleanup_result_t run(unsigned limit, bool dry_run) {
    hls_cache_cleanup_result_t result;
    TEST_ASSERT_EQUAL_INT(0, hls_prune_cache(root, NOW,
        HLS_CACHE_MIN_AGE_SECONDS, limit, dry_run, &result));
    return result;
}

static void remove_tree(const char *directory) {
    DIR *dir = opendir(directory);
    if (!dir) return;
    struct dirent *entry;
    while ((entry = readdir(dir))) {
        if (!strcmp(entry->d_name, ".") || !strcmp(entry->d_name, "..")) continue;
        char file[512];
        snprintf(file, sizeof(file), "%s/%s", directory, entry->d_name);
        struct stat st;
        if (!lstat(file, &st) && S_ISDIR(st.st_mode)) remove_tree(file);
        else unlink(file);
    }
    closedir(dir);
    rmdir(directory);
}

void setUp(void) {
    strcpy(root, "/tmp/lightnvr_hls_cache_XXXXXX");
    TEST_ASSERT_NOT_NULL(mkdtemp(root));
    char camera[256];
    snprintf(camera, sizeof(camera), "%s/camera", root);
    TEST_ASSERT_EQUAL_INT(0, mkdir(camera, 0700));
}

void tearDown(void) { remove_tree(root); }

void test_old_orphan_segments_removed_with_no_playlist(void) {
    create_file("old.ts", "1234", NOW - 1000);
    create_file("old.m4s", "12345", NOW - 1000);
    hls_cache_cleanup_result_t result = run(0, false);
    TEST_ASSERT_EQUAL_UINT(2, result.removed);
    TEST_ASSERT_EQUAL_UINT64(9, result.bytes);
    TEST_ASSERT_FALSE(exists("old.ts"));
    TEST_ASSERT_FALSE(exists("old.m4s"));
}

void test_preserve_all_playlists_and_their_referenced_segments(void) {
    create_file("index.m3u8", "#EXTM3U\nkeep.ts\n", NOW - 1000);
    create_file("alternate.m3u8", "#EXTM3U\nother.m4s?token=ignored\n", NOW - 1000);
    create_file("keep.ts", "data", NOW - 1000);
    create_file("other.m4s", "data", NOW - 1000);
    create_file("orphan.ts", "data", NOW - 1000);
    TEST_ASSERT_EQUAL_UINT(1, run(0, false).removed);
    TEST_ASSERT_TRUE(exists("index.m3u8"));
    TEST_ASSERT_TRUE(exists("alternate.m3u8"));
    TEST_ASSERT_TRUE(exists("keep.ts"));
    TEST_ASSERT_TRUE(exists("other.m4s"));
}

void test_preserve_partial_segments_and_initialization_uri(void) {
    create_file("index.m3u8", "#EXTM3U\n#EXT-X-PART:DURATION=0.2,URI=\"part.m4s\"\n#EXT-X-MAP:URI=\"map.m4s\"\n", NOW - 1000);
    create_file("part.m4s", "data", NOW - 1000);
    create_file("map.m4s", "data", NOW - 1000);
    TEST_ASSERT_EQUAL_UINT(0, run(0, false).removed);
}

void test_recent_boundary_and_future_files_preserved(void) {
    create_file("recent.ts", "data", NOW - 100);
    create_file("boundary.ts", "data", NOW - HLS_CACHE_MIN_AGE_SECONDS);
    create_file("future.ts", "data", NOW + 100);
    TEST_ASSERT_EQUAL_UINT(0, run(0, false).removed);
}

void test_only_known_cache_suffixes_removed(void) {
    create_file("old.ts.tmp", "data", NOW - 1000);
    create_file("old.m4s.tmp", "data", NOW - 1000);
    create_file("old.m3u8.tmp", "data", NOW - 1000);
    create_file("init.mp4", "data", NOW - 1000);
    create_file("recording.mp4", "data", NOW - 1000);
    create_file("recording.ts.backup", "data", NOW - 1000);
    TEST_ASSERT_EQUAL_UINT(3, run(0, false).removed);
    TEST_ASSERT_TRUE(exists("init.mp4"));
    TEST_ASSERT_TRUE(exists("recording.mp4"));
    TEST_ASSERT_TRUE(exists("recording.ts.backup"));
}

void test_dry_run_is_nonmutating_and_removal_budget_is_enforced(void) {
    create_file("one.ts", "data", NOW - 1000);
    create_file("two.ts", "data", NOW - 1000);
    TEST_ASSERT_EQUAL_UINT(2, run(0, true).removed);
    TEST_ASSERT_TRUE(exists("one.ts"));
    TEST_ASSERT_TRUE(exists("two.ts"));
    TEST_ASSERT_EQUAL_UINT(1, run(1, false).removed);
    TEST_ASSERT_EQUAL_UINT(1, run(1, false).removed);
}

void test_symlink_segments_and_directories_are_not_followed(void) {
    create_file("outside.txt", "data", NOW - 1000);
    char file[512], target[512];
    path(target, sizeof(target), "outside.txt");
    path(file, sizeof(file), "linked.ts");
    TEST_ASSERT_EQUAL_INT(0, symlink(target, file));
    snprintf(file, sizeof(file), "%s/linked-camera", root);
    TEST_ASSERT_EQUAL_INT(0, symlink("camera", file));
    TEST_ASSERT_EQUAL_UINT(0, run(0, false).removed);
    TEST_ASSERT_TRUE(exists("linked.ts"));
    TEST_ASSERT_TRUE(exists("outside.txt"));
}

void test_unsupported_playlist_skips_directory(void) {
    create_file("index.m3u8", "#EXTM3U\nsubdir/segment.ts\n", NOW - 1000);
    create_file("old.ts", "data", NOW - 1000);
    hls_cache_cleanup_result_t result = run(0, false);
    TEST_ASSERT_EQUAL_UINT(1, result.skipped_directories);
    TEST_ASSERT_TRUE(exists("old.ts"));
}

void test_symlink_playlist_skips_directory(void) {
    create_file("playlist.txt", "#EXTM3U\nold.ts\n", NOW - 1000);
    create_file("old.ts", "data", NOW - 1000);
    char file[512];
    path(file, sizeof(file), "index.m3u8");
    TEST_ASSERT_EQUAL_INT(0, symlink("playlist.txt", file));
    TEST_ASSERT_EQUAL_UINT(1, run(0, false).skipped_directories);
    TEST_ASSERT_TRUE(exists("old.ts"));
}

void test_invalid_arguments_and_symlink_root_rejected(void) {
    hls_cache_cleanup_result_t result;
    TEST_ASSERT_EQUAL_INT(-1, hls_prune_cache(NULL, NOW, 300, 0, false, &result));
    TEST_ASSERT_EQUAL_INT(-1, hls_prune_cache(root, NOW, 0, 0, false, &result));
    TEST_ASSERT_EQUAL_INT(-1, hls_prune_cache(root, NOW, 300, 0, false, NULL));
    char link[256];
    snprintf(link, sizeof(link), "%s/link", root);
    TEST_ASSERT_EQUAL_INT(0, symlink(root, link));
    TEST_ASSERT_EQUAL_INT(-1, hls_prune_cache(link, NOW, 300, 0, false, &result));
}

int main(void) {
    UNITY_BEGIN();
    RUN_TEST(test_old_orphan_segments_removed_with_no_playlist);
    RUN_TEST(test_preserve_all_playlists_and_their_referenced_segments);
    RUN_TEST(test_preserve_partial_segments_and_initialization_uri);
    RUN_TEST(test_recent_boundary_and_future_files_preserved);
    RUN_TEST(test_only_known_cache_suffixes_removed);
    RUN_TEST(test_dry_run_is_nonmutating_and_removal_budget_is_enforced);
    RUN_TEST(test_symlink_segments_and_directories_are_not_followed);
    RUN_TEST(test_unsupported_playlist_skips_directory);
    RUN_TEST(test_symlink_playlist_skips_directory);
    RUN_TEST(test_invalid_arguments_and_symlink_root_rejected);
    return UNITY_END();
}
