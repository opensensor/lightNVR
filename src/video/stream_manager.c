#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <pthread.h>
#include <unistd.h>
#include <time.h>

#include "video/stream_manager.h"
#include "core/logger.h"
#include "storage/storage_manager.h"

// Stream handle structure
struct stream_handle_s {
    int id;
    stream_config_t config;
    stream_status_t status;
    stream_stats_t stats;
    pthread_t thread;
    bool thread_running;
    void *recording_handle;
};

// Stream manager state
static struct {
    int max_streams;
    int active_streams;
    int total_streams;
    struct stream_handle_s *streams;
    pthread_mutex_t mutex;
    uint64_t used_memory;
    uint64_t peak_memory;
} stream_manager = {
    .max_streams = 0,
    .active_streams = 0,
    .total_streams = 0,
    .streams = NULL,
    .used_memory = 0,
    .peak_memory = 0
};

// Initialize the stream manager
int init_stream_manager(int max_streams) {
    if (max_streams <= 0) {
        log_error("Invalid maximum streams: %d", max_streams);
        return -1;
    }
    
    // Initialize mutex
    if (pthread_mutex_init(&stream_manager.mutex, NULL) != 0) {
        log_error("Failed to initialize stream manager mutex");
        return -1;
    }
    
    // Allocate streams array
    stream_manager.streams = calloc(max_streams, sizeof(struct stream_handle_s));
    if (!stream_manager.streams) {
        log_error("Failed to allocate memory for streams");
        pthread_mutex_destroy(&stream_manager.mutex);
        return -1;
    }
    
    stream_manager.max_streams = max_streams;
    stream_manager.used_memory = max_streams * sizeof(struct stream_handle_s);
    stream_manager.peak_memory = stream_manager.used_memory;
    
    log_info("Stream manager initialized with max streams: %d", max_streams);
    return 0;
}

// Shutdown the stream manager
void shutdown_stream_manager(void) {
    pthread_mutex_lock(&stream_manager.mutex);
    
    // Stop all streams
    for (int i = 0; i < stream_manager.max_streams; i++) {
        if (stream_manager.streams[i].thread_running) {
            // Signal thread to stop
            stream_manager.streams[i].thread_running = false;
            
            // Unlock mutex while waiting for thread to exit
            pthread_mutex_unlock(&stream_manager.mutex);
            pthread_join(stream_manager.streams[i].thread, NULL);
            pthread_mutex_lock(&stream_manager.mutex);
        }
    }
    
    // Free streams array
    free(stream_manager.streams);
    stream_manager.streams = NULL;
    
    pthread_mutex_unlock(&stream_manager.mutex);
    
    // Destroy mutex
    pthread_mutex_destroy(&stream_manager.mutex);
    
    log_info("Stream manager shutdown");
}

// Add a new stream
stream_handle_t add_stream(const stream_config_t *config) {
    if (!config) {
        log_error("Invalid stream configuration");
        return NULL;
    }
    
    pthread_mutex_lock(&stream_manager.mutex);
    
    // Find an empty slot
    int slot = -1;
    for (int i = 0; i < stream_manager.max_streams; i++) {
        if (stream_manager.streams[i].id == 0) {
            slot = i;
            break;
        }
    }
    
    if (slot < 0) {
        log_error("No available stream slots");
        pthread_mutex_unlock(&stream_manager.mutex);
        return NULL;
    }
    
    // Initialize stream
    stream_manager.streams[slot].id = slot + 1;
    stream_manager.streams[slot].config = *config;
    stream_manager.streams[slot].status = STREAM_STATUS_STOPPED;
    memset(&stream_manager.streams[slot].stats, 0, sizeof(stream_stats_t));
    stream_manager.streams[slot].thread_running = false;
    stream_manager.streams[slot].recording_handle = NULL;
    
    stream_manager.total_streams++;
    
    pthread_mutex_unlock(&stream_manager.mutex);

    // After successfully adding the stream, update global config
    extern config_t global_config;
    for (int i = 0; i < global_config.max_streams; i++) {
        if (global_config.streams[i].name[0] == '\0') {
            // Found an empty slot
            memcpy(&global_config.streams[i], config, sizeof(stream_config_t));
            break;
        }
    }
    
    log_info("Added stream: %s", config->name);
    return &stream_manager.streams[slot];
}

