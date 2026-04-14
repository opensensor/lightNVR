/**
 * @file go2rtc_process.c
 * @brief Implementation of the go2rtc process management module
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <signal.h>
#include <sys/types.h>
#include <sys/stat.h>
#include <sys/wait.h>
#include <sys/prctl.h>
#include <fcntl.h>
#include <dirent.h>
#include <errno.h>
#include <limits.h>
#include <curl/curl.h>

#include "video/go2rtc/go2rtc_process.h"
#include "video/go2rtc/go2rtc_api.h"
#include "core/logger.h"
#include "core/config.h"
#include "core/path_utils.h"
#include "utils/strings.h"
#include "database/db_core.h"
#include "database/db_streams.h"
#include "database/db_system_settings.h"

/**
 * Escape a string for use inside a YAML double-quoted scalar.
 * Handles " → \" and \ → \\.  Returns dst.
 */
static char *yaml_escape_string(const char *src, char *dst, size_t dst_size) {
    size_t j = 0;
    for (size_t i = 0; src[i] && j + 2 < dst_size; i++) {
        if (src[i] == '"' || src[i] == '\\') {
            dst[j++] = '\\';
        }
        dst[j++] = src[i];
    }
    dst[j] = '\0';
    return dst;
}


// Define PATH_MAX if not defined
#ifndef PATH_MAX
#define PATH_MAX 4096
#endif

extern config_t g_config;

/* ── Shell-free process/network helpers ─────────────────────────────────── */

/**
 * @brief Scan /proc for processes whose cmdline contains @p pattern.
 *
 * Replaces: popen("ps | grep ... | awk '{print $1}'")
 *           popen("pgrep -f '<pattern>'")
 *
 * @param pattern    Substring to search for in each process's cmdline
 * @param pids       Output array to receive matching PIDs
 * @param max_pids   Capacity of @p pids
 * @return Number of PIDs written into @p pids
 */
static int scan_proc_for_cmdline(const char *pattern, pid_t *pids, int max_pids) {
    int found = 0;
    pid_t self = getpid();

    DIR *proc_dir = opendir("/proc");
    if (!proc_dir) return 0;

    const struct dirent *entry;
    while ((entry = readdir(proc_dir)) != NULL && found < max_pids) {
        // Only numeric directory names are processes
        const char *d = entry->d_name;
        if (*d < '1' || *d > '9') continue;
        char *ep;
        pid_t pid = (pid_t)strtol(d, &ep, 10);
        if (*ep != '\0' || pid <= 0 || pid == self) continue;

        char cmdline_path[64];
        snprintf(cmdline_path, sizeof(cmdline_path), "/proc/%d/cmdline", pid);
        FILE *f = fopen(cmdline_path, "r");
        if (!f) continue;

        char cmdline[1024] = {0};
        size_t bytes = fread(cmdline, 1, sizeof(cmdline) - 1, f);
        fclose(f);

        // cmdline fields are NUL-separated — replace with spaces for strstr
        for (size_t i = 0; i < bytes; i++) {
            if (cmdline[i] == '\0') cmdline[i] = ' ';
        }

        if (strstr(cmdline, pattern)) {
            pids[found++] = pid;
        }
    }

    closedir(proc_dir);
    return found;
}

/**
 * @brief Return true when @p path_or_name has basename exactly @p expected_name.
 */
static bool basename_equals(const char *path_or_name, const char *expected_name) {
    if (!path_or_name || !expected_name || expected_name[0] == '\0') {
        return false;
    }

    const char *base = strrchr(path_or_name, '/');
    base = base ? base + 1 : path_or_name;
    return strcmp(base, expected_name) == 0;
}

/**
 * @brief Read /proc/<pid>/cmdline into @p cmdline.
 *
 * @return true if the file was read successfully, false otherwise.
 */
static bool read_proc_cmdline(pid_t pid, char *cmdline, size_t cmdline_size, size_t *bytes_read) {
    if (!cmdline || cmdline_size == 0) {
        return false;
    }

    char cmdline_path[64];
    snprintf(cmdline_path, sizeof(cmdline_path), "/proc/%d/cmdline", pid);

    FILE *fp = fopen(cmdline_path, "r");
    if (!fp) {
        return false;
    }

    size_t bytes = fread(cmdline, 1, cmdline_size - 1, fp);
    fclose(fp);
    cmdline[bytes] = '\0';

    if (bytes_read) {
        *bytes_read = bytes;
    }

    return true;
}

/**
 * @brief Check whether /proc/<pid>/cmdline argv[0] basename exactly matches @p name.
 */
static bool proc_first_cmdline_arg_basename_equals(pid_t pid, const char *name) {
    char cmdline[1024] = {0};
    size_t bytes_read = 0;

    if (!read_proc_cmdline(pid, cmdline, sizeof(cmdline), &bytes_read) || bytes_read == 0) {
        return false;
    }

    size_t arg0_len = strnlen(cmdline, bytes_read);
    if (arg0_len == 0) {
        return false;
    }

    char arg0[PATH_MAX];
    if (arg0_len >= sizeof(arg0)) {
        arg0_len = sizeof(arg0) - 1;
    }

    memcpy(arg0, cmdline, arg0_len);
    arg0[arg0_len] = '\0';
    return basename_equals(arg0, name);
}

/**
 * @brief Scan /proc for processes whose argv[0] basename exactly matches @p name.
 */
static int scan_proc_for_argv0_basename(const char *name, pid_t *pids, int max_pids) {
    int found = 0;
    pid_t self = getpid();

    DIR *proc_dir = opendir("/proc");
    if (!proc_dir) return 0;

    const struct dirent *entry;
    while ((entry = readdir(proc_dir)) != NULL && found < max_pids) {
        const char *d = entry->d_name;
        if (*d < '1' || *d > '9') continue;

        char *ep;
        pid_t pid = (pid_t)strtol(d, &ep, 10);
        if (*ep != '\0' || pid <= 0 || pid == self) continue;

        if (proc_first_cmdline_arg_basename_equals(pid, name)) {
            pids[found++] = pid;
        }
    }

    closedir(proc_dir);
    return found;
}

/**
 * @brief Check whether a TCP port is open (LISTEN state) via /proc/net/tcp.
 *
 * Replaces: popen("netstat -tlpn 2>/dev/null | grep ':<port>'")
 *
 * @param port  Port number to look for
 * @return true if the port appears in LISTEN state in /proc/net/tcp[6]
 */
