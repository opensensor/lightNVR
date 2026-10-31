/* SPDX-License-Identifier: GPL-3.0-or-later
 * Real demux-context/packet/audio ownership across the production pressure
 * branch. Only pressure, segment I/O and sleeps are controlled test boundaries. */
#include "../../../src/video/mp4_writer_thread.c"
#include "unity.h"

static mp4_writer_t writer;
static mp4_writer_thread_t reader;
static AVFormatContext *audio_input;
static AVPacket *pcm, *aac;
static int segments, waits, pause_ticks, closes, audio_resets;
static bool stop_in_pause, no_input, thread_started, thread_completed;

void __real_avformat_close_input(AVFormatContext **);
void __wrap_avformat_close_input(AVFormatContext **input) {
    if(input==&reader.input_ctx && *input)closes++;
    __real_avformat_close_input(input);
}
void __real_cleanup_audio_transcoder(const char *);
void __wrap_cleanup_audio_transcoder(const char *name) {
    if(strcmp(name,writer.stream_name)==0)audio_resets++;
    __real_cleanup_audio_transcoder(name);
}
int __wrap_reserve_mp4_recording_path(char *path,size_t size,storage_placement_t *placement) {
    (void)path;(void)size;(void)placement;return 123;
}
int __wrap_release_mp4_recording_path(int descriptor,const char *path) {
    (void)descriptor;(void)path;return 0;
}
int __wrap_get_stream_config_by_name(const char *name,stream_config_t *config) {
    (void)name;(void)config;return -1;
}
int __wrap_prepare_placed_mp4_recording_path(const config_t *config,const char *name,
    time_t timestamp,char *path,size_t size,storage_placement_t *placement) {
    (void)config;(void)name;(void)timestamp;
    snprintf(path,size,"/nonexistent/t27-pressure.mp4");
    memset(placement,0,sizeof(*placement));return 0;
}
bool __wrap_storage_should_pause_recording(void) {return segments>0 && waits<pause_ticks;}
void __wrap_trigger_storage_cleanup(bool aggressive) {TEST_ASSERT_TRUE(aggressive);}
int __wrap_av_usleep(unsigned usec) {
    TEST_ASSERT_EQUAL_UINT(1000000,usec);
    // The old session must be gone BEFORE the first pressure sleep, and the
    // same continued pause must not repeatedly reset the audio state.
    TEST_ASSERT_NULL(reader.input_ctx);
    TEST_ASSERT_NULL(reader.segment_info.pending_video_keyframe);
    TEST_ASSERT_EQUAL_INT(no_input?0:1,closes);
    TEST_ASSERT_EQUAL_INT(1,audio_resets);
    TEST_ASSERT_EQUAL_INT(0,reader.segment_info.segment_index);
    TEST_ASSERT_FALSE(reader.segment_info.has_audio);
    TEST_ASSERT_FALSE(reader.video_params_detected);
    TEST_ASSERT_EQUAL_STRING(writer.stream_name,reader.segment_info.stream_name);
    waits++;
    if(stop_in_pause && waits==2)atomic_store(&reader.shutdown_requested,1);
    return 0;
}
static void prime_encoder(const char *name) {
    TEST_ASSERT_EQUAL_INT(0,transcode_audio_packet(name,pcm,aac,audio_input->streams[0]));
    AVPacket **tail=NULL;int count=0,padding=0;
    TEST_ASSERT_EQUAL_INT(0,transcode_audio_flush(name,&tail,&count,&padding));
    for(int n=0;n<count;n++)av_packet_free(&tail[n]);av_free(tail);
    TEST_ASSERT_EQUAL_INT(AVERROR_EOF,transcode_audio_packet(name,pcm,aac,audio_input->streams[0]));
}
int __wrap_record_segment(const char *url,const char *path,int duration,int audio,
    AVFormatContext **input,segment_info_t *segment,record_segment_started_cb started,
    record_segment_activity_cb activity,void *opaque,atomic_int *shutdown_flag) {
    (void)url;(void)path;(void)duration;(void)audio;(void)started;(void)activity;(void)opaque;(void)shutdown_flag;
    segments++;
    TEST_ASSERT_LESS_OR_EQUAL_INT(2,segments);
    if(segments==1) {
        if(!no_input){*input=avformat_alloc_context();TEST_ASSERT_NOT_NULL(*input);}
        segment->pending_video_keyframe=av_packet_alloc();
        TEST_ASSERT_NOT_NULL(segment->pending_video_keyframe);
        TEST_ASSERT_EQUAL_INT(0,av_new_packet(segment->pending_video_keyframe,16));
        segment->segment_index=7;segment->has_audio=true;segment->last_frame_was_key=true;
        reader.video_params_detected=true;
        prime_encoder(writer.stream_name);
    } else {
        TEST_ASSERT_EQUAL_INT(pause_ticks,waits);
        TEST_ASSERT_NULL(*input);TEST_ASSERT_NULL(segment->pending_video_keyframe);
        TEST_ASSERT_EQUAL_INT(0,transcode_audio_packet(writer.stream_name,pcm,aac,audio_input->streams[0]));
        TEST_ASSERT_GREATER_THAN_INT(0,aac->size);
        TEST_ASSERT_EQUAL_INT(AVERROR_EOF,transcode_audio_packet("other-pressure-camera",pcm,aac,audio_input->streams[0]));
        reader.running=0;
    }
    return 0;
}
void setUp(void) {
    g_config.max_streams=MAX_STREAMS;g_config.audio_disabled=false;
    memset(&writer,0,sizeof(writer));memset(&reader,0,sizeof(reader));
    strcpy(writer.stream_name,"pressure-reconnect");strcpy(writer.output_path,"/nonexistent/t27-pressure.mp4");
    writer.segment_duration=3;writer.last_rotation_time=time(NULL);writer.thread_ctx=&reader;writer.has_audio=1;
    reader.writer=&writer;strcpy(reader.rtsp_url,"rtsp://unused/pressure");
    audio_input=avformat_alloc_context();TEST_ASSERT_NOT_NULL(audio_input);
    AVStream *stream=avformat_new_stream(audio_input,NULL);TEST_ASSERT_NOT_NULL(stream);
    stream->time_base=(AVRational){1,8000};stream->codecpar->codec_type=AVMEDIA_TYPE_AUDIO;
    stream->codecpar->codec_id=AV_CODEC_ID_PCM_ALAW;stream->codecpar->sample_rate=8000;
    av_channel_layout_default(&stream->codecpar->ch_layout,1);
    pcm=av_packet_alloc();aac=av_packet_alloc();TEST_ASSERT_NOT_NULL(pcm);TEST_ASSERT_NOT_NULL(aac);
    TEST_ASSERT_EQUAL_INT(0,av_new_packet(pcm,8192));memset(pcm->data,0xd5,pcm->size);pcm->pts=pcm->dts=0;
    segments=waits=closes=audio_resets=0;pause_ticks=2;stop_in_pause=no_input=false;
    thread_started=thread_completed=false;
    prime_encoder("other-pressure-camera");
}
void tearDown(void) {
    avformat_close_input(&reader.input_ctx);av_packet_free(&reader.segment_info.pending_video_keyframe);
    avformat_free_context(audio_input);av_packet_free(&pcm);av_packet_free(&aac);
    cleanup_audio_transcoder(writer.stream_name);cleanup_audio_transcoder("other-pressure-camera");
    if(thread_started && !thread_completed)pthread_mutex_destroy(&reader.context_mutex);
}
static void run_reader(void) {
    thread_started=true;mp4_writer_rtsp_thread(&reader);thread_completed=true;
}
static void test_pressure_discards_old_session_before_sleep_and_resumes_audio(void) {
    run_reader();TEST_ASSERT_EQUAL_INT(2,segments);TEST_ASSERT_EQUAL_INT(1,closes);
}
static void test_long_pause_resets_the_session_only_once(void) {
    pause_ticks=35;run_reader();
    TEST_ASSERT_EQUAL_INT(35,waits);TEST_ASSERT_EQUAL_INT(1,audio_resets);TEST_ASSERT_EQUAL_INT(1,closes);
}
static void test_shutdown_during_pause_never_reopens_the_source(void) {
    stop_in_pause=true;run_reader();
    TEST_ASSERT_EQUAL_INT(1,segments);TEST_ASSERT_EQUAL_INT(2,waits);TEST_ASSERT_EQUAL_INT(1,closes);
}
static void test_audio_and_pending_state_reset_even_without_input_context(void) {
    no_input=true;run_reader();
    TEST_ASSERT_EQUAL_INT(2,segments);TEST_ASSERT_EQUAL_INT(0,closes);TEST_ASSERT_EQUAL_INT(1,audio_resets);
}
int main(void) {
    UNITY_BEGIN();
    RUN_TEST(test_pressure_discards_old_session_before_sleep_and_resumes_audio);
    RUN_TEST(test_long_pause_resets_the_session_only_once);
    RUN_TEST(test_shutdown_during_pause_never_reopens_the_source);
    RUN_TEST(test_audio_and_pending_state_reset_even_without_input_context);
    return UNITY_END();
}
