#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>
#include <unistd.h>
#include <sys/time.h>

// Define CLOCK_MONOTONIC if not available
#ifndef CLOCK_MONOTONIC
#define CLOCK_MONOTONIC 1
#endif

#include "video/motion_detection.h"
#include "video/motion_detection_optimized.h"
#include "video/detection_result.h"
#include "core/logger.h"

#define TEST_WIDTH 640
#define TEST_HEIGHT 480
#define TEST_CHANNELS 3
#define NUM_FRAMES 100
#define STREAM_NAME "test_stream"

// Generate a random frame with some motion
static unsigned char* generate_test_frame(int width, int height, int channels, int frame_num) {
    unsigned char* frame = (unsigned char*)malloc(width * height * channels);
    if (!frame) {
        return NULL;
    }
    
    // Fill with random data
    srand(time(NULL) + frame_num);
    
    for (int i = 0; i < width * height * channels; i++) {
        frame[i] = rand() % 256;
    }
    
    // Add some motion in the middle of the frame every 10 frames
    if (frame_num % 10 == 0) {
        int motion_width = width / 4;
        int motion_height = height / 4;
        int start_x = width / 2 - motion_width / 2;
        int start_y = height / 2 - motion_height / 2;
        
        for (int y = start_y; y < start_y + motion_height; y++) {
            for (int x = start_x; x < start_x + motion_width; x++) {
                for (int c = 0; c < channels; c++) {
                    int idx = (y * width + x) * channels + c;
                    // Make the motion area brighter
                    frame[idx] = (unsigned char)((int)frame[idx] + 100) % 256;
                }
            }
        }
    }
    
    return frame;
}

int main(int argc, char* argv[]) {
    // Initialize logger
    init_logger();
    set_log_level(LOG_LEVEL_DEBUG); // Debug level
    
    printf("Motion Detection Optimization Test\n");
    printf("----------------------------------\n");
    
    // Initialize motion detection systems
    init_motion_detection_optimized();
    
    // Configure optimized motion detection
    configure_motion_detection_optimized(STREAM_NAME, 0.2f, 0.01f, 1);
    configure_advanced_motion_detection_optimized(STREAM_NAME, 1, 10, true, 8, 3);
    set_motion_detection_enabled_optimized(STREAM_NAME, true);
    
    // Configure optimized motion detection
    configure_motion_detection_optimizations(STREAM_NAME, true, 2);
    
    // Performance tracking
    struct timespec start_time, end_time;
    double total_time = 0.0;
    int detections = 0;
    
    // Process frames
    printf("Processing %d frames at %dx%d resolution...\n", NUM_FRAMES, TEST_WIDTH, TEST_HEIGHT);
    
    for (int i = 0; i < NUM_FRAMES; i++) {
        // Generate test frame
        unsigned char* frame = generate_test_frame(TEST_WIDTH, TEST_HEIGHT, TEST_CHANNELS, i);
        if (!frame) {
            printf("Failed to generate test frame %d\n", i);
            continue;
        }
        
        // Process frame with motion detection
        detection_result_t result;
        
        clock_gettime(CLOCK_MONOTONIC, &start_time);
        int ret = detect_motion_optimized(STREAM_NAME, frame, TEST_WIDTH, TEST_HEIGHT, TEST_CHANNELS, time(NULL), &result);
        clock_gettime(CLOCK_MONOTONIC, &end_time);
        
        // Calculate processing time
        double frame_time = (end_time.tv_sec - start_time.tv_sec) * 1000.0 +
                           (end_time.tv_nsec - start_time.tv_nsec) / 1000000.0;
        total_time += frame_time;
        
        if (ret == 0 && result.count > 0) {
            detections++;
            printf("Frame %d: Motion detected with confidence %.2f (%.2f ms)\n", 
                   i, result.detections[0].confidence, frame_time);
        } else {
            printf("Frame %d: No motion detected (%.2f ms)\n", i, frame_time);
        }
        
        free(frame);
    }
    
    // Get performance statistics
    size_t allocated_memory, peak_memory;
    float avg_processing_time, peak_processing_time;
    
    get_motion_detection_memory_usage(STREAM_NAME, &allocated_memory, &peak_memory);
    get_motion_detection_cpu_usage(STREAM_NAME, &avg_processing_time, &peak_processing_time);
    
    // Print performance summary
    printf("\nPerformance Summary:\n");
    printf("------------------\n");
    printf("Total frames processed: %d\n", NUM_FRAMES);
    printf("Motion detections: %d\n", detections);
    printf("Average processing time: %.2f ms\n", total_time / NUM_FRAMES);
    printf("Average processing time (internal): %.2f ms\n", avg_processing_time);
    printf("Peak processing time: %.2f ms\n", peak_processing_time);
    printf("Current memory usage: %zu bytes\n", allocated_memory);
    printf("Peak memory usage: %zu bytes\n", peak_memory);
    
    // Shutdown motion detection
    shutdown_motion_detection_optimized();
    
    return 0;
}
