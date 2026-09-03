/** @file linux_proc.h Bounded parsers and collector for Linux /proc health. */

#ifndef LIGHTNVR_TELEMETRY_COLLECTORS_LINUX_PROC_H
#define LIGHTNVR_TELEMETRY_COLLECTORS_LINUX_PROC_H

#include <stdbool.h>
#include <stdint.h>

#include "telemetry/system_health_collector.h"

typedef struct {
    uint64_t user;
    uint64_t nice;
    uint64_t system;
    uint64_t idle;
    uint64_t iowait;
    uint64_t irq;
    uint64_t softirq;
    uint64_t steal;
} linux_proc_cpu_times_t;

typedef struct {
    double one;
    double five;
    double fifteen;
} linux_proc_loadavg_t;

typedef struct {
    uint64_t total_bytes;
    uint64_t available_bytes;
    uint64_t swap_total_bytes;
    uint64_t swap_free_bytes;
} linux_proc_memory_t;

typedef struct {
    uint64_t major_faults;
    uint64_t swap_in_pages;
    uint64_t swap_out_pages;
} linux_proc_vmstat_t;

typedef struct {
    bool some_present;
    bool full_present;
    double some_avg10_ratio;
    double full_avg10_ratio;
    uint64_t some_total_usec;
    uint64_t full_total_usec;
} linux_proc_pressure_t;

typedef struct {
    bool cpu_valid;
    linux_proc_cpu_times_t cpu;
    uint64_t cpu_monotonic_ms;
    bool vmstat_valid;
    linux_proc_vmstat_t vmstat;
    uint64_t vmstat_monotonic_ms;
    bool pressure_valid[3];
    linux_proc_pressure_t pressure[3];
    uint64_t pressure_monotonic_ms[3];
} linux_proc_state_t;

int linux_proc_parse_cpu_stat(const char *text, linux_proc_cpu_times_t *out);
int linux_proc_parse_loadavg(const char *text, linux_proc_loadavg_t *out);
int linux_proc_parse_meminfo(const char *text, linux_proc_memory_t *out);
int linux_proc_parse_vmstat(const char *text, linux_proc_vmstat_t *out);
int linux_proc_parse_pressure(const char *text, linux_proc_pressure_t *out);

void linux_proc_state_init(linux_proc_state_t *state);
bool linux_proc_collector_init(system_health_collector_t *collector,
                               linux_proc_state_t *state);

#endif /* LIGHTNVR_TELEMETRY_COLLECTORS_LINUX_PROC_H */
