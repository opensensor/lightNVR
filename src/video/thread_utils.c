#include "video/thread_utils.h"
#include "core/logger.h"
#include <stdlib.h>
#include <errno.h>
#include <time.h>

// Define CLOCK_REALTIME if not available
#ifndef CLOCK_REALTIME
#define CLOCK_REALTIME 0
#endif

/**
 * Thread data structure for join helper
 */
typedef struct {
    pthread_t thread;
    void **retval;
    int result;
    pthread_mutex_t mutex;
    pthread_cond_t cond;
    int done;
    int cancelled;
} join_helper_data_t;

/**
 * Helper thread function for pthread_join_with_timeout
 */
static void *join_helper(void *arg) {
    join_helper_data_t *data = (join_helper_data_t *)arg;
    
    // Setup cancellation state
    pthread_setcancelstate(PTHREAD_CANCEL_ENABLE, NULL);
    pthread_setcanceltype(PTHREAD_CANCEL_DEFERRED, NULL);
    
    // Check if we've been cancelled before even starting
    pthread_mutex_lock(&data->mutex);
    if (data->cancelled) {
        pthread_mutex_unlock(&data->mutex);
        return NULL;
    }
    pthread_mutex_unlock(&data->mutex);
    
    // Join the target thread
    data->result = pthread_join(data->thread, data->retval);
    
    // Signal completion
    pthread_mutex_lock(&data->mutex);
    data->done = 1;
    pthread_cond_signal(&data->cond);
    pthread_mutex_unlock(&data->mutex);
    
    return NULL;
}

/**
 * Join a thread with timeout
 */
int pthread_join_with_timeout(pthread_t thread, void **retval, int timeout_sec) {
    int ret = 0;
    pthread_t timeout_thread;
    
    // Allocate helper data on the heap to avoid stack issues
    join_helper_data_t *data = calloc(1, sizeof(join_helper_data_t));
    if (!data) {
        log_error("Failed to allocate memory for join helper data");
        return ENOMEM;
    }
    
    // Initialize data
    data->thread = thread;
    data->retval = retval;
    data->result = -1;
    data->done = 0;
    data->cancelled = 0;
    
    // Initialize mutex and condition variable
    if (pthread_mutex_init(&data->mutex, NULL) != 0) {
        log_error("Failed to initialize mutex for join helper");
        free(data);
        return EAGAIN;
    }
    
    if (pthread_cond_init(&data->cond, NULL) != 0) {
        log_error("Failed to initialize condition variable for join helper");
        pthread_mutex_destroy(&data->mutex);
        free(data);
        return EAGAIN;
    }
    
    // Set up thread cancellation state
    pthread_attr_t attr;
    pthread_attr_init(&attr);
    pthread_attr_setdetachstate(&attr, PTHREAD_CREATE_JOINABLE);
    
    // Create helper thread to join the target thread
    if (pthread_create(&timeout_thread, &attr, join_helper, data) != 0) {
        pthread_attr_destroy(&attr);
        pthread_mutex_destroy(&data->mutex);
        pthread_cond_destroy(&data->cond);
        free(data);
        return EAGAIN;
    }
    
    pthread_attr_destroy(&attr);
    
    // Wait for the helper thread to complete or timeout
    struct timespec ts;
    clock_gettime(CLOCK_REALTIME, &ts);
    ts.tv_sec += timeout_sec;
    
    pthread_mutex_lock(&data->mutex);
    while (!data->done) {
        ret = pthread_cond_timedwait(&data->cond, &data->mutex, &ts);
        if (ret == ETIMEDOUT) {
            // Timeout occurred
            data->cancelled = 1;  // Mark as cancelled before unlocking
            pthread_mutex_unlock(&data->mutex);
            
            // Instead of cancelling, try to join with a shorter timeout
            struct timespec short_wait;
            clock_gettime(CLOCK_REALTIME, &short_wait);
            short_wait.tv_sec += 1;  // Wait just 1 more second
            
            pthread_mutex_lock(&data->mutex);
            if (!data->done) {
                ret = pthread_cond_timedwait(&data->cond, &data->mutex, &short_wait);
            }
            pthread_mutex_unlock(&data->mutex);
            
            // If still not done, we have to cancel
            if (!data->done) {
                log_warn("Thread join helper did not complete in time, cancelling");
                
                // Set cancel type to deferred to allow cleanup
                int old_type;
                pthread_setcanceltype(PTHREAD_CANCEL_DEFERRED, &old_type);
                
                // Cancel the helper thread
                pthread_cancel(timeout_thread);
                
                // Wait for the helper thread to actually terminate
                void *thread_result;
                pthread_join(timeout_thread, &thread_result);
            } else {
                // Thread completed during our short wait
                ret = data->result;
                pthread_join(timeout_thread, NULL);
            }
            
            // Clean up resources
            pthread_mutex_destroy(&data->mutex);
            pthread_cond_destroy(&data->cond);
            free(data);
            
            return ETIMEDOUT;
        }
    }
    
    // Get the join result before unlocking
    ret = data->result;
    pthread_mutex_unlock(&data->mutex);
    
    // Join the helper thread to clean up
    pthread_join(timeout_thread, NULL);
    
    // Clean up resources
    pthread_mutex_destroy(&data->mutex);
    pthread_cond_destroy(&data->cond);
    free(data);
    
    return ret;
}
