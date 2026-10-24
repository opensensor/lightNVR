/**
 * @file test_udt_recording.c
 * @brief Layer 3 tests for the unified detection thread's recording path.
 *
 * Like test_detection_current_frame, this includes the translation unit so the
 * private packet/state-machine functions run unmodified.  Real packets from the
 * recording fixture (plus synthetic G.711 audio) go through process_packet()
 * against a temporary catalog with a healthy default storage target, so
 * recordings are actually placed, written, and catalogued.
 *
 * Covered: detection- and trigger-driven BUFFERING → RECORDING → POST_BUFFER →
 * BUFFERING transitions and keep-alives, the detection-stream hand-off, the
 * pre-buffer flush and start-time correction, per-codec audio setup when a
 * recording starts (including the voice-enhancement opt-in, issue #652),
 * blocked placement, the stop fallback, and the public status/registry API.
 */

#define _GNU_SOURCE
#include "../../src/video/unified_detection_thread.c"

#include <sqlite3.h>

#include "unity.h"
#include "database/db_core.h"
#include "database/db_storage_targets.h"
#include "database/db_detection_engines.h"
#include "video/stream_manager.h"

#define STREAM "udt-rec"
#define LIVE_STREAM "udt-live"      /* points at the fixture file */
#define ENGINE_STREAM "udt-engines" /* carries custom detection engines */
#define AUDIO_RATE 8000
#define AUDIO_PACKET 800            /* 100 ms of mono G.711 per video packet */

static char root[] = "/tmp/lightnvr-udt-rec-XXXXXX";
static char db_path[PATH_MAX];
static AVFormatContext *source;     /* recording fixture: 600 x 1 s H.264, GOP 200 */
static AVPacket **video_packets;
static int video_count;
static int64_t source_duration;     /* fixture length in the video time base */

static unified_detection_ctx_t *ctx;
static AVFormatContext *input;      /* what the UDT believes it is reading */
static int64_t cursor;              /* monotonically increasing packet index */
static int64_t audio_pts;

/* ---- helpers ------------------------------------------------------------ */

static void sql(const char *statement) {
    char *error = NULL;
    int rc = sqlite3_exec(get_db_handle(), statement, NULL, NULL, &error);
    TEST_ASSERT_EQUAL_INT_MESSAGE(SQLITE_OK, rc, error ? error : statement);
}

static int64_t scalar(const char *query, uint64_t id) {
    sqlite3_stmt *stmt = NULL;
    TEST_ASSERT_EQUAL_INT(SQLITE_OK, sqlite3_prepare_v2(get_db_handle(), query, -1, &stmt, NULL));
    if (sqlite3_bind_parameter_count(stmt) > 0) sqlite3_bind_int64(stmt, 1, (sqlite3_int64)id);
    int64_t value = sqlite3_step(stmt) == SQLITE_ROW ? sqlite3_column_int64(stmt, 0) : -1;
    sqlite3_finalize(stmt);
    return value;
}

static void build_input_channels(enum AVCodecID audio_codec, int channels) {
    input = avformat_alloc_context();
    TEST_ASSERT_NOT_NULL(input);
    AVStream *video = avformat_new_stream(input, NULL);
    TEST_ASSERT_NOT_NULL(video);
    TEST_ASSERT_EQUAL_INT(0, avcodec_parameters_copy(video->codecpar, source->streams[0]->codecpar));
    video->time_base = source->streams[0]->time_base;
    ctx->video_stream_idx = 0;
    ctx->audio_stream_idx = -1;
    if (audio_codec != AV_CODEC_ID_NONE) {
        AVStream *audio = avformat_new_stream(input, NULL);
        TEST_ASSERT_NOT_NULL(audio);
        audio->codecpar->codec_type = AVMEDIA_TYPE_AUDIO;
        audio->codecpar->codec_id = audio_codec;
        audio->codecpar->sample_rate = AUDIO_RATE;
        av_channel_layout_default(&audio->codecpar->ch_layout, channels);
        audio->time_base = (AVRational){1, AUDIO_RATE};
        ctx->audio_stream_idx = audio->index;
    }
    ctx->input_ctx = input;
}

static void build_input(enum AVCodecID audio_codec) {
    build_input_channels(audio_codec, 1);
}

/* Feed the next fixture packet (and a G.711 packet when the input has audio). */
static void feed_one(void) {
    const AVPacket *src = video_packets[cursor % video_count];
    AVPacket *pkt = av_packet_clone(src);
    TEST_ASSERT_NOT_NULL(pkt);
    int64_t offset = (cursor / video_count) * source_duration;
    pkt->pts += offset;
    pkt->dts += offset;
    pkt->stream_index = 0;
    TEST_ASSERT_EQUAL_INT(0, process_packet(ctx, pkt));
    av_packet_free(&pkt);
    cursor++;

    if (ctx->audio_stream_idx >= 0 &&
        input->streams[ctx->audio_stream_idx]->codecpar->codec_id == AV_CODEC_ID_PCM_MULAW) {
        AVPacket *audio = av_packet_alloc();
        TEST_ASSERT_NOT_NULL(audio);
        TEST_ASSERT_EQUAL_INT(0, av_new_packet(audio, AUDIO_PACKET));
        for (int i = 0; i < AUDIO_PACKET; i++) audio->data[i] = (uint8_t)(0x80 + (i & 0x3f));
        audio->pts = audio->dts = audio_pts;
        audio->duration = AUDIO_PACKET;
        audio->flags = AV_PKT_FLAG_KEY;
        audio->stream_index = ctx->audio_stream_idx;
        audio_pts += AUDIO_PACKET;
        TEST_ASSERT_EQUAL_INT(0, process_packet(ctx, audio));
        av_packet_free(&audio);
    }
}