static bool check_tcp_port_open(int port) {
    char hex_port[8];
    snprintf(hex_port, sizeof(hex_port), ":%04X", port);

    const char *tcp_files[] = {"/proc/net/tcp", "/proc/net/tcp6", NULL};
    for (int i = 0; tcp_files[i]; i++) {
        FILE *fp = fopen(tcp_files[i], "r");
        if (!fp) continue;
        char line[512];
        fgets(line, sizeof(line), fp); /* skip header */
        // NOLINTNEXTLINE(clang-analyzer-unix.Stream)
        while (fgets(line, sizeof(line), fp)) {
            /* /proc/net/tcp format (space-separated fields):
             *   sl  local_address  rem_address  st  ...
             *    0: 00000000:2EC0  00000000:0000 0A  ...
             *
             * We need to check:
             *  1. The port appears in the LOCAL address (field 1, after "sl:")
             *  2. The state (field 3) is 0A (TCP_LISTEN)
             *
             * Fields are separated by whitespace.  Field indices (0-based):
             *   0 = "sl:"  1 = local_address  2 = rem_address  3 = state
             */
            char *fields[5] = {NULL};
            char *saveptr = NULL;
            char linecopy[512];
            safe_strcpy(linecopy, line, sizeof(linecopy), 0);

            int f = 0;
            char *tok = strtok_r(linecopy, " \t", &saveptr);
            while (tok && f < 5) {
                fields[f++] = tok;
                tok = strtok_r(NULL, " \t", &saveptr);
            }

            /* Need at least 4 fields: sl, local_addr, rem_addr, state */
            if (f < 4 || !fields[1] || !fields[3]) continue;

            /* Check local address contains our port */
            if (!strstr(fields[1], hex_port)) continue;

            /* Check state is 0A (LISTEN) */
            if (strcmp(fields[3], "0A") == 0) {
                fclose(fp);
                return true;
            }
        }
        fclose(fp);
    }
    return false;
}

/**
 * @brief Search PATH for an executable named @p name and write its full path
 *        into @p out. Writes an empty string if not found.
 *
 * Replaces: popen("which <name> 2>/dev/null")
 */
static void find_binary_in_path(const char *name, char *out, size_t out_size) {
    const char *path_env = getenv("PATH");
    if (!path_env) path_env = "/usr/local/bin:/usr/bin:/bin";

    char path_copy[4096];
    safe_strcpy(path_copy, path_env, sizeof(path_copy), 0);

    char *saveptr = NULL;
    const char *dir = strtok_r(path_copy, ":", &saveptr);
    while (dir) {
        char candidate[PATH_MAX];
        struct stat st;
        int n = snprintf(candidate, sizeof(candidate), "%s/%s", dir, name);
        if (n > 0 && n < (int)sizeof(candidate) && access(candidate, X_OK) == 0 && stat(candidate, &st) == 0) {
            if (S_ISREG(st.st_mode)) {
                safe_strcpy(out, candidate, out_size, 0);
                return;
            }
        }
        dir = strtok_r(NULL, ":", &saveptr);
    }
    // Not found
    out[0] = '\0';
}

/**
 * @brief Read /proc/<pid>/comm and check whether it exactly matches @p name.
 *
 * Replaces: system("ps -p <pid> -o comm= | grep -q <name>")
 *
 * @return true if /proc/<pid>/comm exactly matches @p name
 */
static bool proc_comm_equals(pid_t pid, const char *name) {
    char path[64];
    snprintf(path, sizeof(path), "/proc/%d/comm", pid);
    FILE *fp = fopen(path, "r");
    if (!fp) return false;
    char comm[256] = {0};
    bool ok = (fgets(comm, sizeof(comm), fp) != NULL);
    fclose(fp);

    if (!ok) {
        return false;
    }

    comm[strcspn(comm, "\n")] = '\0'; // NOLINT(clang-analyzer-security.ArrayBound)
    return strcmp(comm, name) == 0;
}

/**
 * @brief fd-based recursive helper for recursive_remove().
 *
 * All stat/unlink/rmdir operations are performed relative to an open directory
 * file descriptor, eliminating the TOCTOU window that exists when using full
 * path strings (CWE-367).
 *
 * @param parent_dfd  Open O_RDONLY|O_DIRECTORY fd for the parent directory.
 * @param name        Name of the entry inside parent_dfd to remove.
 */
static void recursive_remove_at(int parent_dfd, const char *name) {
    if (parent_dfd < 0) return;
    struct stat st;
    if (fstatat(parent_dfd, name, &st, AT_SYMLINK_NOFOLLOW) != 0) return;

    if (!S_ISDIR(st.st_mode)) {
        /* Plain file or symlink – unlink relative to the parent fd. */
        unlinkat(parent_dfd, name, 0);
        return;
    }

    /* Open the subdirectory relative to the parent fd (never follow symlinks). */
    int dfd = openat(parent_dfd, name, O_RDONLY | O_DIRECTORY | O_NOFOLLOW);
    if (dfd < 0) return;

    DIR *d = fdopendir(dfd); /* fdopendir takes ownership of dfd */
    if (!d) {
        close(dfd);
        return;
    }

    const struct dirent *entry;
    while ((entry = readdir(d)) != NULL) {
        if (strcmp(entry->d_name, ".") == 0 || strcmp(entry->d_name, "..") == 0) continue;
        recursive_remove_at(dirfd(d), entry->d_name);
    }
    closedir(d); /* also closes dfd */

    unlinkat(parent_dfd, name, AT_REMOVEDIR);
}

/**
 * @brief Recursively remove a directory and all its contents without a shell.
 *
 * Replaces: system("rm -rf <path>")
 */
static void recursive_remove(const char *path) {
    char parent[PATH_MAX];
    safe_strcpy(parent, path, sizeof(parent), 0);

    const char *name;
    int parent_fd;
    char *sep = strrchr(parent, '/');

    if (sep && sep != parent) {
        /* e.g. "/dev/shm/logs/go2rtc"  →  parent="/dev/shm/logs", name="go2rtc" */
        *sep = '\0';
        name = sep + 1;
        parent_fd = open(parent, O_RDONLY | O_DIRECTORY | O_NOFOLLOW);
    } else if (sep == parent) {
        /* path is "/foo" – parent is the filesystem root */
        name = path + 1;
        parent_fd = open("/", O_RDONLY | O_DIRECTORY);
    } else {
        /* Relative path with no slash – parent is cwd */
        name = path;
        parent_fd = open(".", O_RDONLY | O_DIRECTORY);
    }

    if (parent_fd < 0) return;
    recursive_remove_at(parent_fd, name);
    close(parent_fd);
}

// Process management variables
static char *g_binary_path = NULL;
static char *g_config_dir = NULL;
static char *g_config_path = NULL;
static pid_t g_process_pid = -1;
static bool g_initialized = false;
static int g_rtsp_port = 8554; // Default RTSP port
static int g_api_port = 1984;  // Configured API port (updated during init)

// Callback function for libcurl to discard response data
static size_t discard_response_data(void *ptr, size_t size, size_t nmemb, void *userdata) {
    // Just return the size of the data to indicate we handled it
    return size * nmemb;
}

/**
 * @brief Check if go2rtc is already running as a system service using libcurl
 *
 * @param api_port The port to check for go2rtc service
 * @return true if go2rtc is running as a service, false otherwise
 */
