/**
 * Utility functions for MP4 writer
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <dirent.h>
#include <time.h>
#include <sys/stat.h>
#include <limits.h>
#include <signal.h>
#include <errno.h>

// Define PATH_MAX if not defined
#ifndef PATH_MAX
#define PATH_MAX 4096
#endif
// Minimum allowed number of audio channels
#define MIN_AUDIO_CHANNELS 1
// Default AAC frame size (in samples per channel).
// AAC-LC typically uses 1024 samples per frame at common sample rates, so we
// use 1024 as a safe fallback when the encoder's frame_size is not set.
#define DEFAULT_AAC_FRAME_SIZE 1024
#include <libavformat/avformat.h>
#include <libavcodec/avcodec.h>
#include <libavutil/avutil.h>
#include <libavutil/audio_fifo.h>
#include <libavutil/opt.h>
#include <libavutil/time.h>
#include <libswresample/swresample.h>
#include <pthread.h>

#include "core/config.h"
#include "core/logger.h"
#include "core/path_utils.h"
#include "utils/strings.h"
#include "video/mp4_writer.h"
#include "video/mp4_writer_internal.h"

#include "video/ffmpeg_utils.h"
#include "video/faac_encoder.h"
#include "telemetry/recording_io_metrics.h"

// Structure to hold audio transcoding context
typedef struct {
    AVCodecContext *decoder_ctx;
    faac_adapter_t *faac_enc;
    SwrContext *swr_ctx;
    AVAudioFifo *fifo;          // Buffer to accumulate samples for the encoder
    AVFrame *frame;
    AVFrame *resampled_frame;
    AVPacket *in_pkt;
    AVPacket *out_pkt;
    int initialized;
    uint64_t faac_input_samples;
    uint64_t faac_output_frames;
    bool faac_flushed;
    int64_t next_faac_pts;

    // Voice-enhancement filter graph (discussion #395).  filter_graph is NULL
    // when the feature is disabled or the graph hasn't been constructed yet.
    // The actual filter chain (afftdn / highpass / lowpass, codec-aware
    // defaults) is deferred to a follow-up PR — for now this is a stub that
    // routes the per-stream opt-in through to the transcoder lifecycle so
    // future filter-graph code has a stable insertion point.
    bool voice_enhancement_enabled;
    void *filter_graph;            // AVFilterGraph*; void* to avoid header churn
    void *filter_buffersrc_ctx;    // AVFilterContext* (buffer source)
    void *filter_buffersink_ctx;   // AVFilterContext* (buffer sink)
} audio_transcoder_t;

/**
 * Get the effective encoder frame size, applying a fallback when the
 * encoder does not set frame_size.
 */
static int get_encoder_frame_size(const audio_transcoder_t *transcoder)
{
    if (!transcoder) {
        return DEFAULT_AAC_FRAME_SIZE;
    }
    if (transcoder->faac_enc) {
        return faac_adapter_get_frame_samples(transcoder->faac_enc);
    }
    return DEFAULT_AAC_FRAME_SIZE;
}

static faac_encoder_config_t make_faac_config(int sample_rate, int channels)
{
    return (faac_encoder_config_t){
        .sample_rate = (uint32_t)sample_rate,
        .num_channels = (uint32_t)channels,
        .bit_rate_per_channel = sample_rate >= 32000 ? 64000 : 32000,
        .object_type = FAAC_OBJ_AUTO
    };
}

// Global transcoder context for each stream
static audio_transcoder_t audio_transcoders[MAX_STREAMS] = {0};
static char audio_transcoder_stream_names[MAX_STREAMS][MAX_STREAM_NAME] = {{0}};
static pthread_mutex_t audio_transcoder_mutex = PTHREAD_MUTEX_INITIALIZER;

// Voice-enhancement opt-in staged for streams that don't yet have a transcoder
// slot — read once at init_audio_transcoder() time so the flag is correct on
// the very first packet of a recording session (discussion #395).
// Protected by audio_transcoder_mutex.
typedef struct {
    char stream_name[MAX_STREAM_NAME];
    bool enabled;
} voice_enhancement_pending_t;
static voice_enhancement_pending_t voice_enhancement_pending[MAX_STREAMS] = {0};

/**
 * Initialize audio transcoder for a stream
 *
 * @param stream_name Name of the stream
 * @param codec_params Original codec parameters (PCM variants including μ-law, A-law, S16LE, etc.)
 * @param time_base Time base of the original stream
 * @return Index of the transcoder in the array, or -1 on error
 */
static int init_audio_transcoder(const char *stream_name,
                               const AVCodecParameters *codec_params,
                               const AVRational *time_base) {
    int ret = -1;
    int slot = -1;
    const AVCodec *decoder = NULL;

    // Find an empty slot or existing entry for this stream
    for (int i = 0; i < g_config.max_streams; i++) {
        if (!audio_transcoders[i].initialized) {
            slot = i;
            break;
        } else if (audio_transcoder_stream_names[i][0] != '\0' &&
                  strcmp(audio_transcoder_stream_names[i], stream_name) == 0) {
            // Stream already has a transcoder, return its index
            return i;
        }
    }

    if (slot == -1) {
        log_error("No available slots for audio transcoder registration");
        return -1;
    }

    // Find the PCM decoder for the specific codec
    decoder = avcodec_find_decoder(codec_params->codec_id);
    if (!decoder) {
        log_error("Failed to find decoder for PCM audio (codec_id=%d) in %s",
                 codec_params->codec_id, stream_name);
        return -1;
    }

    // Create decoder context
    audio_transcoders[slot].decoder_ctx = avcodec_alloc_context3(decoder);
    if (!audio_transcoders[slot].decoder_ctx) {
        recording_io_report_failure(RECORDING_IO_RESOURCE_RECORDING,
                                    RECORDING_IO_OPERATION_ALLOCATE, ENOMEM);
        log_error("Failed to allocate decoder context for %s", stream_name);
        goto cleanup;
    }

    // Copy parameters to decoder context
    ret = avcodec_parameters_to_context(audio_transcoders[slot].decoder_ctx, codec_params);
    if (ret < 0) {
        log_ffmpeg_error(ret, "Failed to copy parameters to decoder context");
        goto cleanup;
    }

    // Set time base
    audio_transcoders[slot].decoder_ctx->time_base = *time_base;

    // Open decoder
    ret = avcodec_open2(audio_transcoders[slot].decoder_ctx, decoder, NULL);
    if (ret < 0) {
        log_ffmpeg_error(ret, "Failed to open PCM audio decoder");
        goto cleanup;
    }

    int sr = audio_transcoders[slot].decoder_ctx->sample_rate;
    int nb_ch = 1;
#if LIBAVCODEC_VERSION_INT >= AV_VERSION_INT(59, 37, 100)
    if (audio_transcoders[slot].decoder_ctx->ch_layout.nb_channels > 0)
        nb_ch = audio_transcoders[slot].decoder_ctx->ch_layout.nb_channels;
#else
    if (audio_transcoders[slot].decoder_ctx->channels > 0)
        nb_ch = audio_transcoders[slot].decoder_ctx->channels;
#endif
    if (nb_ch < MIN_AUDIO_CHANNELS) nb_ch = MIN_AUDIO_CHANNELS;

    faac_encoder_config_t faac_cfg = make_faac_config(sr, nb_ch);

    audio_transcoders[slot].faac_enc = faac_adapter_open(&faac_cfg);
    if (!audio_transcoders[slot].faac_enc) {
        log_error("Failed to open FAAC encoder for %s", stream_name);
        goto cleanup;
    }

    enum AVSampleFormat enc_sample_fmt = AV_SAMPLE_FMT_S16;

    // Set up sample format conversion for the encoder
#if LIBAVCODEC_VERSION_INT >= AV_VERSION_INT(59, 37, 100)
    AVChannelLayout out_ch_layout;
    av_channel_layout_default(&out_ch_layout, nb_ch);
    ret = swr_alloc_set_opts2(&audio_transcoders[slot].swr_ctx,
                              &out_ch_layout,
                              enc_sample_fmt,
                              sr,
                              &audio_transcoders[slot].decoder_ctx->ch_layout,
                              audio_transcoders[slot].decoder_ctx->sample_fmt,
                              audio_transcoders[slot].decoder_ctx->sample_rate,
                              0, NULL);
    av_channel_layout_uninit(&out_ch_layout);
    if (ret < 0 || !audio_transcoders[slot].swr_ctx) {
        if (ret == AVERROR(ENOMEM) || !audio_transcoders[slot].swr_ctx) {
            recording_io_report_failure(RECORDING_IO_RESOURCE_RECORDING,
                                        RECORDING_IO_OPERATION_ALLOCATE,
                                        ENOMEM);
        }
        log_ffmpeg_error(ret, "Failed to allocate SwrContext");
        goto cleanup;
    }
#else
    audio_transcoders[slot].swr_ctx = swr_alloc_set_opts(NULL,
        av_get_default_channel_layout(nb_ch),
        enc_sample_fmt,
        sr,
        audio_transcoders[slot].decoder_ctx->channel_layout,
        audio_transcoders[slot].decoder_ctx->sample_fmt,
        audio_transcoders[slot].decoder_ctx->sample_rate,
        0, NULL);
    if (!audio_transcoders[slot].swr_ctx) {
        recording_io_report_failure(RECORDING_IO_RESOURCE_RECORDING,
                                    RECORDING_IO_OPERATION_ALLOCATE, ENOMEM);
        log_error("Failed to allocate SwrContext for %s", stream_name);
        ret = AVERROR(ENOMEM);
        goto cleanup;
    }
#endif

    ret = swr_init(audio_transcoders[slot].swr_ctx);
    if (ret < 0) {
        log_ffmpeg_error(ret, "Failed to initialize SwrContext");
        goto cleanup;
    }

    // Allocate an audio FIFO to buffer samples for the AAC encoder.
    {
        int enc_frame_size = get_encoder_frame_size(&audio_transcoders[slot]);
        audio_transcoders[slot].fifo = av_audio_fifo_alloc(
            enc_sample_fmt,
            nb_ch,
            enc_frame_size * 4);
        if (!audio_transcoders[slot].fifo) {
            recording_io_report_failure(RECORDING_IO_RESOURCE_RECORDING,
                                        RECORDING_IO_OPERATION_ALLOCATE,
                                        ENOMEM);
            log_error("Failed to allocate audio FIFO for %s", stream_name);
            goto cleanup;
        }
    }
    audio_transcoders[slot].faac_input_samples = 0;
    audio_transcoders[slot].faac_output_frames = 0;
    audio_transcoders[slot].faac_flushed = false;
    audio_transcoders[slot].next_faac_pts =
        -(int64_t)faac_adapter_get_encoder_delay(audio_transcoders[slot].faac_enc);

    // Allocate frame and packets
    audio_transcoders[slot].frame = av_frame_alloc();
    if (!audio_transcoders[slot].frame) {
        recording_io_report_failure(RECORDING_IO_RESOURCE_RECORDING,
                                    RECORDING_IO_OPERATION_ALLOCATE, ENOMEM);
        log_error("Failed to allocate frame for %s", stream_name);
        goto cleanup;
    }

    // Allocate resampled frame for format-converted audio
    audio_transcoders[slot].resampled_frame = av_frame_alloc();
    if (!audio_transcoders[slot].resampled_frame) {
        recording_io_report_failure(RECORDING_IO_RESOURCE_RECORDING,
                                    RECORDING_IO_OPERATION_ALLOCATE, ENOMEM);
        log_error("Failed to allocate resampled frame for %s", stream_name);
        goto cleanup;
    }

    audio_transcoders[slot].in_pkt = av_packet_alloc();
    if (!audio_transcoders[slot].in_pkt) {
        recording_io_report_failure(RECORDING_IO_RESOURCE_RECORDING,
                                    RECORDING_IO_OPERATION_ALLOCATE, ENOMEM);
        log_error("Failed to allocate input packet for %s", stream_name);
        goto cleanup;
    }

    audio_transcoders[slot].out_pkt = av_packet_alloc();
    if (!audio_transcoders[slot].out_pkt) {
        recording_io_report_failure(RECORDING_IO_RESOURCE_RECORDING,
                                    RECORDING_IO_OPERATION_ALLOCATE, ENOMEM);
        log_error("Failed to allocate output packet for %s", stream_name);
        goto cleanup;
    }

    // Pick up any voice-enhancement opt-in staged before this slot was
    // allocated.  The actual filter graph is built lazily in
    // transcode_audio_packet() once the decoder frame format is known
    // (currently stubbed — discussion #395).
    audio_transcoders[slot].voice_enhancement_enabled = false;
    for (int i = 0; i < g_config.max_streams; i++) {
        if (voice_enhancement_pending[i].stream_name[0] != '\0' &&
            strcmp(voice_enhancement_pending[i].stream_name, stream_name) == 0) {
            audio_transcoders[slot].voice_enhancement_enabled =
                voice_enhancement_pending[i].enabled;
            break;
        }
    }
    audio_transcoders[slot].filter_graph = NULL;
    audio_transcoders[slot].filter_buffersrc_ctx = NULL;
    audio_transcoders[slot].filter_buffersink_ctx = NULL;

    // Mark as initialized
    audio_transcoders[slot].initialized = 1;

    // Store stream name
    safe_strcpy(audio_transcoder_stream_names[slot], stream_name, MAX_STREAM_NAME, 0);

    log_info("Successfully initialized FAAC audio transcoder for %s (Sample rate: %d, Channels: %d, Delay: %d)",
             stream_name, sr, nb_ch, faac_adapter_get_encoder_delay(audio_transcoders[slot].faac_enc));

    return slot;

cleanup:
    if (audio_transcoders[slot].faac_enc) {
        faac_adapter_close(&audio_transcoders[slot].faac_enc);
    }

    if (audio_transcoders[slot].decoder_ctx) {
        avcodec_free_context(&audio_transcoders[slot].decoder_ctx);
        audio_transcoders[slot].decoder_ctx = NULL;
    }

    if (audio_transcoders[slot].swr_ctx) {
        swr_free(&audio_transcoders[slot].swr_ctx);
        audio_transcoders[slot].swr_ctx = NULL;
    }

    if (audio_transcoders[slot].fifo) {
        av_audio_fifo_free(audio_transcoders[slot].fifo);
        audio_transcoders[slot].fifo = NULL;
    }

    if (audio_transcoders[slot].frame) {
        av_frame_free(&audio_transcoders[slot].frame);
        audio_transcoders[slot].frame = NULL;
    }

    if (audio_transcoders[slot].resampled_frame) {
        av_frame_free(&audio_transcoders[slot].resampled_frame);
        audio_transcoders[slot].resampled_frame = NULL;
    }

    if (audio_transcoders[slot].in_pkt) {
        av_packet_free(&audio_transcoders[slot].in_pkt);
        audio_transcoders[slot].in_pkt = NULL;
    }

    if (audio_transcoders[slot].out_pkt) {
        av_packet_free(&audio_transcoders[slot].out_pkt);
        audio_transcoders[slot].out_pkt = NULL;
    }

    audio_transcoders[slot].initialized = 0;
    audio_transcoder_stream_names[slot][0] = '\0';

    return -1;
}

