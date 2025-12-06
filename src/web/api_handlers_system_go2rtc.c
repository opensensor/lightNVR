/**
 * @file api_handlers_system_go2rtc.c
 * @brief Implementation of go2rtc system information retrieval
 */

#include "web/api_handlers_system.h"
#include "video/go2rtc/go2rtc_process.h"
#include "core/logger.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <sys/types.h>
#include <sys/stat.h>
#include <fcntl.h>
#include <errno.h>

/**
 * @brief Get memory usage of go2rtc process
 *
 * @param memory_usage Pointer to store memory usage in bytes
 * @return true if successful, false otherwise
 */
bool get_go2rtc_memory_usage(unsigned long long *memory_usage) {
    if (!memory_usage) {
        return false;
    }

    // Initialize to 0
    *memory_usage = 0;

    // Get go2rtc process ID from the process manager
    // This is more reliable than pgrep as it tracks the actual process we started
    int pid = go2rtc_process_get_pid();
    if (pid <= 0) {
        log_warn("No go2rtc process found (PID: %d)", pid);
        return false;
    }

    // Get memory usage from /proc/{pid}/status
    char status_path[64];
    snprintf(status_path, sizeof(status_path), "/proc/%d/status", pid);

    FILE *status_file = fopen(status_path, "r");
    if (!status_file) {
        log_error("Failed to open %s: %s", status_path, strerror(errno));
        return false;
    }

    char line[256];
    unsigned long vm_rss = 0;

    while (fgets(line, sizeof(line), status_file)) {
        if (strncmp(line, "VmRSS:", 6) == 0) {
            // VmRSS is in kB - actual physical memory used
            sscanf(line + 6, "%lu", &vm_rss);
            break;
        }
    }

    fclose(status_file);

    // Convert kB to bytes
    *memory_usage = vm_rss * 1024;

    log_debug("go2rtc memory usage (PID %d): %llu bytes", pid, *memory_usage);
    return true;
}
