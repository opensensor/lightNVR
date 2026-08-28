#define _GNU_SOURCE
#define _POSIX_C_SOURCE 200809L

#include "video/onvif_event.h"

#include <ctype.h>
#include <errno.h>
#include <math.h>
#include <stdlib.h>
#include <string.h>
#include <strings.h>
#include <time.h>

#include "ezxml.h"

#define ONVIF_EVENT_XML_MAX_BYTES (4U * 1024U * 1024U)

static const char *local_name(ezxml_t node) {
    const char *name = (node && node->name) ? node->name : "";
    const char *colon = strrchr(name, ':');
    return colon ? colon + 1 : name;
}

static bool ci_contains(const char *haystack, const char *needle) {
    if (!haystack || !needle || !needle[0]) return false;
    size_t needle_len = strlen(needle);
    for (const char *p = haystack; *p; ++p) {
        if (strncasecmp(p, needle, needle_len) == 0) return true;
    }
    return false;
}

static void copy_text(char *dst, size_t size, const char *src) {
    if (!dst || size == 0 || !src || dst[0]) return;
    while (*src && isspace((unsigned char)*src)) ++src;
    size_t length = strlen(src);
    while (length > 0 && isspace((unsigned char)src[length - 1])) --length;
    if (length >= size) length = size - 1;
    memcpy(dst, src, length);
    dst[length] = '\0';
}

static void copy_plate_text(char *dst, size_t size, const char *src) {
    if (!dst || size == 0 || !src || dst[0]) return;
    while (*src && isspace((unsigned char)*src)) ++src;
    size_t length = strlen(src);
    while (length > 0 && isspace((unsigned char)src[length - 1])) --length;
    /* Never create a plausible but incorrect read by truncating its value. */
    if (length == 0 || length >= size) return;
    memcpy(dst, src, length);
    dst[length] = '\0';
}

static bool name_is(const char *actual, const char *const *names, size_t count) {
    if (!actual) return false;
    for (size_t i = 0; i < count; ++i) {
        if (strcasecmp(actual, names[i]) == 0) return true;
    }
    return false;
}

static bool topic_is_standard_lpr(const char *topic) {
    return ci_contains(topic, "RuleEngine/Recognition/LicensePlate");
}

static bool topic_is_vendor_lpr(const char *topic) {
    if (!topic) return false;
    if (ci_contains(topic, "LicensePlate") || ci_contains(topic, "ANPR") ||
        ci_contains(topic, "ALPR")) return true;

    /* Treat LPR as a path/token, not a substring of arbitrary topic text. */
    for (const char *p = topic; *p; ++p) {
        if (strncasecmp(p, "LPR", 3) != 0) continue;
        bool left = p == topic || !isalnum((unsigned char)p[-1]);
        bool right = !isalnum((unsigned char)p[3]);
        if (left && right) return true;
    }
    return false;
}

static ezxml_t first_descendant(ezxml_t node, const char *wanted) {
    for (ezxml_t child = node ? node->child : NULL; child; child = child->ordered) {
        if (strcasecmp(local_name(child), wanted) == 0) return child;
        ezxml_t found = first_descendant(child, wanted);
        if (found) return found;
    }
    return NULL;
}

static ezxml_t first_descendant_with_attr(ezxml_t node, const char *wanted,
                                          const char *attribute) {
    for (ezxml_t child = node ? node->child : NULL; child; child = child->ordered) {
        if (strcasecmp(local_name(child), wanted) == 0 &&
            ezxml_attr(child, attribute) != NULL) return child;
        ezxml_t found = first_descendant_with_attr(child, wanted, attribute);
        if (found) return found;
    }
    return NULL;
}

static float normalized_confidence(const char *value, bool *valid) {
    if (valid) *valid = false;
    if (!value || !value[0]) return 0.0f;
    char *end = NULL;
    errno = 0;
    float result = strtof(value, &end);
    if (errno || end == value) return 0.0f;
    while (*end && isspace((unsigned char)*end)) ++end;
    if (*end || result < 0.0f) return 0.0f;
    if (result > 1.0f && result <= 100.0f) result /= 100.0f;
    if (result > 1.0f) return 0.0f;
    if (valid) *valid = true;
    return result;
}

static int64_t parse_utc_ms(const char *value) {
    if (!value || strlen(value) < 20) return 0;
    struct tm utc;
    memset(&utc, 0, sizeof(utc));
    char *end = strptime(value, "%Y-%m-%dT%H:%M:%S", &utc);
    if (!end) return 0;

    int milliseconds = 0;
    if (*end == '.') {
        ++end;
        int digits = 0;
        while (isdigit((unsigned char)*end)) {
            if (digits < 3) milliseconds = milliseconds * 10 + (*end - '0');
            ++digits;
            ++end;
        }
        while (digits++ < 3) milliseconds *= 10;
    }
    if (*end != 'Z' && *end != '\0') return 0;
    time_t seconds = timegm(&utc);
    return seconds < 0 ? 0 : (int64_t)seconds * 1000 + milliseconds;
}

