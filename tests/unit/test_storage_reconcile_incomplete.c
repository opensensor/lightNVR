/**
 * Startup reconciliation of recordings interrupted by an unclean shutdown.
 *
 * Rows that cannot be pruned must not stop the scan: it used to re-read the same
 * oldest batch forever, which kept the storage controller from ever starting
 * retention. CTest's timeout on this binary is what catches a hang.
 */
#define _GNU_SOURCE

#include <errno.h>
#include <fcntl.h>
#include <libavcodec/avcodec.h>
#include <libavformat/avformat.h>
#include <libavutil/time.h>
#include "video/recording_media_probe.h"
#include <limits.h>
#include <sqlite3.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/stat.h>
#include <time.h>
#include <unistd.h>

#include "unity.h"
#include "core/config.h"
#include "database/db_core.h"
#include "database/db_recordings.h"
#include "database/db_storage_targets.h"
#include "storage/storage_deletion.h"
#include "storage/storage_manager.h"
#include "utils/strings.h"

#define BATCH_SIZE 500

static char g_root[64];
static char g_db_path[PATH_MAX];
static char g_present_dir[PATH_MAX];
static char g_absent_dir[PATH_MAX];
static int g_next_file;
static const char *g_failed_stat_path;
static char g_target_uuid[LIGHTNVR_UUID_STRING_SIZE];
static char g_playable[PATH_MAX];
static long g_playable_size;
static bool g_missing_decoder;
static bool g_probe_expired;
static int g_clock_calls;

const AVCodec *__real_avcodec_find_decoder(enum AVCodecID id);
const AVCodec *__wrap_avcodec_find_decoder(enum AVCodecID id) {
    return g_missing_decoder ? NULL : __real_avcodec_find_decoder(id);
}
int64_t __real_av_gettime_relative(void);
int64_t __wrap_av_gettime_relative(void) {
    return g_probe_expired ? (g_clock_calls++ ? 10000000 : 0) : __real_av_gettime_relative();
}

int __real_stat(const char *path, struct stat *st);
int __real_lstat(const char *path, struct stat *st);

int __wrap_stat(const char *path, struct stat *st) {
    if (g_failed_stat_path && strcmp(path, g_failed_stat_path) == 0) {
        errno = EIO;
        return -1;
    }
    return __real_stat(path, st);
}

int __wrap_lstat(const char *path, struct stat *st) {
    if (g_failed_stat_path && strcmp(path, g_failed_stat_path) == 0) {
        errno = EIO;
        return -1;
    }
    return __real_lstat(path, st);
}

static void exec_sql(const char *sql) {
    char *error = NULL;
    int rc = sqlite3_exec(get_db_handle(), sql, NULL, NULL, &error);
    TEST_ASSERT_EQUAL_STRING("", error ? error : "");
    TEST_ASSERT_EQUAL_INT(SQLITE_OK, rc);
}

static long query_count(const char *sql) {
    sqlite3_stmt *statement = NULL;
    TEST_ASSERT_EQUAL_INT(SQLITE_OK,
        sqlite3_prepare_v2(get_db_handle(), sql, -1, &statement, NULL));
    TEST_ASSERT_EQUAL_INT(SQLITE_ROW, sqlite3_step(statement));
    long value = (long)sqlite3_column_int64(statement, 0);
    sqlite3_finalize(statement);
    return value;
}

static void add_row(const char *path, bool complete, bool protected_row, const char *target) {
    char *sql = sqlite3_mprintf(
        "INSERT INTO recordings(stream_name,file_path,start_time,end_time,size_bytes,"
        "is_complete,protected,storage_target_uuid) VALUES('cam','%q',%lld,0,0,%d,%d,%Q);",
        path, (long long)time(NULL) - 7200, complete, protected_row, target);
    TEST_ASSERT_NOT_NULL(sql);
    exec_sql(sql);
    sqlite3_free(sql);
}

static void next_path(char *out, size_t size, const char *directory) {
    int written = snprintf(out, size, "%s/recording_%d.mp4", directory, g_next_file++);
    TEST_ASSERT_TRUE(written > 0 && (size_t)written < size);
}