/**
 * Cleanup audio transcoder for a stream
 *
 * @param stream_name Name of the stream
 */
void cleanup_audio_transcoder(const char *stream_name) {
    pthread_mutex_lock(&audio_transcoder_mutex);

    for (int i = 0; i < g_config.max_streams; i++) {
        if (audio_transcoder_stream_names[i][0] != '\0' &&
            strcmp(audio_transcoder_stream_names[i], stream_name) == 0) {
            // Found the transcoder for this stream
            if (audio_transcoders[i].faac_enc) {
                faac_adapter_close(&audio_transcoders[i].faac_enc);
            }

            if (audio_transcoders[i].decoder_ctx) {
                avcodec_free_context(&audio_transcoders[i].decoder_ctx);
                audio_transcoders[i].decoder_ctx = NULL;
            }

            if (audio_transcoders[i].swr_ctx) {
                swr_free(&audio_transcoders[i].swr_ctx);
                audio_transcoders[i].swr_ctx = NULL;
            }

            if (audio_transcoders[i].fifo) {
                av_audio_fifo_free(audio_transcoders[i].fifo);
                audio_transcoders[i].fifo = NULL;
            }

            if (audio_transcoders[i].frame) {
                av_frame_free(&audio_transcoders[i].frame);
                audio_transcoders[i].frame = NULL;
            }

            if (audio_transcoders[i].resampled_frame) {
                av_frame_free(&audio_transcoders[i].resampled_frame);
                audio_transcoders[i].resampled_frame = NULL;
            }

            if (audio_transcoders[i].in_pkt) {
                av_packet_free(&audio_transcoders[i].in_pkt);
                audio_transcoders[i].in_pkt = NULL;
            }

            if (audio_transcoders[i].out_pkt) {
                av_packet_free(&audio_transcoders[i].out_pkt);
                audio_transcoders[i].out_pkt = NULL;
            }

            audio_transcoders[i].initialized = 0;
            audio_transcoders[i].voice_enhancement_enabled = false;
            // filter_graph et al. will be freed here once the real filter
            // chain lands (discussion #395) — currently always NULL.
            audio_transcoder_stream_names[i][0] = '\0';

            log_info("Cleaned up audio transcoder for stream %s", stream_name);
            break;
        }
    }

    // Drop any staged voice-enhancement opt-in for this stream too.
    for (int i = 0; i < g_config.max_streams; i++) {
        if (voice_enhancement_pending[i].stream_name[0] != '\0' &&
            strcmp(voice_enhancement_pending[i].stream_name, stream_name) == 0) {
            voice_enhancement_pending[i].stream_name[0] = '\0';
            voice_enhancement_pending[i].enabled = false;
            break;
        }
    }

    pthread_mutex_unlock(&audio_transcoder_mutex);
}

void set_audio_voice_enhancement(const char *stream_name, bool enabled) {
    if (!stream_name || stream_name[0] == '\0') {
        return;
    }

    pthread_mutex_lock(&audio_transcoder_mutex);

    // If the transcoder slot already exists, flip the flag in place.  The
    // future filter-graph code reads this on the next packet and rebuilds
    // the graph if the state changed.
    for (int i = 0; i < g_config.max_streams; i++) {
        if (audio_transcoders[i].initialized &&
            audio_transcoder_stream_names[i][0] != '\0' &&
            strcmp(audio_transcoder_stream_names[i], stream_name) == 0) {
            if (audio_transcoders[i].voice_enhancement_enabled != enabled) {
                audio_transcoders[i].voice_enhancement_enabled = enabled;
                log_info("Audio voice enhancement %s for stream %s (live)",
                         enabled ? "enabled" : "disabled", stream_name);
            }
            pthread_mutex_unlock(&audio_transcoder_mutex);
            return;
        }
    }

    // No slot yet — stage the preference so init_audio_transcoder() picks
    // it up when the slot is allocated on the first audio packet.
    int free_slot = -1;
    for (int i = 0; i < g_config.max_streams; i++) {
        if (voice_enhancement_pending[i].stream_name[0] != '\0' &&
            strcmp(voice_enhancement_pending[i].stream_name, stream_name) == 0) {
            voice_enhancement_pending[i].enabled = enabled;
            pthread_mutex_unlock(&audio_transcoder_mutex);
            return;
        }
        if (free_slot < 0 && voice_enhancement_pending[i].stream_name[0] == '\0') {
            free_slot = i;
        }
    }

    if (free_slot >= 0) {
        safe_strcpy(voice_enhancement_pending[free_slot].stream_name, stream_name,
                    MAX_STREAM_NAME, 0);
        voice_enhancement_pending[free_slot].enabled = enabled;
    } else {
        log_warn("No staging slot available for voice-enhancement opt-in (stream=%s)",
                 stream_name);
    }

    pthread_mutex_unlock(&audio_transcoder_mutex);
}

bool get_audio_voice_enhancement(const char *stream_name) {
    if (!stream_name || stream_name[0] == '\0') {
        return false;
    }

    bool enabled = false;
    pthread_mutex_lock(&audio_transcoder_mutex);

    // Prefer the live transcoder slot if one exists — that's the value the
    // transcode path actually uses.
    for (int i = 0; i < g_config.max_streams; i++) {
        if (audio_transcoders[i].initialized &&
            audio_transcoder_stream_names[i][0] != '\0' &&
            strcmp(audio_transcoder_stream_names[i], stream_name) == 0) {
            enabled = audio_transcoders[i].voice_enhancement_enabled;
            pthread_mutex_unlock(&audio_transcoder_mutex);
            return enabled;
        }
    }

    // Otherwise report any staged opt-in that init_audio_transcoder() will pick
    // up when the slot is first allocated.
    for (int i = 0; i < g_config.max_streams; i++) {
        if (voice_enhancement_pending[i].stream_name[0] != '\0' &&
            strcmp(voice_enhancement_pending[i].stream_name, stream_name) == 0) {
            enabled = voice_enhancement_pending[i].enabled;
            break;
        }
    }

    pthread_mutex_unlock(&audio_transcoder_mutex);
    return enabled;
}

