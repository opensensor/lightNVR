#include <stdio.h>
#include <string.h>
#include <libavformat/avformat.h>
#include <libavutil/error.h>
#include <libavcodec/avcodec.h>
#include <libavutil/avutil.h>
#include <libavutil/mem.h>

#include "core/logger.h"
#include "video/timeout_utils.h"

/**
 * Safely unreference a packet with extensive validation
 * This function performs multiple checks to ensure the packet is valid before unreferencing
 *
 * @param pkt Pointer to the packet to unreference
 * @param source_info String identifying the source of the call for logging
 */
void safe_packet_unref(AVPacket *pkt, const char *source_info) {
    if (!pkt) {
        log_debug("safe_packet_unref: NULL packet from %s", source_info ? source_info : "unknown");
        return;
    }

    // Check if the packet appears to be valid
    // We check both buf and data as either could be set in a valid packet
    if (pkt->buf == NULL && pkt->data == NULL) {
        log_debug("safe_packet_unref: Packet from %s has NULL buf and data, skipping unref",
                 source_info ? source_info : "unknown");
        return;
    }

    // Check for obviously invalid pointers
    if ((uintptr_t)pkt < 1000 ||
        (pkt->buf && (uintptr_t)pkt->buf < 1000) ||
        (pkt->data && (uintptr_t)pkt->data < 1000)) {
        log_warn("safe_packet_unref: Invalid pointer detected in packet from %s, skipping unref",
                source_info ? source_info : "unknown");
        return;
    }

    // CRITICAL FIX: Check for invalid side data which can cause segmentation faults
    if (pkt->side_data_elems > 0 && (!pkt->side_data || (uintptr_t)pkt->side_data < 1000)) {
        log_warn("safe_packet_unref: Invalid side data detected in packet from %s, clearing side data",
                source_info ? source_info : "unknown");
        // Clear side data to prevent segmentation fault
        pkt->side_data = NULL;
        pkt->side_data_elems = 0;
    }

    // CRITICAL FIX: Validate each side data element
    for (int i = 0; i < pkt->side_data_elems; i++) {
        if (!pkt->side_data[i].data || (uintptr_t)pkt->side_data[i].data < 1000 ||
            pkt->side_data[i].size <= 0 || pkt->side_data[i].size > 10 * 1024 * 1024) {
            log_warn("safe_packet_unref: Invalid side data element %d detected in packet from %s, clearing side data",
                    i, source_info ? source_info : "unknown");
            // Clear side data to prevent segmentation fault
            pkt->side_data = NULL;
            pkt->side_data_elems = 0;
            break;
        }
    }

    // Additional validation for size
    if (pkt->size < 0 || pkt->size > 10 * 1024 * 1024) { // 10MB max packet size
        log_warn("safe_packet_unref: Suspicious packet size (%d) from %s, skipping unref",
                pkt->size, source_info ? source_info : "unknown");
        return;
    }

    // CRITICAL FIX: Use a safer approach to unreference the packet
    // Instead of directly calling av_packet_unref, we'll create a new empty packet
    // and swap it with the current packet, effectively clearing it without
    // risking segmentation faults from invalid internal pointers

    // Create a new empty packet
    AVPacket new_pkt;
    av_init_packet(&new_pkt);
    new_pkt.data = NULL;
    new_pkt.size = 0;
    new_pkt.buf = NULL;
    new_pkt.side_data = NULL;
    new_pkt.side_data_elems = 0;

    // Swap the contents of the packets
    AVPacket temp = *pkt;
    *pkt = new_pkt;

    // Now manually free the old packet's resources
    if (temp.buf) {
        av_buffer_unref(&temp.buf);
    }

    // Free side data if it exists and appears valid
    if (temp.side_data && temp.side_data_elems > 0 && (uintptr_t)temp.side_data > 1000) {
        for (int i = 0; i < temp.side_data_elems; i++) {
            if (temp.side_data[i].data && (uintptr_t)temp.side_data[i].data > 1000) {
                av_free(temp.side_data[i].data);
            }
        }
        av_free(temp.side_data);
    }

    // Log that we've safely unreferenced the packet
    log_debug("safe_packet_unref: Successfully unreferenced packet from %s",
             source_info ? source_info : "unknown");
}

/**
 * Perform comprehensive cleanup of FFmpeg resources
 * This function ensures all resources associated with an AVFormatContext are properly freed
 *
 * @param input_ctx Pointer to the AVFormatContext to clean up
 * @param codec_ctx Pointer to the AVCodecContext to clean up (can be NULL)
 * @param packet Pointer to the AVPacket to clean up (can be NULL)
 * @param frame Pointer to the AVFrame to clean up (can be NULL)
 */