static bool is_go2rtc_running_as_service(int api_port) {
    // Check if the API port is in use via /proc/net/tcp (no shell needed)
    bool port_in_use = check_tcp_port_open(api_port);

    if (port_in_use) {
        // Also confirm that a go2rtc process is among those found by /proc scan
        pid_t pids[64];
        int n = scan_proc_for_argv0_basename("go2rtc", pids, 64);
        if (n > 0) {
            log_debug("go2rtc is already running as a service on port %d", api_port);
            return true;
        }

        // Use libcurl to make a simple HTTP request to the API endpoint
        CURL *curl;
        CURLcode res;
        char url[256];
        long http_code = 0;

        // Initialize curl
        curl = curl_easy_init();
        if (!curl) {
            log_warn("Failed to initialize curl");
            return false;
        }

        // Format the URL for the API endpoint
        snprintf(url, sizeof(url), "http://localhost:%d" GO2RTC_BASE_PATH "/api/streams", api_port);

        // Set curl options
        curl_easy_setopt(curl, CURLOPT_URL, url);
        curl_easy_setopt(curl, CURLOPT_WRITEFUNCTION, discard_response_data);
        curl_easy_setopt(curl, CURLOPT_TIMEOUT, 2L); // 2 second timeout
        curl_easy_setopt(curl, CURLOPT_CONNECTTIMEOUT, 2L); // 2 second connect timeout
        curl_easy_setopt(curl, CURLOPT_NOSIGNAL, 1L); // Prevent curl from using signals

        // Perform the request
        res = curl_easy_perform(curl);

        // Check for errors
        if (res != CURLE_OK) {
            log_warn("Curl request failed: %s", curl_easy_strerror(res));
            curl_easy_cleanup(curl);
            return false;
        }

        // Get the HTTP response code
        curl_easy_getinfo(curl, CURLINFO_RESPONSE_CODE, &http_code);

        // Clean up
        curl_easy_cleanup(curl);

        if (http_code == 200 || http_code == 401) {
            log_debug("Port %d is responding like go2rtc (HTTP %ld)", api_port, http_code);
            return true;
        }

        log_warn("Port %d returned HTTP %ld, not a go2rtc service", api_port, http_code);
    }

    return false;
}

/**
 * @brief Check if go2rtc is available in PATH
 *
 * @param binary_path Buffer to store the found binary path
 * @param buffer_size Size of the buffer
 * @return true if binary was found, false otherwise
 */
static bool check_go2rtc_in_path(char *binary_path, size_t buffer_size) {
    // Use find_binary_in_path() which searches PATH directories directly
    // (no shell / popen needed)
    find_binary_in_path("go2rtc", binary_path, buffer_size);

    if (binary_path[0] != '\0' && access(binary_path, X_OK) == 0) {
        log_info("Found go2rtc binary in PATH: %s", binary_path);
        return true;
    }

    return false;
}

bool go2rtc_process_init(const char *binary_path, const char *config_dir, int api_port) {
    if (g_initialized) {
        log_warn("go2rtc process manager already initialized");
        return false;
    }

    if (!config_dir) {
        log_error("Invalid config_dir parameter for go2rtc_process_init");
        return false;
    }

    // Check if config directory exists, create if not
    if (mkdir_recursive(config_dir)) {
        log_error("Failed to create go2rtc config directory: %s", config_dir);
        return false;
    }

    // Store config directory
    g_config_dir = strdup(config_dir);

    // Create config path
    size_t config_path_len = strlen(config_dir) + strlen("/go2rtc.yaml") + 1;
    g_config_path = malloc(config_path_len);
    if (!g_config_path) {
        log_error("Memory allocation failed for config path");
        free(g_config_dir);
        g_config_dir = NULL;
        return false;
    }

    snprintf(g_config_path, config_path_len, "%s/go2rtc.yaml", config_dir);

    // Store the configured API port so all runtime checks use the right port
    g_api_port = api_port > 0 ? api_port : 1984;

    // Check if go2rtc is already running as a service
    if (is_go2rtc_running_as_service(g_api_port)) {
        log_info("go2rtc is already running as a service, will use the existing service");
        // Set an empty binary path to indicate we're using an existing service
        g_binary_path = strdup("");
    } else {
        // Check if binary exists at the specified path
        char final_binary_path[PATH_MAX] = {0};

        if (binary_path && access(binary_path, X_OK) == 0) {
            // Use the provided binary path
            safe_strcpy(final_binary_path, binary_path, sizeof(final_binary_path), 0);
            log_info("Using provided go2rtc binary: %s", final_binary_path);
        } else {
            if (binary_path) {
                log_warn("go2rtc binary not found or not executable at specified path: %s", binary_path);
            }

            // Use go2rtc from PATH
            if (!check_go2rtc_in_path(final_binary_path, sizeof(final_binary_path))) {
                log_error("go2rtc binary not found in PATH and no running service detected");
                free(g_config_dir);
                free(g_config_path);
                return false;
            }
        }

        // Store binary path
        g_binary_path = strdup(final_binary_path);
    }

    g_initialized = true;

    if (g_binary_path[0] != '\0') {
        log_info("go2rtc process manager initialized with binary: %s, config dir: %s",
                g_binary_path, g_config_dir);
    } else {
        log_info("go2rtc process manager initialized to use existing service, config dir: %s",
                g_config_dir);
    }

    return true;
}

