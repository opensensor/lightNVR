#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <pthread.h>

#include "video/stream_manager.h"
#include "video/stream_state.h"
#include "video/stream_packet_processor.h"
#include "video/stream_transcoding.h"
#include "video/hls_writer.h"
#include "video/mp4_writer.h"
#include "core/logger.h"

// Mutex for thread-safe access to packet processing
static pthread_mutex_t processor_mutex = PTHREAD_MUTEX_INITIALIZER;
static bool processor_initialized = false;

/**
 * Initialize the packet processor
 */
int init_packet_processor(void) {
    if (processor_initialized) {
        return 0;  // Already initialized
    }
    
    pthread_mutex_lock(&processor_mutex);
    processor_initialized = true;
    pthread_mutex_unlock(&processor_mutex);
    
    log_info("Packet processor initialized");
    return 0;
}

/**
 * Shutdown the packet processor
 */
void shutdown_packet_processor(void) {
    if (!processor_initialized) {
        return;
    }
    
    pthread_mutex_lock(&processor_mutex);
    processor_initialized = false;
    pthread_mutex_unlock(&processor_mutex);
    
    log_info("Packet processor shutdown");
}

/**
 * Process a video packet using the new state management system
 */
int process_packet_with_state(stream_state_manager_t *state, const AVPacket *pkt, 
                             const AVStream *input_stream, int writer_type, void *writer) {
    if (!state || !pkt || !input_stream || !writer || !processor_initialized) {
        log_error("Invalid parameters for process_packet_with_state");
        return -1;
    }
    
    int ret = 0;
    AVPacket *out_pkt = NULL;
    
    // Validate packet data
    if (!pkt->data || pkt->size <= 0) {
        log_warn("Invalid packet (null data or zero size) for stream %s", state->name);
        return -1;
    }
    
    // Create a clean copy of the packet to avoid reference issues
    out_pkt = av_packet_alloc();
    if (!out_pkt) {
        log_error("Failed to allocate packet in process_packet_with_state for stream %s", state->name);
        return -1;
    }
    
    if (av_packet_ref(out_pkt, pkt) < 0) {
        log_error("Failed to reference packet in process_packet_with_state for stream %s", state->name);
        av_packet_free(&out_pkt);
        return -1;
    }
    
    // Check if this is a key frame
    bool is_key_frame = (out_pkt->flags & AV_PKT_FLAG_KEY) != 0;
    
    // Get protocol information
    pthread_mutex_lock(&state->mutex);
    stream_protocol_t protocol = state->protocol_state.protocol;
    bool is_udp = (protocol == STREAM_PROTOCOL_UDP);
    pthread_mutex_unlock(&state->mutex);
    
    // Handle timestamps based on protocol
    if (is_udp) {
        // Get timestamp state
        pthread_mutex_lock(&state->mutex);
        int64_t last_pts = state->timestamp_state.last_pts;
        int64_t expected_next_pts = state->timestamp_state.expected_next_pts;
        bool timestamps_initialized = state->timestamp_state.timestamps_initialized;
        pthread_mutex_unlock(&state->mutex);
        
        // Handle missing timestamps
        if (out_pkt->pts == AV_NOPTS_VALUE && out_pkt->dts != AV_NOPTS_VALUE) {
            out_pkt->pts = out_pkt->dts;
        } else if (out_pkt->dts == AV_NOPTS_VALUE && out_pkt->pts != AV_NOPTS_VALUE) {
            out_pkt->dts = out_pkt->pts;
        } else if (out_pkt->pts == AV_NOPTS_VALUE && out_pkt->dts == AV_NOPTS_VALUE) {
            // Both timestamps missing, try to generate based on previous packet
            if (timestamps_initialized && last_pts != AV_NOPTS_VALUE) {
                // Calculate frame duration based on stream timebase and framerate
                int64_t frame_duration = 0;
                
                // Add safety checks for input_stream
                if (input_stream && input_stream->avg_frame_rate.num > 0) {
                    AVRational tb = input_stream->time_base;
                    AVRational fr = input_stream->avg_frame_rate;
                    
                    // Avoid division by zero
                    if (fr.den > 0) {
                        frame_duration = av_rescale_q(1, av_inv_q(fr), tb);
                    } else {
                        // Default to a reasonable value if framerate is invalid
                        frame_duration = 3000; // Assume 30fps with timebase 1/90000
                    }
                } else {
                    // Default to 1/30 second if framerate not available
                    frame_duration = 3000; // Assume 30fps with timebase 1/90000
                }
                
                // Sanity check on frame duration
                if (frame_duration <= 0) {
                    frame_duration = 3000; // Reasonable default
                }
                
                out_pkt->pts = last_pts + frame_duration;
                out_pkt->dts = out_pkt->pts;
                log_debug("Generated timestamps for UDP stream %s: pts=%lld, dts=%lld", 
                         state->name, (long long)out_pkt->pts, (long long)out_pkt->dts);
            } else {
                // If we don't have a last_pts, set a default starting value
                out_pkt->pts = 1;
                out_pkt->dts = 1;
                log_debug("Set default initial timestamps for UDP stream %s with no previous reference", 
                         state->name);
            }
        }
        
        // Detect and handle timestamp discontinuities
        if (timestamps_initialized && last_pts != AV_NOPTS_VALUE && out_pkt->pts != AV_NOPTS_VALUE) {
            // Calculate frame duration
            int64_t frame_duration = 0;
            
            // Add safety checks for input_stream
            if (input_stream && input_stream->avg_frame_rate.num > 0 && input_stream->avg_frame_rate.den > 0) {
                // Ensure time_base is valid before using it
                if (input_stream->time_base.num > 0 && input_stream->time_base.den > 0) {
                    AVRational tb = input_stream->time_base;
                    AVRational fr = input_stream->avg_frame_rate;
                    frame_duration = av_rescale_q(1, av_inv_q(fr), tb);
                } else {
                    // Default if time_base is invalid
                    frame_duration = 3000; // Assume 30fps with timebase 1/90000
                }
            } else {
                // Default to 1/30 second if framerate not available
                frame_duration = 3000; // Assume 30fps with timebase 1/90000
            }
        
            // Sanity check on frame duration
            if (frame_duration <= 0) {
                frame_duration = 3000; // Reasonable default
            }
            
            // Check for large discontinuities (more than 10x frame duration)
            int64_t pts_diff = llabs(out_pkt->pts - expected_next_pts);
            if (pts_diff > 10 * frame_duration) {
                // Update discontinuity count
                pthread_mutex_lock(&state->mutex);
                state->timestamp_state.pts_discontinuity_count++;
                int local_count = state->timestamp_state.pts_discontinuity_count;
                pthread_mutex_unlock(&state->mutex);
                
                // Only log occasionally to avoid flooding logs
                if (local_count % 10 == 1) {
                    log_warn("Timestamp discontinuity detected in UDP stream %s: expected=%lld, actual=%lld, diff=%lld ms", 
                            state->name, 
                            (long long)expected_next_pts, 
                            (long long)out_pkt->pts,
                            (long long)(pts_diff * 1000 * 
                                       (input_stream && input_stream->time_base.den > 0 ? 
                                        input_stream->time_base.num : 1) / 
                                       (input_stream && input_stream->time_base.den > 0 ? 
                                        input_stream->time_base.den : 90000)));
                }
                
                // For severe discontinuities, try to correct by using expected PTS
                if (pts_diff > 100 * frame_duration) {
                    int64_t original_pts = out_pkt->pts;
                    out_pkt->pts = expected_next_pts;
                    out_pkt->dts = out_pkt->pts;
                    
                    log_debug("Corrected severe timestamp discontinuity: original=%lld, corrected=%lld", 
                             (long long)original_pts, (long long)out_pkt->pts);
                }
            }
        }
    }
    
    // Update timestamp state
    pthread_mutex_lock(&state->mutex);
    state->timestamp_state.last_pts = out_pkt->pts;
    state->timestamp_state.last_dts = out_pkt->dts;
    
    // Calculate expected next PTS
    int64_t frame_duration = 0;
    
    // Try to get framerate from config
    if (state->config.fps > 0) {
        // Assume timebase of 1/90000 (common for MPEG)
        frame_duration = 90000 / state->config.fps;
    } else if (input_stream && input_stream->avg_frame_rate.num > 0 && input_stream->avg_frame_rate.den > 0) {
        // Get from input stream
        AVRational tb = input_stream->time_base;
        AVRational fr = input_stream->avg_frame_rate;
        frame_duration = av_rescale_q(1, av_inv_q(fr), tb);
    } else {
        // Default to 30fps
        frame_duration = 3000;
    }
    
    state->timestamp_state.expected_next_pts = out_pkt->pts + frame_duration;
    state->timestamp_state.timestamps_initialized = true;
    
    // Update statistics
    state->stats.frames_received++;
    state->stats.bytes_received += out_pkt->size;
    state->stats.last_frame_time = time(NULL);
    pthread_mutex_unlock(&state->mutex);
    
    // Process the packet based on writer type
    if (writer_type == 0) {  // HLS writer
        hls_writer_t *hls_writer = (hls_writer_t *)writer;
        
        // Validate HLS writer
        if (!hls_writer) {
            log_error("NULL HLS writer for stream %s", state->name);
            av_packet_free(&out_pkt);
            return -1;
        }
        
        // Check if the writer has been closed
        if (!hls_writer->output_ctx) {
            log_error("HLS writer has been closed for stream %s", state->name);
            av_packet_free(&out_pkt);
            return -1;
        }
        
        // Ensure timestamps are valid before writing
        if (out_pkt->pts == AV_NOPTS_VALUE && out_pkt->dts != AV_NOPTS_VALUE) {
            out_pkt->pts = out_pkt->dts;
        } else if (out_pkt->dts == AV_NOPTS_VALUE && out_pkt->pts != AV_NOPTS_VALUE) {
            out_pkt->dts = out_pkt->pts;
        } else if (out_pkt->pts == AV_NOPTS_VALUE && out_pkt->dts == AV_NOPTS_VALUE) {
            // If both timestamps are missing, set them to 1 to avoid segfault
            out_pkt->pts = 1;
            out_pkt->dts = 1;
            log_debug("Set default timestamps for packet with no PTS/DTS in stream %s", state->name);
        }
        
        ret = hls_writer_write_packet(hls_writer, out_pkt, input_stream);
        if (ret < 0) {
            // Only log errors for keyframes or every 200th packet to reduce log spam
            static int error_count = 0;
            if (is_key_frame || (++error_count % 200 == 0)) {
                log_error("Failed to write packet to HLS for stream %s: %d (count: %d)", 
                         state->name, ret, error_count);
            }
            
            // Update error statistics
            pthread_mutex_lock(&state->mutex);
            state->stats.errors++;
            pthread_mutex_unlock(&state->mutex);
        }
    } else if (writer_type == 1) {  // MP4 writer
        mp4_writer_t *mp4_writer = (mp4_writer_t *)writer;
        
        // Validate MP4 writer
        if (!mp4_writer) {
            log_error("NULL MP4 writer for stream %s", state->name);
            av_packet_free(&out_pkt);
            return -1;
        }
        
        // Ensure timestamps are valid before writing
        if (out_pkt->pts == AV_NOPTS_VALUE && out_pkt->dts != AV_NOPTS_VALUE) {
            out_pkt->pts = out_pkt->dts;
        } else if (out_pkt->dts == AV_NOPTS_VALUE && out_pkt->pts != AV_NOPTS_VALUE) {
            out_pkt->dts = out_pkt->pts;
        } else if (out_pkt->pts == AV_NOPTS_VALUE && out_pkt->dts == AV_NOPTS_VALUE) {
            // If both timestamps are missing, set them to 1 to avoid segfault
            out_pkt->pts = 1;
            out_pkt->dts = 1;
            log_debug("Set default timestamps for packet with no PTS/DTS in stream %s", state->name);
        }
        
        ret = mp4_writer_write_packet(mp4_writer, out_pkt, input_stream);
        if (ret < 0) {
            // Only log errors for keyframes or every 200th packet to reduce log spam
            static int error_count = 0;
            if (is_key_frame || (++error_count % 200 == 0)) {
                log_error("Failed to write packet to MP4 for stream %s: %d (count: %d)", 
                         state->name, ret, error_count);
            }
            
            // Update error statistics
            pthread_mutex_lock(&state->mutex);
            state->stats.errors++;
            pthread_mutex_unlock(&state->mutex);
        }
    } else {
        log_error("Unknown writer type: %d", writer_type);
        ret = -1;
        
        // Update error statistics
        pthread_mutex_lock(&state->mutex);
        state->stats.errors++;
        pthread_mutex_unlock(&state->mutex);
    }
    
    // Clean up packet
    if (out_pkt) {
        av_packet_free(&out_pkt);
    }
    
    return ret;
}

