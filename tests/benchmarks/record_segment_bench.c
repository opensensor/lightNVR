#define _POSIX_C_SOURCE 200809L

#include <errno.h>
#include <inttypes.h>
#include <limits.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/stat.h>
#include <time.h>
#include <unistd.h>

#include <libavcodec/avcodec.h>
#include <libavformat/avformat.h>
#include <libavutil/channel_layout.h>
#include <libavutil/imgutils.h>
#include <libavutil/opt.h>

#include "core/config.h"
#include "video/mp4_segment_recorder.h"
#include "video/mp4_writer_internal.h"

/* This baseline always uses FFmpeg for decoding and muxing. */
void mp4_writer_benchmark_set_native_audio_decoder(bool enabled) {
    (void)enabled;
}

void mp4_writer_benchmark_set_faam_muxer(bool enabled) {
    (void)enabled;
}

#define SOURCE_SECONDS 210
#define SEGMENT_SECONDS 30
#define SEGMENT_COUNT 6
#define VIDEO_RATE 25
#define GOP_SECONDS 2
#define AUDIO_RATE 48000
#define AUDIO_FRAME_SAMPLES 1024
#define AUDIO_SUMMARY_SEED UINT64_C(14695981039346656037)

typedef struct {
    uint64_t video_hash;
    uint64_t audio_hash;
    uint64_t video_packets;
    uint64_t audio_packets;
    uint64_t decoded_audio_samples;
    uint64_t audio_timeline_samples;
    uint64_t decoded_audio_hash;
} packet_summary;

typedef struct audio_case audio_case;

struct audio_case {
    const char *name;
    enum AVCodecID codec_id;
    int sample_rate;
    int channels;
    int samples_per_packet;
    int bytes_per_sample;
    enum AVSampleFormat sample_format;
    const uint8_t *codec_data;
    uint32_t codec_data_len;
    int (*write_packet)(const audio_case *audio_config, AVPacket *packet,
                        int64_t first_sample);
};

static int write_s16le_packet(const audio_case *audio_config, AVPacket *packet,
                              int64_t first_sample) {
    size_t sample_count = (size_t)audio_config->samples_per_packet *
                          audio_config->channels;
    size_t packet_bytes = sample_count * audio_config->bytes_per_sample;
    if (packet_bytes > INT_MAX || av_new_packet(packet, (int)packet_bytes) < 0)
        return AVERROR(ENOMEM);
    for (size_t i = 0; i < sample_count; ++i) {
        int phase = (int)((first_sample + i / audio_config->channels) % 400);
        int value = phase < 200 ? phase * 120 - 12000 : 36000 - phase * 120;
        uint16_t word = (uint16_t)(int16_t)value;
        packet->data[2 * i] = (uint8_t)word;
        packet->data[2 * i + 1] = (uint8_t)(word >> 8);
    }
    return 0;
}

static int g711_segment(int value) {
    static const int ends[8] = {0xFF, 0x1FF, 0x3FF, 0x7FF,
                                0xFFF, 0x1FFF, 0x3FFF, 0x7FFF};
    for (int i = 0; i < 8; ++i)
        if (value <= ends[i]) return i;
    return 8;
}

static uint8_t linear_to_mulaw(int16_t sample) {
    int value = sample;
    int mask;
    if (value < 0) {
        value = 0x84 - value;
        mask = 0x7F;
    } else {
        value += 0x84;
        mask = 0xFF;
    }
    if (value > 32635) value = 32635;
    int segment = g711_segment(value);
    int encoded = segment >= 8 ? 0x7F :
        (segment << 4) | ((value >> (segment + 3)) & 0x0F);
    return (uint8_t)(encoded ^ mask);
}

static uint8_t linear_to_alaw(int16_t sample) {
    int value = sample;
    int mask;
    if (value >= 0) {
        mask = 0xD5;
    } else {
        mask = 0x55;
        value = -value - 1;
    }
    int segment = g711_segment(value);
    int encoded = segment >= 8 ? 0x7F : segment << 4;
    if (segment < 2)
        encoded |= (value >> 4) & 0x0F;
    else
        encoded |= (value >> (segment + 3)) & 0x0F;
    return (uint8_t)(encoded ^ mask);
}

