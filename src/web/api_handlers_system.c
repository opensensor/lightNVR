#define _XOPEN_SOURCE
#define _GNU_SOURCE

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <ctype.h>
#include <unistd.h>
#include <sys/utsname.h>
#include <sys/sysinfo.h>
#include <sys/statvfs.h>
#include <sys/types.h>
#include <sys/stat.h>
#include <sys/socket.h>
#include <sys/ioctl.h>
#include <netdb.h>
#include <ifaddrs.h>
#include <net/if.h>
#include <arpa/inet.h>
#include <fcntl.h>
#include <errno.h>
#include <dirent.h>

#include "web/api_handlers.h"
#include "core/logger.h"
#include "core/config.h"
#include "core/version.h"
#include "core/shutdown_coordinator.h"
#include "video/stream_manager.h"
#include "database/db_streams.h"
#include "database/db_recordings.h"
#include "storage/storage_manager_streams.h"
#include "storage/storage_manager_streams_cache.h"

// External function from api_handlers_system_go2rtc.c
extern bool get_go2rtc_memory_usage(unsigned long long *memory_usage);

// External declarations
extern bool daemon_mode;

/**
 * @brief Get memory usage of light-object-detect process
 *
 * @param memory_usage Pointer to store memory usage in bytes
 * @return true if successful, false otherwise (process not running)
 */
static bool get_detector_memory_usage(unsigned long long *memory_usage) {
    if (!memory_usage) {
        return false;
    }

    // Initialize to 0
    *memory_usage = 0;

    // Find light-object-detect process by scanning /proc (no shell / popen needed)
    pid_t pid = -1;
    pid_t self = getpid();
    DIR *proc_dir = opendir("/proc");
    if (!proc_dir) {
        log_debug("Failed to open /proc for light-object-detect search");
        return false;
    }

    struct dirent *entry;
    while ((entry = readdir(proc_dir)) != NULL) {
        const char *d = entry->d_name;
        if (*d < '1' || *d > '9') continue;
        char *ep;
        pid_t candidate = (pid_t)strtol(d, &ep, 10);
        if (*ep != '\0' || candidate <= 0 || candidate == self) continue;

        char cmdline_path[64];
        snprintf(cmdline_path, sizeof(cmdline_path), "/proc/%d/cmdline", candidate);
        FILE *cf = fopen(cmdline_path, "r");
        if (!cf) continue;

        char cmdline[512] = {0};
        size_t bytes = fread(cmdline, 1, sizeof(cmdline) - 1, cf);
        fclose(cf);

        // cmdline fields are NUL-separated — replace with spaces for strstr
        for (size_t i = 0; i < bytes; i++) {
            if (cmdline[i] == '\0') cmdline[i] = ' ';
        }

        if (strstr(cmdline, "light-object-detect")) {
            pid = candidate;
            break;
        }
    }
    closedir(proc_dir);

    if (pid <= 0) {
        log_debug("No light-object-detect process found");
        return false;
    }

    // Get memory usage from /proc/{pid}/status
    char status_path[64];
    snprintf(status_path, sizeof(status_path), "/proc/%d/status", pid);

    FILE *status_file = fopen(status_path, "r");
    if (!status_file) {
        log_debug("Failed to open %s: %s", status_path, strerror(errno));
        return false;
    }

    char status_line[256];
    unsigned long vm_rss = 0;

    while (fgets(status_line, sizeof(status_line), status_file)) {
        if (strncmp(status_line, "VmRSS:", 6) == 0) {
            // VmRSS is in kB - actual physical memory used
            char *endptr;
            vm_rss = strtoul(status_line + 6, &endptr, 10);
            break;
        }
    }

    fclose(status_file);

    // Convert kB to bytes
    *memory_usage = vm_rss * 1024;

    log_debug("light-object-detect memory usage (PID %d): %llu bytes", pid, *memory_usage);
    return true;
}

// Forward declarations from api_handlers_system_logs.c
extern void handle_get_system_logs(const http_request_t *req, http_response_t *res);
extern void handle_post_system_logs_clear(const http_request_t *req, http_response_t *res);
extern int get_system_logs(char ***logs, int *count);

// ── cgroup-aware resource helpers ──────────────────────────────────────────
// Prefer cgroup limits (container / K8s pod) when available, otherwise fall
// back to host-level syscalls so bare-metal installs keep working.

/**
 * Read a single unsigned long long from a file.  Returns true on success.
 */
static bool read_ull_from_file(const char *path, unsigned long long *out) {
    FILE *fp = fopen(path, "r");
    if (!fp) return false;
    char buf[64] = {0};
    bool ok = false;
    if (fgets(buf, sizeof(buf), fp)) {
        char *endptr;
        *out = strtoull(buf, &endptr, 10);
        ok = (endptr != buf);
    }
    fclose(fp);
    return ok;
}

/**
 * Get the effective number of CPU cores available to this process.
 *
 * Checks cgroup v2 (cpu.max) then cgroup v1 (cpu.cfs_quota_us / period)
 * to derive the fractional CPU limit, rounded up to the nearest integer.
 * Falls back to sysconf(_SC_NPROCESSORS_ONLN) when not cgroup-constrained.
 *
 * Also writes the raw millicores value (0 = unconstrained) for the UI to
 * use if it wants to show "0.5 CPUs" instead of "1 core".
 */
