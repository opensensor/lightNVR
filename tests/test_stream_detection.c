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
#include "video/stream_manager.h"
#include "video/stream_state.h"
#include "core/config.h"
#include "core/logger.h"
#include "utils/strings.h"

// No mock implementations needed anymore

/**
 * Register the stream this test drives.
 *
 * start_unified_detection_thread() resolves its subject through the stream
 * manager, so without this the test could only ever report "Stream test_stream
 * not found" -- which is what it did on any machine that did not already have a
 * provisioned /var/lib/lightnvr database. The URL never has to resolve: the
 * detection thread's connect attempt failing is orthogonal to the thread
 * lifecycle these assertions cover.
 */
static int provision_test_stream(const char *stream_name) {
    if (init_stream_state_manager(MAX_STREAMS) != 0) {
        log_error("Failed to initialize stream state manager");
        return -1;
    }
    if (init_stream_manager(MAX_STREAMS) != 0) {
        log_error("Failed to initialize stream manager");
        return -1;
    }

    stream_config_t config;
    memset(&config, 0, sizeof(config));
    safe_strcpy(config.name, stream_name, sizeof(config.name), 0);
    safe_strcpy(config.url, "rtsp://127.0.0.1:1/unit-test",
                sizeof(config.url), 0);
    config.enabled = true;
    config.width = 640;
    config.height = 480;
    config.fps = 15;
    config.protocol = STREAM_PROTOCOL_TCP;
    config.detection_based_recording = true;

    if (!add_stream(&config)) {
        log_error("Failed to register stream %s", stream_name);
        return -1;
    }
    return 0;
}

/**
 * Simple test for the unified detection system
 */
int main(int argc, char **argv) {
    // Initialize logger
    init_logger();
    set_log_level(LOG_LEVEL_INFO);
    log_info("Starting unified detection test");

    // Initialize detection integration system
    int d_ret = init_detection_integration();
    assert(d_ret == 0);

    // Initialize unified detection system
    int u_ret = init_unified_detection_system();
    assert(u_ret == 0);

    // Check initial state - no threads running
    const char *stream_name = "test_stream";
    assert(!is_unified_detection_running(stream_name));
    log_info("Initial state: no thread running for test stream");

    if (provision_test_stream(stream_name) != 0) {
        return 1;
    }
    log_info("Registered stream %s for the test", stream_name);

    // Test parameters
    const char *model_path = "/var/lib/lightnvr/models/tiny20.sod";
    float threshold = 0.5f;
    int pre_buffer = 5;
    int post_buffer = 10;
    bool annotation_only = false;  // Test in detection-only mode (creates MP4s)

    // Start the unified detection thread
    int ret = start_unified_detection_thread(stream_name, model_path, threshold, pre_buffer, post_buffer, annotation_only);
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

    shutdown_stream_manager();
    shutdown_stream_state_manager();
    log_info("Stream manager shutdown");

    log_info("Unified detection test completed successfully");
    return 0;
}