static void feed(int count) {
    for (int i = 0; i < count; i++) feed_one();
}

/* Feed up to and including the next video keyframe. */
static void feed_keyframe(void) {
    while (!(video_packets[cursor % video_count]->flags & AV_PKT_FLAG_KEY)) feed_one();
    feed_one();
}

static unified_detection_state_t state(void) {
    return (unified_detection_state_t)atomic_load(&ctx->state);
}

static void assert_complete_recording(uint64_t id, int expected_streams) {
    TEST_ASSERT_EQUAL_INT64(1, scalar("SELECT is_complete FROM recordings WHERE id=?", id));
    TEST_ASSERT_GREATER_THAN_INT64(0, scalar("SELECT size_bytes FROM recordings WHERE id=?", id));
    TEST_ASSERT_EQUAL_INT64(RETENTION_TIER_IMPORTANT,
                            scalar("SELECT retention_tier FROM recordings WHERE id=?", id));

    sqlite3_stmt *stmt = NULL;
    TEST_ASSERT_EQUAL_INT(SQLITE_OK, sqlite3_prepare_v2(get_db_handle(),
        "SELECT file_path, trigger_type FROM recordings WHERE id=?", -1, &stmt, NULL));
    sqlite3_bind_int64(stmt, 1, (sqlite3_int64)id);
    TEST_ASSERT_EQUAL_INT(SQLITE_ROW, sqlite3_step(stmt));
    char path[PATH_MAX];
    safe_strcpy(path, (const char *)sqlite3_column_text(stmt, 0), sizeof(path), 0);
    TEST_ASSERT_EQUAL_STRING("detection", (const char *)sqlite3_column_text(stmt, 1));
    sqlite3_finalize(stmt);

    TEST_ASSERT_EQUAL_INT(0, strncmp(path, root, strlen(root)));
    AVFormatContext *clip = NULL;
    TEST_ASSERT_EQUAL_INT(0, avformat_open_input(&clip, path, NULL, NULL));
    TEST_ASSERT_GREATER_OR_EQUAL_INT(0, avformat_find_stream_info(clip, NULL));
    TEST_ASSERT_EQUAL_UINT(expected_streams, clip->nb_streams);
    TEST_ASSERT_EQUAL_INT(AV_CODEC_ID_H264, clip->streams[0]->codecpar->codec_id);
    if (expected_streams > 1)
        TEST_ASSERT_EQUAL_INT(AV_CODEC_ID_AAC, clip->streams[1]->codecpar->codec_id);
    avformat_close_input(&clip);
}

/* ---- fixtures ----------------------------------------------------------- */

void setUp(void) {
    ctx = calloc(1, sizeof(*ctx));
    TEST_ASSERT_NOT_NULL(ctx);
    safe_strcpy(ctx->stream_name, STREAM, sizeof(ctx->stream_name), 0);
    safe_strcpy(ctx->camera_uuid, "00000000-0000-4000-8000-0000000000a1",
                sizeof(ctx->camera_uuid), 0);
    safe_strcpy(ctx->model_path, "onvif", sizeof(ctx->model_path), 0);
    ctx->detection_interval = 1;
    ctx->pre_buffer_seconds = 10;
    ctx->post_buffer_seconds = 30;
    ctx->segment_duration = 900;
    /* Generous enough that a wall-clock second ticking over mid-test never
     * ends a clip; tests that want post-buffer backdate last_detection_time. */
    ctx->recording_grace_interval = 5;
    ctx->packet_buffer = create_packet_buffer(STREAM, ctx->pre_buffer_seconds, BUFFER_MODE_MEMORY);
    TEST_ASSERT_NOT_NULL(ctx->packet_buffer);
    pthread_mutex_init(&ctx->mutex, NULL);
    pthread_mutex_init(&ctx->detection_stream_result_mutex, NULL);
    atomic_store(&ctx->state, UDT_STATE_BUFFERING);
    atomic_store(&ctx->running, 1);
    ctx->stream_is_live = true;
    ctx->first_video_pts = AV_NOPTS_VALUE;
    ctx->record_audio = true;
    g_config.detection_grace_period = 0;
    cursor = 0;
    audio_pts = 0;
    input = NULL;
}

void tearDown(void) {
    if (ctx->mp4_writer) udt_stop_recording(ctx);
    for (int i = 0; i < MAX_UNIFIED_DETECTION_THREADS; i++)
        if (detection_contexts[i] == ctx) detection_contexts[i] = NULL;
    cleanup_audio_transcoder(STREAM);
    destroy_packet_buffer(ctx->packet_buffer);
    pthread_mutex_destroy(&ctx->mutex);
    pthread_mutex_destroy(&ctx->detection_stream_result_mutex);
    free(ctx);
    avformat_free_context(input);
    sql("UPDATE storage_targets SET enabled=1;");
}

/* ---- tests -------------------------------------------------------------- */

/* A detection on a keyframe starts a clip from the pre-buffer, transcodes the
 * G.711 track with the stream's voice-enhancement opt-in applied, and the clip
 * is finalised in the catalog once detection stops and the post-buffer ends. */
