add_executable(test_incomplete_media_validation
    ${CMAKE_SOURCE_DIR}/tests/operational/lifecycle/test_incomplete_media_validation.c)
target_include_directories(test_incomplete_media_validation PRIVATE
    ${CMAKE_SOURCE_DIR}/include ${FFMPEG_INCLUDE_DIRS})
target_link_libraries(test_incomplete_media_validation PRIVATE unity_lib lightnvr_lib
    ${FFMPEG_LIBRARIES} ${SQLITE_LIBRARIES} ${CURL_LIBRARIES} ${SSL_LIBRARIES}
    ${HTTP_BACKEND_LIBS} inih_lib pthread dl m)
add_test(NAME test_incomplete_media_validation COMMAND test_incomplete_media_validation)
set_tests_properties(test_incomplete_media_validation PROPERTIES TIMEOUT 20)
target_link_options(test_incomplete_media_validation PRIVATE
    "-Wl,--wrap=stream_has_continuous_writer" "-Wl,--wrap=mp4_recording_thread_alive")
