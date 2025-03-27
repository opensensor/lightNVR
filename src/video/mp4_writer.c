/**
 * RTSP stream reading implementation for MP4 writer
 * Simplified version that follows rtsp_recorder example
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <time.h>
#include <stdbool.h>
#include <signal.h>
#include <pthread.h>
#include <sys/stat.h>
#include <errno.h>

#include <libavformat/avformat.h>
#include <libavcodec/avcodec.h>
#include <libavutil/avutil.h>
#include <libavutil/time.h>

#include "core/logger.h"
#include "core/shutdown_coordinator.h"
#include "video/mp4_writer.h"
#include "video/mp4_writer_internal.h"
#include "database/database_manager.h"
#include "database/db_recordings.h"

// Thread-related fields for the MP4 writer
typedef struct {
    pthread_t thread;         // Recording thread
    int running;              // Flag indicating if the thread is running
    char rtsp_url[MAX_PATH_LENGTH]; // URL of the RTSP stream to record
    volatile sig_atomic_t shutdown_requested; // Flag indicating if shutdown was requested
    mp4_writer_t *writer;     // MP4 writer instance
    int segment_duration;     // Duration of each segment in seconds
    time_t last_segment_time; // Time when the last segment was created
} mp4_writer_thread_t;




/**
 * Record an RTSP stream to an MP4 file for a specified duration
 * 
 * @param rtsp_url The URL of the RTSP stream to record
 * @param output_file The path to the output MP4 file
 * @param duration The duration to record in seconds
 * @param input_ctx_ptr Pointer to an existing input context (can be NULL)
 * @param has_audio Flag indicating whether to include audio in the recording
 * @return 0 on success, negative value on error
 */