static void test_detection_records_pcm_audio_with_voice_enhancement(void) {
    build_input(AV_CODEC_ID_PCM_MULAW);
    ctx->audio_voice_enhancement = true;

    /* Seed the pre-buffer with the opening keyframe stamped 5 s ago so the
     * clip's start time gets pulled back by the pre-roll it contains. */
    TEST_ASSERT_EQUAL_INT(0, packet_buffer_add_packet(ctx->packet_buffer, video_packets[0],
                                                      time(NULL) - 5));
    cursor = 1;
    feed(150);                                  /* fill the pre-buffer, no motion */
    TEST_ASSERT_EQUAL_INT(UDT_STATE_BUFFERING, state());
    TEST_ASSERT_FALSE(get_audio_voice_enhancement(STREAM));

    atomic_store(&ctx->onvif_motion_detected, 1);
    atomic_store(&ctx->last_detection_check_time, 0);
    feed_keyframe();                            /* packet 200 */
    TEST_ASSERT_EQUAL_INT(UDT_STATE_RECORDING, state());
    uint64_t id = atomic_load(&ctx->current_recording_id);
    TEST_ASSERT_NOT_EQUAL_UINT64(0, id);
    TEST_ASSERT_NOT_NULL(ctx->mp4_writer);
    TEST_ASSERT_TRUE(ctx->mp4_writer->has_audio);
    TEST_ASSERT_TRUE(ctx->mp4_writer->is_initialized);
    TEST_ASSERT_TRUE(ctx->mp4_writer->start_time_corrected);
    TEST_ASSERT_TRUE(get_audio_voice_enhancement(STREAM));
    TEST_ASSERT_EQUAL_INT(1, (int)ctx->total_detections);
    TEST_ASSERT_EQUAL_INT(1, (int)ctx->total_recordings);

    feed(50);
    atomic_store(&ctx->onvif_motion_detected, 0);
    atomic_store(&ctx->last_detection_check_time, 0);
    atomic_store(&ctx->last_detection_time, (long long)time(NULL) - 60);
    ctx->post_buffer_seconds = 0;
    /* Packet 400 finds no detection and enters a zero-length post-buffer; the
     * audio packet that follows it already sees it expired and stops the clip. */
    feed_keyframe();
    TEST_ASSERT_EQUAL_INT(UDT_STATE_BUFFERING, state());
    TEST_ASSERT_EQUAL_UINT64(0, atomic_load(&ctx->current_recording_id));
    TEST_ASSERT_NULL(ctx->mp4_writer);
    assert_complete_recording(id, 2);
}

/* Motion forwarded from another stream drives the same state machine: start,
 * keep-alive while recording, end into post-buffer, revive, end, expire. */
static void test_external_trigger_cycle(void) {
    build_input(AV_CODEC_ID_NONE);              /* record_audio but no track */

    feed(10);
    atomic_store(&ctx->external_motion_trigger, 2);   /* ended while idle: no-op */
    feed_keyframe();
    TEST_ASSERT_EQUAL_INT(UDT_STATE_BUFFERING, state());

    atomic_store(&ctx->external_motion_trigger, 1);
    feed_keyframe();
    TEST_ASSERT_EQUAL_INT(UDT_STATE_RECORDING, state());
    uint64_t id = atomic_load(&ctx->current_recording_id);
    TEST_ASSERT_NOT_EQUAL_UINT64(0, id);
    TEST_ASSERT_FALSE(ctx->mp4_writer->has_audio);
    TEST_ASSERT_EQUAL_INT(0, atomic_load(&ctx->external_motion_trigger));

    atomic_store(&ctx->external_motion_trigger, 1);   /* keep-alive */
    feed_keyframe();
    TEST_ASSERT_EQUAL_INT(UDT_STATE_RECORDING, state());

    atomic_store(&ctx->external_motion_trigger, 2);
    feed_keyframe();
    TEST_ASSERT_EQUAL_INT(UDT_STATE_POST_BUFFER, state());

    atomic_store(&ctx->external_motion_trigger, 1);   /* revive from post-buffer */
    feed_keyframe();
    TEST_ASSERT_EQUAL_INT(UDT_STATE_RECORDING, state());
    TEST_ASSERT_EQUAL_UINT64(id, atomic_load(&ctx->current_recording_id));

    atomic_store(&ctx->external_motion_trigger, 2);
    feed_keyframe();
    TEST_ASSERT_EQUAL_INT(UDT_STATE_POST_BUFFER, state());
    atomic_store(&ctx->post_buffer_end_time, 0);
    feed(1);
    TEST_ASSERT_EQUAL_INT(UDT_STATE_BUFFERING, state());
    assert_complete_recording(id, 1);
}

static void hand_off_detection(void) {
    pthread_mutex_lock(&ctx->detection_stream_result_mutex);
    memset(&ctx->detection_stream_pending, 0, sizeof(ctx->detection_stream_pending));
    ctx->detection_stream_pending.count = 1;
    ctx->detection_stream_pending.detections[0].confidence = 0.9f;
    safe_strcpy(ctx->detection_stream_pending.detections[0].label, "person",
                sizeof(ctx->detection_stream_pending.detections[0].label), 0);
    ctx->detection_stream_pending_ts = time(NULL);
    pthread_mutex_unlock(&ctx->detection_stream_result_mutex);
    atomic_store(&ctx->detection_stream_result, 1);
}

/* Results from the secondary detection stream are consumed on any packet,
 * stored, and start/extend the clip; the ONVIF onset backdates the event. */
