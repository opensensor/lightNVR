/**
 * ONVIF Motion Recording Example
 * 
 * This example demonstrates how to use the ONVIF motion recording feature
 * to automatically record video when motion is detected by an ONVIF camera.
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <time.h>

#include "video/onvif_motion_recording.h"
#include "core/logger.h"

/**
 * Example 1: Enable motion recording for a camera
 */
void example_enable_motion_recording(void) {
    printf("\n=== Example 1: Enable Motion Recording ===\n");
    
    // Configure motion recording
    motion_recording_config_t config = {
        .enabled = true,
        .pre_buffer_seconds = 5,      // Capture 5 seconds before motion
        .post_buffer_seconds = 10,    // Continue 10 seconds after motion ends
        .max_file_duration = 300,     // Max 5 minutes per file
        .retention_days = 30          // Keep recordings for 30 days
    };
    
    // Set codec and quality
    strncpy(config.codec, "h264", sizeof(config.codec) - 1);
    strncpy(config.quality, "high", sizeof(config.quality) - 1);
    
    // Enable for a stream
    const char *stream_name = "front_door";
    int result = enable_motion_recording(stream_name, &config);
    
    if (result == 0) {
        printf("✓ Motion recording enabled for stream: %s\n", stream_name);
        printf("  - Pre-buffer: %d seconds\n", config.pre_buffer_seconds);
        printf("  - Post-buffer: %d seconds\n", config.post_buffer_seconds);
        printf("  - Max file duration: %d seconds\n", config.max_file_duration);
        printf("  - Codec: %s\n", config.codec);
        printf("  - Quality: %s\n", config.quality);
    } else {
        printf("✗ Failed to enable motion recording for stream: %s\n", stream_name);
    }
}

/**
 * Example 2: Process motion events
 */
void example_process_motion_events(void) {
    printf("\n=== Example 2: Process Motion Events ===\n");
    
    const char *stream_name = "front_door";
    
    // Simulate motion detected
    printf("Motion detected at %ld\n", time(NULL));
    process_motion_event(stream_name, true, time(NULL));
    
    // Wait 5 seconds
    printf("Recording for 5 seconds...\n");
    sleep(5);
    
    // Simulate motion ended
    printf("Motion ended at %ld\n", time(NULL));
    process_motion_event(stream_name, false, time(NULL));
    
    // Post-buffer will continue recording for configured duration
    printf("Post-buffer active (will continue for 10 seconds)...\n");
}

/**
 * Example 3: Check recording status
 */
void example_check_status(void) {
    printf("\n=== Example 3: Check Recording Status ===\n");
    
    const char *stream_name = "front_door";
    
    // Check if enabled
    bool enabled = is_motion_recording_enabled(stream_name);
    printf("Motion recording enabled: %s\n", enabled ? "Yes" : "No");
    
    // Get recording state
    recording_state_t state = get_motion_recording_state(stream_name);
    const char *state_str;
    switch (state) {
        case RECORDING_STATE_IDLE:
            state_str = "IDLE";
            break;
        case RECORDING_STATE_BUFFERING:
            state_str = "BUFFERING";
            break;
        case RECORDING_STATE_RECORDING:
            state_str = "RECORDING";
            break;
        case RECORDING_STATE_FINALIZING:
            state_str = "FINALIZING";
            break;
        default:
            state_str = "UNKNOWN";
    }
    printf("Recording state: %s\n", state_str);
    
    // Get statistics
    uint64_t total_recordings, total_events;
    if (get_motion_recording_stats(stream_name, &total_recordings, &total_events) == 0) {
        printf("Statistics:\n");
        printf("  - Total recordings: %llu\n", (unsigned long long)total_recordings);
        printf("  - Total motion events: %llu\n", (unsigned long long)total_events);
    }
    
    // Get current recording path
    char path[512];
    if (get_current_motion_recording_path(stream_name, path, sizeof(path)) == 0) {
        printf("Current recording: %s\n", path);
    } else {
        printf("No active recording\n");
    }
}