static void add_missing_rows(const char *directory, int count) {
    char path[PATH_MAX];
    exec_sql("BEGIN;");
    for (int i = 0; i < count; i++) {
        next_path(path, sizeof(path), directory);
        add_row(path, false, false, NULL);
    }
    exec_sql("COMMIT;");
}

static void create_file(const char *path, const char *contents) {
    int fd = open(path, O_CREAT | O_TRUNC | O_WRONLY, 0644);
    TEST_ASSERT_TRUE(fd >= 0);
    size_t length = strlen(contents);
    TEST_ASSERT_EQUAL_INT((int)length, (int)write(fd, contents, length));
    close(fd);
}

/* Writes a short real MP4. Without the trailer it matches what a crash leaves. */
static void write_mp4(const char *path, bool faststart, bool trailer) {
    const AVCodec *codec = avcodec_find_encoder(AV_CODEC_ID_MPEG4);
    TEST_ASSERT_NOT_NULL(codec);
    AVCodecContext *encoder = avcodec_alloc_context3(codec);
    TEST_ASSERT_NOT_NULL(encoder);
    encoder->width = 64;
    encoder->height = 64;
    encoder->pix_fmt = AV_PIX_FMT_YUV420P;
    encoder->time_base = (AVRational){1, 10};
    encoder->flags |= AV_CODEC_FLAG_GLOBAL_HEADER;
    TEST_ASSERT_EQUAL_INT(0, avcodec_open2(encoder, codec, NULL));

    AVFormatContext *output = NULL;
    TEST_ASSERT_TRUE(avformat_alloc_output_context2(&output, NULL, "mp4", path) >= 0);
    AVStream *stream = avformat_new_stream(output, NULL);
    TEST_ASSERT_NOT_NULL(stream);
    TEST_ASSERT_TRUE(avcodec_parameters_from_context(stream->codecpar, encoder) >= 0);
    stream->time_base = encoder->time_base;
    TEST_ASSERT_TRUE(avio_open(&output->pb, path, AVIO_FLAG_WRITE) >= 0);
    AVDictionary *options = NULL;
    if (faststart) av_dict_set(&options, "movflags", "+faststart", 0);
    TEST_ASSERT_TRUE(avformat_write_header(output, &options) >= 0);
    av_dict_free(&options);

    AVFrame *frame = av_frame_alloc();
    AVPacket *packet = av_packet_alloc();
    TEST_ASSERT_NOT_NULL(frame);
    TEST_ASSERT_NOT_NULL(packet);
    frame->format = encoder->pix_fmt;
    frame->width = encoder->width;
    frame->height = encoder->height;
    TEST_ASSERT_TRUE(av_frame_get_buffer(frame, 0) >= 0);
    for (int i = 0; i <= 10; i++) {
        if (i < 10) {
            TEST_ASSERT_TRUE(av_frame_make_writable(frame) >= 0);
            memset(frame->data[0], 16 + i * 16, (size_t)frame->linesize[0] * 64);
            memset(frame->data[1], 128, (size_t)frame->linesize[1] * 32);
            memset(frame->data[2], 128, (size_t)frame->linesize[2] * 32);
            frame->pts = i;
        }
        TEST_ASSERT_TRUE(avcodec_send_frame(encoder, i < 10 ? frame : NULL) >= 0);
        while (avcodec_receive_packet(encoder, packet) == 0) {
            av_packet_rescale_ts(packet, encoder->time_base, stream->time_base);
            packet->stream_index = stream->index;
            TEST_ASSERT_TRUE(av_interleaved_write_frame(output, packet) >= 0);
        }
    }
    if (trailer) TEST_ASSERT_TRUE(av_write_trailer(output) >= 0);
    avio_closep(&output->pb);
    av_packet_free(&packet);
    av_frame_free(&frame);
    avformat_free_context(output);
    avcodec_free_context(&encoder);
}

