cmake_minimum_required(VERSION 3.10)
project(LightNVR VERSION 0.34.7 LANGUAGES C CXX)

# Set C/C++ standards
set(CMAKE_C_STANDARD 11)
set(CMAKE_CXX_STANDARD 14)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# Option to enable/disable SOD
option(ENABLE_SOD "Enable SOD library for object detection" ON)
option(SOD_DYNAMIC_LINK "Dynamically link SOD library instead of static linking" OFF)

# go2rtc integration options
option(ENABLE_GO2RTC "Enable go2rtc integration for WebRTC streaming" ON)
set(GO2RTC_BINARY_PATH "/usr/local/bin/go2rtc" CACHE STRING "Path to go2rtc binary")
set(GO2RTC_CONFIG_DIR "/etc/lightnvr/go2rtc" CACHE STRING "Directory for go2rtc configuration files")
set(GO2RTC_API_PORT 1984 CACHE STRING "Port for go2rtc HTTP API")

# llhttp download configuration. Avoid the LLHTTP_VERSION name because
# pkg_check_modules(LLHTTP ...) owns that variable when libllhttp is probed.
set(LLHTTP_DOWNLOAD_REF "release/v9.2.1" CACHE STRING "llhttp tag/ref to download from GitHub")

# SSL/TLS options
option(ENABLE_SSL "Enable SSL/TLS support" OFF)
option(USE_MBEDTLS "Use mbedTLS instead of OpenSSL (if SSL is enabled)" OFF)
option(USE_WOLFSSL "Use WolfSSL instead of OpenSSL (if SSL is enabled)" OFF)

# HTTP backend: libuv + llhttp
set(HTTP_BACKEND "libuv" CACHE STRING "HTTP server backend")
set_property(CACHE HTTP_BACKEND PROPERTY STRINGS libuv)

# Compiler flags for optimization and memory usage
add_compile_options(-O2 -ffunction-sections -fdata-sections)
add_link_options(-Wl,--gc-sections)

# Security and quality warning flags
add_compile_options(
    -Wall
    -Wextra
    -Wformat=2
    -Wshadow
    -Werror=format-security
    -Werror=implicit-function-declaration
)

# Export compile commands for clang-tidy and other static analysis tools
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

# Add ASan/UBSan flags for Debug builds with GCC/Clang
if(CMAKE_BUILD_TYPE STREQUAL "Debug")
    if(CMAKE_C_COMPILER_ID MATCHES "GNU|Clang" AND CMAKE_CXX_COMPILER_ID MATCHES "GNU|Clang")
        message(STATUS "Enabling AddressSanitizer and UndefinedBehaviorSanitizer for Debug build")
        add_compile_options(-fsanitize=address,undefined)
        add_link_options(-fsanitize=address,undefined)
        add_compile_options(-ggdb3)

        # Improved debugging experience
        add_compile_options(-fno-omit-frame-pointer -fno-optimize-sibling-calls)
        add_compile_options(-fno-inline -fno-inline-functions)
    else()
        message(STATUS "Compiler (${CMAKE_C_COMPILER_ID}/${CMAKE_CXX_COMPILER_ID}) does not support sanitizers or build type is not Debug, skipping sanitizer flags.")
    endif()
endif()

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
pkg_check_modules(FFMPEG REQUIRED
    libavcodec>=61
    libavformat>=61
    libavutil>=59
    libswscale>=8
    libswresample>=5)
pkg_check_modules(SQLITE REQUIRED sqlite3)
pkg_check_modules(CURL REQUIRED libcurl)
find_package(Threads REQUIRED)

# libyaml (optional-but-preferred) — used for go2rtc override validation
# When available, defines LIGHTNVR_HAVE_LIBYAML=1 so src/utils/yaml_validate.c
# compiles the real parser path; otherwise the stub path is compiled.
pkg_check_modules(YAML QUIET yaml-0.1)
if(YAML_FOUND)
    message(STATUS "Found libyaml: ${YAML_LIBRARIES} (version ${YAML_VERSION})")
    add_compile_definitions(LIGHTNVR_HAVE_LIBYAML=1)
