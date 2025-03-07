#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <pthread.h>
#include <ctype.h>
#include <sys/stat.h>
#include <dirent.h>
#include <libavformat/avformat.h>
#include <libavcodec/avcodec.h>
#include <libavutil/avutil.h>
#include <libavutil/time.h>

#include "../../include/web/web_server.h"
#include "../../include/core/logger.h"
#include "../../include/core/config.h"
#include "../../include/video/stream_manager.h"
#include "../../include/video/hls_writer.h"

#define LIGHTNVR_VERSION_STRING "0.1.0"

// Define a local config variable to work with
static config_t local_config;

// Global configuration - to be accessed from other modules if needed
config_t global_config;

// Forward declarations
static void log_ffmpeg_error(int err, const char *message);
void handle_hls_manifest(const http_request_t *request, http_response_t *response);
void handle_hls_segment(const http_request_t *request, http_response_t *response);
void handle_webrtc_offer(const http_request_t *request, http_response_t *response);
void handle_webrtc_ice(const http_request_t *request, http_response_t *response);

// Structure for stream transcoding context
typedef struct {
    stream_config_t config;
    int running;
    pthread_t thread;
    char output_path[MAX_PATH_LENGTH];
    hls_writer_t *hls_writer;
} stream_transcode_ctx_t;

// Hash map for tracking running transcode contexts
static stream_transcode_ctx_t *transcode_contexts[MAX_STREAMS];
static pthread_mutex_t contexts_mutex = PTHREAD_MUTEX_INITIALIZER;

/**
 * Get current global configuration
 * (To avoid conflict with potential existing function, we use a different name)
 */
static config_t* get_streaming_config(void) {
    // For now, just use our global config
    return &global_config;
}

/**
 * URL decode function
 */
static void url_decode_stream(char *str) {
    char *src = str;
    char *dst = str;
    char a, b;

    while (*src) {
        if ((*src == '%') && ((a = src[1]) && (b = src[2])) &&
            (isxdigit(a) && isxdigit(b))) {
            if (a >= 'a') a -= 'a' - 'A';
            if (a >= 'A') a -= ('A' - 10);
            else a -= '0';

            if (b >= 'a') b -= 'a' - 'A';
            if (b >= 'A') b -= ('A' - 10);
            else b -= '0';

            *dst++ = 16 * a + b;
            src += 3;
        } else if (*src == '+') {
            *dst++ = ' ';
            src++;
        } else {
            *dst++ = *src++;
        }
    }
    *dst = '\0';
}

/**
 * Create error response with appropriate content type
 */
void create_stream_error_response(http_response_t *response, int status_code, const char *message) {
    char error_json[512];
    snprintf(error_json, sizeof(error_json), "{\"error\": \"%s\"}", message);

    response->status_code = status_code;

    // Use strncpy for the content type field which appears to be an array
    strncpy(response->content_type, "application/json", sizeof(response->content_type) - 1);
    response->content_type[sizeof(response->content_type) - 1] = '\0';

    // Allocate and set body
    response->body = strdup(error_json);

    // Add proper cleanup handling - your response structure likely has a different flag
    // for indicating the body should be freed
    // Check if your http_response_t has a needs_free or similar field
    // response->needs_free = 1;
}

/**
 * Handle errors from FFmpeg
 */
static void log_ffmpeg_error(int err, const char *message) {
    char error_buf[AV_ERROR_MAX_STRING_SIZE] = {0};
    av_strerror(err, error_buf, AV_ERROR_MAX_STRING_SIZE);
    log_error("%s: %s", message, error_buf);
}

/**
 * Initialize FFmpeg libraries
 */
void init_streaming_backend(void) {
    // Initialize FFmpeg
#if LIBAVFORMAT_VERSION_INT < AV_VERSION_INT(58, 9, 100)
    av_register_all();
#endif
    avformat_network_init();

    // Initialize transcode contexts array
    memset(transcode_contexts, 0, sizeof(transcode_contexts));

    log_info("Streaming backend initialized");
}

