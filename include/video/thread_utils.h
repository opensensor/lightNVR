#ifndef THREAD_UTILS_H
#define THREAD_UTILS_H

#include <pthread.h>

// Join a thread with timeout
int pthread_join_with_timeout(pthread_t thread, void **retval, int timeout_sec);

#endif // THREAD_UTILS_H