static int encode_faac_block(audio_transcoder_t *transcoder, const void *pcm,
                             uint32_t total_samples, int samples_per_channel,
                             AVPacket *packet)
{
    uint32_t capacity = faac_adapter_get_max_output_bytes(transcoder->faac_enc);
    uint8_t *buffer = av_malloc(capacity);
    if (!buffer) {
        recording_io_report_failure(RECORDING_IO_RESOURCE_RECORDING,
                                    RECORDING_IO_OPERATION_ALLOCATE, ENOMEM);
        return AVERROR(ENOMEM);
    }

    uint32_t bytes_written = 0;
    int ret = faac_adapter_encode(transcoder->faac_enc, pcm, total_samples,
                                  buffer, capacity, &bytes_written);
    if (ret < 0) {
        av_free(buffer);
        recording_io_report_failure(RECORDING_IO_RESOURCE_RECORDING,
                                    RECORDING_IO_OPERATION_PACKET, ret);
        return AVERROR_EXTERNAL;
    }

    if (total_samples > 0) {
        transcoder->faac_input_samples += av_rescale(samples_per_channel,
            faac_adapter_get_sample_rate(transcoder->faac_enc),
            transcoder->decoder_ctx->sample_rate);
    }
    av_packet_unref(packet);
    if (bytes_written > 0) {
        ret = av_new_packet(packet, (int)bytes_written);
        if (ret == AVERROR(ENOMEM)) {
            recording_io_report_failure(RECORDING_IO_RESOURCE_RECORDING,
                                        RECORDING_IO_OPERATION_ALLOCATE, ret);
        }
        if (ret >= 0) {
            memcpy(packet->data, buffer, bytes_written);
            transcoder->faac_output_frames++;
            packet->pts = packet->dts = transcoder->next_faac_pts;
            packet->duration = faac_adapter_get_frame_samples(transcoder->faac_enc);
            packet->time_base = (AVRational){1, (int)faac_adapter_get_sample_rate(transcoder->faac_enc)};
            transcoder->next_faac_pts += packet->duration;
        }
    }
    av_free(buffer);
    return ret < 0 ? ret : 0;
}

static int append_packet(AVPacket ***packets, int *count, AVPacket *packet)
{
    AVPacket **grown = av_realloc_array(*packets, (size_t)*count + 1,
                                        sizeof(**packets));
    if (!grown) {
        recording_io_report_failure(RECORDING_IO_RESOURCE_RECORDING,
                                    RECORDING_IO_OPERATION_ALLOCATE, ENOMEM);
        return AVERROR(ENOMEM);
    }
    *packets = grown;
    (*packets)[(*count)++] = packet;
    return 0;
}

static int encode_faac_and_collect(audio_transcoder_t *transcoder,
                                   const void *pcm, uint32_t total_samples,
                                   int samples_per_channel,
                                   AVPacket ***packets, int *packet_count,
                                   bool *produced_packet)
{
    *produced_packet = false;
    AVPacket *packet = av_packet_alloc();
    if (!packet) {
        recording_io_report_failure(RECORDING_IO_RESOURCE_RECORDING,
                                    RECORDING_IO_OPERATION_ALLOCATE, ENOMEM);
        return AVERROR(ENOMEM);
    }
    int ret = encode_faac_block(transcoder, pcm, total_samples,
                                samples_per_channel, packet);
    if (ret >= 0 && packet->size > 0) {
        ret = append_packet(packets, packet_count, packet);
        if (ret >= 0) {
            *produced_packet = true;
            return 0;
        }
    }
    av_packet_free(&packet);
    return ret;
}

/* Produce one access unit, consuming buffered full frames until FAAC emits
 * output or needs more PCM. A zero-sized output means the caller may stop
 * draining; it must never leave complete PCM frames waiting behind priming. */
static int receive_audio_packet(audio_transcoder_t *t, AVPacket *packet)
{
    av_packet_unref(packet);
    if (t->faac_flushed) return AVERROR_EOF;
    int frame_size = get_encoder_frame_size(t);
#if LIBAVCODEC_VERSION_INT >= AV_VERSION_INT(59, 37, 100)
    int channels = t->decoder_ctx->ch_layout.nb_channels;
#else
    int channels = t->decoder_ctx->channels;
#endif
    if (channels < 1) channels = 1;
    while (av_audio_fifo_size(t->fifo) >= frame_size) {
        AVFrame *f = t->resampled_frame;
        av_frame_unref(f);
        f->format = AV_SAMPLE_FMT_S16;
        f->sample_rate = t->decoder_ctx->sample_rate;
        f->nb_samples = frame_size;
#if LIBAVCODEC_VERSION_INT >= AV_VERSION_INT(59, 37, 100)
        av_channel_layout_default(&f->ch_layout, channels);
#else
        f->channel_layout = av_get_default_channel_layout(channels);
        f->channels = channels;
#endif
        int ret = av_frame_get_buffer(f, 0);
        if (ret < 0) return ret;
        ret = av_audio_fifo_read(t->fifo, (void **)f->data, frame_size);
        if (ret != frame_size) return AVERROR(EINVAL);
        ret = encode_faac_block(t, f->data[0], (uint32_t)frame_size * channels,
                               frame_size, packet);
        if (ret < 0 || packet->size > 0) return ret;
    }
    return 0;
}

/**
 * Transcode an audio packet from PCM (μ-law, A-law, S16LE, etc.) to AAC
 *
 * @param stream_name Name of the stream
 * @param in_pkt Input packet (PCM format)
 * @param out_pkt Output packet (AAC) - must be allocated by caller
 * @param input_stream Original input stream
 * @return 0 on success, negative on error
 */
static int transcode_audio_packet_locked(const char *stream_name,
                          const AVPacket *in_pkt,
                          AVPacket *out_pkt,
                          const AVStream *input_stream) {
    int ret = 0;
    int transcoder_idx = -1;

    // Find the transcoder for this stream. audio_transcoder_mutex is held.
    for (int i = 0; i < g_config.max_streams; i++) {
        if (audio_transcoders[i].initialized &&
            audio_transcoder_stream_names[i][0] != '\0' &&
            strcmp(audio_transcoder_stream_names[i], stream_name) == 0) {
            transcoder_idx = i;
            break;
        }
    }

    if (transcoder_idx == -1) {
        if (!in_pkt) return 0;
        // Initialize a new transcoder
        transcoder_idx = init_audio_transcoder(stream_name,
                                             input_stream->codecpar,
                                             &input_stream->time_base);
        if (transcoder_idx == -1) {
            log_error("Failed to initialize audio transcoder for %s", stream_name);
            return -1;
        }
    }

    if (!in_pkt)
        return receive_audio_packet(&audio_transcoders[transcoder_idx], out_pkt);
    if (audio_transcoders[transcoder_idx].faac_flushed) return AVERROR_EOF;

    // Copy input packet to our internal packet
    av_packet_unref(audio_transcoders[transcoder_idx].in_pkt);
    ret = av_packet_ref(audio_transcoders[transcoder_idx].in_pkt, in_pkt);
    if (ret < 0) {
        log_ffmpeg_error(ret, "Failed to copy input packet");
        return ret;
    }

    // Send packet to decoder
    ret = avcodec_send_packet(audio_transcoders[transcoder_idx].decoder_ctx,
                             audio_transcoders[transcoder_idx].in_pkt);
    if (ret < 0) {
        log_ffmpeg_error(ret, "Failed to send packet to decoder");
        return ret;
    }

    // Receive frame from decoder
    ret = avcodec_receive_frame(audio_transcoders[transcoder_idx].decoder_ctx,
                               audio_transcoders[transcoder_idx].frame);
    if (ret < 0) {
        if (ret == AVERROR(EAGAIN)) {
            // Need more input; not an error
            return 0;
        }
        if (ret == AVERROR_EOF) {
            // End of stream; propagate EOF so caller can distinguish it
            return ret;
        }
        log_ffmpeg_error(ret, "Failed to receive frame from decoder");
        return ret;
    }

    // Convert to interleaved S16 and buffer independently of PCM packet size.
    int sr = audio_transcoders[transcoder_idx].decoder_ctx->sample_rate;
    int nb_ch = 1;
#if LIBAVCODEC_VERSION_INT >= AV_VERSION_INT(59, 37, 100)
    if (audio_transcoders[transcoder_idx].decoder_ctx->ch_layout.nb_channels > 0)
        nb_ch = audio_transcoders[transcoder_idx].decoder_ctx->ch_layout.nb_channels;
#else
    if (audio_transcoders[transcoder_idx].decoder_ctx->channels > 0)
        nb_ch = audio_transcoders[transcoder_idx].decoder_ctx->channels;
#endif
    if (nb_ch < MIN_AUDIO_CHANNELS) nb_ch = MIN_AUDIO_CHANNELS;

    enum AVSampleFormat enc_sample_fmt = AV_SAMPLE_FMT_S16;

    if (audio_transcoders[transcoder_idx].swr_ctx) {
        AVFrame *resampled = audio_transcoders[transcoder_idx].resampled_frame;
        av_frame_unref(resampled);
        resampled->format = enc_sample_fmt;
        resampled->sample_rate = sr;
#if LIBAVCODEC_VERSION_INT >= AV_VERSION_INT(59, 37, 100)
        av_channel_layout_default(&resampled->ch_layout, nb_ch);
#else
        resampled->channel_layout = av_get_default_channel_layout(nb_ch);
        resampled->channels = nb_ch;
#endif
        // Let swr determine the output nb_samples
        resampled->nb_samples = swr_get_out_samples(audio_transcoders[transcoder_idx].swr_ctx,
                                                     audio_transcoders[transcoder_idx].frame->nb_samples);
        if (resampled->nb_samples <= 0) {
            resampled->nb_samples = audio_transcoders[transcoder_idx].frame->nb_samples;
        }

        ret = av_frame_get_buffer(resampled, 0);
        if (ret < 0) {
            if (ret == AVERROR(ENOMEM)) {
                recording_io_report_failure(RECORDING_IO_RESOURCE_RECORDING,
                                            RECORDING_IO_OPERATION_ALLOCATE,
                                            ret);
            }
            log_ffmpeg_error(ret, "Failed to allocate resampled frame buffer");
            return ret;
        }

        ret = swr_convert(audio_transcoders[transcoder_idx].swr_ctx,
                          resampled->data, resampled->nb_samples,
                          (const uint8_t **)audio_transcoders[transcoder_idx].frame->data,
                          audio_transcoders[transcoder_idx].frame->nb_samples);
        if (ret < 0) {
            log_ffmpeg_error(ret, "Failed to convert audio samples");
            return ret;
        }
        resampled->nb_samples = ret;

        // Push converted samples into the FIFO
        if (audio_transcoders[transcoder_idx].fifo) {
            ret = av_audio_fifo_write(audio_transcoders[transcoder_idx].fifo,
                                      (void **)resampled->data, resampled->nb_samples);
            if (ret < resampled->nb_samples) {
                recording_io_report_failure(RECORDING_IO_RESOURCE_RECORDING,
                                            RECORDING_IO_OPERATION_ALLOCATE,
                                            ENOMEM);
                log_error("Failed to write samples to audio FIFO for %s", stream_name);
                return AVERROR(ENOMEM);
            }
        }
    }
    ret = receive_audio_packet(&audio_transcoders[transcoder_idx], out_pkt);
    if (ret >= 0 && out_pkt->size > 0) out_pkt->stream_index = in_pkt->stream_index;
    return ret;
}

