#define _GNU_SOURCE
#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>
#include <signal.h>
#include <sys/types.h>
#include <sys/stat.h>
#include <string.h>
#include <fcntl.h>
#include <errno.h>
#include <stdbool.h>
#include <sys/wait.h>
#include <sys/socket.h>
#include <sys/utsname.h>
#include <pthread.h>

#include "web/web_server.h"
#include "core/logger.h"
#include "core/daemon.h"

extern volatile bool running; // Reference to the global variable defined in main.c

// Global variable to store PID file path
static char pid_file_path[256] = "/run/lightnvr.pid";

// Forward declarations
static void daemon_signal_handler(int sig);
static int write_pid_file(const char *pid_file);
static int check_running_daemon(const char *pid_file);

// Initialize daemon - simplified version that works on all platforms including Linux 4.4
int init_daemon(const char *pid_file) {
    // Store PID file path
    if (pid_file) {
        strncpy(pid_file_path, pid_file, sizeof(pid_file_path) - 1);
        pid_file_path[sizeof(pid_file_path) - 1] = '\0';
    }

    // Check if daemon is already running
    if (check_running_daemon(pid_file_path) != 0) {
        log_error("Daemon is already running");
        return -1;
    }

    log_info("Starting daemon mode with PID file: %s", pid_file_path);
    log_info("Current working directory: %s", getcwd(NULL, 0));

    // Fork the process
    pid_t pid = fork();
    if (pid < 0) {
        log_error("Failed to fork daemon process: %s", strerror(errno));
        return -1;
    }

    // Parent process exits
    if (pid > 0) {
        log_info("Parent process exiting, child PID: %d", pid);
        // Wait briefly to ensure child has time to start and write PID file
        usleep(200000); // 200ms - increased from 100ms for better reliability
        exit(EXIT_SUCCESS);
    }

    // Child process continues
    log_info("Child process starting daemon initialization, PID: %d", getpid());

    // Create a new session
    pid_t sid = setsid();
    if (sid < 0) {
        log_error("Failed to create session: %s", strerror(errno));
        return -1;
    }
    log_info("Created new session with SID: %d", sid);

    // DO NOT change working directory - this causes SQLite locking issues on Linux 4.4
    char *cwd = getcwd(NULL, 0);
    log_info("Keeping current working directory for SQLite compatibility: %s", cwd ? cwd : "unknown");
    if (cwd) free(cwd);

    // Reset file creation mask
    umask(0);
    log_debug("Reset file creation mask");

    // DO NOT close file descriptors in daemon mode
    // This was causing issues with the web server socket
    log_info("Keeping all file descriptors open for web server compatibility");

    // Setup signal handlers with SA_RESTART for better compatibility
    struct sigaction sa;
    memset(&sa, 0, sizeof(sa));
    sa.sa_handler = daemon_signal_handler;
    sigemptyset(&sa.sa_mask);
    sa.sa_flags = SA_RESTART;

    // Handle termination signals
    if (sigaction(SIGTERM, &sa, NULL) != 0) {
        log_warn("Failed to set SIGTERM handler: %s", strerror(errno));
    }
    if (sigaction(SIGINT, &sa, NULL) != 0) {
        log_warn("Failed to set SIGINT handler: %s", strerror(errno));
    }
    if (sigaction(SIGHUP, &sa, NULL) != 0) {
        log_warn("Failed to set SIGHUP handler: %s", strerror(errno));
    }
    if (sigaction(SIGALRM, &sa, NULL) != 0) {
        log_warn("Failed to set SIGALRM handler: %s", strerror(errno));
    }
    log_info("Signal handlers configured");

    // Block SIGPIPE to prevent crashes when writing to closed sockets
    sigset_t set;
    sigemptyset(&set);
    sigaddset(&set, SIGPIPE);
    if (pthread_sigmask(SIG_BLOCK, &set, NULL) != 0) {
        log_warn("Failed to block SIGPIPE: %s", strerror(errno));
    } else {
        log_debug("SIGPIPE blocked successfully");
    }

    // Write PID file
    log_info("Writing PID file...");
    if (write_pid_file(pid_file_path) != 0) {
        log_error("Failed to write PID file, daemon initialization failed");
        return -1;
    }

    log_info("Daemon started successfully with PID %d", getpid());

    // Add a small delay to ensure everything is properly initialized
    usleep(100000); // 100ms

    return 0;
}