else()
    message(WARNING "libyaml (yaml-0.1) not found. YAML validation will be disabled. Install libyaml-dev / yaml-dev to enable.")
endif()

# MQTT support (optional, uses libmosquitto)
option(ENABLE_MQTT "Enable MQTT support for detection event streaming" ON)
if(ENABLE_MQTT)
    pkg_check_modules(MOSQUITTO QUIET libmosquitto)
    if(MOSQUITTO_FOUND)
        message(STATUS "Found libmosquitto: ${MOSQUITTO_LIBRARIES}")
        add_definitions(-DENABLE_MQTT=1)
    else()
        message(WARNING "libmosquitto not found. MQTT support will be disabled. Install libmosquitto-dev to enable.")
        set(ENABLE_MQTT OFF)
    endif()
endif()

# Common mbedTLS detection logic (used both in SSL block and for ONVIF/auth)
macro(DETECT_MBEDTLS_ONCE)
    if(NOT MBEDTLS_DETECTION_DONE)
        pkg_check_modules(MBEDTLS QUIET mbedtls mbedcrypto mbedx509)
        if (NOT MBEDTLS_FOUND)
            find_path(MBEDTLS_INCLUDE_DIRS "mbedtls/version.h" PATH_SUFFIXES "mbedtls" DOC "mbedtls include directory")
            find_library(MBEDTLS_LIBRARY NAMES mbedtls REQUIRED)
            find_library(MBEDCRYPTO_LIBRARY NAMES mbedcrypto REQUIRED)
            find_library(MBEDX509_LIBRARY NAMES mbedx509 REQUIRED)
            set(MBEDTLS_LIBRARIES ${MBEDTLS_LIBRARY} ${MBEDCRYPTO_LIBRARY} ${MBEDX509_LIBRARY})
        endif()
        set(MBEDTLS_DETECTION_DONE TRUE CACHE INTERNAL "mbedTLS detection has been performed")
    endif()
endmacro()

# SSL/TLS configuration
if(ENABLE_SSL)
    if(USE_MBEDTLS)
        DETECT_MBEDTLS_ONCE()
        set(SSL_INCLUDE_DIRS ${MBEDTLS_INCLUDE_DIRS})
        set(SSL_LIBRARIES ${MBEDTLS_LIBRARIES})
        message(STATUS "Using mbedTLS for SSL/TLS support")
    elseif(USE_WOLFSSL)
        pkg_check_modules(WOLFSSL REQUIRED wolfssl)
        set(SSL_INCLUDE_DIRS ${WOLFSSL_INCLUDE_DIRS})
        set(SSL_LIBRARIES ${WOLFSSL_LIBRARIES})
        message(STATUS "Using WolfSSL for SSL/TLS support")
    else()
        pkg_check_modules(OPENSSL REQUIRED openssl)
        set(SSL_INCLUDE_DIRS ${OPENSSL_INCLUDE_DIRS})
        set(SSL_LIBRARIES ${OPENSSL_LIBRARIES})
        message(STATUS "Using OpenSSL for SSL/TLS support")
    endif()
else()
    message(STATUS "SSL/TLS support is disabled")
    # Define empty variables for SSL
    set(SSL_INCLUDE_DIRS "")
    set(SSL_LIBRARIES "")
endif()

# ONVIF and authentication system require mbedTLS for cryptographic functions
DETECT_MBEDTLS_ONCE()
list(APPEND SSL_INCLUDE_DIRS ${MBEDTLS_INCLUDE_DIRS})
list(APPEND SSL_LIBRARIES ${MBEDTLS_LIBRARIES})
message(STATUS "ONVIF and authentication system enabled, linking with mbedTLS for cryptographic functions")

# Explicitly add mbedcrypto to the link libraries
set(COMMON_MBEDTLS_LIB_PATHS
    /usr/lib
    /usr/lib/x86_64-linux-gnu
    /usr/lib/aarch64-linux-gnu
    /usr/lib/arm-linux-gnueabihf
    /lib
    /lib/x86_64-linux-gnu
    /lib/aarch64-linux-gnu
    /lib/arm-linux-gnueabihf
)

find_library(MBEDCRYPTO_LIBRARY mbedcrypto
    HINTS
    ${COMMON_MBEDTLS_LIB_PATHS}
)

