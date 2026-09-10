#ifndef LIGHTNVR_RECORDING_TRANSCODE_H
#define LIGHTNVR_RECORDING_TRANSCODE_H

#include <stdbool.h>

/**
 * Probe a recording file's primary video stream and report whether it is
 * HEVC/H.265 — a codec most browsers cannot play natively in an HTML5
 * <video> element, unlike the H.264 recordings the rest of the fleet
 * produces.
 *
 * Fails open: a missing file, unreadable container, or probe error returns
 * false (no transcode) rather than blocking playback of a file we can't
 * inspect.
 *
 * @param file_path Path to the recording's MP4 file
 * @return true if the primary video stream is HEVC, false otherwise
 */
bool recording_needs_hevc_transcode(const char *file_path);

/**
 * Ensure a browser-playable H.264 copy of an HEVC recording exists at
 * cache_path, transcoding from original_path if it doesn't already.
 *
 * If cache_path already exists and is non-empty, returns success
 * immediately without re-transcoding (the cache is treated as valid for
 * the lifetime of the original recording; callers are responsible for
 * removing it when the original is deleted).
 *
 * Otherwise runs ffmpeg synchronously (VAAPI hardware transcode when
 * /dev/dri/renderD128 is available, software libx264 otherwise) and
 * atomically renames the result into place so a concurrent reader never
 * observes a partially-written cache file.
 * Only one recording transcode runs at a time in this process. Cache misses
 * wait for that slot and recheck the cache, so concurrent requests for the
 * same recording reuse the completed result. Existing cache hits do not wait.
 * Software transcoding limits threads and lookahead buffering to reduce its
 * memory footprint alongside recording and live streaming.
 *
 * @param original_path Path to the source HEVC recording
 * @param cache_path Destination path for the transcoded H.264 copy
 * @return 0 on success (cache_path now holds a playable file), -1 on
 *         failure (caller should fall back to serving original_path)
 */
int ensure_recording_transcode_cache(const char *original_path, const char *cache_path);

typedef enum {
    RECORDING_TRANSCODE_FAILED = -1,
    RECORDING_TRANSCODE_READY = 0,
    RECORDING_TRANSCODE_PENDING = 1
} recording_transcode_status_t;

/**
 * Start or poll preparation on a dedicated thread, without waiting for FFmpeg
 * in an HTTP worker. Only one background job is admitted; other recordings
 * return PENDING and can retry later. Duplicate requests share the same job.
 * Failed jobs return FAILED for 30 seconds to avoid a subprocess retry storm.
 * Paths are copied before returning. Completed cache hits return READY.
 */
recording_transcode_status_t request_recording_transcode_cache(
    const char *original_path, const char *cache_path);

/** Stop admitting playback jobs, abort FFmpeg and join the background worker. */
void shutdown_recording_transcode(void);

#endif /* LIGHTNVR_RECORDING_TRANSCODE_H */
