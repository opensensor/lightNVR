#define _GNU_SOURCE
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <signal.h>
#include <unistd.h>
#include <stdbool.h>
#include <sys/types.h>
#include <sys/stat.h>
#include <fcntl.h>
#include <errno.h>
#include <sys/wait.h>
#include <sys/time.h>
#include <sys/utsname.h>

#include "core/version.h"
#include "core/config.h"
#include "core/logger.h"
#include "core/daemon.h"
#include "core/shutdown_coordinator.h"
#include "video/stream_manager.h"
#include "video/stream_state.h"
#include "video/stream_state_adapter.h"
#include "storage/storage_manager.h"
#include "video/streams.h"
#include "video/hls_streaming.h"
#include "video/mp4_recording.h"
#include "video/stream_transcoding.h"
#include "video/detection_stream.h"
#include "video/detection.h"
#include "video/detection_integration.h"
#include "video/detection_recording.h"
#include "video/stream_packet_processor.h"
#include "video/timestamp_manager.h"
#include "video/onvif_discovery.h"

// Include go2rtc headers if USE_GO2RTC is defined
#ifdef USE_GO2RTC
#include "video/go2rtc/go2rtc_stream.h"
#include "video/go2rtc/go2rtc_integration.h"
#endif

// External function declarations
void init_recordings_system(void);
#include "database/database_manager.h"
#include "database/db_schema_cache.h"
#include "web/http_server.h"
#include "web/mongoose_server.h"
#include "web/api_handlers.h"
#include "web/api_thread_pool.h"
#include "mongoose.h"

// Include necessary headers for signal handling
#include <signal.h>
#include <fcntl.h>

// Global flag for graceful shutdown
volatile bool running = true;

// Global flag for daemon mode (made extern so web_server.c can access it)
bool daemon_mode = false;

// Declare a global variable to store the web server socket
int web_server_socket = -1;

// global config
config_t config;

// Function to set the web server socket
void set_web_server_socket(int socket_fd) {
    web_server_socket = socket_fd;
}

// Improved signal handler with phased shutdown approach
static void signal_handler(int sig) {
    // Log the signal received
    log_info("Received signal %d, shutting down...", sig);
    
    // Check if we're already shutting down
    static bool shutdown_in_progress = false;
    if (shutdown_in_progress) {
        log_warn("Received another signal %d during shutdown, continuing with clean shutdown", sig);
        // Do not force immediate exit, continue with clean shutdown
        return;
    }
    
    // Mark that we're in the process of shutting down
    shutdown_in_progress = true;
    
    // Initiate shutdown sequence through the coordinator
    initiate_shutdown();
    
    // Set the running flag to false to trigger shutdown
    running = false;
    
    // For Linux 4.4 embedded systems, we need a more robust approach
    // Set an alarm to force exit if normal shutdown doesn't work
    // Increased from 10 to 20 seconds to give more time for graceful shutdown
    alarm(20);
    
    // Log that we've started the shutdown process
    log_info("Shutdown process initiated, waiting for components to stop gracefully");
}

// Alarm signal handler for forced exit - improved with phased emergency cleanup
static void alarm_handler(int sig) {
    static bool emergency_cleanup_in_progress = false;
    static int emergency_phase = 0;
    
    // If this is the first time we're called, start emergency cleanup
    if (!emergency_cleanup_in_progress) {
        log_warn("Shutdown timeout reached, forcing exit");
        log_info("Performing emergency cleanup of critical resources");
        emergency_cleanup_in_progress = true;
        
        // Set a new alarm for the next phase (30 seconds)
        alarm(30);
        
        // Phase 1: Try to stop all HLS writers first as they're often the source of hangs
        log_info("Emergency cleanup phase 1: Stopping HLS writers");
        
        // Force cleanup of HLS contexts first
        log_info("Starting detection resources cleanup...");
        cleanup_detection_resources();
        
        // Force cleanup of websocket connections
        if (web_server_socket >= 0) {
            log_info("Closing web server socket %d", web_server_socket);
            close(web_server_socket);
            web_server_socket = -1;
        }
        
        // Mark all components as stopping
        shutdown_coordinator_t *coordinator = get_shutdown_coordinator();
        if (coordinator) {
            pthread_mutex_lock(&coordinator->mutex);
            for (int i = 0; i < atomic_load(&coordinator->component_count); i++) {
                // Only update components that are still running
                if (atomic_load(&coordinator->components[i].state) == COMPONENT_RUNNING) {
                    atomic_store(&coordinator->components[i].state, COMPONENT_STOPPING);
                    log_info("Forcing component %s (ID: %d) to STOPPING state", 
                             coordinator->components[i].name, i);
                }
            }
            pthread_mutex_unlock(&coordinator->mutex);
        }
        
        // Increment phase for next alarm
        emergency_phase = 1;
        return;
    }
    
    // Phase 2: If we're still not exiting, force all components to stopped state
    if (emergency_phase == 1) {
        log_warn("Emergency cleanup phase 1 timed out, proceeding to phase 2");
        
        // Set a final alarm (15 seconds)
        alarm(15);
        
        // Force all components to be marked as stopped
        shutdown_coordinator_t *coordinator = get_shutdown_coordinator();
        if (coordinator) {
            pthread_mutex_lock(&coordinator->mutex);
            for (int i = 0; i < atomic_load(&coordinator->component_count); i++) {
                if (atomic_load(&coordinator->components[i].state) != COMPONENT_STOPPED) {
                    log_warn("Forcing component %s (ID: %d) from state %d to STOPPED", 
                             coordinator->components[i].name, i, 
                             atomic_load(&coordinator->components[i].state));
                    atomic_store(&coordinator->components[i].state, COMPONENT_STOPPED);
                }
            }
            coordinator->all_components_stopped = true;
            pthread_mutex_unlock(&coordinator->mutex);
        }
        
        // Increment phase for next alarm
        emergency_phase = 2;
        return;
    }
    
    // Phase 3: Final forced exit
    log_error("Emergency cleanup timed out after multiple phases, forcing immediate exit");
    _exit(EXIT_SUCCESS); // Use _exit instead of exit to avoid calling atexit handlers
}