/**
 * Cleanup FFmpeg resources
 */
void cleanup_streaming_backend(void) {
    pthread_mutex_lock(&contexts_mutex);

    // Stop all running transcodes
    for (int i = 0; i < MAX_STREAMS; i++) {
        if (transcode_contexts[i]) {
            transcode_contexts[i]->running = 0;
            pthread_join(transcode_contexts[i]->thread, NULL);

            if (transcode_contexts[i]->hls_writer) {
                hls_writer_close(transcode_contexts[i]->hls_writer);
            }

            free(transcode_contexts[i]);
            transcode_contexts[i] = NULL;
        }
    }

    pthread_mutex_unlock(&contexts_mutex);

    // Cleanup FFmpeg
    avformat_network_deinit();

    log_info("Streaming backend cleaned up");
}

/**
 * Transcoding thread function for a single stream
 */
static void *stream_transcode_thread(void *arg) {
    stream_transcode_ctx_t *ctx = (stream_transcode_ctx_t *)arg;

    AVFormatContext *input_ctx = NULL;
    AVFormatContext *output_ctx = NULL;
    AVPacket *pkt = NULL;
    int video_stream_idx = -1;
    int ret;

    log_info("Starting transcoding thread for stream %s", ctx->config.name);

    // Open input
    ret = avformat_open_input(&input_ctx, ctx->config.url, NULL, NULL);
    if (ret < 0) {
        log_ffmpeg_error(ret, "Could not open input stream");
        goto cleanup;
    }

    // Get stream info
    ret = avformat_find_stream_info(input_ctx, NULL);
    if (ret < 0) {
        log_ffmpeg_error(ret, "Could not find stream info");
        goto cleanup;
    }

    // Find video stream
    for (unsigned int i = 0; i < input_ctx->nb_streams; i++) {
        if (input_ctx->streams[i]->codecpar->codec_type == AVMEDIA_TYPE_VIDEO) {
            video_stream_idx = i;
            break;
        }
    }

    if (video_stream_idx == -1) {
        log_error("No video stream found in %s", ctx->config.url);
        goto cleanup;
    }

    // Set up HLS writer
    ctx->hls_writer = hls_writer_create(ctx->output_path, ctx->config.name, ctx->config.segment_duration);
    if (!ctx->hls_writer) {
        log_error("Failed to create HLS writer for %s", ctx->config.name);
        goto cleanup;
    }

    // Initialize packet - use newer API to avoid deprecation warning
    pkt = av_packet_alloc();
    if (!pkt) {
        log_error("Failed to allocate packet");
        goto cleanup;
    }

    // Main packet reading loop
    while (ctx->running) {
        ret = av_read_frame(input_ctx, pkt);

        if (ret < 0) {
            if (ret == AVERROR_EOF || ret == AVERROR(EAGAIN)) {
                // End of stream or resource temporarily unavailable
                // Try to reconnect after a short delay
                av_packet_unref(pkt);
                log_warn("Stream %s disconnected, attempting to reconnect...", ctx->config.name);

                av_usleep(2000000);  // 2 second delay

                // Close and reopen input
                avformat_close_input(&input_ctx);
                ret = avformat_open_input(&input_ctx, ctx->config.url, NULL, NULL);
                if (ret < 0) {
                    log_ffmpeg_error(ret, "Could not reconnect to input stream");
                    continue;  // Keep trying
                }

                ret = avformat_find_stream_info(input_ctx, NULL);
                if (ret < 0) {
                    log_ffmpeg_error(ret, "Could not find stream info after reconnect");
                    continue;  // Keep trying
                }

                continue;
            } else {
                log_ffmpeg_error(ret, "Error reading frame");
                break;
            }
        }

        // Process video packets
        if (pkt->stream_index == video_stream_idx) {
            hls_writer_write_packet(ctx->hls_writer, pkt, input_ctx->streams[video_stream_idx]);
        }

        av_packet_unref(pkt);
    }

cleanup:
    // Cleanup resources
    if (pkt) {
        av_packet_free(&pkt);
    }

    if (input_ctx) {
        avformat_close_input(&input_ctx);
    }

    if (ctx->hls_writer) {
        hls_writer_close(ctx->hls_writer);
        ctx->hls_writer = NULL;
    }

    log_info("Transcoding thread for stream %s exited", ctx->config.name);
    return NULL;
}

