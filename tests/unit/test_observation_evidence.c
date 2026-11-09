/* INT_01 L3: evidence frame queue and encoder worker. Lock-free bounded
 * admission (count + bytes, global and per camera), drop records for
 * queue_full / budget_exhausted, the tmp + fsync + rename + dir-fsync write,
 * failure transitions and shutdown, against a real migrated database. */
#define _GNU_SOURCE
#include <dirent.h>
#include <errno.h>
#include <pthread.h>
#include <sqlite3.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/stat.h>
#include <time.h>
#include <unistd.h>

#include <libavcodec/avcodec.h>
#include <libavutil/frame.h>

#include "unity.h"
#include "core/event_identity.h"
#include "core/observation_evidence.h"
#include "database/db_core.h"
#include "database/db_observations.h"
#include "database/db_streams.h"
#include "utils/strings.h"
#include "utils/uuid.h"
#include "video/ffmpeg_utils.h"

static char db_path[256];
static char storage_dir[256];
static char cam_a[OBS_UUID_SIZE];
static char cam_b[OBS_UUID_SIZE];
static int64_t base_time_ms = 1793000000000LL;  /* 2026-10-26 UTC */

/* ---- helpers ---------------------------------------------------------- */

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

static void exec_sql(const char *sql) {
    char *error = NULL;
    pthread_mutex_lock(get_db_mutex());
    int rc = sqlite3_exec(get_db_handle(), sql, NULL, NULL, &error);
    pthread_mutex_unlock(get_db_mutex());
    if (rc != SQLITE_OK) fprintf(stderr, "%s: %s\n", sql, error ? error : "");
    sqlite3_free(error);
    TEST_ASSERT_EQUAL_INT_MESSAGE(SQLITE_OK, rc, sql);
}

static int count_assets(const char *where) {
    char sql[512];
    snprintf(sql, sizeof(sql), "SELECT count(*) FROM observation_assets WHERE %s", where);
    return scalar(sql);
}

static void make_camera(const char *name, char out[OBS_UUID_SIZE]) {
    stream_config_t *s = calloc(1, sizeof(*s));
    TEST_ASSERT_NOT_NULL(s);
    safe_strcpy(s->name, name, sizeof(s->name), 0);
    safe_strcpy(s->url, "rtsp://camera/stream", sizeof(s->url), 0);
    safe_strcpy(s->codec, "h264", sizeof(s->codec), 0);
    s->enabled = true;
    s->priority = 5;
    s->segment_duration = 60;
    s->observation_evidence_enabled = true;
    safe_strcpy(s->detection_object_filter, "none", sizeof(s->detection_object_filter), 0);
    TEST_ASSERT_GREATER_THAN(0, add_stream_config(s));
    memset(s, 0, sizeof(*s));
    TEST_ASSERT_EQUAL_INT(0, get_stream_config_by_name(name, s));
    safe_strcpy(out, s->camera_uuid, OBS_UUID_SIZE, 0);
    free(s);
}

static observation_batch_input_t make_batch(const char *camera, int objects) {
    observation_batch_input_t batch;
    memset(&batch, 0, sizeof(batch));
    TEST_ASSERT_EQUAL_INT(0, lightnvr_uuid_generate_v4(batch.batch_id));
    safe_strcpy(batch.camera_uuid, camera, sizeof(batch.camera_uuid), 0);
    batch.observed_at_ms = base_time_ms++;
    safe_strcpy(batch.engine_key, "legacy-primary", sizeof(batch.engine_key), 0);
    safe_strcpy(batch.engine_type, "object", sizeof(batch.engine_type), 0);
    safe_strcpy(batch.model, "ssd_mobilenet_v2.sod", sizeof(batch.model), 0);
    batch.object_count = objects;
    for (int i = 0; i < objects; ++i) {
        observation_object_input_t *object = &batch.objects[i];
        object->object_index = i;
        safe_strcpy(object->label, "bird", sizeof(object->label), 0);
        object->confidence = 0.8;
        object->has_box = true;
        object->box_x = 0.1; object->box_y = 0.2;
        object->box_width = 0.3; object->box_height = 0.4;
    }
    return batch;
}

/* Packed RGB24 with the swscale tail padding, like ffmpeg_frame_to_rgb24_padded(). */
static uint8_t *make_rgb(int width, int height) {
    size_t size = (size_t)width * (size_t)height * 3;
    uint8_t *buf = malloc(size + RGB24_SWS_TAIL_PADDING);
    TEST_ASSERT_NOT_NULL(buf);
    for (size_t i = 0; i < size; ++i) buf[i] = (uint8_t)((i * 7) & 0xff);
    memset(buf + size, 0, RGB24_SWS_TAIL_PADDING);
    return buf;
}

static observation_evidence_stats_t stats(void) {
    observation_evidence_stats_t s;
    observation_evidence_get_stats(&s);
    return s;
}

/* Wait until the worker has nothing queued (items and drop records). */
static void wait_idle(void) {
    for (int i = 0; i < 1000; ++i) {
        observation_evidence_stats_t s = stats();
        if (s.backlog_items == 0 && s.drop_ring_depth == 0) {
            usleep(20000);  /* let the last DB commit land */
            s = stats();
            if (s.backlog_items == 0 && s.drop_ring_depth == 0) return;
        }
        usleep(10000);
    }
    TEST_FAIL_MESSAGE("evidence worker did not go idle");
}

static void absolute_path(const char *relative, char *out, size_t size) {
    snprintf(out, size, "%s/%s", storage_dir, relative);
}

static bool file_exists(const char *path) {
    struct stat st;
    return stat(path, &st) == 0;
}

/* ---- stall / fault hooks ---------------------------------------------- */