static int get_effective_cpu_cores(int *out_millicores) {
    int host_cores = (int)sysconf(_SC_NPROCESSORS_ONLN);
    if (out_millicores) *out_millicores = 0;

    // ── cgroup v2: /sys/fs/cgroup/cpu.max  ("quota period" or "max period")
    FILE *fp = fopen("/sys/fs/cgroup/cpu.max", "r");
    if (fp) {
        char quota_str[64] = {0};
        unsigned long long period = 0;
        char cpu_max_line[128] = {0};
        int cpu_max_parsed = 0;
        if (fgets(cpu_max_line, sizeof(cpu_max_line), fp)) {
            const char *p = cpu_max_line;
            // Parse first token (quota: "max" or a number)
            while (*p == ' ' || *p == '\t') p++;
            const char *tok_end = p;
            while (*tok_end && *tok_end != ' ' && *tok_end != '\t' && *tok_end != '\n') tok_end++;
            size_t tok_len = (size_t)(tok_end - p);
            if (tok_len > 0 && tok_len < sizeof(quota_str)) {
                memcpy(quota_str, p, tok_len);
                quota_str[tok_len] = '\0';
                p = tok_end;
                while (*p == ' ' || *p == '\t') p++;
                if (*p && *p != '\n') {
                    char *endptr;
                    period = strtoull(p, &endptr, 10);
                    if (endptr != p) cpu_max_parsed = 1;
                }
            }
        }
        if (cpu_max_parsed && period > 0) {
            fclose(fp);
            if (strcmp(quota_str, "max") != 0) {
                // Quota is a number – compute effective cores
                unsigned long long quota = strtoull(quota_str, NULL, 10);
                if (quota > 0) {
                    int millicores = (int)((quota * 1000) / period);
                    if (out_millicores) *out_millicores = millicores;
                    int cores = (int)((quota + period - 1) / period); // ceil
                    log_debug("cgroup v2 cpu.max: quota=%llu period=%llu → %d cores (%dm)",
                              quota, period, cores, millicores);
                    return cores > 0 ? cores : 1;
                }
            }
            // "max" means unlimited – fall through to host value
            log_debug("cgroup v2 cpu.max: unlimited");
            return host_cores;
        }
        if (!cpu_max_parsed) fclose(fp);
    }

    // ── cgroup v1: cpu.cfs_quota_us / cpu.cfs_period_us
    unsigned long long quota = 0, period = 0;
    if (read_ull_from_file("/sys/fs/cgroup/cpu/cpu.cfs_quota_us", &quota) &&
        read_ull_from_file("/sys/fs/cgroup/cpu/cpu.cfs_period_us", &period) &&
        period > 0) {
        if ((long long)quota > 0) {  // -1 means unlimited
            int millicores = (int)((quota * 1000) / period);
            if (out_millicores) *out_millicores = millicores;
            int cores = (int)((quota + period - 1) / period);
            log_debug("cgroup v1 cpu: quota=%llu period=%llu → %d cores (%dm)",
                      quota, period, cores, millicores);
            return cores > 0 ? cores : 1;
        }
    }

    // No cgroup constraint – use host cores
    log_debug("No cgroup CPU limit detected, using host cores: %d", host_cores);
    return host_cores;
}

/**
 * Get the effective memory limit for this process (in bytes).
 *
 * Checks cgroup v2 (memory.max) then cgroup v1 (memory.limit_in_bytes).
 * Falls back to sysinfo() totalram when not cgroup-constrained.
 */
static unsigned long long get_effective_memory_total(void) {
    // Host fallback
    struct sysinfo si;
    unsigned long long host_total = 0;
    if (sysinfo(&si) == 0) {
        host_total = (unsigned long long)si.totalram * si.mem_unit;
    }

    // ── cgroup v2: /sys/fs/cgroup/memory.max  ("max" or a number)
    FILE *fp = fopen("/sys/fs/cgroup/memory.max", "r");
    if (fp) {
        char buf[64] = {0};
        if (fgets(buf, sizeof(buf), fp)) {
            fclose(fp);
            if (strncmp(buf, "max", 3) != 0) {
                unsigned long long limit = strtoull(buf, NULL, 10);
                if (limit > 0 && limit < host_total) {
                    log_debug("cgroup v2 memory.max: %llu bytes", limit);
                    return limit;
                }
            }
            // "max" or value >= host_total means effectively unlimited
            log_debug("cgroup v2 memory.max: unlimited");
            return host_total;
        }
        fclose(fp);
    }

    // ── cgroup v1: /sys/fs/cgroup/memory/memory.limit_in_bytes
    unsigned long long limit = 0;
    if (read_ull_from_file("/sys/fs/cgroup/memory/memory.limit_in_bytes", &limit)) {
        // Very large values (~PAGE_COUNTER_MAX) mean unlimited
        if (limit > 0 && limit < host_total) {
            log_debug("cgroup v1 memory.limit_in_bytes: %llu bytes", limit);
            return limit;
        }
    }

    log_debug("No cgroup memory limit detected, using host total: %llu bytes", host_total);
    return host_total;
}

/**
 * Get the current memory usage for the cgroup (in bytes).
 *
 * In a container the cgroup tracks memory for all processes in the pod,
 * which is more useful than a single process's VmRSS.
 * Falls back to sysinfo() used-ram on bare metal.
 */
static unsigned long long get_effective_memory_used(void) {
    // ── cgroup v2: /sys/fs/cgroup/memory.current
    unsigned long long used = 0;
    if (read_ull_from_file("/sys/fs/cgroup/memory.current", &used) && used > 0) {
        log_debug("cgroup v2 memory.current: %llu bytes", used);
        return used;
    }

    // ── cgroup v1: /sys/fs/cgroup/memory/memory.usage_in_bytes
    if (read_ull_from_file("/sys/fs/cgroup/memory/memory.usage_in_bytes", &used) && used > 0) {
        log_debug("cgroup v1 memory.usage_in_bytes: %llu bytes", used);
        return used;
    }

    // Fall back to host-wide calculation
    struct sysinfo si;
    if (sysinfo(&si) == 0) {
        used = ((unsigned long long)si.totalram - (unsigned long long)si.freeram) * si.mem_unit;
    }
    return used;
}

/**
 * Return true if a real CPU limit is set in cgroups (i.e. we are in a
 * constrained container), false if unlimited / bare-metal.
 */
static bool has_cgroup_cpu_limit(void) {
    // cgroup v2: /sys/fs/cgroup/cpu.max  – "max <period>" means unlimited
    FILE *fp = fopen("/sys/fs/cgroup/cpu.max", "r");
    if (fp) {
        char quota_str[64] = {0};
        char cpu_max_line[128] = {0};
        bool parsed = false;
        if (fgets(cpu_max_line, sizeof(cpu_max_line), fp)) {
            const char *p = cpu_max_line;
            while (*p == ' ' || *p == '\t') p++;
            const char *tok_end = p;
            while (*tok_end && *tok_end != ' ' && *tok_end != '\t' && *tok_end != '\n') tok_end++;
            size_t tok_len = (size_t)(tok_end - p);
            if (tok_len > 0 && tok_len < sizeof(quota_str)) {
                memcpy(quota_str, p, tok_len);
                quota_str[tok_len] = '\0';
                parsed = true;
            }
        }
        fclose(fp);
        if (parsed) {
            return (strcmp(quota_str, "max") != 0);
        }
    }
    // cgroup v1: quota == -1 means unlimited
    unsigned long long quota = 0;
    if (read_ull_from_file("/sys/fs/cgroup/cpu/cpu.cfs_quota_us", &quota)) {
        return ((long long)quota > 0);
    }
    return false;
}