static void test_detection_stream_hand_off(void) {
    build_input(AV_CODEC_ID_NONE);
    ctx->record_audio = false;
    ctx->has_onvif_engine = true;
    atomic_store(&ctx->onvif_motion_onset, (long long)time(NULL) - 1);
    atomic_store(&ctx->detection_stream_connected, 1);

    feed(20);
    TEST_ASSERT_EQUAL_INT(UDT_STATE_BUFFERING, state());
    hand_off_detection();
    feed(1);
    TEST_ASSERT_EQUAL_INT(UDT_STATE_RECORDING, state());
    TEST_ASSERT_EQUAL_INT(1, (int)ctx->total_detections);
    TEST_ASSERT_EQUAL_INT(0, atomic_load(&ctx->detection_stream_result));
    TEST_ASSERT_EQUAL_INT64(1, scalar("SELECT count(*) FROM detections WHERE stream_name='" STREAM "'", 0));

    atomic_store(&ctx->last_detection_time, (long long)time(NULL) - 60);
    feed(1);                                    /* no result: grace exceeded */
    TEST_ASSERT_EQUAL_INT(UDT_STATE_POST_BUFFER, state());
    hand_off_detection();
    feed(1);                                    /* keep-alive from post-buffer */
    TEST_ASSERT_EQUAL_INT(UDT_STATE_RECORDING, state());
    TEST_ASSERT_EQUAL_INT(2, (int)ctx->total_detections);
    feed_keyframe();                            /* writer is live past the flush */
    uint64_t id = atomic_load(&ctx->current_recording_id);
    TEST_ASSERT_EQUAL_INT(0, udt_stop_recording(ctx));
    assert_complete_recording(id, 1);
}

/* ONVIF motion first seen between sampled checks starts recording on the very
 * next keyframe instead of waiting out detection_interval (#494). */
static void test_onvif_onset_fast_path(void) {
    build_input(AV_CODEC_ID_NONE);
    ctx->detection_interval = 3600;
    feed(5);
    atomic_store(&ctx->last_detection_check_time, (long long)time(NULL));
    atomic_store(&ctx->onvif_motion_detected, 1);
    atomic_store(&ctx->onvif_motion_onset, (long long)time(NULL));
    feed_keyframe();
    TEST_ASSERT_EQUAL_INT(UDT_STATE_RECORDING, state());
    TEST_ASSERT_EQUAL_INT64(atomic_load(&ctx->onvif_motion_onset), ctx->onvif_onset_handled);
}

/* Audio setup when a clip starts: PCM is prepared for AAC transcoding,
 * MP4-compatible tracks are copied, and a PCM layout the encoder can't take
 * or record_audio=off leaves the clip video-only. */
static void test_start_recording_audio_variants(void) {
    const struct {
        enum AVCodecID codec; int channels; bool record; bool has_audio; bool pending;
    } cases[] = {
        { AV_CODEC_ID_PCM_ALAW,  1, true,  true,  true  },
        { AV_CODEC_ID_AAC,       1, true,  true,  true  },
        { AV_CODEC_ID_PCM_MULAW, 9, true,  false, false },
        { AV_CODEC_ID_PCM_ALAW,  1, false, false, false },
    };
    for (size_t i = 0; i < sizeof(cases) / sizeof(cases[0]); i++) {
        build_input_channels(cases[i].codec, cases[i].channels);
        ctx->record_audio = cases[i].record;
        TEST_ASSERT_EQUAL_INT(0, udt_start_recording(ctx));
        TEST_ASSERT_NOT_NULL(ctx->mp4_writer);
        TEST_ASSERT_EQUAL_INT(cases[i].has_audio, ctx->mp4_writer->has_audio != 0);
        TEST_ASSERT_EQUAL_INT(cases[i].pending, ctx->mp4_writer->pending_audio_codecpar != NULL);
        TEST_ASSERT_EQUAL_INT(1, ctx->mp4_writer->schedule_restricted);
        TEST_ASSERT_EQUAL_INT(0, udt_stop_recording(ctx));
        avformat_free_context(input);
        input = NULL;
    }
}

/* A paused/unavailable storage target blocks the clip without side effects. */
static void test_blocked_placement_does_not_start(void) {
    build_input(AV_CODEC_ID_NONE);
    sql("UPDATE storage_targets SET enabled=0;");
    TEST_ASSERT_EQUAL_INT(-1, udt_start_recording(ctx));
    TEST_ASSERT_NULL(ctx->mp4_writer);
    TEST_ASSERT_EQUAL_UINT64(0, atomic_load(&ctx->current_recording_id));

    feed(5);
    atomic_store(&ctx->external_motion_trigger, 1);
    feed_keyframe();
    TEST_ASSERT_EQUAL_INT(UDT_STATE_BUFFERING, state());
}

/* Stopping without a catalog id still records the clip; stopping with no
 * writer is a no-op. */
static void test_stop_fallback_and_noop(void) {
    build_input(AV_CODEC_ID_NONE);
    TEST_ASSERT_EQUAL_INT(0, udt_stop_recording(ctx));
    TEST_ASSERT_EQUAL_INT(-1, udt_stop_recording(NULL));

    TEST_ASSERT_EQUAL_INT(0, udt_start_recording(ctx));
    uint64_t original = atomic_load(&ctx->current_recording_id);
    atomic_store(&ctx->current_recording_id, 0);
    int64_t before = scalar("SELECT count(*) FROM recordings", 0);
    TEST_ASSERT_EQUAL_INT(0, udt_stop_recording(ctx));
    TEST_ASSERT_EQUAL_INT64(before + 1, scalar("SELECT count(*) FROM recordings", 0));
    TEST_ASSERT_EQUAL_INT64(0, scalar("SELECT is_complete FROM recordings WHERE id=?", original));
    TEST_ASSERT_EQUAL_STRING("", ctx->current_recording_path);
}