static pthread_mutex_t hook_lock = PTHREAD_MUTEX_INITIALIZER;
static pthread_cond_t hook_cond = PTHREAD_COND_INITIALIZER;
static bool stall_enabled;
static bool stall_entered;
static bool fail_before_rename;
static bool saw_tmp_without_final;

static int test_hook(observation_evidence_stage_t stage, const char *path, void *ctx) {
    (void)ctx;
    if (stage == OBS_EVIDENCE_STAGE_ITEM_BEGIN) {
        pthread_mutex_lock(&hook_lock);
        stall_entered = true;
        pthread_cond_broadcast(&hook_cond);
        while (stall_enabled) pthread_cond_wait(&hook_cond, &hook_lock);
        pthread_mutex_unlock(&hook_lock);
        return 0;
    }
    if (stage == OBS_EVIDENCE_STAGE_BEFORE_RENAME && fail_before_rename) {
        char tmp[1024];
        snprintf(tmp, sizeof(tmp), "%s.tmp", path);
        saw_tmp_without_final = file_exists(tmp) && !file_exists(path);
        return -1;
    }
    return 0;
}

static void stall_worker(void) {
    pthread_mutex_lock(&hook_lock);
    stall_enabled = true;
    stall_entered = false;
    pthread_mutex_unlock(&hook_lock);
}

static void wait_stalled(void) {
    struct timespec deadline;
    clock_gettime(CLOCK_REALTIME, &deadline);
    deadline.tv_sec += 5;
    pthread_mutex_lock(&hook_lock);
    while (!stall_entered) {
        if (pthread_cond_timedwait(&hook_cond, &hook_lock, &deadline) != 0) break;
    }
    bool entered = stall_entered;
    pthread_mutex_unlock(&hook_lock);
    TEST_ASSERT_TRUE_MESSAGE(entered, "worker never reached the stall hook");
}

static void release_worker(void) {
    pthread_mutex_lock(&hook_lock);
    stall_enabled = false;
    pthread_cond_broadcast(&hook_cond);
    pthread_mutex_unlock(&hook_lock);
}

static bool probe_value = true;
static bool enabled_probe(const char *camera_uuid, void *ctx) {
    (void)camera_uuid; (void)ctx;
    return probe_value;
}

static void init_evidence(int per_camera_items, int64_t global_bytes) {
    observation_evidence_config_t config;
    memset(&config, 0, sizeof(config));
    config.storage_path = storage_dir;
    config.per_camera_max_items = per_camera_items;
    config.global_max_bytes = global_bytes;
    TEST_ASSERT_EQUAL_INT(0, observation_evidence_init(&config));
}

void setUp(void) {
    exec_sql("DELETE FROM object_observations; DELETE FROM observation_assets;");
    observation_evidence_set_stage_hook(test_hook, NULL);
    observation_evidence_set_enabled_probe(NULL, NULL);
    observation_evidence_set_budget_exhausted(cam_a, false);
    observation_evidence_set_budget_exhausted(cam_b, false);
    stall_enabled = false;
    fail_before_rename = false;
    saw_tmp_without_final = false;
    probe_value = true;
}

void tearDown(void) {
    release_worker();
    observation_evidence_shutdown(0);
    observation_evidence_set_stage_hook(NULL, NULL);
}

/* ---- JPEG decode -------------------------------------------------------- */

static void decode_jpeg_dims(const uint8_t *data, size_t size, int *w, int *h) {
    *w = *h = 0;
    const AVCodec *codec = avcodec_find_decoder(AV_CODEC_ID_MJPEG);
    TEST_ASSERT_NOT_NULL(codec);
    AVCodecContext *ctx = avcodec_alloc_context3(codec);
    TEST_ASSERT_NOT_NULL(ctx);
    TEST_ASSERT_EQUAL_INT(0, avcodec_open2(ctx, codec, NULL));
    AVPacket *pkt = av_packet_alloc();
    AVFrame *frame = av_frame_alloc();
    TEST_ASSERT_EQUAL_INT(0, av_new_packet(pkt, (int)size));
    memcpy(pkt->data, data, size);
    TEST_ASSERT_EQUAL_INT(0, avcodec_send_packet(ctx, pkt));
    TEST_ASSERT_EQUAL_INT(0, avcodec_receive_frame(ctx, frame));
    *w = frame->width;
    *h = frame->height;
    av_frame_free(&frame);
    av_packet_free(&pkt);
    avcodec_free_context(&ctx);
}

static uint8_t *read_file(const char *path, size_t *size) {
    FILE *f = fopen(path, "rb");
    TEST_ASSERT_NOT_NULL_MESSAGE(f, path);
    fseek(f, 0, SEEK_END);
    long len = ftell(f);
    fseek(f, 0, SEEK_SET);
    uint8_t *buf = malloc((size_t)len);
    TEST_ASSERT_NOT_NULL(buf);
    TEST_ASSERT_EQUAL_size_t((size_t)len, fread(buf, 1, (size_t)len, f));
    fclose(f);
    *size = (size_t)len;
    return buf;
}

static int count_tmp_files(const char *dir) {
    int count = 0;
    DIR *d = opendir(dir);
    if (!d) return 0;
    struct dirent *entry;
    while ((entry = readdir(d)) != NULL) {
        size_t len = strlen(entry->d_name);
        if (len > 8 && strcmp(entry->d_name + len - 8, ".jpg.tmp") == 0) count++;
    }
    closedir(d);
    return count;
}

/* ---- tests -------------------------------------------------------------- */

