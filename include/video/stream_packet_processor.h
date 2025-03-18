#ifndef LIGHTNVR_STREAM_PACKET_PROCESSOR_H
#define LIGHTNVR_STREAM_PACKET_PROCESSOR_H

#include <libavformat/avformat.h>
#include <libavcodec/avcodec.h>
#include "video/stream_manager.h"
#include "video/stream_state.h"

/**
 * Process a video packet using the new state management system
 * This function replaces the old process_video_packet function
 * 
 * @param state Stream state manager
 * @param pkt Packet to process
 * @param input_stream Input stream information
 * @param writer_type Type of writer (0 for HLS, 1 for MP4)
 * @param writer Writer context
 * @return 0 on success, non-zero on failure
 */
int process_packet_with_state(stream_state_manager_t *state, const AVPacket *pkt, 
                             const AVStream *input_stream, int writer_type, void *writer);

/**
 * Initialize the packet processor
 * 
 * @return 0 on success, non-zero on failure
 */
int init_packet_processor(void);

/**
 * Shutdown the packet processor
 */
void shutdown_packet_processor(void);

/**
 * Adapter function for process_video_packet
 * This function bridges between the old API and the new state management system
 * 
 * @param pkt Packet to process
 * @param input_stream Input stream information
 * @param writer Writer context
 * @param writer_type Type of writer (0 for HLS, 1 for MP4)
 * @param stream_name Stream name
 * @return 0 on success, non-zero on failure
 */
int process_video_packet_adapter(const AVPacket *pkt, const AVStream *input_stream, 
                                void *writer, int writer_type, const char *stream_name);

#endif // LIGHTNVR_STREAM_PACKET_PROCESSOR_H
