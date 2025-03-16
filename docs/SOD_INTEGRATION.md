# SOD Integration in LightNVR

This document explains how SOD (an embedded computer vision & machine learning library) is integrated into LightNVR and how to build the project with or without SOD support.

## Overview

SOD is used for object detection in video streams. In previous versions of LightNVR, SOD was directly linked into the binary. Now, SOD is compiled as a standalone library that is optional and used only when available.

## Building Options

### Building with SOD (Default)

By default, LightNVR is built with SOD support. This means:

1. SOD is compiled as a shared library (`libsod.so`)
2. LightNVR is linked against this library
3. The SOD code is directly accessible to LightNVR

To build with SOD support:

```bash
# Using the standard build script (SOD is enabled by default)
./scripts/build.sh --release

# Or explicitly enable SOD
./scripts/build.sh --release --with-sod

# Or manually
mkdir -p build
cd build
cmake -DCMAKE_BUILD_TYPE=Release -DENABLE_SOD=ON ..
make -j$(nproc)
```

### Building without SOD

You can also build LightNVR without SOD support. In this case:

1. SOD is not compiled
2. LightNVR is not linked against SOD
3. LightNVR will attempt to dynamically load `libsod.so` at runtime if it's available

To build without SOD support:

```bash
# Using the standard build script with the --without-sod option
./scripts/build.sh --release --without-sod

# Or manually
mkdir -p build
cd build
cmake -DCMAKE_BUILD_TYPE=Release -DENABLE_SOD=OFF ..
make -j$(nproc)
```

## Runtime Behavior

### When Built with SOD

When LightNVR is built with SOD support:

- All SOD functionality is available
- Detection models that require SOD can be loaded and used

### When Built without SOD

When LightNVR is built without SOD support:

- The system will attempt to dynamically load `libsod.so` at runtime
- If `libsod.so` is found, SOD functionality will be available
- If `libsod.so` is not found, SOD functionality will be disabled, but LightNVR will still run
- Detection models that require SOD will not be loadable if SOD is not available

## Installation Options

### Installing with SOD (Default)

By default, the installation script will install both LightNVR and the SOD library:

```bash
# Build with SOD support first
./scripts/build.sh --release

# Install with SOD support (default)
sudo ./scripts/install.sh

# Or explicitly specify SOD installation
sudo ./scripts/install.sh --with-sod
```

### Installing without SOD

You can also install LightNVR without the SOD library:

```bash
# Build without SOD support first
./scripts/build.sh --release --without-sod

# Install without SOD
sudo ./scripts/install.sh --without-sod
```

### Installing SOD Separately

If you build LightNVR without SOD support, you can still install SOD separately:

```bash
# Clone the repository
git clone https://github.com/yourusername/lightnvr.git
cd lightnvr

# Build just the SOD library
mkdir -p build_sod
cd build_sod
cmake ../src/sod
make -j$(nproc)

# Install the SOD library
sudo cp lib/libsod.so.1.1.9 /usr/local/lib/
sudo ln -sf /usr/local/lib/libsod.so.1.1.9 /usr/local/lib/libsod.so.1
sudo ln -sf /usr/local/lib/libsod.so.1 /usr/local/lib/libsod.so
sudo ldconfig
```

This will make SOD available to LightNVR at runtime, even if it was built without SOD support.

## Detection Models

The following detection model types are supported:

- SOD models (`.sod` extension)
- SOD RealNet models (`.realnet.sod` extension)
- TensorFlow Lite models (`.tflite` extension)

SOD and SOD RealNet models require SOD to be available (either built-in or dynamically loaded).
TensorFlow Lite models require the TensorFlow Lite library to be available.

## Unified Detection Interface

LightNVR now includes a unified detection interface that supports both RealNet and CNN model architectures. This allows you to use either model type with the same API, making it easy to switch between models based on your requirements.

For more information, see [SOD Unified Detection](SOD_UNIFIED_DETECTION.md).

### Test Programs

Two test programs are provided to demonstrate SOD integration:

1. `test_sod_realnet` - Tests SOD RealNet face detection
   ```bash
   ./test_sod_realnet test.jpg face.realnet.sod output.jpg
   ```

2. `test_sod_unified` - Tests both RealNet and CNN face detection with auto-detection of model type
   ```bash
   # For RealNet models
   ./test_sod_unified test.jpg face.realnet.sod output.jpg
   
   # For CNN models
   ./test_sod_unified test.jpg face_cnn.sod output.jpg
   ```

These test programs are built automatically when SOD support is enabled.
