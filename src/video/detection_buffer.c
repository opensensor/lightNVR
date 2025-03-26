#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <time.h>

#include "../../include/core/logger.h"
#include "../../include/utils/memory.h"
#include "../../include/video/detection_config.h"
#include "../../include/video/detection_buffer.h"

// Buffer pool
static buffer_pool_item_t *buffer_pool = NULL;
static int active_buffers = 0;
static int max_buffers = 0;

// External declaration for memory tracking
extern void track_memory_allocation(size_t size, bool is_allocation);

/**
 * Initialize the buffer pool
 */
int init_buffer_pool(void) {
    // Check if already initialized
    if (buffer_pool) {
        return 0;
    }
    
    // Get configuration
    detection_config_t *config = get_detection_config();
    if (!config) {
        log_error("Failed to get detection configuration");
        return -1;
    }
    
    // Allocate buffer pool
    max_buffers = config->buffer_pool_size;
    buffer_pool = (buffer_pool_item_t *)calloc(max_buffers, sizeof(buffer_pool_item_t));
    if (!buffer_pool) {
        log_error("Failed to allocate buffer pool");
        return -1;
    }
    
    log_info("Buffer pool initialized with %d buffers", max_buffers);
    return 0;
}

/**
 * Cleanup the buffer pool
 */
void cleanup_buffer_pool(void) {
    if (!buffer_pool) {
        return;
    }
    
    // Free all buffers
    for (int i = 0; i < max_buffers; i++) {
        if (buffer_pool[i].buffer) {
            track_memory_allocation(buffer_pool[i].size, false);
            free(buffer_pool[i].buffer);
            buffer_pool[i].buffer = NULL;
            buffer_pool[i].size = 0;
            buffer_pool[i].in_use = false;
        }
    }
    
    // Free buffer pool
    free(buffer_pool);
    buffer_pool = NULL;
    active_buffers = 0;
    max_buffers = 0;
    
    log_info("Buffer pool cleaned up");
}

/**
 * Get a buffer from the pool
 */
uint8_t* get_buffer_from_pool(size_t required_size) {
    // Initialize if not already done
    if (!buffer_pool) {
        if (init_buffer_pool() != 0) {
            log_error("Failed to initialize buffer pool");
            return NULL;
        }
    }
    
    // Get configuration
    detection_config_t *config = get_detection_config();
    if (!config) {
        log_error("Failed to get detection configuration");
        return NULL;
    }
    
    // Try multiple times to get a buffer
    for (int retry = 0; retry < config->buffer_allocation_retries; retry++) {
        // First try to find an existing buffer in the pool
        for (int i = 0; i < max_buffers; i++) {
            if (!buffer_pool[i].in_use && buffer_pool[i].buffer && buffer_pool[i].size >= required_size) {
                buffer_pool[i].in_use = true;
                buffer_pool[i].last_used = time(NULL);
                active_buffers++;
                log_info("Reusing buffer from pool (index %d, size %zu, retry %d)", 
                        i, buffer_pool[i].size, retry);
                return buffer_pool[i].buffer;
            }
        }
        
        // If no suitable buffer found, try to allocate a new one
        // Find an empty slot in the pool
        int empty_slot = -1;
        for (int i = 0; i < max_buffers; i++) {
            if (!buffer_pool[i].buffer) {
                empty_slot = i;
                break;
            }
        }
        
        // If no empty slot, try to find the oldest unused buffer
        if (empty_slot == -1) {
            time_t oldest_time = time(NULL);
            for (int i = 0; i < max_buffers; i++) {
                if (!buffer_pool[i].in_use && buffer_pool[i].last_used < oldest_time) {
                    oldest_time = buffer_pool[i].last_used;
                    empty_slot = i;
                }
            }
        }
        
        // If still no slot, we can't allocate a new buffer
        if (empty_slot == -1) {
            log_error("No available slots in buffer pool (retry %d)", retry);                
            // If this is the last retry, give up
            if (retry == config->buffer_allocation_retries - 1) {
                return NULL;
            }
            
            // Wait a bit before retrying
            usleep(100000); // 100ms
            continue;
        }
        
        // If there's an existing buffer but it's too small, free it
        if (buffer_pool[empty_slot].buffer && buffer_pool[empty_slot].size < required_size) {
            // Track memory before freeing
            track_memory_allocation(buffer_pool[empty_slot].size, false);
            free(buffer_pool[empty_slot].buffer);
            buffer_pool[empty_slot].buffer = NULL;
        }
        
        // Allocate a new buffer if needed
        if (!buffer_pool[empty_slot].buffer) {
            buffer_pool[empty_slot].buffer = (uint8_t *)safe_malloc(required_size);
            if (!buffer_pool[empty_slot].buffer) {
                log_error("Failed to allocate buffer for pool (size: %zu, retry %d)", 
                         required_size, retry);
                
                // If this is the last retry, give up
                if (retry == config->buffer_allocation_retries - 1) {
                    return NULL;
                }
                
                // Wait a bit before retrying
                usleep(100000); // 100ms
                continue;
            }
            
            // Track memory allocation
            track_memory_allocation(required_size, true);
            buffer_pool[empty_slot].size = required_size;
        }
        
        buffer_pool[empty_slot].in_use = true;
        buffer_pool[empty_slot].last_used = time(NULL);
        active_buffers++;
        log_info("Allocated buffer for pool (index %d, size %zu, retry %d)", 
                empty_slot, required_size, retry);
        return buffer_pool[empty_slot].buffer;
    }
    
    log_error("Failed to allocate buffer from pool");
    return NULL;
}

/**
 * Return a buffer to the pool
 */
int return_buffer_to_pool(uint8_t *buffer) {
    if (!buffer || !buffer_pool) {
        return -1;
    }
    
    // Find the buffer in the pool
    for (int i = 0; i < max_buffers; i++) {
        if (buffer_pool[i].buffer == buffer) {
            buffer_pool[i].in_use = false;
            buffer_pool[i].last_used = time(NULL);
            active_buffers--;
            log_info("Returned buffer to pool (index %d)", i);
            return 0;
        }
    }
    
    log_error("Buffer not found in pool");
    return -1;
}

/**
 * Get the number of active buffers
 */
int get_active_buffer_count(void) {
    return active_buffers;
}

/**
 * Get the maximum number of buffers in the pool
 */
int get_max_buffer_count(void) {
    return max_buffers;
}
