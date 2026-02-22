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
# Layer 1: test_storage_pressure
# Pure unit test — only needs unity_lib + project headers.
# disk_pressure_level_str() and evaluate_disk_pressure_level() are
# both static inline in storage_manager.h, so no .c files are linked.
# ====================================================================
add_executable(test_storage_pressure
    ${CMAKE_CURRENT_SOURCE_DIR}/test_storage_pressure.c
)
target_include_directories(test_storage_pressure PRIVATE
    ${CMAKE_SOURCE_DIR}/include
)
target_link_libraries(test_storage_pressure
    unity_lib
)
set_target_properties(test_storage_pressure
    PROPERTIES RUNTIME_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/bin"
)
add_test(NAME test_storage_pressure COMMAND test_storage_pressure)

# ====================================================================
# Layer 2: test_storage_retention_sqlite
# Integration test — runs real SQL queries via db_core / db_recordings.
# ====================================================================
add_executable(test_storage_retention_sqlite
    ${CMAKE_CURRENT_SOURCE_DIR}/test_storage_retention_sqlite.c
)
target_include_directories(test_storage_retention_sqlite PRIVATE
    ${CMAKE_SOURCE_DIR}/include
)
target_link_libraries(test_storage_retention_sqlite
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
    target_link_libraries(test_storage_retention_sqlite cjson_lib)
elseif(CJSON_FOUND)
    target_link_libraries(test_storage_retention_sqlite ${CJSON_LIBRARIES})
endif()
if(ENABLE_MQTT AND MOSQUITTO_FOUND)
    target_link_libraries(test_storage_retention_sqlite ${MOSQUITTO_LIBRARIES})
endif()
if(ENABLE_SOD)
    target_link_libraries(test_storage_retention_sqlite sod)
endif()
set_target_properties(test_storage_retention_sqlite
    PROPERTIES RUNTIME_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/bin"
)
add_test(NAME test_storage_retention_sqlite COMMAND test_storage_retention_sqlite)

# ====================================================================
# CMake helper macros — reduce boilerplate for test registration
# ====================================================================
macro(add_layer1_test TEST_NAME)
    add_executable(${TEST_NAME} ${CMAKE_CURRENT_SOURCE_DIR}/${TEST_NAME}.c)
    target_include_directories(${TEST_NAME} PRIVATE ${CMAKE_SOURCE_DIR}/include)
    target_link_libraries(${TEST_NAME} unity_lib)
    set_target_properties(${TEST_NAME} PROPERTIES RUNTIME_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/bin")
    add_test(NAME ${TEST_NAME} COMMAND ${TEST_NAME})
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
    if(ENABLE_SOD)
        target_link_libraries(${TEST_NAME} sod)
    endif()
    set_target_properties(${TEST_NAME} PROPERTIES RUNTIME_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/bin")
    add_test(NAME ${TEST_NAME} COMMAND ${TEST_NAME})
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
    if(ENABLE_SOD)
        target_link_libraries(${TEST_NAME} sod)
    endif()
    set_target_properties(${TEST_NAME} PROPERTIES RUNTIME_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/bin")
    add_test(NAME ${TEST_NAME} COMMAND ${TEST_NAME})
endmacro()

# ====================================================================
# Layer 1: Pure logic tests — unity_lib + project headers only
# (Only tests whose functions are static inline in headers)
# ====================================================================
add_layer1_test(test_storage_pressure_extended)
add_layer1_test(test_detection_result_structures)

# ====================================================================
# Layer 2: lightnvr_lib tests — compiled functions + optional SQLite
# ====================================================================
add_layer2_test(test_config)
add_layer2_test(test_logger)
add_layer2_test(test_strings)
add_layer2_test(test_memory)
add_layer2_test(test_request_response)
add_layer2_test(test_shutdown_coordinator)
add_layer2_test(test_detection_config)
add_layer2_test(test_detection_model_motion)
target_link_libraries(test_detection_model_motion ${CURL_LIBRARIES})
add_layer2_test(test_db_streams)
add_layer2_test(test_db_recordings_extended)
add_layer2_test(test_db_detections)
add_layer2_test(test_db_zones)
add_layer2_test(test_db_events)
add_layer2_test(test_db_auth)
add_layer2_test(test_db_transactions)
add_layer2_test(test_db_maintenance)
add_layer2_test(test_db_query_builder)
add_layer2_test(test_logger_json)
add_layer2_test(test_batch_delete_progress)
add_layer2_test(test_db_motion_config)
add_layer2_test(test_db_recordings_sync)
add_layer2_test(test_httpd_utils)
add_layer2_test(test_zone_filter)

# ====================================================================
# Layer 3: Integration tests — lightnvr_lib + FFmpeg
# ====================================================================
add_layer3_test(test_stream_manager)
add_layer3_test(test_stream_state)
add_layer3_test(test_packet_buffer)
add_layer3_test(test_timestamp_manager)

message(STATUS "Building Unity unit tests (Layer 1 + Layer 2 + Layer 3)")

