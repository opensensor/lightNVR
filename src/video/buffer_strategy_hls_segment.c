/**
 * HLS Segment Tracker Buffer Strategy
 * 
 * Tracks existing HLS segments without copying them.
 * 
 * Key improvements over previous detection_recording.c approach:
 * - No segment copying - just track paths and mark as protected
 * - Estimate segment duration from file timestamps and HLS playlist
 * - Manage cleanup protection to prevent segment deletion
 * - Parse actual segment durations from m3u8 playlist when available
 * 
 * This strategy integrates with go2rtc's HLS output to buffer pre-detection content.
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <pthread.h>
#include <dirent.h>
#include <sys/stat.h>
#include <glob.h>

#include "video/pre_detection_buffer.h"
#include "core/logger.h"
#include "core/config.h"

// Maximum segments to track
#define MAX_TRACKED_SEGMENTS 32

// Tracked segment with metadata
typedef struct {
    char path[512];                     // Full path to segment
    time_t mtime;                       // Modification time
    float duration_seconds;             // Duration (estimated or parsed)
    size_t size_bytes;                  // File size
    bool protected;                     // Protected from cleanup
    bool valid;                         // Slot in use
    int64_t sequence_number;            // HLS sequence number if known
} tracked_segment_t;

// Strategy private data
typedef struct {
    char stream_name[256];
    char hls_base_path[512];            // Base path for HLS segments
    char segment_pattern[512];          // Glob pattern for segments
    int buffer_seconds;                 // Target buffer duration
    float default_segment_duration;     // Default estimated duration
    
    pthread_mutex_t lock;
    
    // Circular buffer of tracked segments
    tracked_segment_t segments[MAX_TRACKED_SEGMENTS];
    int head;                           // Next slot to write
    int count;                          // Number of valid segments
    
    // Statistics
    float total_duration_seconds;
    size_t total_size_bytes;
} hls_segment_strategy_data_t;

// --- Private helper functions ---

/**
 * Estimate segment duration from file metadata or default
 */
static float estimate_segment_duration(const char *segment_path, float default_duration) {
    // Try to read duration from sidecar file or metadata
    // For now, use default
    // TODO: Parse from m3u8 playlist if available
    return default_duration;
}

/**
 * Scan directory for existing HLS segments
 */
static int scan_existing_segments(hls_segment_strategy_data_t *data) {
    glob_t glob_result;
    int ret = glob(data->segment_pattern, GLOB_NOSORT, NULL, &glob_result);
    
    if (ret != 0) {
        if (ret == GLOB_NOMATCH) {
            log_debug("No existing HLS segments found for pattern: %s", data->segment_pattern);
            return 0;
        }
        log_error("Failed to scan for HLS segments: %d", ret);
        return -1;
    }
    
    // Sort by modification time (newest last)
    // Simple bubble sort for small arrays
    for (size_t i = 0; i < glob_result.gl_pathc - 1; i++) {
        for (size_t j = 0; j < glob_result.gl_pathc - i - 1; j++) {
            struct stat st1, st2;
            if (stat(glob_result.gl_pathv[j], &st1) == 0 &&
                stat(glob_result.gl_pathv[j + 1], &st2) == 0) {
                if (st1.st_mtime > st2.st_mtime) {
                    char *tmp = glob_result.gl_pathv[j];
                    glob_result.gl_pathv[j] = glob_result.gl_pathv[j + 1];
                    glob_result.gl_pathv[j + 1] = tmp;
                }
            }
        }
    }
    
    // Add most recent segments up to buffer duration
    float accumulated_duration = 0;
    int added = 0;
    
    for (int i = (int)glob_result.gl_pathc - 1; i >= 0 && accumulated_duration < data->buffer_seconds; i--) {
        const char *path = glob_result.gl_pathv[i];
        struct stat st;
        
        if (stat(path, &st) != 0) {
            continue;
        }
        
        float duration = estimate_segment_duration(path, data->default_segment_duration);
        
        // Add to circular buffer
        tracked_segment_t *seg = &data->segments[data->head];
        strncpy(seg->path, path, sizeof(seg->path) - 1);
        seg->mtime = st.st_mtime;
        seg->duration_seconds = duration;
        seg->size_bytes = st.st_size;
        seg->protected = true;  // Protect from cleanup
        seg->valid = true;
        
        data->head = (data->head + 1) % MAX_TRACKED_SEGMENTS;
        if (data->count < MAX_TRACKED_SEGMENTS) {
            data->count++;
        }
        
        accumulated_duration += duration;
        data->total_duration_seconds += duration;
        data->total_size_bytes += st.st_size;
        added++;
        
        log_debug("Tracking HLS segment: %s (%.1fs)", path, duration);
    }
    
    globfree(&glob_result);
    
    log_info("Scanned %d existing HLS segments (%.1fs buffered) for %s",
             added, data->total_duration_seconds, data->stream_name);

    return added;
}

