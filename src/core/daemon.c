#define _GNU_SOURCE
#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>
#include <signal.h>
#include <sys/types.h>
#include <sys/stat.h>
#include <string.h>
#include <syslog.h>
#include <fcntl.h>
#include <errno.h>
#include <stdbool.h>
#include <sys/wait.h>
#include <sys/time.h>

#include <sys/socket.h>
#include <netinet/in.h>
#include <arpa/inet.h>

#include "web/web_server.h"
#include "core/logger.h"
#include "core/daemon.h"

extern volatile bool running; // Reference to the global variable defined in main.c

// Global variable to store PID file path
static char pid_file_path[256] = "/var/run/lightnvr.pid";

// Forward declarations
static void daemon_signal_handler(int sig);
static int write_pid_file(const char *pid_file);
static int check_running_daemon(const char *pid_file);

// Initialize daemon
int init_daemon(const char *pid_file) {
    // Check if we're already running
    if (pid_file) {
        strncpy(pid_file_path, pid_file, sizeof(pid_file_path) - 1);
        pid_file_path[sizeof(pid_file_path) - 1] = '\0';
    }

    if (check_running_daemon(pid_file_path) != 0) {
        log_error("Daemon is already running");
        return -1;
    }

    // Fork the process
    pid_t pid = fork();
    if (pid < 0) {
        log_error("Failed to fork daemon process: %s", strerror(errno));
        return -1;
    }

    // Parent process exits
    if (pid > 0) {
        // Wait briefly to ensure child has time to start
        usleep(100000); // 100ms
        exit(EXIT_SUCCESS);
    }

    // Child process continues

    // Create a new session
    pid_t sid = setsid();
    if (sid < 0) {
        log_error("Failed to create session: %s", strerror(errno));
        return -1;
    }

    // Change working directory to a safe location
    if (chdir("/var/lib/lightnvr") < 0) {
        // If that fails, try /tmp as a fallback
        if (chdir("/tmp") < 0) {
            log_error("Failed to change working directory: %s", strerror(errno));
            return -1;
        }
    }

    // Reset file creation mask
    umask(0);

    // Close all open file descriptors except for logging
    int max_fd = sysconf(_SC_OPEN_MAX);
    for (int i = 3; i < max_fd; i++) {
        // Skip the server socket if it's already set up
        extern int web_server_socket;  // From main.c
        if (i != web_server_socket || web_server_socket < 0) {
            close(i);
        }
    }

    // Redirect standard file descriptors to /dev/null
    int dev_null = open("/dev/null", O_RDWR);
    if (dev_null == -1) {
        log_error("Failed to open /dev/null: %s", strerror(errno));
        return -1;
    }

    if (dup2(dev_null, STDIN_FILENO) == -1 ||
        dup2(dev_null, STDOUT_FILENO) == -1 ||
        dup2(dev_null, STDERR_FILENO) == -1) {
        log_error("Failed to redirect standard file descriptors: %s", strerror(errno));
        close(dev_null);
        return -1;
    }

    if (dev_null > STDERR_FILENO) {
        close(dev_null);
    }

    // Setup signal handlers in the child process
    struct sigaction sa;
    memset(&sa, 0, sizeof(sa));
    sa.sa_handler = daemon_signal_handler;
    sigemptyset(&sa.sa_mask);

    // Handle termination signals
    sigaction(SIGTERM, &sa, NULL);
    sigaction(SIGINT, &sa, NULL);
    sigaction(SIGHUP, &sa, NULL);
    sigaction(SIGALRM, &sa, NULL); // Add alarm signal handler for Linux 4.4 compatibility

    // Write PID file
    if (write_pid_file(pid_file_path) != 0) {
        return -1;
    }

    log_info("Daemon started successfully with PID %d", getpid());
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
            // Use a more robust approach for Linux 4.4 embedded systems
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
        
        // On Linux 4.4 embedded, we might need to force exit if signals aren't handled well
        // This is a fallback mechanism to ensure the daemon actually stops
        log_info("Setting up fallback exit timer for Linux 4.4 compatibility");
        alarm(5); // Set an alarm to force exit after 5 seconds if normal shutdown fails
        break;

    case SIGHUP:
        // Reload configuration
        log_info("Received SIGHUP, reloading configuration...");
        // TODO: Implement configuration reload
        break;
        
    case SIGALRM:
        // Handle the alarm signal (fallback for Linux 4.4)
        log_warn("Alarm triggered - forcing daemon exit for Linux 4.4 compatibility");
        _exit(EXIT_SUCCESS); // Use _exit instead of exit to avoid calling atexit handlers
        break;

    default:
        break;
    }
}

