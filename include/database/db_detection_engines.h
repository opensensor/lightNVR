#ifndef LIGHTNVR_DB_DETECTION_ENGINES_H
#define LIGHTNVR_DB_DETECTION_ENGINES_H

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>

#include "core/config.h"

#define DETECTION_ENGINE_KEY_MAX 64
#define DETECTION_ENGINE_TYPE_MAX 16
#define DETECTION_ENGINE_CONFIG_MAX 1024
#define MAX_DETECTION_ENGINES_PER_STREAM 8

typedef struct {
    uint64_t id;
    char engine_key[DETECTION_ENGINE_KEY_MAX];
    char engine_type[DETECTION_ENGINE_TYPE_MAX];
    char model_path[MAX_PATH_LENGTH];
    bool enabled;
    float threshold;
    int interval_seconds;
    int sort_order;
    char config_json[DETECTION_ENGINE_CONFIG_MAX];
} stream_detection_engine_t;

/** Return ordered engines for a stream, including disabled rows. */
int db_detection_engines_list(const char *stream_name,
                              stream_detection_engine_t *engines,
                              size_t capacity);

/** Add or update one non-legacy engine. */
int db_detection_engine_upsert(const char *stream_name,
                               const stream_detection_engine_t *engine);

/** Delete one non-legacy engine. */
int db_detection_engine_delete(const char *stream_name,
                               const char *engine_key);

/** Atomically replace every non-legacy engine for a stream. */
int db_detection_engines_replace_custom(
    const char *stream_name, const stream_detection_engine_t *engines,
    size_t count);

/** Validate a caller-supplied engine without touching the database. */
int db_detection_engine_validate(const stream_detection_engine_t *engine,
                                 char *error, size_t error_size);

#endif