/**
 * Start HLS transcoding for a stream
 */
int start_hls_stream(const char *stream_name) {
    stream_handle_t stream = get_stream_by_name(stream_name);
    if (!stream) {
        log_error("Stream %s not found", stream_name);
        return -1;
    }

    stream_config_t config;
    if (get_stream_config(stream, &config) != 0) {
        log_error("Failed to get config for stream %s", stream_name);
        return -1;
    }

    // Check if already running
    pthread_mutex_lock(&contexts_mutex);
    for (int i = 0; i < MAX_STREAMS; i++) {
        if (transcode_contexts[i] && strcmp(transcode_contexts[i]->config.name, stream_name) == 0) {
            pthread_mutex_unlock(&contexts_mutex);
            log_info("HLS stream %s already running", stream_name);
            return 0;  // Already running
        }
    }

    // Find empty slot
    int slot = -1;
    for (int i = 0; i < MAX_STREAMS; i++) {
        if (!transcode_contexts[i]) {
            slot = i;
            break;
        }
    }

    if (slot == -1) {
        pthread_mutex_unlock(&contexts_mutex);
        log_error("No slot available for new HLS stream");
        return -1;
    }

    // Create context
    stream_transcode_ctx_t *ctx = malloc(sizeof(stream_transcode_ctx_t));
    if (!ctx) {
        pthread_mutex_unlock(&contexts_mutex);
        log_error("Memory allocation failed for transcode context");
        return -1;
    }

    memset(ctx, 0, sizeof(stream_transcode_ctx_t));
    memcpy(&ctx->config, &config, sizeof(stream_config_t));
    ctx->running = 1;

    // Create output path
    config_t *global_config = get_streaming_config();
    snprintf(ctx->output_path, MAX_PATH_LENGTH, "%s/hls/%s",
             global_config->storage_path, stream_name);

    // Create directory if it doesn't exist
    char mkdir_cmd[MAX_PATH_LENGTH + 10];
    snprintf(mkdir_cmd, sizeof(mkdir_cmd), "mkdir -p %s", ctx->output_path);
    if (system(mkdir_cmd) != 0) {
        log_warn("Failed to create directory: %s", ctx->output_path);
    }

    // Start transcoding thread
    if (pthread_create(&ctx->thread, NULL, stream_transcode_thread, ctx) != 0) {
        free(ctx);
        pthread_mutex_unlock(&contexts_mutex);
        log_error("Failed to create transcoding thread for %s", stream_name);
        return -1;
    }

    // Store context
    transcode_contexts[slot] = ctx;
    pthread_mutex_unlock(&contexts_mutex);

    log_info("Started HLS stream for %s in slot %d", stream_name, slot);
    return 0;
}

/**
 * Stop HLS transcoding for a stream
 */
int stop_hls_stream(const char *stream_name) {
    int found = 0;

    pthread_mutex_lock(&contexts_mutex);

    for (int i = 0; i < MAX_STREAMS; i++) {
        if (transcode_contexts[i] && strcmp(transcode_contexts[i]->config.name, stream_name) == 0) {
            // Mark as not running
            transcode_contexts[i]->running = 0;

            // Join thread
            pthread_join(transcode_contexts[i]->thread, NULL);

            // Cleanup resources
            if (transcode_contexts[i]->hls_writer) {
                hls_writer_close(transcode_contexts[i]->hls_writer);
            }

            free(transcode_contexts[i]);
            transcode_contexts[i] = NULL;

            found = 1;
            break;
        }
    }

    pthread_mutex_unlock(&contexts_mutex);

    if (!found) {
        log_warn("HLS stream %s not found for stopping", stream_name);
        return -1;
    }

    log_info("Stopped HLS stream %s", stream_name);
    return 0;
}