// Write PID file
static int write_pid_file(const char *pid_file) {
    FILE *fp = fopen(pid_file, "w");
    if (!fp) {
        log_error("Failed to open PID file %s: %s", pid_file, strerror(errno));
        return -1;
    }

    fprintf(fp, "%d\n", getpid());
    fclose(fp);

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
    if (unlink(pid_file) != 0 && errno != ENOENT) {
        log_error("Failed to remove PID file %s: %s", pid_file, strerror(errno));
        return -1;
    }

    log_info("Removed PID file %s", pid_file);
    return 0;
}

// Check if daemon is already running
static int check_running_daemon(const char *pid_file) {
    FILE *fp = fopen(pid_file, "r");
    if (!fp) {
        if (errno == ENOENT) {
            // PID file doesn't exist, daemon is not running
            return 0;
        }

        log_error("Failed to open PID file %s: %s", pid_file, strerror(errno));
        return -1;
    }

    // Read PID from file
    pid_t pid;
    if (fscanf(fp, "%d", &pid) != 1) {
        log_error("Failed to read PID from file %s", pid_file);
        fclose(fp);
        return -1;
    }

    fclose(fp);

    // Check if process is running
    if (kill(pid, 0) == 0) {
        // Process is running
        log_error("Daemon is already running with PID %d", pid);
        return 1;
    } else {
        if (errno == ESRCH) {
            // Process is not running, remove stale PID file
            log_warn("Found stale PID file %s, removing it", pid_file);
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
    FILE *fp = fopen(file_path, "r");
    if (!fp) {
        log_error("Failed to open PID file %s: %s", file_path, strerror(errno));
        return -1;
    }

    // Read PID from file
    pid_t pid;
    if (fscanf(fp, "%d", &pid) != 1) {
        log_error("Failed to read PID from file %s", file_path);
        fclose(fp);
        return -1;
    }

    fclose(fp);

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
        // Wait for process to terminate after SIGTERM
        for (int i = 0; i < 30; i++) {
            if (kill(pid, 0) != 0) {
                if (errno == ESRCH) {
                    // Process has terminated
                    log_info("Process %d has terminated after SIGTERM", pid);
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

    // Open and read PID file
    FILE *fp = fopen(file_path, "r");
    if (!fp) {
        if (errno == ENOENT) {
            // PID file doesn't exist, daemon is not running
            return 0;
        }

        log_error("Failed to open PID file %s: %s", file_path, strerror(errno));
        return -1;
    }

    // Read PID from file
    pid_t pid;
    if (fscanf(fp, "%d", &pid) != 1) {
        log_error("Failed to read PID from file %s", file_path);
        fclose(fp);
        return -1;
    }

    fclose(fp);

    // Check if process is running
    if (kill(pid, 0) == 0) {
        // Process is running
        return 1;
    } else {
        if (errno == ESRCH) {
            // Process is not running, remove stale PID file
            log_warn("Found stale PID file %s, removing it", file_path);
            remove_daemon_pid_file(file_path);
            return 0;
        } else {
            log_error("Failed to check process status: %s", strerror(errno));
            return -1;
        }
    }
}
