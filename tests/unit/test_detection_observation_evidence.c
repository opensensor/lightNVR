/* INT_01 L6: a synthetic decoded frame run through the production
 * detect_on_decoded_frame() dispatch with a fake engine result yields N
 * observations sharing one evidence asset whose JPEG decodes back to the
 * inference frame (A2: compared against the exact RGB the engine saw, by
 * PSNR), and nothing is queued or allocated when evidence is disabled.
 * Real migrated DB + real L3 worker. */
#include "../../src/video/unified_detection_thread.c"

#include <sys/stat.h>
#include <libswscale/swscale.h>

#include "unity.h"
#include "core/event_identity.h"
#include "core/observation_evidence.h"
#include "database/db_core.h"
#include "database/db_observations.h"
#include "database/db_streams.h"
#include "video/detection_evidence.h"
#include "video/detection_model_internal.h"

#define FRAME_W 320
#define FRAME_H 240

static char db_path[256];
static char storage_dir[256];
static char cam_uuid[OBS_UUID_SIZE];
static unified_detection_ctx_t *ctx;
static model_t fake_model;

/* ---- fakes -------------------------------------------------------------- */

static detection_result_t fake_result;       /* what the object engine reports */
static uint8_t *seen_rgb[4];                 /* copies of every inference input */
static int seen_count;
static int object_calls, motion_calls;
static int geometry_w = 300, geometry_h = 300;

static void remember_input(const unsigned char *frame, int w, int h) {
    TEST_ASSERT_LESS_THAN_INT(4, seen_count);
    size_t size = (size_t)w * h * 3;
    seen_rgb[seen_count] = malloc(size);
    TEST_ASSERT_NOT_NULL(seen_rgb[seen_count]);
    memcpy(seen_rgb[seen_count], frame, size);
    seen_count++;
}

int __wrap_detect_objects(detection_model_t model, const unsigned char *frame,
                          int w, int h, int channels, detection_result_t *result) {
    TEST_ASSERT_NOT_NULL(model);
    TEST_ASSERT_EQUAL_INT(3, channels);
    TEST_ASSERT_EQUAL_INT(FRAME_W, w);
    TEST_ASSERT_EQUAL_INT(FRAME_H, h);
    remember_input(frame, w, h);
    object_calls++;
    *result = fake_result;
    return 0;
}

int __wrap_detect_motion(const char *stream_name, const unsigned char *frame,
                         int w, int h, int channels, time_t ts, detection_result_t *result) {
    (void)stream_name; (void)frame; (void)w; (void)h; (void)channels; (void)ts;
    motion_calls++;
    memset(result, 0, sizeof(*result));
    result->count = 1;
    snprintf(result->detections[0].label, sizeof(result->detections[0].label), "motion");
    result->detections[0].confidence = 1.0f;
    result->detections[0].x = 0.0f; result->detections[0].y = 0.0f;
    result->detections[0].width = 1.0f; result->detections[0].height = 1.0f;
    result->detections[0].track_id = -1;
    return 0;
}

int __wrap_detection_evidence_model_geometry(detection_model_t model, int *w, int *h,
                                             bool *letterbox) {
    (void)model;
    if (geometry_w <= 0) return -1;
    *w = geometry_w;
    *h = geometry_h;
    *letterbox = false;
    return 0;
}

int __wrap_filter_detections_by_zones(const char *name, detection_result_t *result) {
    (void)name; (void)result;
    return 0;
}
int __wrap_filter_detections_by_stream_objects(const char *name, detection_result_t *result) {
    (void)name; (void)result;
    return 0;
}

/* ---- helpers -------------------------------------------------------------- */

static int scalar(const char *sql) {
    sqlite3_stmt *stmt = NULL;
    int value = -999;
    pthread_mutex_lock(get_db_mutex());
    if (sqlite3_prepare_v2(get_db_handle(), sql, -1, &stmt, NULL) == SQLITE_OK &&
        sqlite3_step(stmt) == SQLITE_ROW)
        value = sqlite3_column_int(stmt, 0);
    sqlite3_finalize(stmt);
    pthread_mutex_unlock(get_db_mutex());
    TEST_ASSERT_NOT_EQUAL_INT_MESSAGE(-999, value, sql);
    return value;
}