int record_segment(const char *rtsp_url, const char *output_file, int duration, AVFormatContext **input_ctx_ptr, int has_audio) {
    int ret = 0;
    AVFormatContext *input_ctx = NULL;
    AVFormatContext *output_ctx = NULL;
    AVDictionary *opts = NULL;
    AVDictionary *out_opts = NULL;
    AVPacket pkt;
    int video_stream_idx = -1;
    int audio_stream_idx = -1;
    AVStream *out_video_stream = NULL;
    AVStream *out_audio_stream = NULL;
    int64_t first_video_dts = AV_NOPTS_VALUE;
    int64_t first_audio_dts = AV_NOPTS_VALUE;
    int64_t last_audio_dts = 0;
    int64_t last_audio_pts = 0;
    int audio_packet_count = 0;
    int64_t start_time;
    time_t last_progress = 0;
    
    log_info("Recording from %s", rtsp_url);
    log_info("Output file: %s", output_file);
    log_info("Duration: %d seconds", duration);
    
    // Use existing input context if provided
    if (*input_ctx_ptr) {
        input_ctx = *input_ctx_ptr;
        log_debug("Using existing input context");
    } else {
        // Set up RTSP options for low latency
        av_dict_set(&opts, "rtsp_transport", "tcp", 0);  // Use TCP for RTSP (more reliable than UDP)
        av_dict_set(&opts, "fflags", "nobuffer", 0);     // Reduce buffering
        av_dict_set(&opts, "flags", "low_delay", 0);     // Low delay mode
        av_dict_set(&opts, "max_delay", "500000", 0);    // Maximum delay of 500ms
        av_dict_set(&opts, "stimeout", "5000000", 0);    // Socket timeout in microseconds (5 seconds)
        
        // Open input
        ret = avformat_open_input(&input_ctx, rtsp_url, NULL, &opts);
        if (ret < 0) {
            log_error("Failed to open input: %d", ret);
            goto cleanup;
        }
        
        // Find stream info
        ret = avformat_find_stream_info(input_ctx, NULL);
        if (ret < 0) {
            log_error("Failed to find stream info: %d", ret);
            goto cleanup;
        }
        
        // Store the input context for reuse
        *input_ctx_ptr = input_ctx;
    }
    
    // Log input stream info
    log_debug("Input format: %s", input_ctx->iformat->name);
    log_debug("Number of streams: %d", input_ctx->nb_streams);
    
    // Find video and audio streams
    for (unsigned int i = 0; i < input_ctx->nb_streams; i++) {
        AVStream *stream = input_ctx->streams[i];
        if (stream->codecpar->codec_type == AVMEDIA_TYPE_VIDEO && video_stream_idx < 0) {
            video_stream_idx = i;
            log_debug("Found video stream: %d", i);
            log_debug("  Codec: %s", avcodec_get_name(stream->codecpar->codec_id));
            log_debug("  Resolution: %dx%d", stream->codecpar->width, stream->codecpar->height);
            if (stream->avg_frame_rate.num && stream->avg_frame_rate.den) {
                log_debug("  Frame rate: %.2f fps", 
                       (float)stream->avg_frame_rate.num / stream->avg_frame_rate.den);
            }
        } else if (stream->codecpar->codec_type == AVMEDIA_TYPE_AUDIO && audio_stream_idx < 0) {
            audio_stream_idx = i;
            log_debug("Found audio stream: %d", i);
            log_debug("  Codec: %s", avcodec_get_name(stream->codecpar->codec_id));
            log_debug("  Sample rate: %d Hz", stream->codecpar->sample_rate);
            // Handle channel count for different FFmpeg versions
#if LIBAVCODEC_VERSION_INT >= AV_VERSION_INT(59, 37, 100)
            // For FFmpeg 5.0 and newer
            log_debug("  Channels: %d", stream->codecpar->ch_layout.nb_channels);
#else
            // For older FFmpeg versions
            log_debug("  Channels: %d", stream->codecpar->channels);
#endif
        }
    }
    
    if (video_stream_idx < 0) {
        log_error("No video stream found");
        ret = -1;
        goto cleanup;
    }
    
    // Create output context
    ret = avformat_alloc_output_context2(&output_ctx, NULL, "mp4", output_file);
    if (ret < 0 || !output_ctx) {
        log_error("Failed to create output context: %d", ret);
        goto cleanup;
    }
    
    // Add video stream
    out_video_stream = avformat_new_stream(output_ctx, NULL);
    if (!out_video_stream) {
        log_error("Failed to create output video stream");
        ret = -1;
        goto cleanup;
    }
    
    // Copy video codec parameters
    ret = avcodec_parameters_copy(out_video_stream->codecpar, 
                                 input_ctx->streams[video_stream_idx]->codecpar);
    if (ret < 0) {
        log_error("Failed to copy video codec parameters: %d", ret);
        goto cleanup;
    }
    
    // Set video stream time base
    out_video_stream->time_base = input_ctx->streams[video_stream_idx]->time_base;
    
    // Add audio stream if available and audio is enabled
    if (audio_stream_idx >= 0 && has_audio) {
        log_info("Including audio stream in MP4 recording");
        out_audio_stream = avformat_new_stream(output_ctx, NULL);
        if (!out_audio_stream) {
            log_error("Failed to create output audio stream");
            ret = -1;
            goto cleanup;
        }
        
        // Copy audio codec parameters
        ret = avcodec_parameters_copy(out_audio_stream->codecpar, 
                                     input_ctx->streams[audio_stream_idx]->codecpar);
        if (ret < 0) {
            log_error("Failed to copy audio codec parameters: %d", ret);
            goto cleanup;
        }
        
        // Set audio stream time base
        out_audio_stream->time_base = input_ctx->streams[audio_stream_idx]->time_base;
    }
    
    // CRITICAL FIX: Disable faststart to prevent segmentation faults
    // The faststart option causes a second pass that moves the moov atom to the beginning of the file
    // This second pass is causing segmentation faults during shutdown
    av_dict_set(&out_opts, "movflags", "empty_moov", 0);
    
    // Open output file
    ret = avio_open(&output_ctx->pb, output_file, AVIO_FLAG_WRITE);
    if (ret < 0) {
        log_error("Failed to open output file: %d", ret);
        goto cleanup;
    }
    
    // Write file header
    ret = avformat_write_header(output_ctx, &out_opts);
    if (ret < 0) {
        log_error("Failed to write header: %d", ret);
        goto cleanup;
    }
    
    // Initialize packet
    av_init_packet(&pkt);
    pkt.data = NULL;
    pkt.size = 0;
    
    // Start recording
    start_time = av_gettime();
    log_info("Recording started...");
    
    // Flag to track if we've found the first key frame
    bool found_first_keyframe = false;
    // Flag to track if we're waiting for the final key frame
    bool waiting_for_final_keyframe = false;
    // Flag to track if shutdown was detected
    bool shutdown_detected = false;
    
    // Main recording loop
    while (1) {
        // Check if shutdown has been initiated
        if (!shutdown_detected && !waiting_for_final_keyframe && is_shutdown_initiated()) {
            log_info("Shutdown initiated, waiting for next key frame to end recording");
            waiting_for_final_keyframe = true;
            shutdown_detected = true;
        }
        
        // Check if we've reached the duration limit
        if (duration > 0 && !waiting_for_final_keyframe && !shutdown_detected && 
            (av_gettime() - start_time) / 1000000 >= duration) {
            log_info("Reached duration limit of %d seconds, waiting for next key frame to end recording", duration);
            waiting_for_final_keyframe = true;
        }
        
        // Read packet
        ret = av_read_frame(input_ctx, &pkt);
        if (ret < 0) {
            if (ret == AVERROR_EOF) {
                log_info("End of stream reached");
                break;
            } else if (ret != AVERROR(EAGAIN)) {
                log_error("Error reading frame: %d", ret);
                break;
            }
            // EAGAIN means try again, so we continue
            av_usleep(10000);  // Sleep 10ms to avoid busy waiting
            continue;
        }
        
        // Process video packets
        if (pkt.stream_index == video_stream_idx) {
            // Check if this is a key frame
            bool is_keyframe = (pkt.flags & AV_PKT_FLAG_KEY) != 0;
            
            // If we're waiting for the first key frame, skip non-key frames
            if (!found_first_keyframe) {
                if (is_keyframe) {
                    log_info("Found first key frame, starting recording");
                    found_first_keyframe = true;
                    
                    // Reset start time to when we found the first key frame
                    start_time = av_gettime();
                } else {
                    // Skip this frame as we're waiting for a key frame
                    av_packet_unref(&pkt);
                    continue;
                }
            }
            
            // If we're waiting for the final key frame to end recording
            if (waiting_for_final_keyframe && is_keyframe) {
                log_info("Found final key frame, ending recording");
                
                // Process this final key frame and then break the loop
                // Initialize first DTS if not set
                if (first_video_dts == AV_NOPTS_VALUE && pkt.dts != AV_NOPTS_VALUE) {
                    first_video_dts = pkt.dts;
                    log_debug("First video DTS: %lld", (long long)first_video_dts);
                }
                
                // Adjust timestamps
                if (pkt.dts != AV_NOPTS_VALUE && first_video_dts != AV_NOPTS_VALUE) {
                    pkt.dts -= first_video_dts;
                    if (pkt.dts < 0) pkt.dts = 0;
                }
                
                if (pkt.pts != AV_NOPTS_VALUE && first_video_dts != AV_NOPTS_VALUE) {
                    pkt.pts -= first_video_dts;
                    if (pkt.pts < 0) pkt.pts = 0;
                }
                
                // Explicitly set duration for the final key frame to prevent segmentation fault
                if (pkt.duration == 0 || pkt.duration == AV_NOPTS_VALUE) {
                    // Use the time base of the video stream to calculate a reasonable duration
                    if (input_ctx->streams[video_stream_idx]->avg_frame_rate.num > 0 && 
                        input_ctx->streams[video_stream_idx]->avg_frame_rate.den > 0) {
                        // Calculate duration based on framerate (time_base units)
                        pkt.duration = av_rescale_q(1, 
                                                   av_inv_q(input_ctx->streams[video_stream_idx]->avg_frame_rate),
                                                   input_ctx->streams[video_stream_idx]->time_base);
                    } else {
                        // Default to a reasonable value if framerate is not available
                        pkt.duration = 1;
                    }
                    log_debug("Set final key frame duration to %lld", (long long)pkt.duration);
                }
                
                // Set output stream index
                pkt.stream_index = out_video_stream->index;
                
                // Write packet
                ret = av_interleaved_write_frame(output_ctx, &pkt);
                if (ret < 0) {
                    log_error("Error writing video frame: %d", ret);
                }
                
                // Break the loop after processing the final key frame
                av_packet_unref(&pkt);
                break;
            }
            
            // Initialize first DTS if not set
            if (first_video_dts == AV_NOPTS_VALUE && pkt.dts != AV_NOPTS_VALUE) {
                first_video_dts = pkt.dts;
                log_debug("First video DTS: %lld", (long long)first_video_dts);
            }
            
            // Adjust timestamps
            if (pkt.dts != AV_NOPTS_VALUE && first_video_dts != AV_NOPTS_VALUE) {
                pkt.dts -= first_video_dts;
                if (pkt.dts < 0) pkt.dts = 0;
            }
            
            if (pkt.pts != AV_NOPTS_VALUE && first_video_dts != AV_NOPTS_VALUE) {
                pkt.pts -= first_video_dts;
                if (pkt.pts < 0) pkt.pts = 0;
            }
            
            // Explicitly set duration to prevent segmentation fault during fragment writing
            // This addresses the "Estimating the duration of the last packet in a fragment" error
            if (pkt.duration == 0 || pkt.duration == AV_NOPTS_VALUE) {
                // Use the time base of the video stream to calculate a reasonable duration
                // For most video streams, this will be 1/framerate
                if (input_ctx->streams[video_stream_idx]->avg_frame_rate.num > 0 && 
                    input_ctx->streams[video_stream_idx]->avg_frame_rate.den > 0) {
                    // Calculate duration based on framerate (time_base units)
                    pkt.duration = av_rescale_q(1, 
                                               av_inv_q(input_ctx->streams[video_stream_idx]->avg_frame_rate),
                                               input_ctx->streams[video_stream_idx]->time_base);
                } else {
                    // Default to a reasonable value if framerate is not available
                    pkt.duration = 1;
                }
                log_debug("Set video packet duration to %lld", (long long)pkt.duration);
            }
            
            // Set output stream index
            pkt.stream_index = out_video_stream->index;
            
            // Write packet
            ret = av_interleaved_write_frame(output_ctx, &pkt);
            if (ret < 0) {
                log_error("Error writing video frame: %d", ret);
            }
        }
        // Process audio packets - only if audio is enabled and we have an audio output stream
        else if (has_audio && audio_stream_idx >= 0 && pkt.stream_index == audio_stream_idx && out_audio_stream) {
            // Initialize first audio DTS if not set
            if (first_audio_dts == AV_NOPTS_VALUE && pkt.dts != AV_NOPTS_VALUE) {
                first_audio_dts = pkt.dts;
                log_debug("First audio DTS: %lld", (long long)first_audio_dts);
            }
            
            // For the first audio packet, initialize our tracking variables
            if (audio_packet_count == 0) {
                if (pkt.dts != AV_NOPTS_VALUE && first_audio_dts != AV_NOPTS_VALUE) {
                    last_audio_dts = pkt.dts - first_audio_dts;
                    if (last_audio_dts < 0) last_audio_dts = 0;
                } else {
                    last_audio_dts = 0;
                }
                
                if (pkt.pts != AV_NOPTS_VALUE && first_audio_dts != AV_NOPTS_VALUE) {
                    last_audio_pts = pkt.pts - first_audio_dts;
                    if (last_audio_pts < 0) last_audio_pts = 0;
                } else {
                    last_audio_pts = 0;
                }
                
                audio_packet_count++;
            }
            
            // Adjust timestamps to ensure monotonic increase
            if (pkt.dts != AV_NOPTS_VALUE && first_audio_dts != AV_NOPTS_VALUE) {
                int64_t new_dts = pkt.dts - first_audio_dts;
                if (new_dts < 0) new_dts = 0;
                
                // Ensure DTS is monotonically increasing
                if (new_dts <= last_audio_dts) {
                    new_dts = last_audio_dts + 1;
                }
                
                pkt.dts = new_dts;
                last_audio_dts = new_dts;
            } else {
                // If DTS is not set, use last_audio_dts + 1
                pkt.dts = last_audio_dts + 1;
                last_audio_dts = pkt.dts;
            }
            
            if (pkt.pts != AV_NOPTS_VALUE && first_audio_dts != AV_NOPTS_VALUE) {
                int64_t new_pts = pkt.pts - first_audio_dts;
                if (new_pts < 0) new_pts = 0;
                
                // Ensure PTS is monotonically increasing
                if (new_pts <= last_audio_pts) {
                    new_pts = last_audio_pts + 1;
                }
                
                // Ensure PTS >= DTS
                if (new_pts < pkt.dts) {
                    new_pts = pkt.dts;
                }
                
                pkt.pts = new_pts;
                last_audio_pts = new_pts;
            } else {
                // If PTS is not set, use DTS
                pkt.pts = pkt.dts;
                last_audio_pts = pkt.pts;
            }
            
            // Explicitly set duration to prevent segmentation fault during fragment writing
            // This addresses the "Estimating the duration of the last packet in a fragment" error
            if (pkt.duration == 0 || pkt.duration == AV_NOPTS_VALUE) {
                // For audio, we can calculate duration based on sample rate and frame size
                AVStream *audio_stream = input_ctx->streams[audio_stream_idx];
                if (audio_stream->codecpar->sample_rate > 0) {
                    // If we know the number of samples in this packet, use that
                    int nb_samples = 0;
                    
                    // Try to get the number of samples from the codec parameters
#if LIBAVCODEC_VERSION_INT >= AV_VERSION_INT(59, 37, 100)
                    // For FFmpeg 5.0 and newer
                    if (audio_stream->codecpar->ch_layout.nb_channels > 0 && 
                        audio_stream->codecpar->bits_per_coded_sample > 0) {
                        int bytes_per_sample = audio_stream->codecpar->bits_per_coded_sample / 8;
                        // Ensure we don't divide by zero
                        if (bytes_per_sample > 0) {
                            nb_samples = pkt.size / (audio_stream->codecpar->ch_layout.nb_channels * bytes_per_sample);
                        }
                    }
#else
                    // For older FFmpeg versions
                    if (audio_stream->codecpar->channels > 0 && 
                        audio_stream->codecpar->bits_per_coded_sample > 0) {
                        int bytes_per_sample = audio_stream->codecpar->bits_per_coded_sample / 8;
                        // Ensure we don't divide by zero
                        if (bytes_per_sample > 0) {
                            nb_samples = pkt.size / (audio_stream->codecpar->channels * bytes_per_sample);
                        }
                    }
#endif
                    
                    if (nb_samples > 0) {
                        // Calculate duration based on samples and sample rate
                        pkt.duration = av_rescale_q(nb_samples, 
                                                  (AVRational){1, audio_stream->codecpar->sample_rate},
                                                  audio_stream->time_base);
                    } else {
                        // Default to a reasonable value based on sample rate
                        // Typically audio frames are ~20-40ms, so we'll use 1024 samples as a common value
                        pkt.duration = av_rescale_q(1024, 
                                                  (AVRational){1, audio_stream->codecpar->sample_rate},
                                                  audio_stream->time_base);
                    }
                    
                    log_debug("Set audio packet duration to %lld (time_base units)", (long long)pkt.duration);
                } else {
                    // If we can't calculate based on sample rate, use a default value
                    pkt.duration = 1;
                    log_debug("Set default audio packet duration to 1");
                }
            }
            
            // Set output stream index
            pkt.stream_index = out_audio_stream->index;
            
            // Write packet
            ret = av_interleaved_write_frame(output_ctx, &pkt);
            if (ret < 0) {
                log_error("Error writing audio frame: %d", ret);
            } else {
                audio_packet_count++;
            }
        }
        
        // Unref packet
        av_packet_unref(&pkt);
    }
    
    log_info("Recording segment complete");
    
    // Flag to track if trailer has been written
    bool trailer_written = false;
    
    // Write trailer
    if (output_ctx && output_ctx->pb) {
        ret = av_write_trailer(output_ctx);
        if (ret < 0) {
            log_error("Failed to write trailer: %d", ret);
        } else {
            trailer_written = true;
            log_debug("Successfully wrote trailer to output file");
        }
    }
    
cleanup:
    //  Ensure packet is properly unreferenced before cleanup
    // This prevents memory leaks if we exit the function with an active packet
    av_packet_unref(&pkt);
    
    // Close output
    if (output_ctx) {
        // Write trailer if header was written successfully and trailer hasn't been written yet
        if (output_ctx->pb && ret >= 0 && !trailer_written) {
            int trailer_ret = av_write_trailer(output_ctx);
            if (trailer_ret < 0) {
                log_error("Failed to write trailer: %d", trailer_ret);
            } else {
                log_debug("Successfully wrote trailer to output file during cleanup");
            }
        }
        
        if (output_ctx->pb) {
            // Create a local copy of the pb pointer to avoid double-free issues
            AVIOContext *pb_to_close = output_ctx->pb;
            output_ctx->pb = NULL;
            
            // Close the AVIO context
            avio_closep(&pb_to_close);
        }
        avformat_free_context(output_ctx);
    }
    
    // Free dictionaries
    av_dict_free(&opts);
    av_dict_free(&out_opts);
    
    //  Don't close input_ctx here, as it's managed by the caller
    // The caller will reuse it for the next segment or close it when done
    
    return ret;
}


