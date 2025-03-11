#ifndef STREAMS_H
#define STREAMS_H

#include <pthread.h>
#include "web/web_server.h"
#include "core/config.h"
#include "video/hls_writer.h"
#include "video/mp4_writer.h"


// Structure for stream transcoding context
typedef struct {
    stream_config_t config;
    int running;
    pthread_t thread;
    char output_path[MAX_PATH_LENGTH];
    char mp4_output_path[MAX_PATH_LENGTH];
    hls_writer_t *hls_writer;
    mp4_writer_t *mp4_writer;
} stream_transcode_ctx_t;

// Structure to keep track of active recordings
typedef struct {
    uint64_t recording_id;
    char stream_name[MAX_STREAM_NAME];
    char output_path[MAX_PATH_LENGTH];
    time_t start_time;
} active_recording_t;


config_t* get_streaming_config(void);

// Initialize FFmpeg streaming backend
void init_streaming_backend(void);

// Clean up FFmpeg resources
void cleanup_streaming_backend(void);

// Register streaming API handlers
void register_streaming_api_handlers(void);

// Start HLS transcoding for a stream
int start_hls_stream(const char *stream_name);

// Stop HLS transcoding for a stream
int stop_hls_stream(const char *stream_name);

// Handle request for HLS manifest
void handle_hls_manifest(const http_request_t *request, http_response_t *response);

// Handle request for HLS segment
void handle_hls_segment(const http_request_t *request, http_response_t *response);

// Handle WebRTC offer request
void handle_webrtc_offer(const http_request_t *request, http_response_t *response);

// Handle WebRTC ICE candidate request
void handle_webrtc_ice(const http_request_t *request, http_response_t *response);

// create_stream_error_response
void create_stream_error_response(http_response_t *response, int status_code, const char *message);

// Function to initialize the recordings system
void init_recordings_system(void);

// Start a recording for a stream
uint64_t start_recording(const char *stream_name, const char *output_path);

// Update a recording's metadata
void update_recording(const char *stream_name);

// Stop a recording
void stop_recording(const char *stream_name);

// Clean up HLS directories during shutdown
void cleanup_hls_directories(void);

// Close all MP4 writers during shutdown
void close_all_mp4_writers(void);

#endif /* STREAMS_H */