static int write_g711_packet(const audio_case *audio_config, AVPacket *packet,
                             int64_t first_sample) {
    int sample_count = audio_config->samples_per_packet * audio_config->channels;
    if (av_new_packet(packet, sample_count) < 0) return AVERROR(ENOMEM);
    for (int i = 0; i < sample_count; ++i) {
        int phase = (int)((first_sample + i) % 160);
        int value = phase < 80 ? phase * 400 - 16000 : 48000 - phase * 400;
        int16_t pcm = (int16_t)value;
        packet->data[i] = audio_config->codec_id == AV_CODEC_ID_PCM_ALAW
            ? linear_to_alaw(pcm) : linear_to_mulaw(pcm);
    }
    return 0;
}

static const audio_case audio_cases[] = {
    {"pcm_s16le", AV_CODEC_ID_PCM_S16LE, AUDIO_RATE, 2,
     AUDIO_FRAME_SAMPLES, 2, AV_SAMPLE_FMT_S16, NULL, 0, write_s16le_packet},
    {"ulaw_8k_mono", AV_CODEC_ID_PCM_MULAW, 8000, 1,
     160, 1, AV_SAMPLE_FMT_S16, NULL, 0, write_g711_packet},
    {"alaw_8k_mono", AV_CODEC_ID_PCM_ALAW, 8000, 1,
     160, 1, AV_SAMPLE_FMT_S16, NULL, 0, write_g711_packet}
};

static double elapsed_seconds(struct timespec start, struct timespec end) {
    return (double)(end.tv_sec - start.tv_sec) +
           (double)(end.tv_nsec - start.tv_nsec) / 1e9;
}

static int compare_doubles(const void *left, const void *right) {
    double a = *(const double *)left;
    double b = *(const double *)right;
    return (a > b) - (a < b);
}

static void print_timing_summary(const char *path, double *times, int count) {
    if (count <= 0) return;
    qsort(times, (size_t)count, sizeof(times[0]), compare_doubles);
    double median = count % 2
        ? times[count / 2]
        : (times[count / 2 - 1] + times[count / 2]) / 2.0;
    printf("timing path=%s repeats=%d median_s=%.6f min_s=%.6f max_s=%.6f\n",
           path, count, median, times[0], times[count - 1]);
}

static uint64_t hash_bytes(uint64_t hash, const uint8_t *data, size_t size) {
    for (size_t i = 0; i < size; ++i) {
        hash ^= data[i];
        hash *= UINT64_C(1099511628211);
    }
    return hash;
}

static int summarize_decoded_audio(packet_summary *summary, const AVFrame *frame) {
    if (!summary || !frame) return -1;
    int bytes_per_sample = av_get_bytes_per_sample((enum AVSampleFormat)frame->format);
    int channels = frame->ch_layout.nb_channels;
    if (bytes_per_sample <= 0 || channels <= 0 ||
        frame->nb_samples <= 0 || !frame->extended_data || !frame->extended_data[0])
        return -1;
    summary->decoded_audio_samples += (uint64_t)frame->nb_samples;
    if (av_sample_fmt_is_planar((enum AVSampleFormat)frame->format)) {
        for (int channel = 0; channel < channels; ++channel) {
            if (!frame->extended_data[channel]) return -1;
            summary->decoded_audio_hash = hash_bytes(summary->decoded_audio_hash,
                frame->extended_data[channel], (size_t)frame->nb_samples * bytes_per_sample);
        }
    } else {
        summary->decoded_audio_hash = hash_bytes(summary->decoded_audio_hash,
            frame->extended_data[0], (size_t)frame->nb_samples * channels * bytes_per_sample);
    }
    return 0;
}

static int write_video_packets(AVCodecContext *encoder, AVFormatContext *output,
                               AVStream *stream, AVPacket *packet) {
    int ret;
    while ((ret = avcodec_receive_packet(encoder, packet)) >= 0) {
        if (packet->duration <= 0) packet->duration = 1;
        av_packet_rescale_ts(packet, encoder->time_base, stream->time_base);
        packet->stream_index = stream->index;
        ret = av_interleaved_write_frame(output, packet);
        av_packet_unref(packet);
        if (ret < 0) return ret;
    }
    return ret == AVERROR(EAGAIN) || ret == AVERROR_EOF ? 0 : ret;
}

