add_executable(test_recording_io_metrics
    ${CMAKE_CURRENT_SOURCE_DIR}/test_recording_io_metrics.c
    ${CMAKE_SOURCE_DIR}/src/telemetry/recording_io_metrics.c
)
target_include_directories(test_recording_io_metrics PRIVATE
    ${CMAKE_SOURCE_DIR}/include
)
target_link_libraries(test_recording_io_metrics unity_lib pthread)
set_target_properties(test_recording_io_metrics PROPERTIES
    RUNTIME_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/bin"
)
add_test(NAME test_recording_io_metrics COMMAND test_recording_io_metrics)
