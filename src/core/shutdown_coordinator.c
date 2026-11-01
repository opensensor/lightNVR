/*
 * SPDX-License-Identifier: GPL-3.0-or-later
 * Copyright (C) 2025-2026 OpenSensor Engineering and LightNVR contributors
 *
 * This file is part of LightNVR <https://github.com/opensensor/lightNVR>.
 */

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
#include "utils/strings.h"

// Global shutdown coordinator instance
static shutdown_coordinator_t g_coordinator;

// Independent of g_coordinator on purpose -- see header comment. Never reset
// once set: the only callers (request_restart() and the shutdown signal
// handler) set it right before the process is going to stop or re-exec
// anyway, so there is no scenario where it needs to go back to false within
// a process's lifetime.
static atomic_bool g_background_abort_requested = false;

void request_background_abort(void) {
    atomic_store(&g_background_abort_requested, true);
}

bool is_background_abort_requested(void) {
    return atomic_load(&g_background_abort_requested);
}

// The mutex and condition variable are created once and live for the whole
// process. shutdown_coordinator_cleanup() never destroys them (another thread
// may still be about to lock/wait/signal), so init must never re-initialize
// them either: pthread_*_init on a live object is undefined behavior.
// init/cleanup are only called from the main thread (or test setUp/tearDown),
// so a plain bool is sufficient here.
static bool g_sync_initialized = false;

// Reset all non-synchronization state. Caller holds the mutex (or the mutex
// has just been created and no other thread can see it yet).
static void reset_coordinator_state_locked(void) {
    memset(g_coordinator.components, 0, sizeof(g_coordinator.components));
    atomic_store(&g_coordinator.component_count, 0);
    atomic_store(&g_coordinator.shutdown_initiated, false);
    atomic_store(&g_coordinator.coordinator_destroyed, false);

    // Start in the "all stopped" state: nothing has registered yet, so there is
    // nothing to wait for. The first register_component() flips this to false.
    // This prevents wait_for_all_components_stopped() from blocking for the full
    // timeout during shutdown when no components were ever registered.
    g_coordinator.all_components_stopped = true;
}

// Initialize (or re-initialize) the shutdown coordinator
int init_shutdown_coordinator(void) {
    if (!g_sync_initialized) {
        memset(&g_coordinator, 0, sizeof(shutdown_coordinator_t));

        if (pthread_mutex_init(&g_coordinator.mutex, NULL) != 0) {
            log_error("Failed to initialize shutdown coordinator mutex");
            return -1;
        }

        if (pthread_cond_init(&g_coordinator.all_stopped_cond, NULL) != 0) {
            log_error("Failed to initialize shutdown coordinator condition variable");
            pthread_mutex_destroy(&g_coordinator.mutex);
            return -1;
        }

        g_sync_initialized = true;
        reset_coordinator_state_locked();
    } else {
        // Re-init after a previous cleanup: keep the live sync objects and
        // only reset the bookkeeping under the lock.
        pthread_mutex_lock(&g_coordinator.mutex);
        reset_coordinator_state_locked();
        pthread_mutex_unlock(&g_coordinator.mutex);
    }

    log_info("Shutdown coordinator initialized");
    return 0;
}

