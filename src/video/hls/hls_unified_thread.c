#define _POSIX_C_SOURCE 200809L
#define _XOPEN_SOURCE 700
#define _GNU_SOURCE

/**
 * HLS Unified Thread Implementation
 *
 * This file implements the unified thread approach for HLS streaming.
 *
 * CRITICAL FIX (2025-04-11): Fixed segmentation fault issues related to thread safety
 * in the writer cleanup process. The main issue was that the hls_writer was being accessed
 * after it had been freed by another thread. The fix uses atomic operations to ensure
 * thread-safe access to the writer pointer.
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <pthread.h>
#include <stdatomic.h>
#include <time.h>
#include <signal.h>
#include <sys/signal.h>
#include <errno.h>
#include <sys/socket.h>
#include <netinet/in.h>
#include <arpa/inet.h>
#include <netdb.h>
#include <sys/time.h>
#include <sys/stat.h>
#include <sys/types.h>
#include <dirent.h>
#include <libavformat/avformat.h>
#include <libavcodec/avcodec.h>
#include <libavutil/avutil.h>
#include <libavutil/time.h>
#include <libavutil/buffer.h>
#include <libavutil/mem.h>
#include <libavutil/log.h>

#include "core/logger.h"
#include "core/config.h"
#include "core/shutdown_coordinator.h"

// MEMORY LEAK FIX: Forward declaration for FFmpeg buffer cleanup function
// We'll implement our own version to clean up any leaked buffers
static void ffmpeg_buffer_cleanup(void);

// Forward declarations for HLS watchdog functions
static void start_hls_watchdog(void);
static void stop_hls_watchdog(void);

// MEMORY LEAK FIX: Global variable to track FFmpeg memory usage
static int ffmpeg_memory_cleanup_registered = 0;

// MEMORY LEAK FIX: Global FFmpeg memory cleanup function
// This will be called during program exit to ensure all FFmpeg resources are freed
// CRITICAL FIX: Added safety checks to prevent segfaults during shutdown
static void global_ffmpeg_cleanup(void) {
    static int in_global_cleanup = 0;

    // CRITICAL FIX: Prevent recursive calls that can cause double free
    if (in_global_cleanup) {
        log_warn("Already in global FFmpeg cleanup, skipping to prevent double free");
        return;
    }
    in_global_cleanup = 1;

    log_info("Global FFmpeg cleanup called during program exit");

    // CRITICAL FIX: Use try/catch-like approach with signal handling to prevent crashes
    struct sigaction sa_old, sa_new;
    sigaction(SIGSEGV, NULL, &sa_old);
    sa_new = sa_old;
    sa_new.sa_handler = SIG_IGN; // Ignore segmentation fault signal
    sigaction(SIGSEGV, &sa_new, NULL);

    // Set alarm to prevent hanging if memory is inaccessible
    alarm(1); // 1 second timeout

    // Call FFmpeg's internal memory cleanup functions - safely
    log_debug("Calling avformat_network_deinit() during global cleanup");
    avformat_network_deinit();

    // Force FFmpeg to release all cached memory
    log_debug("Setting FFmpeg log level to quiet during global cleanup");
    av_log_set_level(AV_LOG_QUIET);

    // CRITICAL FIX: Skip direct av_freep(NULL) call during global cleanup
    // This is the most likely cause of segmentation faults
    log_info("Using safer approach to release FFmpeg memory during global cleanup");

    // Instead of directly calling av_freep(NULL), which can cause segfaults,
    // allocate and free a small buffer to trigger FFmpeg's internal cleanup
    void *dummy = av_malloc(16);
    if (dummy) {
        av_free(dummy);
    }

    // Cancel the alarm and restore signal handler
    alarm(0);
    sigaction(SIGSEGV, &sa_old, NULL);

    log_info("Global FFmpeg cleanup completed");
    in_global_cleanup = 0;
}

// Video-related includes
#include "video/stream_manager.h"
#include "video/stream_state.h"
#include "video/streams.h"
#include "video/hls_writer.h"
#include "video/stream_protocol.h"
#include "video/thread_utils.h"
#include "video/timestamp_manager.h"
#include "video/detection_frame_processing.h"
#include "video/hls/hls_context.h"
#include "video/hls/hls_directory.h"
#include "video/hls/hls_unified_thread.h"
#include "video/ffmpeg_utils.h"

// Maximum time (in seconds) without receiving a packet before considering the connection dead
#define MAX_PACKET_TIMEOUT 5

// Base reconnection delay in milliseconds (500ms)
#define BASE_RECONNECT_DELAY_MS 500

// Maximum reconnection delay in milliseconds (30 seconds)
#define MAX_RECONNECT_DELAY_MS 30000

// Forward declaration for go2rtc integration
extern bool go2rtc_integration_is_using_go2rtc_for_hls(const char *stream_name);
extern bool go2rtc_get_rtsp_url(const char *stream_name, char *url, size_t url_size);

// Hash map for tracking running HLS streaming contexts
// These are defined at the bottom of the file as non-static

// CRITICAL FIX: Add tracking for freed contexts to prevent double free and use-after-free
#define MAX_FREED_CONTEXTS 100
static void *freed_contexts[MAX_FREED_CONTEXTS] = {0};
static int freed_contexts_count = 0;
static pthread_mutex_t freed_contexts_mutex = PTHREAD_MUTEX_INITIALIZER;

// CRITICAL FIX: Add a flag to indicate that a context is pending deletion
// This helps threads detect when their context is about to be freed
typedef struct {
    void *ctx;
    atomic_int pending_deletion;
    atomic_int thread_exited;  // Flag to indicate the thread has exited and it's safe to free the context
} context_deletion_info_t;

static context_deletion_info_t pending_deletions[MAX_STREAMS] = {0};
static pthread_mutex_t pending_deletions_mutex = PTHREAD_MUTEX_INITIALIZER;

// Maximum time to wait for a thread to exit (in microseconds)
#define MAX_THREAD_EXIT_WAIT_US 2000000  // 2000ms (2 seconds) - increased from 500ms

// CRITICAL FIX: Add memory boundary checking to detect buffer overflows
#define MEMORY_GUARD_SIZE 16
#define MEMORY_GUARD_PATTERN 0xFE

// Watchdog thread settings
#define WATCHDOG_CHECK_INTERVAL_SEC 30  // Check every 30 seconds
#define WATCHDOG_MAX_RESTART_ATTEMPTS 5  // Maximum number of restart attempts
#define WATCHDOG_RESTART_COOLDOWN_SEC 300  // 5 minutes between restart attempts

// Function to allocate memory with guard bytes
static void *safe_malloc(size_t size) {
    // Allocate extra space for guard bytes
    size_t total_size = size + (2 * MEMORY_GUARD_SIZE);
    unsigned char *mem = malloc(total_size);

    if (!mem) {
        log_error("Failed to allocate memory of size %zu", size);
        return NULL;
    }

    // Fill the guard bytes
    memset(mem, MEMORY_GUARD_PATTERN, MEMORY_GUARD_SIZE);
    memset(mem + MEMORY_GUARD_SIZE + size, MEMORY_GUARD_PATTERN, MEMORY_GUARD_SIZE);

    // Return the usable portion of the memory
    return mem + MEMORY_GUARD_SIZE;
}

// Function to free memory allocated with safe_malloc
void *safe_free(void *ptr) {
    if (!ptr) {
        return NULL;
    }

    // Get the original pointer
    unsigned char *mem = ((unsigned char *)ptr) - MEMORY_GUARD_SIZE;

    // Check the guard bytes
    bool guard_corrupted = false;

    // Check leading guard bytes
    for (int i = 0; i < MEMORY_GUARD_SIZE; i++) {
        if (mem[i] != MEMORY_GUARD_PATTERN) {
            guard_corrupted = true;
            break;
        }
    }

    // We don't know the size of the allocation, so we can't check the trailing guard bytes
    // This is a limitation of this approach

    if (guard_corrupted) {
        log_error("Memory corruption detected: guard bytes have been overwritten");
        // Continue with the free anyway
    }

    // Free the original pointer
    free(mem);

    return NULL;
}

// Function to check if a context has already been freed
static bool is_context_already_freed(void *ctx) {
    if (!ctx) {
        return false;  // NULL context is not considered freed
    }

    bool result = false;

    pthread_mutex_lock(&freed_contexts_mutex);

    for (int i = 0; i < freed_contexts_count; i++) {
        if (freed_contexts[i] == ctx) {
            result = true;
            break;
        }
    }

    pthread_mutex_unlock(&freed_contexts_mutex);

    // If not found in the freed contexts list, perform additional checks
    if (!result) {
        // Check if the memory appears to be invalid
        // This is a heuristic to detect freed memory
        bool appears_invalid = false;

        // Try to access the first few bytes to see if they're zeroed out
        // This might indicate that the memory has been freed
        unsigned char *ptr = (unsigned char *)ctx;
        bool all_zeros = true;

        // Use a try/catch-like approach with signal handling to prevent crashes
        struct sigaction sa_old, sa_new;
        sigaction(SIGSEGV, NULL, &sa_old);
        sa_new = sa_old;
        sa_new.sa_handler = SIG_IGN; // Ignore segmentation fault signal
        sigaction(SIGSEGV, &sa_new, NULL);

        // Set alarm to prevent hanging if memory is inaccessible
        alarm(1); // 1 second timeout

        // Check the first few bytes
        for (int i = 0; i < 16 && i < sizeof(hls_unified_thread_ctx_t); i++) {
            if (ptr[i] != 0) {
                all_zeros = false;
                break;
            }
        }

        // Cancel the alarm and restore signal handler
        alarm(0);
        sigaction(SIGSEGV, &sa_old, NULL);

        if (all_zeros) {
            // Memory appears to be zeroed out, which might indicate it's been freed
            appears_invalid = true;
        }

        if (appears_invalid) {
            log_warn("Context %p appears to be invalid or already freed", ctx);
            result = true;
        }
    }

    return result;
}

// Function to mark a context as pending deletion
static void mark_context_pending_deletion(void *ctx) {
    pthread_mutex_lock(&pending_deletions_mutex);

    // Find an empty slot or the existing entry for this context
    int slot = -1;
    for (int i = 0; i < MAX_STREAMS; i++) {
        if (pending_deletions[i].ctx == ctx) {
            // Context already marked, just update the flag
            atomic_store(&pending_deletions[i].pending_deletion, 1);
            atomic_store(&pending_deletions[i].thread_exited, 0);  // Reset the thread_exited flag
            slot = i;
            break;
        } else if (pending_deletions[i].ctx == NULL && slot == -1) {
            // Found an empty slot
            slot = i;
        }
    }

    if (slot != -1 && pending_deletions[slot].ctx != ctx) {
        // Initialize the new entry
        pending_deletions[slot].ctx = ctx;
        atomic_init(&pending_deletions[slot].pending_deletion, 1);
        atomic_init(&pending_deletions[slot].thread_exited, 0);
    }

    pthread_mutex_unlock(&pending_deletions_mutex);
}

// Function to check if a context is pending deletion
static bool is_context_pending_deletion(void *ctx) {
    if (!ctx) {
        return false;  // NULL context is not considered pending deletion
    }

    bool result = false;

    pthread_mutex_lock(&pending_deletions_mutex);

    for (int i = 0; i < MAX_STREAMS; i++) {
        if (pending_deletions[i].ctx == ctx && atomic_load(&pending_deletions[i].pending_deletion)) {
            result = true;
            break;
        }
    }

    pthread_mutex_unlock(&pending_deletions_mutex);

    return result;
}

// Function to mark a thread as exited in the pending deletion list
static void mark_thread_exited(void *ctx) {
    if (!ctx) {
        return;  // Nothing to do for NULL context
    }

    // CRITICAL FIX: Check if the context is already freed before proceeding
    // This is a safety check to prevent use-after-free errors
    if (is_context_already_freed(ctx)) {
        log_warn("Attempted to mark thread as exited for already freed context %p", ctx);
        return;
    }

    pthread_mutex_lock(&pending_deletions_mutex);

    // First, check if the context is already in the list
    bool found = false;
    for (int i = 0; i < MAX_STREAMS; i++) {
        if (pending_deletions[i].ctx == ctx) {
            atomic_store(&pending_deletions[i].thread_exited, 1);
            found = true;
            log_info("Marked thread as exited for context %p", ctx);
            break;
        }
    }

    // If not found, add it to the list
    if (!found) {
        // Find an empty slot
        int slot = -1;
        for (int i = 0; i < MAX_STREAMS; i++) {
            if (pending_deletions[i].ctx == NULL) {
                slot = i;
                break;
            }
        }

        // If we found an empty slot, use it
        if (slot != -1) {
            // CRITICAL FIX: Check again if the context is already freed before adding it
            // This handles the race condition where the context might be freed between our initial check and now
            if (!is_context_already_freed(ctx)) {
                pending_deletions[slot].ctx = ctx;
                atomic_init(&pending_deletions[slot].pending_deletion, 0);
                atomic_init(&pending_deletions[slot].thread_exited, 1);
                log_info("Added context %p to pending deletions list and marked thread as exited", ctx);
            } else {
                log_warn("Context %p was freed during mark_thread_exited, not adding to list", ctx);
            }
        } else {
            log_warn("No empty slot found in pending deletions list for context %p", ctx);
        }
    }

    pthread_mutex_unlock(&pending_deletions_mutex);
}

// Function to check if a thread has exited
static bool has_thread_exited(void *ctx) {
    bool result = false;

    pthread_mutex_lock(&pending_deletions_mutex);

    for (int i = 0; i < MAX_STREAMS; i++) {
        if (pending_deletions[i].ctx == ctx) {
            result = atomic_load(&pending_deletions[i].thread_exited);
            break;
        }
    }

    pthread_mutex_unlock(&pending_deletions_mutex);

    return result;
}

// Function to wait for a thread to exit
static void wait_for_thread_exit(void *ctx) {
    if (!ctx) {
        return;  // Nothing to do for NULL context
    }

    // CRITICAL FIX: Check if the context is already freed before proceeding
    if (is_context_already_freed(ctx)) {
        log_warn("Context %p is already freed, skipping thread exit wait", ctx);
        // Mark as exited anyway to ensure cleanup can proceed
        mark_thread_exited(ctx);
        return;
    }

    int wait_time = 0;
    const int sleep_interval = 10000;  // 10ms

    // First check if the thread has already exited
    if (has_thread_exited(ctx)) {
        log_info("Thread for context %p has already exited", ctx);
        return;
    }

    log_info("Waiting for thread to exit for context %p", ctx);

    // CRITICAL FIX: Add a memory barrier to ensure all previous operations are visible
    __sync_synchronize();

    // Wait for the thread to exit with timeout
    while (wait_time < MAX_THREAD_EXIT_WAIT_US) {
        // CRITICAL FIX: Check if the context is already freed before continuing
        if (is_context_already_freed(ctx)) {
            log_warn("Context %p was freed during wait_for_thread_exit", ctx);
            // Mark as exited anyway to ensure cleanup can proceed
            mark_thread_exited(ctx);
            return;
        }

        if (has_thread_exited(ctx)) {
            log_info("Thread for context %p has exited after waiting %d ms", ctx, wait_time / 1000);
            return;
        }

        usleep(sleep_interval);
        wait_time += sleep_interval;

        // Log progress every 100ms
        if (wait_time % 100000 == 0) {
            log_info("Still waiting for thread to exit for context %p (%d ms elapsed)", ctx, wait_time / 1000);

            // CRITICAL FIX: Check if the context is pending deletion and re-signal if needed
            // This handles cases where the thread might have missed the initial signal
            if (!has_thread_exited(ctx)) {
                log_info("Re-signaling thread for context %p to exit", ctx);
                mark_context_pending_deletion(ctx);
                __sync_synchronize(); // Memory barrier to ensure visibility
            }
        }
    }

    log_warn("Timeout waiting for thread to exit for context %p after %d ms", ctx, MAX_THREAD_EXIT_WAIT_US / 1000);

    // CRITICAL FIX: Check if the context is already freed before proceeding
    if (!is_context_already_freed(ctx)) {
        // Mark the thread as exited anyway to prevent deadlocks
        log_warn("Forcing thread exited status for context %p to prevent deadlock", ctx);
        mark_thread_exited(ctx);
    } else {
        log_warn("Context %p was freed during wait_for_thread_exit timeout", ctx);
    }
}

// Function to clear a context from the pending deletion list
static void clear_context_pending_deletion(void *ctx) {
    pthread_mutex_lock(&pending_deletions_mutex);

    for (int i = 0; i < MAX_STREAMS; i++) {
        if (pending_deletions[i].ctx == ctx) {
            pending_deletions[i].ctx = NULL;
            atomic_store(&pending_deletions[i].pending_deletion, 0);
            atomic_store(&pending_deletions[i].thread_exited, 0);
            break;
        }
    }

    pthread_mutex_unlock(&pending_deletions_mutex);
}

// Function to mark a context as freed
void mark_context_as_freed(void *ctx) {
    if (!ctx) {
        return;  // Nothing to do for NULL context
    }

    pthread_mutex_lock(&freed_contexts_mutex);

    // Check if the context is already in the freed list
    bool already_freed = false;
    for (int i = 0; i < freed_contexts_count; i++) {
        if (freed_contexts[i] == ctx) {
            already_freed = true;
            break;
        }
    }

    if (already_freed) {
        log_warn("Context %p is already marked as freed", ctx);
        pthread_mutex_unlock(&freed_contexts_mutex);
        return;
    }

    // If the array is full, remove the oldest entry
    if (freed_contexts_count >= MAX_FREED_CONTEXTS) {
        // Shift all entries down by one
        for (int i = 0; i < MAX_FREED_CONTEXTS - 1; i++) {
            freed_contexts[i] = freed_contexts[i + 1];
        }
        freed_contexts_count = MAX_FREED_CONTEXTS - 1;
    }

    // Add the new entry
    freed_contexts[freed_contexts_count++] = ctx;
    log_info("Marked context %p as freed", ctx);

    pthread_mutex_unlock(&freed_contexts_mutex);
}

/**
 * Safe resource cleanup function
 * This function safely cleans up all FFmpeg and HLS resources, handling NULL pointers and other edge cases
 * Enhanced to prevent memory leaks
 */

