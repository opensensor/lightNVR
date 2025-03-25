#ifndef RECORDINGS_PLAYBACK_STATE_H
#define RECORDINGS_PLAYBACK_STATE_H

#include <stdio.h>
#include <stdlib.h>
#include <stdbool.h>
#include <time.h>
#include "../core/config.h"

// Maximum number of concurrent playback sessions
#define MAX_CONCURRENT_PLAYBACKS 32

// Structure to hold recording playback state
typedef struct {
    FILE *file;                  // File handle
    char file_path[MAX_PATH_LENGTH]; // File path
    size_t file_size;            // Total file size
    size_t bytes_sent;           // Bytes sent so far
    uint64_t recording_id;       // Recording ID
    time_t last_activity;        // Last activity timestamp
} recording_playback_state_t;

// Initialize playback sessions
void init_playback_sessions(void);

// Find a free playback slot
int find_free_playback_slot(void);

// Clean up inactive playback sessions
void cleanup_inactive_playback_sessions(void);

// Get a reference to the playback sessions array
recording_playback_state_t* get_playback_sessions(void);

// Get the playback mutex
pthread_mutex_t* get_playback_mutex(void);

#endif // RECORDINGS_PLAYBACK_STATE_H
