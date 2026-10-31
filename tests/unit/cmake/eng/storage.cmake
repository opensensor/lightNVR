# T28 test-only storage seams. GNU ld wrappers affect these executables only.
target_link_options(test_storage_manager_retention PRIVATE "-Wl,--wrap=unlink")
set_tests_properties(test_storage_manager_retention test_storage_retention_sqlite PROPERTIES TIMEOUT 30)

# Use the existing Layer 3 link contract for operational sources outside unit/.
foreach(storage_test IN ITEMS test_storage_recording_faults test_storage_writer_backpressure)
    add_executable(${storage_test} ${CMAKE_SOURCE_DIR}/tests/operational/storage/${storage_test}.c)
    target_include_directories(${storage_test} PRIVATE ${CMAKE_SOURCE_DIR}/include ${FFMPEG_INCLUDE_DIRS})
    target_link_libraries(${storage_test} PRIVATE unity_lib lightnvr_lib ${FFMPEG_LIBRARIES}
        ${SQLITE_LIBRARIES} ${CURL_LIBRARIES} ${SSL_LIBRARIES} ${HTTP_BACKEND_LIBS} inih_lib pthread dl m)
    if(CJSON_BUNDLED)
        target_link_libraries(${storage_test} PRIVATE cjson_lib)
    elseif(CJSON_FOUND)
        target_link_libraries(${storage_test} PRIVATE ${CJSON_LIBRARIES})
    endif()
    if(ENABLE_MQTT AND MOSQUITTO_FOUND)
        target_link_libraries(${storage_test} PRIVATE ${MOSQUITTO_LIBRARIES})
    endif()
    if(ENABLE_SOD)
        target_link_libraries(${storage_test} PRIVATE sod)
    endif()
    set_target_properties(${storage_test} PROPERTIES RUNTIME_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/bin")
    add_test(NAME ${storage_test} COMMAND ${storage_test})
    set_tests_properties(${storage_test} PROPERTIES TIMEOUT 20)
endforeach()
target_link_options(test_storage_recording_faults PRIVATE
    "-Wl,--wrap=av_interleaved_write_frame" "-Wl,--wrap=avio_open" "-Wl,--wrap=avio_closep")
target_link_options(test_storage_writer_backpressure PRIVATE
    "-Wl,--wrap=record_segment" "-Wl,--wrap=av_usleep"
    "-Wl,--wrap=get_stream_config_by_name" "-Wl,--wrap=storage_should_pause_recording"
    "-Wl,--wrap=prepare_placed_mp4_recording_path" "-Wl,--wrap=trigger_storage_cleanup")