if(MBEDCRYPTO_LIBRARY)
    message(STATUS "Found mbedcrypto library: ${MBEDCRYPTO_LIBRARY}")
    list(APPEND SSL_LIBRARIES ${MBEDCRYPTO_LIBRARY})
else()
    # Try to find by manually checking common locations
    foreach(LIB_PATH ${COMMON_MBEDTLS_LIB_PATHS})
        if(EXISTS "${LIB_PATH}/libmbedcrypto.so")
            set(MBEDCRYPTO_LIBRARY "${LIB_PATH}/libmbedcrypto.so")
            message(STATUS "Found mbedcrypto library through manual search: ${MBEDCRYPTO_LIBRARY}")
            list(APPEND SSL_LIBRARIES ${MBEDCRYPTO_LIBRARY})
            break()
        endif()
    endforeach()

    if(NOT MBEDCRYPTO_LIBRARY)
        message(FATAL_ERROR "mbedcrypto library not found. Please install libmbedtls-dev package.")
    endif()
endif()

# Find cJSON (required dependency)
pkg_check_modules(CJSON QUIET libcjson)
if(CJSON_FOUND)
    message(STATUS "Found cJSON via pkg-config: ${CJSON_LIBRARIES}")
    message(STATUS "cJSON include dirs: ${CJSON_INCLUDE_DIRS}")
else()
    # Try alternative pkg-config name
    pkg_check_modules(CJSON QUIET cjson)
    if(CJSON_FOUND)
        message(STATUS "Found cJSON via pkg-config (alternative): ${CJSON_LIBRARIES}")
    else()
        # Try find_package as last resort
        find_package(cJSON QUIET)
        if(cJSON_FOUND)
            set(CJSON_LIBRARIES cjson)
            set(CJSON_INCLUDE_DIRS ${cJSON_INCLUDE_DIRS})
            set(CJSON_FOUND TRUE)
            message(STATUS "Found cJSON via find_package")
        else()
            # Try to find the library and header manually
            find_library(CJSON_LIBRARY NAMES cjson)
            find_path(CJSON_INCLUDE_DIR NAMES cjson/cJSON.h)
            if(CJSON_LIBRARY AND CJSON_INCLUDE_DIR)
                set(CJSON_LIBRARIES ${CJSON_LIBRARY})
                set(CJSON_INCLUDE_DIRS ${CJSON_INCLUDE_DIR})
                set(CJSON_FOUND TRUE)
                message(STATUS "Found cJSON manually: ${CJSON_LIBRARY}")
            else()
                message(FATAL_ERROR "cJSON library not found. Please install libcjson-dev package.")
            endif()
        endif()
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

# HTTP Backend Configuration - libuv + llhttp only
message(STATUS "Using libuv + llhttp HTTP backend")
add_definitions(-DHTTP_BACKEND_LIBUV)

# Find libuv
pkg_check_modules(LIBUV REQUIRED libuv)

