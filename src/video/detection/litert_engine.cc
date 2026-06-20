/**
 * litert_engine.cc
 *
 * In-process LiteRT (TFLite) detection engine.
 *
 * - Refcounted registry keyed by canonical model path (one engine per
 *   unique .tflite, shared across streams that use the same model).
 * - Per-engine std::mutex for inference serialization (tflite::Interpreter
 *   is not thread-safe).
 * - Tensor introspection at init: input dims/dtype, output dims/dtype, and
 *   per-tensor quantization params (scale, zero_point) are read from the
 *   model — nothing hardcoded.
 * - Supports float32, uint8, and int8 input/output dtypes. Any other type
 *   (e.g. float16, int16) causes init to fail with a clear error.
 * - End-to-end YOLO output: [1, N, 6] = (x1, y1, x2, y2, conf, class_id).
 *   Output rank/shape mismatch causes init to fail.
 * - Letterbox preprocess preserving aspect ratio, pad with 114 (Ultralytics
 *   convention). Uses libswscale for the resize.
 * - Labels (v1): sidecar `<basename>.labels.txt` next to the .tflite, with
 *   synthetic `class_<n>` fallback. Embedded TFLite metadata reading
 *   (Ultralytics convention) is a documented follow-up.
 */

extern "C" {
#include "video/detection/litert_engine.h"
#include "core/config.h"
#include "core/logger.h"

#include <libavutil/imgutils.h>
#include <libswscale/swscale.h>
}

#include <algorithm>
#include <atomic>
#include <cerrno>
#include <chrono>
#include <cmath>
#include <cstdio>
#include <cstdlib>
#include <cstring>
#include <fstream>
#include <limits.h>
#include <memory>
#include <mutex>
#include <string>
#include <sstream>
#include <unordered_map>
#include <vector>

#ifdef HAVE_LITERT

#include "tensorflow/lite/interpreter.h"
#include "tensorflow/lite/kernels/register.h"
#include "tensorflow/lite/model_builder.h"

#ifdef HAVE_LITERT_XNNPACK
#include "tensorflow/lite/delegates/xnnpack/xnnpack_delegate.h"
#endif

#ifdef HAVE_LITERT_GPU
#include "tensorflow/lite/delegates/gpu/delegate.h"
#endif

extern config_t g_config;

namespace {

constexpr int kExpectedOutputRank = 3;   // [1, N, 6]
constexpr int kExpectedOutputCols = 6;   // (x1, y1, x2, y2, conf, cls)
constexpr int kExpectedInputRank  = 4;   // [1, H, W, 3]
constexpr int kExpectedInputCh    = 3;
constexpr uint8_t kLetterboxFill  = 114; // Ultralytics convention

struct engine_impl {
    std::string canonical_path;
    std::unique_ptr<tflite::FlatBufferModel> model;
    std::unique_ptr<tflite::Interpreter> interpreter;
    TfLiteDelegate *delegate = nullptr;        // owned; freed in dtor
    void (*delegate_deleter)(TfLiteDelegate *) = nullptr;

    std::mutex inference_mu;
    int refcount = 0;

    int input_w = 0;
    int input_h = 0;
    TfLiteType input_type  = kTfLiteNoType;
    TfLiteType output_type = kTfLiteNoType;
    float   input_scale  = 0.0f;
    int32_t input_zp     = 0;
    float   output_scale = 0.0f;
    int32_t output_zp    = 0;
    int output_num_rows  = 0;   // N in [1, N, 6]

    size_t mem_bytes = 0;       // size of the loaded model flatbuffer (weights+graph), for memory stats

    std::vector<std::string> labels;