/* The public status API reads the registered context. */
static void test_registry_status_api(void) {
    TEST_ASSERT_FALSE(is_unified_detection_running(STREAM));
    TEST_ASSERT_EQUAL_INT(UDT_STATE_STOPPED, get_unified_detection_state(STREAM));
    TEST_ASSERT_EQUAL_INT(STREAM_STATUS_STOPPED, get_unified_detection_effective_status(STREAM));
    TEST_ASSERT_EQUAL_INT(-1, get_unified_detection_stats(STREAM, NULL, NULL, NULL));
    TEST_ASSERT_EQUAL_INT(-1, stop_unified_detection_thread(STREAM));
    TEST_ASSERT_FALSE(is_unified_detection_running(NULL));
    TEST_ASSERT_EQUAL_INT(UDT_STATE_STOPPED, get_unified_detection_state(NULL));
    TEST_ASSERT_EQUAL_INT(STREAM_STATUS_STOPPED, get_unified_detection_effective_status(NULL));
    TEST_ASSERT_EQUAL_INT(0, get_unified_detection_reconnect_attempts(NULL));
    TEST_ASSERT_EQUAL_UINT64(0, get_unified_detection_recording_id(NULL));
    TEST_ASSERT_EQUAL_INT(-1, get_unified_detection_stats(NULL, NULL, NULL, NULL));
    TEST_ASSERT_EQUAL_INT(-1, stop_unified_detection_thread(NULL));
    unified_detection_notify_motion(NULL, true);
    unified_detection_notify_motion(STREAM, true);   /* unknown: silent no-op */

    int slot = find_empty_slot();
    TEST_ASSERT_GREATER_OR_EQUAL_INT(0, slot);
    detection_contexts[slot] = ctx;
    TEST_ASSERT_EQUAL_PTR(ctx, find_context_by_name(STREAM));
    TEST_ASSERT_TRUE(is_unified_detection_running(STREAM));
    ctx->reconnect_attempt = 3;
    atomic_store(&ctx->current_recording_id, 77);
    ctx->total_packets_processed = 5;
    ctx->total_detections = 2;
    ctx->total_recordings = 1;
    TEST_ASSERT_EQUAL_INT(3, get_unified_detection_reconnect_attempts(STREAM));
    TEST_ASSERT_EQUAL_UINT64(77, get_unified_detection_recording_id(STREAM));
    uint64_t packets = 0, detections = 0, recordings = 0;
    TEST_ASSERT_EQUAL_INT(0, get_unified_detection_stats(STREAM, &packets, &detections, &recordings));
    TEST_ASSERT_EQUAL_UINT64(5, packets);
    TEST_ASSERT_EQUAL_UINT64(2, detections);
    TEST_ASSERT_EQUAL_UINT64(1, recordings);
    atomic_store(&ctx->current_recording_id, 0);

    const struct { unified_detection_state_t state; int attempts; stream_status_t status; const char *name; } map[] = {
        { UDT_STATE_INITIALIZING, 0, STREAM_STATUS_STARTING,     "INITIALIZING" },
        { UDT_STATE_CONNECTING,   0, STREAM_STATUS_STARTING,     "CONNECTING" },
        { UDT_STATE_CONNECTING,   2, STREAM_STATUS_RECONNECTING, "CONNECTING" },
        { UDT_STATE_BUFFERING,    0, STREAM_STATUS_RUNNING,      "BUFFERING" },
        { UDT_STATE_RECORDING,    0, STREAM_STATUS_RUNNING,      "RECORDING" },
        { UDT_STATE_POST_BUFFER,  0, STREAM_STATUS_RUNNING,      "POST_BUFFER" },
        { UDT_STATE_RECONNECTING, 0, STREAM_STATUS_RECONNECTING, "RECONNECTING" },
        { UDT_STATE_STOPPING,     0, STREAM_STATUS_STOPPING,     "STOPPING" },
        { UDT_STATE_STOPPED,      0, STREAM_STATUS_STOPPED,      "STOPPED" },
    };
    for (size_t i = 0; i < sizeof(map) / sizeof(map[0]); i++) {
        atomic_store(&ctx->state, map[i].state);
        ctx->reconnect_attempt = map[i].attempts;
        TEST_ASSERT_EQUAL_INT(map[i].state, get_unified_detection_state(STREAM));
        TEST_ASSERT_EQUAL_INT(map[i].status, get_unified_detection_effective_status(STREAM));
        TEST_ASSERT_EQUAL_STRING(map[i].name, state_to_string(map[i].state));
    }
    TEST_ASSERT_EQUAL_STRING("UNKNOWN", state_to_string((unified_detection_state_t)99));

    unified_detection_notify_motion(STREAM, true);
    TEST_ASSERT_EQUAL_INT(1, atomic_load(&ctx->external_motion_trigger));
    unified_detection_notify_motion(STREAM, false);
    TEST_ASSERT_EQUAL_INT(2, atomic_load(&ctx->external_motion_trigger));

    atomic_store(&ctx->state, UDT_STATE_BUFFERING);
    TEST_ASSERT_EQUAL_INT(0, stop_unified_detection_thread(STREAM));
    TEST_ASSERT_EQUAL_INT(UDT_STATE_STOPPING, get_unified_detection_state(STREAM));
    TEST_ASSERT_FALSE(is_unified_detection_running(STREAM));
    TEST_ASSERT_EQUAL_INT(0, init_unified_detection_system());   /* already initialised */
}

/* Shutdown frees every registered context, closing a writer the thread left
 * open; a stopped thread is not waited on. */
