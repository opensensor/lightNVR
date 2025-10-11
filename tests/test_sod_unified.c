#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "video/detection.h"
#include "video/sod_realnet.h"
#include "video/sod_integration.h"
#include "core/logger.h"
#include "sod/sod.h"

/**
 * Unified test program for SOD RealNet and CNN face detection
 *
 * Usage: ./test_sod_unified <image_path> <model_path> [output_path] [model_type]
 *
 * Example for RealNet: ./test_sod_unified test.jpg face.realnet.sod output.jpg realnet
 * Example for CNN: ./test_sod_unified test.jpg face_cnn.sod output.jpg cnn
 *
 * If model_type is not specified, it will be auto-detected based on the file name.
 */

// Model type constants (for compatibility with detect_model_type return values)
#define MODEL_TYPE_REALNET "realnet"
#define MODEL_TYPE_CNN "cnn"

// Helper function to check if model type is RealNet
static bool is_realnet_model(const char *model_type) {
    return (strcasecmp(model_type, MODEL_TYPE_REALNET) == 0 ||
            strcasecmp(model_type, MODEL_TYPE_SOD_REALNET) == 0 ||
            strstr(model_type, "realnet") != NULL);
}

// Helper function to check if model type is CNN/SOD
static bool is_cnn_model(const char *model_type) {
    return (strcasecmp(model_type, MODEL_TYPE_CNN) == 0 ||
            strcasecmp(model_type, MODEL_TYPE_SOD) == 0 ||
            strcmp(model_type, "sod") == 0);
}