static void test_disabled_mode_allocates_nothing(void) {
    /* Not initialized: DISABLED, buffer freed (ASan proves no leak). */
    observation_evidence_shutdown(0);
    observation_batch_input_t batch = make_batch(cam_a, 1);
    char asset[OBS_UUID_SIZE] = "x";
    TEST_ASSERT_EQUAL_INT(OBS_EVIDENCE_DISABLED,
        observation_evidence_submit(&batch, make_rgb(64, 48), 64, 48, NULL, asset));
    TEST_ASSERT_EQUAL_STRING("", asset);

    init_evidence(0, 0);
    observation_evidence_stats_t s = stats();
    TEST_ASSERT_TRUE(s.initialized);
    TEST_ASSERT_FALSE(s.worker_running);
    TEST_ASSERT_FALSE(s.allocated);
    TEST_ASSERT_EQUAL_UINT64(0, s.accepted);
    /* Budget flags live in static memory and do not start the worker. */
    observation_evidence_set_budget_exhausted(cam_a, true);
    TEST_ASSERT_TRUE(observation_evidence_is_budget_exhausted(cam_a));
    s = stats();
    TEST_ASSERT_FALSE(s.allocated);
    TEST_ASSERT_FALSE(s.worker_running);
    TEST_ASSERT_EQUAL_UINT32(1, s.budget_exhausted_cameras);
    observation_evidence_set_budget_exhausted(cam_a, false);
    TEST_ASSERT_EQUAL_INT(0, count_assets("1"));
}

static void test_ready_jpeg_decodes_to_frame_dimensions(void) {
    init_evidence(0, 0);
    observation_batch_input_t batch = make_batch(cam_a, 2);
    char asset[OBS_UUID_SIZE] = "";
    TEST_ASSERT_EQUAL_INT(OBS_EVIDENCE_ACCEPTED,
        observation_evidence_submit(&batch, make_rgb(320, 240), 320, 240, NULL, asset));
    TEST_ASSERT_TRUE(lightnvr_uuid_is_valid(asset));
    TEST_ASSERT_TRUE(stats().allocated);  /* lazily started by the first submit */
    wait_idle();
    TEST_ASSERT_TRUE(stats().worker_running);

    observation_asset_record_t record;
    TEST_ASSERT_EQUAL_INT(DB_OBS_OK, db_observation_asset_get(asset, &record));
    TEST_ASSERT_EQUAL_INT(OBS_ASSET_READY, record.state);
    TEST_ASSERT_EQUAL_INT(320, record.width);
    TEST_ASSERT_EQUAL_INT(240, record.height);
    TEST_ASSERT_EQUAL_STRING("inference_input", record.origin);
    TEST_ASSERT_EQUAL_STRING("image/jpeg", record.media_type);
    TEST_ASSERT_NOT_NULL(strstr(record.transform_json, "\"mode\":\"none\""));

    char expected[512];
    TEST_ASSERT_EQUAL_INT(0, observation_evidence_relative_path(cam_a, asset,
                              batch.observed_at_ms, expected, sizeof(expected)));
    TEST_ASSERT_EQUAL_STRING(expected, record.path);
    char layout[512];
    snprintf(layout, sizeof(layout), "observations/%s/2026-10-26/%s.jpg", cam_a, asset);
    TEST_ASSERT_EQUAL_STRING(layout, record.path);

    char path[1024];
    absolute_path(record.path, path, sizeof(path));
    size_t size = 0;
    uint8_t *jpeg = read_file(path, &size);
    TEST_ASSERT_EQUAL_INT64((int64_t)size, record.bytes);
    int w = 0, h = 0;
    decode_jpeg_dims(jpeg, size, &w, &h);
    free(jpeg);
    TEST_ASSERT_EQUAL_INT(320, w);
    TEST_ASSERT_EQUAL_INT(240, h);

    /* Both observations of the batch share the asset and moved to revision 2. */
    char where[256];
    snprintf(where, sizeof(where),
             "SELECT count(*) FROM object_observations WHERE asset_uuid = '%s' AND revision = 2",
             asset);
    TEST_ASSERT_EQUAL_INT(2, scalar(where));

    char *slash = strrchr(path, '/');
    *slash = '\0';
    TEST_ASSERT_EQUAL_INT(0, count_tmp_files(path));
    observation_evidence_stats_t s = stats();
    TEST_ASSERT_EQUAL_UINT64(1, s.accepted);
    TEST_ASSERT_EQUAL_UINT64(1, s.ready);
    TEST_ASSERT_EQUAL_UINT64(size, s.bytes_written);
}

static void test_jpeg_variant_stores_exact_bytes(void) {
    init_evidence(0, 0);
    /* Produce a real JPEG with the shared encoder, then submit those bytes. */
    uint8_t *rgb = make_rgb(160, 120);
    jpeg_encoder_cache_t *encoder = jpeg_encoder_cache_create(160, 120, 3, 80);
    TEST_ASSERT_NOT_NULL(encoder);
    unsigned char *jpeg = NULL;
    size_t jpeg_size = 0;
    TEST_ASSERT_EQUAL_INT(0, jpeg_encoder_cache_encode_to_memory(encoder, rgb, &jpeg, &jpeg_size));
    jpeg_encoder_cache_destroy(encoder);
    free(rgb);
    uint8_t *copy = malloc(jpeg_size);
    memcpy(copy, jpeg, jpeg_size);

    observation_evidence_transform_t transform = {
        .mode = OBS_TRANSFORM_LETTERBOX, .source_width = 1920, .source_height = 1080,
        .inference_width = 640, .inference_height = 640,
        .scale_x = 4.0, .scale_y = 4.0, .pad_x = 0, .pad_y = 140,
    };
    observation_batch_input_t batch = make_batch(cam_a, 1);
    safe_strcpy(batch.engine_type, "api", sizeof(batch.engine_type), 0);
    char asset[OBS_UUID_SIZE];
    TEST_ASSERT_EQUAL_INT(OBS_EVIDENCE_ACCEPTED,
        observation_evidence_submit_jpeg(&batch, copy, jpeg_size, 160, 120, &transform, asset));
    wait_idle();

    observation_asset_record_t record;
    TEST_ASSERT_EQUAL_INT(DB_OBS_OK, db_observation_asset_get(asset, &record));
    TEST_ASSERT_EQUAL_INT(OBS_ASSET_READY, record.state);
    TEST_ASSERT_EQUAL_STRING("submitted_image", record.origin);
    TEST_ASSERT_NOT_NULL(strstr(record.transform_json, "\"mode\":\"letterbox\""));
    TEST_ASSERT_NOT_NULL(strstr(record.transform_json, "\"pad_y\":140"));
    char path[1024];
    absolute_path(record.path, path, sizeof(path));
    size_t size = 0;
    uint8_t *stored = read_file(path, &size);
    TEST_ASSERT_EQUAL_size_t(jpeg_size, size);
    TEST_ASSERT_EQUAL_MEMORY(jpeg, stored, size);
    free(stored);
    free(jpeg);
}

