#define _XOPEN_SOURCE
#define _GNU_SOURCE

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <pthread.h>
#include <time.h>
#include "web/recordings_playback_state.h"
#include "core/logger.h"

// Maximum number of concurrent playback sessions
#define MAX_CONCURRENT_PLAYBACKS 32

// Array of active playback sessions
static recording_playback_state_t playback_sessions[MAX_CONCURRENT_PLAYBACKS];

// Mutex to protect access to playback sessions
static pthread_mutex_t playback_mutex = PTHREAD_MUTEX_INITIALIZER;

// Initialize playback sessions
void init_playback_sessions(void) {
    static bool initialized = false;
    
    if (!initialized) {
        pthread_mutex_lock(&playback_mutex);
        
        if (!initialized) {
            memset(playback_sessions, 0, sizeof(playback_sessions));
            initialized = true;
            log_info("Initialized recording playback session manager");
        }
        
        pthread_mutex_unlock(&playback_mutex);
    }
}

// Find a free playback session slot
int find_free_playback_slot(void) {
    for (int i = 0; i < MAX_CONCURRENT_PLAYBACKS; i++) {
        if (playback_sessions[i].file == NULL) {
            return i;
        }
    }
    return -1;
}

// Clean up inactive playback sessions (called periodically)
void cleanup_inactive_playback_sessions(void) {
    time_t now = time(NULL);
    
    pthread_mutex_lock(&playback_mutex);
    
    for (int i = 0; i < MAX_CONCURRENT_PLAYBACKS; i++) {
        if (playback_sessions[i].file != NULL) {
            // If no activity for 30 seconds, close the session
            if (difftime(now, playback_sessions[i].last_activity) > 30) {
                log_info("Closing inactive playback session for recording %llu", 
                        (unsigned long long)playback_sessions[i].recording_id);
                
                fclose(playback_sessions[i].file);
                playback_sessions[i].file = NULL;
                playback_sessions[i].recording_id = 0;
                playback_sessions[i].bytes_sent = 0;
            }
        }
    }
    
    pthread_mutex_unlock(&playback_mutex);
}

// Get a reference to the playback sessions array
recording_playback_state_t* get_playback_sessions(void) {
    return playback_sessions;
}

// Get the playback mutex
pthread_mutex_t* get_playback_mutex(void) {
    return &playback_mutex;
}
