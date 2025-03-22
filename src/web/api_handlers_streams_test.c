#define _XOPEN_SOURCE
#define _GNU_SOURCE

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <ctype.h>

#include "web/api_handlers.h"
#include "web/mongoose_adapter.h"
#include "core/logger.h"
#include "core/config.h"
#include "video/stream_manager.h"
#include "video/streams.h"
#include "video/stream_state.h"
#include "mongoose.h"
#include "video/detection_stream.h"
#include "database/database_manager.h"
#include "video/ffmpeg_utils.h"

// FFmpeg includes
#include <libavformat/avformat.h>
#include <libavcodec/avcodec.h>
#include <libavutil/avutil.h>
#include <libavutil/opt.h>

/**
 * @brief Test a stream connection using FFmpeg
 * 
 * @param url Stream URL
 * @param protocol Stream protocol (TCP/UDP)
 * @param width Pointer to store width
 * @param height Pointer to store height
 * @param fps Pointer to store fps
 * @param codec_name Buffer to store codec name
 * @param codec_name_size Size of codec name buffer
 * @param error_msg Buffer to store error message
 * @param error_msg_size Size of error message buffer
 * @return int 0 on success, non-zero on error
 */
static int test_stream_connection(const char *url, int protocol, 
                                 int *width, int *height, double *fps, 
                                 char *codec_name, size_t codec_name_size,
                                 char *error_msg, size_t error_msg_size) {
    AVFormatContext *format_ctx = NULL;
    AVDictionary *options = NULL;
    int video_stream_index = -1;
    int ret = -1;
    
    // Set protocol (TCP/UDP)
    if (protocol == STREAM_PROTOCOL_TCP) {
        av_dict_set(&options, "rtsp_transport", "tcp", 0);
    } else {
        av_dict_set(&options, "rtsp_transport", "udp", 0);
    }
    
    // Set short timeout for connection test
    av_dict_set(&options, "timeout", "5000000", 0); // 5 seconds in microseconds
    
    // Open input
    ret = avformat_open_input(&format_ctx, url, NULL, &options);
    if (ret < 0) {
        av_strerror(ret, error_msg, error_msg_size);
        log_error("Could not open stream: %s", error_msg);
        goto cleanup;
    }
    
    // Find stream info
    ret = avformat_find_stream_info(format_ctx, NULL);
    if (ret < 0) {
        av_strerror(ret, error_msg, error_msg_size);
        log_error("Could not find stream info: %s", error_msg);
        goto cleanup;
    }
    
    // Find video stream
    for (unsigned int i = 0; i < format_ctx->nb_streams; i++) {
        if (format_ctx->streams[i]->codecpar->codec_type == AVMEDIA_TYPE_VIDEO) {
            video_stream_index = i;
            break;
        }
    }
    
    if (video_stream_index == -1) {
        snprintf(error_msg, error_msg_size, "No video stream found");
        log_error("%s", error_msg);
        ret = -1;
        goto cleanup;
    }
    
    // Get video stream info
    AVStream *video_stream = format_ctx->streams[video_stream_index];
    AVCodecParameters *codec_params = video_stream->codecpar;
    
    // Get codec name
    const AVCodec *codec = avcodec_find_decoder(codec_params->codec_id);
    if (codec) {
        strncpy(codec_name, codec->name, codec_name_size - 1);
        codec_name[codec_name_size - 1] = '\0';
    } else {
        strncpy(codec_name, "unknown", codec_name_size - 1);
        codec_name[codec_name_size - 1] = '\0';
    }
    
    // Get dimensions
    *width = codec_params->width;
    *height = codec_params->height;
    
    // Get framerate
    if (video_stream->avg_frame_rate.den && video_stream->avg_frame_rate.num) {
        *fps = (double)video_stream->avg_frame_rate.num / (double)video_stream->avg_frame_rate.den;
    } else {
        *fps = 0.0;
    }
    
    // Success
    ret = 0;
    
cleanup:
    // Clean up
    if (options) {
        av_dict_free(&options);
    }
    
    if (format_ctx) {
        avformat_close_input(&format_ctx);
    }
    
    return ret;
}

/**
 * @brief Direct handler for POST /api/streams/test
 */
void mg_handle_test_stream(struct mg_connection *c, struct mg_http_message *hm) {
    log_info("Handling POST /api/streams/test request");
    
    // Parse JSON from request body
    cJSON *test_json = mg_parse_json_body(hm);
    if (!test_json) {
        log_error("Failed to parse test JSON from request body");
        mg_send_json_error(c, 400, "Invalid JSON in request body");
        return;
    }
    
    // Extract URL and protocol
    cJSON *url = cJSON_GetObjectItem(test_json, "url");
    cJSON *protocol = cJSON_GetObjectItem(test_json, "protocol");
    
    if (!url || !cJSON_IsString(url) || !protocol || !cJSON_IsNumber(protocol)) {
        log_error("Missing required fields in test request");
        cJSON_Delete(test_json);
        mg_send_json_error(c, 400, "Missing required fields (url, protocol)");
        return;
    }
    
    // Get values
    const char *stream_url = url->valuestring;
    int stream_protocol = protocol->valueint;
    
    log_info("Testing stream connection: url=%s, protocol=%d", stream_url, stream_protocol);
    
    // Create response
    cJSON *response = cJSON_CreateObject();
    if (!response) {
        log_error("Failed to create response JSON object");
        cJSON_Delete(test_json);
        mg_send_json_error(c, 500, "Failed to create response JSON");
        return;
    }
    
    // Test stream connection
    int width = 0, height = 0;
    double fps = 0.0;
    char codec_name[64] = {0};
    char error_msg[256] = {0};
    
    int result = test_stream_connection(stream_url, stream_protocol, 
                                       &width, &height, &fps, 
                                       codec_name, sizeof(codec_name),
                                       error_msg, sizeof(error_msg));
    
    bool success = (result == 0);
    cJSON_AddBoolToObject(response, "success", success);
    
    if (success) {
        // Add stream info
        cJSON *info = cJSON_CreateObject();
        if (info) {
            cJSON_AddNumberToObject(info, "width", width);
            cJSON_AddNumberToObject(info, "height", height);
            cJSON_AddNumberToObject(info, "fps", (int)fps);
            cJSON_AddStringToObject(info, "codec", codec_name);
            
            cJSON_AddItemToObject(response, "info", info);
        }
    } else {
        cJSON_AddStringToObject(response, "message", error_msg);
    }
    
    // Convert to string
    char *json_str = cJSON_PrintUnformatted(response);
    if (!json_str) {
        log_error("Failed to convert response JSON to string");
        cJSON_Delete(test_json);
        cJSON_Delete(response);
        mg_send_json_error(c, 500, "Failed to convert response JSON to string");
        return;
    }
    
    // Send response
    mg_send_json_response(c, 200, json_str);
    
    // Clean up
    free(json_str);
    cJSON_Delete(test_json);
    cJSON_Delete(response);
    
    log_info("Stream test completed: success=%d", success);
}
