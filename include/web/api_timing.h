#ifndef LIGHTNVR_WEB_API_TIMING_H
#define LIGHTNVR_WEB_API_TIMING_H

#include <time.h>
#include "core/logger.h"
#include "web/request_response.h"

/* Debug logs identify a phase that never returns; warning logs identify slow
 * completed phases without requiring debug logging on the running server. */
typedef struct {
    struct timespec started;
    const http_request_t *request;
    const char *phase;
    bool valid;
} api_phase_timer_t;

static inline api_phase_timer_t api_phase_begin(
    const http_request_t *request, const char *phase) {
    api_phase_timer_t timer = { .request = request, .phase = phase };
    timer.valid = clock_gettime(CLOCK_MONOTONIC, &timer.started) == 0;
    log_debug("API phase starting: path=%s phase=%s request_id=%s",
              request->path, phase,
              request->request_id[0] ? request->request_id : "-");
    return timer;
}

static inline void api_phase_end(const api_phase_timer_t *timer) {
    struct timespec now;
    if (!timer->valid || clock_gettime(CLOCK_MONOTONIC, &now) != 0) return;
    double elapsed_ms = (now.tv_sec - timer->started.tv_sec) * 1000.0 +
        (now.tv_nsec - timer->started.tv_nsec) / 1000000.0;
    if (elapsed_ms >= 250.0) {
        log_warn("Slow API phase: path=%s phase=%s duration_ms=%.0f request_id=%s",
                 timer->request->path, timer->phase, elapsed_ms,
                 timer->request->request_id[0] ? timer->request->request_id : "-");
    }
}

#endif /* LIGHTNVR_WEB_API_TIMING_H */
