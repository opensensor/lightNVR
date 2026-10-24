#define _POSIX_C_SOURCE 200809L
#define _GNU_SOURCE

#include <dirent.h>
#include <fcntl.h>
#include <pthread.h>
#include <signal.h>
#include <stdatomic.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/stat.h>
#include <time.h>
#include <unistd.h>

#include <math.h>

#include <libavcodec/avcodec.h>
#include <libavformat/avformat.h>
#include <libavutil/channel_layout.h>

#include "unity.h"
#include "core/config.h"
#include "core/logger.h"
#include "core/path_utils.h"
#include "core/shutdown_coordinator.h"
#include "video/hls_writer.h"
#include "video/hls/hls_unified_thread.h"
#include "video/hls/hls_directory.h"
#include "video/stream_manager.h"
#include "video/stream_state.h"

static atomic_int alarm_calls;
static atomic_int disposition_changes;
static char test_dir[128];
static pthread_barrier_t close_barrier;
static atomic_bool pace_input;

int __real_av_read_frame(AVFormatContext *input, AVPacket *packet);

int __wrap_av_read_frame(AVFormatContext *input, AVPacket *packet) {
    // Keep the local media source open long enough to stop a running thread,
    // without requiring an external RTSP server in the unit-test suite.
    if (atomic_load(&pace_input) && input->url &&
        strcmp(input->url, TEST_RECORDING_PATH) == 0) usleep(10000);
    return __real_av_read_frame(input, packet);
}

int __real_sigaction(int sig, const struct sigaction *action,
                     struct sigaction *previous);

int __wrap_sigaction(int sig, const struct sigaction *action,
                     struct sigaction *previous) {
    if (action && (sig == SIGALRM || sig == SIGSEGV))
        atomic_fetch_add(&disposition_changes, 1);
    return __real_sigaction(sig, action, previous);
}

unsigned int __wrap_alarm(unsigned int seconds) {
    (void)seconds;
    atomic_fetch_add(&alarm_calls, 1);
    // Do not let a regression terminate the test runner with SIGALRM.
    return 0;
}

static void remove_test_files(const char *path) {
    DIR *dir = opendir(path);
    if (!dir) return;
    struct dirent *entry;
    while ((entry = readdir(dir))) {
        if (!strcmp(entry->d_name, ".") || !strcmp(entry->d_name, "..")) continue;
        char child[512];
        snprintf(child, sizeof(child), "%s/%s", path, entry->d_name);
        if (unlink(child) != 0) remove_test_files(child);
    }
    closedir(dir);
    rmdir(path);
}

void setUp(void) {
    strcpy(test_dir, "/tmp/lightnvr_hls_cleanup_XXXXXX");
    TEST_ASSERT_NOT_NULL(mkdtemp(test_dir));
    load_default_config(&g_config);
    snprintf(g_config.storage_path, sizeof(g_config.storage_path), "%s", test_dir);
    g_config.max_streams = 4;
    init_shutdown_coordinator();
    atomic_store(&alarm_calls, 0);
    atomic_store(&disposition_changes, 0);
}

void tearDown(void) {
    cleanup_all_hls_writers();
    remove_test_files(test_dir);
}

static void *close_writer(void *arg) {
    pthread_barrier_wait(&close_barrier);
    hls_writer_close(arg);
    return NULL;
}

