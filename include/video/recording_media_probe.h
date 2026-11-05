#ifndef LIGHTNVR_RECORDING_MEDIA_PROBE_H
#define LIGHTNVR_RECORDING_MEDIA_PROBE_H

#include <stdbool.h>

/**
 * Check that an MP4 can actually be played, by decoding its first video frame.
 *
 * A writer that dies before writing the trailer leaves a file with no moov atom,
 * and a faststart header can survive with its video data missing. Neither plays,
 * though both are non-empty.
 *
 * @return true only if a video frame decodes without errors, or if the video
 *         packets read correctly and this build has no decoder for the codec
 */
bool recording_media_has_playable_video(const char *path);

#endif /* LIGHTNVR_RECORDING_MEDIA_PROBE_H */
