cmake_minimum_required(VERSION 3.10)
project(LightNVR VERSION 0.10.23 LANGUAGES C CXX)

# Set C/C++ standards
set(CMAKE_C_STANDARD 11)
set(CMAKE_CXX_STANDARD 14)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# Option to enable/disable SOD
option(ENABLE_SOD "Enable SOD library for object detection" ON)
option(SOD_DYNAMIC_LINK "Dynamically link SOD library instead of static linking" OFF)

# go2rtc integration options
option(ENABLE_GO2RTC "Enable go2rtc integration for WebRTC streaming" ON)
option(GO2RTC_BINARY_PATH "Path to go2rtc binary" "/usr/local/bin/go2rtc")
option(GO2RTC_CONFIG_DIR "Directory for go2rtc configuration files" "/etc/lightnvr/go2rtc")
option(GO2RTC_API_PORT "Port for go2rtc HTTP API" 1984)

# SSL/TLS options
option(ENABLE_SSL "Enable SSL/TLS support" OFF)
option(USE_MBEDTLS "Use mbedTLS instead of OpenSSL (if SSL is enabled)" OFF)
option(USE_WOLFSSL "Use WolfSSL instead of OpenSSL (if SSL is enabled)" OFF)

# Define TLS constants (from mongoose.h)
set(MG_TLS_NONE 0)
set(MG_TLS_MBED 1)
set(MG_TLS_OPENSSL 2)
set(MG_TLS_BUILTIN 3)
set(MG_TLS_WOLFSSL 5)

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
pkg_check_modules(CURL REQUIRED libcurl)

# SSL/TLS configuration for Mongoose
if(ENABLE_SSL)
    if(USE_MBEDTLS)
        add_definitions(-DMG_TLS=${MG_TLS_MBED})
        pkg_check_modules(MBEDTLS REQUIRED mbedtls mbedcrypto mbedx509)
        set(SSL_INCLUDE_DIRS ${MBEDTLS_INCLUDE_DIRS})
        set(SSL_LIBRARIES ${MBEDTLS_LIBRARIES})
        message(STATUS "Using mbedTLS for SSL/TLS support")
    elseif(USE_WOLFSSL)
        add_definitions(-DMG_TLS=${MG_TLS_WOLFSSL})
        pkg_check_modules(WOLFSSL REQUIRED wolfssl)
        set(SSL_INCLUDE_DIRS ${WOLFSSL_INCLUDE_DIRS})
        set(SSL_LIBRARIES ${WOLFSSL_LIBRARIES})
        message(STATUS "Using WolfSSL for SSL/TLS support")
    else()
        add_definitions(-DMG_TLS=${MG_TLS_OPENSSL})
        pkg_check_modules(OPENSSL REQUIRED openssl)
        set(SSL_INCLUDE_DIRS ${OPENSSL_INCLUDE_DIRS})
        set(SSL_LIBRARIES ${OPENSSL_LIBRARIES})
        message(STATUS "Using OpenSSL for SSL/TLS support")
    endif()
else()
    add_definitions(-DMG_TLS=${MG_TLS_NONE})
    message(STATUS "SSL/TLS support is disabled for Mongoose")
    # Define empty variables for SSL
    set(SSL_INCLUDE_DIRS "")
    set(SSL_LIBRARIES "")
endif()

