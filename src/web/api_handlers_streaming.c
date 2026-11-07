/*
 * SPDX-License-Identifier: GPL-3.0-or-later
 * Copyright (C) 2025-2026 OpenSensor Engineering and LightNVR contributors
 *
 * This file is part of LightNVR <https://github.com/opensensor/lightNVR>.
 */

// This file provides implementations of the HLS API functions

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/stat.h>
#include <libavformat/avformat.h>
#include <libavutil/mem.h>
#include "web/request_response.h"
#include "web/httpd_utils.h"
#define LOG_COMPONENT "StreamsAPI"
#include "core/logger.h"
#include "core/config.h"
#include "core/path_utils.h"
#include "utils/strings.h"
#include "web/http_server.h"
#include "video/streams.h"

/* Native HLS uses independent MPEG-TS segments. Remux only video for a
 * principal without audio.listen; container metadata is rebuilt, so neither
 * audio packets nor an advertised audio PID survives. Never invoke a shell or
 * transcode. Bound both input and output to keep this request inexpensive. */
#define VIDEO_SEGMENT_LIMIT (32 * 1024 * 1024)
typedef struct { unsigned char *data; size_t size; } video_segment_t;
static int video_segment_write(void *opaque, const uint8_t *data, int size) {
    video_segment_t *segment = opaque;
    if (size < 0 || segment->size + (size_t)size > VIDEO_SEGMENT_LIMIT) return AVERROR(ENOMEM);
    unsigned char *next = realloc(segment->data, segment->size + size);
    if (!next) return AVERROR(ENOMEM);
    segment->data = next;
    memcpy(next + segment->size, data, size);
    segment->size += size;
    return size;
}
static bool serve_video_segment(const char *path, http_response_t *res) {
    AVFormatContext *input = NULL, *output = NULL;
    AVIOContext *io = NULL;
    AVPacket *packet = NULL;
    AVDictionary *options = NULL;
    video_segment_t segment = {0};
    bool success = false;
    int video = -1;
    av_dict_set(&options, "protocol_whitelist", "file", 0);
    if (avformat_open_input(&input, path, av_find_input_format("mpegts"), &options) < 0) goto done;
    if (avformat_find_stream_info(input, NULL) < 0) goto done;
    video = av_find_best_stream(input, AVMEDIA_TYPE_VIDEO, -1, -1, NULL, 0);
    if (video < 0 || avformat_alloc_output_context2(&output, NULL, "mpegts", NULL) < 0) goto done;
    AVStream *stream = avformat_new_stream(output, NULL);
    if (!stream || avcodec_parameters_copy(stream->codecpar, input->streams[video]->codecpar) < 0) goto done;
    stream->time_base = input->streams[video]->time_base;
    unsigned char *buffer = av_malloc(32768);
    if (!buffer) goto done;
    io = avio_alloc_context(buffer, 32768, 1, &segment, NULL, video_segment_write, NULL);
    if (!io) { av_free(buffer); goto done; }
    output->pb = io;
    output->flags |= AVFMT_FLAG_CUSTOM_IO;
    if (avformat_write_header(output, NULL) < 0) goto done;
    packet = av_packet_alloc();
    if (!packet) goto done;
    int result;
    while ((result = av_read_frame(input, packet)) >= 0) {
        if (packet->stream_index == video) {
            av_packet_rescale_ts(packet, input->streams[video]->time_base, stream->time_base);
            packet->stream_index = 0;
            packet->pos = -1;
            if (av_interleaved_write_frame(output, packet) < 0) goto done;
        }
        av_packet_unref(packet);
    }
    if (result != AVERROR_EOF || av_write_trailer(output) < 0) goto done;
    avio_flush(io);
    if (io->error || !segment.size) goto done;
    res->status_code = 200;
    strcpy(res->content_type, "video/mp2t");
    res->body = segment.data;
    res->body_length = segment.size;
    res->body_allocated = true;
    segment.data = NULL;
    http_response_add_header(res, "Cache-Control", "private, no-store");
    success = true;
done:
    av_dict_free(&options);
    av_packet_free(&packet);
    avformat_close_input(&input);
    avformat_free_context(output);
    if (io) { av_freep(&io->buffer); avio_context_free(&io); }
    free(segment.data);
    return success;
}