/**
 * Example 4: Update configuration
 */
void example_update_configuration(void) {
    printf("\n=== Example 4: Update Configuration ===\n");
    
    const char *stream_name = "front_door";
    
    // New configuration with different settings
    motion_recording_config_t new_config = {
        .enabled = true,
        .pre_buffer_seconds = 10,     // Increased to 10 seconds
        .post_buffer_seconds = 15,    // Increased to 15 seconds
        .max_file_duration = 600,     // Increased to 10 minutes
        .retention_days = 60          // Increased to 60 days
    };
    
    strncpy(new_config.codec, "h265", sizeof(new_config.codec) - 1);
    strncpy(new_config.quality, "medium", sizeof(new_config.quality) - 1);
    
    int result = update_motion_recording_config(stream_name, &new_config);
    
    if (result == 0) {
        printf("✓ Configuration updated for stream: %s\n", stream_name);
        printf("  - Pre-buffer: %d seconds\n", new_config.pre_buffer_seconds);
        printf("  - Post-buffer: %d seconds\n", new_config.post_buffer_seconds);
        printf("  - Max file duration: %d seconds\n", new_config.max_file_duration);
        printf("  - Codec: %s\n", new_config.codec);
        printf("  - Quality: %s\n", new_config.quality);
    } else {
        printf("✗ Failed to update configuration for stream: %s\n", stream_name);
    }
}

/**
 * Example 5: Disable motion recording
 */
void example_disable_motion_recording(void) {
    printf("\n=== Example 5: Disable Motion Recording ===\n");
    
    const char *stream_name = "front_door";
    
    int result = disable_motion_recording(stream_name);
    
    if (result == 0) {
        printf("✓ Motion recording disabled for stream: %s\n", stream_name);
    } else {
        printf("✗ Failed to disable motion recording for stream: %s\n", stream_name);
    }
}

/**
 * Example 6: Multiple cameras
 */
void example_multiple_cameras(void) {
    printf("\n=== Example 6: Multiple Cameras ===\n");
    
    const char *cameras[] = {
        "front_door",
        "back_door",
        "garage",
        "driveway"
    };
    
    int num_cameras = sizeof(cameras) / sizeof(cameras[0]);
    
    // Configure all cameras
    for (int i = 0; i < num_cameras; i++) {
        motion_recording_config_t config = {
            .enabled = true,
            .pre_buffer_seconds = 5,
            .post_buffer_seconds = 10,
            .max_file_duration = 300,
            .retention_days = 30
        };
        
        strncpy(config.codec, "h264", sizeof(config.codec) - 1);
        strncpy(config.quality, "high", sizeof(config.quality) - 1);
        
        int result = enable_motion_recording(cameras[i], &config);
        
        if (result == 0) {
            printf("✓ Enabled motion recording for: %s\n", cameras[i]);
        } else {
            printf("✗ Failed to enable motion recording for: %s\n", cameras[i]);
        }
    }
    
    printf("\nMotion recording enabled for %d cameras\n", num_cameras);
}

/**
 * Main function - runs all examples
 */
int main(int argc, char *argv[]) {
    printf("ONVIF Motion Recording Examples\n");
    printf("================================\n");
    
    // Initialize the motion recording system
    printf("\nInitializing ONVIF motion recording system...\n");
    if (init_onvif_motion_recording() != 0) {
        fprintf(stderr, "Failed to initialize ONVIF motion recording system\n");
        return 1;
    }
    printf("✓ System initialized\n");
    
    // Run examples
    example_enable_motion_recording();
    example_check_status();
    example_process_motion_events();
    sleep(2);  // Wait for event processing
    example_check_status();
    example_update_configuration();
    example_multiple_cameras();
    example_disable_motion_recording();
    
    // Cleanup
    printf("\n=== Cleanup ===\n");
    cleanup_onvif_motion_recording();
    printf("✓ System cleaned up\n");
    
    printf("\nAll examples completed successfully!\n");
    return 0;
}

