/**
 * Header file for MP4 writer
 */

#ifndef MP4_WRITER_H
#define MP4_WRITER_H

#include <stddef.h>
#include <libavformat/avformat.h>
#include <pthread.h>
#include "core/config.h"  // For MAX_PATH_LENGTH and MAX_STREAM_NAME
#include "video/mp4_writer_thread.h"

/**
 * MP4 writer structure
 */

// Audio stream state structure - completely separate from video
typedef struct {
    int stream_idx;           // Index of the audio stream in the output context
    int64_t first_dts;        // First audio DTS for timestamp reference
    int64_t last_pts;         // Last audio PTS written
    int64_t last_dts;         // Last audio DTS written
    int initialized;          // Flag to track if audio has been initialized
    AVRational time_base;     // Audio stream timebase
    int frame_size;           // Audio frame size in samples (needed for MP4 muxing)
    pthread_mutex_t mutex;    // Mutex to protect audio state
} mp4_audio_state_t;

struct mp4_writer {
    char output_path[MAX_PATH_LENGTH];
    char stream_name[MAX_STREAM_NAME];
    AVFormatContext *output_ctx;
    int video_stream_idx;
    int has_audio;            // Flag indicating if audio is enabled
    int64_t first_dts;        // First video DTS
    int64_t first_pts;        // First video PTS
    int64_t last_dts;         // Last video DTS
    AVRational time_base;     // Video stream timebase
    int is_initialized;
    time_t creation_time;
    time_t last_packet_time;  // Time when the last packet was written
    mp4_audio_state_t audio;  // Audio state - completely separate from video
    pthread_mutex_t mutex;    // Mutex to protect video state
    uint64_t current_recording_id; // ID of the current recording in the database

    // Segment-related fields
    int segment_duration;     // Duration of each segment in seconds
    time_t last_rotation_time;// Time of the last rotation
    int waiting_for_keyframe; // Flag indicating if we're waiting for a keyframe to rotate
    int is_rotating;          // Flag indicating if rotation is in progress
    char output_dir[MAX_PATH_LENGTH]; // Directory where MP4 files are stored

    // Recording trigger type
    char trigger_type[16];    // 'scheduled', 'detection', 'motion', 'manual'

    // RTSP thread context
    mp4_writer_thread_t *thread_ctx;  // Changed from void* to proper type

    // Shutdown coordination
    int shutdown_component_id; // ID assigned by the shutdown coordinator
};

/**
 * Create a new MP4 writer
 *
 * @param output_path Full path to the output MP4 file
 * @param stream_name Name of the stream (used for metadata)
 * @return A new MP4 writer instance or NULL on error
 */
mp4_writer_t *mp4_writer_create(const char *output_path, const char *stream_name);

/**
 * Write a packet to the MP4 file
 * This function handles both video and audio packets
 *
 * @param writer The MP4 writer instance
 * @param pkt The packet to write
 * @param input_stream The original input stream (for codec parameters)
 * @return 0 on success, negative on error
 */
int mp4_writer_write_packet(mp4_writer_t *writer, const AVPacket *pkt, const AVStream *input_stream);

/**
 * Safely add audio stream to MP4 writer
 *
 * @param writer The MP4 writer instance
 * @param codec_params Codec parameters for the audio stream
 * @param time_base Time base for the audio stream
 * @return 0 on success, negative on error
 */
int mp4_writer_add_audio_stream(mp4_writer_t *writer, const AVCodecParameters *codec_params,
                                const AVRational *time_base);

/**
 * Close the MP4 writer and release resources
 *
 * @param writer The MP4 writer instance
 */
void mp4_writer_close(mp4_writer_t *writer);

/**
 * Enable or disable audio recording
 *
 * @param writer The MP4 writer instance
 * @param enable 1 to enable audio, 0 to disable
 */
void mp4_writer_set_audio(mp4_writer_t *writer, int enable);

/**
 * Set the segment duration for MP4 rotation
 *
 * @param writer The MP4 writer instance
 * @param segment_duration Duration of each segment in seconds
 */
void mp4_writer_set_segment_duration(mp4_writer_t *writer, int segment_duration);

// Rotation is now handled entirely by the writer thread in mp4_writer_rtsp.c

/**
 * Start a recording thread that reads from the RTSP stream and writes to the MP4 file
 * This function creates a new thread that handles all the recording logic
 *
 * @param writer The MP4 writer instance
 * @param rtsp_url The URL of the RTSP stream to record
 * @return 0 on success, negative on error
 */
int mp4_writer_start_recording_thread(mp4_writer_t *writer, const char *rtsp_url);

/**
 * Stop the recording thread
 * This function signals the recording thread to stop and waits for it to exit
 *
 * @param writer The MP4 writer instance
 */
void mp4_writer_stop_recording_thread(mp4_writer_t *writer);

/**
 * Check if the recording thread is running
 *
 * @param writer The MP4 writer instance
 * @return 1 if running, 0 if not
 */
int mp4_writer_is_recording(mp4_writer_t *writer);

#endif /* MP4_WRITER_H */