static int write_source(const char *path, const audio_case *audio_config) {
    const AVCodec *codec = avcodec_find_encoder_by_name("libx264");
    if (!codec) {
        fprintf(stderr, "source fixture needs the libx264 encoder\n");
        return -1;
    }

    AVFormatContext *output = NULL;
    AVCodecContext *encoder = avcodec_alloc_context3(codec);
    AVFrame *frame = av_frame_alloc();
    AVPacket *packet = av_packet_alloc();
    AVStream *video = NULL;
    AVStream *audio = NULL;
    AVDictionary *encoder_opts = NULL;
    int result = -1;

    if (!encoder || !frame || !packet ||
        avformat_alloc_output_context2(&output, NULL, "matroska", path) < 0 || !output) {
        fprintf(stderr, "failed to allocate source fixture resources\n");
        goto done;
    }

    encoder->width = 96;
    encoder->height = 64;
    encoder->pix_fmt = AV_PIX_FMT_YUV420P;
    encoder->time_base = (AVRational){1, VIDEO_RATE};
    encoder->framerate = (AVRational){VIDEO_RATE, 1};
    encoder->gop_size = VIDEO_RATE * GOP_SECONDS;
    encoder->max_b_frames = 0;
    encoder->bit_rate = 180000;
    encoder->flags |= AV_CODEC_FLAG_GLOBAL_HEADER;
    av_dict_set(&encoder_opts, "preset", "ultrafast", 0);
    av_dict_set(&encoder_opts, "tune", "zerolatency", 0);
    if (avcodec_open2(encoder, codec, &encoder_opts) < 0) {
        fprintf(stderr, "failed to open H.264 source encoder\n");
        goto done;
    }

    video = avformat_new_stream(output, NULL);
    audio = avformat_new_stream(output, NULL);
    if (!video || !audio ||
        avcodec_parameters_from_context(video->codecpar, encoder) < 0) {
        fprintf(stderr, "failed to initialize source streams\n");
        goto done;
    }
    video->time_base = encoder->time_base;
    audio->time_base = (AVRational){1, audio_config->sample_rate};
    audio->codecpar->codec_type = AVMEDIA_TYPE_AUDIO;
    audio->codecpar->codec_id = audio_config->codec_id;
    audio->codecpar->format = audio_config->sample_format;
    audio->codecpar->sample_rate = audio_config->sample_rate;
    audio->codecpar->bits_per_coded_sample = audio_config->bytes_per_sample * 8;
    audio->codecpar->bit_rate = audio_config->sample_rate * audio_config->channels *
                                audio_config->bytes_per_sample * 8;
    av_channel_layout_default(&audio->codecpar->ch_layout, audio_config->channels);
    if (audio_config->codec_data_len) {
        if (!audio_config->codec_data ||
            audio_config->codec_data_len > INT_MAX - AV_INPUT_BUFFER_PADDING_SIZE)
            goto done;
        audio->codecpar->extradata = av_mallocz(audio_config->codec_data_len +
                                                AV_INPUT_BUFFER_PADDING_SIZE);
        if (!audio->codecpar->extradata) goto done;
        memcpy(audio->codecpar->extradata, audio_config->codec_data,
               audio_config->codec_data_len);
        audio->codecpar->extradata_size = (int)audio_config->codec_data_len;
    }

    if (!(output->oformat->flags & AVFMT_NOFILE) &&
        avio_open(&output->pb, path, AVIO_FLAG_WRITE) < 0) {
        fprintf(stderr, "failed to open source fixture %s\n", path);
        goto done;
    }
    if (avformat_write_header(output, NULL) < 0) {
        fprintf(stderr, "failed to write source fixture header\n");
        goto done;
    }

    frame->format = encoder->pix_fmt;
    frame->width = encoder->width;
    frame->height = encoder->height;
    if (av_frame_get_buffer(frame, 32) < 0) goto done;

    int video_frames = SOURCE_SECONDS * VIDEO_RATE;
    int64_t audio_samples = 0;
    for (int vi = 0; vi < video_frames; ++vi) {
        double video_time = (double)vi / VIDEO_RATE;
        while ((double)audio_samples / audio_config->sample_rate <= video_time &&
               audio_samples < (int64_t)SOURCE_SECONDS * audio_config->sample_rate) {
            AVPacket *ap = av_packet_alloc();
            if (!ap || audio_config->write_packet(audio_config, ap, audio_samples) < 0) {
                av_packet_free(&ap);
                goto done;
            }
            ap->pts = av_rescale_q(audio_samples,
                (AVRational){1, audio_config->sample_rate}, audio->time_base);
            ap->dts = ap->pts;
            ap->duration = av_rescale_q(audio_config->samples_per_packet,
                (AVRational){1, audio_config->sample_rate}, audio->time_base);
            ap->stream_index = audio->index;
            int ar = av_interleaved_write_frame(output, ap);
            av_packet_free(&ap);
            if (ar < 0) goto done;
            audio_samples += audio_config->samples_per_packet;
        }

        if (av_frame_make_writable(frame) < 0) goto done;
        for (int y = 0; y < encoder->height; ++y)
            memset(frame->data[0] + y * frame->linesize[0], 72 + (vi % 32), encoder->width);
        for (int plane = 1; plane < 3; ++plane)
            for (int y = 0; y < encoder->height / 2; ++y)
                memset(frame->data[plane] + y * frame->linesize[plane], 128,
                       encoder->width / 2);
        frame->pts = vi;
        if (avcodec_send_frame(encoder, frame) < 0 ||
            write_video_packets(encoder, output, video, packet) < 0) goto done;
    }
    if (avcodec_send_frame(encoder, NULL) < 0 ||
        write_video_packets(encoder, output, video, packet) < 0 ||
        av_write_trailer(output) < 0) goto done;
    result = 0;

done:
    av_dict_free(&encoder_opts);
    if (output && output->pb) avio_closep(&output->pb);
    avformat_free_context(output);
    avcodec_free_context(&encoder);
    av_frame_free(&frame);
    av_packet_free(&packet);
    return result;
}