// Function to initialize signal handlers with improved signal handling
static void init_signals() {
    // Check if we're running on Linux 4.4 or similar embedded system
    struct utsname uts_info;
    bool is_linux_4_4 = false;
    
    if (uname(&uts_info) == 0) {
        // Check if kernel version starts with 4.4
        if (strncmp(uts_info.release, "4.4", 3) == 0) {
            log_info("Detected Linux 4.4 kernel, using compatible signal handling");
            is_linux_4_4 = true;
        }
    }
    
    // Only set up signal handlers if we're not in daemon mode
    // In daemon mode, the signal handlers are set up in daemon.c
    if (!daemon_mode || !is_linux_4_4) {
        // Set up signal handlers for both daemon and non-daemon mode
        // This ensures consistent behavior across all modes
        struct sigaction sa;
        memset(&sa, 0, sizeof(sa));
        sa.sa_handler = signal_handler;
        
        // Add SA_RESTART flag to automatically restart interrupted system calls
        // This helps prevent issues with blocking I/O operations during signal handling
        sa.sa_flags = SA_RESTART;
        
        sigaction(SIGINT, &sa, NULL);
        sigaction(SIGTERM, &sa, NULL);
        sigaction(SIGHUP, &sa, NULL);
        
        // Set up alarm handler for phased forced exit
        struct sigaction sa_alarm;
        memset(&sa_alarm, 0, sizeof(sa_alarm));
        sa_alarm.sa_handler = alarm_handler;
        sa_alarm.sa_flags = SA_RESTART;
        sigaction(SIGALRM, &sa_alarm, NULL);
    } else {
        log_info("Running in daemon mode on Linux 4.4, signal handlers will be set up by daemon.c");
    }
    
    // Set up SIGPIPE handler to ignore broken pipe errors
    // This is important for socket operations to prevent crashes
    struct sigaction sa_pipe;
    memset(&sa_pipe, 0, sizeof(sa_pipe));
    sa_pipe.sa_handler = SIG_IGN;  // Ignore SIGPIPE
    sa_pipe.sa_flags = SA_RESTART;
    sigaction(SIGPIPE, &sa_pipe, NULL);
    
    // Block SIGPIPE to prevent crashes when writing to closed sockets
    // This is especially important for older Linux kernels
    sigset_t set;
    sigemptyset(&set);
    sigaddset(&set, SIGPIPE);
    pthread_sigmask(SIG_BLOCK, &set, NULL);
    
    log_info("Signal handlers initialized with improved handling");
}
/**
 * Check if another instance is running and kill it if needed
 */
static int check_and_kill_existing_instance(const char *pid_file) {
    FILE *fp = fopen(pid_file, "r");
    if (!fp) {
        // No PID file, assume no other instance is running
        return 0;
    }

    // Read PID from file
    pid_t existing_pid;
    if (fscanf(fp, "%d", &existing_pid) != 1) {
        fclose(fp);
        log_warn("Invalid PID file format");
        unlink(pid_file);  // Remove invalid PID file
        return 0;
    }
    fclose(fp);

    // Check if process exists
    if (kill(existing_pid, 0) == 0) {
        // Process exists, ask user if they want to kill it
        log_warn("Another instance with PID %d appears to be running", existing_pid);

        // In a non-interactive environment, we can automatically kill it
        log_info("Attempting to terminate previous instance (PID: %d) with SIGTERM", existing_pid);

        // Send SIGTERM to let it clean up
        if (kill(existing_pid, SIGTERM) == 0) {
            // Wait longer for it to terminate properly (increased from 5 to 15 seconds)
            int timeout = 15;  // 15 seconds
            while (timeout-- > 0 && kill(existing_pid, 0) == 0) {
                sleep(1);
            }

            // If still running, force kill
            if (timeout <= 0 && kill(existing_pid, 0) == 0) {
                log_warn("Process didn't terminate gracefully within timeout, using SIGKILL");
                kill(existing_pid, SIGKILL);
                sleep(1);  // Give it a moment
            }

            // Wait for PID file to be released
            timeout = 5;  // 5 seconds
            while (timeout-- > 0) {
                // Check if PID file still exists and is locked
                int test_fd = open(pid_file, O_RDWR);
                if (test_fd < 0) {
                    if (errno == ENOENT) {
                        // PID file doesn't exist anymore, we're good
                        log_info("Previous instance terminated and PID file released");
                        return 0;
                    }
                    // Some other error, continue waiting
                } else {
                    // Try to lock the file
                    if (lockf(test_fd, F_TLOCK, 0) == 0) {
                        // We got the lock, which means the previous process released it
                        close(test_fd);
                        log_info("Previous instance terminated and PID file lock released");
                        unlink(pid_file);  // Remove the old PID file
                        return 0;
                    }
                    close(test_fd);
                }
                sleep(1);
            }

            // If we get here, the PID file still exists and is locked, or some other issue
            log_warn("Previous instance may have terminated but PID file is still locked or inaccessible");
            // Try to remove it anyway
            if (unlink(pid_file) == 0) {
                log_info("Removed potentially stale PID file");
                return 0;
            } else {
                log_error("Failed to remove PID file: %s", strerror(errno));
                return -1;
            }
        } else {
            log_error("Failed to terminate previous instance: %s", strerror(errno));
            return -1;
        }
    } else {
        // Process doesn't exist, remove stale PID file
        log_warn("Removing stale PID file");
        unlink(pid_file);
    }

    return 0;
}

