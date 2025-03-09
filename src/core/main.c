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

#include "core/version.h"
#include "core/config.h"
#include "core/logger.h"
#include "core/daemon.h"
#include "video/stream_manager.h"
#include "storage/storage_manager.h"
#include "video/streams.h"

// External function declaration
void init_recordings_system(void);
#include "database/database_manager.h"
#include "web/web_server.h"
#include "video/streams.h"

// Include necessary headers for signal handling
#include <signal.h>
#include <fcntl.h>

// Global flag for graceful shutdown
volatile bool running = true;

// Global flag for daemon mode (made extern so web_server.c can access it)
bool daemon_mode = false;

// Declare a global variable to store the web server socket
static int web_server_socket = -1;

// Function to set the web server socket
void set_web_server_socket(int socket_fd) {
    web_server_socket = socket_fd;
}

static void signal_handler(int sig) {
    // Only log and set running flag if not in daemon mode
    // Daemon mode has its own signal handler
    log_info("Received signal %d, shutting down...", sig);
    running = false;
}

// Function to initialize signal handlers
static void init_signals() {
    // Only set up signal handlers in non-daemon mode
    // Daemon mode sets up its own handlers in init_daemon()
    if (!daemon_mode) {
        struct sigaction sa;
        memset(&sa, 0, sizeof(sa));
        sa.sa_handler = signal_handler;
        sigaction(SIGINT, &sa, NULL);
        sigaction(SIGTERM, &sa, NULL);
        sigaction(SIGHUP, &sa, NULL);
    }
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
        log_info("Attempting to terminate previous instance (PID: %d)", existing_pid);

        // Send SIGTERM to let it clean up
        if (kill(existing_pid, SIGTERM) == 0) {
            // Wait a bit for it to terminate
            int timeout = 5;  // 5 seconds
            while (timeout-- > 0 && kill(existing_pid, 0) == 0) {
                sleep(1);
            }

            // If still running, force kill
            if (timeout <= 0 && kill(existing_pid, 0) == 0) {
                log_warn("Process didn't terminate gracefully, using SIGKILL");
                kill(existing_pid, SIGKILL);
                sleep(1);  // Give it a moment
            }

            log_info("Previous instance terminated");
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
    
    fd = open(pid_file, O_RDWR | O_CREAT, 0644);
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
    
    // Write PID to file
    sprintf(pid_str, "%d\n", getpid());
    if (write(fd, pid_str, strlen(pid_str)) != strlen(pid_str)) {
        log_error("Could not write to PID file %s: %s", pid_file, strerror(errno));
        close(fd);
        return -1;
    }
    
    // Keep file open to maintain lock
    return fd;
}

// Function to remove PID file
static void remove_pid_file(int fd, const char *pid_file) {
    if (fd >= 0) {
        close(fd);
    }
    unlink(pid_file);
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
                            if (start_hls_stream(config->streams[i].name) == 0) {
                                log_info("Recording started for stream: %s", config->streams[i].name);
                            } else {
                                log_warn("Failed to start recording for stream: %s", config->streams[i].name);
                            }
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
    config_t config;

    // Print banner
    printf("LightNVR v%s - Lightweight NVR for Ingenic A1\n", LIGHTNVR_VERSION_STRING);
    printf("Build date: %s\n", LIGHTNVR_BUILD_DATE);

    // Initialize logging
    if (init_logger() != 0) {
        fprintf(stderr, "Failed to initialize logger\n");
        return EXIT_FAILURE;
    }

    // Parse command line arguments
    for (int i = 1; i < argc; i++) {
        if (strcmp(argv[i], "-d") == 0 || strcmp(argv[i], "--daemon") == 0) {
            daemon_mode = true;
        } else if (strcmp(argv[i], "-c") == 0 || strcmp(argv[i], "--config") == 0) {
            if (i + 1 < argc) {
                // Set config file path
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

    // Load configuration
    if (load_config(&config) != 0) {
        log_error("Failed to load configuration");
        return EXIT_FAILURE;
    }

    // Copy configuration to global streaming config
    extern config_t global_config;  // Declared in streams.c
    memcpy(&global_config, &config, sizeof(config_t));

    // Verify web root directory exists and is readable
    struct stat st;
    if (stat(config.web_root, &st) != 0 || !S_ISDIR(st.st_mode)) {
        log_error("Web root directory %s does not exist or is not a directory", config.web_root);

        // Try to create it
        if (mkdir(config.web_root, 0755) != 0) {
            log_error("Failed to create web root directory: %s", strerror(errno));
            return EXIT_FAILURE;
        }

        log_info("Created web root directory: %s", config.web_root);
    }

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

    // Initialize storage manager
    if (init_storage_manager(config.storage_path, config.max_storage_size) != 0) {
        log_error("Failed to initialize storage manager");
        goto cleanup;
    }

    // Initialize stream manager
    if (init_stream_manager(config.max_streams) != 0) {
        log_error("Failed to initialize stream manager");
        goto cleanup;
    }
    load_streams_from_config(&config);

    // Initialize FFmpeg streaming backend
    init_streaming_backend();

    // Initialize web server
    if (init_web_server(config.web_port, config.web_root) != 0) {
        log_error("Failed to initialize web server");
        goto cleanup;
    }

    // Register streaming API handlers
    register_streaming_api_handlers();

    log_info("LightNVR initialized successfully");

    // Main loop
    while (running) {
        // Log that the daemon is still running (maybe once per minute)
        static time_t last_log_time = 0;
        time_t now = time(NULL);
        if (now - last_log_time > 60) {
            log_debug("Daemon is still running...");
            last_log_time = now;
        }

        // Process events, monitor system health, etc.
        sleep(1);
    }

    log_info("Shutting down LightNVR...");

    // Cleanup
cleanup:
    log_info("Starting cleanup process...");
    
    // Set up a watchdog timer to force exit if cleanup takes too long
    pid_t cleanup_pid = fork();
    
    if (cleanup_pid == 0) {
        // Child process - watchdog timer
        sleep(30);  // Wait 30 seconds
        log_error("Cleanup process timed out after 30 seconds, forcing exit");
        kill(getppid(), SIGKILL);  // Force kill the parent process
        exit(EXIT_FAILURE);
    } else if (cleanup_pid > 0) {
        // Parent process - continue with cleanup
        
        // First stop all streams to ensure proper finalization of recordings
        log_info("Stopping all active streams...");
        cleanup_streaming_backend();  // Cleanup FFmpeg streaming
        
        // Finalize all MP4 recordings
        log_info("Finalizing all MP4 recordings...");
        close_all_mp4_writers();
        
        // Clean up HLS directories
        log_info("Cleaning up HLS directories...");
        cleanup_hls_directories();
        
        // Now shut down other components
        shutdown_web_server();
        shutdown_stream_manager();
        shutdown_storage_manager();
        shutdown_database();
        
        // Kill the watchdog timer since we completed successfully
        kill(cleanup_pid, SIGKILL);
        waitpid(cleanup_pid, NULL, 0);
    } else {
        // Fork failed
        log_error("Failed to create watchdog process for cleanup timeout");
        // Continue with cleanup anyway
        cleanup_streaming_backend();
        close_all_mp4_writers();
        cleanup_hls_directories();
        shutdown_web_server();
        shutdown_stream_manager();
        shutdown_storage_manager();
        shutdown_database();
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