    ~engine_impl() {
        // Interpreter must be destroyed before the delegate it references.
        interpreter.reset();
        if (delegate && delegate_deleter) {
            delegate_deleter(delegate);
        }
    }
};

std::mutex g_registry_mu;
std::unordered_map<std::string, std::unique_ptr<engine_impl>> g_registry;

// Running total of all loaded engines' memory (model flatbuffer + tensor
// arena), maintained on engine create/destroy under g_registry_mu. Lets the
// stats reader be a single lock-free load that never touches a live interpreter.
std::atomic<uint64_t> g_engine_memory_bytes{0};

std::string canonicalize_path(const char *p) {
    if (!p || !*p) return {};
    char buf[PATH_MAX];
    if (realpath(p, buf) != nullptr) return std::string(buf);
    return std::string(p);  // best-effort; engine will fail at file open
}

std::vector<std::string> load_labels_sidecar(const std::string &model_path) {
    std::vector<std::string> out;

    // Strip the .tflite extension and try <basename>.labels.txt
    std::string base = model_path;
    auto dot = base.find_last_of('.');
    if (dot != std::string::npos) base.resize(dot);
    std::string sidecar = base + ".labels.txt";

    std::ifstream f(sidecar);
    if (!f.is_open()) {
        log_info("LiteRT: no sidecar labels file at %s", sidecar.c_str());
        return out;
    }
    std::string line;
    while (std::getline(f, line)) {
        // strip trailing CR (Windows line endings)
        while (!line.empty() && (line.back() == '\r' || line.back() == '\n')) {
            line.pop_back();
        }
        if (!line.empty()) out.push_back(line);
    }
    log_info("LiteRT: loaded %zu labels from %s", out.size(), sidecar.c_str());
    return out;
}

// TODO(litert): read labels from embedded TFLite Metadata (Ultralytics
// `associated_files` convention). Until then, users of Ultralytics-exported
// models without a sidecar file get "class_<n>" labels.

const char *tflite_type_name(TfLiteType t) {
    switch (t) {
        case kTfLiteFloat32: return "float32";
        case kTfLiteUInt8:   return "uint8";
        case kTfLiteInt8:    return "int8";
        case kTfLiteFloat16: return "float16";
        case kTfLiteInt16:   return "int16";
        case kTfLiteInt32:   return "int32";
        default:             return "?";
    }
}

bool supported_io_dtype(TfLiteType t) {
    return t == kTfLiteFloat32 || t == kTfLiteUInt8 || t == kTfLiteInt8;
}

struct DelegateChoice {
    TfLiteDelegate *delegate;
    void (*deleter)(TfLiteDelegate *);
    const char *name;
};

DelegateChoice make_delegate(const char *requested) {
    DelegateChoice empty = {nullptr, nullptr, "none"};
    if (!requested || !*requested) return empty;

    if (strcmp(requested, "xnnpack") == 0) {
#ifdef HAVE_LITERT_XNNPACK
        TfLiteXNNPackDelegateOptions opts = TfLiteXNNPackDelegateOptionsDefault();
        opts.num_threads = std::max(1, g_config.detection_engine.num_threads);
        TfLiteDelegate *d = TfLiteXNNPackDelegateCreate(&opts);
        if (d) return {d, &TfLiteXNNPackDelegateDelete, "xnnpack"};
        log_warn("LiteRT: XNNPACK delegate create failed; falling back to CPU");
        return empty;
#else
        log_warn("LiteRT: delegate=xnnpack requested but not compiled in; using CPU");
        return empty;
#endif
    }
    if (strcmp(requested, "gpu") == 0) {
#ifdef HAVE_LITERT_GPU
        TfLiteGpuDelegateOptionsV2 opts = TfLiteGpuDelegateOptionsV2Default();
        opts.inference_priority1 = TFLITE_GPU_INFERENCE_PRIORITY_MIN_LATENCY;
        TfLiteDelegate *d = TfLiteGpuDelegateV2Create(&opts);
        if (d) return {d, &TfLiteGpuDelegateV2Delete, "gpu"};
        log_warn("LiteRT: GPU delegate create failed; falling back to CPU");
        return empty;
#else
        log_warn("LiteRT: delegate=gpu requested but not compiled in; using CPU");
        return empty;
#endif
    }
    if (strcmp(requested, "none") == 0) return empty;
    log_warn("LiteRT: unknown delegate '%s'; using CPU", requested);
    return empty;
}

// Initialize an engine_impl from a model path. On failure, logs the cause
// and returns nullptr. On success the engine is fully ready for detect().
std::unique_ptr<engine_impl> create_engine(const std::string &canon_path) {
    auto e = std::make_unique<engine_impl>();
    e->canonical_path = canon_path;

    e->model = tflite::FlatBufferModel::BuildFromFile(canon_path.c_str());
    if (!e->model) {
        log_error("LiteRT: failed to load model file %s", canon_path.c_str());
        return nullptr;
    }
    // Reported as the detector's memory in the system stats. Only the model
    // flatbuffer (weights + graph) is measured: it is accurate and always a
    // subset of this process's RSS. The interpreter's tensor arena is excluded
    // — there is no arena_used_bytes() in this TFLite, and summing tensor sizes
    // wildly over-counts it (the planner reuses arena memory across tensors),
    // which previously made the subtracted detector figure exceed the lightnvr
    // RSS and report 0 for the process.
    if (e->model->allocation()) {
        e->mem_bytes = e->model->allocation()->bytes();
    }

    tflite::ops::builtin::BuiltinOpResolver resolver;
    tflite::InterpreterBuilder builder(*e->model, resolver);
    builder(&e->interpreter);
    if (!e->interpreter) {
        log_error("LiteRT: InterpreterBuilder failed for %s", canon_path.c_str());
        return nullptr;
    }

    int threads = std::max(1, std::min(16, g_config.detection_engine.num_threads));
    e->interpreter->SetNumThreads(threads);

    // Delegate selection. ModifyGraphWithDelegate must happen BEFORE
    // AllocateTensors, otherwise the delegate sees no graph.
    DelegateChoice dc = make_delegate(g_config.detection_engine.delegate);
    if (dc.delegate) {
        if (e->interpreter->ModifyGraphWithDelegate(dc.delegate) != kTfLiteOk) {
            log_warn("LiteRT: ModifyGraphWithDelegate failed for delegate=%s; falling back to CPU",
                     dc.name);
            if (dc.deleter) dc.deleter(dc.delegate);
        } else {
            e->delegate = dc.delegate;
            e->delegate_deleter = dc.deleter;
        }
    }

    if (e->interpreter->AllocateTensors() != kTfLiteOk) {
        log_error("LiteRT: AllocateTensors failed for %s", canon_path.c_str());
        return nullptr;
    }

    // Inspect input tensor.
    if (e->interpreter->inputs().size() != 1) {
        log_error("LiteRT: model has %zu inputs (expected 1)",
                  e->interpreter->inputs().size());
        return nullptr;
    }
    const TfLiteTensor *in = e->interpreter->input_tensor(0);
    if (!in || !in->dims || in->dims->size != kExpectedInputRank) {
        log_error("LiteRT: input tensor has unexpected rank (got %d, expected %d)",
                  in && in->dims ? in->dims->size : -1, kExpectedInputRank);
        return nullptr;
    }
    if (in->dims->data[0] != 1 || in->dims->data[3] != kExpectedInputCh) {
        log_error("LiteRT: input shape [%d,%d,%d,%d] not [1,H,W,3]",
                  in->dims->data[0], in->dims->data[1], in->dims->data[2],
                  in->dims->data[3]);
        return nullptr;
    }
    e->input_h = in->dims->data[1];
    e->input_w = in->dims->data[2];
    e->input_type = in->type;
    if (!supported_io_dtype(e->input_type)) {
        log_error("LiteRT: unsupported input dtype %s (supported: float32, uint8, int8)",
                  tflite_type_name(e->input_type));
        return nullptr;
    }
    e->input_scale = in->params.scale;
    e->input_zp    = in->params.zero_point;

    // Inspect output tensor.
    if (e->interpreter->outputs().size() != 1) {
        log_error("LiteRT: model has %zu outputs (expected 1 end-to-end)",
                  e->interpreter->outputs().size());
        return nullptr;
    }
    const TfLiteTensor *out = e->interpreter->output_tensor(0);
    if (!out || !out->dims || out->dims->size != kExpectedOutputRank) {
        log_error("LiteRT: output tensor has unexpected rank (got %d, expected %d for end-to-end YOLO)",
                  out && out->dims ? out->dims->size : -1, kExpectedOutputRank);
        return nullptr;
    }
    if (out->dims->data[0] != 1 || out->dims->data[2] != kExpectedOutputCols) {
        log_error("LiteRT: output shape [%d,%d,%d] not [1,N,6] (end-to-end YOLO required)",
                  out->dims->data[0], out->dims->data[1], out->dims->data[2]);
        return nullptr;
    }
    e->output_num_rows = out->dims->data[1];
    e->output_type = out->type;
    if (!supported_io_dtype(e->output_type)) {
        log_error("LiteRT: unsupported output dtype %s", tflite_type_name(e->output_type));
        return nullptr;
    }
    e->output_scale = out->params.scale;
    e->output_zp    = out->params.zero_point;

    // Labels.
    e->labels = load_labels_sidecar(canon_path);

    log_info("LiteRT engine ready: model=%s input=%dx%d/%s output=[1,%d,6]/%s "
             "threads=%d delegate=%s classes=%zu",
             canon_path.c_str(), e->input_w, e->input_h, tflite_type_name(e->input_type),
             e->output_num_rows, tflite_type_name(e->output_type),
             threads, e->delegate ? "active" : "cpu", e->labels.size());

    return e;
}

// Letterbox an RGB24 frame into the engine's input tensor, recording the
// transform for later inverse mapping.
struct LetterboxParams {
    float scale;
    int   pad_x;
    int   pad_y;
    int   new_w;
    int   new_h;
};

// Resize+pad src (w,h) into dst (model_w, model_h) RGB24, fill background
// with 114. Returns the transform parameters.
bool letterbox_rgb24(const uint8_t *src, int w, int h,
                     uint8_t *dst, int model_w, int model_h,
                     LetterboxParams *params)
{
    float sx = static_cast<float>(model_w) / w;
    float sy = static_cast<float>(model_h) / h;
    float scale = std::min(sx, sy);
    int new_w = std::max(1, static_cast<int>(std::lround(w * scale)));
    int new_h = std::max(1, static_cast<int>(std::lround(h * scale)));
    int pad_x = (model_w - new_w) / 2;
    int pad_y = (model_h - new_h) / 2;

    // Background fill.
    memset(dst, kLetterboxFill, static_cast<size_t>(model_w) * model_h * 3);

    // Build a one-shot sws context. For latency-sensitive use we could
    // cache per-engine; v1 keeps it simple.
    SwsContext *sws = sws_getContext(w, h, AV_PIX_FMT_RGB24,
                                      new_w, new_h, AV_PIX_FMT_RGB24,
                                      SWS_BILINEAR, nullptr, nullptr, nullptr);
    if (!sws) {
        log_error("LiteRT: sws_getContext failed (%dx%d -> %dx%d)", w, h, new_w, new_h);
        return false;
    }

    // Scratch buffer for the resized image, then we blit into the
    // padded dst at (pad_x, pad_y).
    std::vector<uint8_t> resized(static_cast<size_t>(new_w) * new_h * 3);
    const uint8_t *src_data[1]  = { src };
    int            src_lines[1] = { w * 3 };
    uint8_t       *dst_data[1]  = { resized.data() };
    int            dst_lines[1] = { new_w * 3 };
    sws_scale(sws, src_data, src_lines, 0, h, dst_data, dst_lines);
    sws_freeContext(sws);

    for (int row = 0; row < new_h; ++row) {
        uint8_t *dst_row = dst + ((pad_y + row) * model_w + pad_x) * 3;
        const uint8_t *src_row = resized.data() + row * new_w * 3;
        memcpy(dst_row, src_row, static_cast<size_t>(new_w) * 3);
    }

    params->scale = scale;
    params->pad_x = pad_x;
    params->pad_y = pad_y;
    params->new_w = new_w;
    params->new_h = new_h;
    return true;
}

// Fill the model input tensor with the letterboxed RGB24 buffer, dtype-aware.
void fill_input_tensor(engine_impl *e, const uint8_t *lb_rgb)
{
    const int n = e->input_w * e->input_h * 3;
    switch (e->input_type) {
        case kTfLiteFloat32: {
            float *p = e->interpreter->typed_input_tensor<float>(0);
            for (int i = 0; i < n; ++i) p[i] = lb_rgb[i] / 255.0f;
            break;
        }
        case kTfLiteUInt8: {
            uint8_t *p = e->interpreter->typed_input_tensor<uint8_t>(0);
            if (e->input_scale > 0.0f && std::fabs(e->input_scale - 1.0f/255.0f) > 1e-6) {
                // Non-identity uint8 quantization. q = pixel/255/scale + zp
                for (int i = 0; i < n; ++i) {
                    float v = lb_rgb[i] / 255.0f / e->input_scale + e->input_zp;
                    int q = static_cast<int>(std::lround(v));
                    p[i] = static_cast<uint8_t>(std::clamp(q, 0, 255));
                }
            } else {
                memcpy(p, lb_rgb, n);
            }
            break;
        }
        case kTfLiteInt8: {
            int8_t *p = e->interpreter->typed_input_tensor<int8_t>(0);
            float scale = (e->input_scale > 0.0f) ? e->input_scale : (1.0f / 255.0f);
            for (int i = 0; i < n; ++i) {
                float v = lb_rgb[i] / 255.0f / scale + e->input_zp;
                int q = static_cast<int>(std::lround(v));
                p[i] = static_cast<int8_t>(std::clamp(q, -128, 127));
            }
            break;
        }
        default:
            break; // unreachable: rejected at init
    }
}

// Read row i, column c from the output tensor with dtype-aware dequant.
inline float read_out(const engine_impl *e, int i, int c) {
    int idx = i * kExpectedOutputCols + c;
    switch (e->output_type) {
        case kTfLiteFloat32: {
            const float *p = e->interpreter->typed_output_tensor<float>(0);
            return p[idx];
        }
        case kTfLiteUInt8: {
            const uint8_t *p = e->interpreter->typed_output_tensor<uint8_t>(0);
            return (static_cast<int>(p[idx]) - e->output_zp) * e->output_scale;
        }
        case kTfLiteInt8: {
            const int8_t *p = e->interpreter->typed_output_tensor<int8_t>(0);
            return (static_cast<int>(p[idx]) - e->output_zp) * e->output_scale;
        }
        default:
            return 0.0f;
    }
}

std::string label_for(const engine_impl *e, int class_id) {
    if (class_id >= 0 && static_cast<size_t>(class_id) < e->labels.size()) {
        return e->labels[class_id];
    }
    char buf[32];
    snprintf(buf, sizeof buf, "class_%d", class_id);
    return std::string(buf);
}

}  // namespace

