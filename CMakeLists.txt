# 3.13: add_link_options, target_link_options and target_link_directories.
cmake_minimum_required(VERSION 3.13)

# CMake 4.0 removed compat with cmake_minimum_required(VERSION < 3.5).
# Third-party deps fetched via LiteRT's FetchContent (e.g. neon2sse) still
# declare old versions; set the cache floor so they configure cleanly.
if(CMAKE_VERSION VERSION_GREATER_EQUAL "4.0")
    set(CMAKE_POLICY_VERSION_MINIMUM 3.5 CACHE STRING "" FORCE)
endif()

project(LightNVR VERSION 0.47.1 LANGUAGES C CXX)

# Set C/C++ standards
set(CMAKE_C_STANDARD 11)
set(CMAKE_CXX_STANDARD 14)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# Build profiles select conservative defaults without removing the individual
# feature switches.  The full profile remains the default for existing builds.
option(EMBEDDED_A1_DEVICE "Deprecated alias for LIGHTNVR_PROFILE=embedded" OFF)
set(LIGHTNVR_PROFILE "full" CACHE STRING "Build profile: full or embedded")
set_property(CACHE LIGHTNVR_PROFILE PROPERTY STRINGS full embedded)
string(TOLOWER "${LIGHTNVR_PROFILE}" LIGHTNVR_PROFILE)

if(EMBEDDED_A1_DEVICE)
    message(DEPRECATION
        "EMBEDDED_A1_DEVICE is deprecated; use -DLIGHTNVR_PROFILE=embedded")
    set(LIGHTNVR_PROFILE "embedded")
endif()

if(NOT LIGHTNVR_PROFILE MATCHES "^(full|embedded)$")
    message(FATAL_ERROR
        "Invalid LIGHTNVR_PROFILE='${LIGHTNVR_PROFILE}'; expected full or embedded")
endif()

if(LIGHTNVR_PROFILE STREQUAL "embedded")
    set(LIGHTNVR_OPTIONAL_FEATURE_DEFAULT OFF)
    set(LIGHTNVR_DEFAULT_MAX_STREAMS 16)
else()
    set(LIGHTNVR_OPTIONAL_FEATURE_DEFAULT ON)
    set(LIGHTNVR_DEFAULT_MAX_STREAMS 1024)
endif()

set(LIGHTNVR_MAX_STREAMS "auto" CACHE STRING
    "Compile-time stream ceiling (auto, or an integer from 1 to 1024)")
if(LIGHTNVR_MAX_STREAMS STREQUAL "auto")
    set(LIGHTNVR_RESOLVED_MAX_STREAMS ${LIGHTNVR_DEFAULT_MAX_STREAMS})
elseif(LIGHTNVR_MAX_STREAMS MATCHES "^[0-9]+$")
    # Normalize to canonical decimal to avoid leading-zero forms (for example
    # 08/09 invalid, 010 interpreted as octal) in downstream C definitions.
    set(LIGHTNVR_MAX_STREAMS_NORMALIZED "${LIGHTNVR_MAX_STREAMS}")
    string(REGEX REPLACE "^0+" "" LIGHTNVR_MAX_STREAMS_NORMALIZED
           "${LIGHTNVR_MAX_STREAMS_NORMALIZED}")
    if(LIGHTNVR_MAX_STREAMS_NORMALIZED STREQUAL "")
        set(LIGHTNVR_MAX_STREAMS_NORMALIZED "0")
    endif()
    math(EXPR LIGHTNVR_RESOLVED_MAX_STREAMS "${LIGHTNVR_MAX_STREAMS_NORMALIZED}")
else()
    message(FATAL_ERROR
        "LIGHTNVR_MAX_STREAMS must be 'auto' or an integer from 1 to 1024")
endif()

if(LIGHTNVR_RESOLVED_MAX_STREAMS LESS 1 OR
   LIGHTNVR_RESOLVED_MAX_STREAMS GREATER 1024)
    message(FATAL_ERROR "LIGHTNVR_MAX_STREAMS must be between 1 and 1024")
endif()
if(LIGHTNVR_PROFILE STREQUAL "embedded" AND
   LIGHTNVR_RESOLVED_MAX_STREAMS GREATER 16)
    message(FATAL_ERROR "The embedded profile is limited to 16 streams")
endif()

add_definitions(-DLIGHTNVR_MAX_STREAMS=${LIGHTNVR_RESOLVED_MAX_STREAMS})

# Option to enable/disable SOD
option(ENABLE_SOD "Enable SOD library for object detection" ${LIGHTNVR_OPTIONAL_FEATURE_DEFAULT})
option(SOD_DYNAMIC_LINK "Dynamically link SOD library instead of static linking" OFF)

# In-process LiteRT (TFLite) detection engine. Default ON, but auto-disabled at
# configure time if the third_party/litert submodule isn't initialized, so a
# fresh checkout still builds. When ON (and present), the vendored LiteRT
# submodule is built and linked, and the litert_engine module is compiled.
option(ENABLE_LITERT "Enable in-process LiteRT (TFLite) detection engine" ${LIGHTNVR_OPTIONAL_FEATURE_DEFAULT})
option(LITERT_WITH_XNNPACK "Compile XNNPACK delegate into the LiteRT engine" ON)
option(LITERT_WITH_GPU "Compile GPU (OpenCL/GLES) delegate into the LiteRT engine" OFF)