// Function to create PID file
static int create_pid_file(const char *pid_file) {
    char pid_str[16];
    int fd;
    
    // Make sure the directory exists
    char dir_path[MAX_PATH_LENGTH] = {0};
    char *last_slash = strrchr(pid_file, '/');
    if (last_slash) {
        size_t dir_len = last_slash - pid_file;
        strncpy(dir_path, pid_file, dir_len);
        dir_path[dir_len] = '\0';
        
        // Create directory if it doesn't exist
        struct stat st;
        if (stat(dir_path, &st) != 0) {
            if (mkdir(dir_path, 0755) != 0 && errno != EEXIST) {
                log_error("Could not create directory for PID file: %s", strerror(errno));
                return -1;
            }
        }
    }
    
    // Try to open the PID file with exclusive creation first
    fd = open(pid_file, O_RDWR | O_CREAT | O_EXCL, 0644);
    if (fd < 0 && errno == EEXIST) {
        // File exists, try to open it normally
        fd = open(pid_file, O_RDWR | O_CREAT, 0644);
    }
    
    if (fd < 0) {
        log_error("Could not open PID file %s: %s", pid_file, strerror(errno));
        return -1;
    }
    
    // Lock the PID file to prevent multiple instances
    if (lockf(fd, F_TLOCK, 0) < 0) {
        log_error("Could not lock PID file %s: %s", pid_file, strerror(errno));
        close(fd);
        return -1;
    }
    
    // Truncate the file to ensure we overwrite any existing content
    if (ftruncate(fd, 0) < 0) {
        log_warn("Could not truncate PID file: %s", strerror(errno));
        // Continue anyway
    }
    
    // Write PID to file
    sprintf(pid_str, "%d\n", getpid());
    if (write(fd, pid_str, strlen(pid_str)) != strlen(pid_str)) {
        log_error("Could not write to PID file %s: %s", pid_file, strerror(errno));
        close(fd);
        unlink(pid_file);  // Try to remove the file
        return -1;
    }
    
    // Sync to ensure the PID is written to disk
    fsync(fd);
    
    // Keep file open to maintain lock
    return fd;
}

// Function to remove PID file
static void remove_pid_file(int fd, const char *pid_file) {
    if (fd >= 0) {
        // Release the lock by closing the file
        close(fd);
    }
    
    // Try to remove the file
    if (unlink(pid_file) != 0) {
        log_warn("Failed to remove PID file %s: %s", pid_file, strerror(errno));
    } else {
        log_info("Successfully removed PID file %s", pid_file);
    }
}

// Function to daemonize the process
static int daemonize(const char *pid_file) {
    int result = init_daemon(pid_file);

    // If daemon initialization failed, return error
    if (result != 0) {
        return result;
    }

    // We're now in the child process, set daemon_mode flag
    daemon_mode = true;

    // Make sure the running flag is set to true
    running = true;

    // Return success
    return 0;
}

// Function to check and ensure recording is active for streams that have recording enabled
static void check_and_ensure_recording(void);

// Add this to src/core/main.c after initializing the stream manager
static void load_streams_from_config(const config_t *config) {
    for (int i = 0; i < config->max_streams; i++) {
        if (config->streams[i].name[0] != '\0') {
            log_info("Loading stream from config: %s", config->streams[i].name);
            stream_handle_t stream = add_stream(&config->streams[i]);

            if (stream) {
                log_info("Stream loaded: %s", config->streams[i].name);

                if (config->streams[i].enabled) {
                    if (start_stream(stream) == 0) {
                        log_info("Stream started: %s", config->streams[i].name);
                        
                        // Start recording if record flag is set
                        if (config->streams[i].record) {
                            // Start HLS streaming for the stream
                            #ifdef USE_GO2RTC
                            if (go2rtc_integration_start_hls(config->streams[i].name) == 0) {
                                log_info("HLS streaming started for stream: %s (using go2rtc if available)", config->streams[i].name);
                            } else {
                                log_warn("Failed to start HLS streaming for stream: %s", config->streams[i].name);
                            }
                            
                            // Also start MP4 recording for the stream, regardless of HLS streaming status
                            if (go2rtc_integration_start_recording(config->streams[i].name) == 0) {
                                log_info("MP4 recording started for stream: %s (using go2rtc if available)", config->streams[i].name);
                            } else {
                                log_warn("Failed to start MP4 recording for stream: %s", config->streams[i].name);
                            }
                            #else
                            // Fall back to default implementation if go2rtc is not enabled
                            if (start_hls_stream(config->streams[i].name) == 0) {
                                log_info("HLS streaming started for stream: %s", config->streams[i].name);
                            } else {
                                log_warn("Failed to start HLS streaming for stream: %s", config->streams[i].name);
                            }
                            
                            // Also start MP4 recording for the stream, regardless of HLS streaming status
                            if (start_mp4_recording(config->streams[i].name) == 0) {
                                log_info("MP4 recording started for stream: %s", config->streams[i].name);
                            } else {
                                log_warn("Failed to start MP4 recording for stream: %s", config->streams[i].name);
                            }
                            #endif
                        }
                    } else {
                        log_warn("Failed to start stream: %s", config->streams[i].name);
                    }
                }
            } else {
                log_error("Failed to add stream from config: %s", config->streams[i].name);
            }
        }
    }
}

