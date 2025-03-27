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

    // Check if we're still running after HLS writer creation
    if (!atomic_load(&ctx->running)) {
        log_info("HLS streaming thread for %s stopping after HLS writer creation", stream_name);
        if (ctx->hls_writer) {
            hls_writer_close(ctx->hls_writer);
            ctx->hls_writer = NULL;
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

    // Register with shutdown coordinator
    char component_name[128];
    snprintf(component_name, sizeof(component_name), "hls_manager_%s", stream_name);
    int component_id = register_component(component_name, COMPONENT_HLS_WRITER, ctx, 60); // Lowest priority (60)
    if (component_id >= 0) {
        log_info("Registered HLS manager %s with shutdown coordinator (ID: %d)", stream_name, component_id);
    }

    // Start the HLS writer thread
    log_info("Starting HLS writer thread for stream %s", stream_name);
    ret = hls_writer_start_recording_thread(ctx->hls_writer, config.url, stream_name, config.protocol);
    if (ret != 0) {
        log_error("Failed to start HLS writer thread for stream %s", stream_name);
        
        if (ctx->hls_writer) {
            hls_writer_close(ctx->hls_writer);
            ctx->hls_writer = NULL;
        }
        
        atomic_store(&ctx->running, 0);
        
        if (component_id >= 0) {
            update_component_state(component_id, COMPONENT_STOPPED);
        }
        
        return NULL;
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

        // Check if the writer thread is still running
        if (!hls_writer_is_recording(ctx->hls_writer)) {
            log_warn("HLS writer thread for %s has stopped unexpectedly, restarting", stream_name);
            
            // Restart the writer thread
            ret = hls_writer_start_recording_thread(ctx->hls_writer, config.url, stream_name, config.protocol);
            if (ret != 0) {
                log_error("Failed to restart HLS writer thread for stream %s", stream_name);
                atomic_store(&ctx->running, 0);
                break;
            }
        }

        // Sleep for a short time to avoid busy waiting
        usleep(1000000); // 1 second
    }

    // Stop the HLS writer thread
    log_info("Stopping HLS writer thread for stream %s", stream_name);
    hls_writer_stop_recording_thread(ctx->hls_writer);

    // When done, close writer - use a local copy to avoid double free
    hls_writer_t *writer_to_close = ctx->hls_writer;
    ctx->hls_writer = NULL;  // Clear the pointer first to prevent double close
    
    if (writer_to_close) {
        // Safely close the writer
        hls_writer_close(writer_to_close);
        writer_to_close = NULL;
    }

    // Update component state in shutdown coordinator
    if (component_id >= 0) {
        update_component_state(component_id, COMPONENT_STOPPED);
        log_info("Updated HLS manager %s state to STOPPED in shutdown coordinator", stream_name);
    }

    log_info("HLS streaming manager thread for stream %s exited", stream_name);
    return NULL;
}
