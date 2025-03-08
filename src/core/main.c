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

#include "core/version.h"
#include "core/config.h"
#include "core/logger.h"
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
static volatile bool running = true;

// Declare a global variable to store the web server socket
static int web_server_socket = -1;

// Function to set the web server socket
void set_web_server_socket(int socket_fd) {
    web_server_socket = socket_fd;
}

// Modify the signal handler
static void signal_handler(int sig) {
    log_info("Received signal %d, shutting down...", sig);
    running = false;
}

// Function to initialize signal handlers
static void init_signals() {
    struct sigaction sa;
    memset(&sa, 0, sizeof(sa));
    sa.sa_handler = signal_handler;
    sigaction(SIGINT, &sa, NULL);
    sigaction(SIGTERM, &sa, NULL);
    sigaction(SIGHUP, &sa, NULL);
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
static int daemonize() {
    pid_t pid;
    
    // Fork off the parent process
    pid = fork();
    if (pid < 0) {
        log_error("Failed to fork: %s", strerror(errno));
        return -1;
    }
    
    // If we got a good PID, then we can exit the parent process
    if (pid > 0) {
        exit(EXIT_SUCCESS);
    }
    
    // Create a new session ID for the child process
    if (setsid() < 0) {
        log_error("Failed to create new session: %s", strerror(errno));
        return -1;
    }
    
    // Ignore certain signals
    signal(SIGCHLD, SIG_IGN);
    signal(SIGHUP, SIG_IGN);
    
    // Fork a second time (recommended for daemons)
    pid = fork();
    if (pid < 0) {
        log_error("Failed to fork (second time): %s", strerror(errno));
        return -1;
    }
    
    if (pid > 0) {
        exit(EXIT_SUCCESS);
    }
    
    // Set new file permissions
    umask(0);
    
    // Change the current working directory to root (or another safe directory)
    // Instead of changing to /, change to a more appropriate directory
    // that will definitely exist and have proper permissions
    if (chdir("/var/lib/lightnvr") < 0) {
        // If that fails, try the user's home directory or /tmp
        if (chdir("/tmp") < 0) {
            log_error("Failed to change directory: %s", strerror(errno));
            return -1;
        }
    }
    
    // Redirect standard file descriptors to /dev/null
    int null_fd = open("/dev/null", O_RDWR);
    if (null_fd < 0) {
        log_error("Failed to open /dev/null: %s", strerror(errno));
        return -1;
    }
    
    // Close all open file descriptors except for the null descriptor
    for (int i = 0; i < sysconf(_SC_OPEN_MAX); i++) {
        if (i != null_fd) {
            close(i);
        }
    }
    
    // Redirect stdin, stdout, stderr to /dev/null
    if (dup2(null_fd, STDIN_FILENO) < 0 ||
        dup2(null_fd, STDOUT_FILENO) < 0 ||
        dup2(null_fd, STDERR_FILENO) < 0) {
        log_error("Failed to redirect standard file descriptors: %s", strerror(errno));
        close(null_fd);
        return -1;
    }
    
    close(null_fd);
    
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
    bool daemon_mode = false;
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

    // Daemonize if requested (before creating PID file)
    if (daemon_mode) {
        log_info("Starting in daemon mode");
        if (daemonize() != 0) {
            log_error("Failed to daemonize");
            return EXIT_FAILURE;
        }
    }

    // In main function, before creating the PID file:
    if (check_and_kill_existing_instance(config.pid_file) != 0) {
        log_error("Failed to handle existing instance");
        return EXIT_FAILURE;
    }

    // Create PID file
    pid_fd = create_pid_file(config.pid_file);
    if (pid_fd < 0) {
        log_error("Failed to create PID file");
        return EXIT_FAILURE;
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
        // Process events, monitor system health, etc.
        sleep(1);
    }

    log_info("Shutting down LightNVR...");

    // Cleanup
cleanup:
    shutdown_web_server();
    cleanup_streaming_backend();  // Cleanup FFmpeg streaming
    shutdown_stream_manager();
    shutdown_storage_manager();
    shutdown_database();

    // Remove PID file
    remove_pid_file(pid_fd, config.pid_file);

    // Shutdown logging
    shutdown_logger();

    return EXIT_SUCCESS;
}
