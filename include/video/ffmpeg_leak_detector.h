#ifndef FFMPEG_LEAK_DETECTOR_H
#define FFMPEG_LEAK_DETECTOR_H

#include <libavformat/avformat.h>
#include <libavcodec/avcodec.h>
#include <libavutil/avutil.h>

/**
 * Initialize the FFmpeg leak detector
 * This should be called during application startup
 */
void ffmpeg_leak_detector_init(void);

/**
 * Clean up the FFmpeg leak detector
 * This should be called during application shutdown
 * It will report any potential leaks
 */
void ffmpeg_leak_detector_cleanup(void);

/**
 * Track a new FFmpeg allocation
 * 
 * @param ptr Pointer to the allocated object
 * @param type Type of the object (e.g., "AVFormatContext")
 * @param location Source file location
 * @param line Source line number
 */
void ffmpeg_track_allocation(void *ptr, const char *type, const char *location, int line);

/**
 * Untrack an FFmpeg allocation when it's freed
 * 
 * @param ptr Pointer to the freed object
 */
void ffmpeg_untrack_allocation(void *ptr);

/**
 * Get the current number of tracked allocations
 * 
 * @return Number of tracked allocations
 */
int ffmpeg_get_allocation_count(void);

/**
 * Dump the current allocation list to the log
 * Useful for debugging memory leaks
 */
void ffmpeg_dump_allocations(void);

/**
 * Force cleanup of all tracked allocations
 * This is a last resort to prevent memory leaks
 */
void ffmpeg_force_cleanup_all(void);

// Convenience macros for tracking
#define TRACK_AVFORMAT_CTX(ptr) ffmpeg_track_allocation((void*)(ptr), "AVFormatContext", __FILE__, __LINE__)
#define TRACK_AVPACKET(ptr) ffmpeg_track_allocation((void*)(ptr), "AVPacket", __FILE__, __LINE__)
#define TRACK_AVFRAME(ptr) ffmpeg_track_allocation((void*)(ptr), "AVFrame", __FILE__, __LINE__)
#define TRACK_AVCODEC_CTX(ptr) ffmpeg_track_allocation((void*)(ptr), "AVCodecContext", __FILE__, __LINE__)

// Convenience macros for untracking
#define UNTRACK_AVFORMAT_CTX(ptr) ffmpeg_untrack_allocation((void*)(ptr))
#define UNTRACK_AVPACKET(ptr) ffmpeg_untrack_allocation((void*)(ptr))
#define UNTRACK_AVFRAME(ptr) ffmpeg_untrack_allocation((void*)(ptr))
#define UNTRACK_AVCODEC_CTX(ptr) ffmpeg_untrack_allocation((void*)(ptr))

#endif /* FFMPEG_LEAK_DETECTOR_H */
