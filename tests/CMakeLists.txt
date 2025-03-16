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
pkg_check_modules(FFMPEG REQUIRED libavcodec libavformat libavutil)
pkg_check_modules(SQLITE REQUIRED sqlite3)

# Add include directories for dependencies
include_directories(
    ${FFMPEG_INCLUDE_DIRS}
    ${SQLITE_INCLUDE_DIRS}
    ${CMAKE_CURRENT_SOURCE_DIR}/../external/inih
)

# Define source files needed for tests
set(DETECTION_SOURCES
    ${CMAKE_CURRENT_SOURCE_DIR}/../src/video/detection.c
    ${CMAKE_CURRENT_SOURCE_DIR}/../src/video/sod_realnet.c
    ${CMAKE_CURRENT_SOURCE_DIR}/../src/video/motion_detection.c
    ${CMAKE_CURRENT_SOURCE_DIR}/../src/core/logger.c
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

    # Link libraries
    target_link_libraries(test_sod_unified
        ${FFMPEG_LIBRARIES}
        ${SQLITE_LIBRARIES}
        pthread
        dl
        sod
    )

    # Set output directory for test binaries
    set_target_properties(test_sod_unified
        PROPERTIES
        RUNTIME_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/bin"
    )

    # Add tests to CTest
    add_test(NAME test_sod_unified COMMAND test_sod_unified)

    message(STATUS "Building SOD tests")
else()
    message(STATUS "Skipping SOD tests (SOD disabled)")
endif()

# Output binary to a 'bin' directory
set(CMAKE_RUNTIME_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/bin)