/* Offset of the mdat payload in an MP4 made of 32-bit top-level boxes. */
static off_t mdat_payload_offset(const char *path, off_t *payload_size) {
    FILE *file = fopen(path, "rb");
    TEST_ASSERT_NOT_NULL(file);
    unsigned char header[8];
    off_t offset = 0;
    while (fread(header, 1, sizeof(header), file) == sizeof(header)) {
        off_t size = ((off_t)header[0] << 24) | (header[1] << 16) | (header[2] << 8) | header[3];
        if (memcmp(header + 4, "mdat", 4) == 0) {
            fclose(file);
            *payload_size = size - 8;
            return offset + 8;
        }
        TEST_ASSERT_TRUE(size >= 8);
        offset += size;
        TEST_ASSERT_EQUAL_INT(0, fseeko(file, offset, SEEK_SET));
    }
    fclose(file);
    TEST_FAIL_MESSAGE("no mdat box");
    return 0;
}

static void add_playable_row(const char *path) {
    write_mp4(path, true, true);
    add_row(path, false, false, NULL);
}

static void assert_visible_with_issue(const char *path) {
    TEST_ASSERT_EQUAL_INT(1, storage_reconcile_incomplete_recordings());
    TEST_ASSERT_EQUAL_INT(1, query_count(
        "SELECT COUNT(*) FROM recordings WHERE is_complete=1 AND playback_issue != '' AND deletion_pending=0;"));
    TEST_ASSERT_EQUAL_INT(0, access(path, F_OK));
    recording_metadata_t listing[2];
    TEST_ASSERT_EQUAL_INT(1, get_recording_metadata(0, 0, "cam", listing, 2));
    TEST_ASSERT_TRUE(recording_playback_unavailable(&listing[0]));
    TEST_ASSERT_TRUE(listing[0].size_bytes > 0);
    TEST_ASSERT_TRUE(listing[0].end_time >= listing[0].start_time);
    recording_metadata_t detail;
    TEST_ASSERT_EQUAL_INT(0, get_recording_metadata_by_id(listing[0].id, &detail));
    TEST_ASSERT_EQUAL_STRING(listing[0].playback_issue, detail.playback_issue);
    unlink(path);
}

void setUp(void) {
    g_failed_stat_path = NULL;
    g_missing_decoder = g_probe_expired = false;
    g_clock_calls = 0;
    exec_sql("DELETE FROM recordings;");
}

void tearDown(void) {
    g_failed_stat_path = NULL;
}

static void test_failed_media_lookup_keeps_the_file_and_row(void) {
    char path[PATH_MAX];
    next_path(path, sizeof(path), g_present_dir);
    create_file(path, "footage");
    add_row(path, false, false, NULL);
    g_failed_stat_path = path;

    TEST_ASSERT_EQUAL_INT(0, storage_reconcile_incomplete_recordings());
    TEST_ASSERT_EQUAL_INT(1, query_count(
        "SELECT COUNT(*) FROM recordings WHERE is_complete=0 AND deletion_pending=0;"));
    TEST_ASSERT_EQUAL_INT(0, access(path, F_OK));
    g_failed_stat_path = NULL;
    unlink(path);
}

static void test_nonregular_media_is_retained_without_scheduling_deletion(void) {
    char path[PATH_MAX];
    next_path(path, sizeof(path), g_present_dir);
    TEST_ASSERT_EQUAL_INT(0, symlink("missing.mp4", path));
    add_row(g_present_dir, false, false, NULL);
    add_row(path, false, false, NULL);

    TEST_ASSERT_EQUAL_INT(0, storage_reconcile_incomplete_recordings());
    TEST_ASSERT_EQUAL_INT(2, query_count(
        "SELECT COUNT(*) FROM recordings WHERE deletion_pending=0;"));
    unlink(path);
}

static void test_crash_file_without_a_trailer_stays_visible_with_issue(void) {
    char path[PATH_MAX];
    next_path(path, sizeof(path), g_present_dir);
    write_mp4(path, true, false);
    add_row(path, false, false, NULL);
    assert_visible_with_issue(path);
}

static void test_non_mp4_contents_stay_visible_with_issue(void) {
    char path[PATH_MAX];
    next_path(path, sizeof(path), g_present_dir);
    create_file(path, "footage");
    add_row(path, false, false, NULL);
    assert_visible_with_issue(path);
}

