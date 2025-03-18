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
#include "video/hls_streaming.h"
#include "video/mp4_recording.h"
#include "video/stream_transcoding.h"
#include "video/detection_stream.h"
#include "video/detection_recording.h"
#include "video/detection.h"
#include "video/stream_state.h"
#include "video/stream_packet_processor.h"

// External function declarations
void init_recordings_system(void);
void register_detection_api_handlers(void);
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

// global config
config_t config;

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
                
                // Create a state manager for this stream to ensure we're using the newer patterns
                stream_state_manager_t *state = create_stream_state(&config->streams[i]);
                if (state) {
                    log_info("Created state manager for stream: %s", config->streams[i].name);
                } else {
                    log_warn("Failed to create state manager for stream: %s", config->streams[i].name);
                }

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

    // Copy configuration to global streaming config
    extern config_t global_config;  // Declared in streams.c
    memcpy(&global_config, &config, sizeof(config_t));

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

    // Initialize stream manager
    if (init_stream_manager(config.max_streams) != 0) {
        log_error("Failed to initialize stream manager");
        goto cleanup;
    }
    load_streams_from_config(&config);

    // Initialize FFmpeg streaming backend
    init_transcoding_backend();
    init_hls_streaming_backend();
    init_mp4_recording_backend();
    
    // Initialize packet processor
    if (init_packet_processor() != 0) {
        log_error("Failed to initialize packet processor");
    } else {
        log_info("Packet processor initialized successfully");
    }
    
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
    
    // Check if detection models exist and start detection-based recording
    for (int i = 0; i < config.max_streams; i++) {
        if (config.streams[i].name[0] != '\0' && config.streams[i].enabled && 
            config.streams[i].detection_based_recording && config.streams[i].detection_model[0] != '\0') {
            
            // Check if model file exists
            char model_path[MAX_PATH_LENGTH];
            if (config.streams[i].detection_model[0] != '/') {
                // Relative path, check in models directory
                char cwd[MAX_PATH_LENGTH];
                if (getcwd(cwd, sizeof(cwd)) != NULL) {
                    snprintf(model_path, MAX_PATH_LENGTH, "%s/models/%s", cwd, config.streams[i].detection_model);
                } else {
                    snprintf(model_path, MAX_PATH_LENGTH, "/var/lib/lightnvr/models/%s", config.streams[i].detection_model);
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
                
                // Try alternative locations
                const char *locations[] = {
                    "./", // Current directory
                    "./build/models/", // Build directory
                    "../models/", // Parent directory
                    "/var/lib/lightnvr/models/" // System directory
                };
                
                bool found = false;
                for (int j = 0; j < sizeof(locations)/sizeof(locations[0]); j++) {
                    char alt_path[MAX_PATH_LENGTH];
                    snprintf(alt_path, MAX_PATH_LENGTH, "%s%s", locations[j], config.streams[i].detection_model);
                    
                    FILE *alt_file = fopen(alt_path, "r");
                    if (alt_file) {
                        fclose(alt_file);
                        log_info("Detection model found at alternative location: %s", alt_path);
                        found = true;
                        break;
                    }
                }
                
                if (!found) {
                    log_error("Detection model not found in any location: %s", config.streams[i].detection_model);
                    log_error("Detection will not work properly!");
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

    // Initialize web server
    if (init_web_server(config.web_port, config.web_root) != 0) {
        log_error("Failed to initialize web server");
        goto cleanup;
    }

    // Set authentication if enabled in config
    if (config.web_auth_enabled) {
        log_info("Enabling web authentication");
        if (set_authentication(true, config.web_username, config.web_password) != 0) {
            log_error("Failed to set authentication");
            // Continue anyway, authentication will be disabled
        }
    } else {
        log_info("Web authentication is disabled");
    }

    // Register streaming API handlers
    register_streaming_api_handlers();
    
    // Register detection API handlers
    register_detection_api_handlers();

    log_info("LightNVR initialized successfully");

    // Print initial detection stream status
    print_detection_stream_status();

    // Main loop
    while (running) {
        // Log that the daemon is still running (maybe once per minute)
        static time_t last_log_time = 0;
        static time_t last_status_time = 0;
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
        
        // Clean up packet processor
        log_info("Shutting down packet processor...");
        shutdown_packet_processor();
        
        // Clean up FFmpeg resources
        log_info("Cleaning up transcoding backend...");
        cleanup_transcoding_backend();
        
        // Now shut down other components
        log_info("Shutting down web server...");
        shutdown_web_server();
        
        log_info("Shutting down stream manager...");
        shutdown_stream_manager();
        
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
        shutdown_web_server();
        shutdown_stream_manager();
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