void test_cache_maintenance_uses_dedicated_path_then_default_path(void) {
    char default_dir[256], dedicated_dir[256], default_file[512], dedicated_file[512];
    snprintf(default_dir, sizeof(default_dir), "%s/hls/camera", test_dir);
    snprintf(g_config.storage_path_hls, sizeof(g_config.storage_path_hls),
             "%s/playback", test_dir);
    snprintf(dedicated_dir, sizeof(dedicated_dir), "%s/hls/camera", g_config.storage_path_hls);
    TEST_ASSERT_EQUAL_INT(0, mkdir_recursive(default_dir));
    TEST_ASSERT_EQUAL_INT(0, mkdir_recursive(dedicated_dir));
    snprintf(default_file, sizeof(default_file), "%s/old.ts", default_dir);
    snprintf(dedicated_file, sizeof(dedicated_file), "%s/old.ts", dedicated_dir);
    struct timespec old[2] = {{time(NULL) - 600, 0}, {time(NULL) - 600, 0}};
    const char *files[] = {default_file, dedicated_file};
    for (size_t i = 0; i < sizeof(files) / sizeof(files[0]); ++i) {
        FILE *file = fopen(files[i], "w");
        TEST_ASSERT_NOT_NULL(file);
        TEST_ASSERT_EQUAL_INT(0, fclose(file));
        TEST_ASSERT_EQUAL_INT(0, utimensat(AT_FDCWD, files[i], old, 0));
    }
    cleanup_stale_hls_segments();
    TEST_ASSERT_EQUAL_INT(0, access(default_file, F_OK));
    TEST_ASSERT_EQUAL_INT(-1, access(dedicated_file, F_OK));
    g_config.storage_path_hls[0] = '\0';
    cleanup_stale_hls_segments();
    TEST_ASSERT_EQUAL_INT(-1, access(default_file, F_OK));
}

void test_writer_remaps_input_video_after_audio_to_output_stream_zero(void) {
    AVFormatContext *fixture = NULL;
    TEST_ASSERT_EQUAL_INT(0, avformat_open_input(
        &fixture, TEST_RECORDING_PATH, NULL, NULL));
    TEST_ASSERT_GREATER_OR_EQUAL(0, avformat_find_stream_info(fixture, NULL));

    /* RTSP sources can advertise audio before video. The native HLS writer
     * emits video only, so input stream 1 must become output stream 0. */
    AVFormatContext *input = avformat_alloc_context();
    TEST_ASSERT_NOT_NULL(input);
    AVStream *audio = avformat_new_stream(input, NULL);
    AVStream *video = avformat_new_stream(input, NULL);
    TEST_ASSERT_NOT_NULL(audio);
    TEST_ASSERT_NOT_NULL(video);
    audio->codecpar->codec_type = AVMEDIA_TYPE_AUDIO;
    TEST_ASSERT_EQUAL_INT(0, avcodec_parameters_copy(
        video->codecpar, fixture->streams[0]->codecpar));
    video->time_base = fixture->streams[0]->time_base;
    TEST_ASSERT_EQUAL_INT(1, video->index);

    hls_writer_t *writer = hls_writer_create(test_dir, "audio-first", 2);
    TEST_ASSERT_NOT_NULL(writer);
    TEST_ASSERT_EQUAL_INT(0, hls_writer_initialize(writer, video));
    TEST_ASSERT_EQUAL_INT(1, writer->output_ctx->nb_streams);
    AVPacket *packet = av_packet_alloc();
    TEST_ASSERT_NOT_NULL(packet);
    for (int i = 0; i < 8; ++i) {
        TEST_ASSERT_EQUAL_INT(0, av_read_frame(fixture, packet));
        packet->stream_index = video->index;
        TEST_ASSERT_GREATER_OR_EQUAL(0, hls_writer_write_packet(writer, packet, video));
        TEST_ASSERT_EQUAL_INT(1, packet->stream_index);
        av_packet_unref(packet);
    }

    char playlist[MAX_PATH_LENGTH + 32];
    snprintf(playlist, sizeof(playlist), "%s/index.m3u8", writer->output_dir);
    hls_writer_close(writer);
    AVFormatContext *recorded = NULL;
    TEST_ASSERT_EQUAL_INT(0, avformat_open_input(&recorded, playlist, NULL, NULL));
    TEST_ASSERT_GREATER_OR_EQUAL(0, avformat_find_stream_info(recorded, NULL));
    TEST_ASSERT_EQUAL_INT(AVMEDIA_TYPE_VIDEO, recorded->streams[0]->codecpar->codec_type);
    TEST_ASSERT_EQUAL_INT(AV_CODEC_ID_H264, recorded->streams[0]->codecpar->codec_id);
    avformat_close_input(&recorded);
    av_packet_free(&packet);
    avformat_free_context(input);
    avformat_close_input(&fixture);
}

