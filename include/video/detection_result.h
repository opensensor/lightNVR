#ifndef DETECTION_RESULT_H
#define DETECTION_RESULT_H

#define MAX_DETECTIONS 20
#define MAX_LABEL_LENGTH 32
#define MAX_ZONE_ID_LENGTH 32

// Structure to hold a single detection
typedef struct {
    char label[MAX_LABEL_LENGTH];  // Object class label
    float confidence;              // Detection confidence (0.0-1.0)
    float x, y, width, height;     // Bounding box (normalized 0.0-1.0)
    int track_id;                  // Tracking ID (-1 if not tracked)
    char zone_id[MAX_ZONE_ID_LENGTH]; // Zone ID (empty if not in zone)
} detection_t;

// Structure to hold multiple detections from a single frame
typedef struct {
    int count;                          // Number of detections
    detection_t detections[MAX_DETECTIONS]; // Array of detections
} detection_result_t;

#endif /* DETECTION_RESULT_H */
