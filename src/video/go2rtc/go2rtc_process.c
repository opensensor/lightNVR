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
#include <sys/prctl.h>
#include <fcntl.h>
#include <errno.h>
#include <limits.h>
#include <curl/curl.h>

// Define PATH_MAX if not defined
#ifndef PATH_MAX
#define PATH_MAX 4096
#endif

extern config_t g_config;

// Process management variables
static char *g_binary_path = NULL;
static char *g_config_dir = NULL;
static char *g_config_path = NULL;
static pid_t g_process_pid = -1;
static bool g_initialized = false;
static int g_rtsp_port = 8554; // Default RTSP port

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
    // Check if the API port is in use by any process
    char cmd[128];
    snprintf(cmd, sizeof(cmd), "netstat -tlpn 2>/dev/null | grep ':%d' | grep -v grep", api_port);
    FILE *fp = popen(cmd, "r");
    if (!fp) {
        log_warn("Failed to execute netstat command");
        return false;
    }

    char netstat_line[256];
    bool port_in_use = false;

    if (fgets(netstat_line, sizeof(netstat_line), fp)) {
        port_in_use = true;

        // Check if the process using the port is go2rtc
        if (strstr(netstat_line, "go2rtc") != NULL) {
            log_debug("go2rtc is already running as a service on port %d", api_port);
            pclose(fp);
            return true;
        }
    }

    pclose(fp);

    if (port_in_use) {
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
        snprintf(url, sizeof(url), "http://localhost:%d/api/streams", api_port);

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
    // Try to find go2rtc in PATH
    FILE *fp = popen("which go2rtc 2>/dev/null", "r");
    if (fp) {
        char path[PATH_MAX] = {0};
        if (fgets(path, sizeof(path), fp)) {
            // Remove trailing newline
            size_t len = strlen(path);
            if (len > 0 && path[len-1] == '\n') {
                path[len-1] = '\0';
            }

            if (access(path, X_OK) == 0) {
                log_info("Found go2rtc binary in PATH: %s", path);
                strncpy(binary_path, path, buffer_size - 1);
                binary_path[buffer_size - 1] = '\0';
                pclose(fp);
                return true;
            }
        }
        pclose(fp);
    }

    // If not found in PATH, just use "go2rtc" and let the system resolve it
    strncpy(binary_path, "go2rtc", buffer_size - 1);
    binary_path[buffer_size - 1] = '\0';
    log_info("Using 'go2rtc' from PATH");
    return true;
}

