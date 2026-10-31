#define _POSIX_C_SOURCE 200809L
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>
#include <unistd.h>
#include <sys/stat.h>
#include <libavcodec/avcodec.h>
#include <libavformat/avformat.h>
#include "unity.h"
#include "core/config.h"
#include "core/logger.h"
#include "database/db_core.h"
#include "database/db_recordings.h"
#include "storage/storage_manager.h"
#include "video/mp4_writer.h"

static char directory[] = "/tmp/lightnvr-incomplete-media-XXXXXX";
static char database_path[256], valid_path[256], invalid_path[256], header_only_path[256];
static time_t now;

static void write_packets(AVCodecContext *encoder, AVFormatContext *output, AVPacket *packet) {
    int result;
    while ((result = avcodec_receive_packet(encoder, packet)) >= 0) {
        av_packet_rescale_ts(packet, encoder->time_base, output->streams[0]->time_base);
        packet->stream_index = 0;
        TEST_ASSERT_EQUAL_INT(0, av_interleaved_write_frame(output, packet));
        av_packet_unref(packet);
    }
    TEST_ASSERT_TRUE(result == AVERROR(EAGAIN) || result == AVERROR_EOF);
}
static void create_valid_media(void) {
    const AVCodec *codec = avcodec_find_encoder(AV_CODEC_ID_MPEG4);
    TEST_ASSERT_NOT_NULL(codec);
    AVCodecContext *encoder = avcodec_alloc_context3(codec);
    AVFormatContext *output = NULL;
    AVFrame *frame = av_frame_alloc();
    AVPacket *packet = av_packet_alloc();
    TEST_ASSERT_NOT_NULL(encoder); TEST_ASSERT_NOT_NULL(frame); TEST_ASSERT_NOT_NULL(packet);
    encoder->width = encoder->height = 32;
    encoder->pix_fmt = AV_PIX_FMT_YUV420P;
    encoder->time_base = (AVRational){1, 5};
    encoder->gop_size = 5;
    encoder->max_b_frames = 0;
    encoder->flags |= AV_CODEC_FLAG_GLOBAL_HEADER;
    TEST_ASSERT_EQUAL_INT(0, avcodec_open2(encoder, codec, NULL));
    TEST_ASSERT_EQUAL_INT(0, avformat_alloc_output_context2(&output, NULL, "mp4", valid_path));
    AVStream *stream = avformat_new_stream(output, NULL);
    TEST_ASSERT_NOT_NULL(stream);
    stream->time_base = encoder->time_base;
    TEST_ASSERT_EQUAL_INT(0, avcodec_parameters_from_context(stream->codecpar, encoder));
    TEST_ASSERT_EQUAL_INT(0, avio_open(&output->pb, valid_path, AVIO_FLAG_WRITE));
    AVDictionary *options = NULL;
    av_dict_set(&options, "movflags", "+faststart", 0);
    TEST_ASSERT_EQUAL_INT(0, avformat_write_header(output, &options));
    av_dict_free(&options);
    frame->format = encoder->pix_fmt; frame->width = frame->height = 32;
    TEST_ASSERT_EQUAL_INT(0, av_frame_get_buffer(frame, 0));
    for (int i = 0; i < 10; i++) {
        TEST_ASSERT_EQUAL_INT(0, av_frame_make_writable(frame));
        for (int y = 0; y < 32; y++) memset(frame->data[0] + y * frame->linesize[0], 80, 32);
        for (int plane = 1; plane < 3; plane++)
            for (int y = 0; y < 16; y++) memset(frame->data[plane] + y * frame->linesize[plane], 128, 16);
        frame->pts = i;
        TEST_ASSERT_EQUAL_INT(0, avcodec_send_frame(encoder, frame));
        write_packets(encoder, output, packet);
    }
    TEST_ASSERT_EQUAL_INT(0, avcodec_send_frame(encoder, NULL));
    write_packets(encoder, output, packet);
    TEST_ASSERT_EQUAL_INT(0, av_write_trailer(output));
    avio_closep(&output->pb); avformat_free_context(output);
    avcodec_free_context(&encoder); av_frame_free(&frame); av_packet_free(&packet);
}
static uint64_t add_incomplete(const char *path, time_t start) {
    recording_metadata_t record = {0};
    snprintf(record.stream_name, sizeof(record.stream_name), "recovery_camera");
    snprintf(record.file_path, sizeof(record.file_path), "%s", path);
    strcpy(record.codec, "mpeg4"); strcpy(record.trigger_type, "scheduled");
    record.start_time = start; record.width = record.height = 32; record.fps = 5;
    record.retention_override_days = -1; record.retention_tier = RETENTION_TIER_STANDARD;
    record.disk_pressure_eligible = true; record.schedule_restricted = -1;
    uint64_t id = add_recording_metadata(&record);
    TEST_ASSERT_NOT_EQUAL(0, id);
    return id;
}
static recording_metadata_t read_record(uint64_t id) {
    recording_metadata_t result = {0};
    TEST_ASSERT_EQUAL_INT(0, get_recording_metadata_by_id(id, &result));
    return result;
}
void setUp(void) {
    TEST_ASSERT_EQUAL_INT(SQLITE_OK, sqlite3_exec(get_db_handle(), "DELETE FROM recordings", NULL, NULL, NULL));
    now = time(NULL);
}
void tearDown(void) {}
void test_valid_stale_media_is_finalized(void) {
    uint64_t id = add_incomplete(valid_path, now - 7200);
    TEST_ASSERT_EQUAL_INT(1, storage_reconcile_incomplete_recordings());
    recording_metadata_t record = read_record(id);
    TEST_ASSERT_TRUE(record.is_complete);
    TEST_ASSERT_GREATER_THAN(0, record.size_bytes);
    TEST_ASSERT_TRUE(get_mp4_file_duration_seconds(valid_path) > 0);
}
void test_truncated_nonempty_media_stays_incomplete(void) {
    uint64_t id = add_incomplete(invalid_path, now - 7200);
    TEST_ASSERT_EQUAL_INT(0, storage_reconcile_incomplete_recordings());
    recording_metadata_t record = read_record(id);
    TEST_ASSERT_FALSE(record.is_complete);
    struct stat info;
    TEST_ASSERT_EQUAL_INT(0, stat(invalid_path, &info));
    TEST_ASSERT_EQUAL_INT(40, info.st_size);
}
void test_faststart_header_without_video_payload_stays_incomplete(void) {
    // Container metadata alone still advertises a positive duration.
    TEST_ASSERT_TRUE(get_mp4_file_duration_seconds(header_only_path) > 0);
    uint64_t id = add_incomplete(header_only_path, now - 7200);
    TEST_ASSERT_EQUAL_INT(0, storage_reconcile_incomplete_recordings());
    TEST_ASSERT_FALSE(read_record(id).is_complete);
    TEST_ASSERT_EQUAL_INT(0, access(header_only_path, F_OK));
}
void test_recent_media_remains_incomplete(void) {
    uint64_t id = add_incomplete(valid_path, now);
    TEST_ASSERT_EQUAL_INT(0, storage_reconcile_incomplete_recordings());
    TEST_ASSERT_FALSE(read_record(id).is_complete);
}
void test_scan_advances_past_500_retained_invalid_files(void) {
    char filename[256]; uint64_t first = 0;
    TEST_ASSERT_EQUAL_INT(SQLITE_OK, sqlite3_exec(get_db_handle(), "BEGIN", NULL, NULL, NULL));
    for (int i = 0; i < 500; i++) {
        snprintf(filename, sizeof(filename), "%s/invalid-%d.mp4", directory, i);
        TEST_ASSERT_EQUAL_INT(0, link(invalid_path, filename));
        uint64_t id = add_incomplete(filename, now - 7200);
        if (!i) first = id;
    }
    uint64_t last = add_incomplete(valid_path, now - 7200);
    TEST_ASSERT_EQUAL_INT(SQLITE_OK, sqlite3_exec(get_db_handle(), "COMMIT", NULL, NULL, NULL));
    TEST_ASSERT_EQUAL_INT(1, storage_reconcile_incomplete_recordings());
    TEST_ASSERT_FALSE(read_record(first).is_complete);
    TEST_ASSERT_TRUE(read_record(last).is_complete);
    for (int i = 0; i < 500; i++) {
        snprintf(filename, sizeof(filename), "%s/invalid-%d.mp4", directory, i);
        TEST_ASSERT_EQUAL_INT(0, access(filename, F_OK));
        unlink(filename);
    }
}
int main(void) {
    init_logger(); load_default_config(&g_config);
    if (!mkdtemp(directory)) return 1;
    snprintf(database_path, sizeof(database_path), "%s/catalog.db", directory);
    snprintf(valid_path, sizeof(valid_path), "%s/valid.mp4", directory);
    snprintf(invalid_path, sizeof(invalid_path), "%s/truncated.mp4", directory);
    snprintf(header_only_path, sizeof(header_only_path), "%s/header-only.mp4", directory);
    snprintf(g_config.storage_path, sizeof(g_config.storage_path), "%s", directory);
    if (init_database_ex(database_path, DB_INIT_NO_CHECK | DB_INIT_NO_BACKUP)) return 1;
    UNITY_BEGIN();
    create_valid_media();
    FILE *source = fopen(valid_path, "rb"), *target = fopen(invalid_path, "wb");
    char prefix[40];
    if (!source || !target || fread(prefix, 1, sizeof(prefix), source) != sizeof(prefix) ||
        fwrite(prefix, 1, sizeof(prefix), target) != sizeof(prefix)) return 1;
    fclose(source); fclose(target);
    source = fopen(valid_path, "rb"); target = fopen(header_only_path, "wb");
    if (!source || !target) return 1;
    // Copy complete top-level atoms through the mdat header, omitting payload.
    unsigned char atom[8]; bool found_payload = false;
    while (fread(atom, 1, 8, source) == 8) {
        unsigned size = ((unsigned)atom[0] << 24) | ((unsigned)atom[1] << 16) |
                        ((unsigned)atom[2] << 8) | atom[3];
        if (size < 8 || fwrite(atom, 1, 8, target) != 8) return 1;
        if (memcmp(atom + 4, "mdat", 4) == 0) { found_payload = true; break; }
        for (unsigned i = 8; i < size; i++) { int byte = fgetc(source); if (byte == EOF || fputc(byte, target) == EOF) return 1; }
    }
    fclose(source); fclose(target);
    if (!found_payload) return 1;
    RUN_TEST(test_valid_stale_media_is_finalized);
    RUN_TEST(test_truncated_nonempty_media_stays_incomplete);
    RUN_TEST(test_faststart_header_without_video_payload_stays_incomplete);
    RUN_TEST(test_recent_media_remains_incomplete);
    RUN_TEST(test_scan_advances_past_500_retained_invalid_files);
    int result = UNITY_END();
    shutdown_database();
    unlink(valid_path); unlink(invalid_path); unlink(header_only_path); unlink(database_path);
    char filename[256];
    for (int i = 0; i < 500; i++) { snprintf(filename, sizeof(filename), "%s/invalid-%d.mp4", directory, i); unlink(filename); }
    rmdir(directory);
    return result;
}