void test_parallel_writer_teardown_and_recreation_preserves_signals(void) {
    enum { CAMERAS = 3, CYCLES = 3 };
    for (int cycle = 0; cycle < CYCLES; ++cycle) {
        hls_writer_t *writers[CAMERAS];
        pthread_t threads[CAMERAS];
        char names[CAMERAS][32];
        char playlists[CAMERAS][MAX_PATH_LENGTH + 32];
        for (int camera = 0; camera < CAMERAS; ++camera) {
            snprintf(names[camera], sizeof(names[camera]), "cleanup-%d", camera);
            writers[camera] = hls_writer_create(test_dir, names[camera], 2);
            TEST_ASSERT_NOT_NULL(writers[camera]);
            AVFormatContext *input = NULL;
            TEST_ASSERT_EQUAL_INT(0, avformat_open_input(
                &input, TEST_RECORDING_PATH, NULL, NULL));
            TEST_ASSERT_GREATER_OR_EQUAL(0, avformat_find_stream_info(input, NULL));
            TEST_ASSERT_EQUAL_INT(0, hls_writer_initialize(writers[camera], input->streams[0]));
            AVPacket *packet = av_packet_alloc();
            TEST_ASSERT_NOT_NULL(packet);
            for (int i = 0; i < 8; ++i) {
                TEST_ASSERT_EQUAL_INT(0, av_read_frame(input, packet));
                TEST_ASSERT_GREATER_OR_EQUAL(0, hls_writer_write_packet(
                    writers[camera], packet, input->streams[0]));
                av_packet_unref(packet);
            }
            av_packet_free(&packet);
            avformat_close_input(&input);
            snprintf(playlists[camera], sizeof(playlists[camera]), "%s/index.m3u8",
                     writers[camera]->output_dir);
        }
        TEST_ASSERT_EQUAL_INT(0, pthread_barrier_init(&close_barrier, NULL, CAMERAS));
        for (int camera = 0; camera < CAMERAS; ++camera)
            TEST_ASSERT_EQUAL_INT(0, pthread_create(&threads[camera], NULL,
                                                   close_writer, writers[camera]));
        for (int camera = 0; camera < CAMERAS; ++camera) {
            TEST_ASSERT_EQUAL_INT(0, pthread_join(threads[camera], NULL));
            TEST_ASSERT_NULL(find_hls_writer_by_stream_name(names[camera]));
            AVFormatContext *recorded = NULL;
            TEST_ASSERT_EQUAL_INT(0, avformat_open_input(
                &recorded, playlists[camera], NULL, NULL));
            AVPacket *packet = av_packet_alloc();
            TEST_ASSERT_NOT_NULL(packet);
            TEST_ASSERT_EQUAL_INT(0, av_read_frame(recorded, packet));
            av_packet_free(&packet);
            avformat_close_input(&recorded);
        }
        pthread_barrier_destroy(&close_barrier);
    }
    TEST_ASSERT_EQUAL_INT(0, atomic_load(&alarm_calls));
    TEST_ASSERT_EQUAL_INT(0, atomic_load(&disposition_changes));
}

void test_unified_system_cleanup_preserves_signals(void) {
    cleanup_hls_unified_thread_system();
    TEST_ASSERT_EQUAL_INT(0, atomic_load(&alarm_calls));
    TEST_ASSERT_EQUAL_INT(0, atomic_load(&disposition_changes));
}

typedef struct {
    char name[32];
    bool restart;
    int result;
} lifecycle_task_t;