int main(int argc, char *argv[]) {
    int pid_fd = -1;

    // Print banner
    printf("LightNVR v%s - Lightweight NVR\n", LIGHTNVR_VERSION_STRING);
    printf("Build date: %s\n", LIGHTNVR_BUILD_DATE);

    // Initialize logging
    if (init_logger() != 0) {
        fprintf(stderr, "Failed to initialize logger\n");
        return EXIT_FAILURE;
    }

    // Define a variable to store the custom config path
    char custom_config_path[MAX_PATH_LENGTH] = {0};

    // Parse command line arguments
    for (int i = 1; i < argc; i++) {
        if (strcmp(argv[i], "-d") == 0 || strcmp(argv[i], "--daemon") == 0) {
            daemon_mode = true;
        } else if (strcmp(argv[i], "-c") == 0 || strcmp(argv[i], "--config") == 0) {
            if (i + 1 < argc) {
                // Set config file path
                strncpy(custom_config_path, argv[i+1], MAX_PATH_LENGTH - 1);
                custom_config_path[MAX_PATH_LENGTH - 1] = '\0';
                i++;
            } else {
                log_error("Missing config file path");
                return EXIT_FAILURE;
            }
        } else if (strcmp(argv[i], "-h") == 0 || strcmp(argv[i], "--help") == 0) {
            printf("Usage: %s [options]\n", argv[0]);
            printf("Options:\n");
            printf("  -d, --daemon        Run as daemon\n");
            printf("  -c, --config FILE   Use config file\n");
            printf("  -h, --help          Show this help\n");
            printf("  -v, --version       Show version\n");
            return EXIT_SUCCESS;
        } else if (strcmp(argv[i], "-v") == 0 || strcmp(argv[i], "--version") == 0) {
            // Version already printed in banner
            return EXIT_SUCCESS;
        }
    }

    // Set custom config path if specified
    if (custom_config_path[0] != '\0') {
        set_custom_config_path(custom_config_path);
        log_info("Using custom config path: %s", custom_config_path);
    }

    // Load configuration
    if (load_config(&config) != 0) {
        log_error("Failed to load configuration");
        return EXIT_FAILURE;
    }
    
    // Copy to global config
    memcpy(&g_config, &config, sizeof(config_t));

    // Set log file from configuration
    if (config.log_file[0] != '\0') {
        if (set_log_file(config.log_file) != 0) {
            log_warn("Failed to set log file: %s", config.log_file);
        } else {
            log_info("Logging to file: %s", config.log_file);
        }
    }
    
    // Set log level from configuration
    fprintf(stderr, "Setting log level from config: %d\n", config.log_level);
    set_log_level(config.log_level);
    
    // Use log_error instead of log_info to ensure this message is always logged
    // regardless of the configured log level
    log_error("Log level set to %d (%s)", config.log_level, get_log_level_string(config.log_level));

    // Copy configuration to global config
    memcpy(&g_config, &config, sizeof(config_t));

    // Verify web root directory exists and is readable
    struct stat st;
    if (stat(config.web_root, &st) != 0 || !S_ISDIR(st.st_mode)) {
        log_error("Web root directory %s does not exist or is not a directory", config.web_root);

        // Check if this is a path in /var or another system directory
        if (strncmp(config.web_root, "/var/", 5) == 0 || 
            strncmp(config.web_root, "/tmp/", 5) == 0 ||
            strncmp(config.web_root, "/run/", 5) == 0) {
            
            // Create a symlink from the system directory to our storage path
            char storage_web_path[MAX_PATH_LENGTH];
            snprintf(storage_web_path, sizeof(storage_web_path), "%s/web", config.storage_path);
            
            log_warn("Web root is in system directory (%s), redirecting to storage path (%s)",
                    config.web_root, storage_web_path);
            
            // Create the directory in our storage path
            if (mkdir(storage_web_path, 0755) != 0 && errno != EEXIST) {
                log_error("Failed to create web root in storage path: %s", strerror(errno));
                return EXIT_FAILURE;
            }
            
            // Create parent directory for symlink if needed
            char parent_dir[MAX_PATH_LENGTH];
            strncpy(parent_dir, config.web_root, sizeof(parent_dir) - 1);
            char *last_slash = strrchr(parent_dir, '/');
            if (last_slash) {
                *last_slash = '\0';
                if (mkdir(parent_dir, 0755) != 0 && errno != EEXIST) {
                    log_warn("Failed to create parent directory for web root symlink: %s", strerror(errno));
                }
            }
            
            // Create the symlink
            if (symlink(storage_web_path, config.web_root) != 0) {
                log_error("Failed to create symlink from %s to %s: %s", 
                        config.web_root, storage_web_path, strerror(errno));
                
                // Fall back to using the storage path directly
                strncpy(config.web_root, storage_web_path, MAX_PATH_LENGTH - 1);
                log_warn("Using storage path directly for web root: %s", config.web_root);
            } else {
                log_info("Created symlink from %s to %s", config.web_root, storage_web_path);
            }
        } else {
            // Try to create it directly
            if (mkdir(config.web_root, 0755) != 0) {
                log_error("Failed to create web root directory: %s", strerror(errno));
                return EXIT_FAILURE;
            }
            
            log_info("Created web root directory: %s", config.web_root);
        }
    }

    // Initialize shutdown coordinator
    if (init_shutdown_coordinator() != 0) {
        log_error("Failed to initialize shutdown coordinator");
        return EXIT_FAILURE;
    }
    log_info("Shutdown coordinator initialized");

    // Initialize signal handlers
    init_signals();

    // Check for existing instances and handle PID file
    if (check_and_kill_existing_instance(config.pid_file) != 0) {
        log_error("Failed to handle existing instance");
        return EXIT_FAILURE;
    }

    // Daemonize if requested
    if (daemon_mode) {
        log_info("Starting in daemon mode");
        if (daemonize(config.pid_file) != 0) {
            log_error("Failed to daemonize");
            return EXIT_FAILURE;
        }
        // In daemon mode, the PID file is handled by daemon.c
    } else {
        // Create PID file (only for non-daemon mode)
        pid_fd = create_pid_file(config.pid_file);
        if (pid_fd < 0) {
            log_error("Failed to create PID file");
            return EXIT_FAILURE;
        }
    }

    log_info("LightNVR v%s starting up", LIGHTNVR_VERSION_STRING);

    // Initialize database
    if (init_database(config.db_path) != 0) {
        log_error("Failed to initialize database");
        goto cleanup;
    }
    
    // Initialize schema cache
    log_info("Initializing schema cache...");
    init_schema_cache();
    log_info("Schema cache initialized");

    // Initialize storage manager
    if (init_storage_manager(config.storage_path, config.max_storage_size) != 0) {
        log_error("Failed to initialize storage manager");
        goto cleanup;
    }

    // Initialize stream state manager
    if (init_stream_state_manager(config.max_streams) != 0) {
        log_error("Failed to initialize stream state manager");
        goto cleanup;
    }
    
    // Initialize stream state adapter
    if (init_stream_state_adapter() != 0) {
        log_error("Failed to initialize stream state adapter");
        goto cleanup;
    }
    
    // Initialize stream manager
    if (init_stream_manager(config.max_streams) != 0) {
        log_error("Failed to initialize stream manager");
        goto cleanup;
    }

    // Initialize go2rtc integration if enabled - MOVED TO BEGINNING OF SETUP
    #ifdef USE_GO2RTC
    log_info("Initializing go2rtc integration...");
    
    // Use configuration values if provided, otherwise use defaults
    const char *binary_path = NULL;  // Will use go2rtc from PATH if not specified
    const char *config_dir = "/tmp/go2rtc";    // Default config directory
    int api_port = 1984;                               // Default API port
    
    // Check if custom values are provided in the configuration
    if (config.go2rtc_binary_path[0] != '\0') {
        binary_path = config.go2rtc_binary_path;
        log_info("Using custom go2rtc binary path: %s", binary_path);
    } else {
        log_info("go2rtc binary path not specified, will use from PATH or existing service");
    }
    
    if (config.go2rtc_config_dir[0] != '\0') {
        config_dir = config.go2rtc_config_dir;
        log_info("Using custom go2rtc config directory: %s", config_dir);
    } else {
        log_info("Using default go2rtc config directory: %s", config_dir);
    }
    
    if (config.go2rtc_api_port > 0) {
        api_port = config.go2rtc_api_port;
        log_info("Using custom go2rtc API port: %d", api_port);
    } else {
        log_info("Using default go2rtc API port: %d", api_port);
    }
    
    if (go2rtc_stream_init(binary_path, config_dir, api_port)) {
        log_info("go2rtc integration initialized successfully");

        // Start go2rtc service (or use existing service if already running)
        if (go2rtc_stream_start_service()) {
            log_info("go2rtc service started successfully or existing service detected");

            // Wait for go2rtc service to be fully ready
            log_info("Waiting for go2rtc service to be fully ready...");
            int retries = 10;
            while (retries > 0 && !go2rtc_stream_is_ready()) {
                log_info("Waiting for go2rtc service to be ready... (%d retries left)", retries);
                sleep(1);
                retries--;
            }

            if (!go2rtc_stream_is_ready()) {
                log_error("go2rtc service failed to be ready in time");
            } else {
                log_info("go2rtc service is now fully ready");
            }

            // Initialize go2rtc consumer integration
            if (go2rtc_integration_init()) {
                log_info("go2rtc consumer integration initialized successfully");

                // Register all existing streams with go2rtc
                log_info("Registering all existing streams with go2rtc");
                if (!go2rtc_integration_register_all_streams()) {
                    log_warn("Failed to register all streams with go2rtc");
                    // Continue anyway
                } else {
                    // Wait a bit for streams to be fully registered
                    log_info("Waiting for streams to be fully registered with go2rtc...");
                    sleep(3);
                    log_info("Streams should now be fully registered with go2rtc");
                }
            } else {
                log_error("Failed to initialize go2rtc consumer integration");
            }
        } else {
            log_error("Failed to start go2rtc service");
        }
    } else {
        log_error("Failed to initialize go2rtc integration");
    }
    #endif

    // Initialize FFmpeg streaming backend
    init_transcoding_backend();
    
    // Initialize timestamp trackers
    init_timestamp_trackers();
    log_info("Timestamp trackers initialized");
    
    init_hls_streaming_backend();
    init_mp4_recording_backend();
    
    // Initialize detection system
    if (init_detection_system() != 0) {
        log_error("Failed to initialize detection system");
    } else {
        log_info("Detection system initialized successfully");
    }
    
    
    // Initialize detection recording system
    init_detection_recording_system();
    
    // Initialize detection stream system
    init_detection_stream_system();
    
    // Initialize ONVIF discovery module
    if (init_onvif_discovery() != 0) {
        log_error("Failed to initialize ONVIF discovery module");
    } else {
        log_info("ONVIF discovery module initialized successfully");
        
        // Start ONVIF discovery if enabled in configuration
        if (config.onvif_discovery_enabled) {
            log_info("Starting ONVIF discovery on network %s with interval %d seconds", 
                    config.onvif_discovery_network, config.onvif_discovery_interval);
            
            if (start_onvif_discovery(config.onvif_discovery_network, config.onvif_discovery_interval) != 0) {
                log_error("Failed to start ONVIF discovery");
            } else {
                log_info("ONVIF discovery started successfully");
            }
        }
    }
    
    // Now that go2rtc is initialized (if enabled), load streams from config
    log_info("Loading streams from configuration...");
    load_streams_from_config(&config);

    // Initialize authentication system
    if (init_auth_system() != 0) {
        log_error("Failed to initialize authentication system");
        // Continue anyway, will fall back to config-based authentication
    } else {
        log_info("Authentication system initialized successfully");
    }
    
    // Check if detection models exist and start detection-based recording - MOVED TO END OF SETUP
    for (int i = 0; i < config.max_streams; i++) {
        if (config.streams[i].name[0] != '\0' && config.streams[i].enabled && 
            config.streams[i].detection_based_recording && config.streams[i].detection_model[0] != '\0') {
            
            // Check if model file exists
            char model_path[MAX_PATH_LENGTH];
            if (config.streams[i].detection_model[0] != '/') {
                // Relative path, use configured models path from INI if it exists
                if (config.models_path && strlen(config.models_path) > 0) {
                    snprintf(model_path, sizeof(model_path), "%s/%s", config.models_path, config.streams[i].detection_model);
                } else {
                    // Fall back to default path if INI config doesn't exist
                    snprintf(model_path, MAX_PATH_LENGTH, "/etc/lightnvr/models/%s", config.streams[i].detection_model);
                }
            } else {
                // Absolute path
                strncpy(model_path, config.streams[i].detection_model, MAX_PATH_LENGTH - 1);
            }
            
            // Check if file exists
            FILE *model_file = fopen(model_path, "r");
            if (model_file) {
                fclose(model_file);
                log_info("Detection model found: %s", model_path);
            } else {
                log_error("Detection model not found: %s", model_path);
                log_error("Detection will not work properly!");
                
                // Create the models directory if it doesn't exist
                if (mkdir(config.models_path, 0755) != 0 && errno != EEXIST) {
                    log_error("Failed to create models directory: %s", strerror(errno));
                } else {
                    log_info("Created models directory: %s", config.models_path);
                }
            }
            
            log_info("Starting detection-based recording for stream %s with model %s", 
                    config.streams[i].name, config.streams[i].detection_model);
            
            // Start detection recording
            float threshold = config.streams[i].detection_threshold;
            int pre_buffer = config.streams[i].pre_detection_buffer;
            int post_buffer = config.streams[i].post_detection_buffer;
            
            if (start_detection_recording(config.streams[i].name, config.streams[i].detection_model, 
                                         threshold, pre_buffer, post_buffer) == 0) {
                log_info("Detection-based recording started for stream %s", config.streams[i].name);
            } else {
                log_warn("Failed to start detection-based recording for stream %s", config.streams[i].name);
            }
            
            // Start detection stream reader with more detailed logging
            log_info("Starting detection stream reader for stream %s with model %s", 
                    config.streams[i].name, config.streams[i].detection_model);
            
            int detection_interval = config.streams[i].detection_interval > 0 ? 
                                    config.streams[i].detection_interval : 10;
            
            int result = start_detection_stream_reader(config.streams[i].name, detection_interval);
            if (result == 0) {
                log_info("Successfully started detection stream reader for stream %s", 
                        config.streams[i].name);
                
                // Verify the reader is running
                if (is_detection_stream_reader_running(config.streams[i].name)) {
                    log_info("Confirmed detection stream reader is running for %s", 
                            config.streams[i].name);
                } else {
                    log_warn("Detection stream reader reported as not running for %s despite successful start", 
                            config.streams[i].name);
                }
            } else {
                log_error("Failed to start detection stream reader for stream %s: error code %d", 
                        config.streams[i].name, result);
            }
        }
    }
    
    // Initialize Mongoose web server with direct handlers
    http_server_config_t server_config = {
        .port = config.web_port,
        .web_root = config.web_root,
        .auth_enabled = config.web_auth_enabled,
        .cors_enabled = true,
        .ssl_enabled = false,
        .max_connections = 100,
        .connection_timeout = 30,
        .daemon_mode = daemon_mode,
        .web_thread_pool_size = api_thread_pool_get_size()  // Use the value from api_thread_pool_get_size()
    };
    
    // Set CORS allowed origins, methods, and headers
    strncpy(server_config.allowed_origins, "*", sizeof(server_config.allowed_origins) - 1);
    strncpy(server_config.allowed_methods, "GET, POST, PUT, DELETE, OPTIONS", sizeof(server_config.allowed_methods) - 1);
    strncpy(server_config.allowed_headers, "Content-Type, Authorization", sizeof(server_config.allowed_headers) - 1);
    
    if (config.web_auth_enabled) {
        strncpy(server_config.username, config.web_username, sizeof(server_config.username) - 1);
        strncpy(server_config.password, config.web_password, sizeof(server_config.password) - 1);
    }
    
    // Use the direct mongoose server implementation
    http_server_handle_t http_server = mongoose_server_init(&server_config);
    if (!http_server) {
        log_error("Failed to initialize Mongoose web server");
        goto cleanup;
    }
    
    if (http_server_start(http_server) != 0) {
        log_error("Failed to start Mongoose web server");
        http_server_destroy(http_server);
        goto cleanup;
    }
    
    log_info("Mongoose web server started on port %d", config.web_port);
    
    // No need to register API handlers with the Mongoose server
    // Direct handlers are registered in register_api_handlers

    log_info("LightNVR initialized successfully");

    // Print initial detection stream status
    print_detection_stream_status();

    // Main loop
    while (running) {
        // Log that the daemon is still running (maybe once per minute)
        static time_t last_log_time = 0;
        static time_t last_status_time = 0;
        static time_t last_recording_check_time = 0;
        time_t now = time(NULL);
        
        if (now - last_log_time > 60) {
            log_debug("Daemon is still running...");
            last_log_time = now;
        }
        
        // Print detection stream status every 5 minutes to help diagnose issues
        if (now - last_status_time > 300) {
            print_detection_stream_status();
            last_status_time = now;
        }
        
        // Check and ensure recording is active every minute
        if (now - last_recording_check_time > 60) {
            check_and_ensure_recording();
            
            // Call monitor_all_hls_segments_for_detection to ensure detection threads are started
            // This is important to make sure detection threads are running
            monitor_all_hls_segments_for_detection();
            
            last_recording_check_time = now;
        }

        // Process events, monitor system health, etc.
        sleep(1);
    }

    log_info("Shutting down LightNVR...");

    // Cleanup
cleanup:
    log_info("Starting cleanup process...");
    
    // We'll clean up go2rtc later in the shutdown sequence
    // First clean up go2rtc integration (but not the process yet)
    #ifdef USE_GO2RTC
    log_info("Cleaning up go2rtc integration...");
    go2rtc_integration_cleanup();
    // Note: go2rtc_stream_cleanup() will be called later
    #endif
    
    // Block signals during cleanup to prevent interruptions
    sigset_t block_mask, old_mask;
    sigfillset(&block_mask);
    pthread_sigmask(SIG_BLOCK, &block_mask, &old_mask);
    
    // Set up a watchdog timer to force exit if cleanup takes too long
    pid_t cleanup_pid = fork();
    
    if (cleanup_pid == 0) {
        // Child process - watchdog timer
        sleep(30);  // 30 seconds for first phase timeout
        log_error("Cleanup process phase 1 timed out after 30 seconds");
        kill(getppid(), SIGUSR1);  // Send USR1 to parent to trigger emergency cleanup
        
        // Wait another 30 seconds for emergency cleanup
        sleep(30);
        log_error("Cleanup process phase 2 timed out after 30 seconds, forcing exit");
        kill(getppid(), SIGKILL);  // Force kill the parent process
        exit(EXIT_FAILURE);
    } else if (cleanup_pid > 0) {
        // Parent process - continue with cleanup
        
        // Set up a handler for USR1 to perform emergency cleanup
        struct sigaction sa_usr1;
        memset(&sa_usr1, 0, sizeof(sa_usr1));
        sa_usr1.sa_handler = alarm_handler;  // Reuse the alarm handler for USR1
        sigaction(SIGUSR1, &sa_usr1, NULL);
        
        // Components should already be registered during initialization
        // No need to register them again during shutdown
        log_info("Starting shutdown sequence for all components...");
        
        // First, clear all packet callbacks to prevent further processing
        log_info("Clearing all packet callbacks...");
        for (int i = 0; i < MAX_STREAMS; i++) {
            stream_reader_ctx_t *reader = get_stream_reader_by_index(i);
            if (reader) {
                // Safely clear the callback
                set_packet_callback(reader, NULL, NULL);
                log_info("Cleared packet callback for stream reader %d", i);
            }
        }
        
        // Wait a moment for callbacks to clear and any in-progress operations to complete
        log_info("Waiting for callbacks to clear...");
        usleep(1000000);  // 1000ms (increased from 500ms)
        
        // Stop all detection stream readers first
        log_info("Stopping all detection stream readers...");
        for (int i = 0; i < config.max_streams; i++) {
            if (config.streams[i].name[0] != '\0' && 
                config.streams[i].detection_based_recording && 
                config.streams[i].detection_model[0] != '\0') {
                
                log_info("Stopping detection stream reader for: %s", config.streams[i].name);
                stop_detection_stream_reader(config.streams[i].name);
                
                // Update component state
                char component_name[128];
                snprintf(component_name, sizeof(component_name), "detection_thread_%s", config.streams[i].name);
                int component_id = -1;
                for (int j = 0; j < atomic_load(&get_shutdown_coordinator()->component_count); j++) {
                    if (strcmp(get_shutdown_coordinator()->components[j].name, component_name) == 0) {
                        component_id = j;
                        break;
                    }
                }
                if (component_id >= 0) {
                    update_component_state(component_id, COMPONENT_STOPPED);
                }
            }
        }
        
        // Wait for detection stream readers to stop
        usleep(500000);  // 500ms
        
        // Stop all streams to ensure clean shutdown
        for (int i = 0; i < config.max_streams; i++) {
            if (config.streams[i].name[0] != '\0') {
                stream_handle_t stream = get_stream_by_name(config.streams[i].name);
                if (stream) {
                    log_info("Stopping stream: %s", config.streams[i].name);
                    stop_stream(stream);
                }
            }
        }
        
        // Wait longer for streams to stop
        usleep(1500000);  // 1500ms
        
        // Finalize all MP4 recordings first before cleaning up the backend
        log_info("Finalizing all MP4 recordings...");
        close_all_mp4_writers();
        
        // Update MP4 writer components state
        for (int i = 0; i < config.max_streams; i++) {
            if (config.streams[i].name[0] != '\0' && config.streams[i].record) {
                char component_name[128];
                snprintf(component_name, sizeof(component_name), "mp4_writer_%s", config.streams[i].name);
                int component_id = -1;
                for (int j = 0; j < atomic_load(&get_shutdown_coordinator()->component_count); j++) {
                    if (strcmp(get_shutdown_coordinator()->components[j].name, component_name) == 0) {
                        component_id = j;
                        break;
                    }
                }
                if (component_id >= 0) {
                    update_component_state(component_id, COMPONENT_STOPPED);
                }
            }
        }
        
        // Clean up HLS directories
        log_info("Cleaning up HLS directories...");
        cleanup_hls_directories();
        
        // Update HLS writer components state
        for (int i = 0; i < config.max_streams; i++) {
            if (config.streams[i].name[0] != '\0') {
                char component_name[128];
                snprintf(component_name, sizeof(component_name), "hls_writer_%s", config.streams[i].name);
                int component_id = -1;
                for (int j = 0; j < atomic_load(&get_shutdown_coordinator()->component_count); j++) {
                    if (strcmp(get_shutdown_coordinator()->components[j].name, component_name) == 0) {
                        component_id = j;
                        break;
                    }
                }
                if (component_id >= 0) {
                    update_component_state(component_id, COMPONENT_STOPPED);
                }
            }
        }
        
        // Now clean up the backends in the correct order
        // First stop all detection streams
        log_info("Cleaning up detection stream system...");
        shutdown_detection_stream_system();
        
        // Wait for detection streams to stop
        usleep(1000000);  // 1000ms
        
        // Clean up HLS streaming before MP4 recording
        // This is important because HLS streaming is used by MP4 recording
        log_info("Cleaning up HLS streaming backend...");
        cleanup_hls_streaming_backend();
        
        // Wait for HLS streaming to clean up
        usleep(1000000);  // 1000ms
        
        // Now clean up MP4 recording
        log_info("Cleaning up MP4 recording backend...");
        cleanup_mp4_recording_backend();
        
        // Wait for MP4 recording to clean up
        usleep(1000000);  // 1000ms
        
        // Clean up stream reader backend last to ensure all consumers are stopped
        log_info("Cleaning up stream reader backend...");
        cleanup_stream_reader_backend();
        
        // Clean up FFmpeg resources
        log_info("Cleaning up transcoding backend...");
        cleanup_transcoding_backend();
        
        // Shutdown detection resources with timeout protection
        log_info("Cleaning up detection resources...");
        
        // Set an alarm to prevent hanging during detection resource cleanup
        alarm(5); // 5 second timeout for detection resource cleanup
        cleanup_detection_resources();
        alarm(0); // Cancel the alarm if cleanup completed successfully
        
        
        // Shutdown ONVIF discovery
        log_info("Shutting down ONVIF discovery module...");
        shutdown_onvif_discovery();
        
        // Now shut down other components
        log_info("Shutting down web server...");
        http_server_stop(http_server);
        http_server_destroy(http_server);
        
        // Update server thread pool component state
        for (int j = 0; j < atomic_load(&get_shutdown_coordinator()->component_count); j++) {
            if (strcmp(get_shutdown_coordinator()->components[j].name, "server_thread_pool") == 0) {
                update_component_state(j, COMPONENT_STOPPED);
                break;
            }
        }
        
        log_info("Shutting down stream manager...");
        shutdown_stream_manager();
        
        log_info("Shutting down stream state adapter...");
        shutdown_stream_state_adapter();
        
        log_info("Shutting down stream state manager...");
        shutdown_stream_state_manager();
        
        log_info("Shutting down storage manager...");
        shutdown_storage_manager();
        
        // Add a memory barrier before database shutdown to ensure all previous operations are complete
        __sync_synchronize();
        
        log_info("Shutting down database...");
        shutdown_database();
        
        // Free schema cache
        log_info("Freeing schema cache...");
        free_schema_cache();
        
        // Wait for all components to stop
        log_info("Waiting for all components to stop...");
        if (!wait_for_all_components_stopped(5)) {
            log_warn("Not all components stopped within timeout, continuing anyway");
        }
        
        // Clean up the shutdown coordinator
        log_info("Cleaning up shutdown coordinator...");
        shutdown_coordinator_cleanup();
        
        // Add a small delay after database shutdown to ensure all resources are properly released
        usleep(100000);  // 100ms
        
        // Now clean up go2rtc as one of the last steps
        #ifdef USE_GO2RTC
        log_info("Cleaning up go2rtc stream...");
        go2rtc_stream_cleanup();
        #endif
        
        // Kill the watchdog timer since we completed successfully
        kill(cleanup_pid, SIGKILL);
        waitpid(cleanup_pid, NULL, 0);
        
        // Restore signal mask
        pthread_sigmask(SIG_SETMASK, &old_mask, NULL);
    } else {
        // Fork failed
        log_error("Failed to create watchdog process for cleanup timeout");
        // Continue with cleanup anyway in a simplified manner
        
        // Clear all packet callbacks
        for (int i = 0; i < MAX_STREAMS; i++) {
            stream_reader_ctx_t *reader = get_stream_reader_by_index(i);
            if (reader) {
                set_packet_callback(reader, NULL, NULL);
            }
        }
        
        // Stop all streams first
        for (int i = 0; i < config.max_streams; i++) {
            if (config.streams[i].name[0] != '\0') {
                stream_handle_t stream = get_stream_by_name(config.streams[i].name);
                if (stream) {
                    stop_stream(stream);
                }
            }
        }
        
        // Wait a moment
        usleep(1000000);  // 1 second
        
        // Close all MP4 writers first
        close_all_mp4_writers();
        
        // Then clean up backends in the correct order
        shutdown_detection_stream_system();
        cleanup_mp4_recording_backend();
        cleanup_hls_streaming_backend();
        cleanup_stream_reader_backend();
        cleanup_transcoding_backend();
        
        // Shut down remaining components
        http_server_stop(http_server);
        http_server_destroy(http_server);
        shutdown_stream_manager();
        shutdown_stream_state_adapter();
        shutdown_stream_state_manager();
        shutdown_storage_manager();
        shutdown_database();
        
        // Clean up the shutdown coordinator
        shutdown_coordinator_cleanup();
        
        // Restore signal mask
        pthread_sigmask(SIG_SETMASK, &old_mask, NULL);
    }

    // Handle PID file cleanup based on mode
    if (daemon_mode) {
        // In daemon mode, call cleanup_daemon to handle the PID file
        cleanup_daemon();
    } else if (pid_fd >= 0) {
        // In normal mode, remove the PID file directly
        remove_pid_file(pid_fd, config.pid_file);
    }

    log_info("Cleanup complete, shutting down");
    
    // Shutdown logging
    shutdown_logger();

    return EXIT_SUCCESS;
}

