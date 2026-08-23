#ifndef LIGHTNVR_CORE_EVENT_ENVELOPE_H
#define LIGHTNVR_CORE_EVENT_ENVELOPE_H

#include <stddef.h>
#include <stdint.h>
#include <time.h>

#include <cjson/cJSON.h>

#define EVENT_ID_MAX 37
#define EVENT_TYPE_MAX 96
#define EVENT_SOURCE_MAX 128
#define EVENT_SUBJECT_MAX 96
#define EVENT_TIME_MAX 32
#define EVENT_CONTENT_TYPE_MAX 32
#define EVENT_DATA_MAX_BYTES (64U * 1024U)
#define EVENT_ENVELOPE_MAX_BYTES (96U * 1024U)

typedef enum {
    EVENT_SEVERITY_INFO = 0,
    EVENT_SEVERITY_WARNING,
    EVENT_SEVERITY_ERROR,
    EVENT_SEVERITY_CRITICAL
} event_severity_t;

typedef enum {
    EVENT_SENSITIVITY_OPERATIONAL = 0,
    EVENT_SENSITIVITY_INTERNAL,
    EVENT_SENSITIVITY_RESTRICTED
} event_sensitivity_t;

typedef enum {
    EVENT_MEDIA_FORBIDDEN = 0,
    EVENT_MEDIA_REFERENCE_ALLOWED,
    EVENT_MEDIA_BINARY_OPT_IN
} event_media_policy_t;

typedef enum {
    EVENT_RATE_LOW = 0,
    EVENT_RATE_MEDIUM,
    EVENT_RATE_HIGH
} event_expected_rate_t;

typedef enum {
    EVENT_SUBJECT_CAMERA = 0,
    EVENT_SUBJECT_STORAGE
} event_subject_kind_t;

typedef struct {
    const char *type;
    const char *family;
    const char *description;
    event_severity_t severity;
    event_sensitivity_t sensitivity;
    event_media_policy_t media_policy;
    event_expected_rate_t expected_rate;
    event_subject_kind_t subject_kind;
    int default_expiry_seconds;
} event_type_definition_t;

typedef struct {
    char specversion[4];
    char id[EVENT_ID_MAX];
    char type[EVENT_TYPE_MAX];
    char source[EVENT_SOURCE_MAX];
    char subject[EVENT_SUBJECT_MAX];
    char time[EVENT_TIME_MAX];
    char datacontenttype[EVENT_CONTENT_TYPE_MAX];
    time_t occurred_at;
    time_t expires_at;
    cJSON *data;
} event_envelope_t;

const event_type_definition_t *event_registry_all(int *count);
const event_type_definition_t *event_registry_find(const char *type);

const char *event_severity_name(event_severity_t severity);
const char *event_sensitivity_name(event_sensitivity_t sensitivity);
const char *event_media_policy_name(event_media_policy_t policy);
const char *event_expected_rate_name(event_expected_rate_t rate);

/*
 * Create an immutable-identity event from a registered type. data must be a
 * JSON object and is deep-copied; the caller retains ownership of the input.
 * source must use the urn:lightnvr:<installation-uuid> form. occurred_at=0 uses
 * the current time. Returns 0 on success and -1 with a stable error message on
 * invalid input or allocation/randomness failure.
 */
int event_envelope_create(event_envelope_t *event, const char *type,
                          const char *source, const char *subject,
                          time_t occurred_at, const cJSON *data,
                          char *error, size_t error_size);

/*
 * Validate a constructed or decoded envelope against the registry contract.
 * This runs on every internal hand-off, so it performs no allocation: the
 * EVENT_DATA_MAX_BYTES bound is enforced once by event_envelope_create() and
 * EVENT_ENVELOPE_MAX_BYTES by event_envelope_serialize(). Envelopes are
 * immutable after creation, so both bounds hold for every clone.
 */
int event_envelope_validate(const event_envelope_t *event,
                            char *error, size_t error_size);

/* Serialize to a newly allocated compact JSON string owned by the caller. */
char *event_envelope_serialize(const event_envelope_t *event,
                               char *error, size_t error_size);

/* Deep-copy a validated envelope while preserving its immutable identity. */
int event_envelope_clone(event_envelope_t *destination,
                         const event_envelope_t *source, char *error,
                         size_t error_size);

void event_envelope_clear(event_envelope_t *event);

#endif /* LIGHTNVR_CORE_EVENT_ENVELOPE_H */
