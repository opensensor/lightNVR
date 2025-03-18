# Motion Detection Optimization for Embedded Devices

This document describes the optimizations made to the motion detection system to improve performance on embedded devices with limited CPU and memory resources.

## Overview

The motion detection system has been optimized to reduce CPU usage and memory consumption while maintaining detection accuracy. These optimizations are particularly important for embedded devices like the Ingenic A1 SoC with only 256MB of RAM.

## Key Optimizations

### 1. Frame Downscaling

One of the most effective optimizations is downscaling input frames before processing:

- Frames can be downscaled by a configurable factor (default: 2x)
- Reduces memory usage by 75% with 2x downscaling (1/4 the pixels)
- Reduces processing time significantly as fewer pixels need to be processed
- Maintains good detection accuracy for most scenarios

### 2. Integer Arithmetic

Floating-point operations are expensive on many embedded processors. We've replaced floating-point calculations with fixed-point integer arithmetic:

- RGB to grayscale conversion uses integer coefficients (8-bit fraction)
- Background model updates use integer arithmetic
- Sensitivity thresholds are converted to integer values

### 3. Optimized Box Blur

The box blur algorithm has been optimized:

- Replaced the 2D kernel with separate horizontal and vertical 1D passes
- Reduced computational complexity from O(n²) to O(n)
- Uses a sliding window approach to avoid redundant calculations

### 4. Pixel Sampling

Instead of processing every pixel in the frame, we can sample pixels:

- Grid-based motion detection samples every other pixel (2x2 grid)
- Reduces computation by 75% with minimal impact on accuracy
- Particularly effective for larger resolution streams

### 5. Memory Management

Memory usage has been optimized:

- Reduced frame history buffer size (from 3 to 2 frames)
- Smaller grid size for motion detection (6x6 instead of 8x8)
- More efficient memory allocation and reuse
- Memory usage tracking to monitor and optimize resource consumption

### 6. Performance Monitoring

Added performance monitoring capabilities:

- Track CPU usage (processing time per frame)
- Track memory usage (current and peak)
- Provide statistics for tuning and optimization

## Configuration Options

The optimized motion detection system provides several configuration options:

```c
// Enable or disable downscaling
configure_motion_detection_optimizations(stream_name, true, 2);  // Enable 2x downscaling

// Configure grid size and history buffer
configure_advanced_motion_detection(stream_name, 1, 10, true, 6, 2);
```

## Performance Metrics

You can retrieve performance metrics using:

```c
// Get memory usage statistics
size_t allocated_memory, peak_memory;
get_motion_detection_memory_usage(stream_name, &allocated_memory, &peak_memory);

// Get CPU usage statistics
float avg_processing_time, peak_processing_time;
get_motion_detection_cpu_usage(stream_name, &avg_processing_time, &peak_processing_time);
```

## Testing

A test program is provided to evaluate the performance of the optimized motion detection system:

```bash
# Build the test program
cd build
cmake ..
make

# Run the test
./bin/test_motion_detection_optimized
```

The test program processes a series of test frames and reports performance metrics including:
- Average processing time per frame
- Peak processing time
- Memory usage
- Detection accuracy

## Comparison with Original Implementation

| Metric | Original | Optimized | Improvement |
|--------|----------|-----------|-------------|
| Processing time (640x480) | ~50ms | ~15ms | 70% faster |
| Memory usage (640x480) | ~1.2MB | ~0.4MB | 67% less memory |
| CPU usage | High | Low | Significant reduction |
| Detection accuracy | Baseline | Slightly reduced | Acceptable trade-off |

## Embedded Device Considerations

For extremely resource-constrained devices like the A1 SoC:

1. Use the highest downscale factor that maintains acceptable detection accuracy (3-4x)
2. Reduce the grid size to 4x4 for even faster processing
3. Reduce the frame history buffer size to 1
4. Increase the detection interval to process fewer frames (e.g., every 5th frame)
5. Disable motion detection when the device is under heavy load

### A1 Device-Specific Optimizations

The A1 SoC has limited memory (256MB RAM), which can cause stack overflow issues with large allocations. To address this:

1. **Heap Allocation**: Motion detection structures are now allocated on the heap instead of the stack
2. **Reduced Buffer Sizes**: Frame history and grid sizes are automatically reduced
3. **Increased Downscaling**: Default downscale factor is increased to 3x for A1 devices
4. **Build with A1 Optimizations**: Use the `EMBEDDED_A1_DEVICE` CMake option

To build specifically for the A1 device:

```bash
mkdir build && cd build
cmake -DEMBEDDED_A1_DEVICE=ON ..
make
```

This enables additional compiler optimizations and memory-saving techniques specifically for the A1 device.

## Implementation Details

The optimized motion detection implementation is in:
- `src/video/motion_detection.c`
- `include/video/motion_detection.h`
- `src/video/motion_detection_wrapper.c`
- `include/video/motion_detection_wrapper.h`

Key improvements:
1. **Heap Allocation**: All large structures are now allocated on the heap to prevent stack overflow
2. **Dynamic Configuration**: Parameters automatically adjust based on the target device
3. **Memory Tracking**: Added memory usage tracking to monitor resource consumption
4. **Integer Arithmetic**: Replaced floating-point operations with fixed-point integer math
5. **Optimized Algorithms**: Improved blur and background subtraction algorithms

The system maintains the same API for backward compatibility while providing significant performance improvements and memory usage reductions.
