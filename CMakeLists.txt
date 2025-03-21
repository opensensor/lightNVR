cmake_minimum_required(VERSION 3.10)
project(LightNVR VERSION 0.6.0 LANGUAGES C CXX)

# Set C/C++ standards
set(CMAKE_C_STANDARD 11)
set(CMAKE_CXX_STANDARD 14)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# Option to enable/disable SOD
option(ENABLE_SOD "Enable SOD library for object detection" ON)

# Compiler flags for optimization and memory usage
set(CMAKE_C_FLAGS "${CMAKE_C_FLAGS} -O2 -ffunction-sections -fdata-sections -Wl,--gc-sections -pthread")
set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -O2 -ffunction-sections -fdata-sections -Wl,--gc-sections -pthread")

# Option to build for embedded A1 device
option(EMBEDDED_A1_DEVICE "Build for embedded A1 device with limited memory" OFF)
if(EMBEDDED_A1_DEVICE)
    message(STATUS "Building for embedded A1 device with memory optimizations")
    add_definitions(-DEMBEDDED_A1_DEVICE)
    # Additional optimizations for embedded devices
    set(CMAKE_C_FLAGS "${CMAKE_C_FLAGS} -Os -fno-exceptions -fomit-frame-pointer")
    set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -Os -fno-exceptions -fomit-frame-pointer")
endif()

# Include directories
include_directories(
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_CURRENT_SOURCE_DIR}/include
)

# Find required packages
find_package(PkgConfig REQUIRED)
pkg_check_modules(FFMPEG REQUIRED libavcodec libavformat libavutil libswscale)
pkg_check_modules(SQLITE REQUIRED sqlite3)

# Use Mongoose for web server implementation
set(USE_MONGOOSE ON)
add_definitions(-DUSE_MONGOOSE)

# Find cJSON
find_package(cJSON QUIET)
if(NOT cJSON_FOUND)
    # If not found via find_package, try pkg-config
    pkg_check_modules(CJSON cjson)
    if(NOT CJSON_FOUND)
        message(STATUS "cJSON not found via find_package or pkg-config, will use bundled version")
        set(CJSON_INCLUDE_DIRS "${CMAKE_CURRENT_SOURCE_DIR}/external/cjson")
        set(CJSON_LIBRARIES "cjson")
        set(CJSON_BUNDLED TRUE)
    endif()
endif()

# Add SOD subdirectory if enabled
if(ENABLE_SOD)
    add_subdirectory(src/sod)
    add_definitions(-DSOD_ENABLED)
endif()

# Add Mongoose if enabled
if(USE_MONGOOSE)
    # Check if Mongoose is already in external directory, if not download it
    if(NOT EXISTS "${CMAKE_CURRENT_SOURCE_DIR}/external/mongoose/mongoose.c")
        message(STATUS "Mongoose not found, will download it")
        file(MAKE_DIRECTORY "${CMAKE_CURRENT_SOURCE_DIR}/external/mongoose")
        file(DOWNLOAD 
            "https://raw.githubusercontent.com/cesanta/mongoose/master/mongoose.c"
            "${CMAKE_CURRENT_SOURCE_DIR}/external/mongoose/mongoose.c"
            SHOW_PROGRESS
        )
        file(DOWNLOAD 
            "https://raw.githubusercontent.com/cesanta/mongoose/master/mongoose.h"
            "${CMAKE_CURRENT_SOURCE_DIR}/external/mongoose/mongoose.h"
            SHOW_PROGRESS
        )
    endif()
    add_definitions(-DUSE_MONGOOSE)
    set(MONGOOSE_INCLUDE_DIRS "${CMAKE_CURRENT_SOURCE_DIR}/external/mongoose")
    set(MONGOOSE_SOURCES "${CMAKE_CURRENT_SOURCE_DIR}/external/mongoose/mongoose.c")
endif()

# Add cJSON if using bundled version
if(CJSON_BUNDLED)
    # Check if cJSON is already in external directory, if not download it
    if(NOT EXISTS "${CMAKE_CURRENT_SOURCE_DIR}/external/cjson/cJSON.c")
        message(STATUS "cJSON not found, will download it")
        file(MAKE_DIRECTORY "${CMAKE_CURRENT_SOURCE_DIR}/external/cjson")
        file(DOWNLOAD 
            "https://raw.githubusercontent.com/DaveGamble/cJSON/master/cJSON.c"
            "${CMAKE_CURRENT_SOURCE_DIR}/external/cjson/cJSON.c"
            SHOW_PROGRESS
        )
        file(DOWNLOAD 
            "https://raw.githubusercontent.com/DaveGamble/cJSON/master/cJSON.h"
            "${CMAKE_CURRENT_SOURCE_DIR}/external/cjson/cJSON.h"
            SHOW_PROGRESS
        )
    endif()
    set(CJSON_SOURCES "${CMAKE_CURRENT_SOURCE_DIR}/external/cjson/cJSON.c")
endif()

# Add include directories for dependencies
include_directories(
    ${FFMPEG_INCLUDE_DIRS}
    ${SQLITE_INCLUDE_DIRS}
    ${CMAKE_CURRENT_SOURCE_DIR}/external/inih
    ${CJSON_INCLUDE_DIRS}
)

# Add Mongoose include directory if enabled
if(USE_MONGOOSE)
    include_directories(${MONGOOSE_INCLUDE_DIRS})
endif()

# No libmicrohttpd needed

# Define source files, excluding SOD sources which are built separately
file(GLOB_RECURSE CORE_SOURCES "src/core/*.c")
file(GLOB_RECURSE DATABASE_SOURCES "src/database/*.c")
file(GLOB_RECURSE STORAGE_SOURCES "src/storage/*.c")
file(GLOB_RECURSE UTILS_SOURCES "src/utils/*.c")
file(GLOB_RECURSE WEB_SOURCES "src/web/*.c")
file(GLOB_RECURSE ROOT_SOURCES "src/*.c")

# Explicitly list video sources to exclude motion_detection_optimized.c
file(GLOB VIDEO_SOURCES "src/video/*.c")
list(FILTER VIDEO_SOURCES EXCLUDE REGEX ".*motion_detection_optimized\\.c$")

# We've replaced mongoose_server.c with our direct implementation

# Combine all sources
set(SOURCES
    ${CORE_SOURCES}
    ${DATABASE_SOURCES}
    ${STORAGE_SOURCES}
    ${UTILS_SOURCES}
    ${VIDEO_SOURCES}
    ${WEB_SOURCES}
    ${ROOT_SOURCES}
    "external/inih/ini.c"
)

# Add Mongoose source if enabled
if(USE_MONGOOSE)
    list(APPEND SOURCES ${MONGOOSE_SOURCES})
endif()

# Add cJSON source if using bundled version
if(CJSON_BUNDLED)
    list(APPEND SOURCES ${CJSON_SOURCES})
endif()

# Define the executable
add_executable(lightnvr ${SOURCES})

# Link libraries
target_link_libraries(lightnvr
    ${FFMPEG_LIBRARIES}
    ${SQLITE_LIBRARIES}
    pthread
    dl
    m
)

# Link cJSON if not using bundled version
if(NOT CJSON_BUNDLED AND CJSON_FOUND)
    target_link_libraries(lightnvr ${CJSON_LIBRARIES})
endif()

# Link SOD library if enabled
if(ENABLE_SOD)
    target_link_libraries(lightnvr sod)
endif()

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
message(STATUS "Building LightNVR for memory-constrained environment...")
message(STATUS "Ensure all dependencies are optimized for low memory usage")
