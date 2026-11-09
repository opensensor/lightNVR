# tests/unit/CMakeLists.txt
# Unity-based unit and integration tests
# Layer 1 (test_storage_pressure)      — pure logic, zero heavy deps
# Layer 2 (test_storage_retention_sqlite) — real SQLite via lightnvr_lib

# ---- Unity static library ----
add_library(unity_lib STATIC
    ${CMAKE_SOURCE_DIR}/third_party/unity/unity.c
)
target_include_directories(unity_lib PUBLIC
    ${CMAKE_SOURCE_DIR}/third_party/unity
)

# ====================================================================
# CMake helper macros — reduce boilerplate for test registration
# ====================================================================
macro(add_layer1_test TEST_NAME)
    add_executable(${TEST_NAME} ${CMAKE_CURRENT_SOURCE_DIR}/${TEST_NAME}.c)
    target_include_directories(${TEST_NAME} PRIVATE ${CMAKE_SOURCE_DIR}/include)
    target_link_libraries(${TEST_NAME} unity_lib)
    set_target_properties(${TEST_NAME} PROPERTIES RUNTIME_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/bin")
    add_test(NAME ${TEST_NAME} COMMAND ${TEST_NAME})
    lightnvr_ci_test(${TEST_NAME} ${ARGN})
endmacro()

macro(add_layer2_test TEST_NAME)
    add_executable(${TEST_NAME} ${CMAKE_CURRENT_SOURCE_DIR}/${TEST_NAME}.c)
    target_include_directories(${TEST_NAME} PRIVATE ${CMAKE_SOURCE_DIR}/include)
    target_link_libraries(${TEST_NAME}
        unity_lib
        lightnvr_lib
        ${SQLITE_LIBRARIES}
        ${SSL_LIBRARIES}
        ${HTTP_BACKEND_LIBS}
        inih_lib
        pthread
        dl
    )
    if(CJSON_BUNDLED)
        target_link_libraries(${TEST_NAME} cjson_lib)
    elseif(CJSON_FOUND)
        target_link_libraries(${TEST_NAME} ${CJSON_LIBRARIES})
    endif()
    if(ENABLE_MQTT AND MOSQUITTO_FOUND)
        target_link_libraries(${TEST_NAME} ${MOSQUITTO_LIBRARIES})
    endif()
    set_target_properties(${TEST_NAME} PROPERTIES RUNTIME_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/bin")
    if("${TEST_NAME}" STREQUAL "test_storage_archive")
        find_package(Python3 COMPONENTS Interpreter REQUIRED)
        add_test(NAME test_storage_archive_s3 COMMAND ${Python3_EXECUTABLE}
            ${CMAKE_CURRENT_SOURCE_DIR}/s3_fixture.py $<TARGET_FILE:test_storage_archive>)
        add_test(NAME test_storage_archive_operator COMMAND ${Python3_EXECUTABLE}
            ${CMAKE_CURRENT_SOURCE_DIR}/test_archive_operator.py)
        lightnvr_ci_test(${TEST_NAME} ${ARGN}
            TESTS test_storage_archive_s3 test_storage_archive_operator)
    elseif("${TEST_NAME}" STREQUAL "test_media_authority")
        find_package(Python3 COMPONENTS Interpreter REQUIRED)
        add_test(NAME ${TEST_NAME} COMMAND ${Python3_EXECUTABLE}
            ${CMAKE_CURRENT_SOURCE_DIR}/test_media_authority_fixture.py $<TARGET_FILE:${TEST_NAME}>)
        lightnvr_ci_test(${TEST_NAME} ${ARGN})
    else()
        add_test(NAME ${TEST_NAME} COMMAND ${TEST_NAME})
        lightnvr_ci_test(${TEST_NAME} ${ARGN})
    endif()
endmacro()

# Filesystem-only regression coverage also builds the standalone helper logic.
add_layer1_test(test_hls_cache_cleanup)
target_sources(test_hls_cache_cleanup PRIVATE
    ${CMAKE_SOURCE_DIR}/src/video/hls/hls_cache_cleanup.c)

# Variant for Layer 2 tests that require CURL
macro(add_layer2_test_with_curl TEST_NAME)
    add_layer2_test(${TEST_NAME} ${ARGN})
    target_link_libraries(${TEST_NAME} ${CURL_LIBRARIES})
endmacro()

# Convenience wrappers for Layer 2 tests with common external deps.

macro(add_layer2_test_with_ffmpeg TEST_NAME)
    add_layer2_test(${TEST_NAME} ${ARGN})
    target_link_libraries(${TEST_NAME} ${FFMPEG_LIBRARIES})
endmacro()

macro(add_layer3_test TEST_NAME)
    add_executable(${TEST_NAME} ${CMAKE_CURRENT_SOURCE_DIR}/${TEST_NAME}.c)
    target_include_directories(${TEST_NAME} PRIVATE
        ${CMAKE_SOURCE_DIR}/include
        ${FFMPEG_INCLUDE_DIRS}
    )
    target_link_libraries(${TEST_NAME}
        unity_lib
        lightnvr_lib
        ${FFMPEG_LIBRARIES}
        ${SQLITE_LIBRARIES}
        ${CURL_LIBRARIES}
        ${SSL_LIBRARIES}
        ${HTTP_BACKEND_LIBS}
        inih_lib
        pthread
        dl
        m
    )
    if(CJSON_BUNDLED)
        target_link_libraries(${TEST_NAME} cjson_lib)
    elseif(CJSON_FOUND)
        target_link_libraries(${TEST_NAME} ${CJSON_LIBRARIES})
    endif()
    if(ENABLE_MQTT AND MOSQUITTO_FOUND)
        target_link_libraries(${TEST_NAME} ${MOSQUITTO_LIBRARIES})
    endif()
    set_target_properties(${TEST_NAME} PROPERTIES RUNTIME_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/bin")
    add_test(NAME ${TEST_NAME} COMMAND ${TEST_NAME})
    lightnvr_ci_test(${TEST_NAME} ${ARGN})
endmacro()

# ====================================================================
# Layer 1: test_storage_pressure
# Pure unit test — only needs unity_lib + project headers.
# disk_pressure_level_str() and evaluate_disk_pressure_level() are
# both static inline in storage_manager.h, so no .c files are linked.
# ====================================================================
add_layer1_test(test_storage_pressure CI_UNIT CI_SANITIZER)

# ====================================================================
# Layer 2: test_storage_retention_sqlite
# Integration test — runs real SQL queries via db_core / db_recordings.
# ====================================================================
add_layer2_test(test_storage_retention_sqlite CI_UNIT CI_SANITIZER)

# ====================================================================
# Layer 1: Pure logic tests — unity_lib + project headers only
# (Only tests whose functions are static inline in headers)
# ====================================================================
add_layer1_test(test_storage_pressure_extended CI_UNIT CI_SANITIZER)
add_layer1_test(test_signal_safe CI_UNIT CI_SANITIZER)
target_link_options(test_signal_safe PRIVATE "-Wl,--wrap=write")
add_layer1_test(test_detection_result_structures CI_UNIT CI_SANITIZER)
add_layer1_test(test_stream_frame_rate CI_UNIT CI_SANITIZER)
target_sources(test_stream_frame_rate PRIVATE
    ${CMAKE_SOURCE_DIR}/src/video/stream_frame_rate.c)

# ====================================================================
# Layer 2: lightnvr_lib tests — compiled functions + optional SQLite
# ====================================================================
add_layer2_test(test_config CI_UNIT CI_SANITIZER)
target_link_options(test_config PRIVATE -Wl,--wrap=fchmod -Wl,--wrap=renameat)
add_layer2_test(test_db_core_initialization CI_UNIT CI_SANITIZER)
add_layer2_test(test_db_backup_publication CI_UNIT CI_SANITIZER)
target_link_options(test_db_backup_publication PRIVATE -Wl,--wrap=rename -Wl,--wrap=fsync -Wl,--wrap=write)
set_tests_properties(test_db_backup_publication PROPERTIES TIMEOUT 30)
add_layer2_test(test_sqlite_migration_boundaries CI_UNIT CI_SANITIZER)
set_tests_properties(test_sqlite_migration_boundaries PROPERTIES TIMEOUT 20)
add_layer2_test(test_observation_migration CI_UNIT CI_SANITIZER)
set_tests_properties(test_observation_migration PROPERTIES TIMEOUT 30)
add_layer2_test(test_observation_codec CI_UNIT CI_SANITIZER)
add_layer2_test(test_db_observations CI_UNIT CI_SANITIZER)
set_tests_properties(test_db_observations PROPERTIES TIMEOUT 60)
add_layer2_test(test_db_observation_classifications CI_UNIT CI_SANITIZER)
set_tests_properties(test_db_observation_classifications PROPERTIES TIMEOUT 60)
add_layer2_test_with_ffmpeg(test_observation_evidence CI_UNIT CI_SANITIZER)
set_tests_properties(test_observation_evidence PROPERTIES TIMEOUT 120)
add_layer2_test(test_observation_relay CI_UNIT CI_SANITIZER)
target_compile_definitions(test_observation_relay PRIVATE LIGHTNVR_SOURCE_DIR="${CMAKE_SOURCE_DIR}")
set_tests_properties(test_observation_relay PROPERTIES TIMEOUT 60)
# INT_01 L11 / A6: SIGKILL at each observation persistence boundary, then restart.
add_layer2_test(test_observation_crash_boundaries CI_UNIT CI_SANITIZER)
set_tests_properties(test_observation_crash_boundaries PROPERTIES TIMEOUT 180)
add_layer2_test(test_observation_retention CI_UNIT CI_SANITIZER)
target_link_options(test_observation_retention PRIVATE
    "-Wl,--wrap=unlink" "-Wl,--wrap=db_observations_prune")
set_tests_properties(test_observation_retention PROPERTIES TIMEOUT 60)
add_layer2_test_with_ffmpeg(test_observation_diagnostics CI_UNIT CI_SANITIZER)
set_tests_properties(test_observation_diagnostics PROPERTIES TIMEOUT 60)
add_layer2_test(test_logger CI_UNIT CI_SANITIZER)
add_layer2_test(test_strings CI_UNIT CI_SANITIZER)
add_layer2_test(test_interruptible_sleep)
add_layer2_test(test_stream_metrics)
add_layer2_test(test_memory CI_UNIT CI_SANITIZER)
add_layer2_test(test_path_utils)
add_layer2_test(test_request_id_generation CI_UNIT CI_SANITIZER)
target_link_options(test_request_id_generation PRIVATE -Wl,--wrap=getrandom -Wl,--wrap=clock_gettime)
add_layer2_test(test_request_response CI_UNIT CI_SANITIZER)
target_link_options(test_request_response PRIVATE -Wl,--wrap=malloc)
add_layer2_test(test_shutdown_coordinator CI_UNIT CI_SANITIZER)
target_link_options(test_shutdown_coordinator PRIVATE -Wl,--wrap=pthread_cond_timedwait)
set_tests_properties(test_shutdown_coordinator PROPERTIES TIMEOUT 15)
add_layer2_test(test_go2rtc_lifecycle)
add_layer2_test(test_detection_config CI_UNIT CI_SANITIZER)
add_layer2_test_with_curl(test_detection_model_motion CI_UNIT CI_SANITIZER)
add_layer2_test_with_curl(test_detection_system_onvif)
add_layer3_test(test_onvif_poll_recovery)
add_layer3_test(test_health_thread_lifecycle CI_UNIT CI_SANITIZER)
set_tests_properties(test_health_thread_lifecycle PROPERTIES TIMEOUT 20)
target_link_options(test_health_thread_lifecycle PRIVATE
    "-Wl,--wrap=pthread_create" "-Wl,--wrap=pthread_join" "-Wl,--wrap=usleep"
    "-Wl,--wrap=curl_easy_perform" "-Wl,--wrap=is_shutdown_initiated"
    "-Wl,--wrap=http_server_start" "-Wl,--wrap=http_server_stop"
    "-Wl,--wrap=httpd_secure_cookies" "-Wl,--wrap=_log_message_ctx")
add_layer3_test(test_health_probe_transport CI_UNIT CI_SANITIZER)
target_link_options(test_health_probe_transport PRIVATE "-Wl,--wrap=httpd_secure_cookies"
    "-Wl,--wrap=malloc" "-Wl,--wrap=curl_easy_init" "-Wl,--wrap=_log_message_ctx")
add_layer3_test(test_go2rtc_override_status CI_UNIT CI_SANITIZER)
set_tests_properties(test_go2rtc_override_status PROPERTIES TIMEOUT 15)
target_link_options(test_go2rtc_override_status PRIVATE
    "-Wl,--wrap=fstat" "-Wl,--wrap=malloc" "-Wl,--wrap=read" "-Wl,--wrap=__read_chk" "-Wl,--wrap=close"
    "-Wl,--wrap=cJSON_AddNumberToObject" "-Wl,--wrap=go2rtc_process_get_override_path"
    "-Wl,--wrap=go2rtc_process_try_get_pid" "-Wl,--wrap=db_get_system_setting_alloc"
    "-Wl,--wrap=httpd_authorize_global_action")

add_layer3_test(test_onvif_transport CI_UNIT CI_SANITIZER)
target_link_options(test_onvif_transport PRIVATE "-Wl,--wrap=_log_message_ctx" "-Wl,--wrap=send")
set_tests_properties(test_onvif_transport PROPERTIES TIMEOUT 20)
add_layer3_test(test_onvif_device_test CI_UNIT CI_SANITIZER)
set_tests_properties(test_onvif_device_test PROPERTIES TIMEOUT 60)
target_link_options(test_onvif_poll_recovery PRIVATE
    "-Wl,--wrap=detect_motion_onvif" "-Wl,--wrap=av_usleep"
)
set_tests_properties(test_onvif_poll_recovery PROPERTIES TIMEOUT 15)
add_layer2_test(test_onvif_lpr_event)
target_compile_definitions(test_onvif_lpr_event PRIVATE
    TEST_SOURCE_DIR="${CMAKE_CURRENT_SOURCE_DIR}"
)
add_layer2_test_with_ffmpeg(test_api_detection)
target_compile_definitions(test_api_detection PRIVATE
    OBS_FIXTURE_DIR="${CMAKE_SOURCE_DIR}/tests/fixtures/integrations/observations")
add_layer2_test_with_ffmpeg(test_mp4_failed_open_memory CI_UNIT)
add_layer2_test_with_ffmpeg(test_ffmpeg_rgb24_padding)
if(TARGET lightnvr-ffmpeg-guard)
    add_executable(test_ffmpeg_guard_probe ffmpeg_guard_probe.c)
    set_target_properties(test_ffmpeg_guard_probe PROPERTIES
        RUNTIME_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/bin")
    if(CMAKE_BUILD_TYPE STREQUAL "Debug" AND
       CMAKE_C_COMPILER_ID MATCHES "GNU|Clang" AND
       CMAKE_CXX_COMPILER_ID MATCHES "GNU|Clang")
        target_compile_options(test_ffmpeg_guard_probe PRIVATE
            -fno-sanitize=address,undefined)
        target_link_options(test_ffmpeg_guard_probe PRIVATE
            -fno-sanitize=address,undefined)
    endif()
    add_test(NAME test_ffmpeg_guard COMMAND sh
        "${CMAKE_CURRENT_SOURCE_DIR}/test_ffmpeg_guard.sh"
        "$<TARGET_FILE:lightnvr-ffmpeg-guard>"
        "$<TARGET_FILE:test_ffmpeg_guard_probe>")
endif()
add_layer2_test_with_curl(test_url_utils CI_UNIT CI_SANITIZER)
add_layer2_test_with_curl(test_rtsp_probe CI_UNIT CI_SANITIZER)
add_layer2_test(test_db_streams CI_UNIT CI_SANITIZER)
add_layer2_test(test_db_detection_engines)
add_layer2_test(test_db_lpr_reads)
add_layer2_test(test_api_handlers_lpr)
add_layer2_test(test_api_handlers_detection_engines)
add_layer2_test(test_db_onvif_discovery_inventory)
add_layer2_test(test_db_locations)
add_layer2_test(test_api_handlers_locations)
add_layer2_test(test_db_camera_tags)
add_layer2_test(test_api_handlers_camera_tags CI_UNIT CI_SANITIZER)
target_link_options(test_api_handlers_camera_tags PRIVATE "-Wl,--wrap=httpd_check_action_access")
add_layer2_test(test_camera_selector)
add_layer2_test(test_api_handlers_fleet)
add_layer2_test(test_api_handlers_workspaces)
add_layer2_test(test_db_fleet_saved_views CI_UNIT CI_SANITIZER)
add_layer2_test(test_db_live_saved_layouts)
add_layer2_test(test_api_handlers_live_layouts)
add_layer2_test(test_db_camera_favorites CI_UNIT)
add_layer2_test(test_api_handlers_camera_favorites CI_UNIT)
add_layer2_test(test_api_handlers_observations CI_UNIT CI_SANITIZER)
target_link_options(test_api_handlers_observations PRIVATE
    "-Wl,--wrap=httpd_check_action_access"
    "-Wl,--wrap=httpd_evaluate_camera_identity_action"
    "-Wl,--wrap=authorization_filter_cameras"
    "-Wl,--wrap=http_serve_file"
    "-Wl,--wrap=audit_log_append"
    "-Wl,--wrap=audit_log_operation")
target_compile_definitions(test_api_handlers_observations PRIVATE
    OBS_FIXTURE_DIR="${CMAKE_SOURCE_DIR}/tests/fixtures/integrations/observations")
set_tests_properties(test_api_handlers_observations PROPERTIES TIMEOUT 60)
add_layer2_test(test_db_operator_floor_plans)
add_layer2_test(test_api_handlers_operator_floor_plans)
add_layer2_test(test_floor_plan_background_publication CI_UNIT CI_SANITIZER)
target_link_options(test_floor_plan_background_publication PRIVATE -Wl,--wrap=mkstemp -Wl,--wrap=openat -Wl,--wrap=fstat -Wl,--wrap=syscall -Wl,--wrap=mkdtemp -Wl,--wrap=access -Wl,--wrap=rename -Wl,--wrap=db_operator_floor_plan_set_background)
set_tests_properties(test_floor_plan_background_publication PROPERTIES TIMEOUT 30)
add_layer3_test(test_delete_stream_launch_failure CI_UNIT CI_SANITIZER)
target_link_options(test_delete_stream_launch_failure PRIVATE
    "-Wl,--wrap=pthread_create" "-Wl,--wrap=free"
    "-Wl,--wrap=go2rtc_integration_full_start"
    "-Wl,--wrap=go2rtc_integration_unregister_stream")
set_tests_properties(test_delete_stream_launch_failure PROPERTIES TIMEOUT 15)
add_layer3_test(test_put_stream_launch_failure CI_UNIT CI_SANITIZER)
target_link_options(test_put_stream_launch_failure PRIVATE
    "-Wl,--wrap=pthread_create" "-Wl,--wrap=free"
    "-Wl,--wrap=go2rtc_stream_is_ready"
    "-Wl,--wrap=go2rtc_integration_unregister_stream"
    "-Wl,--wrap=go2rtc_integration_reload_stream"
    "-Wl,--wrap=go2rtc_api_remove_stream"
    "-Wl,--wrap=go2rtc_stream_register_substream"
    "-Wl,--wrap=go2rtc_integration_restart_process")
set_tests_properties(test_put_stream_launch_failure PROPERTIES TIMEOUT 15)
add_layer2_test(test_api_handlers_investigations)
# Counts SQLite VM steps to keep investigation search bounded by its window.
target_link_options(test_api_handlers_investigations PRIVATE
    "-Wl,--wrap=sqlite3_finalize")
add_layer2_test(test_db_investigation_bookmarks)
add_layer2_test(test_api_handlers_investigation_bookmarks)
add_layer2_test(test_db_camera_collections)
add_layer2_test(test_api_handlers_camera_collections)
add_layer2_test(test_db_recordings_extended CI_UNIT CI_SANITIZER)
add_layer2_test(test_recording_camera_scope CI_UNIT CI_SANITIZER)
target_link_options(test_recording_camera_scope PRIVATE
    -Wl,--wrap=db_open_readonly_connection
    -Wl,--wrap=sqlite3_step)
set_tests_properties(test_recording_camera_scope PROPERTIES TIMEOUT 30)
add_layer2_test(test_api_handlers_recordings_list)
add_layer2_test(test_recording_transcode)
add_layer2_test(test_api_handlers_recordings_playback CI_UNIT)
target_link_options(test_api_handlers_recordings_playback PRIVATE
    "-Wl,--wrap=storage_source_resolve"
    "-Wl,--wrap=storage_source_remote"
    "-Wl,--wrap=get_recording_metadata_by_id"
    "-Wl,--wrap=httpd_authorize_camera_identity_action_with_context"
    "-Wl,--wrap=recording_needs_hevc_transcode"
    "-Wl,--wrap=request_recording_transcode_cache"
    "-Wl,--wrap=http_serve_file"
)
add_layer2_test(test_storage_manager_retention CI_UNIT)
add_layer2_test_with_ffmpeg(test_storage_reconcile_incomplete CI_UNIT CI_SANITIZER)
target_link_options(test_storage_reconcile_incomplete PRIVATE
    "-Wl,--wrap=stat" "-Wl,--wrap=lstat"
    "-Wl,--wrap=avcodec_find_decoder" "-Wl,--wrap=av_gettime_relative")
set_tests_properties(test_storage_reconcile_incomplete PROPERTIES TIMEOUT 60)
add_layer2_test(test_storage_target_pressure_cleanup CI_UNIT)
add_layer2_test(test_storage_migration CI_UNIT)
add_layer2_test(test_storage_migration_publication CI_UNIT CI_SANITIZER)
target_link_options(test_storage_migration_publication PRIVATE -Wl,--wrap=lstat -Wl,--wrap=syscall -Wl,--wrap=unlink -Wl,--wrap=db_storage_migration_update_progress -Wl,--wrap=db_storage_migration_commit_location)
set_tests_properties(test_storage_migration_publication PROPERTIES TIMEOUT 30)
add_layer2_test(test_storage_deletion_ledger)   # Layer 2: deletion ledger scoping, pruning, indexes
if(ENABLE_S3_ARCHIVE)
    add_layer2_test(test_storage_archive CI_UNIT)
    target_link_options(test_storage_archive PRIVATE "-Wl,--wrap=uv_timer_start"
        "-Wl,--wrap=db_open_readonly_connection")
endif()
add_layer2_test(test_db_detections CI_UNIT CI_SANITIZER)
add_layer2_test(test_db_zones CI_UNIT CI_SANITIZER)
add_layer2_test(test_api_handlers_zones CI_UNIT)
target_link_options(test_api_handlers_zones PRIVATE
    "-Wl,--wrap=httpd_authorize_stream_action")
add_layer2_test(test_db_events CI_UNIT CI_SANITIZER)
add_layer2_test(test_db_auth CI_UNIT CI_SANITIZER)
target_link_options(test_db_auth PRIVATE "-Wl,--wrap=time")
add_layer2_test(test_totp CI_UNIT CI_SANITIZER)
add_layer2_test(test_authorization CI_UNIT)
target_link_options(test_authorization PRIVATE "-Wl,--wrap=get_recording_detection_summaries_for_cameras")
add_layer2_test(test_openapi_projection CI_UNIT CI_SANITIZER)
target_link_options(test_openapi_projection PRIVATE
    "-Wl,--wrap=authorization_snapshot_setup_complete"
    "-Wl,--wrap=authorization_snapshot_camera_count"
    "-Wl,--wrap=authorization_snapshot_allows_non_scoped_identity"
    "-Wl,--wrap=authorization_snapshot_evaluate")
set_tests_properties(test_openapi_projection PROPERTIES TIMEOUT 30)
add_layer2_test(test_api_handlers_openapi CI_UNIT CI_SANITIZER)
target_link_options(test_api_handlers_openapi PRIVATE
    "-Wl,--wrap=authorization_snapshot_create"
    "-Wl,--wrap=openapi_project"
    "-Wl,--wrap=http_response_set_body"
    "-Wl,--wrap=cJSON_PrintUnformatted")
set_tests_properties(test_api_handlers_openapi PROPERTIES TIMEOUT 30)
add_layer3_test(test_libuv_handler_lifecycle CI_UNIT CI_SANITIZER)
set_tests_properties(test_libuv_handler_lifecycle PROPERTIES TIMEOUT 25)
add_layer2_test(test_authorization_snapshot CI_UNIT CI_SANITIZER)
target_link_options(test_authorization_snapshot PRIVATE
    -Wl,--wrap=malloc -Wl,--wrap=calloc -Wl,--wrap=realloc
    -Wl,--wrap=sqlite3_prepare_v2 -Wl,--wrap=time
    -Wl,--wrap=metrics_get_max_streams -Wl,--wrap=metrics_snapshot_all
    -Wl,--wrap=authorization_evaluate_in_context
    -Wl,--wrap=go2rtc_integration_is_initialized
    -Wl,--wrap=go2rtc_api_get_stream_activity)
set_tests_properties(test_authorization_snapshot PROPERTIES TIMEOUT 30)
add_layer2_test(test_api_handlers_users CI_UNIT)
add_layer2_test(test_audit_log CI_SANITIZER)
add_layer2_test(test_audit_summary CI_SANITIZER)
add_layer2_test(test_event_envelope)
target_compile_definitions(test_event_envelope PRIVATE LIGHTNVR_SOURCE_DIR="${CMAKE_SOURCE_DIR}")
add_layer2_test(test_event_bus)
add_layer2_test(test_event_detection_adapter)
add_layer2_test(test_event_outbox)
add_layer2_test(test_mqtt_delivery_worker)
add_layer2_test(test_db_event_routes)
add_layer2_test(test_event_route_suppression)
add_layer2_test(test_api_handlers_event_routes)
add_layer2_test(test_event_router)
add_layer2_test(test_db_event_destinations)
add_layer2_test(test_api_handlers_event_destinations)
add_layer2_test(test_db_storage_targets CI_UNIT)
add_layer2_test(test_db_storage_policies CI_UNIT)
add_layer2_test(test_api_handlers_storage_targets CI_UNIT CI_SANITIZER)
target_link_options(test_api_handlers_storage_targets PRIVATE -Wl,--wrap=trigger_storage_cleanup)
add_layer2_test(test_api_handlers_storage_policies CI_UNIT)
add_layer2_test(test_db_transactions CI_UNIT CI_SANITIZER)
add_layer2_test(test_db_maintenance CI_UNIT CI_SANITIZER)
add_layer2_test(test_db_backup_cleanup CI_UNIT CI_SANITIZER)
target_link_options(test_db_backup_cleanup PRIVATE -Wl,--wrap=lstat -Wl,--wrap=fstatat
    -Wl,--wrap=openat -Wl,--wrap=close -Wl,--wrap=fstat -Wl,--wrap=mkdirat
    -Wl,--wrap=syscall -Wl,--wrap=unlinkat
    -Wl,--wrap=readdir -Wl,--wrap=lightnvr_uuid_generate_v4)
set_tests_properties(test_db_backup_cleanup PROPERTIES TIMEOUT 30)
add_layer3_test(test_db_backup_retention_identity CI_UNIT CI_SANITIZER)
target_link_options(test_db_backup_retention_identity PRIVATE -Wl,--wrap=stat -Wl,--wrap=fstat
    -Wl,--wrap=openat -Wl,--wrap=close -Wl,--wrap=opendir -Wl,--wrap=closedir
    -Wl,--wrap=readdir -Wl,--wrap=realloc -Wl,--wrap=strdup
    -Wl,--wrap=syscall -Wl,--wrap=unlinkat)
set_tests_properties(test_db_backup_retention_identity PROPERTIES TIMEOUT 30)
add_layer2_test(test_db_query_builder CI_UNIT CI_SANITIZER)
add_layer2_test(test_db_system_settings)
add_layer2_test(test_logger_json CI_UNIT CI_SANITIZER)
add_layer2_test(test_batch_delete_progress CI_UNIT CI_SANITIZER)
add_layer2_test(test_db_recordings_sync CI_UNIT CI_SANITIZER)
add_layer2_test(test_httpd_utils CI_UNIT CI_SANITIZER)
add_layer2_test(test_zone_filter CI_UNIT CI_SANITIZER)
add_layer2_test(test_onvif_soap_fault CI_UNIT)
add_layer2_test(test_onvif_discovery_response)
add_layer2_test(test_go2rtc_base_path)
add_layer2_test(test_yaml_validate)
if(YAML_FOUND)
    if(YAML_LIBRARY_DIRS)
        target_link_directories(test_yaml_validate PRIVATE ${YAML_LIBRARY_DIRS})
    endif()
    target_link_libraries(test_yaml_validate ${YAML_LIBRARIES})
endif()
add_layer2_test(test_yaml_redact)
if(YAML_FOUND)
    if(YAML_LIBRARY_DIRS)
        target_link_directories(test_yaml_redact PRIVATE ${YAML_LIBRARY_DIRS})
    endif()
    target_link_libraries(test_yaml_redact ${YAML_LIBRARIES})
endif()
# HTTPS termination in the web server (#650); only an ENABLE_SSL build has it.
# The go2rtc WebSocket-over-TLS case runs when go2rtc is enabled too.
if(ENABLE_SSL)
    add_layer3_test(test_libuv_tls CI_UNIT CI_SANITIZER)
    set_tests_properties(test_libuv_tls PROPERTIES TIMEOUT 60)
endif()
add_layer2_test_with_curl(test_go2rtc_process_detection)
if(ENABLE_GO2RTC)
    add_layer2_test_with_curl(test_media_authority CI_UNIT CI_SANITIZER)
    target_link_libraries(test_media_authority ${FFMPEG_LIBRARIES})
    add_layer2_test_with_curl(test_go2rtc_proxy_query)
    add_layer3_test(test_go2rtc_multi_source_limits CI_UNIT)
    set_tests_properties(test_go2rtc_multi_source_limits PROPERTIES TIMEOUT 30)
    add_layer3_test(test_go2rtc_websocket)
    set_tests_properties(test_go2rtc_websocket PROPERTIES TIMEOUT 20)
    add_layer3_test(test_go2rtc_proxy_routing)
    set_tests_properties(test_go2rtc_proxy_routing PROPERTIES TIMEOUT 15)
    add_layer2_test_with_curl(test_go2rtc_proxy_thread)
    target_link_options(test_go2rtc_proxy_thread PRIVATE
        "-Wl,--wrap=lightnvr_uuid_generate_v4"
        "-Wl,--wrap=go2rtc_proxy_authorize_request"
        "-Wl,--wrap=libuv_send_response_ex"
        "-Wl,--wrap=curl_easy_perform"
    )
    set_tests_properties(test_go2rtc_proxy_thread PROPERTIES TIMEOUT 45)
    add_layer2_test_with_curl(test_go2rtc_stream_activity)
    add_layer2_test_with_curl(test_go2rtc_publish_async)
    add_executable(fake_go2rtc_server fake_go2rtc_server.c)
    set_target_properties(fake_go2rtc_server PROPERTIES
        RUNTIME_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/bin"
    )
    add_layer2_test_with_curl(test_go2rtc_process_lifecycle)
    add_layer3_test(test_go2rtc_process_ownership CI_UNIT CI_SANITIZER)
    target_compile_definitions(test_go2rtc_process_lifecycle PRIVATE
        FAKE_GO2RTC_PATH="$<TARGET_FILE:fake_go2rtc_server>"
    )
    add_dependencies(test_go2rtc_process_lifecycle fake_go2rtc_server)
    add_layer3_test(test_go2rtc_substream_registration)
    target_compile_definitions(test_go2rtc_substream_registration PRIVATE
        FAKE_GO2RTC_PATH="$<TARGET_FILE:fake_go2rtc_server>"
    )
    target_link_options(test_go2rtc_substream_registration PRIVATE
        "-Wl,--wrap=go2rtc_api_add_stream"
        "-Wl,--wrap=go2rtc_api_add_stream_multi"
        "-Wl,--wrap=get_stream_config_by_name"
    )
    add_dependencies(test_go2rtc_substream_registration fake_go2rtc_server)
    set_tests_properties(test_go2rtc_substream_registration PROPERTIES TIMEOUT 60)
    add_layer3_test(test_go2rtc_unregister_recovery)
    target_compile_definitions(test_go2rtc_unregister_recovery PRIVATE
        FAKE_GO2RTC_PATH="$<TARGET_FILE:fake_go2rtc_server>"
    )
    target_link_options(test_go2rtc_unregister_recovery PRIVATE
        "-Wl,--wrap=go2rtc_api_delete_preload"
        "-Wl,--wrap=go2rtc_api_remove_stream"
    )
    add_dependencies(test_go2rtc_unregister_recovery fake_go2rtc_server)
    set_tests_properties(test_go2rtc_unregister_recovery PROPERTIES TIMEOUT 60)
endif()
if(ENABLE_GO2RTC)
    add_layer2_test_with_curl(test_go2rtc_process_config_generation CI_UNIT CI_SANITIZER)
    target_link_options(test_go2rtc_process_config_generation PRIVATE
        "-Wl,--wrap=fchmod" "-Wl,--wrap=write" "-Wl,--wrap=fsync"
        "-Wl,--wrap=close" "-Wl,--wrap=rename")
    if(TARGET lightnvr-ffmpeg-guard)
        find_package(Python3 COMPONENTS Interpreter REQUIRED)
        add_test(NAME test_go2rtc_live_transcode COMMAND ${Python3_EXECUTABLE}
            "${CMAKE_CURRENT_SOURCE_DIR}/test_go2rtc_live_transcode.py"
            --config-writer "$<TARGET_FILE:test_go2rtc_process_config_generation>"
            --guard "$<TARGET_FILE:lightnvr-ffmpeg-guard>")
        set_tests_properties(test_go2rtc_live_transcode PROPERTIES
            TIMEOUT 180 SKIP_RETURN_CODE 77)
    endif()
    add_layer2_test_with_curl(test_go2rtc_binary_detection)
    add_layer2_test_with_curl(test_go2rtc_two_config_merge)
    add_layer3_test(test_go2rtc_managed_listeners)
    set_tests_properties(test_go2rtc_managed_listeners PROPERTIES TIMEOUT 90)
    add_layer2_test_with_curl(test_go2rtc_server_info CI_UNIT CI_SANITIZER)
endif()

# ====================================================================
# Layer 3: Integration tests — lightnvr_lib + FFmpeg
# ====================================================================
add_layer3_test(test_faac_encoder CI_UNIT CI_SANITIZER)
add_layer3_test(test_audio_transcoder CI_UNIT CI_SANITIZER)
add_layer3_test(test_mp4_forced_reconnect CI_UNIT)
target_link_options(test_mp4_forced_reconnect PRIVATE
    "-Wl,--wrap=record_segment" "-Wl,--wrap=av_usleep"
    "-Wl,--wrap=get_stream_config_by_name" "-Wl,--wrap=storage_should_pause_recording"
    "-Wl,--wrap=prepare_placed_mp4_recording_path")
add_layer3_test(test_mp4_recovery_watchdog CI_UNIT)
target_link_options(test_mp4_recovery_watchdog PRIVATE
    "-Wl,--wrap=sleep" "-Wl,--wrap=mp4_writer_start_recording_thread"
    "-Wl,--wrap=mp4_writer_stop_recording_thread" "-Wl,--wrap=mp4_writer_is_recording"
    "-Wl,--wrap=go2rtc_integration_is_using_go2rtc_for_recording" "-Wl,--wrap=_log_message_ctx")
set_tests_properties(test_mp4_forced_reconnect test_mp4_recovery_watchdog PROPERTIES TIMEOUT 10)
add_layer3_test(test_mp4_audio_recovery CI_UNIT)
target_link_options(test_mp4_audio_recovery PRIVATE
    "-Wl,--wrap=av_read_frame" "-Wl,--wrap=cleanup_audio_transcoder")
add_layer3_test(test_stream_manager CI_UNIT CI_SANITIZER)
add_layer3_test(test_stream_state CI_UNIT CI_SANITIZER)
add_layer3_test(test_stream_state_disposal CI_UNIT CI_SANITIZER)
target_link_options(test_stream_state_disposal PRIVATE
    "-Wl,--wrap=pthread_mutex_destroy"
    "-Wl,--wrap=stream_stop_hls"
    "-Wl,--wrap=stop_mp4_recording"
    "-Wl,--wrap=reset_timestamp_tracker"
    "-Wl,--wrap=remove_timestamp_tracker")
set_tests_properties(test_stream_state_disposal PROPERTIES TIMEOUT 15)
add_layer3_test(test_packet_buffer CI_UNIT CI_SANITIZER)
add_layer3_test(test_hls_cleanup CI_UNIT)
target_compile_definitions(test_hls_cleanup PRIVATE
    TEST_RECORDING_PATH="${CMAKE_SOURCE_DIR}/tests/integration/fixtures/recording.mp4"
)
target_link_options(test_hls_cleanup PRIVATE
    "-Wl,--wrap=alarm" "-Wl,--wrap=sigaction" "-Wl,--wrap=av_read_frame"
)
set_tests_properties(test_hls_cleanup PROPERTIES TIMEOUT 30)
add_layer3_test(test_hls_reconnect_backoff)
target_link_options(test_hls_reconnect_backoff PRIVATE "-Wl,--wrap=av_usleep")
set_tests_properties(test_hls_reconnect_backoff PROPERTIES TIMEOUT 10)
add_layer3_test(test_mp4_writer_watchdog)
add_layer3_test(test_mp4_segment_timestamps)
add_layer3_test(test_mp4_segment_boundaries CI_UNIT CI_SANITIZER)
target_link_options(test_mp4_segment_boundaries PRIVATE
    "-Wl,--wrap=av_interleaved_write_frame")
add_layer3_test(test_mp4_segment_retry_pairing CI_UNIT)
target_link_options(test_mp4_segment_retry_pairing PRIVATE
    "-Wl,--wrap=time"
    "-Wl,--wrap=record_segment" "-Wl,--wrap=av_usleep"
    "-Wl,--wrap=get_stream_config_by_name" "-Wl,--wrap=storage_should_pause_recording"
    "-Wl,--wrap=prepare_placed_mp4_recording_path"
    "-Wl,--wrap=add_recording_metadata" "-Wl,--wrap=update_recording_metadata"
    "-Wl,--wrap=update_stream_storage_cache_add_recording")
set_tests_properties(test_mp4_segment_retry_pairing PROPERTIES TIMEOUT 10)
add_layer3_test(test_recording_video_params CI_UNIT CI_SANITIZER)
target_link_options(test_recording_video_params PRIVATE
    "-Wl,--wrap=update_stream_video_params")
add_layer3_test(test_mp4_faststart)
target_compile_definitions(test_mp4_faststart PRIVATE
    RECORDING_FIXTURE="${CMAKE_SOURCE_DIR}/tests/integration/fixtures/recording.mp4")
add_layer3_test(test_audio_voice_enhancement CI_UNIT CI_SANITIZER)
if(ENABLE_AUDIO_FILTERS)
    # Fault injection for the voice-enhancement graph (issue #652).
    add_layer3_test(test_audio_voice_filter_faults CI_UNIT CI_SANITIZER)
    target_link_options(test_audio_voice_filter_faults PRIVATE
        "-Wl,--wrap=avfilter_inout_alloc" "-Wl,--wrap=av_strdup"
        "-Wl,--wrap=avfilter_graph_config" "-Wl,--wrap=av_buffersrc_add_frame_flags"
        "-Wl,--wrap=av_buffersink_get_frame" "-Wl,--wrap=av_audio_fifo_write"
    )
endif()
add_layer3_test(test_timestamp_manager CI_UNIT CI_SANITIZER)
add_layer3_test(test_api_handlers_system)
add_layer3_test(test_system_info_helpers CI_UNIT CI_SANITIZER)
add_layer3_test(test_api_read_isolation)
set_tests_properties(test_api_read_isolation PROPERTIES TIMEOUT 15)
add_layer1_test(test_external_motion_trigger)    # Layer 1: external_motion_trigger state-machine (PR #356)
add_layer2_test(test_motion_trigger_parse CI_UNIT)       # Layer 2: motion trigger body parsing (#466)
add_layer2_test(test_motion_max_area CI_UNIT)            # Layer 2: motion detector max-area sanity cap
add_layer3_test(test_cross_stream_motion_trigger)
add_layer3_test(test_motion_pulse_worker CI_UNIT CI_SANITIZER)
target_link_options(test_motion_pulse_worker PRIVATE
    "-Wl,--wrap=close_external_motion_detections"
    "-Wl,--wrap=unified_detection_notify_motion"
    "-Wl,--wrap=process_motion_event")
set_tests_properties(test_motion_pulse_worker PROPERTIES TIMEOUT 10)
target_link_options(test_motion_pulse_worker PRIVATE
    "-Wl,--wrap=get_stream_by_name"
    "-Wl,--wrap=get_stream_config"
    "-Wl,--wrap=httpd_authorize_stream_action"
    "-Wl,--wrap=store_external_motion_detections"
    "-Wl,--wrap=get_current_recording_id_for_stream"
    "-Wl,--wrap=calloc"
    "-Wl,--wrap=pthread_create"
    "-Wl,--wrap=pthread_attr_init"
    "-Wl,--wrap=pthread_attr_setdetachstate"
    "-Wl,--wrap=pthread_attr_destroy")
add_layer3_test(test_annotation_writer_registry)  # Layer 3: continuous-writer probe behind annotation mode (#547)
add_layer3_test(test_detection_current_frame CI_UNIT CI_SANITIZER)
target_compile_definitions(test_detection_current_frame PRIVATE
    TEST_FIXTURE_DIR="${CMAKE_SOURCE_DIR}/tests/fixtures/detection"
)
target_link_options(test_detection_current_frame PRIVATE
    "-Wl,--wrap=detect_objects_api_snapshot_with_options"
    "-Wl,--wrap=detect_objects_api_with_evidence"
    "-Wl,--wrap=filter_detections_by_zones"
    "-Wl,--wrap=filter_detections_by_stream_objects"
)
set_tests_properties(test_detection_current_frame PROPERTIES TIMEOUT 15)
add_layer3_test(test_detection_io_recovery CI_UNIT CI_SANITIZER)
target_compile_definitions(test_detection_io_recovery PRIVATE
    TEST_RECORDING_PATH="${CMAKE_SOURCE_DIR}/tests/integration/fixtures/recording.mp4"
)
target_link_options(test_detection_io_recovery PRIVATE
    "-Wl,--wrap=avformat_open_input" "-Wl,--wrap=avformat_find_stream_info"
    "-Wl,--wrap=av_read_frame" "-Wl,--wrap=av_gettime_relative"
    "-Wl,--wrap=_log_message_ctx"
)
set_tests_properties(test_detection_io_recovery PROPERTIES TIMEOUT 30)
add_layer3_test(test_udt_recording CI_UNIT CI_SANITIZER)
target_link_options(test_udt_recording PRIVATE
    "-Wl,--wrap=lightnvr_uuid_generate_v4"
    "-Wl,--wrap=recording_file_open"
    "-Wl,--wrap=go2rtc_stream_get_rtsp_url"
)
target_compile_definitions(test_udt_recording PRIVATE
    TEST_RECORDING_PATH="${CMAKE_SOURCE_DIR}/tests/integration/fixtures/recording.mp4"
)
set_tests_properties(test_udt_recording PROPERTIES TIMEOUT 60)
# INT_01 L6: local detector evidence (pure batch logic + UDT integration).
add_layer2_test_with_ffmpeg(test_detection_evidence CI_UNIT CI_SANITIZER)
add_layer3_test(test_detection_observation_evidence CI_UNIT CI_SANITIZER)
target_link_options(test_detection_observation_evidence PRIVATE
    "-Wl,--wrap=detect_objects"
    "-Wl,--wrap=detect_motion"
    "-Wl,--wrap=detection_evidence_model_geometry"
    "-Wl,--wrap=filter_detections_by_zones"
    "-Wl,--wrap=filter_detections_by_stream_objects"
)
set_tests_properties(test_detection_observation_evidence PROPERTIES TIMEOUT 60)
add_layer3_test(test_api_detection_observation_evidence CI_UNIT CI_SANITIZER)
target_link_options(test_api_detection_observation_evidence PRIVATE
    "-Wl,--wrap=filter_detections_by_zones"
    "-Wl,--wrap=filter_detections_by_stream_objects"
)
set_tests_properties(test_api_detection_observation_evidence PROPERTIES TIMEOUT 60)

# Health tasks register their tests in independent fragments so parallel work
# never needs to modify this shared list.
file(GLOB SYSTEM_HEALTH_TEST_FRAGMENTS CONFIGURE_DEPENDS
    "${CMAKE_CURRENT_SOURCE_DIR}/cmake/system_health/*.cmake")
list(SORT SYSTEM_HEALTH_TEST_FRAGMENTS)
foreach(SYSTEM_HEALTH_TEST_FRAGMENT IN LISTS SYSTEM_HEALTH_TEST_FRAGMENTS)
    include("${SYSTEM_HEALTH_TEST_FRAGMENT}")
endforeach()

message(STATUS "Building Unity unit tests (Layer 1 + Layer 2 + Layer 3)")

# Recording integrity regressions use private files and process-local faults.
add_layer3_test(test_recording_export_io CI_UNIT CI_SANITIZER)
target_link_options(test_recording_export_io PRIVATE
    "-Wl,--wrap=get_recording_metadata_by_id" "-Wl,--wrap=storage_source_resolve")
add_layer3_test(test_recording_file CI_UNIT CI_SANITIZER)
target_link_options(test_recording_file PRIVATE "-Wl,--wrap=write" "-Wl,--wrap=close"
    "-Wl,--wrap=lightnvr_uuid_generate_v4")

add_layer1_test(test_uuid CI_UNIT CI_SANITIZER)
target_sources(test_uuid PRIVATE ${CMAKE_SOURCE_DIR}/src/utils/uuid.c)
# UUIDv5 (observation message IDs) hashes with mbedTLS SHA-1.
target_include_directories(test_uuid PRIVATE ${SSL_INCLUDE_DIRS})
target_link_libraries(test_uuid ${SSL_LIBRARIES})
target_link_options(test_uuid PRIVATE "-Wl,--wrap=getrandom" "-Wl,--wrap=fread")

# Compile the MQTT producer even in the isolated no-MQTT API variant, when the
# development headers are available. Wrappers exercise identity changes during
# capture without a broker or go2rtc process.
find_path(MQTT_SNAPSHOT_TEST_INCLUDE mosquitto.h)
find_library(MQTT_SNAPSHOT_TEST_LIBRARY mosquitto)
if(MQTT_SNAPSHOT_TEST_INCLUDE AND MQTT_SNAPSHOT_TEST_LIBRARY)
    add_layer2_test(test_mqtt_snapshot_identity CI_UNIT CI_SANITIZER)
    target_include_directories(test_mqtt_snapshot_identity PRIVATE ${MQTT_SNAPSHOT_TEST_INCLUDE})
    target_compile_definitions(test_mqtt_snapshot_identity PRIVATE ENABLE_MQTT=1)
    target_compile_options(test_mqtt_snapshot_identity PRIVATE -ffunction-sections -fdata-sections)
    target_link_libraries(test_mqtt_snapshot_identity ${MQTT_SNAPSHOT_TEST_LIBRARY})
    target_link_options(test_mqtt_snapshot_identity PRIVATE -Wl,--gc-sections
        -Wl,--wrap=get_stream_config_by_name -Wl,--wrap=go2rtc_get_snapshot
        -Wl,--wrap=mosquitto_publish)
endif()
add_layer2_test(test_api_handlers_device CI_UNIT CI_SANITIZER)
target_link_options(test_api_handlers_device PRIVATE
    "-Wl,--wrap=discover_onvif_devices" "-Wl,--wrap=get_onvif_device_profiles"
    "-Wl,--wrap=add_onvif_device_as_stream")

# Pulse registry reclamation uses a complete inventory snapshot and unique tokens.
add_layer3_test(test_motion_generation_registry CI_UNIT CI_SANITIZER)
target_link_options(test_motion_generation_registry PRIVATE
    "-Wl,--wrap=db_open_readonly_connection"
    "-Wl,--wrap=db_close_readonly_connection"
    "-Wl,--wrap=close_external_motion_detections"
    "-Wl,--wrap=unified_detection_notify_motion"
    "-Wl,--wrap=process_motion_event")
set_tests_properties(test_motion_generation_registry PROPERTIES TIMEOUT 10)

add_layer3_test(test_motion_pulse_sleep CI_UNIT CI_SANITIZER)
target_link_options(test_motion_pulse_sleep PRIVATE
    "-Wl,--wrap=nanosleep"
    "-Wl,--wrap=usleep"
    "-Wl,--wrap=close_external_motion_detections"
    "-Wl,--wrap=unified_detection_notify_motion"
    "-Wl,--wrap=process_motion_event")
set_tests_properties(test_motion_pulse_sleep PROPERTIES TIMEOUT 10)
