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
#include "video/detection_thread_pool.h"
#include "video/stream_packet_processor.h"
#include "video/timestamp_manager.h"
#include "video/onvif_discovery.h"

// External function declarations
void init_recordings_system(void);
#include "database/database_manager.h"
#include "web/http_server.h"
#include "web/mongoose_server.h"
#include "web/api_handlers.h"
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

static void signal_handler(int sig) {
    // Log the signal received
    log_info("Received signal %d, shutting down...", sig);
    
    // Set the running flag to false to trigger shutdown
    running = false;
    
    // For Linux 4.4 embedded systems, we need a more robust approach
    // Set an alarm to force exit if normal shutdown doesn't work
    alarm(10); // Force exit after 10 seconds if normal shutdown fails
}

// Alarm signal handler for forced exit
static void alarm_handler(int sig) {
    log_warn("Shutdown timeout reached, forcing exit");
    _exit(EXIT_SUCCESS); // Use _exit instead of exit to avoid calling atexit handlers
}

// Function to initialize signal handlers
static void init_signals() {
    // Set up signal handlers for both daemon and non-daemon mode
    // This ensures consistent behavior across all modes
    struct sigaction sa;
    memset(&sa, 0, sizeof(sa));
    sa.sa_handler = signal_handler;
    sigaction(SIGINT, &sa, NULL);
    sigaction(SIGTERM, &sa, NULL);
    sigaction(SIGHUP, &sa, NULL);
    
    // Set up alarm handler for forced exit
    struct sigaction sa_alarm;
    memset(&sa_alarm, 0, sizeof(sa_alarm));
    sa_alarm.sa_handler = alarm_handler;
    sigaction(SIGALRM, &sa_alarm, NULL);
    
    // Block SIGPIPE to prevent crashes when writing to closed sockets
    // This is especially important for older Linux kernels
    sigset_t set;
    sigemptyset(&set);
    sigaddset(&set, SIGPIPE);
    pthread_sigmask(SIG_BLOCK, &set, NULL);
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
    printf("LightNVR v%s - Lightweight NVR\n");
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
            // Disable console logging to avoid double logging
            set_console_logging(0);
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
    load_streams_from_config(&config);

    // Initialize FFmpeg streaming backend
    init_transcoding_backend();
    
    // Initialize timestamp trackers
    init_timestamp_trackers();
    log_info("Timestamp trackers initialized");
    
    // Initialize packet processor
    if (init_packet_processor() != 0) {
        log_error("Failed to initialize packet processor");
    } else {
        log_info("Packet processor initialized successfully");
    }
    
    init_hls_streaming_backend();
    init_mp4_recording_backend();
    
    // Initialize detection system
    if (init_detection_system() != 0) {
        log_error("Failed to initialize detection system");
    } else {
        log_info("Detection system initialized successfully");
    }
    
    // Initialize detection thread pool
    if (init_detection_thread_pool() != 0) {
        log_error("Failed to initialize detection thread pool");
    } else {
        log_info("Detection thread pool initialized successfully");
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
    
    // Check if detection models exist and start detection-based recording
    for (int i = 0; i < config.max_streams; i++) {
        if (config.streams[i].name[0] != '\0' && config.streams[i].enabled && 
            config.streams[i].detection_based_recording && config.streams[i].detection_model[0] != '\0') {
            
            // Check if model file exists
            char model_path[MAX_PATH_LENGTH];
            if (config.streams[i].detection_model[0] != '/') {
                // Relative path, use configured models path from INI if it exists
                if (config.models_path && strlen(config.models_path) > 0) {
                    snprintf(model_path, MAX_PATH_LENGTH, "%s/%s", config.models_path, config.streams[i].detection_model);
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
        .connection_pool_threads = config.connection_pool_threads  // Use the value from the configuration
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
            last_recording_check_time = now;
        }

        // Process events, monitor system health, etc.
        sleep(1);
    }

    log_info("Shutting down LightNVR...");

    // Cleanup
cleanup:
    log_info("Starting cleanup process...");
    
    // Block signals during cleanup to prevent interruptions
    sigset_t block_mask, old_mask;
    sigfillset(&block_mask);
    pthread_sigmask(SIG_BLOCK, &block_mask, &old_mask);
    
    // Set up a watchdog timer to force exit if cleanup takes too long
    pid_t cleanup_pid = fork();
    
    if (cleanup_pid == 0) {
        // Child process - watchdog timer
        sleep(60);  // Wait 60 seconds
        log_error("Cleanup process timed out after 60 seconds, forcing exit");
        kill(getppid(), SIGKILL);  // Force kill the parent process
        exit(EXIT_FAILURE);
    } else if (cleanup_pid > 0) {
        // Parent process - continue with cleanup
        
    // First stop all streams to ensure proper finalization of recordings
    log_info("Stopping all active streams...");
    
    // CRITICAL FIX: First, clear all packet callbacks to prevent further processing
    // This must be done before stopping streams to prevent race conditions
    log_info("Clearing all packet callbacks...");
    for (int i = 0; i < MAX_STREAMS; i++) {
        stream_reader_ctx_t *reader = get_stream_reader_by_index(i);
        if (reader) {
            // Safely clear the callback
            set_packet_callback(reader, NULL, NULL);
        }
    }
    
    // Wait a moment for callbacks to clear and any in-progress operations to complete
    usleep(500000);  // 500ms - increased from 250ms for better safety
    
    // Stop all detection stream readers first
    log_info("Stopping all detection stream readers...");
    for (int i = 0; i < config.max_streams; i++) {
        if (config.streams[i].name[0] != '\0' && 
            config.streams[i].detection_based_recording && 
            config.streams[i].detection_model[0] != '\0') {
            
            log_info("Stopping detection stream reader for: %s", config.streams[i].name);
            stop_detection_stream_reader(config.streams[i].name);
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
    usleep(1500000);  // 1500ms - increased from 1000ms for better safety
        
        // Finalize all MP4 recordings first before cleaning up the backend
        log_info("Finalizing all MP4 recordings...");
        close_all_mp4_writers();
        
        // Clean up HLS directories
        log_info("Cleaning up HLS directories...");
        cleanup_hls_directories();
        
        // Now clean up the backends in the correct order
        log_info("Cleaning up detection stream system...");
        shutdown_detection_stream_system();
        
        log_info("Cleaning up MP4 recording backend...");
        cleanup_mp4_recording_backend();  // Cleanup MP4 recording
        
        log_info("Cleaning up HLS streaming backend...");
        cleanup_hls_streaming_backend();  // Cleanup HLS streaming
        
        // Clean up stream reader backend last to ensure all consumers are stopped
        log_info("Cleaning up stream reader backend...");
        cleanup_stream_reader_backend();
        
        // Clean up FFmpeg resources
        log_info("Cleaning up transcoding backend...");
        cleanup_transcoding_backend();
        
        // Shutdown detection resources
        log_info("Cleaning up detection resources...");
        cleanup_detection_resources();
        
        // Shutdown detection thread pool
        log_info("Shutting down detection thread pool...");
        shutdown_detection_thread_pool();
        
        // Shutdown ONVIF discovery
        log_info("Shutting down ONVIF discovery module...");
        shutdown_onvif_discovery();
        
        // Now shut down other components
        log_info("Shutting down web server...");
        http_server_stop(http_server);
        http_server_destroy(http_server);
        
        log_info("Shutting down stream manager...");
        shutdown_stream_manager();
        
        log_info("Shutting down stream state adapter...");
        shutdown_stream_state_adapter();
        
        log_info("Shutting down stream state manager...");
        shutdown_stream_state_manager();
        
        log_info("Shutting down storage manager...");
        shutdown_storage_manager();
        
        log_info("Shutting down database...");
        shutdown_database();
        
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
            }
        }
    }
}