static void text_column(const char *sql, char *out, size_t size) {
    sqlite3_stmt *stmt = NULL;
    out[0] = '\0';
    pthread_mutex_lock(get_db_mutex());
    if (sqlite3_prepare_v2(get_db_handle(), sql, -1, &stmt, NULL) == SQLITE_OK &&
        sqlite3_step(stmt) == SQLITE_ROW && sqlite3_column_text(stmt, 0))
        snprintf(out, size, "%s", (const char *)sqlite3_column_text(stmt, 0));
    sqlite3_finalize(stmt);
    pthread_mutex_unlock(get_db_mutex());
}

static void exec_sql(const char *sql) {
    char *error = NULL;
    pthread_mutex_lock(get_db_mutex());
    int rc = sqlite3_exec(get_db_handle(), sql, NULL, NULL, &error);
    pthread_mutex_unlock(get_db_mutex());
    sqlite3_free(error);
    TEST_ASSERT_EQUAL_INT_MESSAGE(SQLITE_OK, rc, sql);
}

static void wait_idle(void) {
    for (int i = 0; i < 1000; ++i) {
        observation_evidence_stats_t s;
        observation_evidence_get_stats(&s);
        if (s.backlog_items == 0 && s.drop_ring_depth == 0) {
            usleep(20000);
            observation_evidence_get_stats(&s);
            if (s.backlog_items == 0 && s.drop_ring_depth == 0) return;
        }
        usleep(10000);
    }
    TEST_FAIL_MESSAGE("evidence worker did not go idle");
}

/* Synthetic YUV420P frame: smooth gradient plus a bright 40x40 "bird" whose
 * left edge is at bird_x (the A2 moving-bird fixture moves it). */
static AVFrame *make_frame(int bird_x) {
    AVFrame *f = av_frame_alloc();
    TEST_ASSERT_NOT_NULL(f);
    f->format = AV_PIX_FMT_YUV420P;
    f->width = FRAME_W;
    f->height = FRAME_H;
    TEST_ASSERT_EQUAL_INT(0, av_frame_get_buffer(f, 0));
    for (int y = 0; y < FRAME_H; ++y)
        for (int x = 0; x < FRAME_W; ++x) {
            bool bird = x >= bird_x && x < bird_x + 40 && y >= 100 && y < 140;
            f->data[0][y * f->linesize[0] + x] = bird ? 235 : (uint8_t)(32 + (x + y) / 4);
        }
    for (int y = 0; y < FRAME_H / 2; ++y)
        for (int x = 0; x < FRAME_W / 2; ++x) {
            f->data[1][y * f->linesize[1] + x] = (uint8_t)(96 + x / 4);
            f->data[2][y * f->linesize[2] + x] = (uint8_t)(160 - y / 4);
        }
    return f;
}

static uint8_t *read_file(const char *path, size_t *size) {
    FILE *f = fopen(path, "rb");
    TEST_ASSERT_NOT_NULL_MESSAGE(f, path);
    fseek(f, 0, SEEK_END);
    long n = ftell(f);
    fseek(f, 0, SEEK_SET);
    uint8_t *buf = malloc((size_t)n);
    TEST_ASSERT_NOT_NULL(buf);
    TEST_ASSERT_EQUAL_size_t((size_t)n, fread(buf, 1, (size_t)n, f));
    fclose(f);
    *size = (size_t)n;
    return buf;
}

/* Decode a stored JPEG to packed RGB24 (caller frees). */
static uint8_t *decode_jpeg_rgb(const char *path, int *w, int *h) {
    size_t size = 0;
    uint8_t *data = read_file(path, &size);
    const AVCodec *codec = avcodec_find_decoder(AV_CODEC_ID_MJPEG);
    TEST_ASSERT_NOT_NULL(codec);
    AVCodecContext *dec = avcodec_alloc_context3(codec);
    TEST_ASSERT_EQUAL_INT(0, avcodec_open2(dec, codec, NULL));
    AVPacket *pkt = av_packet_alloc();
    AVFrame *frame = av_frame_alloc();
    TEST_ASSERT_EQUAL_INT(0, av_new_packet(pkt, (int)size));
    memcpy(pkt->data, data, size);
    TEST_ASSERT_EQUAL_INT(0, avcodec_send_packet(dec, pkt));
    TEST_ASSERT_EQUAL_INT(0, avcodec_receive_frame(dec, frame));
    *w = frame->width;
    *h = frame->height;
    uint8_t *rgb = malloc((size_t)frame->width * frame->height * 3 + RGB24_SWS_TAIL_PADDING);
    TEST_ASSERT_NOT_NULL(rgb);
    struct SwsContext *sws = sws_getContext(frame->width, frame->height, frame->format,
                                            frame->width, frame->height, AV_PIX_FMT_RGB24,
                                            SWS_BILINEAR, NULL, NULL, NULL);
    TEST_ASSERT_NOT_NULL(sws);
    uint8_t *dst[4] = {rgb, NULL, NULL, NULL};
    int dst_lines[4] = {frame->width * 3, 0, 0, 0};
    sws_scale(sws, (const uint8_t *const *)frame->data, frame->linesize, 0, frame->height,
              dst, dst_lines);
    sws_freeContext(sws);
    av_frame_free(&frame);
    av_packet_free(&pkt);
    avcodec_free_context(&dec);
    free(data);
    return rgb;
}