#else  // !HAVE_LITERT
extern "C" {
#endif

extern "C" litert_engine_t *litert_engine_acquire(const char *model_path) {
#ifdef HAVE_LITERT
    if (!model_path || !*model_path) return nullptr;
    if (!g_config.detection_engine.enabled) {
        log_error("LiteRT: engine disabled in config ([detection_engine] enabled=false); "
                  "cannot load %s", model_path);
        return nullptr;
    }

    std::string canon = canonicalize_path(model_path);
    if (canon.empty()) {
        log_error("LiteRT: empty/unresolvable model path");
        return nullptr;
    }

    std::lock_guard<std::mutex> lk(g_registry_mu);
    auto it = g_registry.find(canon);
    if (it != g_registry.end()) {
        it->second->refcount++;
        log_info("LiteRT: reused engine for %s (refcount=%d)",
                 canon.c_str(), it->second->refcount);
        return reinterpret_cast<litert_engine_t *>(it->second.get());
    }

    auto e = create_engine(canon);
    if (!e) return nullptr;
    e->refcount = 1;
    engine_impl *raw = e.get();
    g_engine_memory_bytes.fetch_add(raw->mem_bytes, std::memory_order_relaxed);
    g_registry.emplace(canon, std::move(e));
    return reinterpret_cast<litert_engine_t *>(raw);
#else
    (void)model_path;
    return NULL;
#endif
}

extern "C" void litert_engine_release(litert_engine_t *engine) {
#ifdef HAVE_LITERT
    if (!engine) return;
    auto *e = reinterpret_cast<engine_impl *>(engine);
    std::lock_guard<std::mutex> lk(g_registry_mu);
    if (--e->refcount > 0) {
        log_info("LiteRT: released %s (refcount=%d)",
                 e->canonical_path.c_str(), e->refcount);
        return;
    }
    log_info("LiteRT: destroying engine for %s", e->canonical_path.c_str());
    g_engine_memory_bytes.fetch_sub(e->mem_bytes, std::memory_order_relaxed);
    g_registry.erase(e->canonical_path);  // destroys the unique_ptr
#else
    (void)engine;
#endif
}

extern "C" int litert_engine_input_width(const litert_engine_t *engine) {
#ifdef HAVE_LITERT
    if (!engine) return 0;
    return reinterpret_cast<const engine_impl *>(engine)->input_w;
#else
    (void)engine;
    return 0;
#endif
}

extern "C" int litert_engine_input_height(const litert_engine_t *engine) {
#ifdef HAVE_LITERT
    if (!engine) return 0;
    return reinterpret_cast<const engine_impl *>(engine)->input_h;
#else
    (void)engine;
    return 0;
#endif
}

extern "C" uint64_t litert_engine_registry_memory_bytes(void) {
#ifdef HAVE_LITERT
    return g_engine_memory_bytes.load(std::memory_order_relaxed);
#else
    return 0;
#endif
}

extern "C" int litert_engine_detect(litert_engine_t *engine,
                                    const uint8_t *rgb, int w, int h,
                                    float per_stream_threshold,
                                    detection_result_t *out)
{
#ifdef HAVE_LITERT
    if (!engine || !rgb || !out || w <= 0 || h <= 0) return -1;
    auto *e = reinterpret_cast<engine_impl *>(engine);
    if (!e->interpreter) return -1;

    out->count = 0;

    float threshold = std::clamp(per_stream_threshold, 0.0f, 1.0f);

    std::lock_guard<std::mutex> lk(e->inference_mu);

    // Letterbox into a CPU scratch buffer, then convert into the input tensor.
    std::vector<uint8_t> lb(static_cast<size_t>(e->input_w) * e->input_h * 3);
    LetterboxParams lp;
    if (!letterbox_rgb24(rgb, w, h, lb.data(), e->input_w, e->input_h, &lp)) {
        return -1;
    }
    fill_input_tensor(e, lb.data());

    auto t_start = std::chrono::steady_clock::now();
    if (e->interpreter->Invoke() != kTfLiteOk) {
        log_error("LiteRT: Invoke failed for %s", e->canonical_path.c_str());
        return -1;
    }
    double inference_ms = std::chrono::duration<double, std::milli>(
                              std::chrono::steady_clock::now() - t_start).count();
    log_info("LiteRT: inference %.1f ms (%s)", inference_ms, e->canonical_path.c_str());

    // Heuristic: if any box coordinate exceeds 1.5, assume pixel-space
    // (model_w/h). Otherwise treat as normalized 0..1 in model space.
    // (Ultralytics TFLite exports vary; this autodetects either.)
    float coord_max = 0.0f;
    int n = e->output_num_rows;
    for (int i = 0; i < n; ++i) {
        for (int c = 0; c < 4; ++c) {
            coord_max = std::max(coord_max, std::fabs(read_out(e, i, c)));
        }
        if (coord_max > 1.5f) break;  // enough evidence
    }
    bool in_pixels = coord_max > 1.5f;

    int written = 0;
    for (int i = 0; i < n && written < MAX_DETECTIONS; ++i) {
        float conf = read_out(e, i, 4);
        // NOTE: assumes the end-to-end YOLO head emits rows in descending
        // confidence order (the Ultralytics NMS-embedded export convention),
        // so the first sub-threshold row means all remaining rows are too. If
        // a future model emits unsorted rows this would truncate valid
        // detections — switch to `continue` there.
        if (conf < threshold) break;  // early exit on sorted E2E output
        float x1 = read_out(e, i, 0);
        float y1 = read_out(e, i, 1);
        float x2 = read_out(e, i, 2);
        float y2 = read_out(e, i, 3);
        int cls  = static_cast<int>(std::lround(read_out(e, i, 5)));

        if (!in_pixels) {
            x1 *= e->input_w; y1 *= e->input_h;
            x2 *= e->input_w; y2 *= e->input_h;
        }

        // De-letterbox: subtract pad, divide by scale → original frame px.
        x1 = (x1 - lp.pad_x) / lp.scale;
        y1 = (y1 - lp.pad_y) / lp.scale;
        x2 = (x2 - lp.pad_x) / lp.scale;
        y2 = (y2 - lp.pad_y) / lp.scale;

        // Clamp to frame and reorder if needed.
        if (x2 < x1) std::swap(x1, x2);
        if (y2 < y1) std::swap(y1, y2);
        x1 = std::clamp(x1, 0.0f, static_cast<float>(w));
        y1 = std::clamp(y1, 0.0f, static_cast<float>(h));
        x2 = std::clamp(x2, 0.0f, static_cast<float>(w));
        y2 = std::clamp(y2, 0.0f, static_cast<float>(h));
        if (x2 - x1 < 1.0f || y2 - y1 < 1.0f) continue;

        detection_t *d = &out->detections[written];
        d->confidence = conf;
        d->x      = x1 / w;
        d->y      = y1 / h;
        d->width  = (x2 - x1) / w;
        d->height = (y2 - y1) / h;
        d->track_id = -1;
        d->zone_id[0] = '\0';
        std::string lbl = label_for(e, cls);
        snprintf(d->label, MAX_LABEL_LENGTH, "%s", lbl.c_str());
        ++written;
    }

    out->count = written;
    return 0;
#else
    (void)engine; (void)rgb; (void)w; (void)h; (void)per_stream_threshold; (void)out;
    return -1;
#endif
}

#ifndef HAVE_LITERT
}  // extern "C"
#endif
