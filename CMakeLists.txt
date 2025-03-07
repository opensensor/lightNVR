cmake_minimum_required(VERSION 3.10)
project(LightNVR VERSION 0.1.0 LANGUAGES C CXX)

# Set C/C++ standards
set(CMAKE_C_STANDARD 11)
set(CMAKE_CXX_STANDARD 14)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# Compiler flags for optimization and memory usage
set(CMAKE_C_FLAGS "${CMAKE_C_FLAGS} -O2 -ffunction-sections -fdata-sections -Wl,--gc-sections")
set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -O2 -ffunction-sections -fdata-sections -Wl,--gc-sections")

# Include directories
include_directories(
    ${CMAKE_CURRENT_SOURCE_DIR}/include
    ${CMAKE_CURRENT_SOURCE_DIR}/include/core
    ${CMAKE_CURRENT_SOURCE_DIR}/include/video
    ${CMAKE_CURRENT_SOURCE_DIR}/include/storage
    ${CMAKE_CURRENT_SOURCE_DIR}/include/web
    ${CMAKE_CURRENT_SOURCE_DIR}/include/database
    ${CMAKE_CURRENT_SOURCE_DIR}/include/utils
)

# Find required packages
find_package(PkgConfig REQUIRED)
pkg_check_modules(FFMPEG REQUIRED libavcodec libavformat libavutil libswscale)
pkg_check_modules(SQLITE REQUIRED sqlite3)

# Add include directories for dependencies
include_directories(
    ${FFMPEG_INCLUDE_DIRS}
    ${SQLITE_INCLUDE_DIRS}
)

# Define source files
file(GLOB_RECURSE SOURCES 
    "src/*.c"
    "src/*.cpp"
)

# Define the executable
add_executable(lightnvr ${SOURCES})

# Link libraries
target_link_libraries(lightnvr
    ${FFMPEG_LIBRARIES}
    ${SQLITE_LIBRARIES}
    pthread
    dl
)

# Install targets
install(TARGETS lightnvr DESTINATION bin)
install(DIRECTORY config/ DESTINATION /etc/lightnvr)

# Add subdirectories for tests if testing is enabled
option(BUILD_TESTS "Build the test suite" OFF)
if(BUILD_TESTS)
    enable_testing()
    add_subdirectory(tests)
endif()

# Output binary to a 'bin' directory
set(CMAKE_RUNTIME_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/bin)

# Create a version.h file
configure_file(
    ${CMAKE_CURRENT_SOURCE_DIR}/include/core/version.h.in
    ${CMAKE_CURRENT_SOURCE_DIR}/include/core/version.h
)

# Print memory usage warning
message(STATUS "Building LightNVR for memory-constrained environment (256MB RAM)")
message(STATUS "Ensure all dependencies are optimized for low memory usage")
