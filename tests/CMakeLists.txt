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
)

# Add SOD definition if enabled
if(ENABLE_SOD)
    add_definitions(-DSOD_ENABLED)
endif()

# Only build SOD tests if SOD is enabled
if(ENABLE_SOD)
    # Define test executables
    add_executable(test_sod_unified test_sod_unified.c)
    add_executable(test_sod_voc test_sod_voc.c)

    # Add BUILDING_TEST definition for test builds
    target_compile_definitions(test_sod_unified PRIVATE BUILDING_TEST)
    target_compile_definitions(test_sod_voc PRIVATE BUILDING_TEST)

    # Link libraries
    target_link_libraries(test_sod_unified
        lightnvr_lib
        ${FFMPEG_LIBRARIES}
        ${SQLITE_LIBRARIES}
        ${CURL_LIBRARIES}
        ${SSL_LIBRARIES}  # Add SSL libraries which include mbedcrypto
        pthread
        dl
        sod
        sqlite3
        curl
        mongoose_lib
        inih_lib
    )
    if(CJSON_BUNDLED)
        target_link_libraries(test_sod_unified cjson_lib)
    endif()

    target_link_libraries(test_sod_voc
        lightnvr_lib
        ${FFMPEG_LIBRARIES}
        ${SQLITE_LIBRARIES}
        ${CURL_LIBRARIES}
        ${SSL_LIBRARIES}  # Add SSL libraries which include mbedcrypto
        pthread
        dl
        sod
        sqlite3
        curl
        mongoose_lib
        inih_lib
    )
    if(CJSON_BUNDLED)
        target_link_libraries(test_sod_voc cjson_lib)
    endif()

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

# Add database backup test
add_executable(test_db_backup database/db_backup_test.c)

# Link libraries for database backup test
target_link_libraries(test_db_backup
    lightnvr_lib
    ${SQLITE_LIBRARIES}
    ${SSL_LIBRARIES}  # Add SSL libraries which include mbedcrypto
    pthread
    dl
    mongoose_lib
    inih_lib
)

# Set output directory for database backup test
set_target_properties(test_db_backup
    PROPERTIES
    RUNTIME_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/bin"
)

# Add database backup test to CTest
add_test(NAME test_db_backup COMMAND test_db_backup)

# Add stream detection test
add_executable(test_stream_detection test_stream_detection.c)

# Add BUILDING_TEST definition for test builds
target_compile_definitions(test_stream_detection PRIVATE BUILDING_TEST)

# Link libraries for stream detection test
target_link_libraries(test_stream_detection
    lightnvr_lib
    ${FFMPEG_LIBRARIES}
    ${SQLITE_LIBRARIES}
    ${CURL_LIBRARIES}
    ${SSL_LIBRARIES}  # Add SSL libraries which include mbedcrypto
    pthread
    dl
    sqlite3
    curl
    mongoose_lib
    inih_lib
)
if(CJSON_BUNDLED)
    target_link_libraries(test_stream_detection cjson_lib)
endif()

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