static double psnr(const uint8_t *a, const uint8_t *b, size_t n) {
    double mse = 0.0;
    for (size_t i = 0; i < n; ++i) {
        double d = (double)a[i] - (double)b[i];
        mse += d * d;
    }
    mse /= (double)n;
    if (mse <= 1e-12) return 99.0;
    return 10.0 * log10(255.0 * 255.0 / mse);
}

static void asset_file(const char *asset_uuid, char *out, size_t size) {
    char sql[256], rel[512];
    snprintf(sql, sizeof(sql),
             "SELECT path FROM observation_assets WHERE uuid = '%s' AND state = 'ready'",
             asset_uuid);
    text_column(sql, rel, sizeof(rel));
    TEST_ASSERT_TRUE_MESSAGE(rel[0], "asset is not ready");
    snprintf(out, size, "%s/%s", storage_dir, rel);
}

static void set_detections(int n, const char *const *labels, const int *tracks) {
    memset(&fake_result, 0, sizeof(fake_result));
    fake_result.count = n;
    for (int i = 0; i < n; ++i) {
        detection_t *d = &fake_result.detections[i];
        snprintf(d->label, sizeof(d->label), "%s", labels[i]);
        d->confidence = 0.9f - 0.1f * (float)i;
        d->x = 0.1f + 0.2f * (float)i; d->y = 0.4f;
        d->width = 0.15f; d->height = 0.2f;
        d->track_id = tracks[i];
    }
}

/* ---- fixtures ------------------------------------------------------------- */

void setUp(void) {
    exec_sql("DELETE FROM object_observations; DELETE FROM observation_assets;");
    observation_evidence_config_t cfg = { .storage_path = storage_dir };
    TEST_ASSERT_EQUAL_INT(0, observation_evidence_init(&cfg));

    ctx = calloc(1, sizeof(*ctx));
    TEST_ASSERT_NOT_NULL(ctx);
    pthread_mutex_init(&ctx->mutex, NULL);
    snprintf(ctx->stream_name, sizeof(ctx->stream_name), "feeder-east");
    snprintf(ctx->camera_uuid, sizeof(ctx->camera_uuid), "%s", cam_uuid);
    snprintf(ctx->model_path, sizeof(ctx->model_path), "/etc/lightnvr/models/ssd_mobilenet_v2.sod");
    ctx->detection_threshold = 0.5f;
    ctx->model = &fake_model;
    ctx->engine_count = 1;
    snprintf(ctx->engines[0].engine_key, sizeof(ctx->engines[0].engine_key), "legacy-primary");
    snprintf(ctx->engines[0].engine_type, sizeof(ctx->engines[0].engine_type), "object");
    snprintf(ctx->engines[0].model_path, sizeof(ctx->engines[0].model_path), "%s", ctx->model_path);
    ctx->engines[0].enabled = true;
    ctx->engines[0].threshold = 0.5f;
    atomic_store(&ctx->observation_evidence_enabled, true);
    ctx->observation_labels[0] = '\0';

    seen_count = object_calls = motion_calls = 0;
    geometry_w = geometry_h = 300;
}

void tearDown(void) {
    observation_evidence_shutdown(2000);
    for (int i = 0; i < seen_count; ++i) free(seen_rgb[i]);
    seen_count = 0;
    pthread_mutex_destroy(&ctx->mutex);
    free(ctx);
    ctx = NULL;
}

/* ---- tests ------------------------------------------------------------------- */