int transcode_audio_packet(const char *stream_name, const AVPacket *in_pkt,
                           AVPacket *out_pkt, const AVStream *input_stream)
{
    if (!stream_name || !out_pkt || !input_stream) return AVERROR(EINVAL);
    av_packet_unref(out_pkt);
    pthread_mutex_lock(&audio_transcoder_mutex);
    int ret = transcode_audio_packet_locked(stream_name, in_pkt, out_pkt, input_stream);
    pthread_mutex_unlock(&audio_transcoder_mutex);
    return ret;
}

static int transcode_audio_flush_locked(const char *stream_name, AVPacket ***packets_out,
                          int *packet_count_out, int *discard_padding_out)
{
    if (!stream_name || !packets_out || !packet_count_out || !discard_padding_out)
        return AVERROR(EINVAL);
    *packets_out = NULL;
    *packet_count_out = 0;
    *discard_padding_out = 0;
    audio_transcoder_t *t = NULL;
    for (int i = 0; i < g_config.max_streams; i++) {
        if (audio_transcoders[i].initialized && audio_transcoder_stream_names[i][0] &&
            strcmp(audio_transcoder_stream_names[i], stream_name) == 0) {
            t = &audio_transcoders[i]; break;
        }
    }
    if (!t || !t->faac_enc || t->faac_flushed) return 0;

    /* swr may retain samples for filter delay even when input and output rates
       match. Drain those before the FIFO's final partial frame. */
    if (t->swr_ctx && t->fifo) {
        for (;;) {
            int cap = swr_get_out_samples(t->swr_ctx, 0);
            if (cap <= 0) break;
            AVFrame *f = t->resampled_frame;
            av_frame_unref(f);
            f->format = AV_SAMPLE_FMT_S16;
            f->sample_rate = t->decoder_ctx->sample_rate;
            f->nb_samples = cap;
#if LIBAVCODEC_VERSION_INT >= AV_VERSION_INT(59, 37, 100)
            av_channel_layout_copy(&f->ch_layout, &t->decoder_ctx->ch_layout);
#else
            f->channel_layout = t->decoder_ctx->channel_layout;
            f->channels = t->decoder_ctx->channels;
#endif
            int ret = av_frame_get_buffer(f, 0);
            if (ret < 0) return ret;
            ret = swr_convert(t->swr_ctx, f->data, cap, NULL, 0);
            if (ret < 0) return ret;
            if (ret == 0) break;
            f->nb_samples = ret;
            int converted_samples = ret;
            ret = av_audio_fifo_write(t->fifo, (void **)f->data, converted_samples);
            if (ret != converted_samples) return AVERROR(ENOMEM);
        }
    }

#if LIBAVCODEC_VERSION_INT < AV_VERSION_INT(59, 37, 100)
    int channels = t->decoder_ctx->channels;
#else
    int channels = t->decoder_ctx->ch_layout.nb_channels;
#endif
    if (channels < 1) channels = 1;
    int pending = t->fifo ? av_audio_fifo_size(t->fifo) : 0;
    while (pending > 0) {
        int block_samples = FFMIN(pending, get_encoder_frame_size(t));
        AVFrame *f = t->resampled_frame;
        av_frame_unref(f);
        f->format = AV_SAMPLE_FMT_S16; f->sample_rate = t->decoder_ctx->sample_rate;
        f->nb_samples = block_samples;
#if LIBAVCODEC_VERSION_INT >= AV_VERSION_INT(59, 37, 100)
        av_channel_layout_default(&f->ch_layout, channels);
#else
        f->channel_layout = av_get_default_channel_layout(channels); f->channels = channels;
#endif
        int ret = av_frame_get_buffer(f, 0); if (ret < 0) return ret;
        ret = av_audio_fifo_read(t->fifo, (void **)f->data, block_samples);
        if (ret != block_samples) return AVERROR(EINVAL);
        bool produced_packet;
        ret = encode_faac_and_collect(t, f->data[0],
                                      (uint32_t)block_samples * channels, block_samples,
                                      packets_out, packet_count_out,
                                      &produced_packet);
        if (ret < 0) return ret;
        pending -= block_samples;
    }

    /* FAAC explicitly requires zero-sample calls until it returns no bytes. */
    for (;;) {
        bool produced_packet;
        int ret = encode_faac_and_collect(t, NULL, 0, 0,
                                          packets_out, packet_count_out,
                                          &produced_packet);
        if (ret < 0) return ret;
        if (!produced_packet) break;
    }
    t->faac_flushed = true;
    uint64_t coded = t->faac_output_frames * faac_adapter_get_frame_samples(t->faac_enc);
    uint64_t accounted = faac_adapter_get_encoder_delay(t->faac_enc) + t->faac_input_samples;
    if (coded > accounted && coded - accounted <= INT_MAX)
        *discard_padding_out = (int)(coded - accounted);
    return 0;
}

int transcode_audio_flush(const char *stream_name, AVPacket ***packets_out,
                          int *packet_count_out, int *discard_padding_out)
{
    pthread_mutex_lock(&audio_transcoder_mutex);
    int ret = transcode_audio_flush_locked(stream_name, packets_out, packet_count_out,
                                           discard_padding_out);
    pthread_mutex_unlock(&audio_transcoder_mutex);
    return ret;
}

/**
 * Transcode audio from PCM (μ-law, A-law, S16LE, etc.) to AAC format
 *
 * @param codec_params Original codec parameters (PCM format)
 * @param time_base Time base of the original stream
 * @param stream_name Name of the stream (for logging)
 * @param transcoded_params Output parameter to store the transcoded codec parameters
 * @return 0 on success, negative on error
 */
int transcode_pcm_to_aac(const AVCodecParameters *codec_params,
                                 const AVRational *time_base,
                                 const char *stream_name,
                                 AVCodecParameters **transcoded_params) {
    int ret = 0;
    AVCodecContext *decoder_ctx = NULL;
    const AVCodec *decoder = NULL;

    // Allocate output codec parameters
    *transcoded_params = avcodec_parameters_alloc();
    if (!*transcoded_params) {
        recording_io_report_failure(RECORDING_IO_RESOURCE_RECORDING,
                                    RECORDING_IO_OPERATION_ALLOCATE, ENOMEM);
        log_error("Failed to allocate transcoded codec parameters for %s",
                stream_name ? stream_name : "unknown");
        return AVERROR(ENOMEM);
    }

    // Find the PCM decoder for the specific codec
    decoder = avcodec_find_decoder(codec_params->codec_id);
    if (!decoder) {
        log_error("Failed to find decoder for PCM audio (codec_id=%d) in %s",
                codec_params->codec_id, stream_name ? stream_name : "unknown");
        ret = AVERROR_DECODER_NOT_FOUND;
        goto cleanup;
    }

    // Create decoder context
    decoder_ctx = avcodec_alloc_context3(decoder);
    if (!decoder_ctx) {
        recording_io_report_failure(RECORDING_IO_RESOURCE_RECORDING,
                                    RECORDING_IO_OPERATION_ALLOCATE, ENOMEM);
        log_error("Failed to allocate decoder context for %s",
                stream_name ? stream_name : "unknown");
        ret = AVERROR(ENOMEM);
        goto cleanup;
    }

    // Copy parameters to decoder context
    ret = avcodec_parameters_to_context(decoder_ctx, codec_params);
    if (ret < 0) {
        log_ffmpeg_error(ret, "Failed to copy parameters to decoder context");
        goto cleanup;
    }

    // Set time base
    decoder_ctx->time_base = *time_base;

    // Open decoder
    ret = avcodec_open2(decoder_ctx, decoder, NULL);
    if (ret < 0) {
        log_ffmpeg_error(ret, "Failed to open PCM decoder");
        goto cleanup;
    }

    int sr = decoder_ctx->sample_rate;
    int nb_ch = 1;
#if LIBAVCODEC_VERSION_INT >= AV_VERSION_INT(59, 37, 100)
    if (decoder_ctx->ch_layout.nb_channels > 0)
        nb_ch = decoder_ctx->ch_layout.nb_channels;
#else
    if (decoder_ctx->channels > 0)
        nb_ch = decoder_ctx->channels;
#endif
    if (nb_ch < MIN_AUDIO_CHANNELS) nb_ch = MIN_AUDIO_CHANNELS;

    faac_encoder_config_t faac_cfg = make_faac_config(sr, nb_ch);

    faac_adapter_t *faac_enc = faac_adapter_open(&faac_cfg);
    if (!faac_enc) {
        log_error("Failed to open FAAC encoder for %s", stream_name ? stream_name : "unknown");
        ret = -1;
        goto cleanup;
    }

    int profile = faac_adapter_get_object_type(faac_enc) == FAAC_OBJ_HE_AAC_V1
        ? AV_PROFILE_AAC_HE
        : AV_PROFILE_AAC_LOW;

    (*transcoded_params)->codec_type = AVMEDIA_TYPE_AUDIO;
    (*transcoded_params)->codec_id = AV_CODEC_ID_AAC;
    /* FAAC keeps the configured full output rate while coding HE-AAC's core
     * at half rate; its access units therefore span 2048 full-rate samples. */
    (*transcoded_params)->sample_rate = (int)faac_adapter_get_sample_rate(faac_enc);
    (*transcoded_params)->bit_rate = faac_cfg.bit_rate_per_channel * nb_ch;
    (*transcoded_params)->profile = profile;
    (*transcoded_params)->frame_size = faac_adapter_get_frame_samples(faac_enc);
    (*transcoded_params)->initial_padding = faac_adapter_get_encoder_delay(faac_enc);

#if LIBAVCODEC_VERSION_INT >= AV_VERSION_INT(59, 37, 100)
    av_channel_layout_copy(&(*transcoded_params)->ch_layout, &decoder_ctx->ch_layout);
    if ((*transcoded_params)->ch_layout.nb_channels == 0) {
        av_channel_layout_default(&(*transcoded_params)->ch_layout, nb_ch);
    }
#else
    (*transcoded_params)->channels = nb_ch;
    (*transcoded_params)->channel_layout = av_get_default_channel_layout(nb_ch);
#endif

    const uint8_t *asc = NULL;
    size_t asc_len = 0;
    if (faac_adapter_get_asc(faac_enc, &asc, &asc_len) < 0 || asc_len == 0) {
        log_error("FAAC did not provide AudioSpecificConfig for %s", stream_name ? stream_name : "unknown");
        faac_adapter_close(&faac_enc);
        ret = AVERROR_INVALIDDATA;
        goto cleanup;
    }
    {
        (*transcoded_params)->extradata = av_malloc(asc_len + AV_INPUT_BUFFER_PADDING_SIZE);
        if (!(*transcoded_params)->extradata) {
            recording_io_report_failure(RECORDING_IO_RESOURCE_RECORDING, RECORDING_IO_OPERATION_ALLOCATE, ENOMEM);
            faac_adapter_close(&faac_enc);
            ret = AVERROR(ENOMEM);
            goto cleanup;
        }
        memcpy((*transcoded_params)->extradata, asc, asc_len);
        memset((*transcoded_params)->extradata + asc_len, 0, AV_INPUT_BUFFER_PADDING_SIZE);
        (*transcoded_params)->extradata_size = (int)asc_len;
    }
    faac_adapter_close(&faac_enc);

    log_info("Successfully configured transcoding from PCM to AAC via FAAC for %s (Sample rate: %d, Channels: %d, Delay: %d)",
            stream_name ? stream_name : "unknown", sr, nb_ch, (*transcoded_params)->initial_padding);
    ret = 0;

cleanup:
    if (decoder_ctx) avcodec_free_context(&decoder_ctx);

    if (ret < 0 && *transcoded_params) {
        avcodec_parameters_free(transcoded_params);
        *transcoded_params = NULL;
    }

    return ret;
}

