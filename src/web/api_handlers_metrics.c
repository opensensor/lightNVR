/**
 * @file api_handlers_metrics.c
 * @brief Prometheus metrics endpoint and player telemetry ingest handlers
 *
 * GET /api/metrics  – Prometheus text exposition format (text/plain)
 * POST /api/telemetry/player – client-side QoE event ingestion (returns 204)
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <sys/sysinfo.h>
#include <cjson/cJSON.h>

#include "web/api_handlers_metrics.h"
#include "web/request_response.h"
#include "web/httpd_utils.h"
#include "telemetry/stream_metrics.h"
#include "telemetry/player_telemetry.h"
#include "video/stream_manager.h"
#include "storage/storage_manager.h"
#define LOG_COMPONENT "MetricsAPI"
#include "core/logger.h"
#include "core/config.h"
#include "utils/strings.h"

/* ------------------------------------------------------------------ */
/*  Growable buffer for Prometheus output                               */
/* ------------------------------------------------------------------ */

typedef struct {
    char *data;
    size_t len;
    size_t cap;
} prom_buf_t;

static void prom_buf_init(prom_buf_t *b, size_t initial_cap) {
    b->data = malloc(initial_cap);
    b->len = 0;
    b->cap = initial_cap;
    if (b->data) b->data[0] = '\0';
}

static void prom_buf_append(prom_buf_t *b, const char *fmt, ...) {
    if (!b->data) return;

    va_list ap;
    va_start(ap, fmt);
    int needed = vsnprintf(NULL, 0, fmt, ap);
    va_end(ap);

    if (needed < 0) return;

    /* Grow if needed */
    while (b->len + (size_t)needed + 1 > b->cap) {
        size_t new_cap = b->cap * 2;
        char *new_data = realloc(b->data, new_cap);
        if (!new_data) return;
        b->data = new_data;
        b->cap = new_cap;
    }

    va_start(ap, fmt);
    vsnprintf(b->data + b->len, b->cap - b->len, fmt, ap);
    va_end(ap);
    b->len += (size_t)needed;
}

static void prom_buf_free(prom_buf_t *b) {
    free(b->data);
    b->data = NULL;
    b->len = b->cap = 0;
}

/* ------------------------------------------------------------------ */
/*  Instance-level helpers                                              */
/* ------------------------------------------------------------------ */

static double get_process_cpu_percent(void) {
    static unsigned long prev_utime = 0, prev_stime = 0;
    static double prev_time = 0.0;

    FILE *fp = fopen("/proc/self/stat", "r");
    if (!fp) return 0.0;

    /* Skip to field 14 (utime) and 15 (stime) */
    unsigned long utime = 0, stime = 0;
    /* pid (comm) state ppid pgrp session tty_nr tpgid flags minflt cminflt majflt cmajflt utime stime */
    int matched = fscanf(fp, "%*d %*s %*c %*d %*d %*d %*d %*d %*u %*u %*u %*u %*u %lu %lu",
                         &utime, &stime);
    fclose(fp);
    if (matched != 2) return 0.0;

    struct timespec ts;
    clock_gettime(CLOCK_MONOTONIC, &ts);
    double now = (double)ts.tv_sec + (double)ts.tv_nsec / 1e9;

    double cpu_pct = 0.0;
    if (prev_time > 0.0) {
        double dt = now - prev_time;
        if (dt > 0.0) {
            long ticks_per_sec = sysconf(_SC_CLK_TCK);
            double delta_cpu = (double)((utime + stime) - (prev_utime + prev_stime)) / (double)ticks_per_sec;
            cpu_pct = (delta_cpu / dt) * 100.0;
        }
    }
    prev_utime = utime;
    prev_stime = stime;
    prev_time = now;
    return cpu_pct;
}

static uint64_t get_process_rss_bytes(void) {
    FILE *fp = fopen("/proc/self/status", "r");
    if (!fp) return 0;

    char line[256];
    uint64_t rss_kb = 0;
    while (fgets(line, sizeof(line), fp)) {
        if (strncmp(line, "VmRSS:", 6) == 0) {
            sscanf(line + 6, " %lu", (unsigned long *)&rss_kb);
            break;
        }
    }
    fclose(fp);
    return rss_kb * 1024;
}

static uint64_t get_go2rtc_rss_bytes(void) {
#ifdef USE_GO2RTC
    extern bool get_go2rtc_memory_usage(unsigned long long *memory_usage);
    unsigned long long mem = 0;
    if (get_go2rtc_memory_usage(&mem)) {
        return (uint64_t)mem;
    }
#endif
    return 0;
}