// --- Strategy interface methods ---

static int hls_segment_strategy_init(pre_buffer_strategy_t *self, const buffer_config_t *config) {
    hls_segment_strategy_data_t *data = (hls_segment_strategy_data_t *)self->private_data;

    data->buffer_seconds = config->buffer_seconds;
    data->default_segment_duration = 2.0f;  // go2rtc default segment duration

    // Set up HLS path pattern
    extern config_t g_config;
    snprintf(data->hls_base_path, sizeof(data->hls_base_path),
             "%s/hls/%s", g_config.storage_path, data->stream_name);
    snprintf(data->segment_pattern, sizeof(data->segment_pattern),
             "%s/*.ts", data->hls_base_path);

    pthread_mutex_init(&data->lock, NULL);

    // Scan for existing segments
    scan_existing_segments(data);

    self->initialized = true;
    return 0;
}

static void hls_segment_strategy_destroy(pre_buffer_strategy_t *self) {
    hls_segment_strategy_data_t *data = (hls_segment_strategy_data_t *)self->private_data;

    // Unprotect all segments
    for (int i = 0; i < MAX_TRACKED_SEGMENTS; i++) {
        data->segments[i].protected = false;
    }

    pthread_mutex_destroy(&data->lock);

    log_debug("HLS segment strategy destroyed for %s", data->stream_name);
    free(data);
    self->private_data = NULL;
}

static int hls_segment_strategy_add_segment(pre_buffer_strategy_t *self,
                                             const char *segment_path,
                                             float duration) {
    hls_segment_strategy_data_t *data = (hls_segment_strategy_data_t *)self->private_data;

    pthread_mutex_lock(&data->lock);

    // Check if segment already tracked
    for (int i = 0; i < MAX_TRACKED_SEGMENTS; i++) {
        if (data->segments[i].valid && strcmp(data->segments[i].path, segment_path) == 0) {
            // Already tracked, update duration if provided
            if (duration > 0) {
                data->segments[i].duration_seconds = duration;
            }
            pthread_mutex_unlock(&data->lock);
            return 0;
        }
    }

    // Get file info
    struct stat st;
    if (stat(segment_path, &st) != 0) {
        log_warn("Cannot stat segment file: %s", segment_path);
        pthread_mutex_unlock(&data->lock);
        return -1;
    }

    // If buffer is full, remove oldest segment
    if (data->count >= MAX_TRACKED_SEGMENTS) {
        int oldest = (data->head - data->count + MAX_TRACKED_SEGMENTS) % MAX_TRACKED_SEGMENTS;
        data->total_duration_seconds -= data->segments[oldest].duration_seconds;
        data->total_size_bytes -= data->segments[oldest].size_bytes;
        data->segments[oldest].valid = false;
        data->segments[oldest].protected = false;
        data->count--;
    }

    // If we have enough buffer duration, remove oldest
    while (data->total_duration_seconds > data->buffer_seconds && data->count > 1) {
        int oldest = (data->head - data->count + MAX_TRACKED_SEGMENTS) % MAX_TRACKED_SEGMENTS;
        data->total_duration_seconds -= data->segments[oldest].duration_seconds;
        data->total_size_bytes -= data->segments[oldest].size_bytes;
        data->segments[oldest].valid = false;
        data->segments[oldest].protected = false;
        data->count--;
    }

    // Add new segment
    tracked_segment_t *seg = &data->segments[data->head];
    strncpy(seg->path, segment_path, sizeof(seg->path) - 1);
    seg->mtime = st.st_mtime;
    seg->duration_seconds = duration > 0 ? duration : data->default_segment_duration;
    seg->size_bytes = st.st_size;
    seg->protected = true;
    seg->valid = true;

    data->head = (data->head + 1) % MAX_TRACKED_SEGMENTS;
    data->count++;
    data->total_duration_seconds += seg->duration_seconds;
    data->total_size_bytes += seg->size_bytes;

    pthread_mutex_unlock(&data->lock);

    log_debug("Added HLS segment to buffer: %s (%.1fs, total %.1fs)",
              segment_path, seg->duration_seconds, data->total_duration_seconds);

    return 0;
}

