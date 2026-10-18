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
#include <netinet/in.h>
#include <netpacket/packet.h>
#include <arpa/inet.h>
#include <fcntl.h>
#include <errno.h>
#include <dirent.h>
#include <limits.h>
#include <pthread.h>
#include <stdatomic.h>
#include <sqlite3.h>
#include <curl/curl.h>
#include <uv.h>
#include <llhttp.h>
#include <mbedtls/version.h>
#include <libavcodec/avcodec.h>
#include <libavformat/avformat.h>
#include <libavutil/avutil.h>
#include <libswscale/swscale.h>
#include <libswresample/swresample.h>

#include "web/api_handlers.h"
#include "web/api_handlers_health.h"
#include "web/request_response.h"
#include "web/httpd_utils.h"
#include "web/audit_log.h"
#define LOG_COMPONENT "SystemAPI"
#include "core/logger.h"
#include "core/config.h"
#include "core/path_utils.h"
#include "core/version.h"
#include "core/shutdown_coordinator.h"
#include "utils/strings.h"
#include "video/stream_manager.h"
#ifdef HAVE_LITERT
#include "video/detection/litert_engine.h"
#endif
#include "database/db_streams.h"
#include "database/db_recordings.h"
#include "storage/storage_manager_streams.h"
#include "storage/storage_manager_streams_cache.h"
#include "telemetry/system_health.h"

#ifdef USE_GO2RTC
#include "video/go2rtc/go2rtc_api.h"
#endif

// External function from api_handlers_system_go2rtc.c
extern bool get_go2rtc_memory_usage(unsigned long long *memory_usage);

// External declarations
extern bool daemon_mode;

#define SYSTEM_RECORDING_AGGREGATE_TTL_SECONDS 30
#define SYSTEM_INFO_RESPONSE_TTL_SECONDS 15

typedef struct {
    pthread_mutex_t mutex;
    time_t refreshed_at;
    int count;
    int64_t bytes;
} system_recording_aggregate_cache_t;

static system_recording_aggregate_cache_t recording_aggregate_cache = {
    .mutex = PTHREAD_MUTEX_INITIALIZER,
    .refreshed_at = 0,
    .count = 0,
    .bytes = 0,
};

static pthread_mutex_t system_info_response_mutex = PTHREAD_MUTEX_INITIALIZER;
static char *system_info_response_json = NULL;
static time_t system_info_response_refreshed_at = 0;
static uint64_t system_info_response_snapshot_sequence = 0;

static const system_health_observation_t *find_health_observation(
    const system_health_snapshot_t *snapshot, const char *metric,
    const char *resource) {
    if (!snapshot || !metric) return NULL;
    for (size_t index = 0; index < snapshot->observation_count; ++index) {
        const system_health_observation_t *item = &snapshot->observations[index];
        if (strcmp(item->metric, metric) == 0 &&
            (!resource || strcmp(item->resource_id, resource) == 0))
            return item;
    }
    return NULL;
}

static const system_health_observation_t *find_effective_health_observation(
    const system_health_snapshot_t *snapshot, const char *container_metric,
    const char *host_metric) {
    // Prefer the container-scoped observation only when it actually has a
    // value -- not merely when one exists. linux_cgroup.c's collector picks
    // CONTAINER vs. HOST scope for a whole batch of metrics (cpu, memory,
    // pids) based on whether *any* of them has a real, finite limit; systemd
    // gives every unit a default (non-"max") TasksMax=, so a completely
    // unconstrained host (no CPUQuota=/MemoryMax=) still gets bumped to
    // CONTAINER scope purely because of that unrelated pids limit. Its
    // cpu.usage_ratio/memory.limit_bytes/etc. observations still get
    // emitted under "container.*" in that case, just marked unavailable
    // (unsupported/unlimited) -- and since find_health_observation() matches
    // on existence, not availability, this lookup used to latch onto that
    // permanently-unavailable container observation and never fall back to
    // the perfectly good host.cpu.busy_ratio/host.memory.* figures at all.
    const system_health_observation_t *container_item = find_health_observation(
        snapshot, container_metric, NULL);
    if (container_item &&
        container_item->capability == SYSTEM_HEALTH_CAPABILITY_AVAILABLE)
        return container_item;
    const system_health_observation_t *host_item = find_health_observation(
        snapshot, host_metric, NULL);
    return host_item ? host_item : container_item;
}

static bool health_observation_value(
    const system_health_observation_t *observation, double *value) {
    if (!observation || !observation->value_valid ||
        observation->capability != SYSTEM_HEALTH_CAPABILITY_AVAILABLE)
        return false;
    *value = observation->value;
    return true;
}

static void add_health_number(cJSON *object, const char *name,
                              const system_health_observation_t *observation,
                              double scale) {
    double value;
    if (health_observation_value(observation, &value))
        cJSON_AddNumberToObject(object, name, value * scale);
    else
        cJSON_AddNullToObject(object, name);
}

static const char *health_capability(
    const system_health_observation_t *observation) {
    return observation
        ? system_health_capability_name(observation->capability) : "error";
}