/**
 * Get container-scoped CPU usage percentage.
 *
 * Only uses cgroup cpu.stat / cpuacct.usage when an actual CPU limit is set
 * (i.e. inside a container with resource constraints).  On bare metal the
 * root cgroup tracks all cores combined, which would produce values like
 * 800% on an 8-core box — so we skip straight to /proc/stat there.
 *
 * Returns a value 0.0 – 100.0.
 */
static double get_effective_cpu_usage(void) {
    // Only use cgroup-scoped CPU accounting when a real limit is set;
    // otherwise the root cgroup aggregates all cores and the percentage
    // is meaningless (e.g. 234% on a bare-metal 8-core machine).
    if (has_cgroup_cpu_limit()) {
        // ── cgroup v2: /sys/fs/cgroup/cpu.stat  → usage_usec
        FILE *fp = fopen("/sys/fs/cgroup/cpu.stat", "r");
        if (fp) {
            char line[128];
            unsigned long long usage1 = 0;
            while (fgets(line, sizeof(line), fp)) {
                if (strncmp(line, "usage_usec", 10) == 0) {
                    const char *ptr = line + 10;
                    while (*ptr == ' ') ptr++;
                    char *endptr;
                    usage1 = strtoull(ptr, &endptr, 10);
                    break;
                }
            }
            fclose(fp);

            if (usage1 > 0) {
                // Sleep briefly and sample again
                usleep(100000); // 100 ms
                fp = fopen("/sys/fs/cgroup/cpu.stat", "r");
                if (fp) {
                    unsigned long long usage2 = 0;
                    while (fgets(line, sizeof(line), fp)) {
                        if (strncmp(line, "usage_usec", 10) == 0) {
                            const char *ptr2 = line + 10;
                            while (*ptr2 == ' ') ptr2++;
                            char *endptr2;
                            usage2 = strtoull(ptr2, &endptr2, 10);
                            break;
                        }
                    }
                    fclose(fp);
                    if (usage2 > usage1) {
                        // delta in microseconds over 100ms (100000 us)
                        double delta_us = (double)(usage2 - usage1);
                        double pct = (delta_us / 100000.0) * 100.0;
                        log_debug("cgroup v2 cpu usage: %.1f%%", pct);
                        return pct;
                    }
                }
            }
        }

        // ── cgroup v1: /sys/fs/cgroup/cpuacct/cpuacct.usage (nanoseconds)
        unsigned long long ns1 = 0;
        if (read_ull_from_file("/sys/fs/cgroup/cpuacct/cpuacct.usage", &ns1) && ns1 > 0) {
            usleep(100000);
            unsigned long long ns2 = 0;
            if (read_ull_from_file("/sys/fs/cgroup/cpuacct/cpuacct.usage", &ns2) && ns2 > ns1) {
                double delta_ns = (double)(ns2 - ns1);
                double pct = (delta_ns / 100000000.0) * 100.0; // 100ms = 1e8 ns
                log_debug("cgroup v1 cpu usage: %.1f%%", pct);
                return pct;
            }
        }
    }

    // ── Bare-metal / no CPU limit: /proc/stat  (gives proper 0–100%)
    double cpu_usage = 0.0;
    FILE *fp = fopen("/proc/stat", "r");
    if (fp) {
        char stat_line[256] = {0};
        while (fgets(stat_line, sizeof(stat_line), fp)) {
            if (strncmp(stat_line, "cpu ", 4) == 0) {
                unsigned long user = 0, nice = 0, sys = 0, idle = 0, iowait = 0, irq = 0, softirq = 0;
                char *p = stat_line + 4;
                char *ep;
                int fields = 0;
                unsigned long *targets[] = {&user, &nice, &sys, &idle, &iowait, &irq, &softirq};
                for (int i = 0; i < 7; i++) {
                    while (*p == ' ') p++;
                    *targets[i] = strtoul(p, &ep, 10);
                    if (ep == p) break;
                    p = ep;
                    fields++;
                }
                if (fields == 7) {
                    unsigned long total = user + nice + sys + idle + iowait + irq + softirq;
                    unsigned long active = user + nice + sys + irq + softirq;
                    cpu_usage = (double)active / (double)total * 100.0;
                }
                break;
            }
        }
        fclose(fp);
    }
    return cpu_usage;
}

/**
 * @brief Recursively sum the size of all regular files under a directory.
 *
 * Uses opendir/readdir/lstat exclusively — no shell is invoked, so the path
 * is never interpreted by a shell (prevents command injection).
 * Symbolic links are skipped to prevent directory-loop attacks.
 *
 * @param path   Absolute path of the directory to traverse
 * @param depth  Current recursion depth (caller should pass 0)
 * @return Total size in bytes of all regular files found
 */
static unsigned long long sum_directory_size(const char *path, int depth) {
    if (depth > 64) return 0; /* guard against unexpectedly deep trees */

    DIR *dir = opendir(path);
    if (!dir) return 0;

    unsigned long long total = 0;
    struct dirent *entry;
    char child[4096];
    struct stat st;

    while ((entry = readdir(dir)) != NULL) {
        if (strcmp(entry->d_name, ".") == 0 || strcmp(entry->d_name, "..") == 0)
            continue;

        int n = snprintf(child, sizeof(child), "%s/%s", path, entry->d_name);
        if (n < 0 || n >= (int)sizeof(child))
            continue; /* path too long — skip */

        if (lstat(child, &st) != 0)
            continue;

        if (S_ISREG(st.st_mode)) {
            total += (unsigned long long)st.st_size;
        } else if (S_ISDIR(st.st_mode)) {
            total += sum_directory_size(child, depth + 1);
        }
        /* skip symlinks, device nodes, pipes, etc. */
    }

    closedir(dir);
    return total;
}