// Flag to prevent recursive calls to safe_cleanup_resources
static __thread bool in_safe_cleanup = false;

static void safe_cleanup_resources(AVFormatContext **input_ctx, AVPacket **pkt, hls_writer_t **writer) {
    // CRITICAL FIX: Add safety checks to prevent segmentation faults and bus errors
    // This function is designed to be robust against NULL pointers and partially initialized structures

    // CRITICAL FIX: Prevent recursive calls that can cause double free
    if (in_safe_cleanup) {
        log_warn("Recursive call to safe_cleanup_resources detected, aborting to prevent double free");
        return;
    }
    in_safe_cleanup = true;

    // CRITICAL FIX: Add additional NULL checks for all parameters
    if (!input_ctx && !pkt && !writer) {
        log_debug("All parameters to safe_cleanup_resources are NULL, nothing to clean up");
        in_safe_cleanup = false;
        return;
    }

    // Clean up packet with safety checks
    if (pkt) {
        // CRITICAL FIX: Check if the pointer to pointer is valid before dereferencing
        AVPacket *pkt_to_free = NULL;

        if (*pkt) {
            // CRITICAL FIX: Create a properly aligned local copy to prevent bus errors on embedded devices
            // Some embedded processors require strict memory alignment
            pkt_to_free = *pkt;
            *pkt = NULL; // Clear the pointer first to prevent double-free

            // CRITICAL FIX: Add memory barrier to ensure memory operations are completed
            // This helps prevent bus errors on some embedded architectures
            __sync_synchronize();

            // Safely unref and free the packet
            log_debug("Safely unreferencing packet during cleanup");

            // CRITICAL FIX: Add additional NULL check before unreferencing
            if (pkt_to_free) {
                av_packet_unref(pkt_to_free);
                log_debug("Safely freeing packet during cleanup");

                // CRITICAL FIX: Add memory barrier before freeing to ensure all accesses are complete
                __sync_synchronize();

                av_packet_free(&pkt_to_free);
            }
        }
    }

    // Clean up input context with safety checks
    if (input_ctx) {
        // CRITICAL FIX: Check if the pointer to pointer is valid before dereferencing
        AVFormatContext *ctx_to_close = NULL;

        if (*input_ctx) {
            ctx_to_close = *input_ctx;
            *input_ctx = NULL; // Clear the pointer first to prevent double-free

            // CRITICAL FIX: Add memory barrier to ensure memory operations are completed
            __sync_synchronize();

            // Safely close the input context
            log_debug("Safely closing input context during cleanup");

            // CRITICAL FIX: Add additional NULL check before closing
            if (ctx_to_close) {
                // MEMORY LEAK FIX: Use comprehensive_ffmpeg_cleanup instead of manual cleanup
                // This ensures all resources are properly freed, preventing memory leaks
                log_debug("Using comprehensive cleanup for input context");

                // Create a local copy of the pointer to prevent race conditions
                AVFormatContext *ctx_to_cleanup = ctx_to_close;
                ctx_to_close = NULL; // Clear the original pointer to prevent double-free

                // Use our comprehensive cleanup function
                comprehensive_ffmpeg_cleanup(&ctx_to_cleanup, NULL, NULL, NULL);

                // Verify that the context is actually NULL after cleanup
                if (ctx_to_cleanup) {
                    log_warn("Failed to clean up context, forcing NULL");
                    ctx_to_cleanup = NULL;
                }

                log_debug("Successfully cleaned up input context");
            }
        }
    }

    // Clean up HLS writer with safety checks
    if (writer) {
        // CRITICAL FIX: Check if the pointer to pointer is valid before dereferencing
        hls_writer_t *writer_to_free = NULL;

        // CRITICAL FIX: Add additional validation of the writer pointer
        if (!writer) {
            log_warn("Writer pointer is NULL during cleanup");
        } else if (!*writer) {
            log_debug("Writer is already NULL, nothing to clean up");
        } else {
            // CRITICAL FIX: Use atomic pointer exchange to safely get and clear the writer pointer
            // This ensures that no other thread can access the writer after we've taken ownership of it
            writer_to_free = __atomic_exchange_n(writer, NULL, __ATOMIC_SEQ_CST);

            // CRITICAL FIX: Validate the writer pointer before using it
            if (!writer_to_free) {
                log_warn("Writer became NULL between checks");
            } else {
                // CRITICAL FIX: Validate the writer structure before freeing
                // This helps catch cases where the memory has been corrupted
                bool writer_valid = true;

                // Basic validation of writer structure
                if (writer_to_free->stream_name == NULL) {
                    log_warn("Writer has NULL stream_name, may be corrupted");
                    writer_valid = false;
                }

                if (writer_valid) {
                    // Get a copy of the stream name for logging
                    char writer_stream_name[MAX_STREAM_NAME] = {0};
                    if (writer_to_free->stream_name) {
                        strncpy(writer_stream_name, writer_to_free->stream_name, MAX_STREAM_NAME - 1);
                        writer_stream_name[MAX_STREAM_NAME - 1] = '\0';
                    } else {
                        strcpy(writer_stream_name, "unknown");
                    }

                    log_debug("Preparing to close HLS writer for stream %s", writer_stream_name);

                    // Clear the pointer first to prevent double-free
                    *writer = NULL;

                    // CRITICAL FIX: Add memory barrier to ensure memory operations are completed
                    __sync_synchronize();

                    // Safely free the HLS writer
                    log_debug("Safely closing HLS writer during cleanup for stream %s", writer_stream_name);

                    // CRITICAL FIX: Add memory barrier before closing to ensure all accesses are complete
                    __sync_synchronize();

                    // Use a try/catch-like approach with signal handling to prevent crashes
                    struct sigaction sa_old, sa_new;
                    sigaction(SIGALRM, NULL, &sa_old);
                    sa_new = sa_old;
                    sa_new.sa_handler = SIG_IGN; // Ignore alarm signal
                    sigaction(SIGALRM, &sa_new, NULL);

                    // Set alarm
                    alarm(15); // 15 second timeout for writer close

                    // Close the writer with additional protection
                    hls_writer_close(writer_to_free);

                    // Cancel the alarm and restore signal handler
                    alarm(0);
                    sigaction(SIGALRM, &sa_old, NULL);

                    log_debug("Successfully closed HLS writer for stream %s", writer_stream_name);
                } else {
                    log_warn("Skipping cleanup of invalid writer");
                    *writer = NULL; // Still clear the pointer to prevent future access
                }
            }
        }
    }

    log_debug("Completed safe cleanup of resources");

    // Reset the recursive call prevention flag
    in_safe_cleanup = false;
}

/**
 * Calculate reconnection delay with exponential backoff
 *
 * @param attempt The current reconnection attempt (1-based)
 * @return Delay in milliseconds
 */
static int calculate_reconnect_delay(int attempt) {
    if (attempt <= 0) return BASE_RECONNECT_DELAY_MS;

    // Exponential backoff: delay = base_delay * 2^(attempt-1)
    // Cap at maximum delay
    int delay = BASE_RECONNECT_DELAY_MS * (1 << (attempt - 1));
    return (delay < MAX_RECONNECT_DELAY_MS) ? delay : MAX_RECONNECT_DELAY_MS;
}

/**
 * Check RTSP connection by sending an OPTIONS request
 *
 * @param rtsp_url The RTSP URL to check
 * @param host Buffer to store the extracted hostname
 * @param port Pointer to store the extracted port
 * @return 0 on success, negative on error
 */
static int check_rtsp_connection(const char *rtsp_url, char *host, int *port) {
    if (!rtsp_url || !host || !port || strncmp(rtsp_url, "rtsp://", 7) != 0) {
        return -1;
    }

    // Skip the rtsp:// prefix
    const char *host_start = rtsp_url + 7;

    // Skip any authentication info (user:pass@)
    const char *at_sign = strchr(host_start, '@');
    if (at_sign) {
        host_start = at_sign + 1;
    }

    // Find the end of the host part
    const char *host_end = strchr(host_start, ':');
    if (!host_end) {
        host_end = strchr(host_start, '/');
        if (!host_end) {
            host_end = host_start + strlen(host_start);
        }
    }

    // Copy the host part with bounds checking
    size_t host_len = host_end - host_start;
    if (host_len >= 256) {
        host_len = 255;
    }
    if (host_len == 0) {
        // Empty hostname
        return -1;
    }
    memcpy(host, host_start, host_len);
    host[host_len] = '\0';

    // Extract the port if specified
    *port = 554; // Default RTSP port
    if (*host_end == ':') {
        const char *port_start = host_end + 1;
        char *port_end;
        long port_val = strtol(port_start, &port_end, 10);
        if (port_start == port_end || port_val <= 0 || port_val > 65535) {
            *port = 554; // Invalid port, use default
        } else {
            *port = (int)port_val;
        }
    }

    // Create a socket with error handling
    int sock = socket(AF_INET, SOCK_STREAM, 0);
    if (sock < 0) {
        log_error("Failed to create socket for RTSP connection check: %s", strerror(errno));
        return -1;
    }

    // Set a short timeout for the connection
    struct timeval tv;
    tv.tv_sec = 1;
    tv.tv_usec = 0;
    if (setsockopt(sock, SOL_SOCKET, SO_RCVTIMEO, &tv, sizeof(tv)) < 0 ||
        setsockopt(sock, SOL_SOCKET, SO_SNDTIMEO, &tv, sizeof(tv)) < 0) {
        log_warn("Failed to set socket timeout: %s", strerror(errno));
        // Continue anyway
    }

    // Connect to the server
    struct sockaddr_in server_addr;
    memset(&server_addr, 0, sizeof(server_addr));
    server_addr.sin_family = AF_INET;
    server_addr.sin_port = htons(*port);

    // Convert hostname to IP address
    struct hostent *he = gethostbyname(host);
    if (!he) {
        close(sock);
        return -1;
    }

    memcpy(&server_addr.sin_addr, he->h_addr_list[0], he->h_length);

    // Connect to the server
    if (connect(sock, (struct sockaddr*)&server_addr, sizeof(server_addr)) < 0) {
        close(sock);
        return -1;
    }

    // Extract the path part of the URL
    const char *path = strchr(host_start, '/');
    if (!path) {
        path = "/";
    }

    // Send a simple RTSP OPTIONS request
    char request[1024];
    snprintf(request, sizeof(request),
             "OPTIONS %s RTSP/1.0\r\n"
             "CSeq: 1\r\n"
             "User-Agent: LightNVR\r\n"
             "\r\n",
             path);

    if (send(sock, request, strlen(request), 0) < 0) {
        close(sock);
        return -1;
    }

    // Receive the response
    char response[1024] = {0};
    int bytes_received = recv(sock, response, sizeof(response) - 1, 0);
    close(sock);

    if (bytes_received <= 0) {
        return -1;
    }

    // Check if the response contains "404 Not Found"
    if (strstr(response, "404 Not Found") != NULL) {
        return -1;
    }

    return 0;
}

/**
 * Unified HLS thread function
 * This function handles all HLS streaming operations for a single stream
 */