/**
 * Check if a codec is a PCM variant
 *
 * @param codec_id The codec ID to check
 * @return true if it's a PCM codec, false otherwise
 */
bool is_pcm_codec(enum AVCodecID codec_id) {
    switch (codec_id) {
        case AV_CODEC_ID_PCM_S16LE:
        case AV_CODEC_ID_PCM_S16BE:
        case AV_CODEC_ID_PCM_U16LE:
        case AV_CODEC_ID_PCM_U16BE:
        case AV_CODEC_ID_PCM_S8:
        case AV_CODEC_ID_PCM_U8:
        case AV_CODEC_ID_PCM_MULAW:
        case AV_CODEC_ID_PCM_ALAW:
        case AV_CODEC_ID_PCM_S32LE:
        case AV_CODEC_ID_PCM_S32BE:
        case AV_CODEC_ID_PCM_U32LE:
        case AV_CODEC_ID_PCM_U32BE:
        case AV_CODEC_ID_PCM_S24LE:
        case AV_CODEC_ID_PCM_S24BE:
        case AV_CODEC_ID_PCM_U24LE:
        case AV_CODEC_ID_PCM_U24BE:
        case AV_CODEC_ID_PCM_S24DAUD:
        case AV_CODEC_ID_PCM_ZORK:
        case AV_CODEC_ID_PCM_S16LE_PLANAR:
        case AV_CODEC_ID_PCM_DVD:
        case AV_CODEC_ID_PCM_F32BE:
        case AV_CODEC_ID_PCM_F32LE:
        case AV_CODEC_ID_PCM_F64BE:
        case AV_CODEC_ID_PCM_F64LE:
        case AV_CODEC_ID_PCM_BLURAY:
        case AV_CODEC_ID_PCM_LXF:
            return true;
        default:
            return false;
    }
}

/**
 * Check if an audio codec is compatible with MP4 format
 *
 * @param codec_id The codec ID to check
 * @param codec_name Output parameter to store the codec name
 * @return true if compatible, false otherwise
 */
bool is_audio_codec_compatible_with_mp4(enum AVCodecID codec_id, const char **codec_name) {
    bool is_compatible = true;

    switch (codec_id) {
        case AV_CODEC_ID_AAC:
            *codec_name = "AAC";
            break;
        case AV_CODEC_ID_MP3:
            *codec_name = "MP3";
            break;
        case AV_CODEC_ID_AC3:
            *codec_name = "AC3";
            break;
        case AV_CODEC_ID_OPUS:
            *codec_name = "Opus";
            break;
        case AV_CODEC_ID_PCM_MULAW:
            *codec_name = "PCM μ-law (G.711 μ-law)";
            is_compatible = false;
            break;
        case AV_CODEC_ID_PCM_ALAW:
            *codec_name = "PCM A-law (G.711 A-law)";
            is_compatible = false;
            break;
        case AV_CODEC_ID_PCM_S16LE:
            *codec_name = "PCM signed 16-bit little-endian";
            is_compatible = false;
            break;
        case AV_CODEC_ID_PCM_S16BE:
            *codec_name = "PCM signed 16-bit big-endian";
            is_compatible = false;
            break;
        case AV_CODEC_ID_PCM_U8:
            *codec_name = "PCM unsigned 8-bit";
            is_compatible = false;
            break;
        case AV_CODEC_ID_PCM_S8:
            *codec_name = "PCM signed 8-bit";
            is_compatible = false;
            break;
        default:
            // Check if it's any other PCM codec
            if (is_pcm_codec(codec_id)) {
                *codec_name = "PCM (various)";
                is_compatible = false;
            } else {
                *codec_name = "Unknown";
                // Assume compatible for unknown codecs and let FFmpeg handle it
            }
            break;
    }

    return is_compatible;
}

/**
 * Apply h264_mp4toannexb bitstream filter to convert H.264 stream from MP4 format to Annex B format
 * This is needed for some RTSP cameras that send H.264 in MP4 format instead of Annex B format
 *
 * @param packet The packet to convert
 * @param codec_id The codec ID of the stream
 * @return 0 on success, negative on error
 */
int apply_h264_annexb_filter(AVPacket *packet, enum AVCodecID codec_id) {
    // Only apply to H.264 streams
    if (codec_id != AV_CODEC_ID_H264) {
        return 0;
    }

    // Check if the packet already has start codes (Annex B format)
    if (packet->size >= 4 &&
        packet->data[0] == 0x00 &&
        packet->data[1] == 0x00 &&
        packet->data[2] == 0x00 &&
        packet->data[3] == 0x01) {
        // Already in Annex B format
        return 0;
    }

    // MEMORY LEAK FIX: Use a more robust approach with AVPacket functions
    // This avoids potential memory leaks by using FFmpeg's memory management

    // Create a new packet for the filtered data
    AVPacket *new_pkt = av_packet_alloc();
    if (!new_pkt) {
        recording_io_report_failure(RECORDING_IO_RESOURCE_RECORDING,
                                    RECORDING_IO_OPERATION_ALLOCATE, ENOMEM);
        return AVERROR(ENOMEM);
    }

    // Allocate a new buffer with space for the start code + original data
    int new_size = packet->size + 4;
    int ret = av_new_packet(new_pkt, new_size);
    if (ret < 0) {
        if (ret == AVERROR(ENOMEM)) {
            recording_io_report_failure(RECORDING_IO_RESOURCE_RECORDING,
                                        RECORDING_IO_OPERATION_ALLOCATE, ret);
        }
        av_packet_free(&new_pkt);
        return ret;
    }

    // Add start code
    new_pkt->data[0] = 0x00;
    new_pkt->data[1] = 0x00;
    new_pkt->data[2] = 0x00;
    new_pkt->data[3] = 0x01;

    // Copy the packet data
    memcpy(new_pkt->data + 4, packet->data, packet->size);

    // Copy other packet properties
    new_pkt->pts = packet->pts;
    new_pkt->dts = packet->dts;
    new_pkt->flags = packet->flags;
    new_pkt->stream_index = packet->stream_index;

    // Unref the original packet
    av_packet_unref(packet);

    // Move the new packet to the original packet
    av_packet_move_ref(packet, new_pkt);

    // Free the new packet
    av_packet_free(&new_pkt);

    return 0;
}

/**
 * Enhanced MP4 writer initialization with better path handling and logging
 * and proper audio stream handling
 *
 *  Only initialize on keyframes for video packets to ensure clean recordings
 *  Properly handle audio stream initialization
 */