# llhttp - try to find via pkg-config first, otherwise download and build
pkg_check_modules(LLHTTP QUIET libllhttp)
if(NOT LLHTTP_FOUND)
    # Check if llhttp is in external directory, downloading any missing files.
    # A partial prior download can leave src/llhttp.c present while api.c/http.c
    # are absent, which would otherwise fail later with "No SOURCES given".
    set(LLHTTP_DIR "${CMAKE_CURRENT_SOURCE_DIR}/external/llhttp")
    set(LLHTTP_REQUIRED_FILES
        "src/llhttp.c"
        "src/api.c"
        "src/http.c"
        "include/llhttp.h"
    )
    set(LLHTTP_BASE_URL "https://raw.githubusercontent.com/nodejs/llhttp/refs/tags/${LLHTTP_DOWNLOAD_REF}")

    foreach(LLHTTP_FILE IN LISTS LLHTTP_REQUIRED_FILES)
        set(LLHTTP_LOCAL_PATH "${LLHTTP_DIR}/${LLHTTP_FILE}")
        set(LLHTTP_NEEDS_DOWNLOAD FALSE)
        set(LLHTTP_DOWNLOAD_REASON "not found")
        if(NOT EXISTS "${LLHTTP_LOCAL_PATH}")
            set(LLHTTP_NEEDS_DOWNLOAD TRUE)
        else()
            file(READ "${LLHTTP_LOCAL_PATH}" LLHTTP_FILE_PROBE LIMIT 1)
            if(LLHTTP_FILE_PROBE STREQUAL "")
                set(LLHTTP_NEEDS_DOWNLOAD TRUE)
                set(LLHTTP_DOWNLOAD_REASON "is empty")
            endif()
        endif()

        if(LLHTTP_NEEDS_DOWNLOAD)
            message(STATUS "llhttp ${LLHTTP_FILE} ${LLHTTP_DOWNLOAD_REASON}, downloading from GitHub...")
            get_filename_component(LLHTTP_LOCAL_DIR "${LLHTTP_LOCAL_PATH}" DIRECTORY)
            file(MAKE_DIRECTORY "${LLHTTP_LOCAL_DIR}")
            file(REMOVE "${LLHTTP_LOCAL_PATH}")

            file(DOWNLOAD
                "${LLHTTP_BASE_URL}/${LLHTTP_FILE}"
                "${LLHTTP_LOCAL_PATH}"
                SHOW_PROGRESS
                STATUS LLHTTP_DOWNLOAD_STATUS
            )
            list(GET LLHTTP_DOWNLOAD_STATUS 0 LLHTTP_STATUS_CODE)
            list(GET LLHTTP_DOWNLOAD_STATUS 1 LLHTTP_STATUS_MESSAGE)
            if(NOT LLHTTP_STATUS_CODE EQUAL 0)
                message(FATAL_ERROR
                    "Failed to download llhttp ${LLHTTP_FILE}: ${LLHTTP_STATUS_MESSAGE}")
            endif()
            file(READ "${LLHTTP_LOCAL_PATH}" LLHTTP_FILE_PROBE LIMIT 1)
            if(LLHTTP_FILE_PROBE STREQUAL "")
                message(FATAL_ERROR "Downloaded llhttp ${LLHTTP_FILE} is empty")
            endif()
        endif()
    endforeach()

    # Build llhttp from source
    set(LLHTTP_SOURCES
        "${LLHTTP_DIR}/src/llhttp.c"
        "${LLHTTP_DIR}/src/api.c"
        "${LLHTTP_DIR}/src/http.c"
    )
    add_library(llhttp_lib STATIC ${LLHTTP_SOURCES})
    target_include_directories(llhttp_lib PUBLIC "${LLHTTP_DIR}/include")
    set(HTTP_BACKEND_LIBS llhttp_lib ${LIBUV_LIBRARIES})
    set(LLHTTP_INCLUDE_DIRS "${LLHTTP_DIR}/include")
    message(STATUS "Building llhttp from source")
else()
    message(STATUS "Found llhttp via pkg-config: ${LLHTTP_LIBRARIES}")
    set(HTTP_BACKEND_LIBS ${LLHTTP_LIBRARIES} ${LIBUV_LIBRARIES})
endif()

set(HTTP_BACKEND_INCLUDE_DIRS ${LIBUV_INCLUDE_DIRS} ${LLHTTP_INCLUDE_DIRS})

# libuv server sources
set(LIBUV_SERVER_SOURCES
    "${CMAKE_CURRENT_SOURCE_DIR}/src/web/libuv_server.c"
    "${CMAKE_CURRENT_SOURCE_DIR}/src/web/libuv_connection.c"
    "${CMAKE_CURRENT_SOURCE_DIR}/src/web/libuv_response.c"
    "${CMAKE_CURRENT_SOURCE_DIR}/src/web/libuv_file_serve.c"
    "${CMAKE_CURRENT_SOURCE_DIR}/src/web/libuv_api_handlers.c"
    "${CMAKE_CURRENT_SOURCE_DIR}/src/web/thumbnail_thread.c"
    "${CMAKE_CURRENT_SOURCE_DIR}/src/web/go2rtc_proxy_thread.c"
)



# Set up inih library
set(INIH_SOURCES "${CMAKE_CURRENT_SOURCE_DIR}/external/inih/ini.c")
add_library(inih_lib STATIC ${INIH_SOURCES})
target_include_directories(inih_lib INTERFACE "${CMAKE_CURRENT_SOURCE_DIR}/external/inih")

