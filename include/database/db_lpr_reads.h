#ifndef LIGHTNVR_DB_LPR_READS_H
#define LIGHTNVR_DB_LPR_READS_H

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>

#include "core/config.h"
#include "video/onvif_event.h"

#define LPR_READ_UUID_SIZE 37
#define LPR_SOURCE_MAX 32
#define LPR_QUERY_MAX 64

typedef struct {
    char camera_uuid[CAMERA_UUID_STRING_SIZE];
    char stream_name[MAX_STREAM_NAME];
    int64_t observed_at_ms;
    char source[LPR_SOURCE_MAX];
    char vendor_topic[ONVIF_EVENT_TOPIC_MAX];
    char plate[ONVIF_LPR_PLATE_MAX];
    bool has_confidence;
    float confidence;
    char country[ONVIF_LPR_TEXT_MAX];
    char region[ONVIF_LPR_TEXT_MAX];
    char plate_type[ONVIF_LPR_TEXT_MAX];
    char direction[ONVIF_LPR_TEXT_MAX];
    char lane[ONVIF_LPR_TEXT_MAX];
    char vehicle_type[ONVIF_LPR_TEXT_MAX];
    char vehicle_color[ONVIF_LPR_TEXT_MAX];
    char object_id[ONVIF_LPR_ID_MAX];
    char correlation_id[ONVIF_LPR_ID_MAX];
    bool has_bounding_box;
    float bbox_left;
    float bbox_top;
    float bbox_right;
    float bbox_bottom;
    uint64_t recording_id;
} lpr_read_input_t;

typedef struct {
    char uuid[LPR_READ_UUID_SIZE];
    lpr_read_input_t read;
    int64_t received_at_ms;
} lpr_read_t;

typedef enum {
    LPR_MATCH_NONE = 0,
    LPR_MATCH_EXACT,
    LPR_MATCH_PARTIAL
} lpr_match_mode_t;

typedef struct {
    char camera_uuid[CAMERA_UUID_STRING_SIZE]; /* optional */
    int64_t start_at_ms;                       /* required */
    int64_t end_at_ms;                         /* required */
    lpr_match_mode_t match_mode;
    char plate_query[LPR_QUERY_MAX];
    int limit;                                 /* 1..100 */
} lpr_read_query_t;

/** Store a protected read. Returns 0 inserted, 1 duplicate, -1 error. */
int db_lpr_read_insert(const lpr_read_input_t *input,
                       char uuid_out[LPR_READ_UUID_SIZE]);

/** Search and decrypt authorized results. Time bounds are mandatory. */
int db_lpr_reads_search(const lpr_read_query_t *query,
                        lpr_read_t *reads, size_t capacity);

int db_lpr_read_delete(const char *uuid);

/** Resolve ownership without decrypting the protected value. */
int db_lpr_read_get_camera_uuid(
    const char *uuid, char camera_uuid[CAMERA_UUID_STRING_SIZE]);

/** Delete a bounded batch older than the supplied receive-time cutoff. */
int db_lpr_reads_prune(int64_t received_before_ms, int batch_limit);

/** Map the structured parser result into the protected database shape. */
int db_lpr_read_from_onvif(const char *camera_uuid, const char *stream_name,
                           const onvif_lpr_event_t *event,
                           lpr_read_input_t *input);

#endif