static void get_cached_recording_aggregates(int *count, int64_t *bytes) {
    time_t now = time(NULL);
    pthread_mutex_lock(&recording_aggregate_cache.mutex);
    if (recording_aggregate_cache.refreshed_at == 0 ||
        now - recording_aggregate_cache.refreshed_at >=
            SYSTEM_RECORDING_AGGREGATE_TTL_SECONDS) {
        int refreshed_count = get_recording_count(
            0, 0, NULL, 0, NULL, -1, NULL, 0, NULL, NULL, false);
        int64_t refreshed_bytes = get_stream_storage_bytes(NULL);
        if (refreshed_count >= 0) recording_aggregate_cache.count = refreshed_count;
        if (refreshed_bytes >= 0) recording_aggregate_cache.bytes = refreshed_bytes;
        recording_aggregate_cache.refreshed_at = now;
    }
    *count = recording_aggregate_cache.count;
    *bytes = recording_aggregate_cache.bytes;
    pthread_mutex_unlock(&recording_aggregate_cache.mutex);
}

// Copies the src string after removing whitespace and single- or double-quotes.
static void trim_copy_value(char *dest, size_t dest_size, const char *src) {
    if (!dest || dest_size == 0) {
        return;
    }

    dest[0] = '\0';
    if (!src) {
        return;
    }

    const char *start = ltrim_pos(src);
    // rtrim_pos returns a pointer into src one *after* the last printing
    // character, so we need to subtract 1 to check the last characters
    // in the string.
    const char *end = rtrim_pos(src, 0) - 1;

    if ((end - start) >= 2 && ((*start == '"' && *end == '"') ||
                     (*start == '\'' && *end == '\''))) {
        start++;
        end--;
    }

    size_t len = end - start;
    if (len >= dest_size) {
        len = dest_size - 1;
    }

    memcpy(dest, start, len);
    dest[len] = '\0';
}

static bool read_os_release_value(const char *key, char *value, size_t value_size) {
    if (!key || !value || value_size == 0) {
        return false;
    }

    FILE *fp = fopen("/etc/os-release", "r");
    if (!fp) {
        return false;
    }

    bool found = false;
    char line[512];
    size_t key_len = strlen(key);

    while (fgets(line, sizeof(line), fp)) {
        if (strncmp(line, key, key_len) == 0 && line[key_len] == '=') {
            trim_copy_value(value, value_size, line + key_len + 1);
            found = value[0] != '\0';
            break;
        }
    }

    fclose(fp);
    return found;
}

static void add_version_entry(cJSON *items,
                              const char *name,
                              const char *category,
                              const char *version,
                              const char *details) {
    if (!items || !name || !category || !version) {
        return;
    }

    cJSON *item = cJSON_CreateObject();
    if (!item) {
        return;
    }

    cJSON_AddStringToObject(item, "name", name);
    cJSON_AddStringToObject(item, "category", category);
    cJSON_AddStringToObject(item, "version", (version[0] != '\0') ? version : "Unknown");
    if (details && details[0] != '\0') {
        cJSON_AddStringToObject(item, "details", details);
    }

    cJSON_AddItemToArray(items, item);
}

static void format_triplet_version(unsigned version_int, char *buffer, size_t buffer_size) {
    if (!buffer || buffer_size == 0) {
        return;
    }

    snprintf(buffer, buffer_size, "%u.%u.%u",
             AV_VERSION_MAJOR(version_int),
             AV_VERSION_MINOR(version_int),
             AV_VERSION_MICRO(version_int));
}

#define SYSTEM_INFO_MAX_INTERFACES 64

typedef struct {
    char name[IFNAMSIZ];
    char ipv4[INET_ADDRSTRLEN];
    char ipv6[INET6_ADDRSTRLEN];
    char mac[18];
    bool ipv6_link_local;
    unsigned int flags;
} system_info_interface_t;

static system_info_interface_t *find_or_add_interface(
    system_info_interface_t *list, size_t *count, const char *name) {
    for (size_t i = 0; i < *count; ++i) {
        if (strcmp(list[i].name, name) == 0) return &list[i];
    }
    if (*count >= SYSTEM_INFO_MAX_INTERFACES) return NULL;
    system_info_interface_t *entry = &list[(*count)++];
    memset(entry, 0, sizeof(*entry));
    safe_strcpy(entry->name, name, sizeof(entry->name), 0);
    return entry;
}

/* Host interface list for the admin-only System page. The health snapshot
 * only carries hashed interface IDs and carrier state, so read names,
 * addresses and MACs directly. Loopback is omitted, as are interfaces that
 * are administratively down with no address (e.g. unused sit0/ip6tnl0). */
