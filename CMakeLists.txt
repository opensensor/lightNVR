cmake_minimum_required(VERSION 3.10)
project(LightNVR VERSION 0.19.2 LANGUAGES C CXX)

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
add_compile_options(-O2 -ffunction-sections -fdata-sections)
add_link_options(-Wl,--gc-sections)

# Add ASan/UBSan flags for Debug builds with GCC/Clang
if(CMAKE_BUILD_TYPE MATCHES "^[Dd][Ee][Bb][Uu][Gg]$")
    if(CMAKE_C_COMPILER_ID MATCHES "GNU|Clang" AND CMAKE_CXX_COMPILER_ID MATCHES "GNU|Clang")
        message(STATUS "Enabling AddressSanitizer and UndefinedBehaviorSanitizer for Debug build")
        add_compile_options(-fsanitize=address,undefined)
        add_link_options(-fsanitize=address,undefined)
        add_compile_options(-ggdb3)

        # improved debugging experience
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
pkg_check_modules(FFMPEG REQUIRED libavcodec libavformat libavutil libswscale)
pkg_check_modules(SQLITE REQUIRED sqlite3)
pkg_check_modules(CURL REQUIRED libcurl)
find_package(Threads REQUIRED)

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

# SSL/TLS configuration for Mongoose
if(ENABLE_SSL)
    if(USE_MBEDTLS)
        add_definitions(-DMG_TLS=${MG_TLS_MBED})
        pkg_check_modules(MBEDTLS QUIET mbedtls mbedcrypto mbedx509)
        if (NOT MBEDTLS_FOUND)
          find_path(MBEDTLS_INCLUDE_DIRS "version.h" PATH_SUFFIXES "mbedtls" DOC "mbedtls include directory")
          find_library(MBEDTLS_LIBRARY NAMES mbedtls REQUIRED)
          find_library(MBEDCRYPTO_LIBRARY NAMES mbedcrypto REQUIRED)
          find_library(MBEDX509_LIBRARY NAMES mbedx509 REQUIRED)
          set(MBEDTLS_LIBRARIES ${MBEDTLS_LIBRARY} ${MBEDCRYPTO_LIBRARY} ${MBEDX509_LIBRARY})
        endif()
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
pkg_check_modules(MBEDTLS QUIET mbedtls mbedcrypto mbedx509)
if (NOT MBEDTLS_FOUND)
  find_path(MBEDTLS_INCLUDE_DIRS "version.h" PATH_SUFFIXES "mbedtls" DOC "mbedtls include directory")
  find_library(MBEDTLS_LIBRARY NAMES mbedtls REQUIRED)
  find_library(MBEDCRYPTO_LIBRARY NAMES mbedcrypto REQUIRED)
  find_library(MBEDX509_LIBRARY NAMES mbedx509 REQUIRED)
  set(MBEDTLS_LIBRARIES ${MBEDTLS_LIBRARY} ${MBEDCRYPTO_LIBRARY} ${MBEDX509_LIBRARY})
endif()
list(APPEND SSL_INCLUDE_DIRS ${MBEDTLS_INCLUDE_DIRS})
list(APPEND SSL_LIBRARIES ${MBEDTLS_LIBRARIES})
message(STATUS "ONVIF and authentication system enabled, linking with mbedTLS for cryptographic functions")

# Explicitly add mbedcrypto to the link libraries
find_library(MBEDCRYPTO_LIBRARY mbedcrypto
    HINTS
    /usr/lib
    /usr/lib/x86_64-linux-gnu
    /usr/lib/aarch64-linux-gnu
    /usr/lib/arm-linux-gnueabihf
    /lib
    /lib/x86_64-linux-gnu
    /lib/aarch64-linux-gnu
    /lib/arm-linux-gnueabihf
)

if(MBEDCRYPTO_LIBRARY)
    message(STATUS "Found mbedcrypto library: ${MBEDCRYPTO_LIBRARY}")
    list(APPEND SSL_LIBRARIES ${MBEDCRYPTO_LIBRARY})
else()
    # Try to find by manually checking common locations
    foreach(LIB_PATH /usr/lib /usr/lib/x86_64-linux-gnu /usr/lib/aarch64-linux-gnu /usr/lib/arm-linux-gnueabihf /lib /lib/x86_64-linux-gnu /lib/aarch64-linux-gnu /lib/arm-linux-gnueabihf)
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
set(MONGOOSE_SOURCES "${CMAKE_CURRENT_SOURCE_DIR}/external/mongoose/mongoose.c")
add_library(mongoose_lib STATIC ${MONGOOSE_SOURCES})
target_include_directories(mongoose_lib INTERFACE "${CMAKE_CURRENT_SOURCE_DIR}/external/mongoose")

# Set up inih library
set(INIH_SOURCES "${CMAKE_CURRENT_SOURCE_DIR}/external/inih/ini.c")
add_library(inih_lib STATIC ${INIH_SOURCES})
target_include_directories(inih_lib INTERFACE "${CMAKE_CURRENT_SOURCE_DIR}/external/inih")

# Define all include directories in one place
set(LIGHTNVR_INCLUDE_DIRS
        ${LIGHTNVR_INCLUDE_DIR}
        ${CMAKE_CURRENT_SOURCE_DIR}
        ${FFMPEG_INCLUDE_DIRS}
        ${SQLITE_INCLUDE_DIRS}
        ${CURL_INCLUDE_DIRS}
        ${SSL_INCLUDE_DIRS}
        ${EZXML_INCLUDE_DIR}
)

# Add conditional include directories
if(ENABLE_SOD)
    list(APPEND LIGHTNVR_INCLUDE_DIRS ${SOD_INCLUDE_DIR})
endif()

# Add cJSON include directories
if(CJSON_INCLUDE_DIRS)
    list(APPEND LIGHTNVR_INCLUDE_DIRS ${CJSON_INCLUDE_DIRS})
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

# Add the new FFmpeg leak detector source
set(FFMPEG_LEAK_DETECTOR_SOURCES
    "src/video/ffmpeg_leak_detector.c"
)

# Add the leak detector to the video sources
list(APPEND VIDEO_SOURCES ${FFMPEG_LEAK_DETECTOR_SOURCES})

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

# Combine all sources (excluding mongoose, inih, bundled cjson)
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
        include/utils/strings.h
)

# Add go2rtc sources if enabled
if(ENABLE_GO2RTC)
    list(APPEND SOURCES ${GO2RTC_SOURCES})
endif()

# Define the shared library
add_library(lightnvr_lib STATIC ${SOURCES})
target_include_directories(lightnvr_lib PUBLIC ${LIGHTNVR_INCLUDE_DIRS})
target_include_directories(lightnvr_lib PRIVATE
    "${CMAKE_CURRENT_SOURCE_DIR}/external/mongoose"
    "${CMAKE_CURRENT_SOURCE_DIR}/external/inih"
)

# Define the main executable
add_executable(lightnvr src/core/main.c)

# Define source files for rebuild_recordings utility (excluding inih)
set(REBUILD_RECORDINGS_SOURCES
    src/utils/rebuild_recordings.c
    src/core/config.c
    src/core/logger.c
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
        mongoose_lib
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
        lightnvr_lib
        mongoose_lib
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
install(DIRECTORY db/migrations/ DESTINATION share/lightnvr/migrations
        FILES_MATCHING PATTERN "*.sql")

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
