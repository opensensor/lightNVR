#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "video/detection.h"
#include "video/sod_realnet.h"
#include "core/logger.h"
#include "sod/sod.h"

/**
 * Test program for SOD RealNet face detection
 * 
 * Usage: ./test_sod_realnet <image_path> <model_path> [output_path]
 * 
 * Example: ./test_sod_realnet test.jpg face.realnet.sod output.jpg
 */
int main(int argc, char *argv[])
{
    if (argc < 3) {
        printf("Usage: %s <image_path> <model_path> [output_path]\n", argv[0]);
        return 1;
    }

    const char *image_path = argv[1];
    const char *model_path = argv[2];
    const char *output_path = argc > 3 ? argv[3] : "out.jpg";
    
    // Initialize detection system
    if (init_detection_system() != 0) {
        fprintf(stderr, "Failed to initialize detection system\n");
        return 1;
    }
    
    // Check if model is supported
    if (!is_model_supported(model_path)) {
        fprintf(stderr, "Model not supported: %s\n", model_path);
        return 1;
    }
    
    // Load model
    detection_model_t model = load_detection_model(model_path, 5.0f);
    if (!model) {
        fprintf(stderr, "Failed to load model: %s\n", model_path);
        return 1;
    }
    
    // SOD library is now directly linked
    
    // Load grayscale image for detection
    sod_img gray_img = sod_img_load_grayscale(image_path);
    if (gray_img.data == NULL) {
        fprintf(stderr, "Failed to load image: %s\n", image_path);
        unload_detection_model(model);
        return 1;
    }
    
    // Load color image for visualization
    sod_img color_img = sod_img_load_color(image_path);
    if (color_img.data == NULL) {
        fprintf(stderr, "Failed to load color image: %s\n", image_path);
        sod_free_image(gray_img);
        unload_detection_model(model);
        return 1;
    }
    
    // Convert image to blob
    unsigned char *blob = sod_image_to_blob(gray_img);
    if (!blob) {
        fprintf(stderr, "Failed to convert image to blob\n");
        sod_free_image(gray_img);
        sod_free_image(color_img);
        unload_detection_model(model);
        return 1;
    }
    
    // Run detection
    detection_result_t result;
    if (detect_objects(model, blob, gray_img.w, gray_img.h, gray_img.c, &result) != 0) {
        fprintf(stderr, "Detection failed\n");
        sod_image_free_blob(blob);
        sod_free_image(gray_img);
        sod_free_image(color_img);
        unload_detection_model(model);
        return 1;
    }
    
    // Process results
    printf("Detected %d faces\n", result.count);
    
    // Create SOD boxes for visualization
    // Using the sod_box type from sod.h
    
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
    
    // Cleanup
    sod_image_free_blob(blob);
    sod_free_image(gray_img);
    sod_free_image(color_img);
    unload_detection_model(model);
    shutdown_detection_system();
    
    return 0;
}