# go2rtc integration options
option(ENABLE_GO2RTC "Enable go2rtc integration for WebRTC streaming" ON)
set(GO2RTC_BINARY_PATH "/usr/local/bin/go2rtc" CACHE STRING "Path to go2rtc binary")
set(GO2RTC_CONFIG_DIR "/etc/lightnvr/go2rtc" CACHE STRING "Directory for go2rtc configuration files")
set(GO2RTC_API_PORT 1984 CACHE STRING "Port for go2rtc HTTP API")

# llhttp download configuration. Avoid the LLHTTP_VERSION name because
# pkg_check_modules(LLHTTP ...) owns that variable when libllhttp is probed.
set(LLHTTP_DOWNLOAD_REF "release/v9.4.3" CACHE STRING "llhttp tag/ref to download from GitHub")

# HTTPS in the built-in web server (mbedTLS, already required for ONVIF/auth)
option(ENABLE_SSL "Build HTTPS/TLS support into the built-in web server (mbedTLS)" OFF)

# HTTP backend: libuv + llhttp
set(HTTP_BACKEND "libuv" CACHE STRING "HTTP server backend")
set_property(CACHE HTTP_BACKEND PROPERTY STRINGS libuv)
string(TOLOWER "${HTTP_BACKEND}" HTTP_BACKEND)
if(NOT HTTP_BACKEND STREQUAL "libuv")
    message(FATAL_ERROR "Unsupported HTTP_BACKEND='${HTTP_BACKEND}'. Only 'libuv' is currently supported.")
endif()

# Compiler flags for optimization and memory usage.  Do not override the
# toolchain's optimization level (Buildroot supplies -Os for target builds).
add_compile_options(-ffunction-sections -fdata-sections)
add_link_options(-Wl,--gc-sections)

if(LIGHTNVR_PROFILE STREQUAL "embedded")
    add_compile_options(-Os -fomit-frame-pointer)
endif()

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
option(ENABLE_S3_ARCHIVE "Enable S3 archive targets (requires libcurl 7.75+)" ON)
if(ENABLE_S3_ARCHIVE AND CURL_VERSION VERSION_LESS "7.75.0")
    message(FATAL_ERROR "S3 archival requires libcurl >= 7.75. Use -DENABLE_S3_ARCHIVE=OFF for filesystem-only builds.")
endif()
add_compile_definitions(LIGHTNVR_ENABLE_S3=$<BOOL:${ENABLE_S3_ARCHIVE}>)
find_package(ZLIB REQUIRED)
find_package(Threads REQUIRED)

# libyaml (optional-but-preferred) — used for go2rtc override validation
# When available, defines LIGHTNVR_HAVE_LIBYAML=1 so src/utils/yaml_validate.c
# compiles the real parser path; otherwise the stub path is compiled.
pkg_check_modules(YAML QUIET yaml-0.1)
if(YAML_FOUND)
    message(STATUS "Found libyaml: ${YAML_LIBRARIES} (version ${YAML_VERSION})")
    add_compile_definitions(LIGHTNVR_HAVE_LIBYAML=1)
else()
    message(WARNING "libyaml (yaml-0.1) not found. YAML validation and go2rtc override base_path detection will be disabled. Install libyaml-dev / yaml-dev to enable.")
endif()

# libavfilter (optional) — per-stream voice-enhancement filter chain applied
# while transcoding PCM audio to AAC (issue #652).  Without it the opt-in is
# accepted but logs that it is unavailable.  The afftdn denoiser is the costly
# stage, so the embedded profile leaves it out by default.
option(ENABLE_AUDIO_FILTERS "Enable libavfilter audio filters (voice enhancement)" ON)
option(ENABLE_AUDIO_DENOISE "Include the afftdn denoiser in voice enhancement" ${LIGHTNVR_OPTIONAL_FEATURE_DEFAULT})
if(ENABLE_AUDIO_FILTERS)
    pkg_check_modules(AVFILTER QUIET libavfilter>=10)
    if(AVFILTER_FOUND)
        message(STATUS "Found libavfilter: ${AVFILTER_VERSION} (denoise: ${ENABLE_AUDIO_DENOISE})")
        add_compile_definitions(LIGHTNVR_HAVE_AVFILTER=1
            LIGHTNVR_AUDIO_DENOISE=$<BOOL:${ENABLE_AUDIO_DENOISE}>)
    else()
        message(WARNING "libavfilter >= 10 not found. Voice enhancement will be unavailable. Install libavfilter-dev to enable.")
        set(ENABLE_AUDIO_FILTERS OFF)
    endif()
endif()