/**
 * RTSP stream reading thread function
 */
static void *mp4_writer_rtsp_thread(void *arg) {
    // No static variables - we'll use a completely different approach
    
    mp4_writer_thread_t *thread_ctx = (mp4_writer_thread_t *)arg;
    AVFormatContext *input_ctx = NULL;
    AVPacket *pkt = NULL;
    int video_stream_idx = -1;
    int audio_stream_idx = -1;
    int ret;
    time_t start_time = time(NULL);  // Record when we started
    
    // Make a local copy of the stream name for thread safety
    char stream_name[MAX_STREAM_NAME];
    if (thread_ctx->writer && thread_ctx->writer->stream_name[0] != '\0') {
        strncpy(stream_name, thread_ctx->writer->stream_name, MAX_STREAM_NAME - 1);
        stream_name[MAX_STREAM_NAME - 1] = '\0';
    } else {
        strncpy(stream_name, "unknown", MAX_STREAM_NAME - 1);
    }

    log_info("Starting RTSP reading thread for stream %s", stream_name);
    
    // Add initial recording metadata to the database
    if (thread_ctx->writer && thread_ctx->writer->output_path[0] != '\0') {
        recording_metadata_t metadata;
        memset(&metadata, 0, sizeof(recording_metadata_t));
        
        // Fill in the metadata
        strncpy(metadata.stream_name, stream_name, sizeof(metadata.stream_name) - 1);
        strncpy(metadata.file_path, thread_ctx->writer->output_path, sizeof(metadata.file_path) - 1);
        metadata.start_time = start_time;
        metadata.end_time = 0; // Will be updated when recording ends
        metadata.size_bytes = 0; // Will be updated as recording grows
        metadata.is_complete = false;
        
        // Add recording to database
        uint64_t recording_id = add_recording_metadata(&metadata);
        if (recording_id == 0) {
            log_error("Failed to add initial recording metadata for stream %s", stream_name);
        } else {
            log_info("Added initial recording to database with ID: %llu for file: %s", 
                    (unsigned long long)recording_id, thread_ctx->writer->output_path);
            
            // Store the recording ID in the writer for later update
            thread_ctx->writer->current_recording_id = recording_id;
        }
    }

    // Check if we're still running (might have been stopped during initialization)
    if (!thread_ctx->running || thread_ctx->shutdown_requested) {
        log_info("RTSP reading thread for %s exiting early due to shutdown", stream_name);
        return NULL;
    }

    // Set up RTSP options for low latency - match rtsp_recorder.c exactly
    AVDictionary *opts = NULL;
    av_dict_set(&opts, "rtsp_transport", "tcp", 0);  // Use TCP for RTSP (more reliable than UDP)
    av_dict_set(&opts, "fflags", "nobuffer", 0);     // Reduce buffering
    av_dict_set(&opts, "flags", "low_delay", 0);     // Low delay mode
    av_dict_set(&opts, "max_delay", "500000", 0);    // Maximum delay of 500ms
    av_dict_set(&opts, "stimeout", "5000000", 0);    // Socket timeout in microseconds (5 seconds)
    
    // Open the input stream with options
    ret = avformat_open_input(&input_ctx, thread_ctx->rtsp_url, NULL, &opts);
    av_dict_free(&opts);
    if (ret < 0) {
        char error_buf[AV_ERROR_MAX_STRING_SIZE] = {0};
        av_strerror(ret, error_buf, AV_ERROR_MAX_STRING_SIZE);
        log_error("Could not open input stream for %s: %s", stream_name, error_buf);
        
        thread_ctx->running = 0;
        return NULL;
    }
    
    // Find stream info
    ret = avformat_find_stream_info(input_ctx, NULL);
    if (ret < 0) {
        char error_buf[AV_ERROR_MAX_STRING_SIZE] = {0};
        av_strerror(ret, error_buf, AV_ERROR_MAX_STRING_SIZE);
        log_error("Could not find stream info for %s: %s", stream_name, error_buf);
        
        // Clean up
        avformat_close_input(&input_ctx);
        
        thread_ctx->running = 0;
        return NULL;
    }
    
    // Find video and audio streams
    video_stream_idx = -1;
    audio_stream_idx = -1;
    for (unsigned int i = 0; i < input_ctx->nb_streams; i++) {
        if (input_ctx->streams[i]->codecpar->codec_type == AVMEDIA_TYPE_VIDEO) {
            video_stream_idx = i;
            log_info("Found video stream at index %d for %s", video_stream_idx, stream_name);
        } else if (input_ctx->streams[i]->codecpar->codec_type == AVMEDIA_TYPE_AUDIO) {
            audio_stream_idx = i;
            log_info("Found audio stream at index %d for %s (will be used for HLS only)", audio_stream_idx, stream_name);
        }
    }
    
    // Check if we should enable audio recording based on database configuration
    stream_config_t db_stream_config;
    int db_config_result = get_stream_config_by_name(stream_name, &db_stream_config);
    
    if (thread_ctx->writer) {
        // Default to disabled
        thread_ctx->writer->has_audio = 0;
        
        // If we have a valid database config, use the record_audio setting from there
        if (db_config_result == 0) {
            thread_ctx->writer->has_audio = db_stream_config.record_audio ? 1 : 0;
            log_info("Audio for MP4 recording for stream %s is %s (from database configuration)", 
                    stream_name, 
                    thread_ctx->writer->has_audio ? "enabled" : "disabled");
        } else {
            log_info("Disabled audio for MP4 recording for stream %s (audio will still be available in HLS)", stream_name);
        }
    }
    
    if (video_stream_idx == -1) {
        log_error("No video stream found in %s", thread_ctx->rtsp_url);
        
        // Clean up
        avformat_close_input(&input_ctx);
        
        thread_ctx->running = 0;
        return NULL;
    }
    
    // Allocate packet
    pkt = av_packet_alloc();
    if (!pkt) {
        log_error("Failed to allocate packet for %s", stream_name);
        
        // Clean up
        avformat_close_input(&input_ctx);
        
        thread_ctx->running = 0;
        return NULL;
    }
    
    log_info("Successfully set up RTSP reading for stream %s", stream_name);
    
    // Variables for activity tracking
    time_t last_activity_check = time(NULL);
    time_t last_packet_time = 0;
    
    log_info("Running RTSP reading thread for stream %s", stream_name);

    // Main loop to record segments
    while (thread_ctx->running && !thread_ctx->shutdown_requested) {
        // Check if shutdown has been initiated
        if (is_shutdown_initiated()) {
            log_info("RTSP reading thread for %s stopping due to system shutdown", stream_name);
            thread_ctx->running = 0;
            break;
        }
        
        // Get current time
        time_t current_time = time(NULL);
        
        // Fetch the latest stream configuration from the database
        stream_config_t db_stream_config;
        int db_config_result = get_stream_config_by_name(stream_name, &db_stream_config);
        
        // Define segment_duration variable outside the if block
        int segment_duration = thread_ctx->writer->segment_duration;
        
        // Update configuration from database if available
        if (db_config_result == 0) {
            // Update segment duration if available
            if (db_stream_config.segment_duration > 0) {
                segment_duration = db_stream_config.segment_duration;
                
                // Update the writer's segment duration if it has changed
                if (thread_ctx->writer->segment_duration != segment_duration) {
                    log_info("Updating segment duration for stream %s from %d to %d seconds (from database)",
                            stream_name, thread_ctx->writer->segment_duration, segment_duration);
                    thread_ctx->writer->segment_duration = segment_duration;
                }
            }
            
            // Update audio recording setting if it has changed
            int has_audio = db_stream_config.record_audio ? 1 : 0;
            if (thread_ctx->writer->has_audio != has_audio) {
                log_info("Updating audio recording setting for stream %s from %s to %s (from database)",
                        stream_name, 
                        thread_ctx->writer->has_audio ? "enabled" : "disabled",
                        has_audio ? "enabled" : "disabled");
                thread_ctx->writer->has_audio = has_audio;
            }
        }
        
        // Check if it's time to create a new segment based on segment duration
        if (segment_duration > 0 && 
            current_time - thread_ctx->writer->last_rotation_time >= segment_duration) {
            log_info("Time to create new segment for stream %s (segment duration: %d seconds)", 
                     stream_name, thread_ctx->writer->segment_duration);
            
            // Create timestamp for new MP4 filename
            char timestamp_str[32];
            struct tm *tm_info = localtime(&current_time);
            strftime(timestamp_str, sizeof(timestamp_str), "%Y%m%d_%H%M%S", tm_info);
            
            // Create new output path
            char new_path[MAX_PATH_LENGTH];
            snprintf(new_path, MAX_PATH_LENGTH, "%s/recording_%s.mp4",
                     thread_ctx->writer->output_dir, timestamp_str);
            
            // Get the current output path before closing
            char current_path[MAX_PATH_LENGTH];
            strncpy(current_path, thread_ctx->writer->output_path, MAX_PATH_LENGTH - 1);
            current_path[MAX_PATH_LENGTH - 1] = '\0';
            
            // Create recording metadata for the new file
            recording_metadata_t metadata;
            memset(&metadata, 0, sizeof(recording_metadata_t));
            
            // Fill in the metadata
            strncpy(metadata.stream_name, stream_name, sizeof(metadata.stream_name) - 1);
            strncpy(metadata.file_path, new_path, sizeof(metadata.file_path) - 1);
            metadata.start_time = current_time;
            metadata.end_time = 0; // Will be updated when recording ends
            metadata.size_bytes = 0; // Will be updated as recording grows
            metadata.is_complete = false;
            
            // Add recording to database for the new file
            uint64_t new_recording_id = add_recording_metadata(&metadata);
            if (new_recording_id == 0) {
                log_error("Failed to add recording metadata for stream %s during rotation", stream_name);
            } else {
                log_info("Added new recording to database with ID: %llu for rotated file: %s", 
                        (unsigned long long)new_recording_id, new_path);
            }
            
            // Mark the previous recording as complete
            if (thread_ctx->writer->current_recording_id > 0) {
                // Get the file size before marking as complete
                struct stat st;
                uint64_t size_bytes = 0;
                
                if (stat(current_path, &st) == 0) {
                    size_bytes = st.st_size;
                    log_info("File size for %s: %llu bytes", 
                            current_path, (unsigned long long)size_bytes);
                    
                    // Mark the recording as complete with the correct file size
                    update_recording_metadata(thread_ctx->writer->current_recording_id, current_time, size_bytes, true);
                    log_info("Marked previous recording (ID: %llu) as complete for stream %s (size: %llu bytes)", 
                            (unsigned long long)thread_ctx->writer->current_recording_id, stream_name, (unsigned long long)size_bytes);
                } else {
                    log_warn("Failed to get file size for %s: %s", 
                            current_path, strerror(errno));
                    
                    // Still mark the recording as complete, but with size 0
                    update_recording_metadata(thread_ctx->writer->current_recording_id, current_time, 0, true);
                    log_info("Marked previous recording (ID: %llu) as complete for stream %s (size unknown)", 
                            (unsigned long long)thread_ctx->writer->current_recording_id, stream_name);
                }
            }
            
            // Update the output path
            strncpy(thread_ctx->writer->output_path, new_path, MAX_PATH_LENGTH - 1);
            thread_ctx->writer->output_path[MAX_PATH_LENGTH - 1] = '\0';
            
            // Store the new recording ID in the writer for later update
            if (new_recording_id > 0) {
                thread_ctx->writer->current_recording_id = new_recording_id;
            }
            
            // Update rotation time
            thread_ctx->writer->last_rotation_time = current_time;
        }
        
        // Record a segment using the record_segment function
        log_info("Recording segment for stream %s to %s", stream_name, thread_ctx->writer->output_path);
        
        // Use the segment duration from the database or writer
        if (segment_duration > 0) {
            log_info("Using segment duration: %d seconds (from %s)", 
                    segment_duration, 
                    (db_config_result == 0 && db_stream_config.segment_duration > 0) ? "database" : "writer context");
        } else {
            segment_duration = 30;
            log_info("No segment duration configured, using default: %d seconds", segment_duration);
        }
        
        // Record the segment
        ret = record_segment(thread_ctx->rtsp_url, thread_ctx->writer->output_path, segment_duration, &input_ctx, thread_ctx->writer->has_audio);
        if (ret < 0) {
            log_error("Failed to record segment for stream %s, retrying...", stream_name);
            
            // Wait a bit before trying again
            av_usleep(1000000);  // 1 second delay
            
            // If input context was closed, set it to NULL so it will be reopened
            if (!input_ctx) {
                log_info("Input context was closed, will reopen on next attempt");
            }
        }
        
        // Update the last packet time for activity tracking
        thread_ctx->writer->last_packet_time = time(NULL);
        
        // Update the recording metadata with the current file size
        if (thread_ctx->writer->current_recording_id > 0) {
            struct stat st;
            if (stat(thread_ctx->writer->output_path, &st) == 0) {
                uint64_t size_bytes = st.st_size;
                // Update size but don't mark as complete yet
                update_recording_metadata(thread_ctx->writer->current_recording_id, 0, size_bytes, false);
                log_debug("Updated recording metadata for ID: %llu, size: %llu bytes", 
                        (unsigned long long)thread_ctx->writer->current_recording_id, 
                        (unsigned long long)size_bytes);
            }
        }
        
        // No additional processing needed - record_segment handles everything
    }

    // Clean up resources
    if (pkt) {
        // Make a local copy of the packet pointer and NULL out the original
        // to prevent double-free if another thread accesses it
        AVPacket *pkt_to_free = pkt;
        pkt = NULL;
        
        // Now safely free the packet - first unref then free to prevent memory leaks
        av_packet_unref(pkt_to_free);
        av_packet_free(&pkt_to_free);
    }
    
    if (input_ctx) {
        // Make a local copy of the context pointer and NULL out the original
        AVFormatContext *ctx_to_close = input_ctx;
        input_ctx = NULL;
        
        // Now safely close the input context
        avformat_close_input(&ctx_to_close);
    }

    log_info("RTSP reading thread for stream %s exited", stream_name);
    return NULL;
}

