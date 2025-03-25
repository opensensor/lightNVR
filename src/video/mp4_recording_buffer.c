#define _POSIX_C_SOURCE 200809L
#define _XOPEN_SOURCE 700
#define _GNU_SOURCE

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <pthread.h>

#include <libavformat/avformat.h>
#include <libavcodec/avcodec.h>
#include <libavutil/avutil.h>

#include "core/logger.h"
#include "video/mp4_recording.h"
#include "video/mp4_recording_internal.h"
#include "video/mp4_writer.h"

// Global array to store frame buffers
frame_buffer_t frame_buffers[MAX_STREAMS] = {0};

/**
 * Initialize a frame buffer for pre-buffering
 */
int init_frame_buffer(const char *stream_name, int capacity) {
    // Validate input
    if (!stream_name || capacity <= 0) {
        log_error("Invalid parameters for init_frame_buffer");
        return -1;
    }

    // Find empty slot or existing entry for this stream
    int slot = -1;
    for (int i = 0; i < MAX_STREAMS; i++) {
        if (!frame_buffers[i].frames) {
            slot = i;
            break;
        } else if (frame_buffers[i].frames && 
                  mp4_writer_stream_names[i][0] != '\0' && 
                  strcmp(mp4_writer_stream_names[i], stream_name) == 0) {
            // Stream already has a buffer, return its index
            return i;
        }
    }

    if (slot == -1) {
        log_error("No available slots for frame buffer");
        return -1;
    }

    // Initialize the buffer
    frame_buffers[slot].frames = calloc(capacity, sizeof(buffered_packet_t));
    if (!frame_buffers[slot].frames) {
        log_error("Failed to allocate memory for frame buffer");
        return -1;
    }

    frame_buffers[slot].capacity = capacity;
    frame_buffers[slot].count = 0;
    frame_buffers[slot].head = 0;
    frame_buffers[slot].tail = 0;
    pthread_mutex_init(&frame_buffers[slot].mutex, NULL);

    log_info("Initialized frame buffer for stream %s with capacity %d", stream_name, capacity);
    return slot;
}

/**
 * Add a packet to the frame buffer
 */
void add_to_frame_buffer(int buffer_idx, const AVPacket *pkt, const AVStream *stream) {
    // Validate parameters
    if (buffer_idx < 0 || buffer_idx >= MAX_STREAMS || !pkt || !stream) {
        return;
    }
    
    pthread_mutex_lock(&frame_buffers[buffer_idx].mutex);
    
    // Check if buffer still exists after acquiring the lock
    if (!frame_buffers[buffer_idx].frames) {
        pthread_mutex_unlock(&frame_buffers[buffer_idx].mutex);
        return;
    }

    // If buffer is full, remove oldest packet
    if (frame_buffers[buffer_idx].count == frame_buffers[buffer_idx].capacity) {
        if (frame_buffers[buffer_idx].frames[frame_buffers[buffer_idx].head].packet) {
            av_packet_free(&frame_buffers[buffer_idx].frames[frame_buffers[buffer_idx].head].packet);
            frame_buffers[buffer_idx].frames[frame_buffers[buffer_idx].head].packet = NULL;
        }
        frame_buffers[buffer_idx].head = (frame_buffers[buffer_idx].head + 1) % frame_buffers[buffer_idx].capacity;
        frame_buffers[buffer_idx].count--;
    }

    // Add new packet
    int idx = frame_buffers[buffer_idx].tail;
    
    // Allocate new packet
    AVPacket *new_packet = av_packet_alloc();
    if (!new_packet) {
        log_error("Failed to allocate packet for frame buffer");
        pthread_mutex_unlock(&frame_buffers[buffer_idx].mutex);
        return;
    }
    
    // Reference the input packet
    int ret = av_packet_ref(new_packet, pkt);
    if (ret < 0) {
        log_error("Failed to reference packet for frame buffer: error %d", ret);
        av_packet_free(&new_packet);
        pthread_mutex_unlock(&frame_buffers[buffer_idx].mutex);
        return;
    }
    
    // Store the packet in the buffer
    frame_buffers[buffer_idx].frames[idx].packet = new_packet;
    frame_buffers[buffer_idx].frames[idx].time_base = stream->time_base;
    frame_buffers[buffer_idx].tail = (frame_buffers[buffer_idx].tail + 1) % frame_buffers[buffer_idx].capacity;
    frame_buffers[buffer_idx].count++;

    pthread_mutex_unlock(&frame_buffers[buffer_idx].mutex);
}

