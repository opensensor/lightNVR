#include <stdio.h>
#include <stdlib.h>
#include <pthread.h>
#include <errno.h>
#include <time.h>
#include <signal.h>
#include <string.h>

// Define pthread_timedjoin_np if not available
#ifndef __USE_GNU
extern int pthread_timedjoin_np(pthread_t thread, void **retval, const struct timespec *abstime);
#endif

// Define CLOCK_REALTIME if not available
#ifndef CLOCK_REALTIME
#define CLOCK_REALTIME 0
#endif

#include "core/logger.h"
#include "video/thread_utils.h"

/**
 * Join a thread with a timeout
 */
int pthread_join_with_timeout(pthread_t thread, void **retval, int timeout_seconds) {
    struct timespec ts;
    int ret;
    
    // Get current time
    if (clock_gettime(CLOCK_REALTIME, &ts) != 0) {
        log_error("Failed to get current time: %s", strerror(errno));
        return EINVAL;
    }
    
    // Add timeout
    ts.tv_sec += timeout_seconds;
    
    // Try to join with timeout
    ret = pthread_timedjoin_np(thread, retval, &ts);
    
    if (ret == ETIMEDOUT) {
        log_warn("Thread join timed out after %d seconds", timeout_seconds);
        
        // Send a signal to the thread to try to get it to exit
        // This is a last resort and may not work
        pthread_kill(thread, SIGUSR1);
        
        // Try to join one more time with a short timeout
        ts.tv_sec = time(NULL) + 1; // 1 second timeout
        ret = pthread_timedjoin_np(thread, retval, &ts);
        
        if (ret == 0) {
            log_info("Thread joined successfully after sending signal");
        } else {
            log_warn("Thread still not joined after sending signal: %s", strerror(ret));
        }
    } else if (ret != 0) {
        log_error("Failed to join thread: %s", strerror(ret));
    }
    
    return ret;
}