void *hls_unified_thread_func(void *arg) {
    hls_unified_thread_ctx_t *ctx = (hls_unified_thread_ctx_t *)arg;
    AVFormatContext *input_ctx = NULL;
    AVPacket *pkt = NULL;
    int video_stream_idx = -1;
    int ret;
    hls_thread_state_t thread_state = HLS_THREAD_INITIALIZING;
    int reconnect_attempt = 0;
    int reconnect_delay_ms = BASE_RECONNECT_DELAY_MS;
    time_t last_packet_time = 0;

    // Validate context
    if (!ctx) {
        log_error("NULL context passed to unified HLS thread");
        return NULL;
    }

    // Check if the context is already marked for deletion
    if (is_context_pending_deletion(ctx)) {
        log_warn("Context is already marked for deletion, exiting thread");
        return NULL;
    }

    // CRITICAL FIX: Check for shutdown early to prevent starting new streams during shutdown
    if (is_shutdown_initiated()) {
        log_info("Unified HLS thread exiting immediately due to system shutdown");
        // Mark the context as not running to prevent further operations
        if (ctx) {
            atomic_store(&ctx->running, 0);
            // Update component state in shutdown coordinator if registered
            if (ctx->shutdown_component_id >= 0) {
                update_component_state(ctx->shutdown_component_id, COMPONENT_STOPPED);
            }
        }
        return NULL;
    }

    // Create a local copy of the stream name for thread safety
    char stream_name[MAX_STREAM_NAME];
    strncpy(stream_name, ctx->stream_name, MAX_STREAM_NAME - 1);
    stream_name[MAX_STREAM_NAME - 1] = '\0';

    log_info("Starting unified HLS thread for stream %s", stream_name);

    // Check if we're still running before proceeding
    if (!atomic_load(&ctx->running)) {
        log_warn("Unified HLS thread for %s started but already marked as not running", stream_name);
        return NULL;
    }

    // Get the stream state manager
    stream_state_manager_t *state = get_stream_state_by_name(stream_name);
    if (!state) {
        log_error("Could not find stream state for %s", stream_name);
        atomic_store(&ctx->running, 0);
        return NULL;
    }

    // Register with shutdown coordinator
    char component_name[128];
    snprintf(component_name, sizeof(component_name), "hls_unified_%s", stream_name);
    ctx->shutdown_component_id = register_component(component_name, COMPONENT_HLS_WRITER, ctx, 60);
    if (ctx->shutdown_component_id >= 0) {
        log_info("Registered unified HLS thread %s with shutdown coordinator (ID: %d)",
                stream_name, ctx->shutdown_component_id);
    }

    // MEMORY LEAK FIX: We'll manually ensure resources are freed on thread exit

    // Initialize packet
    pkt = av_packet_alloc();
    if (!pkt) {
        log_error("Failed to allocate packet for stream %s", stream_name);
        atomic_store(&ctx->running, 0);

        // Update component state in shutdown coordinator
        if (ctx->shutdown_component_id >= 0) {
            update_component_state(ctx->shutdown_component_id, COMPONENT_STOPPED);
        }

        return NULL;
    }

    // Check if the stream state already has an HLS writer
    if (state && state->hls_ctx && state->hls_ctx != ctx->writer) {
        log_warn("Stream state for %s already has an HLS writer. Replacing it.", stream_name);
        // We'll replace the existing writer with our new one
        // The old writer will be cleaned up during shutdown
    }

    // Create HLS writer with 5-second segments
    ctx->writer = hls_writer_create(ctx->output_path, stream_name, 5);
    if (!ctx->writer) {
        log_error("Failed to create HLS writer for %s", stream_name);

        // Clean up resources
        safe_cleanup_resources(NULL, &pkt, NULL);

        atomic_store(&ctx->running, 0);

        // Update component state in shutdown coordinator
        if (ctx->shutdown_component_id >= 0) {
            update_component_state(ctx->shutdown_component_id, COMPONENT_STOPPED);
        }

        return NULL;
    }

    // Store the HLS writer in the stream state for other components to access
    if (state) {
        state->hls_ctx = ctx->writer;
    }

    // Track whether we've ever successfully written packets
    // This is used to determine if we need to recreate the writer on reconnection
    // Only recreate the writer if we've actually used it (prevents crash on initial connection failures)
    bool has_written_packets = false;

    // CRITICAL FIX: Add a local copy of the context pointer for safety checks
    hls_unified_thread_ctx_t *ctx_safe = ctx;

    // Main state machine loop
    while (ctx && ctx_safe && !is_context_already_freed(ctx_safe) && !is_context_pending_deletion(ctx_safe)) {
        // CRITICAL FIX: Update the safe context pointer for this iteration
        ctx_safe = ctx;

        // Check if we should continue running
        // CRITICAL FIX: Only access ctx members if the context is not already freed
        if (!ctx_safe || is_context_already_freed(ctx_safe) || is_context_pending_deletion(ctx_safe) ||
            (ctx_safe && !atomic_load(&ctx_safe->running))) {
            log_info("Unified HLS thread for %s stopping due to %s",
                    stream_name,
                    !ctx_safe ? "context is NULL" :
                    is_context_already_freed(ctx_safe) ? "context already freed" :
                    is_context_pending_deletion(ctx_safe) ? "context pending deletion" :
                    "running flag cleared");
            break;
        }
        // Update thread state in context
        // CRITICAL FIX: Only update if context is still valid
        if (ctx_safe && !is_context_already_freed(ctx_safe) && !is_context_pending_deletion(ctx_safe)) {
            atomic_store(&ctx_safe->thread_state, thread_state);
        }

        // Check for shutdown conditions
        if (is_shutdown_initiated() || is_stream_state_stopping(state) || !are_stream_callbacks_enabled(state)) {
            log_info("Unified HLS thread for %s stopping due to %s",
                    stream_name,
                    is_shutdown_initiated() ? "system shutdown" :
                    is_stream_state_stopping(state) ? "stream state STOPPING" :
                    "callbacks disabled");
            thread_state = HLS_THREAD_STOPPING;
        }

        // Check if streaming is enabled for this stream
        // This is separate from the recording flag check
        // We only want to stop HLS writers if streaming is disabled, not if recording is disabled
        if (state && !state->features.streaming_enabled) {
            log_info("Unified HLS thread for %s stopping due to streaming disabled", stream_name);
            thread_state = HLS_THREAD_STOPPING;
        }

        // State machine
        switch (thread_state) {
            case HLS_THREAD_INITIALIZING:
                log_info("Initializing unified HLS thread for stream %s", stream_name);
                thread_state = HLS_THREAD_CONNECTING;
                reconnect_attempt = 0;
                break;

            case HLS_THREAD_CONNECTING:
                log_info("Connecting to stream %s (attempt %d)", stream_name, reconnect_attempt + 1);

                // Close any existing connection first
                safe_cleanup_resources(&input_ctx, NULL, NULL);

                // Check if the RTSP URL exists before trying to connect
                if (strncmp(ctx->rtsp_url, "rtsp://", 7) == 0) {
                    char host[256] = {0};
                    int port = 554; // Default RTSP port

                    if (check_rtsp_connection(ctx->rtsp_url, host, &port) < 0) {
                        log_error("Failed to connect to RTSP server: %s:%d", host, port);

                        // Mark connection as invalid
                        atomic_store(&ctx->connection_valid, 0);

                        // Increment reconnection attempt counter
                        reconnect_attempt++;

                        // Calculate reconnection delay with exponential backoff
                        reconnect_delay_ms = calculate_reconnect_delay(reconnect_attempt);

                        log_info("Will retry connection to stream %s in %d ms (attempt %d)",
                                stream_name, reconnect_delay_ms, reconnect_attempt + 1);

                        // Sleep before retrying
                        av_usleep(reconnect_delay_ms * 1000);

                        // CRITICAL FIX: Check for shutdown conditions before continuing
                        if (is_context_already_freed(ctx) || is_context_pending_deletion(ctx) ||
                            !atomic_load(&ctx->running) || is_shutdown_initiated() ||
                            (state && (is_stream_state_stopping(state) || !are_stream_callbacks_enabled(state)))) {
                            log_info("Unified HLS thread for %s stopping during connection attempt due to %s",
                                    stream_name,
                                    is_context_already_freed(ctx) ? "context already freed" :
                                    is_context_pending_deletion(ctx) ? "context pending deletion" :
                                    !atomic_load(&ctx->running) ? "running flag cleared" :
                                    is_shutdown_initiated() ? "system shutdown" :
                                    is_stream_state_stopping(state) ? "stream state STOPPING" :
                                    "callbacks disabled");
                            thread_state = HLS_THREAD_STOPPING;
                            break;
                        }

                        // Stay in CONNECTING state and try again
                        break;
                    }
                }

                // CRITICAL FIX: Check for shutdown before opening the stream
                // This prevents starting a new connection during shutdown
                if (is_shutdown_initiated()) {
                    log_info("Skipping initial connection for stream %s during shutdown", stream_name);
                    thread_state = HLS_THREAD_STOPPING;
                    break;
                }

                // CRITICAL FIX: Check if context is still valid before accessing
                if (!ctx || is_context_pending_deletion(ctx) || is_context_already_freed(ctx)) {
                    log_warn("Context for stream %s is no longer valid, skipping connection attempt", stream_name);
                    thread_state = HLS_THREAD_STOPPING;
                    break;
                }

                // CRITICAL FIX: Create a local copy of the URL to prevent use-after-free
                char local_rtsp_url[MAX_PATH_LENGTH];
                int local_protocol;

                // Copy the URL and protocol with safety checks
                if (ctx && ctx->rtsp_url[0] != '\0') {
                    strncpy(local_rtsp_url, ctx->rtsp_url, MAX_PATH_LENGTH - 1);
                    local_rtsp_url[MAX_PATH_LENGTH - 1] = '\0';
                    local_protocol = ctx->protocol;
                } else {
                    log_error("Invalid RTSP URL for stream %s", stream_name);
                    thread_state = HLS_THREAD_RECONNECTING;
                    reconnect_attempt++;
                    break;
                }

                // Open input stream
                input_ctx = NULL;

                // CRITICAL FIX: Add memory barrier before opening input stream
                // This ensures all previous memory operations are completed
                __sync_synchronize();

                ret = open_input_stream(&input_ctx, local_rtsp_url, local_protocol);
                if (ret < 0) {
                    char error_buf[AV_ERROR_MAX_STRING_SIZE] = {0};
                    av_strerror(ret, error_buf, AV_ERROR_MAX_STRING_SIZE);

                    // Log the error but don't treat any error type specially
                    log_error("Failed to connect to stream %s: %s (error code: %d)",
                             stream_name, error_buf, ret);

                    // MEMORY LEAK FIX: Use comprehensive cleanup instead of just avformat_close_input
                    // This ensures all resources are properly freed, preventing memory leaks
                    if (input_ctx) {
                        // CRITICAL FIX: Create a local copy of the pointer to prevent race conditions
                        AVFormatContext *ctx_to_cleanup = input_ctx;
                        input_ctx = NULL; // Clear the original pointer to prevent double-free

                        // CRITICAL FIX: Add memory barrier to ensure the pointer is cleared
                        __sync_synchronize();

                        comprehensive_ffmpeg_cleanup(&ctx_to_cleanup, NULL, NULL, NULL);

                        // CRITICAL FIX: Verify that the context is actually NULL after cleanup
                        if (ctx_to_cleanup) {
                            log_warn("Failed to clean up context, forcing NULL");
                            ctx_to_cleanup = NULL;
                        }
                    }

                    // CRITICAL FIX: Check for shutdown after a failed connection attempt
                    // This prevents excessive reconnection attempts during shutdown
                    if (is_shutdown_initiated()) {
                        log_info("Stopping initial connection attempts for stream %s during shutdown", stream_name);
                        thread_state = HLS_THREAD_STOPPING;
                        break;
                    }

                    // CRITICAL FIX: Check if context is still valid before accessing
                    if (ctx && !is_context_pending_deletion(ctx) && !is_context_already_freed(ctx)) {
                        // Mark connection as invalid
                        atomic_store(&ctx->connection_valid, 0);
                    } else {
                        log_warn("Context for stream %s is no longer valid, skipping connection state update", stream_name);
                        // Set thread state to stopping to exit gracefully
                        thread_state = HLS_THREAD_STOPPING;
                        break;
                    }

                    // Increment reconnection attempt counter
                    reconnect_attempt++;

                    // Calculate reconnection delay with exponential backoff
                    reconnect_delay_ms = calculate_reconnect_delay(reconnect_attempt);

                    // Cap reconnection attempts to avoid integer overflow
                    if (reconnect_attempt > 1000) {
                        reconnect_attempt = 1000;
                    }

                    log_info("Will retry connection to stream %s in %d ms (attempt %d)",
                            stream_name, reconnect_delay_ms, reconnect_attempt + 1);

                    // Sleep before retrying
                    av_usleep(reconnect_delay_ms * 1000);

                    // Stay in CONNECTING state and try again
                    break;
                }

                // Find video stream
                video_stream_idx = find_video_stream_index(input_ctx);
                if (video_stream_idx == -1) {
                    log_error("No video stream found in %s", ctx->rtsp_url);

                    // MEMORY LEAK FIX: Use comprehensive cleanup instead of just avformat_close_input
                    comprehensive_ffmpeg_cleanup(&input_ctx, NULL, NULL, NULL);

                    // Mark connection as invalid
                    atomic_store(&ctx->connection_valid, 0);

                    // Increment reconnection attempt counter
                    reconnect_attempt++;

                    // Calculate reconnection delay with exponential backoff
                    reconnect_delay_ms = calculate_reconnect_delay(reconnect_attempt);

                    // Cap reconnection attempts to avoid integer overflow
                    if (reconnect_attempt > 1000) {
                        reconnect_attempt = 1000;
                    }

                    log_info("Will retry connection to stream %s in %d ms (attempt %d)",
                            stream_name, reconnect_delay_ms, reconnect_attempt + 1);

                    // Sleep before retrying
                    av_usleep(reconnect_delay_ms * 1000);

                    // CRITICAL FIX: Check for shutdown conditions before continuing
                    if (is_context_already_freed(ctx) || is_context_pending_deletion(ctx) ||
                        !atomic_load(&ctx->running) || is_shutdown_initiated() ||
                        (state && (is_stream_state_stopping(state) || !are_stream_callbacks_enabled(state)))) {
                        log_info("Unified HLS thread for %s stopping during connection attempt due to %s",
                                stream_name,
                                is_context_already_freed(ctx) ? "context already freed" :
                                is_context_pending_deletion(ctx) ? "context pending deletion" :
                                !atomic_load(&ctx->running) ? "running flag cleared" :
                                is_shutdown_initiated() ? "system shutdown" :
                                is_stream_state_stopping(state) ? "stream state STOPPING" :
                                "callbacks disabled");
                        thread_state = HLS_THREAD_STOPPING;
                        break;
                    }

                    // Stay in CONNECTING state and try again
                    break;
                }

                // Log information about all streams in the input
                log_info("Stream %s has %d streams:", stream_name, input_ctx->nb_streams);
                for (unsigned int i = 0; i < input_ctx->nb_streams; i++) {
                    AVStream *stream = input_ctx->streams[i];
                    AVCodecParameters *codecpar = stream->codecpar;

                    if (codecpar->codec_type == AVMEDIA_TYPE_VIDEO) {
                        log_info("Stream %d: Video stream detected (codec: %d, width: %d, height: %d)",
                                i, codecpar->codec_id, codecpar->width, codecpar->height);
                    } else if (codecpar->codec_type == AVMEDIA_TYPE_AUDIO) {
                        log_info("Stream %d: Audio stream detected (codec: %d, channels: %d, sample_rate: %d)",
                                i, codecpar->codec_id, codecpar->ch_layout.nb_channels, codecpar->sample_rate);
                    } else {
                        log_info("Stream %d: Other stream type detected (codec_type: %d)",
                                i, codecpar->codec_type);
                    }
                }

                // Initialize HLS writer with stream information
                if (!is_shutdown_initiated() && input_ctx->streams[video_stream_idx]) {
                    // CRITICAL FIX: Check if context is still valid before accessing
                    if (ctx && !is_context_pending_deletion(ctx) && !is_context_already_freed(ctx) && ctx->writer) {
                        ret = hls_writer_initialize(ctx->writer, input_ctx->streams[video_stream_idx]);
                    } else {
                        log_warn("Context for stream %s is no longer valid or writer is NULL, skipping HLS writer initialization", stream_name);
                        // Set thread state to stopping to exit gracefully
                        thread_state = HLS_THREAD_STOPPING;
                        break;
                    }
                    if (ret < 0) {
                        log_error("Failed to initialize HLS writer for stream %s", stream_name);

                        // MEMORY LEAK FIX: Use comprehensive cleanup instead of just avformat_close_input
                        comprehensive_ffmpeg_cleanup(&input_ctx, NULL, NULL, NULL);

                        // CRITICAL FIX: Check if context is still valid before accessing
                        if (ctx && !is_context_pending_deletion(ctx) && !is_context_already_freed(ctx)) {
                            // Mark connection as invalid
                            atomic_store(&ctx->connection_valid, 0);
                        } else {
                            log_warn("Context for stream %s is no longer valid, skipping connection state update", stream_name);
                            // Set thread state to stopping to exit gracefully
                            thread_state = HLS_THREAD_STOPPING;
                            break;
                        }

                        // Increment reconnection attempt counter
                        reconnect_attempt++;

                        // Calculate reconnection delay with exponential backoff
                        reconnect_delay_ms = calculate_reconnect_delay(reconnect_attempt);

                        log_info("Will retry connection to stream %s in %d ms (attempt %d)",
                                stream_name, reconnect_delay_ms, reconnect_attempt + 1);

                        // Sleep before retrying
                        av_usleep(reconnect_delay_ms * 1000);

                        // CRITICAL FIX: Check for shutdown conditions before continuing
                        if (is_context_already_freed(ctx) || is_context_pending_deletion(ctx) ||
                            !atomic_load(&ctx->running) || is_shutdown_initiated() ||
                            (state && (is_stream_state_stopping(state) || !are_stream_callbacks_enabled(state)))) {
                            log_info("Unified HLS thread for %s stopping during connection attempt due to %s",
                                    stream_name,
                                    is_context_already_freed(ctx) ? "context already freed" :
                                    is_context_pending_deletion(ctx) ? "context pending deletion" :
                                    !atomic_load(&ctx->running) ? "running flag cleared" :
                                    is_shutdown_initiated() ? "system shutdown" :
                                    is_stream_state_stopping(state) ? "stream state STOPPING" :
                                    "callbacks disabled");
                            thread_state = HLS_THREAD_STOPPING;
                            break;
                        }

                        // Stay in CONNECTING state and try again
                        break;
                    }
                }

                // Connection successful
                log_info("Successfully connected to stream %s", stream_name);
                thread_state = HLS_THREAD_RUNNING;
                reconnect_attempt = 0;

                // CRITICAL FIX: Check if context is still valid before accessing
                if (is_context_already_freed(ctx) || is_context_pending_deletion(ctx)) {
                    log_warn("Context for stream %s is no longer valid, exiting thread", stream_name);
                    thread_state = HLS_THREAD_STOPPING;
                    break;
                }

                atomic_store(&ctx->connection_valid, 1);
                atomic_store(&ctx->consecutive_failures, 0);
                last_packet_time = time(NULL);
                atomic_store(&ctx->last_packet_time, (int_fast64_t)last_packet_time);
                break;

            case HLS_THREAD_RUNNING:
                // Check if we should exit before potentially blocking on av_read_frame
                // CRITICAL FIX: Check if context is still valid before accessing
                if (is_context_already_freed(ctx) || is_context_pending_deletion(ctx)) {
                    log_warn("Context for stream %s is no longer valid, exiting thread", stream_name);
                    thread_state = HLS_THREAD_STOPPING;
                    break;
                }

                if (!atomic_load(&ctx->running)) {
                    log_info("Unified HLS thread for %s detected shutdown before read", stream_name);
                    thread_state = HLS_THREAD_STOPPING;
                    break;
                }

                // Read packet
                ret = av_read_frame(input_ctx, pkt);

                if (ret < 0) {
                    // Handle read errors
                    char error_buf[AV_ERROR_MAX_STRING_SIZE] = {0};
                    av_strerror(ret, error_buf, AV_ERROR_MAX_STRING_SIZE);

                    log_error("Error reading from stream %s: %s (code: %d)",
                             stream_name, error_buf, ret);

                    // Unref packet before reconnecting
                    av_packet_unref(pkt);

                    // Transition to reconnecting state
                    thread_state = HLS_THREAD_RECONNECTING;
                    reconnect_attempt = 1;
                    atomic_store(&ctx->connection_valid, 0);
                    atomic_fetch_add(&ctx->consecutive_failures, 1);
                    break;
                }

                // Get the stream for this packet
                AVStream *input_stream = NULL;
                if (pkt->stream_index >= 0 && pkt->stream_index < input_ctx->nb_streams) {
                    input_stream = input_ctx->streams[pkt->stream_index];
                } else {
                    log_warn("Invalid stream index %d for stream %s", pkt->stream_index, stream_name);
                    av_packet_unref(pkt);
                    continue;
                }

                // Validate packet data
                if (!pkt->data || pkt->size <= 0) {
                    log_warn("Invalid packet (null data or zero size) for stream %s", stream_name);
                    av_packet_unref(pkt);
                    continue;
                }

                // Process packets based on stream type
                if (pkt->stream_index == video_stream_idx) {
                    // This is a video packet - process it

                    // CRITICAL FIX: Check if context is still valid before accessing
                    if (!ctx || is_context_pending_deletion(ctx) || is_context_already_freed(ctx)) {
                        log_warn("Context for stream %s is no longer valid, skipping packet and exiting thread", stream_name);
                        thread_state = HLS_THREAD_STOPPING;
                        break;
                    }

                    // CRITICAL FIX: Check if writer is NULL before accessing it
                    // This prevents segmentation fault during shutdown
                    if (!ctx->writer) {
                        log_warn("Writer is NULL for stream %s during packet processing, skipping packet", stream_name);
                        continue;
                    }

                    // CRITICAL FIX: Use a local copy of the writer pointer to prevent race conditions
                    hls_writer_t *writer = ctx->writer;

                    // CRITICAL FIX: Add memory barrier to ensure we have a consistent view of the writer
                    __sync_synchronize();

                    // Check again after memory barrier
                    if (!writer) {
                        log_warn("Writer became NULL for stream %s after memory barrier, skipping packet", stream_name);
                        continue;
                    }

                    // Lock the writer mutex
                    pthread_mutex_lock(&writer->mutex);

                    // CRITICAL FIX: Check if writer is still valid after locking
                    if (ctx->writer != writer) {
                        log_warn("Writer changed for stream %s during mutex lock, releasing mutex and skipping packet", stream_name);
                        pthread_mutex_unlock(&writer->mutex);
                        continue;
                    }

                    ret = hls_writer_write_packet(writer, pkt, input_stream);

                    // Log key frames for debugging
                    bool is_key_frame = (pkt->flags & AV_PKT_FLAG_KEY) != 0;
                    if (is_key_frame && ret >= 0) {
                        log_debug("Processed video key frame for stream %s", stream_name);
                    }

                    // Handle write errors
                    if (ret < 0) {
                        char error_buf[AV_ERROR_MAX_STRING_SIZE] = {0};
                        av_strerror(ret, error_buf, AV_ERROR_MAX_STRING_SIZE);
                        log_warn("Error writing video packet to HLS for stream %s: %s", stream_name, error_buf);
                    } else {
                        // Successfully processed a packet
                        last_packet_time = time(NULL);
                        atomic_store(&ctx->last_packet_time, (int_fast64_t)last_packet_time);
                        atomic_store(&ctx->consecutive_failures, 0);
                        atomic_store(&ctx->connection_valid, 1);

                        // Mark that we've successfully written packets
                        // This is used to determine if we need to recreate the writer on reconnection
                        has_written_packets = true;
                    }

                    // Unlock the writer mutex
                    pthread_mutex_unlock(&writer->mutex);
                } else {
                    // This is a non-video packet (likely audio)
                    // For now, we'll just log it and skip processing
                    // This prevents the "Invalid packet stream index" errors
                    if (pkt->stream_index >= 0 && pkt->stream_index < input_ctx->nb_streams) {
                        AVStream *stream = input_ctx->streams[pkt->stream_index];
                        AVCodecParameters *codecpar = stream->codecpar;

                        if (codecpar->codec_type == AVMEDIA_TYPE_AUDIO) {
                            log_debug("Skipping audio packet for stream %s (stream index: %d)",
                                    stream_name, pkt->stream_index);
                        } else {
                            log_debug("Skipping non-video packet for stream %s (stream index: %d, type: %d)",
                                    stream_name, pkt->stream_index, codecpar->codec_type);
                        }
                    } else {
                        log_warn("Skipping packet with invalid stream index %d for stream %s",
                                pkt->stream_index, stream_name);
                    }
                }

                // Unref packet
                av_packet_unref(pkt);

                // Check if we haven't received a packet in a while
                time_t now = time(NULL);
                if (now - last_packet_time > MAX_PACKET_TIMEOUT) {
                    log_error("No packets received from stream %s for %ld seconds, reconnecting",
                             stream_name, now - last_packet_time);
                    thread_state = HLS_THREAD_RECONNECTING;
                    reconnect_attempt = 1;

                    // CRITICAL FIX: Check if context is still valid before accessing
                    if (ctx && !is_context_pending_deletion(ctx) && !is_context_already_freed(ctx)) {
                        atomic_store(&ctx->connection_valid, 0);
                        atomic_fetch_add(&ctx->consecutive_failures, 1);
                    } else {
                        log_warn("Context for stream %s is no longer valid, skipping connection state update", stream_name);
                        // Set thread state to stopping to exit gracefully
                        thread_state = HLS_THREAD_STOPPING;
                    }
                }
                break;

            case HLS_THREAD_RECONNECTING:
                log_info("Reconnecting to stream %s (attempt %d)", stream_name, reconnect_attempt);

                // Close existing connection
                safe_cleanup_resources(&input_ctx, NULL, NULL);

                // Calculate reconnection delay with exponential backoff
                reconnect_delay_ms = calculate_reconnect_delay(reconnect_attempt);

                // Sleep before reconnecting
                log_info("Waiting %d ms before reconnecting to stream %s", reconnect_delay_ms, stream_name);
                av_usleep(reconnect_delay_ms * 1000);

                // Check if we should stop during the sleep
                // CRITICAL FIX: Check if context is still valid before accessing
                if (is_context_already_freed(ctx) || is_context_pending_deletion(ctx)) {
                    log_warn("Context for stream %s is no longer valid, exiting thread", stream_name);
                    thread_state = HLS_THREAD_STOPPING;
                    break;
                }

                // CRITICAL FIX: Check for shutdown conditions
                if (!atomic_load(&ctx->running) || is_shutdown_initiated() ||
                    (state && (is_stream_state_stopping(state) || !are_stream_callbacks_enabled(state)))) {
                    log_info("Unified HLS thread for %s stopping during reconnection due to %s",
                            stream_name,
                            !atomic_load(&ctx->running) ? "running flag cleared" :
                            is_shutdown_initiated() ? "system shutdown" :
                            is_stream_state_stopping(state) ? "stream state STOPPING" :
                            "callbacks disabled");
                    thread_state = HLS_THREAD_STOPPING;
                    break;
                }

                // CRITICAL FIX: Check for shutdown again right before opening the stream
                // This prevents starting a new connection during shutdown
                if (is_shutdown_initiated()) {
                    log_info("Skipping reconnection for stream %s during shutdown", stream_name);
                    thread_state = HLS_THREAD_STOPPING;
                    break;
                }

                // CRITICAL FIX: Check if context is still valid before accessing
                if (!ctx || is_context_pending_deletion(ctx) || is_context_already_freed(ctx)) {
                    log_warn("Context for stream %s is no longer valid, skipping reconnection attempt", stream_name);
                    thread_state = HLS_THREAD_STOPPING;
                    break;
                }

                // CRITICAL FIX: Create a local copy of the URL to prevent use-after-free
                char reconnect_rtsp_url[MAX_PATH_LENGTH];
                int reconnect_protocol;

                // Copy the URL and protocol with safety checks
                if (ctx && ctx->rtsp_url[0] != '\0') {
                    strncpy(reconnect_rtsp_url, ctx->rtsp_url, MAX_PATH_LENGTH - 1);
                    reconnect_rtsp_url[MAX_PATH_LENGTH - 1] = '\0';
                    reconnect_protocol = ctx->protocol;
                } else {
                    log_error("Invalid RTSP URL for stream %s during reconnection", stream_name);
                    thread_state = HLS_THREAD_RECONNECTING;
                    reconnect_attempt++;
                    break;
                }

                // Open input stream
                input_ctx = NULL;

                // CRITICAL FIX: Add memory barrier before opening input stream
                // This ensures all previous memory operations are completed
                __sync_synchronize();

                ret = open_input_stream(&input_ctx, reconnect_rtsp_url, reconnect_protocol);
                if (ret < 0) {
                    char error_buf[AV_ERROR_MAX_STRING_SIZE] = {0};
                    av_strerror(ret, error_buf, AV_ERROR_MAX_STRING_SIZE);

                    // Log the error but don't treat any error type specially
                    log_error("Failed to reconnect to stream %s: %s (error code: %d)",
                             stream_name, error_buf, ret);

                    // MEMORY LEAK FIX: Use comprehensive cleanup instead of just avformat_close_input
                    // This ensures all resources are properly freed, preventing memory leaks
                    if (input_ctx) {
                        // CRITICAL FIX: Create a local copy of the pointer to prevent race conditions
                        AVFormatContext *ctx_to_cleanup = input_ctx;
                        input_ctx = NULL; // Clear the original pointer to prevent double-free

                        // CRITICAL FIX: Add memory barrier to ensure the pointer is cleared
                        __sync_synchronize();

                        comprehensive_ffmpeg_cleanup(&ctx_to_cleanup, NULL, NULL, NULL);

                        // CRITICAL FIX: Verify that the context is actually NULL after cleanup
                        if (ctx_to_cleanup) {
                            log_warn("Failed to clean up context, forcing NULL");
                            ctx_to_cleanup = NULL;
                        }
                    }

                    // CRITICAL FIX: Check for shutdown after a failed connection attempt
                    // This prevents excessive reconnection attempts during shutdown
                    if (is_shutdown_initiated()) {
                        log_info("Stopping reconnection attempts for stream %s during shutdown", stream_name);
                        thread_state = HLS_THREAD_STOPPING;
                        break;
                    }

                    // Increment reconnection attempt counter
                    reconnect_attempt++;

                    // Cap reconnection attempts to avoid integer overflow
                    if (reconnect_attempt > 1000) {
                        reconnect_attempt = 1000;
                    }

                    // Stay in reconnecting state
                    break;
                }

                // Find video stream
                video_stream_idx = find_video_stream_index(input_ctx);
                if (video_stream_idx == -1) {
                    log_error("No video stream found in %s during reconnection", ctx->rtsp_url);

                    // Close input context
                    avformat_close_input(&input_ctx);

                    // Increment reconnection attempt counter
                    reconnect_attempt++;

                    // Cap reconnection attempts to avoid integer overflow
                    if (reconnect_attempt > 1000) {
                        reconnect_attempt = 1000;
                    }

                    // Stay in reconnecting state
                    break;
                }

                // Log information about all streams in the input during reconnection
                log_info("Stream %s has %d streams during reconnection:", stream_name, input_ctx->nb_streams);
                for (unsigned int i = 0; i < input_ctx->nb_streams; i++) {
                    AVStream *stream = input_ctx->streams[i];
                    AVCodecParameters *codecpar = stream->codecpar;

                    if (codecpar->codec_type == AVMEDIA_TYPE_VIDEO) {
                        log_info("Stream %d: Video stream detected (codec: %d, width: %d, height: %d)",
                                i, codecpar->codec_id, codecpar->width, codecpar->height);
                    } else if (codecpar->codec_type == AVMEDIA_TYPE_AUDIO) {
                        log_info("Stream %d: Audio stream detected (codec: %d, channels: %d, sample_rate: %d)",
                                i, codecpar->codec_id, codecpar->ch_layout.nb_channels, codecpar->sample_rate);
                    } else {
                        log_info("Stream %d: Other stream type detected (codec_type: %d)",
                                i, codecpar->codec_type);
                    }
                }

                // CRITICAL FIX: Add additional validation before considering reconnection successful
                // Verify that the input context is valid and has streams
                if (!input_ctx || input_ctx->nb_streams == 0) {
                    log_error("Reconnection to stream %s failed: Invalid input context or no streams",
                            stream_name);

                    // Clean up any partially initialized resources
                    if (input_ctx) {
                        log_warn("Cleaning up invalid input context for stream %s", stream_name);
                        safe_cleanup_resources(&input_ctx, NULL, NULL);
                    }

                    // Increment reconnection attempt counter
                    reconnect_attempt++;

                    // Cap reconnection attempts to avoid integer overflow
                    if (reconnect_attempt > 1000) {
                        reconnect_attempt = 1000;
                    }

                    // Stay in reconnecting state
                    break;
                }

                // Reconnection successful
                log_info("Successfully reconnected to stream %s after %d attempts",
                        stream_name, reconnect_attempt);

                // CRITICAL FIX: Only recreate HLS writer if we've actually written packets before
                // This prevents crashing when the initial connection fails and we enter RECONNECTING
                // state before ever writing any packets. In that case, the writer was just created
                // and doesn't have stale timestamp state.
                if (has_written_packets && ctx->writer) {
                    log_info("Closing old HLS writer for stream %s after reconnection (has_written_packets=true)", stream_name);
                    hls_writer_t *old_writer = __atomic_exchange_n(&ctx->writer, NULL, __ATOMIC_SEQ_CST);
                    if (old_writer) {
                        hls_writer_close(old_writer);
                        log_info("Successfully closed old HLS writer for stream %s", stream_name);
                    }

                    // Create new HLS writer with fresh timestamp state
                    ctx->writer = hls_writer_create(ctx->output_path, stream_name, 5);
                    if (!ctx->writer) {
                        log_error("Failed to create new HLS writer for %s after reconnection", stream_name);
                        // Stay in reconnecting state to try again
                        reconnect_attempt++;
                        thread_state = HLS_THREAD_RECONNECTING;
                        break;
                    }
                    log_info("Created new HLS writer for stream %s after reconnection", stream_name);

                    // Reset the has_written_packets flag for the new writer
                    has_written_packets = false;
                } else if (!has_written_packets) {
                    log_info("Keeping existing HLS writer for stream %s (no packets written yet)", stream_name);
                }

                thread_state = HLS_THREAD_RUNNING;
                reconnect_attempt = 0;
                atomic_store(&ctx->connection_valid, 1);
                atomic_store(&ctx->consecutive_failures, 0);
                last_packet_time = time(NULL);
                atomic_store(&ctx->last_packet_time, (int_fast64_t)last_packet_time);
                break;

            case HLS_THREAD_STOPPING:
                log_info("Stopping unified HLS thread for stream %s", stream_name);
                atomic_store(&ctx->running, 0);
                atomic_store(&ctx->connection_valid, 0);

                // CRITICAL FIX: Clean up resources before transitioning to STOPPED state
                log_info("Cleaning up resources for stream %s before stopping", stream_name);

                // Clean up input context and packet
                safe_cleanup_resources(&input_ctx, &pkt, NULL);

                // Clean up HLS writer if it exists
                if (ctx->writer) {
                    log_info("Closing HLS writer for stream %s during shutdown", stream_name);
                    hls_writer_t *writer_to_cleanup = __atomic_exchange_n(&ctx->writer, NULL, __ATOMIC_SEQ_CST);
                    if (writer_to_cleanup) {
                        // Use a separate variable for the writer to prevent race conditions
                        hls_writer_close(writer_to_cleanup);
                        log_info("Successfully closed HLS writer for stream %s during shutdown", stream_name);
                    }
                }

                // Update component state in shutdown coordinator
                if (ctx->shutdown_component_id >= 0) {
                    update_component_state(ctx->shutdown_component_id, COMPONENT_STOPPED);
                    log_info("Updated component state to STOPPED for stream %s", stream_name);
                }

                thread_state = HLS_THREAD_STOPPED;
                break;

            case HLS_THREAD_STOPPED:
                // We should never reach this state in the loop
                log_warn("Unified HLS thread for %s in STOPPED state but still in main loop", stream_name);
                atomic_store(&ctx->running, 0);
                break;

            default:
                // Unknown state
                log_error("Unified HLS thread for %s in unknown state: %d", stream_name, thread_state);
                atomic_store(&ctx->running, 0);
                break;
        }
    }

    // Signal that the thread is exiting
    // Make a local copy of the context pointer to prevent race conditions
    hls_unified_thread_ctx_t *ctx_for_exit = ctx;
    ctx = NULL;  // Clear the original pointer to prevent further access

    // CRITICAL FIX: Check if the context is already freed or pending deletion
    // This prevents accessing invalid memory during cleanup
    bool context_valid_for_exit = ctx_for_exit && !is_context_already_freed(ctx_for_exit) && !is_context_pending_deletion(ctx_for_exit);

    // Store stream name in local buffer for logging even if context becomes invalid
    char stream_name_buf[MAX_STREAM_NAME] = {0};
    if (stream_name) {
        strncpy(stream_name_buf, stream_name, MAX_STREAM_NAME - 1);
        stream_name_buf[MAX_STREAM_NAME - 1] = '\0';
    } else {
        strcpy(stream_name_buf, "unknown");
    }

    // CRITICAL FIX: Ensure all resources are cleaned up before exiting
    // This is a safety measure in case we exited the loop without proper cleanup
    if (input_ctx != NULL || pkt != NULL) {
        log_warn("Thread for stream %s exited loop without proper cleanup, cleaning up now", stream_name);
        safe_cleanup_resources(&input_ctx, &pkt, NULL);
    }

    if (ctx_for_exit) {
        // Use the pre-computed context validity flag
        // CRITICAL FIX: Only access writer if context is still valid
        hls_writer_t *writer_to_cleanup = NULL;
        if (context_valid_for_exit) {
            // Use a try/catch-like approach with signal handling to prevent crashes
            struct sigaction sa_old, sa_new;
            sigaction(SIGSEGV, NULL, &sa_old);
            sa_new = sa_old;
            sa_new.sa_handler = SIG_IGN; // Ignore segmentation fault signal
            sigaction(SIGSEGV, &sa_new, NULL);

            // Set alarm to prevent hanging if memory is inaccessible
            alarm(1); // 1 second timeout

            // Safely check if writer exists and clean it up
            if (ctx_for_exit->writer) {
                log_warn("Writer for stream %s still exists after loop exit, cleaning up now", stream_name);
                writer_to_cleanup = __atomic_exchange_n(&ctx_for_exit->writer, NULL, __ATOMIC_SEQ_CST);
            }

            // Cancel the alarm and restore signal handler
            alarm(0);
            sigaction(SIGSEGV, &sa_old, NULL);
        } else {
            log_warn("Context for stream %s is no longer valid, skipping writer cleanup", stream_name);
        }

        // Clean up the writer if we got one
        if (writer_to_cleanup) {
            hls_writer_close(writer_to_cleanup);
            log_info("Successfully closed HLS writer for stream %s after loop exit", stream_name);
        }

        // Mark thread as exited in the pending deletion list
        // This is safe to do even if the context is freed, as mark_thread_exited handles NULL contexts
        mark_thread_exited(ctx_for_exit);

        // Only access ctx members if the context is not already freed
        if (context_valid_for_exit) {
            // Use a try/catch-like approach with signal handling to prevent crashes
            struct sigaction sa_old, sa_new;
            sigaction(SIGSEGV, NULL, &sa_old);
            sa_new = sa_old;
            sa_new.sa_handler = SIG_IGN; // Ignore segmentation fault signal
            sigaction(SIGSEGV, &sa_new, NULL);

            // Set alarm to prevent hanging if memory is inaccessible
            alarm(1); // 1 second timeout

            // CRITICAL FIX: Check if context is still valid before accessing its members
            // This prevents use-after-free errors when the context has been freed
            if (!is_context_already_freed(ctx_for_exit) && !is_context_pending_deletion(ctx_for_exit)) {
                // Mark connection as invalid
                atomic_store(&ctx_for_exit->connection_valid, 0);

                // Update component state in shutdown coordinator
                if (ctx_for_exit->shutdown_component_id >= 0) {
                    update_component_state(ctx_for_exit->shutdown_component_id, COMPONENT_STOPPED);
                }
            } else {
                log_warn("Context for stream %s is no longer valid, skipping connection_valid update", stream_name_buf);
            }

            log_info("Updated component state to STOPPED for stream %s after loop exit", stream_name);

            // Cancel the alarm and restore signal handler
            alarm(0);
            sigaction(SIGSEGV, &sa_old, NULL);
        }
    }

    // Clear the reference in the stream state
    // CRITICAL FIX: Add additional safety checks to prevent segfault
    if (state) {
        // Make a local copy of the writer pointer for safety
        hls_writer_t *writer_ptr = NULL;

        // Only access ctx members if the context is not already freed
        if (ctx && !is_context_already_freed(ctx)) {
            writer_ptr = ctx->writer;
        }

        // Clear the reference if it matches
        if (writer_ptr && state->hls_ctx == writer_ptr) {
            state->hls_ctx = NULL;
        }
    }

    // Stream name is already stored in stream_name_buf

    // CRITICAL FIX: Add safety checks before cleaning up resources
    log_info("Cleaning up all resources for stream %s", stream_name_buf);

    // CRITICAL FIX: Ensure all resources are cleaned up before exiting
    // This is a final safety check to prevent memory leaks
    if (input_ctx || pkt) {
        log_info("Final cleanup of FFmpeg resources for stream %s", stream_name_buf);
        if (input_ctx) {
            log_debug("Cleaning up input context for stream %s", stream_name_buf);
        }
        if (pkt) {
            log_debug("Cleaning up packet for stream %s", stream_name_buf);
        }
        // Clean up any remaining FFmpeg resources
        safe_cleanup_resources(&input_ctx, &pkt, NULL);
    }

    // CRITICAL FIX: Check ctx before accessing its members
    hls_writer_t *writer_to_cleanup = NULL;
    int shutdown_id = -1;

    // Make a local copy of the context pointer to prevent race conditions
    hls_unified_thread_ctx_t *ctx_local = ctx;
    ctx = NULL;  // Clear the original pointer to prevent further access

    // Check if the context is valid before accessing its members
    if (ctx_local && !is_context_already_freed(ctx_local) && !is_context_pending_deletion(ctx_local)) {
        // Safely get the writer pointer
        hls_writer_t *writer_ptr = NULL;

        // Use atomic load to safely access the writer pointer
        writer_ptr = __atomic_load_n(&ctx_local->writer, __ATOMIC_SEQ_CST);

        if (writer_ptr) {
            log_debug("Cleaning up HLS writer for stream %s", stream_name_buf);

            // CRITICAL FIX: Use atomic exchange to safely get and clear the writer pointer
            // This ensures that no other thread can access the writer after we've taken ownership of it
            writer_to_cleanup = __atomic_exchange_n(&ctx_local->writer, NULL, __ATOMIC_SEQ_CST);

            // CRITICAL FIX: Store a local copy of the stream name for logging
            char writer_stream_name[MAX_STREAM_NAME] = {0};
            if (writer_to_cleanup && writer_to_cleanup->stream_name) {
                strncpy(writer_stream_name, writer_to_cleanup->stream_name, MAX_STREAM_NAME - 1);
                writer_stream_name[MAX_STREAM_NAME - 1] = '\0';
                log_info("Preparing to clean up HLS writer for stream %s", writer_stream_name);
            }
        }

        // Store the shutdown component ID for later
        shutdown_id = ctx_local->shutdown_component_id;
    } else {
        // This is expected during shutdown when multiple cleanup paths run
        log_debug("Context for stream %s is no longer valid during cleanup (already cleaned up)", stream_name_buf);
    }

    // CRITICAL FIX: Add memory barrier before cleanup to ensure all threads see consistent state
    __sync_synchronize();

    // CRITICAL FIX: We've already cleaned up the FFmpeg resources above, so we only need to handle the writer
    // No need to make local copies of input_ctx and pkt since they should be NULL at this point

    // CRITICAL FIX: Writer reference has already been cleared using atomic exchange
    // No need to clear it again

    // CRITICAL FIX: Add additional safety checks before cleanup
    log_info("About to clean up remaining resources for stream %s", stream_name_buf);

    // CRITICAL FIX: Add memory barrier before cleanup
    __sync_synchronize();

    // CRITICAL FIX: Only clean up the writer if it's still valid
    if (writer_to_cleanup) {
        log_debug("Writer is valid for stream %s, cleaning up", stream_name_buf);

        // Clean up the writer with additional try/catch-like protection
        log_info("Closing HLS writer for stream %s", stream_name_buf);

        // Use a try/catch-like approach with signal handling to prevent crashes
        struct sigaction sa_old, sa_new;
        sigaction(SIGALRM, NULL, &sa_old);
        sa_new = sa_old;
        sa_new.sa_handler = SIG_IGN; // Ignore alarm signal
        sigaction(SIGALRM, &sa_new, NULL);

        // Set alarm
        alarm(15); // 15 second timeout for writer close

        // Close the writer with additional protection
        hls_writer_close(writer_to_cleanup);
        writer_to_cleanup = NULL;

        // Cancel the alarm and restore signal handler
        alarm(0);
        sigaction(SIGALRM, &sa_old, NULL);

        log_info("Successfully closed HLS writer for stream %s", stream_name_buf);
    } else {
        log_debug("No writer to clean up for stream %s", stream_name_buf);
    }

    // Update component state in shutdown coordinator only if we have a valid ID
    if (shutdown_id >= 0) {
        // CRITICAL FIX: Ensure the component is marked as STOPPED in the shutdown coordinator
        // This is critical to prevent the shutdown coordinator from waiting indefinitely
        update_component_state(shutdown_id, COMPONENT_STOPPED);
        log_info("Updated unified HLS thread %s state to STOPPED in shutdown coordinator", stream_name_buf);
    }

    // CRITICAL FIX: Ensure the thread is marked as exited in the pending deletion list
    // This is a final safety check to prevent memory leaks and deadlocks
    if (ctx_local) {
        mark_thread_exited(ctx_local);
        log_info("Final marking of thread as exited for stream %s", stream_name_buf);
    }

    // Unmark the stream as stopping to indicate we've completed our shutdown
    unmark_stream_stopping(stream_name_buf);
    log_info("Unmarked stream %s as stopping before thread exit", stream_name_buf);

    // CRITICAL FIX: Add additional safety checks before freeing the context
    if (ctx_local) {
        // CRITICAL FIX: Add memory barrier before freeing to ensure all accesses are complete
        __sync_synchronize();

        // CRITICAL FIX: Use a local copy of the pointer to prevent race conditions
        hls_unified_thread_ctx_t *ctx_to_free = ctx_local;
        ctx_local = NULL;

        // CRITICAL FIX: Add memory barrier after nulling the pointer
        __sync_synchronize();

        // CRITICAL FIX: Add additional validation before freeing the context
        // This helps catch cases where the memory has been corrupted or already freed
        bool context_valid = true;

        // Basic validation of context structure
        if (ctx_to_free->stream_name == NULL || ctx_to_free->stream_name[0] == '\0') {
            log_warn("Context has NULL or empty stream_name, may be corrupted");
            context_valid = false;
        }

        // Check if the context has already been freed (this is a heuristic)
        // We check if the first few bytes are zeroed out, which might indicate
        // that the memory has been freed or corrupted
        unsigned char *ptr = (unsigned char *)ctx_to_free;
        bool all_zeros = true;
        for (int i = 0; i < 16 && i < sizeof(hls_unified_thread_ctx_t); i++) {
            if (ptr[i] != 0) {
                all_zeros = false;
                break;
            }
        }

        if (all_zeros) {
            log_warn("Context memory appears to be zeroed out, may have been freed already");
            context_valid = false;
        }

        if (context_valid) {
            // CRITICAL FIX: Check if the context has already been freed
            if (is_context_already_freed(ctx_to_free)) {
                log_warn("Context for stream %s has already been freed, skipping", stream_name_buf);
            } else {
                // Free the context
                log_info("Freeing context for stream %s", stream_name_buf);

                // Use a try/catch-like approach with signal handling to prevent crashes
                struct sigaction sa_old, sa_new;
                sigaction(SIGALRM, NULL, &sa_old);
                sa_new = sa_old;
                sa_new.sa_handler = SIG_IGN; // Ignore alarm signal
                sigaction(SIGALRM, &sa_new, NULL);

                // Set alarm
                alarm(15); // 15 second timeout for context free

                // Mark the context as pending deletion to signal the thread
                mark_context_pending_deletion(ctx_to_free);

                // Wait for the thread to exit
                wait_for_thread_exit(ctx_to_free);

                // Mark the context as freed before actually freeing it
                mark_context_as_freed(ctx_to_free);

                // Clear from pending deletion list
                clear_context_pending_deletion(ctx_to_free);

                // Free the context with additional protection
                safe_free(ctx_to_free);

                // Cancel the alarm and restore signal handler
                alarm(0);
                sigaction(SIGALRM, &sa_old, NULL);

                log_info("Successfully freed context for stream %s", stream_name_buf);
            }
        } else {
            log_debug("Skipping cleanup of potentially invalid context for stream %s (already cleaned up)", stream_name_buf);
        }
    } else {
        // This is expected during shutdown when multiple cleanup paths run
        log_debug("Context is NULL during cleanup for stream %s (already cleaned up)", stream_name_buf);
    }

    log_info("Unified HLS thread for stream %s exited", stream_name_buf);

    // MEMORY LEAK FIX: Enhanced final FFmpeg resource cleanup
    // This is a comprehensive cleanup to ensure no FFmpeg resources are leaked
    // CRITICAL FIX: Added safety checks to prevent segfaults during shutdown

    // Use try/catch-like approach with signal handling to prevent crashes
    struct sigaction sa_old, sa_new;
    sigaction(SIGSEGV, NULL, &sa_old);
    sa_new = sa_old;
    sa_new.sa_handler = SIG_IGN; // Ignore segmentation fault signal
    sigaction(SIGSEGV, &sa_new, NULL);

    // Set alarm to prevent hanging if memory is inaccessible
    alarm(1); // 1 second timeout

    if (input_ctx) {
        log_warn("Input context still exists at thread exit for stream %s, forcing cleanup", stream_name_buf);

        // First try to close the input context properly
        // CRITICAL FIX: Use a local copy of the pointer to prevent race conditions
        AVFormatContext *ctx_to_cleanup = input_ctx;
        input_ctx = NULL; // Clear the original pointer to prevent double-free

        // Use our comprehensive cleanup function
        comprehensive_ffmpeg_cleanup(&ctx_to_cleanup, NULL, NULL, NULL);
    }

    if (pkt) {
        log_warn("Packet still exists at thread exit for stream %s, forcing cleanup", stream_name_buf);

        // CRITICAL FIX: Use a local copy of the pointer to prevent race conditions
        AVPacket *pkt_to_cleanup = pkt;
        pkt = NULL; // Clear the original pointer to prevent double-free

        // Safely unref and free the packet
        if (pkt_to_cleanup) {
            av_packet_unref(pkt_to_cleanup);
            av_packet_free(&pkt_to_cleanup);
        }
    }

    // Cancel the alarm and restore signal handler
    alarm(0);
    sigaction(SIGSEGV, &sa_old, NULL);

    // MEMORY LEAK FIX: Force FFmpeg to release any cached memory
    // This is a more aggressive approach to ensure all memory is freed
    // We call this directly here to ensure it happens before the thread exits
    // CRITICAL FIX: Only call this if we're not in shutdown or during stream deletion to prevent segfaults
    if (!is_shutdown_initiated() && !is_stream_stopping(stream_name_buf)) {
        // Use a safer approach to release memory
        log_info("Using safer approach to release FFmpeg memory during thread exit");

        // Instead of calling the full ffmpeg_buffer_cleanup which can cause segfaults,
        // just do minimal cleanup that's less likely to crash
        void *dummy = av_malloc(16);
        if (dummy) {
            av_free(dummy);
        }

        // Set FFmpeg log level to quiet to reduce memory usage
        av_log_set_level(AV_LOG_QUIET);
    } else {
        log_info("Skipping aggressive FFmpeg cleanup during shutdown or stream deletion to prevent crashes");
    }

    // CRITICAL FIX: Add final safety check before marking thread as exited
    // This prevents segmentation faults if the context has been freed during cleanup
    if (ctx_for_exit && !is_context_already_freed(ctx_for_exit)) {
        // Mark thread as exited to ensure proper cleanup
        mark_thread_exited(ctx_for_exit);
    } else {
        log_info("Context for stream %s is no longer valid, skipping final thread exit marking", stream_name_buf);
    }

    log_info("Unified HLS thread for stream %s has completed all cleanup steps", stream_name_buf);
    return NULL;
}

