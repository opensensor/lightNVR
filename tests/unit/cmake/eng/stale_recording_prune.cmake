add_executable(test_stale_recording_prune
    ${CMAKE_SOURCE_DIR}/tests/operational/lifecycle/test_stale_recording_prune.c)
target_include_directories(test_stale_recording_prune PRIVATE
    ${CMAKE_SOURCE_DIR}/include ${FFMPEG_INCLUDE_DIRS})
target_link_libraries(test_stale_recording_prune PRIVATE unity_lib lightnvr_lib
    ${FFMPEG_LIBRARIES} ${SQLITE_LIBRARIES} ${CURL_LIBRARIES} ${SSL_LIBRARIES}
    ${HTTP_BACKEND_LIBS} inih_lib pthread dl m)
target_link_options(test_stale_recording_prune PRIVATE
    "-Wl,--wrap=stream_has_continuous_writer" "-Wl,--wrap=mp4_recording_thread_alive"
    "-Wl,--wrap=get_stale_incomplete_recordings_page")
add_test(NAME test_stale_recording_prune COMMAND test_stale_recording_prune)
set_tests_properties(test_stale_recording_prune PROPERTIES TIMEOUT 30)
