#ifndef LIGHTNVR_CORE_EVENT_BUS_H
#define LIGHTNVR_CORE_EVENT_BUS_H

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>

#include "core/event_envelope.h"

#define EVENT_BUS_DEFAULT_MAX_EVENTS 1024U
#define EVENT_BUS_DEFAULT_MAX_BYTES (8U * 1024U * 1024U)
#define EVENT_BUS_MAX_SUBSCRIBERS 8
#define EVENT_BUS_SUBSCRIBER_NAME_MAX 48

typedef enum {
    EVENT_BUS_OK = 0,
    EVENT_BUS_NOT_RUNNING = -1,
    EVENT_BUS_INVALID_EVENT = -2,
    EVENT_BUS_QUEUE_FULL = -3,
    EVENT_BUS_ALLOCATION_FAILED = -4
} event_bus_result_t;

typedef int (*event_bus_handler_t)(const event_envelope_t *event,
                                   void *context);

typedef struct {
    uint64_t accepted_events;
    uint64_t dispatched_events;
    uint64_t callback_deliveries;
    uint64_t handler_failures;
    uint64_t dropped_events;
    uint64_t priority_shed_events;
    uint64_t rejected_events;
    size_t queued_events;
    size_t queued_bytes;
    size_t active_dispatches;
    size_t max_events;
    size_t max_bytes;
    int subscriber_count;
    bool running;
} event_bus_stats_t;

/*
 * Register a non-owning callback before the bus starts. Names are unique and
 * make later delivery metrics/debugging stable. The callback and context must
 * remain valid until after shutdown or unsubscription.
 */
int event_bus_subscribe(const char *name, event_bus_handler_t handler,
                        void *context);
int event_bus_unsubscribe(const char *name);

/* Zero limits select the documented bounded defaults. */
int event_bus_init(size_t max_events, size_t max_bytes);

/*
 * Validate and enqueue a deep copy. No subscriber callback or network I/O runs
 * on the caller thread. The caller retains ownership of event.
 */
event_bus_result_t event_bus_publish(const event_envelope_t *event,
                                     char *error, size_t error_size);

/* Wait until the queue and current callback dispatch are empty. */
int event_bus_wait_until_idle(unsigned int timeout_ms);

void event_bus_get_stats(event_bus_stats_t *stats);

/* Stop accepting events. drain=true dispatches queued events before joining. */
void event_bus_shutdown(bool drain);

#endif /* LIGHTNVR_CORE_EVENT_BUS_H */