static void test_queue_saturation_drops_without_blocking(void) {
    init_evidence(3, 0);
    stall_worker();
    char asset[OBS_UUID_SIZE];
    observation_batch_input_t batch = make_batch(cam_a, 1);
    TEST_ASSERT_EQUAL_INT(OBS_EVIDENCE_ACCEPTED,
        observation_evidence_submit(&batch, make_rgb(64, 48), 64, 48, NULL, asset));
    wait_stalled();

    /* The item being processed still holds its reservation: 2 more fit.
     * Submissions must return while the worker remains stalled. CTest's
     * timeout catches a producer waiting for worker progress; the Release
     * performance harness owns the sub-millisecond wall-clock budget. */
    int accepted = 1, dropped = 0;
    char dropped_asset[OBS_UUID_SIZE] = "";
    for (int i = 0; i < 6; ++i) {
        observation_batch_input_t b = make_batch(cam_a, 1);
        uint8_t *rgb = make_rgb(64, 48);
        observation_evidence_result_t rc =
            observation_evidence_submit(&b, rgb, 64, 48, NULL, asset);
        if (rc == OBS_EVIDENCE_ACCEPTED) accepted++;
        else {
            TEST_ASSERT_EQUAL_INT(OBS_EVIDENCE_DROPPED_QUEUE_FULL, rc);
            TEST_ASSERT_TRUE(lightnvr_uuid_is_valid(asset));
            safe_strcpy(dropped_asset, asset, sizeof(dropped_asset), 0);
            dropped++;
        }
    }
    TEST_ASSERT_EQUAL_INT(3, accepted);
    TEST_ASSERT_EQUAL_INT(4, dropped);

    /* Another camera has its own depth. */
    observation_batch_input_t other = make_batch(cam_b, 1);
    TEST_ASSERT_EQUAL_INT(OBS_EVIDENCE_ACCEPTED,
        observation_evidence_submit(&other, make_rgb(64, 48), 64, 48, NULL, asset));

    observation_evidence_stats_t s = stats();
    TEST_ASSERT_EQUAL_UINT64(4, s.accepted);
    TEST_ASSERT_EQUAL_UINT64(4, s.dropped_queue_full);
    TEST_ASSERT_EQUAL_UINT32(4, s.backlog_items);
    TEST_ASSERT_EQUAL_UINT32(4, s.high_water_items);
    TEST_ASSERT_EQUAL_UINT32(4, s.drop_ring_depth);

    release_worker();
    wait_idle();
    TEST_ASSERT_EQUAL_INT(4, count_assets("state = 'ready'"));
    TEST_ASSERT_EQUAL_INT(4, count_assets("state = 'unavailable' AND reason = 'queue_full' "
                                          "AND path IS NULL"));
    observation_asset_record_t record;
    TEST_ASSERT_EQUAL_INT(DB_OBS_OK, db_observation_asset_get(dropped_asset, &record));
    TEST_ASSERT_EQUAL_INT(OBS_ASSET_UNAVAILABLE, record.state);
}

/* X2 (A11): every admission that reaches the queue is timed into a
 * fixed-bucket histogram, so the saturation gate can prove from the running
 * binary that producers never wait on I/O. The configured limits and the
 * per-camera high-water mark are reported so "limits hold" is checkable too.
 * Check metric consistency here, including under sanitizers. Scheduling and
 * instrumentation affect wall time (especially lazy worker startup); the
 * Release evidence gate in tests/performance/observations enforces < 1 ms. */