/**
 * @brief Backend-agnostic handler for direct HLS requests
 * Endpoint: /hls/{stream_name}/{file}
 */
void handle_direct_hls_request(const http_request_t *req, http_response_t *res) {
    if (!req || !res) {
        log_error("Invalid parameters in handle_direct_hls_request");
        return;
    }

    log_debug("HLS API: Handling direct HLS request");

    // Extract URI
    const char *uri = req->path;
    size_t uri_len = strlen(uri);

    // Extract stream name from URI
    // URI format: /hls/{stream_name}/{file}

    // Validate URI format
    if (uri_len < 6 || strncmp(uri, "/hls/", 5) != 0) {
        log_error("Invalid HLS URI format: %s", uri);
        http_response_set_json_error(res, 400, "Invalid HLS path");
        return;
    }

    const char *stream_start = uri + 5; // Skip "/hls/"
    const char *file_part = strchr(stream_start, '/');

    if (!file_part) {
        log_error("Failed to extract stream name from URI: %s", uri);
        http_response_set_json_error(res, 400, "Invalid HLS path");
        return;
    }

    // Extract stream name
    size_t name_len = file_part - stream_start;
    if (name_len == 0 || name_len >= MAX_STREAM_NAME) {
        log_error("Empty stream name in URI: %s", uri);
        http_response_set_json_error(res, 400, "Invalid HLS path - empty stream name");
        return;
    }

    char encoded_stream_name[MAX_STREAM_NAME];
    char stream_name[MAX_STREAM_NAME];
    char stream_path[MAX_STREAM_NAME];

    safe_strcpy(encoded_stream_name, stream_start, MAX_STREAM_NAME, name_len);

    // URL decode the stream name
    url_decode(encoded_stream_name, stream_name, MAX_STREAM_NAME);

    // Make sure we're using a valid path.
    sanitize_stream_name(stream_name, stream_path, MAX_STREAM_NAME);
    if (!httpd_authorize_media_stream_action(req, res, AUTHZ_LIVE_VIEW,
                                             stream_name)) return;

    // Extract file name (everything after the stream name)
    const char *file_name = file_part + 1; // Skip "/"
    if (*file_name == '\0' || strchr(file_name, '/') || strchr(file_name, '%') || strstr(file_name, "..")) {
        log_error("Empty file name in URI: %s", uri);
        http_response_set_json_error(res, 400, "Invalid HLS path - empty file name");
        return;
    }

    // Get the config to find the storage path - make a local copy of needed values
    const config_t *global_config = get_streaming_config();
    if (!global_config) {
        log_error("Failed to get streaming configuration");
        http_response_set_json_error(res, 500, "Internal server error");
        return;
    }

    // Make local copies of the storage paths to avoid race conditions
    char storage_path[MAX_PATH_LENGTH];
    char storage_path_hls[MAX_PATH_LENGTH];

    // Copy the storage paths with bounds checking
    safe_strcpy(storage_path, global_config->storage_path, MAX_PATH_LENGTH, 0);

    safe_strcpy(storage_path_hls, global_config->storage_path_hls, MAX_PATH_LENGTH, 0);

    // Construct the full path to the HLS file
    char hls_file_path[MAX_PATH_LENGTH * 2]; // Double the buffer size to avoid truncation

    // Use storage_path_hls if specified, otherwise fall back to storage_path
    if (storage_path_hls[0] != '\0') {
        snprintf(hls_file_path, sizeof(hls_file_path), "%s/hls/%s/%s",
                storage_path_hls, stream_path, file_name);
        log_debug("Using HLS-specific storage path: %s", storage_path_hls);
    } else {
        snprintf(hls_file_path, sizeof(hls_file_path), "%s/hls/%s/%s",
                storage_path, stream_path, file_name);
        log_debug("Using default storage path for HLS: %s", storage_path);
    }

    log_debug("Serving HLS file directly: %s", hls_file_path);

    // Check if file exists
    struct stat st;
    if (stat(hls_file_path, &st) == 0 && S_ISREG(st.st_mode)) {
        http_response_t audio_response;
        http_response_init(&audio_response);
        bool listen = !g_config.audio_disabled &&
                      httpd_authorize_media_stream_action(req, &audio_response,
                                                          AUTHZ_AUDIO_LISTEN, stream_name);
        http_response_free(&audio_response);
        if (!listen && !strstr(file_name, ".m3u8")) {
            const char *extension = strrchr(file_name, '.');
            if (!extension || strcmp(extension, ".ts") || st.st_size <= 0 ||
                st.st_size > VIDEO_SEGMENT_LIMIT || !serve_video_segment(hls_file_path, res))
                http_response_set_json_error(res, 403, "Video-only HLS segment unavailable");
            return;
        }
        // Determine content type based on file extension
        const char *content_type = "application/octet-stream";
        if (strstr(file_name, ".m3u8")) {
            content_type = "application/vnd.apple.mpegurl";
        } else if (strstr(file_name, ".ts")) {
            content_type = "video/mp2t";
        } else if (strstr(file_name, ".m4s")) {
            content_type = "video/iso.segment";
        } else if (strstr(file_name, "init.mp4")) {
            content_type = "video/mp4";
        }

        // Build extra headers with cache control and CORS
        // Note: Do NOT include "Connection: close" - it kills keep-alive and forces
        // new TCP handshakes for every HLS segment, severely degrading performance
        char extra_headers[512];

        // Use different cache policies for playlists vs segments:
        // - .m3u8 playlists change every few seconds (new segments added, old removed) → must not cache
        // - segments remain private because permissions can be revoked
        const char *cache_control;
        if (strstr(file_name, ".m3u8")) {
            cache_control = "Cache-Control: no-cache, no-store, must-revalidate\r\n";
        } else {
            // Never reuse media across principals or after a permission change.
            cache_control = "Cache-Control: private, no-store\r\n";
        }

        snprintf(extra_headers, sizeof(extra_headers),
            "%s"
            "Access-Control-Allow-Origin: *\r\n"
            "Access-Control-Allow-Methods: GET, OPTIONS\r\n"
            "Access-Control-Allow-Headers: Origin, Content-Type, Accept, Authorization\r\n",
            cache_control);

        // Serve the file using backend-agnostic function
        http_serve_file(req, res, hls_file_path, content_type, extra_headers);
    } else {
        // File doesn't exist yet - FFmpeg is still starting up
        log_debug("HLS file not found: %s (waiting for FFmpeg to create it)", hls_file_path);

        if (strstr(file_name, ".m3u8")) {
            // For .m3u8 requests: return a valid empty HLS playlist instead of a JSON 404.
            // This prevents HLS.js from getting a fatal manifestParsingError (it tries to
            // parse the JSON as m3u8 and fails). With a valid empty playlist, HLS.js will
            // simply retry the manifest load after its configured retry delay (non-fatal).
            res->status_code = 200;
            safe_strcpy(res->content_type, "application/vnd.apple.mpegurl",
                    sizeof(res->content_type), 0);
            http_response_add_header(res, "Cache-Control", "no-cache, no-store, must-revalidate");
            http_response_add_header(res, "Access-Control-Allow-Origin", "*");
            http_response_set_body(res,
                "#EXTM3U\n"
                "#EXT-X-VERSION:3\n"
                "#EXT-X-TARGETDURATION:2\n"
                "#EXT-X-MEDIA-SEQUENCE:0\n");
        } else {
            // For segment requests (.ts, .m4s): return 404 - HLS.js will retry fragment loading
            http_response_set_json_error(res, 404, "HLS segment not found or still being generated");
        }
    }
}