bool go2rtc_process_generate_config(const char *config_path, int api_port) {
    if (!g_initialized) {
        log_error("go2rtc process manager not initialized");
        return false;
    }

    // Use 0600 permissions so credentials written to the go2rtc config file
    // are not world-readable. open()+fdopen() instead of fopen() lets us specify
    // the mode explicitly without relying on the process umask.
    int config_fd = open(config_path, O_WRONLY | O_CREAT | O_TRUNC, 0600);
    if (config_fd < 0) {
        log_error("Failed to open go2rtc config file for writing: %s", config_path);
        return false;
    }
    FILE *config_file = fdopen(config_fd, "w");
    if (!config_file) {
        log_error("Failed to create file stream for go2rtc config file: %s", config_path);
        close(config_fd);
        return false;
    }

    // Get global config for authentication settings
    config_t *global_config = &g_config;

    // Write basic configuration
    fprintf(config_file, "# go2rtc configuration generated by NVR software\n\n");

    // API configuration
    fprintf(config_file, "api:\n");
    fprintf(config_file, "  listen: :%d\n", api_port);
	// base_path must match GO2RTC_BASE_PATH in go2rtc_api.h so all C API calls resolve correctly.
	// IMPORTANT: do not include a trailing slash here. go2rtc registers routes as:
	//   pattern = base_path + "/" + "api/..."
	// so a trailing slash would produce double slashes ("/go2rtc//api") and can cause 404s.
	fprintf(config_file, "  base_path: %s\n", GO2RTC_BASE_PATH);

    // Use wildcard for CORS origin to support both localhost and 127.0.0.1
    // This allows the web interface to access go2rtc API from any local address
    fprintf(config_file, "  origin: '*'\n");
    // NOTE: Do NOT pass username/password to go2rtc. go2rtc's middleware chain
    // runs Auth before CORS, so when auth is enabled, unauthenticated browser
    // requests (including CORS preflights) get rejected with 401 before the
    // CORS headers are added — causing the browser to block the response.
    // lightNVR's own auth on port 8080 protects the main application; go2rtc
    // on its local port does not need a separate auth layer.

    // RTSP configuration - use configured port or default to 8554
    int rtsp_port = global_config->go2rtc_rtsp_port > 0 ? global_config->go2rtc_rtsp_port : 8554;
    fprintf(config_file, "\nrtsp:\n");
    fprintf(config_file, "  listen: \":%d\"\n", rtsp_port);

    // WebRTC configuration for NAT traversal
    if (global_config->go2rtc_webrtc_enabled) {
        fprintf(config_file, "\nwebrtc:\n");

        // WebRTC listen port
        if (global_config->go2rtc_webrtc_listen_port > 0) {
            fprintf(config_file, "  listen: \":%d\"\n", global_config->go2rtc_webrtc_listen_port);
        }

        // ICE servers configuration
        // Include ice_servers section if STUN, custom ICE servers, or TURN is enabled
        if (global_config->go2rtc_stun_enabled || global_config->go2rtc_ice_servers[0] != '\0' ||
            (global_config->turn_enabled && global_config->turn_server_url[0] != '\0')) {
            fprintf(config_file, "  ice_servers:\n");

            // Add custom ICE servers if specified
            if (global_config->go2rtc_ice_servers[0] != '\0') {
                // Parse comma-separated ICE servers
                char ice_servers_copy[512];
                safe_strcpy(ice_servers_copy, global_config->go2rtc_ice_servers, sizeof(ice_servers_copy), 0);

                char *token = strtok(ice_servers_copy, ",");
                while (token != NULL) {
                    // Trim whitespace
                    while (*token == ' ') token++;
                    char *end = token + strlen(token) - 1;
                    while (end > token && *end == ' ') end--;
                    *(end + 1) = '\0';

                    fprintf(config_file, "    - urls: [\"%s\"]\n", token);
                    token = strtok(NULL, ",");
                }
            } else if (global_config->go2rtc_stun_enabled) {
                // Use default STUN servers - multiple servers for redundancy
                fprintf(config_file, "    - urls:\n");
                fprintf(config_file, "      - \"stun:%s\"\n", global_config->go2rtc_stun_server);
                fprintf(config_file, "      - \"stun:stun1.l.google.com:19302\"\n");
                fprintf(config_file, "      - \"stun:stun2.l.google.com:19302\"\n");
                fprintf(config_file, "      - \"stun:stun3.l.google.com:19302\"\n");
                fprintf(config_file, "      - \"stun:stun4.l.google.com:19302\"\n");
            }

            // Add TURN server if configured
            log_info("go2rtc config: turn_enabled=%d, turn_server_url='%s', turn_username='%s'",
                     global_config->turn_enabled,
                     global_config->turn_server_url,
                     global_config->turn_username);
            if (global_config->turn_enabled && global_config->turn_server_url[0] != '\0') {
                log_info("go2rtc config: Adding TURN server to config");
                fprintf(config_file, "    - urls: [\"%s\"]\n", global_config->turn_server_url);
                if (global_config->turn_username[0] != '\0') {
                    fprintf(config_file, "      username: \"%s\"\n", global_config->turn_username);
                }
                if (global_config->turn_password[0] != '\0') {
                    fprintf(config_file, "      credential: \"%s\"\n", global_config->turn_password); // codeql[cpp/cleartext-storage-file] - TURN credential required by go2rtc config; file written with 0600 permissions
                }
            } else {
                log_info("go2rtc config: TURN server NOT added (enabled=%d, url_empty=%d)",
                         global_config->turn_enabled,
                         global_config->turn_server_url[0] == '\0');
            }
        }

        // Candidates configuration for NAT traversal
        fprintf(config_file, "  candidates:\n");

        // If external IP is specified, use it
        if (global_config->go2rtc_external_ip[0] != '\0') {
            fprintf(config_file, "    - \"%s:%d\"\n",
                    global_config->go2rtc_external_ip,
                    global_config->go2rtc_webrtc_listen_port > 0 ? global_config->go2rtc_webrtc_listen_port : 8555);
        } else {
            // Auto-detect external IP using wildcard
            // Use separate entries for IPv4 and IPv6 to handle both
            fprintf(config_file, "    - \"*:%d\"\n",
                    global_config->go2rtc_webrtc_listen_port > 0 ? global_config->go2rtc_webrtc_listen_port : 8555);
        }

        // Add STUN server as candidate for ICE gathering
        if (global_config->go2rtc_stun_enabled) {
            fprintf(config_file, "    - \"stun:%s\"\n", global_config->go2rtc_stun_server);
        }

        fprintf(config_file, "\n");
    }

    // Logging configuration
    fprintf(config_file, "log:\n");
    fprintf(config_file, "  level: debug\n\n");  // Use debug level for more verbose logging

    fprintf(config_file, "ffmpeg:\n");
    fprintf(config_file, "  h264: \"-codec:v libx264 -g:v 30 -preset:v superfast\"\n");
    fprintf(config_file, "  h265: \"-codec:v libx265 -g:v 30 -preset:v superfast\"\n");

    // Streams section — write overridden streams directly into config,
    // other streams will be registered dynamically via the go2rtc API.
    fprintf(config_file, "\nstreams:\n");
    {
        bool has_overridden = false;
        if (get_db_handle() != NULL) {
            int ms = g_config.max_streams > 0 ? g_config.max_streams : 32;
            stream_config_t *streams = calloc(ms, sizeof(stream_config_t));
            int count = streams ? get_all_stream_configs(streams, ms) : 0;

            for (int i = 0; i < count; i++) {
                if (!streams[i].enabled) continue;
                if (streams[i].go2rtc_source_override[0] == '\0') continue;

                has_overridden = true;

                // Escape stream name for YAML double-quoted key safety
                char escaped_name[MAX_STREAM_NAME * 2];
                yaml_escape_string(streams[i].name, escaped_name, sizeof(escaped_name));

                const char *override = streams[i].go2rtc_source_override;
                bool is_single_line = (strchr(override, '\n') == NULL);

                if (is_single_line) {
                    // Single URL: write as inline YAML scalar
                    //   "cam": rtsp://camera/stream
                    fprintf(config_file, "  \"%s\": %s\n", escaped_name, override);
                } else {
                    // Multi-line: write as indented block under the key
                    //   "cam":
                    //     - rtsp://camera/main
                    //     - ffmpeg:cam#video=h264
                    fprintf(config_file, "  \"%s\":\n", escaped_name);
                    const char *p = override;
                    while (*p) {
                        const char *eol = strchr(p, '\n');
                        if (eol) {
                            fprintf(config_file, "    %.*s\n", (int)(eol - p), p);
                            p = eol + 1;
                        } else {
                            fprintf(config_file, "    %s\n", p);
                            break;
                        }
                    }
                }
            }
            free(streams);
        }
        if (!has_overridden) {
            fprintf(config_file, "  # Streams will be added dynamically via API\n");
        }
    }

    // Global go2rtc config override from system settings
    {
        char global_override[4096] = {0};
        if (get_db_handle() != NULL
            && db_get_system_setting("go2rtc_config_override", global_override, sizeof(global_override)) == 0
            && global_override[0] != '\0') {
            fprintf(config_file, "\n# User config override\n");
            fprintf(config_file, "%s\n", global_override);
        }
    }

    fclose(config_file);
    log_info("Generated go2rtc configuration file: %s", config_path);

    // Print the content of the config file at DEBUG level to avoid
    // leaking credentials from overrides into production logs.
    FILE *read_file = fopen(config_path, "r");
    if (read_file) {
        char line[256];
        log_debug("Contents of go2rtc config file:");
        while (fgets(line, sizeof(line), read_file)) {
            // Remove newline character
            size_t len = strlen(line);
            if (len > 0 && line[len-1] == '\n') {
                line[len-1] = '\0';
            }
            log_debug("  %s", line);
        }
        fclose(read_file);
    }

    return true;
}

