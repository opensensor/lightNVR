/**
 * @file api_handlers_system_go2rtc.c
 * @brief Implementation of go2rtc system information retrieval
 */

#include "web/api_handlers_system.h"
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
    
    // Find go2rtc process ID
    FILE *fp = popen("pgrep -f go2rtc | head -1", "r");
    if (!fp) {
        log_error("Failed to execute pgrep command");
        return false;
    }
    
    char pid_str[16] = {0};
    if (!fgets(pid_str, sizeof(pid_str), fp)) {
        log_warn("No go2rtc process found");
        pclose(fp);
        return false;
    }
    
    pclose(fp);
    
    // Remove newline
    pid_str[strcspn(pid_str, "\n")] = 0;
    
    // Get memory usage from /proc/{pid}/status
    char status_path[64];
    snprintf(status_path, sizeof(status_path), "/proc/%s/status", pid_str);
    
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
    
    log_debug("go2rtc memory usage: %llu bytes", *memory_usage);
    return true;
}