# MQTT support (optional, uses libmosquitto)
option(ENABLE_MQTT "Enable MQTT support for detection event streaming" ${LIGHTNVR_OPTIONAL_FEATURE_DEFAULT})
if(ENABLE_MQTT)
    pkg_check_modules(MOSQUITTO QUIET libmosquitto>=2.0)
    if(MOSQUITTO_FOUND)
        message(STATUS "Found libmosquitto: ${MOSQUITTO_LIBRARIES}")
        add_definitions(-DENABLE_MQTT=1)
    else()
        message(WARNING "libmosquitto >= 2.0 not found. MQTT support will be disabled. Install libmosquitto-dev to enable.")
        set(ENABLE_MQTT OFF)
    endif()
endif()

# Common mbedTLS detection logic (used for ONVIF, auth and HTTPS).
# Runs on every configure: pkg-config and find_* results are cached, so this is
# cheap, and it rebuilds MBEDTLS_LIBRARIES (a normal variable) each time. A
# "detect once" cache flag used to skip it on reconfigure, which left the
# find_library fallback's library list empty and broke the link.
unset(MBEDTLS_DETECTION_DONE CACHE)
macro(DETECT_MBEDTLS)
    pkg_check_modules(MBEDTLS QUIET mbedtls mbedcrypto mbedx509)
    if (NOT MBEDTLS_FOUND)
        find_path(MBEDTLS_INCLUDE_DIRS "mbedtls/version.h" DOC "mbedtls include directory")
        find_library(MBEDTLS_LIBRARY NAMES mbedtls)
        find_library(MBEDCRYPTO_LIBRARY NAMES mbedcrypto)
        find_library(MBEDX509_LIBRARY NAMES mbedx509)
        if(NOT MBEDTLS_INCLUDE_DIRS OR NOT MBEDTLS_LIBRARY OR NOT MBEDCRYPTO_LIBRARY OR NOT MBEDX509_LIBRARY)
            message(FATAL_ERROR "mbedTLS is required (for ONVIF, authentication and HTTPS) but was not found. Missing one or more of: headers (mbedtls/version.h), libmbedtls, libmbedcrypto, libmbedx509.")
        endif()
        set(MBEDTLS_LIBRARIES ${MBEDTLS_LIBRARY} ${MBEDCRYPTO_LIBRARY} ${MBEDX509_LIBRARY})
    endif()
endmacro()

# SSL/TLS configuration. The web server terminates TLS with mbedTLS, which is
# linked unconditionally below, so ENABLE_SSL only compiles the TLS code in.
# The former OpenSSL/WolfSSL selections linked libraries nothing used.
foreach(_removed_ssl_option USE_MBEDTLS USE_WOLFSSL)
    if(${_removed_ssl_option})
        message(WARNING "${_removed_ssl_option} is no longer used: HTTPS always uses mbedTLS")
    endif()
    unset(${_removed_ssl_option} CACHE)
endforeach()
set(SSL_INCLUDE_DIRS "")
set(SSL_LIBRARIES "")
if(ENABLE_SSL)
    add_compile_definitions(LIGHTNVR_ENABLE_TLS=1)
    message(STATUS "HTTPS support enabled in the web server (mbedTLS)")
else()
    message(STATUS "HTTPS support is disabled (enable with -DENABLE_SSL=ON)")
endif()

# ONVIF and authentication system require mbedTLS for cryptographic functions
DETECT_MBEDTLS()
list(APPEND SSL_INCLUDE_DIRS ${MBEDTLS_INCLUDE_DIRS})
list(APPEND SSL_LIBRARIES ${MBEDTLS_LIBRARIES})
message(STATUS "ONVIF and authentication system enabled, linking with mbedTLS for cryptographic functions")

# Explicitly add mbedcrypto to the link libraries
# Use CMake's standard library search paths for portability across distros/platforms.
find_library(MBEDCRYPTO_LIBRARY NAMES mbedcrypto)

if(MBEDCRYPTO_LIBRARY)
    message(STATUS "Found mbedcrypto library: ${MBEDCRYPTO_LIBRARY}")
    list(APPEND SSL_LIBRARIES ${MBEDCRYPTO_LIBRARY})
else()
    message(FATAL_ERROR "mbedcrypto library not found. Please install the mbedTLS development package.")
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

# Set up LiteRT (in-process TFLite engine) if enabled. Auto-disable it when the
# submodule isn't initialized so a default configure still succeeds out-of-the-box.
set(LITERT_ENGINE_SOURCES "")
if(ENABLE_LITERT AND NOT EXISTS "${CMAKE_CURRENT_SOURCE_DIR}/third_party/litert/tflite/CMakeLists.txt")
    message(WARNING
        "ENABLE_LITERT is ON but third_party/litert is not initialized; "
        "disabling the in-process LiteRT engine. Run "
        "'git submodule update --init --recursive third_party/litert' to enable it.")
    set(ENABLE_LITERT OFF)
