cmake_minimum_required(VERSION 3.10)
project(SOD VERSION 1.1.9 LANGUAGES C)

# Set C standard
set(CMAKE_C_STANDARD 11)

# Compiler flags for optimization and memory usage
set(CMAKE_C_FLAGS "${CMAKE_C_FLAGS} -O2 -ffunction-sections -fdata-sections -Wl,--gc-sections -pthread")

# Define source files
set(SOD_SOURCES
    sod.c
)

# Define include directories
include_directories(
    ${CMAKE_CURRENT_SOURCE_DIR}/../../include
)

# Build as a shared library
add_library(sod SHARED ${SOD_SOURCES})
target_link_libraries(sod m)

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
