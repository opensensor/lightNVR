#ifndef ONVIF_DISCOVERY_THREAD_H
#define ONVIF_DISCOVERY_THREAD_H

#include <pthread.h>

// Discovery thread data
typedef struct {
    pthread_t thread;
    int running;
    char network[64];
    int interval;
} discovery_thread_t;

// Global discovery thread
extern discovery_thread_t g_discovery_thread;

// Mutex for thread safety
extern pthread_mutex_t g_discovery_mutex;

/**
 * Discovery thread function
 * 
 * @param arg Thread data
 * @return NULL
 */
void *discovery_thread_func(void *arg);

#endif /* ONVIF_DISCOVERY_THREAD_H */