/**
 * Adapter function for process_video_packet
 */
int process_video_packet_adapter(const AVPacket *pkt, const AVStream *input_stream, 
                                void *writer, int writer_type, const char *stream_name) {
    if (!pkt || !input_stream || !writer || !stream_name || !processor_initialized) {
        log_error("Invalid parameters for process_video_packet_adapter");
        return -1;
    }
    
    // Get the state manager by name
    stream_state_manager_t *state = get_stream_state_by_name(stream_name);
    if (!state) {
        // Fall back to the old implementation if state manager not found
        log_warn("Stream state not found for '%s', using direct packet processing", stream_name);
        
        // Create a temporary state structure to use with process_packet_with_state
        // This avoids using the old process_video_packet function which expects the old structure layout
        stream_state_manager_t temp_state;
        memset(&temp_state, 0, sizeof(stream_state_manager_t));
        strncpy(temp_state.name, stream_name, MAX_STREAM_NAME - 1);
        temp_state.name[MAX_STREAM_NAME - 1] = '\0';
        
        // Initialize mutex
        pthread_mutex_init(&temp_state.mutex, NULL);
        
        // Process the packet with the temporary state
        int ret = process_packet_with_state(&temp_state, pkt, input_stream, writer_type, writer);
        
        // Destroy mutex
        pthread_mutex_destroy(&temp_state.mutex);
        
        return ret;
    }
    
    // Process the packet with the state manager
    return process_packet_with_state(state, pkt, input_stream, writer_type, writer);
}