int main(int argc, char *argv[])
{
    if (argc < 3) {
        printf("Usage: %s <image_path> <model_path> [output_path] [model_type]\n", argv[0]);
        printf("  model_type: 'realnet' or 'cnn' (optional, auto-detected if not specified)\n");
        return 1;
    }

    const char *image_path = argv[1];
    const char *model_path = argv[2];
    const char *output_path = argc > 3 ? argv[3] : "out.jpg";

    // Determine model type (auto-detect or from command line)
    const char *model_type = NULL;
    if (argc > 4) {
        model_type = argv[4];
        if (!is_realnet_model(model_type) && !is_cnn_model(model_type)) {
            fprintf(stderr, "Invalid model type: %s (must be 'realnet' or 'cnn')\n", model_type);
            return 1;
        }
    } else {
        // Auto-detect model type
        model_type = detect_model_type(model_path);
        if (!model_type || strcmp(model_type, "unknown") == 0) {
            fprintf(stderr, "Could not auto-detect model type for: %s\n", model_path);
            fprintf(stderr, "Please specify model type as the fourth argument ('realnet' or 'cnn')\n");
            return 1;
        }
        printf("Auto-detected model type: %s\n", model_type);
    }

    // Initialize detection system
    if (init_detection_system() != 0) {
        fprintf(stderr, "Failed to initialize detection system\n");
        return 1;
    }

    // Check if model is supported - only for RealNet
    if (is_realnet_model(model_type) && !is_model_supported(model_path)) {
        fprintf(stderr, "RealNet model not supported: %s\n", model_path);
        shutdown_detection_system();
        return 1;
    }

    // Variables for CNN model (only used if model_type is CNN)
    sod_cnn *cnn_model = NULL;
    float *cnn_blob = NULL;
    sod_box *cnn_boxes = NULL;

    // Variables for detection
    detection_model_t model = NULL;
    detection_result_t result;
    memset(&result, 0, sizeof(detection_result_t)); // Initialize result to zeros

    // Variables for image processing
    sod_img gray_img = {0};
    sod_img color_img = {0};
    unsigned char *blob = NULL;

    // Load model based on type
    if (is_realnet_model(model_type)) {
        // Load RealNet model using existing API
        model = load_detection_model(model_path, 5.0f); // RealNet typically uses 5.0 threshold
        if (!model) {
            fprintf(stderr, "Failed to load RealNet model: %s\n", model_path);
            shutdown_detection_system();
            return 1;
        }
    } else if (is_cnn_model(model_type)) {
        // Load CNN model directly using SOD CNN API
        const char *err_msg = NULL;

        // For CNN models, we need to use the ":face" architecture
        // This tells SOD to use the built-in face detection architecture
        int rc = sod_cnn_create(&cnn_model, ":face", model_path, &err_msg);
        if (rc != SOD_OK || !cnn_model) {
            fprintf(stderr, "Failed to load CNN model: %s - %s\n",
                    model_path, err_msg ? err_msg : "Unknown error");
            shutdown_detection_system();
            return 1;
        }

        // Set detection threshold (0.3 is typical for CNN models)
        sod_cnn_config(cnn_model, SOD_CNN_DETECTION_THRESHOLD, 0.3);
    }

    // Load grayscale image for detection - only needed for RealNet
    if (is_realnet_model(model_type)) {
        gray_img = sod_img_load_grayscale(image_path);
        if (gray_img.data == NULL) {
            fprintf(stderr, "Failed to load grayscale image: %s\n", image_path);
            if (model) unload_detection_model(model);
            shutdown_detection_system();
            return 1;
        }
    }

    // Load color image for visualization and CNN detection
    color_img = sod_img_load_color(image_path);
    if (color_img.data == NULL) {
        fprintf(stderr, "Failed to load color image: %s\n", image_path);
        if (gray_img.data) sod_free_image(gray_img);
        if (cnn_model) sod_cnn_destroy(cnn_model);
        if (model) unload_detection_model(model);
        shutdown_detection_system();
        return 1;
    }

    // Run detection based on model type
    if (is_realnet_model(model_type)) {
        // Convert image to blob for RealNet
        blob = sod_image_to_blob(gray_img);
        if (!blob) {
            fprintf(stderr, "Failed to convert image to blob\n");
            sod_free_image(gray_img);
            sod_free_image(color_img);
            unload_detection_model(model);
            shutdown_detection_system();
            return 1;
        }

        // Run detection using the unified API
        if (detect_objects(model, blob, gray_img.w, gray_img.h, gray_img.c, &result) != 0) {
            fprintf(stderr, "RealNet detection failed\n");
            sod_image_free_blob(blob);
            sod_free_image(gray_img);
            sod_free_image(color_img);
            unload_detection_model(model);
            shutdown_detection_system();
            return 1;
        }
    } else if (is_cnn_model(model_type)) {
        // For CNN, we use the direct CNN API

        // Prepare image for CNN
        cnn_blob = sod_cnn_prepare_image(cnn_model, color_img);
        if (!cnn_blob) {
            fprintf(stderr, "Failed to prepare image for CNN detection\n");
            sod_free_image(color_img);
            sod_cnn_destroy(cnn_model);
            shutdown_detection_system();
            return 1;
        }

        // Run CNN detection
        int box_count = 0;

        sod_cnn_predict(cnn_model, cnn_blob, &cnn_boxes, &box_count);

        // Convert CNN detection results to unified format
        result.count = 0;
        for (int i = 0; i < box_count && result.count < MAX_DETECTIONS; i++) {
            // Apply threshold (already applied by sod_cnn_predict, but double-check)
            if (cnn_boxes[i].score < 0.3) continue;

            // Copy detection data
            strncpy(result.detections[result.count].label,
                    cnn_boxes[i].zName ? cnn_boxes[i].zName : "face",
                    MAX_LABEL_LENGTH - 1);

            result.detections[result.count].confidence = cnn_boxes[i].score;

            // Normalize coordinates to 0.0-1.0 range
            result.detections[result.count].x = (float)cnn_boxes[i].x / color_img.w;
            result.detections[result.count].y = (float)cnn_boxes[i].y / color_img.h;
            result.detections[result.count].width = (float)cnn_boxes[i].w / color_img.w;
            result.detections[result.count].height = (float)cnn_boxes[i].h / color_img.h;

            result.count++;
        }
    }

    // Process results
    printf("Detected %d faces\n", result.count);

    // Create SOD boxes for visualization
    for (int i = 0; i < result.count; i++) {
        // Convert normalized coordinates to pixel coordinates
        int x = (int)(result.detections[i].x * color_img.w);
        int y = (int)(result.detections[i].y * color_img.h);
        int width = (int)(result.detections[i].width * color_img.w);
        int height = (int)(result.detections[i].height * color_img.h);

        // Create SOD box
        struct sod_box box;
        box.zName = (char *)result.detections[i].label;
        box.x = x;
        box.y = y;
        box.w = width;
        box.h = height;
        box.score = result.detections[i].confidence;
        box.pUserData = NULL;

        // Draw bounding box
        sod_image_draw_bbox_width(color_img, box, 3, 255.0, 0.0, 225.0);

        // Print detection info
        printf("Face %d: x=%d, y=%d, w=%d, h=%d, confidence=%.2f\n",
               i+1, x, y, width, height, result.detections[i].confidence);
    }

    // Save output image
    if (sod_img_save_as_png(color_img, output_path) != 0) {
        fprintf(stderr, "Failed to save output image: %s\n", output_path);
    } else {
        printf("Output image saved to: %s\n", output_path);
    }

    // Cleanup - be careful with the order and check for NULL pointers

    // Free RealNet resources
    if (is_realnet_model(model_type)) {
        if (blob) {
            sod_image_free_blob(blob);
            blob = NULL;
        }

        if (gray_img.data) {
            sod_free_image(gray_img);
            // No need to set gray_img.data = NULL as we're exiting soon
        }

        if (model) {
            unload_detection_model(model);
            model = NULL;
        }
    }
    // Free CNN resources
    else if (is_cnn_model(model_type)) {
        if (cnn_blob) {
            // The memory for cnn_blob was allocated by sod_cnn_prepare_image
            // It's managed internally by SOD, so we don't free it directly
            cnn_blob = NULL;
        }

        // NOTE: cnn_boxes is managed by SOD and should NOT be freed manually
        // It's freed when sod_cnn_destroy is called
        cnn_boxes = NULL;

        if (cnn_model) {
            sod_cnn_destroy(cnn_model);
            cnn_model = NULL;
        }
    }

    // Free common resources
    if (color_img.data) {
        sod_free_image(color_img);
    }

    // Shutdown the detection system
    shutdown_detection_system();

    return 0;
}