#define _POSIX_C_SOURCE 200809L
#define _XOPEN_SOURCE 700
#define _GNU_SOURCE

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <time.h>
#include <sys/time.h>

#include "core/logger.h"
#include "core/shutdown_coordinator.h"

// Global shutdown coordinator instance
static shutdown_coordinator_t g_coordinator;

// Initialize the shutdown coordinator
int init_shutdown_coordinator(void) {
    memset(&g_coordinator, 0, sizeof(shutdown_coordinator_t));
    atomic_store(&g_coordinator.shutdown_initiated, false);
    atomic_store(&g_coordinator.coordinator_destroyed, false);
    atomic_store(&g_coordinator.component_count, 0);

    // Initialize mutex and condition variable
    if (pthread_mutex_init(&g_coordinator.mutex, NULL) != 0) {
        log_error("Failed to initialize shutdown coordinator mutex");
        return -1;
    }

    if (pthread_cond_init(&g_coordinator.all_stopped_cond, NULL) != 0) {
        log_error("Failed to initialize shutdown coordinator condition variable");
        pthread_mutex_destroy(&g_coordinator.mutex);
        return -1;
    }

    g_coordinator.all_components_stopped = false;

    log_info("Shutdown coordinator initialized");
    return 0;
}

// Shutdown and cleanup the coordinator
void shutdown_coordinator_cleanup(void) {
    // Mark coordinator as destroyed BEFORE destroying mutex
    // This prevents signal handlers from trying to use the mutex
    atomic_store(&g_coordinator.coordinator_destroyed, true);

    // Small memory barrier to ensure the flag is visible to other threads
    __sync_synchronize();

    pthread_mutex_destroy(&g_coordinator.mutex);
    pthread_cond_destroy(&g_coordinator.all_stopped_cond);
    log_info("Shutdown coordinator cleaned up");
}

// Check if coordinator has been destroyed
bool is_coordinator_destroyed(void) {
    return atomic_load(&g_coordinator.coordinator_destroyed);
}

// Register a component with the coordinator
int register_component(const char *name, component_type_t type, void *context, int priority) {
    if (!name) {
        log_error("Cannot register component with NULL name");
        return -1;
    }
    
    pthread_mutex_lock(&g_coordinator.mutex);
    
    // Check if we've reached the maximum number of components
    int count = atomic_load(&g_coordinator.component_count);
    if (count >= MAX_COMPONENTS) {
        log_error("Cannot register component %s: maximum number of components reached", name);
        pthread_mutex_unlock(&g_coordinator.mutex);
        return -1;
    }
    
    // Initialize the component
    component_info_t *component = &g_coordinator.components[count];
    strncpy(component->name, name, sizeof(component->name) - 1);
    component->name[sizeof(component->name) - 1] = '\0';
    component->type = type;
    atomic_store(&component->state, COMPONENT_RUNNING);
    component->context = context;
    component->priority = priority;
    
    // Increment the component count
    atomic_store(&g_coordinator.component_count, count + 1);
    
    pthread_mutex_unlock(&g_coordinator.mutex);
    
    log_info("Registered component %s (ID: %d, type: %d, priority: %d)", 
             name, count, type, priority);
    
    return count;
}

// Update a component's state
void update_component_state(int component_id, component_state_t state) {
    // Check if coordinator has been destroyed
    if (atomic_load(&g_coordinator.coordinator_destroyed)) {
        return;  // Silently ignore updates after coordinator is destroyed
    }

    if (component_id < 0 || component_id >= atomic_load(&g_coordinator.component_count)) {
        log_error("Invalid component ID: %d", component_id);
        return;
    }

    component_info_t *component = &g_coordinator.components[component_id];
    atomic_store(&component->state, state);

    log_info("Updated component %s (ID: %d) state to %d",
             component->name, component_id, state);

    // If the component is now stopped, check if all components are stopped
    if (state == COMPONENT_STOPPED) {
        // Double-check coordinator isn't destroyed before locking mutex
        if (atomic_load(&g_coordinator.coordinator_destroyed)) {
            return;
        }

        pthread_mutex_lock(&g_coordinator.mutex);

        // Check if all components are stopped
        bool all_stopped = true;
        for (int i = 0; i < atomic_load(&g_coordinator.component_count); i++) {
            if (atomic_load(&g_coordinator.components[i].state) != COMPONENT_STOPPED) {
                all_stopped = false;
                break;
            }
        }

        if (all_stopped && !g_coordinator.all_components_stopped) {
            g_coordinator.all_components_stopped = true;
            pthread_cond_broadcast(&g_coordinator.all_stopped_cond);
            log_info("All components are now stopped");
        }

        pthread_mutex_unlock(&g_coordinator.mutex);
    }
}

// Get a component's state
component_state_t get_component_state(int component_id) {
    if (component_id < 0 || component_id >= atomic_load(&g_coordinator.component_count)) {
        log_error("Invalid component ID: %d", component_id);
        return COMPONENT_STOPPED; // Return stopped as a safe default
    }
    
    return atomic_load(&g_coordinator.components[component_id].state);
}