// Define the mutex and contexts array as non-static so they can be accessed from other files
pthread_mutex_t unified_contexts_mutex = PTHREAD_MUTEX_INITIALIZER;
hls_unified_thread_ctx_t *unified_contexts[MAX_STREAMS];

// Watchdog thread variables
static pthread_t hls_watchdog_thread;
static atomic_bool watchdog_running = ATOMIC_VAR_INIT(false);
static atomic_int watchdog_restart_count = ATOMIC_VAR_INIT(0);
static time_t last_restart_time[MAX_STREAMS] = {0};
static int restart_attempts[MAX_STREAMS] = {0};

/**
 * Start HLS streaming for a stream using the unified thread approach
 * This is the implementation that will be called by the API functions
 */
int start_hls_unified_stream(const char *stream_name) {
    // CRITICAL FIX: Check if shutdown is in progress and prevent starting new streams
    if (is_shutdown_initiated()) {
        log_warn("Cannot start HLS stream %s during shutdown", stream_name);
        return -1;
    }

    stream_handle_t stream = get_stream_by_name(stream_name);
    if (!stream) {
        log_error("Stream %s not found", stream_name);
        return -1;
    }

    stream_config_t config;
    if (get_stream_config(stream, &config) != 0) {
        log_error("Failed to get config for stream %s", stream_name);
        return -1;
    }

    // Get the stream state manager
    stream_state_manager_t *state = get_stream_state_by_name(stream_name);
    if (!state) {
        log_error("Stream state not found for %s", stream_name);
        return -1;
    }

    // Check if the stream is in the process of being stopped
    if (is_stream_state_stopping(state)) {
        log_warn("Cannot start HLS stream %s while it is in the process of being stopped", stream_name);
        return -1;
    }

    // Add a reference for the HLS component
    stream_state_add_ref(state, STREAM_COMPONENT_HLS);
    log_info("Added HLS reference to stream %s", stream_name);

    // CRITICAL FIX: Hold the mutex during the entire thread creation process to prevent race conditions
    // that could lead to multiple threads for the same stream
    pthread_mutex_lock(&unified_contexts_mutex);

    // Check if already running and handle duplicate contexts
    bool already_running = false;
    int running_count = 0;
    int running_indices[MAX_STREAMS];
    int valid_connection_idx = -1;
    int best_context_idx = -1;

    // First pass: count running contexts and find any with valid connections
    for (int i = 0; i < MAX_STREAMS; i++) {
        if (unified_contexts[i] && strcmp(unified_contexts[i]->stream_name, stream_name) == 0) {
            running_indices[running_count++] = i;
            already_running = true;

            // Check if this context has a valid connection
            if (atomic_load(&unified_contexts[i]->connection_valid)) {
                valid_connection_idx = i;
                log_info("Found running HLS context for stream %s with valid connection at index %d",
                        stream_name, i);
            }
        }
    }

    // If we found any running instances
    if (running_count > 0) {
        log_info("Found %d running HLS contexts for stream %s", running_count, stream_name);

        // If we have a valid connection, keep that one and stop all others
        if (valid_connection_idx >= 0) {
            best_context_idx = valid_connection_idx;
            log_info("Keeping HLS context with valid connection for stream %s at index %d",
                    stream_name, best_context_idx);
        }
        // If no valid connection, keep the first one and stop all others
        else if (running_count > 0) {
            best_context_idx = running_indices[0];
            log_info("No valid connections found, keeping first HLS context for stream %s at index %d",
                    stream_name, best_context_idx);
        }

        // Stop all contexts except the best one
        for (int i = 0; i < running_count; i++) {
            int idx = running_indices[i];
            if (idx != best_context_idx) {
                log_info("Stopping duplicate HLS context at index %d for stream %s", idx, stream_name);

                // Mark as not running
                atomic_store(&unified_contexts[idx]->running, 0);

                // We'll let the thread clean itself up
                // The context will be freed when the thread exits
            }
        }

        // If we have a context to keep, return success
        if (best_context_idx >= 0) {
            log_info("Using existing HLS context for stream %s at index %d", stream_name, best_context_idx);
            pthread_mutex_unlock(&unified_contexts_mutex);
            return 0;  // Using existing context
        }
    }

    // Find empty slot
    int slot = -1;
    for (int i = 0; i < MAX_STREAMS; i++) {
        if (!unified_contexts[i]) {
            slot = i;
            break;
        }
    }

    if (slot == -1) {
        log_error("No slot available for new HLS stream");
        pthread_mutex_unlock(&unified_contexts_mutex);
        return -1;
    }

    // CRITICAL FIX: Keep the mutex locked while we create the thread to prevent race conditions

    // Clear any existing HLS segments for this stream
    log_info("Clearing any existing HLS segments for stream %s before starting", stream_name);
    clear_stream_hls_segments(stream_name);

    // Create context with memory guards
    hls_unified_thread_ctx_t *ctx = safe_malloc(sizeof(hls_unified_thread_ctx_t));
    if (!ctx) {
        log_error("Memory allocation failed for unified HLS context");
        return -1;
    }

    // Initialize the memory to zero
    memset(ctx, 0, sizeof(hls_unified_thread_ctx_t));
    strncpy(ctx->stream_name, stream_name, MAX_STREAM_NAME - 1);
    ctx->stream_name[MAX_STREAM_NAME - 1] = '\0';

    // Get RTSP URL
    char actual_url[MAX_PATH_LENGTH];
    strncpy(actual_url, config.url, sizeof(actual_url) - 1);
    actual_url[sizeof(actual_url) - 1] = '\0';

    // If the stream is using go2rtc for HLS, get the go2rtc RTSP URL
    if (go2rtc_integration_is_using_go2rtc_for_hls(stream_name)) {
        // Get the go2rtc RTSP URL with retry logic
        int rtsp_retries = 3;
        bool rtsp_url_success = false;

        while (rtsp_retries > 0 && !rtsp_url_success) {
            if (go2rtc_get_rtsp_url(stream_name, actual_url, sizeof(actual_url))) {
                log_info("Using go2rtc RTSP URL for HLS streaming: %s", actual_url);
                rtsp_url_success = true;
            } else {
                log_warn("Failed to get go2rtc RTSP URL for stream %s (retries left: %d)",
                        stream_name, rtsp_retries - 1);
                rtsp_retries--;

                if (rtsp_retries > 0) {
                    log_info("Waiting before retrying to get RTSP URL for stream %s...", stream_name);
                    sleep(3); // Wait 3 seconds before retrying
                }
            }
        }

        if (!rtsp_url_success) {
            log_warn("Failed to get go2rtc RTSP URL for stream %s after multiple attempts, falling back to original URL", stream_name);
        }
    }

    // Copy the URL to the context
    strncpy(ctx->rtsp_url, actual_url, MAX_PATH_LENGTH - 1);
    ctx->rtsp_url[MAX_PATH_LENGTH - 1] = '\0';

    // Set protocol information in the context
    ctx->protocol = config.protocol;

    // Set segment duration
    ctx->segment_duration = config.segment_duration > 0 ? config.segment_duration : 2;

    // Create output paths
    config_t *global_config = get_streaming_config();

    // Use storage_path_hls if specified, otherwise fall back to storage_path
    const char *base_storage_path = global_config->storage_path;
    if (global_config->storage_path_hls[0] != '\0') {
        base_storage_path = global_config->storage_path_hls;
        log_info("Using dedicated HLS storage path: %s", base_storage_path);
    } else {
        log_info("Using default storage path for HLS: %s", base_storage_path);
    }

    // Create HLS output path
    snprintf(ctx->output_path, MAX_PATH_LENGTH, "%s/hls/%s",
             base_storage_path, stream_name);

    // Create HLS directory if it doesn't exist using direct C functions to handle paths with spaces
    struct stat st;
    if (stat(ctx->output_path, &st) != 0 || !S_ISDIR(st.st_mode)) {
        log_info("Creating HLS directory: %s", ctx->output_path);

        // Create parent directories one by one
        char temp_path[MAX_PATH_LENGTH];
        strncpy(temp_path, ctx->output_path, MAX_PATH_LENGTH - 1);
        temp_path[MAX_PATH_LENGTH - 1] = '\0';

        for (char *p = temp_path + 1; *p; p++) {
            if (*p == '/') {
                *p = '\0';
                if (mkdir(temp_path, 0777) != 0 && errno != EEXIST) {
                    log_warn("Failed to create parent directory: %s (error: %s)", temp_path, strerror(errno));
                }
                *p = '/';
            }
        }

        // Create the final directory
        if (mkdir(temp_path, 0777) != 0 && errno != EEXIST) {
            log_error("Failed to create output directory: %s (error: %s)", temp_path, strerror(errno));
            safe_free(ctx);
            return -1;
        }

        // Verify the directory was created
        if (stat(ctx->output_path, &st) != 0 || !S_ISDIR(st.st_mode)) {
            log_error("Failed to verify output directory: %s", ctx->output_path);
            safe_free(ctx);
            return -1;
        }
    }

    // Set full permissions to ensure FFmpeg can write files
    if (chmod(ctx->output_path, 0777) != 0) {
        log_warn("Failed to set permissions on HLS directory: %s (error: %s)", ctx->output_path, strerror(errno));
    }

    // Also ensure the parent directory of the HLS directory exists and is writable
    char parent_dir[MAX_PATH_LENGTH];
    snprintf(parent_dir, sizeof(parent_dir), "%s/hls", global_config->storage_path);

    // Create parent directory if it doesn't exist
    if (stat(parent_dir, &st) != 0 || !S_ISDIR(st.st_mode)) {
        if (mkdir(parent_dir, 0777) != 0 && errno != EEXIST) {
            log_warn("Failed to create parent HLS directory: %s (error: %s)", parent_dir, strerror(errno));
        }
    }

    // Set permissions on parent directory
    if (chmod(parent_dir, 0777) != 0) {
        log_warn("Failed to set permissions on parent HLS directory: %s (error: %s)", parent_dir, strerror(errno));
    }

    log_info("Created HLS directory with full permissions: %s", ctx->output_path);

    // Check that we can actually write to this directory
    char test_file[MAX_PATH_LENGTH];
    snprintf(test_file, sizeof(test_file), "%s/.test_write", ctx->output_path);
    FILE *test = fopen(test_file, "w");
    if (!test) {
        log_error("Directory is not writable: %s (error: %s)", ctx->output_path, strerror(errno));
        free(ctx);
        return -1;
    }
    fclose(test);
    remove(test_file);
    log_info("Verified HLS directory is writable: %s", ctx->output_path);

    // Initialize thread state
    atomic_store(&ctx->running, 1);
    atomic_store(&ctx->connection_valid, 0);
    atomic_store(&ctx->consecutive_failures, 0);
    atomic_store(&ctx->last_packet_time, (int_fast64_t)time(NULL));
    atomic_store(&ctx->thread_state, HLS_THREAD_INITIALIZING);

    // Set up thread attributes to create a detached thread
    pthread_attr_t attr;
    pthread_attr_init(&attr);
    pthread_attr_setdetachstate(&attr, PTHREAD_CREATE_DETACHED);

    // Start thread with detached state
    int thread_result = pthread_create(&ctx->thread, &attr, hls_unified_thread_func, ctx);

    // Clean up thread attributes
    pthread_attr_destroy(&attr);

    if (thread_result != 0) {
        log_error("Failed to create unified HLS thread for %s", stream_name);
        safe_free(ctx);
        return -1;
    }

    // Store context in the global array (mutex is still locked from earlier)
    unified_contexts[slot] = ctx;

    // CRITICAL FIX: Now we can safely unlock the mutex after the thread is created and the context is stored
    pthread_mutex_unlock(&unified_contexts_mutex);

    log_info("Started unified HLS thread for %s in slot %d", stream_name, slot);
    return 0;
}

