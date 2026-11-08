unsafe extern "C" {
    unsafe fn http_server_register_handler(
        server: http_server_handle_t,
        path: *const ::core::ffi::c_char,
        method: *const ::core::ffi::c_char,
        handler: request_handler_t,
    ) -> ::core::ffi::c_int;
    unsafe fn handle_get_streams(req: *const http_request_t, res: *mut http_response_t);
    unsafe fn handle_get_stream(req: *const http_request_t, res: *mut http_response_t);
    unsafe fn handle_get_stream_full(req: *const http_request_t, res: *mut http_response_t);
    unsafe fn handle_post_stream(req: *const http_request_t, res: *mut http_response_t);
    unsafe fn handle_put_stream(req: *const http_request_t, res: *mut http_response_t);
    unsafe fn handle_delete_stream(req: *const http_request_t, res: *mut http_response_t);
    unsafe fn handle_post_stream_refresh(req: *const http_request_t, res: *mut http_response_t);
    unsafe fn handle_get_settings(req: *const http_request_t, res: *mut http_response_t);
    unsafe fn handle_get_client_config(req: *const http_request_t, res: *mut http_response_t);
    unsafe fn handle_post_settings(req: *const http_request_t, res: *mut http_response_t);
    unsafe fn handle_get_system_info(req: *const http_request_t, res: *mut http_response_t);
    unsafe fn handle_get_system_health(req: *const http_request_t, res: *mut http_response_t);
    unsafe fn handle_get_system_health_incidents(
        req: *const http_request_t,
        res: *mut http_response_t,
    );
    unsafe fn handle_get_system_logs(req: *const http_request_t, res: *mut http_response_t);
    unsafe fn handle_post_system_restart(req: *const http_request_t, res: *mut http_response_t);
    unsafe fn handle_post_system_shutdown(req: *const http_request_t, res: *mut http_response_t);
    unsafe fn handle_post_recordings_sync(req: *const http_request_t, res: *mut http_response_t);
    unsafe fn handle_get_detection_results(req: *const http_request_t, res: *mut http_response_t);
    unsafe fn handle_get_detection_models(req: *const http_request_t, res: *mut http_response_t);
    unsafe fn handle_direct_hls_request(req: *const http_request_t, res: *mut http_response_t);
    unsafe fn handle_post_system_logs_clear(req: *const http_request_t, res: *mut http_response_t);
    unsafe fn handle_post_system_backup(req: *const http_request_t, res: *mut http_response_t);
    unsafe fn handle_get_system_status(req: *const http_request_t, res: *mut http_response_t);
    unsafe fn handle_test_stream(req: *const http_request_t, res: *mut http_response_t);
    unsafe fn handle_get_stream_retention(req: *const http_request_t, res: *mut http_response_t);
    unsafe fn handle_put_stream_retention(req: *const http_request_t, res: *mut http_response_t);
    unsafe fn handle_put_recording_protect(req: *const http_request_t, res: *mut http_response_t);
    unsafe fn handle_put_recording_retention(req: *const http_request_t, res: *mut http_response_t);
    unsafe fn handle_get_recordings(req: *const http_request_t, res: *mut http_response_t);
    unsafe fn handle_auth_login(req: *const http_request_t, res: *mut http_response_t);
    unsafe fn handle_auth_logout(req: *const http_request_t, res: *mut http_response_t);
    unsafe fn handle_auth_verify(req: *const http_request_t, res: *mut http_response_t);
    unsafe fn handle_auth_login_config(req: *const http_request_t, res: *mut http_response_t);
    unsafe fn handle_auth_sessions_list(req: *const http_request_t, res: *mut http_response_t);
    unsafe fn handle_auth_sessions_delete(req: *const http_request_t, res: *mut http_response_t);
    unsafe fn handle_auth_trusted_devices_list(
        req: *const http_request_t,
        res: *mut http_response_t,
    );
    unsafe fn handle_auth_trusted_devices_delete(
        req: *const http_request_t,
        res: *mut http_response_t,
    );
    unsafe fn handle_get_protected_recordings(
        req: *const http_request_t,
        res: *mut http_response_t,
    );
    unsafe fn handle_batch_protect_recordings(
        req: *const http_request_t,
        res: *mut http_response_t,
    );
    unsafe fn handle_get_storage_health(req: *const http_request_t, res: *mut http_response_t);
    unsafe fn handle_post_storage_cleanup(req: *const http_request_t, res: *mut http_response_t);
    unsafe fn handle_get_openapi(request: *const http_request_t, response: *mut http_response_t);
    unsafe fn handle_get_health(req: *const http_request_t, res: *mut http_response_t);
    unsafe fn handle_get_hls_health(req: *const http_request_t, res: *mut http_response_t);
    unsafe fn handle_post_settings_go2rtc_validate(
        request: *const http_request_t,
        response: *mut http_response_t,
    );
    unsafe fn handle_get_system_go2rtc_effective_config(
        request: *const http_request_t,
        response: *mut http_response_t,
    );
    unsafe fn handle_get_system_go2rtc_override_status(
        request: *const http_request_t,
        response: *mut http_response_t,
    );
    unsafe fn handle_get_zones(req: *const http_request_t, res: *mut http_response_t);
    unsafe fn handle_post_zones(req: *const http_request_t, res: *mut http_response_t);
    unsafe fn handle_delete_zones(req: *const http_request_t, res: *mut http_response_t);
    unsafe fn handle_ptz_move(req: *const http_request_t, res: *mut http_response_t);
    unsafe fn handle_ptz_stop(req: *const http_request_t, res: *mut http_response_t);
    unsafe fn handle_ptz_absolute(req: *const http_request_t, res: *mut http_response_t);
    unsafe fn handle_ptz_relative(req: *const http_request_t, res: *mut http_response_t);
    unsafe fn handle_ptz_home(req: *const http_request_t, res: *mut http_response_t);
    unsafe fn handle_ptz_set_home(req: *const http_request_t, res: *mut http_response_t);
    unsafe fn handle_ptz_get_presets(req: *const http_request_t, res: *mut http_response_t);
    unsafe fn handle_ptz_goto_preset(req: *const http_request_t, res: *mut http_response_t);
    unsafe fn handle_ptz_set_preset(req: *const http_request_t, res: *mut http_response_t);
    unsafe fn handle_ptz_capabilities(req: *const http_request_t, res: *mut http_response_t);
    unsafe fn handle_imaging_get_settings(req: *const http_request_t, res: *mut http_response_t);
    unsafe fn handle_imaging_put_settings(req: *const http_request_t, res: *mut http_response_t);
    unsafe fn handle_imaging_get_options(req: *const http_request_t, res: *mut http_response_t);
    unsafe fn handle_daynight_get(req: *const http_request_t, res: *mut http_response_t);
    unsafe fn handle_daynight_put(req: *const http_request_t, res: *mut http_response_t);
    unsafe fn handle_get_detection_snapshot(
        request: *const http_request_t,
        response: *mut http_response_t,
    );
    unsafe fn handle_recordings_playback(req: *const http_request_t, res: *mut http_response_t);
    unsafe fn handle_recordings_download(req: *const http_request_t, res: *mut http_response_t);
    unsafe fn handle_recordings_thumbnail(req: *const http_request_t, res: *mut http_response_t);
    unsafe fn handle_investigation_thumbnail(req: *const http_request_t, res: *mut http_response_t);
    unsafe fn handle_get_recording(req: *const http_request_t, res: *mut http_response_t);
    unsafe fn handle_delete_recording(req: *const http_request_t, res: *mut http_response_t);
    unsafe fn handle_batch_delete_recordings(req: *const http_request_t, res: *mut http_response_t);
    unsafe fn handle_batch_delete_progress(req: *const http_request_t, res: *mut http_response_t);
    unsafe fn handle_check_recording_file(req: *const http_request_t, res: *mut http_response_t);
    unsafe fn handle_delete_recording_file(req: *const http_request_t, res: *mut http_response_t);
    unsafe fn handle_batch_download_recordings(
        req: *const http_request_t,
        res: *mut http_response_t,
    );
    unsafe fn handle_batch_download_status(req: *const http_request_t, res: *mut http_response_t);
    unsafe fn handle_batch_download_result(req: *const http_request_t, res: *mut http_response_t);
    unsafe fn handle_get_timeline_segments(
        request: *const http_request_t,
        response: *mut http_response_t,
    );
    unsafe fn handle_timeline_playback(
        request: *const http_request_t,
        response: *mut http_response_t,
    );
    unsafe fn handle_timeline_manifest(
        request: *const http_request_t,
        response: *mut http_response_t,
    );
    unsafe fn handle_get_timeline_segments_by_ids(
        request: *const http_request_t,
        response: *mut http_response_t,
    );
    unsafe fn handle_post_investigation_timeline(
        request: *const http_request_t,
        response: *mut http_response_t,
    );
    unsafe fn handle_post_investigation_segment_at(
        request: *const http_request_t,
        response: *mut http_response_t,
    );
    unsafe fn handle_post_investigation_search(
        request: *const http_request_t,
        response: *mut http_response_t,
    );
    unsafe fn handle_post_investigation_recording_preview(
        request: *const http_request_t,
        response: *mut http_response_t,
    );
    unsafe fn handle_post_investigation_thumbnail_samples(
        request: *const http_request_t,
        response: *mut http_response_t,
    );
    unsafe fn handle_get_investigation_bookmarks(
        req: *const http_request_t,
        res: *mut http_response_t,
    );
    unsafe fn handle_post_investigation_bookmark(
        req: *const http_request_t,
        res: *mut http_response_t,
    );
    unsafe fn handle_get_investigation_bookmark(
        req: *const http_request_t,
        res: *mut http_response_t,
    );
    unsafe fn handle_put_investigation_bookmark(
        req: *const http_request_t,
        res: *mut http_response_t,
    );
    unsafe fn handle_delete_investigation_bookmark(
        req: *const http_request_t,
        res: *mut http_response_t,
    );
    unsafe fn handle_get_onvif_discovery_status(
        req: *const http_request_t,
        res: *mut http_response_t,
    );
    unsafe fn handle_get_discovered_onvif_devices(
        req: *const http_request_t,
        res: *mut http_response_t,
    );
    unsafe fn handle_post_discover_onvif_devices(
        req: *const http_request_t,
        res: *mut http_response_t,
    );
    unsafe fn handle_get_onvif_device_profiles(
        req: *const http_request_t,
        res: *mut http_response_t,
    );
    unsafe fn handle_post_add_onvif_device_as_stream(
        req: *const http_request_t,
        res: *mut http_response_t,
    );
    unsafe fn handle_post_claim_onvif_device(req: *const http_request_t, res: *mut http_response_t);
    unsafe fn handle_post_test_onvif_connection(
        req: *const http_request_t,
        res: *mut http_response_t,
    );
    unsafe fn handle_users_list(req: *const http_request_t, res: *mut http_response_t);
    unsafe fn handle_users_get(req: *const http_request_t, res: *mut http_response_t);
    unsafe fn handle_users_create(req: *const http_request_t, res: *mut http_response_t);
    unsafe fn handle_users_update(req: *const http_request_t, res: *mut http_response_t);
    unsafe fn handle_users_delete(req: *const http_request_t, res: *mut http_response_t);
    unsafe fn handle_users_generate_api_key(req: *const http_request_t, res: *mut http_response_t);
    unsafe fn handle_users_change_password(req: *const http_request_t, res: *mut http_response_t);
    unsafe fn handle_users_password_lock(req: *const http_request_t, res: *mut http_response_t);
    unsafe fn handle_users_clear_login_lockout(
        req: *const http_request_t,
        res: *mut http_response_t,
    );
    unsafe fn handle_totp_setup(req: *const http_request_t, res: *mut http_response_t);
    unsafe fn handle_totp_verify(req: *const http_request_t, res: *mut http_response_t);
    unsafe fn handle_totp_disable(req: *const http_request_t, res: *mut http_response_t);
    unsafe fn handle_totp_status(req: *const http_request_t, res: *mut http_response_t);
    unsafe fn handle_auth_login_totp(req: *const http_request_t, res: *mut http_response_t);
    unsafe fn handle_get_ice_servers(req: *const http_request_t, res: *mut http_response_t);
    unsafe fn handle_go2rtc_proxy(req: *const http_request_t, res: *mut http_response_t);
    unsafe fn handle_internal_media_lease(req: *const http_request_t, res: *mut http_response_t);
    unsafe fn handle_get_setup_status(req: *const http_request_t, res: *mut http_response_t);
    unsafe fn handle_post_setup_complete(req: *const http_request_t, res: *mut http_response_t);
    unsafe fn handle_get_metrics(req: *const http_request_t, res: *mut http_response_t);
    unsafe fn handle_post_player_telemetry(req: *const http_request_t, res: *mut http_response_t);
    unsafe fn handle_get_recording_tags(req: *const http_request_t, res: *mut http_response_t);
    unsafe fn handle_get_recording_detection_labels(
        req: *const http_request_t,
        res: *mut http_response_t,
    );
    unsafe fn handle_get_recording_tags_by_id(
        req: *const http_request_t,
        res: *mut http_response_t,
    );
    unsafe fn handle_put_recording_tags(req: *const http_request_t, res: *mut http_response_t);
    unsafe fn handle_batch_recording_tags(req: *const http_request_t, res: *mut http_response_t);
    unsafe fn handle_post_motion_trigger(req: *const http_request_t, res: *mut http_response_t);
    unsafe fn handle_get_stream_recording(req: *const http_request_t, res: *mut http_response_t);
    unsafe fn handle_post_stream_recording(req: *const http_request_t, res: *mut http_response_t);
    unsafe fn handle_get_locations(req: *const http_request_t, res: *mut http_response_t);
    unsafe fn handle_post_location(req: *const http_request_t, res: *mut http_response_t);
    unsafe fn handle_get_location(req: *const http_request_t, res: *mut http_response_t);
    unsafe fn handle_put_location(req: *const http_request_t, res: *mut http_response_t);
    unsafe fn handle_delete_location(req: *const http_request_t, res: *mut http_response_t);
    unsafe fn handle_put_camera_location(req: *const http_request_t, res: *mut http_response_t);
    unsafe fn handle_post_fleet_camera_query(req: *const http_request_t, res: *mut http_response_t);
    unsafe fn handle_post_fleet_selector_preview(
        req: *const http_request_t,
        res: *mut http_response_t,
    );
    unsafe fn handle_get_camera_tags(req: *const http_request_t, res: *mut http_response_t);
    unsafe fn handle_post_camera_tag(req: *const http_request_t, res: *mut http_response_t);
    unsafe fn handle_get_camera_tag(req: *const http_request_t, res: *mut http_response_t);
    unsafe fn handle_put_camera_tag(req: *const http_request_t, res: *mut http_response_t);
    unsafe fn handle_delete_camera_tag(req: *const http_request_t, res: *mut http_response_t);
    unsafe fn handle_post_camera_tag_merge(req: *const http_request_t, res: *mut http_response_t);
    unsafe fn handle_get_camera_tag_assignments(
        req: *const http_request_t,
        res: *mut http_response_t,
    );
    unsafe fn handle_put_camera_tag_assignments(
        req: *const http_request_t,
        res: *mut http_response_t,
    );
    unsafe fn handle_get_fleet_saved_views(req: *const http_request_t, res: *mut http_response_t);
    unsafe fn handle_post_fleet_saved_view(req: *const http_request_t, res: *mut http_response_t);
    unsafe fn handle_get_fleet_saved_view(req: *const http_request_t, res: *mut http_response_t);
    unsafe fn handle_put_fleet_saved_view(req: *const http_request_t, res: *mut http_response_t);
    unsafe fn handle_delete_fleet_saved_view(req: *const http_request_t, res: *mut http_response_t);
    unsafe fn handle_get_camera_collections(req: *const http_request_t, res: *mut http_response_t);
    unsafe fn handle_post_camera_collection(req: *const http_request_t, res: *mut http_response_t);
    unsafe fn handle_get_camera_collection(req: *const http_request_t, res: *mut http_response_t);
    unsafe fn handle_put_camera_collection(req: *const http_request_t, res: *mut http_response_t);
    unsafe fn handle_delete_camera_collection(
        req: *const http_request_t,
        res: *mut http_response_t,
    );
    unsafe fn handle_get_camera_collection_members(
        req: *const http_request_t,
        res: *mut http_response_t,
    );
    unsafe fn handle_put_camera_collection_members(
        req: *const http_request_t,
        res: *mut http_response_t,
    );
    unsafe fn handle_post_camera_collection_preview(
        req: *const http_request_t,
        res: *mut http_response_t,
    );
    unsafe fn handle_get_authorization_actions(
        req: *const http_request_t,
        res: *mut http_response_t,
    );
    unsafe fn handle_post_authorization_simulate(
        req: *const http_request_t,
        res: *mut http_response_t,
    );
    unsafe fn handle_get_authorization_roles(req: *const http_request_t, res: *mut http_response_t);
    unsafe fn handle_post_authorization_role(req: *const http_request_t, res: *mut http_response_t);
    unsafe fn handle_put_authorization_role(req: *const http_request_t, res: *mut http_response_t);
    unsafe fn handle_delete_authorization_role(
        req: *const http_request_t,
        res: *mut http_response_t,
    );
    unsafe fn handle_get_user_authorization(req: *const http_request_t, res: *mut http_response_t);
    unsafe fn handle_put_user_authorization(req: *const http_request_t, res: *mut http_response_t);
    unsafe fn handle_get_user_api_tokens(req: *const http_request_t, res: *mut http_response_t);
    unsafe fn handle_post_user_api_token(req: *const http_request_t, res: *mut http_response_t);
    unsafe fn handle_delete_user_api_token(req: *const http_request_t, res: *mut http_response_t);
    unsafe fn handle_get_audit_events(req: *const http_request_t, res: *mut http_response_t);
    unsafe fn handle_get_audit_export(req: *const http_request_t, res: *mut http_response_t);
    unsafe fn handle_get_audit_settings(req: *const http_request_t, res: *mut http_response_t);
    unsafe fn handle_put_audit_settings(req: *const http_request_t, res: *mut http_response_t);
    unsafe fn handle_get_event_catalog(req: *const http_request_t, res: *mut http_response_t);
    unsafe fn handle_get_event_routes(req: *const http_request_t, res: *mut http_response_t);
    unsafe fn handle_post_event_route(req: *const http_request_t, res: *mut http_response_t);
    unsafe fn handle_post_event_route_preview(
        req: *const http_request_t,
        res: *mut http_response_t,
    );
    unsafe fn handle_get_event_route(req: *const http_request_t, res: *mut http_response_t);
    unsafe fn handle_put_event_route(req: *const http_request_t, res: *mut http_response_t);
    unsafe fn handle_delete_event_route(req: *const http_request_t, res: *mut http_response_t);
    unsafe fn handle_get_event_destinations(req: *const http_request_t, res: *mut http_response_t);
    unsafe fn handle_post_event_destination(req: *const http_request_t, res: *mut http_response_t);
    unsafe fn handle_get_event_destination(req: *const http_request_t, res: *mut http_response_t);
    unsafe fn handle_put_event_destination(req: *const http_request_t, res: *mut http_response_t);
    unsafe fn handle_delete_event_destination(
        req: *const http_request_t,
        res: *mut http_response_t,
    );
    unsafe fn handle_get_storage_targets(req: *const http_request_t, res: *mut http_response_t);
    unsafe fn handle_post_storage_target(req: *const http_request_t, res: *mut http_response_t);
    unsafe fn handle_get_storage_target(req: *const http_request_t, res: *mut http_response_t);
    unsafe fn handle_put_storage_target(req: *const http_request_t, res: *mut http_response_t);
    unsafe fn handle_delete_storage_target(req: *const http_request_t, res: *mut http_response_t);
    unsafe fn handle_post_storage_target_probe(
        req: *const http_request_t,
        res: *mut http_response_t,
    );
    unsafe fn handle_get_storage_migrations(req: *const http_request_t, res: *mut http_response_t);
    unsafe fn handle_post_storage_migration(req: *const http_request_t, res: *mut http_response_t);
    unsafe fn handle_get_storage_migration(req: *const http_request_t, res: *mut http_response_t);
    unsafe fn handle_post_storage_migration_cancel(
        req: *const http_request_t,
        res: *mut http_response_t,
    );
    unsafe fn handle_post_storage_migration_retry(
        req: *const http_request_t,
        res: *mut http_response_t,
    );
    unsafe fn handle_get_storage_archive(_: *const http_request_t, _: *mut http_response_t);
    unsafe fn handle_post_storage_archive_retry(_: *const http_request_t, _: *mut http_response_t);
    unsafe fn handle_get_storage_policies(req: *const http_request_t, res: *mut http_response_t);
    unsafe fn handle_post_storage_policy(req: *const http_request_t, res: *mut http_response_t);
    unsafe fn handle_post_storage_policy_preview(
        req: *const http_request_t,
        res: *mut http_response_t,
    );
    unsafe fn handle_get_storage_policy(req: *const http_request_t, res: *mut http_response_t);
    unsafe fn handle_put_storage_policy(req: *const http_request_t, res: *mut http_response_t);
    unsafe fn handle_delete_storage_policy(req: *const http_request_t, res: *mut http_response_t);
    unsafe fn handle_post_storage_policy_apply(_: *const http_request_t, _: *mut http_response_t);
    unsafe fn handle_get_storage_pools(req: *const http_request_t, res: *mut http_response_t);
    unsafe fn handle_post_storage_pool(req: *const http_request_t, res: *mut http_response_t);
    unsafe fn handle_get_storage_pool(req: *const http_request_t, res: *mut http_response_t);
    unsafe fn handle_put_storage_pool(req: *const http_request_t, res: *mut http_response_t);
    unsafe fn handle_delete_storage_pool(req: *const http_request_t, res: *mut http_response_t);
    unsafe fn handle_get_storage_compliance(req: *const http_request_t, res: *mut http_response_t);
    unsafe fn handle_get_ui_workspaces(
        request: *const http_request_t,
        response: *mut http_response_t,
    );
    unsafe fn handle_put_ui_workspaces(
        request: *const http_request_t,
        response: *mut http_response_t,
    );
    unsafe fn handle_get_live_layouts(req: *const http_request_t, res: *mut http_response_t);
    unsafe fn handle_post_live_layout(req: *const http_request_t, res: *mut http_response_t);
    unsafe fn handle_put_live_layout(req: *const http_request_t, res: *mut http_response_t);
    unsafe fn handle_delete_live_layout(req: *const http_request_t, res: *mut http_response_t);
    unsafe fn handle_get_camera_favorites(req: *const http_request_t, res: *mut http_response_t);
    unsafe fn handle_put_camera_favorite(req: *const http_request_t, res: *mut http_response_t);
    unsafe fn handle_delete_camera_favorite(req: *const http_request_t, res: *mut http_response_t);
    unsafe fn handle_get_operator_floor_plans(
        req: *const http_request_t,
        res: *mut http_response_t,
    );
    unsafe fn handle_post_operator_floor_plan(
        req: *const http_request_t,
        res: *mut http_response_t,
    );
    unsafe fn handle_get_operator_floor_plan(req: *const http_request_t, res: *mut http_response_t);
    unsafe fn handle_put_operator_floor_plan(req: *const http_request_t, res: *mut http_response_t);
    unsafe fn handle_delete_operator_floor_plan(
        req: *const http_request_t,
        res: *mut http_response_t,
    );
    unsafe fn handle_get_operator_floor_plan_background(
        req: *const http_request_t,
        res: *mut http_response_t,
    );
    unsafe fn handle_put_operator_floor_plan_background(
        req: *const http_request_t,
        res: *mut http_response_t,
    );
    unsafe fn handle_delete_operator_floor_plan_background(
        req: *const http_request_t,
        res: *mut http_response_t,
    );
    unsafe fn handle_post_lpr_search(req: *const http_request_t, res: *mut http_response_t);
    unsafe fn handle_post_lpr_export(req: *const http_request_t, res: *mut http_response_t);
    unsafe fn handle_delete_lpr_read(req: *const http_request_t, res: *mut http_response_t);
    unsafe fn handle_get_detection_engines(req: *const http_request_t, res: *mut http_response_t);
    unsafe fn handle_put_detection_engines(req: *const http_request_t, res: *mut http_response_t);
    unsafe fn handle_get_integration_capabilities(
        req: *const http_request_t,
        res: *mut http_response_t,
    );
    unsafe fn handle_get_observations(req: *const http_request_t, res: *mut http_response_t);
    unsafe fn handle_get_observation(req: *const http_request_t, res: *mut http_response_t);
    unsafe fn handle_get_observation_snapshot(
        req: *const http_request_t,
        res: *mut http_response_t,
    );
    unsafe fn handle_put_observation_classification(
        req: *const http_request_t,
        res: *mut http_response_t,
    );
    unsafe fn _log_message_ctx(
        level: log_level_t,
        component: *const ::core::ffi::c_char,
        stream: *const ::core::ffi::c_char,
        format: *const ::core::ffi::c_char,
        ...
    );
}
pub type size_t = usize;
pub type uint64_t = u64;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct http_method_t(pub ::core::ffi::c_uint);
impl http_method_t {
    pub const HTTP_METHOD_GET: Self = Self(0);
    pub const HTTP_METHOD_POST: Self = Self(1);
    pub const HTTP_METHOD_PUT: Self = Self(2);
    pub const HTTP_METHOD_DELETE: Self = Self(3);
    pub const HTTP_METHOD_OPTIONS: Self = Self(4);
    pub const HTTP_METHOD_HEAD: Self = Self(5);
    pub const HTTP_METHOD_PATCH: Self = Self(6);
    pub const HTTP_METHOD_UNKNOWN: Self = Self(7);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct http_header_t {
    pub name: [::core::ffi::c_char; 128],
    pub value: [::core::ffi::c_char; 1024],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct http_request_t {
    pub method: http_method_t,
    pub method_str: [::core::ffi::c_char; 16],
    pub path: [::core::ffi::c_char; 512],
    pub uri: [::core::ffi::c_char; 1536],
    pub query_string: [::core::ffi::c_char; 1024],
    pub content_type: [::core::ffi::c_char; 128],
    pub content_length: uint64_t,
    pub user_agent: [::core::ffi::c_char; 256],
    pub body: *mut ::core::ffi::c_void,
    pub body_len: size_t,
    pub headers: [http_header_t; 50],
    pub num_headers: ::core::ffi::c_int,
    pub client_ip: [::core::ffi::c_char; 64],
    pub request_id: [::core::ffi::c_char; 65],
    pub user_data: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct http_response_t {
    pub status_code: ::core::ffi::c_int,
    pub content_type: [::core::ffi::c_char; 128],
    pub body_length: size_t,
    pub body: *mut ::core::ffi::c_void,
    pub body_allocated: bool,
    pub headers: [http_header_t; 50],
    pub num_headers: ::core::ffi::c_int,
    pub user_data: *mut ::core::ffi::c_void,
}
pub type request_handler_t =
    Option<unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct http_server_config_t {
    pub port: ::core::ffi::c_int,
    pub bind_ip: *const ::core::ffi::c_char,
    pub web_root: *const ::core::ffi::c_char,
    pub auth_enabled: bool,
    pub username: [::core::ffi::c_char; 32],
    pub password: [::core::ffi::c_char; 32],
    pub cors_enabled: bool,
    pub allowed_origins: [::core::ffi::c_char; 256],
    pub allowed_methods: [::core::ffi::c_char; 256],
    pub allowed_headers: [::core::ffi::c_char; 256],
    pub ssl_enabled: bool,
    pub cert_path: [::core::ffi::c_char; 512],
    pub key_path: [::core::ffi::c_char; 512],
    pub max_connections: ::core::ffi::c_int,
    pub connection_timeout: ::core::ffi::c_int,
    pub daemon_mode: bool,
    pub pid_file: [::core::ffi::c_char; 512],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct http_server {
    pub config: http_server_config_t,
    pub running: bool,
    pub handlers: *mut C2Rust_Unnamed,
    pub handler_count: ::core::ffi::c_int,
    pub handler_capacity: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed {
    pub path: [::core::ffi::c_char; 256],
    pub method: [::core::ffi::c_char; 16],
    pub handler: request_handler_t,
}
pub type http_server_t = http_server;
pub type http_server_handle_t = *mut http_server_t;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct log_level_t(pub ::core::ffi::c_uint);
impl log_level_t {
    pub const LOG_LEVEL_ERROR: Self = Self(0);
    pub const LOG_LEVEL_WARN: Self = Self(1);
    pub const LOG_LEVEL_INFO: Self = Self(2);
    pub const LOG_LEVEL_DEBUG: Self = Self(3);
}
pub const _log_stream_name: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
pub const LOG_COMPONENT: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"HTTP\0") };
#[unsafe(no_mangle)]
pub unsafe extern "C" fn register_all_libuv_handlers(
    mut server: http_server_handle_t,
) -> ::core::ffi::c_int {
    if server.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"register_all_libuv_handlers: Invalid server handle\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"Registering API handlers with libuv server\0".as_ptr() as *const ::core::ffi::c_char,
    );
    http_server_register_handler(
        server,
        b"/api/openapi.json\0".as_ptr() as *const ::core::ffi::c_char,
        b"GET\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_get_openapi
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/setup/status\0".as_ptr() as *const ::core::ffi::c_char,
        b"GET\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_get_setup_status
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/setup/status\0".as_ptr() as *const ::core::ffi::c_char,
        b"POST\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_post_setup_complete
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/health\0".as_ptr() as *const ::core::ffi::c_char,
        b"GET\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_get_health
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/health/hls\0".as_ptr() as *const ::core::ffi::c_char,
        b"GET\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_get_hls_health
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/metrics\0".as_ptr() as *const ::core::ffi::c_char,
        b"GET\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_get_metrics
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/telemetry/player\0".as_ptr() as *const ::core::ffi::c_char,
        b"POST\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_post_player_telemetry
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/motion/trigger\0".as_ptr() as *const ::core::ffi::c_char,
        b"POST\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_post_motion_trigger
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/streams\0".as_ptr() as *const ::core::ffi::c_char,
        b"GET\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_get_streams
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/streams\0".as_ptr() as *const ::core::ffi::c_char,
        b"POST\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_post_stream
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/streams/test\0".as_ptr() as *const ::core::ffi::c_char,
        b"POST\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_test_stream
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/locations\0".as_ptr() as *const ::core::ffi::c_char,
        b"GET\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_get_locations
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/locations\0".as_ptr() as *const ::core::ffi::c_char,
        b"POST\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_post_location
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/locations/#\0".as_ptr() as *const ::core::ffi::c_char,
        b"GET\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_get_location
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/locations/#\0".as_ptr() as *const ::core::ffi::c_char,
        b"PUT\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_put_location
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/locations/#\0".as_ptr() as *const ::core::ffi::c_char,
        b"DELETE\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_delete_location
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/cameras/#/location\0".as_ptr() as *const ::core::ffi::c_char,
        b"PUT\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_put_camera_location
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/live/plans\0".as_ptr() as *const ::core::ffi::c_char,
        b"GET\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_get_operator_floor_plans
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/live/plans\0".as_ptr() as *const ::core::ffi::c_char,
        b"POST\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_post_operator_floor_plan
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/live/plans/#/background\0".as_ptr() as *const ::core::ffi::c_char,
        b"GET\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_get_operator_floor_plan_background
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/live/plans/#/background\0".as_ptr() as *const ::core::ffi::c_char,
        b"PUT\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_put_operator_floor_plan_background
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/live/plans/#/background\0".as_ptr() as *const ::core::ffi::c_char,
        b"DELETE\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_delete_operator_floor_plan_background
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/live/plans/#\0".as_ptr() as *const ::core::ffi::c_char,
        b"GET\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_get_operator_floor_plan
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/live/plans/#\0".as_ptr() as *const ::core::ffi::c_char,
        b"PUT\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_put_operator_floor_plan
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/live/plans/#\0".as_ptr() as *const ::core::ffi::c_char,
        b"DELETE\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_delete_operator_floor_plan
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/camera-tags\0".as_ptr() as *const ::core::ffi::c_char,
        b"GET\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_get_camera_tags
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/camera-tags\0".as_ptr() as *const ::core::ffi::c_char,
        b"POST\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_post_camera_tag
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/camera-tags/#/merge\0".as_ptr() as *const ::core::ffi::c_char,
        b"POST\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_post_camera_tag_merge
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/camera-tags/#\0".as_ptr() as *const ::core::ffi::c_char,
        b"GET\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_get_camera_tag
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/camera-tags/#\0".as_ptr() as *const ::core::ffi::c_char,
        b"PUT\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_put_camera_tag
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/camera-tags/#\0".as_ptr() as *const ::core::ffi::c_char,
        b"DELETE\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_delete_camera_tag
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/cameras/#/tags\0".as_ptr() as *const ::core::ffi::c_char,
        b"GET\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_get_camera_tag_assignments
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/cameras/#/tags\0".as_ptr() as *const ::core::ffi::c_char,
        b"PUT\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_put_camera_tag_assignments
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/fleet/cameras/query\0".as_ptr() as *const ::core::ffi::c_char,
        b"POST\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_post_fleet_camera_query
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/fleet/selectors/preview\0".as_ptr() as *const ::core::ffi::c_char,
        b"POST\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_post_fleet_selector_preview
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/fleet/views\0".as_ptr() as *const ::core::ffi::c_char,
        b"GET\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_get_fleet_saved_views
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/fleet/views\0".as_ptr() as *const ::core::ffi::c_char,
        b"POST\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_post_fleet_saved_view
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/fleet/views/#\0".as_ptr() as *const ::core::ffi::c_char,
        b"GET\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_get_fleet_saved_view
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/fleet/views/#\0".as_ptr() as *const ::core::ffi::c_char,
        b"PUT\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_put_fleet_saved_view
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/fleet/views/#\0".as_ptr() as *const ::core::ffi::c_char,
        b"DELETE\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_delete_fleet_saved_view
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/live/layouts\0".as_ptr() as *const ::core::ffi::c_char,
        b"GET\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_get_live_layouts
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/live/layouts\0".as_ptr() as *const ::core::ffi::c_char,
        b"POST\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_post_live_layout
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/live/layouts/#\0".as_ptr() as *const ::core::ffi::c_char,
        b"PUT\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_put_live_layout
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/live/layouts/#\0".as_ptr() as *const ::core::ffi::c_char,
        b"DELETE\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_delete_live_layout
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/camera-favorites\0".as_ptr() as *const ::core::ffi::c_char,
        b"GET\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_get_camera_favorites
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/camera-favorites/#\0".as_ptr() as *const ::core::ffi::c_char,
        b"PUT\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_put_camera_favorite
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/camera-favorites/#\0".as_ptr() as *const ::core::ffi::c_char,
        b"DELETE\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_delete_camera_favorite
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/camera-collections\0".as_ptr() as *const ::core::ffi::c_char,
        b"GET\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_get_camera_collections
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/camera-collections\0".as_ptr() as *const ::core::ffi::c_char,
        b"POST\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_post_camera_collection
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/camera-collections/#/members\0".as_ptr() as *const ::core::ffi::c_char,
        b"GET\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_get_camera_collection_members
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/camera-collections/#/members\0".as_ptr() as *const ::core::ffi::c_char,
        b"PUT\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_put_camera_collection_members
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/camera-collections/#/preview\0".as_ptr() as *const ::core::ffi::c_char,
        b"POST\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_post_camera_collection_preview
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/camera-collections/#\0".as_ptr() as *const ::core::ffi::c_char,
        b"GET\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_get_camera_collection
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/camera-collections/#\0".as_ptr() as *const ::core::ffi::c_char,
        b"PUT\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_put_camera_collection
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/camera-collections/#\0".as_ptr() as *const ::core::ffi::c_char,
        b"DELETE\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_delete_camera_collection
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/streams/#/recording\0".as_ptr() as *const ::core::ffi::c_char,
        b"GET\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_get_stream_recording
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/streams/#/recording\0".as_ptr() as *const ::core::ffi::c_char,
        b"POST\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_post_stream_recording
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/streams/#/zones\0".as_ptr() as *const ::core::ffi::c_char,
        b"GET\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_get_zones
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/streams/#/zones\0".as_ptr() as *const ::core::ffi::c_char,
        b"POST\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_post_zones
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/streams/#/zones\0".as_ptr() as *const ::core::ffi::c_char,
        b"DELETE\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_delete_zones
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/streams/#/retention\0".as_ptr() as *const ::core::ffi::c_char,
        b"GET\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_get_stream_retention
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/streams/#/retention\0".as_ptr() as *const ::core::ffi::c_char,
        b"PUT\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_put_stream_retention
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/streams/#/detection-engines\0".as_ptr() as *const ::core::ffi::c_char,
        b"GET\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_get_detection_engines
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/streams/#/detection-engines\0".as_ptr() as *const ::core::ffi::c_char,
        b"PUT\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_put_detection_engines
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/streams/#/refresh\0".as_ptr() as *const ::core::ffi::c_char,
        b"POST\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_post_stream_refresh
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/streams/#/ptz/capabilities\0".as_ptr() as *const ::core::ffi::c_char,
        b"GET\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_ptz_capabilities
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/streams/#/ptz/presets\0".as_ptr() as *const ::core::ffi::c_char,
        b"GET\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_ptz_get_presets
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/streams/#/ptz/move\0".as_ptr() as *const ::core::ffi::c_char,
        b"POST\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_ptz_move
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/streams/#/ptz/stop\0".as_ptr() as *const ::core::ffi::c_char,
        b"POST\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_ptz_stop
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/streams/#/ptz/absolute\0".as_ptr() as *const ::core::ffi::c_char,
        b"POST\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_ptz_absolute
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/streams/#/ptz/relative\0".as_ptr() as *const ::core::ffi::c_char,
        b"POST\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_ptz_relative
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/streams/#/ptz/home\0".as_ptr() as *const ::core::ffi::c_char,
        b"POST\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_ptz_home
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/streams/#/ptz/set-home\0".as_ptr() as *const ::core::ffi::c_char,
        b"POST\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_ptz_set_home
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/streams/#/ptz/goto-preset\0".as_ptr() as *const ::core::ffi::c_char,
        b"POST\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_ptz_goto_preset
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/streams/#/ptz/preset\0".as_ptr() as *const ::core::ffi::c_char,
        b"PUT\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_ptz_set_preset
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/streams/#/imaging/settings\0".as_ptr() as *const ::core::ffi::c_char,
        b"GET\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_imaging_get_settings
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/streams/#/imaging/settings\0".as_ptr() as *const ::core::ffi::c_char,
        b"PUT\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_imaging_put_settings
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/streams/#/imaging/options\0".as_ptr() as *const ::core::ffi::c_char,
        b"GET\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_imaging_get_options
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/streams/#/daynight\0".as_ptr() as *const ::core::ffi::c_char,
        b"GET\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_daynight_get
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/streams/#/daynight\0".as_ptr() as *const ::core::ffi::c_char,
        b"PUT\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_daynight_put
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/streams/#/full\0".as_ptr() as *const ::core::ffi::c_char,
        b"GET\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_get_stream_full
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/streams/#\0".as_ptr() as *const ::core::ffi::c_char,
        b"GET\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_get_stream
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/streams/#\0".as_ptr() as *const ::core::ffi::c_char,
        b"PUT\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_put_stream
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/streams/#\0".as_ptr() as *const ::core::ffi::c_char,
        b"DELETE\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_delete_stream
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/settings\0".as_ptr() as *const ::core::ffi::c_char,
        b"GET\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_get_settings
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/client-config\0".as_ptr() as *const ::core::ffi::c_char,
        b"GET\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_get_client_config
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/settings\0".as_ptr() as *const ::core::ffi::c_char,
        b"POST\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_post_settings
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/settings/go2rtc/validate\0".as_ptr() as *const ::core::ffi::c_char,
        b"POST\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_post_settings_go2rtc_validate
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/ui/workspaces\0".as_ptr() as *const ::core::ffi::c_char,
        b"GET\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_get_ui_workspaces
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/ui/workspaces\0".as_ptr() as *const ::core::ffi::c_char,
        b"PUT\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_put_ui_workspaces
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/ice-servers\0".as_ptr() as *const ::core::ffi::c_char,
        b"GET\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_get_ice_servers
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/system\0".as_ptr() as *const ::core::ffi::c_char,
        b"GET\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_get_system_info
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/system/info\0".as_ptr() as *const ::core::ffi::c_char,
        b"GET\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_get_system_info
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/system/health/incidents\0".as_ptr() as *const ::core::ffi::c_char,
        b"GET\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_get_system_health_incidents
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/system/health\0".as_ptr() as *const ::core::ffi::c_char,
        b"GET\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_get_system_health
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/system/logs\0".as_ptr() as *const ::core::ffi::c_char,
        b"GET\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_get_system_logs
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/system/restart\0".as_ptr() as *const ::core::ffi::c_char,
        b"POST\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_post_system_restart
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/system/shutdown\0".as_ptr() as *const ::core::ffi::c_char,
        b"POST\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_post_system_shutdown
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/system/logs/clear\0".as_ptr() as *const ::core::ffi::c_char,
        b"POST\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_post_system_logs_clear
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/system/backup\0".as_ptr() as *const ::core::ffi::c_char,
        b"POST\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_post_system_backup
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/system/status\0".as_ptr() as *const ::core::ffi::c_char,
        b"GET\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_get_system_status
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/system/go2rtc/effective-config\0".as_ptr() as *const ::core::ffi::c_char,
        b"GET\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_get_system_go2rtc_effective_config
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/system/go2rtc/override-status\0".as_ptr() as *const ::core::ffi::c_char,
        b"GET\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_get_system_go2rtc_override_status
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/detection/results/#\0".as_ptr() as *const ::core::ffi::c_char,
        b"GET\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_get_detection_results
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/detection/models\0".as_ptr() as *const ::core::ffi::c_char,
        b"GET\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_get_detection_models
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/lpr/search\0".as_ptr() as *const ::core::ffi::c_char,
        b"POST\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_post_lpr_search
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/lpr/export\0".as_ptr() as *const ::core::ffi::c_char,
        b"POST\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_post_lpr_export
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/lpr/reads/#\0".as_ptr() as *const ::core::ffi::c_char,
        b"DELETE\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_delete_lpr_read
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/snapshots/#/#\0".as_ptr() as *const ::core::ffi::c_char,
        b"GET\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_get_detection_snapshot
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/integrations/capabilities\0".as_ptr() as *const ::core::ffi::c_char,
        b"GET\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_get_integration_capabilities
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/observations\0".as_ptr() as *const ::core::ffi::c_char,
        b"GET\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_get_observations
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/observations/#/snapshot\0".as_ptr() as *const ::core::ffi::c_char,
        b"GET\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_get_observation_snapshot
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/observations/#/classifications/#/#\0".as_ptr() as *const ::core::ffi::c_char,
        b"PUT\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_put_observation_classification
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/observations/#\0".as_ptr() as *const ::core::ffi::c_char,
        b"GET\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_get_observation
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/storage/health\0".as_ptr() as *const ::core::ffi::c_char,
        b"GET\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_get_storage_health
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/storage/cleanup\0".as_ptr() as *const ::core::ffi::c_char,
        b"POST\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_post_storage_cleanup
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/auth/login/config\0".as_ptr() as *const ::core::ffi::c_char,
        b"GET\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_auth_login_config
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/auth/login\0".as_ptr() as *const ::core::ffi::c_char,
        b"POST\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_auth_login
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/auth/logout\0".as_ptr() as *const ::core::ffi::c_char,
        b"POST\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_auth_logout
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/auth/verify\0".as_ptr() as *const ::core::ffi::c_char,
        b"GET\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_auth_verify
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/auth/sessions\0".as_ptr() as *const ::core::ffi::c_char,
        b"GET\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_auth_sessions_list
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/auth/sessions/#\0".as_ptr() as *const ::core::ffi::c_char,
        b"DELETE\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_auth_sessions_delete
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/auth/trusted-devices\0".as_ptr() as *const ::core::ffi::c_char,
        b"GET\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_auth_trusted_devices_list
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/auth/trusted-devices/#\0".as_ptr() as *const ::core::ffi::c_char,
        b"DELETE\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_auth_trusted_devices_delete
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/logout\0".as_ptr() as *const ::core::ffi::c_char,
        b"GET\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_auth_logout
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/auth/users\0".as_ptr() as *const ::core::ffi::c_char,
        b"GET\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_users_list
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/auth/users\0".as_ptr() as *const ::core::ffi::c_char,
        b"POST\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_users_create
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/auth/users/#\0".as_ptr() as *const ::core::ffi::c_char,
        b"GET\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_users_get
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/auth/users/#\0".as_ptr() as *const ::core::ffi::c_char,
        b"PUT\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_users_update
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/auth/users/#\0".as_ptr() as *const ::core::ffi::c_char,
        b"DELETE\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_users_delete
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/auth/users/#/api-key\0".as_ptr() as *const ::core::ffi::c_char,
        b"POST\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_users_generate_api_key
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/auth/users/#/password\0".as_ptr() as *const ::core::ffi::c_char,
        b"PUT\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_users_change_password
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/auth/users/#/password-lock\0".as_ptr() as *const ::core::ffi::c_char,
        b"PUT\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_users_password_lock
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/auth/users/#/login-lockout/clear\0".as_ptr() as *const ::core::ffi::c_char,
        b"POST\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_users_clear_login_lockout
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/authorization/actions\0".as_ptr() as *const ::core::ffi::c_char,
        b"GET\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_get_authorization_actions
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/authorization/simulate\0".as_ptr() as *const ::core::ffi::c_char,
        b"POST\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_post_authorization_simulate
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/authorization/roles\0".as_ptr() as *const ::core::ffi::c_char,
        b"GET\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_get_authorization_roles
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/authorization/roles\0".as_ptr() as *const ::core::ffi::c_char,
        b"POST\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_post_authorization_role
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/authorization/roles/#\0".as_ptr() as *const ::core::ffi::c_char,
        b"PUT\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_put_authorization_role
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/authorization/roles/#\0".as_ptr() as *const ::core::ffi::c_char,
        b"DELETE\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_delete_authorization_role
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/authorization/users/#/tokens\0".as_ptr() as *const ::core::ffi::c_char,
        b"GET\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_get_user_api_tokens
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/authorization/users/#/tokens\0".as_ptr() as *const ::core::ffi::c_char,
        b"POST\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_post_user_api_token
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/authorization/users/#/tokens/#\0".as_ptr() as *const ::core::ffi::c_char,
        b"DELETE\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_delete_user_api_token
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/authorization/users/#\0".as_ptr() as *const ::core::ffi::c_char,
        b"GET\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_get_user_authorization
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/authorization/users/#\0".as_ptr() as *const ::core::ffi::c_char,
        b"PUT\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_put_user_authorization
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/audit/events/export\0".as_ptr() as *const ::core::ffi::c_char,
        b"GET\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_get_audit_export
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/audit/events\0".as_ptr() as *const ::core::ffi::c_char,
        b"GET\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_get_audit_events
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/audit/settings\0".as_ptr() as *const ::core::ffi::c_char,
        b"GET\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_get_audit_settings
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/audit/settings\0".as_ptr() as *const ::core::ffi::c_char,
        b"PUT\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_put_audit_settings
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/events/catalog\0".as_ptr() as *const ::core::ffi::c_char,
        b"GET\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_get_event_catalog
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/event-destinations\0".as_ptr() as *const ::core::ffi::c_char,
        b"GET\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_get_event_destinations
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/event-destinations\0".as_ptr() as *const ::core::ffi::c_char,
        b"POST\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_post_event_destination
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/event-destinations/#\0".as_ptr() as *const ::core::ffi::c_char,
        b"GET\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_get_event_destination
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/event-destinations/#\0".as_ptr() as *const ::core::ffi::c_char,
        b"PUT\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_put_event_destination
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/event-destinations/#\0".as_ptr() as *const ::core::ffi::c_char,
        b"DELETE\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_delete_event_destination
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/storage-archive\0".as_ptr() as *const ::core::ffi::c_char,
        b"GET\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_get_storage_archive
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/storage-archive/retry\0".as_ptr() as *const ::core::ffi::c_char,
        b"POST\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_post_storage_archive_retry
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/storage-policies/#/apply\0".as_ptr() as *const ::core::ffi::c_char,
        b"POST\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_post_storage_policy_apply
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/storage-targets\0".as_ptr() as *const ::core::ffi::c_char,
        b"GET\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_get_storage_targets
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/storage-targets\0".as_ptr() as *const ::core::ffi::c_char,
        b"POST\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_post_storage_target
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/storage-targets/#/probe\0".as_ptr() as *const ::core::ffi::c_char,
        b"POST\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_post_storage_target_probe
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/storage-targets/#\0".as_ptr() as *const ::core::ffi::c_char,
        b"GET\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_get_storage_target
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/storage-targets/#\0".as_ptr() as *const ::core::ffi::c_char,
        b"PUT\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_put_storage_target
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/storage-targets/#\0".as_ptr() as *const ::core::ffi::c_char,
        b"DELETE\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_delete_storage_target
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/storage-migrations\0".as_ptr() as *const ::core::ffi::c_char,
        b"GET\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_get_storage_migrations
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/storage-migrations\0".as_ptr() as *const ::core::ffi::c_char,
        b"POST\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_post_storage_migration
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/storage-migrations/#/cancel\0".as_ptr() as *const ::core::ffi::c_char,
        b"POST\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_post_storage_migration_cancel
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/storage-migrations/#/retry\0".as_ptr() as *const ::core::ffi::c_char,
        b"POST\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_post_storage_migration_retry
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/storage-migrations/#\0".as_ptr() as *const ::core::ffi::c_char,
        b"GET\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_get_storage_migration
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/storage-pools\0".as_ptr() as *const ::core::ffi::c_char,
        b"GET\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_get_storage_pools
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/storage-pools\0".as_ptr() as *const ::core::ffi::c_char,
        b"POST\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_post_storage_pool
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/storage-pools/#\0".as_ptr() as *const ::core::ffi::c_char,
        b"GET\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_get_storage_pool
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/storage-pools/#\0".as_ptr() as *const ::core::ffi::c_char,
        b"PUT\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_put_storage_pool
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/storage-pools/#\0".as_ptr() as *const ::core::ffi::c_char,
        b"DELETE\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_delete_storage_pool
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/storage-compliance\0".as_ptr() as *const ::core::ffi::c_char,
        b"GET\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_get_storage_compliance
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/storage-policies\0".as_ptr() as *const ::core::ffi::c_char,
        b"GET\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_get_storage_policies
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/storage-policies\0".as_ptr() as *const ::core::ffi::c_char,
        b"POST\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_post_storage_policy
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/storage-policies/preview\0".as_ptr() as *const ::core::ffi::c_char,
        b"POST\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_post_storage_policy_preview
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/storage-policies/#\0".as_ptr() as *const ::core::ffi::c_char,
        b"GET\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_get_storage_policy
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/storage-policies/#\0".as_ptr() as *const ::core::ffi::c_char,
        b"PUT\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_put_storage_policy
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/storage-policies/#\0".as_ptr() as *const ::core::ffi::c_char,
        b"DELETE\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_delete_storage_policy
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/event-routes/preview\0".as_ptr() as *const ::core::ffi::c_char,
        b"POST\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_post_event_route_preview
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/event-routes\0".as_ptr() as *const ::core::ffi::c_char,
        b"GET\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_get_event_routes
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/event-routes\0".as_ptr() as *const ::core::ffi::c_char,
        b"POST\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_post_event_route
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/event-routes/#\0".as_ptr() as *const ::core::ffi::c_char,
        b"GET\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_get_event_route
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/event-routes/#\0".as_ptr() as *const ::core::ffi::c_char,
        b"PUT\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_put_event_route
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/event-routes/#\0".as_ptr() as *const ::core::ffi::c_char,
        b"DELETE\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_delete_event_route
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/auth/users/#/totp/setup\0".as_ptr() as *const ::core::ffi::c_char,
        b"POST\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_totp_setup
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/auth/users/#/totp/verify\0".as_ptr() as *const ::core::ffi::c_char,
        b"POST\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_totp_verify
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/auth/users/#/totp/disable\0".as_ptr() as *const ::core::ffi::c_char,
        b"POST\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_totp_disable
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/auth/users/#/totp/status\0".as_ptr() as *const ::core::ffi::c_char,
        b"GET\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_totp_status
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/auth/login/totp\0".as_ptr() as *const ::core::ffi::c_char,
        b"POST\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_auth_login_totp
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/onvif/discovery/status\0".as_ptr() as *const ::core::ffi::c_char,
        b"GET\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_get_onvif_discovery_status
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/onvif/devices\0".as_ptr() as *const ::core::ffi::c_char,
        b"GET\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_get_discovered_onvif_devices
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/onvif/discovery/discover\0".as_ptr() as *const ::core::ffi::c_char,
        b"POST\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_post_discover_onvif_devices
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/onvif/device/profiles\0".as_ptr() as *const ::core::ffi::c_char,
        b"GET\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_get_onvif_device_profiles
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/onvif/device/add\0".as_ptr() as *const ::core::ffi::c_char,
        b"POST\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_post_add_onvif_device_as_stream
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/onvif/device/claim\0".as_ptr() as *const ::core::ffi::c_char,
        b"POST\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_post_claim_onvif_device
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/onvif/device/test\0".as_ptr() as *const ::core::ffi::c_char,
        b"POST\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_post_test_onvif_connection
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/recordings/thumbnail/#/#\0".as_ptr() as *const ::core::ffi::c_char,
        b"GET\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_recordings_thumbnail
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/recordings/play/#\0".as_ptr() as *const ::core::ffi::c_char,
        b"GET\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_recordings_playback
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/recordings/download/#\0".as_ptr() as *const ::core::ffi::c_char,
        b"GET\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_recordings_download
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/recordings/files/check\0".as_ptr() as *const ::core::ffi::c_char,
        b"GET\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_check_recording_file
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/recordings/files\0".as_ptr() as *const ::core::ffi::c_char,
        b"DELETE\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_delete_recording_file
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/recordings/batch-delete/progress/#\0".as_ptr() as *const ::core::ffi::c_char,
        b"GET\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_batch_delete_progress
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/recordings/batch-delete\0".as_ptr() as *const ::core::ffi::c_char,
        b"POST\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_batch_delete_recordings
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/recordings/batch-download/status/#\0".as_ptr() as *const ::core::ffi::c_char,
        b"GET\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_batch_download_status
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/recordings/batch-download/result/#\0".as_ptr() as *const ::core::ffi::c_char,
        b"GET\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_batch_download_result
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/recordings/batch-download\0".as_ptr() as *const ::core::ffi::c_char,
        b"POST\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_batch_download_recordings
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/recordings/protected\0".as_ptr() as *const ::core::ffi::c_char,
        b"GET\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_get_protected_recordings
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/recordings/batch-protect\0".as_ptr() as *const ::core::ffi::c_char,
        b"POST\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_batch_protect_recordings
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/recordings/sync\0".as_ptr() as *const ::core::ffi::c_char,
        b"POST\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_post_recordings_sync
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/recordings/tags\0".as_ptr() as *const ::core::ffi::c_char,
        b"GET\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_get_recording_tags
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/recordings/detection-labels\0".as_ptr() as *const ::core::ffi::c_char,
        b"GET\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_get_recording_detection_labels
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/recordings/batch-tags\0".as_ptr() as *const ::core::ffi::c_char,
        b"POST\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_batch_recording_tags
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/recordings/#/tags\0".as_ptr() as *const ::core::ffi::c_char,
        b"GET\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_get_recording_tags_by_id
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/recordings/#/tags\0".as_ptr() as *const ::core::ffi::c_char,
        b"PUT\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_put_recording_tags
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/recordings/#/protect\0".as_ptr() as *const ::core::ffi::c_char,
        b"PUT\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_put_recording_protect
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/recordings/#/retention\0".as_ptr() as *const ::core::ffi::c_char,
        b"PUT\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_put_recording_retention
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/recordings/#\0".as_ptr() as *const ::core::ffi::c_char,
        b"GET\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_get_recording
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/recordings/#\0".as_ptr() as *const ::core::ffi::c_char,
        b"DELETE\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_delete_recording
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/recordings\0".as_ptr() as *const ::core::ffi::c_char,
        b"GET\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_get_recordings
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/timeline/segments-by-ids\0".as_ptr() as *const ::core::ffi::c_char,
        b"GET\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_get_timeline_segments_by_ids
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/timeline/segments\0".as_ptr() as *const ::core::ffi::c_char,
        b"GET\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_get_timeline_segments
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/timeline/manifest\0".as_ptr() as *const ::core::ffi::c_char,
        b"GET\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_timeline_manifest
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/timeline/play\0".as_ptr() as *const ::core::ffi::c_char,
        b"GET\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_timeline_playback
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/investigations/timeline\0".as_ptr() as *const ::core::ffi::c_char,
        b"POST\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_post_investigation_timeline
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/investigations/segment-at\0".as_ptr() as *const ::core::ffi::c_char,
        b"POST\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_post_investigation_segment_at
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/investigations/search\0".as_ptr() as *const ::core::ffi::c_char,
        b"POST\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_post_investigation_search
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/investigations/recordings/preview\0".as_ptr() as *const ::core::ffi::c_char,
        b"POST\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_post_investigation_recording_preview
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/investigations/thumbnail-samples\0".as_ptr() as *const ::core::ffi::c_char,
        b"POST\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_post_investigation_thumbnail_samples
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/investigations/thumbnail/#/#\0".as_ptr() as *const ::core::ffi::c_char,
        b"GET\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_investigation_thumbnail
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/investigation-bookmarks/#\0".as_ptr() as *const ::core::ffi::c_char,
        b"GET\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_get_investigation_bookmark
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/investigation-bookmarks/#\0".as_ptr() as *const ::core::ffi::c_char,
        b"PUT\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_put_investigation_bookmark
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/investigation-bookmarks/#\0".as_ptr() as *const ::core::ffi::c_char,
        b"DELETE\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_delete_investigation_bookmark
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/investigation-bookmarks\0".as_ptr() as *const ::core::ffi::c_char,
        b"GET\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_get_investigation_bookmarks
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/investigation-bookmarks\0".as_ptr() as *const ::core::ffi::c_char,
        b"POST\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_post_investigation_bookmark
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/hls/#/#\0".as_ptr() as *const ::core::ffi::c_char,
        b"GET\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_direct_hls_request
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/api/internal/media-lease\0".as_ptr() as *const ::core::ffi::c_char,
        b"GET\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_internal_media_lease
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/go2rtc/api/ws\0".as_ptr() as *const ::core::ffi::c_char,
        b"GET\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_go2rtc_proxy
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/go2rtc/api/webrtc\0".as_ptr() as *const ::core::ffi::c_char,
        b"POST\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_go2rtc_proxy
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/go2rtc/api/streams\0".as_ptr() as *const ::core::ffi::c_char,
        b"GET\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_go2rtc_proxy
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/go2rtc/api/reload\0".as_ptr() as *const ::core::ffi::c_char,
        b"POST\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_go2rtc_proxy
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/go2rtc/api/stream.m3u8\0".as_ptr() as *const ::core::ffi::c_char,
        b"GET\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_go2rtc_proxy
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/go2rtc/api/hls/*\0".as_ptr() as *const ::core::ffi::c_char,
        b"GET\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_go2rtc_proxy
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    http_server_register_handler(
        server,
        b"/go2rtc/api/frame.jpeg\0".as_ptr() as *const ::core::ffi::c_char,
        b"GET\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_go2rtc_proxy
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ),
    );
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"Successfully registered API handlers\0".as_ptr() as *const ::core::ffi::c_char,
    );
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn register_static_file_handler(
    mut server: http_server_handle_t,
) -> ::core::ffi::c_int {
    if server.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"register_static_file_handler: Invalid server handle\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"Static file serving enabled (handled by default fallback)\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    return 0 as ::core::ffi::c_int;
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