static void test_shutdown_cleans_up_stopped_contexts(void) {
    unified_detection_ctx_t *orphan = calloc(1, sizeof(*orphan));
    TEST_ASSERT_NOT_NULL(orphan);
    safe_strcpy(orphan->stream_name, "udt-orphan", sizeof(orphan->stream_name), 0);
    orphan->packet_buffer = create_packet_buffer("udt-orphan", 5, BUFFER_MODE_MEMORY);
    TEST_ASSERT_NOT_NULL(orphan->packet_buffer);
    char path[PATH_MAX];
    snprintf(path, sizeof(path), "%s/orphan.mp4", root);
    orphan->mp4_writer = mp4_writer_create(path, "udt-orphan");
    TEST_ASSERT_NOT_NULL(orphan->mp4_writer);
    pthread_mutex_init(&orphan->mutex, NULL);
    pthread_mutex_init(&orphan->detection_stream_result_mutex, NULL);
    atomic_store(&orphan->state, UDT_STATE_STOPPED);
    detection_contexts[find_empty_slot()] = orphan;

    shutdown_unified_detection_system();
    TEST_ASSERT_FALSE(system_initialized);
    for (int i = 0; i < MAX_UNIFIED_DETECTION_THREADS; i++) TEST_ASSERT_NULL(detection_contexts[i]);
    shutdown_unified_detection_system();        /* second call is a no-op */
    TEST_ASSERT_EQUAL_INT(0, init_unified_detection_system());
}

static void add_engine(const char *key, const char *type, const char *model,
                       float threshold, int interval, bool enabled) {
    stream_detection_engine_t engine = {0};
    safe_strcpy(engine.engine_key, key, sizeof(engine.engine_key), 0);
    safe_strcpy(engine.engine_type, type, sizeof(engine.engine_type), 0);
    safe_strcpy(engine.model_path, model, sizeof(engine.model_path), 0);
    engine.enabled = enabled;
    engine.threshold = threshold;
    engine.interval_seconds = interval;
    TEST_ASSERT_EQUAL_INT(0, db_detection_engine_upsert(ENGINE_STREAM, &engine));
}

static const stream_detection_engine_t *engine_named(const char *key) {
    for (int i = 0; i < ctx->engine_count; i++)
        if (strcmp(ctx->engines[i].engine_key, key) == 0) return &ctx->engines[i];
    return NULL;
}

/* Stored engines replace the legacy single model: disabled rows are skipped,
 * relative object models resolve under models_path, motion/ONVIF get their
 * implicit model path, and the shortest/longest intervals drive sampling and
 * the recording grace window.  Without rows, the legacy model is classified. */
static void test_runtime_engine_config(void) {
    safe_strcpy(ctx->stream_name, ENGINE_STREAM, sizeof(ctx->stream_name), 0);
    safe_strcpy(g_config.models_path, "/opt/models", sizeof(g_config.models_path), 0);
    add_engine("motion-a", "motion", "", 0.3f, 4, true);
    add_engine("people", "object", "people.tflite", 0.6f, 9, true);
    add_engine("doorbell", "onvif", "", 0.5f, 2, true);
    add_engine("retired", "motion", "", 0.3f, 1, false);

    load_runtime_engine_config(ctx, "/opt/models/legacy.tflite", 0.5f, 30);
    TEST_ASSERT_NOT_NULL(engine_named("motion-a"));
    TEST_ASSERT_NULL(engine_named("retired"));
    TEST_ASSERT_EQUAL_STRING("motion", engine_named("motion-a")->model_path);
    TEST_ASSERT_EQUAL_STRING("onvif", engine_named("doorbell")->model_path);
    TEST_ASSERT_EQUAL_STRING("/opt/models/people.tflite", engine_named("people")->model_path);
    const stream_detection_engine_t *legacy = engine_named("legacy-primary");
    if (legacy) TEST_ASSERT_EQUAL_STRING("/opt/models/legacy.tflite", legacy->model_path);
    TEST_ASSERT_EQUAL_INT(2, ctx->detection_interval);
    TEST_ASSERT_GREATER_OR_EQUAL_INT(9, ctx->recording_grace_interval);
    TEST_ASSERT_TRUE(ctx->has_onvif_engine);
    TEST_ASSERT_TRUE(runtime_has_engine_type(ctx, "object"));
    TEST_ASSERT_FALSE(runtime_has_engine_type(ctx, "api"));
    TEST_ASSERT_FALSE(runtime_has_engine_type(NULL, "motion"));

    /* No stored engines: the legacy model becomes the only engine. */
    const struct { const char *model; const char *type; } legacy_models[] = {
        { "motion", "motion" }, { "onvif", "onvif" },
        { "http://127.0.0.1:9001/detect", "api" }, { "/opt/models/x.tflite", "object" },
    };
    safe_strcpy(ctx->stream_name, STREAM, sizeof(ctx->stream_name), 0);
    for (size_t i = 0; i < sizeof(legacy_models) / sizeof(legacy_models[0]); i++) {
        load_runtime_engine_config(ctx, legacy_models[i].model, 0.4f, 0);
        TEST_ASSERT_EQUAL_INT(1, ctx->engine_count);
        TEST_ASSERT_EQUAL_STRING(legacy_models[i].type, ctx->engines[0].engine_type);
        TEST_ASSERT_EQUAL_STRING(legacy_models[i].model, ctx->model_path);
        TEST_ASSERT_EQUAL_INT(DEFAULT_DETECTION_INTERVAL, ctx->detection_interval);
        TEST_ASSERT_FLOAT_WITHIN(0.001f, 0.4f, ctx->detection_threshold);
    }
    /* External-trigger-only: no engines, grace follows the sampling interval. */
    ctx->detection_interval = 7;
    load_runtime_engine_config(ctx, "", 0.4f, 7);
    TEST_ASSERT_EQUAL_INT(0, ctx->engine_count);
    TEST_ASSERT_EQUAL_INT(7, ctx->recording_grace_interval);
}