static void add_network_interfaces_json(cJSON *interfaces) {
    struct ifaddrs *ifaddr = NULL;
    if (getifaddrs(&ifaddr) != 0) {
        log_warn("getifaddrs failed: %s", strerror(errno));
        return;
    }

    system_info_interface_t list[SYSTEM_INFO_MAX_INTERFACES];
    size_t count = 0;
    for (const struct ifaddrs *ifa = ifaddr; ifa; ifa = ifa->ifa_next) {
        if (!ifa->ifa_name || (ifa->ifa_flags & IFF_LOOPBACK)) continue;
        system_info_interface_t *entry =
            find_or_add_interface(list, &count, ifa->ifa_name);
        if (!entry) continue;
        entry->flags |= ifa->ifa_flags;
        if (!ifa->ifa_addr) continue;

        int family = ifa->ifa_addr->sa_family;
        if (family == AF_INET && !entry->ipv4[0]) {
            const struct sockaddr_in *in = (const struct sockaddr_in *)ifa->ifa_addr;
            inet_ntop(AF_INET, &in->sin_addr, entry->ipv4, sizeof(entry->ipv4));
        } else if (family == AF_INET6) {
            const struct sockaddr_in6 *in6 = (const struct sockaddr_in6 *)ifa->ifa_addr;
            bool link_local = IN6_IS_ADDR_LINKLOCAL(&in6->sin6_addr);
            // Prefer the first routable address over link-local fe80::/10
            if (!entry->ipv6[0] || (entry->ipv6_link_local && !link_local)) {
                inet_ntop(AF_INET6, &in6->sin6_addr, entry->ipv6, sizeof(entry->ipv6));
                entry->ipv6_link_local = link_local;
            }
        } else if (family == AF_PACKET) {
            const struct sockaddr_ll *ll = (const struct sockaddr_ll *)ifa->ifa_addr;
            bool nonzero = false;
            for (int i = 0; i < ll->sll_halen && i < 6; ++i) {
                if (ll->sll_addr[i]) nonzero = true;
            }
            if (ll->sll_halen == 6 && nonzero) {
                snprintf(entry->mac, sizeof(entry->mac),
                         "%02x:%02x:%02x:%02x:%02x:%02x",
                         ll->sll_addr[0], ll->sll_addr[1], ll->sll_addr[2],
                         ll->sll_addr[3], ll->sll_addr[4], ll->sll_addr[5]);
            }
        }
    }
    freeifaddrs(ifaddr);

    for (size_t i = 0; i < count; ++i) {
        const system_info_interface_t *entry = &list[i];
        bool has_ipv6 = entry->ipv6[0] && !entry->ipv6_link_local;
        const char *address = entry->ipv4[0] ? entry->ipv4
                            : has_ipv6 ? entry->ipv6 : NULL;
        if (!(entry->flags & IFF_UP) && !address) continue;

        cJSON *iface = cJSON_CreateObject();
        if (!iface) continue;
        cJSON_AddStringToObject(iface, "name", entry->name);
        if (address)
            cJSON_AddStringToObject(iface, "address", address);
        else
            cJSON_AddNullToObject(iface, "address");
        if (entry->ipv4[0] && has_ipv6)
            cJSON_AddStringToObject(iface, "ipv6", entry->ipv6);
        cJSON_AddStringToObject(iface, "mac", entry->mac[0] ? entry->mac : "Unavailable");
        // IFF_RUNNING reflects operational state (carrier), not just admin-up
        cJSON_AddBoolToObject(iface, "up", (entry->flags & IFF_RUNNING) != 0);
        cJSON_AddStringToObject(iface, "capability", "available");
        cJSON_AddItemToArray(interfaces, iface);
    }
}