# ONVIF and authentication system require mbedTLS for cryptographic functions
pkg_check_modules(MBEDTLS REQUIRED mbedtls mbedcrypto mbedx509)
list(APPEND SSL_INCLUDE_DIRS ${MBEDTLS_INCLUDE_DIRS})
list(APPEND SSL_LIBRARIES ${MBEDTLS_LIBRARIES})
message(STATUS "ONVIF and authentication system enabled, linking with mbedTLS for cryptographic functions")

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
    # Add the SOD subdirectory regardless of linking method
    # The CMakeLists.txt in the SOD directory will handle
    # building as static or shared based on SOD_DYNAMIC_LINK
    add_subdirectory(src/sod)

    # Add the SOD_ENABLED define regardless of linking method
    add_definitions(-DSOD_ENABLED)

    # Always set the include directory
    set(SOD_INCLUDE_DIR "${CMAKE_CURRENT_SOURCE_DIR}/src/sod")

    # If dynamic linking is enabled, add the define
    if(SOD_DYNAMIC_LINK)
        add_definitions(-DSOD_DYNAMIC_LINK)
    endif()
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
        ${CURL_INCLUDE_DIRS}
        ${SSL_INCLUDE_DIRS}
        ${EZXML_INCLUDE_DIR}
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
# Exclude rebuild_recordings.c from UTILS_SOURCES to avoid multiple main functions
list(FILTER UTILS_SOURCES EXCLUDE REGEX ".*rebuild_recordings\\.c$")
message(STATUS "Excluding rebuild_recordings.c from main executable")
file(GLOB_RECURSE WEB_SOURCES "src/web/*.c")
file(GLOB_RECURSE ROOT_SOURCES "src/*.c")
# Exclude sod.c and rebuild_recordings.c from ROOT_SOURCES to avoid static linking and multiple main functions
list(FILTER ROOT_SOURCES EXCLUDE REGEX ".*sod/sod\\.c$")
list(FILTER ROOT_SOURCES EXCLUDE REGEX ".*utils/rebuild_recordings\\.c$")
message(STATUS "Excluding rebuild_recordings.c from ROOT_SOURCES")

# Explicitly list video sources to exclude motion_detection_optimized.c, detection_thread_pool.c,
# and the original hls_writer_thread.c (since we're using our split version)
file(GLOB VIDEO_SOURCES "src/video/*.c")
list(FILTER VIDEO_SOURCES EXCLUDE REGEX ".*motion_detection_optimized\\.c$")
list(FILTER VIDEO_SOURCES EXCLUDE REGEX ".*detection_thread_pool\\.c$")
list(FILTER VIDEO_SOURCES EXCLUDE REGEX ".*hls_writer_thread\\.c$")

# Add HLS sources
file(GLOB HLS_SOURCES "src/video/hls/*.c")

# Add unified HLS thread sources explicitly to ensure they're included
set(HLS_UNIFIED_THREAD_SOURCES
    "src/video/hls/hls_unified_thread.c"
)

# Set up go2rtc integration if enabled
set(GO2RTC_SOURCES "")
if(ENABLE_GO2RTC)
    add_subdirectory(src/video/go2rtc)
    add_definitions(-DUSE_GO2RTC)
    add_definitions(-DGO2RTC_BINARY_PATH="${GO2RTC_BINARY_PATH}")
    add_definitions(-DGO2RTC_CONFIG_DIR="${GO2RTC_CONFIG_DIR}")
    add_definitions(-DGO2RTC_API_PORT=${GO2RTC_API_PORT})
    message(STATUS "go2rtc integration enabled with binary path: ${GO2RTC_BINARY_PATH}")
    message(STATUS "go2rtc config directory: ${GO2RTC_CONFIG_DIR}")
    message(STATUS "go2rtc API port: ${GO2RTC_API_PORT}")
endif()

# Combine all sources
set(SOURCES
        ${CORE_SOURCES}
        ${DATABASE_SOURCES}
        ${STORAGE_SOURCES}
        ${UTILS_SOURCES}
        ${VIDEO_SOURCES}
        ${HLS_SOURCES}
        ${HLS_UNIFIED_THREAD_SOURCES}
        ${WEB_SOURCES}
        ${ROOT_SOURCES}
        "${INIH_INCLUDE_DIR}/ini.c"
)

# Add go2rtc sources if enabled
if(ENABLE_GO2RTC)
    list(APPEND SOURCES ${GO2RTC_SOURCES})
endif()

# Add Mongoose source (required component)
list(APPEND SOURCES ${MONGOOSE_SOURCES})

# Add ezxml source
list(APPEND SOURCES ${EZXML_SOURCES})

# Add cJSON source if using bundled version
if(CJSON_BUNDLED)
    list(APPEND SOURCES ${CJSON_SOURCES})
endif()

# Define the main executable
add_executable(lightnvr ${SOURCES})

# Define source files for rebuild_recordings utility
set(REBUILD_RECORDINGS_SOURCES
    src/utils/rebuild_recordings.c
    src/core/config.c
    src/core/logger.c
    src/web/logger_websocket.c
    src/database/db_core.c
    src/database/db_streams.c
    src/database/db_recordings.c
    src/database/db_schema.c
    src/database/db_schema_cache.c
    src/database/db_backup.c
    src/database/db_transaction.c
    "${INIH_INCLUDE_DIR}/ini.c"
)