static void *change_stream_lifecycle(void *arg) {
    lifecycle_task_t *task = arg;
    pthread_barrier_wait(&close_barrier);
    task->result = task->restart ? restart_hls_unified_stream(task->name)
                                 : stop_hls_unified_stream(task->name);
    return NULL;
}

void test_parallel_unified_stream_shutdown_and_restart(void) {
    enum { CAMERAS = 3 };
    lifecycle_task_t tasks[CAMERAS] = {0};
    pthread_t threads[CAMERAS];
    TEST_ASSERT_EQUAL_INT(0, init_stream_state_manager(4));
    TEST_ASSERT_EQUAL_INT(0, init_stream_manager(4));
    atomic_store(&pace_input, true);
    for (int camera = 0; camera < CAMERAS; ++camera) {
        snprintf(tasks[camera].name, sizeof(tasks[camera].name), "lifecycle-%d", camera);
        stream_config_t config = {0};
        snprintf(config.name, sizeof(config.name), "%s", tasks[camera].name);
        snprintf(config.url, sizeof(config.url), "%s", TEST_RECORDING_PATH);
        config.enabled = true;
        config.streaming_enabled = true;
        config.segment_duration = 2;
        config.protocol = STREAM_PROTOCOL_TCP;
        TEST_ASSERT_NOT_NULL(add_stream(&config));
        TEST_ASSERT_EQUAL_INT(0, start_hls_unified_stream(config.name));
    }
    for (int cycle = 0; cycle < 3; ++cycle) {
        for (int camera = 0; camera < CAMERAS; ++camera) {
            int tries = 0;
            while (!is_hls_stream_active(tasks[camera].name) && tries++ < 100)
                usleep(50000);
            TEST_ASSERT_TRUE(is_hls_stream_active(tasks[camera].name));
            tasks[camera].restart = cycle < 2;
        }
        TEST_ASSERT_EQUAL_INT(0, pthread_barrier_init(&close_barrier, NULL, CAMERAS));
        for (int camera = 0; camera < CAMERAS; ++camera)
            TEST_ASSERT_EQUAL_INT(0, pthread_create(&threads[camera], NULL,
                change_stream_lifecycle, &tasks[camera]));
        for (int camera = 0; camera < CAMERAS; ++camera) {
            TEST_ASSERT_EQUAL_INT(0, pthread_join(threads[camera], NULL));
            TEST_ASSERT_EQUAL_INT(0, tasks[camera].result);
        }
        pthread_barrier_destroy(&close_barrier);
    }
    for (int camera = 0; camera < CAMERAS; ++camera) {
        TEST_ASSERT_FALSE(is_hls_stream_active(tasks[camera].name));
        TEST_ASSERT_NULL(find_hls_writer_by_stream_name(tasks[camera].name));
    }
    atomic_store(&pace_input, false);
    shutdown_stream_manager();
    shutdown_stream_state_manager();
    TEST_ASSERT_EQUAL_INT(0, atomic_load(&alarm_calls));
    TEST_ASSERT_EQUAL_INT(0, atomic_load(&disposition_changes));
}

typedef struct {
    AVCodecParameters *params;
    AVRational time_base;
    AVPacket **packets;
    int count;
} audio_fixture_t;

static void audio_fixture_append(audio_fixture_t *fixture, const AVPacket *packet) {
    fixture->packets = realloc(fixture->packets, sizeof(AVPacket *) * (fixture->count + 1));
    TEST_ASSERT_NOT_NULL(fixture->packets);
    fixture->packets[fixture->count] = av_packet_clone(packet);
    TEST_ASSERT_NOT_NULL(fixture->packets[fixture->count]);
    fixture->count++;
}

static void audio_fixture_free(audio_fixture_t *fixture) {
    for (int i = 0; i < fixture->count; ++i) av_packet_free(&fixture->packets[i]);
    free(fixture->packets);
    avcodec_parameters_free(&fixture->params);
}