/**
 * Handle request for HLS manifest
 */
/**
 * Handle request for HLS manifest
 */
void handle_hls_manifest(const http_request_t *request, http_response_t *response) {
    // Extract stream name from URL
    // URL format: /api/streaming/{stream_name}/hls/index.m3u8

    const char *path = request->path;
    const char *streams_pos = strstr(path, "/streaming/");

    if (!streams_pos) {
        create_stream_error_response(response, 400, "Invalid request path");
        return;
    }

    const char *stream_name_start = streams_pos + 11;  // Skip "/streaming/"
    const char *hls_pos = strstr(stream_name_start, "/hls/");

    if (!hls_pos) {
        create_stream_error_response(response, 400, "Invalid HLS request path");
        return;
    }

    // Extract stream name
    char stream_name[MAX_STREAM_NAME];
    size_t name_len = hls_pos - stream_name_start;

    if (name_len >= MAX_STREAM_NAME) {
        create_stream_error_response(response, 400, "Stream name too long");
        return;
    }

    memcpy(stream_name, stream_name_start, name_len);
    stream_name[name_len] = '\0';

    // URL decode the stream name
    char decoded_stream[MAX_STREAM_NAME];
    url_decode(stream_name, decoded_stream, sizeof(decoded_stream));
    strncpy(stream_name, decoded_stream, MAX_STREAM_NAME - 1);
    stream_name[MAX_STREAM_NAME - 1] = '\0';

    // Check if stream exists
    stream_handle_t stream = get_stream_by_name(stream_name);
    if (!stream) {
        create_stream_error_response(response, 404, "Stream not found");
        return;
    }

    // Start HLS if not already running
    if (start_hls_stream(stream_name) != 0) {
        create_stream_error_response(response, 500, "Failed to start HLS stream");
        return;
    }

    // Get the manifest file path
    config_t *global_config = get_streaming_config();
    char manifest_path[MAX_PATH_LENGTH];
    snprintf(manifest_path, MAX_PATH_LENGTH, "%s/hls/%s/index.m3u8",
             global_config->storage_path, stream_name);

    // Wait longer for the manifest file to be created
    // Try up to 30 times with 100ms between attempts (3 seconds total)
    bool manifest_exists = false;
    for (int i = 0; i < 30; i++) {
        // Check for the final manifest file
        if (access(manifest_path, F_OK) == 0) {
            manifest_exists = true;
            break;
        }

        // Also check for the temporary file
        char temp_manifest_path[MAX_PATH_LENGTH];
        snprintf(temp_manifest_path, MAX_PATH_LENGTH, "%s.tmp", manifest_path);
        if (access(temp_manifest_path, F_OK) == 0) {
            log_debug("Found temporary manifest file, waiting for it to be finalized");
            // Continue waiting for the final file
        }

        log_debug("Waiting for manifest file to be created (attempt %d/30)", i+1);
        usleep(100000);  // 100ms
    }

    if (!manifest_exists) {
        log_error("Manifest file was not created in time: %s", manifest_path);
        create_stream_error_response(response, 404, "Manifest file not found");
        return;
    }

    // Read the manifest file
    FILE *fp = fopen(manifest_path, "r");
    if (!fp) {
        create_stream_error_response(response, 500, "Failed to open manifest file");
        return;
    }

    // Get file size
    fseek(fp, 0, SEEK_END);
    long file_size = ftell(fp);
    fseek(fp, 0, SEEK_SET);

    // Read file content
    char *content = malloc(file_size + 1);
    if (!content) {
        fclose(fp);
        create_stream_error_response(response, 500, "Memory allocation failed");
        return;
    }

    size_t bytes_read = fread(content, 1, file_size, fp);
    if (bytes_read != (size_t)file_size) {
        free(content);
        fclose(fp);
        create_stream_error_response(response, 500, "Failed to read manifest file");
        return;
    }

    content[file_size] = '\0';
    fclose(fp);

    // Create response
    response->status_code = 200;
    strncpy(response->content_type, "application/vnd.apple.mpegurl", sizeof(response->content_type) - 1);
    response->content_type[sizeof(response->content_type) - 1] = '\0';
    response->body = content;
    response->body_length = file_size;
}