/**
 * Flush the frame buffer to the MP4 writer
 */
void flush_frame_buffer(int buffer_idx, mp4_writer_t *writer) {
    // Validate parameters
    if (buffer_idx < 0 || buffer_idx >= MAX_STREAMS || !writer) {
        log_warn("Invalid parameters for flush_frame_buffer: buffer_idx=%d, writer=%p", 
                buffer_idx, (void*)writer);
        return;
    }

    pthread_mutex_lock(&frame_buffers[buffer_idx].mutex);
    
    // Check if buffer exists after acquiring the lock
    if (!frame_buffers[buffer_idx].frames) {
        log_warn("Frame buffer at index %d is NULL", buffer_idx);
        pthread_mutex_unlock(&frame_buffers[buffer_idx].mutex);
        return;
    }

    int count = frame_buffers[buffer_idx].count;
    log_info("Flushing %d frames from buffer to MP4 writer", count);

    if (count == 0) {
        // Nothing to flush
        pthread_mutex_unlock(&frame_buffers[buffer_idx].mutex);
        return;
    }

    // Write all buffered packets to the MP4 writer
    int head = frame_buffers[buffer_idx].head;
    int success_count = 0;
    int error_count = 0;

    for (int i = 0; i < count; i++) {
        int idx = (head + i) % frame_buffers[buffer_idx].capacity;
        if (frame_buffers[buffer_idx].frames[idx].packet) {
            // Create a dummy stream with the correct time_base
            AVStream dummy_stream;
            memset(&dummy_stream, 0, sizeof(AVStream));
            dummy_stream.time_base = frame_buffers[buffer_idx].frames[idx].time_base;

            // Write the packet
            int ret = mp4_writer_write_packet(writer, frame_buffers[buffer_idx].frames[idx].packet, &dummy_stream);
            
            if (ret >= 0) {
                success_count++;
            } else {
                error_count++;
                // Log error but continue with other packets
                log_warn("Failed to write packet %d/%d to MP4 writer: error %d", 
                        i+1, count, ret);
            }

            // Free the packet
            av_packet_free(&frame_buffers[buffer_idx].frames[idx].packet);
            frame_buffers[buffer_idx].frames[idx].packet = NULL;
        }
    }

    // Reset the buffer
    frame_buffers[buffer_idx].count = 0;
    frame_buffers[buffer_idx].head = 0;
    frame_buffers[buffer_idx].tail = 0;

    pthread_mutex_unlock(&frame_buffers[buffer_idx].mutex);
    
    log_info("Flushed %d frames to MP4 writer (%d successful, %d errors)", 
            count, success_count, error_count);
}

/**
 * Free a frame buffer
 */