bool go2rtc_process_generate_startup_config(const char *binary_path,
                                            const char *config_dir,
                                            int api_port) {
    if (!config_dir || config_dir[0] == '\0') {
        log_error("Invalid config_dir for go2rtc startup config generation");
        return false;
    }

    int resolved_api_port = api_port > 0 ? api_port : 1984;
    if (!go2rtc_process_init(binary_path, config_dir, resolved_api_port)) {
        log_error("Failed to initialize go2rtc process manager for startup config generation");
        return false;
    }

    char config_path[PATH_MAX];
    int written = snprintf(config_path, sizeof(config_path), "%s/go2rtc.yaml", config_dir);
    if (written < 0 || (size_t)written >= sizeof(config_path)) {
        log_error("go2rtc config path too long for startup config generation: %s", config_dir);
        go2rtc_process_cleanup();
        return false;
    }

    bool ok = go2rtc_process_generate_config(config_path, resolved_api_port);
    go2rtc_process_cleanup();
    return ok;
}

/**
 * @brief Check if a process is a zombie
 *
 * @param pid Process ID to check
 * @return true if the process is a zombie, false otherwise
 */
static bool is_zombie_process(pid_t pid) {
    char stat_path[64];
    snprintf(stat_path, sizeof(stat_path), "/proc/%d/stat", pid);

    FILE *fp = fopen(stat_path, "r");
    if (!fp) {
        return false;  // Process doesn't exist
    }

    // Read the stat file - format: pid (comm) state ...
    char line[512];
    if (fgets(line, sizeof(line), fp)) {
        fclose(fp);

        // Find the closing paren of the command name, then the state is next
        const char *close_paren = strrchr(line, ')');
        if (close_paren && close_paren[1] == ' ') {
            char state = close_paren[2];
            if (state == 'Z') {
                return true;  // Zombie state
            }
        }
    } else {
        fclose(fp);
    }

    return false;
}

/**
 * @brief Reap zombie child processes
 *
 * This function calls waitpid with WNOHANG to reap any zombie children
 * without blocking.
 */
static void reap_zombie_children(pid_t pid) {
    int status;
    pid_t dead;

    // Reap all zombie children with WNOHANG (non-blocking)
    while ((dead = waitpid(pid, &status, WNOHANG)) > 0) {
        if (WIFEXITED(status)) {
            log_debug("Reaped zombie child process %d (exit code %d)", dead, WEXITSTATUS(status));
        } else if (WIFSIGNALED(status)) {
            log_debug("Reaped zombie child process %d (killed by signal %d)", dead, WTERMSIG(status));
        } else {
            log_debug("Reaped zombie child process %d", dead);
        }
    }
}

/**
 * @brief Wait for a specific process to terminate
 *
 * @param pid Process ID to wait for
 * @param timeout_ms Maximum time to wait in milliseconds
 * @return true if process terminated, false if timeout
 */
static bool wait_for_process_termination(pid_t pid, int timeout_ms) {
    int elapsed = 0;
    int interval = 50;  // 50ms check interval

    while (elapsed < timeout_ms) {
        // First try to reap if it's our child
        int status;
        pid_t result = waitpid(pid, &status, WNOHANG);
        // pid can be -1 for any child or -group_id for waiting on a group of
        // processes. It can also be 0 to wait for processes with the same
        // group ID as this process. In these cases, the pid returned will
        // be nonzero on success but not match the input pid.
        if (result == pid || (pid < 0 && result > 0)) {
            // Successfully reaped the process
            log_debug("Process %d successfully reaped", pid);
            return true;
        } else if (result == -1 && errno == ECHILD) {
            // Not our child, check if it still exists
            if (kill(pid, 0) != 0) {
                return true;  // Process no longer exists
            }
            // Check if it's a zombie (might be child of another process)
            if (is_zombie_process(pid)) {
                log_debug("Process %d is zombie (not our child)", pid);
                return true;  // Consider it terminated even if we can't reap it
            }
        }

        usleep(interval * 1000);
        elapsed += interval;
    }

    return false;
}

/**
 * @brief Check if a process is actually a go2rtc process
 *
 * @param pid Process ID to check
 * @return true if it's a go2rtc process, false otherwise
 */
static bool is_go2rtc_process(pid_t pid) {
    // First check if the process exists
    if (kill(pid, 0) != 0) {
        return false;
    }

    // Check if it's a zombie - zombies don't count as running processes
    if (is_zombie_process(pid)) {
        log_debug("Process %d is a zombie, not counting as go2rtc", pid);
        return false;
    }

    // Check argv[0] basename only so later arguments or directory names do not
    // produce false positives.
    if (proc_first_cmdline_arg_basename_equals(pid, "go2rtc")) {
        return true;
    }

    // Fallback: read /proc/{pid}/comm and require an exact name match.
    if (proc_comm_equals(pid, "go2rtc")) {
        return true;
    }

    return false;
}

/**
 * Sends signal to all processes matching the specified command-line.
 *
 * See documentation for kill() and waitpid() for exact meaning of pids: some
 * values have special meaning for killing process groups.
 *
 * @param name String identifying process; only used for logging
 * @param pids The array of pids
 * @param n_pids The size of the pid array
 * @param sig The signal to send to the processes
 * @param timeout_ms The amount of time to wait for each process to terminate
 * @return The number of pids reaped (zero if none are terminated)
 */
static int killall_pids(const char *name, pid_t *pids, int n_pids, int sig, int timeout_ms) {
    int i;
    char *sig_name = strsignal(sig);
    for (i = 0; i < n_pids; i++) {
        log_info("Sending %s to '%s' process with PID: %d", sig_name, name, pids[i]);
        if (kill(pids[i], sig) != 0 && errno != ESRCH) {
            log_warn("Failed to send %s to '%s' process %d: %s",
                        sig_name, name, pids[i], strerror(errno));
        }
    }
    // Reap only the children we killed here if we sent a killing signal to the
    // process. Note that any signal could, in theory, result in the death of
    // the process, but we don't want to warn if we send e.g. SIGUSR2 and the process
    // doesn't terminate.
    int n_reaped = 0;
    for (i = 0; i < n_pids; i++) {
        if (wait_for_process_termination(pids[i], timeout_ms)) {
            n_reaped++;
        } else if (sig == SIGTERM || sig == SIGKILL) {
            log_warn("Timed out waiting for '%s' process %d to terminate", name, pids[i]);
        }
    }

    return n_reaped;
}