static void test_single_engine_batch_shares_one_asset_matching_frame(void) {
    const char *labels[] = {"bird", "bird", "squirrel"};
    const int tracks[] = {17, -1, 4};
    set_detections(3, labels, tracks);
    snprintf(ctx->observation_labels, sizeof(ctx->observation_labels), "bird");

    AVFrame *frame = make_frame(40);
    time_t now = time(NULL);
    detection_result_t result;
    TEST_ASSERT_TRUE(detect_on_decoded_frame(ctx, frame, now, &result));
    av_frame_free(&frame);
    /* The detection result (DB rows, MQTT) is untouched by eligibility. */
    TEST_ASSERT_EQUAL_INT(3, result.count);
    TEST_ASSERT_EQUAL_INT(1, object_calls);
    wait_idle();

    TEST_ASSERT_EQUAL_INT(2, scalar("SELECT count(*) FROM object_observations"));
    TEST_ASSERT_EQUAL_INT(1, scalar("SELECT count(DISTINCT asset_uuid) FROM object_observations"));
    TEST_ASSERT_EQUAL_INT(1, scalar("SELECT count(DISTINCT batch_id) FROM object_observations"));
    TEST_ASSERT_EQUAL_INT(1, scalar("SELECT count(*) FROM observation_assets WHERE state = 'ready' "
                                    "AND origin = 'inference_input' AND width = 320 AND height = 240"));
    TEST_ASSERT_EQUAL_INT(0, scalar("SELECT count(*) FROM object_observations WHERE label != 'bird'"));
    TEST_ASSERT_EQUAL_INT(2, scalar("SELECT count(*) FROM object_observations WHERE "
                                    "engine_key = 'legacy-primary' AND engine_type = 'object' AND "
                                    "model = 'ssd_mobilenet_v2.sod' AND "
                                    "stream_name_at_capture = 'feeder-east'"));
    char sql[512], value[1024];
    snprintf(sql, sizeof(sql), "SELECT count(*) FROM object_observations WHERE camera_uuid = '%s' "
             "AND observed_at_ms / 1000 = %lld", cam_uuid, (long long)now);
    TEST_ASSERT_EQUAL_INT(2, scalar(sql));
    /* Track only where track_id >= 0: namespace = engine key, session = boot UUID. */
    TEST_ASSERT_EQUAL_INT(1, scalar("SELECT count(*) FROM object_observations WHERE track_id IS NULL"));
    snprintf(sql, sizeof(sql), "SELECT count(*) FROM object_observations WHERE track_id = '17' "
             "AND track_namespace = 'legacy-primary' AND track_session = '%s' AND object_index = 0",
             detection_evidence_boot_session());
    TEST_ASSERT_EQUAL_INT(1, scalar(sql));
    /* SOD-style pre-resize is recorded against the decoded frame. */
    text_column("SELECT transform_json FROM observation_assets", value, sizeof(value));
    TEST_ASSERT_NOT_NULL_MESSAGE(strstr(value, "\"mode\":\"resize\""), value);
    TEST_ASSERT_NOT_NULL_MESSAGE(strstr(value, "\"source_width\":320"), value);
    TEST_ASSERT_NOT_NULL_MESSAGE(strstr(value, "\"inference_width\":300"), value);

    char asset[OBS_UUID_SIZE], path[1024];
    text_column("SELECT asset_uuid FROM object_observations LIMIT 1", asset, sizeof(asset));
    asset_file(asset, path, sizeof(path));
    int w = 0, h = 0;
    uint8_t *decoded = decode_jpeg_rgb(path, &w, &h);
    TEST_ASSERT_EQUAL_INT(FRAME_W, w);
    TEST_ASSERT_EQUAL_INT(FRAME_H, h);
    double q = psnr(decoded, seen_rgb[0], (size_t)w * h * 3);
    char msg[64];
    snprintf(msg, sizeof(msg), "PSNR %.2f dB", q);
    TEST_ASSERT_TRUE_MESSAGE(q > 30.0, msg);
    free(decoded);
}