// Start a stream
int start_stream(stream_handle_t handle) {
    // Stub implementation
    if (!handle) {
        return -1;
    }
    
    log_info("Starting stream: %s", handle->config.name);
    handle->status = STREAM_STATUS_RUNNING;
    return 0;
}

// Stop a stream
int stop_stream(stream_handle_t handle) {
    // Stub implementation
    if (!handle) {
        return -1;
    }
    
    log_info("Stopping stream: %s", handle->config.name);
    handle->status = STREAM_STATUS_STOPPED;
    return 0;
}

// Get stream status
stream_status_t get_stream_status(stream_handle_t handle) {
    if (!handle) {
        return STREAM_STATUS_ERROR;
    }
    
    return handle->status;
}

// Get stream statistics
int get_stream_stats(stream_handle_t handle, stream_stats_t *stats) {
    if (!handle || !stats) {
        return -1;
    }
    
    *stats = handle->stats;
    return 0;
}

// Get stream configuration
int get_stream_config(stream_handle_t handle, stream_config_t *config) {
    if (!handle || !config) {
        return -1;
    }
    
    *config = handle->config;
    return 0;
}

// In src/video/stream_manager.c

// Update stream configuration
int update_stream_config(stream_handle_t handle, const stream_config_t *config) {
    if (!handle || !config) {
        log_error("Invalid parameters for update_stream_config");
        return -1;
    }

    pthread_mutex_lock(&stream_manager.mutex);

    // Find the stream slot index
    int slot = handle->id - 1;
    if (slot < 0 || slot >= stream_manager.max_streams) {
        log_error("Invalid stream handle in update_stream_config");
        pthread_mutex_unlock(&stream_manager.mutex);
        return -1;
    }

    // Store old name for comparison
    char old_name[MAX_STREAM_NAME];
    strncpy(old_name, handle->config.name, MAX_STREAM_NAME - 1);
    old_name[MAX_STREAM_NAME - 1] = '\0';

    // Update the stream configuration
    handle->config = *config;

    // If the stream name changed, update any internal references
    if (strcmp(old_name, config->name) != 0) {
        log_info("Stream name changed from '%s' to '%s'", old_name, config->name);

        // Update any internal mappings or references here if needed
        // For example, if you maintain a name-to-stream map
    }

    pthread_mutex_unlock(&stream_manager.mutex);

    // Update the global configuration to keep it in sync
    extern config_t global_config;
    bool config_updated = false;

    // First look for the existing entry with the old name
    for (int i = 0; i < global_config.max_streams; i++) {
        if (strcmp(global_config.streams[i].name, old_name) == 0) {
            // Found the entry, update it
            memcpy(&global_config.streams[i], config, sizeof(stream_config_t));
            config_updated = true;
            break;
        }
    }

    // If we didn't find an entry with the old name (rare case, possible inconsistency)
    // Look for an empty slot to add the new config
    if (!config_updated) {
        for (int i = 0; i < global_config.max_streams; i++) {
            if (global_config.streams[i].name[0] == '\0') {
                memcpy(&global_config.streams[i], config, sizeof(stream_config_t));
                config_updated = true;
                break;
            }
        }
    }

    if (!config_updated) {
        log_warn("Could not update stream '%s' in global config - no matching entry or free slot found", config->name);
    } else {
        // Save the updated configuration to file
        // This ensures changes are persisted immediately and not lost if the service crashes
        if (save_config(&global_config, "/etc/lightnvr/lightnvr.conf") != 0) {
            log_warn("Failed to save configuration after updating stream");
        }
    }

    log_info("Stream configuration updated: %s", config->name);
    return 0;
}

