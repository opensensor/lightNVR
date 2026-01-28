/**
 * go2rtc Native Buffer Strategy
 * 
 * Leverages go2rtc's internal HLS session buffering.
 * 
 * go2rtc maintains an HLS buffer per session (up to 16MB by default).
 * This strategy:
 * - Creates and maintains an HLS session with go2rtc
 * - On flush, fetches the buffered content directly from go2rtc
 * - Converts/muxes to MP4 for recording
 * 
 * Advantages:
 * - No extra RTSP connections
 * - Minimal memory overhead in our process
 * - Leverages go2rtc's optimized buffering
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <pthread.h>
#include <curl/curl.h>

#include "video/pre_detection_buffer.h"
#include "core/logger.h"
#include "core/config.h"

// go2rtc session state
typedef struct {
    char session_id[64];                // go2rtc session ID
    char go2rtc_url[256];               // Base URL for go2rtc API
    char stream_name[256];              // Stream name
    int buffer_seconds;                 // Target buffer duration
    pthread_mutex_t lock;
    bool session_active;
    time_t session_started;
    time_t last_keepalive;
    
    // Statistics
    int estimated_buffer_ms;
    size_t estimated_buffer_bytes;
} go2rtc_strategy_data_t;

// --- Private helper functions ---

static size_t go2rtc_curl_write_cb(void *contents, size_t size, size_t nmemb, void *userp) {
    size_t total = size * nmemb;
    char **response = (char **)userp;

    // Calculate current length safely (0 if *response is NULL)
    size_t current_len = (*response) ? strlen(*response) : 0;
    size_t new_size = current_len + total + 1;

    // Reallocate buffer to hold existing content + new content + null terminator
    char *new_buf = realloc(*response, new_size);
    if (!new_buf) {
        log_error("Failed to allocate memory for curl response");
        return 0;
    }

    // If this is the first allocation, initialize the buffer
    if (!*response) {
        new_buf[0] = '\0';
    }
    *response = new_buf;

    // Append new content using memcpy (safer than strncat for binary data)
    memcpy(*response + current_len, contents, total);
    (*response)[current_len + total] = '\0';

    return total;
}

/**
 * Initialize go2rtc HLS session
 */
static int go2rtc_init_session(go2rtc_strategy_data_t *data) {
    char url[512];
    snprintf(url, sizeof(url), "%s/api/stream.m3u8?src=%s", 
             data->go2rtc_url, data->stream_name);
    
    CURL *curl = curl_easy_init();
    if (!curl) {
        log_error("Failed to initialize curl for go2rtc session");
        return -1;
    }
    
    char *response = NULL;
    curl_easy_setopt(curl, CURLOPT_URL, url);
    curl_easy_setopt(curl, CURLOPT_WRITEFUNCTION, go2rtc_curl_write_cb);
    curl_easy_setopt(curl, CURLOPT_WRITEDATA, &response);
    curl_easy_setopt(curl, CURLOPT_TIMEOUT, 5L);
    
    CURLcode res = curl_easy_perform(curl);
    long http_code = 0;
    curl_easy_getinfo(curl, CURLINFO_RESPONSE_CODE, &http_code);
    curl_easy_cleanup(curl);
    
    if (res != CURLE_OK || http_code != 200) {
        log_error("Failed to initialize go2rtc HLS session: %s (HTTP %ld)",
                  curl_easy_strerror(res), http_code);
        free(response);
        return -1;
    }
    
    // Parse session ID from response (format: playlist.m3u8?id=XXXXXXXX)
    char *id_start = strstr(response, "id=");
    if (id_start) {
        id_start += 3;
        char *id_end = strchr(id_start, '\n');
        if (!id_end) id_end = strchr(id_start, '\r');
        if (!id_end) id_end = id_start + strlen(id_start);
        
        size_t id_len = id_end - id_start;
        if (id_len > 0 && id_len < sizeof(data->session_id)) {
            strncpy(data->session_id, id_start, id_len);
            data->session_id[id_len] = '\0';
            // Remove any trailing whitespace or query params
            char *amp = strchr(data->session_id, '&');
            if (amp) *amp = '\0';
            char *ws = strchr(data->session_id, ' ');
            if (ws) *ws = '\0';
        }
    }
    
    free(response);
    
    if (data->session_id[0] == '\0') {
        log_error("Failed to parse go2rtc session ID from response");
        return -1;
    }
    
    data->session_active = true;
    data->session_started = time(NULL);
    data->last_keepalive = time(NULL);
    
    log_info("Created go2rtc HLS session for %s: session_id=%s",
             data->stream_name, data->session_id);
    
    return 0;
}

/**
 * Send keepalive to go2rtc session
 */