bool go2rtc_process_init(const char *binary_path, const char *config_dir) {
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

    // Check if go2rtc is already running as a service
    if (is_go2rtc_running_as_service(1984)) {
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

    FILE *config_file = fopen(config_path, "w");
    if (!config_file) {
        log_error("Failed to open go2rtc config file for writing: %s", config_path);
        return false;
    }

    // Get global config for authentication settings
    config_t *global_config = &g_config;

    // Write basic configuration
    fprintf(config_file, "# go2rtc configuration generated by NVR software\n\n");

    // API configuration
    fprintf(config_file, "api:\n");
    fprintf(config_file, "  listen: :%d\n", api_port);

    // Use wildcard for CORS origin to support both localhost and 127.0.0.1
    // This allows the web interface to access go2rtc API from any local address
    fprintf(config_file, "  origin: '*'\n");
    fprintf(config_file, "  allow: 'GET, POST, OPTIONS'\n");  // Allow these methods for CORS
    fprintf(config_file, "  headers: 'Origin, X-Requested-With, Content-Type, Accept, Authorization'\n");  // Allow these headers for CORS

    // Add authentication if enabled in the main application
    if (global_config->web_auth_enabled) {
        fprintf(config_file, "  auth:\n");
        fprintf(config_file, "    username: %s\n", global_config->web_username);
        fprintf(config_file, "    password: %s\n", global_config->web_password);
    }

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
        if (global_config->go2rtc_stun_enabled || global_config->go2rtc_ice_servers[0] != '\0') {
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
 * @brief Check if a process is actually a go2rtc process
 *
 * @param pid Process ID to check
 * @return true if it's a go2rtc process, false otherwise
 */
static bool is_go2rtc_process(pid_t pid) {
    char cmd[128];
    char proc_path[64];

    // First check if the process exists
    if (kill(pid, 0) != 0) {
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

    // Alternative method using ps
    snprintf(cmd, sizeof(cmd), "ps -p %d -o comm= | grep -q go2rtc", pid);
    if (system(cmd) == 0) {
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

    // First kill any s6-supervise processes related to go2rtc
    FILE *fp = popen("pgrep -f 's6-supervise go2rtc'", "r");
    if (!fp) {
        log_error("Failed to execute pgrep command for s6-supervise");
        success = false;
    } else {
        char line[32]; // Enough for a PID
        bool found_s6 = false;

        while (fgets(line, sizeof(line), fp)) {
            pid_t pid = atoi(line);
            if (pid > 0) {
                found_s6 = true;
                log_info("Killing s6-supervise process with PID: %d", pid);

                // Send SIGTERM first
                if (kill(pid, SIGTERM) != 0) {
                    log_warn("Failed to send SIGTERM to s6-supervise process %d: %s", pid, strerror(errno));
                    success = false;
                }
            }
        }

        pclose(fp);

        // If we found s6 processes, wait a moment for them to terminate
        if (found_s6) {
            sleep(2); // Increased wait time
        }
    }

    // Also kill any s6-supervise processes related to go2rtc-healthcheck and go2rtc-log
    fp = popen("pgrep -f 's6-supervise go2rtc-'", "r");
    if (fp) {
        char line[32]; // Enough for a PID
        bool found_s6 = false;

        while (fgets(line, sizeof(line), fp)) {
            pid_t pid = atoi(line);
            if (pid > 0) {
                found_s6 = true;
                log_info("Killing s6-supervise process with PID: %d", pid);

                // Send SIGTERM first
                if (kill(pid, SIGTERM) != 0) {
                    log_warn("Failed to send SIGTERM to s6-supervise process %d: %s", pid, strerror(errno));
                    success = false;
                }
            }
        }

        pclose(fp);

        // If we found s6 processes, wait a moment for them to terminate
        if (found_s6) {
            sleep(2); // Increased wait time
        }
    }

    // Use a more compatible command for BusyBox systems
    fp = popen("ps | grep go2rtc | grep -v grep | awk '{print $1}'", "r");
    if (!fp) {
        log_error("Failed to execute ps command for go2rtc");
        success = false;
    } else {
        char line[32]; // Enough for a PID
        bool found_processes = false;

        while (fgets(line, sizeof(line), fp)) {
            pid_t pid = atoi(line);
            if (pid > 0) {
                // Verify this is actually a go2rtc process
                if (is_go2rtc_process(pid)) {
                    found_processes = true;
                    log_info("Killing go2rtc process with PID: %d", pid);

                    // Send SIGTERM first
                    if (kill(pid, SIGTERM) != 0) {
                        log_warn("Failed to send SIGTERM to go2rtc process %d: %s", pid, strerror(errno));
                        success = false;
                    }
                }
            }
        }

        pclose(fp);

        // Wait a moment for processes to terminate
        if (found_processes) {
            sleep(3); // Increased wait time

            // Check if any processes are still running and force kill them
            fp = popen("ps | grep go2rtc | grep -v grep | awk '{print $1}'", "r");
            if (!fp) {
                log_error("Failed to execute second ps command");
                success = false;
            } else {
                while (fgets(line, sizeof(line), fp)) {
                    pid_t pid = atoi(line);
                    if (pid > 0 && is_go2rtc_process(pid)) {
                        log_warn("go2rtc process %d still running, sending SIGKILL", pid);
                        if (kill(pid, SIGKILL) != 0) {
                            log_error("Failed to send SIGKILL to go2rtc process %d: %s", pid, strerror(errno));
                            success = false;
                        }
                    }
                }

                pclose(fp);

                // Wait again and check one more time
                sleep(1);
                fp = popen("ps | grep go2rtc | grep -v grep | awk '{print $1}'", "r");
                if (fp) {
                    bool still_running = false;
                    while (fgets(line, sizeof(line), fp)) {
                        pid_t pid = atoi(line);
                        if (pid > 0 && is_go2rtc_process(pid)) {
                            still_running = true;
                            log_error("go2rtc process %d still running after SIGKILL", pid);

                            // Try one more extreme measure - use kill -9
                            char kill_cmd[64];
                            snprintf(kill_cmd, sizeof(kill_cmd), "kill -9 %d 2>/dev/null", pid);
                            system(kill_cmd);
                        }
                    }
                    pclose(fp);

                    if (still_running) {
                        // Check one final time after kill -9
                        sleep(1);
                        fp = popen("ps | grep go2rtc | grep -v grep | awk '{print $1}'", "r");
                        if (fp) {
                            still_running = false;
                            while (fgets(line, sizeof(line), fp)) {
                                pid_t pid = atoi(line);
                                if (pid > 0 && is_go2rtc_process(pid)) {
                                    still_running = true;
                                    log_error("go2rtc process %d still running after kill -9", pid);
                                }
                            }
                            pclose(fp);

                            if (still_running) {
                                log_error("Some go2rtc processes could not be killed");
                                success = false;
                            }
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

    // Also try to remove any /dev/shm/logs/go2rtc directory
    if (access("/dev/shm/logs/go2rtc", F_OK) == 0) {
        log_info("Removing /dev/shm/logs/go2rtc directory");
        if (system("rm -rf /dev/shm/logs/go2rtc") != 0) {
            log_warn("Failed to remove /dev/shm/logs/go2rtc directory");
            success = false;
        }
    }

    // Check for any TCP ports that might be in use by go2rtc
    // This helps identify if the process is still holding onto the port
    fp = popen("netstat -tlpn 2>/dev/null | grep go2rtc", "r");
    if (fp) {
        char netstat_line[256];
        bool found_ports = false;

        while (fgets(netstat_line, sizeof(netstat_line), fp)) {
            found_ports = true;
            log_warn("go2rtc still has open ports: %s", netstat_line);
        }

        pclose(fp);

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
        return is_go2rtc_running_as_service(1984); // Default API port
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

    // Use a more compatible command for BusyBox systems
    FILE *fp = popen("ps | grep go2rtc | grep -v grep | awk '{print $1}'", "r");
    if (!fp) {
        log_error("Failed to execute ps command");
        return false;
    }

    char line[16]; // Enough for a PID
    bool found = false;

    while (fgets(line, sizeof(line), fp)) {
        pid_t pid = atoi(line);
        if (pid > 0) {
            // Verify this is actually a go2rtc process
            if (is_go2rtc_process(pid)) {
                // If we find a go2rtc process but it's not our tracked one,
                // update our tracked PID
                if (g_process_pid <= 0 || g_process_pid != pid) {
                    log_warn("Found untracked go2rtc process with PID: %d", pid);
                    g_process_pid = pid;
                }
                found = true;
                break;
            }
        }
    }

    pclose(fp);

    // If we didn't find any go2rtc processes, also check if the port is in use
    if (!found) {
        // Check if the API port is in use by any process
        char cmd[128];
        snprintf(cmd, sizeof(cmd), "netstat -tlpn 2>/dev/null | grep ':%d' | grep -v grep", 1984); // Default API port
        fp = popen(cmd, "r");
        if (fp) {
            char netstat_line[256];
            if (fgets(netstat_line, sizeof(netstat_line), fp)) {
                log_warn("Port 1984 is in use but no go2rtc process found: %s", netstat_line);

                // Check if it responds like go2rtc
                if (is_go2rtc_running_as_service(1984)) {
                    log_info("Port 1984 is responding like go2rtc, assuming it's running as a service");
                    found = true;
                }
            }
            pclose(fp);
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
        // Check if the port is actually in use by go2rtc
        char cmd[128];
        snprintf(cmd, sizeof(cmd), "netstat -tlpn 2>/dev/null | grep ':%d' | grep go2rtc", api_port);
        FILE *fp = popen(cmd, "r");
        if (fp) {
            char netstat_line[256];
            bool port_in_use = false;

            if (fgets(netstat_line, sizeof(netstat_line), fp)) {
                port_in_use = true;
                log_info("go2rtc is already running and listening on port %d", api_port);
            }

            pclose(fp);

            if (port_in_use) {
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
                // Instead of killing and restarting, just return false to indicate we couldn't start
                // on the requested port
                return false;
            }
        } else {
            log_info("go2rtc is already running, using existing process");

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
    }

    // If we don't have a binary path (using existing service), but no service was detected,
    // we can't start go2rtc
    if (g_binary_path == NULL || g_binary_path[0] == '\0') {
        log_error("No go2rtc binary available and no running service detected");
        return false;
    }

    // Check if the port is already in use by another process
    char cmd[128];
    snprintf(cmd, sizeof(cmd), "netstat -tlpn 2>/dev/null | grep ':%d' | grep -v grep", api_port);
    FILE *fp = popen(cmd, "r");
    if (fp) {
        char netstat_line[256];
        bool port_in_use = false;

        if (fgets(netstat_line, sizeof(netstat_line), fp)) {
            port_in_use = true;
            log_warn("Port %d is already in use: %s", api_port, netstat_line);
        }

        pclose(fp);

        if (port_in_use) {
            log_error("Cannot start go2rtc because port %d is already in use", api_port);
            return false;
        }
    }

    // Generate configuration file
    if (!go2rtc_process_generate_config(g_config_path, api_port)) {
        log_error("Failed to generate go2rtc configuration");
        return false;
    }

    // Set the RTSP port to the default value (8554) for now
    // We'll try to get the actual value from the API after the process starts
    g_rtsp_port = 8554;

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

        // Execute go2rtc with explicit config path (using correct argument format)
        log_info("Executing go2rtc with command: %s --config %s", g_binary_path, g_config_path);
        execl(g_binary_path, g_binary_path, "--config", g_config_path, NULL);

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
            snprintf(url, sizeof(url), "http://localhost:%d/api", api_port);

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

    // Try to find a running go2rtc process
    FILE *fp = popen("ps | grep go2rtc | grep -v grep | awk '{print $1}'", "r");
    if (!fp) {
        return -1;
    }

    char line[16];
    pid_t found_pid = -1;

    while (fgets(line, sizeof(line), fp)) {
        pid_t pid = atoi(line);
        if (pid > 0 && is_go2rtc_process(pid)) {
            found_pid = pid;
            g_process_pid = pid;  // Update our tracked PID
            break;
        }
    }

    pclose(fp);
    return found_pid;
}