/**
 * @brief Get the total byte size of all regular files under a directory.
 *
 * Safe replacement for: popen("du -sb <path> ...", "r")
 *
 * @param path  Absolute path to the directory
 * @return Total size in bytes, or 0 if the path is not a valid directory
 */
static unsigned long long get_directory_size(const char *path) {
    struct stat st;
    if (!path || lstat(path, &st) != 0 || !S_ISDIR(st.st_mode))
        return 0;
    return sum_directory_size(path, 0);
}

/**
 * @brief Direct handler for GET /api/system/info
 */
void handle_get_system_info(const http_request_t *req, http_response_t *res) {
    log_info("Handling GET /api/system/info request");

    // Create JSON object
    cJSON *info = cJSON_CreateObject();
    if (!info) {
        log_error("Failed to create system info JSON object");
        http_response_set_json_error(res, 500, "Failed to create system info JSON");
        return;
    }

    // Add version information
    cJSON_AddStringToObject(info, "version", LIGHTNVR_VERSION_STRING);

    // Get system information
    struct utsname system_info;
    if (uname(&system_info) == 0) {
        // Create CPU object
        cJSON *cpu = cJSON_CreateObject();
        if (cpu) {
            cJSON_AddStringToObject(cpu, "model", system_info.machine);

            // Get CPU cores (cgroup-aware: prefers container limit)
            int millicores = 0;
            int cores = get_effective_cpu_cores(&millicores);
            cJSON_AddNumberToObject(cpu, "cores", cores);

            // Calculate CPU usage (cgroup-aware: prefers container-scoped stats)
            double cpu_usage = get_effective_cpu_usage();
            cJSON_AddNumberToObject(cpu, "usage", cpu_usage);

            // Add CPU object to info
            cJSON_AddItemToObject(info, "cpu", cpu);
        }
    }

    // Get system-wide memory information first (cgroup-aware)
    unsigned long long system_total = get_effective_memory_total();
    unsigned long long system_used  = get_effective_memory_used();
    unsigned long long system_free  = (system_total > system_used) ? (system_total - system_used) : 0;

    // Get memory information for the LightNVR process
    cJSON *memory = cJSON_CreateObject();
    if (memory) {
        // Get process memory usage using /proc/self/status
        FILE *fp = fopen("/proc/self/status", "r");
        unsigned long vm_rss = 0;

        if (fp) {
            char line[256];
            while (fgets(line, sizeof(line), fp)) {
                if (strncmp(line, "VmRSS:", 6) == 0) {
                    // VmRSS is in kB - actual physical memory used
                    char *endptr;
                    vm_rss = strtoul(line + 6, &endptr, 10);
                    break;
                }
            }
            fclose(fp);
        }

        // Convert kB to bytes
        unsigned long long used = vm_rss * 1024;

        // Use the system total memory as the total for LightNVR as well
        // This makes it simpler to understand the memory usage
        unsigned long long total = system_total;

        // Calculate free as the difference between total and used
        unsigned long long free = (total > used) ? (total - used) : 0;

        cJSON_AddNumberToObject(memory, "total", (double)total);
        cJSON_AddNumberToObject(memory, "used", (double)used);
        cJSON_AddNumberToObject(memory, "free", (double)free);

        // Add memory object to info
        cJSON_AddItemToObject(info, "memory", memory);
    }

    // Get memory information for the go2rtc process
    cJSON *go2rtc_memory = cJSON_CreateObject();
    if (go2rtc_memory) {
        unsigned long long go2rtc_used = 0;

        // Try to get go2rtc memory usage
        if (get_go2rtc_memory_usage(&go2rtc_used)) {
            log_debug("go2rtc memory usage: %llu bytes", go2rtc_used);
        } else {
            log_warn("Failed to get go2rtc memory usage, using 0");
        }

        // Use the system total memory as the total for go2rtc as well
        unsigned long long total = system_total;

        // Calculate free as the difference between total and used
        unsigned long long free = (total > go2rtc_used) ? (total - go2rtc_used) : 0;

        cJSON_AddNumberToObject(go2rtc_memory, "total", (double)total);
        cJSON_AddNumberToObject(go2rtc_memory, "used", (double)go2rtc_used);
        cJSON_AddNumberToObject(go2rtc_memory, "free", (double)free);

        // Add go2rtc memory object to info
        cJSON_AddItemToObject(info, "go2rtcMemory", go2rtc_memory);
    }

    // Get memory information for the light-object-detect process
    cJSON *detector_memory = cJSON_CreateObject();
    if (detector_memory) {
        unsigned long long detector_used = 0;

        // Try to get light-object-detect memory usage (returns false if not running)
        if (get_detector_memory_usage(&detector_used)) {
            log_debug("light-object-detect memory usage: %llu bytes", detector_used);
        } else {
            log_debug("light-object-detect not running or memory unavailable, using 0");
        }

        // Use the system total memory as the total for detector as well
        unsigned long long total = system_total;

        // Calculate free as the difference between total and used
        unsigned long long free = (total > detector_used) ? (total - detector_used) : 0;

        cJSON_AddNumberToObject(detector_memory, "total", (double)total);
        cJSON_AddNumberToObject(detector_memory, "used", (double)detector_used);
        cJSON_AddNumberToObject(detector_memory, "free", (double)free);

        // Add detector memory object to info
        cJSON_AddItemToObject(info, "detectorMemory", detector_memory);
    }

    // Get system-wide memory information
    cJSON *system_memory = cJSON_CreateObject();
    if (system_memory) {
        cJSON_AddNumberToObject(system_memory, "total", (double)system_total);
        cJSON_AddNumberToObject(system_memory, "used", (double)system_used);
        cJSON_AddNumberToObject(system_memory, "free", (double)system_free);

        // Add system memory object to info
        cJSON_AddItemToObject(info, "systemMemory", system_memory);
    }

    // Get uptime of the LightNVR process
    // Use /proc/self/stat to get process start time
    FILE *stat_file = fopen("/proc/self/stat", "r");
    if (stat_file) {
        unsigned long long starttime = 0;
        bool stat_ok = false;

        // /proc/self/stat format: pid (comm) state ppid pgrp session tty_nr tpgid
        //   flags minflt cminflt majflt cmajflt utime stime cutime cstime
        //   priority nice num_threads itrealvalue starttime ...
        // comm may contain spaces but is always enclosed in '( )'.
        // Find the last ')' to handle that correctly.
        char stat_line[1024] = {0};
        if (fgets(stat_line, sizeof(stat_line), stat_file)) {
            char *paren_end = strrchr(stat_line, ')');
            if (paren_end) {
                char *p = paren_end + 1;
                // Skip the state field (single non-space char after whitespace)
                while (*p == ' ') p++;
                if (*p && *p != '\n') p++; // skip state
                // Parse 19 numeric fields; starttime is the 20th (index 19)
                for (int i = 0; i < 20; i++) {
                    while (*p == ' ') p++;
                    if (!*p || *p == '\n') break;
                    char *ep;
                    unsigned long long val = strtoull(p, &ep, 10);
                    if (ep == p) break;
                    if (i == 19) { starttime = val; stat_ok = true; }
                    p = ep;
                }
            }
        }
        fclose(stat_file);

        // Get system uptime
        FILE *uptime_file = fopen("/proc/uptime", "r");
        double system_uptime = 0;
        if (uptime_file) {
            char uptime_buf[64] = {0};
            if (fgets(uptime_buf, sizeof(uptime_buf), uptime_file)) {
                char *ep;
                system_uptime = strtod(uptime_buf, &ep);
                if (ep == uptime_buf) system_uptime = 0;
            }
            fclose(uptime_file);
        }

        // Calculate process uptime in seconds only when starttime was read
        // starttime is in clock ticks since system boot
        // Convert to seconds by dividing by sysconf(_SC_CLK_TCK)
        if (stat_ok) {
            long clock_ticks = sysconf(_SC_CLK_TCK);
            double process_uptime = system_uptime - ((double)starttime / (double)clock_ticks);

            // Add process uptime to info
            cJSON_AddNumberToObject(info, "uptime", process_uptime);
        } else {
            // Fallback to system uptime if stat fields couldn't be read
            cJSON_AddNumberToObject(info, "uptime", system_uptime);
        }
    } else {
        // Fallback to system uptime if process uptime can't be determined
        struct sysinfo sys_info;
        if (sysinfo(&sys_info) == 0) {
            cJSON_AddNumberToObject(info, "uptime", (double)sys_info.uptime);
        }
    }

    // Get disk information for the configured storage path
    struct statvfs disk_info;
    if (statvfs(g_config.storage_path, &disk_info) == 0) {
        // Create disk object for LightNVR storage
        cJSON *disk = cJSON_CreateObject();
        if (disk) {
            // Calculate disk values in bytes for consistency
            unsigned long long total = disk_info.f_blocks * disk_info.f_frsize;
            unsigned long long free = disk_info.f_bfree * disk_info.f_frsize;

            // Get actual usage of the storage directory using native filesystem
            // traversal — avoids passing storage_path to a shell command.
            unsigned long long used = get_directory_size(g_config.storage_path);
            if (used == 0) {
                // Fallback to statvfs estimation when directory traversal returns nothing
                used = (disk_info.f_blocks - disk_info.f_bfree) * disk_info.f_frsize;
            }

            cJSON_AddNumberToObject(disk, "total", (double)total);
            cJSON_AddNumberToObject(disk, "used", (double)used);
            cJSON_AddNumberToObject(disk, "free", (double)free);

            // Add disk object to info
            cJSON_AddItemToObject(info, "disk", disk);
        }

        // Create system-wide disk object
        cJSON *system_disk = cJSON_CreateObject();
        if (system_disk) {
            // Get system-wide disk information
            struct statvfs root_disk_info;
            if (statvfs("/", &root_disk_info) == 0) {
                unsigned long long total = root_disk_info.f_blocks * root_disk_info.f_frsize;
                unsigned long long free = root_disk_info.f_bfree * root_disk_info.f_frsize;
                unsigned long long used = total - free;

                cJSON_AddNumberToObject(system_disk, "total", (double)total);
                cJSON_AddNumberToObject(system_disk, "used", (double)used);
                cJSON_AddNumberToObject(system_disk, "free", (double)free);
            }

            // Add system disk object to info
            cJSON_AddItemToObject(info, "systemDisk", system_disk);
        }
    }

    // Add global storage policy settings so the frontend can display effective retention per stream
    cJSON_AddNumberToObject(info, "global_retention_days", g_config.retention_days);

    // Create network object
    cJSON *network = cJSON_CreateObject();
    if (network) {
        // Create interfaces array
        cJSON *interfaces = cJSON_CreateArray();
        if (interfaces) {
            // Get network interfaces with IP addresses using getifaddrs
            struct ifaddrs *ifaddr, *ifa;
            int family;
            char host[NI_MAXHOST];

            if (getifaddrs(&ifaddr) == 0) {
                // Walk through linked list, maintaining head pointer so we can free list later
                for (ifa = ifaddr; ifa != NULL; ifa = ifa->ifa_next) {
                    if (ifa->ifa_addr == NULL)
                        continue;

                    family = ifa->ifa_addr->sa_family;

                    // Skip loopback interface
                    if (strcmp(ifa->ifa_name, "lo") == 0)
                        continue;

                    // Check if this is an IPv4 address
                    if (family == AF_INET) {
                        // Get IP address
                        int s = getnameinfo(ifa->ifa_addr, sizeof(struct sockaddr_in),
                                           host, NI_MAXHOST, NULL, 0, NI_NUMERICHOST);
                        if (s == 0) {
                            // Check if we already have this interface in our array
                            bool found = false;
                            cJSON *existing_iface = NULL;

                            for (int i = 0; i < cJSON_GetArraySize(interfaces); i++) {
                                existing_iface = cJSON_GetArrayItem(interfaces, i);
                                cJSON *name_obj = cJSON_GetObjectItem(existing_iface, "name");
                                if (name_obj && name_obj->valuestring && strcmp(name_obj->valuestring, ifa->ifa_name) == 0) {
                                    found = true;
                                    break;
                                }
                            }

                            if (!found) {
                                // Create new interface object
                                cJSON *iface = cJSON_CreateObject();
                                if (iface) {
                                    cJSON_AddStringToObject(iface, "name", ifa->ifa_name);
                                    cJSON_AddStringToObject(iface, "address", host);

                                    // Get MAC address (simplified)
                                    char mac[128] = "Unknown";
                                    char mac_path[256];
                                    snprintf(mac_path, sizeof(mac_path), "/sys/class/net/%s/address", ifa->ifa_name);
                                    FILE *mac_file = fopen(mac_path, "r");
                                    if (mac_file) {
                                        if (fgets(mac, sizeof(mac), mac_file)) {
                                            // Remove newline
                                            mac[strcspn(mac, "\n")] = 0;
                                        }
                                        fclose(mac_file);
                                    }

                                    cJSON_AddStringToObject(iface, "mac", mac);
                                    cJSON_AddBoolToObject(iface, "up", (ifa->ifa_flags & IFF_UP) != 0);

                                    cJSON_AddItemToArray(interfaces, iface);
                                }
                            }
                        }
                    }
                }

                freeifaddrs(ifaddr);
            } else {
                // Fallback to /proc/net/dev if getifaddrs fails
                FILE *fp = fopen("/proc/net/dev", "r");
                if (fp) {
                    char line[256];
                    // Skip header lines
                    fgets(line, sizeof(line), fp);
                    fgets(line, sizeof(line), fp);

                    // Read interfaces
                    while (fgets(line, sizeof(line), fp)) {
                        char *name = strtok(line, ":");
                        if (name) {
                            // Trim whitespace
                            while (*name == ' ') name++;

                            // Skip loopback
                            if (strcmp(name, "lo") != 0) {
                                cJSON *iface = cJSON_CreateObject();
                                if (iface) {
                                    cJSON_AddStringToObject(iface, "name", name);

                                    // Try to get IPv4 address using ioctl (no shell needed)
                                    char ip_addr[INET_ADDRSTRLEN] = "Unknown";
                                    int ioc_sock = socket(AF_INET, SOCK_DGRAM, 0);
                                    if (ioc_sock >= 0) {
                                        struct ifreq ifr;
                                        memset(&ifr, 0, sizeof(ifr));
                                        strncpy(ifr.ifr_name, name, IFNAMSIZ - 1);
                                        if (ioctl(ioc_sock, SIOCGIFADDR, &ifr) == 0) {
                                            struct sockaddr_in *sin =
                                                (struct sockaddr_in *)&ifr.ifr_addr;
                                            if (inet_ntop(AF_INET, &sin->sin_addr,
                                                          ip_addr, sizeof(ip_addr)) == NULL) {
                                                strncpy(ip_addr, "Unknown", sizeof(ip_addr) - 1);
                                            }
                                        }
                                        close(ioc_sock);
                                    }

                                    cJSON_AddStringToObject(iface, "address", ip_addr);

                                    // Get MAC address
                                    char mac[128] = "Unknown";
                                    char mac_path[256];
                                    snprintf(mac_path, sizeof(mac_path), "/sys/class/net/%s/address", name);
                                    FILE *mac_file = fopen(mac_path, "r");
                                    if (mac_file) {
                                        if (fgets(mac, sizeof(mac), mac_file)) {
                                            // Remove newline
                                            mac[strcspn(mac, "\n")] = 0;
                                        }
                                        fclose(mac_file);
                                    }

                                    cJSON_AddStringToObject(iface, "mac", mac);

                                    // Check if interface is up
                                    char flags_path[256];
                                    snprintf(flags_path, sizeof(flags_path), "/sys/class/net/%s/flags", name);
                                    FILE *flags_file = fopen(flags_path, "r");
                                    bool is_up = false;
                                    if (flags_file) {
                                        char flags_buf[32] = {0};
                                        if (fgets(flags_buf, sizeof(flags_buf), flags_file)) {
                                            char *flags_ep;
                                            unsigned long flags_val = strtoul(flags_buf, &flags_ep, 16);
                                            if (flags_ep != flags_buf) {
                                                is_up = ((unsigned int)flags_val & 1U) != 0; // IFF_UP is 0x1
                                            }
                                        }
                                        fclose(flags_file);
                                    }

                                    cJSON_AddBoolToObject(iface, "up", is_up);

                                    cJSON_AddItemToArray(interfaces, iface);
                                }
                            }
                        }
                    }
                    fclose(fp);
                }
            }

            // Add interfaces array to network object
            cJSON_AddItemToObject(network, "interfaces", interfaces);
        }

        // Add network object to info
        cJSON_AddItemToObject(info, "network", network);
    }

    // Create streams object
    cJSON *streams_obj = cJSON_CreateObject();
    if (streams_obj) {
        // Get count of enabled streams from the database
        int enabled_streams = get_enabled_stream_count();
        log_debug("Enabled streams count from database: %d", enabled_streams);

        cJSON_AddNumberToObject(streams_obj, "active", enabled_streams);
        cJSON_AddNumberToObject(streams_obj, "total", g_config.max_streams);

        // Add streams object to info
        cJSON_AddItemToObject(info, "streams", streams_obj);
    }

    // Create recordings object
    cJSON *recordings = cJSON_CreateObject();
    if (recordings) {
        // Get recordings count from database using the db_recordings function
        int recording_count = 0;

        // Use the get_recording_count function from db_recordings.h
        // Parameters: start_time, end_time, stream_name, has_detection
        // Pass 0 for start_time and end_time to get all recordings
        // Pass NULL for stream_name to get recordings from all streams
        // Pass 0 for has_detection to get all recordings regardless of detection status
        recording_count = get_recording_count(0, 0, NULL, 0, NULL);
        if (recording_count < 0) {
            recording_count = 0; // Reset if query fails
            log_error("Failed to get recording count from database");
        }

        // Get recordings size from storage directory
        unsigned long long recording_size = 0;

        // Compute recordings directory size using native filesystem traversal.
        // storage_path is NEVER passed to a shell command (prevents injection).
        char recordings_dir[512];
        snprintf(recordings_dir, sizeof(recordings_dir), "%s", g_config.storage_path);
        /* strip any trailing slash so lstat/opendir work consistently */
        size_t rd_len = strlen(recordings_dir);
        if (rd_len > 1 && recordings_dir[rd_len - 1] == '/')
            recordings_dir[rd_len - 1] = '\0';

        struct stat st;
        if (stat(recordings_dir, &st) == 0 && S_ISDIR(st.st_mode)) {
            recording_size = get_directory_size(recordings_dir);
            if (recording_size > 0) {
                log_debug("Recordings size from directory traversal: %llu bytes", recording_size);
            } else {
                log_debug("No recordings found or directory is empty: %s", recordings_dir);
            }
        }

        cJSON_AddNumberToObject(recordings, "count", recording_count);
        cJSON_AddNumberToObject(recordings, "size", (double)recording_size);

        // Add recordings object to info
        cJSON_AddItemToObject(info, "recordings", recordings);
    }

    // Add stream storage usage information with caching
    add_cached_stream_storage_usage_to_json(info, 0);

    // Convert to string
    char *json_str = cJSON_PrintUnformatted(info);
    if (!json_str) {
        log_error("Failed to convert system info JSON to string");
        cJSON_Delete(info);
        http_response_set_json_error(res, 500, "Failed to convert system info JSON to string");
        return;
    }

    // Send response
    http_response_set_json(res, 200, json_str);

    // Clean up
    free(json_str);
    cJSON_Delete(info);

    log_info("Successfully handled GET /api/system/info request");
}

