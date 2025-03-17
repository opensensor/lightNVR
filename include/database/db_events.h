#ifndef LIGHTNVR_DB_EVENTS_H
#define LIGHTNVR_DB_EVENTS_H

#include <stdint.h>
#include <time.h>

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
 * Delete old events from the database
 * 
 * @param max_age Maximum age in seconds
 * @return Number of events deleted, or -1 on error
 */
int delete_old_events(uint64_t max_age);

#endif // LIGHTNVR_DB_EVENTS_H
