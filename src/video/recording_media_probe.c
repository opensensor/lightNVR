#include "video/recording_media_probe.h"

#include <libavcodec/avcodec.h>
#include <libavformat/avformat.h>
#include <libavutil/time.h>
#include <stdint.h>
#include <stdbool.h>
#include <sys/stat.h>

/* Bounds a single file, since startup checks every interrupted recording. */
#define PROBE_TIMEOUT_US (5 * AV_TIME_BASE)
#define PROBE_MAX_BYTES (8 * 1024 * 1024)
#define PROBE_MAX_PACKETS 64

typedef struct {
    int64_t deadline;
    AVFormatContext *format;
    bool exhausted;
} probe_limit_t;

static int probe_limit_reached(void *opaque) {
    probe_limit_t *limit = opaque;
    if (av_gettime_relative() >= limit->deadline ||
        (limit->format->pb && limit->format->pb->bytes_read > PROBE_MAX_BYTES))
        limit->exhausted = true;
    return limit->exhausted;
}

static bool frame_is_clean(const AVFrame *frame) {
    return frame->width > 0 && frame->height > 0 &&
        !(frame->flags & AV_FRAME_FLAG_CORRUPT) && !frame->decode_error_flags;
}

const char *recording_media_probe_issue(const char *path) {
    struct stat info;
    if (!path || lstat(path, &info) != 0 || !S_ISREG(info.st_mode)) return "probe_error";

    AVFormatContext *format = avformat_alloc_context();
    if (!format) return "probe_error";

    AVCodecContext *decoder = NULL;
    AVPacket *packet = NULL;
    AVFrame *frame = NULL;
    AVDictionary *options = NULL;
    bool playable = false;
    const char *issue = "probe_error";
    probe_limit_t limit = { .deadline = av_gettime_relative() + PROBE_TIMEOUT_US, .format = format };

    format->interrupt_callback = (AVIOInterruptCB){ probe_limit_reached, &limit };
    format->probesize = 1024 * 1024;
    format->max_analyze_duration = AV_TIME_BASE;
    av_dict_set(&options, "protocol_whitelist", "file", 0);
    /* On failure this frees the context and clears the pointer. */
    int opened = avformat_open_input(&format, path, av_find_input_format("mov"), &options);
    if (opened < 0) {
        av_dict_free(&options);
        return limit.exhausted ? "probe_limit" :
            opened == AVERROR_INVALIDDATA ? "invalid_media" : "probe_error";
    }
    int info_result = avformat_find_stream_info(format, NULL);
    if (probe_limit_reached(&limit)) { issue = "probe_limit"; goto done; }
    if (info_result < 0) {
        issue = info_result == AVERROR_INVALIDDATA ? "invalid_media" : "probe_error";
        goto done;
    }

    int video = av_find_best_stream(format, AVMEDIA_TYPE_VIDEO, -1, -1, NULL, 0);
    if (video < 0) { issue = "no_video"; goto done; }

    packet = av_packet_alloc();
    frame = av_frame_alloc();
    if (!packet || !frame) goto done;

    /* A missing decoder is inconclusive, not evidence of damaged footage. */
    const AVCodec *codec = avcodec_find_decoder(format->streams[video]->codecpar->codec_id);
    if (!codec) { issue = "decoder_unavailable"; goto done; }
    decoder = avcodec_alloc_context3(codec);
    if (!decoder ||
        avcodec_parameters_to_context(decoder, format->streams[video]->codecpar) < 0) goto done;
    decoder->thread_count = 1;
    decoder->err_recognition = AV_EF_EXPLODE;
    if (avcodec_open2(decoder, codec, NULL) < 0) goto done;

    issue = "probe_limit";
    for (int i = 0; i < PROBE_MAX_PACKETS && !playable && !probe_limit_reached(&limit); i++) {
        int result = av_read_frame(format, packet);
        if (result < 0) {
            issue = result == AVERROR_EOF || result == AVERROR_INVALIDDATA
                ? "decode_error" : "probe_error";
            /* A short file may hold its only frame in the decoder until flushed. */
            if (result == AVERROR_EOF && decoder && avcodec_send_packet(decoder, NULL) >= 0 &&
                avcodec_receive_frame(decoder, frame) == 0) {
                playable = frame_is_clean(frame);
            }
            break;
        }
        if (packet->stream_index == video) {
            int sent = avcodec_send_packet(decoder, packet);
            int received = sent >= 0 ? avcodec_receive_frame(decoder, frame) : sent;
            if (received == 0) {
                playable = frame_is_clean(frame);
                if (!playable) { issue = "decode_error"; break; }
            } else if (received != AVERROR(EAGAIN)) {
                issue = received == AVERROR_INVALIDDATA ? "decode_error" : "probe_error";
                break;
            }
        }
        av_packet_unref(packet);
    }

done:
    if (probe_limit_reached(&limit)) { playable = false; issue = "probe_limit"; }
    av_dict_free(&options);
    av_packet_free(&packet);
    av_frame_free(&frame);
    avcodec_free_context(&decoder);
    avformat_close_input(&format);
    return playable ? "" : issue;
}
