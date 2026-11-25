/*
 * SPDX-License-Identifier: GPL-3.0-or-later
 * Copyright (C) 2025-2026 OpenSensor Engineering and LightNVR contributors
 *
 * This file is part of LightNVR <https://github.com/opensensor/lightNVR>.
 */

/*
 * Main streams interface file that coordinates between different modules
 */
#define _GNU_SOURCE
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <pthread.h>
#include <sys/stat.h>
#include <sys/types.h>

#include "core/logger.h"
#include "core/config.h"
#include "video/stream_manager.h"
#include "video/streams.h"
#include "video/stream_state.h"
#include "database/database_manager.h"

#ifdef USE_GO2RTC
#include "video/go2rtc/go2rtc_integration.h"
#endif

/**
 * Get the HLS writer for a stream
 * 
 * This function returns the HLS writer for a stream, which can be used
 * to check if the stream is recording or to write packets to the HLS stream.
 * 
 * @param stream The stream handle
 * @return Pointer to the HLS writer, or NULL if not available
 */
hls_writer_t *get_stream_hls_writer(stream_handle_t stream) {
    if (!stream) {
        return NULL;
    }
    
    // Get the stream configuration to get the name
    stream_config_t config;
    if (get_stream_config(stream, &config) != 0) {
        return NULL;
    }
    
    // Get the stream state manager using the name
    stream_state_manager_t *state = get_stream_state_by_name(config.name);
    if (!state) {
        return NULL;
    }
    
    // Get the HLS writer from the stream state
    // The hls_ctx field in the stream_state_manager_t structure contains the HLS writer
    return (hls_writer_t *)state->hls_ctx;
}

static pthread_key_t streaming_snapshot_key;
static pthread_once_t streaming_snapshot_once = PTHREAD_ONCE_INIT;
static int streaming_snapshot_key_result = -1;

static void destroy_streaming_snapshot(void *value) {
    config_t *snapshot = value;
    if (snapshot) {
        free(snapshot->streams);
        free(snapshot);
    }
}

static void init_streaming_snapshot_key(void) {
    streaming_snapshot_key_result =
        pthread_key_create(&streaming_snapshot_key, destroy_streaming_snapshot);
}

/**
 * Get current streaming configuration from database
 * 
 * This function queries the database for all stream configurations
 * and returns a borrowed pointer to a per-thread config_t snapshot.
 *
 * The caller must not free the returned pointer. It remains valid until the
 * next call on that thread or until the thread exits.
 * 
 * @return Borrowed pointer to a per-thread config_t structure with
 *         the latest stream configurations
 */
config_t* get_streaming_config(void) {
    static config_t db_config;
    static pthread_mutex_t config_mutex = PTHREAD_MUTEX_INITIALIZER;

    if (pthread_once(&streaming_snapshot_once, init_streaming_snapshot_key) != 0 ||
        streaming_snapshot_key_result != 0) return &g_config;
    config_t *snapshot = pthread_getspecific(streaming_snapshot_key);
    if (!snapshot) {
        snapshot = calloc(1, sizeof(*snapshot));
        if (!snapshot) return &g_config;
        if (pthread_setspecific(streaming_snapshot_key, snapshot) != 0) {
            free(snapshot);
            return &g_config;
        }
    }
    
    pthread_mutex_lock(&config_mutex);
    
    // Use the global configuration instead of loading defaults
    // This ensures we get the latest configuration including the database path
    if (!config_copy(&db_config, &g_config)) {
        pthread_mutex_unlock(&config_mutex);
        return snapshot->streams ? snapshot : &g_config;
    }

    // Clear stale startup or deleted stream rows while retaining per-slot defaults.
    if (config_reset_stream_slots(&db_config) != 0) {
        pthread_mutex_unlock(&config_mutex);
        return snapshot->streams ? snapshot : &g_config;
    }

    // Load stream configurations from database.
    int ms = db_config.max_streams;
    stream_config_t *db_streams = calloc((size_t)ms, sizeof(*db_streams));
    if (db_streams) {
        int count = get_all_stream_configs(db_streams, ms);
        if (count > 0) {
            for (int i = 0; i < count && i < ms; i++) {
                memcpy(&db_config.streams[i], &db_streams[i], sizeof(*db_streams));
            }
        }
        free(db_streams);
    }
    
    // Log the storage path for debugging
    log_debug("get_streaming_config: Using storage path: %s", db_config.storage_path);
    log_debug("get_streaming_config: Using database path: %s", db_config.db_path);

    if (!config_copy(snapshot, &db_config)) {
        pthread_mutex_unlock(&config_mutex);
        return snapshot->streams ? snapshot : &g_config;
    }
    
    pthread_mutex_unlock(&config_mutex);
    
    return snapshot;
}

/**
 * Check at runtime whether go2rtc should be used for HLS.
 * Requires both compile-time support AND the runtime config flag.
 */
static bool should_use_go2rtc(void) {
#ifdef USE_GO2RTC
    return g_config.go2rtc_enabled;
#else
    return false;
#endif
}

/**
 * go2rtc-aware HLS wrapper — start HLS for a stream.
 * When go2rtc is enabled at runtime, uses go2rtc native HLS.
 * Otherwise falls back to the raw ffmpeg-based start_hls_stream().
 */
int stream_start_hls(const char *stream_name) {
#ifdef USE_GO2RTC
    if (should_use_go2rtc()) {
        return go2rtc_integration_start_hls(stream_name);
    }
#endif
    return start_hls_stream(stream_name);
}

/**
 * go2rtc-aware HLS wrapper — stop HLS for a stream.
 */
int stream_stop_hls(const char *stream_name) {
#ifdef USE_GO2RTC
    if (should_use_go2rtc()) {
        return go2rtc_integration_stop_hls(stream_name);
    }
#endif
    return stop_hls_stream(stream_name);
}

/**
 * go2rtc-aware HLS wrapper — restart HLS for a stream.
 * When go2rtc is enabled at runtime, stops and re-starts via go2rtc integration.
 * Otherwise falls back to the raw ffmpeg restart_hls_stream().
 */
int stream_restart_hls(const char *stream_name) {
#ifdef USE_GO2RTC
    if (should_use_go2rtc()) {
        go2rtc_integration_stop_hls(stream_name);
        return go2rtc_integration_start_hls(stream_name);
    }
#endif
    return restart_hls_stream(stream_name);
}

/**
 * Update to stop_transcode_stream to handle decoupled MP4 recording
 */
int stop_transcode_stream(const char *stream_name) {
    if (!stream_name) {
        return -1;
    }

    int result = 0;

    // Use the go2rtc-aware wrapper to stop HLS
    result = stream_stop_hls(stream_name);
    if (result != 0) {
        log_warn("Failed to stop HLS stream: %s", stream_name);
        // Continue anyway
    }

#ifdef USE_GO2RTC
    if (should_use_go2rtc()) {
        // Stop MP4 recording via go2rtc integration
        if (go2rtc_integration_stop_recording(stream_name) != 0) {
            log_warn("Failed to stop MP4 recording using go2rtc integration: %s", stream_name);
            // Continue anyway
        }
    } else {
        unregister_mp4_writer_for_stream(stream_name);
    }
#else
    // Also stop any separate MP4 recording for this stream
    unregister_mp4_writer_for_stream(stream_name);
#endif

    return result;
}