static void test_submit_latency_histogram_and_limits(void) {
    observation_evidence_config_t config;
    memset(&config, 0, sizeof(config));
    config.storage_path = storage_dir;
    config.per_camera_max_items = 2;
    config.global_max_items = 5;
    config.global_max_bytes = 500000;
    TEST_ASSERT_EQUAL_INT(0, observation_evidence_init(&config));

    observation_evidence_stats_t s = stats();
    TEST_ASSERT_EQUAL_UINT64(0, s.submit_latency_count);
    TEST_ASSERT_EQUAL_UINT32(5, s.limit_items);
    TEST_ASSERT_EQUAL_UINT32(2, s.limit_camera_items);
    TEST_ASSERT_EQUAL_UINT64(500000, s.limit_bytes);
    TEST_ASSERT_EQUAL_UINT64(500000, s.limit_camera_bytes);  /* defaults to global */
    TEST_ASSERT_EQUAL_UINT32(0, s.camera_high_water_items);

    stall_worker();
    char asset[OBS_UUID_SIZE];
    observation_batch_input_t batch = make_batch(cam_a, 1);
    TEST_ASSERT_EQUAL_INT(OBS_EVIDENCE_ACCEPTED,
        observation_evidence_submit(&batch, make_rgb(64, 48), 64, 48, NULL, asset));
    wait_stalled();
    int calls = 1;
    for (int i = 0; i < 4; ++i, ++calls) {  /* 1 accepted, 3 queue_full */
        observation_batch_input_t b = make_batch(cam_a, 1);
        observation_evidence_submit(&b, make_rgb(64, 48), 64, 48, NULL, asset);
    }
    observation_batch_input_t onvif = make_batch(cam_b, 1);
    TEST_ASSERT_EQUAL_INT(OBS_EVIDENCE_ACCEPTED,
        observation_evidence_submit_unavailable(&onvif, "no_matching_media", asset));
    calls++;
    uint8_t *jpeg = malloc(16);
    TEST_ASSERT_NOT_NULL(jpeg);
    memset(jpeg, 0xff, 16);
    observation_batch_input_t api = make_batch(cam_b, 1);
    TEST_ASSERT_EQUAL_INT(OBS_EVIDENCE_ACCEPTED,
        observation_evidence_submit_jpeg(&api, jpeg, 16, 8, 8, NULL, asset));
    calls++;

    s = stats();
    TEST_ASSERT_EQUAL_UINT64((uint64_t)calls, s.submit_latency_count);
    uint64_t in_buckets = 0;
    for (int i = 0; i < OBS_EVIDENCE_SUBMIT_LATENCY_BUCKETS; ++i)
        in_buckets += s.submit_latency_bucket[i];
    TEST_ASSERT_EQUAL_UINT64(s.submit_latency_count, in_buckets);
    TEST_ASSERT_TRUE(s.submit_latency_max_ns > 0);
    TEST_ASSERT_TRUE(s.submit_latency_sum_ns >= s.submit_latency_max_ns);
    TEST_ASSERT_TRUE(s.submit_latency_sum_ns <=
                     s.submit_latency_count * s.submit_latency_max_ns);
    /* Bounds are strictly increasing and end at the +Inf bucket. */
    for (int i = 1; i < OBS_EVIDENCE_SUBMIT_LATENCY_BUCKETS - 1; ++i)
        TEST_ASSERT_TRUE(observation_evidence_submit_latency_bounds_ns[i] >
                         observation_evidence_submit_latency_bounds_ns[i - 1]);
    /* The maximum belongs to the highest occupied bucket, including +Inf. */
    int max_bucket = OBS_EVIDENCE_SUBMIT_LATENCY_BUCKETS - 1;
    for (int i = 0; i < OBS_EVIDENCE_SUBMIT_LATENCY_BUCKETS - 1; ++i) {
        if (s.submit_latency_max_ns <= observation_evidence_submit_latency_bounds_ns[i]) {
            max_bucket = i;
            break;
        }
    }
    TEST_ASSERT_TRUE(s.submit_latency_bucket[max_bucket] > 0);
    for (int i = max_bucket + 1; i < OBS_EVIDENCE_SUBMIT_LATENCY_BUCKETS; ++i)
        TEST_ASSERT_EQUAL_UINT64(0, s.submit_latency_bucket[i]);
    TEST_ASSERT_EQUAL_UINT32(2, s.camera_high_water_items);
    TEST_ASSERT_TRUE(s.high_water_items <= s.limit_items);
    TEST_ASSERT_TRUE(s.high_water_bytes <= s.limit_bytes);

    release_worker();
    wait_idle();
    observation_evidence_shutdown(0);
    init_evidence(0, 0);  /* counters reset with the process-run counters */
    s = stats();
    TEST_ASSERT_EQUAL_UINT64(0, s.submit_latency_count);
    TEST_ASSERT_EQUAL_UINT64(0, s.submit_latency_max_ns);
    TEST_ASSERT_EQUAL_UINT32(0, s.camera_high_water_items);
    TEST_ASSERT_EQUAL_UINT32(OBS_EVIDENCE_GLOBAL_DEPTH, s.limit_items);
    TEST_ASSERT_EQUAL_UINT32(OBS_EVIDENCE_PER_CAMERA_DEPTH, s.limit_camera_items);
    TEST_ASSERT_EQUAL_UINT64((uint64_t)OBS_EVIDENCE_DEFAULT_GLOBAL_BYTES, s.limit_bytes);
}

/* X2 finding: free() of a dropped 720p RGB frame (2.7 MB, an munmap) took
 * 1.5-2.6 ms on the detection thread. Rejected frames are handed to the worker
 * for release, bounded by limit_discard_bytes (max(queue bytes, 32 MiB));
 * past that bound the producer frees inline. */
static void test_rejected_frames_are_released_by_the_worker(void) {
    init_evidence(1, 0);
    stall_worker();
    char asset[OBS_UUID_SIZE];
    observation_batch_input_t first = make_batch(cam_a, 1);
    TEST_ASSERT_EQUAL_INT(OBS_EVIDENCE_ACCEPTED,
        observation_evidence_submit(&first, make_rgb(32, 24), 32, 24, NULL, asset));
    wait_stalled();

    const int w = 1280, h = 720;
    const uint64_t frame = (uint64_t)w * h * 3;
    observation_evidence_stats_t s = stats();
    TEST_ASSERT_EQUAL_UINT64((uint64_t)OBS_EVIDENCE_DEFAULT_GLOBAL_BYTES, s.limit_discard_bytes);
    const int fit = (int)(s.limit_discard_bytes / frame);  /* 12 frames in 32 MiB */
    for (int i = 0; i < fit + 2; ++i) {
        observation_batch_input_t b = make_batch(cam_a, 1);
        TEST_ASSERT_EQUAL_INT(OBS_EVIDENCE_DROPPED_QUEUE_FULL,
            observation_evidence_submit(&b, make_rgb(w, h), w, h, NULL, asset));
    }
    s = stats();
    TEST_ASSERT_EQUAL_UINT64((uint64_t)fit, s.discard_deferred);
    TEST_ASSERT_EQUAL_UINT64(2, s.discard_inline);
    TEST_ASSERT_EQUAL_UINT64((uint64_t)fit * frame, s.discard_pending_bytes);
    TEST_ASSERT_TRUE(s.discard_pending_bytes <= s.limit_discard_bytes);

    release_worker();
    wait_idle();
    s = stats();
    TEST_ASSERT_EQUAL_UINT64(0, s.discard_pending_bytes);
    TEST_ASSERT_EQUAL_INT(fit + 2, count_assets("reason = 'queue_full'"));

    /* A larger queue budget raises the deferral bound with it. */
    observation_evidence_shutdown(0);
    init_evidence(0, 64LL * 1024 * 1024);
    TEST_ASSERT_EQUAL_UINT64(64ULL * 1024 * 1024, stats().limit_discard_bytes);
}