# Define the rebuild_recordings utility
add_executable(rebuild_recordings ${REBUILD_RECORDINGS_SOURCES})

# Set the output directory for the binaries
set_target_properties(lightnvr rebuild_recordings PROPERTIES
        RUNTIME_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/bin"
)

# Link libraries for main executable
target_link_libraries(lightnvr
        ${FFMPEG_LIBRARIES}
        ${SQLITE_LIBRARIES}
        ${CURL_LIBRARIES}
        ${SSL_LIBRARIES}
        atomic
        pthread
        dl
        m
)

# Link libraries for rebuild_recordings utility
target_link_libraries(rebuild_recordings
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
    # Always link to the sod target, whether it's built as static or shared
    target_link_libraries(lightnvr sod)

    # Log the linking method for clarity
    if(SOD_DYNAMIC_LINK)
        message(STATUS "Using dynamic linking for SOD library (built from source)")

        # Set proper RPATH settings for the main executable when using dynamic SOD
        set_target_properties(lightnvr PROPERTIES
                BUILD_WITH_INSTALL_RPATH TRUE
                INSTALL_RPATH "$ORIGIN/../src/sod:$ORIGIN/../lib"
        )

        # Make sure the shared library gets installed to lib directory
        install(TARGETS sod
                LIBRARY DESTINATION lib
                RUNTIME DESTINATION bin
        )

        # Copy the SOD shared library to a directory next to the executable
        add_custom_command(TARGET lightnvr POST_BUILD
                COMMAND ${CMAKE_COMMAND} -E make_directory "${CMAKE_BINARY_DIR}/lib"
                COMMAND ${CMAKE_COMMAND} -E copy
                "$<TARGET_FILE:sod>"
                "${CMAKE_BINARY_DIR}/lib/"
                COMMENT "Copying SOD shared library to lib directory"
        )

        # Additional direct RPATH via linker flags for older systems
        if(UNIX AND NOT APPLE)
            set_target_properties(lightnvr PROPERTIES
                    LINK_FLAGS "-Wl,-rpath,\$ORIGIN/../src/sod:\$ORIGIN/../lib"
            )
        endif()
    else()
        message(STATUS "Using static linking for SOD library")
    endif()
endif()

# Install targets
install(TARGETS lightnvr rebuild_recordings DESTINATION bin)
install(DIRECTORY config/ DESTINATION /etc/lightnvr)

# Add subdirectories for tests if testing is enabled
option(BUILD_TESTS "Build the test suite" OFF)
if(BUILD_TESTS)
    enable_testing()
    add_subdirectory(tests)
endif()

# Create a version.h file
configure_file(
        ${CMAKE_CURRENT_SOURCE_DIR}/include/core/version.h.in
        ${CMAKE_CURRENT_SOURCE_DIR}/include/core/version.h
)

# Generate version.js for web interface
add_custom_target(generate_version_js
        COMMAND ${CMAKE_CURRENT_SOURCE_DIR}/scripts/extract_version.sh
        COMMENT "Generating version.js for web interface"
        VERBATIM
)

# Make sure version.js is generated before building the web assets
add_dependencies(lightnvr generate_version_js)

# Print build information
message(STATUS "Building LightNVR ${PROJECT_VERSION} with the following configuration:")
message(STATUS "- SOD object detection: ${ENABLE_SOD}")
if(ENABLE_SOD)
    if(SOD_DYNAMIC_LINK)
        message(STATUS "  - SOD linking method: Dynamic")
    else()
        message(STATUS "  - SOD linking method: Static")
    endif()
endif()
message(STATUS "- go2rtc WebRTC integration: ${ENABLE_GO2RTC}")
if(ENABLE_GO2RTC)
    message(STATUS "  - go2rtc binary path: ${GO2RTC_BINARY_PATH}")
    message(STATUS "  - go2rtc config directory: ${GO2RTC_CONFIG_DIR}")
    message(STATUS "  - go2rtc API port: ${GO2RTC_API_PORT}")
endif()
message(STATUS "- Embedded A1 device optimizations: ${EMBEDDED_A1_DEVICE}")
message(STATUS "- Include directories:")
foreach(dir ${LIGHTNVR_INCLUDE_DIRS})
    message(STATUS "  * ${dir}")
endforeach()