endif()
if(ENABLE_LITERT)
    set(LITERT_SUBDIR "${CMAKE_CURRENT_SOURCE_DIR}/third_party/litert/tflite")

    # Forward delegate selections to the LiteRT subproject.
    set(TFLITE_ENABLE_XNNPACK ${LITERT_WITH_XNNPACK} CACHE BOOL "" FORCE)
    set(TFLITE_ENABLE_GPU ${LITERT_WITH_GPU} CACHE BOOL "" FORCE)
    # ruy is TFLite's ARM CPU GEMM backend; it runs any ops the XNNPACK delegate
    # doesn't handle (falling back to it instead of the slower gemmlowp/Eigen path).
    set(TFLITE_ENABLE_RUY ON CACHE BOOL "" FORCE)
    set(TFLITE_BUILD_SHARED_LIB OFF CACHE BOOL "" FORCE)

    # Build XNNPACK without its hand-written assembly microkernels, so it uses the
    # portable NEON-intrinsic kernels instead. XNNPACK's cortex_a53 asm f32 6x8 GEMM
    # kernel corrupts an A-row pointer under a multi-threaded pthreadpool on aarch64
    # big.LITTLE, SIGSEGV-ing with threads>1 (root-caused from a core dump; present
    # even at XNNPACK HEAD). Disabling the asm path makes multi-threaded inference
    # stable; confirmed on target. Small single-thread perf cost vs tuned asm.
    if(LITERT_WITH_XNNPACK)
        set(XNNPACK_ENABLE_ASSEMBLY OFF CACHE BOOL "" FORCE)
    endif()

    add_subdirectory(${LITERT_SUBDIR} EXCLUDE_FROM_ALL)

    # GCC 15+ made attribute-after-declarator a hard error. XNNPACK generated
    # kernels still use the old syntax; downgrade to a warning for those targets.
    if(LITERT_WITH_XNNPACK AND CMAKE_C_COMPILER_ID STREQUAL "GNU" AND
       CMAKE_C_COMPILER_VERSION VERSION_GREATER_EQUAL "15.0")
        foreach(_xnn_tgt IN ITEMS xnnpack-microkernels-prod xnnpack-microkernels-all xnnpack)
            if(TARGET ${_xnn_tgt})
                target_compile_options(${_xnn_tgt} PRIVATE -Wno-error=attributes)
            endif()
        endforeach()
    endif()

    add_compile_definitions(HAVE_LITERT)

    if(LITERT_WITH_XNNPACK)
        add_compile_definitions(HAVE_LITERT_XNNPACK)
    endif()
    if(LITERT_WITH_GPU)
        add_compile_definitions(HAVE_LITERT_GPU)
    endif()

    set(LITERT_ENGINE_SOURCES
        "${CMAKE_CURRENT_SOURCE_DIR}/src/video/detection/litert_engine.cc"
    )
    # TFLite public headers (and the Abseil/FlatBuffers they pull in) require
    # C++17, but the project default is C++14. Linking the tensorflow-lite
    # target propagates cxx_std_17, yet make the requirement explicit on our
    # own engine source so it compiles correctly regardless of propagation.
    set_source_files_properties(${LITERT_ENGINE_SOURCES} PROPERTIES CXX_STANDARD 17)
    message(STATUS "LiteRT engine enabled (XNNPACK=${LITERT_WITH_XNNPACK}, GPU=${LITERT_WITH_GPU})")