// Remove a stream
int remove_stream(stream_handle_t handle) {
    if (!handle) {
        log_error("Invalid stream handle in remove_stream");
        return -1;
    }

    pthread_mutex_lock(&stream_manager.mutex);

    // Check if stream is valid
    int slot = handle->id - 1;
    if (slot < 0 || slot >= stream_manager.max_streams || stream_manager.streams[slot].id == 0) {
        pthread_mutex_unlock(&stream_manager.mutex);
        log_error("Invalid stream slot in remove_stream: %d", slot);
        return -1;
    }

    // Get stream name for logging and config updates
    char stream_name[MAX_STREAM_NAME];
    strncpy(stream_name, handle->config.name, MAX_STREAM_NAME - 1);
    stream_name[MAX_STREAM_NAME - 1] = '\0';

    // Stop stream if running
    if (stream_manager.streams[slot].thread_running) {
        // Signal thread to stop
        stream_manager.streams[slot].thread_running = false;

        // Unlock mutex while waiting for thread to exit
        pthread_mutex_unlock(&stream_manager.mutex);
        pthread_join(stream_manager.streams[slot].thread, NULL);
        pthread_mutex_lock(&stream_manager.mutex);
    }

    // Clean up any associated resources
    if (stream_manager.streams[slot].recording_handle) {
        // Close recording if active
        // In a real implementation, you would call your close_recording function here
        stream_manager.streams[slot].recording_handle = NULL;
    }

    // Clear stream slot
    memset(&stream_manager.streams[slot], 0, sizeof(struct stream_handle_s));

    // Update stream count
    stream_manager.total_streams--;
    if (stream_manager.active_streams > 0) {
        stream_manager.active_streams--;
    }

    pthread_mutex_unlock(&stream_manager.mutex);

    // Update the global configuration by removing the stream entry
    extern config_t global_config;
    bool found = false;

    for (int i = 0; i < global_config.max_streams; i++) {
        if (strcmp(global_config.streams[i].name, stream_name) == 0) {
            // Found the stream to remove - clear its slot
            memset(&global_config.streams[i], 0, sizeof(stream_config_t));
            found = true;
            break;
        }
    }

    if (!found) {
        log_warn("Stream '%s' not found in global config during removal", stream_name);
    } else {
        // Save configuration to persist the removal
        if (save_config(&global_config, "/etc/lightnvr/lightnvr.conf") != 0) {
            log_warn("Failed to save configuration after removing stream");
        }
    }

    log_info("Stream removed: %s", stream_name);
    return 0;
}

// Get stream by index
stream_handle_t get_stream_by_index(int index) {
    if (index < 0 || index >= stream_manager.max_streams) {
        return NULL;
    }
    
    pthread_mutex_lock(&stream_manager.mutex);
    
    stream_handle_t handle = NULL;
    if (stream_manager.streams[index].id != 0) {
        handle = &stream_manager.streams[index];
    }
    
    pthread_mutex_unlock(&stream_manager.mutex);
    
    return handle;
}

// Get stream by name
stream_handle_t get_stream_by_name(const char *name) {
    if (!name) {
        return NULL;
    }
    
    pthread_mutex_lock(&stream_manager.mutex);
    
    stream_handle_t handle = NULL;
    for (int i = 0; i < stream_manager.max_streams; i++) {
        if (stream_manager.streams[i].id != 0 && 
            strcmp(stream_manager.streams[i].config.name, name) == 0) {
            handle = &stream_manager.streams[i];
            break;
        }
    }
    
    pthread_mutex_unlock(&stream_manager.mutex);
    
    return handle;
}

// Get number of active streams
int get_active_stream_count(void) {
    return stream_manager.active_streams;
}

// Get total number of streams
int get_total_stream_count(void) {
    return stream_manager.total_streams;
}

// Set stream priority
int set_stream_priority(stream_handle_t handle, int priority) {
    if (!handle || priority < 1 || priority > 10) {
        return -1;
    }
    
    handle->config.priority = priority;
    return 0;
}

// Enable/disable recording for a stream
int set_stream_recording(stream_handle_t handle, bool enable) {
    if (!handle) {
        return -1;
    }
    
    handle->config.record = enable;
    return 0;
}

// Get a snapshot from the stream
int get_stream_snapshot(stream_handle_t handle, const char *path) {
    // Stub implementation
    return -1;
}

// Get memory usage statistics for the stream manager
int get_stream_manager_memory_usage(uint64_t *used_memory, uint64_t *peak_memory) {
    if (!used_memory || !peak_memory) {
        return -1;
    }
    
    *used_memory = stream_manager.used_memory;
    *peak_memory = stream_manager.peak_memory;
    
    return 0;
}