static int summarize_output(const char *path, packet_summary *summary) {
    AVFormatContext *input = NULL;
    AVPacket *packet = av_packet_alloc();
    if (!packet || avformat_open_input(&input, path, NULL, NULL) < 0 ||
        avformat_find_stream_info(input, NULL) < 0) {
        av_packet_free(&packet);
        avformat_close_input(&input);
        return -1;
    }
    int video_stream = -1, audio_stream = -1;
    for (unsigned i = 0; i < input->nb_streams; ++i) {
        AVCodecParameters *par = input->streams[i]->codecpar;
        if (par->codec_type == AVMEDIA_TYPE_VIDEO && par->codec_id == AV_CODEC_ID_H264)
            video_stream = (int)i;
        if (par->codec_type == AVMEDIA_TYPE_AUDIO && par->codec_id == AV_CODEC_ID_AAC)
            audio_stream = (int)i;
    }
    if (video_stream < 0 || audio_stream < 0) {
        av_packet_free(&packet);
        avformat_close_input(&input);
        return -1;
    }
    AVStream *audio = input->streams[audio_stream];
    AVCodecParameters *audio_par = audio->codecpar;
    if (audio_par->sample_rate <= 0 || audio->duration == AV_NOPTS_VALUE ||
        audio->duration <= 0 || audio->time_base.num <= 0 || audio->time_base.den <= 0) {
        av_packet_free(&packet);
        avformat_close_input(&input);
        return -1;
    }
    const uint64_t frame_tolerance = audio_par->frame_size > 0
        ? (uint64_t)audio_par->frame_size : AUDIO_FRAME_SAMPLES;
    const AVCodec *audio_decoder = avcodec_find_decoder(AV_CODEC_ID_AAC);
    AVCodecContext *audio_decode_ctx = audio_decoder ?
        avcodec_alloc_context3(audio_decoder) : NULL;
    AVFrame *decoded_audio = av_frame_alloc();
    if (!audio_decode_ctx || !decoded_audio ||
        avcodec_parameters_to_context(audio_decode_ctx,
            input->streams[audio_stream]->codecpar) < 0 ||
        avcodec_open2(audio_decode_ctx, audio_decoder, NULL) < 0) {
        av_frame_free(&decoded_audio);
        avcodec_free_context(&audio_decode_ctx);
        av_packet_free(&packet);
        avformat_close_input(&input);
        return -1;
    }
    uint64_t decoded_audio_frames = 0;
    summary->audio_timeline_samples = (uint64_t)av_rescale_q(
        audio->duration, audio->time_base,
        (AVRational){ .num = 1, .den = audio_par->sample_rate });
    summary->video_hash = UINT64_C(14695981039346656037);
    summary->audio_hash = UINT64_C(14695981039346656037);
    summary->decoded_audio_hash = AUDIO_SUMMARY_SEED;
    summary->decoded_audio_samples = 0;
    summary->video_packets = summary->audio_packets = 0;
    while (av_read_frame(input, packet) >= 0) {
        if (packet->stream_index == video_stream) {
            summary->video_hash = hash_bytes(summary->video_hash, packet->data, packet->size);
            ++summary->video_packets;
        } else if (packet->stream_index == audio_stream) {
            summary->audio_hash = hash_bytes(summary->audio_hash, packet->data, packet->size);
            ++summary->audio_packets;
            int decode_result = avcodec_send_packet(audio_decode_ctx, packet);
            while (decode_result >= 0) {
                decode_result = avcodec_receive_frame(audio_decode_ctx, decoded_audio);
                if (decode_result < 0) break;
                if (summarize_decoded_audio(summary, decoded_audio) != 0) {
                    av_frame_free(&decoded_audio);
                    avcodec_free_context(&audio_decode_ctx);
                    av_packet_free(&packet);
                    avformat_close_input(&input);
                    return -1;
                }
                ++decoded_audio_frames;
                av_frame_unref(decoded_audio);
            }
            if (decode_result != AVERROR(EAGAIN) && decode_result != AVERROR_EOF) {
                av_frame_free(&decoded_audio);
                avcodec_free_context(&audio_decode_ctx);
                av_packet_free(&packet);
                avformat_close_input(&input);
                return -1;
            }
        }
        av_packet_unref(packet);
    }
    if (avcodec_send_packet(audio_decode_ctx, NULL) >= 0) {
        while (avcodec_receive_frame(audio_decode_ctx, decoded_audio) >= 0) {
            if (summarize_decoded_audio(summary, decoded_audio) != 0) {
                av_frame_free(&decoded_audio);
                avcodec_free_context(&audio_decode_ctx);
                av_packet_free(&packet);
                avformat_close_input(&input);
                return -1;
            }
            ++decoded_audio_frames;
            av_frame_unref(decoded_audio);
        }
    }
    av_frame_free(&decoded_audio);
    avcodec_free_context(&audio_decode_ctx);
    av_packet_free(&packet);
    avformat_close_input(&input);
    uint64_t timeline_delta = summary->audio_timeline_samples > summary->decoded_audio_samples
        ? summary->audio_timeline_samples - summary->decoded_audio_samples
        : summary->decoded_audio_samples - summary->audio_timeline_samples;
    return summary->video_packets && summary->audio_packets && decoded_audio_frames &&
        timeline_delta <= frame_tolerance ? 0 : -1;
}