static int go2rtc_keepalive(go2rtc_strategy_data_t *data) {
    if (!data->session_active || data->session_id[0] == '\0') {
        return -1;
    }
    
    char url[512];
    snprintf(url, sizeof(url), "%s/api/hls/playlist.m3u8?id=%s",
             data->go2rtc_url, data->session_id);
    
    CURL *curl = curl_easy_init();
    if (!curl) return -1;
    
    curl_easy_setopt(curl, CURLOPT_URL, url);
    curl_easy_setopt(curl, CURLOPT_NOBODY, 1L);  // HEAD request
    curl_easy_setopt(curl, CURLOPT_TIMEOUT, 2L);
    
    CURLcode res = curl_easy_perform(curl);
    long http_code = 0;
    curl_easy_getinfo(curl, CURLINFO_RESPONSE_CODE, &http_code);
    curl_easy_cleanup(curl);
    
    if (res == CURLE_OK && http_code == 200) {
        data->last_keepalive = time(NULL);
        return 0;
    }
    
    // Session expired, need to recreate
    log_warn("go2rtc session %s expired, recreating", data->session_id);
    data->session_active = false;
    data->session_id[0] = '\0';
    return go2rtc_init_session(data);
}

// --- Strategy interface methods ---

static int go2rtc_strategy_init(pre_buffer_strategy_t *self, const buffer_config_t *config) {
    go2rtc_strategy_data_t *data = (go2rtc_strategy_data_t *)self->private_data;

    if (config->go2rtc_url && config->go2rtc_url[0]) {
        strncpy(data->go2rtc_url, config->go2rtc_url, sizeof(data->go2rtc_url) - 1);
    } else {
        // Default to localhost
        extern config_t g_config;
        snprintf(data->go2rtc_url, sizeof(data->go2rtc_url),
                 "http://127.0.0.1:%d", g_config.go2rtc_api_port);
    }

    data->buffer_seconds = config->buffer_seconds;
    pthread_mutex_init(&data->lock, NULL);

    // Initialize HLS session
    int ret = go2rtc_init_session(data);
    if (ret == 0) {
        self->initialized = true;
    }

    return ret;
}

static void go2rtc_strategy_destroy(pre_buffer_strategy_t *self) {
    go2rtc_strategy_data_t *data = (go2rtc_strategy_data_t *)self->private_data;

    // Note: go2rtc sessions auto-expire, no explicit cleanup needed
    pthread_mutex_destroy(&data->lock);

    log_debug("go2rtc strategy destroyed for %s", data->stream_name);
    free(data);
    self->private_data = NULL;
}

static int go2rtc_strategy_get_stats(pre_buffer_strategy_t *self, buffer_stats_t *stats) {
    go2rtc_strategy_data_t *data = (go2rtc_strategy_data_t *)self->private_data;

    memset(stats, 0, sizeof(*stats));

    pthread_mutex_lock(&data->lock);

    if (data->session_active) {
        // Estimate based on time since session started
        time_t now = time(NULL);
        int elapsed = (int)(now - data->session_started);
        stats->buffered_duration_ms = (elapsed > data->buffer_seconds ?
                                       data->buffer_seconds : elapsed) * 1000;
        stats->memory_usage_bytes = 0;  // In go2rtc's memory, not ours
        stats->oldest_timestamp = data->session_started;
        stats->newest_timestamp = now;
    }

    pthread_mutex_unlock(&data->lock);

    return 0;
}

static bool go2rtc_strategy_is_ready(pre_buffer_strategy_t *self) {
    go2rtc_strategy_data_t *data = (go2rtc_strategy_data_t *)self->private_data;

    if (!data->session_active) {
        return false;
    }

    // Ready if we have at least 1 second buffered
    time_t now = time(NULL);
    return (now - data->session_started) >= 1;
}

static void go2rtc_strategy_clear(pre_buffer_strategy_t *self) {
    go2rtc_strategy_data_t *data = (go2rtc_strategy_data_t *)self->private_data;

    pthread_mutex_lock(&data->lock);

    // Recreate session to clear buffer
    data->session_active = false;
    data->session_id[0] = '\0';
    go2rtc_init_session(data);

    pthread_mutex_unlock(&data->lock);
}

/**
 * Fetch segment data from go2rtc
 */
typedef struct {
    uint8_t *data;
    size_t size;
    size_t capacity;
} segment_buffer_t;

static size_t segment_write_callback(void *contents, size_t size, size_t nmemb, void *userp) {
    size_t total = size * nmemb;
    segment_buffer_t *buf = (segment_buffer_t *)userp;

    if (buf->size + total > buf->capacity) {
        size_t new_cap = buf->capacity * 2;
        if (new_cap < buf->size + total) {
            new_cap = buf->size + total + 1024 * 1024;  // Add 1MB headroom
        }
        uint8_t *new_data = realloc(buf->data, new_cap);
        if (!new_data) return 0;
        buf->data = new_data;
        buf->capacity = new_cap;
    }

    memcpy(buf->data + buf->size, contents, total);
    buf->size += total;
    return total;
}