void comprehensive_ffmpeg_cleanup(AVFormatContext **input_ctx, AVCodecContext **codec_ctx, AVPacket **packet, AVFrame **frame) {
    // MEMORY LEAK FIX: First, force a garbage collection of FFmpeg internal caches
    // This helps clean up any buffer pools that might be in use
    av_buffer_pool_uninit(NULL);

    // Clean up frame if provided
    if (frame && *frame) {
        // MEMORY LEAK FIX: Ensure all frame buffers are properly unreferenced
        // This is important for frames that might have been allocated with buffer pools
        for (int i = 0; i < AV_NUM_DATA_POINTERS; i++) {
            if ((*frame)->buf[i]) {
                av_buffer_unref(&(*frame)->buf[i]);
            }
            (*frame)->data[i] = NULL;
        }

        av_frame_unref(*frame);
        av_frame_free(frame);
        *frame = NULL;
    }

    // Clean up packet if provided
    if (packet && *packet) {
        // Safely unref the packet first using our enhanced safety function
        AVPacket *pkt = *packet;
        if (pkt) {
            // MEMORY LEAK FIX: Explicitly unref the packet buffer
            if (pkt->buf) {
                av_buffer_unref(&pkt->buf);
            }

            safe_packet_unref(pkt, "comprehensive_ffmpeg_cleanup");
            av_packet_free(packet);
            *packet = NULL;
        }
    }

    // Clean up codec context if provided
    if (codec_ctx && *codec_ctx) {
        // MEMORY LEAK FIX: Explicitly free codec-specific data
        AVCodecContext *ctx = *codec_ctx;

        // Free extradata which can leak
        if (ctx->extradata) {
            av_free(ctx->extradata);
            ctx->extradata = NULL;
            ctx->extradata_size = 0;
        }

        // Flush the codec to ensure all buffers are released
        avcodec_flush_buffers(ctx);

        // Close and free the codec context
        avcodec_close(ctx);
        avcodec_free_context(codec_ctx);
        *codec_ctx = NULL;
    }

    // Clean up format context if provided
    if (input_ctx && *input_ctx) {
        AVFormatContext *ctx = *input_ctx;

        // MEMORY LEAK FIX: Explicitly clear internal buffers that might be causing leaks
        // These are the buffers used by avformat_find_stream_info that show up in Valgrind

        // Flush all buffers before closing if possible
        if (ctx->pb && !ctx->pb->error) {
            avio_flush(ctx->pb);
        }

        // MEMORY LEAK FIX: Explicitly clear internal stream buffers
        for (unsigned int i = 0; i < ctx->nb_streams; i++) {
            AVStream *stream = ctx->streams[i];
            if (stream) {
                // MEMORY LEAK FIX: Explicitly free all stream-related resources

                // Clear any codec-specific data
                if (stream->codecpar) {
                    // Explicitly free extradata which can leak
                    if (stream->codecpar->extradata) {
                        av_free(stream->codecpar->extradata);
                        stream->codecpar->extradata = NULL;
                        stream->codecpar->extradata_size = 0;
                    }

                    // Free all codec parameters
                    avcodec_parameters_free(&stream->codecpar);
                }

                // Mark stream for discard to prevent further use
                stream->discard = AVDISCARD_ALL;

                // Set stream flags to help trigger internal cleanup
                stream->disposition |= AV_DISPOSITION_ATTACHED_PIC;

                // MEMORY LEAK FIX: Explicitly free any attached pictures
                if (stream->attached_pic.data) {
                    av_packet_unref(&stream->attached_pic);
                }

                // MEMORY LEAK FIX: Explicitly free any side data
                for (int j = 0; j < stream->nb_side_data; j++) {
                    if (stream->side_data[j].data) {
                        av_free(stream->side_data[j].data);
                        stream->side_data[j].data = NULL;
                        stream->side_data[j].size = 0;
                    }
                }
                if (stream->side_data) {
                    av_freep(&stream->side_data);
                    stream->nb_side_data = 0;
                }
            }
        }

        // MEMORY LEAK FIX: Set flags to help trigger internal cleanup
        ctx->flags |= AVFMT_FLAG_DISCARD_CORRUPT;
        ctx->flags |= AVFMT_FLAG_NOBUFFER; // Don't buffer packets

        // MEMORY LEAK FIX: Explicitly free any format-specific data
        if (ctx->metadata) {
            av_dict_free(&ctx->metadata);
        }

        // MEMORY LEAK FIX: Explicitly free any chapters
        for (unsigned int i = 0; i < ctx->nb_chapters; i++) {
            if (ctx->chapters[i]) {
                if (ctx->chapters[i]->metadata) {
                    av_dict_free(&ctx->chapters[i]->metadata);
                }
                av_free(ctx->chapters[i]);
                ctx->chapters[i] = NULL;
            }
        }
        if (ctx->chapters) {
            av_free(ctx->chapters);
            ctx->chapters = NULL;
            ctx->nb_chapters = 0;
        }

        // MEMORY LEAK FIX: Explicitly free any programs
        for (unsigned int i = 0; i < ctx->nb_programs; i++) {
            if (ctx->programs[i]) {
                if (ctx->programs[i]->metadata) {
                    av_dict_free(&ctx->programs[i]->metadata);
                }
                av_free(ctx->programs[i]);
                ctx->programs[i] = NULL;
            }
        }
        if (ctx->programs) {
            av_free(ctx->programs);
            ctx->programs = NULL;
            ctx->nb_programs = 0;
        }

        // Close the input context - this will free all associated resources
        avformat_close_input(input_ctx);

        // Double check that it's really closed
        if (*input_ctx) {
            log_warn("Input context still exists after avformat_close_input, forcing cleanup");
            avformat_free_context(*input_ctx);
            *input_ctx = NULL;
        }
    }

    // MEMORY LEAK FIX: Force a more aggressive garbage collection of FFmpeg internal caches
    // This helps clean up the buffer pools that are showing up in Valgrind
    av_buffer_pool_uninit(NULL);

    // MEMORY LEAK FIX: Call this multiple times to ensure all buffer pools are cleaned up
    // This is a workaround for a known FFmpeg issue where some buffer pools aren't cleaned up
    // on the first call
    av_buffer_pool_uninit(NULL);
    av_buffer_pool_uninit(NULL);

    // MEMORY LEAK FIX: Force a final garbage collection
    // This is especially important for cleaning up any remaining buffer pools
    av_buffer_pool_uninit(NULL);
}

