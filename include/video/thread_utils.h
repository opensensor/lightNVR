#ifndef THREAD_UTILS_H
#define THREAD_UTILS_H

#include <pthread.h>

/**
 * Join a thread with a timeout
 * 
 * @param thread Thread to join
 * @param retval Pointer to store the return value of the thread
 * @param timeout_seconds Timeout in seconds
 * @return 0 on success, ETIMEDOUT on timeout, other non-zero value on error
 */
int pthread_join_with_timeout(pthread_t thread, void **retval, int timeout_seconds);

#endif /* THREAD_UTILS_H */