else()
    message(STATUS "LiteRT engine disabled (set -DENABLE_LITERT=ON to enable)")
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
    # SHA-256 hashes for LLHTTP_PINNED_REF, in the same order as
    # LLHTTP_REQUIRED_FILES.  Update both together when bumping llhttp.
    # If LLHTTP_DOWNLOAD_REF is overridden to a different ref, the hashes
    # cannot be known here, so downloads proceed unverified with a warning.
    set(LLHTTP_PINNED_REF "release/v9.4.3")
    set(LLHTTP_DOWNLOAD_REF_TYPE "tag" CACHE STRING "Type of LLHTTP_DOWNLOAD_REF: tag, branch, or commit")
    set_property(CACHE LLHTTP_DOWNLOAD_REF_TYPE PROPERTY STRINGS tag branch commit)
    string(TOLOWER "${LLHTTP_DOWNLOAD_REF_TYPE}" LLHTTP_DOWNLOAD_REF_TYPE)
    set(LLHTTP_REQUIRED_HASHES
        "391e7c99912abf3b1c9dd8a9c85fdeaac2663e9fa55b6f3871154d16fef8b892"
        "0f8590206fe2f264db2825401b5fb856b979ee1363dc88b06f36dc9577afb941"
        "a1f2b23168f8e9b5bfa464c89027d3ca259fc649ae3d7e72bfff997d1aeb5d72"
        "5bc82fa51b19aa8bee7d921038393fbfc74e8cc9bae0844c2d82c102fb8cfd68"
    )
    if(LLHTTP_DOWNLOAD_REF_TYPE STREQUAL "tag" AND
       LLHTTP_DOWNLOAD_REF STREQUAL LLHTTP_PINNED_REF)
        set(LLHTTP_VERIFY_HASHES TRUE)
    else()
        set(LLHTTP_VERIFY_HASHES FALSE)
        # A nonempty source-tree cache may belong to the default tag or a
        # different override. Isolate alternate refs (including their type).
        string(SHA256 LLHTTP_REF_CACHE_KEY
               "${LLHTTP_DOWNLOAD_REF_TYPE}:${LLHTTP_DOWNLOAD_REF}")
        set(LLHTTP_DIR "${CMAKE_CURRENT_BINARY_DIR}/_deps/llhttp-${LLHTTP_REF_CACHE_KEY}")
    endif()
    if(LLHTTP_DOWNLOAD_REF_TYPE STREQUAL "tag")
        set(LLHTTP_REF_PATH "refs/tags/${LLHTTP_DOWNLOAD_REF}")
    elseif(LLHTTP_DOWNLOAD_REF_TYPE STREQUAL "branch")
        set(LLHTTP_REF_PATH "refs/heads/${LLHTTP_DOWNLOAD_REF}")
    elseif(LLHTTP_DOWNLOAD_REF_TYPE STREQUAL "commit")
        set(LLHTTP_REF_PATH "${LLHTTP_DOWNLOAD_REF}")
    else()
        message(FATAL_ERROR "Invalid LLHTTP_DOWNLOAD_REF_TYPE='${LLHTTP_DOWNLOAD_REF_TYPE}'. Expected one of: tag, branch, commit.")
    endif()
    set(LLHTTP_BASE_URL "https://raw.githubusercontent.com/nodejs/llhttp/${LLHTTP_REF_PATH}")

    list(LENGTH LLHTTP_REQUIRED_FILES LLHTTP_REQUIRED_FILES_LEN)
    math(EXPR LLHTTP_REQUIRED_FILES_LAST "${LLHTTP_REQUIRED_FILES_LEN} - 1")
    foreach(LLHTTP_INDEX RANGE 0 ${LLHTTP_REQUIRED_FILES_LAST})
        list(GET LLHTTP_REQUIRED_FILES ${LLHTTP_INDEX} LLHTTP_FILE)
        if(LLHTTP_VERIFY_HASHES)
            list(GET LLHTTP_REQUIRED_HASHES ${LLHTTP_INDEX} LLHTTP_EXPECTED_HASH)
            set(LLHTTP_HASH_ARGS EXPECTED_HASH "SHA256=${LLHTTP_EXPECTED_HASH}")
        else()
            set(LLHTTP_HASH_ARGS "")
        endif()
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
            elseif(LLHTTP_VERIFY_HASHES)
                file(SHA256 "${LLHTTP_LOCAL_PATH}" LLHTTP_ACTUAL_HASH)
                if(NOT LLHTTP_ACTUAL_HASH STREQUAL LLHTTP_EXPECTED_HASH)
                    set(LLHTTP_NEEDS_DOWNLOAD TRUE)
                    set(LLHTTP_DOWNLOAD_REASON "hash mismatch")
                endif()
            endif()
        endif()

        if(LLHTTP_NEEDS_DOWNLOAD)
            message(STATUS "llhttp ${LLHTTP_FILE} ${LLHTTP_DOWNLOAD_REASON}, downloading from GitHub...")
            if(NOT LLHTTP_VERIFY_HASHES)
                message(WARNING "llhttp ${LLHTTP_DOWNLOAD_REF_TYPE} '${LLHTTP_DOWNLOAD_REF}' is not the pinned tag '${LLHTTP_PINNED_REF}'; downloading ${LLHTTP_FILE} without SHA-256 verification")
            endif()
            get_filename_component(LLHTTP_LOCAL_DIR "${LLHTTP_LOCAL_PATH}" DIRECTORY)
            file(MAKE_DIRECTORY "${LLHTTP_LOCAL_DIR}")
            file(REMOVE "${LLHTTP_LOCAL_PATH}")

            file(DOWNLOAD
                "${LLHTTP_BASE_URL}/${LLHTTP_FILE}"
                "${LLHTTP_LOCAL_PATH}"
                SHOW_PROGRESS
                STATUS LLHTTP_DOWNLOAD_STATUS
                ${LLHTTP_HASH_ARGS}
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
    "${CMAKE_CURRENT_SOURCE_DIR}/src/web/libuv_tls.c"
    "${CMAKE_CURRENT_SOURCE_DIR}/src/web/libuv_api_handlers.c"
    "${CMAKE_CURRENT_SOURCE_DIR}/src/web/thumbnail_thread.c"
    "${CMAKE_CURRENT_SOURCE_DIR}/src/web/go2rtc_proxy_thread.c"
    "${CMAKE_CURRENT_SOURCE_DIR}/src/web/go2rtc_websocket.c"
)



# Set up inih library
set(INIH_SOURCES "${CMAKE_CURRENT_SOURCE_DIR}/external/inih/ini.c")
add_library(inih_lib STATIC ${INIH_SOURCES})
target_include_directories(inih_lib INTERFACE "${CMAKE_CURRENT_SOURCE_DIR}/external/inih")

