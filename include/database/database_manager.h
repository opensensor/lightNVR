#ifndef LIGHTNVR_DATABASE_MANAGER_H
#define LIGHTNVR_DATABASE_MANAGER_H

#include <stdint.h>
#include <stdbool.h>
#include <time.h>
#include "core/config.h"

// Event types
typedef enum {
    EVENT_RECORDING_START = 0,
    EVENT_RECORDING_STOP,
    EVENT_STREAM_CONNECTED,
    EVENT_STREAM_DISCONNECTED,
    EVENT_STREAM_ERROR,
    EVENT_SYSTEM_START,
    EVENT_SYSTEM_STOP,
    EVENT_STORAGE_LOW,
    EVENT_STORAGE_FULL,
    EVENT_USER_LOGIN,
    EVENT_USER_LOGOUT,
    EVENT_CONFIG_CHANGE,
    EVENT_CUSTOM
} event_type_t;

// Event information structure
typedef struct {
    uint64_t id;
    event_type_t type;
    time_t timestamp;
    char stream_name[64];
    char description[256];
    char details[1024];
} event_info_t;

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
} recording_metadata_t;

/**
 * Initialize the database
 * 
 * @param db_path Path to the database file
 * @return 0 on success, non-zero on failure
 */
int init_database(const char *db_path);

/**
 * Shutdown the database
 */
void shutdown_database(void);

/**
 * Add an event to the database
 * 
 * @param type Event type
 * @param stream_name Stream name (can be NULL for system events)
 * @param description Short description of the event
 * @param details Detailed information about the event (can be NULL)
 * @return Event ID on success, 0 on failure
 */
uint64_t add_event(event_type_t type, const char *stream_name, 
                  const char *description, const char *details);

/**
 * Get events from the database
 * 
 * @param start_time Start time filter (0 for no filter)
 * @param end_time End time filter (0 for no filter)
 * @param type Event type filter (-1 for all types)
 * @param stream_name Stream name filter (NULL for all streams)
 * @param events Array to fill with event information
 * @param max_count Maximum number of events to return
 * @return Number of events found, or -1 on error
 */
int get_events(time_t start_time, time_t end_time, int type, 
              const char *stream_name, event_info_t *events, int max_count);

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
 * Delete old events from the database
 * 
 * @param max_age Maximum age in seconds
 * @return Number of events deleted, or -1 on error
 */
int delete_old_events(uint64_t max_age);

/**
 * Delete old recording metadata from the database
 * 
 * @param max_age Maximum age in seconds
 * @return Number of recordings deleted, or -1 on error
 */
int delete_old_recording_metadata(uint64_t max_age);

/**
 * Begin a database transaction
 * 
 * @return 0 on success, non-zero on failure
 */
int begin_transaction(void);

/**
 * Commit a database transaction
 * 
 * @return 0 on success, non-zero on failure
 */
int commit_transaction(void);

/**
 * Rollback a database transaction
 * 
 * @return 0 on success, non-zero on failure
 */
int rollback_transaction(void);

/**
 * Get the database size
 * 
 * @return Database size in bytes, or -1 on error
 */
int64_t get_database_size(void);

/**
 * Vacuum the database to reclaim space
 * 
 * @return 0 on success, non-zero on failure
 */
int vacuum_database(void);

/**
 * Check database integrity
 * 
 * @return 0 if database is valid, non-zero otherwise
 */
int check_database_integrity(void);

/**
 * Add a stream configuration to the database
 * 
 * @param stream Stream configuration to add
 * @return Stream ID on success, 0 on failure
 */
uint64_t add_stream_config(const stream_config_t *stream);

/**
 * Update a stream configuration in the database
 * 
 * @param name Stream name to update
 * @param stream Updated stream configuration
 * @return 0 on success, non-zero on failure
 */
int update_stream_config(const char *name, const stream_config_t *stream);

/**
 * Delete a stream configuration from the database
 * 
 * @param name Stream name to delete
 * @return 0 on success, non-zero on failure
 */
int delete_stream_config(const char *name);

/**
 * Get a stream configuration from the database
 * 
 * @param name Stream name to get
 * @param stream Stream configuration to fill
 * @return 0 on success, non-zero on failure
 */
int get_stream_config_by_name(const char *name, stream_config_t *stream);

/**
 * Get all stream configurations from the database
 * 
 * @param streams Array to fill with stream configurations
 * @param max_count Maximum number of streams to return
 * @return Number of streams found, or -1 on error
 */
int get_all_stream_configs(stream_config_t *streams, int max_count);

/**
 * Count the number of stream configurations in the database
 * 
 * @return Number of streams, or -1 on error
 */
int count_stream_configs(void);

#endif // LIGHTNVR_DATABASE_MANAGER_H
