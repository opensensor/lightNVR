/**
 * RTSP to MP4 Recorder
 * 
 * A standalone test program that records an RTSP stream to an MP4 file
 * for a specified duration (default 20 seconds).
 * 
 * This is analogous to:
 * ffplay -i rtsp://thingino:thingino@192.168.50.49:554/ch0 -fflags nobuffer -flags low_delay -framedrop
 * 
 * But for recording instead of playing.
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <signal.h>
#include <time.h>

#include <libavformat/avformat.h>
#include <libavcodec/avcodec.h>
#include <libavutil/avutil.h>
#include <libavutil/time.h>
#include <libavutil/opt.h>

// Default values
#define DEFAULT_DURATION 20  // Default recording duration in seconds
#define DEFAULT_OUTPUT "output.mp4"
#define DEFAULT_RTSP_URL "rtsp://thingino:thingino@192.168.50.49:554/ch0"

// Global variables for signal handling
static int stop_recording = 0;

// Signal handler for graceful termination
void handle_signal(int sig) {
    printf("Received signal %d, stopping recording...\n", sig);
    stop_recording = 1;
}

// Function to log FFmpeg errors
void log_error(int err, const char *message) {
    char error_buf[AV_ERROR_MAX_STRING_SIZE] = {0};
    av_strerror(err, error_buf, AV_ERROR_MAX_STRING_SIZE);
    fprintf(stderr, "%s: %s\n", message, error_buf);
}

int main(int argc, char *argv[]) {
    int ret;
    int duration = DEFAULT_DURATION;
    const char *output_file = DEFAULT_OUTPUT;
    const char *rtsp_url = DEFAULT_RTSP_URL;
    
    // Parse command line arguments
    for (int i = 1; i < argc; i++) {
        if (!strcmp(argv[i], "-d") || !strcmp(argv[i], "--duration")) {
            if (i + 1 < argc) {
                duration = atoi(argv[i + 1]);
                if (duration <= 0) {
                    duration = DEFAULT_DURATION;
                }
                i++;
            }
        } else if (!strcmp(argv[i], "-o") || !strcmp(argv[i], "--output")) {
            if (i + 1 < argc) {
                output_file = argv[i + 1];
                i++;
            }
        } else if (!strcmp(argv[i], "-i") || !strcmp(argv[i], "--input")) {
            if (i + 1 < argc) {
                rtsp_url = argv[i + 1];
                i++;
            }
        } else if (!strcmp(argv[i], "-h") || !strcmp(argv[i], "--help")) {
            printf("Usage: %s [options]\n", argv[0]);
            printf("Options:\n");
            printf("  -i, --input URL     RTSP URL to record (default: %s)\n", DEFAULT_RTSP_URL);
            printf("  -o, --output FILE   Output MP4 file (default: %s)\n", DEFAULT_OUTPUT);
            printf("  -d, --duration SEC  Recording duration in seconds (default: %d)\n", DEFAULT_DURATION);
            printf("  -h, --help          Show this help message\n");
            return 0;
        }
    }
    
    printf("Recording from %s\n", rtsp_url);
    printf("Output file: %s\n", output_file);
    printf("Duration: %d seconds\n", duration);
    
    // Register signal handlers for graceful termination
    signal(SIGINT, handle_signal);
    signal(SIGTERM, handle_signal);
    
    // Initialize FFmpeg network
#if LIBAVFORMAT_VERSION_INT < AV_VERSION_INT(58, 9, 100)
    av_register_all();
#endif
    avformat_network_init();
    
    // Input context
    AVFormatContext *input_ctx = NULL;
    
    // Set up RTSP options for low latency
    AVDictionary *opts = NULL;
    av_dict_set(&opts, "rtsp_transport", "tcp", 0);  // Use TCP for RTSP (more reliable than UDP)
    av_dict_set(&opts, "fflags", "nobuffer", 0);     // Reduce buffering
    av_dict_set(&opts, "flags", "low_delay", 0);     // Low delay mode
    av_dict_set(&opts, "max_delay", "500000", 0);    // Maximum delay of 500ms
    av_dict_set(&opts, "stimeout", "5000000", 0);    // Socket timeout in microseconds (5 seconds)
    
    // Open input
    ret = avformat_open_input(&input_ctx, rtsp_url, NULL, &opts);
    if (ret < 0) {
        log_error(ret, "Failed to open input");
        goto cleanup;
    }
    
    // Find stream info
    ret = avformat_find_stream_info(input_ctx, NULL);
    if (ret < 0) {
        log_error(ret, "Failed to find stream info");
        goto cleanup;
    }
    
    // Print input stream info
    printf("Input format: %s\n", input_ctx->iformat->name);
    printf("Number of streams: %d\n", input_ctx->nb_streams);
    
    // Find video and audio streams
    int video_stream_idx = -1;
    int audio_stream_idx = -1;
    
    for (unsigned int i = 0; i < input_ctx->nb_streams; i++) {
        AVStream *stream = input_ctx->streams[i];
        if (stream->codecpar->codec_type == AVMEDIA_TYPE_VIDEO && video_stream_idx < 0) {
            video_stream_idx = i;
            printf("Found video stream: %d\n", i);
            printf("  Codec: %s\n", avcodec_get_name(stream->codecpar->codec_id));
            printf("  Resolution: %dx%d\n", stream->codecpar->width, stream->codecpar->height);
            if (stream->avg_frame_rate.num && stream->avg_frame_rate.den) {
                printf("  Frame rate: %.2f fps\n", 
                       (float)stream->avg_frame_rate.num / stream->avg_frame_rate.den);
            }
        } else if (stream->codecpar->codec_type == AVMEDIA_TYPE_AUDIO && audio_stream_idx < 0) {
            audio_stream_idx = i;
            printf("Found audio stream: %d\n", i);
            printf("  Codec: %s\n", avcodec_get_name(stream->codecpar->codec_id));
            printf("  Sample rate: %d Hz\n", stream->codecpar->sample_rate);
            // Handle channel count for different FFmpeg versions
#if LIBAVCODEC_VERSION_INT >= AV_VERSION_INT(59, 37, 100)
            // For FFmpeg 5.0 and newer
            printf("  Channels: %d\n", stream->codecpar->ch_layout.nb_channels);
#else
            // For older FFmpeg versions
            printf("  Channels: %d\n", stream->codecpar->channels);
#endif
        }
    }
    
    if (video_stream_idx < 0) {
        fprintf(stderr, "No video stream found\n");
        goto cleanup;
    }
    
    // Output context
    AVFormatContext *output_ctx = NULL;
    
    // Create output context
    ret = avformat_alloc_output_context2(&output_ctx, NULL, "mp4", output_file);
    if (ret < 0 || !output_ctx) {
        log_error(ret, "Failed to create output context");
        goto cleanup;
    }
    
    // Create output streams (copy from input)
    AVStream *out_video_stream = NULL;
    AVStream *out_audio_stream = NULL;
    
    // Add video stream
    out_video_stream = avformat_new_stream(output_ctx, NULL);
    if (!out_video_stream) {
        fprintf(stderr, "Failed to create output video stream\n");
        goto cleanup;
    }
    
    // Copy video codec parameters
    ret = avcodec_parameters_copy(out_video_stream->codecpar, 
                                 input_ctx->streams[video_stream_idx]->codecpar);
    if (ret < 0) {
        log_error(ret, "Failed to copy video codec parameters");
        goto cleanup;
    }
    
    // Set video stream time base
    out_video_stream->time_base = input_ctx->streams[video_stream_idx]->time_base;
    
    // Add audio stream if available
    if (audio_stream_idx >= 0) {
        out_audio_stream = avformat_new_stream(output_ctx, NULL);
        if (!out_audio_stream) {
            fprintf(stderr, "Failed to create output audio stream\n");
            goto cleanup;
        }
        
        // Copy audio codec parameters
        ret = avcodec_parameters_copy(out_audio_stream->codecpar, 
                                     input_ctx->streams[audio_stream_idx]->codecpar);
        if (ret < 0) {
            log_error(ret, "Failed to copy audio codec parameters");
            goto cleanup;
        }
        
        // Set audio stream time base
        out_audio_stream->time_base = input_ctx->streams[audio_stream_idx]->time_base;
    }
    
    // Set output options for fast start
    AVDictionary *out_opts = NULL;
    av_dict_set(&out_opts, "movflags", "+faststart", 0);
    
    // Open output file
    ret = avio_open(&output_ctx->pb, output_file, AVIO_FLAG_WRITE);
    if (ret < 0) {
        log_error(ret, "Failed to open output file");
        goto cleanup;
    }
    
    // Write file header
    ret = avformat_write_header(output_ctx, &out_opts);
    if (ret < 0) {
        log_error(ret, "Failed to write header");
        goto cleanup;
    }
    
    // Packet for reading/writing
    AVPacket pkt;
    av_init_packet(&pkt);
    pkt.data = NULL;
    pkt.size = 0;
    
    // Timestamp tracking
    int64_t start_time = av_gettime();
    int64_t first_video_dts = AV_NOPTS_VALUE;
    int64_t first_audio_dts = AV_NOPTS_VALUE;
    
    printf("Recording started...\n");
    
    // Main recording loop
    while (!stop_recording) {
        // Check if we've reached the duration limit
        if (duration > 0 && (av_gettime() - start_time) / 1000000 >= duration) {
            printf("Reached duration limit of %d seconds\n", duration);
            break;
        }
        
        // Read packet
        ret = av_read_frame(input_ctx, &pkt);
        if (ret < 0) {
            if (ret == AVERROR_EOF) {
                printf("End of stream reached\n");
                break;
            } else if (ret != AVERROR(EAGAIN)) {
                log_error(ret, "Error reading frame");
                break;
            }
            // EAGAIN means try again, so we continue
            av_usleep(10000);  // Sleep 10ms to avoid busy waiting
            continue;
        }
        
        // Process video packets
        if (pkt.stream_index == video_stream_idx) {
            // Initialize first DTS if not set
            if (first_video_dts == AV_NOPTS_VALUE && pkt.dts != AV_NOPTS_VALUE) {
                first_video_dts = pkt.dts;
                printf("First video DTS: %lld\n", (long long)first_video_dts);
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
            
            // Set output stream index
            pkt.stream_index = out_video_stream->index;
            
            // Write packet
            ret = av_interleaved_write_frame(output_ctx, &pkt);
            if (ret < 0) {
                log_error(ret, "Error writing video frame");
            }
        }
        // Process audio packets
        else if (audio_stream_idx >= 0 && pkt.stream_index == audio_stream_idx && out_audio_stream) {
            // Initialize first audio DTS if not set
            if (first_audio_dts == AV_NOPTS_VALUE && pkt.dts != AV_NOPTS_VALUE) {
                first_audio_dts = pkt.dts;
                printf("First audio DTS: %lld\n", (long long)first_audio_dts);
            }
            
            // Use a static variable to track the last audio DTS we've seen
            static int64_t last_audio_dts = 0;
            static int64_t last_audio_pts = 0;
            static int audio_packet_count = 0;
            
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
            
            // Set output stream index
            pkt.stream_index = out_audio_stream->index;
            
            // Write packet
            ret = av_interleaved_write_frame(output_ctx, &pkt);
            if (ret < 0) {
                log_error(ret, "Error writing audio frame");
            } else {
                audio_packet_count++;
            }
        }
        
        // Unref packet
        av_packet_unref(&pkt);
        
        // Print progress every second
        static time_t last_progress = 0;
        time_t now = time(NULL);
        if (now != last_progress) {
            printf("\rRecording: %ld/%d seconds...", (now - (start_time / 1000000)), duration);
            fflush(stdout);
            last_progress = now;
        }
    }
    
    printf("\nRecording complete\n");
    
    // Write trailer
    if (output_ctx) {
        ret = av_write_trailer(output_ctx);
        if (ret < 0) {
            log_error(ret, "Failed to write trailer");
        }
    }
    
cleanup:
    // Close input
    if (input_ctx) {
        avformat_close_input(&input_ctx);
    }
    
    // Close output
    if (output_ctx) {
        if (output_ctx->pb) {
            avio_closep(&output_ctx->pb);
        }
        avformat_free_context(output_ctx);
    }
    
    // Free dictionaries
    av_dict_free(&opts);
    
    // Cleanup FFmpeg
    avformat_network_deinit();
    
    printf("Done\n");
    return ret < 0 ? 1 : 0;
}
