#ifndef LIGHTNVR_ONVIF_EVENT_H
#define LIGHTNVR_ONVIF_EVENT_H

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>

#define ONVIF_EVENT_TOPIC_MAX 256
#define ONVIF_LPR_PLATE_MAX 64
#define ONVIF_LPR_TEXT_MAX 64
#define ONVIF_LPR_ID_MAX 96

typedef enum {
    ONVIF_LPR_SOURCE_PROFILE_M = 0,
    ONVIF_LPR_SOURCE_VENDOR_TOPIC = 1
} onvif_lpr_source_t;

typedef struct {
    onvif_lpr_source_t source;
    bool asserted;
    int64_t observed_at_ms;
    char topic[ONVIF_EVENT_TOPIC_MAX];
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
} onvif_lpr_event_t;

/**
 * Parse plate recognition notifications from a PullMessages or metadata XML
 * envelope. Unknown notifications are ignored. A result is emitted only when
 * a recognized LPR topic contains a plate value.
 *
 * @return number of parsed events, or -1 for malformed input/arguments.
 */
int onvif_parse_lpr_events(const char *xml_text,
                           onvif_lpr_event_t *events,
                           size_t capacity);

#endif
