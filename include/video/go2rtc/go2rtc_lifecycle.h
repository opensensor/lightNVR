/**
 * @file go2rtc_lifecycle.h
 * @brief Serialization for the shared go2rtc process lifecycle.
 */

#ifndef GO2RTC_LIFECYCLE_H
#define GO2RTC_LIFECYCLE_H

#include <stdbool.h>
#include <stdint.h>

typedef enum {
    GO2RTC_LIFECYCLE_CHECK = 0,
    GO2RTC_LIFECYCLE_PROCESS_START,
    GO2RTC_LIFECYCLE_PROCESS_STOP,
    GO2RTC_LIFECYCLE_FULL_START,
    GO2RTC_LIFECYCLE_RESTART,
    GO2RTC_LIFECYCLE_RECONFIGURE,
    GO2RTC_LIFECYCLE_CLEANUP,
    GO2RTC_LIFECYCLE_OPERATION_COUNT
} go2rtc_lifecycle_operation_t;

typedef struct {
    bool acquired;
    bool outermost;
    bool coalesced;
    bool result;
    uint64_t generation;
    uint64_t restart_generation;
} go2rtc_lifecycle_guard_t;

/**
 * Enter the single go2rtc lifecycle coordinator.
 *
 * Calls from the current owner are re-entrant. Other threads wait until the
 * active operation completes. When @p coalesce is true, a caller requesting
 * the same operation as the active owner waits for that owner and receives
 * its result instead of repeating the operation.
 */
bool go2rtc_lifecycle_begin(go2rtc_lifecycle_operation_t operation,
                            bool coalesce,
                            bool intentional,
                            go2rtc_lifecycle_guard_t *guard);

/** Complete an acquired lifecycle operation. */
void go2rtc_lifecycle_end(go2rtc_lifecycle_guard_t *guard, bool result);

/** True while an intentional global restart owns the lifecycle. */
bool go2rtc_lifecycle_intentional_restart_active(void);

/** Current completed-operation generation, useful for diagnostics/tests. */
uint64_t go2rtc_lifecycle_generation(void);

/** Generation incremented only when a global start/restart completes. */
uint64_t go2rtc_lifecycle_restart_generation(void);

#endif /* GO2RTC_LIFECYCLE_H */
