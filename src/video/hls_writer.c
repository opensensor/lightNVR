#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <libavformat/avformat.h>
#include <libavcodec/avcodec.h>
#include <libavutil/avutil.h>
#include <libavutil/time.h>

#include "../../include/core/logger.h"
#include "../../include/video/hls_writer.h"

/**
 * Create new HLS writer
 */
hls_writer_t *hls_writer_create(const char *output_dir, const char *stream_name, int segment_duration) {
    // Allocate writer structure
    hls_writer_t *writer = (hls_writer_t *)malloc(sizeof(hls_writer_t));
    if (!writer) {
        log_error("Failed to allocate HLS writer");
        return NULL;
    }

    // Initialize structure
    memset(writer, 0, sizeof(hls_writer_t));

    // Copy output directory and stream name
    strncpy(writer->output_dir, output_dir, MAX_PATH_LENGTH - 1);
    strncpy(writer->stream_name, stream_name, MAX_STREAM_NAME - 1);
    writer->segment_duration = segment_duration;

    // Create output directory if it doesn't exist
    char mkdir_cmd[MAX_PATH_LENGTH + 10];
    snprintf(mkdir_cmd, sizeof(mkdir_cmd), "mkdir -p %s", output_dir);
    if (system(mkdir_cmd) != 0) {
        log_warn("Failed to create directory: %s", output_dir);
    }

    // Initialize output format context for HLS
    char output_path[MAX_PATH_LENGTH];
    snprintf(output_path, MAX_PATH_LENGTH, "%s/index.m3u8", output_dir);

    // Allocate output format context
    int ret = avformat_alloc_output_context2(
        &writer->output_ctx, NULL, "hls", output_path);

    if (ret < 0) {
        char error_buf[AV_ERROR_MAX_STRING_SIZE] = {0};
        av_strerror(ret, error_buf, AV_ERROR_MAX_STRING_SIZE);
        log_error("Failed to allocate output context for HLS: %s", error_buf);
        free(writer);
        return NULL;
    }

    // Set HLS options
    AVDictionary *options = NULL;
    char hls_time[16];
    snprintf(hls_time, sizeof(hls_time), "%d", segment_duration);

    av_dict_set(&options, "hls_time", hls_time, 0);
    av_dict_set(&options, "hls_list_size", "10", 0);
    av_dict_set(&options, "hls_flags", "delete_segments", 0);

    // Create segment filename format string
    char segment_format[MAX_PATH_LENGTH + 32];
    snprintf(segment_format, sizeof(segment_format), "%s/segment_%%d.ts", output_dir);
    av_dict_set(&options, "hls_segment_filename", segment_format, 0);

    // Open output file
    ret = avio_open2(&writer->output_ctx->pb, output_path,
                    AVIO_FLAG_WRITE, NULL, &options);

    if (ret < 0) {
        char error_buf[AV_ERROR_MAX_STRING_SIZE] = {0};
        av_strerror(ret, error_buf, AV_ERROR_MAX_STRING_SIZE);
        log_error("Failed to open HLS output file: %s", error_buf);
        avformat_free_context(writer->output_ctx);
        free(writer);
        av_dict_free(&options);
        return NULL;
    }

    // Free options
    av_dict_free(&options);

    log_info("Created HLS writer for stream %s at %s", stream_name, output_dir);
    return writer;
}


/**
 * Initialize HLS writer with stream information
 */
int hls_writer_initialize(hls_writer_t *writer, const AVStream *input_stream) {
    if (!writer || !input_stream) {
        return -1;
    }

    // Create output stream
    AVStream *out_stream = avformat_new_stream(writer->output_ctx, NULL);
    if (!out_stream) {
        log_error("Failed to create output stream for HLS");
        return -1;
    }

    // Copy codec parameters
    int ret = avcodec_parameters_copy(out_stream->codecpar, input_stream->codecpar);
    if (ret < 0) {
        char error_buf[AV_ERROR_MAX_STRING_SIZE] = {0};
        av_strerror(ret, error_buf, AV_ERROR_MAX_STRING_SIZE);
        log_error("Failed to copy codec parameters: %s", error_buf);
        return ret;
    }

    // Set stream time base
    out_stream->time_base = input_stream->time_base;

    // Write stream header
    ret = avformat_write_header(writer->output_ctx, NULL);
    if (ret < 0) {
        char error_buf[AV_ERROR_MAX_STRING_SIZE] = {0};
        av_strerror(ret, error_buf, AV_ERROR_MAX_STRING_SIZE);
        log_error("Failed to write HLS header: %s", error_buf);
        return ret;
    }

    writer->initialized = 1;
    log_info("Initialized HLS writer for stream %s", writer->stream_name);
    return 0;
}


/**
 * Write packet to HLS stream
 */
int hls_writer_write_packet(hls_writer_t *writer, const AVPacket *pkt, const AVStream *input_stream) {
    if (!writer || !pkt || !input_stream) {
        return -1;
    }

    // Lazy initialization of output stream
    if (!writer->initialized) {
        int ret = hls_writer_initialize(writer, input_stream);
        if (ret < 0) {
            return ret;
        }
    }

    // Clone the packet as we'll need to modify it
    AVPacket out_pkt;
    if (av_packet_ref(&out_pkt, pkt) < 0) {
        log_error("Failed to reference packet");
        return -1;
    }

    // Adjust timestamps
    av_packet_rescale_ts(&out_pkt, input_stream->time_base,
                        writer->output_ctx->streams[0]->time_base);

    // Write the packet
    int ret = av_interleaved_write_frame(writer->output_ctx, &out_pkt);
    av_packet_unref(&out_pkt);

    if (ret < 0) {
        char error_buf[AV_ERROR_MAX_STRING_SIZE] = {0};
        av_strerror(ret, error_buf, AV_ERROR_MAX_STRING_SIZE);
        log_error("Error writing HLS packet: %s", error_buf);
        return ret;
    }

    return 0;
}

/**
 * Close HLS writer and free resources
 */
void hls_writer_close(hls_writer_t *writer) {
    if (!writer) {
        return;
    }

    // Write trailer if initialized
    if (writer->initialized && writer->output_ctx) {
        av_write_trailer(writer->output_ctx);
    }

    // Close output context
    if (writer->output_ctx) {
        if (writer->output_ctx->pb) {
            avio_close(writer->output_ctx->pb);
        }
        avformat_free_context(writer->output_ctx);
    }

    log_info("Closed HLS writer for stream %s", writer->stream_name);

    // Free writer
    free(writer);
}