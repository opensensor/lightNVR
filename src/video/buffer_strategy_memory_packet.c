/**
 * AVPacket Memory Buffer Strategy
 *
 * Wraps the existing packet_buffer.c implementation as a pluggable strategy.
 *
 * This strategy:
 * - Maintains a circular buffer of AVPackets in memory
 * - Provides frame-level precision for pre-detection content
 * - Tracks keyframes for proper GOP alignment
 * - Can flush directly to an MP4 writer for unified recordings
 *
 * Advantages:
 * - Fine-grained control (frame-level precision)
 * - Proper timestamp handling (PTS/DTS)
 * - Keyframe awareness for seamless concatenation
 * - Unified output (single recording file)
 *
 * Disadvantages:
 * - Memory intensive (~10-50MB per stream for 10s of 1080p)
 * - Requires dedicated RTSP reading for packet ingestion
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <pthread.h>

#include "video/pre_detection_buffer.h"
#include "video/packet_buffer.h"
#include "core/logger.h"
#include "core/config.h"

// Strategy private data
typedef struct {
    char stream_name[256];
    packet_buffer_t *packet_buffer;     // Underlying packet buffer
    int buffer_seconds;
    size_t memory_limit_bytes;

    // For flush_to_file, we need to track format context
    AVFormatContext *output_ctx;
    bool writing_file;
} memory_packet_strategy_data_t;

// --- Strategy interface methods ---

static int memory_packet_strategy_init(pre_buffer_strategy_t *self,
                                        const buffer_config_t *config) {
    memory_packet_strategy_data_t *data = (memory_packet_strategy_data_t *)self->private_data;

    data->buffer_seconds = config->buffer_seconds;
    data->memory_limit_bytes = config->memory_limit_bytes;

    // Create underlying packet buffer
    data->packet_buffer = create_packet_buffer(data->stream_name,
                                                config->buffer_seconds,
                                                BUFFER_MODE_MEMORY);

    if (!data->packet_buffer) {
        log_error("Failed to create packet buffer for %s", data->stream_name);
        return -1;
    }

    // Set memory limit if specified
    if (config->memory_limit_bytes > 0) {
        packet_buffer_set_memory_limit(data->packet_buffer,
                                        config->memory_limit_bytes / (1024 * 1024));
    }

    self->initialized = true;
    log_info("Memory packet strategy initialized for %s (%d seconds buffer)",
             data->stream_name, config->buffer_seconds);

    return 0;
}

static void memory_packet_strategy_destroy(pre_buffer_strategy_t *self) {
    memory_packet_strategy_data_t *data = (memory_packet_strategy_data_t *)self->private_data;

    if (data->packet_buffer) {
        destroy_packet_buffer(data->packet_buffer);
        data->packet_buffer = NULL;
    }

    log_debug("Memory packet strategy destroyed for %s", data->stream_name);
    free(data);
    self->private_data = NULL;
}

static int memory_packet_strategy_add_packet(pre_buffer_strategy_t *self,
                                              const AVPacket *packet,
                                              time_t timestamp) {
    memory_packet_strategy_data_t *data = (memory_packet_strategy_data_t *)self->private_data;

    if (!data->packet_buffer) {
        return -1;
    }

    return packet_buffer_add_packet(data->packet_buffer, packet, timestamp);
}

static int memory_packet_strategy_get_stats(pre_buffer_strategy_t *self,
                                             buffer_stats_t *stats) {
    memory_packet_strategy_data_t *data = (memory_packet_strategy_data_t *)self->private_data;

    memset(stats, 0, sizeof(*stats));

    if (!data->packet_buffer) {
        return -1;
    }

    int count = 0;
    size_t memory = 0;
    int duration = 0;

    packet_buffer_get_stats(data->packet_buffer, &count, &memory, &duration);

    stats->packet_count = count;
    stats->memory_usage_bytes = memory;
    stats->buffered_duration_ms = duration * 1000;
    stats->keyframe_count = packet_buffer_get_keyframe_count(data->packet_buffer);
    stats->has_complete_gop = (stats->keyframe_count > 0);
    stats->oldest_timestamp = data->packet_buffer->oldest_packet_time;
    stats->newest_timestamp = data->packet_buffer->newest_packet_time;

    return 0;
}

static bool memory_packet_strategy_is_ready(pre_buffer_strategy_t *self) {
    memory_packet_strategy_data_t *data = (memory_packet_strategy_data_t *)self->private_data;

    if (!data->packet_buffer) {
        return false;
    }

    return packet_buffer_is_ready(data->packet_buffer);
}

static void memory_packet_strategy_clear(pre_buffer_strategy_t *self) {
    memory_packet_strategy_data_t *data = (memory_packet_strategy_data_t *)self->private_data;

    if (data->packet_buffer) {
        packet_buffer_clear(data->packet_buffer);
    }
}

// Callback context for flush_to_file
typedef struct {
    AVFormatContext *output_ctx;
    int video_stream_idx;
    int audio_stream_idx;
    int64_t first_pts;
    int64_t pts_offset;
    bool first_packet;
} flush_context_t;

static int flush_packet_to_file(const AVPacket *packet, void *user_data) {
    flush_context_t *ctx = (flush_context_t *)user_data;

    if (!ctx->output_ctx || !packet) {
        return -1;
    }

    AVPacket *pkt = av_packet_clone(packet);
    if (!pkt) {
        return -1;
    }

    // Adjust timestamps
    if (ctx->first_packet) {
        ctx->first_pts = pkt->pts;
        ctx->pts_offset = -ctx->first_pts;  // Normalize to start at 0
        ctx->first_packet = false;
    }

    if (pkt->pts != AV_NOPTS_VALUE) {
        pkt->pts += ctx->pts_offset;
    }
    if (pkt->dts != AV_NOPTS_VALUE) {
        pkt->dts += ctx->pts_offset;
    }

    // Map stream index
    if (pkt->stream_index == 0) {
        pkt->stream_index = ctx->video_stream_idx;
    } else {
        pkt->stream_index = ctx->audio_stream_idx >= 0 ? ctx->audio_stream_idx : 0;
    }

    int ret = av_interleaved_write_frame(ctx->output_ctx, pkt);
    av_packet_free(&pkt);

    return ret;
}

static int memory_packet_strategy_flush_to_file(pre_buffer_strategy_t *self,
                                                  const char *output_path) {
    memory_packet_strategy_data_t *data = (memory_packet_strategy_data_t *)self->private_data;

    if (!data->packet_buffer || data->packet_buffer->count == 0) {
        log_warn("No packets to flush for %s", data->stream_name);
        return -1;
    }

    // Create output context
    AVFormatContext *output_ctx = NULL;
    int ret = avformat_alloc_output_context2(&output_ctx, NULL, "mp4", output_path);
    if (ret < 0 || !output_ctx) {
        log_error("Failed to create output context for %s", output_path);
        return -1;
    }

    // We need stream info from the first packet
    // For now, create a basic video stream
    // TODO: Get codec info from stream configuration
    AVStream *out_stream = avformat_new_stream(output_ctx, NULL);
    if (!out_stream) {
        log_error("Failed to create output stream");
        avformat_free_context(output_ctx);
        return -1;
    }

    // Set up codec parameters (basic H.264 defaults)
    out_stream->codecpar->codec_type = AVMEDIA_TYPE_VIDEO;
    out_stream->codecpar->codec_id = AV_CODEC_ID_H264;
    out_stream->time_base = (AVRational){1, 90000};

    // Open output file
    if (!(output_ctx->oformat->flags & AVFMT_NOFILE)) {
        ret = avio_open(&output_ctx->pb, output_path, AVIO_FLAG_WRITE);
        if (ret < 0) {
            log_error("Failed to open output file: %s", output_path);
            avformat_free_context(output_ctx);
            return -1;
        }
    }

    // Set movflags for faststart
    AVDictionary *opts = NULL;
    av_dict_set(&opts, "movflags", "+faststart", 0);

    ret = avformat_write_header(output_ctx, &opts);
    av_dict_free(&opts);
    if (ret < 0) {
        log_error("Failed to write header");
        if (!(output_ctx->oformat->flags & AVFMT_NOFILE)) {
            avio_closep(&output_ctx->pb);
        }
        avformat_free_context(output_ctx);
        return -1;
    }

    // Flush packets
    flush_context_t ctx = {
        .output_ctx = output_ctx,
        .video_stream_idx = 0,
        .audio_stream_idx = -1,
        .first_pts = 0,
        .pts_offset = 0,
        .first_packet = true
    };

    int flushed = packet_buffer_flush(data->packet_buffer, flush_packet_to_file, &ctx);

    // Write trailer and cleanup
    av_write_trailer(output_ctx);

    if (!(output_ctx->oformat->flags & AVFMT_NOFILE)) {
        avio_closep(&output_ctx->pb);
    }
    avformat_free_context(output_ctx);

    if (flushed > 0) {
        log_info("Flushed %d packets to %s", flushed, output_path);
        return 0;
    }

    return -1;
}

static int memory_packet_strategy_flush_to_callback(pre_buffer_strategy_t *self,
                                                     packet_write_callback_t callback,
                                                     void *user_data) {
    memory_packet_strategy_data_t *data = (memory_packet_strategy_data_t *)self->private_data;

    if (!data->packet_buffer) {
        return -1;
    }

    return packet_buffer_flush(data->packet_buffer, callback, user_data);
}

// --- Factory function ---

pre_buffer_strategy_t* create_memory_packet_strategy(const char *stream_name,
                                                      const buffer_config_t *config) {
    // Ensure packet buffer pool is initialized
    static bool pool_init_attempted = false;
    if (!pool_init_attempted) {
        extern config_t g_config;
        size_t memory_limit = config->memory_limit_bytes > 0 ?
                              config->memory_limit_bytes / (1024 * 1024) : 256;
        init_packet_buffer_pool(memory_limit);
        pool_init_attempted = true;
    }

    pre_buffer_strategy_t *strategy = calloc(1, sizeof(pre_buffer_strategy_t));
    if (!strategy) {
        log_error("Failed to allocate memory packet strategy");
        return NULL;
    }

    memory_packet_strategy_data_t *data = calloc(1, sizeof(memory_packet_strategy_data_t));
    if (!data) {
        log_error("Failed to allocate memory packet strategy data");
        free(strategy);
        return NULL;
    }

    strncpy(data->stream_name, stream_name, sizeof(data->stream_name) - 1);

    strategy->name = "memory_packet";
    strategy->type = BUFFER_STRATEGY_MEMORY_PACKET;
    strncpy(strategy->stream_name, stream_name, sizeof(strategy->stream_name) - 1);
    strategy->private_data = data;

    // Set interface methods
    strategy->init = memory_packet_strategy_init;
    strategy->destroy = memory_packet_strategy_destroy;
    strategy->add_packet = memory_packet_strategy_add_packet;
    strategy->add_segment = NULL;  // Not used by this strategy
    strategy->protect_segment = NULL;
    strategy->unprotect_segment = NULL;
    strategy->get_segments = NULL;
    strategy->flush_to_file = memory_packet_strategy_flush_to_file;
    strategy->flush_to_writer = NULL;  // TODO: implement
    strategy->flush_to_callback = memory_packet_strategy_flush_to_callback;
    strategy->get_stats = memory_packet_strategy_get_stats;
    strategy->is_ready = memory_packet_strategy_is_ready;
    strategy->clear = memory_packet_strategy_clear;

    // Initialize
    if (strategy->init(strategy, config) != 0) {
        log_error("Failed to initialize memory packet strategy for %s", stream_name);
        free(data);
        free(strategy);
        return NULL;
    }

    return strategy;
}

