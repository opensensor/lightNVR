/**
 * @file libuv_api_handlers.c
 * @brief API handler registration for libuv HTTP server
 *
 * This file registers all API handlers with the libuv HTTP server backend.
 * All handlers use the backend-agnostic http_request_t/http_response_t interface.
 */

#include <stdio.h>
#include <string.h>
#include <sys/stat.h>
#include "web/libuv_server.h"
#include "web/libuv_connection.h"
#include "web/http_server.h"
#include "web/api_handlers.h"
#include "web/api_handlers_health.h"
#include "web/api_handlers_settings.h"
#include "web/api_handlers_system.h"
#include "web/api_handlers_zones.h"
#include "web/api_handlers_ptz.h"
#include "web/api_handlers_imaging.h"
#include "web/api_handlers_detection.h"
#include "web/api_handlers_detection_results.h"
#include "web/api_handlers_recordings_playback.h"
#include "web/api_handlers_recordings_thumbnail.h"
#include "web/api_handlers_recordings.h"
#include "web/api_handlers_recordings_batch_download.h"
#include "web/api_handlers_timeline.h"
#include "web/api_handlers_investigations.h"
#include "web/api_handlers_onvif.h"
#include "web/api_handlers_users.h"
#include "web/api_handlers_totp.h"
#include "web/api_handlers_ice_servers.h"
#include "web/api_handlers_go2rtc_proxy.h"
#include "web/api_handlers_setup.h"
#include "web/api_handlers_recording_tags.h"
#include "web/api_handlers_metrics.h"
#include "web/api_handlers_motion.h"
#include "web/api_handlers_recording_control.h"
#include "web/api_handlers_locations.h"
#include "web/api_handlers_camera_tags.h"
#include "web/api_handlers_fleet.h"
#include "web/api_handlers_camera_collections.h"
#include "web/api_handlers_authorization.h"
#include "web/api_handlers_audit.h"
#include "web/api_handlers_event_routes.h"
#include "web/api_handlers_event_destinations.h"
#define LOG_COMPONENT "HTTP"
#include "core/logger.h"
#include "core/config.h"

// Forward declarations
extern int libuv_serve_file(libuv_connection_t *conn, const char *path,
                            const char *content_type, const char *extra_headers);
extern const char *libuv_get_mime_type(const char *path);

/**
 * @brief Register all API handlers with the libuv server
 * 
 * This function should be called after libuv_server_init() but before http_server_start().
 * It registers all API routes with their corresponding handler functions.
 * 
 * @param server The HTTP server handle
 * @return 0 on success, -1 on error
 */