static void test_header_without_its_video_data_stays_visible_with_issue(void) {
    char path[PATH_MAX];
    next_path(path, sizeof(path), g_present_dir);
    write_mp4(path, true, true);
    off_t payload_size = 0;
    TEST_ASSERT_EQUAL_INT(0, truncate(path, mdat_payload_offset(path, &payload_size)));
    add_row(path, false, false, NULL);
    assert_visible_with_issue(path);
}

static void test_zeroed_video_data_stays_visible_with_issue(void) {
    char path[PATH_MAX];
    next_path(path, sizeof(path), g_present_dir);
    write_mp4(path, true, true);
    off_t payload_size = 0;
    off_t offset = mdat_payload_offset(path, &payload_size);
    TEST_ASSERT_TRUE(payload_size > 0);
    char *zeros = calloc(1, (size_t)payload_size);
    TEST_ASSERT_NOT_NULL(zeros);
    int fd = open(path, O_WRONLY);
    TEST_ASSERT_TRUE(fd >= 0);
    TEST_ASSERT_EQUAL_INT((int)payload_size, (int)pwrite(fd, zeros, (size_t)payload_size, offset));
    close(fd);
    free(zeros);
    add_row(path, false, false, NULL);
    assert_visible_with_issue(path);
}

static void test_unplayable_row_is_rechecked_at_the_next_startup(void) {
    char path[PATH_MAX];
    next_path(path, sizeof(path), g_present_dir);
    create_file(path, "footage");
    add_row(path, false, false, NULL);
    TEST_ASSERT_EQUAL_INT(1, storage_reconcile_incomplete_recordings());

    write_mp4(path, true, true);

    TEST_ASSERT_EQUAL_INT(1, storage_reconcile_incomplete_recordings());
    TEST_ASSERT_EQUAL_INT(1, query_count("SELECT COUNT(*) FROM recordings WHERE is_complete=1 AND playback_issue='';"));
    unlink(path);
}

static void test_playable_file_without_faststart_is_finalized(void) {
    char path[PATH_MAX];
    next_path(path, sizeof(path), g_present_dir);
    write_mp4(path, false, true);
    add_row(path, false, false, NULL);

    TEST_ASSERT_EQUAL_INT(1, storage_reconcile_incomplete_recordings());
    TEST_ASSERT_EQUAL_INT(1, query_count("SELECT COUNT(*) FROM recordings WHERE is_complete=1;"));
    unlink(path);
}

static void test_available_target_media_is_reconciled(void) {
    char missing[PATH_MAX], empty[PATH_MAX], survivor[PATH_MAX];
    next_path(missing, sizeof(missing), g_present_dir);
    next_path(empty, sizeof(empty), g_present_dir);
    next_path(survivor, sizeof(survivor), g_present_dir);
    create_file(empty, "");
    write_mp4(survivor, true, true);
    const char *paths[] = {missing, empty, survivor};
    for (size_t i = 0; i < sizeof(paths) / sizeof(paths[0]); i++) {
        add_row(paths[i], false, false, g_target_uuid);
        char *sql = sqlite3_mprintf(
            "UPDATE recordings SET object_key=%Q WHERE file_path=%Q;",
            strrchr(paths[i], '/') + 1, paths[i]);
        exec_sql(sql);
        sqlite3_free(sql);
    }

    TEST_ASSERT_EQUAL_INT(3, storage_reconcile_incomplete_recordings());
    TEST_ASSERT_EQUAL_INT(1, query_count("SELECT COUNT(*) FROM recordings;"));
    TEST_ASSERT_EQUAL_INT(g_playable_size, query_count(
        "SELECT size_bytes FROM recordings WHERE is_complete=1;"));
    TEST_ASSERT_EQUAL_INT(-1, access(empty, F_OK));
    TEST_ASSERT_EQUAL_INT(0, access(survivor, F_OK));
    unlink(survivor);
}

