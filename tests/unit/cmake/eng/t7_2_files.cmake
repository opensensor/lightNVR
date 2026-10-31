include(CheckCCompilerFlag)
check_c_compiler_flag("-ftrivial-auto-var-init=zero" T7_2_FILES_HAS_ZERO_INIT)
if(NOT T7_2_FILES_HAS_ZERO_INIT)
    message(FATAL_ERROR "T7.2.3 deterministic descriptor probe requires compiler auto-variable initialization")
endif()
# Compile the unchanged helper separately for the permitted-value probe only.
# Do not alter lightnvr_lib's source properties or production compiler flags.
add_library(t7_2_files_path_probe OBJECT ${CMAKE_SOURCE_DIR}/src/core/path_utils.c)
target_include_directories(t7_2_files_path_probe PRIVATE ${CMAKE_SOURCE_DIR}/include)
target_compile_options(t7_2_files_path_probe PRIVATE -ftrivial-auto-var-init=zero)
add_layer3_test(test_t7_2_files)
target_sources(test_t7_2_files PRIVATE $<TARGET_OBJECTS:t7_2_files_path_probe>)
target_link_options(test_t7_2_files PRIVATE
    -Wl,--wrap=open -Wl,--wrap=fchmod -Wl,--wrap=fstatat
    -Wl,--wrap=get_db_handle -Wl,--wrap=get_streaming_config
    -Wl,--wrap=pthread_create -Wl,--wrap=storage_migration_worker_start
    -Wl,--wrap=storage_source_worker_start -Wl,--wrap=storage_target_health_register
    -Wl,--wrap=get_all_stream_names_including_disabled
    -Wl,--wrap=get_orphaned_recording_stream_names)
set_tests_properties(test_t7_2_files PROPERTIES TIMEOUT 30 LABELS "engineering;security")