/* ------------------------------------------------------------------ */
/*  GET /api/metrics  (Prometheus text exposition)                      */
/* ------------------------------------------------------------------ */

void handle_get_metrics(const http_request_t *req, http_response_t *res) {
    if (!httpd_authorize_global_action(req, res, AUTHZ_SYSTEM_ADMIN)) {
        return;
    }

    int max = metrics_get_max_streams();
    if (max <= 0) {
        res->status_code = 503;
        safe_strcpy(res->content_type, "text/plain", sizeof(res->content_type), 0);
        http_response_set_body(res, "# Metrics subsystem not initialized\n");
        return;
    }

    stream_metrics_t *snaps = calloc((size_t)max, sizeof(stream_metrics_t));
    if (!snaps) {
        http_response_set_json_error(res, 500, "Out of memory");
        return;
    }
    int count = metrics_snapshot_all(snaps, max);

    prom_buf_t buf;
    prom_buf_init(&buf, 32768);

    /* --- Stream-level QoS metrics --- */
    prom_buf_append(&buf, "# HELP lightnvr_stream_up Whether stream is connected and producing frames\n");
    prom_buf_append(&buf, "# TYPE lightnvr_stream_up gauge\n");
    for (int i = 0; i < count; i++)
        prom_buf_append(&buf, "lightnvr_stream_up{stream=\"%s\"} %d\n", snaps[i].stream_name, snaps[i].stream_up);

    prom_buf_append(&buf, "# HELP lightnvr_stream_fps Current measured frame rate\n");
    prom_buf_append(&buf, "# TYPE lightnvr_stream_fps gauge\n");
    for (int i = 0; i < count; i++)
        prom_buf_append(&buf, "lightnvr_stream_fps{stream=\"%s\"} %.1f\n", snaps[i].stream_name, snaps[i].current_fps);

    prom_buf_append(&buf, "# HELP lightnvr_stream_bitrate_bps Current measured bitrate in bits per second\n");
    prom_buf_append(&buf, "# TYPE lightnvr_stream_bitrate_bps gauge\n");
    for (int i = 0; i < count; i++)
        prom_buf_append(&buf, "lightnvr_stream_bitrate_bps{stream=\"%s\"} %.0f\n", snaps[i].stream_name, snaps[i].current_bitrate_bps);

    prom_buf_append(&buf, "# HELP lightnvr_stream_frames_total Total frames received since stream start\n");
    prom_buf_append(&buf, "# TYPE lightnvr_stream_frames_total counter\n");
    for (int i = 0; i < count; i++)
        prom_buf_append(&buf, "lightnvr_stream_frames_total{stream=\"%s\"} %llu\n", snaps[i].stream_name, (unsigned long long)snaps[i].frames_total);

    prom_buf_append(&buf, "# HELP lightnvr_stream_frames_dropped Frames dropped due to errors\n");
    prom_buf_append(&buf, "# TYPE lightnvr_stream_frames_dropped counter\n");
    for (int i = 0; i < count; i++)
        prom_buf_append(&buf, "lightnvr_stream_frames_dropped{stream=\"%s\"} %llu\n", snaps[i].stream_name, (unsigned long long)snaps[i].frames_dropped);

    prom_buf_append(&buf, "# HELP lightnvr_stream_reconnects_total Number of RTSP reconnection events\n");
    prom_buf_append(&buf, "# TYPE lightnvr_stream_reconnects_total counter\n");
    for (int i = 0; i < count; i++)
        prom_buf_append(&buf, "lightnvr_stream_reconnects_total{stream=\"%s\"} %llu\n", snaps[i].stream_name, (unsigned long long)snaps[i].reconnects_total);

    prom_buf_append(&buf, "# HELP lightnvr_stream_uptime_seconds Cumulative seconds the stream has been up\n");
    prom_buf_append(&buf, "# TYPE lightnvr_stream_uptime_seconds counter\n");
    for (int i = 0; i < count; i++)
        prom_buf_append(&buf, "lightnvr_stream_uptime_seconds{stream=\"%s\"} %llu\n", snaps[i].stream_name, (unsigned long long)snaps[i].uptime_seconds);

    prom_buf_append(&buf, "# HELP lightnvr_stream_last_frame_ts Timestamp of last received frame\n");
    prom_buf_append(&buf, "# TYPE lightnvr_stream_last_frame_ts gauge\n");
    for (int i = 0; i < count; i++)
        prom_buf_append(&buf, "lightnvr_stream_last_frame_ts{stream=\"%s\"} %lld\n", snaps[i].stream_name, (long long)snaps[i].last_frame_ts);

    prom_buf_append(&buf, "# HELP lightnvr_stream_connection_latency_ms Time from RTSP SETUP to first frame\n");
    prom_buf_append(&buf, "# TYPE lightnvr_stream_connection_latency_ms gauge\n");
    for (int i = 0; i < count; i++)
        prom_buf_append(&buf, "lightnvr_stream_connection_latency_ms{stream=\"%s\"} %.1f\n", snaps[i].stream_name, snaps[i].connection_latency_ms);

    prom_buf_append(&buf, "# HELP lightnvr_stream_error_total Total stream errors by type\n");
    prom_buf_append(&buf, "# TYPE lightnvr_stream_error_total counter\n");
    for (int i = 0; i < count; i++) {
        prom_buf_append(&buf, "lightnvr_stream_error_total{stream=\"%s\",type=\"decode\"} %llu\n", snaps[i].stream_name, (unsigned long long)snaps[i].error_decode);
        prom_buf_append(&buf, "lightnvr_stream_error_total{stream=\"%s\",type=\"timeout\"} %llu\n", snaps[i].stream_name, (unsigned long long)snaps[i].error_timeout);
        prom_buf_append(&buf, "lightnvr_stream_error_total{stream=\"%s\",type=\"protocol\"} %llu\n", snaps[i].stream_name, (unsigned long long)snaps[i].error_protocol);
        prom_buf_append(&buf, "lightnvr_stream_error_total{stream=\"%s\",type=\"io\"} %llu\n", snaps[i].stream_name, (unsigned long long)snaps[i].error_io);
    }

    /* --- Recording/Storage metrics --- */
    prom_buf_append(&buf, "# HELP lightnvr_recording_active Whether recording is active for stream\n");
    prom_buf_append(&buf, "# TYPE lightnvr_recording_active gauge\n");
    for (int i = 0; i < count; i++)
        prom_buf_append(&buf, "lightnvr_recording_active{stream=\"%s\"} %d\n", snaps[i].stream_name, snaps[i].recording_active);

    prom_buf_append(&buf, "# HELP lightnvr_recording_bytes_written Total bytes written to storage\n");
    prom_buf_append(&buf, "# TYPE lightnvr_recording_bytes_written counter\n");
    for (int i = 0; i < count; i++)
        prom_buf_append(&buf, "lightnvr_recording_bytes_written{stream=\"%s\"} %llu\n", snaps[i].stream_name, (unsigned long long)snaps[i].recording_bytes_written);

    prom_buf_append(&buf, "# HELP lightnvr_recording_segments_total Total recording segments created\n");
    prom_buf_append(&buf, "# TYPE lightnvr_recording_segments_total counter\n");
    for (int i = 0; i < count; i++)
        prom_buf_append(&buf, "lightnvr_recording_segments_total{stream=\"%s\"} %llu\n", snaps[i].stream_name, (unsigned long long)snaps[i].recording_segments_total);

    prom_buf_append(&buf, "# HELP lightnvr_recording_gaps_total Recording gaps detected\n");
    prom_buf_append(&buf, "# TYPE lightnvr_recording_gaps_total counter\n");
    for (int i = 0; i < count; i++)
        prom_buf_append(&buf, "lightnvr_recording_gaps_total{stream=\"%s\"} %llu\n", snaps[i].stream_name, (unsigned long long)snaps[i].recording_gaps_total);

    /* Storage metrics (instance-level) */
    storage_health_t storage_health;
    get_storage_health(&storage_health);
    prom_buf_append(&buf, "# HELP lightnvr_storage_used_bytes Total storage consumed by recordings\n");
    prom_buf_append(&buf, "# TYPE lightnvr_storage_used_bytes gauge\n");
    prom_buf_append(&buf, "lightnvr_storage_used_bytes %.0f\n", (double)storage_health.used_space_bytes);
    prom_buf_append(&buf, "# HELP lightnvr_storage_available_bytes Available storage on recording volume\n");
    prom_buf_append(&buf, "# TYPE lightnvr_storage_available_bytes gauge\n");
    prom_buf_append(&buf, "lightnvr_storage_available_bytes %.0f\n", (double)storage_health.free_space_bytes);

    /* --- Instance-level metrics --- */
    prom_buf_append(&buf, "# HELP lightnvr_instance_streams_configured Number of streams configured\n");
    prom_buf_append(&buf, "# TYPE lightnvr_instance_streams_configured gauge\n");
    prom_buf_append(&buf, "lightnvr_instance_streams_configured %d\n", get_total_stream_count());

    int streams_up = 0;
    for (int i = 0; i < count; i++) {
        if (snaps[i].stream_up) streams_up++;
    }
    prom_buf_append(&buf, "# HELP lightnvr_instance_streams_up Number of streams currently up\n");
    prom_buf_append(&buf, "# TYPE lightnvr_instance_streams_up gauge\n");
    prom_buf_append(&buf, "lightnvr_instance_streams_up %d\n", streams_up);

    prom_buf_append(&buf, "# HELP lightnvr_instance_cpu_percent Process CPU usage\n");
    prom_buf_append(&buf, "# TYPE lightnvr_instance_cpu_percent gauge\n");
    prom_buf_append(&buf, "lightnvr_instance_cpu_percent %.1f\n", get_process_cpu_percent());

    prom_buf_append(&buf, "# HELP lightnvr_instance_memory_rss_bytes Resident set size of lightnvr process\n");
    prom_buf_append(&buf, "# TYPE lightnvr_instance_memory_rss_bytes gauge\n");
    prom_buf_append(&buf, "lightnvr_instance_memory_rss_bytes %llu\n", (unsigned long long)get_process_rss_bytes());

    prom_buf_append(&buf, "# HELP lightnvr_instance_go2rtc_memory_bytes RSS of go2rtc companion process\n");
    prom_buf_append(&buf, "# TYPE lightnvr_instance_go2rtc_memory_bytes gauge\n");
    prom_buf_append(&buf, "lightnvr_instance_go2rtc_memory_bytes %llu\n", (unsigned long long)get_go2rtc_rss_bytes());

    /* Send response */
    res->status_code = 200;
    safe_strcpy(res->content_type, "text/plain; version=0.0.4; charset=utf-8", sizeof(res->content_type), 0);
    http_response_set_body(res, buf.data ? buf.data : "");

    prom_buf_free(&buf);
    free(snaps);
}

