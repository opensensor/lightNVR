#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <assert.h>
#include <sys/stat.h>
#include <sys/types.h>

#include "video/detection_stream_thread.h"
#include "video/detection_model.h"
#include "video/detection_integration.h"
#include "video/hls_writer.h"
#include "video/stream_state.h"
#include "core/logger.h"

// No mock implementations needed anymore

/**
 * Simple test for the stream detection system
 */
int main(int argc, char **argv) {
    // Initialize logger
    init_logger();
    set_log_level(LOG_LEVEL_INFO);
    log_info("Starting stream detection test");

    // Initialize detection model system
    assert(init_detection_model_system() == 0);
    log_info("Detection model system initialized");
    
    // Initialize detection integration system
    assert(init_detection_integration() == 0);
    log_info("Detection integration system initialized");

    // Initialize stream detection system
    assert(init_stream_detection_system() == 0);
    log_info("Stream detection system initialized");

    // Check initial state
    assert(get_running_stream_detection_threads() == 0);
    log_info("Initial state: 0 running threads");

    // Start a detection thread for a test stream
    const char *stream_name = "test_stream";
    const char *model_path = "/var/lib/lightnvr/models/tiny20.sod";
    const char *hls_dir = "/var/lib/lightnvr/hls/test_stream";
    float threshold = 0.5f;
    int detection_interval = 5;

    // Create the HLS directory if it doesn't exist
    struct stat st = {0};
    if (stat(hls_dir, &st) == -1) {
        mkdir(hls_dir, 0755);
    }

    // Start the detection thread
    int ret = start_stream_detection_thread(stream_name, model_path, threshold, detection_interval, hls_dir);
    if (ret != 0) {
        log_error("Failed to start detection thread for stream %s", stream_name);
        return 1;
    }
    log_info("Started detection thread for stream %s", stream_name);

    // Check that the thread is running
    assert(is_stream_detection_thread_running(stream_name));
    assert(get_running_stream_detection_threads() == 1);
    log_info("Thread is running for stream %s", stream_name);

    // Sleep for a few seconds to let the thread run
    log_info("Sleeping for 5 seconds...");
    sleep(5);

    // Stop the detection thread
    ret = stop_stream_detection_thread(stream_name);
    if (ret != 0) {
        log_error("Failed to stop detection thread for stream %s", stream_name);
        return 1;
    }
    log_info("Stopped detection thread for stream %s", stream_name);

    // Check that the thread is no longer running
    assert(!is_stream_detection_thread_running(stream_name));
    assert(get_running_stream_detection_threads() == 0);
    log_info("Thread is no longer running for stream %s", stream_name);

    // Shutdown the stream detection system
    shutdown_stream_detection_system();
    log_info("Stream detection system shutdown");

    // Shutdown the detection integration system
    cleanup_detection_resources();
    log_info("Detection integration system shutdown");

    // Shutdown the detection model system
    shutdown_detection_model_system();
    log_info("Detection model system shutdown");

    log_info("Stream detection test completed successfully");
    return 0;
}