// Sends SIGTERM to all processes matching the specified command-line
// Returns the number of processes remaining; if
// all matching processes terminate, will return 0.
static int killall_cmdline(const char *cmdline, int sig, int timeout_ms) {
    pid_t pids[64];
    int n_pids = scan_proc_for_cmdline(cmdline, pids, 64);
    int n_reaped = killall_pids(cmdline, pids, n_pids, sig, timeout_ms);

    return n_pids - n_reaped;
}

// Sends SIGTERM to all processes with the specified binary name.
// Returns the number of processes remaining; if
// all matching processes terminate, will return 0.
static int killall_argv0(const char *cmdline, int sig, int timeout_ms) {
    pid_t pids[64];
    int n_pids = scan_proc_for_argv0_basename(cmdline, pids, 64);
    int n_reaped = killall_pids(cmdline, pids, n_pids, sig, timeout_ms);

    return n_pids - n_reaped;
}

/**
 * @brief Kill all go2rtc and related supervision processes
 *
 * @return true if all processes were killed, false otherwise
 */
static bool kill_all_go2rtc_processes(void) {
    bool success = true;

    // Reap any existing zombie children first. Note that this reaps *all* child
    // processes. If any other subprocesses or threads are terminating and the
    // program expects them to be in a zombie state, this will reap them. As this
    // is only called on shutdown, this catch-all should be safe but may be
    // removed in the future.
    reap_zombie_children(WAIT_ANY);

    // First kill any s6-supervise processes related to go2rtc
    killall_cmdline("s6-supervise go2rtc", SIGTERM, 2000);

    // Also kill any s6-supervise processes related to go2rtc-healthcheck and go2rtc-log
    killall_cmdline("s6-supervise go2rtc-", SIGTERM, 2000);

    // Scan /proc for go2rtc processes (replaces "ps | grep go2rtc | awk '{print $1}'")
    // Wait up to 3 seconds for graceful termination
    int remaining = killall_argv0("go2rtc", SIGTERM, 3000);

    if (remaining > 0) {
        // Forcefully kill
        remaining = killall_argv0("go2rtc", SIGKILL, 500);
    }

    if (remaining > 0) {
        // Final verification - check one more time
        pid_t final_pids[64];
        pid_t pgids[64];
        int n_pgids = 0;
        int nfinal = scan_proc_for_argv0_basename("go2rtc", final_pids, 64);
        for (int j = 0; j < nfinal; j++) {
            pid_t pid = final_pids[j];
            if (is_go2rtc_process(pid) && !is_zombie_process(pid)) {
                log_error("go2rtc process %d still running after SIGKILL", pid);
                pid_t pgid = getpgid(pid);
                if (pgid > 0 && pgid != getpgrp()) {
                    // Use the negative of the process group ID to pass to kill()
                    pgids[n_pgids++] = -pgid;
                }
            }
        }

        if (n_pgids > 0) {
            remaining = n_pgids - killall_pids("go2rtc process group", pgids, n_pgids, SIGKILL, 500);
        }
    }

    if (remaining > 0) {
        pid_t last_pids[64];
        int nlast = scan_proc_for_argv0_basename("go2rtc", last_pids, 64);
        for (int j = 0; j < nlast; j++) {
            if (is_go2rtc_process(last_pids[j]) && !is_zombie_process(last_pids[j])) {
                log_error("go2rtc process %d could not be killed", last_pids[j]);
                success = false;
            }
        }
    }

    // Also try to remove any /dev/shm/go2rtc.yaml file that might be used by s6-supervised go2rtc
    if (access("/dev/shm/go2rtc.yaml", F_OK) == 0) {
        log_info("Removing /dev/shm/go2rtc.yaml file");
        if (unlink("/dev/shm/go2rtc.yaml") != 0) {
            log_warn("Failed to remove /dev/shm/go2rtc.yaml: %s", strerror(errno));
            success = false;
        }
    }

    // Also try to remove any /dev/shm/logs/go2rtc directory (recursive, no shell)
    if (access("/dev/shm/logs/go2rtc", F_OK) == 0) {
        log_info("Removing /dev/shm/logs/go2rtc directory");
        recursive_remove("/dev/shm/logs/go2rtc");
        if (access("/dev/shm/logs/go2rtc", F_OK) == 0) {
            log_warn("Failed to fully remove /dev/shm/logs/go2rtc directory");
            success = false;
        }
    }

    // Check whether any go2rtc process is still alive as a proxy for "ports still held"
    // (correlating /proc/net/tcp inodes to PIDs is expensive; a live-process check suffices)
    {
        pid_t leftover[64];
        int nleft = scan_proc_for_argv0_basename("go2rtc", leftover, 64);
        bool found_ports = false;
        for (int i = 0; i < nleft; i++) {
            if (is_go2rtc_process(leftover[i]) && !is_zombie_process(leftover[i])) {
                found_ports = true;
                log_warn("go2rtc process %d still alive after kill sequence", leftover[i]);
            }
        }
        if (found_ports) {
            log_warn("go2rtc may still have open network connections");
            success = false;
        }
    }

    return success;
}

bool go2rtc_process_is_running(void) {
    if (!g_initialized) {
        return false;
    }

    // If we're using an existing service, check if the service is running
    if (g_binary_path && g_binary_path[0] == '\0') {
        return is_go2rtc_running_as_service(g_api_port);
    }

    // Check if our tracked process is running
    if (g_process_pid > 0) {
        // First check if process exists
        if (kill(g_process_pid, 0) == 0) {
            // Process exists, now check if it's actually a go2rtc process
            if (is_go2rtc_process(g_process_pid)) {
                return true;
            } else {
                log_warn("Tracked PID %d exists but is not a go2rtc process", g_process_pid);
                g_process_pid = -1; // Reset our tracked PID
            }
        } else {
            log_info("Tracked go2rtc process (PID: %d) is no longer running", g_process_pid);
            g_process_pid = -1; // Reset our tracked PID
        }
    }

    // Scan /proc for go2rtc processes (replaces "ps | grep go2rtc | awk '{print $1}'")
    bool found = false;
    {
        pid_t scan_pids[64];
        int n = scan_proc_for_argv0_basename("go2rtc", scan_pids, 64);
        for (int i = 0; i < n && !found; i++) {
            if (is_go2rtc_process(scan_pids[i])) {
                if (g_process_pid <= 0 || g_process_pid != scan_pids[i]) {
                    log_warn("Found untracked go2rtc process with PID: %d", scan_pids[i]);
                    g_process_pid = scan_pids[i];
                }
                found = true;
            }
        }
    }

    // If we didn't find any go2rtc processes, also check if the port is in use
    if (!found) {
        if (check_tcp_port_open(g_api_port)) {
            log_warn("Port %d is in use but no go2rtc process found in /proc", g_api_port);
            // Check if it responds like go2rtc
            if (is_go2rtc_running_as_service(g_api_port)) {
                log_info("Port %d is responding like go2rtc, assuming it's running as a service", g_api_port);
                found = true;
            }
        }
    }

    return found;
}