/* Encode a sine tone with FFmpeg's native AAC encoder, as an RTSP AAC track
 * would deliver it: raw access units plus an AudioSpecificConfig. */
static void make_aac_fixture(audio_fixture_t *fixture, int seconds) {
    const AVCodec *codec = avcodec_find_encoder(AV_CODEC_ID_AAC);
    TEST_ASSERT_NOT_NULL(codec);
    AVCodecContext *encoder = avcodec_alloc_context3(codec);
    TEST_ASSERT_NOT_NULL(encoder);
    encoder->sample_rate = 16000;
    encoder->sample_fmt = AV_SAMPLE_FMT_FLTP;
    encoder->bit_rate = 32000;
    encoder->time_base = (AVRational){1, encoder->sample_rate};
    encoder->flags |= AV_CODEC_FLAG_GLOBAL_HEADER;
    av_channel_layout_default(&encoder->ch_layout, 1);
    TEST_ASSERT_EQUAL_INT(0, avcodec_open2(encoder, codec, NULL));

    AVFrame *frame = av_frame_alloc();
    AVPacket *packet = av_packet_alloc();
    TEST_ASSERT_NOT_NULL(frame);
    TEST_ASSERT_NOT_NULL(packet);
    frame->nb_samples = encoder->frame_size;
    frame->format = encoder->sample_fmt;
    frame->sample_rate = encoder->sample_rate;
    TEST_ASSERT_EQUAL_INT(0, av_channel_layout_copy(&frame->ch_layout, &encoder->ch_layout));
    TEST_ASSERT_EQUAL_INT(0, av_frame_get_buffer(frame, 0));

    for (int64_t pts = 0; ; pts += frame->nb_samples) {
        AVFrame *input = NULL;
        if (pts < (int64_t)seconds * encoder->sample_rate) {
            TEST_ASSERT_EQUAL_INT(0, av_frame_make_writable(frame));
            float *samples = (float *)frame->data[0];
            for (int i = 0; i < frame->nb_samples; ++i)
                samples[i] = 0.5f * sinf(2.0f * (float)M_PI * 440.0f * (float)(pts + i) / 16000.0f);
            frame->pts = pts;
            input = frame;
        }
        TEST_ASSERT_EQUAL_INT(0, avcodec_send_frame(encoder, input));
        while (avcodec_receive_packet(encoder, packet) == 0) {
            audio_fixture_append(fixture, packet);
            av_packet_unref(packet);
        }
        if (!input) break;
    }

    fixture->params = avcodec_parameters_alloc();
    TEST_ASSERT_NOT_NULL(fixture->params);
    TEST_ASSERT_EQUAL_INT(0, avcodec_parameters_from_context(fixture->params, encoder));
    fixture->time_base = encoder->time_base;
    av_packet_free(&packet);
    av_frame_free(&frame);
    avcodec_free_context(&encoder);
}

/* G.711 u-law, which the HLS fallback does not transcode */
static void make_pcmu_fixture(audio_fixture_t *fixture, int seconds) {
    fixture->params = avcodec_parameters_alloc();
    TEST_ASSERT_NOT_NULL(fixture->params);
    fixture->params->codec_type = AVMEDIA_TYPE_AUDIO;
    fixture->params->codec_id = AV_CODEC_ID_PCM_MULAW;
    fixture->params->sample_rate = 8000;
    av_channel_layout_default(&fixture->params->ch_layout, 1);
    fixture->time_base = (AVRational){1, 8000};
    AVPacket *packet = av_packet_alloc();
    TEST_ASSERT_NOT_NULL(packet);
    for (int64_t pts = 0; pts < (int64_t)seconds * 8000; pts += 160) {
        TEST_ASSERT_EQUAL_INT(0, av_new_packet(packet, 160));
        memset(packet->data, 0xff, packet->size);
        packet->pts = packet->dts = pts;
        packet->duration = 160;
        audio_fixture_append(fixture, packet);
        av_packet_unref(packet);
    }
    av_packet_free(&packet);
}

