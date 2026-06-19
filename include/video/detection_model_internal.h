/**
 * detection_model_internal.h
 *
 * Internal shared struct definitions for detection model types.
 * This header is NOT part of the public API; it exists solely to give
 * every translation unit (detection_model.c, sod_detection.c, …) a
 * single, consistent memory layout for model_t so that void* handles
 * can be safely cast back to model_t* without type-confusion UB.
 *
 * Do NOT include this from headers that are part of the public API.
 */
#ifndef DETECTION_MODEL_INTERNAL_H
#define DETECTION_MODEL_INTERNAL_H

#include "core/config.h"   /* MAX_PATH_LENGTH */

/* ------------------------------------------------------------------
 * TFLite sub-structure (embedded in model_t union)
 *
 * Refcounted engine pointer (litert_engine_t*) owned by the LiteRT
 * registry in src/video/detection/litert_engine.cc. The per-stream
 * confidence threshold lives in the shared model_t::threshold field
 * (set at load time, passed to litert_engine_detect() by detection.c),
 * so this sub-struct only needs the engine handle.
 * ------------------------------------------------------------------ */
typedef struct {
    void *engine;                  /* litert_engine_t* (opaque)       */
} tflite_model_t;

/* ------------------------------------------------------------------
 * Canonical generic model structure
 *
 * All model handles (detection_model_t / void*) returned by load_*
 * functions point to an allocation of this type.  Every field offset
 * must be identical across every TU that casts the opaque handle.
 * ------------------------------------------------------------------ */
typedef struct {
    char type[16];          /* Model type string: "sod", "sod_realnet",
                               "tflite", "api", "onvif", "motion"     */
    union {
        void          *sod;        /* SOD CNN model handle (sod_cnn*) */
        void          *sod_realnet;/* SOD RealNet handle              */
        tflite_model_t tflite;     /* TFLite model data               */
    };
    float threshold;        /* Detection confidence threshold          */
    char  path[MAX_PATH_LENGTH]; /* Path to the model file            */
} model_t;

#endif /* DETECTION_MODEL_INTERNAL_H */

