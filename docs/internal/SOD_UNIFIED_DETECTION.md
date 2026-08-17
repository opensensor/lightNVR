# SOD Unified Detection

This document describes the unified detection interface that supports both RealNet and CNN model architectures for object detection.

## Overview

The SOD (Symisc Object Detection) library supports multiple model architectures for object detection:

1. **RealNet** - A lightweight neural network architecture optimized for embedded systems
2. **CNN** - Convolutional Neural Networks with higher accuracy but more computational requirements
   - **Face Detection** - CNN model for detecting faces
   - **VOC Detection** - CNN model for detecting 20 classes of objects (Pascal VOC dataset)

The unified detection interface allows you to use any of these model types with the same API, making it easy to switch between models based on your requirements.

## Model Types

### RealNet Models

RealNet models are typically used for face detection on resource-constrained devices. They are:

- Faster and more lightweight than CNN models
- Optimized for embedded systems
- Identified by the `.realnet.sod` file extension
- Typically use a higher threshold value (around 5.0)

### CNN Models

CNN models provide higher accuracy but require more computational resources:

- More accurate detection results
- Support for multiple object classes
- Identified by the `.sod` file extension (without `.realnet`)
- Typically use a lower threshold value (around 0.3)

#### Face Detection CNN Models

Face detection CNN models are specialized for detecting human faces:

- Use the `:face` architecture identifier when loading
- Typically named with "face" in the filename (e.g., `face_cnn.sod`)
- Return detections with the label "face"

#### VOC Detection CNN Models

VOC (Visual Object Classes) detection models can detect 20 different object classes:

- Use the `:voc` architecture identifier when loading
- Typically named with "voc" in the filename or specifically `tiny20.sod`
- Can detect: person, bicycle, car, motorcycle, airplane, bus, train, truck, boat, traffic light, fire hydrant, stop sign, parking meter, bench, bird, cat, dog, horse, sheep, cow
- Return detections with labels corresponding to the detected object class

## Using the Unified Test Program

The `test_sod_unified` program demonstrates how to use both model types with a unified interface.

### Command Line Usage

```
./test_sod_unified <image_path> <model_path> [output_path] [model_type]
```

Parameters:
- `image_path`: Path to the input image
- `model_path`: Path to the model file (RealNet or CNN)
- `output_path`: (Optional) Path to save the output image (default: "out.jpg")
- `model_type`: (Optional) Explicitly specify the model type ("realnet" or "cnn")

If `model_type` is not specified, it will be auto-detected based on the file name:
- Files with `.realnet.sod` extension are treated as RealNet models
- Files with `.sod` extension (without `.realnet`) are treated as CNN models

### Examples

#### Using a RealNet Model

```bash
./test_sod_unified test.jpg face.realnet.sod output.jpg
```

The program will automatically detect that this is a RealNet model based on the file name.

#### Using a CNN Model

```bash
./test_sod_unified test.jpg face_cnn.sod output.jpg
```

The program will automatically detect that this is a CNN model based on the file name.

#### Explicitly Specifying the Model Type

```bash
./test_sod_unified test.jpg face_model.sod output.jpg cnn
```

This explicitly tells the program to use the CNN API for detection.

## Implementation Details

The unified detection interface works by:

1. Examining the input model file to determine if it's a RealNet or CNN model
2. Using the appropriate loading method based on the detected model type
3. Implementing detection using the correct API calls for each model type
4. Converting the detection results to a common format

### Auto-detection Logic

The model type is auto-detected using the following logic:

```c
const char* detect_model_type(const char *model_path) {
    // Check for RealNet models by file name pattern
    if (strstr(model_path, ".realnet.sod") != NULL) {
        return "realnet";
    }
    
    // For other .sod files, assume CNN model
    const char *ext = strrchr(model_path, '.');
    if (ext && strcasecmp(ext, ".sod") == 0) {
        return "cnn";
    }
    
    return NULL; // Unknown model type
}
```

### Model Loading

Models are loaded differently based on their type:

- RealNet models are loaded using `load_detection_model()` with a threshold of 5.0
- CNN models are loaded using `sod_cnn_create()` with the appropriate architecture identifier and a threshold of 0.3

**Important Note for CNN Models**: When loading CNN models, you must use the correct architecture identifier:

- For face detection, use the `:face` architecture identifier:
  ```c
  int rc = sod_cnn_create(&cnn_model, ":face", model_path, &err_msg);
  ```

- For VOC object detection, use the `:voc` architecture identifier:
  ```c
  int rc = sod_cnn_create(&cnn_model, ":voc", model_path, &err_msg);
  ```

The architecture identifier tells SOD which built-in neural network architecture to use. The model file (e.g., `face_cnn.sod` or `tiny20.sod`) contains the weights for this architecture, but not the architecture itself.

### Detection Process

The detection process also differs based on the model type:

- RealNet models use grayscale images and the `detect_objects()` function
- CNN models use color images and the `sod_cnn_predict()` function

## Test Programs

### Unified Test Program

The `test_sod_unified` program demonstrates how to use both RealNet and CNN models with a unified interface. It's primarily designed for face detection but can be used with other CNN models as well.

### VOC Test Program

The `test_sod_voc` program is specifically designed to demonstrate the VOC object detection capabilities. It uses the same unified detection interface but is optimized for the VOC model.

#### Command Line Usage

```
./test_sod_voc <image_path> <model_path> [output_path]
```

Parameters:
- `image_path`: Path to the input image
- `model_path`: Path to the VOC model file (typically `tiny20.sod`)
- `output_path`: (Optional) Path to save the output image (default: "out.png")

Example:
```bash
./test_sod_voc test.jpg tiny20.sod output.png
```

## Building the Test Programs

The test programs are built automatically when SOD support is enabled:

```bash
mkdir build && cd build
cmake .. -DENABLE_SOD=ON
make
```

This will build the `test_sod_realnet`, `test_sod_unified`, and `test_sod_voc` programs.

## Performance Considerations

- RealNet models are generally faster but may be less accurate
- CNN models provide better accuracy but require more computational resources
- Choose the appropriate model type based on your specific requirements
