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
#include <sys/socket.h>
#include <netdb.h>
#include <ifaddrs.h>
#include <net/if.h>
#include <arpa/inet.h>
#include <fcntl.h>
#include <errno.h>

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
#include "mongoose.h"

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

    // Find light-object-detect process using pgrep
    FILE *fp = popen("pgrep -f 'light-object-detect' 2>/dev/null | head -1", "r");
    if (!fp) {
        log_debug("Failed to execute pgrep for light-object-detect");
        return false;
    }

    char line[32];
    pid_t pid = -1;
    if (fgets(line, sizeof(line), fp)) {
        pid = atoi(line);
    }
    pclose(fp);

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
            sscanf(status_line + 6, "%lu", &vm_rss);
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
extern void mg_handle_get_system_logs(struct mg_connection *c, struct mg_http_message *hm);
extern void mg_handle_post_system_logs_clear(struct mg_connection *c, struct mg_http_message *hm);
extern int get_system_logs(char ***logs, int *count);

/**
 * @brief Direct handler for GET /api/system/info
 */
void mg_handle_get_system_info(struct mg_connection *c, struct mg_http_message *hm) {
    log_info("Handling GET /api/system/info request");

    // Create JSON object
    cJSON *info = cJSON_CreateObject();
    if (!info) {
        log_error("Failed to create system info JSON object");
        mg_send_json_error(c, 500, "Failed to create system info JSON");
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

            // Get CPU cores
            int cores = sysconf(_SC_NPROCESSORS_ONLN);
            cJSON_AddNumberToObject(cpu, "cores", cores);

            // Calculate CPU usage (simplified)
            double cpu_usage = 0.0;
            FILE *fp = fopen("/proc/stat", "r");
            if (fp) {
                unsigned long user, nice, system, idle, iowait, irq, softirq;
                if (fscanf(fp, "cpu %lu %lu %lu %lu %lu %lu %lu",
                          &user, &nice, &system, &idle, &iowait, &irq, &softirq) == 7) {
                    unsigned long total = user + nice + system + idle + iowait + irq + softirq;
                    unsigned long active = user + nice + system + irq + softirq;
                    cpu_usage = (double)active / (double)total * 100.0;
                }
                fclose(fp);
            }
            cJSON_AddNumberToObject(cpu, "usage", cpu_usage);

            // Add CPU object to info
            cJSON_AddItemToObject(info, "cpu", cpu);
        }
    }

    // Get system-wide memory information first
    struct sysinfo sys_info;
    unsigned long long system_total = 0;
    unsigned long long system_free = 0;
    unsigned long long system_used = 0;

    if (sysinfo(&sys_info) == 0) {
        // Calculate memory values in bytes
        system_total = sys_info.totalram * sys_info.mem_unit;
        system_free = sys_info.freeram * sys_info.mem_unit;
        system_used = system_total - system_free;
    }

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
                    sscanf(line + 6, "%lu", &vm_rss);
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

        cJSON_AddNumberToObject(memory, "total", total);
        cJSON_AddNumberToObject(memory, "used", used);
        cJSON_AddNumberToObject(memory, "free", free);

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

        cJSON_AddNumberToObject(go2rtc_memory, "total", total);
        cJSON_AddNumberToObject(go2rtc_memory, "used", go2rtc_used);
        cJSON_AddNumberToObject(go2rtc_memory, "free", free);

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

        cJSON_AddNumberToObject(detector_memory, "total", total);
        cJSON_AddNumberToObject(detector_memory, "used", detector_used);
        cJSON_AddNumberToObject(detector_memory, "free", free);

        // Add detector memory object to info
        cJSON_AddItemToObject(info, "detectorMemory", detector_memory);
    }

    // Get system-wide memory information
    cJSON *system_memory = cJSON_CreateObject();
    if (system_memory) {
        cJSON_AddNumberToObject(system_memory, "total", system_total);
        cJSON_AddNumberToObject(system_memory, "used", system_used);
        cJSON_AddNumberToObject(system_memory, "free", system_free);

        // Add system memory object to info
        cJSON_AddItemToObject(info, "systemMemory", system_memory);
    }

    // Get uptime of the LightNVR process
    // Use /proc/self/stat to get process start time
    FILE *stat_file = fopen("/proc/self/stat", "r");
    if (stat_file) {
        // Fields in /proc/self/stat
        char comm[256];
        char state;
        int ppid, pgrp, session, tty_nr, tpgid;
        unsigned long flags, minflt, cminflt, majflt, cmajflt, utime, stime;
        long cutime, cstime, priority, nice, num_threads, itrealvalue;
        unsigned long long starttime;

        // Read the stat file
        fscanf(stat_file, "%*d %s %c %d %d %d %d %d %lu %lu %lu %lu %lu %lu %lu %ld %ld %ld %ld %ld %ld %llu",
               comm, &state, &ppid, &pgrp, &session, &tty_nr, &tpgid,
               &flags, &minflt, &cminflt, &majflt, &cmajflt, &utime, &stime,
               &cutime, &cstime, &priority, &nice, &num_threads, &itrealvalue, &starttime);
        fclose(stat_file);

        // Get system uptime
        FILE *uptime_file = fopen("/proc/uptime", "r");
        double system_uptime = 0;
        if (uptime_file) {
            fscanf(uptime_file, "%lf", &system_uptime);
            fclose(uptime_file);
        }

        // Calculate process uptime in seconds
        // starttime is in clock ticks since system boot
        // Convert to seconds by dividing by sysconf(_SC_CLK_TCK)
        long clock_ticks = sysconf(_SC_CLK_TCK);
        double process_uptime = system_uptime - ((double)starttime / clock_ticks);

        // Add process uptime to info
        cJSON_AddNumberToObject(info, "uptime", process_uptime);
    } else {
        // Fallback to system uptime if process uptime can't be determined
        struct sysinfo sys_info;
        if (sysinfo(&sys_info) == 0) {
            cJSON_AddNumberToObject(info, "uptime", sys_info.uptime);
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

            // Get actual usage of the storage directory
            unsigned long long used = 0;
            char command[512];
            snprintf(command, sizeof(command), "du -sb %s 2>/dev/null | cut -f1", g_config.storage_path);
            FILE *fp = popen(command, "r");
            if (fp) {
                if (fscanf(fp, "%llu", &used) != 1) {
                    // If du command fails, fall back to statvfs calculation
                    used = (disk_info.f_blocks - disk_info.f_bfree) * disk_info.f_frsize;
                }
                pclose(fp);
            } else {
                // Fallback if popen fails
                used = (disk_info.f_blocks - disk_info.f_bfree) * disk_info.f_frsize;
            }

            cJSON_AddNumberToObject(disk, "total", total);
            cJSON_AddNumberToObject(disk, "used", used);
            cJSON_AddNumberToObject(disk, "free", free);

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

                cJSON_AddNumberToObject(system_disk, "total", total);
                cJSON_AddNumberToObject(system_disk, "used", used);
                cJSON_AddNumberToObject(system_disk, "free", free);
            }

            // Add system disk object to info
            cJSON_AddItemToObject(info, "systemDisk", system_disk);
        }
    }

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

                                    // Try to get IP address using ip command
                                    char ip_cmd[256];
                                    char ip_addr[128] = "Unknown";
                                    snprintf(ip_cmd, sizeof(ip_cmd), "ip -4 addr show %s | grep -oP '(?<=inet\\s)\\d+(\\.\\d+){3}'", name);
                                    FILE *ip_fp = popen(ip_cmd, "r");
                                    if (ip_fp) {
                                        if (fgets(ip_addr, sizeof(ip_addr), ip_fp)) {
                                            // Remove newline
                                            ip_addr[strcspn(ip_addr, "\n")] = 0;
                                        }
                                        pclose(ip_fp);
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
                                        unsigned int flags;
                                        if (fscanf(flags_file, "%x", &flags) == 1) {
                                            is_up = (flags & 1) != 0; // IFF_UP is 0x1
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
        recording_count = get_recording_count(0, 0, NULL, 0);
        if (recording_count < 0) {
            recording_count = 0; // Reset if query fails
            log_error("Failed to get recording count from database");
        }

        // Get recordings size from storage directory
        unsigned long long recording_size = 0;

        // Check if we have a recordings directory
        char recordings_dir[512];
        snprintf(recordings_dir, sizeof(recordings_dir), "%s/", g_config.storage_path);

        struct stat st;
        if (stat(recordings_dir, &st) == 0 && S_ISDIR(st.st_mode)) {
            // Get size of recordings directory using du command
            char command[512];
            snprintf(command, sizeof(command), "du -sb %s 2>/dev/null | cut -f1", recordings_dir);
            log_debug("Executing command: %s", command);

            FILE *fp = popen(command, "r");
            if (fp) {
                if (fscanf(fp, "%llu", &recording_size) != 1) {
                    log_error("Failed to parse du command output");
                    recording_size = 0;
                } else {
                    log_debug("Recordings size from du command: %llu bytes", recording_size);
                }
                pclose(fp);
            } else {
                log_error("Failed to execute du command");
            }

            // If du command failed, try to get size using stat
            if (recording_size == 0) {
                log_debug("Trying to get recordings size using stat");

                // Use a different approach - recursively list all files and sum their sizes
                char find_command[512];
                snprintf(find_command, sizeof(find_command),
                        "find %s -type f -name \"*.mp4\" -exec stat -c %%s {} \\; | awk '{sum+=$1} END {print sum}'",
                        recordings_dir);
                log_debug("Executing command: %s", find_command);

                FILE *find_fp = popen(find_command, "r");
                if (find_fp) {
                    if (fscanf(find_fp, "%llu", &recording_size) != 1) {
                        log_error("Failed to parse find command output");
                        recording_size = 0;
                    } else {
                        log_debug("Recordings size from find command: %llu bytes", recording_size);
                    }
                    pclose(find_fp);
                } else {
                    log_error("Failed to execute find command");
                }
            }
        }

        cJSON_AddNumberToObject(recordings, "count", recording_count);
        cJSON_AddNumberToObject(recordings, "size", recording_size);

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
        mg_send_json_error(c, 500, "Failed to convert system info JSON to string");
        return;
    }

    // Send response
    mg_send_json_response(c, 200, json_str);

    // Clean up
    free(json_str);
    cJSON_Delete(info);

    log_info("Successfully handled GET /api/system/info request");
}

