cmake_minimum_required(VERSION 3.10)
project(SOD VERSION 1.1.9 LANGUAGES C)

# Set C standard
set(CMAKE_C_STANDARD 11)

# Ingenic MXU2/MXU3 is useful only on MIPS targets.  Runtime probing keeps a
# single binary safe across XBurst generations and on MIPS CPUs without MXU.
string(TOLOWER "${CMAKE_SYSTEM_PROCESSOR}" SOD_SYSTEM_PROCESSOR)
set(SOD_INGENIC_MXU_DEFAULT OFF)
if(SOD_SYSTEM_PROCESSOR MATCHES "^mips")
    set(SOD_INGENIC_MXU_DEFAULT ON)
endif()
option(SOD_ENABLE_INGENIC_MXU
    "Enable runtime-dispatched Ingenic MXU2/MXU3 GEMM acceleration"
    ${SOD_INGENIC_MXU_DEFAULT})

# Compiler flags for optimization and memory usage
set(CMAKE_C_FLAGS "${CMAKE_C_FLAGS} -O2 -ffunction-sections -fdata-sections -Wl,--gc-sections -pthread")

# Define source files
set(SOD_SOURCES
    sod.c
)
if(SOD_ENABLE_INGENIC_MXU)
    list(APPEND SOD_SOURCES sod_ingenic_mxu.c)
    find_package(Threads REQUIRED)
endif()

# Define include directories
include_directories(
    ${CMAKE_CURRENT_SOURCE_DIR}/../../include
)

# Build as a shared library
add_library(sod SHARED ${SOD_SOURCES})
target_link_libraries(sod m)
if(SOD_ENABLE_INGENIC_MXU)
    target_compile_definitions(sod PRIVATE SOD_ENABLE_INGENIC_MXU=1)
    target_link_libraries(sod Threads::Threads)
    message(STATUS "SOD Ingenic MXU2/MXU3 acceleration enabled")
endif()

# Set library version
set_target_properties(sod PROPERTIES
    VERSION ${PROJECT_VERSION}
    SOVERSION 1
)

# Install targets
install(TARGETS sod
    LIBRARY DESTINATION lib
)

# Install headers
install(FILES
    ${CMAKE_CURRENT_SOURCE_DIR}/../../include/sod/sod.h
    ${CMAKE_CURRENT_SOURCE_DIR}/../../include/sod/sod_img_reader.h
    ${CMAKE_CURRENT_SOURCE_DIR}/../../include/sod/sod_img_writer.h
    DESTINATION include/sod
)