// Cleanup daemon resources
int cleanup_daemon(void) {
    log_info("Cleaning up daemon resources");
    
    // Remove PID file
    if (remove_daemon_pid_file(pid_file_path) != 0) {
        log_warn("Failed to remove PID file during cleanup");
        // Continue anyway, not a fatal error
    }

    return 0;
}

// Signal handler for daemon
static void daemon_signal_handler(int sig) {
    switch (sig) {
    case SIGTERM:
    case SIGINT:
        log_info("Received signal %d, shutting down daemon...", sig);

        // Set global flag to stop main loop
        running = false;

        // Also signal the web server to shut down
        extern int web_server_socket;
        if (web_server_socket >= 0) {
            // First try to shutdown the socket
            if (shutdown(web_server_socket, SHUT_RDWR) != 0) {
                log_warn("Failed to shutdown server socket: %s", strerror(errno));
            }
            
            // Then close it
            if (close(web_server_socket) != 0) {
                log_warn("Failed to close server socket: %s", strerror(errno));
            }
            
            web_server_socket = -1; // Update the global reference
        }
        
        // Set an alarm to force exit after 30 seconds if normal shutdown fails
        alarm(30);
        break;

    case SIGHUP:
        // Reload configuration
        log_info("Received SIGHUP, reloading configuration...");
        // TODO: Implement configuration reload
        break;
        
    case SIGALRM:
        // Handle the alarm signal (fallback for forced exit)
        log_warn("Alarm triggered - forcing daemon exit");
        
        // Force kill any child processes before exiting
        log_warn("Sending SIGKILL to all child processes");
        kill(0, SIGKILL); // Send SIGKILL to all processes in the process group
        
        // Force exit without calling atexit handlers
        log_warn("Forcing immediate exit");
        _exit(EXIT_SUCCESS); // Use _exit instead of exit to avoid calling atexit handlers
        break;

    default:
        break;
    }
}

// Write PID file
static int write_pid_file(const char *pid_file) {
    // Make sure the directory exists
    char dir_path[256] = {0};
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
    
    // Open the file with exclusive locking
    int fd = open(pid_file, O_RDWR | O_CREAT, 0644);
    if (fd < 0) {
        log_error("Failed to open PID file %s: %s", pid_file, strerror(errno));
        return -1;
    }
    
    // Try to lock the file
    if (lockf(fd, F_TLOCK, 0) < 0) {
        log_error("Failed to lock PID file %s: %s", pid_file, strerror(errno));
        close(fd);
        return -1;
    }
    
    // Truncate the file to ensure we overwrite any existing content
    if (ftruncate(fd, 0) < 0) {
        log_warn("Could not truncate PID file: %s", strerror(errno));
        // Continue anyway
    }
    
    // Write PID to file
    char pid_str[16];
    sprintf(pid_str, "%d\n", getpid());
    if (write(fd, pid_str, strlen(pid_str)) != strlen(pid_str)) {
        log_error("Failed to write to PID file %s: %s", pid_file, strerror(errno));
        close(fd);
        unlink(pid_file);
        return -1;
    }
    
    // Sync to ensure the PID is written to disk
    fsync(fd);
    
    // Keep the file descriptor open to maintain the lock
    // We'll close it when the daemon exits
    
    // Set permissions to allow other processes to read the file
    if (chmod(pid_file, 0644) != 0) {
        log_warn("Failed to set permissions on PID file: %s", strerror(errno));
        // Not a fatal error, continue
    }
    
    log_info("Wrote PID %d to file %s", getpid(), pid_file);
    return 0;
}

// Remove PID file
int remove_daemon_pid_file(const char *pid_file) {
    // Try to remove the file
    if (unlink(pid_file) != 0) {
        if (errno == ENOENT) {
            // File doesn't exist, that's fine
            log_info("PID file %s already removed", pid_file);
            return 0;
        }
        
        log_error("Failed to remove PID file %s: %s", pid_file, strerror(errno));
        return -1;
    }

    log_info("Removed PID file %s", pid_file);
    return 0;
}

