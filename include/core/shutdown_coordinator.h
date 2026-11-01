/*
 * SPDX-License-Identifier: GPL-3.0-or-later
 * Copyright (C) 2025-2026 OpenSensor Engineering and LightNVR contributors
 *
 * This file is part of LightNVR <https://github.com/opensensor/lightNVR>.
 */

 #ifndef SHUTDOWN_COORDINATOR_H
#define SHUTDOWN_COORDINATOR_H

#include <stdatomic.h>
#include <stdbool.h>
#include <pthread.h>

// Maximum number of components that can register with the coordinator
#define MAX_COMPONENTS 32

// Component states
typedef enum {
    COMPONENT_RUNNING = 0,
    COMPONENT_STOPPING = 1,
    COMPONENT_STOPPED = 2
} component_state_t;

// Component type
typedef enum {
    COMPONENT_DETECTION_THREAD = 0,
    COMPONENT_SERVER_THREAD = 1,
    COMPONENT_HLS_WRITER = 2,
    COMPONENT_MP4_WRITER = 3,
    COMPONENT_OTHER = 4
} component_type_t;

// Component information
typedef struct {
    char name[64];
    component_type_t type;
    atomic_int state;
    void *context;
    int priority; // Higher priority components are stopped first
    int generation; // Bumped each time the slot is reused (see component IDs)
} component_info_t;

// Shutdown coordinator
typedef struct {
    atomic_bool shutdown_initiated;
    atomic_bool coordinator_destroyed;  // Flag to indicate coordinator has been destroyed
    atomic_int component_count;
    component_info_t components[MAX_COMPONENTS];
    pthread_mutex_t mutex;
    pthread_cond_t all_stopped_cond;
    bool all_components_stopped;
} shutdown_coordinator_t;

// Initialize the shutdown coordinator
int init_shutdown_coordinator(void);

// Shutdown and cleanup the coordinator
void shutdown_coordinator_cleanup(void);

// Register a component with the coordinator.
// Returns an opaque component ID (or -1 on failure) that can be used to update
// state. IDs encode the slot and the slot's generation: STOPPED slots are
// reused, and a reused slot gets a new ID, so a late update carrying the
// previous owner's ID is ignored instead of changing the new owner's state.
int register_component(const char *name, component_type_t type, void *context, int priority);

// Update a component's state. Stale or invalid IDs are ignored.
void update_component_state(int component_id, component_state_t state);

// Get a component's state. Stale IDs (the slot has since been reused) report
// COMPONENT_STOPPED, since the component they referred to has stopped.
component_state_t get_component_state(int component_id);

// Find the current ID of the most recently registered component with this
// name, or -1 if none. Use this instead of indexing components[] directly.
int find_component_id_by_name(const char *name);

// Slot index of a component ID (exposed for tests/diagnostics).
int component_id_slot(int component_id);

// Initiate shutdown sequence
void initiate_shutdown(void);

// Check if shutdown has been initiated
bool is_shutdown_initiated(void);

// Check if coordinator has been destroyed (safe to call anytime)
bool is_coordinator_destroyed(void);

// Wait for all components to stop (with timeout)
// While active, returns true only after all components report stopped. Timeout/wait failure
// returns false without changing live component states; spurious wakes retain
// the original absolute deadline.
bool wait_for_all_components_stopped(int timeout_seconds);

// Get the global shutdown coordinator instance
shutdown_coordinator_t *get_shutdown_coordinator(void);

// Request that long-running background operations (e.g. the scheduled DB
// backup) abort at their next opportunity. Distinct from initiate_shutdown():
// this is a lightweight, independent flag with no component-teardown side
// effects, so it is safe to raise the instant a restart or shutdown is first
// requested -- from an async-signal-safe context (a signal handler) as well
// as from an ordinary API handler thread -- well before the main loop
// actually exits and initiate_shutdown() runs the real teardown sequence.
void request_background_abort(void);

// Check whether a background-abort has been requested. Long-running
// operations that run outside the main loop (e.g. backup_database()'s
// batched copy loop) should poll this between batches and bail out early
// when it becomes true, rather than blocking a pending restart/shutdown
// until the entire operation finishes.
bool is_background_abort_requested(void);

#endif /* SHUTDOWN_COORDINATOR_H */
