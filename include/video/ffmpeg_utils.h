#ifndef FFMPEG_UTILS_H
#define FFMPEG_UTILS_H

#include <libavformat/avformat.h>
#include <libavcodec/avcodec.h>
#include <libavutil/avutil.h>

// FFmpeg error logging
void log_ffmpeg_error(int err, const char *message);

// Initialize FFmpeg libraries
void init_ffmpeg(void);

// Cleanup FFmpeg resources
void cleanup_ffmpeg(void);

#endif // FFMPEG_UTILS_H