# Bundle ezXML implementation used by ONVIF parsing
set(EZXML_SOURCES
    "${CMAKE_CURRENT_SOURCE_DIR}/src/ezxml/ezxml.c"
)

# Define all include directories in one place
set(LIGHTNVR_INCLUDE_DIRS
        ${LIGHTNVR_INCLUDE_DIR}
        ${CMAKE_CURRENT_SOURCE_DIR}
        ${FFMPEG_INCLUDE_DIRS}
        ${SQLITE_INCLUDE_DIRS}
        ${CURL_INCLUDE_DIRS}
        ${SSL_INCLUDE_DIRS}
)

# Add ezXML include directory if provided
if(DEFINED EZXML_INCLUDE_DIR)
    list(APPEND LIGHTNVR_INCLUDE_DIRS ${EZXML_INCLUDE_DIR})
endif()

# Add conditional include directories
if(ENABLE_SOD)
    list(APPEND LIGHTNVR_INCLUDE_DIRS ${SOD_INCLUDE_DIR})
endif()

# Add cJSON include directories
if(CJSON_INCLUDE_DIRS)
    list(APPEND LIGHTNVR_INCLUDE_DIRS ${CJSON_INCLUDE_DIRS})
endif()

# Add HTTP backend include directories
if(HTTP_BACKEND_INCLUDE_DIRS)
    list(APPEND LIGHTNVR_INCLUDE_DIRS ${HTTP_BACKEND_INCLUDE_DIRS})
endif()

# Add libyaml include directories if available
if(YAML_FOUND AND YAML_INCLUDE_DIRS)
    list(APPEND LIGHTNVR_INCLUDE_DIRS ${YAML_INCLUDE_DIRS})
endif()

# Apply include directories
include_directories(${LIGHTNVR_INCLUDE_DIRS})

# Define source files, excluding SOD sources which are built separately
file(GLOB CORE_SOURCES "src/core/*.c")
# Exclude main.c from CORE_SOURCES so it is not compiled into lightnvr_lib.
# Test executables link against lightnvr_lib and provide their own main(); having
# main.c.o inside the library would produce a duplicate-symbol link error.
# main.c is compiled directly into the lightnvr executable via add_executable().
list(FILTER CORE_SOURCES EXCLUDE REGEX ".*core/main\\.c$")
message(STATUS "Excluding main.c from CORE_SOURCES (library build)")
file(GLOB DATABASE_SOURCES "src/database/*.c")
file(GLOB STORAGE_SOURCES "src/storage/*.c")
file(GLOB UTILS_SOURCES "src/utils/*.c")
file(GLOB_RECURSE WEB_SOURCES "src/web/*.c")

# Filter web sources - exclude libuv sources (they're added separately via LIBUV_SERVER_SOURCES)
list(FILTER WEB_SOURCES EXCLUDE REGEX ".*libuv_.*\\.c$")

# Build ROOT_SOURCES from the existing categorized source lists instead of using GLOB_RECURSE.
# This avoids CMake best-practice issues with recursive globbing for source discovery.
set(ROOT_SOURCES)
list(APPEND ROOT_SOURCES
    ${CORE_SOURCES}
    ${DATABASE_SOURCES}
    ${STORAGE_SOURCES}
    ${UTILS_SOURCES}
    ${WEB_SOURCES}
)
# Exclude files with their own main() from ROOT_SOURCES to avoid multiple-definition
# errors when linking test executables against lightnvr_lib.
list(FILTER ROOT_SOURCES EXCLUDE REGEX ".*sod/sod\\.c$")
list(FILTER ROOT_SOURCES EXCLUDE REGEX ".*core/main\\.c$")
list(FILTER ROOT_SOURCES EXCLUDE REGEX ".*tools/.*\\.c$")
# Also exclude libuv sources from ROOT_SOURCES (they're added separately)
list(FILTER ROOT_SOURCES EXCLUDE REGEX ".*libuv_.*\\.c$")
message(STATUS "Excluding main.c and tools from ROOT_SOURCES")