static void set_named_value(onvif_lpr_event_t *event,
                            const char *name, const char *value) {
    static const char *plate_names[] = {
        "PlateNumber", "LicensePlate", "Plate", "PlateNo", "PlateText"
    };
    static const char *confidence_names[] = {
        "Likelihood", "Confidence", "PlateConfidence", "RecognitionConfidence"
    };
    static const char *country_names[] = {"CountryCode", "Country"};
    static const char *region_names[] = {"IssuingEntity", "Region", "State"};
    static const char *plate_type_names[] = {"PlateType", "LicensePlateType"};
    static const char *direction_names[] = {"Direction", "MotionDirection"};
    static const char *lane_names[] = {"Lane", "LaneId", "LaneID"};
    static const char *vehicle_type_names[] = {"VehicleType", "VehicleClass"};
    static const char *vehicle_color_names[] = {"VehicleColor", "Color"};
    static const char *object_names[] = {"ObjectId", "ObjectID", "ObjectToken"};
    static const char *correlation_names[] = {
        "CorrelationId", "CorrelationID", "FrameId", "FrameID"
    };
    if (!name || !value) return;

    if (name_is(name, plate_names, sizeof(plate_names) / sizeof(plate_names[0]))) {
        copy_plate_text(event->plate, sizeof(event->plate), value);
    } else if (name_is(name, confidence_names,
                       sizeof(confidence_names) / sizeof(confidence_names[0]))) {
        bool valid = false;
        float confidence = normalized_confidence(value, &valid);
        if (valid && !event->has_confidence) {
            event->confidence = confidence;
            event->has_confidence = true;
        }
    } else if (name_is(name, country_names, sizeof(country_names) / sizeof(country_names[0]))) {
        copy_text(event->country, sizeof(event->country), value);
    } else if (name_is(name, region_names, sizeof(region_names) / sizeof(region_names[0]))) {
        copy_text(event->region, sizeof(event->region), value);
    } else if (name_is(name, plate_type_names, sizeof(plate_type_names) / sizeof(plate_type_names[0]))) {
        copy_text(event->plate_type, sizeof(event->plate_type), value);
    } else if (name_is(name, direction_names, sizeof(direction_names) / sizeof(direction_names[0]))) {
        copy_text(event->direction, sizeof(event->direction), value);
    } else if (name_is(name, lane_names, sizeof(lane_names) / sizeof(lane_names[0]))) {
        copy_text(event->lane, sizeof(event->lane), value);
    } else if (name_is(name, vehicle_type_names, sizeof(vehicle_type_names) / sizeof(vehicle_type_names[0]))) {
        copy_text(event->vehicle_type, sizeof(event->vehicle_type), value);
    } else if (name_is(name, vehicle_color_names, sizeof(vehicle_color_names) / sizeof(vehicle_color_names[0]))) {
        copy_text(event->vehicle_color, sizeof(event->vehicle_color), value);
    } else if (name_is(name, object_names, sizeof(object_names) / sizeof(object_names[0]))) {
        copy_text(event->object_id, sizeof(event->object_id), value);
    } else if (name_is(name, correlation_names, sizeof(correlation_names) / sizeof(correlation_names[0]))) {
        copy_text(event->correlation_id, sizeof(event->correlation_id), value);
    }
}

static void read_bbox(onvif_lpr_event_t *event, ezxml_t node) {
    const char *left = ezxml_attr(node, "left");
    const char *top = ezxml_attr(node, "top");
    const char *right = ezxml_attr(node, "right");
    const char *bottom = ezxml_attr(node, "bottom");
    if (!left) left = ezxml_attr(node, "Left");
    if (!top) top = ezxml_attr(node, "Top");
    if (!right) right = ezxml_attr(node, "Right");
    if (!bottom) bottom = ezxml_attr(node, "Bottom");
    if (!left || !top || !right || !bottom) return;
    char *end = NULL;
    errno = 0;
    event->bbox_left = strtof(left, &end);
    if (errno || end == left || *end || !isfinite(event->bbox_left)) return;
    errno = 0;
    event->bbox_top = strtof(top, &end);
    if (errno || end == top || *end || !isfinite(event->bbox_top)) return;
    errno = 0;
    event->bbox_right = strtof(right, &end);
    if (errno || end == right || *end || !isfinite(event->bbox_right)) return;
    errno = 0;
    event->bbox_bottom = strtof(bottom, &end);
    if (errno || end == bottom || *end || !isfinite(event->bbox_bottom)) return;
    if (event->bbox_left < -1.0f || event->bbox_left > 1.0f ||
        event->bbox_top < -1.0f || event->bbox_top > 1.0f ||
        event->bbox_right < -1.0f || event->bbox_right > 1.0f ||
        event->bbox_bottom < -1.0f || event->bbox_bottom > 1.0f ||
        event->bbox_right < event->bbox_left ||
        event->bbox_bottom < event->bbox_top) return;
    event->has_bounding_box = true;
}

