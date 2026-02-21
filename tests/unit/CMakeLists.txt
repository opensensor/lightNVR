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

message(STATUS "Building Unity unit tests (Layer 1 + Layer 2)")

