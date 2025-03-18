#ifndef TIMESTAMP_MANAGER_H
#define TIMESTAMP_MANAGER_H

#include <stdint.h>
#include <stdbool.h>

/**
 * Initialize all timestamp trackers
 * 
 * @return 0 on success, non-zero on failure
 */
int init_timestamp_trackers(void);

/**
 * Cleanup all timestamp trackers
 * 
 * @return 0 on success, non-zero on failure
 */
int cleanup_timestamp_trackers(void);

/**
 * Initialize the timestamp tracker for a stream
 * 
 * @param stream_name Name of the stream
 * @return 0 on success, non-zero on failure
 */
int init_timestamp_tracker(const char *stream_name);

/**
 * Reset the timestamp tracker for a stream
 * 
 * @param stream_name Name of the stream
 * @return 0 on success, non-zero on failure
 */
int reset_timestamp_tracker(const char *stream_name);

/**
 * Remove the timestamp tracker for a stream
 * 
 * @param stream_name Name of the stream
 * @return 0 on success, non-zero on failure
 */
int remove_timestamp_tracker(const char *stream_name);

/**
 * Set the UDP flag for a stream's timestamp tracker
 * 
 * @param stream_name Name of the stream
 * @param is_udp Whether the stream is using UDP protocol
 * @return 0 on success, non-zero on failure
 */
int set_timestamp_tracker_udp_flag(const char *stream_name, bool is_udp);

#endif /* TIMESTAMP_MANAGER_H */
