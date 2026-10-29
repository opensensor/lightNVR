#ifndef GO2RTC_TRANSCODE_H
#define GO2RTC_TRANSCODE_H

/* Browser compatibility copies only. The camera producer remains first in
 * the source list, so recording and native playback keep the original video.
 * Bound decoder, filter and encoder workers independently: output -threads
 * alone does not bound HEVC decoder frame buffers on many-core hosts. */
#define GO2RTC_FFMPEG_RTSP_INPUT \
    "-threads:v 2 -fflags nobuffer -flags low_delay -timeout {timeout} " \
    "-user_agent go2rtc/ffmpeg -rtsp_flags prefer_tcp -i {input}"

#define GO2RTC_PREVIEW_MAX_WIDTH 1920
#define GO2RTC_PREVIEW_MAX_HEIGHT 1080

/* No B-frames/lookahead, no upscaling, even dimensions, original aspect ratio.
 * Keep the filter quoted as one argument for go2rtc's FFmpeg argument parser. */
#define GO2RTC_FFMPEG_H264 \
    "-c:v libx264 -threads:v 2 -filter_threads 2 -g:v 30 " \
    "-preset:v superfast -tune:v zerolatency -pix_fmt:v yuv420p " \
    "-vf \"scale=w='min(iw,1920)':h='min(ih,1080)':" \
    "force_original_aspect_ratio=decrease:force_divisible_by=2\""

#endif