// Check if daemon is already running
static int check_running_daemon(const char *pid_file) {
    // First try to open the file with exclusive locking
    int fd = open(pid_file, O_RDWR);
    if (fd < 0) {
        if (errno == ENOENT) {
            // PID file doesn't exist, daemon is not running
            return 0;
        }
        
        log_error("Failed to open PID file %s: %s", pid_file, strerror(errno));
        return -1;
    }
    
    // Try to lock the file
    if (lockf(fd, F_TLOCK, 0) == 0) {
        // We got the lock, which means no other process has it
        // This is a stale PID file
        close(fd);
        log_warn("Found stale PID file %s (not locked), removing it", pid_file);
        remove_daemon_pid_file(pid_file);
        return 0;
    }
    
    // File is locked by another process, read the PID
    char pid_str[16];
    ssize_t bytes_read = read(fd, pid_str, sizeof(pid_str) - 1);
    close(fd);
    
    if (bytes_read <= 0) {
        log_error("Failed to read PID from file %s", pid_file);
        return -1;
    }
    
    // Null-terminate the string
    pid_str[bytes_read] = '\0';
    
    // Parse the PID
    pid_t pid;
    if (sscanf(pid_str, "%d", &pid) != 1) {
        log_error("Failed to parse PID from file %s", pid_file);
        return -1;
    }
    
    // Check if process is running
    if (kill(pid, 0) == 0) {
        // Process is running
        log_error("Daemon is already running with PID %d", pid);
        return 1;
    } else {
        if (errno == ESRCH) {
            // Process is not running, but file is locked?
            // This is unusual, but could happen if the file is locked by another process
            log_warn("Found stale PID file %s (locked but process %d not running), removing it", pid_file, pid);
            remove_daemon_pid_file(pid_file);
            return 0;
        } else {
            log_error("Failed to check process status: %s", strerror(errno));
            return -1;
        }
    }
}

// Stop running daemon
int stop_daemon(const char *pid_file) {
    char file_path[256];

    if (pid_file) {
        strncpy(file_path, pid_file, sizeof(file_path) - 1);
        file_path[sizeof(file_path) - 1] = '\0';
    } else {
        strncpy(file_path, pid_file_path, sizeof(file_path) - 1);
        file_path[sizeof(file_path) - 1] = '\0';
    }

    // Open and read PID file
    int fd = open(file_path, O_RDONLY);
    if (fd < 0) {
        if (errno == ENOENT) {
            log_info("PID file %s does not exist, daemon is not running", file_path);
            return 0;
        }
        
        log_error("Failed to open PID file %s: %s", file_path, strerror(errno));
        return -1;
    }

    // Read PID from file
    char pid_str[16];
    ssize_t bytes_read = read(fd, pid_str, sizeof(pid_str) - 1);
    close(fd);
    
    if (bytes_read <= 0) {
        log_error("Failed to read PID from file %s", file_path);
        return -1;
    }
    
    // Null-terminate the string
    pid_str[bytes_read] = '\0';
    
    // Parse the PID
    pid_t pid;
    if (sscanf(pid_str, "%d", &pid) != 1) {
        log_error("Failed to parse PID from file %s", file_path);
        return -1;
    }

    // First try SIGTERM for a graceful shutdown
    log_info("Sending SIGTERM to process %d", pid);
    if (kill(pid, SIGTERM) != 0) {
        if (errno == ESRCH) {
            // Process has already terminated
            log_info("Process %d has already terminated", pid);
            remove_daemon_pid_file(file_path);
            return 0;
        } else {
            log_warn("Failed to send SIGTERM to process %d: %s", pid, strerror(errno));
            // Continue to try SIGKILL
        }
    } else {
        // Wait for process to terminate after SIGTERM (doubled from 30 to 60 iterations)
        for (int i = 0; i < 60; i++) {
            if (kill(pid, 0) != 0) {
                if (errno == ESRCH) {
                    // Process has terminated
                    log_info("Process %d has terminated after SIGTERM", pid);
                    
                    // Wait for PID file to be released
                    for (int j = 0; j < 10; j++) {
                        // Check if PID file still exists and is locked
                        int test_fd = open(file_path, O_RDWR);
                        if (test_fd < 0) {
                            if (errno == ENOENT) {
                                // PID file doesn't exist anymore, we're good
                                log_info("PID file has been removed by the process");
                                return 0;
                            }
                            // Some other error, continue waiting
                        } else {
                            // Try to lock the file
                            if (lockf(test_fd, F_TLOCK, 0) == 0) {
                                // We got the lock, which means the previous process released it
                                close(test_fd);
                                log_info("PID file lock released");
                                remove_daemon_pid_file(file_path);
                                return 0;
                            }
                            close(test_fd);
                        }
                        usleep(100000); // 100ms
                    }
                    
                    // If we get here, the PID file still exists and is locked, or some other issue
                    log_warn("Process terminated but PID file is still locked or inaccessible");
                    // Try to remove it anyway
                    remove_daemon_pid_file(file_path);
                    return 0;
                }
            }
            // Sleep for 100ms
            usleep(100000);
        }
        
        log_warn("Process did not terminate after SIGTERM, trying SIGKILL");
    }

    // If SIGTERM didn't work or timed out, use SIGKILL
    log_info("Sending SIGKILL to process %d", pid);
    if (kill(pid, SIGKILL) != 0) {
        if (errno == ESRCH) {
            // Process has terminated
            log_info("Process %d has terminated", pid);
            remove_daemon_pid_file(file_path);
            return 0;
        } else {
            log_error("Failed to send SIGKILL to process %d: %s", pid, strerror(errno));
            return -1;
        }
    }

    // Wait for process to terminate after SIGKILL
    for (int i = 0; i < 50; i++) {
        if (kill(pid, 0) != 0) {
            if (errno == ESRCH) {
                // Process has terminated
                log_info("Process %d has terminated after SIGKILL", pid);
                
                // Wait for PID file to be released
                for (int j = 0; j < 10; j++) {
                    // Check if PID file still exists
                    if (access(file_path, F_OK) != 0) {
                        // PID file doesn't exist anymore, we're good
                        log_info("PID file has been removed");
                        return 0;
                    }
                    usleep(100000); // 100ms
                }
                
                // If we get here, the PID file still exists
                log_warn("Process terminated but PID file still exists");
                remove_daemon_pid_file(file_path);
                return 0;
            }
        }
        // Sleep for 100ms
        usleep(100000);
    }

    log_error("Failed to terminate process %d", pid);
    return -1;
}