static void collect_values(ezxml_t node, onvif_lpr_event_t *event) {
    for (ezxml_t child = node ? node->child : NULL; child; child = child->ordered) {
        const char *name = local_name(child);
        if (strcasecmp(name, "SimpleItem") == 0) {
            set_named_value(event, ezxml_attr(child, "Name"),
                            ezxml_attr(child, "Value"));
        } else {
            const char *text = ezxml_txt(child);
            if (text && text[0]) set_named_value(event, name, text);
            const char *likelihood = ezxml_attr(child, "Likelihood");
            if (likelihood) set_named_value(event, "Likelihood", likelihood);
            if (strcasecmp(name, "BoundingBox") == 0 ||
                strcasecmp(name, "Rectangle") == 0) read_bbox(event, child);
        }
        collect_values(child, event);
    }
}

static bool has_explicit_false_state(ezxml_t node) {
    for (ezxml_t child = node ? node->child : NULL; child; child = child->ordered) {
        const char *operation = ezxml_attr(child, "PropertyOperation");
        if (operation && (strcasecmp(operation, "Deleted") == 0 ||
                          strcasecmp(operation, "Removed") == 0)) return true;
        const char *text = ezxml_txt(child);
        if ((strcasecmp(local_name(child), "State") == 0 ||
             strcasecmp(local_name(child), "IsRecognized") == 0 ||
             strcasecmp(local_name(child), "IsLicensePlate") == 0) &&
            text && (strcasecmp(text, "false") == 0 || strcmp(text, "0") == 0)) {
            return true;
        }
        if (strcasecmp(local_name(child), "SimpleItem") == 0) {
            const char *name = ezxml_attr(child, "Name");
            const char *value = ezxml_attr(child, "Value");
            if (name && value &&
                (strcasecmp(name, "State") == 0 ||
                 strcasecmp(name, "IsRecognized") == 0 ||
                 strcasecmp(name, "IsLicensePlate") == 0) &&
                (strcasecmp(value, "false") == 0 || strcmp(value, "0") == 0)) {
                return true;
            }
        }
        if (has_explicit_false_state(child)) return true;
    }
    return false;
}

static int parse_notification(ezxml_t notification, onvif_lpr_event_t *event) {
    ezxml_t topic_node = first_descendant(notification, "Topic");
    const char *topic = topic_node ? ezxml_txt(topic_node) : NULL;
    bool standard = topic_is_standard_lpr(topic);
    if (!standard && !topic_is_vendor_lpr(topic)) return 0;

    memset(event, 0, sizeof(*event));
    event->source = standard ? ONVIF_LPR_SOURCE_PROFILE_M
                             : ONVIF_LPR_SOURCE_VENDOR_TOPIC;
    event->asserted = !has_explicit_false_state(notification);
    copy_text(event->topic, sizeof(event->topic), topic);

    ezxml_t message = first_descendant_with_attr(notification, "Message", "UtcTime");
    if (message) event->observed_at_ms = parse_utc_ms(ezxml_attr(message, "UtcTime"));
    collect_values(notification, event);

    /* A cleared state and events without readable plate text are not reads. */
    return event->asserted && event->plate[0] ? 1 : 0;
}

static int walk_notifications(ezxml_t node, onvif_lpr_event_t *events,
                              size_t capacity, size_t *count) {
    for (ezxml_t child = node ? node->child : NULL; child; child = child->ordered) {
        if (strcasecmp(local_name(child), "NotificationMessage") == 0) {
            onvif_lpr_event_t parsed;
            if (parse_notification(child, &parsed) == 1 && *count < capacity) {
                events[(*count)++] = parsed;
            }
        } else {
            walk_notifications(child, events, capacity, count);
        }
    }
    return 0;
}

int onvif_parse_lpr_events(const char *xml_text,
                           onvif_lpr_event_t *events,
                           size_t capacity) {
    if (!xml_text || !events || capacity == 0 ||
        strnlen(xml_text, ONVIF_EVENT_XML_MAX_BYTES + 1) >
            ONVIF_EVENT_XML_MAX_BYTES) return -1;
    char *copy = strdup(xml_text);
    if (!copy) return -1;
    ezxml_t root = ezxml_parse_str(copy, strlen(copy));
    if (!root) {
        free(copy);
        return -1;
    }
    if (ezxml_error(root)[0] != '\0') {
        ezxml_free(root);
        free(copy);
        return -1;
    }
    size_t count = 0;
    if (strcasecmp(local_name(root), "NotificationMessage") == 0) {
        onvif_lpr_event_t parsed;
        if (parse_notification(root, &parsed) == 1) events[count++] = parsed;
    } else {
        walk_notifications(root, events, capacity, &count);
    }
    ezxml_free(root);
    free(copy);
    return (int)count;
}