static void add_versions_to_json(cJSON *info) {
    if (!info) {
        return;
    }

    cJSON *versions = cJSON_CreateObject();
    cJSON *items = cJSON_CreateArray();
    if (!versions || !items) {
        if (versions) cJSON_Delete(versions);
        if (items) cJSON_Delete(items);
        return;
    }

    char details[256];
    if (LIGHTNVR_GIT_COMMIT[0] != '\0') {
        snprintf(details, sizeof(details), "Build date %s • commit %s",
                 LIGHTNVR_BUILD_DATE, LIGHTNVR_GIT_COMMIT);
    } else {
        snprintf(details, sizeof(details), "Build date %s", LIGHTNVR_BUILD_DATE);
    }
    add_version_entry(items, "LightNVR", "Application", LIGHTNVR_VERSION_STRING, details);

    struct utsname system_info;
    if (uname(&system_info) == 0) {
        char pretty_name[256] = {0};
        char name[128] = {0};
        char version_id[128] = {0};
        char os_version[256] = {0};

        if (read_os_release_value("PRETTY_NAME", pretty_name, sizeof(pretty_name))) {
            safe_strcpy(os_version, pretty_name, sizeof(os_version), 0);
        } else if (read_os_release_value("NAME", name, sizeof(name))) {
            if (read_os_release_value("VERSION_ID", version_id, sizeof(version_id))) {
                snprintf(os_version, sizeof(os_version), "%s %s", name, version_id);
            } else {
                safe_strcpy(os_version, name, sizeof(os_version), 0);
            }
        } else {
            safe_strcpy(os_version, system_info.sysname, sizeof(os_version), 0);
        }

        snprintf(details, sizeof(details), "%s %s • %s",
                 system_info.sysname,
                 system_info.release,
                 system_info.machine);
        add_version_entry(items, "Base OS", "OS", os_version, details);
    }

#ifdef USE_GO2RTC
    char go2rtc_version[64] = {0};
    char go2rtc_revision[64] = {0};
    int rtsp_port = 0;
    if (go2rtc_api_get_application_info(&rtsp_port,
                                        go2rtc_version, sizeof(go2rtc_version),
                                        go2rtc_revision, sizeof(go2rtc_revision))) {
        snprintf(details, sizeof(details), "RTSP port %d%s%s",
                 (rtsp_port > 0) ? rtsp_port : 8554,
                 (go2rtc_revision[0] != '\0') ? " • revision " : "",
                 (go2rtc_revision[0] != '\0') ? go2rtc_revision : "");
        add_version_entry(items, "go2rtc", "Service", go2rtc_version, details);
    } else {
        add_version_entry(items, "go2rtc", "Service", "Unavailable", "go2rtc API not reachable");
    }
#endif

    add_version_entry(items, "SQLite", "Library", sqlite3_libversion(), NULL);

    const curl_version_info_data *curl_info = curl_version_info(CURLVERSION_NOW);
    if (curl_info && curl_info->version) {
        char curl_details[256] = {0};
        if (curl_info->ssl_version && curl_info->libz_version) {
            snprintf(curl_details, sizeof(curl_details), "%s • zlib %s",
                     curl_info->ssl_version, curl_info->libz_version);
        } else if (curl_info->ssl_version) {
            safe_strcpy(curl_details, curl_info->ssl_version, sizeof(curl_details), 0);
        } else if (curl_info->libz_version) {
            snprintf(curl_details, sizeof(curl_details), "zlib %s", curl_info->libz_version);
        }
        add_version_entry(items, "libcurl", "Library", curl_info->version, curl_details);
    }

    char mbedtls_version[64] = {0};
#if defined(MBEDTLS_VERSION_C)
    // Runtime query reports the version of the library actually linked
    mbedtls_version_get_string_full(mbedtls_version);
#elif defined(MBEDTLS_VERSION_STRING_FULL)
    // MBEDTLS_VERSION_C may be disabled in trimmed builds (e.g. buildroot
    // cross-compiles); fall back to the compile-time version string
    safe_strcpy(mbedtls_version, MBEDTLS_VERSION_STRING_FULL, sizeof(mbedtls_version), 0);
#else
    safe_strcpy(mbedtls_version, "Unknown", sizeof(mbedtls_version), 0);
#endif
    add_version_entry(items, "mbedTLS", "Library", mbedtls_version, NULL);

    add_version_entry(items, "libuv", "Library", uv_version_string(), NULL);

    char llhttp_version[32] = {0};
    snprintf(llhttp_version, sizeof(llhttp_version), "%d.%d.%d",
             LLHTTP_VERSION_MAJOR, LLHTTP_VERSION_MINOR, LLHTTP_VERSION_PATCH);
    add_version_entry(items, "llhttp", "Library", llhttp_version, NULL);

    char version_buf[32] = {0};
    format_triplet_version(avformat_version(), version_buf, sizeof(version_buf));
    add_version_entry(items, "libavformat", "Library", version_buf, NULL);

    format_triplet_version(avcodec_version(), version_buf, sizeof(version_buf));
    add_version_entry(items, "libavcodec", "Library", version_buf, NULL);

    format_triplet_version(avutil_version(), version_buf, sizeof(version_buf));
    add_version_entry(items, "libavutil", "Library", version_buf, NULL);

    format_triplet_version(swscale_version(), version_buf, sizeof(version_buf));
    add_version_entry(items, "libswscale", "Library", version_buf, NULL);

    format_triplet_version(swresample_version(), version_buf, sizeof(version_buf));
    add_version_entry(items, "libswresample", "Library", version_buf, NULL);

    cJSON_AddItemToObject(versions, "items", items);
    cJSON_AddItemToObject(info, "versions", versions);
}

/**
 * @brief Get memory usage of light-object-detect process
 *
 * @param memory_usage Pointer to store memory usage in bytes
 * @return true when memory usage is known, including zero when the optional
 *         detector is not running; false when usage could not be inspected
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

    const struct dirent *entry;
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
        /* Object detection is optional.  An absent detector has a known RSS
         * contribution of zero; reporting it as unavailable makes the System
         * page imply that a healthy LightNVR installation is incomplete. */
        log_debug("No light-object-detect process found; memory usage is zero");
        return true;
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
        /* Close fp if not already closed inside the cpu_max_parsed+period>0 branch */
        if (!(cpu_max_parsed && period > 0)) fclose(fp);
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
 * @brief Direct handler for GET /api/system/info
 */
