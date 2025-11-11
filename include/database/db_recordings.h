#ifndef LIGHTNVR_DB_RECORDINGS_H
#define LIGHTNVR_DB_RECORDINGS_H

#include <stdint.h>
#include <stdbool.h>
#include <time.h>

// Recording metadata structure
typedef struct {
    uint64_t id;
    char stream_name[64];
    char file_path[256];
    time_t start_time;
    time_t end_time;
    uint64_t size_bytes;
    int width;
    int height;
    int fps;
    char codec[16];
    bool is_complete;
    char trigger_type[16];  // 'scheduled', 'detection', 'motion', 'manual'
} recording_metadata_t;

/**
 * Add recording metadata to the database
 * 
 * @param metadata Recording metadata
 * @return Recording ID on success, 0 on failure
 */
uint64_t add_recording_metadata(const recording_metadata_t *metadata);

/**
 * Update recording metadata in the database
 * 
 * @param id Recording ID
 * @param end_time New end time
 * @param size_bytes New size in bytes
 * @param is_complete Whether the recording is complete
 * @return 0 on success, non-zero on failure
 */
int update_recording_metadata(uint64_t id, time_t end_time, 
                             uint64_t size_bytes, bool is_complete);

/**
 * Get recording metadata from the database
 * 
 * @param start_time Start time filter (0 for no filter)
 * @param end_time End time filter (0 for no filter)
 * @param stream_name Stream name filter (NULL for all streams)
 * @param metadata Array to fill with recording metadata
 * @param max_count Maximum number of recordings to return
 * @return Number of recordings found, or -1 on error
 */
int get_recording_metadata(time_t start_time, time_t end_time, 
                          const char *stream_name, recording_metadata_t *metadata, 
                          int max_count);

/**
 * Get total count of recordings matching filter criteria
 * 
 * @param start_time Start time filter (0 for no filter)
 * @param end_time End time filter (0 for no filter)
 * @param stream_name Stream name filter (NULL for all streams)
 * @param has_detection Filter for recordings with detection events (0 for all)
 * @return Total count of matching recordings, or -1 on error
 */
int get_recording_count(time_t start_time, time_t end_time, 
                       const char *stream_name, int has_detection);

/**
 * Get paginated recording metadata from the database with sorting
 * 
 * @param start_time Start time filter (0 for no filter)
 * @param end_time End time filter (0 for no filter)
 * @param stream_name Stream name filter (NULL for all streams)
 * @param has_detection Filter for recordings with detection events (0 for all)
 * @param sort_field Field to sort by (e.g., "start_time", "stream_name", "size_bytes")
 * @param sort_order Sort order ("asc" or "desc")
 * @param metadata Array to fill with recording metadata
 * @param limit Maximum number of recordings to return
 * @param offset Number of recordings to skip (for pagination)
 * @return Number of recordings found, or -1 on error
 */
int get_recording_metadata_paginated(time_t start_time, time_t end_time, 
                                   const char *stream_name, int has_detection,
                                   const char *sort_field, const char *sort_order,
                                   recording_metadata_t *metadata, 
                                   int limit, int offset);

/**
 * Get recording metadata by ID
 * 
 * @param id Recording ID
 * @param metadata Pointer to metadata structure to fill
 * @return 0 on success, non-zero on failure
 */
int get_recording_metadata_by_id(uint64_t id, recording_metadata_t *metadata);

/**
 * Delete recording metadata from the database
 * 
 * @param id Recording ID
 * @return 0 on success, non-zero on failure
 */
int delete_recording_metadata(uint64_t id);

/**
 * Delete old recording metadata from the database
 * 
 * @param max_age Maximum age in seconds
 * @return Number of recordings deleted, or -1 on error
 */
int delete_old_recording_metadata(uint64_t max_age);

#endif // LIGHTNVR_DB_RECORDINGS_H