// Initiate shutdown sequence
void initiate_shutdown(void) {
    // Check if coordinator has already been destroyed
    if (atomic_load(&g_coordinator.coordinator_destroyed)) {
        // Coordinator already cleaned up, just return silently
        return;
    }

    // Set the shutdown flag
    atomic_store(&g_coordinator.shutdown_initiated, true);

    log_info("Shutdown sequence initiated");

    // Double-check coordinator isn't destroyed before locking mutex
    if (atomic_load(&g_coordinator.coordinator_destroyed)) {
        return;
    }

    // Signal all components to stop in priority order
    pthread_mutex_lock(&g_coordinator.mutex);
    
    // First, create a sorted list of component IDs by priority
    typedef struct {
        int id;
        int priority;
    } component_priority_t;
    
    component_priority_t priorities[MAX_COMPONENTS];
    int count = atomic_load(&g_coordinator.component_count);
    
    for (int i = 0; i < count; i++) {
        priorities[i].id = i;
        priorities[i].priority = g_coordinator.components[i].priority;
    }
    
    // Sort by priority (higher priority first)
    for (int i = 0; i < count - 1; i++) {
        for (int j = 0; j < count - i - 1; j++) {
            if (priorities[j].priority < priorities[j + 1].priority) {
                component_priority_t temp = priorities[j];
                priorities[j] = priorities[j + 1];
                priorities[j + 1] = temp;
            }
        }
    }
    
    // Now signal components to stop in priority order
    for (int i = 0; i < count; i++) {
        int id = priorities[i].id;
        component_info_t *component = &g_coordinator.components[id];
        
        // Only signal components that are still running
        if (atomic_load(&component->state) == COMPONENT_RUNNING) {
            atomic_store(&component->state, COMPONENT_STOPPING);
            log_info("Signaling component %s (ID: %d, priority: %d) to stop", 
                     component->name, id, component->priority);
        }
    }
    
    pthread_mutex_unlock(&g_coordinator.mutex);
}

// Check if shutdown has been initiated
bool is_shutdown_initiated(void) {
    return atomic_load(&g_coordinator.shutdown_initiated);
}

// Wait for all components to stop (with timeout)
bool wait_for_all_components_stopped(int timeout_seconds) {
    // Check if coordinator has been destroyed
    if (atomic_load(&g_coordinator.coordinator_destroyed)) {
        log_warn("Coordinator already destroyed, cannot wait for components");
        return true;  // Return true to indicate we should proceed
    }

    struct timespec timeout;
    clock_gettime(CLOCK_REALTIME, &timeout);
    timeout.tv_sec += timeout_seconds;

    pthread_mutex_lock(&g_coordinator.mutex);
    
    // If all components are already stopped, return immediately
    if (g_coordinator.all_components_stopped) {
        pthread_mutex_unlock(&g_coordinator.mutex);
        return true;
    }
    
    // First, log which components are still not stopped
    log_info("Waiting for components to stop (timeout: %d seconds):", timeout_seconds);
    for (int i = 0; i < atomic_load(&g_coordinator.component_count); i++) {
        component_state_t state = atomic_load(&g_coordinator.components[i].state);
        if (state != COMPONENT_STOPPED) {
            log_info("Component %s (ID: %d) is in state %d", 
                     g_coordinator.components[i].name, i, state);
        }
    }
    
    // Wait for the condition with timeout
    int result = pthread_cond_timedwait(&g_coordinator.all_stopped_cond, 
                                        &g_coordinator.mutex, &timeout);
    
    bool all_stopped = g_coordinator.all_components_stopped;
    
    // If we timed out, force all components to be marked as stopped
    if (result != 0 || !all_stopped) {
        log_warn("Timeout waiting for all components to stop, forcing all components to stopped state");
        
        // Force all components to be marked as stopped
        for (int i = 0; i < atomic_load(&g_coordinator.component_count); i++) {
            component_state_t state = atomic_load(&g_coordinator.components[i].state);
            if (state != COMPONENT_STOPPED) {
                log_warn("Forcing component %s (ID: %d) from state %d to STOPPED", 
                         g_coordinator.components[i].name, i, state);
                atomic_store(&g_coordinator.components[i].state, COMPONENT_STOPPED);
                
                // If this is a WebSocket component, try to force close it
                if (strstr(g_coordinator.components[i].name, "websocket") != NULL) {
                    log_warn("Forcing close of WebSocket component: %s", g_coordinator.components[i].name);
                    // The context pointer might be a mg_connection, but we can't safely use it
                    // Just mark it as stopped and let the cleanup continue
                }
                
                // If this is an HLS writer component, try to force close it
                if (strstr(g_coordinator.components[i].name, "hls_writer") != NULL ||
                    strstr(g_coordinator.components[i].name, "hls_manager") != NULL) {
                    log_warn("Forcing close of HLS component: %s", g_coordinator.components[i].name);
                    // We can't safely access the context here, just mark it as stopped
                }
            }
        }
        
        // Mark all components as stopped
        g_coordinator.all_components_stopped = true;
        all_stopped = true;
        
        // Signal the condition in case any other threads are waiting
        pthread_cond_broadcast(&g_coordinator.all_stopped_cond);
    }
    
    pthread_mutex_unlock(&g_coordinator.mutex);
    
    if (all_stopped) {
        log_info("All components are now in stopped state");
        return true;
    } else {
        // This should never happen now, but keep as a fallback
        log_error("Failed to mark all components as stopped, this is unexpected");
        
        // Log which components are still not stopped
        pthread_mutex_lock(&g_coordinator.mutex);
        for (int i = 0; i < atomic_load(&g_coordinator.component_count); i++) {
            component_state_t state = atomic_load(&g_coordinator.components[i].state);
            if (state != COMPONENT_STOPPED) {
                log_error("Component %s (ID: %d) is still in state %d", 
                         g_coordinator.components[i].name, i, state);
            }
        }
        pthread_mutex_unlock(&g_coordinator.mutex);
        
        return false;
    }
}

// Get the global shutdown coordinator instance
shutdown_coordinator_t *get_shutdown_coordinator(void) {
    return &g_coordinator;
}
