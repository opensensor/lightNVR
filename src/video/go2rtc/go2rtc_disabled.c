/**
 * @file go2rtc_disabled.c
 * @brief Disabled-mode stubs for go2rtc public APIs
 *
 * When ENABLE_GO2RTC=OFF we still compile sources that reference the public
 * go2rtc headers. These stubs preserve native HLS/recording fallbacks and make
 * the disabled build link cleanly.
 */

#include "video/go2rtc/go2rtc_integration.h"
#include "video/go2rtc/go2rtc_process.h"
#include "video/go2rtc/go2rtc_snapshot.h"
#include "video/go2rtc/go2rtc_stream.h"
#include "video/hls/hls_api.h"
#include "video/mp4_recording.h"

#define UNUSED(x) (void)(x)

bool go2rtc_process_init(const char *binary_path, const char *config_dir, int api_port) {
    UNUSED(binary_path); UNUSED(config_dir); UNUSED(api_port); return false;
}
bool go2rtc_process_start(int api_port) { UNUSED(api_port); return false; }
bool go2rtc_process_stop(void) { return true; }
bool go2rtc_process_is_running(void) { return false; }
bool go2rtc_process_generate_config(const char *config_path, int api_port) {
    UNUSED(config_path); UNUSED(api_port); return false;
}
bool go2rtc_process_generate_startup_config(const char *binary_path, const char *config_dir, int api_port) {
    UNUSED(binary_path); UNUSED(config_dir); UNUSED(api_port); return false;
}
void go2rtc_process_cleanup(void) {}
int go2rtc_process_get_rtsp_port(void) { return 0; }
int go2rtc_process_get_pid(void) { return -1; }

bool go2rtc_stream_init(const char *binary_path, const char *config_dir, int api_port) {
    UNUSED(binary_path); UNUSED(config_dir); UNUSED(api_port); return false;
}
bool go2rtc_stream_register(const char *stream_id, const char *stream_url,
                            const char *username, const char *password,
                            bool backchannel_enabled, stream_protocol_t protocol,
                            bool record_audio) {
    UNUSED(stream_id); UNUSED(stream_url); UNUSED(username); UNUSED(password);
    UNUSED(backchannel_enabled); UNUSED(protocol); UNUSED(record_audio); return true;
}
bool go2rtc_stream_unregister(const char *stream_id) { UNUSED(stream_id); return true; }
bool go2rtc_stream_get_webrtc_url(const char *stream_id, char *buffer, size_t buffer_size) {
    UNUSED(stream_id); UNUSED(buffer); UNUSED(buffer_size); return false;
}
bool go2rtc_stream_get_rtsp_url(const char *stream_id, char *buffer, size_t buffer_size) {
    UNUSED(stream_id); UNUSED(buffer); UNUSED(buffer_size); return false;
}
bool go2rtc_stream_is_initialized(void) { return false; }
bool go2rtc_stream_is_ready(void) { return false; }
int go2rtc_stream_get_api_port(void) { return 0; }
bool go2rtc_stream_start_service(void) { return false; }
bool go2rtc_stream_stop_service(void) { return true; }
void go2rtc_stream_invalidate_ready_cache(void) {}
void go2rtc_stream_cleanup(void) {}

bool go2rtc_get_snapshot(const char *stream_name, unsigned char **jpeg_data, size_t *jpeg_size) {
    UNUSED(stream_name); UNUSED(jpeg_data); UNUSED(jpeg_size); return false;
}
void go2rtc_snapshot_cleanup_thread(void) {}

bool go2rtc_integration_init(void) { return false; }
int go2rtc_integration_start_recording(const char *stream_name) { return start_mp4_recording(stream_name); }
int go2rtc_integration_stop_recording(const char *stream_name) { return stop_mp4_recording(stream_name); }
int go2rtc_integration_start_hls(const char *stream_name) { return start_hls_stream(stream_name); }
int go2rtc_integration_stop_hls(const char *stream_name) { return stop_hls_stream(stream_name); }
bool go2rtc_integration_is_using_go2rtc_for_recording(const char *stream_name) { UNUSED(stream_name); return false; }
bool go2rtc_integration_is_using_go2rtc_for_hls(const char *stream_name) { UNUSED(stream_name); return false; }
bool go2rtc_integration_register_all_streams(void) { return true; }
bool go2rtc_sync_streams_from_database(void) { return true; }
bool go2rtc_integration_full_start(void) { return false; }
void go2rtc_integration_cleanup(void) {}
bool go2rtc_integration_is_initialized(void) { return false; }
bool go2rtc_get_rtsp_url(const char *stream_name, char *url, size_t url_size) {
    UNUSED(stream_name); UNUSED(url); UNUSED(url_size); return false;
}
bool go2rtc_integration_get_hls_url(const char *stream_name, char *url, size_t url_size) {
    UNUSED(stream_name); UNUSED(url); UNUSED(url_size); return false;
}
bool go2rtc_integration_reload_stream_config(const char *stream_name,
                                             const char *new_url,
                                             const char *new_username,
                                             const char *new_password,
                                             int new_backchannel_enabled,
                                             int new_protocol,
                                             int new_record_audio) {
    UNUSED(stream_name); UNUSED(new_url); UNUSED(new_username); UNUSED(new_password);
    UNUSED(new_backchannel_enabled); UNUSED(new_protocol); UNUSED(new_record_audio); return true;
}
bool go2rtc_integration_reload_stream(const char *stream_name) { UNUSED(stream_name); return true; }
bool go2rtc_integration_unregister_stream(const char *stream_name) { UNUSED(stream_name); return true; }
bool go2rtc_integration_register_stream(const char *stream_name) { UNUSED(stream_name); return true; }
bool go2rtc_integration_monitor_is_running(void) { return false; }
int go2rtc_integration_get_restart_count(void) { return 0; }
time_t go2rtc_integration_get_last_restart_time(void) { return 0; }
bool go2rtc_integration_check_health(void) { return false; }