static void test_inconsistent_target_identity_cannot_delete_other_footage(void) {
    char path[PATH_MAX], actual_path[PATH_MAX];
    next_path(path, sizeof(path), g_present_dir);
    next_path(actual_path, sizeof(actual_path), g_present_dir);
    create_file(path, "");
    create_file(actual_path, "footage");
    add_row(path, false, false, g_target_uuid);
    char *sql = sqlite3_mprintf("UPDATE recordings SET object_key=%Q;",
                                strrchr(actual_path, '/') + 1);
    exec_sql(sql);
    sqlite3_free(sql);

    TEST_ASSERT_EQUAL_INT(0, storage_reconcile_incomplete_recordings());
    TEST_ASSERT_EQUAL_INT(1, query_count(
        "SELECT COUNT(*) FROM recordings WHERE deletion_pending=0;"));
    TEST_ASSERT_EQUAL_INT(0, access(actual_path, F_OK));
    unlink(path);
    unlink(actual_path);
}

static void test_empty_file_on_unmounted_target_is_retained(void) {
    char path[PATH_MAX];
    next_path(path, sizeof(path), g_present_dir);
    create_file(path, "");
    add_row(path, false, false, g_target_uuid);
    char *sql = sqlite3_mprintf(
        "UPDATE recordings SET object_key=%Q;"
        "UPDATE storage_targets SET mount_required=1,mount_guard_path=%Q WHERE uuid=%Q;",
        strrchr(path, '/') + 1, g_absent_dir, g_target_uuid);
    exec_sql(sql);
    sqlite3_free(sql);

    int reconciled = storage_reconcile_incomplete_recordings();
    exec_sql("UPDATE storage_targets SET mount_required=0;");
    TEST_ASSERT_EQUAL_INT(0, reconciled);
    TEST_ASSERT_EQUAL_INT(1, query_count(
        "SELECT COUNT(*) FROM recordings WHERE deletion_pending=0;"));
    TEST_ASSERT_EQUAL_INT(0, access(path, F_OK));
    unlink(path);
}

static void test_full_batches_of_phantom_rows_finish_and_reach_newer_footage(void) {
    add_missing_rows(g_present_dir, BATCH_SIZE * 2 + 1);
    char survivor[PATH_MAX];
    next_path(survivor, sizeof(survivor), g_present_dir);
    add_playable_row(survivor);

    TEST_ASSERT_EQUAL_INT(BATCH_SIZE * 2 + 2, storage_reconcile_incomplete_recordings());

    TEST_ASSERT_EQUAL_INT(1, query_count("SELECT COUNT(*) FROM recordings;"));
    TEST_ASSERT_EQUAL_INT(g_playable_size, query_count(
        "SELECT size_bytes FROM recordings WHERE is_complete=1;"));
    TEST_ASSERT_EQUAL_INT(0, access(survivor, F_OK));
    unlink(survivor);
}

static void test_rows_on_an_absent_volume_survive_without_stopping_the_scan(void) {
    add_missing_rows(g_absent_dir, BATCH_SIZE);
    char survivor[PATH_MAX];
    next_path(survivor, sizeof(survivor), g_present_dir);
    add_playable_row(survivor);

    TEST_ASSERT_EQUAL_INT(1, storage_reconcile_incomplete_recordings());

    TEST_ASSERT_EQUAL_INT(BATCH_SIZE, query_count(
        "SELECT COUNT(*) FROM recordings WHERE is_complete=0 AND deletion_pending=0;"));
    TEST_ASSERT_EQUAL_INT(1, query_count("SELECT COUNT(*) FROM recordings WHERE is_complete=1;"));
    unlink(survivor);
}

static void test_empty_leftover_is_removed_with_its_row(void) {
    char path[PATH_MAX];
    next_path(path, sizeof(path), g_present_dir);
    create_file(path, "");
    add_row(path, false, false, NULL);

    TEST_ASSERT_EQUAL_INT(1, storage_reconcile_incomplete_recordings());

    TEST_ASSERT_EQUAL_INT(0, query_count("SELECT COUNT(*) FROM recordings;"));
    TEST_ASSERT_EQUAL_INT(-1, access(path, F_OK));
}

