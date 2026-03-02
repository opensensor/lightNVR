/**
 * @file go2rtc_process.c
 * @brief Implementation of the go2rtc process management module
 */

#include "video/go2rtc/go2rtc_process.h"
#include "video/go2rtc/go2rtc_api.h"
#include "core/logger.h"
#include "core/config.h"

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
            strncpy(linecopy, line, sizeof(linecopy) - 1);
            linecopy[sizeof(linecopy) - 1] = '\0';

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
 *        into @p out.  Falls back to writing @p name verbatim if not found.
 *
 * Replaces: popen("which <name> 2>/dev/null")
 */
static void find_binary_in_path(const char *name, char *out, size_t out_size) {
    const char *path_env = getenv("PATH");
    if (!path_env) path_env = "/usr/local/bin:/usr/bin:/bin";

    char path_copy[4096];
    strncpy(path_copy, path_env, sizeof(path_copy) - 1);
    path_copy[sizeof(path_copy) - 1] = '\0';

    char *saveptr = NULL;
    const char *dir = strtok_r(path_copy, ":", &saveptr);
    while (dir) {
        char candidate[PATH_MAX];
        int n = snprintf(candidate, sizeof(candidate), "%s/%s", dir, name);
        if (n > 0 && n < (int)sizeof(candidate) && access(candidate, X_OK) == 0) {
            strncpy(out, candidate, out_size - 1);
            out[out_size - 1] = '\0';
            return;
        }
        dir = strtok_r(NULL, ":", &saveptr);
    }
    // Not found – use bare name and let execvp search PATH at exec time
    strncpy(out, name, out_size - 1);
    out[out_size - 1] = '\0';
}

/**
 * @brief Read /proc/<pid>/comm and check whether it contains @p name.
 *
 * Replaces: system("ps -p <pid> -o comm= | grep -q <name>")
 *
 * @return true if /proc/<pid>/comm contains @p name
 */
