/**
 * MP4 Recording Internal Header
 * 
 * This header contains internal declarations for the MP4 recording module.
 * It is not intended to be used by external modules.
 */

#ifndef MP4_RECORDING_INTERNAL_H
#define MP4_RECORDING_INTERNAL_H

#include "video/mp4_recording.h"
#include "video/thread_utils.h"

/**
 * External declarations for shared variables
 * 
 * These variables are defined in mp4_recording_core.c and are used
 * by other files in the MP4 recording module.
 */
extern mp4_recording_ctx_t *recording_contexts[MAX_STREAMS];

/**
 * Update MP4 recording metadata in the database
 * 
 * This function is called periodically to update the recording metadata
 * in the database.
 * 
 * @param stream_name Name of the stream
 */
void update_mp4_recording(const char *stream_name);

// We'll use the pthread_join_with_timeout function from thread_utils.h

#endif /* MP4_RECORDING_INTERNAL_H */