bool go2rtc_process_start(int api_port) {
    if (!g_initialized) {
        log_error("go2rtc process manager not initialized");
        return false;
    }

    // Always regenerate the go2rtc config file fresh at startup to avoid
    // stale/corrupted configs from prior versions causing stream errors
    // (see issue #165). The file is opened with O_TRUNC so any old content
    // is discarded.
    log_info("Regenerating go2rtc configuration file fresh at startup");
    if (!go2rtc_process_generate_config(g_config_path, api_port)) {
        log_warn("Failed to regenerate go2rtc configuration at startup");
        // Continue anyway - the old config may still work
    }

    // Check if go2rtc is already running as a service
    if (is_go2rtc_running_as_service(api_port)) {
        log_info("go2rtc is already running as a service on port %d, using existing service", api_port);

        // Try to get the RTSP port from the API with multiple retries
        int retries = 5;
        bool got_rtsp_port = false;

        while (retries > 0 && !got_rtsp_port) {
            if (go2rtc_api_get_server_info(&g_rtsp_port)) {
                log_info("Retrieved RTSP port from go2rtc API: %d", g_rtsp_port);
                got_rtsp_port = true;
            } else {
                log_warn("Could not retrieve RTSP port from go2rtc API, retrying... (%d retries left)", retries);
                sleep(1);
                retries--;
            }
        }

        if (!got_rtsp_port) {
            log_warn("Could not retrieve RTSP port from go2rtc API after multiple attempts, using default: %d", g_rtsp_port);
        }

        return true;
    }

    // Check if go2rtc is already running as a process we started
    if (go2rtc_process_is_running()) {
        // Check if the port is actually in use via /proc/net/tcp (no shell)
        if (check_tcp_port_open(api_port)) {
            log_info("go2rtc is already running and listening on port %d", api_port);
            // Try to get the RTSP port from the API with multiple retries
            int retries = 5;
            bool got_rtsp_port = false;

            while (retries > 0 && !got_rtsp_port) {
                if (go2rtc_api_get_server_info(&g_rtsp_port)) {
                    log_info("Retrieved RTSP port from go2rtc API: %d", g_rtsp_port);
                    got_rtsp_port = true;
                } else {
                    log_warn("Could not retrieve RTSP port from go2rtc API, retrying... (%d retries left)", retries);
                    sleep(1);
                    retries--;
                }
            }

            if (!got_rtsp_port) {
                log_warn("Could not retrieve RTSP port from go2rtc API after multiple attempts, using default: %d", g_rtsp_port);
            }

            return true;
        } else {
            log_warn("go2rtc is running but not listening on port %d", api_port);
            return false;
        }
    }

    // If we don't have a binary path (using existing service), but no service was detected,
    // we can't start go2rtc
    if (g_binary_path == NULL || g_binary_path[0] == '\0') {
        log_error("No go2rtc binary available and no running service detected");
        return false;
    }

    // Check if the port is already in use by another process (/proc/net/tcp, no shell)
    if (check_tcp_port_open(api_port)) {
        log_warn("Port %d is already in use", api_port);
        log_error("Cannot start go2rtc because port %d is already in use", api_port);
        return false;
    }

    // Seed g_rtsp_port from the user-configured value before starting the process.
    // This ensures that if the post-start API call fails (e.g. because another
    // service such as mediamtx already holds the default port 8554), the fallback
    // still reflects what go2rtc was actually configured to listen on rather than
    // silently using the hardcoded default.
    g_rtsp_port = (g_config.go2rtc_rtsp_port > 0) ? g_config.go2rtc_rtsp_port : 8554;

    // Create a symbolic link from /dev/shm/go2rtc.yaml to our config file
    // This ensures that even if something tries to use the /dev/shm path, it will use our config
    if (access("/dev/shm/go2rtc.yaml", F_OK) == 0) {
        unlink("/dev/shm/go2rtc.yaml");
    }
    if (symlink(g_config_path, "/dev/shm/go2rtc.yaml") != 0) {
        log_warn("Failed to create symlink from /dev/shm/go2rtc.yaml to %s: %s",
                g_config_path, strerror(errno));
        // Continue anyway, this is not critical
    }

    // Fork a new process
    pid_t pid = fork();

    if (pid < 0) {
        // Fork failed
        log_error("Failed to fork process for go2rtc: %s", strerror(errno));
        return false;
    } else if (pid == 0) {
        // Child process

        // Request to receive SIGTERM when parent dies
        // This ensures go2rtc is terminated even if lightNVR is killed with SIGKILL
        if (prctl(PR_SET_PDEATHSIG, SIGTERM) == -1) {
            fprintf(stderr, "Warning: Failed to set parent death signal: %s\n", strerror(errno));
            // Continue anyway, this is not critical but means we might leak go2rtc on SIGKILL
        }

        // Double-check parent is still alive after prctl (race condition protection)
        // If parent died between fork and prctl, we should exit now
        if (getppid() == 1) {
            fprintf(stderr, "Parent died before prctl completed, exiting\n");
            exit(EXIT_FAILURE);
        }

        // Redirect stdout and stderr to log files
        char log_path[PATH_MAX]; // Use PATH_MAX to accommodate full filesystem paths

        // Extract directory from g_config->log_file
        if (g_config.log_file[0] != '\0') {
            char log_dir[PATH_MAX];
            safe_strcpy(log_dir, g_config.log_file, sizeof(log_dir), 0);

            // Find the last slash to get the directory
            char *last_slash = strrchr(log_dir, '/');
            if (last_slash) {
                // Truncate at the last slash to get just the directory
                *(last_slash + 1) = '\0';
                // Create the go2rtc log path in the same directory as the main log file
                snprintf(log_path, sizeof(log_path), "%sgo2rtc.log", log_dir);
            } else {
                // No directory in the path, fall back to g_config_dir
                snprintf(log_path, sizeof(log_path), "%s/go2rtc.log", g_config_dir);
            }
        } else {
            // If g_config.log_file is empty, fall back to g_config_dir
            snprintf(log_path, sizeof(log_path), "%s/go2rtc.log", g_config_dir);
        }

        // Resolve the binary path to a canonical absolute path to prevent
        // path traversal or symlink attacks (CWE-426 / CWE-78).
        char resolved_binary[PATH_MAX];
        if (realpath(g_binary_path, resolved_binary) == NULL) {
            fprintf(stderr, "Failed to resolve go2rtc binary path '%s': %s\n",
                    g_binary_path, strerror(errno));
            exit(EXIT_FAILURE);
        }

        struct stat st;
        if (stat(resolved_binary, &st) == 0) {
            if (!S_ISREG(st.st_mode)) {
                fprintf(stderr, "go2rtc path is not a file: %s\n", resolved_binary);
                exit(EXIT_FAILURE);
            }
        } else {
            // This really shouldn't happen except in a race condition or if something else
            // really nasty is happening, but we'll cover the case anyway.
            fprintf(stderr, "go2rtc path no longer exists? %s (%s)\n", resolved_binary, strerror(errno));
            exit(EXIT_FAILURE);
        }

        // Log the path we're using for the log file
        fprintf(stderr, "Using go2rtc log file: %s\n", log_path);

        int log_fd = open(log_path, O_WRONLY | O_CREAT | O_APPEND, 0644);
        if (log_fd == -1) {
            fprintf(stderr, "Failed to open log file: %s\n", log_path);
            exit(EXIT_FAILURE);
        }

        dup2(log_fd, STDOUT_FILENO);
        dup2(log_fd, STDERR_FILENO);
        close(log_fd);

        // Execute go2rtc with explicit config path (using correct argument format).
        // Always use "go2rtc" as argv[0] (the process name visible in /proc/<pid>/cmdline
        // and /proc/<pid>/comm) regardless of the actual binary path, so that
        // scan_proc_for_argv0_basename("go2rtc") can reliably find the process even when
        // g_binary_path is a full path or an alternate filename.
        log_info("Executing go2rtc with command: %s --config %s", resolved_binary, g_config_path);
        execl(resolved_binary, "go2rtc", "--config", g_config_path, NULL);

        // If execl returns, it failed
        fprintf(stderr, "Failed to execute go2rtc: %s\n", strerror(errno));
        exit(EXIT_FAILURE);
    } else {
        // Parent process
        g_process_pid = pid;
        log_info("Started go2rtc process with PID: %d", pid);

        // Wait a moment for the process to start
        sleep(1);

        // Verify the process is still running
        reap_zombie_children(pid);
        if (kill(pid, 0) != 0) {
            log_error("go2rtc process %d failed to start", pid);
            g_process_pid = -1;
            return false;
        }

        // Wait for the API to be ready with increased retries
        log_info("Waiting for go2rtc API to be ready...");
        int api_retries = 10;
        bool api_ready = false;

        while (api_retries > 0 && !api_ready) {
            // Use libcurl to check if the API is ready
            CURL *curl;
            CURLcode res;
            char url[256];
            long http_code = 0;

            reap_zombie_children(pid);
            if (kill(pid, 0) != 0) {
                log_error("go2rtc process %d no longer running", pid);
                g_process_pid = -1;
                return false;
            }

            // Initialize curl
            curl = curl_easy_init();
            if (!curl) {
                log_warn("Failed to initialize curl");
                sleep(1);
                api_retries--;
                continue;
            }

            // Format the URL for the API endpoint
            snprintf(url, sizeof(url), "http://localhost:%d" GO2RTC_BASE_PATH "/api", api_port);

            // Set curl options
            curl_easy_setopt(curl, CURLOPT_URL, url);
            curl_easy_setopt(curl, CURLOPT_WRITEFUNCTION, discard_response_data);
            curl_easy_setopt(curl, CURLOPT_TIMEOUT, 2L); // 2 second timeout
            curl_easy_setopt(curl, CURLOPT_CONNECTTIMEOUT, 2L); // 2 second connect timeout
            curl_easy_setopt(curl, CURLOPT_NOSIGNAL, 1L); // Prevent curl from using signals

            // Perform the request
            res = curl_easy_perform(curl);

            // Check for errors
            if (res != CURLE_OK) {
                log_warn("Curl request failed: %s", curl_easy_strerror(res));
                curl_easy_cleanup(curl);
                sleep(1);
                api_retries--;
                continue;
            }

            // Get the HTTP response code
            curl_easy_getinfo(curl, CURLINFO_RESPONSE_CODE, &http_code);

            // Clean up
            curl_easy_cleanup(curl);

            if (http_code == 200 || http_code == 401) {
                log_info("go2rtc API is ready (HTTP %ld)", http_code);
                api_ready = true;
                break;
            }

            log_info("Waiting for go2rtc API to be ready... (%d retries left)", api_retries);
            sleep(1);
            api_retries--;
        }

        if (!api_ready) {
            log_warn("go2rtc API did not become ready within timeout, but process is running");
            // Continue anyway, as the process might still be starting up
        }

        // Try to get the RTSP port from the API with multiple retries
        log_info("Attempting to retrieve RTSP port from go2rtc API...");
        int retries = 10;  // Increased retries
        bool got_rtsp_port = false;

        while (retries > 0 && !got_rtsp_port) {
            if (go2rtc_api_get_server_info(&g_rtsp_port)) {
                log_info("Retrieved RTSP port from go2rtc API: %d", g_rtsp_port);
                got_rtsp_port = true;
            } else {
                log_warn("Could not retrieve RTSP port from go2rtc API, retrying... (%d retries left)", retries);
                sleep(1);
                retries--;
            }
        }

        if (!got_rtsp_port) {
            log_warn("Could not retrieve RTSP port from go2rtc API after multiple attempts, using default: %d", g_rtsp_port);
        }

        return true;
    }
}

