# LightNVR Examples

This directory contains example code demonstrating how to use various LightNVR features.

## ONVIF Motion Recording Example

**File**: `onvif_motion_recording_example.c`

This example demonstrates how to use the ONVIF motion recording feature to automatically record video when motion is detected by an ONVIF camera.

### Features Demonstrated

1. **Enable Motion Recording**: Configure and enable motion recording for a camera
2. **Process Motion Events**: Handle motion detection events
3. **Check Recording Status**: Query recording state and statistics
4. **Update Configuration**: Modify recording settings at runtime
5. **Disable Motion Recording**: Stop motion recording for a camera
6. **Multiple Cameras**: Configure motion recording for multiple cameras

### Building the Example

The example is provided for reference and educational purposes. To integrate it into your application:

1. Include the necessary headers:
   ```c
   #include "video/onvif_motion_recording.h"
   #include "core/logger.h"
   ```

2. Initialize the system:
   ```c
   init_onvif_motion_recording();
   ```

3. Configure motion recording:
   ```c
   motion_recording_config_t config = {
       .enabled = true,
       .pre_buffer_seconds = 5,
       .post_buffer_seconds = 10,
       .max_file_duration = 300,
       .retention_days = 30
   };
   enable_motion_recording("camera_name", &config);
   ```

4. Process motion events (automatically called by ONVIF detection):
   ```c
   process_motion_event("camera_name", true, time(NULL));
   ```

### Running the Example

To compile and run the example as a standalone program:

```bash
# From the LightNVR root directory
cd build
cmake ..
make

# The example code can be integrated into your application
# or used as a reference for implementing motion recording
```

### Example Output

```
ONVIF Motion Recording Examples
================================

Initializing ONVIF motion recording system...
✓ System initialized

=== Example 1: Enable Motion Recording ===
✓ Motion recording enabled for stream: front_door
  - Pre-buffer: 5 seconds
  - Post-buffer: 10 seconds
  - Max file duration: 300 seconds
  - Codec: h264
  - Quality: high

=== Example 3: Check Recording Status ===
Motion recording enabled: Yes
Recording state: IDLE
Statistics:
  - Total recordings: 0
  - Total motion events: 0
No active recording

=== Example 2: Process Motion Events ===
Motion detected at 1704902400
Recording for 5 seconds...
Motion ended at 1704902405
Post-buffer active (will continue for 10 seconds)...

=== Example 3: Check Recording Status ===
Motion recording enabled: Yes
Recording state: RECORDING
Statistics:
  - Total recordings: 1
  - Total motion events: 2
Current recording: /var/lib/lightnvr/recordings/front_door/2024/01/10/front_door_20240110_120000_motion.mp4

=== Example 4: Update Configuration ===
✓ Configuration updated for stream: front_door
  - Pre-buffer: 10 seconds
  - Post-buffer: 15 seconds
  - Max file duration: 600 seconds
  - Codec: h265
  - Quality: medium

=== Example 6: Multiple Cameras ===
✓ Enabled motion recording for: front_door
✓ Enabled motion recording for: back_door
✓ Enabled motion recording for: garage
✓ Enabled motion recording for: driveway

Motion recording enabled for 4 cameras

=== Example 5: Disable Motion Recording ===
✓ Motion recording disabled for stream: front_door

=== Cleanup ===
✓ System cleaned up

All examples completed successfully!
```

## Integration with LightNVR

The ONVIF motion recording feature is automatically integrated with LightNVR's ONVIF detection system. When motion is detected by an ONVIF camera:

1. The `detect_motion_onvif()` function detects motion
2. It calls `process_motion_event()` to queue the event
3. The event processor thread starts recording
4. Recording continues until motion ends + post-buffer timeout
5. The recording is saved with a timestamp-based filename

## Configuration

Motion recording can be configured per camera with the following settings:

- **enabled**: Enable/disable motion recording
- **pre_buffer_seconds**: Seconds to capture before motion (5-30)
- **post_buffer_seconds**: Seconds to continue after motion ends (5-60)
- **max_file_duration**: Maximum recording duration in seconds
- **codec**: Video codec (h264, h265)
- **quality**: Recording quality (low, medium, high)
- **retention_days**: Days to keep recordings

## See Also

- [ONVIF Motion Recording Documentation](../docs/ONVIF_MOTION_RECORDING.md)
- [ONVIF Detection Documentation](../docs/ONVIF_DETECTION.md)
- [Product Requirements Document](../PRD-onivf-motion-detect.md)
- [Implementation Summary](../IMPLEMENTATION_SUMMARY.md)