int register_all_libuv_handlers(http_server_handle_t server) {
    if (!server) {
        log_error("register_all_libuv_handlers: Invalid server handle");
        return -1;
    }

    log_info("Registering API handlers with libuv server");

    // Setup wizard API (no authentication guard – must be accessible before auth is configured)
    http_server_register_handler(server, "/api/setup/status", "GET",  handle_get_setup_status);
    http_server_register_handler(server, "/api/setup/status", "POST", handle_post_setup_complete);

    // Health API
    http_server_register_handler(server, "/api/health", "GET", handle_get_health);
    http_server_register_handler(server, "/api/health/hls", "GET", handle_get_hls_health);

    // Metrics & Telemetry API
    http_server_register_handler(server, "/api/metrics", "GET", handle_get_metrics);
    http_server_register_handler(server, "/api/telemetry/player", "POST", handle_post_player_telemetry);

    // Motion API — external trigger for automation (Home Assistant etc.)
    http_server_register_handler(server, "/api/motion/trigger", "POST", handle_post_motion_trigger);

    // Streams API
    http_server_register_handler(server, "/api/streams", "GET", handle_get_streams);
    http_server_register_handler(server, "/api/streams", "POST", handle_post_stream);
    http_server_register_handler(server, "/api/streams/test", "POST", handle_test_stream);

    // Fleet location hierarchy (UUID-based, admin-only until scoped auth lands)
    http_server_register_handler(server, "/api/locations", "GET", handle_get_locations);
    http_server_register_handler(server, "/api/locations", "POST", handle_post_location);
    http_server_register_handler(server, "/api/locations/#", "GET", handle_get_location);
    http_server_register_handler(server, "/api/locations/#", "PUT", handle_put_location);
    http_server_register_handler(server, "/api/locations/#", "DELETE", handle_delete_location);
    http_server_register_handler(server, "/api/cameras/#/location", "PUT",
                                 handle_put_camera_location);

    // Normalized camera tag dictionary and UUID-based assignments
    http_server_register_handler(server, "/api/camera-tags", "GET",
                                 handle_get_camera_tags);
    http_server_register_handler(server, "/api/camera-tags", "POST",
                                 handle_post_camera_tag);
    http_server_register_handler(server, "/api/camera-tags/#/merge", "POST",
                                 handle_post_camera_tag_merge);
    http_server_register_handler(server, "/api/camera-tags/#", "GET",
                                 handle_get_camera_tag);
    http_server_register_handler(server, "/api/camera-tags/#", "PUT",
                                 handle_put_camera_tag);
    http_server_register_handler(server, "/api/camera-tags/#", "DELETE",
                                 handle_delete_camera_tag);
    http_server_register_handler(server, "/api/cameras/#/tags", "GET",
                                 handle_get_camera_tag_assignments);
    http_server_register_handler(server, "/api/cameras/#/tags", "PUT",
                                 handle_put_camera_tag_assignments);

    // Shared selector evaluation and server-side fleet inventory query
    http_server_register_handler(server, "/api/fleet/cameras/query", "POST",
                                 handle_post_fleet_camera_query);
    http_server_register_handler(server, "/api/fleet/selectors/preview", "POST",
                                 handle_post_fleet_selector_preview);

    // Saved static and selector-backed smart camera collections
    http_server_register_handler(server, "/api/camera-collections", "GET",
                                 handle_get_camera_collections);
    http_server_register_handler(server, "/api/camera-collections", "POST",
                                 handle_post_camera_collection);
    http_server_register_handler(server, "/api/camera-collections/#/members", "GET",
                                 handle_get_camera_collection_members);
    http_server_register_handler(server, "/api/camera-collections/#/members", "PUT",
                                 handle_put_camera_collection_members);
    http_server_register_handler(server, "/api/camera-collections/#/preview", "POST",
                                 handle_post_camera_collection_preview);
    http_server_register_handler(server, "/api/camera-collections/#", "GET",
                                 handle_get_camera_collection);
    http_server_register_handler(server, "/api/camera-collections/#", "PUT",
                                 handle_put_camera_collection);
    http_server_register_handler(server, "/api/camera-collections/#", "DELETE",
                                 handle_delete_camera_collection);

    // Stream-specific routes (must come before /api/streams/# wildcard)
    http_server_register_handler(server, "/api/streams/#/recording", "GET",
                                 handle_get_stream_recording);
    http_server_register_handler(server, "/api/streams/#/recording", "POST",
                                 handle_post_stream_recording);

    // Detection Zones API
    http_server_register_handler(server, "/api/streams/#/zones", "GET", handle_get_zones);
    http_server_register_handler(server, "/api/streams/#/zones", "POST", handle_post_zones);
    http_server_register_handler(server, "/api/streams/#/zones", "DELETE", handle_delete_zones);

    // Stream Retention API
    http_server_register_handler(server, "/api/streams/#/retention", "GET", handle_get_stream_retention);
    http_server_register_handler(server, "/api/streams/#/retention", "PUT", handle_put_stream_retention);

    // Stream Refresh API
    http_server_register_handler(server, "/api/streams/#/refresh", "POST", handle_post_stream_refresh);

    // PTZ API
    http_server_register_handler(server, "/api/streams/#/ptz/capabilities", "GET", handle_ptz_capabilities);
    http_server_register_handler(server, "/api/streams/#/ptz/presets", "GET", handle_ptz_get_presets);
    http_server_register_handler(server, "/api/streams/#/ptz/move", "POST", handle_ptz_move);
    http_server_register_handler(server, "/api/streams/#/ptz/stop", "POST", handle_ptz_stop);
    http_server_register_handler(server, "/api/streams/#/ptz/absolute", "POST", handle_ptz_absolute);
    http_server_register_handler(server, "/api/streams/#/ptz/relative", "POST", handle_ptz_relative);
    http_server_register_handler(server, "/api/streams/#/ptz/home", "POST", handle_ptz_home);
    http_server_register_handler(server, "/api/streams/#/ptz/set-home", "POST", handle_ptz_set_home);
    http_server_register_handler(server, "/api/streams/#/ptz/goto-preset", "POST", handle_ptz_goto_preset);
    http_server_register_handler(server, "/api/streams/#/ptz/preset", "PUT", handle_ptz_set_preset);

    // ONVIF Imaging API
    http_server_register_handler(server, "/api/streams/#/imaging/settings", "GET", handle_imaging_get_settings);
    http_server_register_handler(server, "/api/streams/#/imaging/settings", "PUT", handle_imaging_put_settings);
    http_server_register_handler(server, "/api/streams/#/imaging/options", "GET", handle_imaging_get_options);
    http_server_register_handler(server, "/api/streams/#/daynight", "GET", handle_daynight_get);
    http_server_register_handler(server, "/api/streams/#/daynight", "PUT", handle_daynight_put);

    // Stream CRUD (wildcards - must come after specific routes)
    http_server_register_handler(server, "/api/streams/#/full", "GET", handle_get_stream_full);
    http_server_register_handler(server, "/api/streams/#", "GET", handle_get_stream);
    http_server_register_handler(server, "/api/streams/#", "PUT", handle_put_stream);
    http_server_register_handler(server, "/api/streams/#", "DELETE", handle_delete_stream);

    // Settings API
    http_server_register_handler(server, "/api/settings", "GET", handle_get_settings);
    http_server_register_handler(server, "/api/settings", "POST", handle_post_settings);
    http_server_register_handler(server, "/api/settings/go2rtc/validate", "POST",
                                 handle_post_settings_go2rtc_validate);

    // ICE Servers API (WebRTC TURN/STUN configuration)
    http_server_register_handler(server, "/api/ice-servers", "GET", handle_get_ice_servers);

    // System API
    http_server_register_handler(server, "/api/system", "GET", handle_get_system_info);
    http_server_register_handler(server, "/api/system/info", "GET", handle_get_system_info);
    http_server_register_handler(server, "/api/system/logs", "GET", handle_get_system_logs);
    http_server_register_handler(server, "/api/system/restart", "POST", handle_post_system_restart);
    http_server_register_handler(server, "/api/system/shutdown", "POST", handle_post_system_shutdown);
    http_server_register_handler(server, "/api/system/logs/clear", "POST", handle_post_system_logs_clear);
    http_server_register_handler(server, "/api/system/backup", "POST", handle_post_system_backup);
    http_server_register_handler(server, "/api/system/status", "GET", handle_get_system_status);
    http_server_register_handler(server, "/api/system/go2rtc/effective-config", "GET",
                                 handle_get_system_go2rtc_effective_config);
    http_server_register_handler(server, "/api/system/go2rtc/override-status", "GET",
                                 handle_get_system_go2rtc_override_status);

    // Detection API
    http_server_register_handler(server, "/api/detection/results/#", "GET", handle_get_detection_results);
    http_server_register_handler(server, "/api/detection/models", "GET", handle_get_detection_models);
    // Detection event snapshots (saved on MQTT publish, see issue #449)
    http_server_register_handler(server, "/api/snapshots/#/#", "GET", handle_get_detection_snapshot);

    // Storage Management API
    http_server_register_handler(server, "/api/storage/health", "GET", handle_get_storage_health);
    http_server_register_handler(server, "/api/storage/cleanup", "POST", handle_post_storage_cleanup);

    // Auth API (backend-agnostic handlers)
    http_server_register_handler(server, "/api/auth/login/config", "GET", handle_auth_login_config);  // Public, no auth required
    http_server_register_handler(server, "/api/auth/login", "POST", handle_auth_login);
    http_server_register_handler(server, "/api/auth/logout", "POST", handle_auth_logout);
    http_server_register_handler(server, "/api/auth/verify", "GET", handle_auth_verify);
    http_server_register_handler(server, "/api/auth/sessions", "GET", handle_auth_sessions_list);
    http_server_register_handler(server, "/api/auth/sessions/#", "DELETE", handle_auth_sessions_delete);
    http_server_register_handler(server, "/api/auth/trusted-devices", "GET", handle_auth_trusted_devices_list);
    http_server_register_handler(server, "/api/auth/trusted-devices/#", "DELETE", handle_auth_trusted_devices_delete);
    http_server_register_handler(server, "/logout", "GET", handle_auth_logout);  // Simple GET logout route

    // User Management API (backend-agnostic handlers)
    http_server_register_handler(server, "/api/auth/users", "GET", handle_users_list);
    http_server_register_handler(server, "/api/auth/users", "POST", handle_users_create);
    http_server_register_handler(server, "/api/auth/users/#", "GET", handle_users_get);
    http_server_register_handler(server, "/api/auth/users/#", "PUT", handle_users_update);
    http_server_register_handler(server, "/api/auth/users/#", "DELETE", handle_users_delete);
    http_server_register_handler(server, "/api/auth/users/#/api-key", "POST", handle_users_generate_api_key);
    http_server_register_handler(server, "/api/auth/users/#/password", "PUT", handle_users_change_password);
    http_server_register_handler(server, "/api/auth/users/#/password-lock", "PUT", handle_users_password_lock);
    http_server_register_handler(server, "/api/auth/users/#/login-lockout/clear", "POST", handle_users_clear_login_lockout);

    // Action-level authorization catalog and administrator policy simulation
    http_server_register_handler(server, "/api/authorization/actions", "GET",
                                 handle_get_authorization_actions);
    http_server_register_handler(server, "/api/authorization/simulate", "POST",
                                 handle_post_authorization_simulate);
    http_server_register_handler(server, "/api/authorization/roles", "GET",
                                 handle_get_authorization_roles);
    http_server_register_handler(server, "/api/authorization/roles", "POST",
                                 handle_post_authorization_role);
    http_server_register_handler(server, "/api/authorization/roles/#", "PUT",
                                 handle_put_authorization_role);
    http_server_register_handler(server, "/api/authorization/roles/#", "DELETE",
                                 handle_delete_authorization_role);
    http_server_register_handler(server,
                                 "/api/authorization/users/#/tokens", "GET",
                                 handle_get_user_api_tokens);
    http_server_register_handler(server,
                                 "/api/authorization/users/#/tokens", "POST",
                                 handle_post_user_api_token);
    http_server_register_handler(server,
                                 "/api/authorization/users/#/tokens/#", "DELETE",
                                 handle_delete_user_api_token);
    http_server_register_handler(server, "/api/authorization/users/#", "GET",
                                 handle_get_user_authorization);
    http_server_register_handler(server, "/api/authorization/users/#", "PUT",
                                 handle_put_user_authorization);

    // Append-only audit history and retention controls
    http_server_register_handler(server, "/api/audit/events/export", "GET",
                                 handle_get_audit_export);
    http_server_register_handler(server, "/api/audit/events", "GET",
                                 handle_get_audit_events);
    http_server_register_handler(server, "/api/audit/settings", "GET",
                                 handle_get_audit_settings);
    http_server_register_handler(server, "/api/audit/settings", "PUT",
                                 handle_put_audit_settings);

    // Versioned event catalog, destination profiles, and route management
    http_server_register_handler(server, "/api/events/catalog", "GET",
                                 handle_get_event_catalog);
    http_server_register_handler(server, "/api/event-destinations", "GET",
                                 handle_get_event_destinations);
    http_server_register_handler(server, "/api/event-destinations", "POST",
                                 handle_post_event_destination);
    http_server_register_handler(server, "/api/event-destinations/#", "GET",
                                 handle_get_event_destination);
    http_server_register_handler(server, "/api/event-destinations/#", "PUT",
                                 handle_put_event_destination);
    http_server_register_handler(server, "/api/event-destinations/#", "DELETE",
                                 handle_delete_event_destination);
    http_server_register_handler(server, "/api/event-routes/preview", "POST",
                                 handle_post_event_route_preview);
    http_server_register_handler(server, "/api/event-routes", "GET",
                                 handle_get_event_routes);
    http_server_register_handler(server, "/api/event-routes", "POST",
                                 handle_post_event_route);
    http_server_register_handler(server, "/api/event-routes/#", "GET",
                                 handle_get_event_route);
    http_server_register_handler(server, "/api/event-routes/#", "PUT",
                                 handle_put_event_route);
    http_server_register_handler(server, "/api/event-routes/#", "DELETE",
                                 handle_delete_event_route);

    // TOTP MFA API (backend-agnostic handlers)
    http_server_register_handler(server, "/api/auth/users/#/totp/setup", "POST", handle_totp_setup);
    http_server_register_handler(server, "/api/auth/users/#/totp/verify", "POST", handle_totp_verify);
    http_server_register_handler(server, "/api/auth/users/#/totp/disable", "POST", handle_totp_disable);
    http_server_register_handler(server, "/api/auth/users/#/totp/status", "GET", handle_totp_status);
    http_server_register_handler(server, "/api/auth/login/totp", "POST", handle_auth_login_totp);

    // ONVIF API (backend-agnostic handlers)
    http_server_register_handler(server, "/api/onvif/discovery/status", "GET", handle_get_onvif_discovery_status);
    http_server_register_handler(server, "/api/onvif/devices", "GET", handle_get_discovered_onvif_devices);
    http_server_register_handler(server, "/api/onvif/discovery/discover", "POST", handle_post_discover_onvif_devices);
    http_server_register_handler(server, "/api/onvif/device/profiles", "GET", handle_get_onvif_device_profiles);
    http_server_register_handler(server, "/api/onvif/device/add", "POST", handle_post_add_onvif_device_as_stream);
    http_server_register_handler(server, "/api/onvif/device/test", "POST", handle_post_test_onvif_connection);

    // Recordings API (backend-agnostic handlers)
    // Note: More specific routes must come before wildcard routes
    http_server_register_handler(server, "/api/recordings/thumbnail/#/#", "GET", handle_recordings_thumbnail);
    http_server_register_handler(server, "/api/recordings/play/#", "GET", handle_recordings_playback);
    http_server_register_handler(server, "/api/recordings/download/#", "GET", handle_recordings_download);
    http_server_register_handler(server, "/api/recordings/files/check", "GET", handle_check_recording_file);
    http_server_register_handler(server, "/api/recordings/files", "DELETE", handle_delete_recording_file);
    http_server_register_handler(server, "/api/recordings/batch-delete/progress/#", "GET", handle_batch_delete_progress);
    http_server_register_handler(server, "/api/recordings/batch-delete", "POST", handle_batch_delete_recordings);
    http_server_register_handler(server, "/api/recordings/batch-download/status/#", "GET", handle_batch_download_status);
    http_server_register_handler(server, "/api/recordings/batch-download/result/#", "GET", handle_batch_download_result);
    http_server_register_handler(server, "/api/recordings/batch-download", "POST", handle_batch_download_recordings);
    http_server_register_handler(server, "/api/recordings/protected", "GET", handle_get_protected_recordings);
    http_server_register_handler(server, "/api/recordings/batch-protect", "POST", handle_batch_protect_recordings);
    http_server_register_handler(server, "/api/recordings/sync", "POST", handle_post_recordings_sync);
    http_server_register_handler(server, "/api/recordings/tags", "GET", handle_get_recording_tags);
    http_server_register_handler(server, "/api/recordings/detection-labels", "GET", handle_get_recording_detection_labels);
    http_server_register_handler(server, "/api/recordings/batch-tags", "POST", handle_batch_recording_tags);
    http_server_register_handler(server, "/api/recordings/#/tags", "GET", handle_get_recording_tags_by_id);
    http_server_register_handler(server, "/api/recordings/#/tags", "PUT", handle_put_recording_tags);
    http_server_register_handler(server, "/api/recordings/#/protect", "PUT", handle_put_recording_protect);
    http_server_register_handler(server, "/api/recordings/#/retention", "PUT", handle_put_recording_retention);
    http_server_register_handler(server, "/api/recordings/#", "GET", handle_get_recording);
    http_server_register_handler(server, "/api/recordings/#", "DELETE", handle_delete_recording);
    http_server_register_handler(server, "/api/recordings", "GET", handle_get_recordings);

    // Timeline API (backend-agnostic handlers)
    http_server_register_handler(server, "/api/timeline/segments-by-ids", "GET", handle_get_timeline_segments_by_ids);
    http_server_register_handler(server, "/api/timeline/segments", "GET", handle_get_timeline_segments);
    http_server_register_handler(server, "/api/timeline/manifest", "GET", handle_timeline_manifest);
    http_server_register_handler(server, "/api/timeline/play", "GET", handle_timeline_playback);
    http_server_register_handler(server, "/api/investigations/timeline", "POST", handle_post_investigation_timeline);
    http_server_register_handler(server, "/api/investigations/search", "POST", handle_post_investigation_search);

    // HLS Streaming (backend-agnostic handler)
    // Pattern uses # for single-segment wildcards: /hls/{stream_name}/{filename}
    http_server_register_handler(server, "/hls/#/#", "GET", handle_direct_hls_request);

    // go2rtc Streaming Proxy - SCOPED to HLS streaming endpoints only
    // This provides buffered pass-through with concurrency limiting
    // WebRTC connects directly to go2rtc for lower latency
    // Streams list endpoint (for health check)
    http_server_register_handler(server, "/go2rtc/api/streams", "GET", handle_go2rtc_proxy);
    // Stream reload endpoint
    http_server_register_handler(server, "/go2rtc/api/reload", "POST", handle_go2rtc_proxy);
    // HLS manifest endpoint
    http_server_register_handler(server, "/go2rtc/api/stream.m3u8", "GET", handle_go2rtc_proxy);
    // HLS segments (fMP4 and MPEG-TS)
    http_server_register_handler(server, "/go2rtc/api/hls/*", "GET", handle_go2rtc_proxy);
    // Snapshot endpoint
    http_server_register_handler(server, "/go2rtc/api/frame.jpeg", "GET", handle_go2rtc_proxy);

    log_info("Successfully registered API handlers");

    return 0;
}

/**
 * @brief Register static file handler for serving web assets
 *
 * This should be called after registering API handlers to ensure API routes
 * take precedence over static file serving.
 *
 * Note: Static file serving is handled automatically in libuv_connection.c
 * when no handler matches. This function is kept for future extensions.
 *
 * @param server The HTTP server handle
 * @return 0 on success, -1 on error
 */
int register_static_file_handler(http_server_handle_t server) {
    if (!server) {
        log_error("register_static_file_handler: Invalid server handle");
        return -1;
    }

    log_info("Static file serving enabled (handled by default fallback)");

    // Static files are served automatically when no handler matches
    // See libuv_connection.c on_message_complete() for implementation

    return 0;
}