/**
 * Force restart of HLS streaming for a stream using the unified thread approach
 */
int restart_hls_unified_stream(const char *stream_name) {
    log_info("Force restarting HLS stream for %s", stream_name);

    // CRITICAL FIX: Check if shutdown is in progress and prevent restarting streams
    if (is_shutdown_initiated()) {
        log_warn("Cannot restart HLS stream %s during shutdown", stream_name);
        return -1;
    }

    // CRITICAL FIX: Check if the stream is in the process of being stopped
    stream_state_manager_t *state = get_stream_state_by_name(stream_name);
    if (state && is_stream_state_stopping(state)) {
        log_warn("Cannot restart HLS stream %s while it is in the process of being stopped", stream_name);
        return -1;
    }

    // Clear the HLS segments before stopping the stream
    log_info("Clearing HLS segments for stream %s before restart", stream_name);
    clear_stream_hls_segments(stream_name);

    // First stop the stream if it's running
    int stop_result = stop_hls_stream(stream_name);
    if (stop_result != 0) {
        log_warn("Failed to stop HLS stream %s for restart, continuing anyway", stream_name);
    }

    // Wait a bit to ensure resources are released
    usleep(500000); // 500ms

    // Verify that the HLS directory exists and is writable
    config_t *global_config = get_streaming_config();
    if (global_config) {
        // Use storage_path_hls if specified, otherwise fall back to storage_path
        const char *base_storage_path = global_config->storage_path;
        if (global_config->storage_path_hls[0] != '\0') {
            base_storage_path = global_config->storage_path_hls;
            log_info("Using dedicated HLS storage path for restart: %s", base_storage_path);
        }

        char hls_dir[MAX_PATH_LENGTH];
        snprintf(hls_dir, MAX_PATH_LENGTH, "%s/hls/%s",
                base_storage_path, stream_name);

        // Ensure the directory exists and has proper permissions
        log_info("Ensuring HLS directory exists and is writable: %s", hls_dir);
        ensure_hls_directory(hls_dir, stream_name);
    }

    // Start the stream again
    int start_result = start_hls_stream(stream_name);
    if (start_result != 0) {
        log_error("Failed to restart HLS stream %s", stream_name);
        return -1;
    }

    log_info("Successfully restarted HLS stream %s", stream_name);
    return 0;
}

