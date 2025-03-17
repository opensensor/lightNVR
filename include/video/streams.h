#ifndef STREAMS_H
#define STREAMS_H

#include <pthread.h>
#include "web/web_server.h"
#include "core/config.h"
#include "video/hls_writer.h"
#include "video/mp4_writer.h"
#include "video/detection.h"
#include "video/detection_result.h"
#include "video/motion_detection.h"
#include "video/hls_streaming.h"
#include "video/mp4_recording.h"
#include "video/stream_transcoding.h"


// Structure for stream transcoding context (legacy, kept for compatibility)
typedef struct {
    stream_config_t config;
    int running;
    pthread_t thread;
    char output_path[MAX_PATH_LENGTH];
    char mp4_output_path[MAX_PATH_LENGTH];
    hls_writer_t *hls_writer;
    mp4_writer_t *mp4_writer;
    
    // Detection-related fields
    detection_model_t detection_model;
    int frame_count;
    bool recording_active;
    time_t last_detection_time;
} stream_transcode_ctx_t;

// Structure to keep track of active recordings
typedef struct {
    uint64_t recording_id;
    char stream_name[MAX_STREAM_NAME];
    char output_path[MAX_PATH_LENGTH];
    time_t start_time;
} active_recording_t;


// Core streaming functions
config_t* get_streaming_config(void);
void init_streaming_backend(void);
void cleanup_streaming_backend(void);
void register_streaming_api_handlers(void);
void create_stream_error_response(http_response_t *response, int status_code, const char *message);
void serve_video_file(http_response_t *response, const char *file_path, const char *content_type,
                     const char *filename, const http_request_t *request);
int stop_transcode_stream(const char *stream_name);

// HLS streaming functions
int start_hls_stream(const char *stream_name);
int stop_hls_stream(const char *stream_name);
void handle_hls_manifest(const http_request_t *request, http_response_t *response);
void handle_hls_segment(const http_request_t *request, http_response_t *response);
void cleanup_hls_directories(void);

// WebRTC functions (placeholders)
void handle_webrtc_offer(const http_request_t *request, http_response_t *response);
void handle_webrtc_ice(const http_request_t *request, http_response_t *response);

// Recording functions
void init_recordings_system(void);
uint64_t start_recording(const char *stream_name, const char *output_path);
void update_recording(const char *stream_name);
void stop_recording(const char *stream_name);
void close_all_mp4_writers(void);
int start_mp4_recording(const char *stream_name);
int register_mp4_writer_for_stream(const char *stream_name, mp4_writer_t *writer);
mp4_writer_t *get_mp4_writer_for_stream(const char *stream_name);
void unregister_mp4_writer_for_stream(const char *stream_name);
int find_mp4_recording(const char *stream_name, time_t timestamp, char *mp4_path, size_t path_size);
int get_recording_state(const char *stream_name);

// Detection-based recording functions
int start_detection_recording(const char *stream_name, const char *model_path, float threshold,
                             int pre_buffer, int post_buffer);
int stop_detection_recording(const char *stream_name);
int process_frame_for_recording(const char *stream_name, const unsigned char *frame_data,
                               int width, int height, int channels, time_t frame_time,
                               detection_result_t *result);
int get_detection_recording_state(const char *stream_name, bool *recording_active);
void init_detection_recording_system(void);
void shutdown_detection_recording_system(void);

// Motion detection functions
int enable_motion_detection(const char *stream_name, float sensitivity, float min_motion_area, int cooldown_time);
int disable_motion_detection(const char *stream_name);

#endif /* STREAMS_H */