int mp4_writer_initialize(mp4_writer_t *writer, const AVPacket *pkt, const AVStream *input_stream) {
    int ret;

    // Ensure we only initialize on keyframes for video packets
    if (input_stream->codecpar->codec_type == AVMEDIA_TYPE_VIDEO) {
        // Check if this is a keyframe
        bool is_keyframe = (pkt->flags & AV_PKT_FLAG_KEY) != 0;

        // If this is not a keyframe, wait for the next keyframe
        if (!is_keyframe) {
            log_info("Waiting for keyframe to start MP4 recording for %s",
                    writer->stream_name ? writer->stream_name : "unknown");
            return -1;  // Return error to indicate we should wait for a keyframe
        }

        log_info("Starting MP4 recording with keyframe for %s",
                writer->stream_name ? writer->stream_name : "unknown");
    }

    // First, ensure the directory exists
    char *dir_path = malloc(PATH_MAX);
    if (!dir_path) {
        recording_io_report_failure(RECORDING_IO_RESOURCE_RECORDING,
                                    RECORDING_IO_OPERATION_ALLOCATE, ENOMEM);
        log_error("Failed to allocate memory for directory path");
        return -1;
    }

    // Initialize dir_path to empty string to avoid potential issues
    dir_path[0] = '\0';

    if (!writer->output_path) {
        log_error("MP4 writer output_path is NULL");
        free(dir_path);
        return -1;
    }

    const char *last_slash = strrchr(writer->output_path, '/');
    if (last_slash) {
        size_t dir_len = last_slash - writer->output_path;
        safe_strcpy(dir_path, writer->output_path, PATH_MAX, dir_len);

        // Log the directory we're working with
        log_info("Ensuring MP4 output directory exists: %s", dir_path);
    } else {
        // No directory separator found, use current directory
        log_warn("No directory separator found in output path: %s, using current directory",
                writer->output_path);
        dir_path[0] = '.';
        dir_path[1] = '\0';

        // Log the directory we're working with (current directory)
        log_info("Ensuring MP4 output directory exists: %s", dir_path);
    }

    // Create directory if it doesn't exist
    if (mkdir_recursive(dir_path) != 0) {
        int directory_error = errno;
        recording_io_report_failure(RECORDING_IO_RESOURCE_RECORDING,
                                    RECORDING_IO_OPERATION_FILESYSTEM,
                                    directory_error);
        log_warn("Failed to create directory: %s", dir_path);
    }

    // Set permissions to ensure it's writable
    if (chmod_path(dir_path, 0755)) {
        int permission_error = errno;
        recording_io_report_failure(RECORDING_IO_RESOURCE_RECORDING,
                                    RECORDING_IO_OPERATION_FILESYSTEM,
                                    permission_error);
        log_warn("Failed to set permissions: %s", dir_path);
    }

    // Log the full output path
    log_info("Initializing MP4 writer to output file: %s", writer->output_path);

    // We'll use dir_path directly for error handling, no need to duplicate it

    // Create output format context
    ret = avformat_alloc_output_context2(&writer->output_ctx, NULL, "mp4", writer->output_path);
    if (ret < 0 || !writer->output_ctx) {
        recording_io_report_failure(RECORDING_IO_RESOURCE_RECORDING,
                                    RECORDING_IO_OPERATION_ALLOCATE,
                                    ret < 0 ? ret : ENOMEM);
        char error_buf[AV_ERROR_MAX_STRING_SIZE] = {0};
        av_strerror(ret, error_buf, AV_ERROR_MAX_STRING_SIZE);
        log_error("Failed to create output format context for MP4 writer: %s", error_buf);
        free(dir_path);  // Free dir_path before returning
        return -1;
    }

    // Respect the existing has_audio setting (set by mp4_writer_set_audio or from DB config)
    // Do NOT override it here — the caller is responsible for configuring audio before init
    log_info("Audio recording %s for stream %s (preserving configured setting)",
            writer->has_audio ? "enabled" : "disabled", writer->stream_name);

    // Add video stream only if this is a video packet
    if (input_stream->codecpar->codec_type == AVMEDIA_TYPE_VIDEO) {
        AVStream *out_stream = avformat_new_stream(writer->output_ctx, NULL);
        if (!out_stream) {
            recording_io_report_failure(RECORDING_IO_RESOURCE_RECORDING,
                                        RECORDING_IO_OPERATION_ALLOCATE,
                                        ENOMEM);
            log_error("Failed to create output stream for MP4 writer");
            avformat_free_context(writer->output_ctx);
            writer->output_ctx = NULL;
            free(dir_path);  // Free dir_path before returning
            return -1;
        }

        // Copy codec parameters
        ret = avcodec_parameters_copy(out_stream->codecpar, input_stream->codecpar);
        if (ret < 0) {
            char error_buf[AV_ERROR_MAX_STRING_SIZE] = {0};
            av_strerror(ret, error_buf, AV_ERROR_MAX_STRING_SIZE);
            log_error("Failed to copy codec parameters for MP4 writer: %s", error_buf);
            avformat_free_context(writer->output_ctx);
            writer->output_ctx = NULL;
            free(dir_path);  // Free dir_path before returning
            return -1;
        }

        // BUGFIX: Zero out codec_tag so the MP4 muxer selects the correct tag.
        // RTSP/RTP uses different codec tags than MP4; carrying over the input
        // tag produces a malformed moov atom (grey screen in players).
        out_stream->codecpar->codec_tag = 0;

        // CRITICAL FIX: Check for unspecified video dimensions (0x0) and set default values
        // This prevents the "dimensions not set" error and segmentation fault
        if (out_stream->codecpar->width == 0 || out_stream->codecpar->height == 0) {
            log_warn("Video dimensions not set (width=%d, height=%d) for stream %s, using default values",
                    out_stream->codecpar->width, out_stream->codecpar->height,
                    writer->stream_name ? writer->stream_name : "unknown");

            // Set default dimensions (640x480 is a safe choice)
            out_stream->codecpar->width = 640;
            out_stream->codecpar->height = 480;

            log_info("Set default video dimensions to %dx%d for stream %s",
                    out_stream->codecpar->width, out_stream->codecpar->height,
                    writer->stream_name ? writer->stream_name : "unknown");
        }

        // Log H.264 codec info and extradata status for diagnostics
        if (input_stream->codecpar->codec_id == AV_CODEC_ID_H264) {
            log_info("H.264 video stream for %s — extradata (SPS/PPS) %s (%d bytes)",
                    writer->stream_name ? writer->stream_name : "unknown",
                    out_stream->codecpar->extradata ? "present" : "MISSING",
                    out_stream->codecpar->extradata_size);
            // NOTE: Do NOT delete extradata here.  The MP4 container requires
            // SPS/PPS in the avcC box for decoders to initialize.  Removing it
            // produces unplayable grey-screen files.
        }

        // Set stream time base
        out_stream->time_base = input_stream->time_base;
        writer->time_base = input_stream->time_base;

        // Store video stream index
        writer->video_stream_idx = 0;  // First stream is video

        // Declare the audio stream NOW, before avformat_write_header().
        // For MP4 containers all streams must be registered before the header is
        // written — adding a stream afterwards is illegal and silently produces
        // video-only files even if the transcoder later initialises successfully.
        // pending_audio_codecpar is populated by udt_start_recording() via the
        // stateless transcode_pcm_to_aac(); it is NULL for non-UDT paths (e.g.
        // record_segment) which handle audio registration themselves.
        if (writer->has_audio && writer->pending_audio_codecpar) {
            AVStream *a_stream = avformat_new_stream(writer->output_ctx, NULL);
            if (a_stream) {
                int aret = avcodec_parameters_copy(a_stream->codecpar,
                                                   writer->pending_audio_codecpar);
                if (aret < 0) {
                    char errbuf[AV_ERROR_MAX_STRING_SIZE] = {0};
                    av_strerror(aret, errbuf, AV_ERROR_MAX_STRING_SIZE);
                    log_error("Failed to copy pending audio codec params for %s: %s",
                              writer->stream_name ? writer->stream_name : "unknown", errbuf);
                    // The newly created stream is partially configured — leaving it in
                    // output_ctx would produce an invalid stream table and cause
                    // avformat_write_header() to fail with an opaque error.
                    // Treat this as a hard init failure and clean up entirely.
                    avcodec_parameters_free(&writer->pending_audio_codecpar);
                    writer->pending_audio_codecpar = NULL;
                    avformat_free_context(writer->output_ctx);
                    writer->output_ctx = NULL;
                    free(dir_path);
                    return -1;
                } else {
                    a_stream->codecpar->codec_tag = 0;
                    // Use the stored input time_base directly. Reconstructing from
                    // sample_rate is unsafe: it can be 0 for some pass-through codecs,
                    // producing an invalid {1,0} time base. Fall back only when needed.
                    if (writer->pending_audio_time_base.den > 0) {
                        a_stream->time_base = writer->pending_audio_time_base;
                    } else if (writer->pending_audio_codecpar->sample_rate > 0) {
                        a_stream->time_base = (AVRational){1, writer->pending_audio_codecpar->sample_rate};
                    } else {
                        a_stream->time_base = (AVRational){1, 48000}; // safe default
                    }
                    writer->audio.stream_idx = a_stream->index;
                    writer->audio.time_base  = a_stream->time_base;
                    log_info("Audio stream (AAC) declared at index %d before header write for %s",
                             writer->audio.stream_idx,
                             writer->stream_name ? writer->stream_name : "unknown");
                }
            } else {
                recording_io_report_failure(RECORDING_IO_RESOURCE_RECORDING,
                                            RECORDING_IO_OPERATION_ALLOCATE,
                                            ENOMEM);
                log_error("Failed to create audio stream for %s — recording video-only",
                          writer->stream_name ? writer->stream_name : "unknown");
                writer->has_audio = 0;
            }
            avcodec_parameters_free(&writer->pending_audio_codecpar);
            writer->pending_audio_codecpar = NULL;
        } else {
            log_info("Video stream initialized for %s. No pending audio codec params (non-UDT path or audio disabled).",
                    writer->stream_name ? writer->stream_name : "unknown");
        }
    }
    else if (input_stream->codecpar->codec_type == AVMEDIA_TYPE_AUDIO) {
        // Check if the audio codec is compatible with MP4 format
        // MP4 container has limited audio codec support
        const char *codec_name = "unknown";
        bool is_compatible = is_audio_codec_compatible_with_mp4(input_stream->codecpar->codec_id, &codec_name);

        if (!is_compatible) {
            // For incompatible codecs, attempt transcoding if it's a PCM codec
            if (is_pcm_codec(input_stream->codecpar->codec_id)) {
                log_info("Attempting to transcode %s audio to AAC for MP4 compatibility in stream %s",
                        codec_name, writer->stream_name ? writer->stream_name : "unknown");

                // Try to transcode PCM to AAC
                AVCodecParameters *transcoded_params = NULL;
                int transcode_ret = transcode_pcm_to_aac(input_stream->codecpar,
                                                         &input_stream->time_base,
                                                         writer->stream_name,
                                                         &transcoded_params);

                if (transcode_ret >= 0 && transcoded_params) {
                    log_info("Successfully transcoded %s audio to AAC for stream %s",
                            codec_name, writer->stream_name ? writer->stream_name : "unknown");
                    
                    // Create a dummy video stream first (MP4 expects video to be the first stream)
                    log_warn("MP4 writer initialization triggered by transcoded audio packet for %s - creating dummy video stream",
                            writer->stream_name ? writer->stream_name : "unknown");
                    
                    AVStream *dummy_video = avformat_new_stream(writer->output_ctx, NULL);
                    if (!dummy_video) {
                        recording_io_report_failure(RECORDING_IO_RESOURCE_RECORDING,
                                                    RECORDING_IO_OPERATION_ALLOCATE,
                                                    ENOMEM);
                        log_error("Failed to create dummy video stream for MP4 writer");
                        avcodec_parameters_free(&transcoded_params);
                        avformat_free_context(writer->output_ctx);
                        writer->output_ctx = NULL;
                        free(dir_path);
                        return -1;
                    }
                    
                    // Set up a minimal H.264 video stream
                    dummy_video->codecpar->codec_type = AVMEDIA_TYPE_VIDEO;
                    dummy_video->codecpar->codec_id = AV_CODEC_ID_H264;
                    dummy_video->codecpar->width = 640;
                    dummy_video->codecpar->height = 480;
                    dummy_video->time_base = (AVRational){1, 30};
                    writer->time_base = dummy_video->time_base;
                    writer->video_stream_idx = 0;
                    
                    // Now add the transcoded audio stream
                    AVStream *audio_stream = avformat_new_stream(writer->output_ctx, NULL);
                    if (!audio_stream) {
                        recording_io_report_failure(RECORDING_IO_RESOURCE_RECORDING,
                                                    RECORDING_IO_OPERATION_ALLOCATE,
                                                    ENOMEM);
                        log_error("Failed to create audio stream for MP4 writer");
                        avcodec_parameters_free(&transcoded_params);
                        avformat_free_context(writer->output_ctx);
                        writer->output_ctx = NULL;
                        free(dir_path);
                        return -1;
                    }
                    
                    // Copy transcoded audio codec parameters
                    ret = avcodec_parameters_copy(audio_stream->codecpar, transcoded_params);
                    if (ret < 0) {
                        char error_buf[AV_ERROR_MAX_STRING_SIZE] = {0};
                        av_strerror(ret, error_buf, AV_ERROR_MAX_STRING_SIZE);
                        log_error("Failed to copy transcoded audio codec parameters for MP4 writer: %s", error_buf);
                        avcodec_parameters_free(&transcoded_params);
                        avformat_free_context(writer->output_ctx);
                        writer->output_ctx = NULL;
                        free(dir_path);
                        return -1;
                    }
                    
                    // Free the transcoded parameters now that we've copied them
                    avcodec_parameters_free(&transcoded_params);
                    
                    // Set audio stream time base
                    audio_stream->time_base = input_stream->time_base;
                    writer->audio.time_base = input_stream->time_base;
                    
                    // Store audio stream index
                    writer->audio.stream_idx = audio_stream->index;
                    writer->has_audio = 1;
                    writer->audio.initialized = 0;  // Will be initialized when we process the first audio packet
                    
                    log_info("Added transcoded audio stream at index %d during initialization for %s",
                            writer->audio.stream_idx, writer->stream_name ? writer->stream_name : "unknown");
                    
                    // Skip the rest of the audio stream handling
                    goto skip_audio_stream;
                } else {
                    log_error("Failed to transcode %s audio to AAC: %d", codec_name, transcode_ret);
                    log_error("Audio stream parameters: codec_id=%d, sample_rate=%d, channels=%d",
                             input_stream->codecpar->codec_id, input_stream->codecpar->sample_rate,
                             #if LIBAVCODEC_VERSION_INT >= AV_VERSION_INT(59, 37, 100)
                                 input_stream->codecpar->ch_layout.nb_channels
                             #else
                                 input_stream->codecpar->channels
                             #endif
                            );
                    log_error("Disabling audio recording for this stream");
                    
                    // Disable audio for this writer
                    writer->has_audio = 0;
                }
            } else {
                log_error("%s audio codec is not compatible with MP4 format", codec_name);
                log_error("Audio stream parameters: codec_id=%d, sample_rate=%d, channels=%d",
                         input_stream->codecpar->codec_id, input_stream->codecpar->sample_rate,
                         #if LIBAVCODEC_VERSION_INT >= AV_VERSION_INT(59, 37, 100)
                             input_stream->codecpar->ch_layout.nb_channels
                         #else
                             input_stream->codecpar->channels
                         #endif
                        );
                log_error("Disabling audio recording for this stream");
                
                // Disable audio for this writer
                writer->has_audio = 0;
            }

            // We still need to create a dummy video stream to initialize the MP4 writer
            // but we won't add an audio stream
            log_warn("MP4 writer initialization triggered by incompatible audio packet for %s - creating dummy video stream only",
                    writer->stream_name ? writer->stream_name : "unknown");

            AVStream *dummy_video = avformat_new_stream(writer->output_ctx, NULL);
            if (!dummy_video) {
                recording_io_report_failure(RECORDING_IO_RESOURCE_RECORDING,
                                            RECORDING_IO_OPERATION_ALLOCATE,
                                            ENOMEM);
                log_error("Failed to create dummy video stream for MP4 writer");
                avformat_free_context(writer->output_ctx);
                writer->output_ctx = NULL;
                free(dir_path);
                return -1;
            }

            // Set up a minimal H.264 video stream
            dummy_video->codecpar->codec_type = AVMEDIA_TYPE_VIDEO;
            dummy_video->codecpar->codec_id = AV_CODEC_ID_H264;
            dummy_video->codecpar->width = 640;
            dummy_video->codecpar->height = 480;
            dummy_video->time_base = (AVRational){1, 30};
            writer->time_base = dummy_video->time_base;
            writer->video_stream_idx = 0;

            // Skip adding the audio stream
            goto skip_audio_stream;
        } else {
            log_info("Using %s audio codec for stream %s",
                    codec_name, writer->stream_name ? writer->stream_name : "unknown");
        }

        // If initialization is triggered by a compatible audio packet, we need to create a dummy video stream first
        // This is because MP4 format expects video to be the first stream
        log_warn("MP4 writer initialization triggered by audio packet for %s - creating dummy video stream",
                writer->stream_name ? writer->stream_name : "unknown");

        AVStream *dummy_video = avformat_new_stream(writer->output_ctx, NULL);
        if (!dummy_video) {
            recording_io_report_failure(RECORDING_IO_RESOURCE_RECORDING,
                                        RECORDING_IO_OPERATION_ALLOCATE,
                                        ENOMEM);
            log_error("Failed to create dummy video stream for MP4 writer");
            avformat_free_context(writer->output_ctx);
            writer->output_ctx = NULL;
            free(dir_path);
            return -1;
        }

        // Set up a minimal H.264 video stream
        dummy_video->codecpar->codec_type = AVMEDIA_TYPE_VIDEO;
        dummy_video->codecpar->codec_id = AV_CODEC_ID_H264;
        dummy_video->codecpar->width = 640;
        dummy_video->codecpar->height = 480;
        dummy_video->time_base = (AVRational){1, 30};
        writer->time_base = dummy_video->time_base;
        writer->video_stream_idx = 0;

        // Now add the audio stream
        AVStream *audio_stream = avformat_new_stream(writer->output_ctx, NULL);
        if (!audio_stream) {
            recording_io_report_failure(RECORDING_IO_RESOURCE_RECORDING,
                                        RECORDING_IO_OPERATION_ALLOCATE,
                                        ENOMEM);
            log_error("Failed to create audio stream for MP4 writer");
            avformat_free_context(writer->output_ctx);
            writer->output_ctx = NULL;
            free(dir_path);
            return -1;
        }

        // Copy audio codec parameters
        ret = avcodec_parameters_copy(audio_stream->codecpar, input_stream->codecpar);
        if (ret < 0) {
            char error_buf[AV_ERROR_MAX_STRING_SIZE] = {0};
            av_strerror(ret, error_buf, AV_ERROR_MAX_STRING_SIZE);
            log_error("Failed to copy audio codec parameters for MP4 writer: %s", error_buf);
            avformat_free_context(writer->output_ctx);
            writer->output_ctx = NULL;
            free(dir_path);
            return -1;
        }

        // Set audio stream time base
        audio_stream->time_base = input_stream->time_base;
        writer->audio.time_base = input_stream->time_base;

        // Store audio stream index
        writer->audio.stream_idx = audio_stream->index;
        writer->has_audio = 1;
        writer->audio.initialized = 0;  // Will be initialized when we process the first audio packet

        log_info("Added audio stream at index %d during initialization for %s",
                writer->audio.stream_idx, writer->stream_name ? writer->stream_name : "unknown");
    }

skip_audio_stream:
    // Initialize audio state if not already done
    if (writer->audio.stream_idx == -1) {
        writer->audio.first_dts = AV_NOPTS_VALUE;
        writer->audio.last_pts = 0;
        writer->audio.last_dts = AV_NOPTS_VALUE;
        writer->audio.initialized = 0; // Explicitly initialize to 0
    }

    // Add metadata
    av_dict_set(&writer->output_ctx->metadata, "title", writer->stream_name, 0);
    av_dict_set(&writer->output_ctx->metadata, "encoder", "LightNVR", 0);

    // Set options for fast start - EXACTLY match rtsp_recorder.c
    AVDictionary *opts = NULL;
    if (config_get_mp4_faststart(&g_config)) {  // [storage] mp4_faststart (default true)
        av_dict_set(&opts, "movflags", "+faststart", 0);  // This is the ONLY option in rtsp_recorder.c
    }

    // Open output file
    ret = avio_open(&writer->output_ctx->pb, writer->output_path, AVIO_FLAG_WRITE);
    if (ret < 0) {
        recording_io_report_failure(RECORDING_IO_RESOURCE_RECORDING,
                                    RECORDING_IO_OPERATION_OPEN, ret);
        char error_buf[AV_ERROR_MAX_STRING_SIZE] = {0};
        av_strerror(ret, error_buf, AV_ERROR_MAX_STRING_SIZE);
        log_error("Failed to open output file for MP4 writer: %s (error: %s)",
                writer->output_path, error_buf);

        // Try to diagnose the issue
        struct stat st;
        if (stat(dir_path, &st) != 0) {
            log_error("Directory does not exist: %s", dir_path);
        } else if (!S_ISDIR(st.st_mode)) {
            log_error("Path exists but is not a directory: %s", dir_path);
        } else if (access(dir_path, W_OK) != 0) {
            log_error("Directory is not writable: %s", dir_path);
        }

        avformat_free_context(writer->output_ctx);
        writer->output_ctx = NULL;
        av_dict_free(&opts);
        free(dir_path);  // Free dir_path before returning
        return -1;
    }

    // Write file header
    ret = avformat_write_header(writer->output_ctx, &opts);
    if (ret < 0) {
        recording_io_report_failure(RECORDING_IO_RESOURCE_RECORDING,
                                    RECORDING_IO_OPERATION_HEADER, ret);
        char error_buf[AV_ERROR_MAX_STRING_SIZE] = {0};
        av_strerror(ret, error_buf, AV_ERROR_MAX_STRING_SIZE);
        log_error("Failed to write header for MP4 writer: %s", error_buf);
        int close_ret = avio_closep(&writer->output_ctx->pb);
        if (close_ret < 0) {
            recording_io_report_failure(RECORDING_IO_RESOURCE_RECORDING,
                                        RECORDING_IO_OPERATION_CLOSE,
                                        close_ret);
        }
        avformat_free_context(writer->output_ctx);
        writer->output_ctx = NULL;
        av_dict_free(&opts);
        free(dir_path);  // Free dir_path before returning
        return -1;
    }

    av_dict_free(&opts);

    writer->is_initialized = 1;
    log_info("Successfully initialized MP4 writer for stream %s at %s",
            writer->stream_name, writer->output_path);

    // Free dir_path now that we're done with it
    free(dir_path);

    return 0;
}

