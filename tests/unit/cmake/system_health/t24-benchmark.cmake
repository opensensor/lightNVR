# T24 owns a non-Unity performance/replay executable outside tests/unit.
if(NOT TARGET test_system_health_benchmark)
    add_executable(test_system_health_benchmark
        ${CMAKE_SOURCE_DIR}/tests/performance/system_health_benchmark.c)
    target_include_directories(test_system_health_benchmark PRIVATE
        ${CMAKE_SOURCE_DIR}/include)
    target_link_libraries(test_system_health_benchmark
        lightnvr_lib
        ${SQLITE_LIBRARIES}
        ${SSL_LIBRARIES}
        ${HTTP_BACKEND_LIBS}
        inih_lib
        pthread
        dl
        m)
    if(CJSON_BUNDLED)
        target_link_libraries(test_system_health_benchmark cjson_lib)
    elseif(CJSON_FOUND)
        target_link_libraries(test_system_health_benchmark ${CJSON_LIBRARIES})
    endif()
    if(ENABLE_MQTT AND MOSQUITTO_FOUND)
        target_link_libraries(test_system_health_benchmark ${MOSQUITTO_LIBRARIES})
    endif()
    if(ENABLE_SOD)
        target_link_libraries(test_system_health_benchmark sod)
    endif()
    set_target_properties(test_system_health_benchmark PROPERTIES
        RUNTIME_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/bin")
    add_test(NAME test_system_health_benchmark
        COMMAND test_system_health_benchmark
            --samples 360
            --heap-limit-bytes 524288
            --cpu-ratio-limit 0.005)
endif()