static void test_protected_and_unresolvable_target_rows_are_retained(void) {
    char protected_path[PATH_MAX], target_path[PATH_MAX];
    next_path(protected_path, sizeof(protected_path), g_present_dir);
    next_path(target_path, sizeof(target_path), g_present_dir);
    add_row(protected_path, false, true, NULL);
    exec_sql("PRAGMA foreign_keys = OFF;");
    add_row(target_path, false, false, "00000000-0000-4000-8000-000000000000");
    exec_sql("PRAGMA foreign_keys = ON;");

    TEST_ASSERT_EQUAL_INT(0, storage_reconcile_incomplete_recordings());

    TEST_ASSERT_EQUAL_INT(2, query_count(
        "SELECT COUNT(*) FROM recordings WHERE deletion_pending=0;"));
}

static void test_complete_rows_are_out_of_scope(void) {
    char path[PATH_MAX];
    next_path(path, sizeof(path), g_present_dir);
    add_row(path, true, false, NULL);
    uint64_t id = (uint64_t)query_count("SELECT MAX(id) FROM recordings;");

    TEST_ASSERT_EQUAL_INT(0, storage_reconcile_incomplete_recordings());
    TEST_ASSERT_EQUAL_INT(-2, storage_recording_delete_stale_incomplete(id));

    TEST_ASSERT_EQUAL_INT(1, query_count("SELECT COUNT(*) FROM recordings;"));
}

static void test_ordinary_deletion_still_refuses_incomplete_rows(void) {
    char path[PATH_MAX];
    next_path(path, sizeof(path), g_present_dir);
    add_row(path, false, false, NULL);
    uint64_t id = (uint64_t)query_count("SELECT MAX(id) FROM recordings;");

    TEST_ASSERT_EQUAL_INT(-2, storage_recording_delete(id, "manual deletion", NULL));
    TEST_ASSERT_EQUAL_INT(-2, delete_recording_metadata(id));

    TEST_ASSERT_EQUAL_INT(1, query_count("SELECT COUNT(*) FROM recordings;"));
}

static void test_inconclusive_checks_stay_visible_and_do_not_block_playback(void) {
    const char *issues[] = {"decoder_unavailable", "probe_limit"};
    for (int i = 0; i < 2; ++i) {
        exec_sql("DELETE FROM recordings;");
        char path[PATH_MAX];
        next_path(path, sizeof(path), g_present_dir);
        add_playable_row(path);
        g_missing_decoder = i == 0;
        g_probe_expired = i == 1;
        g_clock_calls = 0;
        TEST_ASSERT_EQUAL_INT(1, storage_reconcile_incomplete_recordings());
        recording_metadata_t listing[2];
        TEST_ASSERT_EQUAL_INT(1, get_recording_metadata(0, 0, "cam", listing, 2));
        TEST_ASSERT_EQUAL_STRING(issues[i], listing[0].playback_issue);
        TEST_ASSERT_FALSE(recording_playback_unavailable(&listing[0]));
        unlink(path);
    }
}

static void test_damaged_recordings_keep_protection_and_can_be_deleted(void) {
    char path[PATH_MAX];
    next_path(path, sizeof(path), g_present_dir);
    create_file(path, "not a video");
    add_row(path, false, true, NULL);
    TEST_ASSERT_EQUAL_INT(1, storage_reconcile_incomplete_recordings());
    recording_metadata_t rows[1];
    TEST_ASSERT_EQUAL_INT(1, get_recording_metadata(0, 0, "cam", rows, 1));
    TEST_ASSERT_TRUE(rows[0].protected);
    TEST_ASSERT_TRUE(recording_playback_unavailable(&rows[0]));
    exec_sql("UPDATE recordings SET start_time=strftime('%s','now')-172800;");
    recording_metadata_t eligible[1];
    TEST_ASSERT_EQUAL_INT(0, get_recordings_for_retention("cam", 1, 1, eligible, 1));
    TEST_ASSERT_EQUAL_INT(0, get_recordings_for_pressure_cleanup(eligible, 1));
    exec_sql("UPDATE recordings SET protected=0;");
    TEST_ASSERT_EQUAL_INT(1, get_recordings_for_retention("cam", 1, 1, eligible, 1));
    TEST_ASSERT_EQUAL_UINT64(rows[0].id, eligible[0].id);
    TEST_ASSERT_EQUAL_INT(1, get_recordings_for_pressure_cleanup(eligible, 1));
    TEST_ASSERT_EQUAL_INT(0, delete_recording_metadata(rows[0].id));
    TEST_ASSERT_EQUAL_INT(0, query_count("SELECT COUNT(*) FROM recordings;"));
    TEST_ASSERT_EQUAL_INT(-1, access(path, F_OK));
}