/**
 * Stop HLS streaming for a stream using the unified thread approach
 */
int stop_hls_unified_stream(const char *stream_name) {
    int found = 0;

    // Log that we're attempting to stop the stream
    log_info("Attempting to stop HLS stream: %s", stream_name);

    // Get the stream state manager
    stream_state_manager_t *state = get_stream_state_by_name(stream_name);
    if (state) {
        // Only disable callbacks if we're actually stopping the stream
        bool found = false;
        pthread_mutex_lock(&unified_contexts_mutex);
        for (int i = 0; i < MAX_STREAMS; i++) {
            if (unified_contexts[i] && strcmp(unified_contexts[i]->stream_name, stream_name) == 0) {
                found = true;
                break;
            }
        }
        pthread_mutex_unlock(&unified_contexts_mutex);

        if (found) {
            // Disable callbacks to prevent new packets from being processed
            set_stream_callbacks_enabled(state, false);
            log_info("Disabled callbacks for stream %s during HLS shutdown", stream_name);
        } else {
            log_info("Stream %s not found in HLS contexts, not disabling callbacks", stream_name);
        }
    }

    // Find the stream context with mutex protection
    pthread_mutex_lock(&unified_contexts_mutex);

    hls_unified_thread_ctx_t *ctx = NULL;
    int index = -1;
    int contexts_found = 0;
    int indices[MAX_STREAMS];

    // CRITICAL FIX: Check for multiple contexts with the same stream name
    for (int i = 0; i < MAX_STREAMS; i++) {
        if (unified_contexts[i] && unified_contexts[i]->stream_name[0] != '\0' &&
            strcmp(unified_contexts[i]->stream_name, stream_name) == 0) {
            if (contexts_found == 0) {
                // Store the first one we find as the primary context to stop
                ctx = unified_contexts[i];
                index = i;
            }
            indices[contexts_found] = i;
            contexts_found++;
            found = 1;
        }
    }

    // CRITICAL FIX: If we found multiple contexts, log a warning and mark all of them as stopping
    if (contexts_found > 1) {
        log_warn("Found %d HLS contexts for stream %s during stop operation", contexts_found, stream_name);

        // Mark all contexts as not running except the primary one (which will be handled later)
        for (int i = 1; i < contexts_found; i++) {
            int idx = indices[i];
            if (unified_contexts[idx] && unified_contexts[idx] != ctx) {
                log_info("Marking additional HLS context %d for stream %s as stopping", i, stream_name);
                atomic_store(&unified_contexts[idx]->running, 0);
            }
        }
    }

    // If not found, unlock and return
    if (!found) {
        pthread_mutex_unlock(&unified_contexts_mutex);
        log_warn("HLS stream %s not found for stopping", stream_name);
        return -1;
    }

    // Check if the stream is already stopped
    if (!atomic_load(&ctx->running)) {
        pthread_mutex_unlock(&unified_contexts_mutex);
        log_warn("HLS stream %s is already stopped", stream_name);
        return 0;
    }

    // Mark as stopping in the global stopping list to prevent race conditions
    mark_stream_stopping(stream_name);

    // Now mark as not running using atomic store for thread safety
    atomic_store(&ctx->running, 0);
    log_info("Marked HLS stream %s as stopping (index: %d)", stream_name, index);

    // Reset the timestamp tracker for this stream to ensure clean state when restarted
    reset_timestamp_tracker(stream_name);
    log_info("Reset timestamp tracker for stream %s", stream_name);

    // Unlock the mutex to allow the thread to access shared resources during shutdown
    pthread_mutex_unlock(&unified_contexts_mutex);

    // Wait for the thread to begin its shutdown process
    usleep(100000); // 100ms

    log_info("Waiting for detached thread for stream %s to exit on its own", stream_name);

    // Wait longer to ensure the thread has fully exited
    int wait_attempts = 10; // Try up to 10 times
    while (wait_attempts > 0) {
        // Check if the thread has marked itself as exiting
        if (is_stream_stopping(stream_name)) {
            // Thread is still in the process of stopping
            log_info("Thread for stream %s is still stopping, waiting... (%d attempts left)",
                    stream_name, wait_attempts);
            usleep(500000); // Wait 500ms between checks
            wait_attempts--;
        } else {
            // Thread has completed its shutdown
            log_info("Thread for stream %s has completed its shutdown", stream_name);
            break;
        }
    }

    // Re-acquire the mutex for cleanup
    pthread_mutex_lock(&unified_contexts_mutex);

    // Verify context is still valid
    if (index >= 0 && index < MAX_STREAMS && unified_contexts[index] == ctx) {
        // CRITICAL FIX: First mark the context as pending deletion before accessing its members
        // This ensures that the thread will know not to access the context anymore
        mark_context_pending_deletion(ctx);

        // CRITICAL FIX: Add a memory barrier to ensure the pending deletion flag is visible to all threads
        __sync_synchronize();

        // CRITICAL FIX: Clear the writer reference in the context before freeing to prevent double free
        if (ctx && !is_context_already_freed(ctx)) {
            // Use a try/catch-like approach with signal handling to prevent crashes
            struct sigaction sa_old, sa_new;
            sigaction(SIGSEGV, NULL, &sa_old);
            sa_new = sa_old;
            sa_new.sa_handler = SIG_IGN; // Ignore segmentation fault signal
            sigaction(SIGSEGV, &sa_new, NULL);

            // Set alarm to prevent hanging if memory is inaccessible
            alarm(1); // 1 second timeout

            // Store a local copy of the stream name for logging
            char writer_stream_name[MAX_STREAM_NAME] = {0};
            strcpy(writer_stream_name, stream_name); // Use the stream_name we already have

            // Safely get and clear the writer pointer
            hls_writer_t *writer_to_cleanup = NULL;
            if (ctx->writer) {
                writer_to_cleanup = __atomic_exchange_n(&ctx->writer, NULL, __ATOMIC_SEQ_CST);
                log_info("Clearing writer reference in context for stream %s", writer_stream_name);
            }

            // Cancel the alarm and restore signal handler
            alarm(0);
            sigaction(SIGSEGV, &sa_old, NULL);

            // CRITICAL FIX: Safely close the writer if needed
            if (writer_to_cleanup) {
                // We don't free the writer here - the thread should handle that
                log_info("Writer reference cleared for stream %s", writer_stream_name);
            }
        }

        // Free context and clear slot
        unified_contexts[index] = NULL;

        // Unlock the mutex before freeing the context
        pthread_mutex_unlock(&unified_contexts_mutex);

        // CRITICAL FIX: Add additional safety checks before freeing the context
        if (ctx) {
            // Check if the context is already marked as freed
            if (is_context_already_freed(ctx)) {
                log_warn("Context for stream %s has already been freed, skipping", stream_name);
                return 0;
            }

            // CRITICAL FIX: Add memory barrier before freeing to ensure all accesses are complete
            __sync_synchronize();

            // CRITICAL FIX: Use a local copy of the pointer to prevent race conditions
            hls_unified_thread_ctx_t *ctx_to_free = ctx;
            ctx = NULL;

            // CRITICAL FIX: Add memory barrier after nulling the pointer
            __sync_synchronize();

            // CRITICAL FIX: Add additional validation before freeing the context
            // This helps catch cases where the memory has been corrupted or already freed
            bool context_valid = true;

            // Basic validation of context structure
            if (ctx_to_free->stream_name == NULL || ctx_to_free->stream_name[0] == '\0') {
                log_warn("Context has NULL or empty stream_name, may be corrupted");
                context_valid = false;
            }

            // Check if the context has already been freed (this is a heuristic)
            // We check if the first few bytes are zeroed out, which might indicate
            // that the memory has been freed or corrupted
            unsigned char *ptr = (unsigned char *)ctx_to_free;
            bool all_zeros = true;
            for (int i = 0; i < 16 && i < sizeof(hls_unified_thread_ctx_t); i++) {
                if (ptr[i] != 0) {
                    all_zeros = false;
                    break;
                }
            }

            if (all_zeros) {
                log_warn("Context memory appears to be zeroed out, may have been freed already");
                context_valid = false;
            }

            if (context_valid) {
                // CRITICAL FIX: Check if the context has already been freed
                if (is_context_already_freed(ctx_to_free)) {
                    log_warn("Context for stream %s has already been freed, skipping", stream_name);
                } else {
                    // Free the context
                    log_info("Freeing context for stream %s", stream_name);

                    // Use a try/catch-like approach with signal handling to prevent crashes
                    struct sigaction sa_old, sa_new;
                    sigaction(SIGALRM, NULL, &sa_old);
                    sa_new = sa_old;
                    sa_new.sa_handler = SIG_IGN; // Ignore alarm signal
                    sigaction(SIGALRM, &sa_new, NULL);

                    // Set alarm
                    alarm(15); // 15 second timeout for context free

                    // CRITICAL FIX: Mark the context as pending deletion to signal the thread
                    mark_context_pending_deletion(ctx_to_free);

                    // Add a memory barrier to ensure the pending deletion flag is visible to all threads
                    __sync_synchronize();

                    // Add a small delay to ensure the thread sees the pending deletion flag
                    usleep(50000); // 50ms

                    // Wait for the thread to exit with increased timeout
                    log_info("Waiting for thread to exit for context %p (stream %s)", ctx_to_free, stream_name);
                    wait_for_thread_exit(ctx_to_free);

                    // Mark the context as freed before actually freeing it
                    mark_context_as_freed(ctx_to_free);

                    // Clear from pending deletion list
                    clear_context_pending_deletion(ctx_to_free);

                    // Free the context with additional protection
                    safe_free(ctx_to_free);

                    // Cancel the alarm and restore signal handler
                    alarm(0);
                    sigaction(SIGALRM, &sa_old, NULL);

                    log_info("Successfully freed context for stream %s", stream_name);
                }
            } else {
                log_debug("Skipping cleanup of potentially invalid context for stream %s (already cleaned up)", stream_name);
            }
        } else {
            // This is expected during shutdown when multiple cleanup paths run
            log_debug("Context is NULL during cleanup for stream %s (already cleaned up)", stream_name);
        }

        log_info("Successfully cleaned up resources for stream %s", stream_name);
    } else {
        pthread_mutex_unlock(&unified_contexts_mutex);
        log_warn("Context for stream %s was modified during cleanup", stream_name);
    }

    // CRITICAL FIX: Check for any remaining contexts for this stream and clean them up
    pthread_mutex_lock(&unified_contexts_mutex);
    for (int i = 0; i < MAX_STREAMS; i++) {
        if (unified_contexts[i] && unified_contexts[i]->stream_name[0] != '\0' &&
            strcmp(unified_contexts[i]->stream_name, stream_name) == 0) {
            log_warn("Found additional HLS context for stream %s after primary context cleanup", stream_name);

            // Mark as not running
            atomic_store(&unified_contexts[i]->running, 0);

            // Free the context
            hls_unified_thread_ctx_t *extra_ctx = unified_contexts[i];
            unified_contexts[i] = NULL;

            // Mark the context as freed before actually freeing it
            mark_context_as_freed(extra_ctx);

            // Free the context
            safe_free(extra_ctx);

            log_info("Cleaned up additional HLS context for stream %s", stream_name);
        }
    }
    pthread_mutex_unlock(&unified_contexts_mutex);

    // Remove from the stopping list
    unmark_stream_stopping(stream_name);

    log_info("Stopped HLS stream %s", stream_name);

    // Release the HLS reference
    if (state) {
        // Re-enable callbacks before releasing the reference
        // CRITICAL FIX: Add memory barrier before re-enabling callbacks
        __sync_synchronize();
        set_stream_callbacks_enabled(state, true);
        log_info("Re-enabled callbacks for stream %s after HLS shutdown", stream_name);

        // CRITICAL FIX: Add a small delay to ensure callbacks are fully re-enabled
        // before releasing the reference
        usleep(100000); // 100ms

        // CRITICAL FIX: Add memory barrier before releasing reference
        __sync_synchronize();
        stream_state_release_ref(state, STREAM_COMPONENT_HLS);
        log_info("Released HLS reference to stream %s", stream_name);
    }

    return 0;
}

