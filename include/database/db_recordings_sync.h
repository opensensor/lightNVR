/**
 * Database Recordings Synchronization Header
 * 
 * This module provides functionality to synchronize recording metadata in the database
 * with actual file sizes on disk.
 */

#ifndef DB_RECORDINGS_SYNC_H
#define DB_RECORDINGS_SYNC_H

#include <stdint.h>
#include <stdbool.h>

/**
 * Start the recording sync thread
 * 
 * @param interval_seconds Interval between syncs in seconds (minimum 10)
 * @return 0 on success, -1 on error
 */
int start_recording_sync_thread(int interval_seconds);

/**
 * Stop the recording sync thread
 * 
 * @return 0 on success, -1 on error
 */
int stop_recording_sync_thread(void);

/**
 * Force an immediate sync of all recordings
 * 
 * @return Number of recordings updated, or -1 on error
 */
int force_recording_sync(void);

#endif // DB_RECORDINGS_SYNC_H