# Set up vendored FAAC library (libfaac encoder only, HE-AAC v1/SBR, no PS).
# Mirrors upstream meson: encoder-sbr=true, encoder-ps=false, max-channels=8.
set(FAAC_DIR "${CMAKE_CURRENT_SOURCE_DIR}/third_party/faac")
set(FAAC_SOURCES
    "${FAAC_DIR}/common/fft.c"
    "${FAAC_DIR}/common/sbr_tables.c"
    "${FAAC_DIR}/common/sfb_tables.c"
    "${FAAC_DIR}/libfaac/bitstream.c"
    "${FAAC_DIR}/libfaac/blockswitch.c"
    "${FAAC_DIR}/libfaac/channels.c"
    "${FAAC_DIR}/libfaac/cpu_compute.c"
    "${FAAC_DIR}/libfaac/faac.c"
    "${FAAC_DIR}/libfaac/filtbank.c"
    "${FAAC_DIR}/libfaac/frame.c"
    "${FAAC_DIR}/libfaac/huff2.c"
    "${FAAC_DIR}/libfaac/huffdata.c"
    "${FAAC_DIR}/libfaac/quantize.c"
    "${FAAC_DIR}/libfaac/ratecontrol.c"
    "${FAAC_DIR}/libfaac/resample.c"
    "${FAAC_DIR}/libfaac/sbr.c"
    "${FAAC_DIR}/libfaac/sbr_analysis.c"
    "${FAAC_DIR}/libfaac/sbr_bitstream.c"
    "${FAAC_DIR}/libfaac/sbr_huff_tables.c"
    "${FAAC_DIR}/libfaac/stereo.c"
    "${FAAC_DIR}/libfaac/tns.c"
    "${FAAC_DIR}/libfaac/util.c"
)
set(FAAC_DEFINITIONS
    PACKAGE_VERSION="3.0.0"
    PACKAGE="faac"
    MAX_CHANNELS=8
    FAAC_SBR_DECIMATION=1
    FAAC_ENCODER_SBR=1
    FAAC_ENCODER_PS=0
    COMMON_PREFIX=faac_
)
include(CheckIncludeFile)
check_include_file(immintrin.h FAAC_HAVE_IMMINTRIN_H)
if(FAAC_HAVE_IMMINTRIN_H AND CMAKE_SYSTEM_PROCESSOR MATCHES "^(x86_64|AMD64|amd64|i[3-6]86|x86)$")
    list(APPEND FAAC_DEFINITIONS HAVE_SSE2=1)
    set(FAAC_QUANTIZE_SSE "${FAAC_DIR}/libfaac/quantize_sse.c")
    list(APPEND FAAC_SOURCES "${FAAC_QUANTIZE_SSE}")
    if(NOT MSVC)
        set_source_files_properties("${FAAC_QUANTIZE_SSE}" PROPERTIES COMPILE_OPTIONS "-msse2")
    endif()
endif()
add_library(faac_lib STATIC ${FAAC_SOURCES})
target_compile_definitions(faac_lib PRIVATE ${FAAC_DEFINITIONS})
target_include_directories(faac_lib PRIVATE
    "${FAAC_DIR}"
    "${FAAC_DIR}/common"
    "${FAAC_DIR}/libfaac"
)
target_include_directories(faac_lib PUBLIC "${FAAC_DIR}/include")
if(UNIX)
    target_link_libraries(faac_lib PUBLIC m)
endif()

# Bundle ezXML implementation used by ONVIF parsing
set(EZXML_SOURCES
    "${CMAKE_CURRENT_SOURCE_DIR}/src/ezxml/ezxml.c"
)

# Put generated headers first, including for test subdirectories. An older
# in-source build may have left include/core/version.h in the source tree.
set(GENERATED_INCLUDE_DIR "${CMAKE_CURRENT_BINARY_DIR}/include")

# Define all include directories in one place
set(LIGHTNVR_INCLUDE_DIRS
        ${GENERATED_INCLUDE_DIR}
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

# Add HLS sources
file(GLOB HLS_SOURCES "src/video/hls/*.c")
# hls_unified_thread.c is added explicitly via HLS_UNIFIED_THREAD_SOURCES below.
# Exclude it here to avoid duplicate source listing.
list(FILTER HLS_SOURCES EXCLUDE REGEX ".*hls_unified_thread\\.c$")
file(GLOB TELEMETRY_SOURCES "src/telemetry/*.c")
file(GLOB HEALTH_COLLECTOR_SOURCES "src/telemetry/collectors/*.c")
file(GLOB HEALTH_PROVIDER_SOURCES "src/telemetry/providers/*.c")

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
        "${CMAKE_CURRENT_SOURCE_DIR}/src/video/go2rtc/go2rtc_lifecycle.c"
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
        ${HEALTH_COLLECTOR_SOURCES}
        ${HEALTH_PROVIDER_SOURCES}
        ${EZXML_SOURCES}
        ${LITERT_ENGINE_SOURCES}
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
        faac_lib
        ${FFMPEG_LIBRARIES}
        ${SQLITE_LIBRARIES}
        ${CURL_LIBRARIES}
        ${CJSON_LIBRARIES}
        ZLIB::ZLIB
        ${SSL_LIBRARIES}
        pthread
        dl
        m
)