/* A2 moving bird: each asset is its own inference frame, never a later one. */
static void test_moving_bird_assets_match_their_inference_frames(void) {
    const char *labels[] = {"bird"};
    const int tracks[] = {-1};
    set_detections(1, labels, tracks);
    detection_result_t result;

    AVFrame *first = make_frame(20);
    TEST_ASSERT_TRUE(detect_on_decoded_frame(ctx, first, time(NULL), &result));
    av_frame_free(&first);
    AVFrame *second = make_frame(240);
    TEST_ASSERT_TRUE(detect_on_decoded_frame(ctx, second, time(NULL), &result));
    av_frame_free(&second);
    wait_idle();

    TEST_ASSERT_EQUAL_INT(2, scalar("SELECT count(*) FROM observation_assets WHERE state = 'ready'"));
    for (int i = 0; i < 2; ++i) {
        char sql[256], asset[OBS_UUID_SIZE], path[1024];
        snprintf(sql, sizeof(sql),
                 "SELECT asset_uuid FROM object_observations ORDER BY observed_at_ms, rowid "
                 "LIMIT 1 OFFSET %d", i);
        text_column(sql, asset, sizeof(asset));
        asset_file(asset, path, sizeof(path));
        int w = 0, h = 0;
        uint8_t *decoded = decode_jpeg_rgb(path, &w, &h);
        size_t n = (size_t)w * h * 3;
        double own = psnr(decoded, seen_rgb[i], n);
        double other = psnr(decoded, seen_rgb[1 - i], n);
        char msg[96];
        snprintf(msg, sizeof(msg), "asset %d: own %.2f dB, other %.2f dB", i, own, other);
        TEST_ASSERT_TRUE_MESSAGE(own > 30.0, msg);
        TEST_ASSERT_TRUE_MESSAGE(own > other + 6.0, msg);
        free(decoded);
    }
}

static void test_disabled_submits_nothing_and_allocates_nothing(void) {
    const char *labels[] = {"bird", "bird"};
    const int tracks[] = {-1, -1};
    set_detections(2, labels, tracks);
    atomic_store(&ctx->observation_evidence_enabled, false);

    AVFrame *frame = make_frame(40);
    detection_result_t result;
    TEST_ASSERT_TRUE(detect_on_decoded_frame(ctx, frame, time(NULL), &result));
    TEST_ASSERT_EQUAL_INT(2, result.count);

    /* Enabled but no eligible label: no submit either. */
    atomic_store(&ctx->observation_evidence_enabled, true);
    snprintf(ctx->observation_labels, sizeof(ctx->observation_labels), "cat,dog");
    TEST_ASSERT_TRUE(detect_on_decoded_frame(ctx, frame, time(NULL), &result));
    av_frame_free(&frame);

    observation_evidence_stats_t s;
    observation_evidence_get_stats(&s);
    TEST_ASSERT_FALSE(s.allocated);
    TEST_ASSERT_FALSE(s.worker_running);
    TEST_ASSERT_EQUAL_UINT64(0, s.accepted);
    TEST_ASSERT_EQUAL_INT(0, scalar("SELECT count(*) FROM object_observations"));
    TEST_ASSERT_EQUAL_INT(0, scalar("SELECT count(*) FROM observation_assets"));
}

/* Multi-engine fan-out: one batch (and asset) per object engine with
 * survivors; the motion engine's result never becomes an observation. */