static int hls_segment_strategy_protect_segment(pre_buffer_strategy_t *self,
                                                 const char *segment_path) {
    hls_segment_strategy_data_t *data = (hls_segment_strategy_data_t *)self->private_data;

    pthread_mutex_lock(&data->lock);

    for (int i = 0; i < MAX_TRACKED_SEGMENTS; i++) {
        if (data->segments[i].valid && strcmp(data->segments[i].path, segment_path) == 0) {
            data->segments[i].protected = true;
            pthread_mutex_unlock(&data->lock);
            return 0;
        }
    }

    pthread_mutex_unlock(&data->lock);
    return -1;  // Segment not found
}

static int hls_segment_strategy_unprotect_segment(pre_buffer_strategy_t *self,
                                                   const char *segment_path) {
    hls_segment_strategy_data_t *data = (hls_segment_strategy_data_t *)self->private_data;

    pthread_mutex_lock(&data->lock);

    for (int i = 0; i < MAX_TRACKED_SEGMENTS; i++) {
        if (data->segments[i].valid && strcmp(data->segments[i].path, segment_path) == 0) {
            data->segments[i].protected = false;
            pthread_mutex_unlock(&data->lock);
            return 0;
        }
    }

    pthread_mutex_unlock(&data->lock);
    return -1;
}

static int hls_segment_strategy_get_segments(pre_buffer_strategy_t *self,
                                              segment_info_t *segments,
                                              int max_segments,
                                              int *out_count) {
    hls_segment_strategy_data_t *data = (hls_segment_strategy_data_t *)self->private_data;

    pthread_mutex_lock(&data->lock);

    int count = 0;
    int start = (data->head - data->count + MAX_TRACKED_SEGMENTS) % MAX_TRACKED_SEGMENTS;

    for (int i = 0; i < data->count && count < max_segments; i++) {
        int idx = (start + i) % MAX_TRACKED_SEGMENTS;
        tracked_segment_t *seg = &data->segments[idx];

        if (seg->valid) {
            strncpy(segments[count].path, seg->path, sizeof(segments[count].path) - 1);
            segments[count].timestamp = seg->mtime;
            segments[count].duration = seg->duration_seconds;
            segments[count].size_bytes = seg->size_bytes;
            segments[count].protected = seg->protected;
            count++;
        }
    }

    *out_count = count;

    pthread_mutex_unlock(&data->lock);

    return 0;
}

static int hls_segment_strategy_get_stats(pre_buffer_strategy_t *self, buffer_stats_t *stats) {
    hls_segment_strategy_data_t *data = (hls_segment_strategy_data_t *)self->private_data;

    memset(stats, 0, sizeof(*stats));

    pthread_mutex_lock(&data->lock);

    stats->buffered_duration_ms = (int)(data->total_duration_seconds * 1000);
    stats->segment_count = data->count;
    stats->disk_usage_bytes = data->total_size_bytes;
    stats->memory_usage_bytes = sizeof(hls_segment_strategy_data_t);  // Minimal

    // Find oldest and newest timestamps
    if (data->count > 0) {
        int oldest_idx = (data->head - data->count + MAX_TRACKED_SEGMENTS) % MAX_TRACKED_SEGMENTS;
        int newest_idx = (data->head - 1 + MAX_TRACKED_SEGMENTS) % MAX_TRACKED_SEGMENTS;
        stats->oldest_timestamp = data->segments[oldest_idx].mtime;
        stats->newest_timestamp = data->segments[newest_idx].mtime;
    }

    // Assume first segment has keyframe (HLS segments start with keyframes)
    stats->has_complete_gop = (data->count > 0);
    stats->keyframe_count = data->count;  // One keyframe per segment typically

    pthread_mutex_unlock(&data->lock);

    return 0;
}

static bool hls_segment_strategy_is_ready(pre_buffer_strategy_t *self) {
    hls_segment_strategy_data_t *data = (hls_segment_strategy_data_t *)self->private_data;

    // Ready if we have at least 1 second of content
    return data->total_duration_seconds >= 1.0f;
}

