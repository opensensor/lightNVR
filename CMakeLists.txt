cmake_minimum_required(VERSION 3.10)
project(LightNVR VERSION 0.8.2 LANGUAGES C CXX)

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

# Create project-specific include directory
set(LIGHTNVR_INCLUDE_DIR ${CMAKE_CURRENT_SOURCE_DIR}/include)

# Find required packages
find_package(PkgConfig REQUIRED)
pkg_check_modules(FFMPEG REQUIRED libavcodec libavformat libavutil libswscale)
pkg_check_modules(SQLITE REQUIRED sqlite3)

# Mongoose is required for web server implementation
add_definitions(-DUSE_MONGOOSE)

# Find cJSON
find_package(cJSON QUIET)
if(NOT cJSON_FOUND)
    # If not found via find_package, try pkg-config
    pkg_check_modules(CJSON cjson)
    if(NOT CJSON_FOUND)
        message(STATUS "cJSON not found via find_package or pkg-config, will use bundled version")
        set(CJSON_INCLUDE_DIR "${CMAKE_CURRENT_SOURCE_DIR}/external/cjson")
        set(CJSON_LIBRARIES "cjson")
        set(CJSON_BUNDLED TRUE)
    endif()
endif()

# Set up SOD library if enabled
if(ENABLE_SOD)
    add_subdirectory(src/sod)
    add_definitions(-DSOD_ENABLED)
    set(SOD_INCLUDE_DIR "${CMAKE_CURRENT_SOURCE_DIR}/src/sod")
endif()

# Set up Mongoose (required component)
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
set(MONGOOSE_INCLUDE_DIR "${CMAKE_CURRENT_SOURCE_DIR}/external/mongoose")
set(MONGOOSE_SOURCES "${CMAKE_CURRENT_SOURCE_DIR}/external/mongoose/mongoose.c")

# Set up cJSON if using bundled version
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

# Set up inih library
set(INIH_INCLUDE_DIR "${CMAKE_CURRENT_SOURCE_DIR}/external/inih")

# Define all include directories in one place
set(LIGHTNVR_INCLUDE_DIRS
        ${LIGHTNVR_INCLUDE_DIR}
        ${CMAKE_CURRENT_SOURCE_DIR}
        ${FFMPEG_INCLUDE_DIRS}
        ${SQLITE_INCLUDE_DIRS}
        ${INIH_INCLUDE_DIR}
)

# Add conditional include directories
if(ENABLE_SOD)
    list(APPEND LIGHTNVR_INCLUDE_DIRS ${SOD_INCLUDE_DIR})
endif()

# Mongoose is required, always include it
list(APPEND LIGHTNVR_INCLUDE_DIRS ${MONGOOSE_INCLUDE_DIR})

if(CJSON_FOUND AND NOT CJSON_BUNDLED)
    list(APPEND LIGHTNVR_INCLUDE_DIRS ${CJSON_INCLUDE_DIRS})
else()
    list(APPEND LIGHTNVR_INCLUDE_DIRS ${CJSON_INCLUDE_DIR})
endif()

# Apply include directories
include_directories(${LIGHTNVR_INCLUDE_DIRS})

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

# Combine all sources
set(SOURCES
        ${CORE_SOURCES}
        ${DATABASE_SOURCES}
        ${STORAGE_SOURCES}
        ${UTILS_SOURCES}
        ${VIDEO_SOURCES}
        ${WEB_SOURCES}
        ${ROOT_SOURCES}
        "${INIH_INCLUDE_DIR}/ini.c"
)

# Add Mongoose source (required component)
list(APPEND SOURCES ${MONGOOSE_SOURCES})

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

# Print build information
message(STATUS "Building LightNVR ${PROJECT_VERSION} with the following configuration:")
message(STATUS "- SOD object detection: ${ENABLE_SOD}")
message(STATUS "- Embedded A1 device optimizations: ${EMBEDDED_A1_DEVICE}")
message(STATUS "- Include directories:")
foreach(dir ${LIGHTNVR_INCLUDE_DIRS})
    message(STATUS "  * ${dir}")
endforeach()
message(STATUS "Ensure all dependencies are optimized for low memory usage")
