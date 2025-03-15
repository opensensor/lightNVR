# Tests CMakeLists.txt

# Include directories
include_directories(
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_CURRENT_SOURCE_DIR}/../include
)

# Find required packages
find_package(PkgConfig REQUIRED)
pkg_check_modules(FFMPEG REQUIRED libavcodec libavformat libavutil)
pkg_check_modules(SQLITE REQUIRED sqlite3)

# Find SOD library
find_library(SOD_LIBRARY NAMES sod REQUIRED)
message(STATUS "SOD library found for tests: ${SOD_LIBRARY}")

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
    ${CMAKE_CURRENT_SOURCE_DIR}/../src/core/logger.c
)

# Define test executables
add_executable(test_sod_realnet 
    test_sod_realnet.c
    ${DETECTION_SOURCES}
)

# Link libraries
target_link_libraries(test_sod_realnet
    ${FFMPEG_LIBRARIES}
    ${SQLITE_LIBRARIES}
    ${SOD_LIBRARY}
    pthread
    dl
)

# Add test to CTest
add_test(NAME test_sod_realnet COMMAND test_sod_realnet)

# Output binary to a 'bin' directory
set(CMAKE_RUNTIME_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/bin)
