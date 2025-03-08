/**
 * Improved implementation of MP4 writer for storing camera streams
 * Save this file as src/video/mp4_writer.c
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <libavformat/avformat.h>
#include <libavcodec/avcodec.h>
#include <libavutil/avutil.h>
#include <libavutil/opt.h>
#include <libavutil/time.h>

#include "core/logger.h"
#include "video/mp4_writer.h"

struct mp4_writer {
    char output_path[1024];
    char stream_name[64];
    AVFormatContext *output_ctx;
    int video_stream_idx;
    int has_audio;
    int64_t first_dts;
    int64_t last_dts;
    AVRational time_base;
    int is_initialized;
};

/**
 * Create a new MP4 writer
 */
mp4_writer_t *mp4_writer_create(const char *output_path, const char *stream_name) {
    mp4_writer_t *writer = calloc(1, sizeof(mp4_writer_t));
    if (!writer) {
        log_error("Failed to allocate memory for MP4 writer");
        return NULL;
    }

    // Initialize writer
    strncpy(writer->output_path, output_path, sizeof(writer->output_path) - 1);
    strncpy(writer->stream_name, stream_name, sizeof(writer->stream_name) - 1);
    writer->first_dts = AV_NOPTS_VALUE;
    writer->last_dts = AV_NOPTS_VALUE;
    writer->is_initialized = 0;
    
    log_info("Created MP4 writer for stream %s at %s", stream_name, output_path);
    
    return writer;
}

/**
 * Initialize the MP4 writer with the first packet's stream information
 */
static int mp4_writer_initialize(mp4_writer_t *writer, const AVPacket *pkt, const AVStream *input_stream) {
    int ret;
    
    // Create output format context
    ret = avformat_alloc_output_context2(&writer->output_ctx, NULL, "mp4", writer->output_path);
    if (ret < 0 || !writer->output_ctx) {
        log_error("Failed to create output format context for MP4 writer");
        return -1;
    }
    
    // Add video stream
    AVStream *out_stream = avformat_new_stream(writer->output_ctx, NULL);
    if (!out_stream) {
        log_error("Failed to create output stream for MP4 writer");
        avformat_free_context(writer->output_ctx);
        writer->output_ctx = NULL;
        return -1;
    }
    
    // Copy codec parameters
    ret = avcodec_parameters_copy(out_stream->codecpar, input_stream->codecpar);
    if (ret < 0) {
        log_error("Failed to copy codec parameters for MP4 writer");
        avformat_free_context(writer->output_ctx);
        writer->output_ctx = NULL;
        return -1;
    }
    
    // Set stream time base
    out_stream->time_base = input_stream->time_base;
    writer->time_base = input_stream->time_base;
    
    // Store video stream index
    writer->video_stream_idx = 0;  // We only have one stream
    
    // Add metadata
    av_dict_set(&writer->output_ctx->metadata, "title", writer->stream_name, 0);
    av_dict_set(&writer->output_ctx->metadata, "encoder", "LightNVR", 0);
    
    // Set options for fast start (moov atom at beginning of file)
    av_dict_set(&writer->output_ctx->metadata, "movflags", "+faststart", 0);
    
    // Open output file
    AVDictionary *opts = NULL;
    av_dict_set(&opts, "movflags", "+faststart", 0);  // Ensure moov atom is at start of file
    
    ret = avio_open(&writer->output_ctx->pb, writer->output_path, AVIO_FLAG_WRITE);
    if (ret < 0) {
        char error_buf[AV_ERROR_MAX_STRING_SIZE] = {0};
        av_strerror(ret, error_buf, AV_ERROR_MAX_STRING_SIZE);
        log_error("Failed to open output file for MP4 writer: %s (error: %s)", 
                writer->output_path, error_buf);
        avformat_free_context(writer->output_ctx);
        writer->output_ctx = NULL;
        return -1;
    }
    
    // Write file header
    ret = avformat_write_header(writer->output_ctx, &opts);
    if (ret < 0) {
        char error_buf[AV_ERROR_MAX_STRING_SIZE] = {0};
        av_strerror(ret, error_buf, AV_ERROR_MAX_STRING_SIZE);
        log_error("Failed to write header for MP4 writer: %s", error_buf);
        avio_closep(&writer->output_ctx->pb);
        avformat_free_context(writer->output_ctx);
        writer->output_ctx = NULL;
        return -1;
    }
    
    av_dict_free(&opts);
    
    writer->is_initialized = 1;
    log_info("Initialized MP4 writer for stream %s", writer->stream_name);
    
    return 0;
}

/**
 * Write a packet to the MP4 file
 */
int mp4_writer_write_packet(mp4_writer_t *writer, const AVPacket *in_pkt, const AVStream *input_stream) {
    if (!writer) {
        return -1;
    }
    
    // Initialize on first packet
    if (!writer->is_initialized) {
        int ret = mp4_writer_initialize(writer, in_pkt, input_stream);
        if (ret < 0) {
            return ret;
        }
    }
    
    // Create a copy of the packet so we don't modify the original
    AVPacket pkt;
    av_packet_ref(&pkt, in_pkt);
    
    // Fix timestamps
    if (writer->first_dts == AV_NOPTS_VALUE) {
        // First packet - use its DTS as reference
        writer->first_dts = pkt.dts;
        pkt.pts = 0;
        pkt.dts = 0;
    } else {
        // Ensure monotonically increasing timestamp
        if (pkt.dts <= writer->last_dts) {
            // If timestamp goes backwards, adjust it to be just after the last one
            pkt.dts = writer->last_dts + 1;
        }
        
        // Set PTS relative to first DTS
        int64_t dts_diff = pkt.dts - writer->first_dts;
        pkt.dts = dts_diff;
        
        // If PTS is valid, adjust it too
        if (pkt.pts != AV_NOPTS_VALUE) {
            int64_t pts_diff = pkt.pts - writer->first_dts;
            pkt.pts = pts_diff;
        } else {
            // If PTS is invalid, set it to DTS
            pkt.pts = pkt.dts;
        }
    }
    
    // Update last DTS
    writer->last_dts = in_pkt->dts;
    
    // Write packet
    pkt.stream_index = writer->video_stream_idx;
    int ret = av_interleaved_write_frame(writer->output_ctx, &pkt);
    if (ret < 0) {
        char error_buf[AV_ERROR_MAX_STRING_SIZE] = {0};
        av_strerror(ret, error_buf, AV_ERROR_MAX_STRING_SIZE);
        log_error("Error writing MP4 packet: %s", error_buf);
    }
    
    // Free packet resources
    av_packet_unref(&pkt);
    
    return ret;
}

/**
 * Close the MP4 writer and release resources
 */
void mp4_writer_close(mp4_writer_t *writer) {
    if (!writer) {
        return;
    }
    
    if (writer->output_ctx) {
        // Write trailer if initialized
        if (writer->is_initialized) {
            // Write file trailer
            int ret = av_write_trailer(writer->output_ctx);
            if (ret < 0) {
                char error_buf[AV_ERROR_MAX_STRING_SIZE] = {0};
                av_strerror(ret, error_buf, AV_ERROR_MAX_STRING_SIZE);
                log_error("Failed to write trailer for MP4 writer: %s", error_buf);
            }
        }
        
        // Close output
        if (writer->output_ctx->pb) {
            avio_closep(&writer->output_ctx->pb);
        }
        
        // Free context
        avformat_free_context(writer->output_ctx);
        writer->output_ctx = NULL;
    }
    
    log_info("Closed MP4 writer for stream %s", writer->stream_name);
    
    // Free writer
    free(writer);
}