# Define the main executable
# Hosted builds opt in; ordinary installations do not report telemetry.
option(ENABLE_SENTRY "Build optional Sentry native crash reporting" OFF)
if(ENABLE_SENTRY)
    include(FetchContent)
    set(SENTRY_BACKEND "inproc" CACHE STRING "Native crash backend" FORCE)
    set(SENTRY_BUILD_SHARED_LIBS OFF CACHE BOOL "Link Sentry statically" FORCE)
    set(SENTRY_BUILD_EXAMPLES OFF CACHE BOOL "No SDK examples" FORCE)
    set(SENTRY_BUILD_TESTS OFF CACHE BOOL "No SDK tests" FORCE)
    FetchContent_Declare(sentry_native
        GIT_REPOSITORY https://github.com/getsentry/sentry-native.git
        GIT_TAG af9e9367913edf07f54e4f2f9655ce96af0c1681 # 0.17.5
        GIT_SUBMODULES "")
    FetchContent_MakeAvailable(sentry_native)
    target_link_libraries(lightnvr_lib PUBLIC sentry::sentry)
    target_compile_definitions(lightnvr_lib PUBLIC ENABLE_SENTRY=1)
    target_compile_options(lightnvr_lib PRIVATE -g -fdebug-prefix-map=${CMAKE_SOURCE_DIR}=.)
endif()

add_executable(lightnvr src/core/main.c)
if(ENABLE_SENTRY)
    target_compile_options(lightnvr PRIVATE -g -fdebug-prefix-map=${CMAKE_SOURCE_DIR}=.)
    target_link_options(lightnvr PRIVATE -Wl,--build-id)