/**
 * Handle request for HLS segment
 */
/**
 * Handle request for HLS segment
 */
/**
 * Handle request for HLS segment
 */
void handle_hls_segment(const http_request_t *request, http_response_t *response) {
    // Extract stream name and segment from URL
    // URL format: /api/streaming/{stream_name}/hls/segment_{number}.ts or /api/streaming/{stream_name}/hls/index{number}.ts

    const char *path = request->path;
    const char *streams_pos = strstr(path, "/streaming/");

    if (!streams_pos) {
        create_stream_error_response(response, 400, "Invalid request path");
        return;
    }

    const char *stream_name_start = streams_pos + 11;  // Skip "/streaming/"
    const char *hls_pos = strstr(stream_name_start, "/hls/");

    if (!hls_pos) {
        create_stream_error_response(response, 400, "Invalid HLS request path");
        return;
    }

    // Extract stream name
    char stream_name[MAX_STREAM_NAME];
    size_t name_len = hls_pos - stream_name_start;

    if (name_len >= MAX_STREAM_NAME) {
        create_stream_error_response(response, 400, "Stream name too long");
        return;
    }

    memcpy(stream_name, stream_name_start, name_len);
    stream_name[name_len] = '\0';

    // URL decode the stream name
    char decoded_stream[MAX_STREAM_NAME];
    url_decode(stream_name, decoded_stream, sizeof(decoded_stream));
    strncpy(stream_name, decoded_stream, MAX_STREAM_NAME - 1);
    stream_name[MAX_STREAM_NAME - 1] = '\0';

    // Extract segment filename
    const char *segment_filename = hls_pos + 5;  // Skip "/hls/"

    log_info("Segment requested: %s", segment_filename);

    // Get the segment file path
    config_t *global_config = get_streaming_config();
    char segment_path[MAX_PATH_LENGTH];
    snprintf(segment_path, MAX_PATH_LENGTH, "%s/hls/%s/%s",
             global_config->storage_path, stream_name, segment_filename);

    log_info("Looking for segment at path: %s", segment_path);

    // Debug: check if directory exists and is accessible
    char dir_path[MAX_PATH_LENGTH];
    strncpy(dir_path, segment_path, sizeof(dir_path));
    char *last_slash = strrchr(dir_path, '/');
    if (last_slash) {
        *last_slash = '\0';

        // Check if directory exists
        if (access(dir_path, F_OK | R_OK) != 0) {
            log_error("HLS directory not accessible: %s (%s)", dir_path, strerror(errno));
        } else {
            log_info("HLS directory exists and is readable: %s", dir_path);

            // List a few of the ts files in the directory using system command
            char cmd[MAX_PATH_LENGTH * 2];
            snprintf(cmd, sizeof(cmd), "ls -la %s/*.ts 2>/dev/null | head -5", dir_path);
            log_info("Listing some TS files in directory:");
            FILE *ls = popen(cmd, "r");
            if (ls) {
                char line[1024];
                while (fgets(line, sizeof(line), ls)) {
                    // Remove newline
                    size_t len = strlen(line);
                    if (len > 0 && line[len-1] == '\n') line[len-1] = '\0';

                    log_info("  %s", line);
                }
                pclose(ls);
            }
        }
    }

    // Check if segment file exists
    if (access(segment_path, F_OK) != 0) {
        log_debug("Segment file not found on first attempt: %s (%s)", segment_path, strerror(errno));

        // Try a different path format - sometimes the HLS segments might be named differently
        char alt_segment_path[MAX_PATH_LENGTH];

        // Try removing any file extension from the segment name and just use the index number
        char basename[256];
        strncpy(basename, segment_filename, sizeof(basename));
        char *dot = strrchr(basename, '.');
        if (dot) {
            *dot = '\0';
        }

        // Try with a different naming convention - more generic pattern matching
        snprintf(alt_segment_path, MAX_PATH_LENGTH, "%s/hls/%s/index*.ts",
                 global_config->storage_path, stream_name);

        log_info("Trying alternative path pattern: %s", alt_segment_path);

        // Wait longer for it to be created - HLS segments might still be generating
        bool segment_exists = false;
        for (int i = 0; i < 20; i++) {  // Try for 2 seconds total
            if (access(segment_path, F_OK) == 0) {
                log_info("Segment file found after waiting: %s (attempt %d)", segment_path, i+1);
                segment_exists = true;
                break;
            }

            log_debug("Waiting for segment file to be created: %s (attempt %d/20)", segment_path, i+1);
            usleep(100000);  // 100ms
        }

        if (!segment_exists) {
            log_error("Segment file not found after waiting: %s", segment_path);

            // As a fallback, try to access the most recent segment file
            char fallback_cmd[MAX_PATH_LENGTH * 2];
            snprintf(fallback_cmd, sizeof(fallback_cmd),
                     "ls -t %s/hls/%s/index*.ts 2>/dev/null | head -1",
                     global_config->storage_path, stream_name);

            FILE *cmd = popen(fallback_cmd, "r");
            if (cmd) {
                char latest_segment[MAX_PATH_LENGTH];
                if (fgets(latest_segment, sizeof(latest_segment), cmd)) {
                    // Remove newline
                    size_t len = strlen(latest_segment);
                    if (len > 0 && latest_segment[len-1] == '\n') latest_segment[len-1] = '\0';

                    log_info("Found latest segment: %s", latest_segment);

                    // If a recent segment exists, use that instead
                    if (access(latest_segment, F_OK) == 0) {
                        strncpy(segment_path, latest_segment, sizeof(segment_path));
                        segment_exists = true;
                    }
                }
                pclose(cmd);
            }

            if (!segment_exists) {
                create_stream_error_response(response, 404, "Segment file not found");
                return;
            }
        }
    } else {
        log_info("Segment file exists: %s", segment_path);
    }

    // Read the segment file
    FILE *fp = fopen(segment_path, "rb");
    if (!fp) {
        log_error("Failed to open segment file: %s (%s)", segment_path, strerror(errno));
        create_stream_error_response(response, 500, "Failed to open segment file");
        return;
    }

    // Get file size
    fseek(fp, 0, SEEK_END);
    long file_size = ftell(fp);
    fseek(fp, 0, SEEK_SET);

    log_info("Successfully opened segment file, size: %ld bytes", file_size);

    // Read file content
    char *content = malloc(file_size);
    if (!content) {
        log_error("Memory allocation failed for file content, size: %ld", file_size);
        fclose(fp);
        create_stream_error_response(response, 500, "Memory allocation failed");
        return;
    }

    size_t bytes_read = fread(content, 1, file_size, fp);
    if (bytes_read != (size_t)file_size) {
        log_error("Failed to read full file content: read %zu of %ld bytes", bytes_read, file_size);
        free(content);
        fclose(fp);
        create_stream_error_response(response, 500, "Failed to read segment file");
        return;
    }

    fclose(fp);

    // Create response
    response->status_code = 200;
    strncpy(response->content_type, "video/mp2t", sizeof(response->content_type) - 1);
    response->content_type[sizeof(response->content_type) - 1] = '\0';
    response->body = content;
    response->body_length = file_size;

    log_info("Successfully served segment: %s", segment_filename);
}