static int run_series(const char *source, const char *output_dir, int repeats,
                      const char *decoder,
                      const char *muxer,
                      const audio_case *audio_config,
                      packet_summary *summary, double *elapsed) {
    struct timespec start, end;
    static const char *const path_names[] = {
        "native+faam", "native+ffmpeg", "ffmpeg+faam", "ffmpeg+ffmpeg"
    };
    double path_times[4][20] = {{0}};
    int path_counts[4] = {0};
    packet_summary expected = {0};
    bool have_expected = false;
    for (int run = 0; run < repeats; ++run) {
        bool use_native = strcmp(decoder, "alternating") == 0
            ? (run % 2 == 0)
            : strcmp(decoder, "native") == 0;
        bool use_faam = strcmp(muxer, "alternating") == 0
            ? (run % 2 == 0)
            : strcmp(muxer, "faam") == 0;
        int path_index = (use_native ? 0 : 2) + (use_faam ? 0 : 1);
        mp4_writer_benchmark_set_native_audio_decoder(use_native);
        mp4_writer_benchmark_set_faam_muxer(use_faam);
        packet_summary run_summary = {
            .video_hash = AUDIO_SUMMARY_SEED,
            .audio_hash = AUDIO_SUMMARY_SEED,
            .decoded_audio_hash = AUDIO_SUMMARY_SEED
        };
        int64_t frame_packets = 0, audio_packets = 0;
        AVFormatContext *input = NULL;
        segment_info_t segment = {0};
        char path[PATH_MAX];
        if (avformat_open_input(&input, source, NULL, NULL) < 0 ||
            avformat_find_stream_info(input, NULL) < 0) {
            fprintf(stderr, "could not open generated camera fixture\n");
            avformat_close_input(&input);
            return -1;
        }
        if (clock_gettime(CLOCK_MONOTONIC, &start) != 0) return -1;
        for (int segment_index = 0; segment_index < SEGMENT_COUNT; ++segment_index) {
            snprintf(path, sizeof(path), "%s/segment-%d.mp4", output_dir, segment_index);
            if (record_segment(source, path, SEGMENT_SECONDS, 1, &input, &segment,
                               NULL, NULL, NULL, NULL) != 0) {
                fprintf(stderr, "record_segment failed at run %d segment %d\n",
                        run, segment_index);
                avformat_close_input(&input);
                av_packet_free(&segment.pending_video_keyframe);
                cleanup_audio_transcoder(source);
                return -1;
            }
        }
        if (clock_gettime(CLOCK_MONOTONIC, &end) != 0) return -1;
        *elapsed = elapsed_seconds(start, end);
        path_times[path_index][path_counts[path_index]++] = *elapsed;
        avformat_close_input(&input);
        av_packet_free(&segment.pending_video_keyframe);
        cleanup_audio_transcoder(source);

        for (int segment_index = 0; segment_index < SEGMENT_COUNT; ++segment_index) {
            char summary_path[PATH_MAX];
            packet_summary current = {0};
            snprintf(summary_path, sizeof(summary_path), "%s/segment-%d.mp4",
                     output_dir, segment_index);
            if (summarize_output(summary_path, &current) != 0) {
                fprintf(stderr, "invalid LightNVR MP4 output: %s\n", summary_path);
                return -1;
            }
            printf("segment run=%d index=%d video_packets=%" PRIu64
                   " audio_packets=%" PRIu64 " decoded_audio_samples=%" PRIu64
                   " audio_timeline_samples=%" PRIu64
                   " audio_hash=%016" PRIx64 " decoded_audio_hash=%016" PRIx64 "\n",
                   run, segment_index, current.video_packets, current.audio_packets,
                   current.decoded_audio_samples, current.audio_timeline_samples,
                   current.audio_hash,
                   current.decoded_audio_hash);
            frame_packets += current.video_packets;
            audio_packets += current.audio_packets;
            run_summary.video_hash = hash_bytes(run_summary.video_hash,
                (const uint8_t *)&current.video_hash, sizeof(current.video_hash));
            run_summary.audio_hash = hash_bytes(run_summary.audio_hash,
                (const uint8_t *)&current.audio_hash, sizeof(current.audio_hash));
            run_summary.decoded_audio_hash = hash_bytes(run_summary.decoded_audio_hash,
                (const uint8_t *)&current.decoded_audio_hash,
                sizeof(current.decoded_audio_hash));
            run_summary.decoded_audio_samples += current.decoded_audio_samples;
            run_summary.audio_timeline_samples += current.audio_timeline_samples;
        }
        run_summary.video_packets = (uint64_t)frame_packets;
        run_summary.audio_packets = (uint64_t)audio_packets;
        if (!have_expected) {
            expected = run_summary;
            have_expected = true;
        } else if (run_summary.video_packets != expected.video_packets ||
                   run_summary.audio_packets != expected.audio_packets ||
                   run_summary.decoded_audio_samples != expected.decoded_audio_samples ||
                   run_summary.audio_timeline_samples != expected.audio_timeline_samples ||
                   run_summary.video_hash != expected.video_hash ||
                   run_summary.audio_hash != expected.audio_hash ||
                   run_summary.decoded_audio_hash != expected.decoded_audio_hash) {
            fprintf(stderr,
                    "recording output mismatch at run %d (decoder=%s muxer=%s); "
                    "packet/sample counts or output hashes differ\n",
                    run, use_native ? "native" : "ffmpeg",
                    use_faam ? "faam" : "ffmpeg");
            fprintf(stderr,
                    "expected: video=%" PRIu64 " audio=%" PRIu64
                    " decoded_samples=%" PRIu64 " timeline_samples=%" PRIu64
                    " video_hash=%016" PRIx64 " audio_hash=%016" PRIx64
                    " decoded_hash=%016" PRIx64 "\n",
                    expected.video_packets, expected.audio_packets,
                    expected.decoded_audio_samples, expected.audio_timeline_samples,
                    expected.video_hash, expected.audio_hash,
                    expected.decoded_audio_hash);
            fprintf(stderr,
                    "actual:   video=%" PRIu64 " audio=%" PRIu64
                    " decoded_samples=%" PRIu64 " timeline_samples=%" PRIu64
                    " video_hash=%016" PRIx64 " audio_hash=%016" PRIx64
                    " decoded_hash=%016" PRIx64 "\n",
                    run_summary.video_packets, run_summary.audio_packets,
                    run_summary.decoded_audio_samples, run_summary.audio_timeline_samples,
                    run_summary.video_hash, run_summary.audio_hash,
                    run_summary.decoded_audio_hash);
            return -1;
        }
        *summary = run_summary;
        printf("run=%d audio_case=%s decoder=%s muxer=%s elapsed_s=%.6f video_packets=%" PRIu64
               " audio_packets=%" PRIu64 " audio_samples=%" PRIu64
               " audio_timeline_samples=%" PRIu64
               " video_hash=%016" PRIx64 " audio_hash=%016" PRIx64
               " decoded_audio_hash=%016" PRIx64 "\n",
               run, audio_config->name, use_native ? "native" : "ffmpeg",
               use_faam ? "faam" : "ffmpeg", *elapsed, run_summary.video_packets,
               run_summary.audio_packets, run_summary.decoded_audio_samples,
               run_summary.audio_timeline_samples, run_summary.video_hash,
               run_summary.audio_hash,
               run_summary.decoded_audio_hash);
    }
    for (int path = 0; path < 4; ++path)
        print_timing_summary(path_names[path], path_times[path], path_counts[path]);
    return 0;
}

