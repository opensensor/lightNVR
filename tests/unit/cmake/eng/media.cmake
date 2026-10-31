# T27 preserves all existing media tests and declares bounded native deadlines.
set_tests_properties(test_mp4_audio_recovery test_mp4_recovery_watchdog
    test_mp4_writer_watchdog test_mp4_segment_boundaries test_mp4_segment_timestamps
    test_mp4_faststart PROPERTIES TIMEOUT 30 LABELS "engineering;media")

add_executable(test_mp4_path_collision ${CMAKE_SOURCE_DIR}/tests/operational/media/test_mp4_path_collision.c)
target_include_directories(test_mp4_path_collision PRIVATE ${CMAKE_SOURCE_DIR}/include ${FFMPEG_INCLUDE_DIRS})
target_link_libraries(test_mp4_path_collision PRIVATE unity_lib lightnvr_lib ${FFMPEG_LIBRARIES}
    ${SQLITE_LIBRARIES} ${CURL_LIBRARIES} ${SSL_LIBRARIES} ${HTTP_BACKEND_LIBS} inih_lib pthread dl m)
if(CJSON_BUNDLED)
    target_link_libraries(test_mp4_path_collision PRIVATE cjson_lib)
elseif(CJSON_FOUND)
    target_link_libraries(test_mp4_path_collision PRIVATE ${CJSON_LIBRARIES})
endif()
if(ENABLE_MQTT AND MOSQUITTO_FOUND)
    target_link_libraries(test_mp4_path_collision PRIVATE ${MOSQUITTO_LIBRARIES})
endif()
if(ENABLE_SOD)
    target_link_libraries(test_mp4_path_collision PRIVATE sod)
endif()
set_target_properties(test_mp4_path_collision PROPERTIES RUNTIME_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/bin")
add_test(NAME test_mp4_path_collision COMMAND test_mp4_path_collision)
set_tests_properties(test_mp4_path_collision PROPERTIES TIMEOUT 20 LABELS "engineering;media")
target_link_options(test_mp4_path_collision PRIVATE "-Wl,--wrap=time" "-Wl,--wrap=open"
    "-Wl,--wrap=record_segment" "-Wl,--wrap=get_stream_config_by_name"
    "-Wl,--wrap=storage_should_pause_recording" "-Wl,--wrap=av_usleep"
    "-Wl,--wrap=prepare_placed_mp4_recording_path"
    "-Wl,--wrap=add_recording_metadata" "-Wl,--wrap=update_recording_metadata")

# Preserve existing deterministic loop seams; reservation itself is exercised
# with real files/threads by test_mp4_path_collision above.
target_link_options(test_mp4_forced_reconnect PRIVATE "-Wl,--wrap=reserve_mp4_recording_path"
    "-Wl,--wrap=release_mp4_recording_path")

add_executable(test_mp4_pressure_reconnect ${CMAKE_SOURCE_DIR}/tests/operational/media/test_mp4_pressure_reconnect.c)
target_include_directories(test_mp4_pressure_reconnect PRIVATE ${CMAKE_SOURCE_DIR}/include ${FFMPEG_INCLUDE_DIRS})
target_link_libraries(test_mp4_pressure_reconnect PRIVATE unity_lib lightnvr_lib ${FFMPEG_LIBRARIES}
    ${SQLITE_LIBRARIES} ${CURL_LIBRARIES} ${SSL_LIBRARIES} ${HTTP_BACKEND_LIBS} inih_lib pthread dl m)
if(CJSON_BUNDLED)
    target_link_libraries(test_mp4_pressure_reconnect PRIVATE cjson_lib)
elseif(CJSON_FOUND)
    target_link_libraries(test_mp4_pressure_reconnect PRIVATE ${CJSON_LIBRARIES})
endif()
if(ENABLE_MQTT AND MOSQUITTO_FOUND)
    target_link_libraries(test_mp4_pressure_reconnect PRIVATE ${MOSQUITTO_LIBRARIES})
endif()
if(ENABLE_SOD)
    target_link_libraries(test_mp4_pressure_reconnect PRIVATE sod)
endif()
set_target_properties(test_mp4_pressure_reconnect PROPERTIES RUNTIME_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/bin")
add_test(NAME test_mp4_pressure_reconnect COMMAND test_mp4_pressure_reconnect)
set_tests_properties(test_mp4_pressure_reconnect PROPERTIES TIMEOUT 20 LABELS "engineering;media")
target_link_options(test_mp4_pressure_reconnect PRIVATE
    "-Wl,--wrap=record_segment" "-Wl,--wrap=get_stream_config_by_name"
    "-Wl,--wrap=storage_should_pause_recording" "-Wl,--wrap=trigger_storage_cleanup"
    "-Wl,--wrap=av_usleep" "-Wl,--wrap=prepare_placed_mp4_recording_path"
    "-Wl,--wrap=reserve_mp4_recording_path" "-Wl,--wrap=release_mp4_recording_path"
    "-Wl,--wrap=avformat_close_input" "-Wl,--wrap=cleanup_audio_transcoder")
