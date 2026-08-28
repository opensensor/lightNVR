#define _POSIX_C_SOURCE 200809L

#include "unity.h"

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>

#include "core/config.h"
#include "database/db_core.h"
#include "database/db_detection_engines.h"
#include "database/db_streams.h"

static char db_path[256];

void setUp(void) {}
void tearDown(void) {}

static stream_config_t test_stream(const char *name, const char *model) {
    stream_config_t stream;
    memset(&stream, 0, sizeof(stream));
    snprintf(stream.name, sizeof(stream.name), "%s", name);
    snprintf(stream.url, sizeof(stream.url), "rtsp://127.0.0.1/%s", name);
    stream.enabled = true;
    stream.streaming_enabled = true;
    stream.detection_based_recording = true;
    snprintf(stream.detection_model, sizeof(stream.detection_model), "%s", model);
    stream.detection_threshold = 0.6f;
    stream.detection_interval = 7;
    return stream;
}

static void test_legacy_engine_is_backfilled_and_kept_in_sync(void) {
    stream_config_t stream = test_stream("drive", "motion");
    TEST_ASSERT_NOT_EQUAL_UINT64(0, add_stream_config(&stream));

    stream_detection_engine_t engines[MAX_DETECTION_ENGINES_PER_STREAM];
    int count = db_detection_engines_list("drive", engines,
                                          MAX_DETECTION_ENGINES_PER_STREAM);
    TEST_ASSERT_EQUAL_INT(1, count);
    TEST_ASSERT_EQUAL_STRING("legacy-primary", engines[0].engine_key);
    TEST_ASSERT_EQUAL_STRING("motion", engines[0].engine_type);
    TEST_ASSERT_FLOAT_WITHIN(0.001f, 0.6f, engines[0].threshold);
    TEST_ASSERT_EQUAL_INT(7, engines[0].interval_seconds);

    snprintf(stream.detection_model, sizeof(stream.detection_model), "%s", "model.tflite");
    stream.detection_threshold = 0.75f;
    stream.detection_interval = 3;
    TEST_ASSERT_EQUAL_INT(0, update_stream_config("drive", &stream));

    count = db_detection_engines_list("drive", engines,
                                      MAX_DETECTION_ENGINES_PER_STREAM);
    TEST_ASSERT_EQUAL_INT(1, count);
    TEST_ASSERT_EQUAL_STRING("object", engines[0].engine_type);
    TEST_ASSERT_EQUAL_STRING("model.tflite", engines[0].model_path);
    TEST_ASSERT_FLOAT_WITHIN(0.001f, 0.75f, engines[0].threshold);
    TEST_ASSERT_EQUAL_INT(3, engines[0].interval_seconds);
}

static void test_multiple_engines_are_ordered_and_independent(void) {
    stream_config_t stream = test_stream("ditch", "model.tflite");
    TEST_ASSERT_NOT_EQUAL_UINT64(0, add_stream_config(&stream));

    stream_detection_engine_t motion;
    memset(&motion, 0, sizeof(motion));
    snprintf(motion.engine_key, sizeof(motion.engine_key), "%s", "local-motion");
    snprintf(motion.engine_type, sizeof(motion.engine_type), "%s", "motion");
    snprintf(motion.model_path, sizeof(motion.model_path), "%s", "motion");
    snprintf(motion.config_json, sizeof(motion.config_json), "%s", "{}");
    motion.enabled = true;
    motion.threshold = 0.25f;
    motion.interval_seconds = 1;
    motion.sort_order = -10;
    TEST_ASSERT_EQUAL_INT(0, db_detection_engine_upsert("ditch", &motion));

    stream_detection_engine_t engines[MAX_DETECTION_ENGINES_PER_STREAM];
    int count = db_detection_engines_list("ditch", engines,
                                          MAX_DETECTION_ENGINES_PER_STREAM);
    TEST_ASSERT_EQUAL_INT(2, count);
    TEST_ASSERT_EQUAL_STRING("local-motion", engines[0].engine_key);
    TEST_ASSERT_EQUAL_STRING("legacy-primary", engines[1].engine_key);
    TEST_ASSERT_EQUAL_INT(0, db_detection_engine_delete("ditch", "local-motion"));
    TEST_ASSERT_EQUAL_INT(1, db_detection_engines_list(
        "ditch", engines, MAX_DETECTION_ENGINES_PER_STREAM));
}