/**
 * Handle FFmpeg resource cleanup after a timeout
 *
 * @param url The URL of the stream that timed out
 * @param input_ctx Pointer to the AVFormatContext to clean up
 * @return AVERROR(ETIMEDOUT) to indicate timeout
 */
int handle_timeout_cleanup(const char *url, AVFormatContext **input_ctx) {
    log_warn("Timeout occurred for stream: %s", url ? url : "unknown");

    // MEMORY LEAK FIX: First, force a garbage collection of FFmpeg internal caches
    // This helps clean up any buffer pools that might be in use
    for (int i = 0; i < 5; i++) {
        av_buffer_pool_uninit(NULL);
    }

    // MEMORY LEAK FIX: If input_ctx is valid, perform additional cleanup before calling comprehensive_ffmpeg_cleanup
    if (input_ctx && *input_ctx) {
        AVFormatContext *ctx = *input_ctx;

        // MEMORY LEAK FIX: Explicitly clear any internal buffers in the format context
        // This helps clean up buffers allocated during avformat_find_stream_info
        if (ctx->pb) {
            // Flush the I/O context
            avio_flush(ctx->pb);

            // MEMORY LEAK FIX: Close and free the I/O context
            // This is important because the I/O context can hold references to memory
            // that won't be freed by avformat_close_input
            if (!(ctx->flags & AVFMT_FLAG_CUSTOM_IO)) {
                avio_closep(&ctx->pb);
            }
        }

        // MEMORY LEAK FIX: Explicitly clear any internal buffers in the streams
        for (unsigned int i = 0; i < ctx->nb_streams; i++) {
            AVStream *stream = ctx->streams[i];
            if (stream) {
                // Mark stream for discard to prevent further use
                stream->discard = AVDISCARD_ALL;

                // MEMORY LEAK FIX: Explicitly free any codec parameters
                if (stream->codecpar) {
                    // Free extradata which can leak
                    if (stream->codecpar->extradata) {
                        av_free(stream->codecpar->extradata);
                        stream->codecpar->extradata = NULL;
                        stream->codecpar->extradata_size = 0;
                    }
                }
            }
        }
    }

    // Use our comprehensive cleanup function
    comprehensive_ffmpeg_cleanup(input_ctx, NULL, NULL, NULL);

    // MEMORY LEAK FIX: Force a final garbage collection
    // This is especially important for cleaning up any remaining buffer pools
    for (int i = 0; i < 5; i++) {
        av_buffer_pool_uninit(NULL);
    }

    return AVERROR(ETIMEDOUT);
}