static void test_global_byte_budget_bounds_admission(void) {
    /* 64x48 RGB24 = 9216 bytes (+ overhead); a 20000-byte budget fits one. */
    init_evidence(0, 20000);
    stall_worker();
    char asset[OBS_UUID_SIZE];
    observation_batch_input_t a = make_batch(cam_a, 1);
    observation_batch_input_t b = make_batch(cam_b, 1);
    TEST_ASSERT_EQUAL_INT(OBS_EVIDENCE_ACCEPTED,
        observation_evidence_submit(&a, make_rgb(64, 48), 64, 48, NULL, asset));
    wait_stalled();
    TEST_ASSERT_EQUAL_INT(OBS_EVIDENCE_DROPPED_QUEUE_FULL,
        observation_evidence_submit(&b, make_rgb(64, 48), 64, 48, NULL, asset));
    /* A frame larger than the whole budget never fits. */
    observation_batch_input_t c = make_batch(cam_b, 1);
    TEST_ASSERT_EQUAL_INT(OBS_EVIDENCE_DROPPED_QUEUE_FULL,
        observation_evidence_submit(&c, make_rgb(128, 96), 128, 96, NULL, asset));
    TEST_ASSERT_TRUE(stats().high_water_bytes > 9216);
    release_worker();
    wait_idle();
    TEST_ASSERT_EQUAL_INT(1, count_assets("state = 'ready'"));
    TEST_ASSERT_EQUAL_INT(2, count_assets("reason = 'queue_full'"));
}

static void test_drop_ring_overflow_counts_untracked(void) {
    init_evidence(1, 0);
    stall_worker();
    char asset[OBS_UUID_SIZE];
    observation_batch_input_t first = make_batch(cam_a, 1);
    TEST_ASSERT_EQUAL_INT(OBS_EVIDENCE_ACCEPTED,
        observation_evidence_submit(&first, make_rgb(32, 32), 32, 32, NULL, asset));
    wait_stalled();
    int total = OBS_EVIDENCE_DROP_RING_SIZE + 10;
    for (int i = 0; i < total; ++i) {
        observation_batch_input_t b = make_batch(cam_a, 1);
        TEST_ASSERT_EQUAL_INT(OBS_EVIDENCE_DROPPED_QUEUE_FULL,
            observation_evidence_submit(&b, make_rgb(32, 32), 32, 32, NULL, asset));
    }
    observation_evidence_stats_t s = stats();
    TEST_ASSERT_EQUAL_UINT64((uint64_t)total, s.dropped_queue_full);
    TEST_ASSERT_EQUAL_UINT64(10, s.dropped_untracked);
    TEST_ASSERT_EQUAL_UINT32(OBS_EVIDENCE_DROP_RING_SIZE, s.drop_ring_depth);
    /* Tracked drops hand their frame to the worker; untracked ones (ring
     * full) can only be freed inline. */
    TEST_ASSERT_EQUAL_UINT64(OBS_EVIDENCE_DROP_RING_SIZE, s.discard_deferred);
    TEST_ASSERT_EQUAL_UINT64(10, s.discard_inline);
    release_worker();
    wait_idle();
    char where[64];
    snprintf(where, sizeof(where), "reason = 'queue_full'");
    TEST_ASSERT_EQUAL_INT(OBS_EVIDENCE_DROP_RING_SIZE, count_assets(where));
}

static void test_budget_flag_drops_as_budget_exhausted(void) {
    init_evidence(0, 0);
    observation_evidence_set_budget_exhausted(cam_a, true);
    char asset[OBS_UUID_SIZE];
    observation_batch_input_t batch = make_batch(cam_a, 2);
    TEST_ASSERT_EQUAL_INT(OBS_EVIDENCE_DROPPED_BUDGET,
        observation_evidence_submit(&batch, make_rgb(64, 48), 64, 48, NULL, asset));
    /* Unaffected camera still accepted. */
    observation_batch_input_t other = make_batch(cam_b, 1);
    TEST_ASSERT_EQUAL_INT(OBS_EVIDENCE_ACCEPTED,
        observation_evidence_submit(&other, make_rgb(64, 48), 64, 48, NULL, NULL));
    wait_idle();
    observation_asset_record_t record;
    TEST_ASSERT_EQUAL_INT(DB_OBS_OK, db_observation_asset_get(asset, &record));
    TEST_ASSERT_EQUAL_INT(OBS_ASSET_UNAVAILABLE, record.state);
    TEST_ASSERT_EQUAL_STRING("budget_exhausted", record.reason);
    TEST_ASSERT_EQUAL_INT(2, scalar("SELECT count(*) FROM object_observations o JOIN "
                                    "observation_assets a ON a.uuid = o.asset_uuid "
                                    "WHERE a.reason = 'budget_exhausted'"));
    observation_evidence_stats_t s = stats();
    TEST_ASSERT_EQUAL_UINT64(1, s.dropped_budget);
    TEST_ASSERT_EQUAL_UINT64(1, s.accepted);
}