int main(void) {
    strcpy(g_root, "/tmp/lightnvr_reconcile_XXXXXX");
    if (!mkdtemp(g_root)) return 1;
    snprintf(g_db_path, sizeof(g_db_path), "%s/test.db", g_root);
    snprintf(g_present_dir, sizeof(g_present_dir), "%s/present", g_root);
    snprintf(g_absent_dir, sizeof(g_absent_dir), "%s/unmounted/cam", g_root);
    if (mkdir(g_present_dir, 0755) != 0) return 1;
    safe_strcpy(g_config.storage_path, g_root, sizeof(g_config.storage_path), 0);
    if (init_database(g_db_path) != 0) return 1;
    storage_target_t target = {0};
    safe_strcpy(target.name, "Reconcile test", sizeof(target.name), 0);
    safe_strcpy(target.root_path, g_present_dir, sizeof(target.root_path), 0);
    strcpy(target.target_type, "filesystem");
    strcpy(target.storage_class, "hot");
    target.enabled = true;
    target.high_watermark_pct = 90;
    target.low_watermark_pct = 80;
    if (db_storage_target_create(&target) != DB_STORAGE_TARGET_OK) return 1;
    safe_strcpy(g_target_uuid, target.uuid, sizeof(g_target_uuid), 0);

    UNITY_BEGIN();
    /* Size every finalized fixture should report. */
    snprintf(g_playable, sizeof(g_playable), "%s/playable.mp4", g_root);
    write_mp4(g_playable, true, true);
    struct stat playable;
    if (stat(g_playable, &playable) != 0) return 1;
    g_playable_size = (long)playable.st_size;

    RUN_TEST(test_full_batches_of_phantom_rows_finish_and_reach_newer_footage);
    RUN_TEST(test_rows_on_an_absent_volume_survive_without_stopping_the_scan);
    RUN_TEST(test_empty_leftover_is_removed_with_its_row);
    RUN_TEST(test_protected_and_unresolvable_target_rows_are_retained);
    RUN_TEST(test_complete_rows_are_out_of_scope);
    RUN_TEST(test_ordinary_deletion_still_refuses_incomplete_rows);
    RUN_TEST(test_failed_media_lookup_keeps_the_file_and_row);
    RUN_TEST(test_nonregular_media_is_retained_without_scheduling_deletion);
    RUN_TEST(test_inconclusive_checks_stay_visible_and_do_not_block_playback);
    RUN_TEST(test_damaged_recordings_keep_protection_and_can_be_deleted);
    RUN_TEST(test_crash_file_without_a_trailer_stays_visible_with_issue);
    RUN_TEST(test_non_mp4_contents_stay_visible_with_issue);
    RUN_TEST(test_header_without_its_video_data_stays_visible_with_issue);
    RUN_TEST(test_zeroed_video_data_stays_visible_with_issue);
    RUN_TEST(test_unplayable_row_is_rechecked_at_the_next_startup);
    RUN_TEST(test_playable_file_without_faststart_is_finalized);
    RUN_TEST(test_available_target_media_is_reconciled);
    RUN_TEST(test_inconsistent_target_identity_cannot_delete_other_footage);
    RUN_TEST(test_empty_file_on_unmounted_target_is_retained);
    int result = UNITY_END();

    shutdown_database();
    char command[PATH_MAX + 16];
    snprintf(command, sizeof(command), "rm -rf %s", g_root);
    if (system(command) != 0) result = 1;
    return result;
}