/**
 * @brief Direct handler for POST /api/system/restart
 */
void mg_handle_post_system_restart(struct mg_connection *c, struct mg_http_message *hm) {
    log_info("Handling POST /api/system/restart request");

    // Create success response using cJSON
    cJSON *success = cJSON_CreateObject();
    if (!success) {
        log_error("Failed to create success JSON object");
        mg_send_json_error(c, 500, "Failed to create success JSON");
        return;
    }

    cJSON_AddBoolToObject(success, "success", true);
    cJSON_AddStringToObject(success, "message", "System is restarting");

    // Convert to string
    char *json_str = cJSON_PrintUnformatted(success);
    if (!json_str) {
        log_error("Failed to convert success JSON to string");
        cJSON_Delete(success);
        mg_send_json_error(c, 500, "Failed to convert success JSON to string");
        return;
    }

    // Send response
    mg_send_json_response(c, 200, json_str);

    // Clean up
    free(json_str);
    cJSON_Delete(success);

    // Flush response
    c->is_resp = 0;

    // Log restart
    log_info("System restart requested via API");

    // Schedule restart
    extern volatile bool running;
    running = false;

    log_info("Successfully handled POST /api/system/restart request");
}

/**
 * @brief Direct handler for POST /api/system/shutdown
 */
void mg_handle_post_system_shutdown(struct mg_connection *c, struct mg_http_message *hm) {
    log_info("Handling POST /api/system/shutdown request");

    // Create success response using cJSON
    cJSON *success = cJSON_CreateObject();
    if (!success) {
        log_error("Failed to create success JSON object");
        mg_send_json_error(c, 500, "Failed to create success JSON");
        return;
    }

    cJSON_AddBoolToObject(success, "success", true);
    cJSON_AddStringToObject(success, "message", "System is shutting down");

    // Convert to string
    char *json_str = cJSON_PrintUnformatted(success);
    if (!json_str) {
        log_error("Failed to convert success JSON to string");
        cJSON_Delete(success);
        mg_send_json_error(c, 500, "Failed to convert success JSON to string");
        return;
    }

    // Send response
    mg_send_json_response(c, 200, json_str);

    // Clean up
    free(json_str);
    cJSON_Delete(success);

    // Flush response
    c->is_resp = 0;

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
void mg_handle_post_system_backup(struct mg_connection *c, struct mg_http_message *hm) {
    log_info("Handling POST /api/system/backup request");

    // Create a timestamp for the backup filename
    time_t now = time(NULL);
    struct tm* tm_info = localtime(&now);
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
            mg_send_json_error(c, 500, "Failed to create error JSON");
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
            mg_send_json_error(c, 500, "Failed to convert error JSON to string");
            return;
        }

        // Send response
        mg_send_json_error(c, 500, json_str);

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
        mg_send_json_error(c, 500, "Failed to create backup JSON");
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
        mg_send_json_error(c, 500, "Failed to create config JSON");
        return;
    }

    // Add config properties
    cJSON_AddNumberToObject(config, "web_port", g_config.web_port);
    cJSON_AddStringToObject(config, "web_root", g_config.web_root);
    cJSON_AddStringToObject(config, "log_file", g_config.log_file);
    cJSON_AddStringToObject(config, "pid_file", g_config.pid_file);
    cJSON_AddStringToObject(config, "db_path", g_config.db_path);
    cJSON_AddStringToObject(config, "storage_path", g_config.storage_path);
    cJSON_AddNumberToObject(config, "max_storage_size", g_config.max_storage_size);
    cJSON_AddNumberToObject(config, "max_streams", g_config.max_streams);

    // Add streams array
    cJSON *streams = cJSON_CreateArray();
    if (!streams) {
        log_error("Failed to create streams JSON array");
        cJSON_Delete(config);
        cJSON_Delete(backup);
        fclose(backup_file);
        mg_send_json_error(c, 500, "Failed to create streams JSON");
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
        mg_send_json_error(c, 500, "Failed to convert backup JSON to string");
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
        mg_send_json_error(c, 500, "Failed to create success JSON");
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
        mg_send_json_error(c, 500, "Failed to convert success JSON to string");
        return;
    }

    // Send response
    mg_send_json_response(c, 200, json_str);

    // Clean up
    free(json_str);
    cJSON_Delete(success);

    log_info("Successfully handled POST /api/system/backup request");
}

/**
 * @brief Direct handler for GET /api/system/status
 */
void mg_handle_get_system_status(struct mg_connection *c, struct mg_http_message *hm) {
    log_info("Handling GET /api/system/status request");

    // Create status response using cJSON
    cJSON *status = cJSON_CreateObject();
    if (!status) {
        log_error("Failed to create status JSON object");
        mg_send_json_error(c, 500, "Failed to create status JSON");
        return;
    }

    cJSON_AddStringToObject(status, "status", "ok");
    cJSON_AddStringToObject(status, "message", "System running normally");

    // Convert to string
    char *json_str = cJSON_PrintUnformatted(status);
    if (!json_str) {
        log_error("Failed to convert status JSON to string");
        cJSON_Delete(status);
        mg_send_json_error(c, 500, "Failed to convert status JSON to string");
        return;
    }

    // Send response
    mg_send_json_response(c, 200, json_str);

    // Clean up
    free(json_str);
    cJSON_Delete(status);

    log_info("Successfully handled GET /api/system/status request");
}
