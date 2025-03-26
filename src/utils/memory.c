#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdint.h>
#include <stdbool.h>

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

// Safe string duplication
char *safe_strdup(const char *str) {
    if (!str) {
        return NULL;
    }

    char *ptr = strdup(str);
    if (!ptr) {
        log_error("String duplication failed for length %zu", strlen(str));
    }

    return ptr;
}

// Safe string copy
int safe_strcpy(char *dest, const char *src, size_t size) {
    if (!dest || !src || size == 0) {
        return -1;
    }

    size_t src_len = strlen(src);
    if (src_len >= size) {
        // Not enough space, truncate but still null-terminate
        memcpy(dest, src, size - 1);
        dest[size - 1] = '\0';
        return -1;
    }

    memcpy(dest, src, src_len + 1); // Include null terminator
    return 0;
}

// Safe string concatenation
int safe_strcat(char *dest, const char *src, size_t size) {
    if (!dest || !src || size == 0) {
        return -1;
    }

    size_t dest_len = strlen(dest);
    if (dest_len >= size) {
        // Destination already fills the buffer
        return -1;
    }

    return safe_strcpy(dest + dest_len, src, size - dest_len);
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
