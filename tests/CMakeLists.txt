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
pkg_check_modules(CURL REQUIRED libcurl)

# Add include directories for dependencies
include_directories(
    ${FFMPEG_INCLUDE_DIRS}
    ${SQLITE_INCLUDE_DIRS}
    ${CURL_INCLUDE_DIRS}
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

# Define database backup test sources
set(DB_BACKUP_SOURCES
    ${CMAKE_CURRENT_SOURCE_DIR}/../src/core/logger.c
    ${CMAKE_CURRENT_SOURCE_DIR}/../src/web/logger_websocket.c
    ${CMAKE_CURRENT_SOURCE_DIR}/../src/database/db_core.c
    ${CMAKE_CURRENT_SOURCE_DIR}/../src/database/db_backup.c
    ${CMAKE_CURRENT_SOURCE_DIR}/../src/database/db_schema.c
    ${CMAKE_CURRENT_SOURCE_DIR}/../src/database/db_transaction.c
    ${CMAKE_CURRENT_SOURCE_DIR}/../external/inih/ini.c
    ${CMAKE_CURRENT_SOURCE_DIR}/../external/mongoose/mongoose.c
)

# Add database backup test
add_executable(test_db_backup
    database/db_backup_test.c
    ${DB_BACKUP_SOURCES}
)

# Link libraries for database backup test
target_link_libraries(test_db_backup
    ${SQLITE_LIBRARIES}
    pthread
    dl
)

# Set output directory for database backup test
set_target_properties(test_db_backup
    PROPERTIES
    RUNTIME_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/bin"
)

# Add database backup test to CTest
add_test(NAME test_db_backup COMMAND test_db_backup)

# Define stream detection test sources
set(STREAM_DETECTION_SOURCES
    ${CMAKE_CURRENT_SOURCE_DIR}/../src/video/detection_stream_thread.c
    ${CMAKE_CURRENT_SOURCE_DIR}/../src/video/detection_model.c
    ${CMAKE_CURRENT_SOURCE_DIR}/../src/video/sod_detection.c
    ${CMAKE_CURRENT_SOURCE_DIR}/../src/video/sod_realnet.c
    ${CMAKE_CURRENT_SOURCE_DIR}/../src/video/sod_integration.c
    ${CMAKE_CURRENT_SOURCE_DIR}/../src/video/detection.c
    ${CMAKE_CURRENT_SOURCE_DIR}/../src/video/detection_embedded.c
    ${CMAKE_CURRENT_SOURCE_DIR}/../src/video/detection_frame_processing.c
    ${CMAKE_CURRENT_SOURCE_DIR}/../src/video/detection_recording.c
    ${CMAKE_CURRENT_SOURCE_DIR}/../src/video/detection_config.c
    ${CMAKE_CURRENT_SOURCE_DIR}/../src/video/detection_integration.c
    ${CMAKE_CURRENT_SOURCE_DIR}/../src/video/detection_buffer.c
    ${CMAKE_CURRENT_SOURCE_DIR}/../src/video/motion_detection.c
    ${CMAKE_CURRENT_SOURCE_DIR}/../src/video/stream_manager.c
    ${CMAKE_CURRENT_SOURCE_DIR}/../src/video/streams.c
    ${CMAKE_CURRENT_SOURCE_DIR}/../src/video/detection_stream.c
    ${CMAKE_CURRENT_SOURCE_DIR}/../src/video/recording.c
    ${CMAKE_CURRENT_SOURCE_DIR}/../src/video/mp4_recording_core.c
    ${CMAKE_CURRENT_SOURCE_DIR}/../src/video/mp4_recording_writer.c
    ${CMAKE_CURRENT_SOURCE_DIR}/../src/video/mp4_writer_core.c
    ${CMAKE_CURRENT_SOURCE_DIR}/../src/video/mp4_writer_utils.c
    ${CMAKE_CURRENT_SOURCE_DIR}/../src/video/mp4_writer_thread.c
    ${CMAKE_CURRENT_SOURCE_DIR}/../src/video/mp4_segment_recorder.c
    ${CMAKE_CURRENT_SOURCE_DIR}/../src/video/mp4_writer.c
    ${CMAKE_CURRENT_SOURCE_DIR}/../src/video/thread_utils.c
    ${CMAKE_CURRENT_SOURCE_DIR}/../src/video/hls_writer.c
    ${CMAKE_CURRENT_SOURCE_DIR}/../src/video/hls/hls_context.c
    ${CMAKE_CURRENT_SOURCE_DIR}/../src/video/hls/hls_directory.c
    ${CMAKE_CURRENT_SOURCE_DIR}/../src/video/hls/hls_unified_thread.c
    ${CMAKE_CURRENT_SOURCE_DIR}/../src/video/hls/hls_api.c
    ${CMAKE_CURRENT_SOURCE_DIR}/../src/video/timestamp_manager.c
    ${CMAKE_CURRENT_SOURCE_DIR}/../src/video/stream_protocol.c
    ${CMAKE_CURRENT_SOURCE_DIR}/../src/video/stream_state.c
    ${CMAKE_CURRENT_SOURCE_DIR}/../src/video/ffmpeg_utils.c
    ${CMAKE_CURRENT_SOURCE_DIR}/../src/video/timeout_utils.c
    ${CMAKE_CURRENT_SOURCE_DIR}/../src/video/go2rtc/go2rtc_integration.c
    ${CMAKE_CURRENT_SOURCE_DIR}/../src/video/go2rtc/go2rtc_consumer.c
    ${CMAKE_CURRENT_SOURCE_DIR}/../src/video/go2rtc/go2rtc_api.c
    ${CMAKE_CURRENT_SOURCE_DIR}/../src/video/go2rtc/go2rtc_stream.c
    ${CMAKE_CURRENT_SOURCE_DIR}/../src/video/go2rtc/go2rtc_process.c
    ${CMAKE_CURRENT_SOURCE_DIR}/../src/database/db_core.c
    ${CMAKE_CURRENT_SOURCE_DIR}/../src/database/db_detections.c
    ${CMAKE_CURRENT_SOURCE_DIR}/../src/database/db_streams.c
    ${CMAKE_CURRENT_SOURCE_DIR}/../src/database/db_schema.c
    ${CMAKE_CURRENT_SOURCE_DIR}/../src/database/db_schema_cache.c
    ${CMAKE_CURRENT_SOURCE_DIR}/../src/database/db_recordings.c
    ${CMAKE_CURRENT_SOURCE_DIR}/../src/web/api_handlers_detection_results.c
    ${CMAKE_CURRENT_SOURCE_DIR}/../src/core/logger.c
    ${CMAKE_CURRENT_SOURCE_DIR}/../src/core/config.c
    ${CMAKE_CURRENT_SOURCE_DIR}/../src/core/shutdown_coordinator.c
    ${CMAKE_CURRENT_SOURCE_DIR}/../src/web/logger_websocket.c
    ${CMAKE_CURRENT_SOURCE_DIR}/../src/utils/memory.c
    ${CMAKE_CURRENT_SOURCE_DIR}/../external/inih/ini.c
    ${CMAKE_CURRENT_SOURCE_DIR}/../external/cjson/cJSON.c
)

# Add stream detection test
add_executable(test_stream_detection
    test_stream_detection.c
    ${STREAM_DETECTION_SOURCES}
)

# Link libraries for stream detection test
target_link_libraries(test_stream_detection
    ${FFMPEG_LIBRARIES}
    ${SQLITE_LIBRARIES}
    ${CURL_LIBRARIES}
    pthread
    dl
    sqlite3
    curl
)

if(ENABLE_SOD)
    target_link_libraries(test_stream_detection sod)
endif()

# Set output directory for stream detection test
set_target_properties(test_stream_detection
    PROPERTIES
    RUNTIME_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/bin"
)

# Add stream detection test to CTest
add_test(NAME test_stream_detection COMMAND test_stream_detection)

message(STATUS "Building motion detection optimization tests")
message(STATUS "Building database backup tests")
message(STATUS "Building stream detection tests")