/**
 * Safely add audio stream to MP4 writer with improved error handling
 * Returns 0 on success, -1 on failure
 */
int mp4_writer_add_audio_stream(mp4_writer_t *writer, const AVCodecParameters *codec_params,
                                const AVRational *time_base) {
    // MAJOR REFACTOR: Complete rewrite of audio stream addition with robust error handling
    int ret = -1;
    AVStream *audio_stream = NULL;

    // Validate input parameters
    if (!writer) {
        log_error("NULL writer passed to mp4_writer_add_audio_stream");
        return -1;
    }

    if (!codec_params) {
        log_error("NULL codec parameters passed to mp4_writer_add_audio_stream for %s",
                 writer->stream_name ? writer->stream_name : "unknown");
        return -1;
    }

    if (!time_base) {
        log_error("NULL time base passed to mp4_writer_add_audio_stream for %s",
                 writer->stream_name ? writer->stream_name : "unknown");
        return -1;
    }

    if (!writer->output_ctx) {
        log_error("NULL output context in mp4_writer_add_audio_stream for %s",
                 writer->stream_name ? writer->stream_name : "unknown");
        return -1;
    }

    // Initialize audio.first_dts to AV_NOPTS_VALUE if not already set
    if (writer->audio.first_dts != AV_NOPTS_VALUE) {
        log_debug("Audio first_dts is not at default AV_NOPTS_VALUE; current value: %lld for %s",
                 (long long)writer->audio.first_dts,
                 writer->stream_name ? writer->stream_name : "unknown");
    }

    // Check if we already have an audio stream
    if (writer->audio.stream_idx != -1) {
        log_info("Audio stream already exists for %s, skipping initialization",
                writer->stream_name ? writer->stream_name : "unknown");
        return 0;  // Already initialized, nothing to do
    }

    // Verify the codec parameters are for audio
    if (codec_params->codec_type != AVMEDIA_TYPE_AUDIO) {
        log_error("Invalid codec type (not audio) for %s",
                 writer->stream_name ? writer->stream_name : "unknown");
        return -1;
    }

    // Verify the codec ID is valid
    if (codec_params->codec_id == AV_CODEC_ID_NONE) {
        log_error("Invalid audio codec ID (NONE) for %s",
                 writer->stream_name ? writer->stream_name : "unknown");
        return -1;
    }

    // MAJOR REFACTOR: Create a local copy of codec parameters to avoid modifying the original
    AVCodecParameters *local_codec_params = avcodec_parameters_alloc();
    if (!local_codec_params) {
        log_error("Failed to allocate codec parameters for audio stream in %s",
                 writer->stream_name ? writer->stream_name : "unknown");
        return -1;
    }

    // Copy parameters to our local copy
    ret = avcodec_parameters_copy(local_codec_params, codec_params);
    if (ret < 0) {
        char error_buf[AV_ERROR_MAX_STRING_SIZE] = {0};
        av_strerror(ret, error_buf, AV_ERROR_MAX_STRING_SIZE);
        log_error("Failed to copy audio codec parameters for %s: %s",
                 writer->stream_name ? writer->stream_name : "unknown", error_buf);
        avcodec_parameters_free(&local_codec_params);
        return -1;
    }

    // Log audio stream parameters for debugging
    log_info("Audio stream parameters for %s: codec_id=%d, sample_rate=%d, format=%d",
            writer->stream_name ? writer->stream_name : "unknown",
            local_codec_params->codec_id, local_codec_params->sample_rate,
            local_codec_params->format);

    // Ensure channel layout is valid
    // In newer FFmpeg versions, ch_layout is used instead of channels
    #if LIBAVCODEC_VERSION_INT >= AV_VERSION_INT(59, 37, 100)
        // For FFmpeg 5.0 and newer
        if (local_codec_params->ch_layout.nb_channels <= 0) {
            log_warn("Invalid channel count in ch_layout for audio stream in %s, setting to mono",
                    writer->stream_name ? writer->stream_name : "unknown");
            // Only set default if invalid
            av_channel_layout_default(&local_codec_params->ch_layout, 1); // Set to mono
        }
    #else
        // For older FFmpeg versions
        if (local_codec_params->channel_layout == 0) {
            log_warn("Invalid channel layout for audio stream in %s, setting to mono",
                    writer->stream_name ? writer->stream_name : "unknown");
            local_codec_params->channel_layout = AV_CH_LAYOUT_MONO;
        }
    #endif

    // Ensure sample rate is valid
    if (local_codec_params->sample_rate <= 0) {
        log_warn("Invalid sample rate for audio stream in %s, setting to 48000",
                writer->stream_name ? writer->stream_name : "unknown");
        local_codec_params->sample_rate = 48000;
    }

    // Ensure format is valid
    if (local_codec_params->format < 0) {
        log_warn("Invalid format for audio stream in %s, setting to S16",
                writer->stream_name ? writer->stream_name : "unknown");
        local_codec_params->format = AV_SAMPLE_FMT_S16;
    }

    // Create a completely safe timebase
    AVRational safe_time_base = {1, 48000};  // Default to 48kHz

    // Only use the provided timebase if it's valid
    if (time_base && time_base->num > 0 && time_base->den > 0) {
        // Make a copy to avoid any potential issues with the original
        safe_time_base.num = time_base->num;
        safe_time_base.den = time_base->den;

        log_debug("Using provided timebase (%d/%d) for audio stream in %s",
                 safe_time_base.num, safe_time_base.den,
                 writer->stream_name ? writer->stream_name : "unknown");
    } else {
        log_warn("Invalid timebase for audio stream in %s, using default (1/48000)",
                writer->stream_name ? writer->stream_name : "unknown");
    }

    // MAJOR REFACTOR: Initialize audio timestamp tracking BEFORE creating the stream
    // This ensures that these values are set even if stream creation fails
    writer->audio.first_dts = AV_NOPTS_VALUE;  // Initialize to AV_NOPTS_VALUE to match rtsp_recorder.c
    writer->audio.last_pts = 0;
    writer->audio.last_dts = AV_NOPTS_VALUE;
    writer->audio.initialized = 0;  // Don't mark as initialized until we receive the first packet
    writer->audio.time_base = safe_time_base;  // Store the timebase in the audio state

    // Check if the audio codec is compatible with MP4 format
    // MP4 container has limited audio codec support
    const char *codec_name = "unknown";
    bool is_compatible = is_audio_codec_compatible_with_mp4(codec_params->codec_id, &codec_name);

    if (!is_compatible) {
        // For incompatible codecs, attempt transcoding if it's a PCM codec
        if (is_pcm_codec(codec_params->codec_id)) {
            log_info("Attempting to transcode %s audio to AAC for MP4 compatibility in stream %s",
                    codec_name, writer->stream_name ? writer->stream_name : "unknown");

            // Try to transcode PCM to AAC
            AVCodecParameters *transcoded_params = NULL;
            int transcode_ret = transcode_pcm_to_aac(codec_params, &safe_time_base,
                                                     writer->stream_name, &transcoded_params);

            if (transcode_ret >= 0 && transcoded_params) {
                log_info("Successfully transcoded %s audio to AAC for stream %s",
                        codec_name, writer->stream_name ? writer->stream_name : "unknown");

                // Free the original codec parameters
                avcodec_parameters_free(&local_codec_params);

                // Use the transcoded parameters instead
                local_codec_params = transcoded_params;

                // Update codec name for logging
                codec_name = "AAC (transcoded from PCM)";
            } else {
                log_error("Failed to transcode %s audio to AAC: %d", codec_name, transcode_ret);
                log_error("Audio stream parameters: codec_id=%d, sample_rate=%d, channels=%d",
                         codec_params->codec_id, codec_params->sample_rate,
                         #if LIBAVCODEC_VERSION_INT >= AV_VERSION_INT(59, 37, 100)
                             codec_params->ch_layout.nb_channels
                         #else
                             codec_params->channels
                         #endif
                        );
                log_error("Disabling audio recording for this stream");

                // Disable audio for this writer
                writer->has_audio = 0;

                // Free the codec parameters
                avcodec_parameters_free(&local_codec_params);

                // Return success but don't add the audio stream
                return 0;
            }
        } else {
            // For other incompatible codecs, disable audio recording
            log_error("%s audio codec is not compatible with MP4 format", codec_name);
            log_error("Audio stream parameters: codec_id=%d, sample_rate=%d, channels=%d",
                     codec_params->codec_id, codec_params->sample_rate,
                     #if LIBAVCODEC_VERSION_INT >= AV_VERSION_INT(59, 37, 100)
                         codec_params->ch_layout.nb_channels
                     #else
                         codec_params->channels
                     #endif
                    );
            log_error("Disabling audio recording for this stream");
            
            // Disable audio for this writer
            writer->has_audio = 0;
            
            // Free the codec parameters
            avcodec_parameters_free(&local_codec_params);
            
            // Return success but don't add the audio stream
            return 0;
        }
    } else {
        log_info("Using %s audio codec for stream %s",
                codec_name, writer->stream_name ? writer->stream_name : "unknown");
    }

    // Set default frame size for audio codec
    if (codec_params->codec_id == AV_CODEC_ID_OPUS) {
        writer->audio.frame_size = 960;  // Opus typically uses 960 samples per frame (20ms at 48kHz)
        log_debug("Setting Opus frame size to 960 samples for stream %s",
                 writer->stream_name ? writer->stream_name : "unknown");
    } else if (codec_params->codec_id == AV_CODEC_ID_AAC) {
        writer->audio.frame_size = DEFAULT_AAC_FRAME_SIZE;  // AAC typically uses 1024 samples per frame
        log_debug("Setting AAC frame size to 1024 samples for stream %s",
                 writer->stream_name ? writer->stream_name : "unknown");
    } else {
        // Default to DEFAULT_AAC_FRAME_SIZE for other codecs
        writer->audio.frame_size = DEFAULT_AAC_FRAME_SIZE;
        log_debug("Setting default frame size to 1024 samples for codec %d in stream %s",
                 codec_params->codec_id, writer->stream_name ? writer->stream_name : "unknown");
    }

    // Create a new audio stream in the output
    audio_stream = avformat_new_stream(writer->output_ctx, NULL);
    if (!audio_stream) {
        recording_io_report_failure(RECORDING_IO_RESOURCE_RECORDING,
                                    RECORDING_IO_OPERATION_ALLOCATE, ENOMEM);
        log_error("Failed to create audio stream for MP4 writer for %s",
                 writer->stream_name ? writer->stream_name : "unknown");
        avcodec_parameters_free(&local_codec_params);
        return -1;
    }

    // Set codec tag to 0 to let FFmpeg choose the appropriate tag
    local_codec_params->codec_tag = 0;

    // Copy our modified parameters to the stream
    ret = avcodec_parameters_copy(audio_stream->codecpar, local_codec_params);
    if (ret < 0) {
        char error_buf[AV_ERROR_MAX_STRING_SIZE] = {0};
        av_strerror(ret, error_buf, AV_ERROR_MAX_STRING_SIZE);
        log_error("Failed to copy audio codec parameters to stream for %s: %s",
                 writer->stream_name ? writer->stream_name : "unknown", error_buf);
        avcodec_parameters_free(&local_codec_params);
        return -1;
    }

    // CRITICAL FIX: Set the frame_size in the codec parameters
    // This prevents the "track 1: codec frame size is not set" error
    if (audio_stream->codecpar->frame_size == 0) {
        audio_stream->codecpar->frame_size = writer->audio.frame_size;
        log_info("Setting audio codec frame_size to %d for stream %s",
                writer->audio.frame_size, writer->stream_name ? writer->stream_name : "unknown");
    }

    // Free our local copy now that we've copied it to the stream
    avcodec_parameters_free(&local_codec_params);

    // Set the timebase
    audio_stream->time_base = safe_time_base;

    // Log the frame_size if available
    if (audio_stream->codecpar->frame_size > 0) {
        log_debug("Audio frame_size=%d for audio stream in %s",
                 audio_stream->codecpar->frame_size,
                 writer->stream_name ? writer->stream_name : "unknown");
    } else {
        log_debug("No frame_size available for audio stream in %s, codec will determine it",
                 writer->stream_name ? writer->stream_name : "unknown");
    }

    // Verify the stream index is valid
    if (audio_stream->index < 0) {
        log_error("Invalid audio stream index %d for %s",
                 audio_stream->index, writer->stream_name ? writer->stream_name : "unknown");
        return -1;
    }

    // Verify the stream index is within bounds
    if (audio_stream->index >= writer->output_ctx->nb_streams) {
        log_error("Audio stream index %d exceeds number of streams %d for %s",
                 audio_stream->index, writer->output_ctx->nb_streams,
                 writer->stream_name ? writer->stream_name : "unknown");
        return -1;
    }

    // Store audio stream index
    writer->audio.stream_idx = audio_stream->index;

    // Mark audio as enabled
    writer->has_audio = 1;

    log_info("Successfully added audio stream (index %d) to MP4 recording for %s",
            writer->audio.stream_idx, writer->stream_name ? writer->stream_name : "unknown");

    return 0;  // Success
}