# Collect video sources from src/video, then exclude motion_detection_optimized.c,
# detection_thread_pool.c, and the original hls_writer_thread.c (since we're using our split version).
file(GLOB VIDEO_SOURCES "src/video/*.c")

list(FILTER VIDEO_SOURCES EXCLUDE REGEX ".*motion_detection_optimized\\.c$")
list(FILTER VIDEO_SOURCES EXCLUDE REGEX ".*detection_thread_pool\\.c$")
list(FILTER VIDEO_SOURCES EXCLUDE REGEX ".*hls_writer_thread\\.c$")

# Add the new FFmpeg leak detector source
set(FFMPEG_LEAK_DETECTOR_SOURCES
    "src/video/ffmpeg_leak_detector.c"
)

# Add the leak detector to the video sources
list(APPEND VIDEO_SOURCES ${FFMPEG_LEAK_DETECTOR_SOURCES})

# Add HLS sources
file(GLOB HLS_SOURCES "src/video/hls/*.c")
file(GLOB TELEMETRY_SOURCES "src/telemetry/*.c")

# Add unified HLS thread sources explicitly to ensure they're included
set(HLS_UNIFIED_THREAD_SOURCES
    "src/video/hls/hls_unified_thread.c"
)

# Set up go2rtc integration if enabled
set(GO2RTC_SOURCES "")
if(ENABLE_GO2RTC)
    add_subdirectory(src/video/go2rtc)
    add_definitions(-DUSE_GO2RTC)
    add_definitions(-DGO2RTC_BINARY_PATH_RAW=\"${GO2RTC_BINARY_PATH}\")
    add_definitions(-DGO2RTC_CONFIG_DIR_RAW=\"${GO2RTC_CONFIG_DIR}\")
    add_definitions(-DGO2RTC_API_PORT=${GO2RTC_API_PORT})
    message(STATUS "go2rtc integration enabled with binary path: ${GO2RTC_BINARY_PATH}")
    message(STATUS "go2rtc config directory: ${GO2RTC_CONFIG_DIR}")
    message(STATUS "go2rtc API port: ${GO2RTC_API_PORT}")
else()
    list(APPEND GO2RTC_SOURCES
        "${CMAKE_CURRENT_SOURCE_DIR}/src/video/go2rtc/go2rtc_disabled.c"
    )
endif()

# Combine all sources (excluding inih, bundled cjson)
set(SOURCES
        ${ROOT_SOURCES}
        ${VIDEO_SOURCES}
        ${HLS_SOURCES}
        ${HLS_UNIFIED_THREAD_SOURCES}
        ${TELEMETRY_SOURCES}
        ${EZXML_SOURCES}
)


# Add go2rtc sources or disabled stubs
list(APPEND SOURCES ${GO2RTC_SOURCES})

# Add libuv server sources
list(APPEND SOURCES ${LIBUV_SERVER_SOURCES})

# Define the shared library
add_library(lightnvr_lib STATIC ${SOURCES})
target_include_directories(lightnvr_lib PUBLIC ${LIGHTNVR_INCLUDE_DIRS})
target_include_directories(lightnvr_lib PRIVATE
    "${CMAKE_CURRENT_SOURCE_DIR}/external/inih"
)

# Add HTTP backend include directories to library
target_include_directories(lightnvr_lib PRIVATE ${HTTP_BACKEND_INCLUDE_DIRS})

# Publish core link dependencies for the static library so downstream
# test binaries and utilities automatically receive the libraries needed
# by objects pulled from lightnvr_lib.
target_link_libraries(lightnvr_lib PUBLIC
        ${HTTP_BACKEND_LIBS}
        inih_lib
        ${FFMPEG_LIBRARIES}
        ${SQLITE_LIBRARIES}
        ${CURL_LIBRARIES}
        ${SSL_LIBRARIES}
        pthread
        dl
        m
)

# Define the main executable
add_executable(lightnvr src/core/main.c)

# Define source files for rebuild_recordings utility (excluding inih)
set(REBUILD_RECORDINGS_SOURCES
    src/tools/rebuild_recordings.c
    src/core/config.c
    src/core/logger.c
    src/core/path_utils.c
    src/utils/strings.c
    src/database/db_core.c
    src/database/db_streams.c
    src/database/db_recordings.c
    src/database/db_schema.c
    src/database/db_schema_cache.c
    src/database/db_backup.c
    src/database/db_transaction.c
    src/database/sqlite_migrate.c
    src/database/db_migrations.c
    src/database/db_query_builder.c
)

# Define the rebuild_recordings utility
add_executable(rebuild_recordings ${REBUILD_RECORDINGS_SOURCES})

# Set the output directory for the binaries
set_target_properties(lightnvr rebuild_recordings PROPERTIES
        RUNTIME_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/bin"
)

# Link libraries for main executable
target_link_libraries(lightnvr
        lightnvr_lib
        ${HTTP_BACKEND_LIBS}
        inih_lib
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
        inih_lib
        ${FFMPEG_LIBRARIES}
        ${SQLITE_LIBRARIES}
        pthread
        dl
        m
)

# Link cJSON (system library)
target_link_libraries(lightnvr ${CJSON_LIBRARIES})
target_link_libraries(rebuild_recordings ${CJSON_LIBRARIES})

# Link libyaml if available (used by src/utils/yaml_validate.c)
# Use LDFLAGS to get both -L and -l from pkg-config so non-default prefixes work.
if(YAML_FOUND)
    if(YAML_LIBRARY_DIRS)
        target_link_directories(lightnvr_lib PUBLIC ${YAML_LIBRARY_DIRS})
        target_link_directories(lightnvr PUBLIC ${YAML_LIBRARY_DIRS})
    endif()
    target_link_libraries(lightnvr_lib PUBLIC ${YAML_LIBRARIES})
    target_link_libraries(lightnvr ${YAML_LIBRARIES})
endif()

# Link MQTT library if enabled
if(ENABLE_MQTT AND MOSQUITTO_FOUND)
    target_link_libraries(lightnvr ${MOSQUITTO_LIBRARIES})
    target_include_directories(lightnvr PRIVATE ${MOSQUITTO_INCLUDE_DIRS})
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
                INSTALL_RPATH "$ORIGIN/../lib"
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
            target_link_options(lightnvr PRIVATE "LINKER:-rpath,\$ORIGIN/../src/sod:\$ORIGIN/../lib")
        endif()


    else()
        message(STATUS "Using static linking for SOD library")
    endif()
endif()

# Install targets
install(TARGETS lightnvr rebuild_recordings DESTINATION bin)
install(DIRECTORY config/ DESTINATION /etc/lightnvr)
install(DIRECTORY db/migrations/ DESTINATION share/lightnvr/migrations
        FILES_MATCHING PATTERN "*.sql")

# Add subdirectories for tests if testing is enabled
option(BUILD_TESTS "Build the test suite" OFF)
if(BUILD_TESTS)
    enable_testing()
    add_subdirectory(tests)
endif()

# Obtain git short commit hash for version stamping
set(GIT_COMMIT "unknown")
find_package(Git QUIET)
if(GIT_FOUND)
    execute_process(
        COMMAND ${GIT_EXECUTABLE} rev-parse --short HEAD
        WORKING_DIRECTORY ${CMAKE_SOURCE_DIR}
        OUTPUT_VARIABLE GIT_COMMIT
        OUTPUT_STRIP_TRAILING_WHITESPACE
        ERROR_QUIET
    )
endif()

# Build date in ISO-8601
string(TIMESTAMP BUILD_DATE "%Y-%m-%d")

# Create a version.h file
file(READ
        "${CMAKE_CURRENT_SOURCE_DIR}/include/core/version.h.in"
        VERSION_H_IN
)
string(CONFIGURE "${VERSION_H_IN}" VERSION_H_OUT @ONLY)
file(GENERATE
        OUTPUT "${CMAKE_CURRENT_SOURCE_DIR}/include/core/version.h"
        CONTENT "${VERSION_H_OUT}"
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
message(STATUS "- HTTP backend: libuv + llhttp")
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
if(YAML_FOUND)
    message(STATUS "- libyaml (YAML validation): ENABLED (${YAML_VERSION})")
else()
    message(STATUS "- libyaml (YAML validation): DISABLED (stub only)")
endif()