static AVFrame *gray_frame(void) {
    AVFrame *frame = av_frame_alloc();
    TEST_ASSERT_NOT_NULL(frame);
    frame->format = AV_PIX_FMT_YUV420P;
    frame->width = 64;
    frame->height = 64;
    TEST_ASSERT_EQUAL_INT(0, av_frame_get_buffer(frame, 0));
    memset(frame->data[0], 0x80, (size_t)frame->linesize[0] * 64);
    memset(frame->data[1], 0x80, (size_t)frame->linesize[1] * 32);
    memset(frame->data[2], 0x80, (size_t)frame->linesize[2] * 32);
    return frame;
}

static void set_engine(int i, const char *key, const char *type, const char *model, int interval) {
    stream_detection_engine_t *engine = &ctx->engines[i];
    memset(engine, 0, sizeof(*engine));
    safe_strcpy(engine->engine_key, key, sizeof(engine->engine_key), 0);
    safe_strcpy(engine->engine_type, type, sizeof(engine->engine_type), 0);
    safe_strcpy(engine->model_path, model, sizeof(engine->model_path), 0);
    engine->enabled = true;
    engine->threshold = 0.5f;
    engine->interval_seconds = interval;
}

/* Multi-engine fan-out on one decoded frame: ONVIF reports its async flag,
 * an object model that can't load is disabled once, API engines are isolated,
 * and every engine then waits out its own interval. */
static void test_runtime_engine_fan_out(void) {
    set_engine(0, "motion-a", "motion", "motion", 60);
    set_engine(1, "people", "object", "/nonexistent/people.tflite", 60);
    set_engine(2, "doorbell", "onvif", "onvif", 60);
    set_engine(3, "cloud", "api", "http://127.0.0.1:9/detect", 60);
    ctx->engine_count = 4;
    AVFrame *frame = gray_frame();
    detection_result_t result;
    time_t now = time(NULL);

    atomic_store(&ctx->onvif_motion_detected, 1);
    TEST_ASSERT_TRUE(detect_on_decoded_frame(ctx, frame, now, &result));
    TEST_ASSERT_EQUAL_INT(0, result.count);
    TEST_ASSERT_TRUE(ctx->engine_load_failed[1]);
    TEST_ASSERT_TRUE(ctx->engine_load_failed[3]);
    for (int i = 0; i < 4; i++) TEST_ASSERT_EQUAL_INT64(now, ctx->engine_last_run[i]);

    /* Within every interval: nothing is due, so not even ONVIF is re-read. */
    TEST_ASSERT_FALSE(detect_on_decoded_frame(ctx, frame, now + 1, &result));

    /* append_engine_result keeps only detections above the engine threshold. */
    detection_result_t combined = {0}, candidate = {0};
    candidate.count = 2;
    candidate.detections[0].confidence = 0.9f;
    safe_strcpy(candidate.detections[0].label, "person", sizeof(candidate.detections[0].label), 0);
    candidate.detections[1].confidence = 0.1f;
    safe_strcpy(candidate.detections[1].label, "cat", sizeof(candidate.detections[1].label), 0);
    append_engine_result(ctx, &ctx->engines[1], &combined, &candidate);
    TEST_ASSERT_EQUAL_INT(1, combined.count);
    TEST_ASSERT_EQUAL_STRING("person", combined.detections[0].label);

    /* Single-engine dispatch: motion runs, a missing embedded model fails once. */
    ctx->engine_count = 1;
    safe_strcpy(ctx->model_path, "motion", sizeof(ctx->model_path), 0);
    TEST_ASSERT_FALSE(detect_on_decoded_frame(ctx, frame, now, &result));
    safe_strcpy(ctx->model_path, "/nonexistent/legacy.tflite", sizeof(ctx->model_path), 0);
    TEST_ASSERT_FALSE(detect_on_decoded_frame(ctx, frame, now, &result));
    TEST_ASSERT_TRUE(ctx->model_load_failed);
    TEST_ASSERT_FALSE(detect_on_decoded_frame(ctx, frame, now, &result));
    ctx->model_path[0] = '\0';
    ctx->model_load_failed = false;
    TEST_ASSERT_FALSE(detect_on_decoded_frame(ctx, frame, now, &result));
    TEST_ASSERT_FALSE(detect_on_decoded_frame(NULL, frame, now, &result));

    unload_runtime_models(ctx);
    av_frame_free(&frame);
}

static bool live_context_exists(void) {
    pthread_mutex_lock(&contexts_mutex);
    bool exists = find_context_by_name(LIVE_STREAM) != NULL;
    pthread_mutex_unlock(&contexts_mutex);
    return exists;
}

/* Start a real UDT against the fixture file: it connects, reads and processes
 * packets, and on stop disconnects and frees its own context.  The detection
 * URL is refused (file access is not whitelisted), so that thread backs off
 * until it is stopped, and the ONVIF engine has no derivable endpoint. */