typedef struct {
    int video_streams;
    int audio_streams;
    int audio_packets;
    enum AVCodecID audio_codec;
    double first_video_s;
    double first_audio_s;
} hls_output_summary_t;

/* Feed fixture video plus the given audio track (listed before video, as
 * some RTSP cameras do) through the native HLS writer and summarize the
 * resulting playlist. mux_audio = 0 mirrors a stream with audio disabled:
 * the writer gets no audio stream but still receives audio packets. */
static hls_output_summary_t write_av_hls(const char *name, const audio_fixture_t *audio_fixture,
                                         int mux_audio) {
    AVFormatContext *fixture = NULL;
    TEST_ASSERT_EQUAL_INT(0, avformat_open_input(&fixture, TEST_RECORDING_PATH, NULL, NULL));
    TEST_ASSERT_GREATER_OR_EQUAL(0, avformat_find_stream_info(fixture, NULL));

    AVFormatContext *input = avformat_alloc_context();
    TEST_ASSERT_NOT_NULL(input);
    AVStream *audio = avformat_new_stream(input, NULL);
    AVStream *video = avformat_new_stream(input, NULL);
    TEST_ASSERT_NOT_NULL(audio);
    TEST_ASSERT_NOT_NULL(video);
    TEST_ASSERT_EQUAL_INT(0, avcodec_parameters_copy(audio->codecpar, audio_fixture->params));
    audio->time_base = audio_fixture->time_base;
    TEST_ASSERT_EQUAL_INT(0, avcodec_parameters_copy(video->codecpar, fixture->streams[0]->codecpar));
    video->time_base = fixture->streams[0]->time_base;

    hls_writer_t *writer = hls_writer_create(test_dir, name, 1);
    TEST_ASSERT_NOT_NULL(writer);
    TEST_ASSERT_EQUAL_INT(0, hls_writer_initialize_av(writer, video, mux_audio ? audio : NULL));

    AVPacket *packet = av_packet_alloc();
    TEST_ASSERT_NOT_NULL(packet);
    int next_audio = 0;
    for (;;) {
        TEST_ASSERT_EQUAL_INT(0, av_read_frame(fixture, packet));
        if (av_compare_ts(packet->dts, video->time_base, 3, (AVRational){1, 1}) >= 0) {
            av_packet_unref(packet);
            break;
        }
        // Interleave audio up to this video packet's timestamp
        while (next_audio < audio_fixture->count &&
               av_compare_ts(audio_fixture->packets[next_audio]->dts, audio->time_base,
                             packet->dts, video->time_base) <= 0) {
            AVPacket *audio_packet = audio_fixture->packets[next_audio++];
            audio_packet->stream_index = audio->index;
            TEST_ASSERT_GREATER_OR_EQUAL(0, hls_writer_write_packet(writer, audio_packet, audio));
        }
        packet->stream_index = video->index;
        TEST_ASSERT_GREATER_OR_EQUAL(0, hls_writer_write_packet(writer, packet, video));
        av_packet_unref(packet);
    }

    char playlist[MAX_PATH_LENGTH + 32];
    snprintf(playlist, sizeof(playlist), "%s/index.m3u8", writer->output_dir);
    hls_writer_close(writer);

    hls_output_summary_t summary = {0};
    summary.first_video_s = summary.first_audio_s = -1;
    AVFormatContext *recorded = NULL;
    TEST_ASSERT_EQUAL_INT(0, avformat_open_input(&recorded, playlist, NULL, NULL));
    TEST_ASSERT_GREATER_OR_EQUAL(0, avformat_find_stream_info(recorded, NULL));
    for (unsigned int i = 0; i < recorded->nb_streams; ++i) {
        const AVCodecParameters *par = recorded->streams[i]->codecpar;
        if (par->codec_type == AVMEDIA_TYPE_VIDEO) summary.video_streams++;
        if (par->codec_type == AVMEDIA_TYPE_AUDIO) {
            summary.audio_streams++;
            summary.audio_codec = par->codec_id;
        }
    }
    while (av_read_frame(recorded, packet) == 0) {
        const AVStream *stream = recorded->streams[packet->stream_index];
        double seconds = packet->pts * av_q2d(stream->time_base);
        if (stream->codecpar->codec_type == AVMEDIA_TYPE_AUDIO) {
            summary.audio_packets++;
            if (summary.first_audio_s < 0) summary.first_audio_s = seconds;
        } else if (stream->codecpar->codec_type == AVMEDIA_TYPE_VIDEO &&
                   summary.first_video_s < 0) {
            summary.first_video_s = seconds;
        }
        av_packet_unref(packet);
    }
    avformat_close_input(&recorded);
    av_packet_free(&packet);
    avformat_free_context(input);
    avformat_close_input(&fixture);
    return summary;
}