static bool proc_comm_contains(pid_t pid, const char *name) {
    char path[64];
    snprintf(path, sizeof(path), "/proc/%d/comm", pid);
    FILE *fp = fopen(path, "r");
    if (!fp) return false;
    char comm[256] = {0};
    bool ok = (fgets(comm, sizeof(comm), fp) != NULL);
    fclose(fp);
    return ok && (strstr(comm, name) != NULL);
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
    strncpy(parent, path, sizeof(parent) - 1);
    parent[sizeof(parent) - 1] = '\0';

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
        int n = scan_proc_for_cmdline("go2rtc", pids, 64);
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
    char path[PATH_MAX] = {0};
    find_binary_in_path("go2rtc", path, sizeof(path));

    if (path[0] != '\0' && access(path, X_OK) == 0) {
        log_info("Found go2rtc binary in PATH: %s", path);
        strncpy(binary_path, path, buffer_size - 1);
        binary_path[buffer_size - 1] = '\0';
        return true;
    }

    // If not found in PATH, just use "go2rtc" and let execl resolve it
    strncpy(binary_path, "go2rtc", buffer_size - 1);
    binary_path[buffer_size - 1] = '\0';
    log_info("Using 'go2rtc' from PATH");
    return true;
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
    struct stat st = {0};
    if (stat(config_dir, &st) == -1) {
        if (mkdir(config_dir, 0755) == -1) {
            log_error("Failed to create go2rtc config directory: %s", config_dir);
            return false;
        }
    }

    // Store config directory
    g_config_dir = strdup(config_dir);

    // Create config path
    size_t config_path_len = strlen(config_dir) + strlen("/go2rtc.yaml") + 1;
    g_config_path = malloc(config_path_len);
    if (!g_config_path) {
        log_error("Memory allocation failed for config path");
        free(g_config_dir);
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
            strncpy(final_binary_path, binary_path, sizeof(final_binary_path) - 1);
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
        log_error("Failed to open go2rtc config file for writing: %s", config_path);
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
                strncpy(ice_servers_copy, global_config->go2rtc_ice_servers, sizeof(ice_servers_copy) - 1);
                ice_servers_copy[sizeof(ice_servers_copy) - 1] = '\0';

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

    // Streams section (will be populated dynamically)
    fprintf(config_file, "streams:\n");
    fprintf(config_file, "  # Streams will be added dynamically\n");

    fclose(config_file);
    log_info("Generated go2rtc configuration file: %s", config_path);

    // Print the content of the config file for debugging
    FILE *read_file = fopen(config_path, "r");
    if (read_file) {
        char line[256];
        log_info("Contents of go2rtc config file:");
        while (fgets(line, sizeof(line), read_file)) {
            // Remove newline character
            size_t len = strlen(line);
            if (len > 0 && line[len-1] == '\n') {
                line[len-1] = '\0';
            }
            log_info("  %s", line);
        }
        fclose(read_file);
    }

    return true;
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
static void reap_zombie_children(void) {
    int status;
    pid_t pid;

    // Reap all zombie children with WNOHANG (non-blocking)
    while ((pid = waitpid(-1, &status, WNOHANG)) > 0) {
        if (WIFEXITED(status)) {
            log_debug("Reaped zombie child process %d (exit code %d)", pid, WEXITSTATUS(status));
        } else if (WIFSIGNALED(status)) {
            log_debug("Reaped zombie child process %d (killed by signal %d)", pid, WTERMSIG(status));
        } else {
            log_debug("Reaped zombie child process %d", pid);
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
        if (result == pid) {
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
    char proc_path[64];

    // First check if the process exists
    if (kill(pid, 0) != 0) {
        return false;
    }

    // Check if it's a zombie - zombies don't count as running processes
    if (is_zombie_process(pid)) {
        log_debug("Process %d is a zombie, not counting as go2rtc", pid);
        return false;
    }

    // Check if it's a go2rtc process by examining /proc/{pid}/cmdline
    snprintf(proc_path, sizeof(proc_path), "/proc/%d/cmdline", pid);
    FILE *fp = fopen(proc_path, "r");
    if (fp) {
        char cmdline[1024] = {0};
        size_t bytes_read = fread(cmdline, 1, sizeof(cmdline) - 1, fp);
        fclose(fp);

        if (bytes_read > 0) {
            // Replace null bytes with spaces for easier searching
            for (size_t i = 0; i < bytes_read; i++) {
                if (cmdline[i] == '\0') {
                    cmdline[i] = ' ';
                }
            }

            // Check if cmdline contains go2rtc as the executable name
            if (strstr(cmdline, "go2rtc") != NULL) {
                return true;
            }
        }
    }

    // Alternative: read /proc/{pid}/comm and check for "go2rtc"
    if (proc_comm_contains(pid, "go2rtc")) {
        return true;
    }

    return false;
}

/**
 * @brief Kill all go2rtc and related supervision processes
 *
 * @return true if all processes were killed, false otherwise
 */
static bool kill_all_go2rtc_processes(void) {
    bool success = true;
    pid_t pids_to_kill[64];  // Track PIDs we've sent signals to
    int num_pids = 0;

    // Reap any existing zombie children first
    reap_zombie_children();

    // First kill any s6-supervise processes related to go2rtc
    {
        pid_t s6_pids[64];
        int ns6 = scan_proc_for_cmdline("s6-supervise go2rtc", s6_pids, 64);
        bool found_s6 = (ns6 > 0);
        for (int i = 0; i < ns6; i++) {
            log_info("Killing s6-supervise process with PID: %d", s6_pids[i]);
            if (kill(s6_pids[i], SIGTERM) != 0 && errno != ESRCH) {
                log_warn("Failed to send SIGTERM to s6-supervise process %d: %s",
                         s6_pids[i], strerror(errno));
            }
        }
        if (found_s6) {
            sleep(2);
            reap_zombie_children();
        }
    }

    // Also kill any s6-supervise processes related to go2rtc-healthcheck and go2rtc-log
    {
        pid_t s6_pids[64];
        int ns6 = scan_proc_for_cmdline("s6-supervise go2rtc-", s6_pids, 64);
        bool found_s6 = (ns6 > 0);
        for (int i = 0; i < ns6; i++) {
            log_info("Killing s6-supervise process with PID: %d", s6_pids[i]);
            if (kill(s6_pids[i], SIGTERM) != 0 && errno != ESRCH) {
                log_warn("Failed to send SIGTERM to s6-supervise process %d: %s",
                         s6_pids[i], strerror(errno));
            }
        }
        if (found_s6) {
            sleep(2);
            reap_zombie_children();
        }
    }

    // Scan /proc for go2rtc processes (replaces "ps | grep go2rtc | awk '{print $1}'")
    {
        pid_t scan_pids[64];
        int nscan = scan_proc_for_cmdline("go2rtc", scan_pids, 64);
        num_pids = 0;
        for (int i = 0; i < nscan && num_pids < 64; i++) {
            pid_t pid = scan_pids[i];
            if (is_go2rtc_process(pid)) {
                pids_to_kill[num_pids++] = pid;
                log_info("Killing go2rtc process with PID: %d", pid);
                if (kill(pid, SIGTERM) != 0 && errno != ESRCH) {
                    log_warn("Failed to send SIGTERM to go2rtc process %d: %s",
                             pid, strerror(errno));
                }
            }
        }

        // Wait for processes to terminate with proper reaping
        if (num_pids > 0) {
            log_info("Waiting for %d go2rtc processes to terminate...", num_pids);

            // Wait up to 3 seconds for graceful termination
            bool all_terminated = true;
            for (int i = 0; i < num_pids; i++) {
                if (!wait_for_process_termination(pids_to_kill[i], 3000)) {
                    all_terminated = false;
                }
            }

            // Reap any zombies that might have been created
            reap_zombie_children();

            // Check if any processes are still running and force kill them
            if (!all_terminated) {
                pid_t still_pids[64];
                int nstill = scan_proc_for_cmdline("go2rtc", still_pids, 64);
                for (int j = 0; j < nstill; j++) {
                    if (is_go2rtc_process(still_pids[j])) {
                        log_warn("go2rtc process %d still running, sending SIGKILL", still_pids[j]);
                        if (kill(still_pids[j], SIGKILL) != 0 && errno != ESRCH) {
                            log_error("Failed to send SIGKILL to go2rtc process %d: %s",
                                      still_pids[j], strerror(errno));
                        }
                    }
                }
                // Wait for SIGKILL to take effect and reap
                usleep(500000);  // 500ms
                reap_zombie_children();
            }

            // Final verification - check one more time
            {
                pid_t final_pids[64];
                int nfinal = scan_proc_for_cmdline("go2rtc", final_pids, 64);
                bool still_running = false;
                for (int j = 0; j < nfinal; j++) {
                    pid_t pid = final_pids[j];
                    if (is_go2rtc_process(pid) && !is_zombie_process(pid)) {
                        still_running = true;
                        log_error("go2rtc process %d still running after SIGKILL", pid);
                        pid_t pgid = getpgid(pid);
                        if (pgid > 0 && pgid != getpgrp()) {
                            log_info("Killing process group %d", pgid);
                            killpg(pgid, SIGKILL);
                        }
                    }
                }

                if (still_running) {
                    usleep(500000);
                    reap_zombie_children();

                    pid_t last_pids[64];
                    int nlast = scan_proc_for_cmdline("go2rtc", last_pids, 64);
                    for (int j = 0; j < nlast; j++) {
                        if (is_go2rtc_process(last_pids[j]) && !is_zombie_process(last_pids[j])) {
                            log_error("go2rtc process %d could not be killed", last_pids[j]);
                            success = false;
                        }
                    }
                }
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
        int nleft = scan_proc_for_cmdline("go2rtc", leftover, 64);
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
        int n = scan_proc_for_cmdline("go2rtc", scan_pids, 64);
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

    // Generate configuration file
    if (!go2rtc_process_generate_config(g_config_path, api_port)) {
        log_error("Failed to generate go2rtc configuration");
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
        char log_path[1024]; // Use a reasonable fixed size instead of PATH_MAX

        // Extract directory from g_config->log_file
        char log_dir[1024] = {0};
        if (g_config.log_file[0] != '\0') {
            strncpy(log_dir, g_config.log_file, sizeof(log_dir) - 1);

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

        // Log the path we're using for the log file
        log_info("Using go2rtc log file: %s", log_path);

        int log_fd = open(log_path, O_WRONLY | O_CREAT | O_APPEND, 0644);
        if (log_fd == -1) {
            log_error("Failed to open log file: %s", log_path);
            exit(EXIT_FAILURE);
        }

        dup2(log_fd, STDOUT_FILENO);
        dup2(log_fd, STDERR_FILENO);
        close(log_fd);

        // Resolve the binary path to a canonical absolute path to prevent
        // path traversal or symlink attacks (CWE-426 / CWE-78).
        char resolved_binary[PATH_MAX];
        if (realpath(g_binary_path, resolved_binary) == NULL) {
            fprintf(stderr, "Failed to resolve go2rtc binary path '%s': %s\n",
                    g_binary_path, strerror(errno));
            exit(EXIT_FAILURE);
        }

        // Execute go2rtc with explicit config path (using correct argument format).
        // Always use "go2rtc" as argv[0] (the process name visible in /proc/<pid>/cmdline
        // and /proc/<pid>/comm) regardless of the actual binary path, so that
        // scan_proc_for_cmdline("go2rtc") can reliably find the process even when
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

    // Scan /proc for go2rtc processes (replaces "ps | grep go2rtc | awk '{print $1}'")
    pid_t scan_pids[64];
    int n = scan_proc_for_cmdline("go2rtc", scan_pids, 64);
    for (int i = 0; i < n; i++) {
        if (is_go2rtc_process(scan_pids[i])) {
            g_process_pid = scan_pids[i];  // Update our tracked PID
            return scan_pids[i];
        }
    }
    return -1;
}
