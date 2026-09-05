#define _POSIX_C_SOURCE 200809L

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdint.h>
#include <stdbool.h>
#include <errno.h>
#include <fcntl.h>
#include <unistd.h>

#include "utils/memory.h"
#include "core/logger.h"

// Memory tracking variables
static size_t total_memory_allocated = 0;
static size_t peak_memory_allocated = 0;

// Safe memory allocation
void *safe_malloc(size_t size) {
    if (size == 0) {
        return NULL;
    }

    void *ptr = malloc(size);
    if (!ptr) {
        log_error("Memory allocation failed for size %zu", size);
    }

    return ptr;
}

// Safe memory reallocation
void *safe_realloc(void *ptr, size_t size) {
    if (size == 0) {
        free(ptr);
        return NULL;
    }

    void *new_ptr = realloc(ptr, size);
    if (!new_ptr) {
        log_error("Memory reallocation failed for size %zu", size);
    }

    return new_ptr;
}

// Safe calloc - allocate zeroed memory
void *safe_calloc(size_t nmemb, size_t size) {
    if (nmemb == 0 || size == 0) {
        return NULL;
    }

    void *ptr = calloc(nmemb, size);
    if (!ptr) {
        log_error("Memory calloc failed for %zu x %zu bytes", nmemb, size);
    }

    return ptr;
}

// Safe free - handles NULL pointers
void safe_free(void *ptr) {
    if (ptr) {
        free(ptr);
    }
}

// Secure memory clearing that won't be optimized away
void secure_zero_memory(void *ptr, size_t size) {
    if (!ptr || size == 0) {
        return;
    }

    volatile uint8_t *p = (volatile uint8_t *)ptr;
    while (size--) {
        *p++ = 0;
    }
}

// Track memory allocations
void track_memory_allocation(size_t size, bool is_allocation) {
    if (is_allocation) {
        total_memory_allocated += size;
        if (total_memory_allocated > peak_memory_allocated) {
            peak_memory_allocated = total_memory_allocated;
        }
    } else {
        if (size <= total_memory_allocated) {
            total_memory_allocated -= size;
        } else {
            // This should not happen, but handle it gracefully
            log_warn("Memory tracking inconsistency: trying to free %zu bytes when only %zu are tracked",
                    size, total_memory_allocated);
            total_memory_allocated = 0;
        }
    }
}

// Get total memory allocated
size_t get_total_memory_allocated(void) {
    return total_memory_allocated;
}

// Get peak memory allocated
size_t get_peak_memory_allocated(void) {
    return peak_memory_allocated;
}

bool file_cache_release_due(off_t released_through, off_t current_position,
                            off_t interval_bytes) {
    if (released_through < 0 || current_position < released_through ||
        interval_bytes <= 0) {
        return false;
    }

    return current_position - released_through >= interval_bytes;
}

int file_cache_flush_and_release(int fd, off_t offset, off_t length) {
    if (fd < 0 || offset < 0 || length < 0) {
        return EINVAL;
    }

    /* POSIX_FADV_DONTNEED is only reliable for clean pages. Persist the file
     * first so a memory-limited recorder can actually reclaim the range. */
    if (fdatasync(fd) != 0) {
        return errno;
    }

#ifdef POSIX_FADV_DONTNEED
    return posix_fadvise(fd, offset, length, POSIX_FADV_DONTNEED);
#else
    (void)offset;
    (void)length;
    return ENOTSUP;
#endif
}