static void test_thread_lifecycle_reads_local_source(void) {
    TEST_ASSERT_EQUAL_INT(-1, start_unified_detection_thread(NULL, "onvif", 0.5f, 5, 5, false));
    TEST_ASSERT_EQUAL_INT(-1, start_unified_detection_thread(LIVE_STREAM, NULL, 0.5f, 5, 5, false));
    TEST_ASSERT_EQUAL_INT(-1, start_unified_detection_thread("udt-missing", "onvif", 0.5f, 5, 5, false));

    TEST_ASSERT_EQUAL_INT(0, start_unified_detection_thread(LIVE_STREAM, "onvif", 0.5f, 0, 0, true));
    TEST_ASSERT_EQUAL_INT(0, start_unified_detection_thread(LIVE_STREAM, "onvif", 0.5f, 0, 0, true));
    TEST_ASSERT_TRUE(is_unified_detection_running(LIVE_STREAM));

    uint64_t packets = 0;
    for (int i = 0; i < 200 && packets == 0; i++) {
        usleep(50000);
        TEST_ASSERT_EQUAL_INT(0, get_unified_detection_stats(LIVE_STREAM, &packets, NULL, NULL));
    }
    TEST_ASSERT_GREATER_THAN_UINT64(0, packets);
    TEST_ASSERT_EQUAL_INT(STREAM_STATUS_RUNNING, get_unified_detection_effective_status(LIVE_STREAM));

    TEST_ASSERT_EQUAL_INT(0, stop_unified_detection_thread(LIVE_STREAM));
    for (int i = 0; i < 300 && live_context_exists(); i++) usleep(50000);
    TEST_ASSERT_FALSE(live_context_exists());
    TEST_ASSERT_EQUAL_INT(UDT_STATE_STOPPED, get_unified_detection_state(LIVE_STREAM));
}

/* ---- main --------------------------------------------------------------- */

static int load_fixture(void) {
    if (avformat_open_input(&source, TEST_RECORDING_PATH, NULL, NULL) != 0) return -1;
    if (avformat_find_stream_info(source, NULL) < 0) return -1;
    AVPacket *pkt = av_packet_alloc();
    if (!pkt) return -1;
    int64_t end = 0;
    while (av_read_frame(source, pkt) >= 0) {
        if (pkt->stream_index == 0) {
            AVPacket **grown = realloc(video_packets, sizeof(*grown) * (size_t)(video_count + 1));
            if (!grown) return -1;
            video_packets = grown;
            video_packets[video_count++] = av_packet_clone(pkt);
            if (pkt->pts + pkt->duration > end) end = pkt->pts + pkt->duration;
        }
        av_packet_unref(pkt);
    }
    av_packet_free(&pkt);
    source_duration = end;
    return video_count > 0 ? 0 : -1;
}

int main(void) {
    init_logger();
    set_log_level(LOG_LEVEL_ERROR);
    load_default_config(&g_config);
    if (!mkdtemp(root)) return 2;
    snprintf(db_path, sizeof(db_path), "%s/catalog.sqlite", root);
    safe_strcpy(g_config.storage_path, root, sizeof(g_config.storage_path), 0);
    if (init_database_ex(db_path, DB_INIT_NO_BACKUP | DB_INIT_NO_CHECK) != 0) return 2;

    stream_config_t camera = {0};
    safe_strcpy(camera.name, STREAM, sizeof(camera.name), 0);
    safe_strcpy(camera.url, "rtsp://fixture/live", sizeof(camera.url), 0);
    camera.enabled = true;
    camera.detection_record_on_schedule = true;
    camera.audio_voice_enhancement = true;
    if (!add_stream_config(&camera)) return 2;
    stream_config_t live = {0};
    safe_strcpy(live.name, LIVE_STREAM, sizeof(live.name), 0);
    safe_strcpy(live.url, TEST_RECORDING_PATH, sizeof(live.url), 0);
    safe_strcpy(live.detection_url, TEST_RECORDING_PATH, sizeof(live.detection_url), 0);
    live.enabled = true;
    live.record_audio = true;
    live.detection_interval = 1;
    if (!add_stream_config(&live)) return 2;
    stream_config_t engines = {0};
    safe_strcpy(engines.name, ENGINE_STREAM, sizeof(engines.name), 0);
    safe_strcpy(engines.url, "rtsp://fixture/engines", sizeof(engines.url), 0);
    engines.enabled = true;
    if (!add_stream_config(&engines)) return 2;
    if (init_stream_manager(MAX_STREAMS) != 0) return 2;
    char target_uuid[LIGHTNVR_UUID_STRING_SIZE];
    if (db_storage_target_bootstrap_default(root, target_uuid) != 0) return 2;
    char *error = NULL;
    if (sqlite3_exec(get_db_handle(),
            "UPDATE storage_targets SET health_status='healthy', enabled=1;",
            NULL, NULL, &error) != SQLITE_OK) return 2;
    if (load_fixture() != 0) return 2;
    if (init_unified_detection_system() != 0) return 2;

    UNITY_BEGIN();
    RUN_TEST(test_detection_records_pcm_audio_with_voice_enhancement);
    RUN_TEST(test_external_trigger_cycle);
    RUN_TEST(test_detection_stream_hand_off);
    RUN_TEST(test_onvif_onset_fast_path);
    RUN_TEST(test_start_recording_audio_variants);
    RUN_TEST(test_blocked_placement_does_not_start);
    RUN_TEST(test_stop_fallback_and_noop);
    RUN_TEST(test_runtime_engine_config);
    RUN_TEST(test_runtime_engine_fan_out);
    RUN_TEST(test_registry_status_api);
    RUN_TEST(test_thread_lifecycle_reads_local_source);
    RUN_TEST(test_shutdown_cleans_up_stopped_contexts);
    int failures = UNITY_END();

    shutdown_unified_detection_system();
    shutdown_stream_manager();
    for (int i = 0; i < video_count; i++) av_packet_free(&video_packets[i]);
    free(video_packets);
    avformat_close_input(&source);
    shutdown_database();
    char command[PATH_MAX + 16];
    snprintf(command, sizeof(command), "rm -rf '%s'", root);
    if (system(command) != 0) return failures ? failures : 3;
    return failures;
}