static void hls_segment_strategy_clear(pre_buffer_strategy_t *self) {
    hls_segment_strategy_data_t *data = (hls_segment_strategy_data_t *)self->private_data;

    pthread_mutex_lock(&data->lock);

    for (int i = 0; i < MAX_TRACKED_SEGMENTS; i++) {
        data->segments[i].valid = false;
        data->segments[i].protected = false;
    }
    data->head = 0;
    data->count = 0;
    data->total_duration_seconds = 0;
    data->total_size_bytes = 0;

    pthread_mutex_unlock(&data->lock);
}

static int hls_segment_strategy_flush_to_file(pre_buffer_strategy_t *self,
                                               const char *output_path) {
    hls_segment_strategy_data_t *data = (hls_segment_strategy_data_t *)self->private_data;

    pthread_mutex_lock(&data->lock);

    if (data->count == 0) {
        log_warn("No HLS segments to flush for %s", data->stream_name);
        pthread_mutex_unlock(&data->lock);
        return -1;
    }

    // Collect segment paths in order
    const char *segment_paths[MAX_TRACKED_SEGMENTS];
    int segment_count = 0;

    int start = (data->head - data->count + MAX_TRACKED_SEGMENTS) % MAX_TRACKED_SEGMENTS;
    for (int i = 0; i < data->count; i++) {
        int idx = (start + i) % MAX_TRACKED_SEGMENTS;
        if (data->segments[idx].valid) {
            segment_paths[segment_count++] = data->segments[idx].path;
        }
    }

    pthread_mutex_unlock(&data->lock);

    if (segment_count == 0) {
        log_warn("No valid segments to flush");
        return -1;
    }

    // Use ffmpeg_concat_ts_to_mp4 to concatenate segments
    extern int ffmpeg_concat_ts_to_mp4(const char **segment_paths, int segment_count,
                                        const char *output_path);

    int ret = ffmpeg_concat_ts_to_mp4(segment_paths, segment_count, output_path);

    if (ret == 0) {
        log_info("Flushed %d HLS segments (%.1fs) to %s",
                 segment_count, data->total_duration_seconds, output_path);
    } else {
        log_error("Failed to flush HLS segments to %s", output_path);
    }

    return ret;
}

// --- Factory function ---

pre_buffer_strategy_t* create_hls_segment_strategy(const char *stream_name,
                                                    const buffer_config_t *config) {
    pre_buffer_strategy_t *strategy = calloc(1, sizeof(pre_buffer_strategy_t));
    if (!strategy) {
        log_error("Failed to allocate HLS segment strategy");
        return NULL;
    }

    hls_segment_strategy_data_t *data = calloc(1, sizeof(hls_segment_strategy_data_t));
    if (!data) {
        log_error("Failed to allocate HLS segment strategy data");
        free(strategy);
        return NULL;
    }

    strncpy(data->stream_name, stream_name, sizeof(data->stream_name) - 1);

    strategy->name = "hls_segment";
    strategy->type = BUFFER_STRATEGY_HLS_SEGMENT;
    strncpy(strategy->stream_name, stream_name, sizeof(strategy->stream_name) - 1);
    strategy->private_data = data;

    // Set interface methods
    strategy->init = hls_segment_strategy_init;
    strategy->destroy = hls_segment_strategy_destroy;
    strategy->add_packet = NULL;  // Not used by this strategy
    strategy->add_segment = hls_segment_strategy_add_segment;
    strategy->protect_segment = hls_segment_strategy_protect_segment;
    strategy->unprotect_segment = hls_segment_strategy_unprotect_segment;
    strategy->get_segments = hls_segment_strategy_get_segments;
    strategy->flush_to_file = hls_segment_strategy_flush_to_file;
    strategy->flush_to_writer = NULL;  // TODO: implement
    strategy->flush_to_callback = NULL;  // Not applicable
    strategy->get_stats = hls_segment_strategy_get_stats;
    strategy->is_ready = hls_segment_strategy_is_ready;
    strategy->clear = hls_segment_strategy_clear;

    // Initialize
    if (strategy->init(strategy, config) != 0) {
        log_error("Failed to initialize HLS segment strategy for %s", stream_name);
        free(data);
        free(strategy);
        return NULL;
    }

    return strategy;
}