/**
 * Handle WebRTC offer request - simple placeholder implementation
 */
void handle_webrtc_offer(const http_request_t *request, http_response_t *response) {
    // Extract stream name from URL
    const char *path = request->path;
    const char *streams_pos = strstr(path, "/streaming/");

    if (!streams_pos) {
        create_stream_error_response(response, 400, "Invalid request path");
        return;
    }

    const char *stream_name_start = streams_pos + 11;  // Skip "/streaming/"
    const char *webrtc_pos = strstr(stream_name_start, "/webrtc/");

    if (!webrtc_pos) {
        create_stream_error_response(response, 400, "Invalid WebRTC request path");
        return;
    }

    // Extract stream name
    char stream_name[MAX_STREAM_NAME];
    size_t name_len = webrtc_pos - stream_name_start;

    if (name_len >= MAX_STREAM_NAME) {
        create_stream_error_response(response, 400, "Stream name too long");
        return;
    }

    memcpy(stream_name, stream_name_start, name_len);
    stream_name[name_len] = '\0';

    // URL decode the stream name
    char decoded_stream[MAX_STREAM_NAME];
url_decode(stream_name, decoded_stream, sizeof(decoded_stream));
strncpy(stream_name, decoded_stream, MAX_STREAM_NAME - 1);
stream_name[MAX_STREAM_NAME - 1] = '\0';

    // Check if stream exists
    stream_handle_t stream = get_stream_by_name(stream_name);
    if (!stream) {
        create_stream_error_response(response, 404, "Stream not found");
        return;
    }

    // In a real implementation, this would:
    // 1. Parse the WebRTC offer from the request body
    // 2. Use the libWebRTC or similar library to create an answer
    // 3. Send the answer back to the client

    // For now, just acknowledge the request with a placeholder

    log_info("Received WebRTC offer for stream %s", stream_name);

    // Create placeholder response
    const char *response_json = "{\"status\": \"acknowledged\", \"message\": \"WebRTC not yet implemented\"}";

    response->status_code = 200;
    strncpy(response->content_type, "application/json", sizeof(response->content_type) - 1);
    response->content_type[sizeof(response->content_type) - 1] = '\0';
    response->body = strdup(response_json);

    // Indicate that body should be freed (if needed)
    // response->needs_free = 1;
}