/**
 * Start a recording thread that reads from the RTSP stream and writes to the MP4 file
 * This function creates a new thread that handles all the recording logic
 */
int mp4_writer_start_recording_thread(mp4_writer_t *writer, const char *rtsp_url) {
    if (!writer || !rtsp_url || rtsp_url[0] == '\0') {
        log_error("Invalid parameters passed to mp4_writer_start_recording_thread");
        return -1;
    }
    
    // Create thread context
    mp4_writer_thread_t *thread_ctx = calloc(1, sizeof(mp4_writer_thread_t));
    if (!thread_ctx) {
        log_error("Failed to allocate memory for thread context");
        return -1;
    }
    
    // Initialize thread context
    thread_ctx->running = 1;
    thread_ctx->shutdown_requested = 0;
    thread_ctx->writer = writer;
    strncpy(thread_ctx->rtsp_url, rtsp_url, MAX_PATH_LENGTH - 1);
    
    // Create thread
    if (pthread_create(&thread_ctx->thread, NULL, mp4_writer_rtsp_thread, thread_ctx) != 0) {
        log_error("Failed to create RTSP reading thread for %s", writer->stream_name);
        free(thread_ctx);
        return -1;
    }
    
    // Store thread context in writer
    writer->thread_ctx = thread_ctx;
    
    // Register with shutdown coordinator
    writer->shutdown_component_id = register_component(
        writer->stream_name, 
        COMPONENT_MP4_WRITER, 
        writer, 
        10  // Medium priority
    );
    
    if (writer->shutdown_component_id >= 0) {
        log_info("Registered MP4 writer for %s with shutdown coordinator, component ID: %d", 
                writer->stream_name, writer->shutdown_component_id);
    } else {
        log_warn("Failed to register MP4 writer for %s with shutdown coordinator", writer->stream_name);
    }
    
    log_info("Started RTSP reading thread for %s", writer->stream_name);
    
    return 0;
}