void free_frame_buffer(int buffer_idx) {
    if (buffer_idx < 0 || buffer_idx >= MAX_STREAMS) {
        return;
    }

    // Use a local variable to store the frames pointer
    buffered_packet_t *frames_to_free = NULL;
    
    pthread_mutex_lock(&frame_buffers[buffer_idx].mutex);
    
    // Check again with the lock held
    if (!frame_buffers[buffer_idx].frames) {
        pthread_mutex_unlock(&frame_buffers[buffer_idx].mutex);
        return;
    }

    // Free all packets
    for (int i = 0; i < frame_buffers[buffer_idx].capacity; i++) {
        if (frame_buffers[buffer_idx].frames[i].packet) {
            av_packet_free(&frame_buffers[buffer_idx].frames[i].packet);
            frame_buffers[buffer_idx].frames[i].packet = NULL;
        }
    }

    // Store the pointer to free after releasing the lock
    frames_to_free = frame_buffers[buffer_idx].frames;
    
    // Clear the buffer state
    frame_buffers[buffer_idx].frames = NULL;
    frame_buffers[buffer_idx].capacity = 0;
    frame_buffers[buffer_idx].count = 0;
    frame_buffers[buffer_idx].head = 0;
    frame_buffers[buffer_idx].tail = 0;

    pthread_mutex_unlock(&frame_buffers[buffer_idx].mutex);
    
    // Now free the memory outside the lock
    if (frames_to_free) {
        free(frames_to_free);
    }
    
    // Destroy the mutex
    pthread_mutex_destroy(&frame_buffers[buffer_idx].mutex);
    
    log_info("Freed frame buffer at index %d", buffer_idx);
}

/**
 * Add a packet to the pre-buffer for a stream
 * This is called from the HLS streaming thread for every packet
 */
void add_packet_to_prebuffer(const char *stream_name, const AVPacket *pkt, const AVStream *stream) {
    // Validate parameters
    if (!stream_name || !pkt || !stream) {
        return;
    }
    
    // Make a local copy of the stream name for thread safety
    char local_stream_name[MAX_STREAM_NAME];
    strncpy(local_stream_name, stream_name, MAX_STREAM_NAME - 1);
    local_stream_name[MAX_STREAM_NAME - 1] = '\0';

    // Find the buffer for this stream
    int buffer_idx = -1;
    for (int i = 0; i < MAX_STREAMS; i++) {
        if (frame_buffers[i].frames && 
            mp4_writer_stream_names[i][0] != '\0' && 
            strcmp(mp4_writer_stream_names[i], local_stream_name) == 0) {
            buffer_idx = i;
            break;
        }
    }

    if (buffer_idx >= 0) {
        // Only add to buffer if we found a valid buffer index
        add_to_frame_buffer(buffer_idx, pkt, stream);
    }
}

/**
 * Flush the pre-buffered frames to the MP4 writer
 * This is called when a detection event occurs
 */
void flush_prebuffer_to_mp4(const char *stream_name) {
    // Validate parameters
    if (!stream_name || stream_name[0] == '\0') {
        log_warn("Invalid stream name passed to flush_prebuffer_to_mp4");
        return;
    }

    log_info("Attempting to flush pre-buffer for stream %s", stream_name);
    
    // Make a local copy of the stream name for thread safety
    char local_stream_name[MAX_STREAM_NAME];
    strncpy(local_stream_name, stream_name, MAX_STREAM_NAME - 1);
    local_stream_name[MAX_STREAM_NAME - 1] = '\0';

    // Find the buffer for this stream
    int buffer_idx = -1;
    for (int i = 0; i < MAX_STREAMS; i++) {
        if (frame_buffers[i].frames && 
            mp4_writer_stream_names[i][0] != '\0' && 
            strcmp(mp4_writer_stream_names[i], local_stream_name) == 0) {
            buffer_idx = i;
            break;
        }
    }
    
    // Get the MP4 writer for this stream while still holding the lock
    mp4_writer_t *writer = NULL;
    for (int i = 0; i < MAX_STREAMS; i++) {
        if (mp4_writers[i] && 
            mp4_writer_stream_names[i][0] != '\0' && 
            strcmp(mp4_writer_stream_names[i], local_stream_name) == 0) {
            writer = mp4_writers[i];
            break;
        }
    }

    if (buffer_idx < 0) {
        log_info("No pre-buffer found for stream %s", local_stream_name);
        return;
    }

    if (!writer) {
        log_error("No MP4 writer found for stream %s", local_stream_name);
        return;
    }

    // Flush the buffer to the MP4 writer
    log_info("Flushing pre-buffer to MP4 writer for stream %s", local_stream_name);
    flush_frame_buffer(buffer_idx, writer);
}
