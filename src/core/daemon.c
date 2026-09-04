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
#include "core/path_utils.h"
#include "core/shutdown_coordinator.h"
#include "utils/strings.h"

extern volatile bool running; // Reference to the global variable defined in main.c

// Global variable to store PID file path
static char pid_file_path[MAX_PATH_LENGTH] = "/run/lightnvr.pid";

// Forward declarations
static void daemon_signal_handler(int sig);
static int check_running_daemon(const char *pid_file);

// Initialize daemon - simplified version that works on all platforms including Linux 4.4
int init_daemon(const char *pid_file) {
    // Store PID file path
    if (pid_file) {
        safe_strcpy(pid_file_path, pid_file, sizeof(pid_file_path), 0);
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
    int fd = write_pid_file(pid_file_path);
    if (fd < 0) {
        log_error("Failed to write PID file, daemon initialization failed");
        return -1;
    }

    log_info("Daemon started successfully with PID %d", getpid());

    // Add a small delay to ensure everything is properly initialized
    usleep(100000); // 100ms

    return fd;
}

// Async-signal-safe write helper for daemon signal handler
static void daemon_signal_safe_write(const char *msg) {
    // write() is async-signal-safe
    if (msg) {
        size_t len = 0;
        while (msg[len] != '\0') len++;
        write(STDERR_FILENO, msg, len);
    }
}

// Signal handler for daemon
// IMPORTANT: This handler must be async-signal-safe - no mutex operations!
// Only use: write(), _exit(), alarm(), close(), shutdown(), atomic operations on sig_atomic_t
static void daemon_signal_handler(int sig) {
    switch (sig) {
    case SIGTERM:
    case SIGINT:
        // Use write() instead of log_info() - it's async-signal-safe
        daemon_signal_safe_write("[DAEMON] Received shutdown signal, initiating shutdown...\n");

        // Set global flag to stop main loop
        // This is safe because running is declared as volatile bool
        running = false;

        // Also wake up any long-running background operation (e.g. a
        // scheduled DB backup) that isn't polling `running` itself, so it
        // aborts promptly instead of blocking this shutdown until it
        // finishes on its own. This handler is the one actually registered
        // for SIGTERM/SIGINT in daemon mode -- daemonize() installs it via
        // sigaction() *after* main.c's init_signals() already ran, silently
        // overriding that handler's equivalent call for every real
        // `systemctl stop`/`kill` in production. request_background_abort()
        // is async-signal-safe (atomic store only).
        request_background_abort();

        // Also signal the web server to shut down
        extern int web_server_socket;
        if (web_server_socket >= 0) {
            // shutdown() and close() are async-signal-safe
            shutdown(web_server_socket, SHUT_RDWR);
            close(web_server_socket);
            web_server_socket = -1; // Update the global reference
        }

        // Deliberately NOT using alarm() here as a "force exit if shutdown
        // hangs" watchdog: alarm() is a single process-wide timer, and this
        // codebase's HLS writer/context-close code (hls_unified_thread.c,
        // hls_writer.c) uses alarm() extensively as a short per-operation
        // timeout (save disposition, alarm(N), do the call, alarm(0),
        // restore disposition). Any one of those firing during shutdown
        // would silently discard whatever time was left on an alarm set
        // here, since alarm() has no pause/resume -- confirmed live via
        // gdb: a shutdown that should have had ~570s left was killed at 45s
        // by one of those unrelated short-lived alarms, in this exact
        // handler, right after this comment block previously called
        // alarm(570) here. main.c's start_shutdown_watchdog_thread()
        // (a dedicated thread polling `running`, spawned once at startup)
        // now owns this responsibility instead, immune to that collision.
        break;

    case SIGHUP:
        // Just write a message, don't try to reload config in signal handler
        // Config reload is NOT async-signal-safe
        daemon_signal_safe_write("[DAEMON] Received SIGHUP signal\n");
        break;

    case SIGALRM:
        // No longer the shutdown watchdog (see main.c's
        // start_shutdown_watchdog_thread()). Kept registered, and
        // deliberately harmless, purely so SIGALRM has a caught
        // (non-terminating) disposition as a safety net: the many
        // hls_unified_thread.c/hls_writer.c call sites that use alarm() for
        // their own short per-operation timeouts save whatever handler is
        // installed here before temporarily switching it to SIG_IGN, and
        // restore it afterward. If this weren't registered, SIGALRM's
        // default disposition (process termination) would apply during any
        // brief window where none of those local overrides happen to be
        // active.
        daemon_signal_safe_write("[DAEMON] Stray SIGALRM caught at top level (harmless, ignored)\n");
        break;

    default:
        break;
    }
}

// Write PID file
int write_pid_file(const char *pid_file) {
    // Make sure the directory exists
    if (ensure_path(pid_file) != 0) {
        log_error("Failed to create PID file directory %s: %s", pid_file, strerror(errno));
        return -1;
    }
    
    // Open the file initially without exclusive locking
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
    
    // Set permissions to allow other processes to read the file.
    // Use fchmod(fd) instead of chmod(path) to avoid TOCTOU race condition:
    // operating on the already-open fd guarantees we chmod the same file we wrote.
    if (fchmod(fd, 0644) != 0) {
        log_warn("Failed to set permissions on PID file: %s", strerror(errno));
        // Not a fatal error, continue
    }
    
    log_info("Wrote PID %d to file %s", getpid(), pid_file);
    return fd;
}

// Remove PID file
int remove_pid_file(int fd, const char *pid_file) {
    if (fd >= 0) {
        // Release the lock by closing the file
        close(fd);
    }

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
        log_warn("Found stale PID file %s (not locked), removing it", pid_file);
        remove_pid_file(fd, pid_file);
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
    char *end_ptr;
    long pid_val = strtol(pid_str, &end_ptr, 10);
    if (end_ptr == pid_str || pid_val <= 0) {
        log_error("Failed to parse PID from file %s", pid_file);
        return -1;
    }
    pid_t pid = (pid_t)pid_val;

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
            remove_pid_file(-1, pid_file);
            return 0;
        } else {
            log_error("Failed to check process status: %s", strerror(errno));
            return -1;
        }
    }
}

// Get status of daemon
int daemon_status(const char *pid_file) {
    char file_path[MAX_PATH_LENGTH];

    if (pid_file) {
        safe_strcpy(file_path, pid_file, sizeof(file_path), 0);
    } else {
        safe_strcpy(file_path, pid_file_path, sizeof(file_path), 0);
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
        log_warn("Found stale PID file %s (not locked), removing it", file_path);
        remove_pid_file(fd, file_path);
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
    char *end_ptr2;
    long pid_val2 = strtol(pid_str, &end_ptr2, 10);
    if (end_ptr2 == pid_str || pid_val2 <= 0) {
        log_error("Failed to parse PID from file %s", file_path);
        return -1;
    }
    pid_t pid = (pid_t)pid_val2;

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
            remove_pid_file(-1, file_path);
            return 0;
        } else {
            log_error("Failed to check process status: %s", strerror(errno));
            return -1;
        }
    }
}