static void test_multi_engine_one_batch_per_object_engine(void) {
    const char *labels[] = {"bird", "bird"};
    const int tracks[] = {5, -1};
    set_detections(2, labels, tracks);

    ctx->engine_count = 3;
    const char *keys[] = {"motion", "birds-a", "birds-b"};
    const char *types[] = {"motion", "object", "object"};
    for (int i = 0; i < 3; ++i) {
        stream_detection_engine_t *e = &ctx->engines[i];
        memset(e, 0, sizeof(*e));
        snprintf(e->engine_key, sizeof(e->engine_key), "%s", keys[i]);
        snprintf(e->engine_type, sizeof(e->engine_type), "%s", types[i]);
        snprintf(e->model_path, sizeof(e->model_path), "/m/%s.tflite", keys[i]);
        e->enabled = true;
        e->threshold = 0.5f;
        e->interval_seconds = 1;
        ctx->engine_models[i] = (i > 0) ? &fake_model : NULL;
        ctx->engine_last_run[i] = 0;
    }
    geometry_w = -1;  /* no fixed input: mode none */

    AVFrame *frame = make_frame(100);
    detection_result_t result;
    TEST_ASSERT_TRUE(detect_on_decoded_frame(ctx, frame, time(NULL), &result));
    av_frame_free(&frame);
    TEST_ASSERT_EQUAL_INT(1, motion_calls);
    TEST_ASSERT_EQUAL_INT(2, object_calls);
    TEST_ASSERT_EQUAL_INT(5, result.count);  /* motion + 2 + 2, unchanged */
    wait_idle();

    TEST_ASSERT_EQUAL_INT(4, scalar("SELECT count(*) FROM object_observations"));
    TEST_ASSERT_EQUAL_INT(0, scalar("SELECT count(*) FROM object_observations "
                                    "WHERE label = 'motion' OR engine_key = 'motion'"));
    TEST_ASSERT_EQUAL_INT(2, scalar("SELECT count(DISTINCT asset_uuid) FROM object_observations"));
    TEST_ASSERT_EQUAL_INT(2, scalar("SELECT count(DISTINCT batch_id) FROM object_observations"));
    TEST_ASSERT_EQUAL_INT(1, scalar("SELECT count(DISTINCT asset_uuid) FROM object_observations "
                                    "WHERE engine_key = 'birds-a'"));
    TEST_ASSERT_EQUAL_INT(1, scalar("SELECT count(*) FROM object_observations WHERE "
                                    "engine_key = 'birds-b' AND model = 'birds-b.tflite' "
                                    "AND track_namespace = 'birds-b' AND track_id = '5'"));
    TEST_ASSERT_EQUAL_INT(2, scalar("SELECT count(*) FROM observation_assets WHERE state = 'ready' "
                                    "AND transform_json LIKE '%\"mode\":\"none\"%'"));
    /* Both copies are the shared inference frame. */
    for (int i = 0; i < 2; ++i) {
        char sql[256], asset[OBS_UUID_SIZE], path[1024];
        snprintf(sql, sizeof(sql), "SELECT asset_uuid FROM object_observations WHERE engine_key = '%s'",
                 i == 0 ? "birds-a" : "birds-b");
        text_column(sql, asset, sizeof(asset));
        asset_file(asset, path, sizeof(path));
        int w = 0, h = 0;
        uint8_t *decoded = decode_jpeg_rgb(path, &w, &h);
        TEST_ASSERT_TRUE(psnr(decoded, seen_rgb[i], (size_t)w * h * 3) > 30.0);
        free(decoded);
    }
}

static void test_settings_setter_reaches_context(void) {
    /* No running context with this name. */
    TEST_ASSERT_EQUAL_INT(-1, unified_detection_set_observation_settings("nope", true, "bird"));
}

int main(void) {
    init_logger();
    set_log_level(LOG_LEVEL_ERROR);
    snprintf(db_path, sizeof(db_path), "/tmp/lightnvr_det_evidence_%ld.db", (long)getpid());
    snprintf(storage_dir, sizeof(storage_dir), "/tmp/lightnvr_det_evidence_%ld", (long)getpid());
    unlink(db_path);
    mkdir(storage_dir, 0750);
    if (init_database(db_path) != 0) return 1;
    if (event_identity_init() != 0) return 1;

    stream_config_t *s = calloc(1, sizeof(*s));
    if (!s) return 1;
    safe_strcpy(s->name, "feeder-east", sizeof(s->name), 0);
    safe_strcpy(s->url, "rtsp://camera/stream", sizeof(s->url), 0);
    safe_strcpy(s->codec, "h264", sizeof(s->codec), 0);
    s->enabled = true;
    s->priority = 5;
    s->segment_duration = 60;
    s->observation_evidence_enabled = true;
    safe_strcpy(s->detection_object_filter, "none", sizeof(s->detection_object_filter), 0);
    if (add_stream_config(s) <= 0) return 1;
    memset(s, 0, sizeof(*s));
    if (get_stream_config_by_name("feeder-east", s) != 0) return 1;
    safe_strcpy(cam_uuid, s->camera_uuid, sizeof(cam_uuid), 0);
    free(s);
    safe_strcpy(fake_model.type, MODEL_TYPE_SOD, sizeof(fake_model.type), 0);

    UNITY_BEGIN();
    RUN_TEST(test_disabled_submits_nothing_and_allocates_nothing);
    RUN_TEST(test_single_engine_batch_shares_one_asset_matching_frame);
    RUN_TEST(test_moving_bird_assets_match_their_inference_frames);
    RUN_TEST(test_multi_engine_one_batch_per_object_engine);
    RUN_TEST(test_settings_setter_reaches_context);
    int failures = UNITY_END();

    event_identity_shutdown();
    shutdown_database();
    unlink(db_path);
    char cmd[600];
    snprintf(cmd, sizeof(cmd), "rm -rf '%s'", storage_dir);
    if (system(cmd) != 0) fprintf(stderr, "cleanup failed: %s\n", storage_dir);
    return failures;
}