// External function from main.c to request a restart
extern void request_restart(void);

/**
 * @brief Direct handler for POST /api/system/restart
 */
void handle_post_system_restart(const http_request_t *req, http_response_t *res) {
    log_info("Handling POST /api/system/restart request");

    // Create success response using cJSON
    cJSON *success = cJSON_CreateObject();
    if (!success) {
        log_error("Failed to create success JSON object");
        http_response_set_json_error(res, 500, "Failed to create success JSON");
        return;
    }

    cJSON_AddBoolToObject(success, "success", true);
    cJSON_AddStringToObject(success, "message", "System is restarting");

    // Convert to string
    char *json_str = cJSON_PrintUnformatted(success);
    if (!json_str) {
        log_error("Failed to convert success JSON to string");
        cJSON_Delete(success);
        http_response_set_json_error(res, 500, "Failed to convert success JSON to string");
        return;
    }

    // Send response
    http_response_set_json(res, 200, json_str);

    // Clean up
    free(json_str);
    cJSON_Delete(success);

    // Log restart
    log_info("System restart requested via API");

    // Request restart - this sets restart_requested flag and running to false
    // After cleanup, main() will re-exec the program
    request_restart();

    log_info("Successfully handled POST /api/system/restart request");
}

/**
 * @brief Direct handler for POST /api/system/shutdown
 */