static void test_invalid_or_reserved_engines_are_rejected(void) {
    stream_detection_engine_t engine;
    memset(&engine, 0, sizeof(engine));
    snprintf(engine.engine_key, sizeof(engine.engine_key), "%s", "legacy-primary");
    snprintf(engine.engine_type, sizeof(engine.engine_type), "%s", "motion");
    snprintf(engine.config_json, sizeof(engine.config_json), "%s", "{}");
    engine.threshold = 0.5f;
    engine.interval_seconds = 5;
    char error[128];
    TEST_ASSERT_EQUAL_INT(-1, db_detection_engine_validate(&engine, error, sizeof(error)));
    TEST_ASSERT_NOT_NULL(strstr(error, "managed"));

    snprintf(engine.engine_key, sizeof(engine.engine_key), "%s", "bad key");
    TEST_ASSERT_EQUAL_INT(-1, db_detection_engine_validate(&engine, error, sizeof(error)));
    snprintf(engine.engine_key, sizeof(engine.engine_key), "%s", "object-2");
    snprintf(engine.engine_type, sizeof(engine.engine_type), "%s", "object");
    TEST_ASSERT_EQUAL_INT(-1, db_detection_engine_validate(&engine, error, sizeof(error)));
}

static void test_replace_custom_is_atomic_and_preserves_legacy(void) {
    stream_config_t stream = test_stream("gate", "model.tflite");
    TEST_ASSERT_NOT_EQUAL_UINT64(0, add_stream_config(&stream));
    stream_detection_engine_t custom[2];
    memset(custom, 0, sizeof(custom));
    snprintf(custom[0].engine_key, sizeof(custom[0].engine_key), "%s", "motion-fast");
    snprintf(custom[0].engine_type, sizeof(custom[0].engine_type), "%s", "motion");
    snprintf(custom[0].model_path, sizeof(custom[0].model_path), "%s", "motion");
    snprintf(custom[0].config_json, sizeof(custom[0].config_json), "%s", "{}");
    custom[0].enabled = true;
    custom[0].threshold = 0.2f;
    custom[0].interval_seconds = 1;
    snprintf(custom[1].engine_key, sizeof(custom[1].engine_key), "%s", "onvif-events");
    snprintf(custom[1].engine_type, sizeof(custom[1].engine_type), "%s", "onvif");
    snprintf(custom[1].model_path, sizeof(custom[1].model_path), "%s", "onvif");
    snprintf(custom[1].config_json, sizeof(custom[1].config_json), "%s", "{}");
    custom[1].enabled = true;
    custom[1].threshold = 1.0f;
    custom[1].interval_seconds = 2;
    TEST_ASSERT_EQUAL_INT(0, db_detection_engines_replace_custom("gate", custom, 2));

    stream_detection_engine_t engines[MAX_DETECTION_ENGINES_PER_STREAM];
    TEST_ASSERT_EQUAL_INT(3, db_detection_engines_list(
        "gate", engines, MAX_DETECTION_ENGINES_PER_STREAM));
    TEST_ASSERT_EQUAL_INT(0, db_detection_engines_replace_custom("gate", NULL, 0));
    TEST_ASSERT_EQUAL_INT(1, db_detection_engines_list(
        "gate", engines, MAX_DETECTION_ENGINES_PER_STREAM));
    TEST_ASSERT_EQUAL_STRING("legacy-primary", engines[0].engine_key);
}

int main(void) {
    snprintf(db_path, sizeof(db_path), "/tmp/lightnvr_engines_%ld.db", (long)getpid());
    unlink(db_path);
    setenv("LIGHTNVR_MIGRATIONS_DIR", "./db/migrations", 1);
    if (init_database(db_path) != 0) return 1;

    UNITY_BEGIN();
    RUN_TEST(test_legacy_engine_is_backfilled_and_kept_in_sync);
    RUN_TEST(test_multiple_engines_are_ordered_and_independent);
    RUN_TEST(test_invalid_or_reserved_engines_are_rejected);
    RUN_TEST(test_replace_custom_is_atomic_and_preserves_legacy);
    int result = UNITY_END();

    shutdown_database();
    unlink(db_path);
    return result;
}
