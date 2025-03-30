# go2rtc integration module

# Define source files with full paths
set(GO2RTC_SOURCES
    ${CMAKE_CURRENT_SOURCE_DIR}/go2rtc_process.c
    ${CMAKE_CURRENT_SOURCE_DIR}/go2rtc_api.c
    ${CMAKE_CURRENT_SOURCE_DIR}/go2rtc_stream.c
    ${CMAKE_CURRENT_SOURCE_DIR}/go2rtc_consumer.c
    ${CMAKE_CURRENT_SOURCE_DIR}/go2rtc_integration.c
)

# Add sources to parent scope
set(GO2RTC_SOURCES ${GO2RTC_SOURCES} PARENT_SCOPE)

# Print information
message(STATUS "go2rtc integration module configured")