void handle_post_system_shutdown(const http_request_t *req, http_response_t *res) {
    log_info("Handling POST /api/system/shutdown request");

    // Create success response using cJSON
    cJSON *success = cJSON_CreateObject();
    if (!success) {
        log_error("Failed to create success JSON object");
        http_response_set_json_error(res, 500, "Failed to create success JSON");
        return;
    }

    cJSON_AddBoolToObject(success, "success", true);
    cJSON_AddStringToObject(success, "message", "System is shutting down");

    // Convert to string
    char *json_str = cJSON_PrintUnformatted(success);
    if (!json_str) {
        log_error("Failed to convert success JSON to string");
        cJSON_Delete(success);
        http_response_set_json_error(res, 500, "Failed to convert success JSON to string");
        return;
    }

    // Send response
    http_response_set_json(res, 200, json_str);

    // Clean up
    free(json_str);
    cJSON_Delete(success);

    // Log shutdown
    log_info("System shutdown requested via API");

    // Include shutdown coordinator header
    #include "core/shutdown_coordinator.h"

    // Initiate shutdown through the coordinator first
    log_info("Initiating shutdown through coordinator");
    initiate_shutdown();

    // Schedule shutdown with a more robust approach for MIPS systems
    extern volatile bool running;
    running = false;

    // Set an alarm to force exit if normal shutdown doesn't work
    // This is especially important for Linux 4.4 embedded MIPS systems
    log_info("Setting up fallback exit timer for Linux 4.4 compatibility");
    alarm(15); // Force exit after 15 seconds if normal shutdown fails

    // NOTE: We previously sent SIGTERM to self here, but this was causing system-wide shutdown
    // instead of just application shutdown. This has been removed to fix that issue.
    // The shutdown coordinator and running=false flag are sufficient to trigger application shutdown.

    log_info("Successfully handled POST /api/system/shutdown request");
}

