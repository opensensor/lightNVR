/**
 * litert_engine.h
 *
 * C ABI for the in-process LiteRT (TFLite) detection engine.
 *
 * Consumed only by src/video/detection_model.c and src/video/detection.c.
 * Implementation in litert_engine.cc.
 *
 * All entry points are no-ops returning NULL/-1 when ENABLE_LITERT=OFF
 * (HAVE_LITERT undefined). Callers must still gate calls with
 * #ifdef HAVE_LITERT to avoid unresolved-symbol linkage.
 */
#ifndef LITERT_ENGINE_H
#define LITERT_ENGINE_H

#include <stdbool.h>
#include <stdint.h>

#include "video/detection_result.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef struct litert_engine litert_engine_t;

/* Acquire (or create) a refcounted engine for the given model file.
 * Multiple acquires of the same canonical path return the same engine,
 * with refcount bumped. Returns NULL on failure (errors logged).
 *
 * Failure modes (each logs a clear message before returning NULL):
 *   - g_config.detection_engine.enabled == false
 *   - model file missing or unreadable
 *   - flatbuffer parse failure
 *   - unsupported input/output dtype (only float32/uint8/int8)
 *   - unsupported output shape (must be [1, N, 6])
 *   - delegate ModifyGraphWithDelegate failure (after CPU fallback attempt)
 */
litert_engine_t *litert_engine_acquire(const char *model_path);

/* Decrement refcount; when it reaches 0, destroy the engine and release
 * delegate resources. Safe to call with NULL. */
void litert_engine_release(litert_engine_t *engine);

/* Model input dimensions, as read from the .tflite at init time.
 * Returns 0 if engine is NULL. */
int litert_engine_input_width(const litert_engine_t *engine);
int litert_engine_input_height(const litert_engine_t *engine);

/* Combined size of the loaded model flatbuffers (weights + graph) across all
 * engines — the detector's resident footprint for the system memory stats.
 * The tensor arena is deliberately excluded: it can't be measured accurately
 * here and over-counting it made the reported figure exceed process RSS.
 * Returns 0 when ENABLE_LITERT=OFF or no engines are loaded. Thread-safe. */
uint64_t litert_engine_registry_memory_bytes(void);

/* Run inference on a single RGB24 frame (packed HxWx3 in original frame
 * coordinates). The engine letterboxes internally to its model HxW.
 *
 * Detection count is written to out->count. Boxes are in
 * out->detections[*].x/y/width/height as normalized 0..1 in the original
 * frame coordinate space. Labels are read from a <basename>.labels.txt
 * sidecar next to the model, falling back to "class_<n>".
 *
 * per_stream_threshold is the only confidence cutoff (no engine-side
 * floor). Range 0.0..1.0; the engine internally clamps to [0, 1].
 *
 * Returns 0 on success (matching detect_objects() / detect_with_sod_model()
 * convention — non-zero is treated as failure by the UDT), or -1 on
 * error. On success out->count is 0 when no objects pass the threshold.
 *
 * Thread-safe — each engine has its own mutex; concurrent calls
 * serialize through it. */
int litert_engine_detect(litert_engine_t *engine,
                         const uint8_t *rgb, int w, int h,
                         float per_stream_threshold,
                         detection_result_t *out);

#ifdef __cplusplus
}
#endif

#endif /* LITERT_ENGINE_H */
