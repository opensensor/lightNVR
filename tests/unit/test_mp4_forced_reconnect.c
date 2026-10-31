/* Exercise the actual reader loop with deterministic segment/clock boundaries.
 * Audio transcoding, input cleanup, reconnect signaling and state resets are real. */
#include "../../src/video/mp4_writer_thread.c"
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
static AVFormatContext *audio_input;
static AVPacket *pcm, *aac;
static int segment_calls, reconnect_waits;
static bool reconnect_before_first_segment;

static void prime_flushed_encoder(const char *name) {
    TEST_ASSERT_EQUAL_INT(0, transcode_audio_packet(name, pcm, aac, audio_input->streams[0]));
    AVPacket **tail = NULL;
    int count = 0, padding = 0;
    TEST_ASSERT_EQUAL_INT(0, transcode_audio_flush(name, &tail, &count, &padding));
    for (int i = 0; i < count; i++) av_packet_free(&tail[i]);
    av_free(tail);
    TEST_ASSERT_EQUAL_INT(AVERROR_EOF,
        transcode_audio_packet(name, pcm, aac, audio_input->streams[0]));
}

int __wrap_av_usleep(unsigned usec) {
    TEST_ASSERT_EQUAL_UINT(500000, usec);
    TEST_ASSERT_NULL(reader.input_ctx);
    TEST_ASSERT_NULL(reader.segment_info.pending_video_keyframe);
    TEST_ASSERT_EQUAL_INT(0, reader.retry_count);
    TEST_ASSERT_FALSE(reader.video_params_detected);
    reconnect_waits++;
    TEST_ASSERT_LESS_OR_EQUAL_INT(6, reconnect_waits);
    return 0;
}

int __wrap_get_stream_config_by_name(const char *name, stream_config_t *config) {
    (void)name; (void)config;
    return -1; // isolate the loop from the configuration database
}

bool __wrap_storage_should_pause_recording(void) { return false; }

int __wrap_prepare_placed_mp4_recording_path(const config_t *config,
    const char *name, time_t timestamp, char *path, size_t size,
    storage_placement_t *placement) {
    (void)config; (void)name; (void)timestamp;
    snprintf(path, size, "/tmp/lightnvr-reconnect-test.mp4");
    memset(placement, 0, sizeof(*placement));
    return 0;
}

int __wrap_record_segment(const char *url, const char *path, int duration, int audio,
    AVFormatContext **input, segment_info_t *segment,
    record_segment_started_cb started, record_segment_activity_cb activity,
    void *opaque, atomic_int *shutdown_flag) {
    (void)url; (void)path; (void)duration; (void)audio; (void)started;
    (void)activity; (void)opaque; (void)shutdown_flag;
    segment_calls++;
    TEST_ASSERT_LESS_OR_EQUAL_INT(2, segment_calls);
    if (segment_calls == 1 && !reconnect_before_first_segment) {
        // Leave a real stale demuxer, carried keyframe and flushed AAC slot.
        *input = avformat_alloc_context();
        TEST_ASSERT_NOT_NULL(*input);
        segment->pending_video_keyframe = av_packet_alloc();
        TEST_ASSERT_NOT_NULL(segment->pending_video_keyframe);
        TEST_ASSERT_EQUAL_INT(0, av_new_packet(segment->pending_video_keyframe, 16));
        reader.retry_count = 4;
        reader.video_params_detected = true;
        prime_flushed_encoder(writer.stream_name);
        mp4_writer_signal_reconnect(&writer);
        TEST_ASSERT_EQUAL_INT(1, atomic_load(&reader.force_reconnect));
    } else {
        TEST_ASSERT_EQUAL_INT(6, reconnect_waits);
        TEST_ASSERT_NULL(*input);
        TEST_ASSERT_NULL(segment->pending_video_keyframe);
        TEST_ASSERT_EQUAL_INT(0, atomic_load(&reader.force_reconnect));
        // No synthetic cleanup here: the production reconnect branch must
        // make the previously EOF-only slot accept PCM and emit AAC again.
        TEST_ASSERT_EQUAL_INT(0, transcode_audio_packet(writer.stream_name,
            pcm, aac, audio_input->streams[0]));
        TEST_ASSERT_GREATER_THAN_INT(0, aac->size);
        TEST_ASSERT_EQUAL_INT(AVERROR_EOF, transcode_audio_packet("other-camera",
            pcm, aac, audio_input->streams[0]));
        reader.running = 0;
    }
    return 0;
}

void setUp(void) {
    g_config.max_streams = MAX_STREAMS;
    g_config.audio_disabled = false;
    memset(&writer, 0, sizeof(writer));
    memset(&reader, 0, sizeof(reader));
    strcpy(writer.stream_name, "forced-reconnect");
    strcpy(writer.output_path, "/tmp/lightnvr-reconnect-test.mp4");
    writer.segment_duration = 30;
    writer.last_rotation_time = time(NULL);
    writer.has_audio = 1;
    writer.thread_ctx = &reader;
    reader.writer = &writer;
    strcpy(reader.rtsp_url, "rtsp://unused/test");
    audio_input = avformat_alloc_context();
    TEST_ASSERT_NOT_NULL(audio_input);
    AVStream *stream = avformat_new_stream(audio_input, NULL);
    TEST_ASSERT_NOT_NULL(stream);
    stream->time_base = (AVRational){1, 8000};
    stream->codecpar->codec_type = AVMEDIA_TYPE_AUDIO;
    stream->codecpar->codec_id = AV_CODEC_ID_PCM_ALAW;
    stream->codecpar->sample_rate = 8000;
    av_channel_layout_default(&stream->codecpar->ch_layout, 1);
    pcm = av_packet_alloc();
    aac = av_packet_alloc();
    TEST_ASSERT_NOT_NULL(pcm);
    TEST_ASSERT_NOT_NULL(aac);
    TEST_ASSERT_EQUAL_INT(0, av_new_packet(pcm, 8192));
    memset(pcm->data, 0xd5, pcm->size);
    pcm->pts = pcm->dts = 0;
    segment_calls = reconnect_waits = 0;
    reconnect_before_first_segment = false;
    prime_flushed_encoder("other-camera");
}

void tearDown(void) {
    avformat_close_input(&reader.input_ctx);
    av_packet_free(&reader.segment_info.pending_video_keyframe);
    avformat_free_context(audio_input);
    av_packet_free(&pcm);
    av_packet_free(&aac);
    cleanup_audio_transcoder(writer.stream_name);
    cleanup_audio_transcoder("other-camera");
}

static void test_reconnect_discards_old_session_and_resumes_audio(void) {
    mp4_writer_rtsp_thread(&reader);
    TEST_ASSERT_EQUAL_INT(2, segment_calls);
    TEST_ASSERT_EQUAL_INT(6, reconnect_waits);
}

static void test_reconnect_without_demuxer_still_resets_flushed_audio(void) {
    reconnect_before_first_segment = true;
    prime_flushed_encoder(writer.stream_name);
    atomic_store(&reader.force_reconnect, 1);
    mp4_writer_rtsp_thread(&reader);
    TEST_ASSERT_EQUAL_INT(1, segment_calls);
    TEST_ASSERT_EQUAL_INT(6, reconnect_waits);
}

int main(void) {
    UNITY_BEGIN();
    RUN_TEST(test_reconnect_discards_old_session_and_resumes_audio);
    RUN_TEST(test_reconnect_without_demuxer_still_resets_flushed_audio);
    return UNITY_END();
}
