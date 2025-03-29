# Tests CMakeLists.txt

# Get the ENABLE_SOD option from parent
if(NOT DEFINED ENABLE_SOD)
    option(ENABLE_SOD "Enable SOD library for object detection" ON)
endif()

# Include directories
include_directories(
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_CURRENT_SOURCE_DIR}/../include
)

# Find required packages
find_package(PkgConfig REQUIRED)
pkg_check_modules(FFMPEG REQUIRED libavcodec libavformat libavutil libswscale)
pkg_check_modules(SQLITE REQUIRED sqlite3)

# Add include directories for dependencies
include_directories(
    ${FFMPEG_INCLUDE_DIRS}
    ${SQLITE_INCLUDE_DIRS}
    ${CMAKE_CURRENT_SOURCE_DIR}/../external/inih
    ${CMAKE_CURRENT_SOURCE_DIR}/../external/mongoose
    ${CMAKE_CURRENT_SOURCE_DIR}/../external/cjson
)

# Define source files needed for tests
set(DETECTION_SOURCES
    ${CMAKE_CURRENT_SOURCE_DIR}/../src/video/detection.c
    ${CMAKE_CURRENT_SOURCE_DIR}/../src/video/detection_model.c
    ${CMAKE_CURRENT_SOURCE_DIR}/../src/video/sod_detection.c
    ${CMAKE_CURRENT_SOURCE_DIR}/../src/video/sod_realnet.c
    ${CMAKE_CURRENT_SOURCE_DIR}/../src/video/motion_detection.c
    ${CMAKE_CURRENT_SOURCE_DIR}/../src/core/logger.c
    ${CMAKE_CURRENT_SOURCE_DIR}/../src/core/config.c
    ${CMAKE_CURRENT_SOURCE_DIR}/../src/core/shutdown_coordinator.c
    ${CMAKE_CURRENT_SOURCE_DIR}/../src/web/logger_websocket.c
    ${CMAKE_CURRENT_SOURCE_DIR}/../src/web/api_handlers_system_ws.c
    ${CMAKE_CURRENT_SOURCE_DIR}/../src/video/onvif_discovery_messages.c
    ${CMAKE_CURRENT_SOURCE_DIR}/../src/web/api_handlers_system.c
    ${CMAKE_CURRENT_SOURCE_DIR}/../src/web/mongoose_server.c
    ${CMAKE_CURRENT_SOURCE_DIR}/../src/web/mongoose_server_websocket.c
    ${CMAKE_CURRENT_SOURCE_DIR}/../src/database/db_core.c
    ${CMAKE_CURRENT_SOURCE_DIR}/../src/database/db_streams.c
    ${CMAKE_CURRENT_SOURCE_DIR}/../src/database/db_schema.c
    ${CMAKE_CURRENT_SOURCE_DIR}/../external/inih/ini.c
    ${CMAKE_CURRENT_SOURCE_DIR}/../external/cjson/cJSON.c
    ${CMAKE_CURRENT_SOURCE_DIR}/../external/mongoose/mongoose.c
)

# Define motion detection sources
set(MOTION_DETECTION_SOURCES
    ${CMAKE_CURRENT_SOURCE_DIR}/../src/video/motion_detection.c
    ${CMAKE_CURRENT_SOURCE_DIR}/../src/core/logger.c
    ${CMAKE_CURRENT_SOURCE_DIR}/../src/core/config.c
    ${CMAKE_CURRENT_SOURCE_DIR}/../src/web/logger_websocket.c
    ${CMAKE_CURRENT_SOURCE_DIR}/../src/web/api_handlers_system_ws.c
    ${CMAKE_CURRENT_SOURCE_DIR}/../src/video/onvif_discovery_messages.c
    ${CMAKE_CURRENT_SOURCE_DIR}/../src/web/api_handlers_system.c
    ${CMAKE_CURRENT_SOURCE_DIR}/../src/web/mongoose_server.c
    ${CMAKE_CURRENT_SOURCE_DIR}/../src/web/mongoose_server_websocket.c
    ${CMAKE_CURRENT_SOURCE_DIR}/../src/database/db_core.c
    ${CMAKE_CURRENT_SOURCE_DIR}/../src/database/db_streams.c
    ${CMAKE_CURRENT_SOURCE_DIR}/../src/database/db_schema.c
    ${CMAKE_CURRENT_SOURCE_DIR}/../external/inih/ini.c
    ${CMAKE_CURRENT_SOURCE_DIR}/../external/cjson/cJSON.c
    ${CMAKE_CURRENT_SOURCE_DIR}/../external/mongoose/mongoose.c
)

# Add SOD definition if enabled
if(ENABLE_SOD)
    add_definitions(-DSOD_ENABLED)
endif()

# Only build SOD tests if SOD is enabled
if(ENABLE_SOD)
    # Define test executables
    add_executable(test_sod_unified
        test_sod_unified.c
        ${DETECTION_SOURCES}
    )

    add_executable(test_sod_voc
        test_sod_voc.c
        ${DETECTION_SOURCES}
    )

    # Link libraries
    target_link_libraries(test_sod_unified
        ${FFMPEG_LIBRARIES}
        ${SQLITE_LIBRARIES}
        pthread
        dl
        sod
        sqlite3
    )

    target_link_libraries(test_sod_voc
        ${FFMPEG_LIBRARIES}
        ${SQLITE_LIBRARIES}
        pthread
        dl
        sod
        sqlite3
    )

    # Set output directory for test binaries
    set_target_properties(test_sod_unified test_sod_voc
        PROPERTIES
        RUNTIME_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/bin"
    )

    # Add tests to CTest
    add_test(NAME test_sod_unified COMMAND test_sod_unified)
    add_test(NAME test_sod_voc COMMAND test_sod_voc)

    message(STATUS "Building SOD tests")
else()
    message(STATUS "Skipping SOD tests (SOD disabled)")
endif()

message(STATUS "Building motion detection optimization tests")

