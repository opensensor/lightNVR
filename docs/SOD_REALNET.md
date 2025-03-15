# SOD RealNet Integration

This document describes the integration of SOD RealNet face detection into the LightNVR system.

## Overview

SOD RealNet is a real-time face detection system that is part of the SOD library. It provides fast and accurate face detection capabilities that can be used for video surveillance and security applications.

## Requirements

- SOD library with RealNet support (directly linked)
- RealNet model file (e.g., `face.realnet.sod`)

## Integration

The SOD RealNet integration consists of the following components:

1. **SOD RealNet Interface** (`include/video/sod_realnet.h` and `src/video/sod_realnet.c`): Provides a wrapper around the SOD RealNet API for loading models and performing detection.

2. **Detection System Integration** (`src/video/detection.c`): Integrates SOD RealNet into the existing detection system, allowing it to be used alongside other detection methods.

3. **Test Program** (`tests/test_sod_realnet.c`): A standalone program for testing SOD RealNet face detection on images.

## Usage

### Loading a RealNet Model

```c
#include "video/detection.h"

// Initialize detection system
init_detection_system();

// Load RealNet model
detection_model_t model = load_detection_model("path/to/face.realnet.sod", 5.0f);
if (model) {
    // Model loaded successfully
    // ...
    
    // Unload model when done
    unload_detection_model(model);
}

// Shutdown detection system
shutdown_detection_system();
```

### Performing Detection

```c
#include "video/detection.h"
#include "video/detection_result.h"

// Assuming you have a frame in grayscale format
unsigned char *frame_data = ...;
int width = 640;
int height = 480;
int channels = 1; // Grayscale

// Create result structure
detection_result_t result;

// Perform detection
if (detect_objects(model, frame_data, width, height, channels, &result) == 0) {
    // Detection successful
    printf("Detected %d faces\n", result.count);
    
    // Process detections
    for (int i = 0; i < result.count; i++) {
        printf("Face %d: x=%.2f, y=%.2f, width=%.2f, height=%.2f, confidence=%.2f\n",
               i+1,
               result.detections[i].x,
               result.detections[i].y,
               result.detections[i].width,
               result.detections[i].height,
               result.detections[i].confidence);
    }
}
```

## Testing

A test program is provided to test SOD RealNet face detection on images:

```bash
# Build the test program
cd build
cmake -DBUILD_TESTS=ON ..
make

# Run the test program
./bin/test_sod_realnet path/to/image.jpg path/to/face.realnet.sod output.jpg
```

The test program will load the specified image, perform face detection using the specified model, and save the result to the specified output file.

## Model Format

SOD RealNet models have the `.realnet.sod` extension. These models are specifically designed for real-time face detection and are optimized for speed and accuracy.

## Performance Considerations

- SOD RealNet is designed for real-time processing and can achieve high frame rates on modern hardware.
- For optimal performance, use grayscale images for detection (1 channel).
- The detection threshold can be adjusted to balance between detection accuracy and false positives. A typical value is 5.0.
