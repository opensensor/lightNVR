#ifndef LIGHTNVR_STREAM_OBSERVATION_SETTINGS_H
#define LIGHTNVR_STREAM_OBSERVATION_SETTINGS_H

/*
 * INT_01 L9: per-camera observation evidence settings on the stream API
 * (docs/integrations/OBSERVATIONS_CONTRACT.md section 13).
 *
 * JSON fields (stream GET/POST/PUT):
 *   observation_evidence_enabled  bool, default false
 *   observation_labels            CSV, <= 16 labels of 1..31 bytes; "" = all
 *   observation_max_age_days      integer 1..90, default 7
 *   observation_max_bytes         integer 64 MiB..1 TiB, default 2 GiB
 *
 * Migration 0095 only enforces > 0, so these bounds are enforced here.
 */

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>

#include <cjson/cJSON.h>
#include "core/config.h"

#define OBSERVATION_LABELS_MAX_COUNT 16
#define OBSERVATION_LABEL_MAX_BYTES 31
#define OBSERVATION_MAX_AGE_DAYS_MIN 1
#define OBSERVATION_MAX_AGE_DAYS_MAX 90
#define OBSERVATION_MAX_BYTES_MIN (64LL * 1024 * 1024)
#define OBSERVATION_MAX_BYTES_MAX (1024LL * 1024 * 1024 * 1024)

/* Add the four fields to a stream JSON object (budgets <= 0 are reported as
 * the defaults the database stores). */
void stream_observation_settings_to_json(cJSON *object, const stream_config_t *config);

/*
 * Validate and apply whichever of the four fields body carries. Nothing in
 * config changes unless every present field is valid. Returns 0 on success
 * (changed set when a value differs) or -1 with a client message in error.
 */
int stream_observation_settings_apply_json(const cJSON *body, stream_config_t *config,
                                           bool *changed, char *error, size_t error_size);

/* Normalize a label CSV (trim each entry) into out. Returns 0, or -1 with a
 * message in error when the list violates the bounds. */
int stream_observation_labels_normalize(const char *input, char *out, size_t out_size,
                                        char *error, size_t error_size);

#endif
