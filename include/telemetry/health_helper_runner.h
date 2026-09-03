/** @file health_helper_runner.h Bounded direct-exec helper process runner. */

#ifndef LIGHTNVR_HEALTH_HELPER_RUNNER_H
#define LIGHTNVR_HEALTH_HELPER_RUNNER_H

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>

#define HEALTH_HELPER_OUTPUT_MAX 4096U
#define HEALTH_HELPER_ABANDONED_MAX 4U

typedef enum {
    HEALTH_HELPER_OK = 0,
    HEALTH_HELPER_EXITED,
    HEALTH_HELPER_TIMED_OUT,
    HEALTH_HELPER_BUSY,
    HEALTH_HELPER_EXEC_ERROR,
    HEALTH_HELPER_SYSTEM_ERROR
} health_helper_outcome_t;

typedef struct {
    const char *program;
    char *const *argv;
    uint32_t timeout_ms;
    uint32_t terminate_grace_ms;
    size_t output_limit;
} health_helper_request_t;

typedef struct {
    health_helper_outcome_t outcome;
    int exit_code;
    int term_signal;
    uint32_t latency_ms;
    bool output_truncated;
    bool abandoned;
    size_t output_length;
    char output[HEALTH_HELPER_OUTPUT_MAX + 1U];
} health_helper_result_t;

/** Execute an absolute program directly (never through a shell). */
int health_helper_run(const health_helper_request_t *request,
                      health_helper_result_t *result);
void health_helper_reap_abandoned(void);
uint32_t health_helper_abandoned_count(void);

#endif /* LIGHTNVR_HEALTH_HELPER_RUNNER_H */
