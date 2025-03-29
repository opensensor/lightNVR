#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <pthread.h>
#include <sys/stat.h>
#include <sys/types.h>
#include <fcntl.h>
#include <errno.h>
#include <time.h>
#include <stdbool.h>
#include <sys/time.h>
#include <stdatomic.h>
#include <math.h>

#include "core/logger.h"
#include "core/config.h"
#include "core/shutdown_coordinator.h"
#include "video/stream_manager.h"
#include "video/stream_state.h"
#include "video/streams.h"
#include "video/hls_writer.h"
#include "video/hls_writer_thread.h"
#include "video/hls_streaming.h"
#include "video/thread_utils.h"
#include "video/timestamp_manager.h"
#include "video/hls/hls_context.h"
#include "video/hls/hls_stream_thread.h"
#include "video/hls/hls_directory.h"

// Define retry parameters
#define INITIAL_RETRY_DELAY_SEC 1
#define MAX_RETRY_DELAY_SEC 60

/**
 * HLS streaming thread function for a single stream
 * This is now a manager of hls_writer threads rather than doing the HLS writing itself
 */
void *hls_stream_thread(void *arg) {
    hls_stream_ctx_t *ctx = (hls_stream_ctx_t *)arg;
    int ret;

    // Add extra validation for context
    if (!ctx) {
        log_error("NULL context passed to HLS streaming thread");
        return NULL;
    }

    // Create a local copy of the stream name for thread safety
    char stream_name[MAX_STREAM_NAME];
    strncpy(stream_name, ctx->config.name, MAX_STREAM_NAME - 1);
    stream_name[MAX_STREAM_NAME - 1] = '\0';

    // Get the stream state manager
    stream_state_manager_t *state = get_stream_state_by_name(stream_name);
    if (!state) {
        log_error("Could not find stream state for %s", stream_name);
        atomic_store(&ctx->running, 0);
        return NULL;
    }

    log_info("Starting HLS streaming manager thread for stream %s", stream_name);

    // Check if we're still running before proceeding using atomic load
    if (!atomic_load(&ctx->running)) {
        log_warn("HLS streaming thread for %s started but already marked as not running", stream_name);
        return NULL;
    }

    // Verify output directory exists and is writable
    if (ensure_hls_directory(ctx->output_path, stream_name) != 0) {
        log_error("Failed to ensure HLS output directory: %s", ctx->output_path);
        atomic_store(&ctx->running, 0);
        return NULL;
    }

    // Check if we're still running after directory creation
    if (!atomic_load(&ctx->running)) {
        log_info("HLS streaming thread for %s stopping after directory creation", stream_name);
        return NULL;
    }

    // Create HLS writer - adding the segment_duration parameter
    // Use a smaller segment duration for lower latency
    int segment_duration = ctx->config.segment_duration > 0 ?
                          ctx->config.segment_duration : 0.5;

    ctx->hls_writer = hls_writer_create(ctx->output_path, stream_name, segment_duration);
    if (!ctx->hls_writer) {
        log_error("Failed to create HLS writer for %s", stream_name);
        atomic_store(&ctx->running, 0);
        return NULL;
    }

    // Register with shutdown coordinator
    char component_name[128];
    snprintf(component_name, sizeof(component_name), "hls_manager_%s", stream_name);
    int component_id = register_component(component_name, COMPONENT_HLS_WRITER, ctx, 60); // Lowest priority (60)
    if (component_id >= 0) {
        log_info("Registered HLS manager %s with shutdown coordinator (ID: %d)", stream_name, component_id);
    }

    // Check if we're still running after HLS writer creation
    if (!atomic_load(&ctx->running) || is_shutdown_initiated()) {
        log_info("HLS streaming thread for %s stopping after HLS writer creation", stream_name);
        if (ctx->hls_writer) {
            log_info("Closing HLS writer during early shutdown for %s", stream_name);
            hls_writer_close(ctx->hls_writer);
            ctx->hls_writer = NULL;
        }
        
        // Update component state if registered
        if (component_id >= 0) {
            update_component_state(component_id, COMPONENT_STOPPED);
            log_info("Updated HLS manager %s state to STOPPED during early shutdown", stream_name);
        }
        
        return NULL;
    }

    // Get stream configuration
    stream_handle_t stream = get_stream_by_name(stream_name);
    if (!stream) {
        log_error("Stream %s not found", stream_name);

        if (ctx->hls_writer) {
            hls_writer_close(ctx->hls_writer);
            ctx->hls_writer = NULL;
        }

        atomic_store(&ctx->running, 0);
        return NULL;
    }

    stream_config_t config;
    if (get_stream_config(stream, &config) != 0) {
        log_error("Failed to get config for stream %s", stream_name);

        if (ctx->hls_writer) {
            hls_writer_close(ctx->hls_writer);
            ctx->hls_writer = NULL;
        }

        atomic_store(&ctx->running, 0);
        return NULL;
    }

    // Start the HLS writer thread with retry mechanism
    log_info("Starting HLS writer thread for stream %s", stream_name);
    
    int retry_delay = INITIAL_RETRY_DELAY_SEC;
    int consecutive_failures = 0;
    bool connected = false;
    
    // Keep trying to connect until successful or thread is stopped
    while (atomic_load(&ctx->running) && !connected) {
        ret = hls_writer_start_recording_thread(ctx->hls_writer, config.url, stream_name, config.protocol);
        if (ret == 0) {
            // Log success at ERROR level for visibility
            log_error("Successfully started HLS writer thread for stream %s", stream_name);
            connected = true;
        } else {
            consecutive_failures++;
            // Log at ERROR level to ensure visibility regardless of log level setting
            log_error("Failed to start HLS writer thread for stream %s (attempt %d), retrying in %d seconds", 
                    stream_name, consecutive_failures, retry_delay);
            
            // Sleep for the retry delay
            for (int i = 0; i < retry_delay && atomic_load(&ctx->running); i++) {
                sleep(1);
                
                // Check if shutdown has been initiated during sleep
                if (is_shutdown_initiated()) {
                    log_info("HLS streaming thread for %s stopping due to system shutdown during retry", stream_name);
                    atomic_store(&ctx->running, 0);
                    
                    if (ctx->hls_writer) {
                        hls_writer_close(ctx->hls_writer);
                        ctx->hls_writer = NULL;
                    }
                    
                    if (component_id >= 0) {
                        update_component_state(component_id, COMPONENT_STOPPED);
                    }
                    
                    return NULL;
                }
            }
            
            // Implement exponential backoff with a maximum delay
            retry_delay = fmin(retry_delay * 2, MAX_RETRY_DELAY_SEC);
        }
    }
    
    // Reset for reconnection attempts
    retry_delay = INITIAL_RETRY_DELAY_SEC;
    consecutive_failures = 0;
    
    // Force an immediate check of the writer thread status
    // This helps detect issues with the initial connection faster
    if (!hls_writer_is_recording(ctx->hls_writer)) {
        log_error("Initial HLS writer thread for %s is not recording immediately after start, will attempt restart", stream_name);
        // Don't increment consecutive_failures yet, just trigger the reconnection logic below
    }

    // Main monitoring loop - just check if we should stop and monitor the writer thread
    while (atomic_load(&ctx->running)) {
        // Check if shutdown has been initiated
        if (is_shutdown_initiated()) {
            log_info("HLS streaming thread for %s stopping due to system shutdown", stream_name);
            atomic_store(&ctx->running, 0);
            break;
        }
        
        // Check if the stream state indicates we should stop
        if (is_stream_state_stopping(state)) {
            log_info("HLS streaming thread for %s stopping due to stream state STOPPING", stream_name);
            atomic_store(&ctx->running, 0);
            break;
        }

        if (!are_stream_callbacks_enabled(state)) {
            log_info("HLS streaming thread for %s stopping due to callbacks disabled", stream_name);
            atomic_store(&ctx->running, 0);
            break;
        }

            // Check if the writer thread is still running and has a valid connection
            // Check more frequently (every 1 second instead of waiting for the sleep at the end)
            if (!hls_writer_is_recording(ctx->hls_writer)) {
                // Log at ERROR level to ensure visibility regardless of log level setting
                log_error("HLS writer thread for %s is not recording (stopped, invalid connection, or timed out)", stream_name);
                
                // Check if the thread context is NULL, which means the thread has exited
                if (!ctx->hls_writer->thread_ctx) {
                    log_error("HLS writer thread for %s has exited, recreating thread context", stream_name);
                    
                    // The thread has exited, so we need to recreate it
                    // First, make sure the writer is still valid
                    if (!ctx->hls_writer) {
                        log_error("HLS writer for %s is NULL, cannot restart", stream_name);
                        atomic_store(&ctx->running, 0);
                        break;
                    }
                }
                
                // Track how long the connection has been invalid
                static time_t connection_invalid_start = 0;
                if (connection_invalid_start == 0) {
                    connection_invalid_start = time(NULL);
                    log_info("Stream %s connection marked as invalid, monitoring for %d seconds before reconnecting", 
                            stream_name, retry_delay);
                    
                    // Continue monitoring for a while before attempting reconnection
                    // This helps prevent rapid reconnection cycles
                    continue;
                }
                
                // Check if we've waited long enough before attempting reconnection
                time_t current_time = time(NULL);
                if (current_time - connection_invalid_start < retry_delay) {
                    // Not time to reconnect yet, continue monitoring
                    continue;
                }
                
                // Time to attempt reconnection
                log_error("Stream %s has been invalid for %ld seconds, attempting reconnection", 
                        stream_name, current_time - connection_invalid_start);
                
                // Reset the invalid start time
                connection_invalid_start = 0;
                
                // Restart the writer thread with retry mechanism
                bool reconnected = false;
                consecutive_failures++;
                
                // Only attempt reconnection once per monitoring cycle
                ret = hls_writer_start_recording_thread(ctx->hls_writer, config.url, stream_name, config.protocol);
                if (ret == 0) {
                    // Log success at ERROR level for visibility
                    log_error("Successfully restarted HLS writer thread for stream %s", stream_name);
                    reconnected = true;
                    consecutive_failures = 0;
                    retry_delay = INITIAL_RETRY_DELAY_SEC;
                } else {
                    // Log failure at ERROR level for visibility
                    log_error("Failed to restart HLS writer thread for stream %s (attempt %d), will retry in %d seconds", 
                            stream_name, consecutive_failures, retry_delay);
                    
                    // Implement exponential backoff with a maximum delay
                    retry_delay = fmin(retry_delay * 2, MAX_RETRY_DELAY_SEC);
                }
            } else {
                // Reset consecutive failures counter when the connection is stable
                consecutive_failures = 0;
                retry_delay = INITIAL_RETRY_DELAY_SEC;
            }

        // Sleep for a short time to avoid busy waiting
        // Use a shorter sleep time to check connection status more frequently
        usleep(500000); // 500 milliseconds
    }

    // Stop the HLS writer thread with proper error handling
    log_info("Stopping HLS writer thread for stream %s", stream_name);
    
    // Make a local copy of the writer pointer
    hls_writer_t *writer_to_close = ctx->hls_writer;
    
    // Clear the pointer first to prevent double close from other threads
    ctx->hls_writer = NULL;
    
    if (writer_to_close) {
        // First stop any running recording thread
        hls_writer_stop_recording_thread(writer_to_close);
        
        // Add a small delay to ensure the thread has fully stopped
        usleep(100000); // 100ms
        
        // Now safely close the writer
        log_info("Closing HLS writer for stream %s at %s", 
                stream_name, writer_to_close->output_dir);
        hls_writer_close(writer_to_close);
        writer_to_close = NULL;
    } else {
        log_warn("HLS writer for stream %s was already NULL during shutdown", stream_name);
    }

    // Update component state in shutdown coordinator - always do this regardless of writer status
    if (component_id >= 0) {
        update_component_state(component_id, COMPONENT_STOPPED);
        log_info("Updated HLS manager %s state to STOPPED in shutdown coordinator", stream_name);
    } else {
        log_warn("No component ID for HLS manager %s during shutdown", stream_name);
    }

    log_info("HLS streaming manager thread for stream %s exited", stream_name);
    return NULL;
}
