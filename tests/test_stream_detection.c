#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <assert.h>
#include <sys/stat.h>
#include <sys/types.h>

#include "video/unified_detection_thread.h"
#include "video/detection_model.h"
#include "video/detection_integration.h"
#include "video/hls_writer.h"
#include "video/stream_state.h"
#include "core/logger.h"

// No mock implementations needed anymore

/**
 * Simple test for the unified detection system
 */
int main(int argc, char **argv) {
    // Initialize logger
    init_logger();
    set_log_level(LOG_LEVEL_INFO);
    log_info("Starting unified detection test");

    // Initialize detection model system
    assert(init_detection_model_system() == 0);
    log_info("Detection model system initialized");

    // Initialize detection integration system
    assert(init_detection_integration() == 0);
    log_info("Detection integration system initialized");

    // Initialize unified detection system
    assert(init_unified_detection_system() == 0);
    log_info("Unified detection system initialized");

    // Check initial state - no threads running
    const char *stream_name = "test_stream";
    assert(!is_unified_detection_running(stream_name));
    log_info("Initial state: no thread running for test stream");

    // Test parameters
    const char *model_path = "/var/lib/lightnvr/models/tiny20.sod";
    float threshold = 0.5f;
    int pre_buffer = 5;
    int post_buffer = 10;

    // Start the unified detection thread
    int ret = start_unified_detection_thread(stream_name, model_path, threshold, pre_buffer, post_buffer);
    if (ret != 0) {
        log_error("Failed to start unified detection thread for stream %s", stream_name);
        return 1;
    }
    log_info("Started unified detection thread for stream %s", stream_name);

    // Check that the thread is running
    assert(is_unified_detection_running(stream_name));
    log_info("Thread is running for stream %s", stream_name);

    // Check state
    unified_detection_state_t state = get_unified_detection_state(stream_name);
    log_info("Current state for stream %s: %d", stream_name, state);

    // Sleep for a few seconds to let the thread run
    log_info("Sleeping for 5 seconds...");
    sleep(5);

    // Stop the detection thread
    ret = stop_unified_detection_thread(stream_name);
    if (ret != 0) {
        log_error("Failed to stop unified detection thread for stream %s", stream_name);
        return 1;
    }
    log_info("Stopped unified detection thread for stream %s", stream_name);

    // Give thread time to stop
    sleep(1);

    // Check that the thread is no longer running
    assert(!is_unified_detection_running(stream_name));
    log_info("Thread is no longer running for stream %s", stream_name);

    // Shutdown the unified detection system
    shutdown_unified_detection_system();
    log_info("Unified detection system shutdown");

    // Shutdown the detection integration system
    cleanup_detection_resources();
    log_info("Detection integration system shutdown");

    // Shutdown the detection model system
    shutdown_detection_model_system();
    log_info("Detection model system shutdown");

    log_info("Unified detection test completed successfully");
    return 0;
}
