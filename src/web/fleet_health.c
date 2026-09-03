#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>

#include "core/config.h"
#include "video/go2rtc/go2rtc_api.h"
#include "video/go2rtc/go2rtc_integration.h"
#include "web/fleet_health.h"

static bool activity_matches_camera(
    const go2rtc_stream_activity_t *entry, const fleet_camera_t *camera) {
    return entry && entry->video_active && entry->stream_name && camera &&
        strcmp(entry->stream_name, camera->name) == 0;
}

void fleet_health_apply_go2rtc_activity(
    fleet_camera_t *cameras, int camera_count,
    const go2rtc_stream_activity_t *activity, size_t activity_count,
    time_t observed_at) {
    if (!cameras || camera_count <= 0 || !activity) return;

    for (int i = 0; i < camera_count; i++) {
        fleet_camera_t *camera = &cameras[i];
        if (!camera->enabled || !camera->streaming_enabled ||
            camera->health != FLEET_HEALTH_UNKNOWN) {
            continue;
        }
        bool active = false;
        for (size_t j = 0; j < activity_count; j++) {
            if (activity_matches_camera(&activity[j], camera)) {
                active = true;
                break;
            }
        }
        if (!active) continue;

        camera->health = FLEET_HEALTH_UP;
        camera->availability = FLEET_AVAILABILITY_LIVE;
        camera->health_changed_at = (int64_t)observed_at;
        camera->last_frame_ts = (int64_t)observed_at;
        if (camera->first_video_at == 0) {
            camera->first_video_at = (int64_t)observed_at;
        }
    }
}

void fleet_health_enrich_go2rtc_activity(fleet_camera_t *cameras,
                                         int camera_count) {
    if (!cameras || camera_count <= 0 ||
        !go2rtc_integration_is_initialized()) {
        return;
    }

    go2rtc_stream_activity_t *activity =
        calloc((size_t)camera_count, sizeof(*activity));
    char (*substream_names)[MAX_STREAM_NAME + 5] =
        calloc((size_t)camera_count, sizeof(*substream_names));
    if (!activity || !substream_names) {
        free(activity);
        free(substream_names);
        return;
    }

    size_t activity_count = 0;
    for (int i = 0; i < camera_count; i++) {
        if (!cameras[i].enabled || !cameras[i].streaming_enabled ||
            cameras[i].health != FLEET_HEALTH_UNKNOWN) {
            continue;
        }
        int written = snprintf(substream_names[activity_count],
                               sizeof(substream_names[activity_count]),
                               "%s_sub", cameras[i].name);
        activity[activity_count].stream_name = cameras[i].name;
        if (written > 0 &&
            (size_t)written < sizeof(substream_names[activity_count])) {
            activity[activity_count].alternate_stream_name =
                substream_names[activity_count];
        }
        activity_count++;
    }

    if (activity_count > 0 &&
        go2rtc_api_get_stream_activity(activity, activity_count)) {
        fleet_health_apply_go2rtc_activity(
            cameras, camera_count, activity, activity_count, time(NULL));
    }
    free(substream_names);
    free(activity);
}