static const audio_case *find_audio_case(const char *name) {
    for (size_t i = 0; i < sizeof(audio_cases) / sizeof(audio_cases[0]); ++i)
        if (strcmp(audio_cases[i].name, name) == 0) return &audio_cases[i];
    return NULL;
}

int main(int argc, char **argv) {
    const char *output_dir = argc > 1 ? argv[1] : "/tmp/lightnvr-record-bench";
    const char *audio_name = argc > 2 ? argv[2] : "pcm_s16le";
    int repeats = argc > 3 ? atoi(argv[3]) : 5;
    const char *decoder = argc > 4 ? argv[4] : "native";
    const char *muxer = argc > 5 ? argv[5] : "faam";
    const audio_case *audio_config = find_audio_case(audio_name);
    if (!audio_config) {
        fprintf(stderr, "unknown audio case '%s'; available:", audio_name);
        for (size_t i = 0; i < sizeof(audio_cases) / sizeof(audio_cases[0]); ++i)
            fprintf(stderr, " %s", audio_cases[i].name);
        fputc('\n', stderr);
        return 2;
    }
    if (repeats < 1 || repeats > 20) {
        fprintf(stderr, "usage: %s [output-dir] [audio-case] [1..20 repeat cycles] [native|ffmpeg|alternating] [faam|ffmpeg|alternating]\n",
                argv[0]);
        return 2;
    }
    if (strcmp(decoder, "native") != 0 && strcmp(decoder, "ffmpeg") != 0 &&
        strcmp(decoder, "alternating") != 0) {
        fprintf(stderr, "audio decoder must be 'native', 'ffmpeg', or 'alternating'\n");
        return 2;
    }
    if (strcmp(muxer, "faam") != 0 && strcmp(muxer, "ffmpeg") != 0 &&
        strcmp(muxer, "alternating") != 0) {
        fprintf(stderr, "MP4 muxer must be 'faam', 'ffmpeg', or 'alternating'\n");
        return 2;
    }
    if (mkdir(output_dir, 0755) != 0 && errno != EEXIST) {
        perror("mkdir output directory");
        return 1;
    }

    memset(&g_config, 0, sizeof(g_config));
    g_config.max_streams = 1;
    config_set_mp4_faststart(&g_config, true);
    char source[PATH_MAX];
    if (snprintf(source, sizeof(source), "%s/camera-source.mkv", output_dir) >= (int)sizeof(source)) {
        fprintf(stderr, "output path too long\n");
        return 1;
    }
    if (write_source(source, audio_config) != 0) return 1;

    packet_summary summary = {
        .video_hash = AUDIO_SUMMARY_SEED,
        .audio_hash = AUDIO_SUMMARY_SEED
    };
    double elapsed = 0;
    if (run_series(source, output_dir, repeats, decoder, muxer, audio_config,
                   &summary, &elapsed) != 0)
        return 1;
    unlink(source);
    printf("summary repeats=%d segments_per_repeat=%d source_seconds=%d"
           " audio_case=%s decoder=%s muxer=%s last_repeat_elapsed_s=%.6f\n",
           repeats, SEGMENT_COUNT, SOURCE_SECONDS, audio_config->name, decoder,
           muxer, elapsed);
    return 0;
}
