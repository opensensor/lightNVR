#ifndef LIGHTNVR_RECORDING_MEDIA_PROBE_H
#define LIGHTNVR_RECORDING_MEDIA_PROBE_H

/* Decode an initial video frame from a local MP4 within a best-effort budget.
 * Returns an empty string on success, a media-failure code, or an inconclusive
 * code (probe_limit, decoder_unavailable, probe_error). Returned strings are static.
 * A successful initial frame does not certify the rest of a damaged file. */
const char *recording_media_probe_issue(const char *path);

#endif