// Get status of daemon
int daemon_status(const char *pid_file) {
    char file_path[256];

    if (pid_file) {
        strncpy(file_path, pid_file, sizeof(file_path) - 1);
        file_path[sizeof(file_path) - 1] = '\0';
    } else {
        strncpy(file_path, pid_file_path, sizeof(file_path) - 1);
        file_path[sizeof(file_path) - 1] = '\0';
    }

    // First try to open the file with exclusive locking
    int fd = open(file_path, O_RDWR);
    if (fd < 0) {
        if (errno == ENOENT) {
            // PID file doesn't exist, daemon is not running
            return 0;
        }
        
        log_error("Failed to open PID file %s: %s", file_path, strerror(errno));
        return -1;
    }
    
    // Try to lock the file
    if (lockf(fd, F_TLOCK, 0) == 0) {
        // We got the lock, which means no other process has it
        // This is a stale PID file
        close(fd);
        log_warn("Found stale PID file %s (not locked), removing it", file_path);
        remove_daemon_pid_file(file_path);
        return 0;
    }
    
    // File is locked by another process, read the PID
    char pid_str[16];
    ssize_t bytes_read = read(fd, pid_str, sizeof(pid_str) - 1);
    close(fd);
    
    if (bytes_read <= 0) {
        log_error("Failed to read PID from file %s", file_path);
        return -1;
    }
    
    // Null-terminate the string
    pid_str[bytes_read] = '\0';
    
    // Parse the PID
    pid_t pid;
    if (sscanf(pid_str, "%d", &pid) != 1) {
        log_error("Failed to parse PID from file %s", file_path);
        return -1;
    }
    
    // Check if process is running
    if (kill(pid, 0) == 0) {
        // Process is running
        log_info("Daemon is running with PID %d", pid);
        return 1;
    } else {
        if (errno == ESRCH) {
            // Process is not running, but file is locked?
            // This is unusual, but could happen if the file is locked by another process
            log_warn("Found stale PID file %s (locked but process %d not running), removing it", file_path, pid);
            remove_daemon_pid_file(file_path);
            return 0;
        } else {
            log_error("Failed to check process status: %s", strerror(errno));
            return -1;
        }
    }
}