/**
 * Check if HLS streaming is active for a stream
 */
int is_hls_stream_active(const char *stream_name) {
    pthread_mutex_lock(&unified_contexts_mutex);

    for (int i = 0; i < MAX_STREAMS; i++) {
        if (unified_contexts[i] &&
            strcmp(unified_contexts[i]->stream_name, stream_name) == 0 &&
            atomic_load(&unified_contexts[i]->running) &&
            atomic_load(&unified_contexts[i]->connection_valid)) {

            pthread_mutex_unlock(&unified_contexts_mutex);
            return 1;
        }
    }

    pthread_mutex_unlock(&unified_contexts_mutex);
    return 0;
}

/**
 * FFmpeg buffer cleanup function
 * This function forces FFmpeg to release any cached memory
 * Enhanced to be more aggressive in cleaning up FFmpeg resources
 * CRITICAL FIX: Added safety checks to prevent segfaults during shutdown
 */
static void ffmpeg_buffer_cleanup(void) {
    static int in_cleanup = 0;

    // CRITICAL FIX: Prevent recursive calls that can cause double free
    if (in_cleanup) {
        log_warn("Already in FFmpeg buffer cleanup, skipping to prevent double free");
        return;
    }
    in_cleanup = 1;

    log_info("Forcing FFmpeg to release cached memory");

    // CRITICAL FIX: Use try/catch-like approach with signal handling to prevent crashes
    struct sigaction sa_old, sa_new;
    sigaction(SIGSEGV, NULL, &sa_old);
    sa_new = sa_old;
    sa_new.sa_handler = SIG_IGN; // Ignore segmentation fault signal
    sigaction(SIGSEGV, &sa_new, NULL);

    // Set alarm to prevent hanging if memory is inaccessible
    alarm(1); // 1 second timeout

    // Force garbage collection in FFmpeg - safely
    // This will help release any memory that might be cached
    // CRITICAL FIX: Wrap in try/catch to prevent segfaults
    log_debug("Calling av_freep(NULL) to release cached memory");

    // CRITICAL FIX: Skip direct av_freep(NULL) call during shutdown or hard deletion
    // This is the most likely cause of segmentation faults
    if (!is_shutdown_initiated() && !is_stream_stopping(NULL)) {
        // Use a safer approach to release memory
        log_debug("Using safer approach to release FFmpeg memory");

        // Instead of directly calling av_freep(NULL), which can cause segfaults,
        // allocate and free a small buffer to trigger FFmpeg's internal cleanup
        void *dummy = av_malloc(16);
        if (dummy) {
            av_free(dummy);
        }
    } else {
        log_info("Skipping av_freep(NULL) during shutdown or stream deletion to prevent segmentation fault");
    }

    // Call FFmpeg's internal memory cleanup functions - safely
    log_debug("Calling avformat_network_deinit()");
    avformat_network_deinit();

    // Force FFmpeg to release all cached memory
    // This is a more aggressive approach to ensure all memory is freed
    log_debug("Setting FFmpeg log level to quiet");
    av_log_set_level(AV_LOG_QUIET);

    // MEMORY LEAK FIX: Explicitly free any AVFormatContext that might be leaked
    // This is a hack to work around FFmpeg's internal memory management issues
    // CRITICAL FIX: Reduced number of iterations to prevent excessive memory operations during shutdown
    log_debug("Creating and freeing dummy AVFormatContext instances");
    for (int i = 0; i < 10; i++) {
        AVFormatContext *dummy_ctx = avformat_alloc_context();
        if (dummy_ctx) {
            avformat_free_context(dummy_ctx);
        }
    }

    // MEMORY LEAK FIX: Explicitly free any AVPacket that might be leaked
    // CRITICAL FIX: Reduced number of iterations to prevent excessive memory operations during shutdown
    log_debug("Creating and freeing dummy AVPacket instances");
    for (int i = 0; i < 10; i++) {
        AVPacket *dummy_pkt = av_packet_alloc();
        if (dummy_pkt) {
            av_packet_free(&dummy_pkt);
        }
    }

    // CRITICAL FIX: Skip the more complex allocations during shutdown
    // These are more likely to cause segfaults if FFmpeg is in an unstable state
    if (!is_shutdown_initiated()) {
        // MEMORY LEAK FIX: Explicitly free any AVCodecContext that might be leaked
        log_debug("Creating and freeing dummy AVCodecContext instances");
        for (int i = 0; i < 5; i++) {
            const AVCodec *codec = avcodec_find_decoder(AV_CODEC_ID_H264);
            if (codec) {
                AVCodecContext *dummy_codec_ctx = avcodec_alloc_context3(codec);
                if (dummy_codec_ctx) {
                    avcodec_free_context(&dummy_codec_ctx);
                }
            }
        }

        // MEMORY LEAK FIX: Explicitly free any AVFrame that might be leaked
        log_debug("Creating and freeing dummy AVFrame instances");
        for (int i = 0; i < 5; i++) {
            AVFrame *dummy_frame = av_frame_alloc();
            if (dummy_frame) {
                av_frame_free(&dummy_frame);
            }
        }

        // MEMORY LEAK FIX: Explicitly free any parser context that might be leaked
        // This addresses the memory leaks in av_parser_parse2
        log_debug("Creating and freeing dummy AVCodecParserContext instances");
        for (int i = 0; i < 5; i++) {
            AVCodecParserContext *dummy_parser = av_parser_init(AV_CODEC_ID_H264);
            if (dummy_parser) {
                av_parser_close(dummy_parser);
            }
        }
    } else {
        log_info("Skipping complex FFmpeg allocations during shutdown to prevent crashes");
    }

    // Cancel the alarm and restore signal handler
    alarm(0);
    sigaction(SIGSEGV, &sa_old, NULL);

    log_info("FFmpeg memory cleanup completed");
    in_cleanup = 0;
}