// Shutdown and cleanup the coordinator
void shutdown_coordinator_cleanup(void) {
    // Atomically claim cleanup ownership to prevent duplicate cleanup work.
    // atomic_exchange returns the previous value.
    if (atomic_exchange(&g_coordinator.coordinator_destroyed, true)) {
        return;  // Already cleaned up (or cleanup in progress/completed)
    }

    // IMPORTANT:
    // Do not destroy the mutex/condition variable here. Other threads may still
    // be about to lock/wait/signal, and destroying these objects concurrently
    // is undefined behavior. They are process-lifetime objects (see
    // g_sync_initialized); a later init_shutdown_coordinator() reuses them.
    if (!g_sync_initialized) {
        return;
    }

    // Wake anyone blocked in wait_for_all_components_stopped() so they observe
    // the destroyed flag instead of sleeping until their full timeout.
    pthread_mutex_lock(&g_coordinator.mutex);
    pthread_cond_broadcast(&g_coordinator.all_stopped_cond);
    pthread_mutex_unlock(&g_coordinator.mutex);

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

    // Refuse late registrations once the coordinator has been cleaned up:
    // nobody will wait for or signal the component any more.
    if (atomic_load(&g_coordinator.coordinator_destroyed)) {
        pthread_mutex_unlock(&g_coordinator.mutex);
        log_warn("Cannot register component %s: shutdown coordinator already cleaned up", name);
        return -1;
    }

    int count = atomic_load(&g_coordinator.component_count);

    // First, try to reuse a STOPPED slot.  Dead recordings mark their slot
    // STOPPED but the old code never freed it, so repeated restart cycles
    // would exhaust all MAX_COMPONENTS slots.  Prefer a slot with the same
    // name (natural for stream restarts), otherwise take any STOPPED slot.
    // Slot reuse is intentional (covered by
    // test_repeated_restart_reuses_stopped_slots): without it a long-running
    // NVR with flapping cameras eventually cannot register new components.
    int reuse_idx = -1;
    for (int i = 0; i < count; i++) {
        if (atomic_load(&g_coordinator.components[i].state) == COMPONENT_STOPPED) {
            if (reuse_idx < 0) {
                reuse_idx = i;  // remember first available STOPPED slot
            }
            if (strcmp(g_coordinator.components[i].name, name) == 0) {
                reuse_idx = i;  // same-name slot is preferred
                break;
            }
        }
    }

    int slot;
    if (reuse_idx >= 0) {
        slot = reuse_idx;
        log_info("Reusing STOPPED component slot %d (was '%s') for new component %s",
                 slot, g_coordinator.components[slot].name, name);
    } else if (count < MAX_COMPONENTS) {
        slot = count;
        atomic_store(&g_coordinator.component_count, count + 1);
    } else {
        log_error("Cannot register component %s: maximum number of components reached", name);
        pthread_mutex_unlock(&g_coordinator.mutex);
        return -1;
    }

    // Initialize the component slot
    component_info_t *component = &g_coordinator.components[slot];
    safe_strcpy(component->name, name, sizeof(component->name), 0);
    component->type = type;
    atomic_store(&component->state, COMPONENT_RUNNING);
    component->context = context;
    component->priority = priority;

    // A live component now exists, so shutdown must wait for it to stop.
    g_coordinator.all_components_stopped = false;

    pthread_mutex_unlock(&g_coordinator.mutex);

    log_info("Registered component %s (ID: %d, type: %d, priority: %d)",
             name, slot, type, priority);

    return slot;
}

// Update a component's state
void update_component_state(int component_id, component_state_t state) {
    // Note: state is recorded even after cleanup. The sync objects are never
    // destroyed, so this is safe, and it keeps get_component_state() and the
    // main.c component scans accurate for components that finish late.
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

    // Signal all components to stop in priority order
    pthread_mutex_lock(&g_coordinator.mutex);
    
    // First, create a sorted list of component IDs by priority
    typedef struct {
        int id;
        int priority;
    } component_priority_t;
    
    component_priority_t priorities[MAX_COMPONENTS];
    memset(priorities, 0, sizeof(priorities));
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
    int component_count = atomic_load(&g_coordinator.component_count);
    log_info("Waiting for %d components to stop (timeout: %d seconds):", component_count, timeout_seconds);
    for (int i = 0; i < component_count; i++) {
        component_state_t state = atomic_load(&g_coordinator.components[i].state);
        if (state != COMPONENT_STOPPED) {
            log_info("Component %s (ID: %d) is in state %d", 
                     g_coordinator.components[i].name, i, state);
        }
    }
    
    // A condition wake is only a hint: retain the original absolute deadline
    // until every component actually reports STOPPED or the wait fails.
    // shutdown_coordinator_cleanup() broadcasts the condition, so also stop
    // waiting once the coordinator has been cleaned up.
    int result = 0;
    while (!g_coordinator.all_components_stopped && result == 0 &&
           !atomic_load(&g_coordinator.coordinator_destroyed)) {
        result = pthread_cond_timedwait(&g_coordinator.all_stopped_cond,
                                       &g_coordinator.mutex, &timeout);
    }
    bool all_stopped = g_coordinator.all_components_stopped;
    bool destroyed = atomic_load(&g_coordinator.coordinator_destroyed);
    pthread_mutex_unlock(&g_coordinator.mutex);

    if (!all_stopped && destroyed) {
        log_warn("Coordinator cleaned up while waiting for components; proceeding");
        return true;  // Same contract as the destroyed pre-check above
    }
    if (!all_stopped) {
        log_warn("Deadline or wait error before all components stopped (wait result: %d)", result);
    }
    return all_stopped;
}

// Get the global shutdown coordinator instance
shutdown_coordinator_t *get_shutdown_coordinator(void) {
    return &g_coordinator;
}