endif()
if(CMAKE_SYSTEM_NAME STREQUAL "Linux")
    add_executable(lightnvr-ffmpeg-guard src/tools/ffmpeg_rss_guard.c)
    set_target_properties(lightnvr-ffmpeg-guard PROPERTIES
        RUNTIME_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/bin")
    # Audio workers receive RLIMIT_AS. ASan reserves a large virtual shadow
    # mapping before main(), so instrumenting this supervisor defeats that
    # limit in Debug builds. Keep gcov instrumentation from CMAKE_C_FLAGS.
    if(CMAKE_BUILD_TYPE STREQUAL "Debug" AND
       CMAKE_C_COMPILER_ID MATCHES "GNU|Clang" AND
       CMAKE_CXX_COMPILER_ID MATCHES "GNU|Clang")
        target_compile_options(lightnvr-ffmpeg-guard PRIVATE
            -fno-sanitize=address,undefined)
        target_link_options(lightnvr-ffmpeg-guard PRIVATE
            -fno-sanitize=address,undefined)
    endif()
endif()

# Define source files for rebuild_recordings utility (excluding inih)
set(REBUILD_RECORDINGS_SOURCES
    src/tools/rebuild_recordings.c
    src/core/config.c
    src/core/logger.c
    src/core/path_utils.c
    src/core/shutdown_coordinator.c
    src/utils/strings.c
    src/utils/uuid.c
    src/telemetry/system_health_policy.c
    src/telemetry/system_health_types.c
    src/database/db_camera_tags.c
    src/database/db_core.c
    src/database/db_streams.c
    src/database/db_recordings.c
    src/database/db_storage_targets.c
    src/storage/storage_s3.c
    src/storage/storage_deletion.c
    src/ezxml/ezxml.c
    src/video/recording_file.c
    src/video/recording_path.c
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
        ZLIB::ZLIB
        ${SSL_LIBRARIES}
        atomic
        pthread
        dl
        m
)

# Link libraries for rebuild_recordings utility
target_link_libraries(rebuild_recordings
        ${CURL_LIBRARIES}
        ${SSL_LIBRARIES}
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

# Link libavfilter if enabled (voice enhancement)
if(ENABLE_AUDIO_FILTERS AND AVFILTER_FOUND)
    if(AVFILTER_LIBRARY_DIRS)
        target_link_directories(lightnvr_lib PUBLIC ${AVFILTER_LIBRARY_DIRS})
    endif()
    target_link_libraries(lightnvr_lib PUBLIC ${AVFILTER_LIBRARIES})
    target_include_directories(lightnvr_lib PUBLIC ${AVFILTER_INCLUDE_DIRS})
endif()

# Link MQTT library if enabled
if(ENABLE_MQTT AND MOSQUITTO_FOUND)
    if(MOSQUITTO_LIBRARY_DIRS)
        target_link_directories(lightnvr_lib PUBLIC ${MOSQUITTO_LIBRARY_DIRS})
    endif()
    target_link_libraries(lightnvr_lib PUBLIC ${MOSQUITTO_LIBRARIES})
    target_include_directories(lightnvr_lib PUBLIC ${MOSQUITTO_INCLUDE_DIRS})
endif()

# Link LiteRT if enabled
if(ENABLE_LITERT)
    # LiteRT's CMake target is named tensorflow-lite (provided by the LiteRT
    # submodule's tflite/CMakeLists.txt).
    target_link_libraries(lightnvr_lib PUBLIC tensorflow-lite)
    target_link_libraries(lightnvr tensorflow-lite)
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

# Keep the runtime and service configuration path as the installation default.
# Packagers may opt into a prefix-relative layout with -DCMAKE_INSTALL_SYSCONFDIR=etc.
if(NOT DEFINED CMAKE_INSTALL_SYSCONFDIR)
    set(CMAKE_INSTALL_SYSCONFDIR "/etc" CACHE PATH "System configuration directory")
endif()

# Install targets
include(GNUInstallDirs)
install(TARGETS lightnvr rebuild_recordings DESTINATION bin)
if(TARGET lightnvr-ffmpeg-guard)
    install(TARGETS lightnvr-ffmpeg-guard DESTINATION bin)
endif()
install(DIRECTORY config/ DESTINATION "${CMAKE_INSTALL_SYSCONFDIR}/lightnvr")
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
if(DEFINED ENV{LIGHTNVR_GIT_COMMIT} AND NOT "$ENV{LIGHTNVR_GIT_COMMIT}" STREQUAL "")
    set(GIT_COMMIT_INPUT "$ENV{LIGHTNVR_GIT_COMMIT}")
    string(LENGTH "${GIT_COMMIT_INPUT}" GIT_COMMIT_LENGTH)
    if(NOT GIT_COMMIT_INPUT MATCHES "^[0-9a-fA-F]+$" OR
       GIT_COMMIT_LENGTH LESS 7 OR GIT_COMMIT_LENGTH GREATER 40)
        message(FATAL_ERROR
            "Invalid LIGHTNVR_GIT_COMMIT: expected a 7-40 character hexadecimal commit")
    endif()
    string(SUBSTRING "${GIT_COMMIT_INPUT}" 0 7 GIT_COMMIT)
    string(TOLOWER "${GIT_COMMIT}" GIT_COMMIT)
else()
    find_package(Git QUIET)
    if(GIT_FOUND)
        execute_process(
            COMMAND ${GIT_EXECUTABLE} rev-parse --short HEAD
            WORKING_DIRECTORY ${CMAKE_SOURCE_DIR}
            OUTPUT_VARIABLE GIT_COMMIT_CANDIDATE
            RESULT_VARIABLE GIT_REV_PARSE_RESULT
            OUTPUT_STRIP_TRAILING_WHITESPACE
            ERROR_QUIET
        )
        if(GIT_REV_PARSE_RESULT EQUAL 0 AND NOT "${GIT_COMMIT_CANDIDATE}" STREQUAL "")
            set(GIT_COMMIT "${GIT_COMMIT_CANDIDATE}")
            string(TOLOWER "${GIT_COMMIT}" GIT_COMMIT)
        endif()
    endif()
endif()

# Build date in ISO-8601
string(TIMESTAMP BUILD_DATE "%Y-%m-%d")

# Create a version.h file in the build tree (out-of-source safe)
file(MAKE_DIRECTORY "${GENERATED_INCLUDE_DIR}/core")
file(READ
        "${CMAKE_CURRENT_SOURCE_DIR}/include/core/version.h.in"
        VERSION_H_IN
)
string(CONFIGURE "${VERSION_H_IN}" VERSION_H_OUT @ONLY)
file(GENERATE
        OUTPUT "${GENERATED_INCLUDE_DIR}/core/version.h"
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
message(STATUS "- Build profile: ${LIGHTNVR_PROFILE}")
message(STATUS "- Compile-time stream ceiling: ${LIGHTNVR_RESOLVED_MAX_STREAMS}")
message(STATUS "- HTTP backend: libuv + llhttp")
message(STATUS "- SOD object detection: ${ENABLE_SOD}")
if(ENABLE_SOD)
    if(SOD_DYNAMIC_LINK)
        message(STATUS "  - SOD linking method: Dynamic")
    else()
        message(STATUS "  - SOD linking method: Static")
    endif()
endif()
message(STATUS "- LiteRT (in-process TFLite engine): ${ENABLE_LITERT}")
if(ENABLE_LITERT)
    message(STATUS "  - XNNPACK delegate: ${LITERT_WITH_XNNPACK}")
    message(STATUS "  - GPU delegate: ${LITERT_WITH_GPU}")
endif()
message(STATUS "- go2rtc WebRTC integration: ${ENABLE_GO2RTC}")
if(ENABLE_GO2RTC)
    message(STATUS "  - go2rtc binary path: ${GO2RTC_BINARY_PATH}")
    message(STATUS "  - go2rtc config directory: ${GO2RTC_CONFIG_DIR}")
    message(STATUS "  - go2rtc API port: ${GO2RTC_API_PORT}")
endif()
if(YAML_FOUND)
    message(STATUS "- libyaml (YAML validation): ENABLED (${YAML_VERSION})")
else()
    message(STATUS "- libyaml (YAML validation): DISABLED (stub only)")
endif()
