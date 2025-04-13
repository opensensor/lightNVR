#include "video/ffmpeg_leak_detector.h"
#include "core/logger.h"
#include <pthread.h>
#include <stdlib.h>
#include <string.h>

// Structure to track FFmpeg allocations
typedef struct {
    void *ptr;
    const char *type;
    const char *location;
    int line;
} ffmpeg_allocation_t;

// Array to store tracked allocations
static ffmpeg_allocation_t *tracked_allocations = NULL;
static int allocation_count = 0;
static int allocation_capacity = 0;
static pthread_mutex_t allocation_mutex = PTHREAD_MUTEX_INITIALIZER;

// Initialize the leak detector
void ffmpeg_leak_detector_init(void) {
    pthread_mutex_lock(&allocation_mutex);

    // Initialize with capacity for 1000 allocations
    allocation_capacity = 1000;
    tracked_allocations = calloc(allocation_capacity, sizeof(ffmpeg_allocation_t));

    if (!tracked_allocations) {
        log_error("Failed to initialize FFmpeg leak detector");
        allocation_capacity = 0;
    } else {
        log_info("FFmpeg leak detector initialized with capacity for %d allocations", allocation_capacity);
    }

    pthread_mutex_unlock(&allocation_mutex);
}

// Clean up the leak detector
void ffmpeg_leak_detector_cleanup(void) {
    pthread_mutex_lock(&allocation_mutex);

    // Report any leaks
    if (allocation_count > 0) {
        log_warn("FFmpeg leak detector found %d potential leaks:", allocation_count);
        for (int i = 0; i < allocation_count; i++) {
            log_warn("  Leak %d: %s at %p (from %s:%d)",
                    i + 1,
                    tracked_allocations[i].type,
                    tracked_allocations[i].ptr,
                    tracked_allocations[i].location,
                    tracked_allocations[i].line);
        }
    } else {
        log_info("FFmpeg leak detector found no leaks");
    }

    // Free the allocation tracker
    free(tracked_allocations);
    tracked_allocations = NULL;
    allocation_count = 0;
    allocation_capacity = 0;

    pthread_mutex_unlock(&allocation_mutex);
}

// Track a new allocation
void ffmpeg_track_allocation(void *ptr, const char *type, const char *location, int line) {
    if (!ptr) return;

    pthread_mutex_lock(&allocation_mutex);

    // Check if we need to expand the array
    if (allocation_count >= allocation_capacity && tracked_allocations) {
        int new_capacity = allocation_capacity * 2;
        ffmpeg_allocation_t *new_array = realloc(tracked_allocations,
                                                new_capacity * sizeof(ffmpeg_allocation_t));

        if (new_array) {
            tracked_allocations = new_array;
            allocation_capacity = new_capacity;
            log_debug("FFmpeg leak detector expanded to capacity %d", allocation_capacity);
        } else {
            log_error("Failed to expand FFmpeg leak detector capacity");
            pthread_mutex_unlock(&allocation_mutex);
            return;
        }
    }

    // Add the allocation to the array
    if (tracked_allocations && allocation_count < allocation_capacity) {
        tracked_allocations[allocation_count].ptr = ptr;
        tracked_allocations[allocation_count].type = type;
        tracked_allocations[allocation_count].location = location;
        tracked_allocations[allocation_count].line = line;
        allocation_count++;
    }

    pthread_mutex_unlock(&allocation_mutex);
}

// Untrack an allocation when it's freed
void ffmpeg_untrack_allocation(void *ptr) {
    if (!ptr) return;

    pthread_mutex_lock(&allocation_mutex);

    // Find the allocation in the array
    for (int i = 0; i < allocation_count; i++) {
        if (tracked_allocations[i].ptr == ptr) {
            // Remove it by shifting all subsequent elements
            if (i < allocation_count - 1) {
                memmove(&tracked_allocations[i],
                        &tracked_allocations[i + 1],
                        (allocation_count - i - 1) * sizeof(ffmpeg_allocation_t));
            }
            allocation_count--;
            break;
        }
    }

    pthread_mutex_unlock(&allocation_mutex);
}

// Get the current number of tracked allocations
int ffmpeg_get_allocation_count(void) {
    pthread_mutex_lock(&allocation_mutex);
    int count = allocation_count;
    pthread_mutex_unlock(&allocation_mutex);
    return count;
}

// Dump the current allocation list to the log
void ffmpeg_dump_allocations(void) {
    pthread_mutex_lock(&allocation_mutex);

    log_info("FFmpeg allocation dump (%d allocations):", allocation_count);
    for (int i = 0; i < allocation_count; i++) {
        log_info("  Allocation %d: %s at %p (from %s:%d)",
                i + 1,
                tracked_allocations[i].type,
                tracked_allocations[i].ptr,
                tracked_allocations[i].location,
                tracked_allocations[i].line);
    }

    pthread_mutex_unlock(&allocation_mutex);
}

// Force cleanup of all tracked allocations
void ffmpeg_force_cleanup_all(void) {
    pthread_mutex_lock(&allocation_mutex);

    // Just log the number of allocations but don't try to free them
    // This avoids potential use-after-free issues during shutdown
    if (allocation_count > 0) {
        log_warn("Skipping cleanup of %d FFmpeg allocations to avoid potential crashes", allocation_count);

        // Just clear the tracking array without attempting to free anything
        allocation_count = 0;
    } else {
        log_info("No FFmpeg allocations to clean up");
    }

    pthread_mutex_unlock(&allocation_mutex);
}
