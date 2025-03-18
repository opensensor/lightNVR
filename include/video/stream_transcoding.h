#ifndef STREAM_TRANSCODING_H
#define STREAM_TRANSCODING_H

/**
 * Initialize transcoding backend
 * Sets up FFmpeg and timestamp trackers
 */
void init_transcoding_backend(void);

/**
 * Cleanup transcoding backend
 * Cleans up FFmpeg and timestamp trackers
 */
void cleanup_transcoding_backend(void);

#endif /* STREAM_TRANSCODING_H */