static void test_failure_before_rename_never_leaves_final_file(void) {
    init_evidence(0, 0);
    fail_before_rename = true;
    char asset[OBS_UUID_SIZE];
    observation_batch_input_t batch = make_batch(cam_a, 1);
    TEST_ASSERT_EQUAL_INT(OBS_EVIDENCE_ACCEPTED,
        observation_evidence_submit(&batch, make_rgb(64, 48), 64, 48, NULL, asset));
    wait_idle();
    TEST_ASSERT_TRUE_MESSAGE(saw_tmp_without_final,
                             "tmp must be complete and the final name absent before rename");

    observation_asset_record_t record;
    TEST_ASSERT_EQUAL_INT(DB_OBS_OK, db_observation_asset_get(asset, &record));
    TEST_ASSERT_EQUAL_INT(OBS_ASSET_UNAVAILABLE, record.state);
    TEST_ASSERT_EQUAL_STRING("write_failed", record.reason);

    char relative[512], path[1024];
    observation_evidence_relative_path(cam_a, asset, batch.observed_at_ms, relative, sizeof(relative));
    absolute_path(relative, path, sizeof(path));
    TEST_ASSERT_FALSE(file_exists(path));
    char tmp[1100];
    snprintf(tmp, sizeof(tmp), "%s.tmp", path);
    TEST_ASSERT_FALSE(file_exists(tmp));
    TEST_ASSERT_EQUAL_UINT64(1, stats().failed_write);
}

static void test_disabled_between_admission_and_encode(void) {
    init_evidence(0, 0);
    observation_evidence_set_enabled_probe(enabled_probe, NULL);
    probe_value = false;
    char asset[OBS_UUID_SIZE];
    observation_batch_input_t batch = make_batch(cam_a, 1);
    TEST_ASSERT_EQUAL_INT(OBS_EVIDENCE_ACCEPTED,
        observation_evidence_submit(&batch, make_rgb(64, 48), 64, 48, NULL, asset));
    wait_idle();
    observation_asset_record_t record;
    TEST_ASSERT_EQUAL_INT(DB_OBS_OK, db_observation_asset_get(asset, &record));
    TEST_ASSERT_EQUAL_INT(OBS_ASSET_UNAVAILABLE, record.state);
    TEST_ASSERT_EQUAL_STRING("disabled", record.reason);
    TEST_ASSERT_EQUAL_UINT64(1, stats().disabled_at_encode);
}

static void test_stock_probe_reads_stream_setting(void) {
    TEST_ASSERT_TRUE(observation_evidence_stream_enabled_probe(cam_a, NULL));
    char sql[256];
    snprintf(sql, sizeof(sql),
             "UPDATE streams SET observation_evidence_enabled = 0 WHERE camera_uuid = '%s'", cam_a);
    exec_sql(sql);
    TEST_ASSERT_FALSE(observation_evidence_stream_enabled_probe(cam_a, NULL));
    snprintf(sql, sizeof(sql),
             "UPDATE streams SET observation_evidence_enabled = 1 WHERE camera_uuid = '%s'", cam_a);
    exec_sql(sql);
    TEST_ASSERT_FALSE(observation_evidence_stream_enabled_probe(
        "00000000-0000-4000-8000-000000000000", NULL));
}

static void test_unavailable_submission_records_reason(void) {
    init_evidence(0, 0);
    char asset[OBS_UUID_SIZE];
    observation_batch_input_t batch = make_batch(cam_a, 1);
    safe_strcpy(batch.engine_type, "onvif", sizeof(batch.engine_type), 0);
    TEST_ASSERT_EQUAL_INT(OBS_EVIDENCE_INVALID,
        observation_evidence_submit_unavailable(&batch, "write_failed", asset));
    TEST_ASSERT_EQUAL_INT(OBS_EVIDENCE_ACCEPTED,
        observation_evidence_submit_unavailable(&batch, "no_matching_media", asset));
    wait_idle();
    observation_asset_record_t record;
    TEST_ASSERT_EQUAL_INT(DB_OBS_OK, db_observation_asset_get(asset, &record));
    TEST_ASSERT_EQUAL_INT(OBS_ASSET_UNAVAILABLE, record.state);
    TEST_ASSERT_EQUAL_STRING("no_matching_media", record.reason);
    TEST_ASSERT_EQUAL_STRING("", record.transform_json);
}

static void test_invalid_arguments_free_buffer(void) {
    init_evidence(0, 0);
    observation_batch_input_t batch = make_batch(cam_a, 1);
    TEST_ASSERT_EQUAL_INT(OBS_EVIDENCE_INVALID,
        observation_evidence_submit(&batch, make_rgb(8, 8), 0, 8, NULL, NULL));
    TEST_ASSERT_EQUAL_INT(OBS_EVIDENCE_INVALID,
        observation_evidence_submit(NULL, make_rgb(8, 8), 8, 8, NULL, NULL));
    batch.object_count = 0;
    TEST_ASSERT_EQUAL_INT(OBS_EVIDENCE_INVALID,
        observation_evidence_submit(&batch, make_rgb(8, 8), 8, 8, NULL, NULL));
    TEST_ASSERT_FALSE(stats().allocated);
}

static void *shutdown_thread(void *arg) {
    (void)arg;
    observation_evidence_shutdown(0);
    return NULL;
}