/**
 * Stop the recording thread
 * This function signals the recording thread to stop and waits for it to exit
 */
void mp4_writer_stop_recording_thread(mp4_writer_t *writer) {
    if (!writer) {
        log_warn("NULL writer passed to mp4_writer_stop_recording_thread");
        return;
    }
    
    mp4_writer_thread_t *thread_ctx = (mp4_writer_thread_t *)writer->thread_ctx;
    if (!thread_ctx) {
        log_warn("No thread context found for writer %s", writer->stream_name ? writer->stream_name : "unknown");
        return;
    }
    
    // Make a local copy of the stream name for logging
    char stream_name[MAX_STREAM_NAME];
    if (writer->stream_name && writer->stream_name[0] != '\0') {
        strncpy(stream_name, writer->stream_name, MAX_STREAM_NAME - 1);
        stream_name[MAX_STREAM_NAME - 1] = '\0';
    } else {
        strncpy(stream_name, "unknown", MAX_STREAM_NAME - 1);
    }
    
    log_info("Signaling RTSP reading thread for %s to stop", stream_name);
    
    // Signal thread to stop
    thread_ctx->running = 0;
    thread_ctx->shutdown_requested = 1;
    
    // Wait for thread to exit with timeout
    #include "video/thread_utils.h"
    int join_result = pthread_join_with_timeout(thread_ctx->thread, NULL, 5);
    if (join_result != 0) {
        log_warn("Failed to join RTSP reading thread for %s within timeout: %s", 
                stream_name, strerror(join_result));
        
        //  Don't free the thread context if join failed
        // Instead, detach the thread to let it clean up itself when it eventually exits
        pthread_detach(thread_ctx->thread);
        
        // Set the thread context pointer to NULL to prevent further access
        // but don't free it as the thread might still be using it
        writer->thread_ctx = NULL;
        
        log_info("Detached RTSP reading thread for %s to prevent memory corruption", stream_name);
    } else {
        log_info("Successfully joined RTSP reading thread for %s", stream_name);
        
        // Free thread context only after successful join
        free(thread_ctx);
        writer->thread_ctx = NULL;
    }
    
    // Ensure we update the component state even if join failed
    
    // Update component state in shutdown coordinator
    if (writer->shutdown_component_id >= 0) {
        update_component_state(writer->shutdown_component_id, COMPONENT_STOPPED);
        log_info("Updated MP4 writer component state to STOPPED for %s", stream_name);
    }
    
    log_info("Stopped RTSP reading thread for %s", stream_name);
}

/**
 * Check if the recording thread is running
 */
int mp4_writer_is_recording(mp4_writer_t *writer) {
    if (!writer) {
        return 0;
    }
    
    // If the writer is in the process of rotating, consider it as still recording
    if (writer->is_rotating) {
        return 1;
    }
    
    mp4_writer_thread_t *thread_ctx = (mp4_writer_thread_t *)writer->thread_ctx;
    if (!thread_ctx) {
        return 0;
    }
    
    return thread_ctx->running;
}
