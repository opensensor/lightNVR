/*
 * SPDX-License-Identifier: GPL-3.0-or-later
 * Copyright (C) 2025-2026 OpenSensor Engineering and LightNVR contributors
 *
 * This file is part of LightNVR <https://github.com/opensensor/lightNVR>.
 */

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
#include <stdatomic.h>

#include "web/web_server.h"
#include "core/logger.h"
#include "core/daemon.h"
#include "core/path_utils.h"
#include "core/shutdown_coordinator.h"
#include "utils/strings.h"

extern _Atomic bool running; // Reference to the global variable defined in main.c

// Global variable to store PID file path
static char pid_file_path[MAX_PATH_LENGTH] = "/run/lightnvr.pid";

// Forward declarations
static void daemon_signal_handler(int sig);
static int check_running_daemon(const char *pid_file);

// Report child startup result to the waiting parent and close the pipe.
static void notify_startup_status(int fd, unsigned char status) {
    ssize_t n;
    do {
        n = write(fd, &status, 1);
    } while (n < 0 && errno == EINTR);
    if (n != 1) {
        log_warn("Failed to report daemon startup status to parent: %s",
                 n < 0 ? strerror(errno) : "short write");
    }
    close(fd);
}

// Initialize daemon - simplified version that works on all platforms including Linux 4.4
int init_daemon(const char *pid_file) {
    // Store PID file path
    if (pid_file) {
        safe_strcpy(pid_file_path, pid_file, sizeof(pid_file_path), 0);
    }

    // Check if daemon is already running
    int daemon_check = check_running_daemon(pid_file_path);
    if (daemon_check > 0) {
        log_error("Daemon is already running");
        return -1;
    } else if (daemon_check < 0) {
        log_error("Failed to verify daemon state from PID file: %s", pid_file_path);
        return -1;
    }

    log_info("Starting daemon mode with PID file: %s", pid_file_path);
    char *startup_cwd = getcwd(NULL, 0);
    if (startup_cwd != NULL) {
        log_info("Current working directory: %s", startup_cwd);
        free(startup_cwd);
    } else {
        log_warn("Current working directory unavailable: %s", strerror(errno));
    }

    // Create startup status pipe for parent/child handshake
    int status_pipe[2];
    if (pipe(status_pipe) != 0) {
        log_error("Failed to create startup status pipe: %s", strerror(errno));
        return -1;
    }

    // Fork the process
    pid_t pid = fork();
    if (pid < 0) {
        log_error("Failed to fork daemon process: %s", strerror(errno));
        close(status_pipe[0]);
        close(status_pipe[1]);
        return -1;
    }

    // Parent process waits for child initialization result
    if (pid > 0) {
        close(status_pipe[1]);
        log_info("Parent process waiting for child startup, child PID: %d", pid);

        unsigned char child_status = 0;
        ssize_t n;
        do {
            n = read(status_pipe[0], &child_status, 1);
        } while (n < 0 && errno == EINTR);
        close(status_pipe[0]);

        if (n == 1 && child_status == 1) {
            log_info("Child daemon initialization succeeded, child PID: %d", pid);
            exit(EXIT_SUCCESS);
        }

        log_error("Child daemon initialization failed, child PID: %d", pid);
        exit(EXIT_FAILURE);
    }

    // Child process continues
    close(status_pipe[0]);
    // Redirection below may replace descriptors 0-2. Keep the status pipe
    // above that range even when the launcher started with stdio closed.
    if (status_pipe[1] <= STDERR_FILENO) {
        int status_fd = fcntl(status_pipe[1], F_DUPFD_CLOEXEC, STDERR_FILENO + 1);
        if (status_fd < 0) {
            notify_startup_status(status_pipe[1], 0);
            return -1;
        }
        close(status_pipe[1]);
        status_pipe[1] = status_fd;
    }
    log_info("Child process starting daemon initialization, PID: %d", getpid());

    // Create a new session
    pid_t sid = setsid();
    if (sid < 0) {
        log_error("Failed to create session: %s", strerror(errno));
        notify_startup_status(status_pipe[1], 0);
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

    // Preserve non-stdio descriptors (for web server compatibility), but
    // redirect standard streams to avoid leaking launcher pipes.
    int null_fd = open("/dev/null", O_RDWR);
    if (null_fd < 0) {
        log_error("Failed to open /dev/null for stdio redirection: %s", strerror(errno));
        notify_startup_status(status_pipe[1], 0);
        return -1;
    }

    if (dup2(null_fd, STDIN_FILENO) < 0 ||
        dup2(null_fd, STDOUT_FILENO) < 0 ||
        dup2(null_fd, STDERR_FILENO) < 0) {
        log_error("Failed to redirect standard file descriptors to /dev/null: %s", strerror(errno));
        if (null_fd > STDERR_FILENO) {
            close(null_fd);
        }
        notify_startup_status(status_pipe[1], 0);
        return -1;
    }

    if (null_fd > STDERR_FILENO) {
        close(null_fd);
    }
    log_info("Redirected standard file descriptors to /dev/null; preserved other descriptors");

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

    // Ignore SIGPIPE process-wide to prevent termination on closed socket writes
    struct sigaction sa_pipe;
    memset(&sa_pipe, 0, sizeof(sa_pipe));
    sa_pipe.sa_handler = SIG_IGN;
    sigemptyset(&sa_pipe.sa_mask);
    if (sigaction(SIGPIPE, &sa_pipe, NULL) != 0) {
        log_warn("Failed to ignore SIGPIPE: %s", strerror(errno));
    } else {
        log_debug("SIGPIPE ignored process-wide");
    }

    // Write PID file
    log_info("Writing PID file...");
    int fd = write_pid_file(pid_file_path);
    if (fd < 0) {
        log_error("Failed to write PID file, daemon initialization failed");
        notify_startup_status(status_pipe[1], 0);
        return -1;
    }

    log_info("Daemon started successfully with PID %d", getpid());

    // Notify parent that daemon initialization has completed successfully
    notify_startup_status(status_pipe[1], 1);

    // Add a small delay to ensure everything is properly initialized
    usleep(100000); // 100ms

    return fd;
}

// Signal handler for daemon
// IMPORTANT: This handler must be async-signal-safe - no mutex operations!
// Only use: volatile sig_atomic_t and lock-free
// C11 atomics (main.c asserts ATOMIC_BOOL_LOCK_FREE == 2)
static void daemon_signal_handler(int sig) {
    switch (sig) {
    case SIGTERM:
    case SIGINT:
        // Set global flag to stop main loop
        // Safe: running is a lock-free _Atomic bool (asserted in main.c)
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

        // main.c's dedicated shutdown watchdog owns the deadline. Do not
        // introduce a process-global alarm shared with unrelated threads.
        break;

    case SIGHUP:
        // No reload work is safe in this signal handler.
        break;

    case SIGALRM:
        // Compatibility for externally delivered SIGALRM; HLS cleanup no
        // longer arms alarms or changes the process signal dispositions.
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
    
    // Open PID file securely: reject symlinks and avoid fd inheritance.
    int fd = open(pid_file, O_RDWR | O_CREAT | O_NOFOLLOW | O_CLOEXEC, 0644);
    if (fd < 0) {
        log_error("Failed to open PID file %s: %s", pid_file, strerror(errno));
        return -1;
    }

    // Validate that we opened a regular file (not device/FIFO/etc.) and not a hardlink.
    struct stat st;
    if (fstat(fd, &st) != 0) {
        log_error("Failed to stat PID file %s: %s", pid_file, strerror(errno));
        close(fd);
        return -1;
    }
    if (!S_ISREG(st.st_mode) || st.st_nlink != 1) {
        log_error("Unsafe PID file %s: expected regular file with a single link", pid_file);
        close(fd);
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
        log_error("Failed to truncate PID file %s: %s", pid_file, strerror(errno));
        close(fd);
        return -1;
    }
    
    // Write PID to file
    char pid_str[16];
    sprintf(pid_str, "%d\n", getpid());
    if (write(fd, pid_str, strlen(pid_str)) != strlen(pid_str)) {
        log_error("Failed to write to PID file %s: %s", pid_file, strerror(errno));
        close(fd);
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
    // Do not unlink the PID file path: removing it can cause split-inode races
    // with concurrent starters. Keep the inode in place and only release the lock.
    if (fd >= 0) {
        if (close(fd) != 0) {
            log_error("Failed to close PID file %s: %s", pid_file, strerror(errno));
            return -1;
        }
    }

    log_info("Released PID file lock for %s (file left in place)", pid_file);
    return 0;
}

// Check if daemon is already running
static int check_running_daemon(const char *pid_file) {
    // First try to open the file with exclusive locking
    int fd = open(pid_file, O_RDWR | O_NOFOLLOW | O_CLOEXEC | O_NONBLOCK);
    if (fd < 0) {
        if (errno == ENOENT) {
            // PID file doesn't exist, daemon is not running
            return 0;
        }

        log_error("Failed to open PID file %s: %s", pid_file, strerror(errno));
        return -1;
    }

    struct stat st;
    if (fstat(fd, &st) != 0) {
        log_error("Failed to stat PID file %s: %s", pid_file, strerror(errno));
        close(fd);
        return -1;
    }
    if (!S_ISREG(st.st_mode) || st.st_nlink != 1) {
        log_error("PID file %s is not a safe regular single-link file", pid_file);
        close(fd);
        return -1;
    }
    
    // Try to lock the file
    for (;;) {
        if (lockf(fd, F_TLOCK, 0) == 0) {
            // We got the lock, which means no other process has it
            // Keep the inode in place; write_pid_file will re-open, lock and
            // overwrite it. Unlinking here creates a second-lock race.
            log_warn("Found stale PID file %s (not locked)", pid_file);
            close(fd);
            return 0;
        }

        if (errno == EINTR) {
            continue; // Retry interrupted lock attempt
        }

        if (errno == EACCES || errno == EAGAIN) {
            // File is locked by another process
            break;
        }

        log_error("Failed to lock PID file %s: %s", pid_file, strerror(errno));
        close(fd);
        return -1;
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
            // Process in PID file is not running, but lock is held by another process.
            // Do not unlink: removing a lock-held PID path can allow multiple daemon instances.
            log_error("PID file %s is lock-held but process %d is not running; refusing to remove due to ambiguous lock ownership", pid_file, pid);
            return -1;
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

    // Open read-only so status checks can work across users (e.g., 0644 PID files)
    int fd = open(file_path, O_RDONLY | O_NOFOLLOW | O_CLOEXEC | O_NONBLOCK);
    if (fd < 0) {
        if (errno == ENOENT) {
            // PID file doesn't exist, daemon is not running
            return 0;
        }
        
        log_error("Failed to open PID file %s: %s", file_path, strerror(errno));
        return -1;
    }

    struct stat st;
    if (fstat(fd, &st) < 0) {
        log_error("Failed to stat PID file %s: %s", file_path, strerror(errno));
        close(fd);
        return -1;
    }

    if (!S_ISREG(st.st_mode) || st.st_nlink != 1) {
        log_error("PID file %s is not a regular single-link file", file_path);
        close(fd);
        return -1;
    }
    
    // Read PID from file and check process liveness directly.
    // Avoid F_GETLK here: it does not report locks owned by this process,
    // which can misreport "stopped" during in-process status checks.
    char pid_str[16];
    ssize_t bytes_read;
    do {
        bytes_read = read(fd, pid_str, sizeof(pid_str) - 1);
    } while (bytes_read < 0 && errno == EINTR);
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
    } else if (errno == EPERM) {
        // Process exists, but we don't have permission to signal it.
        // For status checks, this still means the daemon is running.
        log_info("Daemon is running with PID %d (no permission to signal)", pid);
        return 1;
    } else {
        if (errno == ESRCH) {
            // Process is not running; leave PID file handling to startup/cleanup paths.
            log_warn("PID file %s contains PID %d but process is not running", file_path, pid);
            return 0;
        } else {
            log_error("Failed to check process status: %s", strerror(errno));
            return -1;
        }
    }
}