static void test_shutdown_with_full_queue_marks_interrupted(void) {
    init_evidence(4, 0);
    stall_worker();
    char asset[OBS_UUID_SIZE];
    for (int i = 0; i < 4; ++i) {
        observation_batch_input_t b = make_batch(cam_a, 1);
        TEST_ASSERT_EQUAL_INT(OBS_EVIDENCE_ACCEPTED,
            observation_evidence_submit(&b, make_rgb(64, 48), 64, 48, NULL, asset));
        if (i == 0) wait_stalled();
    }
    observation_batch_input_t over = make_batch(cam_a, 1);
    TEST_ASSERT_EQUAL_INT(OBS_EVIDENCE_DROPPED_QUEUE_FULL,
        observation_evidence_submit(&over, make_rgb(64, 48), 64, 48, NULL, asset));
    /* Stop admission first, then let the stalled worker continue: with no
     * drain budget everything still queued is abandoned as interrupted. */
    pthread_t stopper;
    TEST_ASSERT_EQUAL_INT(0, pthread_create(&stopper, NULL, shutdown_thread, NULL));
    for (int i = 0; i < 500 && stats().initialized; ++i) usleep(2000);
    TEST_ASSERT_FALSE(stats().initialized);
    release_worker();
    pthread_join(stopper, NULL);

    observation_evidence_stats_t s = stats();
    TEST_ASSERT_FALSE(s.worker_running);
    TEST_ASSERT_FALSE(s.allocated);
    TEST_ASSERT_EQUAL_UINT32(0, s.backlog_items);
    TEST_ASSERT_EQUAL_UINT64(0, s.backlog_bytes);
    /* Every admitted batch is visible: ready, or unavailable(interrupted). */
    TEST_ASSERT_EQUAL_UINT64(4, s.ready + s.interrupted);
    TEST_ASSERT_TRUE(s.interrupted >= 3);
    TEST_ASSERT_EQUAL_INT((int)s.interrupted,
                          count_assets("state = 'unavailable' AND reason = 'interrupted'"));
    TEST_ASSERT_EQUAL_INT(1, count_assets("reason = 'queue_full'"));
    TEST_ASSERT_EQUAL_INT(0, count_assets("state = 'pending'"));

    TEST_ASSERT_EQUAL_INT(OBS_EVIDENCE_DISABLED,
        observation_evidence_submit(&over, make_rgb(8, 8), 8, 8, NULL, asset));
}

static void test_relative_path_and_transform_json(void) {
    char out[256];
    TEST_ASSERT_EQUAL_INT(0, observation_evidence_relative_path(
        "22222222-2222-4222-8222-222222222222", "88888888-8888-4888-8888-888888888888",
        1794146592481LL, out, sizeof(out)));
    TEST_ASSERT_EQUAL_STRING("observations/22222222-2222-4222-8222-222222222222/2026-11-08/"
                             "88888888-8888-4888-8888-888888888888.jpg", out);
    TEST_ASSERT_EQUAL_INT(-1, observation_evidence_relative_path("../etc", cam_a, 0, out, sizeof(out)));
    TEST_ASSERT_EQUAL_INT(0, observation_evidence_transform_json(NULL, 640, 360, out, sizeof(out)));
    TEST_ASSERT_EQUAL_STRING("{\"mode\":\"none\",\"source_width\":640,\"source_height\":360,"
                             "\"inference_width\":640,\"inference_height\":360,"
                             "\"scale_x\":1,\"scale_y\":1,\"pad_x\":0,\"pad_y\":0}", out);
    observation_evidence_transform_t bad = {.mode = OBS_TRANSFORM_RESIZE, .scale_x = 0};
    TEST_ASSERT_EQUAL_INT(-1, observation_evidence_transform_json(&bad, 10, 10, out, sizeof(out)));
}

int main(void) {
    snprintf(db_path, sizeof(db_path), "/tmp/lightnvr_obs_evidence_%ld.db", (long)getpid());
    snprintf(storage_dir, sizeof(storage_dir), "/tmp/lightnvr_obs_evidence_%ld", (long)getpid());
    unlink(db_path);
    mkdir(storage_dir, 0750);
    if (init_database(db_path) != 0) return 1;
    if (event_identity_init() != 0) return 1;
    make_camera("evidence-a", cam_a);
    make_camera("evidence-b", cam_b);

    UNITY_BEGIN();
    RUN_TEST(test_disabled_mode_allocates_nothing);
    RUN_TEST(test_relative_path_and_transform_json);
    RUN_TEST(test_ready_jpeg_decodes_to_frame_dimensions);
    RUN_TEST(test_jpeg_variant_stores_exact_bytes);
    RUN_TEST(test_queue_saturation_drops_without_blocking);
    RUN_TEST(test_submit_latency_histogram_and_limits);
    RUN_TEST(test_rejected_frames_are_released_by_the_worker);
    RUN_TEST(test_global_byte_budget_bounds_admission);
    RUN_TEST(test_drop_ring_overflow_counts_untracked);
    RUN_TEST(test_budget_flag_drops_as_budget_exhausted);
    RUN_TEST(test_failure_before_rename_never_leaves_final_file);
    RUN_TEST(test_disabled_between_admission_and_encode);
    RUN_TEST(test_stock_probe_reads_stream_setting);
    RUN_TEST(test_unavailable_submission_records_reason);
    RUN_TEST(test_invalid_arguments_free_buffer);
    RUN_TEST(test_shutdown_with_full_queue_marks_interrupted);
    int failures = UNITY_END();

    event_identity_shutdown();
    shutdown_database();
    unlink(db_path);
    char cmd[1200];
    snprintf(cmd, sizeof(cmd), "rm -rf '%s' '%s-wal' '%s-shm'", storage_dir, db_path, db_path);
    if (system(cmd) != 0) fprintf(stderr, "cleanup failed\n");
    return failures;
}