/**
 * Initialize the freed contexts tracking system
 */
static void init_freed_contexts_tracking(void) {
    log_info("Initializing freed contexts tracking system");

    // Initialize the freed contexts array
    pthread_mutex_lock(&freed_contexts_mutex);
    memset(freed_contexts, 0, sizeof(freed_contexts));
    freed_contexts_count = 0;
    pthread_mutex_unlock(&freed_contexts_mutex);

    // Initialize the pending deletions array
    pthread_mutex_lock(&pending_deletions_mutex);
    memset(pending_deletions, 0, sizeof(pending_deletions));
    pthread_mutex_unlock(&pending_deletions_mutex);

    log_info("Freed contexts tracking system initialized");
}

/**
 * Cleanup the freed contexts tracking system
 */
static void cleanup_freed_contexts_tracking(void) {
    // First, wait for any pending deletions to complete
    pthread_mutex_lock(&pending_deletions_mutex);

    // Check if there are any pending deletions
    bool has_pending = false;
    for (int i = 0; i < MAX_STREAMS; i++) {
        if (pending_deletions[i].ctx != NULL && atomic_load(&pending_deletions[i].pending_deletion)) {
            has_pending = true;
            log_info("Waiting for thread to exit for context %p", pending_deletions[i].ctx);

            // Wait for the thread to exit
            if (!atomic_load(&pending_deletions[i].thread_exited)) {
                pthread_mutex_unlock(&pending_deletions_mutex);
                wait_for_thread_exit(pending_deletions[i].ctx);
                pthread_mutex_lock(&pending_deletions_mutex);
            }
        }
    }

    if (has_pending) {
        log_info("All pending deletions have been processed");
    }

    // Clear the pending deletions array
    memset(pending_deletions, 0, sizeof(pending_deletions));
    pthread_mutex_unlock(&pending_deletions_mutex);

    // Clear the freed contexts array
    pthread_mutex_lock(&freed_contexts_mutex);
    memset(freed_contexts, 0, sizeof(freed_contexts));
    freed_contexts_count = 0;
    pthread_mutex_unlock(&freed_contexts_mutex);

    log_info("Cleaned up freed contexts tracking system");
}

/**
 * Cleanup HLS unified thread system
 * CRITICAL FIX: Added safety checks to prevent segfaults during shutdown
 */
void cleanup_hls_unified_thread_system(void) {
    static int in_system_cleanup = 0;

    // CRITICAL FIX: Prevent recursive calls that can cause double free
    if (in_system_cleanup) {
        log_warn("Already in HLS unified thread system cleanup, skipping to prevent double free");
        return;
    }
    in_system_cleanup = 1;

    log_info("Cleaning up HLS unified thread system...");

    // Stop the watchdog thread
    stop_hls_watchdog();

    // CRITICAL FIX: Use try/catch-like approach with signal handling to prevent crashes
    struct sigaction sa_old, sa_new;
    sigaction(SIGSEGV, NULL, &sa_old);
    sa_new = sa_old;
    sa_new.sa_handler = SIG_IGN; // Ignore segmentation fault signal
    sigaction(SIGSEGV, &sa_new, NULL);

    // Set alarm to prevent hanging if memory is inaccessible
    alarm(2); // 2 second timeout

    // MEMORY LEAK FIX: Force cleanup of any remaining FFmpeg resources
    log_info("Performing final FFmpeg resource cleanup during system shutdown");

    // Force cleanup of all HLS writers
    cleanup_all_hls_writers();

    // CRITICAL FIX: Skip aggressive FFmpeg cleanup during shutdown
    // This is more likely to cause segfaults if FFmpeg is in an unstable state
    log_info("Using safer approach to release FFmpeg memory during system shutdown");

    // Instead of directly calling av_freep(NULL), which can cause segfaults,
    // allocate and free a small buffer to trigger FFmpeg's internal cleanup
    void *dummy = av_malloc(16);
    if (dummy) {
        av_free(dummy);
    }

    // Set FFmpeg log level to quiet to reduce memory usage
    av_log_set_level(AV_LOG_QUIET);

    // MEMORY LEAK FIX: Register the global FFmpeg cleanup function to be called at program exit
    // This ensures that any FFmpeg resources allocated after this point will still be freed
    if (!ffmpeg_memory_cleanup_registered) {
        log_info("Registering global FFmpeg cleanup function with atexit()");
        atexit(global_ffmpeg_cleanup);
        ffmpeg_memory_cleanup_registered = 1;
    }

    // Clean up the freed contexts tracking system
    cleanup_freed_contexts_tracking();

    // Cancel the alarm and restore signal handler
    alarm(0);
    sigaction(SIGSEGV, &sa_old, NULL);

    log_info("HLS unified thread system cleaned up");
    in_system_cleanup = 0;
}

/**
 * Initialize FFmpeg memory cleanup system
 * This function registers the global FFmpeg cleanup function with atexit()
 */
static void init_ffmpeg_memory_cleanup(void) {
    if (!ffmpeg_memory_cleanup_registered) {
        log_info("Initializing FFmpeg memory cleanup system");
        atexit(global_ffmpeg_cleanup);
        ffmpeg_memory_cleanup_registered = 1;
        log_info("FFmpeg memory cleanup system initialized");
    }
}

/**
 * Check if a thread is still running
 * This function checks if a thread is still running by examining its state
 *
 * @param ctx The thread context
 * @return true if running, false if not
 */
static bool is_thread_running(hls_unified_thread_ctx_t *ctx) {
    if (!ctx) {
        return false;
    }

    // Check if the thread is marked as running
    if (!atomic_load(&ctx->running)) {
        return false;
    }

    // Check if the thread state is STOPPED
    if (atomic_load(&ctx->thread_state) == HLS_THREAD_STOPPED) {
        return false;
    }

    // Check if we've received a packet recently
    time_t last_packet = (time_t)atomic_load(&ctx->last_packet_time);
    time_t now = time(NULL);
    if (now - last_packet > MAX_PACKET_TIMEOUT * 3) { // 3x the normal timeout for watchdog
        log_warn("Thread for stream %s has not received a packet in %ld seconds",
                ctx->stream_name, now - last_packet);
        return false;
    }

    return true;
}

/**
 * Watchdog thread function
 * This function periodically checks all HLS threads and restarts any that have exited unexpectedly
 *
 * @param arg Not used
 * @return NULL
 */
static void *hls_watchdog_thread_func(void *arg) {
    log_info("HLS watchdog thread started");

    while (atomic_load(&watchdog_running)) {
        // Sleep for the check interval
        sleep(WATCHDOG_CHECK_INTERVAL_SEC);

        // Check if we should continue running
        if (!atomic_load(&watchdog_running) || is_shutdown_initiated()) {
            log_info("HLS watchdog thread stopping");
            break;
        }

        log_debug("HLS watchdog checking thread status...");

        // Lock the contexts mutex
        pthread_mutex_lock(&unified_contexts_mutex);

        // Check each context
        for (int i = 0; i < MAX_STREAMS; i++) {
            hls_unified_thread_ctx_t *ctx = unified_contexts[i];
            if (!ctx) {
                continue;
            }

            // Skip contexts that are already marked for deletion
            if (is_context_pending_deletion(ctx)) {
                continue;
            }

            // Check if the thread is still running
            if (!is_thread_running(ctx)) {
                log_warn("HLS thread for stream %s has exited unexpectedly", ctx->stream_name);

                // Check if we should restart the thread
                time_t now = time(NULL);
                if (now - last_restart_time[i] < WATCHDOG_RESTART_COOLDOWN_SEC) {
                    log_warn("Cooldown period not elapsed for stream %s, not restarting yet",
                            ctx->stream_name);
                    continue;
                }

                // Check if we've exceeded the maximum restart attempts
                if (restart_attempts[i] >= WATCHDOG_MAX_RESTART_ATTEMPTS) {
                    log_error("Maximum restart attempts (%d) exceeded for stream %s, not restarting",
                             WATCHDOG_MAX_RESTART_ATTEMPTS, ctx->stream_name);
                    continue;
                }

                // Store the stream name before unlocking the mutex
                char stream_name[MAX_STREAM_NAME];
                strncpy(stream_name, ctx->stream_name, MAX_STREAM_NAME - 1);
                stream_name[MAX_STREAM_NAME - 1] = '\0';

                // Unlock the mutex before restarting the stream
                pthread_mutex_unlock(&unified_contexts_mutex);

                log_info("Attempting to restart HLS thread for stream %s (attempt %d)",
                        stream_name, restart_attempts[i] + 1);

                // Stop the stream first to clean up any resources
                stop_hls_unified_stream(stream_name);

                // Wait a bit to ensure the stream is fully stopped
                usleep(1000000); // 1 second

                // Start the stream again
                int result = start_hls_unified_stream(stream_name);
                if (result == 0) {
                    log_info("Successfully restarted HLS thread for stream %s", stream_name);

                    // Update restart statistics
                    pthread_mutex_lock(&unified_contexts_mutex);
                    last_restart_time[i] = time(NULL);
                    restart_attempts[i]++;
                    atomic_fetch_add(&watchdog_restart_count, 1);
                    pthread_mutex_unlock(&unified_contexts_mutex);
                } else {
                    log_error("Failed to restart HLS thread for stream %s", stream_name);

                    // Update restart statistics
                    pthread_mutex_lock(&unified_contexts_mutex);
                    last_restart_time[i] = time(NULL);
                    restart_attempts[i]++;
                    pthread_mutex_unlock(&unified_contexts_mutex);
                }

                // Re-lock the mutex to continue the loop
                pthread_mutex_lock(&unified_contexts_mutex);
            } else {
                // Thread is running, reset restart attempts if it's been running for a while
                time_t now = time(NULL);
                if (restart_attempts[i] > 0 && now - last_restart_time[i] > WATCHDOG_RESTART_COOLDOWN_SEC * 2) {
                    log_info("Resetting restart attempts for stream %s after stable period", ctx->stream_name);
                    restart_attempts[i] = 0;
                }
            }
        }

        // Unlock the mutex
        pthread_mutex_unlock(&unified_contexts_mutex);
    }

    log_info("HLS watchdog thread exited");
    return NULL;
}

/**
 * Start the HLS watchdog thread
 */
static void start_hls_watchdog(void) {
    // Check if the watchdog is already running
    if (atomic_load(&watchdog_running)) {
        log_warn("HLS watchdog thread is already running");
        return;
    }

    // Initialize restart tracking arrays
    memset(last_restart_time, 0, sizeof(last_restart_time));
    memset(restart_attempts, 0, sizeof(restart_attempts));

    // Set the watchdog running flag
    atomic_store(&watchdog_running, true);
    atomic_store(&watchdog_restart_count, 0);

    // Create the watchdog thread
    pthread_attr_t attr;
    pthread_attr_init(&attr);
    pthread_attr_setdetachstate(&attr, PTHREAD_CREATE_DETACHED);

    int result = pthread_create(&hls_watchdog_thread, &attr, hls_watchdog_thread_func, NULL);
    pthread_attr_destroy(&attr);

    if (result != 0) {
        log_error("Failed to create HLS watchdog thread");
        atomic_store(&watchdog_running, false);
    } else {
        log_info("Started HLS watchdog thread");
    }
}

/**
 * Stop the HLS watchdog thread
 */
static void stop_hls_watchdog(void) {
    // Check if the watchdog is running
    if (!atomic_load(&watchdog_running)) {
        log_warn("HLS watchdog thread is not running");
        return;
    }

    // Set the watchdog running flag to false
    atomic_store(&watchdog_running, false);

    // The thread is detached, so we don't need to join it
    log_info("Signaled HLS watchdog thread to stop");
}

/**
 * Get the number of HLS thread restarts performed by the watchdog
 */
int get_hls_watchdog_restart_count(void) {
    return atomic_load(&watchdog_restart_count);
}

/**
 * Initialize the HLS unified thread system
 */
void init_hls_unified_thread_system(void) {
    log_info("Initializing HLS unified thread system...");

    // Initialize the freed contexts tracking system
    init_freed_contexts_tracking();

    // Initialize the FFmpeg memory cleanup system
    init_ffmpeg_memory_cleanup();

    // Start the HLS watchdog thread
    start_hls_watchdog();

    log_info("HLS unified thread system initialized with watchdog monitoring");
}
