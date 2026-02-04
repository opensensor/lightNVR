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

// Register a component with the coordinator
// Returns a component ID that can be used to update state
int register_component(const char *name, component_type_t type, void *context, int priority);

// Update a component's state
void update_component_state(int component_id, component_state_t state);

// Get a component's state
component_state_t get_component_state(int component_id);

// Initiate shutdown sequence
void initiate_shutdown(void);

// Check if shutdown has been initiated
bool is_shutdown_initiated(void);

// Check if coordinator has been destroyed (safe to call anytime)
bool is_coordinator_destroyed(void);

// Wait for all components to stop (with timeout)
// Returns true if all components stopped, false if timeout
bool wait_for_all_components_stopped(int timeout_seconds);

// Get the global shutdown coordinator instance
shutdown_coordinator_t *get_shutdown_coordinator(void);

#endif /* SHUTDOWN_COORDINATOR_H */