void test_writer_muxes_aac_audio_listed_before_video(void) {
    audio_fixture_t aac = {0};
    make_aac_fixture(&aac, 4);
    TEST_ASSERT_TRUE(hls_writer_audio_supported(aac.params));

    hls_output_summary_t summary = write_av_hls("aac-audio", &aac, 1);
    TEST_ASSERT_EQUAL_INT(1, summary.video_streams);
    TEST_ASSERT_EQUAL_INT(1, summary.audio_streams);
    TEST_ASSERT_EQUAL_INT(AV_CODEC_ID_AAC, summary.audio_codec);
    TEST_ASSERT_GREATER_THAN(0, summary.audio_packets);
    // Both tracks keep their own timebase origin, so they start together
    TEST_ASSERT_TRUE(fabs(summary.first_audio_s - summary.first_video_s) < 0.2);
    audio_fixture_free(&aac);
}

void test_writer_emits_video_only_when_audio_disabled(void) {
    audio_fixture_t aac = {0};
    make_aac_fixture(&aac, 4);
    hls_output_summary_t summary = write_av_hls("audio-disabled", &aac, 0);
    TEST_ASSERT_EQUAL_INT(1, summary.video_streams);
    TEST_ASSERT_EQUAL_INT(0, summary.audio_streams);
    TEST_ASSERT_EQUAL_INT(0, summary.audio_packets);
    audio_fixture_free(&aac);
}

void test_writer_skips_unsupported_audio_codec(void) {
    audio_fixture_t pcmu = {0};
    make_pcmu_fixture(&pcmu, 4);
    TEST_ASSERT_FALSE(hls_writer_audio_supported(pcmu.params));
    hls_output_summary_t summary = write_av_hls("pcmu-audio", &pcmu, 1);
    TEST_ASSERT_EQUAL_INT(1, summary.video_streams);
    TEST_ASSERT_EQUAL_INT(0, summary.audio_streams);
    TEST_ASSERT_EQUAL_INT(0, summary.audio_packets);
    audio_fixture_free(&pcmu);
}

int main(void) {
    init_logger();
    UNITY_BEGIN();
    RUN_TEST(test_cache_maintenance_uses_dedicated_path_then_default_path);
    RUN_TEST(test_writer_remaps_input_video_after_audio_to_output_stream_zero);
    RUN_TEST(test_writer_muxes_aac_audio_listed_before_video);
    RUN_TEST(test_writer_emits_video_only_when_audio_disabled);
    RUN_TEST(test_writer_skips_unsupported_audio_codec);
    RUN_TEST(test_parallel_writer_teardown_and_recreation_preserves_signals);
    RUN_TEST(test_parallel_unified_stream_shutdown_and_restart);
    RUN_TEST(test_unified_system_cleanup_preserves_signals);
    int result = UNITY_END();
    shutdown_logger();
    return result;
}
