#ifndef LIGHTNVR_WEB_FLEET_HEALTH_H
#define LIGHTNVR_WEB_FLEET_HEALTH_H

#include <stddef.h>
#include <time.h>

#include "core/camera_selector.h"
#include "video/go2rtc/go2rtc_api.h"

/* Apply an already-fetched go2rtc activity snapshot. This is public so the
 * merge rules can be tested without starting a go2rtc process. */
void fleet_health_apply_go2rtc_activity(
    fleet_camera_t *cameras, int camera_count,
    const go2rtc_stream_activity_t *activity, size_t activity_count,
    time_t observed_at);

/* Promote otherwise-unobserved cameras when go2rtc is currently receiving
 * video for either the main stream or its grid-view substream. This performs
 * one bulk local API request, never one request per camera. */
void fleet_health_enrich_go2rtc_activity(fleet_camera_t *cameras,
                                         int camera_count);

#endif /* LIGHTNVR_WEB_FLEET_HEALTH_H */