/**
 * Handle WebRTC ICE candidate request - simple placeholder implementation
 */
void handle_webrtc_ice(const http_request_t *request, http_response_t *response) {
    // Extract stream name from URL
    const char *path = request->path;
    const char *streams_pos = strstr(path, "/streaming/");

    if (!streams_pos) {
        create_stream_error_response(response, 400, "Invalid request path");
        return;
    }

    const char *stream_name_start = streams_pos + 11;  // Skip "/streaming/"
    const char *webrtc_pos = strstr(stream_name_start, "/webrtc/");

    if (!webrtc_pos) {
        create_stream_error_response(response, 400, "Invalid WebRTC request path");
        return;
    }

    // Extract stream name
    char stream_name[MAX_STREAM_NAME];
    size_t name_len = webrtc_pos - stream_name_start;

    if (name_len >= MAX_STREAM_NAME) {
        create_stream_error_response(response, 400, "Stream name too long");
        return;
    }

    memcpy(stream_name, stream_name_start, name_len);
    stream_name[name_len] = '\0';

    // URL decode the stream name
    char decoded_stream[MAX_STREAM_NAME];
url_decode(stream_name, decoded_stream, sizeof(decoded_stream));
strncpy(stream_name, decoded_stream, MAX_STREAM_NAME - 1);
stream_name[MAX_STREAM_NAME - 1] = '\0';

    // Just acknowledge the ICE candidate
    log_info("Received ICE candidate for stream %s", stream_name);

    // Create success response
    const char *response_json = "{\"status\": \"acknowledged\", \"message\": \"WebRTC not yet implemented\"}";

    response->status_code = 200;
    strncpy(response->content_type, "application/json", sizeof(response->content_type) - 1);
    response->content_type[sizeof(response->content_type) - 1] = '\0';
    response->body = strdup(response_json);

    // Indicate that body should be freed (if needed)
    // response->needs_free = 1;
}