/* ------------------------------------------------------------------ */
/*  POST /api/telemetry/player                                         */
/* ------------------------------------------------------------------ */

void handle_post_player_telemetry(const http_request_t *req, http_response_t *res) {
    if (!req->body || req->body_len == 0) {
        res->status_code = 204;
        return;
    }

    /* Parse JSON body */
    char *body_str = malloc(req->body_len + 1);
    if (!body_str) {
        res->status_code = 204;
        return;
    }
    memcpy(body_str, req->body, req->body_len);
    body_str[req->body_len] = '\0';

    cJSON *json = cJSON_Parse(body_str);
    free(body_str);
    if (!json) {
        res->status_code = 204;
        return;
    }

    player_telemetry_event_t event;
    memset(&event, 0, sizeof(event));

    cJSON *item;
    if ((item = cJSON_GetObjectItem(json, "stream_name")) && cJSON_IsString(item))
        safe_strcpy(event.stream_name, item->valuestring, sizeof(event.stream_name), 0);
    if ((item = cJSON_GetObjectItem(json, "session_id")) && cJSON_IsString(item))
        safe_strcpy(event.session_id, item->valuestring, sizeof(event.session_id), 0);
    if ((item = cJSON_GetObjectItem(json, "transport")) && cJSON_IsString(item))
        safe_strcpy(event.transport, item->valuestring, sizeof(event.transport), 0);
    if ((item = cJSON_GetObjectItem(json, "ttff_ms")) && cJSON_IsNumber(item))
        event.ttff_ms = item->valuedouble;
    if ((item = cJSON_GetObjectItem(json, "rebuffer_count")) && cJSON_IsNumber(item))
        event.rebuffer_count = item->valueint;
    if ((item = cJSON_GetObjectItem(json, "rebuffer_duration_ms")) && cJSON_IsNumber(item))
        event.rebuffer_duration_ms = item->valuedouble;
    if ((item = cJSON_GetObjectItem(json, "resolution_switches")) && cJSON_IsNumber(item))
        event.resolution_switches = item->valueint;
    if ((item = cJSON_GetObjectItem(json, "webrtc_rtt_ms")) && cJSON_IsNumber(item))
        event.webrtc_rtt_ms = item->valuedouble;

    event.timestamp = time(NULL);

    cJSON_Delete(json);

    if (event.stream_name[0] == '\0') {
        res->status_code = 204;
        return;
    }
    if (!httpd_authorize_stream_action(req, res, AUTHZ_LIVE_VIEW,
                                       event.stream_name)) return;

    player_telemetry_record(&event);

    res->status_code = 204;
}
