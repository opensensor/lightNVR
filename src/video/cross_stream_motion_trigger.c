/**
 * Cross-Stream Motion Trigger Implementation
 *
 * Propagates a motion event from one stream to any linked streams via the
 * `motion_trigger_source` configuration. See cross_stream_motion_trigger.h
 * for the module-level rationale.
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <pthread.h>
#include <time.h>
#include <sys/stat.h>
#include <errno.h>

#include "video/cross_stream_motion_trigger.h"
#include "video/streams.h"
#include "video/stream_manager.h"
#include "video/unified_detection_thread.h"
#include "core/logger.h"
#include "core/config.h"
#include "core/path_utils.h"
#include "core/shutdown_coordinator.h"
#include "utils/strings.h"
#include "database/database_manager.h"
#include "database/db_recordings.h"
#include "database/db_streams.h"

/**
 * Process a motion event
 */
int process_motion_event(const char *stream_name, bool motion_detected, time_t timestamp, bool is_propagated) {
    if (!stream_name) {
        return -1;
    }

    // Create motion event
    motion_event_t event;
    memset(&event, 0, sizeof(motion_event_t));
    safe_strcpy(event.stream_name, stream_name, MAX_STREAM_NAME, 0);
    event.timestamp = timestamp;
    event.active = motion_detected;
    event.confidence = 1.0f;
    safe_strcpy(event.event_type, "motion", sizeof(event.event_type), 0);

    log_debug("Processing motion event for stream: %s (active: %d)", stream_name, motion_detected);

    // Cross-stream motion trigger: propagate this event to any streams that
    // have their motion_trigger_source set to the current stream's name,
    // AND — for bidirectional dual-lens support — also notify the stream
    // that is configured as THIS stream's own motion_trigger_source.
    //
    // Bidirectional design (TP-Link C545D and similar):
    //   Both lenses post motion events to a single shared ONVIF endpoint.
    //   Whichever lens's ONVIF thread consumes the event first should
    //   immediately trigger recording on the other lens, regardless of
    //   which direction the motion_trigger_source relationship is configured.
    //
    // Loop prevention:
    //   The is_propagated flag is set on every forwarded event.  A stream
    //   that receives a propagated event will NOT propagate it further,
    //   breaking any potential ping-pong between two linked streams.
    //
    // The target stream is triggered by calling unified_detection_notify_motion,
    // which sets the external motion trigger for the designated stream.
    //
    // Both paths are always attempted; whichever system does not own the
    // target stream silently ignores the call.

    // Only propagate if this event was not itself already a propagated copy.
    // is_propagated is passed explicitly by the caller; the local event struct
    // is always zero-initialised and cannot carry this flag reliably.
    if (is_propagated) {
        return 0;
    }

    int max_streams = g_config.max_streams > 0 ? g_config.max_streams : MAX_STREAMS;
    stream_config_t *all_streams = calloc(max_streams, sizeof(stream_config_t));
    if (all_streams) {
        int count = get_all_stream_configs(all_streams, max_streams);

        // Collect our own motion_trigger_source (reverse direction)
        char own_trigger_source[MAX_STREAM_NAME] = {0};
        for (int i = 0; i < count; i++) {
            if (strcmp(all_streams[i].name, stream_name) == 0 &&
                all_streams[i].motion_trigger_source[0] != '\0') {
                safe_strcpy(own_trigger_source, all_streams[i].motion_trigger_source,
                            MAX_STREAM_NAME, 0);
                break;
            }
        }

        for (int i = 0; i < count; i++) {
            // Forward direction: streams that list us as their trigger source
            bool forward = (all_streams[i].motion_trigger_source[0] != '\0' &&
                            strcmp(all_streams[i].motion_trigger_source, stream_name) == 0);
            // Reverse direction: the stream we list as our own trigger source
            bool reverse = (own_trigger_source[0] != '\0' &&
                            strcmp(all_streams[i].name, own_trigger_source) == 0);

            if (!forward && !reverse) continue;
            // Never echo back to ourselves
            if (strcmp(all_streams[i].name, stream_name) == 0) continue;

            motion_event_t linked_event;
            memset(&linked_event, 0, sizeof(motion_event_t));
            safe_strcpy(linked_event.stream_name, all_streams[i].name, MAX_STREAM_NAME, 0);
            linked_event.timestamp = timestamp;
            linked_event.active = motion_detected;
            linked_event.confidence = 1.0f;
            linked_event.is_propagated = true;  // block second-order propagation
            safe_strcpy(linked_event.event_type, "motion", sizeof(linked_event.event_type), 0);

            log_info("Propagated motion event (%s) from '%s' to linked ONVIF stream '%s' (%s)",
                         motion_detected ? "start" : "end", stream_name, all_streams[i].name,
                         reverse ? "reverse" : "forward");

            // UDT external trigger
            unified_detection_notify_motion(all_streams[i].name, motion_detected);
        }
        free(all_streams);
    }

    return 0;
}