/**
 * Function to check and ensure recording is active for streams that have recording enabled
 */
static void check_and_ensure_recording(void) {
    for (int i = 0; i < config.max_streams; i++) {
        if (config.streams[i].name[0] != '\0' && config.streams[i].enabled && config.streams[i].record) {
            // Check if MP4 recording is active for this stream
            int recording_state = get_recording_state(config.streams[i].name);
            
            if (recording_state == 0) {
                // Recording is not active, start it
                log_info("Ensuring MP4 recording is active for stream: %s", config.streams[i].name);
                
                #ifdef USE_GO2RTC
                // First ensure HLS streaming is active (required for MP4 recording)
                if (go2rtc_integration_start_hls(config.streams[i].name) != 0) {
                    log_warn("Failed to start HLS streaming for stream: %s", config.streams[i].name);
                    // Continue anyway, as the HLS streaming might already be running
                }
                
                // Start MP4 recording
                if (go2rtc_integration_start_recording(config.streams[i].name) != 0) {
                    log_warn("Failed to start MP4 recording for stream: %s", config.streams[i].name);
                } else {
                    log_info("Successfully started MP4 recording for stream: %s (using go2rtc if available)", config.streams[i].name);
                }
                #else
                // First ensure HLS streaming is active (required for MP4 recording)
                if (start_hls_stream(config.streams[i].name) != 0) {
                    log_warn("Failed to start HLS streaming for stream: %s", config.streams[i].name);
                    // Continue anyway, as the HLS streaming might already be running
                }
                
                // Start MP4 recording
                if (start_mp4_recording(config.streams[i].name) != 0) {
                    log_warn("Failed to start MP4 recording for stream: %s", config.streams[i].name);
                } else {
                    log_info("Successfully started MP4 recording for stream: %s", config.streams[i].name);
                }
                #endif
            }
        }
    }
}