/**
 * @brief Direct handler for POST /api/system/backup
 */
void handle_post_system_backup(const http_request_t *req, http_response_t *res) {
    log_info("Handling POST /api/system/backup request");

    // Create a timestamp for the backup filename
    time_t now = time(NULL);
    struct tm tm_buf;
    struct tm* tm_info = localtime_r(&now, &tm_buf);
    char timestamp[20];
    strftime(timestamp, sizeof(timestamp), "%Y%m%d_%H%M%S", tm_info);

    // Create backup filename
    char backup_filename[256];
    snprintf(backup_filename, sizeof(backup_filename), "lightnvr_backup_%s.json", timestamp);

    // Create backup path in the web root directory
    char backup_path[512];
    snprintf(backup_path, sizeof(backup_path), "%s/backups", g_config.web_root);

    // Create backups directory if it doesn't exist
    mkdir(backup_path, 0755);

    // Append filename to path
    snprintf(backup_path, sizeof(backup_path), "%s/backups/%s", g_config.web_root, backup_filename);

    // Open backup file
    FILE* backup_file = fopen(backup_path, "w");
    if (!backup_file) {
        log_error("Failed to create backup file: %s", strerror(errno));

        // Create error response using cJSON
        cJSON *error = cJSON_CreateObject();
        if (!error) {
            log_error("Failed to create error JSON object");
            http_response_set_json_error(res, 500, "Failed to create error JSON");
            return;
        }

        cJSON_AddBoolToObject(error, "success", false);

        // Create error message with the specific error
        char error_msg[256];
        snprintf(error_msg, sizeof(error_msg), "Failed to create backup: %s", strerror(errno));
        cJSON_AddStringToObject(error, "message", error_msg);

        // Convert to string
        char *json_str = cJSON_PrintUnformatted(error);
        if (!json_str) {
            log_error("Failed to convert error JSON to string");
            cJSON_Delete(error);
            http_response_set_json_error(res, 500, "Failed to convert error JSON to string");
            return;
        }

        // Send response
        http_response_set_json_error(res, 500, json_str);

        // Clean up
        free(json_str);
        cJSON_Delete(error);
        return;
    }

    // Create JSON object for backup
    cJSON *backup = cJSON_CreateObject();
    if (!backup) {
        log_error("Failed to create backup JSON object");
        fclose(backup_file);
        http_response_set_json_error(res, 500, "Failed to create backup JSON");
        return;
    }

    // Add version and timestamp
    cJSON_AddStringToObject(backup, "version", LIGHTNVR_VERSION_STRING);
    cJSON_AddStringToObject(backup, "timestamp", timestamp);

    // Add config object
    cJSON *config = cJSON_CreateObject();
    if (!config) {
        log_error("Failed to create config JSON object");
        cJSON_Delete(backup);
        fclose(backup_file);
        http_response_set_json_error(res, 500, "Failed to create config JSON");
        return;
    }

    // Add config properties
    cJSON_AddNumberToObject(config, "web_port", g_config.web_port);
    cJSON_AddStringToObject(config, "web_root", g_config.web_root);
    cJSON_AddStringToObject(config, "log_file", g_config.log_file);
    cJSON_AddStringToObject(config, "pid_file", g_config.pid_file);
    cJSON_AddStringToObject(config, "db_path", g_config.db_path);
    cJSON_AddStringToObject(config, "storage_path", g_config.storage_path);
    cJSON_AddNumberToObject(config, "max_storage_size", (double)g_config.max_storage_size);
    cJSON_AddNumberToObject(config, "max_streams", g_config.max_streams);

    // Add streams array
    cJSON *streams = cJSON_CreateArray();
    if (!streams) {
        log_error("Failed to create streams JSON array");
        cJSON_Delete(config);
        cJSON_Delete(backup);
        fclose(backup_file);
        http_response_set_json_error(res, 500, "Failed to create streams JSON");
        return;
    }

    // Add streams to array
    for (int i = 0; i < g_config.max_streams; i++) {
        if (g_config.streams[i].name[0] != '\0') {
            cJSON *stream = cJSON_CreateObject();
            if (!stream) {
                log_error("Failed to create stream JSON object");
                continue;
            }

            cJSON_AddStringToObject(stream, "name", g_config.streams[i].name);
            cJSON_AddStringToObject(stream, "url", g_config.streams[i].url);
            cJSON_AddBoolToObject(stream, "enabled", g_config.streams[i].enabled);
            cJSON_AddNumberToObject(stream, "width", g_config.streams[i].width);
            cJSON_AddNumberToObject(stream, "height", g_config.streams[i].height);
            cJSON_AddNumberToObject(stream, "fps", g_config.streams[i].fps);
            cJSON_AddStringToObject(stream, "codec", g_config.streams[i].codec);
            cJSON_AddBoolToObject(stream, "record", g_config.streams[i].record);
            cJSON_AddNumberToObject(stream, "priority", g_config.streams[i].priority);
            cJSON_AddNumberToObject(stream, "segment_duration", g_config.streams[i].segment_duration);

            cJSON_AddItemToArray(streams, stream);
        }
    }

    // Add streams to config
    cJSON_AddItemToObject(config, "streams", streams);

    // Add config to backup
    cJSON_AddItemToObject(backup, "config", config);

    // Convert to string
    char *json_str = cJSON_Print(backup);
    if (!json_str) {
        log_error("Failed to convert backup JSON to string");
        cJSON_Delete(backup);
        fclose(backup_file);
        http_response_set_json_error(res, 500, "Failed to convert backup JSON to string");
        return;
    }

    // Write to file
    fprintf(backup_file, "%s", json_str);

    // Clean up
    free(json_str);
    cJSON_Delete(backup);
    fclose(backup_file);

    log_info("Configuration backup created: %s", backup_path);

    // Create success response with download URL using cJSON
    cJSON *success = cJSON_CreateObject();
    if (!success) {
        log_error("Failed to create success JSON object");
        http_response_set_json_error(res, 500, "Failed to create success JSON");
        return;
    }

    cJSON_AddBoolToObject(success, "success", true);
    cJSON_AddStringToObject(success, "message", "Backup created successfully");

    // Add backup URL and filename
    char backup_url[256];
    snprintf(backup_url, sizeof(backup_url), "/backups/%s", backup_filename);
    cJSON_AddStringToObject(success, "backupUrl", backup_url);
    cJSON_AddStringToObject(success, "filename", backup_filename);

    // Convert to string
    json_str = cJSON_PrintUnformatted(success);
    if (!json_str) {
        log_error("Failed to convert success JSON to string");
        cJSON_Delete(success);
        http_response_set_json_error(res, 500, "Failed to convert success JSON to string");
        return;
    }

    // Send response
    http_response_set_json(res, 200, json_str);

    // Clean up
    free(json_str);
    cJSON_Delete(success);

    log_info("Successfully handled POST /api/system/backup request");
}

/**
 * @brief Direct handler for GET /api/system/status
 */
void handle_get_system_status(const http_request_t *req, http_response_t *res) {
    log_info("Handling GET /api/system/status request");

    // Create status response using cJSON
    cJSON *status = cJSON_CreateObject();
    if (!status) {
        log_error("Failed to create status JSON object");
        http_response_set_json_error(res, 500, "Failed to create status JSON");
        return;
    }

    cJSON_AddStringToObject(status, "status", "ok");
    cJSON_AddStringToObject(status, "message", "System running normally");

    // Convert to string
    char *json_str = cJSON_PrintUnformatted(status);
    if (!json_str) {
        log_error("Failed to convert status JSON to string");
        cJSON_Delete(status);
        http_response_set_json_error(res, 500, "Failed to convert status JSON to string");
        return;
    }

    // Send response
    http_response_set_json(res, 200, json_str);

    // Clean up
    free(json_str);
    cJSON_Delete(status);

    log_info("Successfully handled GET /api/system/status request");
}
