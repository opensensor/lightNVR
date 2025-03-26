#ifndef DETECTION_BUFFER_H
#define DETECTION_BUFFER_H

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>
#include <time.h>

// Buffer pool item structure
typedef struct {
    uint8_t *buffer;
    size_t size;
    bool in_use;
    time_t last_used;  // Track when the buffer was last used
} buffer_pool_item_t;

/**
 * Initialize the buffer pool
 * This should be called at startup
 * 
 * @return 0 on success, non-zero on failure
 */
int init_buffer_pool(void);

/**
 * Cleanup the buffer pool
 * This should be called at shutdown
 */
void cleanup_buffer_pool(void);

/**
 * Get a buffer from the pool
 * 
 * @param required_size The required buffer size
 * @return Pointer to the buffer or NULL on failure
 */
uint8_t* get_buffer_from_pool(size_t required_size);

/**
 * Return a buffer to the pool
 * 
 * @param buffer Pointer to the buffer
 * @return 0 on success, non-zero on failure
 */
int return_buffer_to_pool(uint8_t *buffer);

/**
 * Get the number of active buffers
 * 
 * @return Number of active buffers
 */
int get_active_buffer_count(void);

/**
 * Get the maximum number of buffers in the pool
 * 
 * @return Maximum number of buffers
 */
int get_max_buffer_count(void);

/**
 * Emergency cleanup of buffer pool
 * This frees all buffers that have been in use for too long
 * Call this when buffer allocation fails to recover from leaks
 */
void emergency_buffer_pool_cleanup(void);

/**
 * Track memory allocations
 * 
 * @param size Size of the allocation
 * @param is_allocation True if allocating, false if freeing
 */
void track_memory_allocation(size_t size, bool is_allocation);

#endif /* DETECTION_BUFFER_H */