static int go2rtc_strategy_flush_to_file(pre_buffer_strategy_t *self, const char *output_path) {
    go2rtc_strategy_data_t *data = (go2rtc_strategy_data_t *)self->private_data;

    if (!data->session_active || data->session_id[0] == '\0') {
        log_error("Cannot flush: go2rtc session not active");
        return -1;
    }

    pthread_mutex_lock(&data->lock);

    // Fetch init segment (fMP4) or current segment (TS)
    char url[512];
    snprintf(url, sizeof(url), "%s/api/hls/segment.ts?id=%s",
             data->go2rtc_url, data->session_id);

    CURL *curl = curl_easy_init();
    if (!curl) {
        pthread_mutex_unlock(&data->lock);
        return -1;
    }

    segment_buffer_t buf = {0};
    buf.capacity = 4 * 1024 * 1024;  // Start with 4MB
    buf.data = malloc(buf.capacity);
    if (!buf.data) {
        curl_easy_cleanup(curl);
        pthread_mutex_unlock(&data->lock);
        return -1;
    }

    curl_easy_setopt(curl, CURLOPT_URL, url);
    curl_easy_setopt(curl, CURLOPT_WRITEFUNCTION, segment_write_callback);
    curl_easy_setopt(curl, CURLOPT_WRITEDATA, &buf);
    curl_easy_setopt(curl, CURLOPT_TIMEOUT, 30L);

    CURLcode res = curl_easy_perform(curl);
    long http_code = 0;
    curl_easy_getinfo(curl, CURLINFO_RESPONSE_CODE, &http_code);
    curl_easy_cleanup(curl);

    pthread_mutex_unlock(&data->lock);

    if (res != CURLE_OK || http_code != 200 || buf.size == 0) {
        log_error("Failed to fetch go2rtc segment: %s (HTTP %ld, size %zu)",
                  curl_easy_strerror(res), http_code, buf.size);
        free(buf.data);
        return -1;
    }

    // Write to output file
    // Note: For TS segments, we may need to convert to MP4
    // For now, write raw TS and let caller convert if needed
    FILE *fp = fopen(output_path, "wb");
    if (!fp) {
        log_error("Failed to create output file: %s", output_path);
        free(buf.data);
        return -1;
    }

    size_t written = fwrite(buf.data, 1, buf.size, fp);
    fclose(fp);
    free(buf.data);

    if (written != buf.size) {
        log_error("Failed to write all data to %s", output_path);
        return -1;
    }

    log_info("Flushed %zu bytes from go2rtc buffer to %s", buf.size, output_path);

    // Recreate session for next detection
    go2rtc_strategy_clear(self);

    return 0;
}

// --- Factory function ---

pre_buffer_strategy_t* create_go2rtc_strategy(const char *stream_name,
                                               const buffer_config_t *config) {
    pre_buffer_strategy_t *strategy = calloc(1, sizeof(pre_buffer_strategy_t));
    if (!strategy) {
        log_error("Failed to allocate go2rtc strategy");
        return NULL;
    }

    go2rtc_strategy_data_t *data = calloc(1, sizeof(go2rtc_strategy_data_t));
    if (!data) {
        log_error("Failed to allocate go2rtc strategy data");
        free(strategy);
        return NULL;
    }

    strncpy(data->stream_name, stream_name, sizeof(data->stream_name) - 1);

    strategy->name = "go2rtc_native";
    strategy->type = BUFFER_STRATEGY_GO2RTC_NATIVE;
    strncpy(strategy->stream_name, stream_name, sizeof(strategy->stream_name) - 1);
    strategy->private_data = data;

    // Set interface methods
    strategy->init = go2rtc_strategy_init;
    strategy->destroy = go2rtc_strategy_destroy;
    strategy->add_packet = NULL;  // Not used - go2rtc handles internally
    strategy->add_segment = NULL; // Not used - go2rtc handles internally
    strategy->flush_to_file = go2rtc_strategy_flush_to_file;
    strategy->flush_to_writer = NULL;  // TODO: implement
    strategy->flush_to_callback = NULL; // TODO: implement
    strategy->get_stats = go2rtc_strategy_get_stats;
    strategy->is_ready = go2rtc_strategy_is_ready;
    strategy->clear = go2rtc_strategy_clear;

    // Initialize
    if (strategy->init(strategy, config) != 0) {
        log_error("Failed to initialize go2rtc strategy for %s", stream_name);
        free(data);
        free(strategy);
        return NULL;
    }

    return strategy;
}