void handle_get_system_info(const http_request_t *req, http_response_t *res) {
    log_info("Handling GET /api/system/info request");

    // System info is sensitive — require admin privileges
    if (!httpd_authorize_global_action(req, res, AUTHZ_SYSTEM_ADMIN)) {
        return;  // Error response already set
    }

    system_health_snapshot_t health_snapshot;
    memset(&health_snapshot, 0, sizeof(health_snapshot));
    bool health_snapshot_available =
        system_health_snapshot_copy(&health_snapshot);
    uint64_t health_sequence = health_snapshot_available
        ? health_snapshot.sequence : 0U;

    pthread_mutex_lock(&system_info_response_mutex);
    time_t response_now = time(NULL);
    if (system_info_response_json &&
        system_info_response_snapshot_sequence == health_sequence &&
        response_now - system_info_response_refreshed_at <
            SYSTEM_INFO_RESPONSE_TTL_SECONDS) {
        char *cached_json = strdup(system_info_response_json);
        pthread_mutex_unlock(&system_info_response_mutex);
        if (cached_json) {
            http_response_set_json(res, 200, cached_json);
            free(cached_json);
            return;
        }
    } else {
        pthread_mutex_unlock(&system_info_response_mutex);
    }

    int cached_recording_count = 0;
    int64_t cached_recording_bytes = 0;
    get_cached_recording_aggregates(&cached_recording_count,
                                    &cached_recording_bytes);

    // Create JSON object
    cJSON *info = cJSON_CreateObject();
    if (!info) {
        log_error("Failed to create system info JSON object");
        http_response_set_json_error(res, 500, "Failed to create system info JSON");
        return;
    }

    // Add version information
    cJSON_AddStringToObject(info, "version", LIGHTNVR_VERSION_STRING);
    if (LIGHTNVR_GIT_COMMIT[0] != '\0') {
        cJSON_AddStringToObject(info, "git_commit", LIGHTNVR_GIT_COMMIT);
    }

    // Get system information
    struct utsname system_info;
    if (uname(&system_info) == 0) {
        // Create CPU object
        cJSON *cpu = cJSON_CreateObject();
        if (cpu) {
            cJSON_AddStringToObject(cpu, "model", system_info.machine);

            // Prefer the effective quota already captured in this generation.
            const system_health_observation_t *cpu_quota =
                find_health_observation(&health_snapshot,
                                        "container.cpu.quota_cores", NULL);
            double quota_value = 0.0;
            int cores = (int)sysconf(_SC_NPROCESSORS_ONLN);
            if (health_observation_value(cpu_quota, &quota_value) &&
                quota_value > 0.0) {
                cores = (int)quota_value;
                if ((double)cores < quota_value) cores++;
                if (cores < 1) cores = 1;
            }
            cJSON_AddNumberToObject(cpu, "cores", cores);

            // Reuse the sampler's completed CPU delta. HTTP never establishes
            // or advances the CPU baseline.
            const system_health_observation_t *cpu_usage =
                find_effective_health_observation(
                    &health_snapshot, "container.cpu.usage_ratio",
                    "host.cpu.busy_ratio");
            add_health_number(cpu, "usage", cpu_usage, 100.0);
            cJSON_AddStringToObject(cpu, "usageCapability",
                                    health_capability(cpu_usage));

            // Add CPU object to info
            cJSON_AddItemToObject(info, "cpu", cpu);
        }
    }

    // Select one effective memory scope from the same immutable snapshot used
    // by the evaluator.
    const system_health_observation_t *memory_total =
        find_effective_health_observation(
            &health_snapshot, "container.memory.limit_bytes",
            "host.memory.total_bytes");
    const system_health_observation_t *memory_free =
        find_effective_health_observation(
            &health_snapshot, "container.memory.available_bytes",
            "host.memory.available_bytes");
    const system_health_observation_t *memory_current =
        find_health_observation(&health_snapshot,
                                "container.memory.current_bytes", NULL);
    double system_total_value = 0.0;
    double system_free_value = 0.0;
    double system_used_value = 0.0;
    bool system_total_valid =
        health_observation_value(memory_total, &system_total_value);
    bool system_free_valid =
        health_observation_value(memory_free, &system_free_value);
    bool system_used_valid =
        health_observation_value(memory_current, &system_used_value);
    if (!system_used_valid && system_total_valid && system_free_valid) {
        system_used_value = system_total_value > system_free_value
            ? system_total_value - system_free_value : 0.0;
        system_used_valid = true;
    }
    unsigned long long system_total = system_total_valid
        ? (unsigned long long)system_total_value : 0U;

    // Get memory information for the LightNVR process
    // Compute the in-process LiteRT engine footprint once. It lives inside this
    // process, so it is subtracted from the lightnvr process figure below and
    // reported separately as detectorMemory — otherwise the UI, which sums the
    // process and detector figures, would double-count it.
    unsigned long long litert_used = 0;
#ifdef HAVE_LITERT
    litert_used = (unsigned long long)litert_engine_registry_memory_bytes();
#endif

    // No resource filter: linux_process.c's collector stamps these
    // observations with resource_id "lightnvr" (see prepare_observation()),
    // not "process" -- passing "process" here made this lookup never match
    // anything, unconditionally reporting process memory/thread count as
    // unavailable. api_handlers_metrics.c looks up the same two metrics
    // correctly with no resource filter at all (find_observation(..., NULL))
    // -- matched that established, correct pattern instead of trying to
    // guess the right resource string.
    const system_health_observation_t *process_rss = find_health_observation(
        &health_snapshot, "process.rss_bytes", NULL);
    const system_health_observation_t *process_thread_observation =
        find_health_observation(&health_snapshot, "process.threads", NULL);
    double process_rss_value = 0.0;
    double process_threads_value = 0.0;
    bool process_rss_valid =
        health_observation_value(process_rss, &process_rss_value);
    bool process_threads_valid = health_observation_value(
        process_thread_observation, &process_threads_value);

    cJSON *memory = cJSON_CreateObject();
    if (memory) {
        double used = process_rss_value > (double)litert_used
            ? process_rss_value - (double)litert_used : 0.0;
        add_health_number(memory, "total", memory_total, 1.0);
        if (process_rss_valid)
            cJSON_AddNumberToObject(memory, "used", used);
        else
            cJSON_AddNullToObject(memory, "used");
        if (system_total_valid && process_rss_valid)
            cJSON_AddNumberToObject(memory, "free",
                system_total_value > used ? system_total_value - used : 0.0);
        else
            cJSON_AddNullToObject(memory, "free");
        cJSON_AddStringToObject(memory, "capability",
                                health_capability(process_rss));

        // Add memory object to info
        cJSON_AddItemToObject(info, "memory", memory);
    }

    // Add process thread count and web thread pool size
    if (process_threads_valid)
        cJSON_AddNumberToObject(info, "threads", process_threads_value);
    else
        cJSON_AddNullToObject(info, "threads");
    cJSON_AddNumberToObject(info, "webThreadPoolSize", (double)g_config.web_thread_pool_size);

    // Get memory information for the go2rtc process
    cJSON *go2rtc_memory = cJSON_CreateObject();
    if (go2rtc_memory) {
        unsigned long long go2rtc_used = 0;

        // Try to get go2rtc memory usage
        bool go2rtc_memory_valid = get_go2rtc_memory_usage(&go2rtc_used);
        if (go2rtc_memory_valid) {
            log_debug("go2rtc memory usage: %llu bytes", go2rtc_used);
        } else {
            log_warn("Failed to get go2rtc memory usage");
        }

        add_health_number(go2rtc_memory, "total", memory_total, 1.0);
        if (go2rtc_memory_valid)
            cJSON_AddNumberToObject(go2rtc_memory, "used",
                                    (double)go2rtc_used);
        else
            cJSON_AddNullToObject(go2rtc_memory, "used");
        if (go2rtc_memory_valid && system_total_valid)
            cJSON_AddNumberToObject(go2rtc_memory, "free",
                system_total > go2rtc_used ? (double)(system_total - go2rtc_used)
                                           : 0.0);
        else
            cJSON_AddNullToObject(go2rtc_memory, "free");

        // Add go2rtc memory object to info
        cJSON_AddItemToObject(info, "go2rtcMemory", go2rtc_memory);
    }

    // Get memory information for the light-object-detect process
    cJSON *detector_memory = cJSON_CreateObject();
    if (detector_memory) {
        unsigned long long detector_used = 0;

        // An optional detector that is not running contributes a known zero.
        // False is reserved for failures that prevent memory inspection.
        bool detector_memory_valid = get_detector_memory_usage(&detector_used);
        if (detector_memory_valid) {
            log_debug("light-object-detect memory usage: %llu bytes", detector_used);
        } else {
            log_debug("light-object-detect not running or memory unavailable");
        }

        // Add the in-process LiteRT engine footprint (computed above and
        // subtracted from the lightnvr process figure, so process + detector
        // sum correctly). Detection runs inside this process now, so there is
        // no separate detector process to scan for the LiteRT backend.
        if (litert_used > 0) {
            detector_used += litert_used;
            detector_memory_valid = true;
            log_debug("in-process LiteRT engine memory: %llu bytes", litert_used);
        }

        add_health_number(detector_memory, "total", memory_total, 1.0);
        if (detector_memory_valid)
            cJSON_AddNumberToObject(detector_memory, "used",
                                    (double)detector_used);
        else
            cJSON_AddNullToObject(detector_memory, "used");
        if (detector_memory_valid && system_total_valid)
            cJSON_AddNumberToObject(detector_memory, "free",
                system_total > detector_used ? (double)(system_total - detector_used)
                                             : 0.0);
        else
            cJSON_AddNullToObject(detector_memory, "free");

        // Add detector memory object to info
        cJSON_AddItemToObject(info, "detectorMemory", detector_memory);
    }

    // Get system-wide memory information
    cJSON *system_memory = cJSON_CreateObject();
    if (system_memory) {
        add_health_number(system_memory, "total", memory_total, 1.0);
        if (system_used_valid)
            cJSON_AddNumberToObject(system_memory, "used", system_used_value);
        else
            cJSON_AddNullToObject(system_memory, "used");
        add_health_number(system_memory, "free", memory_free, 1.0);
        cJSON_AddStringToObject(system_memory, "capability",
            system_total_valid && system_free_valid ? "available" :
            health_capability(!system_total_valid ? memory_total : memory_free));

        // Add system memory object to info
        cJSON_AddItemToObject(info, "systemMemory", system_memory);
    }

    /* The System page describes this LightNVR instance, so its uptime follows
     * the process/container lifecycle. Host uptime remains available through
     * the system-health API and Prometheus metrics. Using the host observation
     * here made freshly deployed pods appear as old as their Kubernetes node. */
    cJSON_AddNumberToObject(info, "uptime", get_process_uptime_seconds());
    cJSON_AddStringToObject(
        info, "uptimeCapability",
        system_health_capability_name(SYSTEM_HEALTH_CAPABILITY_AVAILABLE));

    // Filesystem capacity comes from the completed sampler generation. The
    // recording DB byte count retains the legacy meaning of disk.used.
    const system_health_observation_t *recording_total =
        find_health_observation(&health_snapshot, "filesystem.capacity_bytes",
                                "recording");
    const system_health_observation_t *recording_free =
        find_health_observation(&health_snapshot, "filesystem.available_bytes",
                                "recording");
    if (!recording_total)
        recording_total = find_health_observation(
            &health_snapshot, "filesystem.capacity_bytes", "root");
    if (!recording_free)
        recording_free = find_health_observation(
            &health_snapshot, "filesystem.available_bytes", "root");
    cJSON *disk = cJSON_CreateObject();
    if (disk) {
        add_health_number(disk, "total", recording_total, 1.0);
        cJSON_AddNumberToObject(disk, "used", (double)cached_recording_bytes);
        add_health_number(disk, "free", recording_free, 1.0);
        cJSON_AddStringToObject(disk, "capability",
            health_capability(recording_total ? recording_total : recording_free));
        cJSON_AddItemToObject(info, "disk", disk);
    }

    const system_health_observation_t *root_total =
        find_health_observation(&health_snapshot, "filesystem.capacity_bytes",
                                "root");
    const system_health_observation_t *root_free =
        find_health_observation(&health_snapshot, "filesystem.available_bytes",
                                "root");
    double root_total_value = 0.0;
    double root_free_value = 0.0;
    bool root_total_valid = health_observation_value(root_total,
                                                      &root_total_value);
    bool root_free_valid = health_observation_value(root_free,
                                                     &root_free_value);
    cJSON *system_disk = cJSON_CreateObject();
    if (system_disk) {
        add_health_number(system_disk, "total", root_total, 1.0);
        if (root_total_valid && root_free_valid)
            cJSON_AddNumberToObject(system_disk, "used",
                root_total_value > root_free_value
                    ? root_total_value - root_free_value : 0.0);
        else
            cJSON_AddNullToObject(system_disk, "used");
        add_health_number(system_disk, "free", root_free, 1.0);
        cJSON_AddStringToObject(system_disk, "capability",
            root_total_valid && root_free_valid ? "available" :
            health_capability(!root_total_valid ? root_total : root_free));
        cJSON_AddItemToObject(info, "systemDisk", system_disk);
    }

    // Add global storage policy settings so the frontend can display effective retention per stream
    cJSON_AddNumberToObject(info, "global_retention_days", g_config.retention_days);

    // Add runtime software/library versions for vulnerability auditing and support
    add_versions_to_json(info);

    cJSON *network = cJSON_CreateObject();
    cJSON *interfaces = cJSON_CreateArray();
    if (network && interfaces) {
        add_network_interfaces_json(interfaces);
        cJSON_AddItemToObject(network, "interfaces", interfaces);
        cJSON_AddItemToObject(info, "network", network);
    } else {
        cJSON_Delete(network);
        cJSON_Delete(interfaces);
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
        int recording_count = cached_recording_count;

        // Recordings size comes from the DB (SUM of completed recording sizes).
        // Avoids a full filesystem walk that could take many minutes on HDD
        // deployments with hundreds of thousands of segments (#368).
        int64_t recording_size_db = cached_recording_bytes;
        unsigned long long recording_size =
            (recording_size_db > 0) ? (unsigned long long)recording_size_db : 0;

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
    pthread_mutex_lock(&system_info_response_mutex);
    char *cached_copy = strdup(json_str);
    if (cached_copy) {
        free(system_info_response_json);
        system_info_response_json = cached_copy;
        system_info_response_refreshed_at = time(NULL);
        system_info_response_snapshot_sequence = health_sequence;
    }
    pthread_mutex_unlock(&system_info_response_mutex);
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

    // Restart is a destructive admin operation — require admin privileges
    user_t audit_user;
    authorization_evaluation_t evaluation;
    if (!httpd_authorize_action(req, res, AUTHZ_SYSTEM_ADMIN, NULL,
                                &audit_user, &evaluation)) {
        return;  // Error response already set
    }

    // Create success response using cJSON
    cJSON *success = cJSON_CreateObject();
    if (!success) {
        log_error("Failed to create success JSON object");
        http_response_set_json_error(res, 500, "Failed to create success JSON");
        audit_log_operation(req, &audit_user, "system.admin", "system", NULL,
                            "system.restart", "error", NULL);
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
        audit_log_operation(req, &audit_user, "system.admin", "system", NULL,
                            "system.restart", "error", NULL);
        return;
    }

    // Send response
    http_response_set_json(res, 200, json_str);

    // Clean up
    free(json_str);
    cJSON_Delete(success);

    // Log restart
    log_info("System restart requested via API");

    /* Persist the outcome before stopping the process; cleanup can otherwise
     * race the append and lose the operator-attributed restart event. */
    audit_log_operation(req, &audit_user, "system.admin", "system", NULL,
                        "system.restart", "success", NULL);

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

    // Shutdown is a destructive admin operation — require admin privileges
    user_t audit_user;
    authorization_evaluation_t evaluation;
    if (!httpd_authorize_action(req, res, AUTHZ_SYSTEM_ADMIN, NULL,
                                &audit_user, &evaluation)) {
        return;  // Error response already set
    }

    // Create success response using cJSON
    cJSON *success = cJSON_CreateObject();
    if (!success) {
        log_error("Failed to create success JSON object");
        http_response_set_json_error(res, 500, "Failed to create success JSON");
        audit_log_operation(req, &audit_user, "system.admin", "system", NULL,
                            "system.shutdown", "error", NULL);
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
        audit_log_operation(req, &audit_user, "system.admin", "system", NULL,
                            "system.shutdown", "error", NULL);
        return;
    }

    // Send response
    http_response_set_json(res, 200, json_str);

    // Clean up
    free(json_str);
    cJSON_Delete(success);

    // Log shutdown
    log_info("System shutdown requested via API");

    audit_log_operation(req, &audit_user, "system.admin", "system", NULL,
                        "system.shutdown", "success", NULL);

    // Include shutdown coordinator header
    #include "core/shutdown_coordinator.h"

    // Initiate shutdown through the coordinator first
    log_info("Initiating shutdown through coordinator");
    initiate_shutdown();

    extern _Atomic bool running;
    running = false;
    // Let a long-running background operation (e.g. a scheduled DB backup)
    // abort promptly instead of silently blocking this shutdown until it
    // finishes -- mirrors request_restart()'s identical call for the
    // restart path (main.c); see shutdown_coordinator.c's header comment
    // for why this is a separate, lighter-weight flag from
    // initiate_shutdown() above.
    request_background_abort();

    // Deliberately no local alarm()-based force-exit timer here (this used
    // to set one, framed as "more robust for Linux 4.4 embedded MIPS
    // systems"): alarm() is a single process-wide timer, and it would race
    // with the many short-lived, save/restore-disposition alarm() calls
    // hls_unified_thread.c/hls_writer.c make elsewhere in the process --
    // whichever fires first silently cancels whatever time was left on the
    // other, since alarm() has no pause/resume. main()'s
    // shutdown_watchdog_thread (a plain thread polling `running`, immune to
    // that collision) already force-kills the process group process-wide if
    // shutdown doesn't complete in time, covering this endpoint too -- see
    // its comment in main.c for the full story, including the live gdb
    // trace that first caught this exact collision on the restart path.

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

    // Backup is an admin operation — require admin privileges
    user_t audit_user;
    authorization_evaluation_t evaluation;
    if (!httpd_authorize_action(req, res, AUTHZ_SYSTEM_ADMIN, NULL,
                                &audit_user, &evaluation)) {
        return;  // Error response already set
    }

    // Create a timestamp for the backup filename
    time_t now = time(NULL);
    struct tm tm_buf;
    const struct tm* tm_info = localtime_r(&now, &tm_buf);
    char timestamp[20];
    strftime(timestamp, sizeof(timestamp), "%Y%m%d_%H%M%S", tm_info);

    // Create backup filename
    char backup_filename[256];
    snprintf(backup_filename, sizeof(backup_filename), "lightnvr_backup_%s.json", timestamp);

    // Create backup path in the web root directory
    char backup_path[MAX_PATH_LENGTH];
    snprintf(backup_path, sizeof(backup_path), "%s/backups", g_config.web_root);

    // Create backups directory if it doesn't exist
    if (ensure_dir(backup_path)) {
        log_error("Failed to create backup directory %s: %s", backup_path, strerror(errno));
        http_response_set_json_error(res, 500, "Failed to create backup directory");
        audit_log_operation(req, &audit_user, "system.admin", "backup", NULL,
                            "backup.create", "failure", NULL);
        return;
    }

    // Append filename to path
    snprintf(backup_path, sizeof(backup_path), "%s/backups/%s", g_config.web_root, backup_filename);

    // Open backup file with restricted permissions (owner read/write only)
    int backup_fd = open(backup_path, O_WRONLY | O_CREAT | O_TRUNC, 0600);
    if (backup_fd < 0) {
        log_error("Failed to create backup file: %s", strerror(errno));
        audit_log_operation(req, &audit_user, "system.admin", "backup", NULL,
                            "backup.create", "failure", NULL);

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

    FILE* backup_file = fdopen(backup_fd, "w");
    if (!backup_file) {
        log_error("Failed to open backup file stream: %s", strerror(errno));
        close(backup_fd);
        http_response_set_json_error(res, 500, "Failed to open backup file stream");
        audit_log_operation(req, &audit_user, "system.admin", "backup", NULL,
                            "backup.create", "failure", NULL);
        return;
    }

    // Create JSON object for backup
    cJSON *backup = cJSON_CreateObject();
    if (!backup) {
        log_error("Failed to create backup JSON object");
        fclose(backup_file);
        http_response_set_json_error(res, 500, "Failed to create backup JSON");
        audit_log_operation(req, &audit_user, "system.admin", "backup", NULL,
                            "backup.create", "failure", NULL);
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
        audit_log_operation(req, &audit_user, "system.admin", "backup", NULL,
                            "backup.create", "failure", NULL);
        return;
    }

    // Add config properties
    cJSON_AddNumberToObject(config, "web_port", g_config.web_port);
    cJSON_AddStringToObject(config, "web_bind_ip", g_config.web_bind_ip);
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
        audit_log_operation(req, &audit_user, "system.admin", "backup", NULL,
                            "backup.create", "failure", NULL);
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

            cJSON_AddStringToObject(stream, "camera_uuid",
                                    g_config.streams[i].camera_uuid);
            cJSON_AddStringToObject(stream, "location_uuid",
                                    g_config.streams[i].location_uuid);
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
            cJSON_AddBoolToObject(stream, "is_onvif", g_config.streams[i].is_onvif);
            cJSON_AddStringToObject(stream, "onvif_username", g_config.streams[i].onvif_username);
            cJSON_AddStringToObject(stream, "onvif_password", g_config.streams[i].onvif_password);
            cJSON_AddStringToObject(stream, "onvif_profile", g_config.streams[i].onvif_profile);
            cJSON_AddNumberToObject(stream, "onvif_port", g_config.streams[i].onvif_port);

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
        audit_log_operation(req, &audit_user, "system.admin", "backup", NULL,
                            "backup.create", "failure", NULL);
        return;
    }

    // Write to file
    int write_result = fputs(json_str, backup_file);

    // Clean up
    free(json_str);
    cJSON_Delete(backup);
    int close_result = fclose(backup_file);

    if (write_result == EOF || close_result != 0) {
        log_error("Failed to finish backup file: %s", strerror(errno));
        audit_log_operation(req, &audit_user, "system.admin", "backup", NULL,
                            "backup.create", "failure", NULL);
        http_response_set_json_error(res, 500, "Failed to write backup file");
        return;
    }

    audit_log_operation(req, &audit_user, "system.admin", "backup", NULL,
                        "backup.create", "success", NULL);

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

    if (!httpd_authorize_global_action(req, res, AUTHZ_SYSTEM_ADMIN)) return;

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
