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
        // First try to find an existing buffer in the pool that's not in use and is large enough
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
        
        // If no suitable buffer found, try to find an empty slot or reuse an existing one
        
        // First, look for an empty slot (no buffer allocated yet)
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
            int oldest_idx = -1;
            
            for (int i = 0; i < max_buffers; i++) {
                if (!buffer_pool[i].in_use && buffer_pool[i].last_used < oldest_time) {
                    oldest_time = buffer_pool[i].last_used;
                    oldest_idx = i;
                }
            }
            
            // If we found an unused buffer, use it
            if (oldest_idx >= 0) {
                empty_slot = oldest_idx;
            }
        }
        
        // If still no slot, try emergency cleanup to free leaked buffers
        if (empty_slot == -1) {
            log_warn("No available slots in buffer pool, attempting emergency cleanup (retry %d)", retry);
            emergency_buffer_pool_cleanup();
            
            // Try again to find an unused buffer after cleanup
            for (int i = 0; i < max_buffers; i++) {
                if (!buffer_pool[i].in_use) {
                    empty_slot = i;
                    break;
                }
            }
        }
        
        // If still no slot, we can't allocate a new buffer
        if (empty_slot == -1) {
            log_error("No available slots in buffer pool after cleanup (retry %d)", retry);
            
            // If this is the last retry, give up
            if (retry == config->buffer_allocation_retries - 1) {
                return NULL;
            }
            
            // Wait a bit before retrying
            usleep(200000); // 200ms - increased from 100ms
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
            // Add a small buffer margin to reduce reallocations
            size_t allocation_size = required_size + 1024;
            buffer_pool[empty_slot].buffer = (uint8_t *)safe_malloc(allocation_size);
            
            if (!buffer_pool[empty_slot].buffer) {
                log_error("Failed to allocate buffer for pool (size: %zu, retry %d)", 
                         allocation_size, retry);
                
                // If this is the last retry, give up
                if (retry == config->buffer_allocation_retries - 1) {
                    return NULL;
                }
                
                // Wait a bit before retrying
                usleep(200000); // 200ms - increased from 100ms
                continue;
            }
            
            // Track memory allocation
            track_memory_allocation(allocation_size, true);
            buffer_pool[empty_slot].size = allocation_size;
        }
        
        buffer_pool[empty_slot].in_use = true;
        buffer_pool[empty_slot].last_used = time(NULL);
        active_buffers++;
        log_info("Allocated buffer for pool (index %d, size %zu, retry %d)", 
                empty_slot, buffer_pool[empty_slot].size, retry);
        return buffer_pool[empty_slot].buffer;
    }
    
    log_error("Failed to allocate buffer from pool after %d retries", config->buffer_allocation_retries);
    return NULL;
}

/**
 * Return a buffer to the pool
 */
int return_buffer_to_pool(uint8_t *buffer) {
    if (!buffer) {
        log_warn("Attempted to return NULL buffer to pool");
        return -1;
    }
    
    if (!buffer_pool) {
        log_warn("Buffer pool not initialized when returning buffer");
        // Free the buffer directly to prevent memory leak
        free(buffer);
        return -1;
    }
    
    // Find the buffer in the pool
    for (int i = 0; i < max_buffers; i++) {
        if (buffer_pool[i].buffer == buffer) {
            if (buffer_pool[i].in_use) {
                buffer_pool[i].in_use = false;
                buffer_pool[i].last_used = time(NULL);
                if (active_buffers > 0) {
                    active_buffers--;
                }
                // Log memory tracking
                track_memory_allocation(buffer_pool[i].size, false);
                log_debug("Memory freed: %zu bytes, Total: %zu bytes, Peak: %zu bytes", 
                         buffer_pool[i].size, get_total_memory_allocated(), get_peak_memory_allocated());
                log_info("Returned buffer to pool (index %d, active: %d/%d)", 
                        i, active_buffers, max_buffers);
                return 0;
            } else {
                log_warn("Buffer already marked as not in use (index %d)", i);
                return 0; // Still consider this a success
            }
        }
    }
    
    //  If buffer not found in pool, free it directly to prevent memory leak
    log_error("Buffer %p not found in pool, freeing directly", (void*)buffer);
    free(buffer);
    return -1;
}

/**
 * Emergency cleanup of buffer pool - free all buffers that have been in use for too long
 * This helps recover from situations where buffers aren't properly returned
 */
void emergency_buffer_pool_cleanup(void) {
    if (!buffer_pool) {
        return;
    }
    
    time_t current_time = time(NULL);
    int freed_count = 0;
    
    log_warn("Performing emergency buffer pool cleanup");
    
    // First pass: free buffers that have been in use for too long
    for (int i = 0; i < max_buffers; i++) {
        if (buffer_pool[i].in_use && buffer_pool[i].buffer) {
            // If buffer has been in use for more than 15 seconds (reduced from 30), assume it's leaked
            if (current_time - buffer_pool[i].last_used > 15) {
                log_warn("Freeing leaked buffer (index %d, in use for %ld seconds)",
                        i, current_time - buffer_pool[i].last_used);
                
                // Track memory before freeing
                track_memory_allocation(buffer_pool[i].size, false);
                free(buffer_pool[i].buffer);
                buffer_pool[i].buffer = NULL;
                buffer_pool[i].size = 0;
                buffer_pool[i].in_use = false;
                freed_count++;
                
                if (active_buffers > 0) {
                    active_buffers--;
                }
            }
        }
    }
    
    // Second pass: if we didn't free any buffers, force free the oldest buffer
    if (freed_count == 0 && active_buffers >= max_buffers) {
        time_t oldest_time = current_time;
        int oldest_idx = -1;
        
        // Find the oldest buffer
        for (int i = 0; i < max_buffers; i++) {
            if (buffer_pool[i].buffer && buffer_pool[i].last_used < oldest_time) {
                oldest_time = buffer_pool[i].last_used;
                oldest_idx = i;
            }
        }
        
        // Free the oldest buffer if found
        if (oldest_idx >= 0) {
            log_warn("Force freeing oldest buffer (index %d, last used %ld seconds ago)",
                    oldest_idx, current_time - buffer_pool[oldest_idx].last_used);
            
            // Track memory before freeing
            track_memory_allocation(buffer_pool[oldest_idx].size, false);
            free(buffer_pool[oldest_idx].buffer);
            buffer_pool[oldest_idx].buffer = NULL;
            buffer_pool[oldest_idx].size = 0;
            buffer_pool[oldest_idx].in_use = false;
            freed_count++;
            
            if (active_buffers > 0) {
                active_buffers--;
            }
        }
    }
    
    if (freed_count > 0) {
        log_info("Emergency cleanup freed %d buffers, active buffers now: %d/%d", 
                freed_count, active_buffers, max_buffers);
    } else {
        log_info("Emergency cleanup found no leaked buffers");
    }
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