/**
 * @brief Get the RTSP port used by go2rtc
 *
 * @return int The RTSP port
 */
int go2rtc_process_get_rtsp_port(void) {
    return g_rtsp_port;
}

bool go2rtc_process_stop(void) {
    if (!g_initialized) {
        log_error("go2rtc process manager not initialized");
        return false;
    }

    // Only stop go2rtc if we started it (g_binary_path is not empty)
    if (g_binary_path && g_binary_path[0] != '\0') {
        log_info("Stopping go2rtc process that we started");

        // Kill all go2rtc processes, not just the one we started
        bool result = kill_all_go2rtc_processes();

        // Reset our tracked PID
        g_process_pid = -1;

        if (result) {
            log_info("Stopped all go2rtc processes");
        } else {
            log_warn("Some go2rtc processes may still be running");
        }

        return result;
    } else {
        log_info("Not stopping go2rtc as we're using an existing service");
        return true; // Return success as we're intentionally not stopping it
    }
}

void go2rtc_process_cleanup(void) {
    if (!g_initialized) {
        return;
    }

    // Only stop go2rtc if we started it (g_binary_path is not empty)
    if (g_binary_path && g_binary_path[0] != '\0') {
        log_info("Stopping go2rtc process that we started during cleanup");
        kill_all_go2rtc_processes();
    } else {
        log_info("Not stopping go2rtc during cleanup as we're using an existing service");
    }

    // Free allocated memory
    free(g_binary_path);
    free(g_config_dir);
    free(g_config_path);

    g_binary_path = NULL;
    g_config_dir = NULL;
    g_config_path = NULL;
    g_process_pid = -1;
    g_api_port = 1984;
    g_initialized = false;

    log_info("go2rtc process manager cleaned up");
}

/**
 * @brief Get the PID of the go2rtc process
 *
 * This function returns the tracked PID of the go2rtc process.
 * If the process is not running or not tracked, it returns -1.
 *
 * @return int The process ID, or -1 if not running
 */
int go2rtc_process_get_pid(void) {
    // First check if our tracked process is still running
    if (g_process_pid > 0) {
        if (kill(g_process_pid, 0) == 0) {
            return g_process_pid;
        } else {
            // Process no longer exists
            g_process_pid = -1;
        }
    }

    // Scan /proc for go2rtc processes (by exact argv[0] basename)
    pid_t scan_pids[64];
    int n = scan_proc_for_argv0_basename("go2rtc", scan_pids, 64);
    for (int i = 0; i < n; i++) {
        if (is_go2rtc_process(scan_pids[i])) {
            g_process_pid = scan_pids[i];  // Update our tracked PID
            return scan_pids[i];
        }
    }
    return -1;
}
